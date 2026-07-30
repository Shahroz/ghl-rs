//! Opportunities — pipeline deals, their stages, and status transitions.
//!
//! Access via [`Ghl::opportunities`](crate::Ghl::opportunities). See the
//! [full opportunities reference][ref] for all 12 v2 endpoints.
//!
//! | Method | Endpoint | Scope |
//! |---|---|---|
//! | [`OpportunitiesService::pipelines`] | `GET /opportunities/pipelines` | `opportunities.readonly` |
//! | [`OpportunitiesService::search`] | `GET /opportunities/search` | `opportunities.readonly` |
//! | [`OpportunitiesService::get`] | `GET /opportunities/{id}` | `opportunities.readonly` |
//! | [`OpportunitiesService::create`] | `POST /opportunities/` | `opportunities.write` |
//! | [`OpportunitiesService::update`] | `PUT /opportunities/{id}` | `opportunities.write` |
//! | [`OpportunitiesService::update_status`] | `PUT /opportunities/{id}/status` | `opportunities.write` |
//! | [`OpportunitiesService::delete`] | `DELETE /opportunities/{id}` | `opportunities.write` |
//!
//! Statuses are `open`, `won`, `lost`, and `abandoned`.
//!
//! # Examples
//!
//! Stage ids live on the pipeline, so fetch it first:
//!
//! ```no_run
//! # use ghl_sdk::{Ghl, opportunities::CreateOpportunity};
//! # async fn demo(ghl: Ghl, loc: String, contact_id: String) -> Result<(), ghl_sdk::Error> {
//! let pipelines = ghl.opportunities().pipelines(&loc).await?;
//! let pipeline = &pipelines[0];
//!
//! let deal = ghl.opportunities().create(CreateOpportunity {
//!     location_id: loc,
//!     pipeline_id: pipeline.id.clone(),
//!     name: "Acme — annual plan".into(),
//!     pipeline_stage_id: Some(pipeline.stages[0].id.clone()),
//!     monetary_value: Some(12_000.0),
//!     contact_id: Some(contact_id),
//!     ..Default::default()
//! }).await?;
//!
//! // Mark it won
//! ghl.opportunities().update_status(&deal.id, "won").await?;
//! # Ok(()) }
//! ```
//!
//! Search with filters, then page or stream:
//!
//! ```no_run
//! # use ghl_sdk::Ghl;
//! # async fn demo(ghl: Ghl, loc: &str) -> Result<(), ghl_sdk::Error> {
//! let page = ghl.opportunities().search(loc)
//!     .status("open")
//!     .limit(50)
//!     .page()
//!     .await?;
//! println!("{} open deals", page.opportunities.len());
//! # Ok(()) }
//! ```
//!
//! # A wire-format quirk
//!
//! Unlike the rest of the API, `GET /opportunities/search` takes **snake_case**
//! query parameters (`location_id`, `pipeline_id`). This module absorbs that for
//! you — [`SearchOpportunities`] sends the right names.
//!
//! [ref]: https://github.com/Shahroz/ghl-rs/blob/main/docs/api/opportunities.md

use futures_util::stream::{self, Stream, StreamExt, TryStreamExt};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::client::Ghl;
use crate::contacts::ListMeta;
use crate::error::Result;

/// A GoHighLevel opportunity (a deal in a pipeline).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)] // fields mirror the API wire format 1:1
pub struct Opportunity {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pipeline_stage_id: Option<String>,
    /// One of `open`, `won`, `lost`, `abandoned`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monetary_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Any fields this SDK doesn't model yet.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Payload for [`OpportunitiesService::create`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)] // fields mirror the API wire format 1:1
pub struct CreateOpportunity {
    pub location_id: String,
    pub pipeline_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_stage_id: Option<String>,
    /// One of `open`, `won`, `lost`, `abandoned` (defaults to `open` upstream).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monetary_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
}

/// Payload for [`OpportunitiesService::update`]. Only set fields are sent.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)] // fields mirror the API wire format 1:1
pub struct UpdateOpportunity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_stage_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monetary_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
}

/// A pipeline definition with its ordered stages.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)] // fields mirror the API wire format 1:1
pub struct Pipeline {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stages: Vec<PipelineStage>,
    /// Any fields this SDK doesn't model yet.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// One stage within a [`Pipeline`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)] // fields mirror the API wire format 1:1
pub struct PipelineStage {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    /// Any fields this SDK doesn't model yet.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Deserialize)]
struct OpportunityEnvelope {
    opportunity: Opportunity,
}

#[derive(Deserialize)]
struct PipelineList {
    #[serde(default)]
    pipelines: Vec<Pipeline>,
}

/// One page of opportunities plus cursor metadata.
#[derive(Debug, Clone, Deserialize)]
pub struct OpportunityPage {
    /// The opportunities on this page.
    #[serde(default)]
    pub opportunities: Vec<Opportunity>,
    /// Cursor metadata; `meta.start_after_id` feeds the next page request.
    #[serde(default)]
    pub meta: Option<ListMeta>,
}

/// Access to the Opportunities API. Obtained via [`Ghl::opportunities`].
pub struct OpportunitiesService {
    client: Ghl,
}

impl OpportunitiesService {
    pub(crate) fn new(client: Ghl) -> Self {
        Self { client }
    }

