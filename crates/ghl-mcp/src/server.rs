//! MCP tool definitions, delegating to `ghl-sdk`.

use ghl_sdk::contacts::{CreateContact, UpdateContact};
use ghl_sdk::Ghl;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_handler, tool_router, ErrorData, ServerHandler};
use serde::Deserialize;
use serde_json::json;

/// Shared server state.
#[derive(Clone)]
pub struct GhlServer {
    ghl: Ghl,
    default_location: Option<String>,
    allow_destructive: bool,
}

fn internal(err: ghl_sdk::Error) -> ErrorData {
    ErrorData::internal_error(err.to_string(), None)
}

fn ok_json(value: &serde_json::Value) -> Result<String, ErrorData> {
    serde_json::to_string_pretty(value)
        .map_err(|e| ErrorData::internal_error(format!("serialization failed: {e}"), None))
}

/// Trim a contact to the fields agents actually need (full payload available via extra=true).
fn contact_summary(c: &ghl_sdk::contacts::Contact) -> serde_json::Value {
    json!({
        "id": c.id,
        "locationId": c.location_id,
        "email": c.email,
        "phone": c.phone,
        "firstName": c.first_name,
        "lastName": c.last_name,
        "tags": c.tags,
        "dateAdded": c.date_added,
    })
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchContactsParams {
    /// Free-text search (name, email, phone). Omit to list all contacts.
    pub query: Option<String>,
    /// Sub-account (location) id. Omit to use the server's default location.
    pub location_id: Option<String>,
    /// Max results, 1-100 (default 20).
    pub limit: Option<u32>,
    /// Cursor from a previous call's `next_cursor` to fetch the next page.
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetContactParams {
    /// The contact id.
    pub contact_id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateContactParams {
    /// Sub-account (location) id. Omit to use the server's default location.
    pub location_id: Option<String>,
    pub email: Option<String>,
    /// Phone number in E.164 format, e.g. "+15551234567".
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    /// Tags to apply, e.g. ["hot-lead"].
    pub tags: Option<Vec<String>>,
    /// Attribution source, e.g. "claude-agent".
    pub source: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UpdateContactParams {
    /// The contact id to update.
    pub contact_id: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    /// Replaces the full tag list when provided.
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DeleteContactParams {
    /// The contact id to delete.
    pub contact_id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListLocationsParams {
    /// Agency (company) id to filter by. Usually unnecessary.
    pub company_id: Option<String>,
    /// Max results (default 50).
    pub limit: Option<u32>,
}

#[tool_router]
impl GhlServer {
    pub fn new(ghl: Ghl, default_location: Option<String>, allow_destructive: bool) -> Self {
        Self {
            ghl,
            default_location,
            allow_destructive,
        }
    }

    fn resolve_location(&self, param: Option<String>) -> Result<String, ErrorData> {
        param
            .or_else(|| self.default_location.clone())
            .ok_or_else(|| {
                ErrorData::invalid_params(
                    "no location_id given and no default configured — pass `location_id`, \
                     or start the server with --location-id / GHL_LOCATION_ID. \
                     Use ghl_list_locations to discover available locations.",
                    None,
                )
            })
    }

    #[tool(
        description = "Search or list contacts in a GoHighLevel location. Returns a page of \
                       contact summaries and a `next_cursor` for pagination. Read-only."
    )]
    async fn ghl_search_contacts(
        &self,
        Parameters(p): Parameters<SearchContactsParams>,
    ) -> Result<String, ErrorData> {
        let location = self.resolve_location(p.location_id)?;
        let mut request = self
            .ghl
            .contacts()
            .list(&location)
            .limit(p.limit.unwrap_or(20));
        if let Some(q) = p.query {
            request = request.query(q);
        }
        if let Some(cursor) = p.cursor {
            request = request.start_after_id(cursor);
        }
        let page = request.page().await.map_err(internal)?;
        let next_cursor = page.meta.as_ref().and_then(|m| m.start_after_id.clone());
        ok_json(&json!({
            "contacts": page.contacts.iter().map(contact_summary).collect::<Vec<_>>(),
            "count": page.contacts.len(),
            "total": page.meta.as_ref().and_then(|m| m.total),
            "next_cursor": next_cursor,
        }))
    }

    #[tool(description = "Fetch one contact by id, with all fields. Read-only.")]
    async fn ghl_get_contact(
        &self,
        Parameters(p): Parameters<GetContactParams>,
    ) -> Result<String, ErrorData> {
        let contact = self
            .ghl
            .contacts()
            .get(&p.contact_id)
            .await
            .map_err(internal)?;
        ok_json(&serde_json::to_value(&contact).unwrap_or_default())
    }

    #[tool(
        description = "Create a contact in a GoHighLevel location. Provide at least an email \
                       or a phone number."
    )]
    async fn ghl_create_contact(
        &self,
        Parameters(p): Parameters<CreateContactParams>,
    ) -> Result<String, ErrorData> {
        if p.email.is_none() && p.phone.is_none() {
            return Err(ErrorData::invalid_params(
                "provide at least `email` or `phone`",
                None,
            ));
        }
        let location = self.resolve_location(p.location_id)?;
        let contact = self
            .ghl
            .contacts()
            .create(CreateContact {
                location_id: location,
                email: p.email,
                phone: p.phone,
                first_name: p.first_name,
                last_name: p.last_name,
                tags: p.tags.unwrap_or_default(),
                source: p.source,
                ..Default::default()
            })
            .await
            .map_err(internal)?;
        ok_json(&contact_summary(&contact))
    }

    #[tool(description = "Update fields on an existing contact. Only provided fields change.")]
    async fn ghl_update_contact(
        &self,
        Parameters(p): Parameters<UpdateContactParams>,
    ) -> Result<String, ErrorData> {
        let contact = self
            .ghl
            .contacts()
            .update(
                &p.contact_id,
                UpdateContact {
                    email: p.email,
                    phone: p.phone,
                    first_name: p.first_name,
                    last_name: p.last_name,
                    tags: p.tags,
                    ..Default::default()
                },
            )
            .await
            .map_err(internal)?;
        ok_json(&contact_summary(&contact))
    }

    #[tool(
        description = "Permanently delete a contact. DESTRUCTIVE — only available when the \
                       server was started with --allow-destructive."
    )]
    async fn ghl_delete_contact(
        &self,
        Parameters(p): Parameters<DeleteContactParams>,
    ) -> Result<String, ErrorData> {
        if !self.allow_destructive {
            return Err(ErrorData::invalid_request(
                "destructive tools are disabled — restart ghl-mcp with --allow-destructive \
                 (or GHL_ALLOW_DESTRUCTIVE=true) to enable contact deletion",
                None,
            ));
        }
        self.ghl
            .contacts()
            .delete(&p.contact_id)
            .await
            .map_err(internal)?;
        ok_json(&json!({ "deleted": p.contact_id }))
    }

    #[tool(
        description = "List GoHighLevel locations (sub-accounts) visible to this credential. \
                       Use this to discover location ids for other tools. Read-only."
    )]
    async fn ghl_list_locations(
        &self,
        Parameters(p): Parameters<ListLocationsParams>,
    ) -> Result<String, ErrorData> {
        let locations = self
            .ghl
            .locations()
            .search(p.company_id.as_deref(), p.limit.unwrap_or(50))
            .await
            .map_err(internal)?;
        let summaries: Vec<_> = locations
            .iter()
            .map(|l| {
                json!({
                    "id": l.id,
                    "name": l.name,
                    "city": l.city,
                    "country": l.country,
                    "timezone": l.timezone,
                })
            })
            .collect();
        ok_json(&json!({ "locations": summaries, "count": summaries.len() }))
    }

    #[tool(
        description = "Report remaining GoHighLevel API rate-limit budget (burst window and \
                       daily) as last observed. Read-only, makes no API call."
    )]
    async fn ghl_rate_status(&self) -> Result<String, ErrorData> {
        let status = self.ghl.rate_status();
        ok_json(&json!({
            "burst_remaining": status.burst_remaining,
            "daily_remaining": status.daily_remaining,
            "note": "null means no request has been made yet this session",
        }))
    }
}

#[tool_handler(
    name = "ghl-mcp",
    version = "0.1.0",
    instructions = "Tools for working with a GoHighLevel (HighLevel) CRM account. \
                    Contacts and locations are supported today. If no default location is \
                    configured, call ghl_list_locations first and pass location_id explicitly. \
                    Paginate with the returned next_cursor. Deletion requires the server to be \
                    started with --allow-destructive."
)]
impl ServerHandler for GhlServer {}
