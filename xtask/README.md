# xtask

Maintenance scripts. Not part of the published crates.

## Regenerating the operations catalog

`crates/ghl-mcp/operations.json` is generated from GoHighLevel's official
OpenAPI specs and embedded in the `ghl-mcp` binary. Refresh it when HighLevel
ships new endpoints:

```bash
curl -sL https://codeload.github.com/GoHighLevel/highlevel-api-docs/tar.gz/refs/heads/main | tar xz
```

```bash
python3 xtask/build_operations_index.py highlevel-api-docs-main/apps crates/ghl-mcp/operations.json
```

Then run `cargo test -p ghl-mcp` — the catalog tests assert the module count and
that every entry still has a usable method/path shape. Commit the regenerated
JSON along with any test updates.

The generator keeps only what the meta-tools need (id, module, method, path,
summary, params, body field names, scopes, `Version` header) so the embedded
catalog stays around 300 KiB rather than shipping full request/response schemas.