    /// `GET /opportunities/pipelines` — pipelines (and their stages) for a location.
    pub async fn pipelines(&self, location_id: &str) -> Result<Vec<Pipeline>> {
        let list: PipelineList = self
            .client
            .send(
                Method::GET,
                "/opportunities/pipelines",
                &[("locationId".into(), location_id.to_owned())],
                None::<&()>,
            )
            .await?;
        Ok(list.pipelines)
    }

    /// `POST /opportunities/`
    pub async fn create(&self, opportunity: CreateOpportunity) -> Result<Opportunity> {
        let envelope: OpportunityEnvelope = self
            .client
            .send(Method::POST, "/opportunities/", &[], Some(&opportunity))
            .await?;
        Ok(envelope.opportunity)
    }

    /// `GET /opportunities/{id}`
    pub async fn get(&self, opportunity_id: &str) -> Result<Opportunity> {
        let envelope: OpportunityEnvelope = self
            .client
            .send(
                Method::GET,
                &format!("/opportunities/{opportunity_id}"),
                &[],
                None::<&()>,
            )
            .await?;
        Ok(envelope.opportunity)
    }

    /// `PUT /opportunities/{id}`
    pub async fn update(
        &self,
        opportunity_id: &str,
        update: UpdateOpportunity,
    ) -> Result<Opportunity> {
        let envelope: OpportunityEnvelope = self
            .client
            .send(
                Method::PUT,
                &format!("/opportunities/{opportunity_id}"),
                &[],
                Some(&update),
            )
            .await?;
        Ok(envelope.opportunity)
    }

    /// `PUT /opportunities/{id}/status` — change only the status
    /// (`open` | `won` | `lost` | `abandoned`).
    pub async fn update_status(&self, opportunity_id: &str, status: &str) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .send(
                Method::PUT,
                &format!("/opportunities/{opportunity_id}/status"),
                &[],
                Some(&serde_json::json!({ "status": status })),
            )
            .await?;
        Ok(())
    }

    /// `DELETE /opportunities/{id}`
    pub async fn delete(&self, opportunity_id: &str) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .send(
                Method::DELETE,
                &format!("/opportunities/{opportunity_id}"),
                &[],
                None::<&()>,
            )
            .await?;
        Ok(())
    }

    /// `GET /opportunities/search` — returns a lazy request builder.
    pub fn search(&self, location_id: impl Into<String>) -> SearchOpportunities {
        SearchOpportunities {
            client: self.client.clone(),
            location_id: location_id.into(),
            limit: 20,
            query: None,
            pipeline_id: None,
            status: None,
            start_after_id: None,
            start_after: None,
        }
    }
}

/// Builder for searching opportunities. Finish with [`SearchOpportunities::page`]
/// (one page) or [`SearchOpportunities::stream`] (auto-pagination).
#[derive(Clone)]
pub struct SearchOpportunities {
    client: Ghl,
    location_id: String,
    limit: u32,
    query: Option<String>,
    pipeline_id: Option<String>,
    status: Option<String>,
    start_after_id: Option<String>,
    start_after: Option<i64>,
}

impl SearchOpportunities {
    /// Page size, 1–100 (API default 20).
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = limit.clamp(1, 100);
        self
    }

    /// Free-text search query.
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// Restrict to one pipeline.
    pub fn pipeline_id(mut self, pipeline_id: impl Into<String>) -> Self {
        self.pipeline_id = Some(pipeline_id.into());
        self
    }

    /// Filter by status: `open` | `won` | `lost` | `abandoned` | `all`.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Resume from a previous page's cursor.
    pub fn start_after_id(mut self, cursor: impl Into<String>) -> Self {
        self.start_after_id = Some(cursor.into());
        self
    }

    /// Fetch a single page.
    pub async fn page(&self) -> Result<OpportunityPage> {
        // This endpoint uses snake_case query params, unlike the rest of the API.
        let mut query: Vec<(String, String)> = vec![
            ("location_id".into(), self.location_id.clone()),
            ("limit".into(), self.limit.to_string()),
        ];
        if let Some(q) = &self.query {
            query.push(("q".into(), q.clone()));
        }
        if let Some(p) = &self.pipeline_id {
            query.push(("pipeline_id".into(), p.clone()));
        }
        if let Some(s) = &self.status {
            query.push(("status".into(), s.clone()));
        }
        if let Some(id) = &self.start_after_id {
            query.push(("startAfterId".into(), id.clone()));
        }
        if let Some(ts) = self.start_after {
            query.push(("startAfter".into(), ts.to_string()));
        }
        self.client
            .send(Method::GET, "/opportunities/search", &query, None::<&()>)
            .await
    }

    /// Auto-paginating stream of opportunities, following `meta.startAfterId` cursors.
    pub fn stream(self) -> impl Stream<Item = Result<Opportunity>> {
        stream::try_unfold(Some(self), |state| async move {
            let Some(mut request) = state else {
                return Ok::<_, crate::Error>(None);
            };
            let page = request.page().await?;
            let full_page = page.opportunities.len() as u32 >= request.limit;
            let cursor = page.meta.as_ref().and_then(|m| m.start_after_id.clone());
            let start_after = page.meta.as_ref().and_then(|m| m.start_after);

            let next = match (full_page, cursor) {
                (true, Some(cursor)) => {
                    request.start_after_id = Some(cursor);
                    request.start_after = start_after;
                    Some(request)
                }
                _ => None,
            };
            Ok(Some((
                stream::iter(page.opportunities.into_iter().map(Ok)),
                next,
            )))
        })
        .try_flatten()
        .boxed()
    }
}
