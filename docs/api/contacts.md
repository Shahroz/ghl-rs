# `contacts`

**32** operations / **61** models in API v2 · **31** operations / **60** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `contacts` cargo feature on `ghl-sdk`, then call any of the 63 generated methods on `ghl.contacts()` (v2) or `ghl.v3().contacts()` (v3):

```toml
ghl-sdk = { version = "0.5", features = ["contacts"] }
```

This module also has hand-written ergonomic helpers on the same `ghl.contacts()`: `create()`, `get()`, `update()`, `delete()`, `list()` (envelope unwrapping, paginated `Stream`s).

MCP tools: `ghl_search_contacts`, `ghl_get_contact`, `ghl_create_contact`, `ghl_update_contact`, `ghl_delete_contact`.


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/contacts/` | Get Contacts | `get_contacts()` | `contacts.get_contacts` |
| `POST` | `/contacts/` | Create Contact | `create_contact()` | `contacts.post_contacts` |
| `POST` | `/contacts/bulk/business` | Add/Remove Contacts From Business | `add_remove_contacts_from_business()` | `contacts.post_contacts_bulk_business` |
| `POST` | `/contacts/bulk/tags/update/{type}` | Update Contacts Tags | `update_contacts_tags()` | `contacts.post_contacts_bulk_tags_update_by_type` |
| `GET` | `/contacts/business/{businessId}` | Get Contacts By BusinessId | `get_contacts_by_business_id()` | `contacts.get_contacts_business_by_businessId` |
| `POST` | `/contacts/search` | Search Contacts | `search_contacts()` | `contacts.post_contacts_search` |
| `GET` | `/contacts/search/duplicate` | Get Duplicate Contact | `get_duplicate_contact()` | `contacts.get_contacts_search_duplicate` |
| `POST` | `/contacts/upsert` | Upsert Contact | `upsert_contact()` | `contacts.post_contacts_upsert` |
| `DELETE` | `/contacts/{contactId}` | Delete Contact | `delete_contact()` | `contacts.delete_contacts_by_contactId` |
| `GET` | `/contacts/{contactId}` | Get Contact | `get_contact()` | `contacts.get_contacts_by_contactId` |
| `PUT` | `/contacts/{contactId}` | Update Contact | `update_contact()` | `contacts.put_contacts_by_contactId` |
| `GET` | `/contacts/{contactId}/appointments` | Get Appointments for Contact | `get_appointments_for_contact()` | `contacts.get_contacts_by_contactId_appointments` |
| `DELETE` | `/contacts/{contactId}/campaigns/removeAll` | Remove Contact From Every Campaign | `remove_contact_from_every_campaign()` | `contacts.delete_contacts_by_contactId_campaigns_removeAll` |
| `DELETE` | `/contacts/{contactId}/campaigns/{campaignId}` | Remove Contact From Campaign | `remove_contact_from_campaign()` | `contacts.delete_contacts_by_contactId_campaigns_by_campaignId` |
| `POST` | `/contacts/{contactId}/campaigns/{campaignId}` | Add Contact to Campaign | `add_contact_to_campaign()` | `contacts.post_contacts_by_contactId_campaigns_by_campaignId` |
| `DELETE` | `/contacts/{contactId}/followers` | Remove Followers | `remove_followers()` | `contacts.delete_contacts_by_contactId_followers` |
| `POST` | `/contacts/{contactId}/followers` | Add Followers | `add_followers()` | `contacts.post_contacts_by_contactId_followers` |
| `GET` | `/contacts/{contactId}/notes` | Get All Notes | `get_all_notes()` | `contacts.get_contacts_by_contactId_notes` |
| `POST` | `/contacts/{contactId}/notes` | Create Note | `create_note()` | `contacts.post_contacts_by_contactId_notes` |
| `DELETE` | `/contacts/{contactId}/notes/{id}` | Delete Note | `delete_note()` | `contacts.delete_contacts_by_contactId_notes_by_id` |
| `GET` | `/contacts/{contactId}/notes/{id}` | Get Note | `get_note()` | `contacts.get_contacts_by_contactId_notes_by_id` |
| `PUT` | `/contacts/{contactId}/notes/{id}` | Update Note | `update_note()` | `contacts.put_contacts_by_contactId_notes_by_id` |
| `DELETE` | `/contacts/{contactId}/tags` | Remove Tags | `remove_tags()` | `contacts.delete_contacts_by_contactId_tags` |
| `POST` | `/contacts/{contactId}/tags` | Add Tags | `add_tags()` | `contacts.post_contacts_by_contactId_tags` |
| `GET` | `/contacts/{contactId}/tasks` | Get all Tasks | `get_all_tasks()` | `contacts.get_contacts_by_contactId_tasks` |
| `POST` | `/contacts/{contactId}/tasks` | Create Task | `create_task()` | `contacts.post_contacts_by_contactId_tasks` |
| `DELETE` | `/contacts/{contactId}/tasks/{taskId}` | Delete Task | `delete_task()` | `contacts.delete_contacts_by_contactId_tasks_by_taskId` |
| `GET` | `/contacts/{contactId}/tasks/{taskId}` | Get Task | `get_task()` | `contacts.get_contacts_by_contactId_tasks_by_taskId` |
| `PUT` | `/contacts/{contactId}/tasks/{taskId}` | Update Task | `update_task()` | `contacts.put_contacts_by_contactId_tasks_by_taskId` |
| `PUT` | `/contacts/{contactId}/tasks/{taskId}/completed` | Update Task Completed | `update_task_completed()` | `contacts.put_contacts_by_contactId_tasks_by_taskId_completed` |
| `DELETE` | `/contacts/{contactId}/workflow/{workflowId}` | Delete Contact from Workflow | `delete_contact_from_workflow()` | `contacts.delete_contacts_by_contactId_workflow_by_workflowId` |
| `POST` | `/contacts/{contactId}/workflow/{workflowId}` | Add Contact to Workflow | `add_contact_to_workflow()` | `contacts.post_contacts_by_contactId_workflow_by_workflowId` |

### Endpoint details — v2

#### `GET /contacts/`

**Get Contacts**

Get Contacts **Note:** This API endpoint is deprecated. Please use the [Search Contacts](https://marketplace.gohighlevel.com/docs/ghl/contacts/search-contacts-advanced) endpoint instead.

Operation id: `contacts.get_contacts` · `Version: 2021-07-28` · Scopes: `contacts.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `startAfterId` | string | no | Start After Id |
| `startAfter` | number | no | Start Afte |
| `query` | string | no | Contact Query |
| `limit` | number | no | Limit Per Page records count. will allow maximum up to 100 and default will be 20 |

