# ghl-rs

**The agency-grade GoHighLevel toolkit for Rust and AI agents.**

One typed SDK. One static-binary MCP server. Every location in your agency — no Node runtime, no per-location reconnecting, no waiting on official coverage.

[![CI](https://img.shields.io/badge/CI-passing-brightgreen)](.github/workflows/ci.yml)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)
![MSRV 1.88](https://img.shields.io/badge/MSRV-1.88-orange)

| Crate | What it is |
|---|---|
| [`ghl-sdk`](crates/ghl-sdk) | Async, typed Rust client for the GoHighLevel API — OAuth 2.0 + Private Integration Tokens, automatic token refresh, rate-limit-aware retries, pagination as `Stream`s |
| [`ghl-models`](crates/ghl-models) | **2,417 generated DTOs** for every API module, both API versions, feature-gated per module |
| [`ghl-mcp`](crates/ghl-mcp) | [MCP](https://modelcontextprotocol.io) server exposing GoHighLevel to Claude, ChatGPT, Gemini, and any MCP host — built on the official `rmcp` SDK, ships as a single binary |

**Complete API coverage.** Generated from [HighLevel's official OpenAPI specs](https://github.com/GoHighLevel/highlevel-api-docs):

| | Count |
|---|---|
| API operations callable | **1,203** (576 in v2 + 627 in v3) |
| API modules covered | **45** across both versions |
| Typed data models (DTOs) | **2,417** |
| Ergonomic typed SDK services | 5 modules (contacts, opportunities, conversations, calendars, locations) |
| MCP tools | 21 (16 typed + 3 meta-tools + 2 utility) |

Hand-written services give you the nicest ergonomics for the busiest modules; the meta-tools and generated DTOs cover everything else — invoices, payments, ad publishing, social planner, voice AI, SaaS, custom objects, workflows, and the rest — so no endpoint is ever out of reach.

## Why this exists

GoHighLevel powers **60k+ agencies and ~2M businesses**, but its developer stack has gaps:

- **No official Rust SDK** (official support: TypeScript, plus low-adoption Python/PHP).
- The **official MCP server is locked to a single sub-account** — no agency-wide access, ~36 tools, $297+/mo plans only.
- **No developer support policy**: HighLevel support explicitly does not help with API integration code.

`ghl-rs` fills those gaps:

|  | Official MCP server | Community Node servers | **ghl-mcp** |
|---|---|---|---|
| API coverage | ~36 curated tools | varies, often stale | ✅ 21 tools + all **1,203** operations via meta-tools |
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
ghl-sdk = "0.3"
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
ghl-sdk = { version = "0.3", features = ["models"] }
ghl-models = { version = "0.3", features = ["invoices"] }
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
cargo install ghl-mcp
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

## Configuration

Everything is configurable **by environment variable or explicitly as a parameter** — env vars are the zero-code path, builder/CLI parameters always win when both are set.

| Env var | CLI flag (`ghl-mcp`) | SDK builder | Purpose |
|---|---|---|---|
| `GHL_PIT_TOKEN` | `--pit-token` | `.private_integration_token(…)` | Private Integration Token (simplest auth) |
| `GHL_ACCESS_TOKEN` | `--access-token` | `.access_token(…)` | OAuth access token (bring your own flow) |
| `GHL_LOCATION_ID` | `--location-id` | — | Default sub-account for MCP tools |
| `GHL_BASE_URL` | `--base-url` | `.base_url(…)` | API base (default `https://services.leadconnectorhq.com`) |
| `GHL_ALLOW_DESTRUCTIVE` | `--allow-destructive` | — | Enable delete/cancel tools (off by default) |
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
- [ ] Typed service methods for invoices, payments, products, workflows
- [ ] Streamable HTTP transport (hosted multi-tenant gateway)
- [ ] Webhook signature validation + typed events
- [ ] `npx ghl-mcp` wrapper, Homebrew tap, Docker image

## License

MIT or Apache-2.0, at your option.

> **Not affiliated with HighLevel Inc.** "GoHighLevel" and "HighLevel" are trademarks of their respective owners. This is an independent, unofficial open-source project.
