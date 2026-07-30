# npm wrapper for `ghl-mcp`

Publishing `ghl-mcp` to npm so MCP hosts can launch it with `npx ghl-mcp`, which
is what most host configs assume. The package ships no JavaScript logic beyond a
downloader and a launcher: the server itself is the Rust binary.

## Release order

1. Tag the repo — `git tag v0.5.0 && git push --tags`. That runs
   `.github/workflows/release.yml`, which builds `ghl-mcp` for macOS
   (arm64/x64), Linux (x64/arm64) and Windows (x64) and attaches
   `ghl-mcp-<target>.tar.gz` to the GitHub Release.
2. Wait for the release assets to appear — `install.js` downloads from them, so
   publishing npm first would leave `npx` broken.
3. `cd npm/ghl-mcp && npm publish`.

Keep `npm/ghl-mcp/package.json`'s `version` in lockstep with the crate version:
the installer builds its download URL from it.

## Verifying

```bash
npx --yes ghl-mcp@0.5.0 --version
```

If a platform has no prebuilt binary the installer says so and points at
`cargo install ghl-mcp` rather than failing silently.
