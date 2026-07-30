//! Contacts — create, read, update, delete, and stream contact records.
//!
//! Access via [`Ghl::contacts`](crate::Ghl::contacts). This is the busiest module
//! in most integrations; the [full contacts reference][ref] documents all 32 v2
//! endpoints, of which the five below are typed.
//!
//! | Method | Endpoint | Scope |
//! |---|---|---|
//! | [`ContactsService::create`] | `POST /contacts/` | `contacts.write` |
//! | [`ContactsService::get`] | `GET /contacts/{id}` | `contacts.readonly` |
//! | [`ContactsService::update`] | `PUT /contacts/{id}` | `contacts.write` |
//! | [`ContactsService::delete`] | `DELETE /contacts/{id}` | `contacts.write` |
//! | [`ContactsService::list`] | `GET /contacts/` | `contacts.readonly` |
//!
//! # Examples
//!
//! Create a contact (at least an email or phone is required by the API):
//!
//! ```no_run
//! # use ghl_sdk::{Ghl, contacts::CreateContact};
//! # async fn demo(ghl: Ghl, loc: String) -> Result<(), ghl_sdk::Error> {
//! let contact = ghl.contacts().create(CreateContact {
//!     location_id: loc,
//!     email: Some("ada@example.com".into()),
//!     phone: Some("+15551234567".into()),
//!     first_name: Some("Ada".into()),
//!     tags: vec!["newsletter".into()],
//!     ..Default::default()
//! }).await?;
//! # Ok(()) }
//! ```
//!
//! Update only the fields you set — omitted fields are left untouched:
//!
//! ```no_run
//! # use ghl_sdk::{Ghl, contacts::UpdateContact};
//! # async fn demo(ghl: Ghl, id: &str) -> Result<(), ghl_sdk::Error> {
//! ghl.contacts().update(id, UpdateContact {
//!     last_name: Some("Lovelace".into()),
//!     ..Default::default()
//! }).await?;
//! # Ok(()) }
//! ```
//!
//! Stream every contact, following GoHighLevel's `startAfterId` cursor
//! automatically:
//!
//! ```no_run
//! # use ghl_sdk::Ghl;
//! use futures_util::TryStreamExt;
//!
//! # async fn demo(ghl: Ghl, loc: &str) -> Result<(), ghl_sdk::Error> {
//! let mut stream = ghl.contacts().list(loc).limit(100).stream();
//! while let Some(contact) = stream.try_next().await? {
//!     println!("{} {:?}", contact.id, contact.email);
//! }
//! # Ok(()) }
//! ```
//!
//! [ref]: https://github.com/Shahroz/ghl-rs/blob/main/docs/api/contacts.md

use futures_util::stream::{self, Stream, StreamExt, TryStreamExt};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::client::Ghl;
use crate::error::Result;

/// A GoHighLevel contact.
///
/// Unknown fields are preserved in [`Contact::extra`] so payload drift upstream
/// never breaks deserialization.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)] // fields mirror the API wire format 1:1
pub struct Contact {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_added: Option<String>,
    /// Any fields this SDK doesn't model yet.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Payload for [`ContactsService::create`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)] // fields mirror the API wire format 1:1
pub struct CreateContact {
    pub location_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// Payload for [`ContactsService::update`]. Only set fields are sent.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)] // fields mirror the API wire format 1:1
pub struct UpdateContact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct ContactEnvelope {
    contact: Contact,
}

/// One page of contacts plus the cursor for the next page.
#[derive(Debug, Clone, Deserialize)]
pub struct ContactPage {
    /// The contacts on this page.
    #[serde(default)]
    pub contacts: Vec<Contact>,
    /// Cursor metadata; `meta.start_after_id` feeds the next page request.
    #[serde(default)]
    pub meta: Option<ListMeta>,
}

/// Cursor metadata returned by list endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListMeta {
    /// Cursor: pass as `startAfterId` to fetch the next page.
    #[serde(default)]
    pub start_after_id: Option<String>,
    /// Cursor timestamp companion to `start_after_id`.
    #[serde(default)]
    pub start_after: Option<i64>,
    /// Total matching records, when the API reports it.
    #[serde(default)]
    pub total: Option<i64>,
}

/// Access to the Contacts API. Obtained via [`Ghl::contacts`].
pub struct ContactsService {
    client: Ghl,
}

impl ContactsService {
    pub(crate) fn new(client: Ghl) -> Self {
        Self { client }
    }

