# `calendars`

**41** operations / **62** models in API v2 · **59** operations / **94** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `calendars` cargo feature on `ghl-sdk`, then call any of the 41 generated methods on `ghl.calendars()`:

```toml
ghl-sdk = { version = "0.4", features = ["calendars"] }
```

This module also has hand-written ergonomic helpers on the same `ghl.calendars()`: `list()`, `free_slots()`, `create_appointment()`, `get_appointment()` (envelope unwrapping, paginated `Stream`s).

MCP tools: `ghl_list_calendars`, `ghl_get_free_slots`, `ghl_book_appointment`.


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/calendars/` | Get Calendars | `get_calendars()` | `calendars.get_calendars` |
| `POST` | `/calendars/` | Create Calendar | `create_calendar()` | `calendars.post_calendars` |
| `GET` | `/calendars/appointments/{appointmentId}/notes` | Get Notes | `get_notes()` | `calendars.get_calendars_appointments_by_appointmentId_notes` |
| `POST` | `/calendars/appointments/{appointmentId}/notes` | Create Note | `create_note()` | `calendars.post_calendars_appointments_by_appointmentId_notes` |
| `DELETE` | `/calendars/appointments/{appointmentId}/notes/{noteId}` | Delete Note | `delete_note()` | `calendars.delete_calendars_appointments_by_appointmentId_notes_by_noteId` |
| `PUT` | `/calendars/appointments/{appointmentId}/notes/{noteId}` | Update Note | `update_note()` | `calendars.put_calendars_appointments_by_appointmentId_notes_by_noteId` |
| `GET` | `/calendars/blocked-slots` | Get Blocked Slots | `get_blocked_slots()` | `calendars.get_calendars_blocked_slots` |
| `GET` | `/calendars/events` | Get Calendar Events | `get_calendar_events()` | `calendars.get_calendars_events` |
| `POST` | `/calendars/events/appointments` | Create appointment | `create_appointment_op()` | `calendars.post_calendars_events_appointments` |
| `GET` | `/calendars/events/appointments/{eventId}` | Get Appointment | `get_appointment_op()` | `calendars.get_calendars_events_appointments_by_eventId` |
| `PUT` | `/calendars/events/appointments/{eventId}` | Update Appointment | `update_appointment()` | `calendars.put_calendars_events_appointments_by_eventId` |
| `POST` | `/calendars/events/block-slots` | Create Block Slot | `create_block_slot()` | `calendars.post_calendars_events_block_slots` |
| `PUT` | `/calendars/events/block-slots/{eventId}` | Update Block Slot | `update_block_slot()` | `calendars.put_calendars_events_block_slots_by_eventId` |
| `DELETE` | `/calendars/events/{eventId}` | Delete Event | `delete_event()` | `calendars.delete_calendars_events_by_eventId` |
| `GET` | `/calendars/groups` | Get Groups | `get_groups()` | `calendars.get_calendars_groups` |
| `POST` | `/calendars/groups` | Create Calendar Group | `create_calendar_group()` | `calendars.post_calendars_groups` |
| `POST` | `/calendars/groups/validate-slug` | Validate group slug | `validate_group_slug()` | `calendars.post_calendars_groups_validate_slug` |
| `DELETE` | `/calendars/groups/{groupId}` | Delete Group | `delete_group()` | `calendars.delete_calendars_groups_by_groupId` |
| `PUT` | `/calendars/groups/{groupId}` | Update Group | `update_group()` | `calendars.put_calendars_groups_by_groupId` |
| `PUT` | `/calendars/groups/{groupId}/status` | Disable Group | `disable_group()` | `calendars.put_calendars_groups_by_groupId_status` |
| `GET` | `/calendars/resources/{resourceType}` | List Calendar Resources | `list_calendar_resources()` | `calendars.get_calendars_resources_by_resourceType` |
| `POST` | `/calendars/resources/{resourceType}` | Create Calendar Resource | `create_calendar_resource()` | `calendars.post_calendars_resources_by_resourceType` |
| `DELETE` | `/calendars/resources/{resourceType}/{id}` | Delete Calendar Resource | `delete_calendar_resource()` | `calendars.delete_calendars_resources_by_resourceType_by_id` |
| `GET` | `/calendars/resources/{resourceType}/{id}` | Get Calendar Resource | `get_calendar_resource()` | `calendars.get_calendars_resources_by_resourceType_by_id` |
| `PUT` | `/calendars/resources/{resourceType}/{id}` | Update Calendar Resource | `update_calendar_resource()` | `calendars.put_calendars_resources_by_resourceType_by_id` |
| `POST` | `/calendars/schedules` | Create user availability schedule | `create_user_availability_schedule()` | `calendars.post_calendars_schedules` |
| `GET` | `/calendars/schedules/search` | List user availability schedule | `list_user_availability_schedule()` | `calendars.get_calendars_schedules_search` |
| `DELETE` | `/calendars/schedules/{id}` | Delete user availability schedule | `delete_user_availability_schedule()` | `calendars.delete_calendars_schedules_by_id` |
| `GET` | `/calendars/schedules/{id}` | Get user availability schedule | `get_user_availability_schedule()` | `calendars.get_calendars_schedules_by_id` |
| `PUT` | `/calendars/schedules/{id}` | Update user availability schedule | `update_user_availability_schedule()` | `calendars.put_calendars_schedules_by_id` |
| `DELETE` | `/calendars/schedules/{id}/associations/{calendarId}` | Remove user availability schedule from a calendar | `remove_user_availability_schedule_from_a_calendar()` | `calendars.delete_calendars_schedules_by_id_associations_by_calendarId` |
| `PUT` | `/calendars/schedules/{id}/associations/{calendarId}` | Apply user availability schedule to a calendar | `apply_user_availability_schedule_to_a_calendar()` | `calendars.put_calendars_schedules_by_id_associations_by_calendarId` |
| `DELETE` | `/calendars/{calendarId}` | Delete Calendar | `delete_calendar()` | `calendars.delete_calendars_by_calendarId` |
| `GET` | `/calendars/{calendarId}` | Get Calendar | `get_calendar()` | `calendars.get_calendars_by_calendarId` |
| `PUT` | `/calendars/{calendarId}` | Update Calendar | `update_calendar()` | `calendars.put_calendars_by_calendarId` |
| `GET` | `/calendars/{calendarId}/free-slots` | Get Free Slots | `get_free_slots()` | `calendars.get_calendars_by_calendarId_free_slots` |
| `GET` | `/calendars/{calendarId}/notifications` | Get notifications | `get_notifications()` | `calendars.get_calendars_by_calendarId_notifications` |
| `POST` | `/calendars/{calendarId}/notifications` | Create notification | `create_notification()` | `calendars.post_calendars_by_calendarId_notifications` |
| `DELETE` | `/calendars/{calendarId}/notifications/{notificationId}` | Delete Notification | `delete_notification()` | `calendars.delete_calendars_by_calendarId_notifications_by_notificationId` |
| `GET` | `/calendars/{calendarId}/notifications/{notificationId}` | Get notification | `get_notification()` | `calendars.get_calendars_by_calendarId_notifications_by_notificationId` |
| `PUT` | `/calendars/{calendarId}/notifications/{notificationId}` | Update notification | `update_notification()` | `calendars.put_calendars_by_calendarId_notifications_by_notificationId` |

### Endpoint details — v2

#### `GET /calendars/`

**Get Calendars**

Get all calendars in a location.

Operation id: `calendars.get_calendars` · `Version: 2021-04-15` · Scopes: `calendars.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `groupId` | string | no | Group Id |
| `showDrafted` | boolean | no | Show drafted |

