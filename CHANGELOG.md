# Changelog

All three crates (`ghl-models`, `ghl-sdk`, `ghl-mcp`) share a version number and
are released together. Dates are release dates.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/);
versions are `0.x`, so a minor bump may carry breaking changes.

## [0.5.2] — 2026-07-30

### Fixed

- Docs-only release. The `models` re-export's rustdoc still showed `version = "0.3"`
  in its example and the feature list said "all 41" rather than 45 — both live on
  docs.rs, hence a publish rather than just a commit.
- `ghl-models`' README opened with a garbled sentence that double-counted the type
  totals.

### Added

- This changelog, covering every release back to 0.1.0.
- [`docs/PROPOSAL.md`](docs/PROPOSAL.md) now states up front that it is the
  historical design record, with a table of where the shipped result diverges from
  what was planned — it claimed 5 typed modules and ~413 operations, both long
  since overtaken.

## [0.5.1] — 2026-07-30

### Added

- **Bearer auth for the MCP HTTP transport** — `--http-auth-token` /
  `GHL_HTTP_AUTH_TOKEN`. Compared in constant time; unauthenticated requests get
  `401` plus `WWW-Authenticate: Bearer`. Without it the endpoint stays open and
  the server warns loudly at startup.
- **Homebrew formula** at `Formula/ghl-mcp.rb`, installable straight from this
  repo (`brew tap shahroz/ghl-rs <url>`, then `brew trust`, then
  `brew install ghl-mcp`) — no separate `homebrew-*` tap repo needed. The release
  workflow repoints it at each tag with fresh checksums.
- **MCP registry manifests**: `server.json` (official registry — declares the npm
  stdio package and the OCI Streamable-HTTP package) and `smithery.yaml`.
- [`docs/DISTRIBUTION.md`](docs/DISTRIBUTION.md) — the full release path in order,
  with the ordering constraints that matter.

### Fixed

- Install instructions omitted `brew trust`, which current Homebrew requires for
  any third-party tap — `brew install` failed without it.

## [0.5.0] — 2026-07-30

### Added

- **Generated typed services for API v3** — 627 more methods, reached through
  `ghl.v3()`, which sends `Version: v3`. Combined with v2 that is **1,203 typed
  methods across all 45 modules**: every endpoint in both API versions.
