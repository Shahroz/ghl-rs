//! Locations (sub-accounts) — fetch one, or list those a credential can see.
//!
//! Access via [`Ghl::locations`](crate::Ghl::locations). See the
//! [full locations reference][ref] for all 29 v2 endpoints.
//!
//! | Method | Endpoint | Scope |
//! |---|---|---|
//! | [`LocationsService::get`] | `GET /locations/{id}` | `locations.readonly` |
//! | [`LocationsService::search`] | `GET /locations/search` | `locations.readonly` |
//!
//! # An API restriction worth knowing
//!
//! [`LocationsService::search`] is **agency-only**. Called with a location-scoped
//! Private Integration Token it returns `403 Forbidden` no matter which scopes
//! you grant — this is an API restriction, not a permissions mistake. Use an
//! agency (Company) token to enumerate sub-accounts, or
//! [`LocationsService::get`] when you already know the id.
//!
//! # Examples
//!
//! ```no_run
//! # use ghl_sdk::Ghl;
//! # async fn demo(ghl: Ghl, loc: &str) -> Result<(), ghl_sdk::Error> {
//! let location = ghl.locations().get(loc).await?;
//! println!("{:?} in {:?}", location.name, location.timezone);
//! # Ok(()) }
//! ```
//!
//! Fanning out across an agency's sub-accounts, minting a per-location client
//! with [`Ghl::as_location`](crate::Ghl::as_location):
//!
//! ```no_run
//! # use ghl_sdk::Ghl;
//! # async fn demo(agency: Ghl, company_id: &str) -> Result<(), ghl_sdk::Error> {
//! for location in agency.locations().search(Some(company_id), 100).await? {
//!     let client = agency.as_location(company_id, &location.id).await?;
//!     let page = client.contacts().list(&location.id).limit(100).page().await?;
//!     println!("{:?}: {} contacts", location.name, page.contacts.len());
//! }
//! # Ok(()) }
//! ```
//!
//! [ref]: https://github.com/Shahroz/ghl-rs/blob/main/docs/api/locations.md

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::client::Ghl;
use crate::error::Result;

/// A GoHighLevel location (sub-account).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)] // fields mirror the API wire format 1:1
pub struct Location {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Any fields this SDK doesn't model yet.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Deserialize)]
struct LocationEnvelope {
    location: Location,
}

#[derive(Deserialize)]
struct LocationList {
    #[serde(default)]
    locations: Vec<Location>,
}

/// Access to the Locations API. Obtained via [`Ghl::locations`].
pub struct LocationsService {
    pub(crate) client: Ghl,
}

impl LocationsService {
    pub(crate) fn new(client: Ghl) -> Self {
        Self { client }
    }

    /// `GET /locations/{id}`
    pub async fn get(&self, location_id: &str) -> Result<Location> {
        let envelope: LocationEnvelope = self
            .client
            .send(
                Method::GET,
                &format!("/locations/{location_id}"),
                &[],
                None::<&()>,
            )
            .await?;
        Ok(envelope.location)
    }

    /// `GET /locations/search` — list locations visible to the credential.
    ///
    /// With an agency (Company) token this lists the agency's sub-accounts;
    /// pass `company_id` to filter explicitly.
    pub async fn search(&self, company_id: Option<&str>, limit: u32) -> Result<Vec<Location>> {
        let mut query: Vec<(String, String)> = vec![("limit".into(), limit.to_string())];
        if let Some(id) = company_id {
            query.push(("companyId".into(), id.to_owned()));
        }
        let list: LocationList = self
            .client
            .send(Method::GET, "/locations/search", &query, None::<&()>)
            .await?;
        Ok(list.locations)
    }
}
