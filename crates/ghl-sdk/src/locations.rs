//! Locations (sub-accounts) API.

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
    client: Ghl,
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
