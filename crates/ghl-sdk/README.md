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

Typed coverage today: **contacts, opportunities, conversations, calendars, locations**, plus OAuth/token exchange. Any other endpoint in GoHighLevel's 41-module API is reachable right now through `ghl.request_raw()` (arbitrary method/path/query/body, same auth + retry + rate-limit handling) or the convenience `get_raw()` / `post_raw()` helpers.

Looking for the AI-agent side? See [`ghl-mcp`](https://crates.io/crates/ghl-mcp), the MCP server built on this SDK.

License: MIT or Apache-2.0. *Not affiliated with HighLevel Inc.*