- **Webhooks** (`webhooks` feature) — `verify()` checks HighLevel's RSA-SHA256
  (PKCS#1 v1.5) signature against their published key, `verify_with_key()` takes a
  replacement after a key rotation, and `WebhookEvent` types the envelope its 58
  event types share while keeping event-specific fields in a `data` map.
  `is_stale()` implements the replay window HighLevel recommends.
- **Streamable HTTP transport** for `ghl-mcp` (`--http <addr>`, endpoint `/mcp`),
  stateless per MCP `2026-07-28` so it scales horizontally with no session store.
- **Distribution**: distroless Docker image (~64 MB), an npm wrapper so
  `npx ghl-mcp` works in MCP host configs, and a release workflow producing
  binaries for macOS arm64/x64, Linux x64/arm64 and Windows x64.

### Changed

- Module cargo features now cover all **45** modules and enable both API versions
  of a module together, matching how `ghl-models` features already behaved.

## [0.4.1] — 2026-07-30

### Fixed

- `dead_code` warning in a default build (no module features): the services
  helper is only reachable from feature-gated code. CI now lints both feature
  extremes so this class of warning can't recur.

## [0.4.0] — 2026-07-30

### Added

- **576 generated typed service methods** — one per API v2 endpoint, across all
  41 v2 modules, wired to the `ghl-models` DTOs. Each module is a cargo feature
  that forwards to the matching `ghl-models` feature.
- Uniform method shape: path parameters positional in URL order, query parameters
  in one generated `Params` struct (required fields as `new()` arguments, optional
  ones as chainable setters), bodies as generated DTOs, and the spec's response
  type as the return type where it names one.

### Changed

- **Breaking:** response DTOs are now lenient — every field is `Option`/defaulted
  on schemas reachable from responses (586 response-only + 83 dual-use), while
  request-only schemas (402) keep their required fields non-`Option`. A wiremock
  test caught the previous strict types failing to deserialize valid responses
  where GoHighLevel omits a field its own spec marks required. Callers will see
  `Option<T>` where response types previously had `T`.

## [0.3.1] — 2026-07-30

### Fixed

- docs.rs pages were near-empty stubs: the substantive docs lived in repo
  markdown, which docs.rs never renders. Crate- and module-level rustdoc now
  carries the coverage tables, auth guidance, per-module endpoint/scope mappings
  and runnable examples. `ghl-sdk` renders on docs.rs with a `_docs` feature so
  the models re-export appears populated.

## [0.3.0] — 2026-07-30

### Added

- **`ghl-models`** — 2,417 DTOs generated from the official OpenAPI specs
  (1,074 v2 + 1,329 v3 structs, 14 string-enum aliases) across 45 modules, each
  behind its own cargo feature because compiling all of them is ~20× slower than
  compiling one.
- MCP operations catalog extended to **both API versions**: 1,203 operations
  (576 v2 + 627 v3). V3 ids are prefixed `v3:`, and each entry carries the
  `Version` header from its own spec — the V3 `ad-publishing` module declares
  `2021-07-28`, and ~29 operations declare none.
- `Ghl::request_raw` for arbitrary method/path/query/body with the same auth,
  retry and rate-limit handling, plus a per-request `Version` override.

### Fixed

- `ghl_search_operations` treated a blank `module`/`api_version` string as a
  filter rather than "no filter", breaking the browse path agents hit first.

## [0.2.0] — 2026-07-29

### Added

- **MCP meta-tools** (`ghl_search_operations`, `ghl_describe_operation`,
  `ghl_execute_operation`) reaching all 576 API v2 operations from an embedded
  catalog, so agents could call modules with no typed code. Writes route through
  the same `--allow-destructive` gate as the typed destructive tools.
- Typed `conversations` (search threads, read messages, send SMS/email) and
  `calendars` (list, free slots, book/fetch appointments) modules.
- `altType` is auto-filled alongside `altId` — every spec that has one enumerates
  only `location`, and omitting it produced a 422.

## [0.1.1] — 2026-07-29

### Fixed

- `ghl-mcp`'s crates.io Documentation link pointed at a docs.rs page that will
  never exist: docs.rs doesn't build docs for binary-only crates. It now points
  at the README.

## [0.1.0] — 2026-07-29

Initial release.

### Added

- **`ghl-sdk`** — async client for the GoHighLevel API 2.0: Private Integration
  Tokens, raw OAuth access tokens, or full OAuth 2.0 with single-flight automatic
  refresh behind a pluggable `TokenStore`; agency→location token exchange;
  retries with exponential backoff and jitter honoring `Retry-After`, idempotent
  methods only; live rate-limit headroom from response headers; cursor pagination
  as `Stream`s; secrets in `secrecy` types, redacted from `Debug`.
- Typed `contacts`, `opportunities` and `locations` modules.
- **`ghl-mcp`** — MCP server on the official `rmcp` SDK, stdio transport,
  destructive actions gated off by default.
- `ghl_list_locations` falls back to the configured location when the credential
  is location-scoped: `/locations/search` is agency-token-only and returns 403
  regardless of granted scopes.

[0.5.2]: https://github.com/Shahroz/ghl-rs/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/Shahroz/ghl-rs/releases/tag/v0.5.1
[0.5.0]: https://github.com/Shahroz/ghl-rs/releases/tag/v0.5.0
[0.4.1]: https://github.com/Shahroz/ghl-rs/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/Shahroz/ghl-rs/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/Shahroz/ghl-rs/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/Shahroz/ghl-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/Shahroz/ghl-rs/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/Shahroz/ghl-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/Shahroz/ghl-rs/releases/tag/v0.1.0
