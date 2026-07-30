# ghl-rs usage guide

Everything you need to call GoHighLevel from Rust or from an AI agent. **Every API v2 endpoint has a typed Rust method** — 576 of them — so you shouldn't need HighLevel's docs open alongside.

- **[Full API reference](api/README.md)** — every module, endpoint, struct, and enum
- [docs.rs/ghl-sdk](https://docs.rs/ghl-sdk) · [docs.rs/ghl-models](https://docs.rs/ghl-models)

**Contents**

1. [Which crate do I need?](#1-which-crate-do-i-need)
2. [Authentication](#2-authentication)
3. [Calling the API from Rust](#3-calling-the-api-from-rust)
4. [Coverage](#4-coverage)
5. [Typed module cookbook](#5-typed-module-cookbook)
6. [Reaching any other endpoint](#6-reaching-any-other-endpoint)
7. [Data models, structs & enums](#7-data-models-structs-and-enums)
8. [Pagination](#8-pagination)
9. [Errors](#9-errors)
10. [Rate limits](#10-rate-limits)
11. [Multi-location (agency) usage](#11-multi-location-agency-usage)
12. [API v2 vs v3](#12-api-v2-vs-v3)
13. [Using it from an AI agent (MCP)](#13-using-it-from-an-ai-agent-mcp)
14. [Webhooks](#14-webhooks)
15. [Running the MCP server over HTTP](#15-running-the-mcp-server-over-http)
16. [Troubleshooting](#16-troubleshooting)

---

## 1. Which crate do I need?

| You want to… | Use | Install |
|---|---|---|
| Call the API from Rust code | `ghl-sdk` | `cargo add ghl-sdk --features invoices,contacts` |
| Typed request/response structs for any module | `ghl-models` | `cargo add ghl-models --features invoices,payments` |
| Let Claude/ChatGPT/Gemini use your CRM | `ghl-mcp` | `cargo install ghl-mcp`, `npx ghl-mcp`, `brew install`, or Docker |

`ghl-sdk` re-exports the models when you enable its `models` feature, so you can depend on just the SDK if you prefer:

```toml
# Generated services + their DTOs, per module:
ghl-sdk = { version = "0.5", features = ["invoices", "contacts"] }

# Or just the DTOs, no services:
ghl-sdk = { version = "0.5", features = ["models"] }
ghl-models = { version = "0.5", features = ["invoices"] }
```

Everything is feature-gated per module for a real reason: one module compiles in about a second, all 41 take closer to a minute.

---

## 2. Authentication

GoHighLevel offers three credential styles. This is the single most common source of confusion, so here's the decision tree:

| Your situation | Use | Notes |
|---|---|---|
| Internal tool, one sub-account | **Private Integration Token (PIT)** | Simplest. Create in the sub-account: *Settings → Private Integrations*. Looks like `pit-…` |
| You already ran OAuth yourself | **Access token** | Passed through as-is, never refreshed |
| Marketplace app, many customers | **OAuth 2.0** | Automatic refresh; you supply a `TokenStore` |
| Agency-wide, many sub-accounts | **OAuth with a Company token** | Then exchange per location — see [§11](#11-multi-location-agency-usage) |

### Private Integration Token

```rust,ignore
// From the environment: GHL_PIT_TOKEN (or GHL_ACCESS_TOKEN), optional GHL_BASE_URL
let ghl = Ghl::from_env()?;

// Or explicitly — parameters always win over env vars
let ghl = Ghl::builder()
    .private_integration_token("pit-…")
    .build()?;
```

Give the integration the scopes your endpoints need. Every endpoint's required scopes are listed in the [API reference](api/README.md) — e.g. `POST /invoices/` needs `invoices.write`.

### OAuth 2.0 with automatic refresh

```rust,ignore
use ghl_sdk::{Auth, Ghl, MemoryTokenStore, OAuthConfig, TokenSet, UserType};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

// You perform the authorization-code exchange once, then hand the SDK the pair.
let tokens = TokenSet::new(
    access_token,
    refresh_token,
    SystemTime::now() + Duration::from_secs(86_399),
);

let ghl = Ghl::builder()
    .auth(Auth::oauth(
        OAuthConfig::new(client_id, client_secret, UserType::Location),
        Arc::new(MemoryTokenStore::new(tokens)),
    ))
    .build()?;
```

The SDK refreshes automatically ~60s before expiry, with single-flight locking so concurrent requests trigger only one refresh.

> **Important:** GoHighLevel rotates the refresh token on every use. `MemoryTokenStore` loses tokens on restart — implement `TokenStore` against Redis/Postgres/a file for anything long-lived:

```rust,ignore
#[async_trait::async_trait]
impl TokenStore for MyStore {
    async fn load(&self) -> ghl_sdk::Result<Option<TokenSet>> { /* … */ }
    async fn save(&self, tokens: TokenSet) -> ghl_sdk::Result<()> {
        // MUST be durable before returning, or you lose the session.
    }
}
```

Tokens are held in [`secrecy`](https://docs.rs/secrecy) types and redacted from all `Debug` output, so they can't leak into logs.

---

## 3. Calling the API from Rust

```rust,no_run
use ghl_sdk::{contacts::CreateContact, Ghl};

#[tokio::main]
async fn main() -> Result<(), ghl_sdk::Error> {
    let ghl = Ghl::from_env()?;

    let contact = ghl.contacts().create(CreateContact {
        location_id: "LOCATION_ID".into(),
        email: Some("ada@example.com".into()),
        first_name: Some("Ada".into()),
        tags: vec!["hot-lead".into()],
        ..Default::default()
    }).await?;

    println!("created {}", contact.id);
    Ok(())
}
```

`Ghl` is cheap to clone (`Arc` inside) — build one and share it across tasks.

Builder options:

| Method | Default | Purpose |
|---|---|---|
| `.base_url(url)` | `https://services.leadconnectorhq.com` | Proxy or test server |
| `.timeout(dur)` | 30s | Per-request timeout |
| `.max_retries(n)` | 3 | Retry attempts after the first try |

---

## 4. Coverage

**Every endpoint has a typed Rust method** — 1,203 operations across 45 modules in both API versions, generated from HighLevel's specs with typed parameters and responses. You should never need to open HighLevel's API docs to make a call.

| What | Covers | You get |
|---|---|---|
| **Generated services** | **all 45 modules, 1,203 methods** (v2 + v3) | Typed params + response structs, one method per endpoint |
| **Hand-written helpers** | 5 modules, 21 methods | Envelope unwrapping, paginated `Stream`s — on the same services |
| **`request_raw`** | any path you like | Escape hatch; same auth, retry, rate limiting |
| **MCP meta-tools** | all 1,203 operations | For AI agents |

Enable a module by cargo feature:

```toml
ghl-sdk = { version = "0.5", features = ["invoices", "payments"] }
```

```rust,ignore
use ghl_sdk::services::invoices::ListInvoicesParams;

let params = ListInvoicesParams::new(&loc, "location", "20", "0").status("draft");
let page = ghl.invoices().list_invoices(&params).await?;
println!("{:?} invoices", page.total);
```

**API v3** lives behind `ghl.v3()` and sends `Version: v3` automatically:

```rust,ignore
let dup = ghl.v3().contacts().get_duplicate_contact(&params).await?;
```

Features matter for compile time: one module is a second or two, all 45 (`features = ["full"]`) is closer to a minute.

### The generated method shape

Every generated method looks the same:

```text
async fn <name>(&self, <path params…>, params: &XParams, body: &Dto) -> Result<Response>
```

- **Path parameters** — positional `&str` args in URL order.
- **Query parameters** — one `XParams` struct: required fields are `new()` arguments, optional ones are chainable setters. Omitted entirely when an endpoint has no query params.
- **Body** — the generated DTO from `ghl-models`.
- **Returns** — the response type the spec names (about 3 in 4 endpoints), else `serde_json::Value`.

Find the exact method for any endpoint in the [API reference](api/README.md) — every endpoint lists its Rust method name and a ready-to-paste call.

### Strict on send, lenient on receive

Request types keep the spec's required fields non-`Option`, so forgetting a mandatory field is a compile error. Response types make **everything** optional: GoHighLevel sometimes omits fields its own spec marks required, and a strict response type would turn that into an unrecoverable deserialization error. This follows Postel's law and is why `page.total` is `Option<f64>` rather than `f64`.

---

## 5. Typed module cookbook

The five modules below have hand-written helpers in addition to their generated methods. For every *other* module, the pattern is the generated one from §4 — look up the method name in the [API reference](api/README.md).

### Contacts

```rust,ignore
use ghl_sdk::contacts::{CreateContact, UpdateContact};

// Create — needs at least an email or phone
let c = ghl.contacts().create(CreateContact {
    location_id: loc.clone(),
    email: Some("ada@example.com".into()),
    phone: Some("+15551234567".into()),
    tags: vec!["newsletter".into()],
    ..Default::default()
}).await?;

let fetched = ghl.contacts().get(&c.id).await?;

// Update — only the fields you set are sent
ghl.contacts().update(&c.id, UpdateContact {
    last_name: Some("Lovelace".into()),
    ..Default::default()
}).await?;

ghl.contacts().delete(&c.id).await?;
```

### Opportunities (pipeline deals)

```rust,ignore
use ghl_sdk::opportunities::{CreateOpportunity, UpdateOpportunity};

// Stage ids live on the pipeline — fetch it first
let pipelines = ghl.opportunities().pipelines(&loc).await?;
let pipeline = &pipelines[0];
let first_stage = &pipeline.stages[0];

let deal = ghl.opportunities().create(CreateOpportunity {
    location_id: loc.clone(),
    pipeline_id: pipeline.id.clone(),
    name: "Acme — annual plan".into(),
    pipeline_stage_id: Some(first_stage.id.clone()),
    monetary_value: Some(12_000.0),
    contact_id: Some(contact_id.clone()),
    ..Default::default()
}).await?;

// Move a stage and/or set status
ghl.opportunities().update(&deal.id, UpdateOpportunity {
    pipeline_stage_id: Some(pipeline.stages[1].id.clone()),
    ..Default::default()
}).await?;

// Status only: open | won | lost | abandoned
ghl.opportunities().update_status(&deal.id, "won").await?;

// Search with filters
let page = ghl.opportunities().search(&loc)
    .status("open")
    .pipeline_id(&pipeline.id)
    .limit(50)
    .page()
    .await?;
```

### Conversations

```rust,ignore
use ghl_sdk::conversations::SendMessage;

let threads = ghl.conversations().search(&loc, Some("ada"), 20).await?;
let msgs = ghl.conversations().messages(&threads.conversations[0].id, 50).await?;

// Channels: SMS | Email | WhatsApp | IG | FB | Custom | Live_Chat
ghl.conversations().send_message(SendMessage {
    message_type: "SMS".into(),
    contact_id: contact_id.clone(),
    message: Some("Thanks for reaching out!".into()),
    ..Default::default()
}).await?;
```

### Calendars

```rust,ignore
use ghl_sdk::calendars::CreateAppointment;

let calendars = ghl.calendars().list(&loc).await?;

// Date range is epoch MILLISECONDS
let slots = ghl.calendars()
    .free_slots(&calendars[0].id, start_ms, end_ms, Some("America/New_York"))
    .await?;
println!("{} slots across {} days", slots.all().len(), slots.by_date.len());

// start_time is ISO-8601 WITH offset
let appt = ghl.calendars().create_appointment(CreateAppointment {
    calendar_id: calendars[0].id.clone(),
    location_id: loc.clone(),
    contact_id: contact_id.clone(),
    start_time: "2026-08-01T14:00:00+05:00".into(),
    title: Some("Discovery call".into()),
    to_notify: Some(true),
    ..Default::default()
}).await?;
```

### Locations

```rust,ignore
let location = ghl.locations().get(&loc).await?;

// Agency-wide listing needs a Company token; a location PIT gets 403 here
let all = ghl.locations().search(None, 50).await?;
```

---

## 6. Reaching any other endpoint

API v3 endpoints and anything else without a generated method are callable with `request_raw`, which goes through the same auth, retry, and rate-limit handling as the typed services:

```rust,ignore
// request_raw(method, path, query, body, version_header_override)
let invoices = ghl.request_raw(
    "GET",
    "/invoices/",
    &[
        ("altId".into(), loc.clone()),
        ("altType".into(), "location".into()),
        ("limit".into(), "20".into()),
    ],
    None,
    None,                       // None = default Version: 2021-07-28
).await?;
```

Combine it with a generated DTO for a typed request body:

```rust,ignore
use ghl_models::v2::invoices::CreateInvoiceDto;

let body = serde_json::to_value(CreateInvoiceDto {
    alt_id: loc.clone(),
    alt_type: "location".into(),
    name: "August retainer".into(),
    currency: "USD".into(),
    ..Default::default()
})?;

let created = ghl.request_raw("POST", "/invoices/", &[], Some(&body), None).await?;
```

Convenience wrappers exist for the common cases: `ghl.get_raw(path, query)` and `ghl.post_raw(path, &body)`.

To call a **v3** endpoint, pass the version override:

```rust,ignore
let dup = ghl.request_raw(
    "GET", "/contacts/search/duplicate",
    &[("locationId".into(), loc.clone()), ("email".into(), email)],
    None,
    Some("v3"),
).await?;
```

Look up the exact path, params, and required scopes for any endpoint in the **[API reference](api/README.md)**.

---

## 7. Data models, structs and enums

`ghl-models` ships **2,417 structs** — 1,074 for v2, 1,329 for v3 — across 45 modules.

```toml
ghl-models = { version = "0.5", features = ["invoices", "payments", "products"] }
```

```rust,ignore
use ghl_models::v2::invoices::{CreateInvoiceDto, InvoiceItemDto, DiscountDto};
use ghl_models::v3::social_planner::*;   // v3 lives alongside v2
```

Module path pattern: `ghl_models::{v2|v3}::{module_with_underscores}::TypeName`. So `social-planner` becomes `ghl_models::v3::social_planner`.

### The conventions, and why

| Convention | Reason |
|---|---|
| `required` fields are non-`Option` | Matches the spec; you can't forget a mandatory field |
| Optional fields are `Option<T>` + `skip_serializing_if` | Partial updates serialize to only what you set |
| Arrays are `Vec<T>` with `#[serde(default)]` | An absent list reads as empty rather than erroring |
| **String enums are `String`, not Rust enums** | GoHighLevel adds enum values without notice; a closed enum would turn that into a deserialization failure. Allowed values are in each field's doc comment |
| Nested anonymous objects are `serde_json::Value` | The spec doesn't name them |
| Unknown response fields are ignored | Upstream additions can never break your build |

### Finding enum values

Because enums are `String`, you need to know the legal values. Three places to look:

1. The field's doc comment on [docs.rs](https://docs.rs/ghl-models) — *"Allowed values: `location`."*
2. The module page in the [API reference](api/README.md) — the model tables spell out every value
3. [Shared enums](api/shared-enums.md) — big repeated lists (country codes, timezones) hoisted out

Common ones worth memorizing:

| Field | Values |
|---|---|
| `altType` (invoices, payments, products…) | `location` — the only legal value |
| Opportunity `status` | `open`, `won`, `lost`, `abandoned` |
| Message `type` | `SMS`, `Email`, `WhatsApp`, `IG`, `FB`, `Custom`, `Live_Chat` |
| Appointment `appointmentStatus` | `confirmed`, `cancelled`, `showed`, `noshow`, `invalid` |
| Custom field `model` | `contact`, `opportunity` |

---

## 8. Pagination

GoHighLevel's pagination is **not consistent across modules** — some endpoints use cursors, others `skip`/`offset`, others `page`. The typed services absorb that for you.

Cursor-based (contacts, opportunities) as an auto-paginating `Stream`:

```rust,ignore
use futures_util::TryStreamExt;

let mut stream = ghl.contacts().list(&loc).limit(100).stream();
while let Some(contact) = stream.try_next().await? {
    println!("{}", contact.id);
}
```

One page at a time, if you want control over the cursor:

```rust,ignore
let page = ghl.contacts().list(&loc).limit(100).page().await?;
let next = page.meta.as_ref().and_then(|m| m.start_after_id.clone());
// resume later:
let page2 = ghl.contacts().list(&loc).limit(100).start_after_id(cursor).page().await?;
```

For untyped endpoints, check the endpoint's query params in the [reference](api/README.md) — you'll see whether it takes `startAfterId`, `offset`, or `page`.

---

## 9. Errors

```rust,ignore
use ghl_sdk::Error;

match ghl.contacts().get(id).await {
    Ok(c) => { /* … */ }
    Err(Error::Api { status, message, request_id }) => {
        // API said no. 404 not found, 422 validation, 403 scope missing…
        eprintln!("{status}: {message} (request {request_id:?})");
    }
    Err(Error::RateLimited { retry_after }) => {
        // Retries were already exhausted
        eprintln!("slow down; retry after {retry_after:?}");
    }
    Err(Error::Auth(msg)) => eprintln!("credentials/refresh problem: {msg}"),
    Err(Error::Transport(e)) => eprintln!("network: {e}"),
    Err(Error::Decode { endpoint, source }) => eprintln!("bad body from {endpoint}: {source}"),
    Err(Error::Config(msg)) => eprintln!("client misconfigured: {msg}"),
}
```

GoHighLevel returns errors as `{"statusCode":…, "message":…}` where `message` may be a **string or an array of strings**; the SDK normalizes both into `Error::Api::message`.

What each status usually means:

| Status | Typical cause |
|---|---|
| 401 | Token invalid or expired |
| 403 | Missing scope, **or** an agency-only endpoint called with a location token |
| 404 | Wrong id — or a v3 path called without `Version: v3` |
| 422 | Validation: a required query param or body field is missing/invalid |
| 429 | Rate limited |

---

## 10. Rate limits

Documented limits are 100 requests/10s burst and 200,000/day, **but your actual budget depends on the credential.** Measured on a real Private Integration Token: ~20/10s burst and 10,000/day. Don't hardcode assumptions — read the live headers:

```rust,ignore
let status = ghl.rate_status();
println!("{:?} burst, {:?} today", status.burst_remaining, status.daily_remaining);
```

The SDK already handles the mechanics: 429s are retried honoring `Retry-After`, and 5xx/transport failures are retried with exponential backoff + jitter — **for idempotent methods only** (GET/PUT/DELETE/HEAD), so a POST is never silently duplicated.

---

## 11. Multi-location (agency) usage

With an agency (Company) OAuth token you can mint a per-location client:

```rust,ignore
// agency-scoped client
let agency = Ghl::builder().auth(Auth::oauth(company_config, store)).build()?;

// exchange for a location-scoped client (POST /oauth/locationToken)
let loc_client = agency.as_location(&company_id, &location_id).await?;
let contacts = loc_client.contacts().list(&location_id).limit(100).page().await?;
```

Each returned client is independent, so you can fan out across sub-accounts concurrently — just remember the rate budget is per credential per location.

> A **location** PIT cannot list agency sub-accounts: `GET /locations/search` returns 403 no matter which scopes you grant. That's an API restriction, not a scope problem. `ghl_list_locations` in the MCP server detects this and falls back to the configured location.

---

## 12. API v2 vs v3

Both are live, on the same host, distinguished by the `Version` header.

| | v2 | v3 |
|---|---|---|
| `Version` header | `2021-07-28` (some `2021-04-15`) | `v3` |
| Modules | 41 | 42 |
| Operations | 576 | 627 |
| Models | 1,074 | 1,329 |
| Rust service | `ghl.invoices()` | `ghl.v3().invoices()` |
| Rust DTOs | `ghl_models::v2::*` | `ghl_models::v3::*` |
| MCP operation id | `invoices.get_invoices` | `v3:invoices.get_invoices` |

v3 renames three modules and adds one:

| v2 | v3 |
|---|---|
| `ad-manager` | `ad-publishing` |
| `social-media-posting` | `social-planner` |
| `saas-api` | `saas` |
| — | `chat-widget` (new) |

Some paths also change from camelCase to kebab-case, e.g. `/contacts/{id}/campaigns/removeAll` → `/contacts/{id}/campaigns/remove-all`.

**Two gotchas found in the real specs:**

1. The header value is **not uniform per version** — v3's `ad-publishing` module declares `Version: 2021-07-28`, not `v3`. Always use the value the reference lists for that specific endpoint.
2. About 29 operations declare no `Version` at all; the SDK sends its default (`2021-07-28`).

v2 remains the stable default. Use v3 for the modules that only exist there.

---

## 13. Using it from an AI agent (MCP)

```bash
cargo install ghl-mcp
```

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

21 tools: 16 typed ones for contacts/opportunities/conversations/calendars, 2 utility, and 3 meta-tools that reach everything else. Full list in the [ghl-mcp README](../crates/ghl-mcp/README.md).

### The meta-tool workflow

An agent that needs an uncovered endpoint follows three steps:

```json
{"name": "ghl_search_operations", "arguments": {"query": "create invoice"}}
{"name": "ghl_describe_operation", "arguments": {"operation_id": "invoices.post_invoices"}}
{"name": "ghl_execute_operation", "arguments": {"operation_id": "invoices.post_invoices", "body": {"…": "…"}}}
```

Call `ghl_search_operations` with an empty query to get the full module map, or pass `api_version: "v3"` to filter.

### Safety model

Read-only GETs always work. Everything that writes — including through `ghl_execute_operation` — requires the server to be started with `--allow-destructive` (or `GHL_ALLOW_DESTRUCTIVE=true`). That covers deletes, sending messages, and booking appointments, so an agent can't message a real customer unless you opted in.

Convenience: required `locationId`/`altId` params default to your configured location, and `altType` is filled in as `location` automatically.

---

## 14. Webhooks

```toml
ghl-sdk = { version = "0.5", features = ["webhooks"] }
```

GoHighLevel signs every webhook with **RSA-SHA256** (PKCS#1 v1.5) over the raw body and puts the base64 signature in the `x-wh-signature` header. Verify the **raw bytes** — re-serializing parsed JSON can reorder keys and invalidate the signature.

```rust,ignore
use ghl_sdk::webhooks::{self, WebhookEvent};

webhooks::verify(raw_body, signature_header)?;      // uses HighLevel's published key
let event: WebhookEvent = serde_json::from_slice(raw_body)?;

// HighLevel's recommended replay guard: bound the age, and reject repeat ids.
if event.is_stale(std::time::Duration::from_secs(300)) { return; }
if already_seen(event.webhook_id.as_deref()) { return; }

match event.event_type() {
    "ContactCreate" => { /* … */ }
    "InvoicePaid"   => { /* … */ }
    _ => {}
}
```

`WebhookEvent` types the envelope HighLevel's 58 event types share (`type`, `timestamp`, `webhookId`, `locationId`, `companyId`) and keeps everything else in `event.data`, so a new upstream event never breaks your handler. Use `event.parse_as::<T>()` to re-deserialize into a concrete DTO once you know the type.

HighLevel rotates the signing key occasionally and announces it by email and in the developer Slack. If verification starts failing across the board, check for a rotation notice and pass the new key to `verify_with_key` until this crate ships an update.

---

## 15. Running the MCP server over HTTP

stdio is the default and what most MCP hosts launch. To share one server between several agents:

```sh
ghl-mcp --http 127.0.0.1:8000 --http-auth-token "$(openssl rand -hex 32)"
# MCP endpoint: http://127.0.0.1:8000/mcp
```

Callers send `Authorization: Bearer <token>`; anything else gets `401` with a `WWW-Authenticate: Bearer` header. The comparison is constant-time, so the token can't be recovered from response timings.

Or in a container:

```sh
docker run -p 8000:8000 -e GHL_PIT_TOKEN=pit-… -e GHL_LOCATION_ID=… ghcr.io/shahroz/ghl-mcp
```

> **Omit `--http-auth-token` and the endpoint is unauthenticated** — every caller gets whatever GoHighLevel credentials the server was started with. The server logs a warning at startup in that case. Set a token whenever the port is reachable by anything other than localhost.

The transport is stateless (MCP `2026-07-28`), so it scales horizontally behind a load balancer with no shared session store.

---

## 16. Troubleshooting

| Symptom | Cause & fix |
|---|---|
| `422 altType should not be empty` | Invoices/payments/products need **both** `altId` and `altType=location` |
| `403 Forbidden` on `/locations/search` | Agency-only endpoint; a location PIT can't call it regardless of scopes |
| `403` elsewhere | Missing scope — check the endpoint in the [reference](api/README.md) and add it to your Private Integration |
| `404` on a path you copied from v3 docs | Pass `Some("v3")` as the version argument to `request_raw` |
| Agent says "destructive tools are disabled" | Intended. Restart with `--allow-destructive` |
| `no credentials configured` | Set `GHL_PIT_TOKEN`, or pass `.private_integration_token(…)` |
| OAuth works then fails after restart | `MemoryTokenStore` isn't durable and GHL rotates refresh tokens — implement `TokenStore` |
| Webhook verification always fails | Verify the **raw** body bytes, not re-serialized JSON; check for a key-rotation notice |
| MCP server shows no output | Logs go to **stderr** (stdout is the protocol channel). Set `RUST_LOG=debug` |
| Deserialization error on a response | Please [open an issue](https://github.com/Shahroz/ghl-rs/issues) with the endpoint — the typed structs tolerate unknown fields, so this means a type mismatch |

---

*Not affiliated with HighLevel Inc. "GoHighLevel" and "HighLevel" are trademarks of their respective owners.*
