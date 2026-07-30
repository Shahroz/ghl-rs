//! # ghl-sdk
//!
//! Unofficial async Rust SDK for the [GoHighLevel](https://www.gohighlevel.com)
//! (HighLevel) API 2.0.
//!
//! - **Auth**: Private Integration Tokens, raw OAuth access tokens, or full
//!   OAuth 2.0 with automatic refresh ([`auth`]).
//! - **Resilience**: rate-limit-aware retries with exponential backoff and
//!   `Retry-After` support; idempotent-only retry on 5xx/transport errors.
//! - **Pagination**: cursor handling exposed as [`futures_util::Stream`]s.
//! - **Config**: everything works via explicit builder parameters *or*
//!   environment variables ([`Ghl::from_env`]).
//! - **Secret hygiene**: tokens live in [`secrecy`] types and are redacted
//!   from all `Debug` output.
//!
//! ```no_run
//! use ghl_sdk::{Ghl, contacts::CreateContact};
//!
//! # async fn demo() -> Result<(), ghl_sdk::Error> {
//! let ghl = Ghl::from_env()?; // GHL_PIT_TOKEN or GHL_ACCESS_TOKEN
//! let contact = ghl.contacts().create(CreateContact {
//!     location_id: "LOCATION_ID".into(),
//!     email: Some("ada@example.com".into()),
//!     ..Default::default()
//! }).await?;
//! println!("created {}", contact.id);
//! # Ok(()) }
//! ```
//!
//! *Not affiliated with HighLevel Inc.*

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
