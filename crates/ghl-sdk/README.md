# ghl-sdk

Unofficial async Rust SDK for the [GoHighLevel](https://www.gohighlevel.com) (HighLevel) API 2.0.

- **Auth**: Private Integration Tokens (`pit-…`), raw OAuth access tokens, or full OAuth 2.0 with automatic refresh and pluggable `TokenStore`s. Agency→location token exchange (`Ghl::as_location`) for multi-tenant work.
- **Resilience**: retries with exponential backoff + jitter, `Retry-After` support on 429s, idempotent-only retry on 5xx/transport errors, live rate-limit headroom via `Ghl::rate_status()`.
- **Pagination**: GoHighLevel's cursor scheme handled for you, exposed as `Stream`s.
- **Config**: builder parameters or environment variables (`GHL_PIT_TOKEN`, `GHL_ACCESS_TOKEN`, `GHL_BASE_URL`) — explicit parameters always win.
- **Secret hygiene**: tokens are `secrecy` types, redacted from all `Debug` output.
- **Forward-compatible types**: unknown response fields are preserved in an `extra` map instead of breaking deserialization.

```rust,no_run
use ghl_sdk::{Ghl, contacts::CreateContact};
use futures_util::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), ghl_sdk::Error> {
    let ghl = Ghl::from_env()?;

    let contact = ghl.contacts().create(CreateContact {
        location_id: "LOCATION_ID".into(),
        email: Some("ada@example.com".into()),
        ..Default::default()
    }).await?;

    let mut all = ghl.contacts().list("LOCATION_ID").limit(100).stream();
    while let Some(c) = all.try_next().await? {
        println!("{} {:?}", c.id, c.email);
    }
    Ok(())
}
```

## Every v2 endpoint is a typed method

576 generated methods across all 41 API v2 modules. Enable the feature named after the module:

```toml
ghl-sdk = { version = "0.4", features = ["invoices"] }
```

```rust,ignore
use ghl_sdk::services::invoices::ListInvoicesParams;

// Required query params are constructor args; optional ones are setters.
let params = ListInvoicesParams::new(&loc, "location", "20", "0").status("draft");
let page = ghl.invoices().list_invoices(&params).await?;   // typed response
```

One module compiles in a second or two; `features = ["full"]` (all 41) takes closer to a minute.

API v3 (627 more operations) is reachable via `ghl.request_raw(…, Some("v3"))` with [`ghl-models`](https://crates.io/crates/ghl-models) v3 DTOs.

**Strict on send, lenient on receive:** request types keep the spec's required fields non-`Option`; response types make everything optional, because GoHighLevel sometimes omits fields its own spec marks required and a strict type would make that unrecoverable.

**Docs:** [usage guide](https://github.com/Shahroz/ghl-rs/blob/main/docs/GUIDE.md) · [full API reference](https://github.com/Shahroz/ghl-rs/blob/main/docs/api/README.md) (all 45 modules, every endpoint/struct/enum).

Looking for the AI-agent side? See [`ghl-mcp`](https://crates.io/crates/ghl-mcp), the MCP server built on this SDK.

License: MIT or Apache-2.0. *Not affiliated with HighLevel Inc.*
