# ghl-rs

**The agency-grade GoHighLevel toolkit for Rust and AI agents.**

One typed SDK. One static-binary MCP server. Every location in your agency — no Node runtime, no per-location reconnecting, no waiting on official coverage.

[![CI](https://img.shields.io/badge/CI-passing-brightgreen)](.github/workflows/ci.yml)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)
![MSRV 1.88](https://img.shields.io/badge/MSRV-1.88-orange)

| Crate | What it is |
|---|---|
| [`ghl-sdk`](crates/ghl-sdk) | Async, typed Rust client for the GoHighLevel API — OAuth 2.0 + Private Integration Tokens, automatic token refresh, rate-limit-aware retries, pagination as `Stream`s, webhook signature verification |
| [`ghl-models`](crates/ghl-models) | **2,417 generated DTOs** for every API module, both API versions, feature-gated per module |
| [`ghl-mcp`](crates/ghl-mcp) | [MCP](https://modelcontextprotocol.io) server exposing GoHighLevel to Claude, ChatGPT, Gemini, and any MCP host — built on the official `rmcp` SDK, ships as a single binary |

**Complete API coverage.** Generated from [HighLevel's official OpenAPI specs](https://github.com/GoHighLevel/highlevel-api-docs):

| | Count |
|---|---|
| **Typed Rust methods** | **1,203** — one per endpoint, API v2 **and** v3 |
| Typed data models (DTOs) | **2,417** (v2 + v3) |
| API modules covered | **45** across both versions |
| MCP tools | 21 (16 typed + 3 meta-tools + 2 utility) |

**You never have to leave the library.** Every endpoint in both API versions is a real method with typed parameters and a typed response — invoices, payments, ad manager, social planner, voice AI, SaaS, custom objects, workflows, all of it. Five busy modules also get hand-written helpers (envelope unwrapping, paginated `Stream`s) on the same services.

```rust
use ghl_sdk::services::invoices::ListInvoicesParams;

let params = ListInvoicesParams::new(&location_id, "location", "20", "0").status("draft");
let page = ghl.invoices().list_invoices(&params).await?;      // API v2

let dup = ghl.v3().contacts().get_duplicate_contact(&p).await?;   // API v3
```

## Documentation

| Doc | What's in it |
|---|---|
| **[Usage guide](docs/GUIDE.md)** | Auth decision tree, per-module cookbook, pagination, errors, rate limits, agency/multi-location, v2-vs-v3, troubleshooting |
| **[API reference](docs/api/README.md)** | All **45 modules**: every endpoint (params, body fields, scopes, `Version`), every struct, every enum value |
| [docs.rs/ghl-sdk](https://docs.rs/ghl-sdk) | Client API docs |
| [docs.rs/ghl-models](https://docs.rs/ghl-models) | All 2,417 DTOs, field by field |
| [Design proposal](docs/PROPOSAL.md) | Research and architecture rationale |
| [Release & distribution](docs/DISTRIBUTION.md) | Tagging, crates.io, Homebrew, npm, Docker, MCP registries |
| [Changelog](CHANGELOG.md) | What changed in every release, including the breaking ones |

## Why this exists

GoHighLevel powers **60k+ agencies and ~2M businesses**, but its developer stack has gaps:

- **No official Rust SDK** (official support: TypeScript, plus low-adoption Python/PHP).
- The **official MCP server is locked to a single sub-account** — no agency-wide access, ~36 tools, $297+/mo plans only.
- **No developer support policy**: HighLevel support explicitly does not help with API integration code.

`ghl-rs` fills those gaps:

|  | Official MCP server | Community Node servers | **ghl-mcp** |
|---|---|---|---|
| API coverage | ~36 curated tools | varies, often stale | ✅ **1,203 typed Rust methods** + all of them via meta-tools |
| API v3 support | ❌ | ❌ | ✅ v2 and v3 side by side |
| Typed data models | — | ❌ | ✅ 2,417 generated DTOs |
| Agency (multi-location) access | ❌ one location per connection | ⚠️ varies | ✅ agency token → per-location routing |
| Self-hostable | ❌ | ✅ | ✅ |
| Runtime | hosted | Node.js | **single static binary** |
| Typed end-to-end | — | ❌ | ✅ Rust types from official OpenAPI specs |
| Rate-limit awareness | opaque | mostly none | ✅ header-driven backoff, per-location budgets |
| Destructive-action gating | ❌ | mostly none | ✅ off by default, `--allow-destructive` to enable |

## Quickstart — SDK

```toml
[dependencies]
ghl-sdk = { version = "0.5", features = ["contacts", "invoices"] }
tokio = { version = "1", features = ["full"] }
```

```rust,no_run
use ghl_sdk::{Ghl, contacts::CreateContact};

#[tokio::main]
async fn main() -> Result<(), ghl_sdk::Error> {
    // Reads GHL_PIT_TOKEN (or GHL_ACCESS_TOKEN) from the environment…
    let ghl = Ghl::from_env()?;
    // …or configure explicitly:
    // let ghl = Ghl::builder().private_integration_token("pit-…").build()?;

    let contact = ghl
        .contacts()
        .create(CreateContact {
            location_id: "LOCATION_ID".into(),
            email: Some("ada@example.com".into()),
            first_name: Some("Ada".into()),
            ..Default::default()
        })
        .await?;

    println!("created contact {}", contact.id);
    Ok(())
}
```

Need an endpoint without a typed service? Use the generated DTOs plus `request_raw`:

```toml
ghl-sdk = { version = "0.5", features = ["models"] }
ghl-models = { version = "0.5", features = ["invoices"] }
```

```rust,ignore
use ghl_models::v2::invoices::CreateInvoiceDto;

let body = serde_json::to_value(CreateInvoiceDto {
    alt_id: location_id.clone(),
    alt_type: "location".into(),
    name: "August retainer".into(),
    currency: "USD".into(),
    ..Default::default()
})?;
let created = ghl.request_raw("POST", "/invoices/", &[], Some(&body), None).await?;
```

Receiving webhooks? Verify HighLevel's RSA signature before trusting a payload:

```toml
ghl-sdk = { version = "0.5", features = ["webhooks"] }
```

```rust,ignore
ghl_sdk::webhooks::verify(raw_body, signature_header)?;   // raw bytes, not re-serialized
let event: ghl_sdk::webhooks::WebhookEvent = serde_json::from_slice(raw_body)?;
if event.is_stale(std::time::Duration::from_secs(300)) { return; }   // replay guard
```

Pagination is a `Stream` — the SDK handles GoHighLevel's cursor scheme (`startAfterId`) for you:

```rust,ignore
use futures_util::TryStreamExt;

let mut contacts = ghl.contacts().list("LOCATION_ID").limit(100).stream();
while let Some(c) = contacts.try_next().await? {
    println!("{} {:?}", c.id, c.email);
}
```

## Quickstart — MCP server

```sh
cargo install ghl-mcp    # or: npx ghl-mcp · brew install · docker run
```

Claude Desktop / Claude Code config:

```json
{
  "mcpServers": {
    "gohighlevel": {
      "command": "ghl-mcp",
      "env": {
        "GHL_PIT_TOKEN": "pit-…",
        "GHL_LOCATION_ID": "your-location-id"
      }
    }
  }
}
```

Then ask your agent things like *"find every contact tagged `hot-lead` added this week and summarize them"* — the server handles auth, retries, rate limits, and pagination.

To share one server between several agents, serve **Streamable HTTP** instead of stdio:

```sh
ghl-mcp --http 127.0.0.1:8000 --http-auth-token "$(openssl rand -hex 32)"
```

Callers then send `Authorization: Bearer <token>`. Omit the flag and the endpoint is unauthenticated (the server warns at startup) — only do that on localhost.

Install without Rust:

```sh
npx ghl-mcp
```

```sh
brew tap shahroz/ghl-rs https://github.com/Shahroz/ghl-rs
brew trust shahroz/ghl-rs        # Homebrew requires this for any third-party tap
brew install ghl-mcp
```

```sh
docker run -p 8000:8000 -e GHL_PIT_TOKEN=pit-… ghcr.io/shahroz/ghl-mcp
```

## Configuration

Everything is configurable **by environment variable or explicitly as a parameter** — env vars are the zero-code path, builder/CLI parameters always win when both are set.

| Env var | CLI flag (`ghl-mcp`) | SDK builder | Purpose |
|---|---|---|---|
| `GHL_PIT_TOKEN` | `--pit-token` | `.private_integration_token(…)` | Private Integration Token (simplest auth) |
| `GHL_ACCESS_TOKEN` | `--access-token` | `.access_token(…)` | OAuth access token (bring your own flow) |
| `GHL_LOCATION_ID` | `--location-id` | — | Default sub-account for MCP tools |
| `GHL_BASE_URL` | `--base-url` | `.base_url(…)` | API base (default `https://services.leadconnectorhq.com`) |
| `GHL_ALLOW_DESTRUCTIVE` | `--allow-destructive` | — | Enable write/delete/send tools (off by default) |
| `GHL_HTTP_ADDR` | `--http` | — | Serve Streamable HTTP instead of stdio |
| `GHL_HTTP_AUTH_TOKEN` | `--http-auth-token` | — | Require a bearer token on the HTTP endpoint |
| `RUST_LOG` | — | — | Log filter (logs go to stderr, MCP-safe) |

Secrets are held in [`secrecy`](https://docs.rs/secrecy) types — they never appear in `Debug` output or logs.

## Status & roadmap

The MCP server reaches the **entire** API today; typed SDK services cover the busiest modules and keep growing. Design rationale lives in [docs/PROPOSAL.md](docs/PROPOSAL.md).

- [x] Private Integration Token + OAuth token refresh + `/oauth/locationToken` exchange
- [x] Contacts (CRUD + cursor-paginated list as `Stream`)
- [x] Opportunities (pipelines, search, CRUD, stage/status moves)
- [x] Conversations (search threads, read messages, send SMS/email)
- [x] Calendars (list, free slots, book/fetch appointments)
- [x] Locations (get + search, with location-scoped fallback)
- [x] MCP server: 21 tools over stdio, write/destructive gating
- [x] Meta-tools reaching all **1,203 operations / 45 modules**, v2 **and** v3
- [x] **2,417 generated DTOs** in `ghl-models`, feature-gated per module
- [x] **1,203 generated typed service methods** — every endpoint in API v2 and v3
- [x] Webhook RSA signature verification + typed events (`webhooks` feature)
- [x] Streamable HTTP transport for the MCP server (`--http`)
- [x] `npx ghl-mcp` wrapper, Docker image, prebuilt release binaries
- [x] Homebrew formula, MCP registry manifests (`server.json`, `smithery.yaml`)
- [x] Bearer-token auth on the HTTP endpoint
- [ ] Hosted gateway as a product: token vault, per-tenant rate pooling, audit trail, billing

## License

MIT or Apache-2.0, at your option.

> **Not affiliated with HighLevel Inc.** "GoHighLevel" and "HighLevel" are trademarks of their respective owners. This is an independent, unofficial open-source project.
