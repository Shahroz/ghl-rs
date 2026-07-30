//! Unofficial async Rust SDK for the [GoHighLevel](https://www.gohighlevel.com)
//! (HighLevel) CRM API — covering **1,203 operations across 45 API modules** in
//! both API v2 and v3.
//!
//! ```no_run
//! use ghl_sdk::{contacts::CreateContact, Ghl};
//!
//! # async fn demo() -> Result<(), ghl_sdk::Error> {
//! // Reads GHL_PIT_TOKEN (or GHL_ACCESS_TOKEN) from the environment.
//! let ghl = Ghl::from_env()?;
//!
//! let contact = ghl.contacts().create(CreateContact {
//!     location_id: "LOCATION_ID".into(),
//!     email: Some("ada@example.com".into()),
//!     first_name: Some("Ada".into()),
//!     ..Default::default()
//! }).await?;
//!
//! println!("created {}", contact.id);
//! # Ok(()) }
//! ```
//!
//! # What you get
//!
//! - **Auth** — Private Integration Tokens, raw OAuth access tokens, or full
//!   OAuth 2.0 with automatic single-flight refresh and a pluggable
//!   [`TokenStore`]. Agency→location token exchange via [`Ghl::as_location`].
//!   See the [`auth`] module.
//! - **Resilience** — 429s retried honoring `Retry-After`; 5xx and transport
//!   failures retried with exponential backoff + jitter, **idempotent methods
//!   only**, so a `POST` is never silently duplicated.
//! - **Rate-limit awareness** — live headroom from response headers via
//!   [`Ghl::rate_status`].
//! - **Pagination** — GoHighLevel's cursor scheme handled for you and exposed as
//!   [`futures_util::Stream`]s.
//! - **Config by env var or parameter** — [`Ghl::from_env`] or
//!   [`Ghl::builder`]; explicit parameters always win.
//! - **Secret hygiene** — tokens live in [`secrecy`] types and are redacted from
//!   all `Debug` output.
//! - **Forward-compatible types** — unknown response fields are preserved in an
//!   `extra` map instead of failing deserialization.
//!
//! # Three coverage tiers
//!
//! The guarantees differ by tier — know which one you're using:
//!
//! | Tier | Covers | What you get |
//! |---|---|---|
//! | **1.** Typed services (below) | 5 modules, 21 methods | Real Rust types, compile-time field checks, parsed responses, paginated `Stream`s |
//! | **2.** [`Ghl::request_raw`] + [`ghl-models`](https://docs.rs/ghl-models) DTOs | all 45 modules, 2,417 structs | Typed bodies you serialize; you supply the path |
//! | **3.** [`ghl-mcp`](https://crates.io/crates/ghl-mcp) meta-tools | all 1,203 operations | For AI agents; params validated, body passed through |
//!
//! ## Tier 1 — typed service modules
//!
//! | Module | Service | Methods |
//! |---|---|---|
//! | [`contacts`] | [`Ghl::contacts`] | `create`, `get`, `update`, `delete`, `list` (streaming) |
//! | [`opportunities`] | [`Ghl::opportunities`] | `pipelines`, `create`, `get`, `update`, `update_status`, `delete`, `search` (streaming) |
//! | [`conversations`] | [`Ghl::conversations`] | `search`, `messages`, `send_message` |
//! | [`calendars`] | [`Ghl::calendars`] | `list`, `free_slots`, `create_appointment`, `get_appointment` |
//! | [`locations`] | [`Ghl::locations`] | `get`, `search` |
//!
//! ## Tier 2 — every other endpoint
//!
//! [`Ghl::request_raw`] reaches any endpoint with the same auth, retry, and
//! rate-limit handling. Pair it with a generated DTO for a typed body:
//!
//! ```ignore
//! use ghl_models::v2::invoices::CreateInvoiceDto;
//!
//! let body = serde_json::to_value(CreateInvoiceDto {
//!     alt_id: location_id.clone(),
//!     alt_type: "location".into(),      // the only legal value
//!     name: "August retainer".into(),
//!     currency: "USD".into(),
//!     ..Default::default()
//! })?;
//!
//! let created = ghl.request_raw("POST", "/invoices/", &[], Some(&body), None).await?;
//! ```
//!
//! To call an **API v3** endpoint, pass the version override:
//!
//! ```ignore
//! let dup = ghl.request_raw(
//!     "GET", "/contacts/search/duplicate",
//!     &[("locationId".into(), loc), ("email".into(), email)],
//!     None,
//!     Some("v3"),
//! ).await?;
//! ```
//!
//! # Authentication at a glance
//!
//! | Situation | Use |
//! |---|---|
//! | Internal tool, one sub-account | [`Auth::private_integration`] (a `pit-…` token) |
//! | You ran OAuth yourself | [`Auth::access_token`] (used as-is, never refreshed) |
//! | Marketplace app | [`Auth::oauth`] with a [`TokenStore`] — auto-refresh |
//! | Agency, many sub-accounts | [`Auth::oauth`] with [`UserType::Company`], then [`Ghl::as_location`] |
//!
//! GoHighLevel rotates the refresh token on every use, so a [`TokenStore`]
//! implementation must persist durably — [`MemoryTokenStore`] loses the session
//! on restart.
//!
//! # Errors
//!
//! [`Error`] distinguishes [`Error::Api`] (the API said no, with status and
//! message), [`Error::RateLimited`] (retries exhausted), [`Error::Auth`],
//! [`Error::Transport`], [`Error::Decode`], and [`Error::Config`]. GoHighLevel
//! returns `message` as either a string or an array of strings; both normalize
//! into [`Error::Api`]'s `message`.
//!
//! # Cargo features
//!
//! | Feature | Default | Effect |
//! |---|---|---|
//! | `models` | no | Re-exports [`ghl-models`](https://docs.rs/ghl-models) as [`models`], giving typed DTOs for every API module |
//!
//! # Further reading
//!
//! - [Usage guide](https://github.com/Shahroz/ghl-rs/blob/main/docs/GUIDE.md) —
//!   auth decision tree, per-module cookbook, pagination, rate limits,
//!   multi-location, v2 vs v3, troubleshooting
//! - [Full API reference](https://github.com/Shahroz/ghl-rs/blob/main/docs/api/README.md)
//!   — all 45 modules: every endpoint, struct, and enum value
//! - [`ghl-mcp`](https://crates.io/crates/ghl-mcp) — MCP server built on this
//!   SDK, exposing GoHighLevel to AI agents
//!
//! *Not affiliated with HighLevel Inc. "GoHighLevel" and "HighLevel" are
//! trademarks of their respective owners.*

#![warn(missing_docs)]
#![warn(clippy::all)]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Generated data models (DTOs) for the whole GoHighLevel API, re-exported from
/// the [`ghl-models`](https://docs.rs/ghl-models) crate.
///
/// Enable the `models` feature, then pick the API modules you need through
/// `ghl-models`' own per-module features:
///
/// ```toml
/// ghl-sdk = { version = "0.3", features = ["models"] }
/// ghl-models = { version = "0.3", features = ["invoices", "payments"] }
/// ```
///
/// Types live under `models::v2::*` and `models::v3::*`.
#[cfg(feature = "models")]
#[cfg_attr(docsrs, doc(cfg(feature = "models")))]
pub use ghl_models as models;

pub mod auth;
pub mod calendars;
mod client;
pub mod contacts;
pub mod conversations;
mod error;
pub mod locations;
pub mod opportunities;

pub use auth::{Auth, MemoryTokenStore, OAuthConfig, TokenSet, TokenStore, UserType};
pub use client::{Ghl, GhlBuilder, RateStatus, API_VERSION, DEFAULT_BASE_URL};
pub use error::{Error, Result};