    /// `POST /contacts/`
    pub async fn create(&self, contact: CreateContact) -> Result<Contact> {
        let envelope: ContactEnvelope = self
            .client
            .send(Method::POST, "/contacts/", &[], Some(&contact))
            .await?;
        Ok(envelope.contact)
    }

    /// `GET /contacts/{id}`
    pub async fn get(&self, contact_id: &str) -> Result<Contact> {
        let envelope: ContactEnvelope = self
            .client
            .send(
                Method::GET,
                &format!("/contacts/{contact_id}"),
                &[],
                None::<&()>,
            )
            .await?;
        Ok(envelope.contact)
    }

    /// `PUT /contacts/{id}`
    pub async fn update(&self, contact_id: &str, update: UpdateContact) -> Result<Contact> {
        let envelope: ContactEnvelope = self
            .client
            .send(
                Method::PUT,
                &format!("/contacts/{contact_id}"),
                &[],
                Some(&update),
            )
            .await?;
        Ok(envelope.contact)
    }

    /// `DELETE /contacts/{id}`
    pub async fn delete(&self, contact_id: &str) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .send(
                Method::DELETE,
                &format!("/contacts/{contact_id}"),
                &[],
                None::<&()>,
            )
            .await?;
        Ok(())
    }

    /// `GET /contacts/` — returns a lazy request builder.
    pub fn list(&self, location_id: impl Into<String>) -> ListContacts {
        ListContacts {
            client: self.client.clone(),
            location_id: location_id.into(),
            limit: 20,
            query: None,
            start_after_id: None,
            start_after: None,
        }
    }
}

/// Builder for listing contacts. Finish with [`ListContacts::page`] (one page)
/// or [`ListContacts::stream`] (auto-pagination).
#[derive(Clone)]
pub struct ListContacts {
    client: Ghl,
    location_id: String,
    limit: u32,
    query: Option<String>,
    start_after_id: Option<String>,
    start_after: Option<i64>,
}

impl ListContacts {
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

    /// Resume from a previous page's cursor.
    pub fn start_after_id(mut self, cursor: impl Into<String>) -> Self {
        self.start_after_id = Some(cursor.into());
        self
    }

    /// Fetch a single page.
    pub async fn page(&self) -> Result<ContactPage> {
        let mut query: Vec<(String, String)> = vec![
            ("locationId".into(), self.location_id.clone()),
            ("limit".into(), self.limit.to_string()),
        ];
        if let Some(q) = &self.query {
            query.push(("query".into(), q.clone()));
        }
        if let Some(id) = &self.start_after_id {
            query.push(("startAfterId".into(), id.clone()));
        }
        if let Some(ts) = self.start_after {
            query.push(("startAfter".into(), ts.to_string()));
        }
        self.client
            .send(Method::GET, "/contacts/", &query, None::<&()>)
            .await
    }

    /// Auto-paginating stream of contacts, following `meta.startAfterId` cursors.
    pub fn stream(self) -> impl Stream<Item = Result<Contact>> {
        stream::try_unfold(Some(self), |state| async move {
            let Some(mut request) = state else {
                return Ok::<_, crate::Error>(None);
            };
            let page = request.page().await?;
            let full_page = page.contacts.len() as u32 >= request.limit;
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
                stream::iter(page.contacts.into_iter().map(Ok)),
                next,
            )))
        })
        .try_flatten()
        .boxed()
    }
}
