# `conversations`

**29** operations / **44** models in API v2 · **25** operations / **44** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `conversations` cargo feature on `ghl-sdk`, then call any of the 54 generated methods on `ghl.conversations()` (v2) or `ghl.v3().conversations()` (v3):

```toml
ghl-sdk = { version = "0.5", features = ["conversations"] }
```

This module also has hand-written ergonomic helpers on the same `ghl.conversations()`: `search()`, `messages()`, `send_message()` (envelope unwrapping, paginated `Stream`s).

MCP tools: `ghl_search_conversations`, `ghl_get_messages`, `ghl_send_message`.


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `POST` | `/conversations/` | Create Conversation | `create_conversation()` | `conversations.post_conversations` |
| `GET` | `/conversations/locations/{locationId}/messages/{messageId}/transcription` | Get transcription by Message ID | `get_transcription_by_message_id()` | `conversations.get_conversations_locations_by_locationId_messages_by_messageId_transcription` |
| `GET` | `/conversations/locations/{locationId}/messages/{messageId}/transcription/download` | Download transcription by Message ID | `download_transcription_by_message_id()` | `conversations.get_conversations_locations_by_locationId_messages_by_messageId_transcription_download` |
| `POST` | `/conversations/messages` | Send a new message | `send_a_new_message()` | `conversations.post_conversations_messages` |
| `DELETE` | `/conversations/messages/email/{emailMessageId}/schedule` | Cancel a scheduled email message. | `cancel_a_scheduled_email_message()` | `conversations.delete_conversations_messages_email_by_emailMessageId_schedule` |
| `GET` | `/conversations/messages/email/{id}` | Get email by Id | `get_email_by_id()` | `conversations.get_conversations_messages_email_by_id` |
| `GET` | `/conversations/messages/export` | Export messages by location ID | `export_messages_by_location_id()` | `conversations.get_conversations_messages_export` |
| `POST` | `/conversations/messages/inbound` | Add an inbound message | `add_an_inbound_message()` | `conversations.post_conversations_messages_inbound` |
| `POST` | `/conversations/messages/outbound` | Add an external outbound call | `add_an_external_outbound_call()` | `conversations.post_conversations_messages_outbound` |
| `POST` | `/conversations/messages/review-reply` | Send a review reply to Google My Business | `send_a_review_reply_to_google_my_business()` | `conversations.post_conversations_messages_review_reply` |
| `POST` | `/conversations/messages/upload` | Upload file attachments | `upload_file_attachments()` | `conversations.post_conversations_messages_upload` |
| `POST` | `/conversations/messages/upload/complete` | Complete file upload | `complete_file_upload()` | `conversations.post_conversations_messages_upload_complete` |
| `POST` | `/conversations/messages/upload/initiate` | Initiate file upload to GCS | `initiate_file_upload_to_gcs()` | `conversations.post_conversations_messages_upload_initiate` |
| `GET` | `/conversations/messages/{id}` | Get message by message id | `get_message_by_message_id()` | `conversations.get_conversations_messages_by_id` |
| `PUT` | `/conversations/messages/{messageId}/attachments` | Add message attachments | `add_message_attachments()` | `conversations.put_conversations_messages_by_messageId_attachments` |
| `GET` | `/conversations/messages/{messageId}/locations/{locationId}/recording` | Get Recording by Message ID | `get_recording_by_message_id()` | `conversations.get_conversations_messages_by_messageId_locations_by_locationId_recording` |
| `DELETE` | `/conversations/messages/{messageId}/schedule` | Cancel a scheduled message. | `cancel_a_scheduled_message()` | `conversations.delete_conversations_messages_by_messageId_schedule` |
| `PUT` | `/conversations/messages/{messageId}/status` | Update message status | `update_message_status()` | `conversations.put_conversations_messages_by_messageId_status` |
| `GET` | `/conversations/preferences/custom-subtypes` | Get All Custom Subtypes | `get_all_custom_subtypes()` | `conversations.get_conversations_preferences_custom_subtypes` |
| `POST` | `/conversations/preferences/custom-subtypes` | Create Custom Subtype | `create_custom_subtype()` | `conversations.post_conversations_preferences_custom_subtypes` |
| `PUT` | `/conversations/preferences/custom-subtypes/{id}` | Update Custom Subtype | `update_custom_subtype()` | `conversations.put_conversations_preferences_custom_subtypes_by_id` |
| `GET` | `/conversations/preferences/unsubscriptions/status` | Get Contact Unsubscription Status | `get_contact_unsubscription_status()` | `conversations.get_conversations_preferences_unsubscriptions_status` |
| `POST` | `/conversations/preferences/unsubscriptions/user-change` | User Subscription Change | `user_subscription_change()` | `conversations.post_conversations_preferences_unsubscriptions_user_change` |
| `POST` | `/conversations/providers/live-chat/typing` | Agent/Ai-Bot is typing a message indicator for live chat | `agent_ai_bot_is_typing_a_message_indicator_for_live_chat()` | `conversations.post_conversations_providers_live_chat_typing` |
| `GET` | `/conversations/search` | Search Conversations | `search_conversations()` | `conversations.get_conversations_search` |
| `DELETE` | `/conversations/{conversationId}` | Delete Conversation | `delete_conversation()` | `conversations.delete_conversations_by_conversationId` |
| `GET` | `/conversations/{conversationId}` | Get Conversation | `get_conversation()` | `conversations.get_conversations_by_conversationId` |
| `PUT` | `/conversations/{conversationId}` | Update Conversation | `update_conversation()` | `conversations.put_conversations_by_conversationId` |
| `GET` | `/conversations/{conversationId}/messages` | Get messages by conversation id | `get_messages_by_conversation_id()` | `conversations.get_conversations_by_conversationId_messages` |

### Endpoint details — v2

#### `POST /conversations/`

**Create Conversation**

Creates a new conversation with the data provided

Operation id: `conversations.post_conversations` · `Version: 2021-04-15` · Scopes: `conversations.write`

*Request body*: [`CreateConversationDto`](#createconversationdto)

*Response*: [`CreateConversationSuccessResponse`](#createconversationsuccessresponse)

*Rust*:

```rust,ignore
let out = ghl.conversations().create_conversation(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.post_conversations",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /conversations/locations/{locationId}/messages/{messageId}/transcription`

**Get transcription by Message ID**

Get the recording transcription for a message by passing the message id

Operation id: `conversations.get_conversations_locations_by_locationId_messages_by_messageId_transcription` · `Version: 2021-04-15` · Scopes: `conversations/message.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID as string |
| `messageId` | string | **yes** | Message ID as string |

*Response*: [`GetMessageTranscriptionResponseDto`](#getmessagetranscriptionresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversations().get_transcription_by_message_id(&locationId, &messageId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.get_conversations_locations_by_locationId_messages_by_messageId_transcription",
    "path_params": {
      "locationId": "<locationId>",
      "messageId": "<messageId>"
    }
  }
}
```

</details>

#### `GET /conversations/locations/{locationId}/messages/{messageId}/transcription/download`

**Download transcription by Message ID**

Download the recording transcription for a message by passing the message id

Operation id: `conversations.get_conversations_locations_by_locationId_messages_by_messageId_transcription_download` · `Version: 2021-04-15` · Scopes: `conversations/message.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID as string |
| `messageId` | string | **yes** | Message ID as string |

*Rust*:

```rust,ignore
let out = ghl.conversations().download_transcription_by_message_id(&locationId, &messageId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.get_conversations_locations_by_locationId_messages_by_messageId_transcription_download",
    "path_params": {
      "locationId": "<locationId>",
      "messageId": "<messageId>"
    }
  }
}
```

</details>

#### `POST /conversations/messages`

**Send a new message**

Post the necessary fields for the API to send a new message.

Operation id: `conversations.post_conversations_messages` · `Version: 2021-04-15` · Scopes: `conversations/message.write`

*Request body*: [`SendMessageBodyDto`](#sendmessagebodydto)

*Response*: [`SendMessageResponseDto`](#sendmessageresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversations().send_a_new_message(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.post_conversations_messages",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /conversations/messages/email/{emailMessageId}/schedule`

**Cancel a scheduled email message.**

Post the messageId for the API to delete a scheduled email message.

Operation id: `conversations.delete_conversations_messages_email_by_emailMessageId_schedule`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `emailMessageId` | string | **yes** | Email Message Id |

*Response*: [`CancelScheduledResponseDto`](#cancelscheduledresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversations().cancel_a_scheduled_email_message(&emailMessageId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.delete_conversations_messages_email_by_emailMessageId_schedule",
    "path_params": {
      "emailMessageId": "<emailMessageId>"
    }
  }
}
```

</details>

#### `GET /conversations/messages/email/{id}`

**Get email by Id**

Operation id: `conversations.get_conversations_messages_email_by_id`

*Response*: [`GetEmailMessageResponseDto`](#getemailmessageresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversations().get_email_by_id().await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.get_conversations_messages_email_by_id"
  }
}
```

</details>

#### `GET /conversations/messages/export`

**Export messages by location ID**

Export messages for a specific location with cursor-based pagination support. Response includes messageType (string), source, and subType fields. The channel parameter is optional - if not provided, all non-email message types will be returned including activity messages (opportunity updates, appointments, etc.).

Operation id: `conversations.get_conversations_messages_export` · `Version: 2021-04-15` · Scopes: `conversations/message.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID to filter messages by |
| `limit` | number | no | Number of messages to return per page |
| `cursor` | string | no | Cursor for pagination. Pass the nextCursor from previous response to get next page. |
| `sortBy` | enum: `createdAt`, `updatedAt` | no | Field to sort by |
| `sortOrder` | enum: `asc`, `desc` | no | Sort order |
| `conversationId` | string | no | Filter messages by conversation ID |
| `contactId` | string | no | Filter messages by contact ID |
| `channel` | enum: `Call`, `SMS`, `Email`, `WhatsApp`, `Instagram`, `Facebook` | no | Filter by message channel. If not provided, all non-email message types will be returned including activity messages (opportunity updates, appointments, etc.) |
| `startDate` | string | no | Start date to filter messages by |
| `endDate` | string | no | End date to filter messages by |

*Response*: [`ExportMessagesResponseDto`](#exportmessagesresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::conversations::ExportMessagesByLocationIdParams;

let params = ExportMessagesByLocationIdParams::new("locationId");
let out = ghl.conversations().export_messages_by_location_id(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.get_conversations_messages_export",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /conversations/messages/inbound`

**Add an inbound message**

Post the necessary fields for the API to add a new inbound message.

Operation id: `conversations.post_conversations_messages_inbound` · `Version: 2021-04-15` · Scopes: `conversations/message.write`

