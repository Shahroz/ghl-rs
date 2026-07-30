#!/usr/bin/env python3
"""Point Formula/ghl-mcp.rb at a release tag, with fresh checksums.

Usage:
    python3 xtask/bump_formula.py <version>      # e.g. 0.5.0

Downloads each release asset, hashes it, and rewrites the formula's version,
urls and sha256s in place. Run by the release workflow after the binaries are
attached; safe to run by hand too.
"""
import hashlib
import pathlib
import re
import sys
import urllib.request

TARGETS = [
    "aarch64-apple-darwin",
    "x86_64-apple-darwin",
    "aarch64-unknown-linux-gnu",
    "x86_64-unknown-linux-gnu",
]
FORMULA = pathlib.Path("Formula/ghl-mcp.rb")


def sha256_of(url: str) -> str:
    with urllib.request.urlopen(url) as response:
        return hashlib.sha256(response.read()).hexdigest()


def main() -> int:
    if len(sys.argv) != 2:
        print(__doc__.strip(), file=sys.stderr)
        return 2
    version = sys.argv[1].lstrip("v")
    text = FORMULA.read_text()

    for target in TARGETS:
        url = (
            f"https://github.com/Shahroz/ghl-rs/releases/download/"
            f"v{version}/ghl-mcp-{target}.tar.gz"
        )
        digest = sha256_of(url)
        # Rewrite the url line and the sha256 that immediately follows it.
        pattern = (
            r'url "[^"]*ghl-mcp-' + re.escape(target) + r'\.tar\.gz"\n(\s*)sha256 "[0-9a-f]{64}"'
        )
        text, count = re.subn(pattern, f'url "{url}"\n\\g<1>sha256 "{digest}"', text)
        if count != 1:
            print(f"!! expected 1 block for {target}, rewrote {count}", file=sys.stderr)
            return 1
        print(f"{target} -> {digest[:12]}…")

    text, count = re.subn(r'^  version "[^"]+"', f'  version "{version}"', text, flags=re.M)
    if count != 1:
        print(f"!! expected 1 version line, rewrote {count}", file=sys.stderr)
        return 1

    FORMULA.write_text(text)
    print(f"formula bumped to {version}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