*Response*: [`ContactsSearchSuccessfulResponseDto`](#contactssearchsuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::contacts::GetContactsParams;

let params = GetContactsParams::new("locationId");
let out = ghl.contacts().get_contacts(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.get_contacts",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /contacts/`

**Create Contact**

Please find the list of acceptable values for the `country` field <a href="https://highlevel.stoplight.io/docs/integrations/ZG9jOjI4MzUzNDIy-country-list" target="_blank">here</a>

Operation id: `contacts.post_contacts` · `Version: 2021-07-28` · Scopes: `contacts.write`

*Request body*: [`CreateContactDto`](#createcontactdto)

*Response*: [`CreateContactsSuccessfulResponseDto`](#createcontactssuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().create_contact(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.post_contacts",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /contacts/bulk/business`

**Add/Remove Contacts From Business**

Add/Remove Contacts From Business . Passing a `null` businessId will remove the businessId from the contacts

Operation id: `contacts.post_contacts_bulk_business` · `Version: 2021-07-28`

*Request body*: [`ContactsBusinessUpdate`](#contactsbusinessupdate)

*Response*: [`ContactsBulkUpateResponse`](#contactsbulkupateresponse)

*Rust*:

```rust,ignore
let out = ghl.contacts().add_remove_contacts_from_business(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.post_contacts_bulk_business",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /contacts/bulk/tags/update/{type}`

**Update Contacts Tags**

Allows you to update tags to multiple contacts at once, you can add or remove tags from the contacts

Operation id: `contacts.post_contacts_bulk_tags_update_by_type` · `Version: 2021-07-28`

*Request body*: [`UpdateTagsDTO`](#updatetagsdto)

*Response*: [`UpdateTagsResponseDTO`](#updatetagsresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().update_contacts_tags(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.post_contacts_bulk_tags_update_by_type",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /contacts/business/{businessId}`

**Get Contacts By BusinessId**

Operation id: `contacts.get_contacts_business_by_businessId` · `Version: 2021-07-28` · Scopes: `contacts.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `businessId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | string | no | — |
| `locationId` | string | **yes** | — |
| `skip` | string | no | — |
| `query` | string | no | — |

*Response*: [`ContactsSearchSuccessfulResponseDto`](#contactssearchsuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::contacts::GetContactsByBusinessIdParams;

let params = GetContactsByBusinessIdParams::new("locationId");
let out = ghl.contacts().get_contacts_by_business_id(&businessId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.get_contacts_business_by_businessId",
    "path_params": {
      "businessId": "<businessId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /contacts/search`

**Search Contacts**

Search contacts based on combinations of advanced filters. Documentation Link - https://doc.clickup.com/8631005/d/h/87cpx-158396/6e629989abe7fad

Operation id: `contacts.post_contacts_search` · `Version: 2021-07-28` · Scopes: `contacts.readonly`

*Request body*: [`SearchBodyV2DTO`](#searchbodyv2dto)

*Rust*:

```rust,ignore
let out = ghl.contacts().search_contacts(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.post_contacts_search",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /contacts/search/duplicate`

**Get Duplicate Contact**

Get Duplicate Contact. If `Allow Duplicate Contact` is disabled under Settings, the global unique identifier will be used for searching the contact. If the setting is enabled, first priority for search is `email` and the second priority will be `phone`.

Operation id: `contacts.get_contacts_search_duplicate` · `Version: 2021-07-28` · Scopes: `contacts.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `number` | string | no | Phone Number - Pass in URL Encoded form. i.e +1423164516 will become `%2B1423164516` |
| `email` | string | no | Email - Pass in URL Encoded form. i.e test+abc@gmail.com will become `test%2Babc%40gmail.com` |

*Rust*:

```rust,ignore
use ghl_sdk::services::contacts::GetDuplicateContactParams;

let params = GetDuplicateContactParams::new("locationId");
let out = ghl.contacts().get_duplicate_contact(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.get_contacts_search_duplicate",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /contacts/upsert`

**Upsert Contact**

Please find the list of acceptable values for the `country` field <a href="https://highlevel.stoplight.io/docs/integrations/ZG9jOjI4MzUzNDIy-country-list" target="_blank">here</a> The Upsert API will adhere to the configuration defined under the “Allow Duplicate Contact” setting at the Location level. If the setting is configured to check both Email and Phone, the API will attempt to identify an existing contact based on the priority sequence specified in the setting, and will create or update t…

Operation id: `contacts.post_contacts_upsert` · `Version: 2021-07-28` · Scopes: `contacts.write`

*Request body*: [`UpsertContactDto`](#upsertcontactdto)

*Response*: [`UpsertContactsSuccessfulResponseDto`](#upsertcontactssuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().upsert_contact(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.post_contacts_upsert",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /contacts/{contactId}`

**Delete Contact**

Operation id: `contacts.delete_contacts_by_contactId` · `Version: 2021-07-28` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Response*: [`DeleteContactsSuccessfulResponseDto`](#deletecontactssuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().delete_contact(&contactId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.delete_contacts_by_contactId",
    "path_params": {
      "contactId": "<contactId>"
    }
  }
}
```

</details>

#### `GET /contacts/{contactId}`

**Get Contact**

Operation id: `contacts.get_contacts_by_contactId` · `Version: 2021-07-28` · Scopes: `contacts.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Response*: [`ContactsByIdSuccessfulResponseDto`](#contactsbyidsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().get_contact(&contactId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.get_contacts_by_contactId",
    "path_params": {
      "contactId": "<contactId>"
    }
  }
}
```

</details>

#### `PUT /contacts/{contactId}`

**Update Contact**

Please find the list of acceptable values for the `country` field <a href="https://highlevel.stoplight.io/docs/integrations/ZG9jOjI4MzUzNDIy-country-list" target="_blank">here</a>

Operation id: `contacts.put_contacts_by_contactId` · `Version: 2021-07-28` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Request body*: [`UpdateContactDto`](#updatecontactdto)

*Response*: [`UpdateContactsSuccessfulResponseDto`](#updatecontactssuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().update_contact(&contactId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.put_contacts_by_contactId",
    "path_params": {
      "contactId": "<contactId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /contacts/{contactId}/appointments`

**Get Appointments for Contact**

Operation id: `contacts.get_contacts_by_contactId_appointments` · `Version: 2021-07-28` · Scopes: `contacts.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Response*: [`GetEventsSuccessfulResponseDto`](#geteventssuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().get_appointments_for_contact(&contactId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.get_contacts_by_contactId_appointments",
    "path_params": {
      "contactId": "<contactId>"
    }
  }
}
```

</details>

#### `DELETE /contacts/{contactId}/campaigns/removeAll`

**Remove Contact From Every Campaign**

Operation id: `contacts.delete_contacts_by_contactId_campaigns_removeAll` · `Version: 2021-07-28` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Response*: [`CreateDeleteCantactsCampaignsSuccessfulResponseDto`](#createdeletecantactscampaignssuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().remove_contact_from_every_campaign(&contactId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.delete_contacts_by_contactId_campaigns_removeAll",
    "path_params": {
      "contactId": "<contactId>"
    }
  }
}
```

</details>

#### `DELETE /contacts/{contactId}/campaigns/{campaignId}`

**Remove Contact From Campaign**

Operation id: `contacts.delete_contacts_by_contactId_campaigns_by_campaignId` · `Version: 2021-07-28` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `campaignId` | string | **yes** | Campaigns Id |

*Response*: [`CreateDeleteCantactsCampaignsSuccessfulResponseDto`](#createdeletecantactscampaignssuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().remove_contact_from_campaign(&contactId, &campaignId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.delete_contacts_by_contactId_campaigns_by_campaignId",
    "path_params": {
      "contactId": "<contactId>",
      "campaignId": "<campaignId>"
    }
  }
}
```

</details>

#### `POST /contacts/{contactId}/campaigns/{campaignId}`

**Add Contact to Campaign**

Add contact to Campaign

Operation id: `contacts.post_contacts_by_contactId_campaigns_by_campaignId` · `Version: 2021-07-28` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `campaignId` | string | **yes** | Campaigns Id |

*Request body*: [`AddContactToCampaignDto`](#addcontacttocampaigndto)

*Response*: [`CreateDeleteCantactsCampaignsSuccessfulResponseDto`](#createdeletecantactscampaignssuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().add_contact_to_campaign(&contactId, &campaignId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.post_contacts_by_contactId_campaigns_by_campaignId",
    "path_params": {
      "contactId": "<contactId>",
      "campaignId": "<campaignId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /contacts/{contactId}/followers`

**Remove Followers**

Operation id: `contacts.delete_contacts_by_contactId_followers` · `Version: 2021-07-28` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Request body*: [`FollowersDTO`](#followersdto)

*Response*: [`DeleteFollowersSuccessfulResponseDto`](#deletefollowerssuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().remove_followers(&contactId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.delete_contacts_by_contactId_followers",
    "path_params": {
      "contactId": "<contactId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /contacts/{contactId}/followers`

**Add Followers**

Operation id: `contacts.post_contacts_by_contactId_followers` · `Version: 2021-07-28` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Request body*: [`FollowersDTO`](#followersdto)

*Response*: [`CreateAddFollowersSuccessfulResponseDto`](#createaddfollowerssuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().add_followers(&contactId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.post_contacts_by_contactId_followers",
    "path_params": {
      "contactId": "<contactId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /contacts/{contactId}/notes`

**Get All Notes**

Operation id: `contacts.get_contacts_by_contactId_notes` · `Version: 2021-07-28` · Scopes: `contacts.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Response*: [`GetNotesListSuccessfulResponseDto`](#getnoteslistsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().get_all_notes(&contactId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.get_contacts_by_contactId_notes",
    "path_params": {
      "contactId": "<contactId>"
    }
  }
}
```

</details>

#### `POST /contacts/{contactId}/notes`

**Create Note**

Operation id: `contacts.post_contacts_by_contactId_notes` · `Version: 2021-07-28` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Request body*: [`NotesDTO`](#notesdto)

*Response*: [`GetCreateUpdateNoteSuccessfulResponseDto`](#getcreateupdatenotesuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().create_note(&contactId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.post_contacts_by_contactId_notes",
    "path_params": {
      "contactId": "<contactId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /contacts/{contactId}/notes/{id}`

**Delete Note**

Operation id: `contacts.delete_contacts_by_contactId_notes_by_id` · `Version: 2021-07-28` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `id` | string | **yes** | Note Id |

*Response*: [`DeleteNoteSuccessfulResponseDto`](#deletenotesuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().delete_note(&contactId, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.delete_contacts_by_contactId_notes_by_id",
    "path_params": {
      "contactId": "<contactId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /contacts/{contactId}/notes/{id}`

**Get Note**

Operation id: `contacts.get_contacts_by_contactId_notes_by_id` · `Version: 2021-07-28` · Scopes: `contacts.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `id` | string | **yes** | Note Id |

*Response*: [`GetCreateUpdateNoteSuccessfulResponseDto`](#getcreateupdatenotesuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().get_note(&contactId, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.get_contacts_by_contactId_notes_by_id",
    "path_params": {
      "contactId": "<contactId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `PUT /contacts/{contactId}/notes/{id}`

**Update Note**

Operation id: `contacts.put_contacts_by_contactId_notes_by_id` · `Version: 2021-07-28` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `id` | string | **yes** | Note Id |

*Request body*: [`UpdateNoteDTO`](#updatenotedto)

*Response*: [`GetCreateUpdateNoteSuccessfulResponseDto`](#getcreateupdatenotesuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().update_note(&contactId, &id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.put_contacts_by_contactId_notes_by_id",
    "path_params": {
      "contactId": "<contactId>",
      "id": "<id>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /contacts/{contactId}/tags`

**Remove Tags**

Operation id: `contacts.delete_contacts_by_contactId_tags` · `Version: 2021-07-28` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Request body*: [`TagsDTO`](#tagsdto)

*Response*: [`CreateDeleteTagSuccessfulResponseDto`](#createdeletetagsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().remove_tags(&contactId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.delete_contacts_by_contactId_tags",
    "path_params": {
      "contactId": "<contactId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /contacts/{contactId}/tags`

**Add Tags**

Operation id: `contacts.post_contacts_by_contactId_tags` · `Version: 2021-07-28` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Request body*: [`TagsDTO`](#tagsdto)

*Response*: [`CreateAddTagSuccessfulResponseDto`](#createaddtagsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().add_tags(&contactId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.post_contacts_by_contactId_tags",
    "path_params": {
      "contactId": "<contactId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /contacts/{contactId}/tasks`

**Get all Tasks**

Operation id: `contacts.get_contacts_by_contactId_tasks` · `Version: 2021-07-28` · Scopes: `contacts.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Response*: [`TasksListSuccessfulResponseDto`](#taskslistsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().get_all_tasks(&contactId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.get_contacts_by_contactId_tasks",
    "path_params": {
      "contactId": "<contactId>"
    }
  }
}
```

</details>

#### `POST /contacts/{contactId}/tasks`

**Create Task**

Operation id: `contacts.post_contacts_by_contactId_tasks` · `Version: 2021-07-28` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Request body*: [`CreateTaskParams`](#createtaskparams)

*Response*: [`TaskByIsSuccessfulResponseDto`](#taskbyissuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().create_task(&contactId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.post_contacts_by_contactId_tasks",
    "path_params": {
      "contactId": "<contactId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /contacts/{contactId}/tasks/{taskId}`

**Delete Task**

Operation id: `contacts.delete_contacts_by_contactId_tasks_by_taskId` · `Version: 2021-07-28` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `taskId` | string | **yes** | Task Id |

*Response*: [`DeleteTaskSuccessfulResponseDto`](#deletetasksuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().delete_task(&contactId, &taskId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.delete_contacts_by_contactId_tasks_by_taskId",
    "path_params": {
      "contactId": "<contactId>",
      "taskId": "<taskId>"
    }
  }
}
```

</details>

#### `GET /contacts/{contactId}/tasks/{taskId}`

**Get Task**

Operation id: `contacts.get_contacts_by_contactId_tasks_by_taskId` · `Version: 2021-07-28` · Scopes: `contacts.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `taskId` | string | **yes** | Task Id |

*Response*: [`TaskByIsSuccessfulResponseDto`](#taskbyissuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().get_task(&contactId, &taskId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.get_contacts_by_contactId_tasks_by_taskId",
    "path_params": {
      "contactId": "<contactId>",
      "taskId": "<taskId>"
    }
  }
}
```

</details>

#### `PUT /contacts/{contactId}/tasks/{taskId}`

**Update Task**

Operation id: `contacts.put_contacts_by_contactId_tasks_by_taskId` · `Version: 2021-07-28` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `taskId` | string | **yes** | Task Id |

*Request body*: [`UpdateTaskBody`](#updatetaskbody)

*Response*: [`TaskByIsSuccessfulResponseDto`](#taskbyissuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().update_task(&contactId, &taskId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.put_contacts_by_contactId_tasks_by_taskId",
    "path_params": {
      "contactId": "<contactId>",
      "taskId": "<taskId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /contacts/{contactId}/tasks/{taskId}/completed`

**Update Task Completed**

Operation id: `contacts.put_contacts_by_contactId_tasks_by_taskId_completed` · `Version: 2021-07-28` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `taskId` | string | **yes** | Task Id |

*Request body*: [`UpdateTaskStatusParams`](#updatetaskstatusparams)

*Response*: [`TaskByIsSuccessfulResponseDto`](#taskbyissuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().update_task_completed(&contactId, &taskId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.put_contacts_by_contactId_tasks_by_taskId_completed",
    "path_params": {
      "contactId": "<contactId>",
      "taskId": "<taskId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /contacts/{contactId}/workflow/{workflowId}`

**Delete Contact from Workflow**

Operation id: `contacts.delete_contacts_by_contactId_workflow_by_workflowId` · `Version: 2021-07-28` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `workflowId` | string | **yes** | Workflow Id |

*Request body*: [`CreateWorkflowDto`](#createworkflowdto)

*Response*: [`ContactsWorkflowSuccessfulResponseDto`](#contactsworkflowsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().delete_contact_from_workflow(&contactId, &workflowId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.delete_contacts_by_contactId_workflow_by_workflowId",
    "path_params": {
      "contactId": "<contactId>",
      "workflowId": "<workflowId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /contacts/{contactId}/workflow/{workflowId}`

**Add Contact to Workflow**

Operation id: `contacts.post_contacts_by_contactId_workflow_by_workflowId` · `Version: 2021-07-28` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `workflowId` | string | **yes** | Workflow Id |

*Request body*: [`CreateWorkflowDto`](#createworkflowdto)

*Response*: [`ContactsWorkflowSuccessfulResponseDto`](#contactsworkflowsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.contacts().add_contact_to_workflow(&contactId, &workflowId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "contacts.post_contacts_by_contactId_workflow_by_workflowId",
    "path_params": {
      "contactId": "<contactId>",
      "workflowId": "<workflowId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

## Endpoints — API v3

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `POST` | `/contacts/` | Create Contact | `create_contact()` | `v3:contacts.post_contacts` |
| `POST` | `/contacts/bulk/business` | Add/Remove Contacts From Business | `add_remove_contacts_from_business()` | `v3:contacts.post_contacts_bulk_business` |
| `POST` | `/contacts/bulk/tags/update/{type}` | Update Contacts Tags | `update_contacts_tags()` | `v3:contacts.post_contacts_bulk_tags_update_by_type` |
| `GET` | `/contacts/business/{businessId}` | Get Contacts By BusinessId | `get_contacts_by_business_id()` | `v3:contacts.get_contacts_business_by_businessId` |
| `POST` | `/contacts/search` | Search Contacts | `search_contacts()` | `v3:contacts.post_contacts_search` |
| `GET` | `/contacts/search/duplicate` | Get Duplicate Contact | `get_duplicate_contact()` | `v3:contacts.get_contacts_search_duplicate` |
| `POST` | `/contacts/upsert` | Upsert Contact | `upsert_contact()` | `v3:contacts.post_contacts_upsert` |
| `DELETE` | `/contacts/{contactId}` | Delete Contact | `delete_contact()` | `v3:contacts.delete_contacts_by_contactId` |
| `GET` | `/contacts/{contactId}` | Get Contact | `get_contact()` | `v3:contacts.get_contacts_by_contactId` |
| `PUT` | `/contacts/{contactId}` | Update Contact | `update_contact()` | `v3:contacts.put_contacts_by_contactId` |
| `GET` | `/contacts/{contactId}/appointments` | Get Appointments for Contact | `get_appointments_for_contact()` | `v3:contacts.get_contacts_by_contactId_appointments` |
| `DELETE` | `/contacts/{contactId}/campaigns/remove-all` | Remove Contact From Every Campaign | `remove_contact_from_every_campaign()` | `v3:contacts.delete_contacts_by_contactId_campaigns_remove_all` |
| `DELETE` | `/contacts/{contactId}/campaigns/{campaignId}` | Remove Contact From Campaign | `remove_contact_from_campaign()` | `v3:contacts.delete_contacts_by_contactId_campaigns_by_campaignId` |
| `POST` | `/contacts/{contactId}/campaigns/{campaignId}` | Add Contact to Campaign | `add_contact_to_campaign()` | `v3:contacts.post_contacts_by_contactId_campaigns_by_campaignId` |
| `DELETE` | `/contacts/{contactId}/followers` | Remove Followers | `remove_followers()` | `v3:contacts.delete_contacts_by_contactId_followers` |
| `POST` | `/contacts/{contactId}/followers` | Add Followers | `add_followers()` | `v3:contacts.post_contacts_by_contactId_followers` |
| `GET` | `/contacts/{contactId}/notes` | Get All Notes | `get_all_notes()` | `v3:contacts.get_contacts_by_contactId_notes` |
| `POST` | `/contacts/{contactId}/notes` | Create Note | `create_note()` | `v3:contacts.post_contacts_by_contactId_notes` |
| `DELETE` | `/contacts/{contactId}/notes/{id}` | Delete Note | `delete_note()` | `v3:contacts.delete_contacts_by_contactId_notes_by_id` |
| `GET` | `/contacts/{contactId}/notes/{id}` | Get Note | `get_note()` | `v3:contacts.get_contacts_by_contactId_notes_by_id` |
| `PUT` | `/contacts/{contactId}/notes/{id}` | Update Note | `update_note()` | `v3:contacts.put_contacts_by_contactId_notes_by_id` |
| `DELETE` | `/contacts/{contactId}/tags` | Remove Tags | `remove_tags()` | `v3:contacts.delete_contacts_by_contactId_tags` |
| `POST` | `/contacts/{contactId}/tags` | Add Tags | `add_tags()` | `v3:contacts.post_contacts_by_contactId_tags` |
| `GET` | `/contacts/{contactId}/tasks` | Get all Tasks | `get_all_tasks()` | `v3:contacts.get_contacts_by_contactId_tasks` |
| `POST` | `/contacts/{contactId}/tasks` | Create Task | `create_task()` | `v3:contacts.post_contacts_by_contactId_tasks` |
| `DELETE` | `/contacts/{contactId}/tasks/{taskId}` | Delete Task | `delete_task()` | `v3:contacts.delete_contacts_by_contactId_tasks_by_taskId` |
| `GET` | `/contacts/{contactId}/tasks/{taskId}` | Get Task | `get_task()` | `v3:contacts.get_contacts_by_contactId_tasks_by_taskId` |
| `PUT` | `/contacts/{contactId}/tasks/{taskId}` | Update Task | `update_task()` | `v3:contacts.put_contacts_by_contactId_tasks_by_taskId` |
| `PUT` | `/contacts/{contactId}/tasks/{taskId}/completed` | Update Task Completed | `update_task_completed()` | `v3:contacts.put_contacts_by_contactId_tasks_by_taskId_completed` |
| `DELETE` | `/contacts/{contactId}/workflow/{workflowId}` | Delete Contact from Workflow | `delete_contact_from_workflow()` | `v3:contacts.delete_contacts_by_contactId_workflow_by_workflowId` |
| `POST` | `/contacts/{contactId}/workflow/{workflowId}` | Add Contact to Workflow | `add_contact_to_workflow()` | `v3:contacts.post_contacts_by_contactId_workflow_by_workflowId` |

### Endpoint details — v3

#### `POST /contacts/`

**Create Contact**

Create a new contact

Operation id: `v3:contacts.post_contacts` · `Version: v3` · Scopes: `contacts.write`

*Request body*: [`CreateContactDtoV3`](#createcontactdtov3)

*Response*: [`CreateContactsSuccessfulResponseDtoV3`](#createcontactssuccessfulresponsedtov3)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().create_contact(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.post_contacts",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /contacts/bulk/business`

**Add/Remove Contacts From Business**

Add/Remove Contacts From Business . Passing a `null` businessId will remove the businessId from the contacts

Operation id: `v3:contacts.post_contacts_bulk_business` · `Version: v3`

*Request body*: [`ContactsBusinessUpdate`](#contactsbusinessupdate)

*Response*: [`ContactsBulkUpateResponse`](#contactsbulkupateresponse)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().add_remove_contacts_from_business(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.post_contacts_bulk_business",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /contacts/bulk/tags/update/{type}`

**Update Contacts Tags**

Allows you to update tags to multiple contacts at once, you can add or remove tags from the contacts

Operation id: `v3:contacts.post_contacts_bulk_tags_update_by_type` · `Version: v3`

*Request body*: [`UpdateTagsDTO`](#updatetagsdto)

*Response*: [`UpdateTagsResponseDTO`](#updatetagsresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().update_contacts_tags(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.post_contacts_bulk_tags_update_by_type",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /contacts/business/{businessId}`

**Get Contacts By BusinessId**

Operation id: `v3:contacts.get_contacts_business_by_businessId` · `Version: v3` · Scopes: `contacts.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `businessId` | string | **yes** | Business Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | string | no | Maximum number of records per page (up to 100, default 25) |
| `locationId` | string | **yes** | Location Id |
| `skip` | string | no | Number of records to skip |
| `query` | string | no | Search query (name, email, phone) |
| `startAfter` | array | no | Cursor for pagination (comma-separated name,id pair) |

*Response*: [`ContactsSearchSuccessfulResponseDto`](#contactssearchsuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::contacts::GetContactsByBusinessIdParams;

let params = GetContactsByBusinessIdParams::new("locationId");
let out = ghl.v3().contacts().get_contacts_by_business_id(&businessId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.get_contacts_business_by_businessId",
    "path_params": {
      "businessId": "<businessId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /contacts/search`

**Search Contacts**

Search contacts based on combinations of advanced filters. Documentation Link - https://doc.clickup.com/8631005/d/h/87cpx-158396/6e629989abe7fad

Operation id: `v3:contacts.post_contacts_search` · `Version: v3` · Scopes: `contacts.readonly`

*Request body*: [`SearchBodyV2DTO`](#searchbodyv2dto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().search_contacts(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.post_contacts_search",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /contacts/search/duplicate`

**Get Duplicate Contact**

Get Duplicate Contact. If `Allow Duplicate Contact` is disabled under Settings, the global unique identifier will be used for searching the contact. If the setting is enabled, first priority for search is `email` and the second priority will be `phone`.

Operation id: `v3:contacts.get_contacts_search_duplicate` · `Version: v3` · Scopes: `contacts.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `number` | string | no | Phone Number — URL-encoded. E.g. +1423164516 → %2B1423164516 |
| `email` | string | no | Email — URL-encoded. E.g. test+abc@gmail.com → test%2Babc%40gmail.com |

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::contacts::GetDuplicateContactParams;

let params = GetDuplicateContactParams::new("locationId");
let out = ghl.v3().contacts().get_duplicate_contact(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.get_contacts_search_duplicate",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /contacts/upsert`

**Upsert Contact**

The Upsert API will adhere to the configuration defined under the "Allow Duplicate Contact" setting at the Location level. If the setting is configured to check both Email and Phone, the API will attempt to identify an existing contact based on the priority sequence specified in the setting, and will create or update the contact accordingly. If two separate contacts already exist—one with the same email and another with the same phone—and an upsert request includes both the email and phone, the …

Operation id: `v3:contacts.post_contacts_upsert` · `Version: v3` · Scopes: `contacts.write`

*Request body*: [`UpsertContactDtoV3`](#upsertcontactdtov3)

*Response*: [`UpsertContactsSuccessfulResponseDtoV3`](#upsertcontactssuccessfulresponsedtov3)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().upsert_contact(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.post_contacts_upsert",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /contacts/{contactId}`

**Delete Contact**

Operation id: `v3:contacts.delete_contacts_by_contactId` · `Version: v3` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Response*: [`DeleteContactsSuccessfulResponseDto`](#deletecontactssuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().delete_contact(&contactId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.delete_contacts_by_contactId",
    "path_params": {
      "contactId": "<contactId>"
    }
  }
}
```

</details>

#### `GET /contacts/{contactId}`

**Get Contact**

Retrieves a contact by its unique identifier.

Operation id: `v3:contacts.get_contacts_by_contactId` · `Version: v3` · Scopes: `contacts.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Unique identifier of the contact |

*Response*: [`ContactsByIdSuccessfulResponseDtoV3`](#contactsbyidsuccessfulresponsedtov3)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().get_contact(&contactId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.get_contacts_by_contactId",
    "path_params": {
      "contactId": "<contactId>"
    }
  }
}
```

</details>

#### `PUT /contacts/{contactId}`

**Update Contact**

Update a contact using contactId

Operation id: `v3:contacts.put_contacts_by_contactId` · `Version: v3` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Unique identifier of the contact |

*Request body*: [`UpdateContactDtoV3`](#updatecontactdtov3)

*Response*: [`UpdateContactsSuccessfulResponseDtoV3`](#updatecontactssuccessfulresponsedtov3)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().update_contact(&contactId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.put_contacts_by_contactId",
    "path_params": {
      "contactId": "<contactId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /contacts/{contactId}/appointments`

**Get Appointments for Contact**

Operation id: `v3:contacts.get_contacts_by_contactId_appointments` · `Version: v3` · Scopes: `contacts.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Response*: [`GetEventsSuccessfulResponseDto`](#geteventssuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().get_appointments_for_contact(&contactId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.get_contacts_by_contactId_appointments",
    "path_params": {
      "contactId": "<contactId>"
    }
  }
}
```

</details>

#### `DELETE /contacts/{contactId}/campaigns/remove-all`

**Remove Contact From Every Campaign**

Removes the contact from every campaign it is enrolled in.

Operation id: `v3:contacts.delete_contacts_by_contactId_campaigns_remove_all` · `Version: v3` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Response*: [`CreateDeleteCantactsCampaignsSuccessfulResponseDto`](#createdeletecantactscampaignssuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().remove_contact_from_every_campaign(&contactId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.delete_contacts_by_contactId_campaigns_remove_all",
    "path_params": {
      "contactId": "<contactId>"
    }
  }
}
```

</details>

#### `DELETE /contacts/{contactId}/campaigns/{campaignId}`

**Remove Contact From Campaign**

Operation id: `v3:contacts.delete_contacts_by_contactId_campaigns_by_campaignId` · `Version: v3` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `campaignId` | string | **yes** | Campaign Id |

*Response*: [`CreateDeleteCantactsCampaignsSuccessfulResponseDto`](#createdeletecantactscampaignssuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().remove_contact_from_campaign(&contactId, &campaignId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.delete_contacts_by_contactId_campaigns_by_campaignId",
    "path_params": {
      "contactId": "<contactId>",
      "campaignId": "<campaignId>"
    }
  }
}
```

</details>

#### `POST /contacts/{contactId}/campaigns/{campaignId}`

**Add Contact to Campaign**

Add contact to Campaign

Operation id: `v3:contacts.post_contacts_by_contactId_campaigns_by_campaignId` · `Version: v3` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `campaignId` | string | **yes** | Campaign Id |

*Request body*: [`AddContactToCampaignDto`](#addcontacttocampaigndto)

*Response*: [`CreateDeleteCantactsCampaignsSuccessfulResponseDto`](#createdeletecantactscampaignssuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().add_contact_to_campaign(&contactId, &campaignId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.post_contacts_by_contactId_campaigns_by_campaignId",
    "path_params": {
      "contactId": "<contactId>",
      "campaignId": "<campaignId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /contacts/{contactId}/followers`

**Remove Followers**

Operation id: `v3:contacts.delete_contacts_by_contactId_followers` · `Version: v3` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Request body*: [`FollowersDTO`](#followersdto)

*Response*: [`DeleteFollowersSuccessfulResponseDto`](#deletefollowerssuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().remove_followers(&contactId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.delete_contacts_by_contactId_followers",
    "path_params": {
      "contactId": "<contactId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /contacts/{contactId}/followers`

**Add Followers**

Operation id: `v3:contacts.post_contacts_by_contactId_followers` · `Version: v3` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Request body*: [`FollowersDTO`](#followersdto)

*Response*: [`CreateAddFollowersSuccessfulResponseDto`](#createaddfollowerssuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().add_followers(&contactId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.post_contacts_by_contactId_followers",
    "path_params": {
      "contactId": "<contactId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /contacts/{contactId}/notes`

**Get All Notes**

Operation id: `v3:contacts.get_contacts_by_contactId_notes` · `Version: v3` · Scopes: `contacts.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Response*: [`GetNotesListSuccessfulResponseDto`](#getnoteslistsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().get_all_notes(&contactId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.get_contacts_by_contactId_notes",
    "path_params": {
      "contactId": "<contactId>"
    }
  }
}
```

</details>

#### `POST /contacts/{contactId}/notes`

**Create Note**

Operation id: `v3:contacts.post_contacts_by_contactId_notes` · `Version: v3` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Request body*: [`NotesDTO`](#notesdto)

*Response*: [`GetCreateUpdateNoteSuccessfulResponseDto`](#getcreateupdatenotesuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().create_note(&contactId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.post_contacts_by_contactId_notes",
    "path_params": {
      "contactId": "<contactId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /contacts/{contactId}/notes/{id}`

**Delete Note**

Operation id: `v3:contacts.delete_contacts_by_contactId_notes_by_id` · `Version: v3` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `id` | string | **yes** | Note Id |

*Response*: [`DeleteNoteSuccessfulResponseDto`](#deletenotesuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().delete_note(&contactId, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.delete_contacts_by_contactId_notes_by_id",
    "path_params": {
      "contactId": "<contactId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /contacts/{contactId}/notes/{id}`

**Get Note**

Operation id: `v3:contacts.get_contacts_by_contactId_notes_by_id` · `Version: v3` · Scopes: `contacts.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `id` | string | **yes** | Note Id |

*Response*: [`GetCreateUpdateNoteSuccessfulResponseDto`](#getcreateupdatenotesuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().get_note(&contactId, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.get_contacts_by_contactId_notes_by_id",
    "path_params": {
      "contactId": "<contactId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `PUT /contacts/{contactId}/notes/{id}`

**Update Note**

Operation id: `v3:contacts.put_contacts_by_contactId_notes_by_id` · `Version: v3` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `id` | string | **yes** | Note Id |

*Request body*: [`UpdateNoteDTO`](#updatenotedto)

*Response*: [`GetCreateUpdateNoteSuccessfulResponseDto`](#getcreateupdatenotesuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().update_note(&contactId, &id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.put_contacts_by_contactId_notes_by_id",
    "path_params": {
      "contactId": "<contactId>",
      "id": "<id>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /contacts/{contactId}/tags`

**Remove Tags**

Operation id: `v3:contacts.delete_contacts_by_contactId_tags` · `Version: v3` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Request body*: [`TagsDTO`](#tagsdto)

*Response*: [`CreateDeleteTagSuccessfulResponseDto`](#createdeletetagsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().remove_tags(&contactId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.delete_contacts_by_contactId_tags",
    "path_params": {
      "contactId": "<contactId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /contacts/{contactId}/tags`

**Add Tags**

Operation id: `v3:contacts.post_contacts_by_contactId_tags` · `Version: v3` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Request body*: [`TagsDTO`](#tagsdto)

*Response*: [`CreateAddTagSuccessfulResponseDto`](#createaddtagsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().add_tags(&contactId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.post_contacts_by_contactId_tags",
    "path_params": {
      "contactId": "<contactId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /contacts/{contactId}/tasks`

**Get all Tasks**

Operation id: `v3:contacts.get_contacts_by_contactId_tasks` · `Version: v3` · Scopes: `contacts.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Response*: [`TasksListSuccessfulResponseDto`](#taskslistsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().get_all_tasks(&contactId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.get_contacts_by_contactId_tasks",
    "path_params": {
      "contactId": "<contactId>"
    }
  }
}
```

</details>

#### `POST /contacts/{contactId}/tasks`

**Create Task**

Operation id: `v3:contacts.post_contacts_by_contactId_tasks` · `Version: v3` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |

*Request body*: [`CreateTaskParams`](#createtaskparams)

*Response*: [`TaskByIsSuccessfulResponseDto`](#taskbyissuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().create_task(&contactId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.post_contacts_by_contactId_tasks",
    "path_params": {
      "contactId": "<contactId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /contacts/{contactId}/tasks/{taskId}`

**Delete Task**

Operation id: `v3:contacts.delete_contacts_by_contactId_tasks_by_taskId` · `Version: v3` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `taskId` | string | **yes** | Task Id |

*Response*: [`DeleteTaskSuccessfulResponseDto`](#deletetasksuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().delete_task(&contactId, &taskId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.delete_contacts_by_contactId_tasks_by_taskId",
    "path_params": {
      "contactId": "<contactId>",
      "taskId": "<taskId>"
    }
  }
}
```

</details>

#### `GET /contacts/{contactId}/tasks/{taskId}`

**Get Task**

Operation id: `v3:contacts.get_contacts_by_contactId_tasks_by_taskId` · `Version: v3` · Scopes: `contacts.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `taskId` | string | **yes** | Task Id |

*Response*: [`TaskByIsSuccessfulResponseDto`](#taskbyissuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().get_task(&contactId, &taskId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.get_contacts_by_contactId_tasks_by_taskId",
    "path_params": {
      "contactId": "<contactId>",
      "taskId": "<taskId>"
    }
  }
}
```

</details>

#### `PUT /contacts/{contactId}/tasks/{taskId}`

**Update Task**

Operation id: `v3:contacts.put_contacts_by_contactId_tasks_by_taskId` · `Version: v3` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `taskId` | string | **yes** | Task Id |

*Request body*: [`UpdateTaskBody`](#updatetaskbody)

*Response*: [`TaskByIsSuccessfulResponseDto`](#taskbyissuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().update_task(&contactId, &taskId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.put_contacts_by_contactId_tasks_by_taskId",
    "path_params": {
      "contactId": "<contactId>",
      "taskId": "<taskId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /contacts/{contactId}/tasks/{taskId}/completed`

**Update Task Completed**

Operation id: `v3:contacts.put_contacts_by_contactId_tasks_by_taskId_completed` · `Version: v3` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `taskId` | string | **yes** | Task Id |

*Request body*: [`UpdateTaskStatusParams`](#updatetaskstatusparams)

*Response*: [`TaskByIsSuccessfulResponseDto`](#taskbyissuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().update_task_completed(&contactId, &taskId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.put_contacts_by_contactId_tasks_by_taskId_completed",
    "path_params": {
      "contactId": "<contactId>",
      "taskId": "<taskId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /contacts/{contactId}/workflow/{workflowId}`

**Delete Contact from Workflow**

Operation id: `v3:contacts.delete_contacts_by_contactId_workflow_by_workflowId` · `Version: v3` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `workflowId` | string | **yes** | Workflow Id |

*Request body*: [`CreateWorkflowDto`](#createworkflowdto)

*Response*: [`ContactsWorkflowSuccessfulResponseDto`](#contactsworkflowsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().delete_contact_from_workflow(&contactId, &workflowId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.delete_contacts_by_contactId_workflow_by_workflowId",
    "path_params": {
      "contactId": "<contactId>",
      "workflowId": "<workflowId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /contacts/{contactId}/workflow/{workflowId}`

**Add Contact to Workflow**

Operation id: `v3:contacts.post_contacts_by_contactId_workflow_by_workflowId` · `Version: v3` · Scopes: `contacts.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact Id |
| `workflowId` | string | **yes** | Workflow Id |

*Request body*: [`CreateWorkflowDto`](#createworkflowdto)

*Response*: [`ContactsWorkflowSuccessfulResponseDto`](#contactsworkflowsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().contacts().add_contact_to_workflow(&contactId, &workflowId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:contacts.post_contacts_by_contactId_workflow_by_workflowId",
    "path_params": {
      "contactId": "<contactId>",
      "workflowId": "<workflowId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::contacts::*` (enable the `contacts` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/contacts/).

### `AddContactToCampaignDto`

_No fields defined in the spec._

### `AttributionSource`

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | String | **yes** | — |
| `campaign` | String | no | — |
| `utmSource` | String | no | — |
| `utmMedium` | String | no | — |
| `utmContent` | String | no | — |
| `referrer` | String | no | — |
| `campaignId` | String | no | — |
| `fbclid` | String | no | — |
| `gclid` | String | no | — |
| `msclikid` | String | no | — |
| `dclid` | String | no | — |
| `fbc` | String | no | — |
| `fbp` | String | no | — |
| `fbEventId` | String | no | — |
| `userAgent` | String | no | — |
| `ip` | String | no | — |
| `medium` | String | no | — |
| `mediumId` | String | no | — |

### `CheckboxField`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | — |
| `key` | String | no | — |
| `field_value` | Vec<String> | no | — |

### `ContactsBulkUpateResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |
| `ids` | Vec<String> | **yes** | — |

### `ContactsBusinessUpdate`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | — |
| `ids` | Vec<String> | **yes** | — |
| `businessId` | String | **yes** | — |

### `ContactsByIdSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `contact` | [`GetContectByIdSchema`](#getcontectbyidschema) | no | — |

### `ContactsMetaSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `total` | f64 | no | — |
| `nextPageUrl` | String | no | — |
| `startAfterId` | String | no | — |
| `startAfter` | f64 | no | — |
| `currentPage` | f64 | no | — |
| `nextPage` | f64 | no | — |
| `prevPage` | f64 | no | — |

### `ContactsSearchSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `locationId` | String | no | — |
| `email` | String | no | — |
| `timezone` | String | no | — |
| `country` | String | no | — |
| `source` | String | no | — |
| `dateAdded` | String | no | — |
| `customFields` | Vec<CustomFieldSchema> | no | — |
| `tags` | Vec<String> | no | — |
| `businessId` | String | no | — |
| `attributions` | Vec<AttributionSource> | no | — |
| `followers` | Vec<String> | no | — |

### `ContactsSearchSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `contacts` | Vec<ContactsSearchSchema> | no | — |
| `count` | f64 | no | — |

### `ContactsWorkflowSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeded` | bool | no | — |

### `CreateAddFollowersSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `followers` | Vec<String> | no | — |
| `followersAdded` | Vec<String> | no | — |

### `CreateAddTagSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `tags` | Vec<String> | no | — |

### `CreateContactDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `firstName` | String | no | — |
| `lastName` | String | no | — |
| `name` | String | no | — |
| `email` | String | no | — |
| `locationId` | String | **yes** | — |
| `gender` | String | no | — |
| `phone` | String | no | — |
| `address1` | String | no | — |
| `city` | String | no | — |
| `state` | String | no | — |
| `postalCode` | String | no | — |
| `website` | String | no | — |
| `timezone` | String | no | — |
| `dnd` | bool | no | — |
| `dndSettings` | [`DndSettingsSchema`](#dndsettingsschema) | no | — |
| `inboundDndSettings` | [`InboundDndSettingsSchema`](#inbounddndsettingsschema) | no | — |
| `tags` | Vec<String> | no | — |
| `customFields` | Vec<JSON> | no | — |
| `source` | String | no | — |
| `dateOfBirth` | JSON | no | The birth date of the contact. Supported formats: YYYY/MM/DD, MM/DD/YYYY, YYYY-MM-DD, MM-DD-YYYY, YYYY.MM.DD, MM.DD.YYYY, YYYY_MM_DD, MM_DD_YYYY |
| `country` | String | no | — |
| `companyName` | String | no | — |
| `assignedTo` | String | no | User's Id |

### `CreateContactSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `dateAdded` | String | no | — |
| `dateUpdated` | String | no | — |
| `deleted` | bool | no | — |
| `tags` | Vec<String> | no | — |
| `type` | String | no | — |
| `customFields` | Vec<CustomFieldSchema> | no | — |
| `locationId` | String | no | — |
| `firstName` | String | no | — |
| `firstNameLowerCase` | String | no | — |
| `fullNameLowerCase` | String | no | — |
| `lastName` | String | no | — |
| `lastNameLowerCase` | String | no | — |
| `email` | String | no | — |
| `emailLowerCase` | String | no | — |
| `bounceEmail` | bool | no | — |
| `unsubscribeEmail` | bool | no | — |
| `dnd` | bool | no | — |
| `dndSettings` | [`DndSettingsSchema`](#dndsettingsschema) | no | — |
| `phone` | String | no | — |
| `address1` | String | no | — |
| `city` | String | no | — |
| `state` | String | no | — |
| `country` | String | no | — |
| `postalCode` | String | no | — |
| `website` | String | no | — |
| `source` | String | no | — |
| `companyName` | String | no | — |
| `dateOfBirth` | String | no | — |
| `birthMonth` | f64 | no | — |
| `birthDay` | f64 | no | — |
| `lastSessionActivityAt` | String | no | — |
| `offers` | Vec<String> | no | — |
| `products` | Vec<String> | no | — |
| `businessId` | String | no | — |
| `assignedTo` | String | no | User's Id |

### `CreateContactsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `contact` | [`CreateContactSchema`](#createcontactschema) | no | — |

### `CreateDeleteCantactsCampaignsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeded` | bool | no | — |

### `CreateDeleteTagSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `tags` | Vec<String> | no | — |

### `CreateTaskParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | — |
| `body` | String | no | — |
| `dueDate` | String | **yes** | — |
| `completed` | bool | **yes** | — |
| `assignedTo` | String | no | — |

### `CreateWorkflowDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `eventStartTime` | String | no | — |

### `CustomFieldSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `value` | String | no | — |

### `DeleteContactsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeded` | bool | no | — |

### `DeleteFollowersSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `followers` | Vec<String> | no | — |
| `followersRemoved` | Vec<String> | no | — |

### `DeleteNoteSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeded` | bool | no | — |

### `DeleteTaskSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeded` | bool | no | — |

### `DndSettingSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | String — `active`, `inactive`, `permanent` | **yes** | — |
| `message` | String | no | — |
| `code` | String | no | — |

### `DndSettingsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `Call` | [`DndSettingSchema`](#dndsettingschema) | no | — |
| `Email` | [`DndSettingSchema`](#dndsettingschema) | no | — |
| `SMS` | [`DndSettingSchema`](#dndsettingschema) | no | — |
| `WhatsApp` | [`DndSettingSchema`](#dndsettingschema) | no | — |
| `GMB` | [`DndSettingSchema`](#dndsettingschema) | no | — |
| `FB` | [`DndSettingSchema`](#dndsettingschema) | no | — |

### `FileField`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | — |
| `key` | String | no | — |
| `field_value` | JSON | no | — |

### `FollowersDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `followers` | Vec<String> | **yes** | — |

### `GetContectByIdSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `locationId` | String | no | — |
| `firstName` | String | no | — |
| `lastName` | String | no | — |
| `email` | String | no | — |
| `emailLowerCase` | String | no | — |
| `timezone` | String | no | — |
| `companyName` | String | no | — |
| `phone` | String | no | — |
| `dnd` | bool | no | — |
| `dndSettings` | [`DndSettingsSchema`](#dndsettingsschema) | no | — |
| `type` | String | no | — |
| `source` | String | no | — |
| `assignedTo` | String | no | — |
| `address1` | String | no | — |
| `city` | String | no | — |
| `state` | String | no | — |
| `country` | String | no | — |
| `postalCode` | String | no | — |
| `website` | String | no | — |
| `tags` | Vec<String> | no | — |
| `dateOfBirth` | String | no | — |
| `dateAdded` | String | no | — |
| `dateUpdated` | String | no | — |
| `attachments` | String | no | — |
| `ssn` | String | no | — |
| `keyword` | String | no | — |
| `firstNameLowerCase` | String | no | — |
| `fullNameLowerCase` | String | no | — |
| `lastNameLowerCase` | String | no | — |
| `lastActivity` | String | no | — |
| `customFields` | Vec<CustomFieldSchema> | no | — |
| `businessId` | String | no | — |
| `attributionSource` | [`AttributionSource`](#attributionsource) | no | — |
| `lastAttributionSource` | [`AttributionSource`](#attributionsource) | no | — |
| `visitorId` | String | no | visitorId is the Unique ID assigned to each Live chat visitor. |

### `GetCreateUpdateNoteSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `note` | [`GetNoteSchema`](#getnoteschema) | no | — |

### `GetEventSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `calendarId` | String | no | — |
| `status` | String | no | — |
| `title` | String | no | — |
| `assignedUserId` | String | no | — |
| `notes` | String | no | — |
| `startTime` | String | no | — |
| `endTime` | String | no | — |
| `address` | String | no | — |
| `locationId` | String | no | — |
| `contactId` | String | no | — |
| `groupId` | String | no | — |
| `appointmentStatus` | String | no | — |
| `users` | Vec<String> | no | — |
| `dateAdded` | String | no | — |
| `dateUpdated` | String | no | — |
| `assignedResources` | Vec<String> | no | — |

### `GetEventsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `events` | Vec<GetEventSchema> | no | — |

### `GetNoteSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `body` | String | no | — |
| `userId` | String | no | — |
| `dateAdded` | String | no | — |
| `contactId` | String | no | — |
| `title` | String | no | — |
| `color` | String | no | — |
| `pinned` | bool | no | — |

### `GetNotesListSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `notes` | Vec<GetNoteSchema> | no | — |

### `InboundDndSettingSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | String — `active`, `inactive` | **yes** | — |
| `message` | String | no | — |

### `InboundDndSettingsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `all` | [`InboundDndSettingSchema`](#inbounddndsettingschema) | no | — |

### `LargeTextField`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | — |
| `key` | String | no | — |
| `field_value` | String | no | — |

### `MonetoryField`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | — |
| `key` | String | no | — |
| `field_value` | JSON | no | — |

### `MultiSelectField`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | — |
| `key` | String | no | — |
| `field_value` | Vec<String> | no | — |

### `NotesDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `userId` | String | no | — |
| `body` | String | **yes** | — |
| `title` | String | no | — |
| `color` | String | no | — |
| `pinned` | bool | no | — |

### `NumericField`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | — |
| `key` | String | no | — |
| `field_value` | JSON | no | — |

### `RadioField`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | — |
| `key` | String | no | — |
| `field_value` | String | no | — |

### `SearchBodyV2DTO`

_No fields defined in the spec._

### `SingleSelectField`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | — |
| `key` | String | no | — |
| `field_value` | String | no | — |

### `TagsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `tags` | Vec<String> | **yes** | — |

### `TaskByIsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `task` | [`TaskSchema`](#taskschema) | no | — |

### `TaskSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `title` | String | no | — |
| `body` | String | no | — |
| `assignedTo` | String | no | — |
| `dueDate` | String | no | — |
| `completed` | bool | no | — |
| `contactId` | String | no | — |

### `TasksListSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `tasks` | Vec<TaskSchema> | no | — |

### `TextField`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | — |
| `key` | String | no | — |
| `field_value` | String | no | — |

### `UpdateContactDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `firstName` | String | no | — |
| `lastName` | String | no | — |
| `name` | String | no | — |
| `email` | String | no | — |
| `phone` | String | no | — |
| `address1` | String | no | — |
| `city` | String | no | — |
| `state` | String | no | — |
| `postalCode` | String | no | — |
| `website` | String | no | — |
| `timezone` | String | no | — |
| `dnd` | bool | no | — |
| `dndSettings` | [`DndSettingsSchema`](#dndsettingsschema) | no | — |
| `inboundDndSettings` | [`InboundDndSettingsSchema`](#inbounddndsettingsschema) | no | — |
| `tags` | Vec<String> | no | This field will overwrite all current tags associated with the contact. To update a tags, it is recommended to use the Add Tag or Remove Tag API instead. |
| `customFields` | Vec<JSON> | no | — |
| `source` | String | no | — |
| `dateOfBirth` | JSON | no | The birth date of the contact. Supported formats: YYYY/MM/DD, MM/DD/YYYY, YYYY-MM-DD, MM-DD-YYYY, YYYY.MM.DD, MM.DD.YYYY, YYYY_MM_DD, MM_DD_YYYY |
| `country` | String | no | — |
| `assignedTo` | String | no | User's Id |

### `UpdateContactsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeded` | bool | no | — |
| `contact` | [`GetContectByIdSchema`](#getcontectbyidschema) | no | — |

### `UpdateNoteDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `userId` | String | no | — |
| `body` | String | no | — |
| `title` | String | no | — |
| `color` | String | no | — |
| `pinned` | bool | no | — |

### `UpdateTagsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `contacts` | Vec<String> | **yes** | list of contact ids to be processed |
| `tags` | Vec<String> | **yes** | list of tags to be added or removed |
| `locationId` | String | **yes** | location id from where the bulk request is executed |
| `removeAllTags` | bool | no | Option to implement remove all tags. if true, all tags will be removed from the contacts. Can only be used with remove type. |

### `UpdateTagsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeded` | bool | **yes** | Indicates if the operation was successful |
| `errorCount` | f64 | **yes** | Number of errors encountered during the operation |
| `responses` | Vec<String> | **yes** | Responses for each contact processed |

### `UpdateTaskBody`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | no | — |
| `body` | String | no | — |
| `dueDate` | String | no | — |
| `completed` | bool | no | — |
| `assignedTo` | String | no | — |

### `UpdateTaskStatusParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `completed` | bool | **yes** | — |

### `UpsertContactDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `firstName` | String | no | — |
| `lastName` | String | no | — |
| `name` | String | no | — |
| `email` | String | no | — |
| `locationId` | String | **yes** | — |
| `gender` | String | no | — |
| `phone` | String | no | — |
| `address1` | String | no | — |
| `city` | String | no | — |
| `state` | String | no | — |
| `postalCode` | String | no | — |
| `website` | String | no | — |
| `timezone` | String | no | — |
| `dnd` | bool | no | — |
| `dndSettings` | [`DndSettingsSchema`](#dndsettingsschema) | no | — |
| `inboundDndSettings` | [`InboundDndSettingsSchema`](#inbounddndsettingsschema) | no | — |
| `tags` | Vec<String> | no | This field will overwrite all current tags associated with the contact. To update a tags, it is recommended to use the Add Tag or Remove Tag API instead. |
| `customFields` | Vec<JSON> | no | — |
| `source` | String | no | — |
| `dateOfBirth` | JSON | no | The birth date of the contact. Supported formats: YYYY/MM/DD, MM/DD/YYYY, YYYY-MM-DD, MM-DD-YYYY, YYYY.MM.DD, MM.DD.YYYY, YYYY_MM_DD, MM_DD_YYYY |
| `country` | String | no | — |
| `companyName` | String | no | — |
| `assignedTo` | String | no | User's Id |
| `createNewIfDuplicateAllowed` | bool | no | Controls whether to create a new contact or update an existing duplicate. **Scenario 1:** If this value is `true` and the location allows duplicate contacts, a new contact will be created immediately … |

### `UpsertContactsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `new` | bool | no | — |
| `contact` | [`GetContectByIdSchema`](#getcontectbyidschema) | no | — |
| `traceId` | String | no | — |

### `customFieldsInputArraySchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | — |
| `key` | String | no | — |
| `field_value` | Vec<String> | no | — |

### `customFieldsInputObjectSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | — |
| `key` | String | no | — |
| `field_value` | JSON | no | — |

### `customFieldsInputStringSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Pass either `id` or `key` of custom field |
| `key` | String | no | Pass either `id` or `key` of custom field |
| `field_value` | String | no | — |

## Data models — API v3

In Rust: `ghl_models::v3::contacts::*` (enable the `contacts` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/contacts/).

### `AddContactToCampaignDto`

_No fields defined in the spec._

### `AttributionSource`

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | String | **yes** | Attribution source type |
| `campaign` | String | no | Campaign name |
| `utmSource` | String | no | UTM source parameter |
| `utmMedium` | String | no | UTM medium parameter |
| `utmContent` | String | no | UTM content parameter |
| `referrer` | String | no | Referrer URL |
| `campaignId` | String | no | Campaign Id |
| `fbclid` | String | no | Facebook click Id |
| `gclid` | String | no | Google click Id |
| `msclikid` | String | no | Microsoft click Id |
| `dclid` | String | no | DoubleClick click Id |
| `fbc` | String | no | Facebook browser Id |
| `fbp` | String | no | Facebook pixel Id |
| `fbEventId` | String | no | Facebook event Id |
| `userAgent` | String | no | Browser user agent string |
| `ip` | String | no | IP address of the visitor |
| `medium` | String | no | Attribution medium (e.g. survey, funnel) |
| `mediumId` | String | no | Id of the attribution medium |

### `CheckboxField`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Pass either `id` or `key` of custom field |
| `key` | String | no | Pass either `id` or `key` of custom field |
| `fieldValue` | Vec<String> | no | Array of selected checkbox values for the custom field (preferred). |
| `field_value` | Vec<String> | no | Deprecated. Use `fieldValue` instead. |

### `ContactsBulkUpateResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Whether the bulk update was successful |
| `ids` | Vec<String> | **yes** | List of contact Ids that were updated |

### `ContactsBusinessUpdate`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location Id |
| `ids` | Vec<String> | **yes** | List of contact Ids to update (maximum 50) |
| `businessId` | String | **yes** | Business Id to assign to contacts. Pass null to remove business association. |

### `ContactsByIdSuccessfulResponseDtoV3`

| Field | Type | Required | Description |
|---|---|---|---|
| `contact` | [`GetContactByIdSchemaV3`](#getcontactbyidschemav3) | no | Contact details |

### `ContactsMetaSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `total` | f64 | no | — |
| `nextPageUrl` | String | no | — |
| `startAfterId` | String | no | — |
| `startAfter` | f64 | no | — |
| `currentPage` | f64 | no | — |
| `nextPage` | f64 | no | — |
| `prevPage` | f64 | no | — |

### `ContactsSearchSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Unique identifier of the contact |
| `locationId` | String | no | Location Id the contact belongs to |
| `email` | String | no | Email address of the contact |
| `timezone` | String | no | Timezone of the contact |
| `country` | String | no | Country of the contact |
| `source` | String | no | Source from which the contact was created |
| `dateAdded` | String | no | Date and time the contact was added (ISO 8601) |
| `customFields` | Vec<CustomFieldSchema> | no | List of custom field values for the contact |
| `tags` | Vec<String> | no | List of tags associated with the contact |
| `businessId` | String | no | Business Id the contact is associated with |
| `attributions` | Vec<AttributionSource> | no | List of attribution sources for the contact |
| `followers` | Vec<String> | no | List of user Ids following this contact |

### `ContactsSearchSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `contacts` | Vec<ContactsSearchSchema> | no | List of contacts associated with the business |
| `count` | f64 | no | Total number of contacts matching the query |

### `ContactsWorkflowSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeeded` | bool | no | Whether the workflow operation was successful |
| `succeded` | bool | no | Legacy misspelling of `succeeded`. Deprecated; use `succeeded`. |

### `CreateAddFollowersSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `followers` | Vec<String> | no | Current followers after the operation |
| `followersAdded` | Vec<String> | no | Followers that were added |

### `CreateAddTagSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `tags` | Vec<String> | no | Current tags on the contact after the operation |

### `CreateContactDtoV3`

| Field | Type | Required | Description |
|---|---|---|---|
| `firstName` | String | no | First name of the contact |
| `lastName` | String | no | Last name of the contact |
| `name` | String | no | Full name of the contact |
| `email` | String | no | Email address of the contact |
| `locationId` | String | **yes** | Location Id the contact should be created under |
| `gender` | String | no | Gender of the contact |
| `phone` | String | no | Phone number of the contact |
| `address1` | String | no | Street address of the contact |
| `city` | String | no | City of the contact |
| `state` | String | no | State of the contact |
| `postalCode` | String | no | Postal code of the contact |
| `website` | String | no | Website URL of the contact |
| `timezone` | String | no | Timezone of the contact |
| `dnd` | bool | no | Whether Do Not Disturb is enabled for the contact |
| `inboundDndSettings` | [`InboundDndSettingsSchema`](#inbounddndsettingsschema) | no | Inbound DND settings per channel for the contact |
| `tags` | Vec<String> | no | List of tags to assign to the contact |
| `customFields` | Vec<JSON> | no | List of custom field values to assign to the contact |
| `source` | String | no | Source from which the contact was created |
| `dateOfBirth` | JSON | no | The birth date of the contact. Supported formats: YYYY/MM/DD, MM/DD/YYYY, YYYY-MM-DD, MM-DD-YYYY, YYYY.MM.DD, MM.DD.YYYY, YYYY_MM_DD, MM_DD_YYYY |
| `country` | String | no | Country code of the contact (ISO 3166-1 alpha-2) |
| `companyName` | String | no | Company name of the contact |
| `assignedTo` | String | no | User's Id |
| `dndSettings` | [`DndSettingsSchemaV3`](#dndsettingsschemav3) | no | Per-channel DND settings for the contact |

### `CreateContactsSuccessfulResponseDtoV3`

| Field | Type | Required | Description |
|---|---|---|---|
| `contact` | [`GetContactByIdSchemaV3`](#getcontactbyidschemav3) | no | Contact details |

### `CreateDeleteCantactsCampaignsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeeded` | bool | no | Whether the campaign operation was successful |
| `succeded` | bool | no | Legacy misspelling of `succeeded`. Deprecated; use `succeeded`. |

### `CreateDeleteTagSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `tags` | Vec<String> | no | Current tags on the contact after the operation |

### `CreateTaskParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | Title of the task |
| `body` | String | no | Body or description of the task |
| `dueDate` | String | **yes** | Due date of the task (ISO 8601 format) |
| `completed` | bool | **yes** | Whether the task is completed |
| `assignedTo` | String | no | User Id to whom the task is assigned |

### `CreateWorkflowDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `eventStartTime` | String | no | Start time of the workflow event (ISO 8601 format) |

### `CustomFieldSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Unique identifier of the custom field |
| `value` | String | no | Value of the custom field |

### `DeleteContactsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeeded` | bool | no | Whether the delete operation succeeded |
| `succeded` | bool | no | Legacy misspelling of `succeeded`. Deprecated; use `succeeded`. |

### `DeleteFollowersSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `followers` | Vec<String> | no | Current followers after the operation |
| `followersRemoved` | Vec<String> | no | Followers that were removed |

### `DeleteNoteSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeeded` | bool | no | Whether the note was successfully deleted |
| `succeded` | bool | no | Legacy misspelling of `succeeded`. Deprecated; use `succeeded`. |

### `DeleteTaskSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeeded` | bool | no | Whether the task was successfully deleted |
| `succeded` | bool | no | Legacy misspelling of `succeeded`. Deprecated; use `succeeded`. |

### `DndSettingSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | String — `active`, `inactive`, `permanent` | **yes** | Do Not Disturb status for this channel |
| `message` | String | no | Custom message associated with the DND setting |
| `code` | String | no | DND code or reason |

### `DndSettingsSchemaV3`

| Field | Type | Required | Description |
|---|---|---|---|
| `call` | [`DndSettingSchema`](#dndsettingschema) | no | DND settings for phone calls |
| `email` | [`DndSettingSchema`](#dndsettingschema) | no | DND settings for email |
| `sms` | [`DndSettingSchema`](#dndsettingschema) | no | DND settings for SMS |
| `whatsApp` | [`DndSettingSchema`](#dndsettingschema) | no | DND settings for WhatsApp |
| `gmb` | [`DndSettingSchema`](#dndsettingschema) | no | DND settings for Google My Business |
| `fb` | [`DndSettingSchema`](#dndsettingschema) | no | DND settings for Facebook |

### `FileField`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Pass either `id` or `key` of custom field |
| `key` | String | no | Pass either `id` or `key` of custom field |
| `fieldValue` | JSON | no | File upload value — a map of UUID to file metadata and download URL (preferred). |
| `field_value` | JSON | no | Deprecated. Use `fieldValue` instead. |

### `FollowersDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `followers` | Vec<String> | **yes** | List of user Ids to follow or unfollow the contact |

### `GetContactByIdSchemaV3`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Unique identifier of the contact |
| `name` | String | no | Full name of the contact |
| `locationId` | String | no | Location Id the contact belongs to |
| `firstName` | String | no | First name of the contact |
| `lastName` | String | no | Last name of the contact |
| `email` | String | no | Email address of the contact |
| `emailLowerCase` | String | no | Lowercase version of the contact email |
| `timezone` | String | no | Timezone of the contact |
| `companyName` | String | no | Company name of the contact |
| `phone` | String | no | Phone number of the contact |
| `dnd` | bool | no | Whether Do Not Disturb is enabled for the contact |
| `type` | String | no | Contact type classification |
| `source` | String | no | Source from which the contact was created |
| `assignedTo` | String | no | User Id the contact is assigned to |
| `address1` | String | no | Street address of the contact |
| `city` | String | no | City of the contact |
| `state` | String | no | State of the contact |
| `country` | String | no | Country of the contact |
| `postalCode` | String | no | Postal code of the contact |
| `website` | String | no | Website URL of the contact |
| `tags` | Vec<String> | no | List of tags associated with the contact |
| `dateOfBirth` | String | no | Date of birth of the contact (YYYY-MM-DD) |
| `dateAdded` | String | no | Date and time the contact was added (ISO 8601) |
| `dateUpdated` | String | no | Date and time the contact was last updated (ISO 8601) |
| `attachments` | String | no | List of attachment URLs associated with the contact |
| `ssn` | String | no | Social Security Number (if applicable) |
| `keyword` | String | no | Search keyword associated with the contact |
| `firstNameLowerCase` | String | no | Lowercase version of the contact first name |
| `fullNameLowerCase` | String | no | Lowercase version of the contact full name |
| `lastNameLowerCase` | String | no | Lowercase version of the contact last name |
| `lastActivity` | String | no | Date and time of last activity on this contact (ISO 8601) |
| `customFields` | Vec<CustomFieldSchema> | no | List of custom field values for the contact |
| `businessId` | String | no | Business Id the contact is associated with |
| `attributionSource` | [`AttributionSource`](#attributionsource) | no | First-touch attribution source details for the contact |
| `lastAttributionSource` | [`AttributionSource`](#attributionsource) | no | Last-touch attribution source details for the contact |
| `visitorId` | String | no | visitorId is the Unique ID assigned to each Live chat visitor. |
| `dndSettings` | [`DndSettingsSchemaV3`](#dndsettingsschemav3) | no | Per-channel DND settings for the contact |

### `GetCreateUpdateNoteSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `note` | [`GetNoteSchema`](#getnoteschema) | no | Note details |

### `GetEventSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Unique identifier of the appointment |
| `calendarId` | String | no | Calendar Id associated with the appointment |
| `status` | String | no | Status of the appointment |
| `title` | String | no | Title of the appointment |
| `assignedUserId` | String | no | User Id assigned to the appointment |
| `notes` | String | no | Notes for the appointment |
| `startTime` | String | no | Start time of the appointment |
| `endTime` | String | no | End time of the appointment |
| `address` | String | no | Address for the appointment |
| `locationId` | String | no | Location Id of the appointment |
| `contactId` | String | no | Contact Id associated with the appointment |
| `groupId` | String | no | Group Id of the appointment |
| `appointmentStatus` | String | no | Appointment status |
| `users` | Vec<String> | no | List of user Ids assigned to the appointment |
| `dateAdded` | String | no | Date the appointment was created |
| `dateUpdated` | String | no | Date the appointment was last updated |
| `assignedResources` | Vec<String> | no | List of resource Ids assigned to the appointment |

### `GetEventsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `events` | Vec<GetEventSchema> | no | List of appointments |

### `GetNoteSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Unique identifier of the note |
| `body` | String | no | Body content of the note |
| `userId` | String | no | User Id of the note author |
| `dateAdded` | String | no | Date the note was added (ISO 8601 format) |
| `contactId` | String | no | Contact Id associated with the note |
| `title` | String | no | Title of the note |
| `color` | String | no | Hex color code for the note |
| `pinned` | bool | no | Whether the note is pinned |

### `GetNotesListSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `notes` | Vec<GetNoteSchema> | no | List of notes |

### `InboundDndSettingSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | String — `active`, `inactive` | **yes** | Inbound DND status for this channel |
| `message` | String | no | Custom message associated with the inbound DND setting |

### `InboundDndSettingsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `all` | [`InboundDndSettingSchema`](#inbounddndsettingschema) | no | Inbound DND settings applied to all channels |

### `LargeTextField`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Pass either `id` or `key` of custom field |
| `key` | String | no | Pass either `id` or `key` of custom field |
| `fieldValue` | String | no | Large text value for the custom field (preferred). |
| `field_value` | String | no | Deprecated. Use `fieldValue` instead. |

### `MonetoryField`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Pass either `id` or `key` of custom field |
| `key` | String | no | Pass either `id` or `key` of custom field |
| `fieldValue` | JSON | no | Monetary value for the custom field (preferred). |
| `field_value` | JSON | no | Deprecated. Use `fieldValue` instead. |

### `MultiSelectField`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Pass either `id` or `key` of custom field |
| `key` | String | no | Pass either `id` or `key` of custom field |
| `fieldValue` | Vec<String> | no | Array of selected values for the custom field (preferred). |
| `field_value` | Vec<String> | no | Deprecated. Use `fieldValue` instead. |

### `NotesDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `userId` | String | no | User Id of the note author |
| `body` | String | **yes** | Body content of the note |
| `title` | String | no | Title of the note |
| `color` | String | no | Hex color code for the note |
| `pinned` | bool | no | Whether the note is pinned |

### `NumericField`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Pass either `id` or `key` of custom field |
| `key` | String | no | Pass either `id` or `key` of custom field |
| `fieldValue` | JSON | no | Numeric value for the custom field (preferred). |
| `field_value` | JSON | no | Deprecated. Use `fieldValue` instead. |

### `RadioField`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Pass either `id` or `key` of custom field |
| `key` | String | no | Pass either `id` or `key` of custom field |
| `fieldValue` | String | no | Selected radio option value for the custom field (preferred). |
| `field_value` | String | no | Deprecated. Use `fieldValue` instead. |

### `SearchBodyV2DTO`

_No fields defined in the spec._

### `SingleSelectField`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Pass either `id` or `key` of custom field |
| `key` | String | no | Pass either `id` or `key` of custom field |
| `fieldValue` | String | no | Selected option value for the custom field (preferred). |
| `field_value` | String | no | Deprecated. Use `fieldValue` instead. |

### `TagsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `tags` | Vec<String> | **yes** | List of tags to add or remove |

### `TaskByIsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `task` | [`TaskSchema`](#taskschema) | no | Task details |

### `TaskSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Unique identifier of the task |
| `title` | String | no | Title of the task |
| `body` | String | no | Body or description of the task |
| `assignedTo` | String | no | User Id to whom the task is assigned |
| `dueDate` | String | no | Due date of the task (ISO 8601 format) |
| `completed` | bool | no | Whether the task is completed |
| `contactId` | String | no | Contact Id associated with the task |

### `TasksListSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `tasks` | Vec<TaskSchema> | no | List of tasks |

### `TextField`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Pass either `id` or `key` of custom field |
| `key` | String | no | Pass either `id` or `key` of custom field |
| `fieldValue` | String | no | Text value for the custom field (preferred). |
| `field_value` | String | no | Deprecated. Use `fieldValue` instead. |

### `UpdateContactDtoV3`

| Field | Type | Required | Description |
|---|---|---|---|
| `firstName` | String | no | First name of the contact |
| `lastName` | String | no | Last name of the contact |
| `name` | String | no | Full name of the contact |
| `email` | String | no | Email address of the contact |
| `phone` | String | no | Phone number of the contact |
| `address1` | String | no | Street address of the contact |
| `city` | String | no | City of the contact |
| `state` | String | no | State of the contact |
| `postalCode` | String | no | Postal code of the contact |
| `website` | String | no | Website URL of the contact |
| `timezone` | String | no | Timezone of the contact |
| `dnd` | bool | no | Whether Do Not Disturb is enabled for the contact |
| `inboundDndSettings` | [`InboundDndSettingsSchema`](#inbounddndsettingsschema) | no | Inbound DND settings per channel for the contact |
| `tags` | Vec<String> | no | This field will overwrite all current tags associated with the contact. To update a tags, it is recommended to use the Add Tag or Remove Tag API instead. |
| `customFields` | Vec<JSON> | no | List of custom field values to assign to the contact |
| `source` | String | no | Source from which the contact was updated |
| `dateOfBirth` | JSON | no | The birth date of the contact. Supported formats: YYYY/MM/DD, MM/DD/YYYY, YYYY-MM-DD, MM-DD-YYYY, YYYY.MM.DD, MM.DD.YYYY, YYYY_MM_DD, MM_DD_YYYY |
| `country` | String | no | Country code of the contact (ISO 3166-1 alpha-2), Refer country list from documentaion, documentation has list of all countries |
| `assignedTo` | String | no | User's Id |
| `dndSettings` | [`DndSettingsSchemaV3`](#dndsettingsschemav3) | no | Per-channel DND settings for the contact |

### `UpdateContactsSuccessfulResponseDtoV3`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeeded` | bool | no | Whether the update operation succeeded |
| `contact` | [`GetContactByIdSchemaV3`](#getcontactbyidschemav3) | no | Contact details |

### `UpdateNoteDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `userId` | String | no | User Id of the note author |
| `body` | String | no | Body content of the note |
| `title` | String | no | Title of the note |
| `color` | String | no | Hex color code for the note |
| `pinned` | bool | no | Whether the note is pinned |

### `UpdateTagsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `contacts` | Vec<String> | **yes** | list of contact ids to be processed |
| `tags` | Vec<String> | **yes** | list of tags to be added or removed |
| `locationId` | String | **yes** | location id from where the bulk request is executed |
| `removeAllTags` | bool | no | Option to implement remove all tags. if true, all tags will be removed from the contacts. Can only be used with remove type. |

### `UpdateTagsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeeded` | bool | **yes** | Indicates if the operation was successful |
| `succeded` | bool | **yes** | Legacy misspelling of `succeeded`. Deprecated; use `succeeded`. |
| `errorCount` | f64 | **yes** | Number of errors encountered during the operation |
| `responses` | Vec<String> | **yes** | Responses for each contact processed |

### `UpdateTaskBody`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | no | Title of the task |
| `body` | String | no | Body or description of the task |
| `dueDate` | String | no | Due date of the task (ISO 8601 format) |
| `completed` | bool | no | Whether the task is completed |
| `assignedTo` | String | no | User Id to whom the task is assigned |

### `UpdateTaskStatusParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `completed` | bool | **yes** | Whether the task is completed |

### `UpsertContactDtoV3`

| Field | Type | Required | Description |
|---|---|---|---|
| `firstName` | String | no | First name of the contact |
| `lastName` | String | no | Last name of the contact |
| `name` | String | no | Full name of the contact |
| `email` | String | no | Email address of the contact |
| `locationId` | String | **yes** | Location Id the contact should be created under |
| `gender` | String | no | Gender of the contact |
| `phone` | String | no | Phone number of the contact |
| `address1` | String | no | Street address of the contact |
| `city` | String | no | City of the contact |
| `state` | String | no | State of the contact |
| `postalCode` | String | no | Postal code of the contact |
| `website` | String | no | Website URL of the contact |
| `timezone` | String | no | Timezone of the contact |
| `dnd` | bool | no | Whether Do Not Disturb is enabled for the contact |
| `inboundDndSettings` | [`InboundDndSettingsSchema`](#inbounddndsettingsschema) | no | Inbound DND settings per channel for the contact |
| `tags` | Vec<String> | no | This field will overwrite all current tags associated with the contact. To update a tags, it is recommended to use the Add Tag or Remove Tag API instead. |
| `customFields` | Vec<JSON> | no | List of custom field values to assign to the contact |
| `source` | String | no | Source from which the contact was created |
| `dateOfBirth` | JSON | no | The birth date of the contact. Supported formats: YYYY/MM/DD, MM/DD/YYYY, YYYY-MM-DD, MM-DD-YYYY, YYYY.MM.DD, MM.DD.YYYY, YYYY_MM_DD, MM_DD_YYYY |
| `country` | String | no | Country code of the contact (ISO 3166-1 alpha-2) |
| `companyName` | String | no | Company name of the contact |
| `assignedTo` | String | no | User's Id |
| `createNewIfDuplicateAllowed` | bool | no | Controls whether to create a new contact or update an existing duplicate. **Scenario 1:** If this value is `true` and the location allows duplicate contacts, a new contact will be created immediately … |
| `dndSettings` | [`DndSettingsSchemaV3`](#dndsettingsschemav3) | no | Per-channel DND settings for the contact |

### `UpsertContactsSuccessfulResponseDtoV3`

| Field | Type | Required | Description |
|---|---|---|---|
| `new` | bool | no | Whether a new contact was created (true) or an existing one was updated (false) |
| `contact` | [`GetContactByIdSchemaV3`](#getcontactbyidschemav3) | no | Contact details |
| `traceId` | String | no | Unique trace identifier for this operation |

### `customFieldsInputArraySchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Pass either `id` or `key` of custom field |
| `key` | String | no | Pass either `id` or `key` of custom field |
| `fieldValue` | Vec<String> | no | Array value for the custom field (preferred). |
| `field_value` | Vec<String> | no | Deprecated. Use `fieldValue` instead. |

### `customFieldsInputObjectSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Pass either `id` or `key` of custom field |
| `key` | String | no | Pass either `id` or `key` of custom field |
| `fieldValue` | JSON | no | Object value for the custom field (preferred). |
| `field_value` | JSON | no | Deprecated. Use `fieldValue` instead. |

### `customFieldsInputStringSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Pass either `id` or `key` of custom field |
| `key` | String | no | Pass either `id` or `key` of custom field |
| `fieldValue` | String | no | Value for the custom field (preferred). |
| `field_value` | String | no | Deprecated. Use `fieldValue` instead. |