*Request body*: [`ProcessMessageBodyDto`](#processmessagebodydto)

*Response*: [`ProcessMessageResponseDto`](#processmessageresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversations().add_an_inbound_message(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.post_conversations_messages_inbound",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /conversations/messages/outbound`

**Add an external outbound call**

Post the necessary fields for the API to add a new outbound call.

Operation id: `conversations.post_conversations_messages_outbound` · `Version: 2021-04-15` · Scopes: `conversations/message.write`

*Request body*: [`ProcessOutboundMessageBodyDto`](#processoutboundmessagebodydto)

*Response*: [`ProcessMessageResponseDto`](#processmessageresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversations().add_an_external_outbound_call(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.post_conversations_messages_outbound",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /conversations/messages/review-reply`

**Send a review reply to Google My Business**

Post a reply to a customer review on Google My Business

Operation id: `conversations.post_conversations_messages_review_reply` · `Version: 2021-04-15` · Scopes: `conversations/message.write`

*Request body*: [`SendReviewReplyDto`](#sendreviewreplydto)

*Response*: [`SendMessageResponseDto`](#sendmessageresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversations().send_a_review_reply_to_google_my_business(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.post_conversations_messages_review_reply",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /conversations/messages/upload`

**Upload file attachments**

Post the necessary fields for the API to upload files. The files need to be a buffer with the key "fileAttachment". The allowed file types are: <ul><li>JPG</li><li>JPEG</li><li>PNG</li><li>MP4</li><li>MPEG</li><li>ZIP</li><li>RAR</li><li>PDF</li><li>DOC</li><li>DOCX</li><li>TXT</li><li>MP3</li><li>WAV</li></ul> The API will return an object with the URLs

Operation id: `conversations.post_conversations_messages_upload` · `Version: 2021-04-15` · Scopes: `conversations/message.write`

*Response*: [`UploadFilesResponseDto`](#uploadfilesresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversations().upload_file_attachments().await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.post_conversations_messages_upload"
  }
}
```

</details>

#### `POST /conversations/messages/upload/complete`

**Complete file upload**

Validates the uploaded file in GCS and returns the public URL. Call this endpoint after successfully uploading the file to the signed URL.

Operation id: `conversations.post_conversations_messages_upload_complete` · `Version: 2021-04-15` · Scopes: `conversations/message.write`

*Request body*: [`CompleteFileUploadDto`](#completefileuploaddto)

*Response*: [`CompleteFileUploadResponseDto`](#completefileuploadresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversations().complete_file_upload(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.post_conversations_messages_upload_complete",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /conversations/messages/upload/initiate`

**Initiate file upload to GCS**

Generates a signed URL for direct file upload to Google Cloud Storage. Returns a signed URL valid for 15 minutes. Upload file via PUT request, then call /complete to finalize.

Operation id: `conversations.post_conversations_messages_upload_initiate` · `Version: 2021-04-15` · Scopes: `conversations/message.write`

*Request body*: [`InitiateFileUploadDto`](#initiatefileuploaddto)

*Response*: [`InitiateFileUploadResponseDto`](#initiatefileuploadresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversations().initiate_file_upload_to_gcs(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.post_conversations_messages_upload_initiate",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /conversations/messages/{id}`

**Get message by message id**

Get message by message id.

Operation id: `conversations.get_conversations_messages_by_id` · `Version: 2021-04-15` · Scopes: `conversations/message.readonly`

*Response*: [`GetMessageResponseDto`](#getmessageresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversations().get_message_by_message_id().await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.get_conversations_messages_by_id"
  }
}
```

</details>

#### `PUT /conversations/messages/{messageId}/attachments`

**Add message attachments**

Set attachments on an existing message (replaces existing). Maximum 5 URLs. Supported for TYPE_CUSTOM_CALL (34) and TYPE_CALL (1) with subType EXTERNAL_CALL.

Operation id: `conversations.put_conversations_messages_by_messageId_attachments` · `Version: 2021-04-15` · Scopes: `conversations/message.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `messageId` | string | **yes** | Message Id |

*Request body*: [`AddMessageAttachmentsDto`](#addmessageattachmentsdto)

*Rust*:

```rust,ignore
let out = ghl.conversations().add_message_attachments(&messageId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.put_conversations_messages_by_messageId_attachments",
    "path_params": {
      "messageId": "<messageId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /conversations/messages/{messageId}/locations/{locationId}/recording`

**Get Recording by Message ID**

Get the recording for a message by passing the message id

Operation id: `conversations.get_conversations_messages_by_messageId_locations_by_locationId_recording` · `Version: 2021-04-15` · Scopes: `conversations/message.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID as string |
| `messageId` | string | **yes** | Message ID as string |

*Rust*:

```rust,ignore
let out = ghl.conversations().get_recording_by_message_id(&locationId, &messageId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.get_conversations_messages_by_messageId_locations_by_locationId_recording",
    "path_params": {
      "locationId": "<locationId>",
      "messageId": "<messageId>"
    }
  }
}
```

</details>

#### `DELETE /conversations/messages/{messageId}/schedule`

**Cancel a scheduled message.**

Post the messageId for the API to delete a scheduled message.

Operation id: `conversations.delete_conversations_messages_by_messageId_schedule` · `Version: 2021-04-15` · Scopes: `conversations/message.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `messageId` | string | **yes** | Message Id |

*Response*: [`CancelScheduledResponseDto`](#cancelscheduledresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversations().cancel_a_scheduled_message(&messageId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.delete_conversations_messages_by_messageId_schedule",
    "path_params": {
      "messageId": "<messageId>"
    }
  }
}
```

</details>

#### `PUT /conversations/messages/{messageId}/status`

**Update message status**

Post the necessary fields for the API to update message status.

Operation id: `conversations.put_conversations_messages_by_messageId_status` · `Version: 2021-04-15` · Scopes: `conversations/message.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `messageId` | string | **yes** | Message Id |

*Request body*: [`UpdateMessageStatusDto`](#updatemessagestatusdto)

*Response*: [`SendMessageResponseDto`](#sendmessageresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversations().update_message_status(&messageId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.put_conversations_messages_by_messageId_status",
    "path_params": {
      "messageId": "<messageId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /conversations/preferences/custom-subtypes`

**Get All Custom Subtypes**

Get all custom subtypes for a location

Operation id: `conversations.get_conversations_preferences_custom_subtypes` · `Version: 2021-04-15`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Rust*:

```rust,ignore
use ghl_sdk::services::conversations::GetAllCustomSubtypesParams;

let params = GetAllCustomSubtypesParams::new("locationId");
let out = ghl.conversations().get_all_custom_subtypes(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.get_conversations_preferences_custom_subtypes",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /conversations/preferences/custom-subtypes`

**Create Custom Subtype**

Create a new custom subtype for a location. Requires agency or account admin role.

Operation id: `conversations.post_conversations_preferences_custom_subtypes` · `Version: 2021-04-15`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`CreateCustomSubtypeDto`](#createcustomsubtypedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::conversations::CreateCustomSubtypeParams;

let params = CreateCustomSubtypeParams::new("locationId");
let out = ghl.conversations().create_custom_subtype(&params, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.post_conversations_preferences_custom_subtypes",
    "query": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /conversations/preferences/custom-subtypes/{id}`

**Update Custom Subtype**

Update or archive a custom subtype. Requires agency or account admin role.

Operation id: `conversations.put_conversations_preferences_custom_subtypes_by_id` · `Version: 2021-04-15`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Custom Subtype Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`UpdateCustomSubtypeDto`](#updatecustomsubtypedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::conversations::UpdateCustomSubtypeParams;

let params = UpdateCustomSubtypeParams::new("locationId");
let out = ghl.conversations().update_custom_subtype(&id, &params, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.put_conversations_preferences_custom_subtypes_by_id",
    "path_params": {
      "id": "<id>"
    },
    "query": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /conversations/preferences/unsubscriptions/status`

**Get Contact Unsubscription Status**

Get all subscription statuses for a contact (all emails or specific email)

Operation id: `conversations.get_conversations_preferences_unsubscriptions_status` · `Version: 2021-04-15`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `contactId` | string | **yes** | Contact Id |
| `email` | string | no | Email address (optional - if not provided, gets all emails for contact) |

*Rust*:

```rust,ignore
use ghl_sdk::services::conversations::GetContactUnsubscriptionStatusParams;

let params = GetContactUnsubscriptionStatusParams::new("locationId", "contactId");
let out = ghl.conversations().get_contact_unsubscription_status(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.get_conversations_preferences_unsubscriptions_status",
    "query": {
      "locationId": "<locationId>",
      "contactId": "<contactId>"
    }
  }
}
```

</details>

#### `POST /conversations/preferences/unsubscriptions/user-change`

**User Subscription Change**

Process subscription change initiated by a user (admin/agent). Supports individual custom subscription changes and resub all functionality. Legal forms are automatically created for user-initiated resubscribe actions on custom subscriptions.

Operation id: `conversations.post_conversations_preferences_unsubscriptions_user_change` · `Version: 2021-04-15`

*Request body*: [`UserSubscriptionChangeDto`](#usersubscriptionchangedto)

*Rust*:

```rust,ignore
let out = ghl.conversations().user_subscription_change(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.post_conversations_preferences_unsubscriptions_user_change",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /conversations/providers/live-chat/typing`

**Agent/Ai-Bot is typing a message indicator for live chat**

Agent/AI-Bot will call this when they are typing a message in live chat message

Operation id: `conversations.post_conversations_providers_live_chat_typing` · `Version: 2021-04-15` · Scopes: `conversations/livechat.write`

*Request body*: [`UserTypingBody`](#usertypingbody)

*Response*: [`CreateLiveChatMessageFeedbackResponse`](#createlivechatmessagefeedbackresponse)

*Rust*:

```rust,ignore
let out = ghl.conversations().agent_ai_bot_is_typing_a_message_indicator_for_live_chat(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.post_conversations_providers_live_chat_typing",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /conversations/search`

**Search Conversations**

Returns a list of all conversations matching the search criteria along with the sort and filter options selected.

Operation id: `conversations.get_conversations_search` · `Version: 2021-04-15` · Scopes: `conversations.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `contactId` | string | no | Contact Id |
| `assignedTo` | string | no | User IDs that conversations are assigned to. Multiple IDs can be provided as comma-separated values. Use "unassigned" to fetch conversations not assigned to any… |
| `followers` | string | no | User IDs of followers to filter conversations by. Multiple IDs can be provided as comma-separated values. |
| `mentions` | string | no | User Id of the mention. Multiple values are comma separated. |
| `query` | string | no | Search paramater as a string |
| `sort` | enum: `asc`, `desc` | no | Sort paramater - asc or desc |
| `startAfterDate` | any | no | Search to begin after the specified date - should contain the sort value of the last document |
| `id` | string | no | Id of the conversation |
| `limit` | number | no | Limit of conversations - Default is 20 |
| `lastMessageType` | enum (42 values — see [shared enums](shared-enums.md)) | no | Type of the last message in the conversation as a string |
| `lastMessageAction` | enum: `automated`, `manual` | no | Action of the last outbound message in the conversation as string. |
| `lastMessageDirection` | enum: `inbound`, `outbound` | no | Direction of the last message in the conversation as string. |
| `status` | enum: `all`, `read`, `unread`, `starred`, `recents` | no | The status of the conversation to be filtered - all, read, unread, starred |
| `sortBy` | enum: `last_manual_message_date`, `last_message_date`, `score_profile`, `overdue_at`, `due_at` | no | The sorting of the conversation to be filtered as - manual messages or all messages |
| `sortScoreProfile` | string | no | Id of score profile on which sortBy.ScoreProfile should sort on |
| `scoreProfile` | string | no | Id of score profile on which conversations should get filtered out, works with scoreProfileMin & scoreProfileMax |
| `scoreProfileMin` | number | no | Minimum value for score |
| `scoreProfileMax` | number | no | Maximum value for score |
| `startDate` | number | no | Start date filter for dateAdded field (Unix timestamp in milliseconds) |
| `endDate` | number | no | End date filter for dateAdded field (Unix timestamp in milliseconds) |

*Response*: [`SendConversationResponseDto`](#sendconversationresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::conversations::SearchConversationsParams;

let params = SearchConversationsParams::new("locationId");
let out = ghl.conversations().search_conversations(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.get_conversations_search",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `DELETE /conversations/{conversationId}`

**Delete Conversation**

Delete the conversation details based on the conversation ID

Operation id: `conversations.delete_conversations_by_conversationId` · `Version: 2021-04-15` · Scopes: `conversations.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `conversationId` | string | **yes** | Conversation ID as string |

*Response*: [`DeleteConversationSuccessfulResponse`](#deleteconversationsuccessfulresponse)

*Rust*:

```rust,ignore
let out = ghl.conversations().delete_conversation(&conversationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.delete_conversations_by_conversationId",
    "path_params": {
      "conversationId": "<conversationId>"
    }
  }
}
```

</details>

#### `GET /conversations/{conversationId}`

**Get Conversation**

Get the conversation details based on the conversation ID

Operation id: `conversations.get_conversations_by_conversationId` · `Version: 2021-04-15` · Scopes: `conversations.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `conversationId` | string | **yes** | Conversation ID as string |

*Response*: [`GetConversationByIdResponse`](#getconversationbyidresponse)

*Rust*:

```rust,ignore
let out = ghl.conversations().get_conversation(&conversationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.get_conversations_by_conversationId",
    "path_params": {
      "conversationId": "<conversationId>"
    }
  }
}
```

</details>

#### `PUT /conversations/{conversationId}`

**Update Conversation**

Update the conversation details based on the conversation ID

Operation id: `conversations.put_conversations_by_conversationId` · `Version: 2021-04-15` · Scopes: `conversations.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `conversationId` | string | **yes** | Conversation ID as string |

*Request body*: [`UpdateConversationDto`](#updateconversationdto)

*Response*: [`GetConversationSuccessfulResponse`](#getconversationsuccessfulresponse)

*Rust*:

```rust,ignore
let out = ghl.conversations().update_conversation(&conversationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.put_conversations_by_conversationId",
    "path_params": {
      "conversationId": "<conversationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /conversations/{conversationId}/messages`

**Get messages by conversation id**

Get messages by conversation id.

Operation id: `conversations.get_conversations_by_conversationId_messages` · `Version: 2021-04-15` · Scopes: `conversations/message.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `conversationId` | string | **yes** | Conversation ID as string |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `lastMessageId` | string | no | Message ID of the last message in the list as a string |
| `limit` | number | no | Number of messages to be fetched from the conversation. Default limit is 20 |
| `type` | enum: `TYPE_CALL`, `TYPE_SMS`, `TYPE_RCS`, `TYPE_EMAIL`, `TYPE_FACEBOOK`, `TYPE_GMB`, `TYPE_INSTAGRAM`, `TYPE_WHATSAPP`, `TYPE_ACTIVITY_APPOINTMENT`, `TYPE_ACTIVITY_CONTACT`, `TYPE_ACTIVITY_INVOICE`, `TYPE_ACTIVITY_PAYMENT`, `TYPE_ACTIVITY_OPPORTUNITY`, `TYPE_LIVE_CHAT`, `TYPE_INTERNAL_COMMENTS`, `TYPE_ACTIVITY_EMPLOYEE_ACTION_LOG`, `TYPE_TIKTOK`, `TYPE_ACTIVITY_WHATSAPP`, `TYPE_FORM_SUBMISSION` | no | Types of message to fetched separated with comma |

*Response*: [`GetMessagesByConversationResponseDto`](#getmessagesbyconversationresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::conversations::GetMessagesByConversationIdParams;

let params = GetMessagesByConversationIdParams::new();
let out = ghl.conversations().get_messages_by_conversation_id(&conversationId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversations.get_conversations_by_conversationId_messages",
    "path_params": {
      "conversationId": "<conversationId>"
    }
  }
}
```

</details>

## Endpoints — API v3

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `POST` | `/conversations/` | Create Conversation | `create_conversation()` | `v3:conversations.post_conversations` |
| `GET` | `/conversations/locations/{locationId}/messages/{messageId}/transcription` | Get transcription by Message ID | `get_transcription_by_message_id()` | `v3:conversations.get_conversations_locations_by_locationId_messages_by_messageId_transcription` |
| `GET` | `/conversations/locations/{locationId}/messages/{messageId}/transcription/download` | Download transcription by Message ID | `download_transcription_by_message_id()` | `v3:conversations.get_conversations_locations_by_locationId_messages_by_messageId_transcription_download` |
| `POST` | `/conversations/messages` | Send a new message | `send_a_new_message()` | `v3:conversations.post_conversations_messages` |
| `DELETE` | `/conversations/messages/email/{emailMessageId}/schedule` | Cancel a scheduled email message. | `cancel_a_scheduled_email_message()` | `v3:conversations.delete_conversations_messages_email_by_emailMessageId_schedule` |
| `GET` | `/conversations/messages/email/{id}` | Get email by Id | `get_email_by_id()` | `v3:conversations.get_conversations_messages_email_by_id` |
| `PUT` | `/conversations/messages/email/{id}/status` | Update email message status | `update_email_message_status()` | `v3:conversations.put_conversations_messages_email_by_id_status` |
| `GET` | `/conversations/messages/export` | Export messages by location ID | `export_messages_by_location_id()` | `v3:conversations.get_conversations_messages_export` |
| `POST` | `/conversations/messages/inbound` | Add an inbound message | `add_an_inbound_message()` | `v3:conversations.post_conversations_messages_inbound` |
| `POST` | `/conversations/messages/outbound` | Add an external outbound call | `add_an_external_outbound_call()` | `v3:conversations.post_conversations_messages_outbound` |
| `POST` | `/conversations/messages/review-reply` | Send a review reply to Google My Business | `send_a_review_reply_to_google_my_business()` | `v3:conversations.post_conversations_messages_review_reply` |
| `POST` | `/conversations/messages/upload` | Upload file attachments | `upload_file_attachments()` | `v3:conversations.post_conversations_messages_upload` |
| `POST` | `/conversations/messages/upload/complete` | Complete file upload | `complete_file_upload()` | `v3:conversations.post_conversations_messages_upload_complete` |
| `POST` | `/conversations/messages/upload/initiate` | Initiate file upload to GCS | `initiate_file_upload_to_gcs()` | `v3:conversations.post_conversations_messages_upload_initiate` |
| `GET` | `/conversations/messages/{id}` | Get message by message id | `get_message_by_message_id()` | `v3:conversations.get_conversations_messages_by_id` |
| `PUT` | `/conversations/messages/{messageId}/attachments` | Add message attachments | `add_message_attachments()` | `v3:conversations.put_conversations_messages_by_messageId_attachments` |
| `GET` | `/conversations/messages/{messageId}/locations/{locationId}/recording` | Get Recording by Message ID | `get_recording_by_message_id()` | `v3:conversations.get_conversations_messages_by_messageId_locations_by_locationId_recording` |
| `DELETE` | `/conversations/messages/{messageId}/schedule` | Cancel a scheduled message. | `cancel_a_scheduled_message()` | `v3:conversations.delete_conversations_messages_by_messageId_schedule` |
| `PUT` | `/conversations/messages/{messageId}/status` | Update message status | `update_message_status()` | `v3:conversations.put_conversations_messages_by_messageId_status` |
| `POST` | `/conversations/providers/live-chat/typing` | Agent/Ai-Bot is typing a message indicator for live chat | `agent_ai_bot_is_typing_a_message_indicator_for_live_chat()` | `v3:conversations.post_conversations_providers_live_chat_typing` |
| `GET` | `/conversations/search` | Search Conversations | `search_conversations()` | `v3:conversations.get_conversations_search` |
| `DELETE` | `/conversations/{conversationId}` | Delete Conversation | `delete_conversation()` | `v3:conversations.delete_conversations_by_conversationId` |
| `GET` | `/conversations/{conversationId}` | Get Conversation | `get_conversation()` | `v3:conversations.get_conversations_by_conversationId` |
| `PUT` | `/conversations/{conversationId}` | Update Conversation | `update_conversation()` | `v3:conversations.put_conversations_by_conversationId` |
| `GET` | `/conversations/{conversationId}/messages` | Get messages by conversation id | `get_messages_by_conversation_id()` | `v3:conversations.get_conversations_by_conversationId_messages` |

### Endpoint details — v3

#### `POST /conversations/`

**Create Conversation**

Creates a new conversation with the data provided

Operation id: `v3:conversations.post_conversations` · `Version: v3` · Scopes: `conversations.write`

*Request body*: [`CreateConversationDto`](#createconversationdto)

*Response*: [`CreateConversationSuccessResponse`](#createconversationsuccessresponse)

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().create_conversation(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.post_conversations",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /conversations/locations/{locationId}/messages/{messageId}/transcription`

**Get transcription by Message ID**

Get the recording transcription for a message by passing the message id

Operation id: `v3:conversations.get_conversations_locations_by_locationId_messages_by_messageId_transcription` · `Version: v3` · Scopes: `conversations/message.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID as string |
| `messageId` | string | **yes** | Message ID as string |

*Response*: [`GetMessageTranscriptionResponseDto`](#getmessagetranscriptionresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().get_transcription_by_message_id(&locationId, &messageId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.get_conversations_locations_by_locationId_messages_by_messageId_transcription",
    "path_params": {
      "locationId": "<locationId>",
      "messageId": "<messageId>"
    }
  }
}
```

</details>

#### `GET /conversations/locations/{locationId}/messages/{messageId}/transcription/download`

**Download transcription by Message ID**

Download the recording transcription for a message by passing the message id

Operation id: `v3:conversations.get_conversations_locations_by_locationId_messages_by_messageId_transcription_download` · `Version: v3` · Scopes: `conversations/message.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID as string |
| `messageId` | string | **yes** | Message ID as string |

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().download_transcription_by_message_id(&locationId, &messageId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.get_conversations_locations_by_locationId_messages_by_messageId_transcription_download",
    "path_params": {
      "locationId": "<locationId>",
      "messageId": "<messageId>"
    }
  }
}
```

</details>

#### `POST /conversations/messages`

**Send a new message**

Post the necessary fields for the API to send a new message.

Operation id: `v3:conversations.post_conversations_messages` · `Version: v3` · Scopes: `conversations/message.write`

*Request body*: [`SendMessageBodyDto`](#sendmessagebodydto)

*Response*: [`SendMessageResponseDto`](#sendmessageresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().send_a_new_message(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.post_conversations_messages",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /conversations/messages/email/{emailMessageId}/schedule`

**Cancel a scheduled email message.**

Post the messageId for the API to delete a scheduled email message.

Operation id: `v3:conversations.delete_conversations_messages_email_by_emailMessageId_schedule` · `Version: v3` · Scopes: `conversations/message.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `emailMessageId` | string | **yes** | Email Message Id |

*Response*: [`CancelScheduledResponseDto`](#cancelscheduledresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().cancel_a_scheduled_email_message(&emailMessageId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.delete_conversations_messages_email_by_emailMessageId_schedule",
    "path_params": {
      "emailMessageId": "<emailMessageId>"
    }
  }
}
```

</details>

#### `GET /conversations/messages/email/{id}`

**Get email by Id**

Operation id: `v3:conversations.get_conversations_messages_email_by_id` · `Version: v3` · Scopes: `conversations/message.readonly`

*Response*: [`GetEmailMessageResponseDto`](#getemailmessageresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().get_email_by_id().await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.get_conversations_messages_email_by_id"
  }
}
```

</details>

#### `PUT /conversations/messages/email/{id}/status`

**Update email message status**

Update delivery events, per-recipient statuses, and the overall message status for an email sent via a custom conversation provider. ### Authorization - Requires the `conversations/message.write` OAuth scope. - The calling OAuth app must own the conversation provider that originally sent the email. - Attempts to update emails sent via LC Email or Mailgun will return `403 Forbidden`. ### Updatable Fields **`status`** is required on every request. You may also include **`events`** and/or **`recipi…

Operation id: `v3:conversations.put_conversations_messages_email_by_id_status` · `Version: v3` · Scopes: `conversations/message.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Email message id |

*Request body*: [`UpdateEmailMessageStatusDto`](#updateemailmessagestatusdto)

*Response*: [`UpdateEmailMessageStatusResponseDto`](#updateemailmessagestatusresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().update_email_message_status(&id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.put_conversations_messages_email_by_id_status",
    "path_params": {
      "id": "<id>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /conversations/messages/export`

**Export messages by location ID**

Export messages for a specific location with cursor-based pagination support. Response includes messageType (string), source, and subType fields. The channel parameter is optional - if not provided, all non-email message types will be returned including activity messages (opportunity updates, appointments, etc.).

Operation id: `v3:conversations.get_conversations_messages_export` · `Version: v3` · Scopes: `conversations/message.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID to filter messages by |
| `limit` | number | no | Number of messages to return per page |
| `cursor` | string | no | Cursor for pagination. Pass the nextCursor from previous response to get next page. |
| `sortBy` | enum: `createdAt`, `updatedAt` | no | Field to sort by |
| `sortOrder` | enum: `asc`, `desc` | no | Sort order |
| `conversationId` | string | no | Filter messages by conversation ID |
| `contactId` | string | no | Filter messages by contact ID |
| `channel` | enum: `Call`, `SMS`, `Email`, `WhatsApp`, `Instagram`, `Facebook` | no | Filter by message channel. Optional - when not provided, all non-email message types will be returned including activity messages (opportunity updates, appointm… |
| `startDate` | string | no | Start date to filter messages by |
| `endDate` | string | no | End date to filter messages by |

*Response*: [`ExportMessagesResponseDto`](#exportmessagesresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::conversations::ExportMessagesByLocationIdParams;

let params = ExportMessagesByLocationIdParams::new("locationId");
let out = ghl.v3().conversations().export_messages_by_location_id(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.get_conversations_messages_export",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /conversations/messages/inbound`

**Add an inbound message**

Post the necessary fields for the API to add a new inbound message.

Operation id: `v3:conversations.post_conversations_messages_inbound` · `Version: v3` · Scopes: `conversations/message.write`

*Request body*: [`ProcessMessageBodyDto`](#processmessagebodydto)

*Response*: [`ProcessMessageResponseDto`](#processmessageresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().add_an_inbound_message(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.post_conversations_messages_inbound",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /conversations/messages/outbound`

**Add an external outbound call**

Post the necessary fields for the API to add a new outbound call.

Operation id: `v3:conversations.post_conversations_messages_outbound` · `Version: v3` · Scopes: `conversations/message.write`

*Request body*: [`ProcessOutboundMessageBodyDto`](#processoutboundmessagebodydto)

*Response*: [`ProcessMessageResponseDto`](#processmessageresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().add_an_external_outbound_call(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.post_conversations_messages_outbound",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /conversations/messages/review-reply`

**Send a review reply to Google My Business**

Post a reply to a customer review on Google My Business This endpoint is internal-only and is not supported for OAuth or public API integrations. It will be removed from the public OpenAPI specification in a future release.

Operation id: `v3:conversations.post_conversations_messages_review_reply` · `Version: v3` · Scopes: `conversations/message.write`

*Request body*: [`SendReviewReplyDto`](#sendreviewreplydto)

*Response*: [`SendMessageResponseDto`](#sendmessageresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().send_a_review_reply_to_google_my_business(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.post_conversations_messages_review_reply",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /conversations/messages/upload`

**Upload file attachments**

Post the necessary fields for the API to upload files. The files need to be a buffer with the key "fileAttachment". The allowed file types are: <ul><li>JPG</li><li>JPEG</li><li>PNG</li><li>MP4</li><li>MPEG</li><li>ZIP</li><li>RAR</li><li>PDF</li><li>DOC</li><li>DOCX</li><li>TXT</li><li>MP3</li><li>WAV</li></ul> The API will return an object with the URLs

Operation id: `v3:conversations.post_conversations_messages_upload` · `Version: v3` · Scopes: `conversations/message.write`

*Response*: [`UploadFilesResponseDto`](#uploadfilesresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().upload_file_attachments().await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.post_conversations_messages_upload"
  }
}
```

</details>

#### `POST /conversations/messages/upload/complete`

**Complete file upload**

Validates the uploaded file in GCS and returns the public URL. Call this endpoint after successfully uploading the file to the signed URL. This endpoint is internal-only and is not supported for OAuth or public API integrations. It will be removed from the public OpenAPI specification in a future release.

Operation id: `v3:conversations.post_conversations_messages_upload_complete` · `Version: v3` · Scopes: `conversations/message.write`

*Request body*: [`CompleteFileUploadDto`](#completefileuploaddto)

*Response*: [`CompleteFileUploadResponseDto`](#completefileuploadresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().complete_file_upload(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.post_conversations_messages_upload_complete",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /conversations/messages/upload/initiate`

**Initiate file upload to GCS**

Generates a signed URL for direct file upload to Google Cloud Storage. Returns a signed URL valid for 15 minutes. Upload file via PUT request, then call /complete to finalize. This endpoint is internal-only and is not supported for OAuth or public API integrations. It will be removed from the public OpenAPI specification in a future release.

Operation id: `v3:conversations.post_conversations_messages_upload_initiate` · `Version: v3` · Scopes: `conversations/message.write`

*Request body*: [`InitiateFileUploadDto`](#initiatefileuploaddto)

*Response*: [`InitiateFileUploadResponseDto`](#initiatefileuploadresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().initiate_file_upload_to_gcs(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.post_conversations_messages_upload_initiate",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /conversations/messages/{id}`

**Get message by message id**

Get message by message id.

Operation id: `v3:conversations.get_conversations_messages_by_id` · `Version: v3` · Scopes: `conversations/message.readonly`

*Response*: [`GetMessageResponseDto`](#getmessageresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().get_message_by_message_id().await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.get_conversations_messages_by_id"
  }
}
```

</details>

#### `PUT /conversations/messages/{messageId}/attachments`

**Add message attachments**

Set attachments on an existing message (replaces existing). Maximum 5 URLs. Supported for TYPE_CUSTOM_CALL (34) and TYPE_CALL (1) with subType EXTERNAL_CALL.

Operation id: `v3:conversations.put_conversations_messages_by_messageId_attachments` · `Version: v3` · Scopes: `conversations/message.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `messageId` | string | **yes** | Message Id |

*Request body*: [`AddMessageAttachmentsDto`](#addmessageattachmentsdto)

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().add_message_attachments(&messageId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.put_conversations_messages_by_messageId_attachments",
    "path_params": {
      "messageId": "<messageId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /conversations/messages/{messageId}/locations/{locationId}/recording`

**Get Recording by Message ID**

Get the recording for a message by passing the message id

Operation id: `v3:conversations.get_conversations_messages_by_messageId_locations_by_locationId_recording` · `Version: v3` · Scopes: `conversations/message.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID as string |
| `messageId` | string | **yes** | Message ID as string |

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().get_recording_by_message_id(&locationId, &messageId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.get_conversations_messages_by_messageId_locations_by_locationId_recording",
    "path_params": {
      "locationId": "<locationId>",
      "messageId": "<messageId>"
    }
  }
}
```

</details>

#### `DELETE /conversations/messages/{messageId}/schedule`

**Cancel a scheduled message.**

Post the messageId for the API to delete a scheduled message.

Operation id: `v3:conversations.delete_conversations_messages_by_messageId_schedule` · `Version: v3` · Scopes: `conversations/message.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `messageId` | string | **yes** | Message Id |

*Response*: [`CancelScheduledResponseDto`](#cancelscheduledresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().cancel_a_scheduled_message(&messageId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.delete_conversations_messages_by_messageId_schedule",
    "path_params": {
      "messageId": "<messageId>"
    }
  }
}
```

</details>

#### `PUT /conversations/messages/{messageId}/status`

**Update message status**

Post the necessary fields for the API to update message status.

Operation id: `v3:conversations.put_conversations_messages_by_messageId_status` · `Version: v3` · Scopes: `conversations/message.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `messageId` | string | **yes** | Message Id |

*Request body*: [`UpdateMessageStatusDto`](#updatemessagestatusdto)

*Response*: [`SendMessageResponseDto`](#sendmessageresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().update_message_status(&messageId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.put_conversations_messages_by_messageId_status",
    "path_params": {
      "messageId": "<messageId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /conversations/providers/live-chat/typing`

**Agent/Ai-Bot is typing a message indicator for live chat**

Agent/AI-Bot will call this when they are typing a message in live chat message

Operation id: `v3:conversations.post_conversations_providers_live_chat_typing` · `Version: v3` · Scopes: `conversations/livechat.write`

*Request body*: [`UserTypingBody`](#usertypingbody)

*Response*: [`CreateLiveChatMessageFeedbackResponse`](#createlivechatmessagefeedbackresponse)

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().agent_ai_bot_is_typing_a_message_indicator_for_live_chat(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.post_conversations_providers_live_chat_typing",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /conversations/search`

**Search Conversations**

Returns a list of all conversations matching the search criteria along with the sort and filter options selected.

Operation id: `v3:conversations.get_conversations_search` · `Version: v3` · Scopes: `conversations.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `contactId` | string | no | Contact Id |
| `assignedTo` | string | no | User IDs that conversations are assigned to. Multiple IDs can be provided as comma-separated values. Use "unassigned" to fetch conversations not assigned to any… |
| `followers` | string | no | User IDs of followers to filter conversations by. Multiple IDs can be provided as comma-separated values. |
| `mentions` | string | no | User Id of the mention. Multiple values are comma separated. |
| `query` | string | no | Search paramater as a string |
| `sort` | enum: `asc`, `desc` | no | Sort paramater - asc or desc |
| `startAfterDate` | any | no | Search to begin after the specified date - should contain the sort value of the last document |
| `id` | string | no | Id of the conversation |
| `limit` | number | no | Limit of conversations - Default is 20 |
| `lastMessageType` | enum (42 values — see [shared enums](shared-enums.md)) | no | Type of the last message in the conversation as a string |
| `lastMessageAction` | enum: `automated`, `manual` | no | Action of the last outbound message in the conversation as string. |
| `lastMessageDirection` | enum: `inbound`, `outbound` | no | Direction of the last message in the conversation as string. |
| `status` | enum: `all`, `read`, `unread`, `starred`, `recents` | no | The status of the conversation to be filtered - all, read, unread, starred |
| `sortBy` | enum: `last_manual_message_date`, `last_message_date`, `score_profile`, `overdue_at`, `due_at` | no | The sorting of the conversation to be filtered as - manual messages or all messages |
| `sortScoreProfile` | string | no | Id of score profile on which sortBy.ScoreProfile should sort on |
| `scoreProfile` | string | no | Id of score profile on which conversations should get filtered out, works with scoreProfileMin & scoreProfileMax |
| `scoreProfileMin` | number | no | Minimum value for score |
| `scoreProfileMax` | number | no | Maximum value for score |
| `startDate` | number | no | Start date filter for dateAdded field (Unix timestamp in milliseconds) |
| `endDate` | number | no | End date filter for dateAdded field (Unix timestamp in milliseconds) |

*Response*: [`SendConversationResponseDto`](#sendconversationresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::conversations::SearchConversationsParams;

let params = SearchConversationsParams::new("locationId");
let out = ghl.v3().conversations().search_conversations(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.get_conversations_search",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `DELETE /conversations/{conversationId}`

**Delete Conversation**

Delete the conversation details based on the conversation ID

Operation id: `v3:conversations.delete_conversations_by_conversationId` · `Version: v3` · Scopes: `conversations.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `conversationId` | string | **yes** | Conversation ID as string |

*Response*: [`DeleteConversationSuccessfulResponse`](#deleteconversationsuccessfulresponse)

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().delete_conversation(&conversationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.delete_conversations_by_conversationId",
    "path_params": {
      "conversationId": "<conversationId>"
    }
  }
}
```

</details>

#### `GET /conversations/{conversationId}`

**Get Conversation**

Get the conversation details based on the conversation ID

Operation id: `v3:conversations.get_conversations_by_conversationId` · `Version: v3` · Scopes: `conversations.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `conversationId` | string | **yes** | Conversation ID as string |

*Response*: [`GetConversationByIdResponse`](#getconversationbyidresponse)

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().get_conversation(&conversationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.get_conversations_by_conversationId",
    "path_params": {
      "conversationId": "<conversationId>"
    }
  }
}
```

</details>

#### `PUT /conversations/{conversationId}`

**Update Conversation**

Update the conversation details based on the conversation ID

Operation id: `v3:conversations.put_conversations_by_conversationId` · `Version: v3` · Scopes: `conversations.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `conversationId` | string | **yes** | Conversation ID as string |

*Request body*: [`UpdateConversationDto`](#updateconversationdto)

*Response*: [`GetConversationSuccessfulResponse`](#getconversationsuccessfulresponse)

*Rust*:

```rust,ignore
let out = ghl.v3().conversations().update_conversation(&conversationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.put_conversations_by_conversationId",
    "path_params": {
      "conversationId": "<conversationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /conversations/{conversationId}/messages`

**Get messages by conversation id**

Get messages by conversation id.

Operation id: `v3:conversations.get_conversations_by_conversationId_messages` · `Version: v3` · Scopes: `conversations/message.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `conversationId` | string | **yes** | Conversation ID as string |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `lastMessageId` | string | no | Message ID of the last message in the list as a string |
| `limit` | number | no | Number of messages to be fetched from the conversation. Default limit is 20 |
| `type` | enum: `TYPE_CALL`, `TYPE_SMS`, `TYPE_RCS`, `TYPE_EMAIL`, `TYPE_FACEBOOK`, `TYPE_GMB`, `TYPE_INSTAGRAM`, `TYPE_WHATSAPP`, `TYPE_ACTIVITY_APPOINTMENT`, `TYPE_ACTIVITY_CONTACT`, `TYPE_ACTIVITY_INVOICE`, `TYPE_ACTIVITY_PAYMENT`, `TYPE_ACTIVITY_OPPORTUNITY`, `TYPE_LIVE_CHAT`, `TYPE_INTERNAL_COMMENTS`, `TYPE_ACTIVITY_EMPLOYEE_ACTION_LOG`, `TYPE_TIKTOK`, `TYPE_ACTIVITY_WHATSAPP`, `TYPE_FORM_SUBMISSION` | no | Types of message to fetched separated with comma |

*Response*: [`GetMessagesByConversationResponseDto`](#getmessagesbyconversationresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::conversations::GetMessagesByConversationIdParams;

let params = GetMessagesByConversationIdParams::new();
let out = ghl.v3().conversations().get_messages_by_conversation_id(&conversationId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversations.get_conversations_by_conversationId_messages",
    "path_params": {
      "conversationId": "<conversationId>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::conversations::*` (enable the `conversations` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/conversations/).

### `AddMessageAttachmentsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `attachments` | Vec<String> | **yes** | Array of attachment URLs to set on the message (replaces existing). Maximum 5 URLs. |

### `CallDataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `to` | String | no | Phone number of the receiver |
| `from` | String | no | Phone number of the dialer |
| `status` | String — `pending`, `completed`, `answered`, `busy`, `no-answer`, `failed`, `canceled`, `voicemail` | no | Call status |

### `CancelScheduledResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | f64 | **yes** | HTTP Status code of the request |
| `message` | String | **yes** | Error message of the request |

### `CompleteFileUploadDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `uploadId` | String | **yes** | Upload ID from request response |
| `filePath` | String | **yes** | File path from request response |
| `locationId` | String | **yes** | Location ID |
| `conversationId` | String | **yes** | Conversation ID |
| `filename` | String | **yes** | Original filename (for response mapping) |

### `CompleteFileUploadResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `uploadedFiles` | JSON | **yes** | Map of filename to public URL |
| `metadata` | JSON | **yes** | File metadata |

### `ConversationCreateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the conversation |
| `dateUpdated` | String | **yes** | Date when the conversation was last updated |
| `dateAdded` | String | **yes** | Date when the conversation was created |
| `deleted` | bool | **yes** | Flag indicating if this conversation has been deleted |
| `contactId` | String | **yes** | Unique identifier of the contact associated with this conversation |
| `locationId` | String | **yes** | Unique identifier of the business location where this conversation takes place |
| `lastMessageDate` | String | **yes** | Date of the last message in the conversation |
| `assignedTo` | String | no | Unique identifier of the team member assigned to this conversation |

### `ConversationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Contact ID as string |
| `locationId` | String | **yes** | Location ID as string |
| `contactId` | String | **yes** | Contact ID as string |
| `assignedTo` | String | no | Assigned User ID as string |
| `userId` | String | no | User ID as string |
| `lastMessageBody` | String | no | Last message body as string |
| `lastMessageDate` | String | no | Last message date as UTC |
| `lastMessageType` | String — 42 values ([shared](shared-enums.md)) | no | Type of the last message sent/received in the conversation. |
| `unreadCount` | f64 | no | Count of unread messages in the conversation |
| `inbox` | bool | no | Inbox status of the conversation. |
| `starred` | bool | no | Starred status of the conversation. |
| `deleted` | bool | **yes** | Deleted status of the conversation. |

### `ConversationSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Conversation Id |
| `contactId` | String | **yes** | Contact Id |
| `locationId` | String | **yes** | Location Id |
| `lastMessageBody` | String | **yes** | Content of the most recent message in the conversation |
| `lastMessageType` | String — 42 values ([shared](shared-enums.md)) | **yes** | Channel/type of the most recent message (SMS, Email, Call, etc) |
| `type` | String — `TYPE_PHONE`, `TYPE_EMAIL`, `TYPE_FB_MESSENGER`, `TYPE_REVIEW`, `TYPE_GROUP_SMS` | **yes** | Primary channel/type of the conversation (Phone, Email, etc) |
| `unreadCount` | f64 | **yes** | Number of unread messages in this conversation |
| `fullName` | String | **yes** | Complete name of the contact (first and last name) |
| `contactName` | String | **yes** | Alternative display name for the contact - used when full name is not available |
| `email` | String | **yes** | Primary email address of the contact |
| `phone` | String | **yes** | Primary phone number of the contact |

### `CreateConversationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID as string |
| `contactId` | String | **yes** | Contact ID as string |

### `CreateConversationSuccessResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Indicates whether the API request was successful. |
| `conversation` | [`ConversationCreateResponseDto`](#conversationcreateresponsedto) | **yes** | Conversation data of the provided conversation ID. |

### `CreateCustomSubtypeDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Name of the custom subtype (max 100 characters) |
| `description` | String | no | Description of the custom subtype (max 100 characters) |
| `channel` | String — `email`, `sms` | **yes** | Communication channel |
| `language` | String | **yes** | Language code |

### `CreateLiveChatMessageFeedbackResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |

### `DeleteConversationSuccessfulResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Boolean value as the API response. |

### `ErrorDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `code` | String | **yes** | Error Code |
| `type` | String | **yes** | Error Type |
| `message` | String | **yes** | Error Message |

### `ExportMessagesResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `messages` | Vec<GetMessageResponseDto> | **yes** | Array of messages |
| `nextCursor` | String | no | Cursor for fetching next page. Null if no more results. |
| `total` | f64 | **yes** | Total number of messages matching the query |

### `ForwardConfigDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `isForwarded` | bool | **yes** | Specify if this is a forwarded email |
| `forwardWholeThread` | bool | no | Specify if forwarding the whole thread or just a single email |
| `messageId` | String | no | Message ID of the email thread being forwarded (source) - REQUIRED for forwarding |
| `emailMessageId` | String | no | Email Message ID of the specific email being forwarded (source) - Required for single email forward, ignored for thread forward |
| `sourceContactId` | String | no | Contact ID where the forwarded email originated from (source) - Auto-populated if not provided |
| `sourceConversationId` | String | no | Conversation ID where the forwarded email originated from (source) - Auto-populated if not provided |
| `toEmail` | String | no | Email address to forward to (destination) |
| `recipientContactId` | String | no | Contact ID of recipient when forwarding (destination) |
| `recipientConversationId` | String | no | Conversation ID of recipient when forwarding (destination) |

### `ForwardResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `forwardWholeThread` | bool | no | Whether the entire thread was forwarded |
| `messageId` | String | no | Message ID of the forwarded message (source) |
| `emailMessageId` | String | no | Email Message ID of the forwarded email (source) |
| `sourceContactId` | String | no | Contact ID where the forwarded email originated from (source) |
| `sourceConversationId` | String | no | Conversation ID where the forwarded email originated from (source) |
| `forwardToEmail` | String | no | Email address the message was forwarded to (destination) |
| `recipientContactId` | String | no | Contact ID of the recipient of the forwarded email (destination) |
| `recipientConversationId` | String | no | Conversation ID of the recipient of the forwarded email (destination) |

### `GetConversationByIdResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `contactId` | String | **yes** | Unique identifier of the contact associated with this conversation |
| `locationId` | String | **yes** | Unique identifier of the business location where this conversation takes place |
| `deleted` | bool | **yes** | Flag indicating if this conversation has been moved to trash/deleted |
| `inbox` | bool | **yes** | Flag indicating if this conversation is currently in the main inbox view |
| `type` | f64 | **yes** | Communication channel type for this conversation: 1 (Phone), 2 (Email), 3 (Facebook Messenger), 4 (Review), 5 (Group SMS), 6 (Internal Chat - coming soon) |
| `unreadCount` | f64 | **yes** | Number of messages in this conversation that have not been read by the user |
| `assignedTo` | String | no | Unique identifier of the team member currently responsible for handling this conversation |
| `id` | String | **yes** | Unique identifier for this specific conversation thread |
| `starred` | bool | no | Flag indicating if this conversation has been marked as important/starred by the user |

### `GetConversationSuccessfulResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Boolean value as the API response. |
| `conversation` | [`ConversationDto`](#conversationdto) | **yes** | Conversation data of the provided conversation ID. |

### `GetEmailMessageResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | — |
| `altId` | String | no | External Id |
| `threadId` | String | **yes** | Message Id or thread Id |
| `locationId` | String | **yes** | — |
| `contactId` | String | **yes** | — |
| `conversationId` | String | **yes** | — |
| `dateAdded` | String | **yes** | — |
| `subject` | String | no | — |
| `body` | String | **yes** | — |
| `direction` | String — `inbound`, `outbound` | **yes** | — |
| `status` | String — `pending`, `scheduled`, `sent`, `delivered`, `read`, `undelivered`, `connected`, `failed`, `opened` | no | — |
| `contentType` | String | **yes** | — |
| `attachments` | Vec<String> | no | An array of attachment URLs. |
| `provider` | String | no | — |
| `from` | String | **yes** | Name and Email Id of the sender |
| `to` | Vec<String> | **yes** | List of email Ids of the receivers |
| `cc` | Vec<String> | no | List of email Ids of the people in the cc field |
| `bcc` | Vec<String> | no | List of email Ids of the people in the bcc field |
| `replyToMessageId` | String | no | In case of reply, email message Id of the reply to email |
| `source` | String — `workflow`, `bulk_actions`, `campaign`, `api`, `app` | no | Email source |
| `conversationProviderId` | String | no | Conversation provider ID |

### `GetMessageResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | — |
| `type` | f64 | **yes** | — |
| `messageType` | String — 42 values ([shared](shared-enums.md)) | **yes** | Type of the message as a string |
| `locationId` | String | **yes** | — |
| `contactId` | String | **yes** | — |
| `conversationId` | String | **yes** | — |
| `dateAdded` | String | **yes** | — |
| `body` | String | no | — |
| `direction` | String — `inbound`, `outbound` | **yes** | — |
| `status` | String — `connected`, `delivered`, `failed`, `opened`, `pending`, `read`, `scheduled`, `sent`, `undelivered`, `clicked`, `opt_out`, `queued` | no | — |
| `contentType` | String | **yes** | — |
| `attachments` | Vec<String> | no | An array of attachment URLs. Attachments will be empty for Call and Voicemails, type 1 and 10. Please use get call recording API to fetch call recording and voicemails. |
| `meta` | [`MessageMeta`](#messagemeta) | no | — |
| `source` | String — `workflow`, `bulk_actions`, `campaign`, `api`, `app` | no | Message source |
| `userId` | String | no | User Id |
| `conversationProviderId` | String | no | Conversation Provider Id |
| `chatWidgetId` | String | no | Chat Widget Id |

### `GetMessageTranscriptionResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `mediaChannel` | f64 | **yes** | Media channel describes the user interaction channel |
| `sentenceIndex` | f64 | **yes** | Index of the sentence in the transcription |
| `startTime` | f64 | **yes** | Start time of the sentence in milliseconds |
| `endTime` | f64 | **yes** | End time of the sentence in milliseconds |
| `transcript` | String | **yes** | Transcript of the sentence |
| `confidence` | f64 | **yes** | Confidence of the transcription |

### `GetMessagesByConversationResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `lastMessageId` | String | **yes** | Id of the last message in the messages array |
| `nextPage` | bool | **yes** | Next page value true indicates only 20 message is in the response. Rest of the messages are in the next page. Please use the lastMessageId value in the query to get the next page messages |
| `messages` | Vec<GetMessageResponseDto> | **yes** | Array of messages |

### `InitiateFileUploadDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `conversationId` | String | **yes** | Conversation ID |
| `filename` | String | **yes** | Original filename with extension |
| `contentType` | String | **yes** | MIME type of the file |
| `fileSize` | f64 | no | File size in bytes (optional, for pre-validation) |
| `channel` | String | **yes** | Channel type for size limits (WHATSAPP for 100MB limit, others for 5MB) |

### `InitiateFileUploadResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `uploadUrl` | String | **yes** | Signed URL for direct upload to GCS. Use PUT request with file content. |
| `uploadId` | String | **yes** | Unique upload ID for tracking and completing the upload |
| `filePath` | String | **yes** | File path in GCS bucket (needed for confirmation endpoint) |
| `expiresAt` | f64 | **yes** | URL expiration timestamp (Unix milliseconds) |
| `maxFileSize` | f64 | **yes** | Maximum allowed file size in bytes |

### `MessageMeta`

| Field | Type | Required | Description |
|---|---|---|---|
| `callDuration` | String | no | Call duration in seconds |
| `callStatus` | String — `pending`, `completed`, `answered`, `busy`, `no-answer`, `failed`, `canceled`, `voicemail` | no | Call status - can be pending, completed, answered, busy, no-answer, failed, canceled, or voicemail |
| `email` | JSON | no | meta will contain email, for message type 3 (email). messageIds is list of all email message ids under the message thread |

### `ProcessMessageBodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `SMS`, `RCS`, `Email`, `WhatsApp`, `GMB`, `IG`, `FB`, `Custom`, `WebChat`, `Live_Chat`, `Call`, `IVR_Call`, `Campaign_Call`, `Campaign_VoiceMail`, `TIKTOK`, `ALL_IN_ONE_CHAT`, `FORM_SUBMISSION` | **yes** | Message Type |
| `attachments` | Vec<String> | no | Array of attachments |
| `message` | String | no | Message Body |
| `conversationId` | String | **yes** | Conversation Id |
| `contactId` | String | **yes** | Contact Id |
| `conversationProviderId` | String | **yes** | Conversation Provider Id |
| `html` | String | no | HTML Body of Email |
| `subject` | String | no | Subject of the Email |
| `emailFrom` | String | no | Email address to send from. This field is associated with the contact record and cannot be dynamically changed. |
| `emailTo` | String | no | Recipient email address. This field is associated with the contact record and cannot be dynamically changed. |
| `emailCc` | Vec<String> | no | List of email address to CC |
| `emailBcc` | Vec<String> | no | List of email address to BCC |
| `emailMessageId` | String | no | Send the email message id for which this email should be threaded. This is for replying to a specific email |
| `altId` | String | no | external mail provider's message id |
| `direction` | JSON | no | Message direction, if required can be set manually, default is outbound |
| `date` | String | no | Date of the inbound message |
| `call` | [`CallDataDTO`](#calldatadto) | no | Phone call dialer and receiver information |

### `ProcessMessageResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |
| `conversationId` | String | **yes** | Conversation ID. |
| `messageId` | String | **yes** | This is the main Message ID |
| `message` | String | **yes** | — |
| `contactId` | String | no | — |
| `dateAdded` | String | no | — |
| `emailMessageId` | String | no | — |

### `ProcessOutboundMessageBodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `Call` | **yes** | Message Type |
| `attachments` | Vec<String> | no | Array of attachments |
| `conversationId` | String | **yes** | Conversation Id |
| `conversationProviderId` | String | **yes** | Conversation Provider Id |
| `altId` | String | no | external mail provider's message id |
| `date` | String | no | Date of the outbound message |
| `call` | [`CallDataDTO`](#calldatadto) | no | Phone call dialer and receiver information |

### `SendConversationResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `conversations` | Vec<ConversationSchema> | **yes** | The list of all conversations found for the given query |
| `total` | f64 | **yes** | Total Number of results found for the given query |

### `SendMessageBodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `SMS`, `RCS`, `Email`, `WhatsApp`, `IG`, `FB`, `Custom`, `Live_Chat`, `TIKTOK` | **yes** | Type of message being sent |
| `subType` | JSON | **yes** | Type of message being sent |
| `contactId` | String | **yes** | ID of the contact receiving the message |
| `appointmentId` | String | no | ID of the associated appointment |
| `attachments` | Vec<String> | no | Array of attachment URLs |
| `emailFrom` | String | no | Email address to send from |
| `emailCc` | Vec<String> | no | Array of CC email addresses |
| `emailBcc` | Vec<String> | no | Array of BCC email addresses |
| `html` | String | no | HTML content of the message |
| `message` | String | no | Text content of the message |
| `subject` | String | no | Subject line for email messages |
| `replyMessageId` | String | no | ID of message being replied to |
| `templateId` | String | no | ID of message template |
| `threadId` | String | no | ID of message thread. For email messages, this is the message ID that contains multiple email messages in the thread |
| `scheduledTimestamp` | f64 | no | UTC Timestamp (in seconds) at which the message should be scheduled |
| `conversationProviderId` | String | no | ID of conversation provider |
| `emailTo` | String | no | Email address to send to, if different from contact's primary email. This should be a valid email address associated with the contact. |
| `customSubtypeId` | String | no | Custom subtype ID for email unsubscription preferences. Only applies to email messages. |
| `emailReplyMode` | String — `reply`, `reply_all` | no | Mode for email replies |
| `fromNumber` | String | no | Phone number used as the sender number for outbound messages |
| `toNumber` | String | no | Recipient phone number for outbound messages |
| `forward` | [`ForwardConfigDto`](#forwardconfigdto) | no | Forwarding configuration for emails |
| `status` | String — `delivered`, `failed`, `pending`, `read` | **yes** | Message status |
| `usesNativeSchedulingAi` | bool | no | Whether the scheduled email uses native AI for the email scheduling |
| `optimizationPeriod` | String — `24h`, `48h`, `72h` | no | Optimization period in hours (24h, 48h, or 72h) |

### `SendMessageResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `conversationId` | String | **yes** | Conversation ID. |
| `emailMessageId` | String | no | This contains the email message id (only for Email type). Use this ID to send inbound replies to GHL to create a threaded email. |
| `messageId` | String | **yes** | This is the main Message ID |
| `messageIds` | Vec<String> | no | When sending via the GMB channel, we will be returning list of `messageIds` instead of single `messageId`. |
| `msg` | String | no | Additional response message when sending a workflow message |
| `forwardData` | [`ForwardResponseDto`](#forwardresponsedto) | no | Optional metadata for forwarded email |
| `status` | String — `delivered`, `failed`, `pending`, `read` | **yes** | Message status |

### `SendReviewReplyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `conversationId` | String | **yes** | Conversation ID (must have reviewId) |
| `locationId` | String | **yes** | Location ID |
| `message` | String | **yes** | Review reply message text |

### `StartAfterArrayNumberSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `startAfterDate` | Vec<String> | no | Search to begin after the specified date - should contain the sort value of the last document |

### `StartAfterNumberSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `startAfterDate` | f64 | no | Search to begin after the specified date - should contain the sort value of the last document |

### `SubscriptionActionDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `default`, `custom`, `resub_all` | **yes** | Type of subscription action |
| `subtype_name` | String — `One on One` | no | Subscription type name (required for default types: "One on One") |
| `subtype_id` | String | no | Custom subscription type ID (required for custom types) |
| `subtype_status` | String — `subscribed`, `unsubscribed` | **yes** | Subscription status |

### `UpdateConversationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID as string |
| `unreadCount` | f64 | no | Count of unread messages in the conversation |
| `starred` | bool | no | Starred status of the conversation. |
| `feedback` | JSON | no | — |

### `UpdateCustomSubtypeDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Name of the custom subtype (max 100 characters) |
| `description` | String | no | Description of the custom subtype (max 100 characters) |
| `archived` | bool | no | Whether the custom subtype is archived |
| `resubscription_legal_form_id` | String | no | Resubscription legal form ID (optional when archiving) |

### `UpdateMessageStatusDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | String — `delivered`, `failed`, `pending`, `read` | **yes** | Message status |
| `error` | [`ErrorDto`](#errordto) | no | Error object from the conversation provider |
| `emailMessageId` | String | no | Email message Id |
| `recipients` | Vec<String> | no | Email delivery status for additional email recipients. |

### `UploadFilesDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `conversationId` | String | **yes** | Conversation Id |
| `contactId` | String | **yes** | Contact Id |
| `locationId` | String | **yes** | — |
| `attachmentUrls` | Vec<String> | **yes** | — |
| `chatServiceSid` | String | no | Twilio chat service SID for group SMS uploads |
| `isGroupSms` | String | no | Flag to indicate group SMS upload flow. When true, only 1 file upload is allowed per request. |

### `UploadFilesErrorResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | String — `400`, `413`, `415` | **yes** | HTTP Status code of the request |
| `message` | String | **yes** | Error message of the request |

### `UploadFilesResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `uploadedFiles` | JSON | **yes** | — |
| `twilioMediaSids` | Vec<String> | no | Twilio media SIDs for group SMS (when isGroupSms=true) |

### `UserSubscriptionChangeDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location Id |
| `contactId` | String | **yes** | Contact Id |
| `email` | String | **yes** | Email address |
| `subscription_action` | [`SubscriptionActionDto`](#subscriptionactiondto) | **yes** | Subscription action details |
| `legal_reason` | String | no | Legal reason for the change (required only for resubscribe and resub_all actions) |
| `legal_description` | String | no | Legal description/details |

### `UserTypingBody`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location Id |
| `isTyping` | String | **yes** | Typing status |
| `visitorId` | String | **yes** | visitorId is the Unique ID assigned to each Live chat visitor. visitorId will be added soon in <a href="https://highlevel.stoplight.io/docs/integrations/00c5ff21f0030-get-contact" target="_blank">GET … |
| `conversationId` | String | **yes** | Conversation Id |

## Data models — API v3

In Rust: `ghl_models::v3::conversations::*` (enable the `conversations` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/conversations/).

### `AddMessageAttachmentsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `attachments` | Vec<String> | **yes** | Array of attachment URLs to set on the message (replaces existing). Maximum 5 URLs. |

### `CallDataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `to` | String | no | Phone number of the receiver |
| `from` | String | no | Phone number of the dialer |
| `status` | String — `pending`, `completed`, `answered`, `busy`, `no-answer`, `failed`, `canceled`, `voicemail` | no | Call status |

### `CancelScheduledResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | f64 | **yes** | HTTP Status code of the request |
| `message` | String | **yes** | Error message of the request |

### `CompleteFileUploadDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `uploadId` | String | **yes** | Upload ID from request response |
| `filePath` | String | **yes** | File path from request response |
| `locationId` | String | **yes** | Location ID |
| `conversationId` | String | **yes** | Conversation ID |
| `filename` | String | **yes** | Original filename (for response mapping) |

### `CompleteFileUploadResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `uploadedFiles` | JSON | **yes** | Map of filename to public URL |
| `metadata` | JSON | **yes** | File metadata |

### `ConversationCreateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the conversation |
| `dateUpdated` | String | **yes** | Date when the conversation was last updated |
| `dateAdded` | String | **yes** | Date when the conversation was created |
| `deleted` | bool | **yes** | Flag indicating if this conversation has been deleted |
| `contactId` | String | **yes** | Unique identifier of the contact associated with this conversation |
| `locationId` | String | **yes** | Unique identifier of the business location where this conversation takes place |
| `lastMessageDate` | String | **yes** | Date of the last message in the conversation |
| `assignedTo` | String | no | Unique identifier of the team member assigned to this conversation |

### `ConversationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Contact ID as string |
| `locationId` | String | **yes** | Location ID as string |
| `contactId` | String | **yes** | Contact ID as string |
| `assignedTo` | String | no | Assigned User ID as string |
| `userId` | String | no | User ID as string |
| `lastMessageBody` | String | no | Last message body as string |
| `lastMessageDate` | String | no | Last message date as UTC |
| `lastMessageType` | String — 42 values ([shared](shared-enums.md)) | no | Type of the last message sent/received in the conversation. |
| `unreadCount` | f64 | no | Count of unread messages in the conversation |
| `inbox` | bool | no | Inbox status of the conversation. |
| `starred` | bool | no | Starred status of the conversation. |
| `deleted` | bool | **yes** | Deleted status of the conversation. |

### `ConversationSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Conversation Id |
| `contactId` | String | **yes** | Contact Id |
| `locationId` | String | **yes** | Location Id |
| `lastMessageBody` | String | **yes** | Content of the most recent message in the conversation |
| `lastMessageType` | String — 42 values ([shared](shared-enums.md)) | **yes** | Channel/type of the most recent message (SMS, Email, Call, etc) |
| `type` | String — `TYPE_PHONE`, `TYPE_EMAIL`, `TYPE_FB_MESSENGER`, `TYPE_REVIEW`, `TYPE_GROUP_SMS` | **yes** | Primary channel/type of the conversation (Phone, Email, etc) |
| `unreadCount` | f64 | **yes** | Number of unread messages in this conversation |
| `fullName` | String | **yes** | Complete name of the contact (first and last name) |
| `contactName` | String | **yes** | Alternative display name for the contact - used when full name is not available |
| `email` | String | **yes** | Primary email address of the contact |
| `phone` | String | **yes** | Primary phone number of the contact |

### `CreateConversationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID as string |
| `contactId` | String | **yes** | Contact ID as string |

### `CreateConversationSuccessResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Indicates whether the API request was successful. |
| `conversation` | [`ConversationCreateResponseDto`](#conversationcreateresponsedto) | **yes** | Conversation data of the provided conversation ID. |

### `CreateLiveChatMessageFeedbackResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Whether the live chat feedback was recorded successfully |

### `DeleteConversationSuccessfulResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Boolean value as the API response. |

### `EmailEventsDto`

Aggregate delivery event counters for the email message. Values are merged into existing stored counters (additive update, not replacement). A value of `0` is treated as a no-op and will not reset the stored counter. Only include fields you want to update. Provider-specific counters such as `permanent_fail`, `temporary_fail`, and `esp_isp_block` may be sent using those snake_case keys.

| Field | Type | Required | Description |
|---|---|---|---|
| `delivered` | i64 | no | Number of successful deliveries — the recipient mail server accepted the message. Automatically inferred (set to 1) when `opened`, `clicked`, `complained`, `unsubscribed`, or `replied` events are repo… |
| `opened` | i64 | no | Number of unique open events (typically tracked via a tracking pixel). Automatically inferred (set to 1) when `clicked`, `complained`, `unsubscribed`, or `replied` events are reported, provided open t… |
| `clicked` | i64 | no | Number of link click events within the email body. Triggers automatic inference of both `opened` and `delivered`. |
| `replied` | i64 | no | Number of reply events. Triggers automatic inference of both `opened` (if open tracking is enabled) and `delivered`. |
| `failed` | i64 | no | Total number of delivery failures (includes both permanent and temporary). For more granular failure tracking, use `permanent_fail` and `temporary_fail` instead. |
| `accepted` | i64 | no | Number of messages accepted by the receiving mail server for delivery. Automatically inferred (set to 1) when `delivered`, `permanent_fail`, or `temporary_fail` events are reported. |
| `rejected` | i64 | no | Number of messages rejected outright by the receiving mail server (before acceptance). |
| `unsubscribed` | i64 | no | Number of unsubscribe events triggered by the recipient. Triggers automatic inference of `opened` and `delivered`. |
| `complained` | i64 | no | Number of spam complaint events (e.g., recipient marked the email as spam). Triggers automatic inference of `opened` and `delivered`. |
| `stored` | i64 | no | Number of messages stored by the email service provider (ESP-specific event). |

### `ErrorDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `code` | String | **yes** | Error Code |
| `type` | String | **yes** | Error Type |
| `message` | String | **yes** | Error Message |

### `ExportMessagesResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `messages` | Vec<GetMessageResponseDto> | **yes** | Array of messages |
| `nextCursor` | String | no | Cursor for fetching next page. Null if no more results. |
| `total` | f64 | **yes** | Total number of messages matching the query |

### `ForwardConfigDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `isForwarded` | bool | **yes** | Specify if this is a forwarded email |
| `forwardWholeThread` | bool | no | Specify if forwarding the whole thread or just a single email |
| `messageId` | String | no | Message ID of the email thread being forwarded (source) - REQUIRED for forwarding |
| `emailMessageId` | String | no | Email Message ID of the specific email being forwarded (source) - Required for single email forward, ignored for thread forward |
| `sourceContactId` | String | no | Contact ID where the forwarded email originated from (source) - Auto-populated if not provided |
| `sourceConversationId` | String | no | Conversation ID where the forwarded email originated from (source) - Auto-populated if not provided |
| `toEmail` | String | no | Email address to forward to (destination) |
| `recipientContactId` | String | no | Contact ID of recipient when forwarding (destination) |
| `recipientConversationId` | String | no | Conversation ID of recipient when forwarding (destination) |

### `ForwardResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `forwardWholeThread` | bool | no | Whether the entire thread was forwarded |
| `messageId` | String | no | Message ID of the forwarded message (source) |
| `emailMessageId` | String | no | Email Message ID of the forwarded email (source) |
| `sourceContactId` | String | no | Contact ID where the forwarded email originated from (source) |
| `sourceConversationId` | String | no | Conversation ID where the forwarded email originated from (source) |
| `forwardToEmail` | String | no | Email address the message was forwarded to (destination) |
| `recipientContactId` | String | no | Contact ID of the recipient of the forwarded email (destination) |
| `recipientConversationId` | String | no | Conversation ID of the recipient of the forwarded email (destination) |

### `GetConversationByIdResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `contactId` | String | **yes** | Unique identifier of the contact associated with this conversation |
| `locationId` | String | **yes** | Unique identifier of the business location where this conversation takes place |
| `deleted` | bool | **yes** | Flag indicating if this conversation has been moved to trash/deleted |
| `inbox` | bool | **yes** | Flag indicating if this conversation is currently in the main inbox view |
| `type` | f64 | **yes** | Communication channel type for this conversation: 1 (Phone), 2 (Email), 3 (Facebook Messenger), 4 (Review), 5 (Group SMS), 6 (Internal Chat - coming soon) |
| `unreadCount` | f64 | **yes** | Number of messages in this conversation that have not been read by the user |
| `assignedTo` | String | no | Unique identifier of the team member currently responsible for handling this conversation |
| `id` | String | **yes** | Unique identifier for this specific conversation thread |
| `starred` | bool | no | Flag indicating if this conversation has been marked as important/starred by the user |

### `GetConversationSuccessfulResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Boolean value as the API response. |
| `conversation` | [`ConversationDto`](#conversationdto) | **yes** | Conversation data of the provided conversation ID. |

### `GetEmailMessageResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the email message |
| `altId` | String | no | External Id |
| `threadId` | String | **yes** | Message Id or thread Id |
| `locationId` | String | **yes** | Location ID associated with the email message |
| `contactId` | String | **yes** | Contact ID associated with the email message |
| `conversationId` | String | **yes** | Conversation ID associated with the email message |
| `dateAdded` | String | **yes** | Timestamp when the email message was created |
| `subject` | String | no | Subject line of the email message |
| `body` | String | **yes** | Body content of the email message |
| `direction` | String — `inbound`, `outbound` | **yes** | Direction of the email message |
| `status` | String — `pending`, `scheduled`, `sent`, `delivered`, `read`, `undelivered`, `connected`, `failed`, `opened` | no | Delivery status of the email message |
| `contentType` | String | **yes** | Content type of the email body |
| `attachments` | Vec<String> | no | An array of attachment URLs. |
| `provider` | String | no | Email provider used to send or receive the message |
| `from` | String | **yes** | Name and Email Id of the sender |
| `to` | Vec<String> | **yes** | List of email Ids of the receivers |
| `cc` | Vec<String> | no | List of email Ids of the people in the cc field |
| `bcc` | Vec<String> | no | List of email Ids of the people in the bcc field |
| `replyToMessageId` | String | no | In case of reply, email message Id of the reply to email |
| `source` | String — `workflow`, `bulk_actions`, `campaign`, `api`, `app` | no | Email source |
| `conversationProviderId` | String | no | Conversation provider ID |

### `GetMessageResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the message |
| `type` | f64 | **yes** | Numeric message type code |
| `messageType` | String — 42 values ([shared](shared-enums.md)) | **yes** | Type of the message as a string |
| `locationId` | String | **yes** | Location ID associated with the message |
| `contactId` | String | **yes** | Contact ID associated with the message |
| `conversationId` | String | **yes** | Conversation ID associated with the message |
| `dateAdded` | String | **yes** | Timestamp when the message was created |
| `body` | String | no | Body content of the message |
| `direction` | String — `inbound`, `outbound` | **yes** | Direction of the message |
| `status` | String — `connected`, `delivered`, `failed`, `opened`, `pending`, `read`, `scheduled`, `sent`, `undelivered`, `clicked`, `opt_out`, `queued` | no | Delivery status of the message |
| `contentType` | String | **yes** | Content type of the message body |
| `attachments` | Vec<String> | no | An array of attachment URLs. Attachments will be empty for Call and Voicemails, type 1 and 10. Please use get call recording API to fetch call recording and voicemails. |
| `meta` | [`MessageMeta`](#messagemeta) | no | Additional metadata associated with the message |
| `source` | String — `workflow`, `bulk_actions`, `campaign`, `api`, `app` | no | Message source |
| `userId` | String | no | User Id |
| `conversationProviderId` | String | no | Conversation Provider Id |
| `chatWidgetId` | String | no | Chat Widget Id |

### `GetMessageTranscriptionResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `mediaChannel` | f64 | **yes** | Media channel describes the user interaction channel |
| `sentenceIndex` | f64 | **yes** | Index of the sentence in the transcription |
| `startTime` | f64 | **yes** | Start time of the sentence in milliseconds |
| `endTime` | f64 | **yes** | End time of the sentence in milliseconds |
| `transcript` | String | **yes** | Transcript of the sentence |
| `confidence` | f64 | **yes** | Confidence of the transcription |

### `GetMessagesByConversationResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `lastMessageId` | String | **yes** | Id of the last message in the messages array |
| `nextPage` | bool | **yes** | Next page value true indicates only 20 message is in the response. Rest of the messages are in the next page. Please use the lastMessageId value in the query to get the next page messages |
| `messages` | Vec<GetMessageResponseDto> | **yes** | Array of messages |

### `InitiateFileUploadDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `conversationId` | String | **yes** | Conversation ID |
| `filename` | String | **yes** | Original filename with extension |
| `contentType` | String | **yes** | MIME type of the file |
| `fileSize` | f64 | no | File size in bytes (optional, for pre-validation) |
| `channel` | String | **yes** | Channel type for size limits (WHATSAPP for 100MB limit, others for 5MB) |

### `InitiateFileUploadResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `uploadUrl` | String | **yes** | Signed URL for direct upload to GCS. Use PUT request with file content. |
| `uploadId` | String | **yes** | Unique upload ID for tracking and completing the upload |
| `filePath` | String | **yes** | File path in GCS bucket (needed for confirmation endpoint) |
| `expiresAt` | f64 | **yes** | URL expiration timestamp (Unix milliseconds) |
| `maxFileSize` | f64 | **yes** | Maximum allowed file size in bytes |

### `MessageMeta`

| Field | Type | Required | Description |
|---|---|---|---|
| `callDuration` | String | no | Call duration in seconds |
| `callStatus` | String — `pending`, `completed`, `answered`, `busy`, `no-answer`, `failed`, `canceled`, `voicemail` | no | Call status - can be pending, completed, answered, busy, no-answer, failed, canceled, or voicemail |
| `email` | JSON | no | meta will contain email, for message type 3 (email). messageIds is list of all email message ids under the message thread |

### `ProcessMessageBodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `SMS`, `RCS`, `Email`, `WhatsApp`, `GMB`, `IG`, `FB`, `Custom`, `WebChat`, `Live_Chat`, `Call`, `IVR_Call`, `Campaign_Call`, `Campaign_VoiceMail`, `TIKTOK`, `ALL_IN_ONE_CHAT`, `FORM_SUBMISSION` | **yes** | Message Type |
| `attachments` | Vec<String> | no | Array of attachments |
| `message` | String | no | Message Body |
| `conversationId` | String | **yes** | Conversation Id |
| `contactId` | String | **yes** | Contact Id |
| `conversationProviderId` | String | **yes** | Conversation Provider Id |
| `html` | String | no | HTML Body of Email |
| `subject` | String | no | Subject of the Email |
| `emailFrom` | String | no | Email address to send from. This field is associated with the contact record and cannot be dynamically changed. |
| `emailTo` | String | no | Recipient email address. This field is associated with the contact record and cannot be dynamically changed. |
| `emailCc` | Vec<String> | no | List of email address to CC |
| `emailBcc` | Vec<String> | no | List of email address to BCC |
| `emailMessageId` | String | no | Send the email message id for which this email should be threaded. This is for replying to a specific email |
| `altId` | String | no | external mail provider's message id |
| `direction` | JSON | no | Message direction, if required can be set manually, default is outbound |
| `date` | String | no | Date of the inbound message |
| `call` | [`CallDataDTO`](#calldatadto) | no | Phone call dialer and receiver information |

### `ProcessMessageResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Whether the message processing succeeded |
| `conversationId` | String | **yes** | Conversation ID. |
| `messageId` | String | **yes** | This is the main Message ID |
| `message` | String | **yes** | Result message returned after processing |
| `contactId` | String | no | Contact ID associated with the processed message |
| `dateAdded` | String | no | Timestamp when the processed message was created |
| `emailMessageId` | String | no | Email message ID created for email conversations |

### `ProcessOutboundMessageBodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `Call` | **yes** | Message Type |
| `attachments` | Vec<String> | no | Array of attachments |
| `conversationId` | String | **yes** | Conversation Id |
| `conversationProviderId` | String | **yes** | Conversation Provider Id |
| `altId` | String | no | external mail provider's message id |
| `date` | String | no | Date of the outbound message |
| `call` | [`CallDataDTO`](#calldatadto) | no | Phone call dialer and receiver information |

### `SendConversationResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `conversations` | Vec<ConversationSchema> | **yes** | The list of all conversations found for the given query |
| `total` | f64 | **yes** | Total Number of results found for the given query |

### `SendMessageBodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `SMS`, `RCS`, `Email`, `WhatsApp`, `IG`, `FB`, `Custom`, `Live_Chat`, `TIKTOK` | **yes** | Type of message being sent |
| `subType` | JSON | **yes** | Type of message being sent |
| `contactId` | String | **yes** | ID of the contact receiving the message |
| `appointmentId` | String | no | ID of the associated appointment |
| `attachments` | Vec<String> | no | Array of attachment URLs |
| `emailFrom` | String | no | Email address to send from |
| `emailCc` | Vec<String> | no | Array of CC email addresses |
| `emailBcc` | Vec<String> | no | Array of BCC email addresses |
| `html` | String | no | HTML content of the message |
| `message` | String | no | Text content of the message |
| `subject` | String | no | Subject line for email messages |
| `replyMessageId` | String | no | ID of message being replied to |
| `templateId` | String | no | ID of message template |
| `threadId` | String | no | ID of message thread. For email messages, this is the message ID that contains multiple email messages in the thread |
| `scheduledTimestamp` | f64 | no | UTC Timestamp (in seconds) at which the message should be scheduled |
| `conversationProviderId` | String | no | ID of conversation provider |
| `emailTo` | String | no | Email address to send to, if different from contact's primary email. This should be a valid email address associated with the contact. |
| `customSubtypeId` | String | no | Custom subtype ID for email unsubscription preferences. Only applies to email messages. |
| `emailReplyMode` | String — `reply`, `reply_all` | no | Mode for email replies |
| `fromNumber` | String | no | Phone number used as the sender number for outbound messages |
| `toNumber` | String | no | Recipient phone number for outbound messages |
| `forward` | [`ForwardConfigDto`](#forwardconfigdto) | no | Forwarding configuration for emails |
| `status` | String — `delivered`, `failed`, `pending`, `read` | **yes** | Message status |
| `usesNativeSchedulingAi` | bool | no | Whether the scheduled email uses native AI for the email scheduling |
| `optimizationPeriod` | String — `24h`, `48h`, `72h` | no | Optimization period in hours (24h, 48h, or 72h) |

### `SendMessageResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `conversationId` | String | **yes** | Conversation ID. |
| `emailMessageId` | String | no | This contains the email message ID (only for Email type). Use this ID to send inbound replies through this API to create a threaded email. |
| `messageId` | String | **yes** | This is the main Message ID |
| `messageIds` | Vec<String> | no | When sending via the GMB channel, we will be returning list of `messageIds` instead of single `messageId`. |
| `msg` | String | no | Additional response message when sending a workflow message |
| `forwardData` | [`ForwardResponseDto`](#forwardresponsedto) | no | Optional metadata for forwarded email |
| `status` | String — `delivered`, `failed`, `pending`, `read` | **yes** | Message status |

### `SendReviewReplyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `conversationId` | String | **yes** | Conversation ID (must have reviewId) |
| `locationId` | String | **yes** | Location ID |
| `message` | String | **yes** | Review reply message text |

### `StartAfterArrayNumberSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `startAfterDate` | Vec<String> | no | Search to begin after the specified date - should contain the sort value of the last document |

### `StartAfterNumberSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `startAfterDate` | f64 | no | Search to begin after the specified date - should contain the sort value of the last document |

### `UpdateConversationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID as string |
| `unreadCount` | f64 | no | Count of unread messages in the conversation |
| `starred` | bool | no | Starred status of the conversation. |
| `feedback` | String — `positive`, `negative` | no | Live chat feedback value for the conversation |

### `UpdateEmailMessageStatusDto`

Request body to update the delivery status of an email message. `status` is required. `events` and `recipients` are optional and may be included together with `status`. - Use `events` to report aggregate delivery metrics (delivered count, open count, etc.). - Use `recipients` to track per-recipient delivery outcomes for multi-recipient emails. - Use `status` to set the overall message status. Mult…

| Field | Type | Required | Description |
|---|---|---|---|
| `events` | [`EmailEventsDto`](#emaileventsdto) | no | Aggregate delivery event counters. Counters are merged into existing values. The API automatically infers related events (e.g., reporting `clicked` will also set `opened` and `delivered` if not alread… |
| `recipients` | Vec<UpdateRecipientMessageStatusDto> | no | Per-recipient delivery statuses. Each entry maps a recipient email address to a delivery status. Entries are upserted — if a recipient already has a status, it will be overwritten with the new value. |
| `status` | String — `pending`, `scheduled`, `sent`, `delivered`, `read`, `undelivered`, `connected`, `failed`, `opened`, `clicked` | **yes** | The overall status of the email message. Required on every request. For emails with multiple recipients, consider using the `recipients` array for granular tracking and this field for the aggregate st… |

### `UpdateEmailMessageStatusResponseDto`

Response returned after successfully updating the email message status.

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Whether the status update was persisted successfully. |
| `message` | String | **yes** | Human-readable result message. |

### `UpdateMessageStatusDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | String — `delivered`, `failed`, `pending`, `read` | **yes** | Message status |
| `error` | [`ErrorDto`](#errordto) | no | Error object from the conversation provider |
| `emailMessageId` | String | no | Email message Id |
| `recipients` | Vec<String> | no | Email delivery status for additional email recipients. |

### `UpdateRecipientMessageStatusDto`

Delivery status for an individual recipient of a multi-recipient email. Each entry is keyed by the recipient's email address and tracks their specific delivery outcome independently from the aggregate event counters.

| Field | Type | Required | Description |
|---|---|---|---|
| `emailId` | String | **yes** | The recipient's email address. This is used as the key to store and update the per-recipient status. Must match one of the original recipients of the email. |
| `status` | String — `pending`, `scheduled`, `sent`, `delivered`, `read`, `undelivered`, `connected`, `failed`, `opened`, `clicked` | **yes** | The delivery status for this specific recipient. Common values for status updates: `delivered` (successfully delivered), `failed` (delivery failed — provide `failReason`), `opened` (recipient opened t… |
| `failReason` | String | no | Human-readable reason for the delivery failure. Only applicable when `status` is `failed`. Examples: "Mailbox not found", "Quota exceeded", "Blocked by recipient server". |

### `UploadFilesDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `conversationId` | String | **yes** | Conversation Id |
| `contactId` | String | **yes** | Contact Id |
| `locationId` | String | **yes** | Location ID associated with the upload request |
| `attachmentUrls` | Vec<String> | **yes** | Array of attachment URLs to upload for the conversation |
| `chatServiceSid` | String | no | Twilio chat service SID for group SMS uploads |
| `isGroupSms` | String | no | Flag to indicate group SMS upload flow. When true, only 1 file upload is allowed per request. |

### `UploadFilesErrorResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | String — `400`, `413`, `415` | **yes** | HTTP Status code of the request |
| `message` | String | **yes** | Error message of the request |

### `UploadFilesResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `uploadedFiles` | JSON | **yes** | Map of uploaded file names to their accessible URLs |
| `twilioMediaSids` | Vec<String> | no | Twilio media SIDs for group SMS (when isGroupSms=true) |

### `UserTypingBody`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location Id |
| `isTyping` | String | **yes** | Typing status |
| `visitorId` | String | **yes** | Unique ID assigned to each live chat visitor. |
| `conversationId` | String | **yes** | Conversation Id |

