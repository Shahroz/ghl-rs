//! MCP tool definitions, delegating to `ghl-sdk`.

use ghl_sdk::calendars::CreateAppointment;
use ghl_sdk::contacts::{CreateContact, UpdateContact};
use ghl_sdk::conversations::SendMessage;
use ghl_sdk::opportunities::{CreateOpportunity, UpdateOpportunity};
use ghl_sdk::Ghl;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_handler, tool_router, ErrorData, ServerHandler};
use serde::Deserialize;
use serde_json::json;

use crate::operations;

/// Percent-encode a value going into a path segment.
fn urlencode_segment(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => vec![c],
            _ => format!("%{:02X}", c as u32).chars().collect(),
        })
        .collect()
}

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

fn opportunity_summary(o: &ghl_sdk::opportunities::Opportunity) -> serde_json::Value {
    json!({
        "id": o.id,
        "name": o.name,
        "status": o.status,
        "pipelineId": o.pipeline_id,
        "pipelineStageId": o.pipeline_stage_id,
        "contactId": o.contact_id,
        "monetaryValue": o.monetary_value,
        "assignedTo": o.assigned_to,
        "updatedAt": o.updated_at,
    })
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListPipelinesParams {
    /// Sub-account (location) id. Omit to use the server's default location.
    pub location_id: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchOpportunitiesParams {
    /// Free-text search on opportunity/contact name. Omit to list all.
    pub query: Option<String>,
    /// Sub-account (location) id. Omit to use the server's default location.
    pub location_id: Option<String>,
    /// Restrict to one pipeline id (see ghl_list_pipelines).
    pub pipeline_id: Option<String>,
    /// Filter: "open", "won", "lost", "abandoned", or "all".
    pub status: Option<String>,
    /// Max results, 1-100 (default 20).
    pub limit: Option<u32>,
    /// Cursor from a previous call's `next_cursor` to fetch the next page.
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetOpportunityParams {
    /// The opportunity id.
    pub opportunity_id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateOpportunityParams {
    /// Sub-account (location) id. Omit to use the server's default location.
    pub location_id: Option<String>,
    /// Pipeline id (find via ghl_list_pipelines).
    pub pipeline_id: String,
    /// Opportunity name, e.g. "Acme Corp — annual plan".
    pub name: String,
    /// Stage id within the pipeline (defaults to the first stage).
    pub pipeline_stage_id: Option<String>,
    /// Contact id to attach the deal to.
    pub contact_id: Option<String>,
    /// Deal value in the location's currency.
    pub monetary_value: Option<f64>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MoveOpportunityParams {
    /// The opportunity id.
    pub opportunity_id: String,
    /// Target stage id within the same pipeline (see ghl_list_pipelines).
    pub pipeline_stage_id: Option<String>,
    /// New status: "open", "won", "lost", or "abandoned".
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchConversationsParams {
    /// Free-text search over contact name / message content.
    pub query: Option<String>,
    /// Sub-account (location) id. Omit to use the server's default location.
    pub location_id: Option<String>,
    /// Max results, 1-100 (default 20).
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetMessagesParams {
    /// Conversation id from ghl_search_conversations.
    pub conversation_id: String,
    /// Max messages, 1-100 (default 20). Newest first.
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SendMessageParams {
    /// Contact id to message.
    pub contact_id: String,
    /// Channel: "SMS", "Email", "WhatsApp", "IG", "FB", or "Live_Chat".
    pub message_type: String,
    /// Message body (plain text).
    pub message: Option<String>,
    /// Subject line — Email only.
    pub subject: Option<String>,
    /// HTML body — Email only.
    pub html: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListCalendarsParams {
    /// Sub-account (location) id. Omit to use the server's default location.
    pub location_id: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FreeSlotsParams {
    /// Calendar id from ghl_list_calendars.
    pub calendar_id: String,
    /// Range start as epoch milliseconds.
    pub start_date: i64,
    /// Range end as epoch milliseconds.
    pub end_date: i64,
    /// IANA timezone for the returned slots, e.g. "America/New_York".
    pub timezone: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct BookAppointmentParams {
    /// Calendar id from ghl_list_calendars.
    pub calendar_id: String,
    /// Contact id to book for.
    pub contact_id: String,
    /// Start time, ISO-8601 with offset, e.g. "2026-08-01T14:00:00+05:00".
    /// Use a slot from ghl_get_free_slots.
    pub start_time: String,
    /// End time; defaults to the calendar's slot duration when omitted.
    pub end_time: Option<String>,
    /// Appointment title.
    pub title: Option<String>,
    /// Sub-account (location) id. Omit to use the server's default location.
    pub location_id: Option<String>,
    /// Notify the contact and assigned user (default: API's own default).
    pub to_notify: Option<bool>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchOperationsParams {
    /// What you want to do, e.g. "create invoice", "list calendar events",
    /// "send sms", "upload media". Leave empty with a module filter to browse.
    pub query: String,
    /// Restrict to one API module, e.g. "invoices", "calendars", "payments".
    /// Call with an empty query and no module to see every module name.
    pub module: Option<String>,
    /// API version: "v2" (stable, the default preference) or "v3" (newer, adds
    /// modules like ad-publishing, social-planner, saas, chat-widget).
    pub api_version: Option<String>,
    /// Max results (default 10, max 50).
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DescribeOperationParams {
    /// Operation id from ghl_search_operations, e.g. "invoices.post_invoices".
    pub operation_id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ExecuteOperationParams {
    /// Operation id from ghl_search_operations, e.g. "invoices.get_invoices".
    pub operation_id: String,
    /// Path placeholder values, e.g. {"invoiceId": "abc123"}. Required for any
    /// path containing {braces}.
    pub path_params: Option<serde_json::Map<String, serde_json::Value>>,
    /// Query string values, e.g. {"limit": "20", "altId": "loc_x"}.
    pub query: Option<serde_json::Map<String, serde_json::Value>>,
    /// JSON request body for POST/PUT/PATCH operations.
    pub body: Option<serde_json::Value>,
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
        // /locations/search needs an agency (Company) token. Location-scoped
        // credentials (the common PIT case) get 403 there but can still fetch
        // their own location — fall back to that so the tool stays useful.
        let (locations, note) = match self
            .ghl
            .locations()
            .search(p.company_id.as_deref(), p.limit.unwrap_or(50))
            .await
        {
            Ok(list) => (list, None),
            Err(ghl_sdk::Error::Api { status, .. })
                if status.as_u16() == 403 && self.default_location.is_some() =>
            {
                let id = self.default_location.as_deref().unwrap();
                let location = self.ghl.locations().get(id).await.map_err(internal)?;
                (
                    vec![location],
                    Some(
                        "credential is location-scoped (agency-wide search is forbidden); \
                         showing the configured location only",
                    ),
                )
            }
            Err(e) => return Err(internal(e)),
        };
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
        ok_json(&json!({ "locations": summaries, "count": summaries.len(), "note": note }))
    }

    #[tool(
        description = "List sales pipelines and their ordered stages for a location. Call this \
                       before creating or moving opportunities to get pipeline/stage ids. \
                       Read-only."
    )]
    async fn ghl_list_pipelines(
        &self,
        Parameters(p): Parameters<ListPipelinesParams>,
    ) -> Result<String, ErrorData> {
        let location = self.resolve_location(p.location_id)?;
        let pipelines = self
            .ghl
            .opportunities()
            .pipelines(&location)
            .await
            .map_err(internal)?;
        let summaries: Vec<_> = pipelines
            .iter()
            .map(|pl| {
                json!({
                    "id": pl.id,
                    "name": pl.name,
                    "stages": pl.stages.iter().map(|s| json!({
                        "id": s.id, "name": s.name, "position": s.position,
                    })).collect::<Vec<_>>(),
                })
            })
            .collect();
        ok_json(&json!({ "pipelines": summaries, "count": summaries.len() }))
    }

    #[tool(
        description = "Search or list opportunities (pipeline deals) in a location, optionally \
                       filtered by pipeline, status, or free text. Returns summaries and a \
                       `next_cursor` for pagination. Read-only."
    )]
    async fn ghl_search_opportunities(
        &self,
        Parameters(p): Parameters<SearchOpportunitiesParams>,
    ) -> Result<String, ErrorData> {
        let location = self.resolve_location(p.location_id)?;
        let mut request = self
            .ghl
            .opportunities()
            .search(&location)
            .limit(p.limit.unwrap_or(20));
        if let Some(q) = p.query {
            request = request.query(q);
        }
        if let Some(id) = p.pipeline_id {
            request = request.pipeline_id(id);
        }
        if let Some(s) = p.status {
            request = request.status(s);
        }
        if let Some(cursor) = p.cursor {
            request = request.start_after_id(cursor);
        }
        let page = request.page().await.map_err(internal)?;
        let next_cursor = page.meta.as_ref().and_then(|m| m.start_after_id.clone());
        ok_json(&json!({
            "opportunities": page.opportunities.iter().map(opportunity_summary).collect::<Vec<_>>(),
            "count": page.opportunities.len(),
            "total": page.meta.as_ref().and_then(|m| m.total),
            "next_cursor": next_cursor,
        }))
    }

    #[tool(description = "Fetch one opportunity by id, with all fields. Read-only.")]
    async fn ghl_get_opportunity(
        &self,
        Parameters(p): Parameters<GetOpportunityParams>,
    ) -> Result<String, ErrorData> {
        let opportunity = self
            .ghl
            .opportunities()
            .get(&p.opportunity_id)
            .await
            .map_err(internal)?;
        ok_json(&serde_json::to_value(&opportunity).unwrap_or_default())
    }

    #[tool(
        description = "Create an opportunity (deal) in a pipeline. Get pipeline_id and stage \
                       ids from ghl_list_pipelines first."
    )]
    async fn ghl_create_opportunity(
        &self,
        Parameters(p): Parameters<CreateOpportunityParams>,
    ) -> Result<String, ErrorData> {
        let location = self.resolve_location(p.location_id)?;
        let opportunity = self
            .ghl
            .opportunities()
            .create(CreateOpportunity {
                location_id: location,
                pipeline_id: p.pipeline_id,
                name: p.name,
                pipeline_stage_id: p.pipeline_stage_id,
                contact_id: p.contact_id,
                monetary_value: p.monetary_value,
                ..Default::default()
            })
            .await
            .map_err(internal)?;
        ok_json(&opportunity_summary(&opportunity))
    }

    #[tool(
        description = "Move an opportunity to another pipeline stage and/or change its status \
                       (open/won/lost/abandoned). Provide at least one of the two."
    )]
    async fn ghl_move_opportunity(
        &self,
        Parameters(p): Parameters<MoveOpportunityParams>,
    ) -> Result<String, ErrorData> {
        if p.pipeline_stage_id.is_none() && p.status.is_none() {
            return Err(ErrorData::invalid_params(
                "provide `pipeline_stage_id`, `status`, or both",
                None,
            ));
        }
        let opportunity = self
            .ghl
            .opportunities()
            .update(
                &p.opportunity_id,
                UpdateOpportunity {
                    pipeline_stage_id: p.pipeline_stage_id,
                    status: p.status,
                    ..Default::default()
                },
            )
            .await
            .map_err(internal)?;
        ok_json(&opportunity_summary(&opportunity))
    }

    #[tool(
        description = "Search conversation threads in a location, with the latest message \
                       preview and unread counts. Read-only."
    )]
    async fn ghl_search_conversations(
        &self,
        Parameters(p): Parameters<SearchConversationsParams>,
    ) -> Result<String, ErrorData> {
        let location = self.resolve_location(p.location_id)?;
        let page = self
            .ghl
            .conversations()
            .search(&location, p.query.as_deref(), p.limit.unwrap_or(20))
            .await
            .map_err(internal)?;
        let listed: Vec<_> = page
            .conversations
            .iter()
            .map(|c| {
                json!({
                    "id": c.id,
                    "contactId": c.contact_id,
                    "fullName": c.full_name,
                    "lastMessageBody": c.last_message_body,
                    "lastMessageType": c.last_message_type,
                    "unreadCount": c.unread_count,
                })
            })
            .collect();
        ok_json(&json!({
            "conversations": listed,
            "count": listed.len(),
            "total": page.total,
        }))
    }

    #[tool(description = "Read messages in a conversation thread, newest first. Read-only.")]
    async fn ghl_get_messages(
        &self,
        Parameters(p): Parameters<GetMessagesParams>,
    ) -> Result<String, ErrorData> {
        let page = self
            .ghl
            .conversations()
            .messages(&p.conversation_id, p.limit.unwrap_or(20))
            .await
            .map_err(internal)?;
        let listed: Vec<_> = page
            .messages
            .iter()
            .map(|m| {
                json!({
                    "id": m.id,
                    "body": m.body,
                    "messageType": m.message_type,
                    "direction": m.direction,
                    "status": m.status,
                    "dateAdded": m.date_added,
                })
            })
            .collect();
        ok_json(&json!({
            "messages": listed,
            "count": listed.len(),
            "has_more": page.next_page,
        }))
    }

    #[tool(
        description = "Send an SMS, email, or channel message to a contact. This contacts a real \
                       person — requires the server started with --allow-destructive."
    )]
    async fn ghl_send_message(
        &self,
        Parameters(p): Parameters<SendMessageParams>,
    ) -> Result<String, ErrorData> {
        // Sending reaches a real recipient and cannot be undone, so it sits
        // behind the same gate as deletes.
        if !self.allow_destructive {
            return Err(ErrorData::invalid_request(
                "sending messages is disabled — restart ghl-mcp with --allow-destructive \
                 (or GHL_ALLOW_DESTRUCTIVE=true) to permit outbound messages",
                None,
            ));
        }
        if p.message.is_none() && p.html.is_none() {
            return Err(ErrorData::invalid_params(
                "provide `message` (or `html` for email)",
                None,
            ));
        }
        let result = self
            .ghl
            .conversations()
            .send_message(SendMessage {
                message_type: p.message_type,
                contact_id: p.contact_id,
                message: p.message,
                subject: p.subject,
                html: p.html,
                ..Default::default()
            })
            .await
            .map_err(internal)?;
        ok_json(&json!({
            "conversationId": result.conversation_id,
            "messageId": result.message_id,
            "sent": true,
        }))
    }

    #[tool(
        description = "List bookable calendars in a location. Call before checking free slots or \
                       booking. Read-only."
    )]
    async fn ghl_list_calendars(
        &self,
        Parameters(p): Parameters<ListCalendarsParams>,
    ) -> Result<String, ErrorData> {
        let location = self.resolve_location(p.location_id)?;
        let calendars = self
            .ghl
            .calendars()
            .list(&location)
            .await
            .map_err(internal)?;
        let listed: Vec<_> = calendars
            .iter()
            .map(|c| {
                json!({
                    "id": c.id,
                    "name": c.name,
                    "calendarType": c.calendar_type,
                    "slotDurationMinutes": c.slot_duration,
                    "isActive": c.is_active,
                })
            })
            .collect();
        ok_json(&json!({ "calendars": listed, "count": listed.len() }))
    }

    #[tool(
        description = "Get bookable free slots for a calendar in a date range (epoch \
                       milliseconds). Returns slots grouped by date. Read-only."
    )]
    async fn ghl_get_free_slots(
        &self,
        Parameters(p): Parameters<FreeSlotsParams>,
    ) -> Result<String, ErrorData> {
        if p.end_date <= p.start_date {
            return Err(ErrorData::invalid_params(
                "end_date must be after start_date (both epoch milliseconds)",
                None,
            ));
        }
        let slots = self
            .ghl
            .calendars()
            .free_slots(
                &p.calendar_id,
                p.start_date,
                p.end_date,
                p.timezone.as_deref(),
            )
            .await
            .map_err(internal)?;
        ok_json(&json!({
            "by_date": slots.by_date.iter().map(|(d, s)| json!({ "date": d, "slots": s }))
                .collect::<Vec<_>>(),
            "total_slots": slots.all().len(),
        }))
    }

    #[tool(
        description = "Book an appointment on a calendar for a contact. Creates a real booking \
                       (and may notify the contact) — requires the server started with \
                       --allow-destructive."
    )]
    async fn ghl_book_appointment(
        &self,
        Parameters(p): Parameters<BookAppointmentParams>,
    ) -> Result<String, ErrorData> {
        if !self.allow_destructive {
            return Err(ErrorData::invalid_request(
                "booking is disabled — restart ghl-mcp with --allow-destructive \
                 (or GHL_ALLOW_DESTRUCTIVE=true) to permit appointment creation",
                None,
            ));
        }
        let location = self.resolve_location(p.location_id)?;
        let appointment = self
            .ghl
            .calendars()
            .create_appointment(CreateAppointment {
                calendar_id: p.calendar_id,
                location_id: location,
                contact_id: p.contact_id,
                start_time: p.start_time,
                end_time: p.end_time,
                title: p.title,
                to_notify: p.to_notify,
                ..Default::default()
            })
            .await
            .map_err(internal)?;
        ok_json(&json!({
            "id": appointment.id,
            "title": appointment.title,
            "startTime": appointment.start_time,
            "endTime": appointment.end_time,
            "status": appointment.appointment_status,
        }))
    }

    #[tool(
        description = "Discover any GoHighLevel API operation across all 41 modules (invoices, \
                       calendars, payments, workflows, forms, products, social planner, custom \
                       objects, and more). Use this when no dedicated tool covers what you need, \
                       then ghl_describe_operation and ghl_execute_operation. Read-only."
    )]
    async fn ghl_search_operations(
        &self,
        Parameters(p): Parameters<SearchOperationsParams>,
    ) -> Result<String, ErrorData> {
        let limit = p.limit.unwrap_or(10).clamp(1, 50) as usize;
        // Treat blank strings as absent — agents pass "" for "no filter".
        let blank = |s: &Option<String>| s.as_deref().is_none_or(|v| v.trim().is_empty());
        let module = if blank(&p.module) {
            None
        } else {
            p.module.clone()
        };
        let api_version = if blank(&p.api_version) {
            None
        } else {
            p.api_version.clone()
        };

        // Empty query with no module filter: return the module map so an agent
        // can orient itself in one call.
        if p.query.trim().is_empty() && module.is_none() {
            return ok_json(&json!({
                "hint": "pass a query like \"create invoice\", or a module name to browse it. \
                         Modules prefixed `v3:` belong to API v3; pass api_version to filter.",
                "total_operations": operations::operation_count(),
                "modules": operations::modules(),
            }));
        }

        let hits = operations::search(&p.query, module.as_deref(), api_version.as_deref(), limit);
        if hits.is_empty() {
            return ok_json(&json!({
                "operations": [],
                "hint": format!(
                    "nothing matched. Try fewer words, or browse a module: {}",
                    operations::modules().keys().take(12).cloned().collect::<Vec<_>>().join(", ")
                ),
            }));
        }
        let listed: Vec<_> = hits
            .iter()
            .map(|o| {
                json!({
                    "operation_id": o.id,
                    "module": o.module,
                    "api_version": o.api_version,
                    "method": o.method,
                    "path": o.path,
                    "summary": o.summary,
                })
            })
            .collect();
        ok_json(&json!({
            "operations": listed,
            "count": listed.len(),
            "next_step": "call ghl_describe_operation with an operation_id for parameters",
        }))
    }

    #[tool(
        description = "Show the full call signature of an API operation: path and query \
                       parameters, request-body fields, required OAuth scopes. Read-only."
    )]
    async fn ghl_describe_operation(
        &self,
        Parameters(p): Parameters<DescribeOperationParams>,
    ) -> Result<String, ErrorData> {
        let op = operations::find(&p.operation_id).ok_or_else(|| {
            ErrorData::invalid_params(
                format!(
                    "unknown operation_id `{}` — find valid ids with ghl_search_operations",
                    p.operation_id
                ),
                None,
            )
        })?;
        ok_json(&json!({
            "operation_id": op.id,
            "module": op.module,
            "api_version": op.api_version,
            "method": op.method,
            "path": op.path,
            "summary": op.summary,
            "description": op.desc,
            "path_params": op.params.iter().filter(|q| q.location == "path").map(|q| json!({
                "name": q.name, "type": q.r#type, "required": q.required, "description": q.desc,
            })).collect::<Vec<_>>(),
            "query_params": op.params.iter().filter(|q| q.location == "query").map(|q| json!({
                "name": q.name, "type": q.r#type, "required": q.required, "description": q.desc,
            })).collect::<Vec<_>>(),
            "body": op.body.as_ref().map(|b| json!({
                "fields": b.fields, "required": b.required, "schema_ref": b.r#ref,
            })),
            "required_scopes": op.scopes,
            "next_step": "call ghl_execute_operation with this operation_id",
        }))
    }

    #[tool(
        description = "Execute any GoHighLevel API operation by id, with path params, query \
                       params, and JSON body. Covers the entire API — use ghl_search_operations \
                       and ghl_describe_operation first. Write operations (POST/PUT/PATCH) need \
                       the server started with --allow-destructive; DELETE is always gated."
    )]
    async fn ghl_execute_operation(
        &self,
        Parameters(p): Parameters<ExecuteOperationParams>,
    ) -> Result<String, ErrorData> {
        let op = operations::find(&p.operation_id).ok_or_else(|| {
            ErrorData::invalid_params(
                format!(
                    "unknown operation_id `{}` — find valid ids with ghl_search_operations",
                    p.operation_id
                ),
                None,
            )
        })?;

        // Anything that mutates data goes through the same gate as the typed
        // destructive tools — an agent must not be able to route around it.
        if op.method != "GET" && !self.allow_destructive {
            return Err(ErrorData::invalid_request(
                format!(
                    "`{}` is a {} (write) operation; restart ghl-mcp with --allow-destructive \
                     (or GHL_ALLOW_DESTRUCTIVE=true) to permit writes. Read-only GET operations \
                     work without it.",
                    op.id, op.method
                ),
                None,
            ));
        }

        // Substitute {placeholders}; refuse rather than send a malformed path.
        let mut path = op.path.clone();
        let supplied = p.path_params.unwrap_or_default();
        for param in op.params.iter().filter(|q| q.location == "path") {
            let placeholder = format!("{{{}}}", param.name);
            match supplied.get(&param.name) {
                Some(value) => {
                    let raw = value
                        .as_str()
                        .map(str::to_owned)
                        .unwrap_or_else(|| value.to_string().trim_matches('"').to_owned());
                    path = path.replace(&placeholder, &urlencode_segment(&raw));
                }
                None => {
                    return Err(ErrorData::invalid_params(
                        format!(
                            "missing path_params.{} for `{}` (path {})",
                            param.name, op.id, op.path
                        ),
                        None,
                    ))
                }
            }
        }
        if path.contains('{') {
            return Err(ErrorData::invalid_params(
                format!("unresolved path placeholders in `{path}` — see ghl_describe_operation"),
                None,
            ));
        }

        // Default locationId-style params to the configured location when the
        // caller omitted them, so simple calls just work.
        let mut query: Vec<(String, String)> = p
            .query
            .unwrap_or_default()
            .into_iter()
            .map(|(k, v)| {
                let s = v
                    .as_str()
                    .map(str::to_owned)
                    .unwrap_or_else(|| v.to_string().trim_matches('"').to_owned());
                (k, s)
            })
            .collect();
        let requires = |name: &str| {
            op.params
                .iter()
                .any(|q| q.location == "query" && q.name == name && q.required)
        };
        let missing = |q: &[(String, String)], name: &str| !q.iter().any(|(k, _)| k == name);

        if let Some(default_loc) = &self.default_location {
            for name in ["locationId", "location_id", "altId"] {
                if requires(name) && missing(&query, name) {
                    query.push((name.to_owned(), default_loc.clone()));
                }
            }
        }
        // `altId` is meaningless to the API without its companion discriminator,
        // and every spec that has one enumerates only "location".
        if requires("altType") && missing(&query, "altType") {
            query.push(("altType".to_owned(), "location".to_owned()));
        }

        let result = self
            .ghl
            .request_raw(
                &op.method,
                &path,
                &query,
                p.body.as_ref(),
                op.version.as_deref(),
            )
            .await
            .map_err(internal)?;
        ok_json(&result)
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
    version = "0.4.1",
    instructions = "Tools for working with a GoHighLevel (HighLevel) CRM account. Dedicated \
                    typed tools cover contacts, opportunities (pipeline deals), and locations. \
                    For anything else — invoices, calendars, payments, workflows, forms, \
                    products, social planner, custom objects, and every other module — use \
                    ghl_search_operations to find the endpoint, ghl_describe_operation to see \
                    its parameters, then ghl_execute_operation to call it; together these reach \
                    the entire API. If no default location is configured, call \
                    ghl_list_locations first and pass location_id explicitly. For opportunity \
                    work, call ghl_list_pipelines first to get pipeline and stage ids. Paginate \
                    with the returned next_cursor. Writes and deletions require the server to \
                    be started with --allow-destructive."
)]
impl ServerHandler for GhlServer {}