*Response*: [`CalendarsGetSuccessfulResponseDTO`](#calendarsgetsuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::calendars::GetCalendarsParams;

let params = GetCalendarsParams::new("locationId");
let out = ghl.calendars().get_calendars(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.get_calendars",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /calendars/`

**Create Calendar**

Create calendar in a location.

Operation id: `calendars.post_calendars` · `Version: 2021-04-15` · Scopes: `calendars.write`

*Request body*: [`CalendarCreateDTO`](#calendarcreatedto)

*Response*: [`CalendarByIdSuccessfulResponseDTO`](#calendarbyidsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().create_calendar(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.post_calendars",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /calendars/appointments/{appointmentId}/notes`

**Get Notes**

Get Appointment Notes

Operation id: `calendars.get_calendars_appointments_by_appointmentId_notes` · `Version: 2021-04-15` · Scopes: `calendars/events.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `appointmentId` | string | **yes** | Appointment ID |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | number | **yes** | Limit of notes to fetch |
| `offset` | number | **yes** | Offset of notes to fetch |

*Response*: [`GetNotesListSuccessfulResponseDto`](#getnoteslistsuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::calendars::GetNotesParams;

let params = GetNotesParams::new("limit", "offset");
let out = ghl.calendars().get_notes(&appointmentId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.get_calendars_appointments_by_appointmentId_notes",
    "path_params": {
      "appointmentId": "<appointmentId>"
    },
    "query": {
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `POST /calendars/appointments/{appointmentId}/notes`

**Create Note**

Operation id: `calendars.post_calendars_appointments_by_appointmentId_notes` · `Version: 2021-04-15` · Scopes: `calendars/events.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `appointmentId` | string | **yes** | Appointment ID |

*Request body*: [`NotesDTO`](#notesdto)

*Response*: [`GetCreateUpdateNoteSuccessfulResponseDto`](#getcreateupdatenotesuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().create_note(&appointmentId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.post_calendars_appointments_by_appointmentId_notes",
    "path_params": {
      "appointmentId": "<appointmentId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /calendars/appointments/{appointmentId}/notes/{noteId}`

**Delete Note**

Operation id: `calendars.delete_calendars_appointments_by_appointmentId_notes_by_noteId` · `Version: 2021-04-15` · Scopes: `calendars/events.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `appointmentId` | string | **yes** | Appointment ID |

*Response*: [`DeleteNoteSuccessfulResponseDto`](#deletenotesuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().delete_note(&appointmentId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.delete_calendars_appointments_by_appointmentId_notes_by_noteId",
    "path_params": {
      "appointmentId": "<appointmentId>"
    }
  }
}
```

</details>

#### `PUT /calendars/appointments/{appointmentId}/notes/{noteId}`

**Update Note**

Operation id: `calendars.put_calendars_appointments_by_appointmentId_notes_by_noteId` · `Version: 2021-04-15` · Scopes: `calendars/events.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `appointmentId` | string | **yes** | Appointment ID |

*Request body*: [`NotesDTO`](#notesdto)

*Response*: [`GetCreateUpdateNoteSuccessfulResponseDto`](#getcreateupdatenotesuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().update_note(&appointmentId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.put_calendars_appointments_by_appointmentId_notes_by_noteId",
    "path_params": {
      "appointmentId": "<appointmentId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /calendars/blocked-slots`

**Get Blocked Slots**

Operation id: `calendars.get_calendars_blocked_slots` · `Version: 2021-04-15` · Scopes: `calendars/events.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `userId` | string | no | User Id - Owner of an appointment. Either of userId, groupId or calendarId is required |
| `calendarId` | string | no | Either of calendarId, userId or groupId is required |
| `groupId` | string | no | Either of groupId, calendarId or userId is required |
| `startTime` | string | **yes** | Start Time (in millis) |
| `endTime` | string | **yes** | End Time (in millis) |

*Response*: [`GetCalendarEventsSuccessfulResponseDTO`](#getcalendareventssuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::calendars::GetBlockedSlotsParams;

let params = GetBlockedSlotsParams::new("locationId", "startTime", "endTime");
let out = ghl.calendars().get_blocked_slots(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.get_calendars_blocked_slots",
    "query": {
      "locationId": "<locationId>",
      "startTime": "<startTime>",
      "endTime": "<endTime>"
    }
  }
}
```

</details>

#### `GET /calendars/events`

**Get Calendar Events**

Operation id: `calendars.get_calendars_events` · `Version: 2021-04-15` · Scopes: `calendars/events.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `userId` | string | no | User Id - Owner of an appointment. Either of userId, groupId or calendarId is required |
| `calendarId` | string | no | Either of calendarId, userId or groupId is required |
| `groupId` | string | no | Either of groupId, calendarId or userId is required |
| `startTime` | string | **yes** | Start Time (in millis) |
| `endTime` | string | **yes** | End Time (in millis) |

*Response*: [`GetCalendarEventsSuccessfulResponseDTO`](#getcalendareventssuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::calendars::GetCalendarEventsParams;

let params = GetCalendarEventsParams::new("locationId", "startTime", "endTime");
let out = ghl.calendars().get_calendar_events(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.get_calendars_events",
    "query": {
      "locationId": "<locationId>",
      "startTime": "<startTime>",
      "endTime": "<endTime>"
    }
  }
}
```

</details>

#### `POST /calendars/events/appointments`

**Create appointment**

Operation id: `calendars.post_calendars_events_appointments` · `Version: 2021-04-15` · Scopes: `calendars/events.write`

*Request body*: [`AppointmentCreateSchema`](#appointmentcreateschema)

*Response*: [`AppointmentSchemaResponse`](#appointmentschemaresponse)

*Rust*:

```rust,ignore
let out = ghl.calendars().create_appointment_op(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.post_calendars_events_appointments",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /calendars/events/appointments/{eventId}`

**Get Appointment**

Get appointment by ID

Operation id: `calendars.get_calendars_events_appointments_by_eventId` · `Version: 2021-04-15` · Scopes: `calendars/events.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `eventId` | string | **yes** | Event Id or Instance id. For recurring appointments send masterEventId to modify original series. |

*Response*: [`GetCalendarEventSuccessfulResponseDTO`](#getcalendareventsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().get_appointment_op(&eventId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.get_calendars_events_appointments_by_eventId",
    "path_params": {
      "eventId": "<eventId>"
    }
  }
}
```

</details>

#### `PUT /calendars/events/appointments/{eventId}`

**Update Appointment**

Update appointment

Operation id: `calendars.put_calendars_events_appointments_by_eventId` · `Version: 2021-04-15` · Scopes: `calendars/events.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `eventId` | string | **yes** | Event Id or Instance id. For recurring appointments send masterEventId to modify original series. |

*Request body*: [`AppointmentEditSchema`](#appointmenteditschema)

*Response*: [`AppointmentSchemaResponse`](#appointmentschemaresponse)

*Rust*:

```rust,ignore
let out = ghl.calendars().update_appointment(&eventId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.put_calendars_events_appointments_by_eventId",
    "path_params": {
      "eventId": "<eventId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /calendars/events/block-slots`

**Create Block Slot**

Create block slot

Operation id: `calendars.post_calendars_events_block_slots` · `Version: 2021-04-15` · Scopes: `calendars/events.write`

*Request body*: [`BlockSlotCreateRequestDTO`](#blockslotcreaterequestdto)

*Response*: [`BlockedSlotSuccessfulResponseDto`](#blockedslotsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().create_block_slot(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.post_calendars_events_block_slots",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /calendars/events/block-slots/{eventId}`

**Update Block Slot**

Update block slot by ID

Operation id: `calendars.put_calendars_events_block_slots_by_eventId` · `Version: 2021-04-15` · Scopes: `calendars/events.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `eventId` | string | **yes** | Event Id or Instance id. For recurring appointments send masterEventId to modify original series. |

*Request body*: [`BlockSlotEditRequestDTO`](#blocksloteditrequestdto)

*Response*: [`BlockedSlotSuccessfulResponseDto`](#blockedslotsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().update_block_slot(&eventId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.put_calendars_events_block_slots_by_eventId",
    "path_params": {
      "eventId": "<eventId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /calendars/events/{eventId}`

**Delete Event**

Delete event by ID

Operation id: `calendars.delete_calendars_events_by_eventId` · `Version: 2021-04-15` · Scopes: `calendars/events.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `eventId` | string | **yes** | Event Id or Instance id. For recurring appointments send masterEventId to modify original series. |

*Request body*: [`DeleteAppointmentSchema`](#deleteappointmentschema)

*Response*: [`DeleteEventSuccessfulResponseDto`](#deleteeventsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().delete_event(&eventId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.delete_calendars_events_by_eventId",
    "path_params": {
      "eventId": "<eventId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /calendars/groups`

**Get Groups**

Get all calendar groups in a location.

Operation id: `calendars.get_calendars_groups` · `Version: 2021-04-15` · Scopes: `calendars/groups.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Response*: [`AllGroupsSuccessfulResponseDTO`](#allgroupssuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::calendars::GetGroupsParams;

let params = GetGroupsParams::new("locationId");
let out = ghl.calendars().get_groups(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.get_calendars_groups",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /calendars/groups`

**Create Calendar Group**

Operation id: `calendars.post_calendars_groups` · `Version: 2021-04-15` · Scopes: `calendars/groups.write`

*Request body*: [`GroupCreateDTO`](#groupcreatedto)

*Response*: [`GroupCreateSuccessfulResponseDTO`](#groupcreatesuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().create_calendar_group(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.post_calendars_groups",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /calendars/groups/validate-slug`

**Validate group slug**

Validate if group slug is available or not.

Operation id: `calendars.post_calendars_groups_validate_slug` · `Version: 2021-04-15` · Scopes: `calendars/groups.write`

*Request body*: [`ValidateGroupSlugPostBody`](#validategroupslugpostbody)

*Response*: [`ValidateGroupSlugSuccessResponseDTO`](#validategroupslugsuccessresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().validate_group_slug(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.post_calendars_groups_validate_slug",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /calendars/groups/{groupId}`

**Delete Group**

Operation id: `calendars.delete_calendars_groups_by_groupId` · `Version: 2021-04-15` · Scopes: `calendars/groups.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `groupId` | string | **yes** | Group Id |

*Response*: [`GroupSuccessfulResponseDTO`](#groupsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().delete_group(&groupId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.delete_calendars_groups_by_groupId",
    "path_params": {
      "groupId": "<groupId>"
    }
  }
}
```

</details>

#### `PUT /calendars/groups/{groupId}`

**Update Group**

Update Group by group ID

Operation id: `calendars.put_calendars_groups_by_groupId` · `Version: 2021-04-15` · Scopes: `calendars/groups.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `groupId` | string | **yes** | Group Id |

*Request body*: [`GroupUpdateDTO`](#groupupdatedto)

*Response*: [`GroupCreateSuccessfulResponseDTO`](#groupcreatesuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().update_group(&groupId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.put_calendars_groups_by_groupId",
    "path_params": {
      "groupId": "<groupId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /calendars/groups/{groupId}/status`

**Disable Group**

Operation id: `calendars.put_calendars_groups_by_groupId_status` · `Version: 2021-04-15` · Scopes: `calendars/groups.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `groupId` | string | **yes** | Group Id |

*Request body*: [`GroupStatusUpdateParams`](#groupstatusupdateparams)

*Response*: [`GroupSuccessfulResponseDTO`](#groupsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().disable_group(&groupId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.put_calendars_groups_by_groupId_status",
    "path_params": {
      "groupId": "<groupId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /calendars/resources/{resourceType}`

**List Calendar Resources**

List calendar resources by resource type and location ID

Operation id: `calendars.get_calendars_resources_by_resourceType` · `Version: 2021-04-15` · Scopes: `calendars/resources.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `resourceType` | enum: `equipments`, `rooms` | **yes** | Calendar Resource Type |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `limit` | number | **yes** | — |
| `skip` | number | **yes** | — |

*Rust*:

```rust,ignore
use ghl_sdk::services::calendars::ListCalendarResourcesParams;

let params = ListCalendarResourcesParams::new("locationId", "limit", "skip");
let out = ghl.calendars().list_calendar_resources(&resourceType, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.get_calendars_resources_by_resourceType",
    "path_params": {
      "resourceType": "<resourceType>"
    },
    "query": {
      "locationId": "<locationId>",
      "limit": "<limit>",
      "skip": "<skip>"
    }
  }
}
```

</details>

#### `POST /calendars/resources/{resourceType}`

**Create Calendar Resource**

Create calendar resource by resource type

Operation id: `calendars.post_calendars_resources_by_resourceType` · `Version: 2021-04-15` · Scopes: `calendars/resources.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `resourceType` | enum: `equipments`, `rooms` | **yes** | Calendar Resource Type |

*Request body*: [`CreateCalendarResourceDTO`](#createcalendarresourcedto)

*Response*: [`CalendarResourceByIdResponseDTO`](#calendarresourcebyidresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().create_calendar_resource(&resourceType, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.post_calendars_resources_by_resourceType",
    "path_params": {
      "resourceType": "<resourceType>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /calendars/resources/{resourceType}/{id}`

**Delete Calendar Resource**

Delete calendar resource by ID

Operation id: `calendars.delete_calendars_resources_by_resourceType_by_id` · `Version: 2021-04-15` · Scopes: `calendars/resources.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `resourceType` | enum: `equipments`, `rooms` | **yes** | Calendar Resource Type |
| `id` | string | **yes** | Calendar Resource ID |

*Response*: [`ResourceDeleteResponseDTO`](#resourcedeleteresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().delete_calendar_resource(&resourceType, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.delete_calendars_resources_by_resourceType_by_id",
    "path_params": {
      "resourceType": "<resourceType>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /calendars/resources/{resourceType}/{id}`

**Get Calendar Resource**

Get calendar resource by ID

Operation id: `calendars.get_calendars_resources_by_resourceType_by_id` · `Version: 2021-04-15` · Scopes: `calendars/resources.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `resourceType` | enum: `equipments`, `rooms` | **yes** | Calendar Resource Type |
| `id` | string | **yes** | Calendar Resource ID |

*Response*: [`CalendarResourceByIdResponseDTO`](#calendarresourcebyidresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().get_calendar_resource(&resourceType, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.get_calendars_resources_by_resourceType_by_id",
    "path_params": {
      "resourceType": "<resourceType>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `PUT /calendars/resources/{resourceType}/{id}`

**Update Calendar Resource**

Update calendar resource by ID

Operation id: `calendars.put_calendars_resources_by_resourceType_by_id` · `Version: 2021-04-15` · Scopes: `calendars/resources.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `resourceType` | enum: `equipments`, `rooms` | **yes** | Calendar Resource Type |
| `id` | string | **yes** | Calendar Resource ID |

*Request body*: [`UpdateCalendarResourceDTO`](#updatecalendarresourcedto)

*Response*: [`CalendarResourceResponseDTO`](#calendarresourceresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().update_calendar_resource(&resourceType, &id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.put_calendars_resources_by_resourceType_by_id",
    "path_params": {
      "resourceType": "<resourceType>",
      "id": "<id>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /calendars/schedules`

**Create user availability schedule**

Create new schedule with specified rules, timezone, location, user and calendar associations.

Operation id: `calendars.post_calendars_schedules` · `Version: 2021-04-15` · Scopes: `calendars.write`

*Request body*: [`CreateScheduleDTO`](#createscheduledto)

*Response*: [`ScheduleResponseDTO`](#scheduleresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().create_user_availability_schedule(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.post_calendars_schedules",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /calendars/schedules/search`

**List user availability schedule**

Retrieve user availability schedules based on various filters including location, calendar, and user. Supports pagination.

Operation id: `calendars.get_calendars_schedules_search` · `Version: 2021-04-15` · Scopes: `calendars.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID to filter schedules by |
| `userId` | string | **yes** | User ID to filter schedules by specific user |
| `calendarId` | string | no | Calendar ID for filtering schedules by specific calendar |
| `skip` | number | no | Number of items to skip for pagination |
| `limit` | number | no | Maximum number of items to return (max 500) |

*Response*: [`GetAllSchedulesResponseDTO`](#getallschedulesresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::calendars::ListUserAvailabilityScheduleParams;

let params = ListUserAvailabilityScheduleParams::new("locationId", "userId");
let out = ghl.calendars().list_user_availability_schedule(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.get_calendars_schedules_search",
    "query": {
      "locationId": "<locationId>",
      "userId": "<userId>"
    }
  }
}
```

</details>

#### `DELETE /calendars/schedules/{id}`

**Delete user availability schedule**

Permanently remove a schedule and all its associated rules. This action cannot be undone.

Operation id: `calendars.delete_calendars_schedules_by_id` · `Version: 2021-04-15` · Scopes: `calendars.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Unique identifier of the schedule to delete |

*Rust*:

```rust,ignore
let out = ghl.calendars().delete_user_availability_schedule(&id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.delete_calendars_schedules_by_id",
    "path_params": {
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /calendars/schedules/{id}`

**Get user availability schedule**

Retrieve a specific schedule by its unique identifier. Returns detailed information including rules, timezone, and associated calendars/users.

Operation id: `calendars.get_calendars_schedules_by_id` · `Version: 2021-04-15` · Scopes: `calendars.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Unique identifier of the schedule |

*Response*: [`ScheduleResponseDTO`](#scheduleresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().get_user_availability_schedule(&id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.get_calendars_schedules_by_id",
    "path_params": {
      "id": "<id>"
    }
  }
}
```

</details>

#### `PUT /calendars/schedules/{id}`

**Update user availability schedule**

Modify an existing schedule by updating its rules, timezone, and name All fields are optional - only provided fields will be updated.

Operation id: `calendars.put_calendars_schedules_by_id` · `Version: 2021-04-15` · Scopes: `calendars.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Unique identifier of the schedule to update |

*Request body*: [`UpdateScheduleDTO`](#updatescheduledto)

*Response*: [`ScheduleResponseDTO`](#scheduleresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().update_user_availability_schedule(&id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.put_calendars_schedules_by_id",
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

#### `DELETE /calendars/schedules/{id}/associations/{calendarId}`

**Remove user availability schedule from a calendar**

Removes the association between a team calendar and the given schedule by removing the calendarId from the schedule

Operation id: `calendars.delete_calendars_schedules_by_id_associations_by_calendarId` · `Version: 2021-04-15` · Scopes: `calendars.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Unique identifier of the schedule |
| `calendarId` | string | **yes** | Unique identifier of the calendar to remove from the schedule |

*Rust*:

```rust,ignore
let out = ghl.calendars().remove_user_availability_schedule_from_a_calendar(&id, &calendarId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.delete_calendars_schedules_by_id_associations_by_calendarId",
    "path_params": {
      "id": "<id>",
      "calendarId": "<calendarId>"
    }
  }
}
```

</details>

#### `PUT /calendars/schedules/{id}/associations/{calendarId}`

**Apply user availability schedule to a calendar**

Associates a calendar with the given schedule by adding the calendarId to a schedule

Operation id: `calendars.put_calendars_schedules_by_id_associations_by_calendarId` · `Version: 2021-04-15` · Scopes: `calendars.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Unique identifier of the schedule |
| `calendarId` | string | **yes** | Unique identifier of the team calendar to add to the schedule |

*Rust*:

```rust,ignore
let out = ghl.calendars().apply_user_availability_schedule_to_a_calendar(&id, &calendarId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.put_calendars_schedules_by_id_associations_by_calendarId",
    "path_params": {
      "id": "<id>",
      "calendarId": "<calendarId>"
    }
  }
}
```

</details>

#### `DELETE /calendars/{calendarId}`

**Delete Calendar**

Delete calendar by ID

Operation id: `calendars.delete_calendars_by_calendarId` · `Version: 2021-04-15` · Scopes: `calendars.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | Calendar Id |

*Response*: [`CalendarDeleteSuccessfulResponseDTO`](#calendardeletesuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().delete_calendar(&calendarId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.delete_calendars_by_calendarId",
    "path_params": {
      "calendarId": "<calendarId>"
    }
  }
}
```

</details>

#### `GET /calendars/{calendarId}`

**Get Calendar**

Get calendar by ID

Operation id: `calendars.get_calendars_by_calendarId` · `Version: 2021-04-15` · Scopes: `calendars.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | Calendar Id |

*Response*: [`CalendarByIdSuccessfulResponseDTO`](#calendarbyidsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().get_calendar(&calendarId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.get_calendars_by_calendarId",
    "path_params": {
      "calendarId": "<calendarId>"
    }
  }
}
```

</details>

#### `PUT /calendars/{calendarId}`

**Update Calendar**

Update calendar by ID.

Operation id: `calendars.put_calendars_by_calendarId` · `Version: 2021-04-15` · Scopes: `calendars.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | Calendar Id |

*Request body*: [`CalendarUpdateDTO`](#calendarupdatedto)

*Response*: [`CalendarByIdSuccessfulResponseDTO`](#calendarbyidsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().update_calendar(&calendarId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.put_calendars_by_calendarId",
    "path_params": {
      "calendarId": "<calendarId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /calendars/{calendarId}/free-slots`

**Get Free Slots**

Get free slots for a calendar between a date range. Optionally a consumer can also request free slots in a particular timezone and also for a particular user.

Operation id: `calendars.get_calendars_by_calendarId_free_slots` · `Version: 2021-04-15` · Scopes: `calendars.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | Calendar Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `startDate` | number | **yes** | Start Date (**⚠️ Important:** Date range cannot be more than 31 days) |
| `endDate` | number | **yes** | End Date (**⚠️ Important:** Date range cannot be more than 31 days) |
| `timezone` | string | no | The timezone in which the free slots are returned |
| `userId` | string | no | The user for whom the free slots are returned |
| `userIds` | array | no | The users for whom the free slots are returned |

*Rust*:

```rust,ignore
use ghl_sdk::services::calendars::GetFreeSlotsParams;

let params = GetFreeSlotsParams::new("startDate", "endDate");
let out = ghl.calendars().get_free_slots(&calendarId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.get_calendars_by_calendarId_free_slots",
    "path_params": {
      "calendarId": "<calendarId>"
    },
    "query": {
      "startDate": "<startDate>",
      "endDate": "<endDate>"
    }
  }
}
```

</details>

#### `GET /calendars/{calendarId}/notifications`

**Get notifications**

Get calendar notifications based on query

Operation id: `calendars.get_calendars_by_calendarId_notifications` · `Version: 2021-04-15` · Scopes: `calendars/events.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `isActive` | boolean | no | — |
| `deleted` | boolean | no | — |
| `limit` | number | no | Number of records to return |
| `skip` | number | no | Number of records to skip |

*Rust*:

```rust,ignore
use ghl_sdk::services::calendars::GetNotificationsParams;

let params = GetNotificationsParams::new();
let out = ghl.calendars().get_notifications(&calendarId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.get_calendars_by_calendarId_notifications",
    "path_params": {
      "calendarId": "<calendarId>"
    }
  }
}
```

</details>

#### `POST /calendars/{calendarId}/notifications`

**Create notification**

Create Calendar notifications, either one or multiple. All notification settings must be for single calendar only

Operation id: `calendars.post_calendars_by_calendarId_notifications` · `Version: 2021-04-15` · Scopes: `calendars/events.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | — |

*Rust*:

```rust,ignore
let out = ghl.calendars().create_notification(&calendarId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.post_calendars_by_calendarId_notifications",
    "path_params": {
      "calendarId": "<calendarId>"
    }
  }
}
```

</details>

#### `DELETE /calendars/{calendarId}/notifications/{notificationId}`

**Delete Notification**

Delete notification

Operation id: `calendars.delete_calendars_by_calendarId_notifications_by_notificationId` · `Version: 2021-04-15` · Scopes: `calendars/events.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | — |
| `notificationId` | string | **yes** | — |

*Response*: [`CalendarNotificationDeleteResponseDTO`](#calendarnotificationdeleteresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().delete_notification(&calendarId, &notificationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.delete_calendars_by_calendarId_notifications_by_notificationId",
    "path_params": {
      "calendarId": "<calendarId>",
      "notificationId": "<notificationId>"
    }
  }
}
```

</details>

#### `GET /calendars/{calendarId}/notifications/{notificationId}`

**Get notification**

Find Event notification by notificationId

Operation id: `calendars.get_calendars_by_calendarId_notifications_by_notificationId` · `Version: 2021-04-15` · Scopes: `calendars/events.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | — |
| `notificationId` | string | **yes** | — |

*Response*: [`CalendarNotificationResponseDTO`](#calendarnotificationresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().get_notification(&calendarId, &notificationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.get_calendars_by_calendarId_notifications_by_notificationId",
    "path_params": {
      "calendarId": "<calendarId>",
      "notificationId": "<notificationId>"
    }
  }
}
```

</details>

#### `PUT /calendars/{calendarId}/notifications/{notificationId}`

**Update notification**

Update Event notification by id

Operation id: `calendars.put_calendars_by_calendarId_notifications_by_notificationId` · `Version: 2021-04-15` · Scopes: `calendars/events.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | — |
| `notificationId` | string | **yes** | — |

*Request body*: [`UpdateCalendarNotificationsDTO`](#updatecalendarnotificationsdto)

*Response*: [`CalendarNotificationDeleteResponseDTO`](#calendarnotificationdeleteresponsedto)

*Rust*:

```rust,ignore
let out = ghl.calendars().update_notification(&calendarId, &notificationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "calendars.put_calendars_by_calendarId_notifications_by_notificationId",
    "path_params": {
      "calendarId": "<calendarId>",
      "notificationId": "<notificationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

## Endpoints — API v3

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/calendars/` | Get Calendars | `v3:calendars.get_calendars` |
| `POST` | `/calendars/` | Create Calendar | `v3:calendars.post_calendars` |
| `GET` | `/calendars/appointments/{appointmentId}/notes` | Get Notes | `v3:calendars.get_calendars_appointments_by_appointmentId_notes` |
| `POST` | `/calendars/appointments/{appointmentId}/notes` | Create Note | `v3:calendars.post_calendars_appointments_by_appointmentId_notes` |
| `DELETE` | `/calendars/appointments/{appointmentId}/notes/{noteId}` | Delete Note | `v3:calendars.delete_calendars_appointments_by_appointmentId_notes_by_noteId` |
| `PUT` | `/calendars/appointments/{appointmentId}/notes/{noteId}` | Update Note | `v3:calendars.put_calendars_appointments_by_appointmentId_notes_by_noteId` |
| `GET` | `/calendars/blocked-slots` | Get Blocked Slots | `v3:calendars.get_calendars_blocked_slots` |
| `GET` | `/calendars/events` | Get Calendar Events | `v3:calendars.get_calendars_events` |
| `POST` | `/calendars/events/appointments` | Create appointment | `v3:calendars.post_calendars_events_appointments` |
| `GET` | `/calendars/events/appointments/{eventId}` | Get Appointment | `v3:calendars.get_calendars_events_appointments_by_eventId` |
| `PUT` | `/calendars/events/appointments/{eventId}` | Update Appointment | `v3:calendars.put_calendars_events_appointments_by_eventId` |
| `POST` | `/calendars/events/block-slots` | Create Block Slot | `v3:calendars.post_calendars_events_block_slots` |
| `PUT` | `/calendars/events/block-slots/{eventId}` | Update Block Slot | `v3:calendars.put_calendars_events_block_slots_by_eventId` |
| `DELETE` | `/calendars/events/{eventId}` | Delete Event | `v3:calendars.delete_calendars_events_by_eventId` |
| `GET` | `/calendars/groups` | Get Groups | `v3:calendars.get_calendars_groups` |
| `POST` | `/calendars/groups` | Create Calendar Group | `v3:calendars.post_calendars_groups` |
| `POST` | `/calendars/groups/validate-slug` | Validate group slug | `v3:calendars.post_calendars_groups_validate_slug` |
| `DELETE` | `/calendars/groups/{groupId}` | Delete Group | `v3:calendars.delete_calendars_groups_by_groupId` |
| `PUT` | `/calendars/groups/{groupId}` | Update Group | `v3:calendars.put_calendars_groups_by_groupId` |
| `PUT` | `/calendars/groups/{groupId}/status` | Disable Group | `v3:calendars.put_calendars_groups_by_groupId_status` |
| `GET` | `/calendars/resources/{resourceType}` | List Calendar Resources | `v3:calendars.get_calendars_resources_by_resourceType` |
| `POST` | `/calendars/resources/{resourceType}` | Create Calendar Resource | `v3:calendars.post_calendars_resources_by_resourceType` |
| `DELETE` | `/calendars/resources/{resourceType}/{id}` | Delete Calendar Resource | `v3:calendars.delete_calendars_resources_by_resourceType_by_id` |
| `GET` | `/calendars/resources/{resourceType}/{id}` | Get Calendar Resource | `v3:calendars.get_calendars_resources_by_resourceType_by_id` |
| `PUT` | `/calendars/resources/{resourceType}/{id}` | Update Calendar Resource | `v3:calendars.put_calendars_resources_by_resourceType_by_id` |
| `POST` | `/calendars/schedules` | Create user availability schedule | `v3:calendars.post_calendars_schedules` |
| `GET` | `/calendars/schedules/event-calendar/{calendarId}` | Get event calendar availability schedule | `v3:calendars.get_calendars_schedules_event_calendar_by_calendarId` |
| `POST` | `/calendars/schedules/event-calendar/{calendarId}` | Create event calendar availability schedule | `v3:calendars.post_calendars_schedules_event_calendar_by_calendarId` |
| `PUT` | `/calendars/schedules/event-calendar/{calendarId}` | Update event calendar availability schedule | `v3:calendars.put_calendars_schedules_event_calendar_by_calendarId` |
| `GET` | `/calendars/schedules/search` | List user availability schedule | `v3:calendars.get_calendars_schedules_search` |
| `DELETE` | `/calendars/schedules/{id}` | Delete user availability schedule | `v3:calendars.delete_calendars_schedules_by_id` |
| `GET` | `/calendars/schedules/{id}` | Get user availability schedule | `v3:calendars.get_calendars_schedules_by_id` |
| `PUT` | `/calendars/schedules/{id}` | Update user availability schedule | `v3:calendars.put_calendars_schedules_by_id` |
| `DELETE` | `/calendars/schedules/{id}/associations/{calendarId}` | Remove user availability schedule from a calendar | `v3:calendars.delete_calendars_schedules_by_id_associations_by_calendarId` |
| `PUT` | `/calendars/schedules/{id}/associations/{calendarId}` | Apply user availability schedule to a calendar | `v3:calendars.put_calendars_schedules_by_id_associations_by_calendarId` |
| `GET` | `/calendars/services/bookings` | Get Service Bookings | `v3:calendars.get_calendars_services_bookings` |
| `POST` | `/calendars/services/bookings` | Create Service Booking | `v3:calendars.post_calendars_services_bookings` |
| `DELETE` | `/calendars/services/bookings/{bookingId}` | Delete Service Booking | `v3:calendars.delete_calendars_services_bookings_by_bookingId` |
| `GET` | `/calendars/services/bookings/{bookingId}` | Get Service Booking by ID | `v3:calendars.get_calendars_services_bookings_by_bookingId` |
| `PUT` | `/calendars/services/bookings/{bookingId}` | Update Service Booking | `v3:calendars.put_calendars_services_bookings_by_bookingId` |
| `GET` | `/calendars/services/catalog` | Get Services | `v3:calendars.get_calendars_services_catalog` |
| `POST` | `/calendars/services/catalog` | Create Service | `v3:calendars.post_calendars_services_catalog` |
| `DELETE` | `/calendars/services/catalog/{serviceId}` | Delete Service | `v3:calendars.delete_calendars_services_catalog_by_serviceId` |
| `GET` | `/calendars/services/catalog/{serviceId}` | Get Service by ID | `v3:calendars.get_calendars_services_catalog_by_serviceId` |
| `PUT` | `/calendars/services/catalog/{serviceId}` | Update Service | `v3:calendars.put_calendars_services_catalog_by_serviceId` |
| `GET` | `/calendars/services/locations` | Get Service Locations | `v3:calendars.get_calendars_services_locations` |
| `POST` | `/calendars/services/locations` | Create Service Location | `v3:calendars.post_calendars_services_locations` |
| `DELETE` | `/calendars/services/locations/{serviceLocationId}` | Delete Service Location | `v3:calendars.delete_calendars_services_locations_by_serviceLocationId` |
| `GET` | `/calendars/services/locations/{serviceLocationId}` | Get Service Location by ID | `v3:calendars.get_calendars_services_locations_by_serviceLocationId` |
| `PUT` | `/calendars/services/locations/{serviceLocationId}` | Update Service Location | `v3:calendars.put_calendars_services_locations_by_serviceLocationId` |
| `DELETE` | `/calendars/{calendarId}` | Delete Calendar | `v3:calendars.delete_calendars_by_calendarId` |
| `GET` | `/calendars/{calendarId}` | Get Calendar | `v3:calendars.get_calendars_by_calendarId` |
| `PUT` | `/calendars/{calendarId}` | Update Calendar | `v3:calendars.put_calendars_by_calendarId` |
| `GET` | `/calendars/{calendarId}/free-slots` | Get Free Slots | `v3:calendars.get_calendars_by_calendarId_free_slots` |
| `GET` | `/calendars/{calendarId}/notifications` | Get notifications | `v3:calendars.get_calendars_by_calendarId_notifications` |
| `POST` | `/calendars/{calendarId}/notifications` | Create notification | `v3:calendars.post_calendars_by_calendarId_notifications` |
| `DELETE` | `/calendars/{calendarId}/notifications/{notificationId}` | Delete Notification | `v3:calendars.delete_calendars_by_calendarId_notifications_by_notificationId` |
| `GET` | `/calendars/{calendarId}/notifications/{notificationId}` | Get notification | `v3:calendars.get_calendars_by_calendarId_notifications_by_notificationId` |
| `PUT` | `/calendars/{calendarId}/notifications/{notificationId}` | Update notification | `v3:calendars.put_calendars_by_calendarId_notifications_by_notificationId` |

### Endpoint details — v3

#### `GET /calendars/`

**Get Calendars**

Get all calendars in a location.

Operation id: `v3:calendars.get_calendars` · `Version: v3` · Scopes: `calendars.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `groupId` | string | no | Group Id |
| `showDrafted` | boolean | no | Show drafted |

*Response*: [`CalendarsGetSuccessfulResponseDTO`](#calendarsgetsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /calendars/`

**Create Calendar**

Create calendar in a location.

Operation id: `v3:calendars.post_calendars` · `Version: v3` · Scopes: `calendars.write`

*Request body*: [`CalendarCreateDTO`](#calendarcreatedto)

*Response*: [`CalendarByIdSuccessfulResponseDTO`](#calendarbyidsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.post_calendars",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /calendars/appointments/{appointmentId}/notes`

**Get Notes**

Get Appointment Notes

Operation id: `v3:calendars.get_calendars_appointments_by_appointmentId_notes` · `Version: v3` · Scopes: `calendars/events.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `appointmentId` | string | **yes** | Appointment ID |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | number | **yes** | Limit of notes to fetch |
| `offset` | number | **yes** | Offset of notes to fetch |

*Response*: [`GetNotesListSuccessfulResponseDto`](#getnoteslistsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars_appointments_by_appointmentId_notes",
    "path_params": {
      "appointmentId": "<appointmentId>"
    },
    "query": {
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `POST /calendars/appointments/{appointmentId}/notes`

**Create Note**

Operation id: `v3:calendars.post_calendars_appointments_by_appointmentId_notes` · `Version: v3` · Scopes: `calendars/events.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `appointmentId` | string | **yes** | Appointment ID |

*Request body*: [`NotesDTO`](#notesdto)

*Response*: [`GetCreateUpdateNoteSuccessfulResponseDto`](#getcreateupdatenotesuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.post_calendars_appointments_by_appointmentId_notes",
    "path_params": {
      "appointmentId": "<appointmentId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /calendars/appointments/{appointmentId}/notes/{noteId}`

**Delete Note**

Operation id: `v3:calendars.delete_calendars_appointments_by_appointmentId_notes_by_noteId` · `Version: v3` · Scopes: `calendars/events.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `appointmentId` | string | **yes** | Appointment ID |

*Response*: [`DeleteNoteSuccessfulResponseDto`](#deletenotesuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.delete_calendars_appointments_by_appointmentId_notes_by_noteId",
    "path_params": {
      "appointmentId": "<appointmentId>"
    }
  }
}
```

</details>

#### `PUT /calendars/appointments/{appointmentId}/notes/{noteId}`

**Update Note**

Operation id: `v3:calendars.put_calendars_appointments_by_appointmentId_notes_by_noteId` · `Version: v3` · Scopes: `calendars/events.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `appointmentId` | string | **yes** | Appointment ID |

*Request body*: [`NotesDTO`](#notesdto)

*Response*: [`GetCreateUpdateNoteSuccessfulResponseDto`](#getcreateupdatenotesuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.put_calendars_appointments_by_appointmentId_notes_by_noteId",
    "path_params": {
      "appointmentId": "<appointmentId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /calendars/blocked-slots`

**Get Blocked Slots**

Operation id: `v3:calendars.get_calendars_blocked_slots` · `Version: v3` · Scopes: `calendars/events.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `userId` | string | no | User Id - Owner of an appointment. Either of userId, groupId or calendarId is required |
| `calendarId` | string | no | Either of calendarId, userId or groupId is required |
| `groupId` | string | no | Either of groupId, calendarId or userId is required |
| `startTime` | string | **yes** | Start Time (in millis) |
| `endTime` | string | **yes** | End Time (in millis) |

*Response*: [`GetCalendarEventsSuccessfulResponseDTO`](#getcalendareventssuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars_blocked_slots",
    "query": {
      "locationId": "<locationId>",
      "startTime": "<startTime>",
      "endTime": "<endTime>"
    }
  }
}
```

</details>

#### `GET /calendars/events`

**Get Calendar Events**

Operation id: `v3:calendars.get_calendars_events` · `Version: v3` · Scopes: `calendars/events.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `userId` | string | no | User Id - Owner of an appointment. Either of userId, groupId or calendarId is required |
| `calendarId` | string | no | Either of calendarId, userId or groupId is required |
| `groupId` | string | no | Either of groupId, calendarId or userId is required |
| `startTime` | string | **yes** | Start Time (in millis) |
| `endTime` | string | **yes** | End Time (in millis) |

*Response*: [`GetCalendarEventsSuccessfulResponseDTO`](#getcalendareventssuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars_events",
    "query": {
      "locationId": "<locationId>",
      "startTime": "<startTime>",
      "endTime": "<endTime>"
    }
  }
}
```

</details>

#### `POST /calendars/events/appointments`

**Create appointment**

Operation id: `v3:calendars.post_calendars_events_appointments` · `Version: v3` · Scopes: `calendars/events.write`

*Request body*: [`AppointmentCreateSchema`](#appointmentcreateschema)

*Response*: [`AppointmentSchemaResponse`](#appointmentschemaresponse)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.post_calendars_events_appointments",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /calendars/events/appointments/{eventId}`

**Get Appointment**

Get appointment by ID

Operation id: `v3:calendars.get_calendars_events_appointments_by_eventId` · `Version: v3` · Scopes: `calendars/events.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `eventId` | string | **yes** | Event Id or Instance id. For recurring appointments send masterEventId to modify original series. |

*Response*: [`GetCalendarEventSuccessfulResponseDTO`](#getcalendareventsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars_events_appointments_by_eventId",
    "path_params": {
      "eventId": "<eventId>"
    }
  }
}
```

</details>

#### `PUT /calendars/events/appointments/{eventId}`

**Update Appointment**

Update appointment

Operation id: `v3:calendars.put_calendars_events_appointments_by_eventId` · `Version: v3` · Scopes: `calendars/events.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `eventId` | string | **yes** | Event Id or Instance id. For recurring appointments send masterEventId to modify original series. |

*Request body*: [`AppointmentEditSchema`](#appointmenteditschema)

*Response*: [`AppointmentSchemaResponse`](#appointmentschemaresponse)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.put_calendars_events_appointments_by_eventId",
    "path_params": {
      "eventId": "<eventId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /calendars/events/block-slots`

**Create Block Slot**

Create block slot

Operation id: `v3:calendars.post_calendars_events_block_slots` · `Version: v3` · Scopes: `calendars/events.write`

*Request body*: [`BlockSlotCreateRequestDTO`](#blockslotcreaterequestdto)

*Response*: [`BlockedSlotSuccessfulResponseDto`](#blockedslotsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.post_calendars_events_block_slots",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /calendars/events/block-slots/{eventId}`

**Update Block Slot**

Update block slot by ID

Operation id: `v3:calendars.put_calendars_events_block_slots_by_eventId` · `Version: v3` · Scopes: `calendars/events.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `eventId` | string | **yes** | Event Id or Instance id. For recurring appointments send masterEventId to modify original series. |

*Request body*: [`BlockSlotEditRequestDTO`](#blocksloteditrequestdto)

*Response*: [`BlockedSlotSuccessfulResponseDto`](#blockedslotsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.put_calendars_events_block_slots_by_eventId",
    "path_params": {
      "eventId": "<eventId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /calendars/events/{eventId}`

**Delete Event**

Delete event by ID

Operation id: `v3:calendars.delete_calendars_events_by_eventId` · `Version: v3` · Scopes: `calendars/events.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `eventId` | string | **yes** | Event Id or Instance id. For recurring appointments send masterEventId to modify original series. |

*Request body*: [`DeleteAppointmentSchema`](#deleteappointmentschema)

*Response*: [`DeleteEventSuccessfulResponseDto`](#deleteeventsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.delete_calendars_events_by_eventId",
    "path_params": {
      "eventId": "<eventId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /calendars/groups`

**Get Groups**

Get all calendar groups in a location.

Operation id: `v3:calendars.get_calendars_groups` · `Version: v3` · Scopes: `calendars/groups.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Response*: [`AllGroupsSuccessfulResponseDTO`](#allgroupssuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars_groups",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /calendars/groups`

**Create Calendar Group**

Operation id: `v3:calendars.post_calendars_groups` · `Version: v3` · Scopes: `calendars/groups.write`

*Request body*: [`GroupCreateDTO`](#groupcreatedto)

*Response*: [`GroupCreateSuccessfulResponseDTO`](#groupcreatesuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.post_calendars_groups",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /calendars/groups/validate-slug`

**Validate group slug**

Validate if group slug is available or not.

Operation id: `v3:calendars.post_calendars_groups_validate_slug` · `Version: v3` · Scopes: `calendars/groups.write`

*Request body*: [`ValidateGroupSlugPostBody`](#validategroupslugpostbody)

*Response*: [`ValidateGroupSlugSuccessResponseDTO`](#validategroupslugsuccessresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.post_calendars_groups_validate_slug",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /calendars/groups/{groupId}`

**Delete Group**

Operation id: `v3:calendars.delete_calendars_groups_by_groupId` · `Version: v3` · Scopes: `calendars/groups.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `groupId` | string | **yes** | Group Id |

*Response*: [`GroupSuccessfulResponseDTO`](#groupsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.delete_calendars_groups_by_groupId",
    "path_params": {
      "groupId": "<groupId>"
    }
  }
}
```

</details>

#### `PUT /calendars/groups/{groupId}`

**Update Group**

Update Group by group ID

Operation id: `v3:calendars.put_calendars_groups_by_groupId` · `Version: v3` · Scopes: `calendars/groups.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `groupId` | string | **yes** | Group Id |

*Request body*: [`GroupUpdateDTO`](#groupupdatedto)

*Response*: [`GroupCreateSuccessfulResponseDTO`](#groupcreatesuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.put_calendars_groups_by_groupId",
    "path_params": {
      "groupId": "<groupId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /calendars/groups/{groupId}/status`

**Disable Group**

Operation id: `v3:calendars.put_calendars_groups_by_groupId_status` · `Version: v3` · Scopes: `calendars/groups.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `groupId` | string | **yes** | Group Id |

*Request body*: [`GroupStatusUpdateParams`](#groupstatusupdateparams)

*Response*: [`GroupSuccessfulResponseDTO`](#groupsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.put_calendars_groups_by_groupId_status",
    "path_params": {
      "groupId": "<groupId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /calendars/resources/{resourceType}`

**List Calendar Resources**

List calendar resources by resource type and location ID (Services V1)

Operation id: `v3:calendars.get_calendars_resources_by_resourceType` · `Version: v3` · Scopes: `calendars/resources.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `resourceType` | enum: `equipments`, `rooms` | **yes** | Calendar Resource Type |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `limit` | number | **yes** | — |
| `skip` | number | **yes** | — |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars_resources_by_resourceType",
    "path_params": {
      "resourceType": "<resourceType>"
    },
    "query": {
      "locationId": "<locationId>",
      "limit": "<limit>",
      "skip": "<skip>"
    }
  }
}
```

</details>

#### `POST /calendars/resources/{resourceType}`

**Create Calendar Resource**

Create calendar resource by resource type (Services V1)

Operation id: `v3:calendars.post_calendars_resources_by_resourceType` · `Version: v3` · Scopes: `calendars/resources.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `resourceType` | enum: `equipments`, `rooms` | **yes** | Calendar Resource Type |

*Request body*: [`CreateCalendarResourceDTO`](#createcalendarresourcedto)

*Response*: [`CalendarResourceByIdResponseDTO`](#calendarresourcebyidresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.post_calendars_resources_by_resourceType",
    "path_params": {
      "resourceType": "<resourceType>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /calendars/resources/{resourceType}/{id}`

**Delete Calendar Resource**

Delete calendar resource by ID (Services V1)

Operation id: `v3:calendars.delete_calendars_resources_by_resourceType_by_id` · `Version: v3` · Scopes: `calendars/resources.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `resourceType` | enum: `equipments`, `rooms` | **yes** | Calendar Resource Type |
| `id` | string | **yes** | Calendar Resource ID |

*Response*: [`ResourceDeleteResponseDTO`](#resourcedeleteresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.delete_calendars_resources_by_resourceType_by_id",
    "path_params": {
      "resourceType": "<resourceType>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /calendars/resources/{resourceType}/{id}`

**Get Calendar Resource**

Get calendar resource by ID (Services V1)

Operation id: `v3:calendars.get_calendars_resources_by_resourceType_by_id` · `Version: v3` · Scopes: `calendars/resources.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `resourceType` | enum: `equipments`, `rooms` | **yes** | Calendar Resource Type |
| `id` | string | **yes** | Calendar Resource ID |

*Response*: [`CalendarResourceByIdResponseDTO`](#calendarresourcebyidresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars_resources_by_resourceType_by_id",
    "path_params": {
      "resourceType": "<resourceType>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `PUT /calendars/resources/{resourceType}/{id}`

**Update Calendar Resource**

Update calendar resource by ID (Services V1)

Operation id: `v3:calendars.put_calendars_resources_by_resourceType_by_id` · `Version: v3` · Scopes: `calendars/resources.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `resourceType` | enum: `equipments`, `rooms` | **yes** | Calendar Resource Type |
| `id` | string | **yes** | Calendar Resource ID |

*Request body*: [`UpdateCalendarResourceDTO`](#updatecalendarresourcedto)

*Response*: [`CalendarResourceResponseDTO`](#calendarresourceresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.put_calendars_resources_by_resourceType_by_id",
    "path_params": {
      "resourceType": "<resourceType>",
      "id": "<id>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /calendars/schedules`

**Create user availability schedule**

Create new schedule with specified rules, timezone, location, user and calendar associations.

Operation id: `v3:calendars.post_calendars_schedules` · `Version: v3` · Scopes: `calendars.write`

*Request body*: [`CreateScheduleDTO`](#createscheduledto)

*Response*: [`ScheduleResponseDTO`](#scheduleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.post_calendars_schedules",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /calendars/schedules/event-calendar/{calendarId}`

**Get event calendar availability schedule**

Retrieve the availability schedule for a specific event calendar. Returns the schedule associated with the calendar ID provided in the path.

Operation id: `v3:calendars.get_calendars_schedules_event_calendar_by_calendarId` · `Version: v3` · Scopes: `calendars.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | Unique identifier of the event calendar |

*Response*: [`EventCalendarScheduleWrapperDTO`](#eventcalendarschedulewrapperdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars_schedules_event_calendar_by_calendarId",
    "path_params": {
      "calendarId": "<calendarId>"
    }
  }
}
```

</details>

#### `POST /calendars/schedules/event-calendar/{calendarId}`

**Create event calendar availability schedule**

Create a new availability schedule specifically for an event calendar. The calendar ID is provided in the path, and schedule rules and timezone are provided in the request body.

Operation id: `v3:calendars.post_calendars_schedules_event_calendar_by_calendarId` · `Version: v3` · Scopes: `calendars.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | Unique identifier of the event calendar |

*Request body*: [`CreateEventCalendarScheduleDTO`](#createeventcalendarscheduledto)

*Response*: [`EventCalendarScheduleWrapperDTO`](#eventcalendarschedulewrapperdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.post_calendars_schedules_event_calendar_by_calendarId",
    "path_params": {
      "calendarId": "<calendarId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /calendars/schedules/event-calendar/{calendarId}`

**Update event calendar availability schedule**

Update the availability schedule for a specific event calendar. Only provided fields will be updated. The calendar ID is provided in the path.

Operation id: `v3:calendars.put_calendars_schedules_event_calendar_by_calendarId` · `Version: v3` · Scopes: `calendars.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | Unique identifier of the event calendar |

*Request body*: [`UpdateEventCalendarScheduleDTO`](#updateeventcalendarscheduledto)

*Response*: [`EventCalendarScheduleWrapperDTO`](#eventcalendarschedulewrapperdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.put_calendars_schedules_event_calendar_by_calendarId",
    "path_params": {
      "calendarId": "<calendarId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /calendars/schedules/search`

**List user availability schedule**

Retrieve user availability schedules based on various filters including location, calendar, and user. Supports pagination.

Operation id: `v3:calendars.get_calendars_schedules_search` · `Version: v3` · Scopes: `calendars.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID to filter schedules by |
| `userId` | string | **yes** | User ID to filter schedules by specific user |
| `calendarId` | string | no | Calendar ID for filtering schedules by specific calendar |
| `skip` | number | no | Number of items to skip for pagination |
| `limit` | number | no | Maximum number of items to return (max 500) |

*Response*: [`GetAllSchedulesResponseDTO`](#getallschedulesresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars_schedules_search",
    "query": {
      "locationId": "<locationId>",
      "userId": "<userId>"
    }
  }
}
```

</details>

#### `DELETE /calendars/schedules/{id}`

**Delete user availability schedule**

Permanently remove a schedule and all its associated rules. This action cannot be undone.

Operation id: `v3:calendars.delete_calendars_schedules_by_id` · `Version: v3` · Scopes: `calendars.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Unique identifier of the schedule to delete |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.delete_calendars_schedules_by_id",
    "path_params": {
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /calendars/schedules/{id}`

**Get user availability schedule**

Retrieve a specific schedule by its unique identifier. Returns detailed information including rules, timezone, and associated calendars/users.

Operation id: `v3:calendars.get_calendars_schedules_by_id` · `Version: v3` · Scopes: `calendars.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Unique identifier of the schedule |

*Response*: [`ScheduleResponseDTO`](#scheduleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars_schedules_by_id",
    "path_params": {
      "id": "<id>"
    }
  }
}
```

</details>

#### `PUT /calendars/schedules/{id}`

**Update user availability schedule**

Modify an existing schedule by updating its rules, timezone, and name All fields are optional - only provided fields will be updated.

Operation id: `v3:calendars.put_calendars_schedules_by_id` · `Version: v3` · Scopes: `calendars.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Unique identifier of the schedule to update |

*Request body*: [`UpdateScheduleDTO`](#updatescheduledto)

*Response*: [`ScheduleResponseDTO`](#scheduleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.put_calendars_schedules_by_id",
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

#### `DELETE /calendars/schedules/{id}/associations/{calendarId}`

**Remove user availability schedule from a calendar**

Removes the association between a team calendar and the given schedule by removing the calendarId from the schedule

Operation id: `v3:calendars.delete_calendars_schedules_by_id_associations_by_calendarId` · `Version: v3` · Scopes: `calendars.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Unique identifier of the schedule |
| `calendarId` | string | **yes** | Unique identifier of the calendar to remove from the schedule |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.delete_calendars_schedules_by_id_associations_by_calendarId",
    "path_params": {
      "id": "<id>",
      "calendarId": "<calendarId>"
    }
  }
}
```

</details>

#### `PUT /calendars/schedules/{id}/associations/{calendarId}`

**Apply user availability schedule to a calendar**

Associates a calendar with the given schedule by adding the calendarId to a schedule

Operation id: `v3:calendars.put_calendars_schedules_by_id_associations_by_calendarId` · `Version: v3` · Scopes: `calendars.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Unique identifier of the schedule |
| `calendarId` | string | **yes** | Unique identifier of the team calendar to add to the schedule |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.put_calendars_schedules_by_id_associations_by_calendarId",
    "path_params": {
      "id": "<id>",
      "calendarId": "<calendarId>"
    }
  }
}
```

</details>

#### `GET /calendars/services/bookings`

**Get Service Bookings**

Retrieve service bookings for a location within a given date range, with an optional service location filter.

Operation id: `v3:calendars.get_calendars_services_bookings` · `Version: v3` · Scopes: `calendars/events.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `startTime` | string | **yes** | Start Time (timestamp in milliseconds as string) |
| `endTime` | string | **yes** | End Time (timestamp in milliseconds as string) |
| `timezone` | string | no | Timezone |
| `serviceLocationId` | string | no | Service Location ID |

*Response*: [`ServiceBookingsListResponseDTO`](#servicebookingslistresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars_services_bookings",
    "query": {
      "locationId": "<locationId>",
      "startTime": "<startTime>",
      "endTime": "<endTime>"
    }
  }
}
```

</details>

#### `POST /calendars/services/bookings`

**Create Service Booking**

Create a new service booking

Operation id: `v3:calendars.post_calendars_services_bookings` · `Version: v3` · Scopes: `calendars/events.write`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `overrideAvailability` | boolean | no | If true the time slot validation would be avoided for any booking creation/update (even the skipSchedulingNotice) |
| `skipSchedulingNotice` | boolean | no | If set to true, the minimum scheduling notice and date range would be ignored |

*Request body*: [`CreatePublicServiceBookingDTO`](#createpublicservicebookingdto)

*Response*: [`CreateOrUpdateServiceBookingResponseDTO`](#createorupdateservicebookingresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.post_calendars_services_bookings",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /calendars/services/bookings/{bookingId}`

**Delete Service Booking**

Delete a service booking by ID

Operation id: `v3:calendars.delete_calendars_services_bookings_by_bookingId` · `Version: v3` · Scopes: `calendars/events.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `bookingId` | string | **yes** | Unique Service Booking ID |

*Response*: [`DeleteServiceBookingResponseDTO`](#deleteservicebookingresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.delete_calendars_services_bookings_by_bookingId",
    "path_params": {
      "bookingId": "<bookingId>"
    }
  }
}
```

</details>

#### `GET /calendars/services/bookings/{bookingId}`

**Get Service Booking by ID**

Get a specific service booking by ID

Operation id: `v3:calendars.get_calendars_services_bookings_by_bookingId` · `Version: v3` · Scopes: `calendars/events.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `bookingId` | string | **yes** | Unique Service Booking ID |

*Response*: [`ServiceBookingResponseDTO`](#servicebookingresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars_services_bookings_by_bookingId",
    "path_params": {
      "bookingId": "<bookingId>"
    }
  }
}
```

</details>

#### `PUT /calendars/services/bookings/{bookingId}`

**Update Service Booking**

Update an existing service booking

Operation id: `v3:calendars.put_calendars_services_bookings_by_bookingId` · `Version: v3` · Scopes: `calendars/events.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `bookingId` | string | **yes** | Unique Service Booking ID |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `overrideAvailability` | boolean | no | If true the time slot validation would be avoided for any booking creation/update (even the skipSchedulingNotice) |
| `skipSchedulingNotice` | boolean | no | If set to true, the minimum scheduling notice and date range would be ignored |

*Request body*: [`UpdateServiceBookingDTO`](#updateservicebookingdto)

*Response*: [`CreateOrUpdateServiceBookingResponseDTO`](#createorupdateservicebookingresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.put_calendars_services_bookings_by_bookingId",
    "path_params": {
      "bookingId": "<bookingId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /calendars/services/catalog`

**Get Services**

Get all services in a location.

Operation id: `v3:calendars.get_calendars_services_catalog` · `Version: v3` · Scopes: `calendars.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `serviceCategoryId` | string | no | Filter by service category ID |
| `isPrivate` | boolean | no | Filter services: true = private only, false = public only, unset = all services |

*Response*: [`ServicesListResponseDTO`](#serviceslistresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars_services_catalog",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /calendars/services/catalog`

**Create Service**

Create new service in a location.

Operation id: `v3:calendars.post_calendars_services_catalog` · `Version: v3` · Scopes: `calendars.write`

*Request body*: [`CreateServiceDTO`](#createservicedto)

*Response*: [`ServiceResponseWrapperDTO`](#serviceresponsewrapperdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.post_calendars_services_catalog",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /calendars/services/catalog/{serviceId}`

**Delete Service**

Delete service by ID.

Operation id: `v3:calendars.delete_calendars_services_catalog_by_serviceId` · `Version: v3` · Scopes: `calendars.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `serviceId` | string | **yes** | Service ID |

*Response*: [`DeleteServiceResponseDTO`](#deleteserviceresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.delete_calendars_services_catalog_by_serviceId",
    "path_params": {
      "serviceId": "<serviceId>"
    }
  }
}
```

</details>

#### `GET /calendars/services/catalog/{serviceId}`

**Get Service by ID**

Get service by ID.

Operation id: `v3:calendars.get_calendars_services_catalog_by_serviceId` · `Version: v3` · Scopes: `calendars.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `serviceId` | string | **yes** | Service ID |

*Response*: [`ServiceResponseWrapperDTO`](#serviceresponsewrapperdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars_services_catalog_by_serviceId",
    "path_params": {
      "serviceId": "<serviceId>"
    }
  }
}
```

</details>

#### `PUT /calendars/services/catalog/{serviceId}`

**Update Service**

Update service by ID.

Operation id: `v3:calendars.put_calendars_services_catalog_by_serviceId` · `Version: v3` · Scopes: `calendars.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `serviceId` | string | **yes** | Service ID |

*Request body*: [`UpdateServiceDTO`](#updateservicedto)

*Response*: [`ServiceResponseWrapperDTO`](#serviceresponsewrapperdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.put_calendars_services_catalog_by_serviceId",
    "path_params": {
      "serviceId": "<serviceId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /calendars/services/locations`

**Get Service Locations**

Get all service locations

Operation id: `v3:calendars.get_calendars_services_locations` · `Version: v3` · Scopes: `calendars.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Response*: [`ServiceLocationListResponseDTO`](#servicelocationlistresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars_services_locations",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /calendars/services/locations`

**Create Service Location**

Create a new service location

Operation id: `v3:calendars.post_calendars_services_locations` · `Version: v3` · Scopes: `calendars.write`

*Request body*: [`CreateServiceLocationDTO`](#createservicelocationdto)

*Response*: [`ServiceLocationResponseDTO`](#servicelocationresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.post_calendars_services_locations",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /calendars/services/locations/{serviceLocationId}`

**Delete Service Location**

Delete a service location by ID

Operation id: `v3:calendars.delete_calendars_services_locations_by_serviceLocationId` · `Version: v3` · Scopes: `calendars.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `serviceLocationId` | string | **yes** | Unique Service Location ID |

*Response*: [`DeleteServiceLocationResponseDTO`](#deleteservicelocationresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.delete_calendars_services_locations_by_serviceLocationId",
    "path_params": {
      "serviceLocationId": "<serviceLocationId>"
    }
  }
}
```

</details>

#### `GET /calendars/services/locations/{serviceLocationId}`

**Get Service Location by ID**

Get service location by ID

Operation id: `v3:calendars.get_calendars_services_locations_by_serviceLocationId` · `Version: v3` · Scopes: `calendars.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `serviceLocationId` | string | **yes** | Unique Service Location ID |

*Response*: [`ServiceLocationResponseDTO`](#servicelocationresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars_services_locations_by_serviceLocationId",
    "path_params": {
      "serviceLocationId": "<serviceLocationId>"
    }
  }
}
```

</details>

#### `PUT /calendars/services/locations/{serviceLocationId}`

**Update Service Location**

Update an existing service location

Operation id: `v3:calendars.put_calendars_services_locations_by_serviceLocationId` · `Version: v3` · Scopes: `calendars.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `serviceLocationId` | string | **yes** | Unique Service Location ID |

*Request body*: [`UpdateServiceLocationDTO`](#updateservicelocationdto)

*Response*: [`ServiceLocationResponseDTO`](#servicelocationresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.put_calendars_services_locations_by_serviceLocationId",
    "path_params": {
      "serviceLocationId": "<serviceLocationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /calendars/{calendarId}`

**Delete Calendar**

Delete calendar by ID

Operation id: `v3:calendars.delete_calendars_by_calendarId` · `Version: v3` · Scopes: `calendars.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | Calendar Id |

*Response*: [`CalendarDeleteSuccessfulResponseDTO`](#calendardeletesuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.delete_calendars_by_calendarId",
    "path_params": {
      "calendarId": "<calendarId>"
    }
  }
}
```

</details>

#### `GET /calendars/{calendarId}`

**Get Calendar**

Get calendar by ID

Operation id: `v3:calendars.get_calendars_by_calendarId` · `Version: v3` · Scopes: `calendars.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | Calendar Id |

*Response*: [`CalendarByIdSuccessfulResponseDTO`](#calendarbyidsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars_by_calendarId",
    "path_params": {
      "calendarId": "<calendarId>"
    }
  }
}
```

</details>

#### `PUT /calendars/{calendarId}`

**Update Calendar**

Update calendar by ID.

Operation id: `v3:calendars.put_calendars_by_calendarId` · `Version: v3` · Scopes: `calendars.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | Calendar Id |

*Request body*: [`CalendarUpdateDTO`](#calendarupdatedto)

*Response*: [`CalendarByIdSuccessfulResponseDTO`](#calendarbyidsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.put_calendars_by_calendarId",
    "path_params": {
      "calendarId": "<calendarId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /calendars/{calendarId}/free-slots`

**Get Free Slots**

Get free slots for a calendar between a date range. Optionally a consumer can also request free slots in a particular timezone and also for a particular user.

Operation id: `v3:calendars.get_calendars_by_calendarId_free_slots` · `Version: v3` · Scopes: `calendars.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | Calendar Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `startDate` | number | **yes** | Start Date (**⚠️ Important:** Date range cannot be more than 31 days) |
| `endDate` | number | **yes** | End Date (**⚠️ Important:** Date range cannot be more than 31 days) |
| `timezone` | string | no | The timezone in which the free slots are returned |
| `userId` | string | no | The user for whom the free slots are returned |
| `userIds` | array | no | The users for whom the free slots are returned |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars_by_calendarId_free_slots",
    "path_params": {
      "calendarId": "<calendarId>"
    },
    "query": {
      "startDate": "<startDate>",
      "endDate": "<endDate>"
    }
  }
}
```

</details>

#### `GET /calendars/{calendarId}/notifications`

**Get notifications**

Get calendar notifications based on query

Operation id: `v3:calendars.get_calendars_by_calendarId_notifications` · `Version: v3` · Scopes: `calendars/events.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `isActive` | boolean | no | — |
| `deleted` | boolean | no | — |
| `limit` | number | no | Number of records to return |
| `skip` | number | no | Number of records to skip |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars_by_calendarId_notifications",
    "path_params": {
      "calendarId": "<calendarId>"
    }
  }
}
```

</details>

#### `POST /calendars/{calendarId}/notifications`

**Create notification**

Create Calendar notifications, either one or multiple. All notification settings must be for single calendar only

Operation id: `v3:calendars.post_calendars_by_calendarId_notifications` · `Version: v3` · Scopes: `calendars/events.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | — |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.post_calendars_by_calendarId_notifications",
    "path_params": {
      "calendarId": "<calendarId>"
    }
  }
}
```

</details>

#### `DELETE /calendars/{calendarId}/notifications/{notificationId}`

**Delete Notification**

Delete notification

Operation id: `v3:calendars.delete_calendars_by_calendarId_notifications_by_notificationId` · `Version: v3` · Scopes: `calendars/events.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | — |
| `notificationId` | string | **yes** | — |

*Response*: [`CalendarNotificationDeleteResponseDTO`](#calendarnotificationdeleteresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.delete_calendars_by_calendarId_notifications_by_notificationId",
    "path_params": {
      "calendarId": "<calendarId>",
      "notificationId": "<notificationId>"
    }
  }
}
```

</details>

#### `GET /calendars/{calendarId}/notifications/{notificationId}`

**Get notification**

Find Event notification by notificationId

Operation id: `v3:calendars.get_calendars_by_calendarId_notifications_by_notificationId` · `Version: v3` · Scopes: `calendars/events.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | — |
| `notificationId` | string | **yes** | — |

*Response*: [`CalendarNotificationResponseDTO`](#calendarnotificationresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.get_calendars_by_calendarId_notifications_by_notificationId",
    "path_params": {
      "calendarId": "<calendarId>",
      "notificationId": "<notificationId>"
    }
  }
}
```

</details>

#### `PUT /calendars/{calendarId}/notifications/{notificationId}`

**Update notification**

Update Event notification by id

Operation id: `v3:calendars.put_calendars_by_calendarId_notifications_by_notificationId` · `Version: v3` · Scopes: `calendars/events.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `calendarId` | string | **yes** | — |
| `notificationId` | string | **yes** | — |

*Request body*: [`UpdateCalendarNotificationsDTO`](#updatecalendarnotificationsdto)

*Response*: [`CalendarNotificationDeleteResponseDTO`](#calendarnotificationdeleteresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:calendars.put_calendars_by_calendarId_notifications_by_notificationId",
    "path_params": {
      "calendarId": "<calendarId>",
      "notificationId": "<notificationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::calendars::*` (enable the `calendars` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/calendars/).

### `AllGroupsSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `groups` | Vec<GroupDTO> | no | — |

### `AppointmentCreateSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | no | Title |
| `meetingLocationType` | String — `custom`, `zoom`, `gmeet`, `phone`, `address`, `ms_teams`, `google` | no | Meeting location type. - If `address` is provided in the request body, the `meetingLocationType` defaults to **custom**. |
| `meetingLocationId` | String | no | The unique identifier for the meeting location. - This value can be found in `calendar.locationConfigurations`or `calendar.teamMembers[].locationConfigurations` |
| `overrideLocationConfig` | bool | no | Flag to override location config - **false** - If only `meetingLocationId` is provided - **true** - If only `meetingLocationType` is provided |
| `appointmentStatus` | String — `new`, `confirmed`, `cancelled`, `showed`, `noshow`, `invalid` | no | — |
| `assignedUserId` | String | no | Assigned User Id |
| `description` | String | no | Appointment Description |
| `address` | String | no | Appointment Address |
| `ignoreDateRange` | bool | no | If set to true, the minimum scheduling notice and date range would be ignored |
| `toNotify` | bool | no | If set to false, the automations will not run |
| `ignoreFreeSlotValidation` | bool | no | If true the time slot validation would be avoided for any appointment creation (even the ignoreDateRange) |
| `rrule` | String | no | RRULE as per the iCalendar (RFC 5545) specification for recurring events. DTSTART is not required, instance ids are calculated on the basis of startTime of the event. The rrule only be applied if igno… |
| `calendarId` | String | **yes** | Calendar Id |
| `locationId` | String | **yes** | Location Id |
| `contactId` | String | **yes** | Contact Id |
| `startTime` | String | **yes** | Start Time |
| `endTime` | String | no | End Time |

### `AppointmentEditSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | no | Title |
| `meetingLocationType` | String — `custom`, `zoom`, `gmeet`, `phone`, `address`, `ms_teams`, `google` | no | Meeting location type. - If `address` is provided in the request body, the `meetingLocationType` defaults to **custom**. |
| `meetingLocationId` | String | no | The unique identifier for the meeting location. - This value can be found in `calendar.locationConfigurations`or `calendar.teamMembers[].locationConfigurations` |
| `overrideLocationConfig` | bool | no | Flag to override location config - **false** - If only `meetingLocationId` is provided - **true** - If only `meetingLocationType` is provided |
| `appointmentStatus` | String — `new`, `confirmed`, `cancelled`, `showed`, `noshow`, `invalid` | no | — |
| `assignedUserId` | String | no | Assigned User Id |
| `description` | String | no | Appointment Description |
| `address` | String | no | Appointment Address |
| `ignoreDateRange` | bool | no | If set to true, the minimum scheduling notice and date range would be ignored |
| `toNotify` | bool | no | If set to false, the automations will not run |
| `ignoreFreeSlotValidation` | bool | no | If true the time slot validation would be avoided for any appointment creation (even the ignoreDateRange) |
| `rrule` | String | no | RRULE as per the iCalendar (RFC 5545) specification for recurring events. DTSTART is not required, instance ids are calculated on the basis of startTime of the event. The rrule only be applied if igno… |
| `calendarId` | String | no | Calendar Id |
| `startTime` | String | no | Start Time |
| `endTime` | String | no | End Time |

### `AppointmentSchemaResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `calendarId` | String | **yes** | Calendar Id |
| `locationId` | String | **yes** | Location Id |
| `contactId` | String | **yes** | Contact Id |
| `startTime` | String | no | Start Time |
| `endTime` | String | no | End Time |
| `title` | String | no | Title |
| `meetingLocationType` | String | no | Meeting Location Type |
| `appointmentStatus` | String — `new`, `confirmed`, `cancelled`, `showed`, `noshow`, `invalid`, `active`, `completed` | no | — |
| `assignedUserId` | String | no | Assigned User Id |
| `address` | String | no | Appointment Address |
| `isRecurring` | bool | no | true if the event is recurring otherwise false |
| `rrule` | String | no | RRULE as per the iCalendar (RFC 5545) specification for recurring events |
| `id` | String | **yes** | Id |

### `Availability`

| Field | Type | Required | Description |
|---|---|---|---|
| `date` | String | **yes** | Formulate the date string in the format of `<YYYY-MM-DD in local timezone>T00:00:00.000Z`. |
| `hours` | Vec<Hour> | **yes** | — |
| `deleted` | bool | no | — |

### `BlockSlotCreateRequestDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | no | Title |
| `calendarId` | String | **yes** | Either calendarId or assignedUserId can be set, not both. |
| `assignedUserId` | String | no | Either calendarId or assignedUserId can be set, not both. |
| `locationId` | String | **yes** | Location Id |
| `startTime` | String | no | Start Time |
| `endTime` | String | no | End Time |

### `BlockSlotEditRequestDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | no | Title |
| `calendarId` | String | **yes** | Either calendarId or assignedUserId can be set, not both. |
| `assignedUserId` | String | no | Either calendarId or assignedUserId can be set, not both. |
| `locationId` | String | **yes** | Location Id |
| `startTime` | String | no | Start Time |
| `endTime` | String | no | End Time |

### `BlockedSlotSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Id |
| `locationId` | String | **yes** | Location Id |
| `title` | String | **yes** | Title |
| `startTime` | JSON | **yes** | Start Time |
| `endTime` | JSON | **yes** | End Time |
| `calendarId` | String | no | Calendar id |
| `assignedUserId` | String | no | Assigned User Id |

### `CalendarByIdSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `calendar` | [`CalendarDTO`](#calendardto) | **yes** | — |

### `CalendarCreateDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `isActive` | bool | no | Should the created calendar be active or draft |
| `notifications` | Vec<CalendarNotification> | no | 🚨 Deprecated! Please use 'Calendar Notifications APIs' instead. |
| `locationId` | String | **yes** | — |
| `groupId` | String | no | Group Id |
| `teamMembers` | Vec<TeamMember> | no | Team members are required for calendars of type: Round Robin, Collective, Class, Service. Personal calendar must have exactly one team member. |
| `eventType` | String — `RoundRobin_OptimizeForAvailability`, `RoundRobin_OptimizeForEqualDistribution` | no | — |
| `name` | String | **yes** | — |
| `description` | String | no | — |
| `slug` | String | no | — |
| `widgetSlug` | String | no | — |
| `calendarType` | String — `round_robin`, `event`, `class_booking`, `collective`, `service_booking`, `personal` | no | — |
| `widgetType` | String — `default`, `classic` | no | Calendar widget type. Choose "default" for "neo" and "classic" for "classic" layout. |
| `eventTitle` | String | no | — |
| `eventColor` | String | no | — |
| `meetingLocation` | String | no | 🚨 Deprecated! Use `locationConfigurations.location` or `teamMembers[].locationConfigurations.location` instead. |
| `locationConfigurations` | Vec<LocationConfiguration> | no | Meeting location configuration for event calendar |
| `slotDuration` | f64 | no | This controls the duration of the meeting |
| `slotDurationUnit` | String — `mins`, `hours` | no | Unit for slot duration. |
| `slotInterval` | f64 | no | Slot interval reflects the amount of time the between booking slots that will be shown in the calendar. |
| `slotIntervalUnit` | String — `mins`, `hours` | no | Unit for slot interval. |
| `slotBuffer` | f64 | no | Slot-Buffer is additional time that can be added after an appointment, allowing for extra time to wrap up |
| `slotBufferUnit` | String — `mins`, `hours` | no | Unit for slot buffer. |
| `preBuffer` | f64 | no | Pre-Buffer is additional time that can be added before an appointment, allowing for extra time to get ready |
| `preBufferUnit` | String — `mins`, `hours` | no | Unit for pre-buffer. |
| `appoinmentPerSlot` | f64 | no | Maximum bookings per slot (per user). Maximum seats per slot in case of Class Booking Calendar. |
| `appoinmentPerDay` | f64 | no | Number of appointments that can be booked for a given day |
| `allowBookingAfter` | f64 | no | Minimum scheduling notice for events |
| `allowBookingAfterUnit` | String — `hours`, `days`, `weeks`, `months` | no | Unit for minimum scheduling notice |
| `allowBookingFor` | f64 | no | Minimum number of days/weeks/months for which to allow booking events |
| `allowBookingForUnit` | String — `days`, `weeks`, `months` | no | Unit for controlling the duration for which booking would be allowed for |
| `openHours` | Vec<OpenHour> | no | This is only to set the standard availability. For custom availability, use the availabilities property |
| `enableRecurring` | bool | no | Enable recurring appointments for the calendars. Please note that only one member should be added in the calendar to enable this |
| `recurring` | [`Recurring`](#recurring) | no | — |
| `formId` | String | no | — |
| `stickyContact` | bool | no | — |
| `isLivePaymentMode` | bool | no | — |
| `autoConfirm` | bool | no | — |
| `shouldSendAlertEmailsToAssignedMember` | bool | no | — |
| `alertEmail` | String | no | — |
| `googleInvitationEmails` | bool | no | — |
| `allowReschedule` | bool | no | — |
| `allowCancellation` | bool | no | — |
| `shouldAssignContactToTeamMember` | bool | no | — |
| `shouldSkipAssigningContactForExisting` | bool | no | — |
| `notes` | String | no | — |
| `pixelId` | String | no | — |
| `formSubmitType` | String — `RedirectURL`, `ThankYouMessage` | no | — |
| `formSubmitRedirectURL` | String | no | — |
| `formSubmitThanksMessage` | String | no | — |
| `availabilityType` | String — `0`, `1` | no | Determines which availability type to consider: - **1**: Only custom availabilities will be used. - **0**: Only open hours will be used. - **null**: Both custom availabilities and open hours will be c… |
| `availabilities` | Vec<Availability> | no | This is only to set the custom availability. For standard availability, use the openHours property |
| `guestType` | String — `count_only`, `collect_detail` | no | — |
| `consentLabel` | String | no | — |
| `calendarCoverImage` | String | no | — |
| `lookBusyConfig` | [`LookBusyConfiguration`](#lookbusyconfiguration) | no | Look Busy Configuration |

### `CalendarDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `isActive` | bool | no | Should the created calendar be active or draft |
| `notifications` | Vec<CalendarNotification> | no | 🚨 Deprecated! Please use 'Calendar Notifications APIs' instead. |
| `locationId` | String | **yes** | — |
| `groupId` | String | no | Group Id |
| `teamMembers` | Vec<TeamMemberResponse> | no | Team members are for calendars of type: Round Robin, Collective, Class, Service. Personal calendar must have exactly one team member. |
| `eventType` | String — `RoundRobin_OptimizeForAvailability`, `RoundRobin_OptimizeForEqualDistribution` | no | — |
| `name` | String | **yes** | — |
| `description` | String | no | — |
| `slug` | String | no | — |
| `widgetSlug` | String | no | — |
| `calendarType` | String — `round_robin`, `event`, `class_booking`, `collective`, `service_booking`, `personal` | no | — |
| `widgetType` | String — `default`, `classic` | no | Calendar widget type. Choose "default" for "neo" and "classic" for "classic" layout. |
| `eventTitle` | String | no | — |
| `eventColor` | String | no | — |
| `meetingLocation` | String | no | 🚨 Deprecated! Use `locationConfigurations.location` or `teamMembers[].locationConfigurations.location` instead. |
| `locationConfigurations` | Vec<LocationConfigurationResponse> | no | Meeting location configuration for event calendar |
| `slotDuration` | f64 | no | This controls the duration of the meeting |
| `slotDurationUnit` | String — `mins`, `hours` | no | Unit for slot duration. |
| `slotInterval` | f64 | no | Slot interval reflects the amount of time the between booking slots that will be shown in the calendar. |
| `slotIntervalUnit` | String — `mins`, `hours` | no | Unit for slot interval. |
| `slotBuffer` | f64 | no | Slot-Buffer is additional time that can be added after an appointment, allowing for extra time to wrap up |
| `slotBufferUnit` | String — `mins`, `hours` | no | Unit for slot buffer. |
| `preBuffer` | f64 | no | Pre-Buffer is additional time that can be added before an appointment, allowing for extra time to get ready |
| `preBufferUnit` | String — `mins`, `hours` | no | Unit for pre-buffer. |
| `appoinmentPerSlot` | f64 | no | Maximum bookings per slot (per user). Maximum seats per slot in case of Class Booking Calendar. |
| `appoinmentPerDay` | f64 | no | Number of appointments that can be booked for a given day |
| `allowBookingAfter` | f64 | no | Minimum scheduling notice for events |
| `allowBookingAfterUnit` | String — `hours`, `days`, `weeks`, `months` | no | Unit for minimum scheduling notice |
| `allowBookingFor` | f64 | no | Minimum number of days/weeks/months for which to allow booking events |
| `allowBookingForUnit` | String — `days`, `weeks`, `months` | no | Unit for controlling the duration for which booking would be allowed for |
| `openHours` | Vec<OpenHour> | no | This is only to set the standard availability. For custom availability, use the availabilities property |
| `enableRecurring` | bool | no | Enable recurring appointments for the calendars. Please note that only one member should be added in the calendar to enable this |
| `recurring` | [`Recurring`](#recurring) | no | — |
| `formId` | String | no | — |
| `stickyContact` | bool | no | — |
| `isLivePaymentMode` | bool | no | — |
| `autoConfirm` | bool | no | — |
| `shouldSendAlertEmailsToAssignedMember` | bool | no | — |
| `alertEmail` | String | no | — |
| `googleInvitationEmails` | bool | no | — |
| `allowReschedule` | bool | no | — |
| `allowCancellation` | bool | no | — |
| `shouldAssignContactToTeamMember` | bool | no | — |
| `shouldSkipAssigningContactForExisting` | bool | no | — |
| `notes` | String | no | — |
| `pixelId` | String | no | — |
| `formSubmitType` | String — `RedirectURL`, `ThankYouMessage` | no | — |
| `formSubmitRedirectURL` | String | no | — |
| `formSubmitThanksMessage` | String | no | — |
| `availabilityType` | String — `0`, `1` | no | Determines which availability type to consider: - **1**: Only custom availabilities will be used. - **0**: Only open hours will be used. - **null**: Both custom availabilities and open hours will be c… |
| `availabilities` | Vec<Availability> | no | This is only to set the custom availability. For standard availability, use the openHours property |
| `guestType` | String — `count_only`, `collect_detail` | no | — |
| `consentLabel` | String | no | — |
| `calendarCoverImage` | String | no | — |
| `lookBusyConfig` | [`LookBusyConfiguration`](#lookbusyconfiguration) | no | Look Busy Configuration |
| `id` | String | **yes** | — |

### `CalendarDeleteSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success |

### `CalendarEventDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Event Id or Instance id for a recurring event |
| `address` | String | no | Calendar Event address |
| `title` | String | **yes** | Calendar Event title |
| `calendarId` | String | **yes** | Calendar ID |
| `locationId` | String | **yes** | Location ID |
| `contactId` | String | **yes** | Contact ID |
| `groupId` | String | **yes** | Group ID |
| `appointmentStatus` | String | **yes** | Appointment Status |
| `assignedUserId` | String | **yes** | AssignedUser - the primary owner of an appointment |
| `users` | Vec<String> | **yes** | Users - the secondary owners of an appointment. |
| `notes` | String | no | Notes |
| `description` | String | no | Description |
| `isRecurring` | bool | no | true if the event is recurring otherwise false |
| `rrule` | String | no | RRULE as per the iCalendar (RFC 5545) specification for recurring events. DTSTART is not required, instance ids are calculated on the basis of startTime of the event. |
| `startTime` | JSON | **yes** | Start Time |
| `endTime` | JSON | **yes** | End Time |
| `dateAdded` | JSON | **yes** | Date Added |
| `dateUpdated` | JSON | **yes** | Date Updated |
| `assignedResources` | Vec<String> | no | Ids of associated resources rooms and/or equipments |
| `createdBy` | [`CreatedOrUpdatedBy`](#createdorupdatedby) | no | Appointment booked by metadata |
| `masterEventId` | String | no | Master event id for a recurring instance |

### `CalendarNotification`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `email` | no | Calendar Notification |
| `shouldSendToContact` | bool | **yes** | — |
| `shouldSendToGuest` | bool | **yes** | — |
| `shouldSendToUser` | bool | **yes** | — |
| `shouldSendToSelectedUsers` | bool | **yes** | — |
| `selectedUsers` | String | **yes** | Comma separated emails |

### `CalendarNotificationDeleteResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | **yes** | Result of delete/update operation |

### `CalendarNotificationResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | Notification ID |
| `receiverType` | String — `contact`, `guest`, `assignedUser`, `emails`, `phoneNumbers`, `business` | no | — |
| `additionalEmailIds` | Vec<String> | no | — |
| `additionalPhoneNumbers` | Vec<String> | no | — |
| `channel` | String — `email`, `inApp`, `sms`, `whatsapp` | no | — |
| `notificationType` | String — `booked`, `confirmation`, `cancellation`, `reminder`, `followup`, `reschedule` | no | — |
| `isActive` | bool | no | — |
| `additionalWhatsappNumbers` | Vec<String> | no | — |
| `templateId` | String | no | — |
| `body` | String | no | — |
| `subject` | String | no | — |
| `afterTime` | Vec<SchedulesDTO> | no | — |
| `beforeTime` | Vec<SchedulesDTO> | no | — |
| `selectedUsers` | Vec<String> | no | — |
| `deleted` | bool | no | — |

### `CalendarResourceByIdResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID of the resource |
| `name` | String | **yes** | Name of the resource |
| `resourceType` | String — `equipments`, `rooms` | **yes** | — |
| `isActive` | bool | **yes** | Whether the resource is active |
| `description` | String | no | Description of the resource |
| `quantity` | f64 | no | Quantity of the resource |
| `outOfService` | f64 | no | Indicates if the resource is out of service |
| `capacity` | f64 | no | Capacity of the resource |
| `calendarIds` | Vec<String> | **yes** | Calendar IDs |

### `CalendarResourceResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID of the resource |
| `name` | String | **yes** | Name of the resource |
| `resourceType` | String — `equipments`, `rooms` | **yes** | — |
| `isActive` | bool | **yes** | Whether the resource is active |
| `description` | String | no | Description of the resource |
| `quantity` | f64 | no | Quantity of the resource |
| `outOfService` | f64 | no | Indicates if the resource is out of service |
| `capacity` | f64 | no | Capacity of the resource |

### `CalendarUpdateDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `notifications` | Vec<CalendarNotification> | no | 🚨 Deprecated! Please use 'Calendar Notifications APIs' instead. |
| `groupId` | String | no | Group Id |
| `teamMembers` | Vec<TeamMember> | no | Team members are required for calendars of type: Round Robin, Collective, Class, Service. Personal calendar must have exactly one team member. |
| `eventType` | String — `RoundRobin_OptimizeForAvailability`, `RoundRobin_OptimizeForEqualDistribution` | no | — |
| `name` | String | no | — |
| `description` | String | no | — |
| `slug` | String | no | — |
| `widgetSlug` | String | no | — |
| `widgetType` | String — `default`, `classic` | no | Calendar widget type. Choose "default" for "neo" and "classic" for "classic" layout. |
| `eventTitle` | String | no | — |
| `eventColor` | String | no | — |
| `locationConfigurations` | Vec<LocationConfiguration> | no | Meeting location configuration for event calendar |
| `meetingLocation` | String | no | 🚨 Deprecated! Use `locationConfigurations.location` or `teamMembers[].locationConfigurations.location` instead. |
| `slotDuration` | f64 | no | This controls the duration of the meeting |
| `slotDurationUnit` | String — `mins`, `hours` | no | Unit for slot duration. |
| `preBufferUnit` | String — `mins`, `hours` | no | Unit for pre-buffer. |
| `slotInterval` | f64 | no | Slot interval reflects the amount of time the between booking slots that will be shown in the calendar. |
| `slotIntervalUnit` | String — `mins`, `hours` | no | Unit for slot interval. |
| `slotBuffer` | f64 | no | Slot-Buffer is additional time that can be added after an appointment, allowing for extra time to wrap up |
| `preBuffer` | f64 | no | Pre-Buffer is additional time that can be added before an appointment, allowing for extra time to get ready |
| `appoinmentPerSlot` | f64 | no | — |
| `appoinmentPerDay` | f64 | no | Number of appointments that can be booked for a given day |
| `allowBookingAfter` | f64 | no | Minimum scheduling notice for events |
| `allowBookingAfterUnit` | String — `hours`, `days`, `weeks`, `months` | no | Unit for minimum scheduling notice |
| `allowBookingFor` | f64 | no | Minimum number of days/weeks/months for which to allow booking events |
| `allowBookingForUnit` | String — `days`, `weeks`, `months` | no | Unit for controlling the duration for which booking would be allowed for |
| `openHours` | Vec<OpenHour> | no | — |
| `enableRecurring` | bool | no | Enable recurring appointments for the calendars. Please note that only one member should be added in the calendar to enable this |
| `recurring` | [`Recurring`](#recurring) | no | — |
| `formId` | String | no | — |
| `stickyContact` | bool | no | — |
| `isLivePaymentMode` | bool | no | — |
| `autoConfirm` | bool | no | — |
| `shouldSendAlertEmailsToAssignedMember` | bool | no | — |
| `alertEmail` | String | no | — |
| `googleInvitationEmails` | bool | no | — |
| `allowReschedule` | bool | no | — |
| `allowCancellation` | bool | no | — |
| `shouldAssignContactToTeamMember` | bool | no | — |
| `shouldSkipAssigningContactForExisting` | bool | no | — |
| `notes` | String | no | — |
| `pixelId` | String | no | — |
| `formSubmitType` | String — `RedirectURL`, `ThankYouMessage` | no | — |
| `formSubmitRedirectURL` | String | no | — |
| `formSubmitThanksMessage` | String | no | — |
| `availabilityType` | String — `0`, `1` | no | Determines which availability type to consider: - **1**: Only custom availabilities will be used. - **0**: Only open hours will be used. - **null**: Both the custom availabilities and open hours will … |
| `availabilities` | Vec<UpdateAvailability> | no | This is only to set the custom availability. For standard availability, use the openHours property |
| `guestType` | String — `count_only`, `collect_detail` | no | — |
| `consentLabel` | String | no | — |
| `calendarCoverImage` | String | no | — |
| `lookBusyConfig` | [`LookBusyConfiguration`](#lookbusyconfiguration) | no | Look Busy Configuration |
| `isActive` | bool | no | — |

### `CalendarsGetSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `calendars` | Vec<CalendarDTO> | no | — |

### `CreateCalendarNotificationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `receiverType` | String — `contact`, `guest`, `assignedUser`, `emails`, `phoneNumbers`, `business` | **yes** | notification recipient type |
| `channel` | String — `email`, `inApp`, `sms`, `whatsapp` | **yes** | Notification channel |
| `notificationType` | String — `booked`, `confirmation`, `cancellation`, `reminder`, `followup`, `reschedule` | **yes** | Notification type |
| `isActive` | bool | no | Is the notification active |
| `templateId` | String | no | Template ID for email notification. Not necessary for in-App notification |
| `body` | String | no | Body for email notification. Not necessary for in-App notification |
| `subject` | String | no | Subject for email notification. Not necessary for in-App notification |
| `afterTime` | Vec<SchedulesDTO> | no | Specifies the time after which the follow-up notification should be sent. This is not required for other notification types. |
| `beforeTime` | Vec<SchedulesDTO> | no | Specifies the time before which the reminder notification should be sent. This is not required for other notification types. |
| `additionalEmailIds` | Vec<String> | no | Additional email addresses to receive notifications. |
| `additionalPhoneNumbers` | Vec<String> | no | Additional phone numbers to receive notifications. |
| `selectedUsers` | Vec<String> | no | Selected users for in-App and business email notifications. Supports user IDs and special keyword "sub_account_admin" |
| `fromAddress` | String | no | from address for email notification |
| `fromName` | String | no | from name for email/sms notification |
| `fromNumber` | String | no | from number for sms notification |

### `CreateCalendarResourceDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | — |
| `name` | String | **yes** | — |
| `description` | String | **yes** | — |
| `quantity` | f64 | **yes** | Quantity of the equipment. |
| `outOfService` | f64 | **yes** | Quantity of the out of service equipment. |
| `capacity` | f64 | **yes** | Capacity of the room. |
| `calendarIds` | Vec<String> | **yes** | Service calendar IDs to be mapped with the resource. One equipment can only be mapped with one service calendar. One room can be mapped with multiple service calendars. |

### `CreateScheduleDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `rules` | Vec<ScheduleRuleDTO> | no | Schedule rules defining when the schedule is active |
| `timezone` | String | **yes** | Timezone for the schedule (IANA timezone identifier) |
| `locationId` | String | **yes** | Location ID where this schedule applies |
| `name` | String | **yes** | Human-readable name for the schedule |
| `userId` | String | **yes** | User ID associated with the schedule |
| `calendarIds` | Vec<String> | no | Calendar IDs associated with the schedule |

### `CreatedOrUpdatedBy`

| Field | Type | Required | Description |
|---|---|---|---|
| `userId` | String | no | The ID of the user who created or updated the appointment |
| `source` | String | **yes** | The source of the appointment |

### `DeleteAppointmentSchema`

_No fields defined in the spec._

### `DeleteEventSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeeded` | bool | no | — |

### `DeleteNoteSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | no | — |

### `GetAllSchedulesResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `schedules` | Vec<ScheduleObjectResponseDTO> | **yes** | Array of schedules |

### `GetCalendarEventSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `event` | [`CalendarEventDTO`](#calendareventdto) | no | — |

### `GetCalendarEventsSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `events` | Vec<CalendarEventDTO> | no | — |

### `GetCreateUpdateNoteSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `note` | [`GetNoteSchema`](#getnoteschema) | no | — |

### `GetNoteSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `body` | String | no | — |
| `userId` | String | no | — |
| `dateAdded` | String | no | — |
| `contactId` | String | no | — |
| `createdBy` | [`NoteCreatedBySchema`](#notecreatedbyschema) | no | — |

### `GetNotesListSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `notes` | Vec<GetNoteSchema> | no | — |
| `hasMore` | bool | no | — |

### `GroupCreateDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | — |
| `name` | String | **yes** | — |
| `description` | String | **yes** | — |
| `slug` | String | **yes** | — |
| `isActive` | bool | no | — |

### `GroupCreateSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `group` | [`GroupDTO`](#groupdto) | no | — |

### `GroupDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | — |
| `name` | String | **yes** | — |
| `description` | String | **yes** | — |
| `slug` | String | **yes** | — |
| `isActive` | bool | no | — |
| `id` | String | no | — |

### `GroupStatusUpdateParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `isActive` | bool | **yes** | Is Active? |

### `GroupSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | no | Success |

### `GroupUpdateDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | — |
| `description` | String | **yes** | — |
| `slug` | String | **yes** | — |

### `Hour`

| Field | Type | Required | Description |
|---|---|---|---|
| `openHour` | f64 | **yes** | — |
| `openMinute` | f64 | **yes** | — |
| `closeHour` | f64 | **yes** | — |
| `closeMinute` | f64 | **yes** | — |

### `LocationConfiguration`

| Field | Type | Required | Description |
|---|---|---|---|
| `kind` | String — `custom`, `zoom_conference`, `google_conference`, `inbound_call`, `outbound_call`, `physical`, `booker`, `ms_teams_conference` | **yes** | Type of meeting location. zoom_conference/google_conference/ms_teams_conference is not supported in event calendar type |
| `location` | String | no | Address for meeting location. Not applicable on "zoom_conference", "google_conference" and "ms_teams_conference" kind |

### `LocationConfigurationResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `kind` | String — `custom`, `zoom_conference`, `google_conference`, `inbound_call`, `outbound_call`, `physical`, `booker`, `ms_teams_conference` | **yes** | Type of meeting location. zoom_conference/google_conference/ms_teams_conference is not supported in event calendar type |
| `location` | String | no | Address for meeting location. Not applicable on "zoom_conference", "google_conference" and "ms_teams_conference" kind |
| `meetingId` | String | no | Unique ID used to select a specific meeting location |

### `LookBusyConfiguration`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | Apply Look Busy |
| `LookBusyPercentage` | f64 | **yes** | Percentage of slots that will be hidden |

### `NoteCreatedBySchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |

### `NotesDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `userId` | String | no | — |
| `body` | String | **yes** | Note body |

### `OpenHour`

| Field | Type | Required | Description |
|---|---|---|---|
| `daysOfTheWeek` | Vec<f64> | **yes** | — |
| `hours` | Vec<Hour> | **yes** | — |

### `Recurring`

| Field | Type | Required | Description |
|---|---|---|---|
| `freq` | String — `DAILY`, `WEEKLY`, `MONTHLY` | no | — |
| `count` | f64 | no | Number of recurrences |
| `bookingOption` | String — `skip`, `continue`, `book_next` | no | This setting contols what to do incase a recurring slot is unavailable |
| `bookingOverlapDefaultStatus` | String — `confirmed`, `new` | no | This setting contols what to do incase a recurring slot is unavailable |

### `ResourceDeleteResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | no | Success |

### `ScheduleIntervalDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `from` | String | **yes** | Start time in HH:MM format (24-hour format) |
| `to` | String | **yes** | End time in HH:MM format (24-hour format) |

### `ScheduleObjectResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the schedule |
| `name` | String | **yes** | Human-readable name for the schedule |
| `locationId` | String | **yes** | Location ID where this schedule applies |
| `rules` | Vec<ScheduleRuleDTO> | **yes** | Schedule rules defining when the schedule is active |
| `timezone` | String | **yes** | Timezone for the schedule (IANA timezone identifier) |
| `dateAdded` | String | **yes** | ISO date string when the schedule was created |
| `dateUpdated` | String | **yes** | ISO date string when the schedule was last updated |
| `userId` | String | **yes** | User ID associated with the schedule |
| `calendarIds` | Vec<String> | no | Calendar IDs associated with the schedule |
| `deleted` | bool | **yes** | Whether the schedule has been deleted |

### `ScheduleResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `schedule` | [`ScheduleObjectResponseDTO`](#scheduleobjectresponsedto) | **yes** | Schedule |

### `ScheduleRuleDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `wday`, `date` | **yes** | Type of schedule rule - weekday (recurring) or date (specific date) |
| `intervals` | Vec<ScheduleIntervalDTO> | **yes** | Time intervals for the rule (e.g., 9 AM to 5 PM) |
| `date` | String | no | Specific date in YYYY-MM-DD format (only for date-type rules) |
| `day` | String — `sunday`, `monday`, `tuesday`, `wednesday`, `thursday`, `friday`, `saturday` | no | Day of week (only for weekday-type rules) |

### `SchedulesDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `timeOffset` | f64 | no | — |
| `unit` | String | no | — |

### `SlotsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `slots` | Vec<String> | **yes** | — |

### `TeamMember`

| Field | Type | Required | Description |
|---|---|---|---|
| `userId` | String | **yes** | — |
| `priority` | String — `0`, `0.5`, `1` | no | — |
| `meetingLocationType` | String — `custom`, `zoom`, `gmeet`, `phone`, `address`, `teams`, `booker` | no | 🚨 Deprecated! Use `locationConfigurations.kind` instead. |
| `meetingLocation` | String | no | 🚨 Deprecated! Use `locationConfigurations.location` instead. |
| `isPrimary` | bool | no | Marks a user as primary. This property is required in case of collective booking calendars. Only one user can be primary. |
| `locationConfigurations` | Vec<LocationConfiguration> | no | Meeting location configuration for event calendar. - *Multiple locations are allowed only when one team member is selected.* - *For **Class booking** and **Collective** calendars, only one location co… |

### `TeamMemberResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `userId` | String | **yes** | — |
| `priority` | String — `0`, `0.5`, `1` | no | — |
| `meetingLocationType` | String — `custom`, `zoom`, `gmeet`, `phone`, `address`, `teams`, `booker` | no | 🚨 Deprecated! Use `locationConfigurations.kind` instead. |
| `meetingLocation` | String | no | 🚨 Deprecated! Use `locationConfigurations.location` instead. |
| `isPrimary` | bool | no | Marks a user as primary. This property is required in case of collective booking calendars. Only one user can be primary. |
| `locationConfigurations` | Vec<LocationConfigurationResponse> | no | Meeting location configurations |

### `UpdateAvailability`

| Field | Type | Required | Description |
|---|---|---|---|
| `date` | String | **yes** | Formulate the date string in the format of `<YYYY-MM-DD in local timezone>T00:00:00.000Z`. |
| `hours` | Vec<Hour> | **yes** | — |
| `deleted` | bool | no | — |
| `id` | String | no | The ID of the custom availability object. It is required while updating or deleting the existing custom date availability |

### `UpdateCalendarNotificationsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `receiverType` | String — `contact`, `guest`, `assignedUser`, `emails`, `phoneNumbers`, `business` | no | Notification recipient type |
| `additionalEmailIds` | Vec<String> | no | Additional email addresses to receive notifications. |
| `additionalPhoneNumbers` | Vec<String> | no | Additional phone numbers to receive notifications. |
| `selectedUsers` | Vec<String> | no | Selected users for in-App and business email notifications. Supports user IDs and special keyword "sub_account_admin" |
| `channel` | String — `email`, `inApp`, `sms`, `whatsapp` | no | Notification channel |
| `notificationType` | String — `booked`, `confirmation`, `cancellation`, `reminder`, `followup`, `reschedule` | no | Notification type |
| `isActive` | bool | no | Is the notification active |
| `deleted` | bool | no | Marks the notification as deleted (soft delete) |
| `templateId` | String | no | Template ID for email notification |
| `body` | String | no | Body for email notification. Not necessary for in-App notification |
| `subject` | String | no | Subject for email notification. Not necessary for in-App notification |
| `afterTime` | Vec<SchedulesDTO> | no | Specifies the time after which the follow-up notification should be sent. This is not required for other notification types. |
| `beforeTime` | Vec<SchedulesDTO> | no | Specifies the time before which the reminder notification should be sent. This is not required for other notification types. |
| `fromAddress` | String | no | From address for email notification |
| `fromNumber` | String | no | from number for sms notification |
| `fromName` | String | no | From name for email/sms notification |

### `UpdateCalendarResourceDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | no | — |
| `name` | String | no | — |
| `description` | String | no | — |
| `quantity` | f64 | no | Quantity of the equipment. |
| `outOfService` | f64 | no | Quantity of the out of service equipment. |
| `capacity` | f64 | no | Capacity of the room. |
| `calendarIds` | Vec<String> | no | Service calendar IDs to be mapped with the resource. One equipment can only be mapped with one service calendar. One room can be mapped with multiple service calendars. |
| `isActive` | bool | no | — |

### `UpdateScheduleDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Human-readable name for the schedule |
| `rules` | Vec<ScheduleRuleDTO> | no | Updated schedule rules defining when the schedule is active |
| `timezone` | String | no | Updated timezone for the schedule (IANA timezone identifier) |

### `ValidateGroupSlugPostBody`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location Id |
| `slug` | String | **yes** | Slug |

### `ValidateGroupSlugSuccessResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `available` | bool | **yes** | — |

## Data models — API v3

In Rust: `ghl_models::v3::calendars::*` (enable the `calendars` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/calendars/).

### `AllGroupsSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `groups` | Vec<GroupDTO> | no | — |

### `AppointmentCreateSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | no | Title |
| `meetingLocationType` | String — `custom`, `zoom`, `gmeet`, `phone`, `address`, `ms_teams`, `google` | no | Meeting location type. - If `address` is provided in the request body, the `meetingLocationType` defaults to **custom**. |
| `meetingLocationId` | String | no | The unique identifier for the meeting location. - This value can be found in `calendar.locationConfigurations`or `calendar.teamMembers[].locationConfigurations` |
| `overrideLocationConfig` | bool | no | Flag to override location config - **false** - If only `meetingLocationId` is provided - **true** - If only `meetingLocationType` is provided |
| `appointmentStatus` | String — `new`, `confirmed`, `cancelled`, `showed`, `noshow`, `invalid`, `completed`, `active` | no | — |
| `assignedUserId` | String | no | Assigned User Id |
| `description` | String | no | Appointment Description |
| `address` | String | no | Appointment Address |
| `ignoreDateRange` | bool | no | If set to true, the minimum scheduling notice and date range would be ignored |
| `toNotify` | bool | no | If set to false, the automations will not run. Defaults to true |
| `ignoreFreeSlotValidation` | bool | no | If true the time slot validation would be avoided for any appointment creation (even the ignoreDateRange) |
| `rrule` | String | no | RRULE as per the iCalendar (RFC 5545) specification for recurring events. DTSTART is not required, instance ids are calculated on the basis of startTime of the event. The rrule only be applied if igno… |
| `calendarId` | String | **yes** | Calendar Id |
| `locationId` | String | **yes** | Location Id |
| `contactId` | String | **yes** | Contact Id |
| `startTime` | String | **yes** | Start Time |
| `endTime` | String | no | End Time |

### `AppointmentEditSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | no | Title |
| `meetingLocationType` | String — `custom`, `zoom`, `gmeet`, `phone`, `address`, `ms_teams`, `google` | no | Meeting location type. - If `address` is provided in the request body, the `meetingLocationType` defaults to **custom**. |
| `meetingLocationId` | String | no | The unique identifier for the meeting location. - This value can be found in `calendar.locationConfigurations`or `calendar.teamMembers[].locationConfigurations` |
| `overrideLocationConfig` | bool | no | Flag to override location config - **false** - If only `meetingLocationId` is provided - **true** - If only `meetingLocationType` is provided |
| `appointmentStatus` | String — `new`, `confirmed`, `cancelled`, `showed`, `noshow`, `invalid`, `completed`, `active` | no | — |
| `assignedUserId` | String | no | Assigned User Id |
| `description` | String | no | Appointment Description |
| `address` | String | no | Appointment Address |
| `ignoreDateRange` | bool | no | If set to true, the minimum scheduling notice and date range would be ignored |
| `toNotify` | bool | no | If set to false, the automations will not run. Defaults to true |
| `ignoreFreeSlotValidation` | bool | no | If true the time slot validation would be avoided for any appointment creation (even the ignoreDateRange) |
| `rrule` | String | no | RRULE as per the iCalendar (RFC 5545) specification for recurring events. DTSTART is not required, instance ids are calculated on the basis of startTime of the event. The rrule only be applied if igno… |
| `calendarId` | String | no | Calendar Id |
| `startTime` | String | no | Start Time |
| `endTime` | String | no | End Time |

### `AppointmentSchemaResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `calendarId` | String | **yes** | Calendar Id |
| `locationId` | String | **yes** | Location Id |
| `contactId` | String | **yes** | Contact Id |
| `startTime` | String | no | Start Time |
| `endTime` | String | no | End Time |
| `title` | String | no | Title |
| `meetingLocationType` | String | no | Meeting Location Type |
| `appointmentStatus` | String — `new`, `confirmed`, `cancelled`, `showed`, `noshow`, `invalid`, `active`, `completed` | no | — |
| `assignedUserId` | String | no | Assigned User Id |
| `address` | String | no | Appointment Address |
| `isRecurring` | bool | no | true if the event is recurring otherwise false |
| `rrule` | String | no | RRULE as per the iCalendar (RFC 5545) specification for recurring events |
| `dateAdded` | String | **yes** | Date Added |
| `dateUpdated` | String | **yes** | Date Updated |
| `id` | String | **yes** | Id |

### `Availability`

| Field | Type | Required | Description |
|---|---|---|---|
| `date` | String | **yes** | Formulate the date string in the format of `<YYYY-MM-DD in local timezone>T00:00:00.000Z`. |
| `hours` | Vec<Hour> | **yes** | — |
| `deleted` | bool | no | — |

### `BlockSlotCreateRequestDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | no | Title |
| `calendarId` | String | **yes** | Either calendarId or assignedUserId can be set, not both. |
| `assignedUserId` | String | no | Either calendarId or assignedUserId can be set, not both. |
| `locationId` | String | **yes** | Location Id |
| `startTime` | String | no | Start Time |
| `endTime` | String | no | End Time |

### `BlockSlotEditRequestDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | no | Title |
| `calendarId` | String | **yes** | Either calendarId or assignedUserId can be set, not both. |
| `assignedUserId` | String | no | Either calendarId or assignedUserId can be set, not both. |
| `locationId` | String | **yes** | Location Id |
| `startTime` | String | no | Start Time |
| `endTime` | String | no | End Time |

### `BlockedSlotSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Id |
| `locationId` | String | **yes** | Location Id |
| `title` | String | **yes** | Title |
| `startTime` | JSON | **yes** | Start Time |
| `endTime` | JSON | **yes** | End Time |
| `calendarId` | String | no | Calendar id |
| `assignedUserId` | String | no | Assigned User Id |

### `CalendarByIdSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `calendar` | [`CalendarDTO`](#calendardto) | **yes** | — |

### `CalendarCreateDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `isActive` | bool | no | Should the created calendar be active or draft |
| `notifications` | Vec<CalendarNotification> | no | 🚨 Deprecated! Please use 'Calendar Notifications APIs' instead. |
| `locationId` | String | **yes** | — |
| `groupId` | String | no | Group Id |
| `teamMembers` | Vec<TeamMember> | no | Team members are required for calendars of type: Round Robin, Collective, Class, Service. Personal calendar must have exactly one team member. |
| `eventType` | String — `RoundRobin_OptimizeForAvailability`, `RoundRobin_OptimizeForEqualDistribution` | no | — |
| `name` | String | **yes** | — |
| `description` | String | no | — |
| `slug` | String | no | — |
| `widgetSlug` | String | no | — |
| `calendarType` | String — `round_robin`, `event`, `class_booking`, `collective`, `service_booking`, `personal` | no | — |
| `widgetType` | String — `default`, `classic` | no | Calendar widget type. Choose "default" for "neo" and "classic" for "classic" layout. |
| `eventTitle` | String | no | — |
| `eventColor` | String | no | — |
| `meetingLocation` | String | no | 🚨 Deprecated! Use `locationConfigurations.location` or `teamMembers[].locationConfigurations.location` instead. |
| `locationConfigurations` | Vec<LocationConfiguration> | no | Meeting location configuration for event calendar |
| `slotDuration` | f64 | no | This controls the duration of the meeting |
| `slotDurationUnit` | String — `mins`, `hours` | no | Unit for slot duration. |
| `slotInterval` | f64 | no | Slot interval reflects the amount of time the between booking slots that will be shown in the calendar. |
| `slotIntervalUnit` | String — `mins`, `hours` | no | Unit for slot interval. |
| `slotBuffer` | f64 | no | Slot-Buffer is additional time that can be added after an appointment, allowing for extra time to wrap up |
| `slotBufferUnit` | String — `mins`, `hours` | no | Unit for slot buffer. |
| `preBuffer` | f64 | no | Pre-Buffer is additional time that can be added before an appointment, allowing for extra time to get ready |
| `preBufferUnit` | String — `mins`, `hours` | no | Unit for pre-buffer. |
| `appoinmentPerSlot` | f64 | no | Maximum bookings per slot (per user). Maximum seats per slot in case of Class Booking Calendar. |
| `appoinmentPerDay` | f64 | no | Number of appointments that can be booked for a given day |
| `allowBookingAfter` | f64 | no | Minimum scheduling notice for events |
| `allowBookingAfterUnit` | String — `hours`, `days`, `weeks`, `months`, `mins` | no | Unit for minimum scheduling notice |
| `allowBookingFor` | f64 | no | Minimum number of days/weeks/months for which to allow booking events |
| `allowBookingForUnit` | String — `days`, `weeks`, `months` | no | Unit for controlling the duration for which booking would be allowed for |
| `openHours` | Vec<OpenHour> | no | While we will support this property for backward compatibility, it is recommended to use 'Availability' APIs instead. |
| `enableRecurring` | bool | no | Enable recurring appointments for the calendars. Please note that only one member should be added in the calendar to enable this |
| `recurring` | [`Recurring`](#recurring) | no | — |
| `formId` | String | no | — |
| `stickyContact` | bool | no | — |
| `isLivePaymentMode` | bool | no | — |
| `autoConfirm` | bool | no | — |
| `shouldSendAlertEmailsToAssignedMember` | bool | no | — |
| `alertEmail` | String | no | — |
| `googleInvitationEmails` | bool | no | — |
| `allowReschedule` | bool | no | — |
| `allowCancellation` | bool | no | — |
| `shouldAssignContactToTeamMember` | bool | no | — |
| `shouldSkipAssigningContactForExisting` | bool | no | — |
| `notes` | String | no | — |
| `pixelId` | String | no | — |
| `formSubmitType` | String — `RedirectURL`, `ThankYouMessage` | no | — |
| `formSubmitRedirectURL` | String | no | — |
| `formSubmitThanksMessage` | String | no | — |
| `availabilityType` | String — `0`, `1` | no | While we will support this property for backward compatibility, it is not required anymore. |
| `availabilities` | Vec<Availability> | no | While we will support this property for backward compatibility, it is recommended to use 'Availability' APIs instead. |
| `guestType` | String — `count_only`, `collect_detail` | no | — |
| `consentLabel` | String | no | — |
| `calendarCoverImage` | String | no | — |
| `lookBusyConfig` | [`LookBusyConfiguration`](#lookbusyconfiguration) | no | Look Busy Configuration |

### `CalendarDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `isActive` | bool | no | Should the created calendar be active or draft |
| `notifications` | Vec<CalendarNotification> | no | 🚨 Deprecated! Please use 'Calendar Notifications APIs' instead. |
| `locationId` | String | **yes** | — |
| `groupId` | String | no | Group Id |
| `teamMembers` | Vec<TeamMemberResponse> | no | Team members are for calendars of type: Round Robin, Collective, Class, Service. Personal calendar must have exactly one team member. |
| `eventType` | String — `RoundRobin_OptimizeForAvailability`, `RoundRobin_OptimizeForEqualDistribution` | no | — |
| `name` | String | **yes** | — |
| `description` | String | no | — |
| `slug` | String | no | — |
| `widgetSlug` | String | no | — |
| `calendarType` | String — `round_robin`, `event`, `class_booking`, `collective`, `service_booking`, `personal` | no | — |
| `widgetType` | String — `default`, `classic` | no | Calendar widget type. Choose "default" for "neo" and "classic" for "classic" layout. |
| `eventTitle` | String | no | — |
| `eventColor` | String | no | — |
| `meetingLocation` | String | no | 🚨 Deprecated! Use `locationConfigurations.location` or `teamMembers[].locationConfigurations.location` instead. |
| `locationConfigurations` | Vec<LocationConfigurationResponse> | no | Meeting location configuration for event calendar |
| `slotDuration` | f64 | no | This controls the duration of the meeting |
| `slotDurationUnit` | String — `mins`, `hours` | no | Unit for slot duration. |
| `slotInterval` | f64 | no | Slot interval reflects the amount of time the between booking slots that will be shown in the calendar. |
| `slotIntervalUnit` | String — `mins`, `hours` | no | Unit for slot interval. |
| `slotBuffer` | f64 | no | Slot-Buffer is additional time that can be added after an appointment, allowing for extra time to wrap up |
| `slotBufferUnit` | String — `mins`, `hours` | no | Unit for slot buffer. |
| `preBuffer` | f64 | no | Pre-Buffer is additional time that can be added before an appointment, allowing for extra time to get ready |
| `preBufferUnit` | String — `mins`, `hours` | no | Unit for pre-buffer. |
| `appoinmentPerSlot` | f64 | no | Maximum bookings per slot (per user). Maximum seats per slot in case of Class Booking Calendar. |
| `appoinmentPerDay` | f64 | no | Number of appointments that can be booked for a given day |
| `allowBookingAfter` | f64 | no | Minimum scheduling notice for events |
| `allowBookingAfterUnit` | String — `hours`, `days`, `weeks`, `months`, `mins` | no | Unit for minimum scheduling notice |
| `allowBookingFor` | f64 | no | Minimum number of days/weeks/months for which to allow booking events |
| `allowBookingForUnit` | String — `days`, `weeks`, `months` | no | Unit for controlling the duration for which booking would be allowed for |
| `openHours` | Vec<OpenHour> | no | While we will support this property for backward compatibility, it is recommended to use 'Availability' APIs instead. |
| `enableRecurring` | bool | no | Enable recurring appointments for the calendars. Please note that only one member should be added in the calendar to enable this |
| `recurring` | [`Recurring`](#recurring) | no | — |
| `formId` | String | no | — |
| `stickyContact` | bool | no | — |
| `isLivePaymentMode` | bool | no | — |
| `autoConfirm` | bool | no | — |
| `shouldSendAlertEmailsToAssignedMember` | bool | no | — |
| `alertEmail` | String | no | — |
| `googleInvitationEmails` | bool | no | — |
| `allowReschedule` | bool | no | — |
| `allowCancellation` | bool | no | — |
| `shouldAssignContactToTeamMember` | bool | no | — |
| `shouldSkipAssigningContactForExisting` | bool | no | — |
| `notes` | String | no | — |
| `pixelId` | String | no | — |
| `formSubmitType` | String — `RedirectURL`, `ThankYouMessage` | no | — |
| `formSubmitRedirectURL` | String | no | — |
| `formSubmitThanksMessage` | String | no | — |
| `availabilityType` | String — `0`, `1` | no | While we will support this property for backward compatibility, it is not required anymore. |
| `availabilities` | Vec<Availability> | no | While we will support this property for backward compatibility, it is recommended to use 'Availability' APIs instead. |
| `guestType` | String — `count_only`, `collect_detail` | no | — |
| `consentLabel` | String | no | — |
| `calendarCoverImage` | String | no | — |
| `lookBusyConfig` | [`LookBusyConfiguration`](#lookbusyconfiguration) | no | Look Busy Configuration |
| `id` | String | **yes** | — |

### `CalendarDeleteSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success |

### `CalendarEventDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Event Id or Instance id for a recurring event |
| `address` | String | no | Calendar Event address |
| `title` | String | **yes** | Calendar Event title |
| `calendarId` | String | **yes** | Calendar ID |
| `locationId` | String | **yes** | Location ID |
| `contactId` | String | **yes** | Contact ID |
| `groupId` | String | **yes** | Group ID |
| `appointmentStatus` | String | **yes** | Appointment Status |
| `assignedUserId` | String | **yes** | AssignedUser - the primary owner of an appointment |
| `users` | Vec<String> | **yes** | Users - the secondary owners of an appointment. |
| `notes` | String | no | Notes |
| `description` | String | no | Description |
| `isRecurring` | bool | no | true if the event is recurring otherwise false |
| `rrule` | String | no | RRULE as per the iCalendar (RFC 5545) specification for recurring events. DTSTART is not required, instance ids are calculated on the basis of startTime of the event. |
| `deleted` | bool | no | Tells if a calendar event has been deleted |
| `startTime` | JSON | **yes** | Start Time |
| `endTime` | JSON | **yes** | End Time |
| `dateAdded` | JSON | **yes** | Date Added |
| `dateUpdated` | JSON | **yes** | Date Updated |
| `assignedResources` | Vec<String> | no | Ids of associated resources rooms and/or equipments |
| `createdBy` | [`CreatedOrUpdatedBy`](#createdorupdatedby) | no | Appointment booked by metadata |
| `masterEventId` | String | no | Master event id for a recurring instance |

### `CalendarNotification`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `email` | no | Calendar Notification |
| `shouldSendToContact` | bool | **yes** | — |
| `shouldSendToGuest` | bool | **yes** | — |
| `shouldSendToUser` | bool | **yes** | — |
| `shouldSendToSelectedUsers` | bool | **yes** | — |
| `selectedUsers` | String | **yes** | Comma separated emails |

### `CalendarNotificationDeleteResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | **yes** | Result of delete/update operation |

### `CalendarNotificationResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | Notification ID |
| `receiverType` | String — `contact`, `guest`, `assignedUser`, `emails`, `phoneNumbers`, `business` | no | — |
| `additionalEmailIds` | Vec<String> | no | — |
| `additionalPhoneNumbers` | Vec<String> | no | — |
| `channel` | String — `email`, `inApp`, `sms`, `whatsapp` | no | — |
| `notificationType` | String — `booked`, `confirmation`, `cancellation`, `reminder`, `followup`, `reschedule` | no | — |
| `isActive` | bool | no | — |
| `additionalWhatsappNumbers` | Vec<String> | no | — |
| `templateId` | String | no | — |
| `body` | String | no | — |
| `subject` | String | no | — |
| `afterTime` | Vec<SchedulesDTO> | no | — |
| `beforeTime` | Vec<SchedulesDTO> | no | — |
| `selectedUsers` | Vec<String> | no | — |
| `deleted` | bool | no | — |

### `CalendarResourceByIdResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID of the resource |
| `name` | String | **yes** | Name of the resource |
| `resourceType` | String — `equipments`, `rooms` | **yes** | — |
| `isActive` | bool | **yes** | Whether the resource is active |
| `description` | String | no | Description of the resource |
| `quantity` | f64 | no | Quantity of the resource |
| `outOfService` | f64 | no | Indicates if the resource is out of service |
| `capacity` | f64 | no | Capacity of the resource |
| `calendarIds` | Vec<String> | **yes** | Calendar IDs |

### `CalendarResourceResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID of the resource |
| `name` | String | **yes** | Name of the resource |
| `resourceType` | String — `equipments`, `rooms` | **yes** | — |
| `isActive` | bool | **yes** | Whether the resource is active |
| `description` | String | no | Description of the resource |
| `quantity` | f64 | no | Quantity of the resource |
| `outOfService` | f64 | no | Indicates if the resource is out of service |
| `capacity` | f64 | no | Capacity of the resource |

### `CalendarUpdateDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `notifications` | Vec<CalendarNotification> | no | 🚨 Deprecated! Please use 'Calendar Notifications APIs' instead. |
| `groupId` | String | no | Group Id |
| `teamMembers` | Vec<TeamMember> | no | Team members are required for calendars of type: Round Robin, Collective, Class, Service. Personal calendar must have exactly one team member. |
| `eventType` | String — `RoundRobin_OptimizeForAvailability`, `RoundRobin_OptimizeForEqualDistribution` | no | — |
| `name` | String | no | — |
| `description` | String | no | — |
| `slug` | String | no | — |
| `widgetSlug` | String | no | — |
| `widgetType` | String — `default`, `classic` | no | Calendar widget type. Choose "default" for "neo" and "classic" for "classic" layout. |
| `eventTitle` | String | no | — |
| `eventColor` | String | no | — |
| `locationConfigurations` | Vec<LocationConfiguration> | no | Meeting location configuration for event calendar |
| `meetingLocation` | String | no | 🚨 Deprecated! Use `locationConfigurations.location` or `teamMembers[].locationConfigurations.location` instead. |
| `slotDuration` | f64 | no | This controls the duration of the meeting |
| `slotDurationUnit` | String — `mins`, `hours` | no | Unit for slot duration. |
| `preBufferUnit` | String — `mins`, `hours` | no | Unit for pre-buffer. |
| `slotInterval` | f64 | no | Slot interval reflects the amount of time the between booking slots that will be shown in the calendar. |
| `slotIntervalUnit` | String — `mins`, `hours` | no | Unit for slot interval. |
| `slotBuffer` | f64 | no | Slot-Buffer is additional time that can be added after an appointment, allowing for extra time to wrap up |
| `preBuffer` | f64 | no | Pre-Buffer is additional time that can be added before an appointment, allowing for extra time to get ready |
| `appoinmentPerSlot` | f64 | no | — |
| `appoinmentPerDay` | f64 | no | Number of appointments that can be booked for a given day |
| `allowBookingAfter` | f64 | no | Minimum scheduling notice for events |
| `allowBookingAfterUnit` | String — `hours`, `days`, `weeks`, `months`, `mins` | no | Unit for minimum scheduling notice |
| `allowBookingFor` | f64 | no | Minimum number of days/weeks/months for which to allow booking events |
| `allowBookingForUnit` | String — `days`, `weeks`, `months` | no | Unit for controlling the duration for which booking would be allowed for |
| `openHours` | Vec<OpenHour> | no | While we will support this property for backward compatibility, it is recommended to use 'Availability' APIs instead. |
| `enableRecurring` | bool | no | Enable recurring appointments for the calendars. Please note that only one member should be added in the calendar to enable this |
| `recurring` | [`Recurring`](#recurring) | no | — |
| `formId` | String | no | — |
| `stickyContact` | bool | no | — |
| `isLivePaymentMode` | bool | no | — |
| `autoConfirm` | bool | no | — |
| `shouldSendAlertEmailsToAssignedMember` | bool | no | — |
| `alertEmail` | String | no | — |
| `googleInvitationEmails` | bool | no | — |
| `allowReschedule` | bool | no | — |
| `allowCancellation` | bool | no | — |
| `shouldAssignContactToTeamMember` | bool | no | — |
| `shouldSkipAssigningContactForExisting` | bool | no | — |
| `notes` | String | no | — |
| `pixelId` | String | no | — |
| `formSubmitType` | String — `RedirectURL`, `ThankYouMessage` | no | — |
| `formSubmitRedirectURL` | String | no | — |
| `formSubmitThanksMessage` | String | no | — |
| `availabilityType` | String — `0`, `1` | no | While we will support this property for backward compatibility, it is not required anymore. |
| `availabilities` | Vec<UpdateAvailability> | no | While we will support this property for backward compatibility, it is recommended to use 'Availability' APIs instead. |
| `guestType` | String — `count_only`, `collect_detail` | no | — |
| `consentLabel` | String | no | — |
| `calendarCoverImage` | String | no | — |
| `lookBusyConfig` | [`LookBusyConfiguration`](#lookbusyconfiguration) | no | Look Busy Configuration |
| `isActive` | bool | no | — |

### `CalendarsGetSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `calendars` | Vec<CalendarDTO> | no | — |

### `CreateBookingServiceDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Service ID |
| `staffId` | String | no | Staff ID |
| `position` | f64 | no | Position |
| `addOns` | Vec<ServiceAddOnDTO> | no | Add-ons |

### `CreateCalendarNotificationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `receiverType` | String — `contact`, `guest`, `assignedUser`, `emails`, `phoneNumbers`, `business` | **yes** | notification recipient type |
| `channel` | String — `email`, `inApp`, `sms`, `whatsapp` | **yes** | Notification channel |
| `notificationType` | String — `booked`, `confirmation`, `cancellation`, `reminder`, `followup`, `reschedule` | **yes** | Notification type |
| `isActive` | bool | no | Is the notification active |
| `templateId` | String | no | Template ID for email notification. Not necessary for in-App notification |
| `body` | String | no | Body for email notification. Not necessary for in-App notification |
| `subject` | String | no | Subject for email notification. Not necessary for in-App notification |
| `afterTime` | Vec<SchedulesDTO> | no | Specifies the time after which the follow-up notification should be sent. This is not required for other notification types. |
| `beforeTime` | Vec<SchedulesDTO> | no | Specifies the time before which the reminder notification should be sent. This is not required for other notification types. |
| `additionalEmailIds` | Vec<String> | no | Additional email addresses to receive notifications. |
| `additionalPhoneNumbers` | Vec<String> | no | Additional phone numbers to receive notifications. |
| `selectedUsers` | Vec<String> | no | Selected users for in-App and business email notifications. Supports user IDs and special keyword "sub_account_admin" |
| `fromAddress` | String | no | from address for email notification |
| `fromName` | String | no | from name for email/sms notification |
| `fromNumber` | String | no | from number for sms notification |

### `CreateCalendarResourceDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | — |
| `name` | String | **yes** | — |
| `description` | String | **yes** | — |
| `quantity` | f64 | **yes** | Quantity of the equipment. |
| `outOfService` | f64 | **yes** | Quantity of the out of service equipment. |
| `capacity` | f64 | **yes** | Capacity of the room. |
| `calendarIds` | Vec<String> | **yes** | Service calendar IDs to be mapped with the resource. One equipment can only be mapped with one service calendar. One room can be mapped with multiple service calendars. |

### `CreateEventCalendarScheduleDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `rules` | Vec<ScheduleRuleDTO> | **yes** | Schedule rules defining when the schedule is active |
| `timezone` | String | **yes** | Timezone for the schedule (IANA timezone identifier) |

### `CreateOrUpdateServiceBookingResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `bookingId` | String | **yes** | Booking ID |
| `locationId` | String | **yes** | Location ID |
| `contactId` | String | **yes** | Contact ID |
| `serviceLocationId` | String | **yes** | Service Location ID |
| `title` | String | **yes** | Service Booking Title |
| `startTime` | String | **yes** | Start Time |
| `endTime` | String | **yes** | End Time |
| `services` | Vec<ServiceDTO> | **yes** | Services |
| `timezone` | String | **yes** | Timezone |
| `status` | String | **yes** | Status |
| `deleted` | bool | **yes** | Tells if the booking is deleted |
| `dateAdded` | String | **yes** | Date Added |
| `dateUpdated` | String | **yes** | Date Updated |
| `createdBy` | [`CreatedOrUpdatedBy`](#createdorupdatedby) | **yes** | Booking booked by metadata |
| `meetingLocation` | String | no | Meeting Location (If service location is an ask the booker, then the meeting location is used for the booking) |
| `messages` | Vec<Vec<JSON>> | no | Optional informative or warning messages (e.g. meeting location ignored for non-ask-booker locations) |

### `CreatePublicServiceBookingDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `contactId` | String | **yes** | Contact ID |
| `startTime` | String | **yes** | Start Time |
| `endTime` | String | **yes** | End Time |
| `timezone` | String | **yes** | Timezone |
| `services` | Vec<CreateBookingServiceDTO> | **yes** | Services |
| `serviceLocationId` | String | no | Service Location ID (If not provided, then the default service location will be used) |
| `meetingLocation` | String | no | Meeting Location (If service location is an ask the booker, then the meeting location is required) |
| `title` | String | no | Service Booking Title |
| `status` | String — `confirmed`, `new` | no | Status. (If not provided, the status configured in Service Global Settings will be used.) |

### `CreateScheduleDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `rules` | Vec<ScheduleRuleDTO> | no | Schedule rules defining when the schedule is active |
| `timezone` | String | **yes** | Timezone for the schedule (IANA timezone identifier) |
| `locationId` | String | **yes** | Location ID where this schedule applies |
| `name` | String | **yes** | Human-readable name for the schedule |
| `userId` | String | **yes** | User ID associated with the schedule |
| `calendarIds` | Vec<String> | no | Calendar IDs associated with the schedule |

### `CreateServiceDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `name` | String | **yes** | Service name |
| `slug` | String | **yes** | Unique URL-friendly identifier |
| `staff` | Vec<StaffDTO> | **yes** | Assigned staff members (at least one required) |
| `description` | String | no | Service description |
| `eventColor` | String | no | Service event color (hex) |
| `coverImage` | String | no | Service cover image URL |
| `serviceCategoryId` | String | no | Service category ID (uses default category if not provided) |
| `payment` | [`PaymentDTO`](#paymentdto) | no | Payment details (default amount is 0, currency configured in Service Global Settings is used.) |
| `serviceDuration` | f64 | no | This controls the duration of the appointment |
| `serviceDurationUnit` | String — `mins`, `hours` | no | Duration unit |
| `preBuffer` | f64 | no | Pre-Buffer is additional time that can be added before an appointment, allowing for extra time to get ready |
| `preBufferUnit` | String — `mins`, `hours` | no | Pre-buffer unit |
| `postBuffer` | f64 | no | Post-buffer: Additional time that can be added after an appointment, allowing for extra time to wrap up |
| `postBufferUnit` | String — `mins`, `hours` | no | Post-buffer unit |
| `isPrivate` | bool | no | Whether service is private (not shown publicly) |
| `formId` | String | no | Custom form ID (will be used to display the custom form on the booking page, if only one service is selected) |
| `variations` | Vec<CreateServiceVariationDTO> | no | Service variations (pass empty array for no variations) |

### `CreateServiceLocationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `name` | String | **yes** | Location name |
| `slug` | String | **yes** | URL-friendly slug identifier |
| `phone` | String | no | Phone number |
| `address` | String | no | Use a full street address when locationType is offline. Use a user-facing label when locationType is ask_booker. |
| `coverImage` | String | no | URL of the cover image for this service location |
| `locationType` | String — `offline`, `ask_booker` | no | Location type |

### `CreateServiceVariationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `serviceDuration` | f64 | no | This controls the duration of the appointment |
| `serviceDurationUnit` | String — `mins`, `hours` | no | Duration unit |
| `preBuffer` | f64 | no | Pre-Buffer is additional time that can be added before an appointment, allowing for extra time to get ready |
| `preBufferUnit` | String — `mins`, `hours` | no | Pre-buffer unit |
| `postBuffer` | f64 | no | Post-buffer: Additional time that can be added after an appointment, allowing for extra time to wrap up |
| `postBufferUnit` | String — `mins`, `hours` | no | Post-buffer unit |
| `payment` | [`PaymentDTO`](#paymentdto) | no | Payment details |
| `name` | String | **yes** | Variation name |

### `CreatedOrUpdatedBy`

| Field | Type | Required | Description |
|---|---|---|---|
| `userId` | String | no | The ID of the user who created or updated the appointment |
| `source` | String | **yes** | The source of the appointment |

### `DeleteAppointmentSchema`

_No fields defined in the spec._

### `DeleteEventSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeeded` | bool | no | — |

### `DeleteNoteSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | no | — |

### `DeleteServiceBookingResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Indicates if the deletion was successful |
| `message` | String | **yes** | Response message |

### `DeleteServiceLocationResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success |
| `message` | String | no | Success message |

### `DeleteServiceResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success |
| `message` | String | no | Success message |

### `EventCalendarScheduleResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `timezone` | String | **yes** | Timezone for the schedule (IANA timezone identifier) |
| `rules` | Vec<ScheduleRuleDTO> | **yes** | Schedule rules defining when the schedule is active |
| `calendarId` | String | **yes** | Calendar ID associated with the schedule |
| `dateAdded` | String | no | Information about who created the schedule |
| `dateUpdated` | String | no | Information about who last updated the schedule |

### `EventCalendarScheduleWrapperDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `schedule` | [`EventCalendarScheduleResponseDTO`](#eventcalendarscheduleresponsedto) | **yes** | The event calendar schedule |

### `GetAllSchedulesResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `schedules` | Vec<ScheduleObjectResponseDTO> | **yes** | Array of schedules |

### `GetCalendarEventSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `event` | [`CalendarEventDTO`](#calendareventdto) | no | — |

### `GetCalendarEventsSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `events` | Vec<CalendarEventDTO> | no | — |

### `GetCreateUpdateNoteSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `note` | [`GetNoteSchema`](#getnoteschema) | no | — |

### `GetNoteSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `body` | String | no | — |
| `userId` | String | no | — |
| `dateAdded` | String | no | — |
| `contactId` | String | no | — |
| `createdBy` | [`NoteCreatedBySchema`](#notecreatedbyschema) | no | — |

### `GetNotesListSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `notes` | Vec<GetNoteSchema> | no | — |
| `hasMore` | bool | no | — |

### `GroupCreateDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | — |
| `name` | String | **yes** | — |
| `description` | String | **yes** | — |
| `slug` | String | **yes** | — |
| `isActive` | bool | no | — |

### `GroupCreateSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `group` | [`GroupDTO`](#groupdto) | no | — |

### `GroupDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | — |
| `name` | String | **yes** | — |
| `description` | String | **yes** | — |
| `slug` | String | **yes** | — |
| `isActive` | bool | no | — |
| `id` | String | no | — |

### `GroupStatusUpdateParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `isActive` | bool | **yes** | Is Active? |

### `GroupSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | no | Success |

### `GroupUpdateDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | — |
| `description` | String | **yes** | — |
| `slug` | String | **yes** | — |

### `Hour`

| Field | Type | Required | Description |
|---|---|---|---|
| `openHour` | f64 | **yes** | — |
| `openMinute` | f64 | **yes** | — |
| `closeHour` | f64 | **yes** | — |
| `closeMinute` | f64 | **yes** | — |

### `LocationConfiguration`

| Field | Type | Required | Description |
|---|---|---|---|
| `kind` | String — `custom`, `zoom_conference`, `google_conference`, `inbound_call`, `outbound_call`, `physical`, `booker`, `ms_teams_conference` | **yes** | Type of meeting location. zoom_conference/google_conference/ms_teams_conference is not supported in event calendar type |
| `location` | String | no | Address for meeting location. Not applicable on "zoom_conference", "google_conference" and "ms_teams_conference" kind |

### `LocationConfigurationResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `kind` | String — `custom`, `zoom_conference`, `google_conference`, `inbound_call`, `outbound_call`, `physical`, `booker`, `ms_teams_conference` | **yes** | Type of meeting location. zoom_conference/google_conference/ms_teams_conference is not supported in event calendar type |
| `location` | String | no | Address for meeting location. Not applicable on "zoom_conference", "google_conference" and "ms_teams_conference" kind |
| `meetingId` | String | no | Unique ID used to select a specific meeting location |

### `LookBusyConfiguration`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | Apply Look Busy |
| `LookBusyPercentage` | f64 | **yes** | Percentage of slots that will be hidden |

### `NoteCreatedBySchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |

### `NotesDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `userId` | String | no | — |
| `body` | String | **yes** | Note body |

### `OpenHour`

| Field | Type | Required | Description |
|---|---|---|---|
| `daysOfTheWeek` | Vec<f64> | **yes** | — |
| `hours` | Vec<Hour> | **yes** | — |

### `PaymentDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `amount` | f64 | no | Service price |
| `deposit` | f64 | no | Deposit amount or percentage value |
| `depositType` | String — `percentage`, `amount` | no | Deposit type |

### `Recurring`

| Field | Type | Required | Description |
|---|---|---|---|
| `freq` | String — `DAILY`, `WEEKLY`, `MONTHLY` | no | — |
| `count` | f64 | no | Number of recurrences |
| `bookingOption` | String — `skip`, `continue`, `book_next` | no | This setting contols what to do incase a recurring slot is unavailable |
| `bookingOverlapDefaultStatus` | String — `confirmed`, `new` | no | This setting contols what to do incase a recurring slot is unavailable |

### `ResourceDeleteResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | no | Success |

### `ScheduleIntervalDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `from` | String | **yes** | Start time in HH:MM format (24-hour format) |
| `to` | String | **yes** | End time in HH:MM format (24-hour format) |

### `ScheduleObjectResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the schedule |
| `name` | String | **yes** | Human-readable name for the schedule |
| `locationId` | String | **yes** | Location ID where this schedule applies |
| `rules` | Vec<ScheduleRuleDTO> | **yes** | Schedule rules defining when the schedule is active |
| `timezone` | String | **yes** | Timezone for the schedule (IANA timezone identifier) |
| `dateAdded` | String | **yes** | ISO date string when the schedule was created |
| `dateUpdated` | String | **yes** | ISO date string when the schedule was last updated |
| `userId` | String | **yes** | User ID associated with the schedule |
| `calendarIds` | Vec<String> | no | Calendar IDs associated with the schedule |
| `deleted` | bool | **yes** | Whether the schedule has been deleted |

### `ScheduleResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `schedule` | [`ScheduleObjectResponseDTO`](#scheduleobjectresponsedto) | **yes** | Schedule |

### `ScheduleRuleDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `wday`, `date` | **yes** | Type of schedule rule - weekday (recurring) or date (specific date) |
| `intervals` | Vec<ScheduleIntervalDTO> | **yes** | Time intervals for the rule (e.g., 9 AM to 5 PM) |
| `date` | String | no | Specific date in YYYY-MM-DD format (only for date-type rules) |
| `day` | String — `sunday`, `monday`, `tuesday`, `wednesday`, `thursday`, `friday`, `saturday` | no | Day of week (only for weekday-type rules) |

### `SchedulesDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `timeOffset` | f64 | no | — |
| `unit` | String | no | — |

### `ServiceAddOnDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Add-on ID |
| `quantity` | f64 | no | Add-on quantity |
| `duration` | f64 | no | Add-on duration (in minutes) |

### `ServiceAddOnResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Add-on ID |
| `quantity` | f64 | no | Add-on quantity |

### `ServiceBookingQueryDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `overrideAvailability` | bool | no | If true the time slot validation would be avoided for any booking creation/update (even the skipSchedulingNotice) |
| `skipSchedulingNotice` | bool | no | If set to true, the minimum scheduling notice and date range would be ignored |

### `ServiceBookingResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `bookingId` | String | **yes** | Booking ID |
| `locationId` | String | **yes** | Location ID |
| `contactId` | String | **yes** | Contact ID |
| `serviceLocationId` | String | **yes** | Service Location ID |
| `title` | String | **yes** | Service Booking Title |
| `startTime` | String | **yes** | Start Time |
| `endTime` | String | **yes** | End Time |
| `services` | Vec<ServiceDTO> | **yes** | Services |
| `timezone` | String | **yes** | Timezone |
| `status` | String | **yes** | Status |
| `deleted` | bool | **yes** | Tells if the booking is deleted |
| `dateAdded` | String | **yes** | Date Added |
| `dateUpdated` | String | **yes** | Date Updated |
| `createdBy` | [`CreatedOrUpdatedBy`](#createdorupdatedby) | **yes** | Booking booked by metadata |
| `meetingLocation` | String | no | Meeting Location (If service location is an ask the booker, then the meeting location is used for the booking) |

### `ServiceBookingsListResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `bookings` | Vec<ServiceBookingResponseDTO> | **yes** | Service Bookings |

### `ServiceDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Service ID |
| `serviceCategoryId` | String | **yes** | Service Category ID |
| `serviceStaffId` | String | **yes** | Service Staff ID |
| `serviceStartTime` | String | **yes** | Service Start Time |
| `serviceEndTime` | String | **yes** | Service End Time |
| `serviceResources` | Vec<ServiceResourceDTO> | no | Service Resources |
| `serviceAddOns` | Vec<ServiceAddOnResponseDTO> | no | Service Add-ons |

### `ServiceLocationListResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `serviceLocations` | Vec<ServiceLocationResponseDTO> | **yes** | List of service locations |

### `ServiceLocationResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Service Location ID |
| `locationId` | String | **yes** | Location ID |
| `name` | String | **yes** | Location name |
| `slug` | String | **yes** | Unique URL-friendly identifier for the service location |
| `isActive` | bool | no | Whether location is active |
| `isPrivate` | bool | no | Whether location is private (not shown publicly) |
| `coverImage` | String | no | URL of the cover image displayed for this location |
| `locationType` | String — `offline`, `ask_booker` | no | Location type |
| `address` | String | no | Use a full street address when locationType is offline. Use a user-facing label when locationType is ask_booker. |
| `phone` | String | no | Contact phone number for the service location |

### `ServiceResourceDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Resource ID |

### `ServiceResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Service ID |
| `locationId` | String | **yes** | Location ID |
| `name` | String | **yes** | Service name |
| `description` | String | no | Service description |
| `slug` | String | no | Unique URL-friendly identifier |
| `eventColor` | String | no | Service event color (hex) |
| `coverImage` | String | no | Service cover image URL |
| `serviceCategoryId` | String | no | Service category ID |
| `payment` | [`PaymentDTO`](#paymentdto) | no | Payment details |
| `serviceDuration` | f64 | no | This controls the duration of the appointment |
| `serviceDurationUnit` | String — `mins`, `hours` | no | Duration unit |
| `preBuffer` | f64 | no | Pre-Buffer is additional time that can be added before an appointment, allowing for extra time to get ready |
| `preBufferUnit` | String — `mins`, `hours` | no | Pre-buffer unit |
| `postBuffer` | f64 | no | Post-buffer: Additional time that can be added after an appointment, allowing for extra time to wrap up |
| `postBufferUnit` | String — `mins`, `hours` | no | Post-buffer unit |
| `isPrivate` | bool | no | Whether service is private (not shown publicly) |
| `formId` | String | no | Custom form ID (will be used to display the custom form on the booking page, if only one service is selected) |
| `variations` | Vec<ServiceVariationDTO> | no | Service variations |
| `staff` | Vec<StaffDTO> | no | Assigned staff members |

### `ServiceResponseWrapperDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `service` | [`ServiceResponseDTO`](#serviceresponsedto) | **yes** | Service details |

### `ServiceVariationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Variation ID |
| `name` | String | **yes** | Variation name |
| `serviceDuration` | f64 | no | This controls the duration of the appointment |
| `serviceDurationUnit` | String — `mins`, `hours` | no | Duration unit |
| `payment` | [`PaymentDTO`](#paymentdto) | no | Payment details |
| `preBuffer` | f64 | no | Pre-Buffer is additional time that can be added before an appointment, allowing for extra time to get ready |
| `preBufferUnit` | String — `mins`, `hours` | no | Pre-buffer unit |
| `postBuffer` | f64 | no | Post-buffer: Additional time that can be added after an appointment, allowing for extra time to wrap up |
| `postBufferUnit` | String — `mins`, `hours` | no | Post-buffer unit |

### `ServicesListResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `services` | Vec<ServiceResponseDTO> | **yes** | List of services |

### `SlotsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `slots` | Vec<String> | **yes** | — |

### `StaffDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Staff ID |

### `TeamMember`

| Field | Type | Required | Description |
|---|---|---|---|
| `userId` | String | **yes** | — |
| `priority` | String — `0`, `0.5`, `1` | no | — |
| `meetingLocationType` | String — `custom`, `zoom`, `gmeet`, `phone`, `address`, `teams`, `booker` | no | 🚨 Deprecated! Use `locationConfigurations.kind` instead. |
| `meetingLocation` | String | no | 🚨 Deprecated! Use `locationConfigurations.location` instead. |
| `isPrimary` | bool | no | Marks a user as primary. This property is required in case of collective booking calendars. Only one user can be primary. |
| `locationConfigurations` | Vec<LocationConfiguration> | no | Meeting location configuration for event calendar. - *Multiple locations are allowed only when one team member is selected.* - *For **Class booking** and **Collective** calendars, only one location co… |

### `TeamMemberResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `userId` | String | **yes** | — |
| `priority` | String — `0`, `0.5`, `1` | no | — |
| `meetingLocationType` | String — `custom`, `zoom`, `gmeet`, `phone`, `address`, `teams`, `booker` | no | 🚨 Deprecated! Use `locationConfigurations.kind` instead. |
| `meetingLocation` | String | no | 🚨 Deprecated! Use `locationConfigurations.location` instead. |
| `isPrimary` | bool | no | Marks a user as primary. This property is required in case of collective booking calendars. Only one user can be primary. |
| `locationConfigurations` | Vec<LocationConfigurationResponse> | no | Meeting location configurations |

### `UpdateAvailability`

| Field | Type | Required | Description |
|---|---|---|---|
| `date` | String | **yes** | Formulate the date string in the format of `<YYYY-MM-DD in local timezone>T00:00:00.000Z`. |
| `hours` | Vec<Hour> | **yes** | — |
| `deleted` | bool | no | — |
| `id` | String | no | The ID of the custom availability object. It is required while updating or deleting the existing custom date availability |

### `UpdateCalendarNotificationsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `receiverType` | String — `contact`, `guest`, `assignedUser`, `emails`, `phoneNumbers`, `business` | no | Notification recipient type |
| `additionalEmailIds` | Vec<String> | no | Additional email addresses to receive notifications. |
| `additionalPhoneNumbers` | Vec<String> | no | Additional phone numbers to receive notifications. |
| `selectedUsers` | Vec<String> | no | Selected users for in-App and business email notifications. Supports user IDs and special keyword "sub_account_admin" |
| `channel` | String — `email`, `inApp`, `sms`, `whatsapp` | no | Notification channel |
| `notificationType` | String — `booked`, `confirmation`, `cancellation`, `reminder`, `followup`, `reschedule` | no | Notification type |
| `isActive` | bool | no | Is the notification active |
| `deleted` | bool | no | Marks the notification as deleted (soft delete) |
| `templateId` | String | no | Template ID for email notification |
| `body` | String | no | Body for email notification. Not necessary for in-App notification |
| `subject` | String | no | Subject for email notification. Not necessary for in-App notification |
| `afterTime` | Vec<SchedulesDTO> | no | Specifies the time after which the follow-up notification should be sent. This is not required for other notification types. |
| `beforeTime` | Vec<SchedulesDTO> | no | Specifies the time before which the reminder notification should be sent. This is not required for other notification types. |
| `fromAddress` | String | no | From address for email notification |
| `fromNumber` | String | no | from number for sms notification |
| `fromName` | String | no | From name for email/sms notification |

### `UpdateCalendarResourceDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | no | — |
| `name` | String | no | — |
| `description` | String | no | — |
| `quantity` | f64 | no | Quantity of the equipment. |
| `outOfService` | f64 | no | Quantity of the out of service equipment. |
| `capacity` | f64 | no | Capacity of the room. |
| `calendarIds` | Vec<String> | no | Service calendar IDs to be mapped with the resource. One equipment can only be mapped with one service calendar. One room can be mapped with multiple service calendars. |
| `isActive` | bool | no | — |

### `UpdateEventCalendarScheduleDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `rules` | Vec<ScheduleRuleDTO> | no | Updated schedule rules defining when the schedule is active |
| `timezone` | String | no | Updated timezone for the schedule (IANA timezone identifier) |

### `UpdateScheduleDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Human-readable name for the schedule |
| `rules` | Vec<ScheduleRuleDTO> | no | Updated schedule rules defining when the schedule is active |
| `timezone` | String | no | Updated timezone for the schedule (IANA timezone identifier) |

### `UpdateServiceBookingDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `serviceLocationId` | String | no | Service Location ID |
| `meetingLocation` | String | no | Meeting Location (If service location is an ask the booker, then the meeting location is required) |
| `title` | String | no | Title |
| `status` | String — `confirmed`, `cancelled`, `invalid`, `new`, `showed`, `no_show` | no | Status |
| `startTime` | String | no | Start Time |
| `endTime` | String | no | End Time |
| `timezone` | String | no | Timezone |
| `services` | Vec<CreateBookingServiceDTO> | no | If provided, services sent in the request will replace the existing services in the booking. |

### `UpdateServiceDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Service name |
| `description` | String | no | Service description |
| `slug` | String | no | Unique URL-friendly identifier |
| `eventColor` | String | no | Service event color (hex) |
| `coverImage` | String | no | Service cover image URL |
| `serviceCategoryId` | String | no | Service category ID |
| `payment` | [`PaymentDTO`](#paymentdto) | no | Payment details (currency configured in Service Global Settings is used.) |
| `serviceDuration` | f64 | no | This controls the duration of the appointment |
| `serviceDurationUnit` | String — `mins`, `hours` | no | Duration unit |
| `preBuffer` | f64 | no | Pre-Buffer is additional time that can be added before an appointment, allowing for extra time to get ready |
| `preBufferUnit` | String — `mins`, `hours` | no | Pre-buffer unit |
| `postBuffer` | f64 | no | Post-buffer: Additional time that can be added after an appointment, allowing for extra time to wrap up |
| `postBufferUnit` | String — `mins`, `hours` | no | Post-buffer unit |
| `isPrivate` | bool | no | Whether service is private (not shown publicly) |
| `formId` | String | no | Custom form ID (will be used to display the custom form on the booking page, if only one service is selected) |
| `staff` | Vec<StaffDTO> | no | Assigned staff members |
| `variations` | Vec<UpdateServiceVariationDTO> | no | Service variations (an empty array removes all variations). Include an id to update an existing variation; omit the id to create a new one. |

### `UpdateServiceLocationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Location name |
| `slug` | String | no | Updated URL-friendly slug identifier |
| `phone` | String | no | Updated contact phone number |
| `address` | String | no | Use a full street address when locationType is offline. Use a user-facing label when locationType is ask_booker. |
| `coverImage` | String | no | Updated URL of the cover image |
| `locationType` | String — `offline`, `ask_booker` | no | Location type |

### `UpdateServiceVariationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `serviceDuration` | f64 | no | This controls the duration of the appointment |
| `serviceDurationUnit` | String — `mins`, `hours` | no | Duration unit |
| `preBuffer` | f64 | no | Pre-Buffer is additional time that can be added before an appointment, allowing for extra time to get ready |
| `preBufferUnit` | String — `mins`, `hours` | no | Pre-buffer unit |
| `postBuffer` | f64 | no | Post-buffer: Additional time that can be added after an appointment, allowing for extra time to wrap up |
| `postBufferUnit` | String — `mins`, `hours` | no | Post-buffer unit |
| `payment` | [`PaymentDTO`](#paymentdto) | no | Payment details |
| `id` | String | no | Variation ID |
| `name` | String | no | Variation name |

### `ValidateGroupSlugPostBody`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location Id |
| `slug` | String | **yes** | Slug |

### `ValidateGroupSlugSuccessResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `available` | bool | **yes** | — |

