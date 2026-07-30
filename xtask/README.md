# xtask

Maintenance scripts. Not part of the published crates.

Both generators read a local checkout of HighLevel's official spec repo:

```bash
git clone https://github.com/GoHighLevel/highlevel-api-docs.git
```

Layout that matters: `apps/*.json` are the V2 module specs, `apps/v3/*.json` the
V3 ones, and `common/` + `apps/v3/common/` hold shared error DTOs
(`BadRequestDTO`, `UnauthorizedDTO`, `UnprocessableDTO`, …) whose shape the SDK's
error handling already mirrors.

## 1. Operations catalog (for `ghl-mcp`)

`crates/ghl-mcp/operations.json` is embedded in the binary and powers the
meta-tools. It covers **both** API versions — V3 ids are prefixed `v3:`.

```bash
python3 xtask/build_operations_index.py ../highlevel-api-docs crates/ghl-mcp/operations.json
```

Keeps only what the meta-tools need (id, module, api_version, method, path,
summary, params, body field names, scopes, `Version` header) so the catalog stays
around 650 KiB instead of shipping full schemas.

## 2. Data models (the `ghl-models` crate)

```bash
python3 xtask/generate_models.py ../highlevel-api-docs crates/ghl-models
```

Regenerates every DTO (~2,400 structs) into `crates/ghl-models/src/{v2,v3}/`,
one Rust module per API module. **Also update `crates/ghl-models/Cargo.toml`** if
HighLevel adds or renames a module, since each one is its own cargo feature.

## 3. API reference docs

```bash
python3 xtask/generate_api_docs.py ../highlevel-api-docs docs/api
```

Writes `docs/api/<module>.md` for all 45 modules (every endpoint with params,
body fields, scopes, `Version` header, and a copy-pasteable MCP call; every model
with field types; every enum with allowed values), plus `docs/api/README.md` as
the index and `docs/api/shared-enums.md` for big repeated value lists.

## After regenerating any of these

```bash
cargo test --workspace && cargo clippy --workspace --all-targets
```

The catalog tests assert operation counts per version, module presence, and that
every entry has a sane method/path/`Version`. Commit the regenerated JSON and
Rust files together with any test updates.
