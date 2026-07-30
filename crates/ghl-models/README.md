# ghl-models

Rust data models (DTOs) for the [GoHighLevel](https://www.gohighlevel.com) (HighLevel) API, generated from [HighLevel's official OpenAPI specifications](https://github.com/GoHighLevel/highlevel-api-docs).

**2,417 structs covering both API versions** — `v2` (2,417 total: 1,074 V2 + 1,329 V3 structs and 14 documented string-enum aliases) across **45 API modules**, from `contacts` and `invoices` to `ad-publishing`, `social-planner`, `voice-ai`, and `saas`.

## Usage

Every module is behind a cargo feature, because enabling everything means thousands of structs. Pick what you need:

```toml
[dependencies]
ghl-models = { version = "0.5", features = ["invoices", "payments"] }
```

```rust,ignore
use ghl_models::v2::invoices::{CreateInvoiceDto, InvoiceItemDto};

let invoice = CreateInvoiceDto {
    alt_id: location_id,
    alt_type: "location".into(),
    name: "August retainer".into(),
    currency: "USD".into(),
    items: vec![/* InvoiceItemDto { .. } */],
    ..Default::default()
};
```

Feature-gating matters: one module compiles in **~1.3s**, all 45 take **~30s**.

Use `features = ["full"]` to get everything (mostly useful for exploring in docs.rs).

## API versions

| | Modules | Operations | Structs |
|---|---|---|---|
| `v2` — current stable (`Version: 2021-07-28`) | 41 | 576 | 1,074 |
| `v3` — newer (`Version: v3`) | 42 | 627 | 1,329 |

V3 renames three modules (`ad-manager`→`ad-publishing`, `social-media-posting`→`social-planner`, `saas-api`→`saas`), adds `chat-widget`, and switches some paths from camelCase to kebab-case (`/campaigns/removeAll` → `/campaigns/remove-all`). Both live side by side: `ghl_models::v2::*` and `ghl_models::v3::*`.

## How these types are generated

- Fields the spec marks `required` are non-`Option`; the rest are `Option<T>` that skip serialization when `None`, so partial updates work naturally.
- Arrays are `Vec<T>` with `#[serde(default)]` — an absent list reads as empty.
- **String enums are `String`**, with allowed values in the doc comment. GoHighLevel adds enum values without notice, and a closed Rust enum would turn that into a deserialization failure.
- Anonymous nested objects and multi-branch compositions (`oneOf`/`anyOf`) become `serde_json::Value`; a single-`$ref` `allOf` wrapper resolves to the referenced type.
- Unknown response fields are ignored, never rejected, so upstream additions can't break your build.

Regenerate after HighLevel updates its specs:

```bash
python3 xtask/generate_models.py /path/to/highlevel-api-docs crates/ghl-models
```

## Full reference

Every model, field, and enum value is documented per module in the [API reference](https://github.com/Shahroz/ghl-rs/blob/main/docs/api/README.md), and field-by-field on [docs.rs](https://docs.rs/ghl-models). Large repeated value lists (country codes, timezones) live in [shared enums](https://github.com/Shahroz/ghl-rs/blob/main/docs/api/shared-enums.md).

## Related crates

- [`ghl-sdk`](https://crates.io/crates/ghl-sdk) — the async API client. Its per-module features give you **generated typed methods for every v2 endpoint**, wired to these DTOs, so you rarely construct requests by hand.
- [`ghl-mcp`](https://crates.io/crates/ghl-mcp) — MCP server exposing GoHighLevel to AI agents.

License: MIT or Apache-2.0. *Not affiliated with HighLevel Inc.*
