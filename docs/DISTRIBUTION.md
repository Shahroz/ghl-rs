# Releasing and distributing `ghl-mcp`

Everything needed to publish a release, in order. Steps 1–3 are automated; 4–6 need credentials only you have.

## 1. Tag

```bash
git tag v0.6.0 && git push origin v0.6.0
```

`.github/workflows/release.yml` then:

- builds `ghl-mcp` for macOS (arm64/x64), Linux (x64/arm64) and Windows (x64),
- attaches `ghl-mcp-<target>.tar.gz` to the GitHub Release,
- runs `xtask/bump_formula.py` to point `Formula/ghl-mcp.rb` at the new tag with fresh checksums, and commits that to `main`.

## 2. crates.io

Order matters — `ghl-sdk` depends on `ghl-models`, and `ghl-mcp` on both:

```bash
cargo publish -p ghl-models && sleep 25 && cargo publish -p ghl-sdk && sleep 25 && cargo publish -p ghl-mcp
```

Commit first: `cargo publish` refuses a dirty tree, and publishing from one loses the provenance link.

## 3. Homebrew

No separate tap repo — this repo *is* the tap:

```bash
brew tap shahroz/ghl-rs https://github.com/Shahroz/ghl-rs
brew trust shahroz/ghl-rs
brew install ghl-mcp
```

`brew tap` takes an explicit URL, so `Formula/ghl-mcp.rb` here works without a `homebrew-*` repo. The release workflow keeps it current.

**`brew trust` is not optional.** Current Homebrew refuses to load a formula from any untrusted third-party tap — `Refusing to load formula … from untrusted tap`. This applies to every third-party tap, not just this one, so the instructions must include it. Verified end to end: tap → trust → install → `brew test` passes → `ghl-mcp --version` reports the tagged version.

To move to the canonical `Shahroz/homebrew-tap` later, copy `Formula/` there and point the bump script at it.

## 4. npm (for `npx ghl-mcp`)

Wait for the release assets first — `install.js` downloads from them, so publishing npm early leaves `npx` broken.

```bash
cd npm/ghl-mcp
npm version 0.6.0 --no-git-tag-version
npm publish
```

Verify: `npx --yes ghl-mcp@0.6.0 --version`

## 5. Container image

```bash
docker build -t ghcr.io/shahroz/ghl-mcp:0.6.0 -t ghcr.io/shahroz/ghl-mcp:latest .
docker push ghcr.io/shahroz/ghl-mcp:0.6.0
docker push ghcr.io/shahroz/ghl-mcp:latest
```

Needs `docker login ghcr.io` with a PAT carrying `write:packages`. The image is distroless and about 64 MB.

## 6. MCP registries

Manifests are committed and versioned with the crate; each registry needs an interactive login you hold.

**Official MCP registry** — `server.json` declares both the npm (stdio) and OCI (Streamable HTTP) packages:

```bash
# one-time: brew install mcp-publisher   (or download from the registry repo)
mcp-publisher login github        # opens a browser; ties io.github.shahroz/* to the account
mcp-publisher publish             # reads ./server.json
```

The namespace `io.github.shahroz/ghl-mcp` requires proving ownership of that GitHub account, which is why this can't be automated from here.

**Smithery** — `smithery.yaml` describes the container build and its config schema. Connect the repo at [smithery.ai/new](https://smithery.ai/new); Smithery builds from the `Dockerfile` and serves Streamable HTTP.

**mcp.so** — submit at [mcp.so/submit](https://mcp.so/submit) with the repo URL; it reads the README.

After each, bump `version` in `server.json` and `npm/ghl-mcp/package.json` alongside the crate version so the manifests never point at a release that doesn't exist.

## Checklist

```
[ ] CI green on main
[ ] versions bumped in Cargo.toml, server.json, npm/ghl-mcp/package.json
[ ] git tag pushed, release workflow green, 5 assets attached
[ ] crates.io: models -> sdk -> mcp
[ ] formula bumped (automatic) — verify `brew install` on a clean machine
[ ] npm published, `npx ghl-mcp --version` works
[ ] container pushed to ghcr.io
[ ] registries updated (official / Smithery / mcp.so)
```
