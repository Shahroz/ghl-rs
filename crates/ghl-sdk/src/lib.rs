//! Unofficial async Rust SDK for the [GoHighLevel](https://www.gohighlevel.com)
//! (HighLevel) CRM API.
//!
//! **Every endpoint has a typed Rust method** — 1,203 operations across 45
//! modules, in API v2 *and* v3, each with generated request/response types, so
//! you never have to leave the library to read HighLevel's docs.
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
//! # Generated services — every endpoint, typed
//!
//! Enable the cargo feature named after an API module and its whole surface
//! appears on the client, with generated parameter and body types:
//!
//! ```toml
//! ghl-sdk = { version = "0.5", features = ["invoices"] }
//! ```
//!
//! ```ignore
//! use ghl_sdk::services::invoices::ListInvoicesParams;
//!
//! // Required query params are constructor arguments; optional ones are setters.
//! let params = ListInvoicesParams::new(&location_id, "location", "20", "0")
//!     .status("draft");
//!
//! let page = ghl.invoices().list_invoices(&params).await?;   // typed response
//! println!("{:?} invoices", page.total);
//! ```
//!
//! Every generated method has the same predictable shape:
//!
//! ```text
//! async fn <name>(&self, <path params…>, params: &XParams, body: &Dto) -> Result<Response>
//! ```
//!
//! - **Path parameters** are positional `&str` arguments, in URL order.
//! - **Query parameters** collapse into one `XParams` struct — required fields
//!   are `new()` arguments, optional ones are chainable setters. The argument is
//!   absent entirely when an endpoint takes no query parameters.
//! - **Bodies** take the generated DTO from [`ghl-models`](https://docs.rs/ghl-models).
//! - **Returns** the response type the spec names (about 3 in 4 endpoints), else
//!   [`serde_json::Value`].
//!
//! ## API v3
//!
//! v3 is a parallel, newer surface (627 operations) reached through [`Ghl::v3`],
//! which sends `Version: v3` for you:
//!
//! ```ignore
//! let dup = ghl.v3().contacts().get_duplicate_contact(&params).await?;
//! ```
//!
//! It has modules v2 lacks — `ad-publishing`, `social-planner`, `saas`,
//! `chat-widget` — and renames three others.
//!
//! See [`services`] for the module list, and the
//! [API reference](https://github.com/Shahroz/ghl-rs/blob/main/docs/api/README.md)
//! for the Rust method behind every endpoint.
//!
//! ## Hand-written helpers
//!
//! Five modules also carry curated helpers that go beyond a 1:1 endpoint
//! mapping — they unwrap response envelopes and turn cursor pagination into
//! [`futures_util::Stream`]s. They live on the same services, so both styles are
//! available together:
//!
//! | Module | Helpers |
//! |---|---|
//! | [`contacts`] | `create`, `get`, `update`, `delete`, `list` (streaming) |
//! | [`opportunities`] | `pipelines`, `create`, `get`, `update`, `update_status`, `delete`, `search` (streaming) |
//! | [`conversations`] | `search`, `messages`, `send_message` |
//! | [`calendars`] | `list`, `free_slots`, `create_appointment`, `get_appointment` |
//! | [`locations`] | `get`, `search` |
//!
//! ## Anything not generated
//!
//! [`Ghl::request_raw`] reaches any endpoint — including all of API v3 — with
//! the same auth, retry, and rate-limit handling:
//!
//! ```ignore
//! let dup = ghl.request_raw(
//!     "GET", "/contacts/search/duplicate",
//!     &[("locationId".into(), loc), ("email".into(), email)],
//!     None,
//!     Some("v3"),          // v3 endpoints need their own Version header
//! ).await?;
//! ```
//!
//! # Strict on send, lenient on receive
//!
//! Request types keep the spec's required fields non-`Option`, so a missing
//! mandatory field is a compile error. Response types make everything optional:
//! GoHighLevel sometimes omits fields its own spec marks required, and a strict
//! response type would turn that into an unrecoverable deserialization failure.
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
//! # Webhooks
//!
//! With the `webhooks` feature, `webhooks::verify` checks HighLevel's
//! RSA-SHA256 signature and `webhooks::WebhookEvent` types the envelope its 58
//! event types share. Verify the raw bytes before parsing. See the [`webhooks`]
//! module.
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
//! Nothing is on by default. Each API module is its own feature so you compile
//! only the surface you use — one module is a second or two, all 45 is closer to
//! a minute.
//!
//! | Feature | Effect |
//! |---|---|
//! | `<module>` (45 of them, e.g. `invoices`, `payments`, `products`) | That module's generated services (v2 and v3) plus its DTOs |
//! | `full` | Every generated service. Convenient, slow to compile |
//! | `models` | Just the [`ghl-models`](https://docs.rs/ghl-models) re-export, no services |
//! | `webhooks` | RSA signature verification and typed events ([`webhooks`]) |
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
/// Most callers don't need this directly: enabling a module feature (e.g.
/// `invoices`) already brings in that module's generated service *and* its DTOs.
/// Reach for `models` when you want the types without the services:
///
/// ```toml
/// ghl-sdk = { version = "0.5", features = ["models"] }
/// ghl-models = { version = "0.5", features = ["invoices", "payments"] }
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
pub mod services;

// Module docs live in webhooks.rs; a second doc comment here would shadow the
// intra-doc link resolution inside it.
#[cfg(feature = "webhooks")]
#[cfg_attr(docsrs, doc(cfg(feature = "webhooks")))]
pub mod webhooks;

pub use auth::{Auth, MemoryTokenStore, OAuthConfig, TokenSet, TokenStore, UserType};
pub use client::{Ghl, GhlBuilder, RateStatus, API_VERSION, DEFAULT_BASE_URL};
pub use error::{Error, Result};
