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
