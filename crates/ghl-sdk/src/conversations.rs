//! Conversations — search threads, read messages, and send SMS/email.
//!
//! Access via [`Ghl::conversations`](crate::Ghl::conversations). See the
//! [full conversations reference][ref] for all 29 v2 endpoints.
//!
//! | Method | Endpoint | Scope |
//! |---|---|---|
//! | [`ConversationsService::search`] | `GET /conversations/search` | `conversations.readonly` |
//! | [`ConversationsService::messages`] | `GET /conversations/{id}/messages` | `conversations/message.readonly` |
//! | [`ConversationsService::send_message`] | `POST /conversations/messages` | `conversations/message.write` |
//!
//! Channels accepted by [`SendMessage::message_type`]: `SMS`, `Email`,
//! `WhatsApp`, `IG`, `FB`, `Custom`, `Live_Chat`.
//!
//! # Examples
//!
//! ```no_run
//! # use ghl_sdk::{Ghl, conversations::SendMessage};
//! # async fn demo(ghl: Ghl, loc: &str, contact_id: String) -> Result<(), ghl_sdk::Error> {
//! // Find a thread, then read it (newest message first)
//! let threads = ghl.conversations().search(loc, Some("ada"), 20).await?;
//! let msgs = ghl.conversations().messages(&threads.conversations[0].id, 50).await?;
//! for m in &msgs.messages {
//!     println!("[{:?}] {:?}", m.direction, m.body);
//! }
//!
//! // Send an SMS
//! ghl.conversations().send_message(SendMessage {
//!     message_type: "SMS".into(),
//!     contact_id,
//!     message: Some("Thanks for reaching out!".into()),
//!     ..Default::default()
//! }).await?;
//! # Ok(()) }
//! ```
//!
//! Email needs a `subject` and usually `html`:
//!
//! ```no_run
//! # use ghl_sdk::{Ghl, conversations::SendMessage};
//! # async fn demo(ghl: Ghl, contact_id: String) -> Result<(), ghl_sdk::Error> {
//! ghl.conversations().send_message(SendMessage {
//!     message_type: "Email".into(),
//!     contact_id,
//!     subject: Some("Your August invoice".into()),
//!     html: Some("<p>Attached.</p>".into()),
//!     ..Default::default()
//! }).await?;
//! # Ok(()) }
//! ```
//!
//! [ref]: https://github.com/Shahroz/ghl-rs/blob/main/docs/api/conversations.md

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::client::Ghl;
use crate::error::Result;

/// A conversation thread with a contact.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)] // fields mirror the API wire format 1:1
pub struct Conversation {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_message_body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_message_date: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_message_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unread_count: Option<i64>,
    /// Any fields this SDK doesn't model yet.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// A single message within a conversation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)] // fields mirror the API wire format 1:1
pub struct Message {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// `SMS`, `Email`, `WhatsApp`, `IG`, `FB`, `Live_Chat`, `CALL`, …
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// `inbound` or `outbound`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_added: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// Any fields this SDK doesn't model yet.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Payload for [`ConversationsService::send_message`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)] // fields mirror the API wire format 1:1
pub struct SendMessage {
    /// Channel: `SMS`, `Email`, `WhatsApp`, `IG`, `FB`, `Custom`, `Live_Chat`.
    #[serde(rename = "type")]
    pub message_type: String,
    pub contact_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Email subject (email only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub attachments: Vec<String>,
}

/// Result of sending a message.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub struct SendMessageResult {
    #[serde(default)]
    pub conversation_id: Option<String>,
    #[serde(default)]
    pub message_id: Option<String>,
    /// Any fields this SDK doesn't model yet.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Deserialize)]
struct ConversationSearch {
    #[serde(default)]
    conversations: Vec<Conversation>,
    #[serde(default)]
    total: Option<i64>,
}

#[derive(Deserialize)]
struct MessagesEnvelope {
    messages: MessagesInner,
}

#[derive(Deserialize)]
struct MessagesInner {
    #[serde(default)]
    messages: Vec<Message>,
    #[serde(default, rename = "nextPage")]
    next_page: Option<bool>,
}

/// One page of conversations.
#[derive(Debug, Clone)]
pub struct ConversationPage {
    /// The conversations on this page.
    pub conversations: Vec<Conversation>,
    /// Total matches, when the API reports it.
    pub total: Option<i64>,
}

/// One page of messages within a conversation.
#[derive(Debug, Clone)]
pub struct MessagePage {
    /// Messages, newest first.
    pub messages: Vec<Message>,
    /// Whether more pages exist.
    pub next_page: Option<bool>,
}

/// Access to the Conversations API. Obtained via [`Ghl::conversations`].
pub struct ConversationsService {
    client: Ghl,
}

impl ConversationsService {
    pub(crate) fn new(client: Ghl) -> Self {
        Self { client }
    }

    /// `GET /conversations/search` — find threads in a location.
    pub async fn search(
        &self,
        location_id: &str,
        query: Option<&str>,
        limit: u32,
    ) -> Result<ConversationPage> {
        let mut params: Vec<(String, String)> = vec![
            ("locationId".into(), location_id.to_owned()),
            ("limit".into(), limit.clamp(1, 100).to_string()),
        ];
        if let Some(q) = query {
            params.push(("query".into(), q.to_owned()));
        }
        let result: ConversationSearch = self
            .client
            .send(Method::GET, "/conversations/search", &params, None::<&()>)
            .await?;
        Ok(ConversationPage {
            conversations: result.conversations,
            total: result.total,
        })
    }

    /// `GET /conversations/{id}/messages` — messages in a thread, newest first.
    pub async fn messages(&self, conversation_id: &str, limit: u32) -> Result<MessagePage> {
        let envelope: MessagesEnvelope = self
            .client
            .send(
                Method::GET,
                &format!("/conversations/{conversation_id}/messages"),
                &[("limit".into(), limit.clamp(1, 100).to_string())],
                None::<&()>,
            )
            .await?;
        Ok(MessagePage {
            messages: envelope.messages.messages,
            next_page: envelope.messages.next_page,
        })
    }

    /// `POST /conversations/messages` — send an SMS, email, or channel message.
    pub async fn send_message(&self, message: SendMessage) -> Result<SendMessageResult> {
        self.client
            .send(Method::POST, "/conversations/messages", &[], Some(&message))
            .await
    }
}
