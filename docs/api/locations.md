# `locations`

**29** operations / **47** models in API v2 · **32** operations / **53** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `locations` cargo feature on `ghl-sdk`, then call any of the 29 generated methods on `ghl.locations()`:

```toml
ghl-sdk = { version = "0.4", features = ["locations"] }
```

This module also has hand-written ergonomic helpers on the same `ghl.locations()`: `get()`, `search()` (envelope unwrapping, paginated `Stream`s).

MCP tools: `ghl_list_locations`.


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `POST` | `/locations/` | Create Sub-Account (Formerly Location) | `create_sub_account_formerly_location()` | `locations.post_locations` |
| `GET` | `/locations/search` | Search | `search_op()` | `locations.get_locations_search` |
| `DELETE` | `/locations/{locationId}` | Delete Sub-Account (Formerly Location) | `delete_sub_account_formerly_location()` | `locations.delete_locations_by_locationId` |
| `GET` | `/locations/{locationId}` | Get Sub-Account (Formerly Location) | `get_sub_account_formerly_location()` | `locations.get_locations_by_locationId` |
| `PUT` | `/locations/{locationId}` | Put Sub-Account (Formerly Location) | `put_sub_account_formerly_location()` | `locations.put_locations_by_locationId` |
| `GET` | `/locations/{locationId}/customFields` | Get Custom Fields | `get_custom_fields()` | `locations.get_locations_by_locationId_customFields` |
| `POST` | `/locations/{locationId}/customFields` | Create Custom Field | `create_custom_field()` | `locations.post_locations_by_locationId_customFields` |
| `POST` | `/locations/{locationId}/customFields/upload` | Uploads File to customFields | `uploads_file_to_custom_fields()` | `locations.post_locations_by_locationId_customFields_upload` |
| `DELETE` | `/locations/{locationId}/customFields/{id}` | Delete Custom Field | `delete_custom_field()` | `locations.delete_locations_by_locationId_customFields_by_id` |
| `GET` | `/locations/{locationId}/customFields/{id}` | Get Custom Field | `get_custom_field()` | `locations.get_locations_by_locationId_customFields_by_id` |
| `PUT` | `/locations/{locationId}/customFields/{id}` | Update Custom Field | `update_custom_field()` | `locations.put_locations_by_locationId_customFields_by_id` |
| `GET` | `/locations/{locationId}/customValues` | Get Custom Values | `get_custom_values()` | `locations.get_locations_by_locationId_customValues` |
| `POST` | `/locations/{locationId}/customValues` | Create Custom Value | `create_custom_value()` | `locations.post_locations_by_locationId_customValues` |
| `DELETE` | `/locations/{locationId}/customValues/{id}` | Delete Custom Value | `delete_custom_value()` | `locations.delete_locations_by_locationId_customValues_by_id` |
| `GET` | `/locations/{locationId}/customValues/{id}` | Get Custom Value | `get_custom_value()` | `locations.get_locations_by_locationId_customValues_by_id` |
| `PUT` | `/locations/{locationId}/customValues/{id}` | Update Custom Value | `update_custom_value()` | `locations.put_locations_by_locationId_customValues_by_id` |
| `POST` | `/locations/{locationId}/recurring-tasks` | Create Recurring Task | `create_recurring_task()` | `locations.post_locations_by_locationId_recurring_tasks` |
| `DELETE` | `/locations/{locationId}/recurring-tasks/{id}` | Delete Recurring Task | `delete_recurring_task()` | `locations.delete_locations_by_locationId_recurring_tasks_by_id` |
| `GET` | `/locations/{locationId}/recurring-tasks/{id}` | Get Recurring Task By Id | `get_recurring_task_by_id()` | `locations.get_locations_by_locationId_recurring_tasks_by_id` |
| `PUT` | `/locations/{locationId}/recurring-tasks/{id}` | Update Recurring Task | `update_recurring_task()` | `locations.put_locations_by_locationId_recurring_tasks_by_id` |
| `GET` | `/locations/{locationId}/tags` | Get Tags | `get_tags()` | `locations.get_locations_by_locationId_tags` |
| `POST` | `/locations/{locationId}/tags` | Create Tag | `create_tag()` | `locations.post_locations_by_locationId_tags` |
| `DELETE` | `/locations/{locationId}/tags/{tagId}` | Delete tag | `delete_tag()` | `locations.delete_locations_by_locationId_tags_by_tagId` |
| `GET` | `/locations/{locationId}/tags/{tagId}` | Get tag by id | `get_tag_by_id()` | `locations.get_locations_by_locationId_tags_by_tagId` |
| `PUT` | `/locations/{locationId}/tags/{tagId}` | Update tag | `update_tag()` | `locations.put_locations_by_locationId_tags_by_tagId` |
| `POST` | `/locations/{locationId}/tasks/search` | Task Search Filter | `task_search_filter()` | `locations.post_locations_by_locationId_tasks_search` |
| `GET` | `/locations/{locationId}/templates` | GET all or email/sms templates | `get_all_or_email_sms_templates()` | `locations.get_locations_by_locationId_templates` |
| `DELETE` | `/locations/{locationId}/templates/{id}` | DELETE an email/sms template | `delete_an_email_sms_template()` | `locations.delete_locations_by_locationId_templates_by_id` |
| `GET` | `/locations/{locationId}/timezones` | Fetch Timezones | `fetch_timezones()` | `locations.get_locations_by_locationId_timezones` |

### Endpoint details — v2

#### `POST /locations/`

**Create Sub-Account (Formerly Location)**

<div> <p>Create a new Sub-Account (Formerly Location) based on the data provided</p> <div> <span style= "display: inline-block; width: 25px; height: 25px; background-color: yellow; color: black; font-weight: bold; font-size: 24px; text-align: center; line-height: 22px; border: 2px solid black; border-radius: 10%; margin-right: 10px;"> ! </span> <span> <strong> This feature is only available on Agency Pro ($497) plan. </strong> </span> </div> </div>

Operation id: `locations.post_locations` · `Version: 2021-07-28` · Scopes: `locations.write`

*Request body*: [`CreateLocationDto`](#createlocationdto)

*Response*: [`CreateLocationSuccessfulResponseDto`](#createlocationsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().create_sub_account_formerly_location(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.post_locations",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /locations/search`

**Search**

Search Sub-Account (Formerly Location)

Operation id: `locations.get_locations_search` · `Version: 2021-07-28` · Scopes: `locations.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | no | The company/agency id on which you want to perform the search |
| `skip` | string | no | The value by which the results should be skipped. Default will be 0 |
| `limit` | string | no | The value by which the results should be limited. Default will be 10 |
| `order` | string | no | The order in which the results should be returned - Allowed values asc, desc. Default will be asc |
| `email` | string | no | — |

*Response*: [`SearchSuccessfulResponseDto`](#searchsuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::locations::SearchOpParams;

let params = SearchOpParams::new();
let out = ghl.locations().search_op(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.get_locations_search"
  }
}
```

</details>

#### `DELETE /locations/{locationId}`

**Delete Sub-Account (Formerly Location)**

Delete a Sub-Account (Formerly Location) from the Agency

Operation id: `locations.delete_locations_by_locationId` · `Version: 2021-07-28` · Scopes: `locations.internal-access-only`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `deleteTwilioAccount` | boolean | **yes** | Boolean value to indicate whether to delete Twilio Account or not |

*Response*: [`LocationDeletedSuccessfulResponseDto`](#locationdeletedsuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::locations::DeleteSubAccountFormerlyLocationParams;

let params = DeleteSubAccountFormerlyLocationParams::new("deleteTwilioAccount");
let out = ghl.locations().delete_sub_account_formerly_location(&locationId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.delete_locations_by_locationId",
    "path_params": {
      "locationId": "<locationId>"
    },
    "query": {
      "deleteTwilioAccount": "<deleteTwilioAccount>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}`

**Get Sub-Account (Formerly Location)**

Get details of a Sub-Account (Formerly Location) by passing the sub-account id

Operation id: `locations.get_locations_by_locationId` · `Version: 2021-07-28` · Scopes: `locations.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Response*: [`GetLocationByIdSuccessfulResponseDto`](#getlocationbyidsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().get_sub_account_formerly_location(&locationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.get_locations_by_locationId",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /locations/{locationId}`

**Put Sub-Account (Formerly Location)**

Update a Sub-Account (Formerly Location) based on the data provided

Operation id: `locations.put_locations_by_locationId` · `Version: 2021-07-28` · Scopes: `locations.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`UpdateLocationDto`](#updatelocationdto)

*Response*: [`CreateLocationSuccessfulResponseDto`](#createlocationsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().put_sub_account_formerly_location(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.put_locations_by_locationId",
    "path_params": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}/customFields`

**Get Custom Fields**

Operation id: `locations.get_locations_by_locationId_customFields` · `Version: 2021-07-28` · Scopes: `locations/customFields.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `model` | enum: `contact`, `opportunity`, `all` | no | Model of the custom field you want to retrieve |

*Response*: [`CustomFieldsListSuccessfulResponseDto`](#customfieldslistsuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::locations::GetCustomFieldsParams;

let params = GetCustomFieldsParams::new();
let out = ghl.locations().get_custom_fields(&locationId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.get_locations_by_locationId_customFields",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /locations/{locationId}/customFields`

**Create Custom Field**

Operation id: `locations.post_locations_by_locationId_customFields` · `Version: 2021-07-28` · Scopes: `locations/customFields.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`CreateCustomFieldsDTO`](#createcustomfieldsdto)

*Response*: [`CustomFieldSuccessfulResponseDto`](#customfieldsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().create_custom_field(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.post_locations_by_locationId_customFields",
    "path_params": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /locations/{locationId}/customFields/upload`

**Uploads File to customFields**

Operation id: `locations.post_locations_by_locationId_customFields_upload` · `Version: 2021-07-28` · Scopes: `locations/customFields.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Response*: [`FileUploadResponseDto`](#fileuploadresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().uploads_file_to_custom_fields(&locationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.post_locations_by_locationId_customFields_upload",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `DELETE /locations/{locationId}/customFields/{id}`

**Delete Custom Field**

Operation id: `locations.delete_locations_by_locationId_customFields_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Custom Field Id |

*Response*: [`CustomFieldDeleteSuccessfulResponseDto`](#customfielddeletesuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().delete_custom_field(&locationId, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.delete_locations_by_locationId_customFields_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}/customFields/{id}`

**Get Custom Field**

Operation id: `locations.get_locations_by_locationId_customFields_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Custom Field Id or Field Key (e.g. "contact.first_name" or "opportunity.pipeline_id") |

*Response*: [`CustomFieldSuccessfulResponseDto`](#customfieldsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().get_custom_field(&locationId, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.get_locations_by_locationId_customFields_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `PUT /locations/{locationId}/customFields/{id}`

**Update Custom Field**

Operation id: `locations.put_locations_by_locationId_customFields_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Custom Field Id |

*Request body*: [`UpdateCustomFieldsDTO`](#updatecustomfieldsdto)

*Response*: [`CustomFieldSuccessfulResponseDto`](#customfieldsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().update_custom_field(&locationId, &id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.put_locations_by_locationId_customFields_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}/customValues`

**Get Custom Values**

Operation id: `locations.get_locations_by_locationId_customValues` · `Version: 2021-07-28` · Scopes: `locations/customValues.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Response*: [`CustomValuesListSuccessfulResponseDto`](#customvalueslistsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().get_custom_values(&locationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.get_locations_by_locationId_customValues",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /locations/{locationId}/customValues`

**Create Custom Value**

Operation id: `locations.post_locations_by_locationId_customValues` · `Version: 2021-07-28` · Scopes: `locations/customValues.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`customValuesDTO`](#customvaluesdto)

*Response*: [`CustomValueIdSuccessfulResponseDto`](#customvalueidsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().create_custom_value(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.post_locations_by_locationId_customValues",
    "path_params": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /locations/{locationId}/customValues/{id}`

**Delete Custom Value**

Operation id: `locations.delete_locations_by_locationId_customValues_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Custom Value Id |

*Response*: [`CustomValueDeleteSuccessfulResponseDto`](#customvaluedeletesuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().delete_custom_value(&locationId, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.delete_locations_by_locationId_customValues_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}/customValues/{id}`

**Get Custom Value**

Operation id: `locations.get_locations_by_locationId_customValues_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Custom Value Id |

*Response*: [`CustomValueIdSuccessfulResponseDto`](#customvalueidsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().get_custom_value(&locationId, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.get_locations_by_locationId_customValues_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `PUT /locations/{locationId}/customValues/{id}`

**Update Custom Value**

Operation id: `locations.put_locations_by_locationId_customValues_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Custom Value Id |

*Request body*: [`customValuesDTO`](#customvaluesdto)

*Response*: [`CustomValueIdSuccessfulResponseDto`](#customvalueidsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().update_custom_value(&locationId, &id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.put_locations_by_locationId_customValues_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /locations/{locationId}/recurring-tasks`

**Create Recurring Task**

Operation id: `locations.post_locations_by_locationId_recurring_tasks` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Request body*: [`RecurringTaskCreateDTO`](#recurringtaskcreatedto)

*Response*: [`RecurringTaskSingleResponseDTO`](#recurringtasksingleresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().create_recurring_task(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.post_locations_by_locationId_recurring_tasks",
    "path_params": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /locations/{locationId}/recurring-tasks/{id}`

**Delete Recurring Task**

Operation id: `locations.delete_locations_by_locationId_recurring_tasks_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Recurring Task Id |
| `locationId` | string | **yes** | Location Id |

*Response*: [`DeleteRecurringTaskResponseDTO`](#deleterecurringtaskresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().delete_recurring_task(&id, &locationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.delete_locations_by_locationId_recurring_tasks_by_id",
    "path_params": {
      "id": "<id>",
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}/recurring-tasks/{id}`

**Get Recurring Task By Id**

Operation id: `locations.get_locations_by_locationId_recurring_tasks_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Recurring Task Id |
| `locationId` | string | **yes** | Location Id |

*Response*: [`RecurringTaskSingleResponseDTO`](#recurringtasksingleresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().get_recurring_task_by_id(&id, &locationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.get_locations_by_locationId_recurring_tasks_by_id",
    "path_params": {
      "id": "<id>",
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /locations/{locationId}/recurring-tasks/{id}`

**Update Recurring Task**

Operation id: `locations.put_locations_by_locationId_recurring_tasks_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Recurring Task Id |
| `locationId` | string | **yes** | Location Id |

*Request body*: [`RecurringTaskUpdateDTO`](#recurringtaskupdatedto)

*Response*: [`RecurringTaskSingleResponseDTO`](#recurringtasksingleresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().update_recurring_task(&id, &locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.put_locations_by_locationId_recurring_tasks_by_id",
    "path_params": {
      "id": "<id>",
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}/tags`

**Get Tags**

Get Sub-Account (Formerly Location) Tags

Operation id: `locations.get_locations_by_locationId_tags` · `Version: 2021-07-28` · Scopes: `locations/tags.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Response*: [`LocationTagsSuccessfulResponseDto`](#locationtagssuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().get_tags(&locationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.get_locations_by_locationId_tags",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /locations/{locationId}/tags`

**Create Tag**

Create tag

Operation id: `locations.post_locations_by_locationId_tags` · `Version: 2021-07-28` · Scopes: `locations/tags.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`tagBody`](#tagbody)

*Response*: [`LocationTagSuccessfulResponseDto`](#locationtagsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().create_tag(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.post_locations_by_locationId_tags",
    "path_params": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /locations/{locationId}/tags/{tagId}`

**Delete tag**

Operation id: `locations.delete_locations_by_locationId_tags_by_tagId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `tagId` | string | **yes** | Tag Id |

*Response*: [`LocationTagDeleteSuccessfulResponseDto`](#locationtagdeletesuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().delete_tag(&locationId, &tagId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.delete_locations_by_locationId_tags_by_tagId",
    "path_params": {
      "locationId": "<locationId>",
      "tagId": "<tagId>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}/tags/{tagId}`

**Get tag by id**

Operation id: `locations.get_locations_by_locationId_tags_by_tagId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `tagId` | string | **yes** | Tag Id |

*Response*: [`LocationTagSuccessfulResponseDto`](#locationtagsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().get_tag_by_id(&locationId, &tagId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.get_locations_by_locationId_tags_by_tagId",
    "path_params": {
      "locationId": "<locationId>",
      "tagId": "<tagId>"
    }
  }
}
```

</details>

#### `PUT /locations/{locationId}/tags/{tagId}`

**Update tag**

Operation id: `locations.put_locations_by_locationId_tags_by_tagId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `tagId` | string | **yes** | Tag Id |

*Request body*: [`tagBody`](#tagbody)

*Response*: [`LocationTagSuccessfulResponseDto`](#locationtagsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().update_tag(&locationId, &tagId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.put_locations_by_locationId_tags_by_tagId",
    "path_params": {
      "locationId": "<locationId>",
      "tagId": "<tagId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /locations/{locationId}/tasks/search`

**Task Search Filter**

Task Search

Operation id: `locations.post_locations_by_locationId_tasks_search` · `Version: 2021-07-28` · Scopes: `locations/tasks.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`TaskSearchParamsDto`](#tasksearchparamsdto)

*Response*: [`LocationTaskListSuccessfulResponseDto`](#locationtasklistsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.locations().task_search_filter(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.post_locations_by_locationId_tasks_search",
    "path_params": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}/templates`

**GET all or email/sms templates**

Operation id: `locations.get_locations_by_locationId_templates` · `Version: 2021-07-28` · Scopes: `locations/templates.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `deleted` | boolean | no | — |
| `skip` | string | no | — |
| `limit` | string | no | — |
| `type` | enum: `sms`, `email`, `whatsapp` | no | — |
| `originId` | string | **yes** | Origin Id |

*Response*: [`GetTemplatesSuccessfulResponseDto`](#gettemplatessuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::locations::GetAllOrEmailSmsTemplatesParams;

let params = GetAllOrEmailSmsTemplatesParams::new("originId");
let out = ghl.locations().get_all_or_email_sms_templates(&locationId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.get_locations_by_locationId_templates",
    "path_params": {
      "locationId": "<locationId>"
    },
    "query": {
      "originId": "<originId>"
    }
  }
}
```

</details>

#### `DELETE /locations/{locationId}/templates/{id}`

**DELETE an email/sms template**

Operation id: `locations.delete_locations_by_locationId_templates_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Template Id |

*Rust*:

```rust,ignore
let out = ghl.locations().delete_an_email_sms_template(&locationId, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.delete_locations_by_locationId_templates_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}/timezones`

**Fetch Timezones**

Fetch the available timezones

Operation id: `locations.get_locations_by_locationId_timezones` · `Version: 2021-07-28` · Scopes: `locations.readonly`

*Rust*:

```rust,ignore
let out = ghl.locations().fetch_timezones().await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "locations.get_locations_by_locationId_timezones"
  }
}
```

</details>

## Endpoints — API v3

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `POST` | `/locations/` | Create Sub-Account (Formerly Location) | `v3:locations.post_locations` |
| `GET` | `/locations/search` | Search | `v3:locations.get_locations_search` |
| `DELETE` | `/locations/{locationId}` | Delete Sub-Account (Formerly Location) | `v3:locations.delete_locations_by_locationId` |
| `GET` | `/locations/{locationId}` | Get Sub-Account (Formerly Location) | `v3:locations.get_locations_by_locationId` |
| `PUT` | `/locations/{locationId}` | Put Sub-Account (Formerly Location) | `v3:locations.put_locations_by_locationId` |
| `GET` | `/locations/{locationId}/conversationChannels/{type}` | Get Conversation Channel | `v3:locations.get_locations_by_locationId_conversationChannels_by_type` |
| `GET` | `/locations/{locationId}/customFields` | Get Custom Fields | `v3:locations.get_locations_by_locationId_customFields` |
| `POST` | `/locations/{locationId}/customFields` | Create Custom Field | `v3:locations.post_locations_by_locationId_customFields` |
| `POST` | `/locations/{locationId}/customFields/upload` | Uploads File to customFields | `v3:locations.post_locations_by_locationId_customFields_upload` |
| `DELETE` | `/locations/{locationId}/customFields/{id}` | Delete Custom Field | `v3:locations.delete_locations_by_locationId_customFields_by_id` |
| `GET` | `/locations/{locationId}/customFields/{id}` | Get Custom Field | `v3:locations.get_locations_by_locationId_customFields_by_id` |
| `PUT` | `/locations/{locationId}/customFields/{id}` | Update Custom Field | `v3:locations.put_locations_by_locationId_customFields_by_id` |
| `GET` | `/locations/{locationId}/customValues` | Get Custom Values | `v3:locations.get_locations_by_locationId_customValues` |
| `POST` | `/locations/{locationId}/customValues` | Create Custom Value | `v3:locations.post_locations_by_locationId_customValues` |
| `DELETE` | `/locations/{locationId}/customValues/{id}` | Delete Custom Value | `v3:locations.delete_locations_by_locationId_customValues_by_id` |
| `GET` | `/locations/{locationId}/customValues/{id}` | Get Custom Value | `v3:locations.get_locations_by_locationId_customValues_by_id` |
| `PUT` | `/locations/{locationId}/customValues/{id}` | Update Custom Value | `v3:locations.put_locations_by_locationId_customValues_by_id` |
| `GET` | `/locations/{locationId}/permissions` | Get Permissions | `v3:locations.get_locations_by_locationId_permissions` |
| `PUT` | `/locations/{locationId}/permissions` | Update Permissions | `v3:locations.put_locations_by_locationId_permissions` |
| `POST` | `/locations/{locationId}/recurring-tasks` | Create Recurring Task | `v3:locations.post_locations_by_locationId_recurring_tasks` |
| `DELETE` | `/locations/{locationId}/recurring-tasks/{id}` | Delete Recurring Task | `v3:locations.delete_locations_by_locationId_recurring_tasks_by_id` |
| `GET` | `/locations/{locationId}/recurring-tasks/{id}` | Get Recurring Task By Id | `v3:locations.get_locations_by_locationId_recurring_tasks_by_id` |
| `PUT` | `/locations/{locationId}/recurring-tasks/{id}` | Update Recurring Task | `v3:locations.put_locations_by_locationId_recurring_tasks_by_id` |
| `GET` | `/locations/{locationId}/tags` | Get Tags | `v3:locations.get_locations_by_locationId_tags` |
| `POST` | `/locations/{locationId}/tags` | Create Tag | `v3:locations.post_locations_by_locationId_tags` |
| `DELETE` | `/locations/{locationId}/tags/{tagId}` | Delete tag | `v3:locations.delete_locations_by_locationId_tags_by_tagId` |
| `GET` | `/locations/{locationId}/tags/{tagId}` | Get tag by id | `v3:locations.get_locations_by_locationId_tags_by_tagId` |
| `PUT` | `/locations/{locationId}/tags/{tagId}` | Update tag | `v3:locations.put_locations_by_locationId_tags_by_tagId` |
| `POST` | `/locations/{locationId}/tasks/search` | Task Search Filter | `v3:locations.post_locations_by_locationId_tasks_search` |
| `GET` | `/locations/{locationId}/templates` | GET all or email/sms templates | `v3:locations.get_locations_by_locationId_templates` |
| `DELETE` | `/locations/{locationId}/templates/{id}` | DELETE an email/sms template | `v3:locations.delete_locations_by_locationId_templates_by_id` |
| `GET` | `/locations/{locationId}/timezones` | Fetch Timezones | `v3:locations.get_locations_by_locationId_timezones` |

### Endpoint details — v3

#### `POST /locations/`

**Create Sub-Account (Formerly Location)**

<div> <p>Create a new Sub-Account (Formerly Location) based on the data provided</p> <div> <span> :::info This feature is only available on Agency Pro ($497) plan. ::: </span> </div> </div>

Operation id: `v3:locations.post_locations` · `Version: v3` · Scopes: `locations.write`

*Request body*: [`CreateLocationDto`](#createlocationdto)

*Response*: [`CreateLocationSuccessfulResponseDto`](#createlocationsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.post_locations",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /locations/search`

**Search**

Search Sub-Account (Formerly Location)

Operation id: `v3:locations.get_locations_search` · `Version: v3` · Scopes: `locations.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | no | The company/agency id on which you want to perform the search |
| `skip` | string | no | The value by which the results should be skipped. Default will be 0 |
| `limit` | string | no | The value by which the results should be limited. Default will be 10 |
| `order` | string | no | The order in which the results should be returned - Allowed values asc, desc. Default will be asc |
| `email` | string | no | — |

*Response*: [`SearchSuccessfulResponseDto`](#searchsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.get_locations_search"
  }
}
```

</details>

#### `DELETE /locations/{locationId}`

**Delete Sub-Account (Formerly Location)**

Delete a Sub-Account (Formerly Location) from the Agency

Operation id: `v3:locations.delete_locations_by_locationId` · `Version: v3` · Scopes: `locations.internal-access-only`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `deleteTwilioAccount` | boolean | **yes** | Boolean value to indicate whether to delete Twilio Account or not |

*Response*: [`LocationDeletedSuccessfulResponseDto`](#locationdeletedsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.delete_locations_by_locationId",
    "path_params": {
      "locationId": "<locationId>"
    },
    "query": {
      "deleteTwilioAccount": "<deleteTwilioAccount>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}`

**Get Sub-Account (Formerly Location)**

Get details of a Sub-Account (Formerly Location) by passing the sub-account id

Operation id: `v3:locations.get_locations_by_locationId` · `Version: v3` · Scopes: `locations.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Response*: [`GetLocationByIdSuccessfulResponseDto`](#getlocationbyidsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.get_locations_by_locationId",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /locations/{locationId}`

**Put Sub-Account (Formerly Location)**

Update a Sub-Account (Formerly Location) based on the data provided

Operation id: `v3:locations.put_locations_by_locationId` · `Version: v3` · Scopes: `locations.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`UpdateLocationDto`](#updatelocationdto)

*Response*: [`CreateLocationSuccessfulResponseDto`](#createlocationsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.put_locations_by_locationId",
    "path_params": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}/conversationChannels/{type}`

**Get Conversation Channel**

Get the conversation channel providers configured for a location by type (SMS or Email)

Operation id: `v3:locations.get_locations_by_locationId_conversationChannels_by_type` · `Version: v3` · Scopes: `locations.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `type` | enum: `SMS`, `Email` | **yes** | Channel type to retrieve providers for |

*Response*: [`GetConversationChannelListSuccessfulResponseDto`](#getconversationchannellistsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.get_locations_by_locationId_conversationChannels_by_type",
    "path_params": {
      "locationId": "<locationId>",
      "type": "<type>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}/customFields`

**Get Custom Fields**

Operation id: `v3:locations.get_locations_by_locationId_customFields` · `Version: v3` · Scopes: `locations/customFields.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `model` | enum: `contact`, `opportunity`, `all` | no | Model of the custom field you want to retrieve |

*Response*: [`CustomFieldsListSuccessfulResponseDto`](#customfieldslistsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.get_locations_by_locationId_customFields",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /locations/{locationId}/customFields`

**Create Custom Field**

Operation id: `v3:locations.post_locations_by_locationId_customFields` · `Version: v3` · Scopes: `locations/customFields.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`CreateCustomFieldsDTO`](#createcustomfieldsdto)

*Response*: [`CustomFieldSuccessfulResponseDto`](#customfieldsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.post_locations_by_locationId_customFields",
    "path_params": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /locations/{locationId}/customFields/upload`

**Uploads File to customFields**

Operation id: `v3:locations.post_locations_by_locationId_customFields_upload` · `Version: v3` · Scopes: `locations/customFields.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Response*: [`FileUploadResponseDto`](#fileuploadresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.post_locations_by_locationId_customFields_upload",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `DELETE /locations/{locationId}/customFields/{id}`

**Delete Custom Field**

Operation id: `v3:locations.delete_locations_by_locationId_customFields_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Custom Field Id |

*Response*: [`CustomFieldDeleteSuccessfulResponseDto`](#customfielddeletesuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.delete_locations_by_locationId_customFields_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}/customFields/{id}`

**Get Custom Field**

Operation id: `v3:locations.get_locations_by_locationId_customFields_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Custom Field Id or Field Key (e.g. "contact.first_name" or "opportunity.pipeline_id") |

*Response*: [`CustomFieldSuccessfulResponseDto`](#customfieldsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.get_locations_by_locationId_customFields_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `PUT /locations/{locationId}/customFields/{id}`

**Update Custom Field**

Operation id: `v3:locations.put_locations_by_locationId_customFields_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Custom Field Id |

*Request body*: [`UpdateCustomFieldsDTO`](#updatecustomfieldsdto)

*Response*: [`CustomFieldSuccessfulResponseDto`](#customfieldsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.put_locations_by_locationId_customFields_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}/customValues`

**Get Custom Values**

Operation id: `v3:locations.get_locations_by_locationId_customValues` · `Version: v3` · Scopes: `locations/customValues.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Response*: [`CustomValuesListSuccessfulResponseDto`](#customvalueslistsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.get_locations_by_locationId_customValues",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /locations/{locationId}/customValues`

**Create Custom Value**

Operation id: `v3:locations.post_locations_by_locationId_customValues` · `Version: v3` · Scopes: `locations/customValues.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`customValuesDTO`](#customvaluesdto)

*Response*: [`CustomValueIdSuccessfulResponseDto`](#customvalueidsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.post_locations_by_locationId_customValues",
    "path_params": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /locations/{locationId}/customValues/{id}`

**Delete Custom Value**

Operation id: `v3:locations.delete_locations_by_locationId_customValues_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Custom Value Id |

*Response*: [`CustomValueDeleteSuccessfulResponseDto`](#customvaluedeletesuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.delete_locations_by_locationId_customValues_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}/customValues/{id}`

**Get Custom Value**

Operation id: `v3:locations.get_locations_by_locationId_customValues_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Custom Value Id |

*Response*: [`CustomValueIdSuccessfulResponseDto`](#customvalueidsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.get_locations_by_locationId_customValues_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `PUT /locations/{locationId}/customValues/{id}`

**Update Custom Value**

Operation id: `v3:locations.put_locations_by_locationId_customValues_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Custom Value Id |

*Request body*: [`customValuesDTO`](#customvaluesdto)

*Response*: [`CustomValueIdSuccessfulResponseDto`](#customvalueidsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.put_locations_by_locationId_customValues_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}/permissions`

**Get Permissions**

Get Sub-Account (Formerly Location) permissions

Operation id: `v3:locations.get_locations_by_locationId_permissions` · `Version: v3` · Scopes: `locations/write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Response*: [`PermissionsResponseDto`](#permissionsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.get_locations_by_locationId_permissions",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /locations/{locationId}/permissions`

**Update Permissions**

Update Sub-Account (Formerly Location) permissions

Operation id: `v3:locations.put_locations_by_locationId_permissions` · `Version: v3` · Scopes: `locations/write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`UpdatePermissionsDto`](#updatepermissionsdto)

*Response*: [`PermissionsResponseDto`](#permissionsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.put_locations_by_locationId_permissions",
    "path_params": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /locations/{locationId}/recurring-tasks`

**Create Recurring Task**

Operation id: `v3:locations.post_locations_by_locationId_recurring_tasks` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Request body*: [`RecurringTaskCreateDTO`](#recurringtaskcreatedto)

*Response*: [`RecurringTaskSingleResponseDTO`](#recurringtasksingleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.post_locations_by_locationId_recurring_tasks",
    "path_params": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /locations/{locationId}/recurring-tasks/{id}`

**Delete Recurring Task**

Operation id: `v3:locations.delete_locations_by_locationId_recurring_tasks_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Recurring Task Id |
| `locationId` | string | **yes** | Location Id |

*Response*: [`DeleteRecurringTaskResponseDTO`](#deleterecurringtaskresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.delete_locations_by_locationId_recurring_tasks_by_id",
    "path_params": {
      "id": "<id>",
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}/recurring-tasks/{id}`

**Get Recurring Task By Id**

Operation id: `v3:locations.get_locations_by_locationId_recurring_tasks_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Recurring Task Id |
| `locationId` | string | **yes** | Location Id |

*Response*: [`RecurringTaskSingleResponseDTO`](#recurringtasksingleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.get_locations_by_locationId_recurring_tasks_by_id",
    "path_params": {
      "id": "<id>",
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /locations/{locationId}/recurring-tasks/{id}`

**Update Recurring Task**

Operation id: `v3:locations.put_locations_by_locationId_recurring_tasks_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Recurring Task Id |
| `locationId` | string | **yes** | Location Id |

*Request body*: [`RecurringTaskUpdateDTO`](#recurringtaskupdatedto)

*Response*: [`RecurringTaskSingleResponseDTO`](#recurringtasksingleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.put_locations_by_locationId_recurring_tasks_by_id",
    "path_params": {
      "id": "<id>",
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}/tags`

**Get Tags**

Get Sub-Account (Formerly Location) Tags

Operation id: `v3:locations.get_locations_by_locationId_tags` · `Version: v3` · Scopes: `locations/tags.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Response*: [`LocationTagsSuccessfulResponseDto`](#locationtagssuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.get_locations_by_locationId_tags",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /locations/{locationId}/tags`

**Create Tag**

Create tag

Operation id: `v3:locations.post_locations_by_locationId_tags` · `Version: v3` · Scopes: `locations/tags.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`tagBody`](#tagbody)

*Response*: [`LocationTagSuccessfulResponseDto`](#locationtagsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.post_locations_by_locationId_tags",
    "path_params": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /locations/{locationId}/tags/{tagId}`

**Delete tag**

Operation id: `v3:locations.delete_locations_by_locationId_tags_by_tagId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `tagId` | string | **yes** | Tag Id |

*Response*: [`LocationTagDeleteSuccessfulResponseDto`](#locationtagdeletesuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.delete_locations_by_locationId_tags_by_tagId",
    "path_params": {
      "locationId": "<locationId>",
      "tagId": "<tagId>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}/tags/{tagId}`

**Get tag by id**

Operation id: `v3:locations.get_locations_by_locationId_tags_by_tagId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `tagId` | string | **yes** | Tag Id |

*Response*: [`LocationTagSuccessfulResponseDto`](#locationtagsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.get_locations_by_locationId_tags_by_tagId",
    "path_params": {
      "locationId": "<locationId>",
      "tagId": "<tagId>"
    }
  }
}
```

</details>

#### `PUT /locations/{locationId}/tags/{tagId}`

**Update tag**

Operation id: `v3:locations.put_locations_by_locationId_tags_by_tagId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `tagId` | string | **yes** | Tag Id |

*Request body*: [`tagBody`](#tagbody)

*Response*: [`LocationTagSuccessfulResponseDto`](#locationtagsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.put_locations_by_locationId_tags_by_tagId",
    "path_params": {
      "locationId": "<locationId>",
      "tagId": "<tagId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /locations/{locationId}/tasks/search`

**Task Search Filter**

Task Search

Operation id: `v3:locations.post_locations_by_locationId_tasks_search` · `Version: v3` · Scopes: `locations/tasks.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`TaskSearchParamsDto`](#tasksearchparamsdto)

*Response*: [`LocationTaskListSuccessfulResponseDto`](#locationtasklistsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.post_locations_by_locationId_tasks_search",
    "path_params": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}/templates`

**GET all or email/sms templates**

Operation id: `v3:locations.get_locations_by_locationId_templates` · `Version: v3` · Scopes: `locations/templates.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `deleted` | boolean | no | — |
| `skip` | string | no | — |
| `limit` | string | no | — |
| `type` | enum: `sms`, `email`, `whatsapp` | no | — |
| `originId` | string | **yes** | Origin Id |

*Response*: [`GetTemplatesSuccessfulResponseDto`](#gettemplatessuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.get_locations_by_locationId_templates",
    "path_params": {
      "locationId": "<locationId>"
    },
    "query": {
      "originId": "<originId>"
    }
  }
}
```

</details>

#### `DELETE /locations/{locationId}/templates/{id}`

**DELETE an email/sms template**

Operation id: `v3:locations.delete_locations_by_locationId_templates_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Template Id |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.delete_locations_by_locationId_templates_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /locations/{locationId}/timezones`

**Fetch Timezones**

Fetch the available timezones

Operation id: `v3:locations.get_locations_by_locationId_timezones` · `Version: v3` · Scopes: `locations.readonly`

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:locations.get_locations_by_locationId_timezones"
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::locations::*` (enable the `locations` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/locations/).

### `BusinessSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | — |
| `address` | String | no | — |
| `city` | String | no | — |
| `state` | String | no | — |
| `country` | String | no | — |
| `postalCode` | String | no | — |
| `website` | String | no | — |
| `timezone` | String | no | — |
| `logoUrl` | String | no | — |

### `CreateCustomFieldsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | — |
| `dataType` | String | **yes** | — |
| `placeholder` | String | no | — |
| `acceptedFormat` | Vec<String> | no | — |
| `isMultipleFile` | bool | no | — |
| `maxNumberOfFiles` | f64 | no | — |
| `textBoxListOptions` | Vec<JSON> | no | — |
| `position` | f64 | no | — |
| `model` | String — `contact`, `opportunity` | no | Model of the custom field you want to create |

### `CreateLocationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | The name for the sub-account/location |
| `phone` | String | no | The phone number of the business for which sub-account is created with the appropriate country-code |
| `companyId` | String | **yes** | Company/Agency Id |
| `address` | String | no | The address of the business for which sub-account is created |
| `city` | String | no | The city where the business is located for which sub-account is created |
| `state` | String | no | The state in which the business operates for which sub-account is created |
| `country` | String — 247 values ([shared](shared-enums.md)) | no | The 2 letter country-code in which the business is present for which sub-account is created |
| `postalCode` | String | no | The postal code of the business for which sub-account is created |
| `website` | String | no | The website of the business for which sub-account is created |
| `timezone` | String | no | The timezone of the business for which sub-account is created |
| `prospectInfo` | [`ProspectInfoDto`](#prospectinfodto) | no | — |
| `settings` | [`SettingsSchema`](#settingsschema) | no | The default settings for location |
| `social` | [`SocialSchema`](#socialschema) | no | The social media links for location |
| `twilio` | [`TwilioSchema`](#twilioschema) | no | The twilio credentials for location |
| `mailgun` | [`MailgunSchema`](#mailgunschema) | no | The mailgun credentials for location |
| `snapshotId` | String | no | The snapshot ID to be loaded into the location. |

### `CreateLocationSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Location Id |
| `companyId` | String | no | Company/Agency Id |
| `name` | String | no | The name for the sub-account/location |
| `phone` | String | no | The phone number of the business for which sub-account is created |
| `email` | String | no | The email for the sub-account/location |
| `address` | String | no | The address of the business for which sub-account is created |
| `city` | String | no | The city where the business is located for which sub-account is created |
| `state` | String | no | The state in which the business operates for which sub-account is created |
| `domain` | String | no | — |
| `country` | String — 247 values ([shared](shared-enums.md)) | no | The country in which the business is present for which sub-account is created |
| `postalCode` | String | no | The postal code of the business for which sub-account is created |
| `website` | String | no | The website of the business for which sub-account is created |
| `timezone` | String | no | The timezone of the business for which sub-account is created |
| `settings` | [`SettingsSchema`](#settingsschema) | no | The default settings for location |
| `social` | [`SocialSchema`](#socialschema) | no | The social media links for location |

### `CustomFieldDeleteSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeded` | bool | no | — |

### `CustomFieldSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `fieldKey` | String | no | — |
| `placeholder` | String | no | — |
| `dataType` | String | no | — |
| `position` | f64 | no | — |
| `picklistOptions` | Vec<String> | no | — |
| `picklistImageOptions` | Vec<String> | no | — |
| `isAllowedCustomOption` | bool | no | — |
| `isMultiFileAllowed` | bool | no | — |
| `maxFileLimit` | f64 | no | — |
| `locationId` | String | no | — |
| `model` | String — `contact`, `opportunity` | no | Model of the custom field |

### `CustomFieldSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `customField` | [`CustomFieldSchema`](#customfieldschema) | no | — |

### `CustomFieldsListSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `customFields` | Vec<CustomFieldSchema> | no | — |

### `CustomRRulesOptions`

| Field | Type | Required | Description |
|---|---|---|---|
| `intervalType` | String — `yearly`, `monthly`, `weekly`, `daily`, `hourly` | **yes** | — |
| `interval` | f64 | **yes** | — |
| `startDate` | String | **yes** | Start Date |
| `endDate` | String | no | End Date |
| `dayOfMonth` | f64 | no | 1, 2, 3, ..., 27, 31 |
| `dayOfWeek` | String — `MO`, `TU`, `WE`, `TH`, `FR`, `SA`, `SU` | no | — |
| `monthOfYear` | f64 | no | 1, 2, ....., 11, 12 |
| `count` | f64 | no | Max number of task executions |
| `createTaskIfOverDue` | bool | no | Create Task If Over Due |
| `dueAfterSeconds` | f64 | **yes** | Due after seconds |

### `CustomValueDeleteSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeded` | bool | no | — |

### `CustomValueIdSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `customValue` | [`CustomValueSchema`](#customvalueschema) | no | — |

### `CustomValueSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `fieldKey` | String | no | — |
| `value` | String | no | — |
| `locationId` | String | no | — |

### `CustomValuesListSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `customValues` | Vec<CustomValueSchema> | no | — |

### `DeleteRecurringTaskResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Recurring Task Id |
| `success` | bool | **yes** | Success |

### `EmailTemplateSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `subject` | String | no | — |
| `attachments` | Vec<Vec<JSON>> | no | — |
| `html` | String | no | — |

### `FileUploadBody`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Id(Contact Id/Opportunity Id/Custom Field Id) |
| `maxFiles` | String | no | Max number of files |

### `FileUploadResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `uploadedFiles` | JSON | no | Uploaded files |
| `meta` | Vec<String> | no | Meta data of uploaded files |

### `GetEmailTemplateResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `type` | String | no | — |
| `dateAdded` | String | no | — |
| `template` | [`EmailTemplateSchema`](#emailtemplateschema) | no | — |
| `locationId` | String | no | — |

### `GetLocationByIdSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `companyId` | String | no | — |
| `name` | String | no | — |
| `domain` | String | no | — |
| `address` | String | no | — |
| `city` | String | no | — |
| `state` | String | no | — |
| `logoUrl` | String | no | — |
| `country` | String | no | — |
| `postalCode` | String | no | — |
| `website` | String | no | — |
| `timezone` | String | no | — |
| `firstName` | String | no | — |
| `lastName` | String | no | — |
| `email` | String | no | — |
| `phone` | String | no | — |
| `business` | [`BusinessSchema`](#businessschema) | no | — |
| `social` | [`SocialSchema`](#socialschema) | no | — |
| `settings` | [`SettingsSchema`](#settingsschema) | no | — |
| `reseller` | JSON | no | — |

### `GetLocationByIdSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `location` | [`GetLocationByIdSchema`](#getlocationbyidschema) | no | — |

### `GetLocationSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Location Id |
| `name` | String | no | The name for the sub-account/location |
| `phone` | String | no | The phone number of the business for which sub-account is created |
| `email` | String | no | The email for the sub-account/location |
| `address` | String | no | The address of the business for which sub-account is created |
| `city` | String | no | The city where the business is located for which sub-account is created |
| `state` | String | no | The state in which the business operates for which sub-account is created |
| `country` | String | no | The country in which the business is present for which sub-account is created |
| `postalCode` | String | no | The postal code of the business for which sub-account is created |
| `website` | String | no | The website of the business for which sub-account is created |
| `timezone` | String | no | The timezone of the business for which sub-account is created |
| `settings` | [`SettingsSchema`](#settingsschema) | no | The default settings for location |
| `social` | [`SocialSchema`](#socialschema) | no | The social media links for location |

### `GetSmsTemplateResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `type` | String | no | — |
| `template` | [`SmsTemplateSchema`](#smstemplateschema) | no | — |
| `dateAdded` | String | no | — |
| `locationId` | String | no | — |
| `urlAttachments` | Vec<String> | no | — |

### `GetTemplatesSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `templates` | Vec<JSON> | no | — |
| `totalCount` | f64 | no | — |

### `LocationDeletedSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status of the API |
| `message` | String | **yes** | Success message of the API |

### `LocationTagDeleteSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeded` | bool | no | — |

### `LocationTagSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `tag` | [`LocationTagsSchema`](#locationtagsschema) | no | — |

### `LocationTagsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | — |
| `locationId` | String | no | — |
| `id` | String | no | — |

### `LocationTagsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `tags` | Vec<LocationTagsSchema> | no | — |

### `LocationTaskListSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `tasks` | Vec<Vec<JSON>> | no | — |

### `MailgunSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `apiKey` | String | **yes** | API key provided by Mailgun |
| `domain` | String | **yes** | Domain connected with Mailgun |

### `ProspectInfoDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `firstName` | String | **yes** | First name of the prospect |
| `lastName` | String | **yes** | Last name of the prospect |
| `email` | String | **yes** | Email of the prospect |

### `RecurringTaskCreateDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | Name of the task |
| `description` | String | no | Description of the task |
| `contactIds` | Vec<String> | no | Contact Id |
| `owners` | Vec<String> | no | Assigned To |
| `rruleOptions` | [`CustomRRulesOptions`](#customrrulesoptions) | **yes** | Recurring rules |
| `ignoreTaskCreation` | bool | no | Create initial task or not |

### `RecurringTaskResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Recurring Task Id |
| `title` | String | **yes** | Name of the task |
| `description` | String | **yes** | Description of the task |
| `locationId` | String | **yes** | Location Id |
| `updatedAt` | String | **yes** | Updated At |
| `createdAt` | String | **yes** | Created At |
| `rruleOptions` | [`CustomRRulesOptions`](#customrrulesoptions) | **yes** | Recurring rules |
| `totalOccurrence` | f64 | **yes** | Total Occurrence |
| `deleted` | bool | **yes** | Deleted |
| `assignedTo` | String | no | Assigned To |
| `contactId` | String | no | Contact Id |

### `RecurringTaskSingleResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `recurringTask` | [`RecurringTaskResponseDTO`](#recurringtaskresponsedto) | **yes** | Recurring Tasks |

### `RecurringTaskUpdateDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | no | Name of the task |
| `description` | String | no | Description of the task |
| `contactIds` | Vec<String> | no | Contact Id |
| `owners` | Vec<String> | no | Assigned To |
| `rruleOptions` | [`CustomRRulesOptions`](#customrrulesoptions) | no | Recurring rules |
| `ignoreTaskCreation` | bool | no | Create initial task or not |

### `SearchSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locations` | Vec<GetLocationSchema> | no | — |

### `SettingsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `allowDuplicateContact` | bool | no | — |
| `allowDuplicateOpportunity` | bool | no | — |
| `allowFacebookNameMerge` | bool | no | — |
| `disableContactTimezone` | bool | no | — |

### `SmsTemplateSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `body` | String | no | — |
| `attachments` | Vec<Vec<JSON>> | no | — |

### `SnapshotPutSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Snaptshot ID |
| `override` | bool | no | If you want override all conflicted assets then pass true. Default value is false. |

### `SocialSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `facebookUrl` | String | no | Facebook URL |
| `googlePlus` | String | no | Googleplus URL |
| `linkedIn` | String | no | LinkedIn URL |
| `foursquare` | String | no | Foursquare URL |
| `twitter` | String | no | Twitter URL |
| `yelp` | String | no | Yelp URL |
| `instagram` | String | no | Instagram URL |
| `youtube` | String | no | Instagram URL |
| `pinterest` | String | no | Instagram URL |
| `blogRss` | String | no | Instagram URL |
| `googlePlacesId` | String | no | Google Business Places ID |

### `TaskSearchParamsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `contactId` | Vec<String> | no | Contact Ids |
| `completed` | bool | no | Task Completed Or Pending |
| `assignedTo` | Vec<String> | no | Assigned User Ids |
| `query` | String | no | Search Value |
| `limit` | f64 | no | Limit To Api |
| `skip` | f64 | no | Number Of Tasks To Skip |
| `businessId` | String | no | Bussiness Id |

### `TwilioSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `sid` | String | **yes** | SID provided by Twilio |
| `authToken` | String | **yes** | Auth token provided by Twilio |

### `UpdateCustomFieldsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | — |
| `placeholder` | String | no | — |
| `acceptedFormat` | Vec<String> | no | — |
| `isMultipleFile` | bool | no | — |
| `maxNumberOfFiles` | f64 | no | — |
| `textBoxListOptions` | Vec<JSON> | no | — |
| `position` | f64 | no | — |
| `model` | String — `contact`, `opportunity` | no | Model of the custom field you want to update |

### `UpdateLocationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | The name for the sub-account/location |
| `phone` | String | no | The phone number of the business for which sub-account is created |
| `companyId` | String | **yes** | Company/Agency Id |
| `address` | String | no | The address of the business for which sub-account is created |
| `city` | String | no | The city where the business is located for which sub-account is created |
| `state` | String | no | The state in which the business operates for which sub-account is created |
| `country` | String — 247 values ([shared](shared-enums.md)) | no | The country in which the business is present for which sub-account is created |
| `postalCode` | String | no | The postal code of the business for which sub-account is created |
| `website` | String | no | The website of the business for which sub-account is created |
| `timezone` | String | no | The timezone of the business for which sub-account is created |
| `prospectInfo` | [`ProspectInfoDto`](#prospectinfodto) | no | — |
| `settings` | [`SettingsSchema`](#settingsschema) | no | The default settings for location |
| `social` | [`SocialSchema`](#socialschema) | no | The social media links for location |
| `twilio` | [`TwilioSchema`](#twilioschema) | no | The twilio credentials for location |
| `mailgun` | [`MailgunSchema`](#mailgunschema) | no | The mailgun credentials for location |
| `snapshot` | [`SnapshotPutSchema`](#snapshotputschema) | no | The snapshot to be updated in the location. |

### `customValuesDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | — |
| `value` | String | **yes** | — |

### `tagBody`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Tag name |

### `textBoxListOptionsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `label` | String | no | — |
| `prefillValue` | String | no | — |

## Data models — API v3

In Rust: `ghl_models::v3::locations::*` (enable the `locations` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/locations/).

### `BusinessSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | — |
| `address` | String | no | — |
| `city` | String | no | — |
| `state` | String | no | — |
| `country` | String | no | — |
| `postalCode` | String | no | — |
| `website` | String | no | — |
| `timezone` | String | no | — |
| `logoUrl` | String | no | — |

### `ConversationChannelEntrySchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `conversationProvider` | [`ConversationProviderSchema`](#conversationproviderschema) | **yes** | — |

### `ConversationChannelSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `SMS` | Vec<ConversationChannelEntrySchema> | no | List of SMS providers configured for this location |
| `Email` | Vec<ConversationChannelEntrySchema> | no | List of Email providers configured for this location |

### `ConversationProviderSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Provider ID |
| `name` | String | **yes** | Provider name |
| `type` | String — `SMS`, `Email` | **yes** | Provider type |
| `default` | bool | **yes** | Whether this is the default provider |

### `CreateCustomFieldsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | — |
| `dataType` | String | **yes** | — |
| `placeholder` | String | no | — |
| `acceptedFormat` | Vec<String> | no | — |
| `isMultipleFile` | bool | no | — |
| `maxNumberOfFiles` | f64 | no | — |
| `textBoxListOptions` | Vec<JSON> | no | — |
| `position` | f64 | no | — |
| `model` | String — `contact`, `opportunity` | no | Model of the custom field you want to create |

### `CreateLocationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | The name for the sub-account/location |
| `phone` | String | no | The phone number of the business for which sub-account is created with the appropriate country-code |
| `companyId` | String | **yes** | Company/Agency Id |
| `address` | String | no | The address of the business for which sub-account is created |
| `city` | String | no | The city where the business is located for which sub-account is created |
| `state` | String | no | The state in which the business operates for which sub-account is created |
| `country` | String — 247 values ([shared](shared-enums.md)) | no | The 2 letter country-code in which the business is present for which sub-account is created |
| `postalCode` | String | no | The postal code of the business for which sub-account is created |
| `website` | String | no | The website of the business for which sub-account is created |
| `timezone` | String | no | The timezone of the business for which sub-account is created |
| `prospectInfo` | [`ProspectInfoDto`](#prospectinfodto) | no | — |
| `settings` | [`SettingsSchema`](#settingsschema) | no | The default settings for location |
| `social` | [`SocialSchema`](#socialschema) | no | The social media links for location |
| `twilio` | [`TwilioSchema`](#twilioschema) | no | (DEPRECATED) The twilio credentials for location |
| `mailgun` | [`MailgunSchema`](#mailgunschema) | no | The mailgun credentials for location |
| `snapshotId` | String | no | The snapshot ID to be loaded into the location. |

### `CreateLocationSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Location Id |
| `companyId` | String | no | Company/Agency Id |
| `name` | String | no | The name for the sub-account/location |
| `phone` | String | no | The phone number of the business for which sub-account is created |
| `email` | String | no | The email for the sub-account/location |
| `address` | String | no | The address of the business for which sub-account is created |
| `city` | String | no | The city where the business is located for which sub-account is created |
| `state` | String | no | The state in which the business operates for which sub-account is created |
| `domain` | String | no | — |
| `country` | String — 247 values ([shared](shared-enums.md)) | no | The country in which the business is present for which sub-account is created |
| `postalCode` | String | no | The postal code of the business for which sub-account is created |
| `website` | String | no | The website of the business for which sub-account is created |
| `timezone` | String | no | The timezone of the business for which sub-account is created |
| `settings` | [`SettingsSchema`](#settingsschema) | no | The default settings for location |
| `social` | [`SocialSchema`](#socialschema) | no | The social media links for location |

### `CustomFieldDeleteSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeded` | bool | no | — |

### `CustomFieldSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `fieldKey` | String | no | — |
| `placeholder` | String | no | — |
| `dataType` | String | no | — |
| `position` | f64 | no | — |
| `picklistOptions` | Vec<String> | no | — |
| `picklistImageOptions` | Vec<String> | no | — |
| `isAllowedCustomOption` | bool | no | — |
| `isMultiFileAllowed` | bool | no | — |
| `maxFileLimit` | f64 | no | — |
| `locationId` | String | no | — |
| `model` | String — `contact`, `opportunity` | no | Model of the custom field |

### `CustomFieldSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `customField` | [`CustomFieldSchema`](#customfieldschema) | no | — |

### `CustomFieldsListSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `customFields` | Vec<CustomFieldSchema> | no | — |

### `CustomRRulesOptions`

| Field | Type | Required | Description |
|---|---|---|---|
| `intervalType` | String — `yearly`, `monthly`, `weekly`, `daily`, `hourly` | **yes** | — |
| `interval` | f64 | **yes** | — |
| `startDate` | String | **yes** | Start Date |
| `endDate` | String | no | End Date |
| `dayOfMonth` | f64 | no | 1, 2, 3, ..., 27, 31 |
| `dayOfWeek` | String — `MO`, `TU`, `WE`, `TH`, `FR`, `SA`, `SU` | no | — |
| `monthOfYear` | f64 | no | 1, 2, ....., 11, 12 |
| `count` | f64 | no | Max number of task executions |
| `createTaskIfOverDue` | bool | no | Create Task If Over Due |
| `dueAfterSeconds` | f64 | **yes** | Due after seconds |

### `CustomValueDeleteSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeded` | bool | no | — |

### `CustomValueIdSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `customValue` | [`CustomValueSchema`](#customvalueschema) | no | — |

### `CustomValueSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `fieldKey` | String | no | — |
| `value` | String | no | — |
| `locationId` | String | no | — |

### `CustomValuesListSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `customValues` | Vec<CustomValueSchema> | no | — |

### `DeleteRecurringTaskResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Recurring Task Id |
| `success` | bool | **yes** | Success |

### `EmailTemplateSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `subject` | String | no | — |
| `attachments` | Vec<Vec<JSON>> | no | — |
| `html` | String | no | — |

### `FileUploadBody`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Id(Contact Id/Opportunity Id/Custom Field Id) |
| `maxFiles` | String | no | Max number of files |

### `FileUploadResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `uploadedFiles` | JSON | no | Uploaded files |
| `meta` | Vec<String> | no | Meta data of uploaded files |

### `GetConversationChannelListSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `conversationChannel` | [`ConversationChannelSchema`](#conversationchannelschema) | **yes** | — |

### `GetEmailTemplateResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `type` | String | no | — |
| `dateAdded` | String | no | — |
| `template` | [`EmailTemplateSchema`](#emailtemplateschema) | no | — |
| `locationId` | String | no | — |

### `GetLocationByIdSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `companyId` | String | no | — |
| `name` | String | no | — |
| `domain` | String | no | — |
| `address` | String | no | — |
| `city` | String | no | — |
| `state` | String | no | — |
| `logoUrl` | String | no | — |
| `country` | String | no | — |
| `postalCode` | String | no | — |
| `website` | String | no | — |
| `timezone` | String | no | — |
| `firstName` | String | no | — |
| `lastName` | String | no | — |
| `email` | String | no | — |
| `phone` | String | no | — |
| `business` | [`BusinessSchema`](#businessschema) | no | — |
| `social` | [`SocialSchema`](#socialschema) | no | — |
| `settings` | [`SettingsSchema`](#settingsschema) | no | — |
| `reseller` | JSON | no | — |

### `GetLocationByIdSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `location` | [`GetLocationByIdSchema`](#getlocationbyidschema) | no | — |

### `GetLocationSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Location Id |
| `name` | String | no | The name for the sub-account/location |
| `phone` | String | no | The phone number of the business for which sub-account is created |
| `email` | String | no | The email for the sub-account/location |
| `address` | String | no | The address of the business for which sub-account is created |
| `city` | String | no | The city where the business is located for which sub-account is created |
| `state` | String | no | The state in which the business operates for which sub-account is created |
| `country` | String | no | The country in which the business is present for which sub-account is created |
| `postalCode` | String | no | The postal code of the business for which sub-account is created |
| `website` | String | no | The website of the business for which sub-account is created |
| `timezone` | String | no | The timezone of the business for which sub-account is created |
| `settings` | [`SettingsSchema`](#settingsschema) | no | The default settings for location |
| `social` | [`SocialSchema`](#socialschema) | no | The social media links for location |

### `GetSmsTemplateResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `type` | String | no | — |
| `template` | [`SmsTemplateSchema`](#smstemplateschema) | no | — |
| `dateAdded` | String | no | — |
| `locationId` | String | no | — |
| `urlAttachments` | Vec<String> | no | — |

### `GetTemplatesSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `templates` | Vec<JSON> | no | — |
| `totalCount` | f64 | no | — |

### `LocationDeletedSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status of the API |
| `message` | String | **yes** | Success message of the API |

### `LocationTagDeleteSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeded` | bool | no | — |

### `LocationTagSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `tag` | [`LocationTagsSchema`](#locationtagsschema) | no | — |

### `LocationTagsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | — |
| `locationId` | String | no | — |
| `id` | String | no | — |

### `LocationTagsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `tags` | Vec<LocationTagsSchema> | no | — |

### `LocationTaskListSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `tasks` | Vec<Vec<JSON>> | no | — |

### `MailgunSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `apiKey` | String | **yes** | API key provided by Mailgun |
| `domain` | String | **yes** | Domain connected with Mailgun |

### `PermissionsResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `permissions` | Vec<String (enum)> | **yes** | Enabled permission names for the sub-account |

### `ProspectInfoDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `firstName` | String | **yes** | First name of the prospect |
| `lastName` | String | **yes** | Last name of the prospect |
| `email` | String | **yes** | Email of the prospect |

### `RecurringTaskCreateDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | Name of the task |
| `description` | String | no | Description of the task |
| `contactIds` | Vec<String> | no | Contact Id |
| `owners` | Vec<String> | no | Assigned To |
| `rruleOptions` | [`CustomRRulesOptions`](#customrrulesoptions) | **yes** | Recurring rules |
| `ignoreTaskCreation` | bool | no | Create initial task or not |

### `RecurringTaskResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Recurring Task Id |
| `title` | String | **yes** | Name of the task |
| `description` | String | **yes** | Description of the task |
| `locationId` | String | **yes** | Location Id |
| `updatedAt` | String | **yes** | Updated At |
| `createdAt` | String | **yes** | Created At |
| `rruleOptions` | [`CustomRRulesOptions`](#customrrulesoptions) | **yes** | Recurring rules |
| `totalOccurrence` | f64 | **yes** | Total Occurrence |
| `deleted` | bool | **yes** | Deleted |
| `assignedTo` | String | no | Assigned To |
| `contactId` | String | no | Contact Id |

### `RecurringTaskSingleResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `recurringTask` | [`RecurringTaskResponseDTO`](#recurringtaskresponsedto) | **yes** | Recurring Tasks |

### `RecurringTaskUpdateDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | no | Name of the task |
| `description` | String | no | Description of the task |
| `contactIds` | Vec<String> | no | Contact Id |
| `owners` | Vec<String> | no | Assigned To |
| `rruleOptions` | [`CustomRRulesOptions`](#customrrulesoptions) | no | Recurring rules |
| `ignoreTaskCreation` | bool | no | Create initial task or not |

### `SearchSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locations` | Vec<GetLocationSchema> | no | — |

### `SettingsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `allowDuplicateContact` | bool | no | — |
| `allowDuplicateOpportunity` | bool | no | — |
| `allowFacebookNameMerge` | bool | no | — |
| `disableContactTimezone` | bool | no | — |

### `SmsTemplateSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `body` | String | no | — |
| `attachments` | Vec<Vec<JSON>> | no | — |

### `SnapshotPutSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Snaptshot ID |
| `override` | bool | no | If you want override all conflicted assets then pass true. Default value is false. |

### `SocialSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `facebookUrl` | String | no | Facebook URL |
| `googlePlus` | String | no | Googleplus URL |
| `linkedIn` | String | no | LinkedIn URL |
| `foursquare` | String | no | Foursquare URL |
| `twitter` | String | no | Twitter URL |
| `yelp` | String | no | Yelp URL |
| `instagram` | String | no | Instagram URL |
| `youtube` | String | no | Instagram URL |
| `pinterest` | String | no | Instagram URL |
| `blogRss` | String | no | Instagram URL |
| `googlePlacesId` | String | no | Google Business Places ID |

### `TaskSearchParamsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `contactId` | Vec<String> | no | Contact Ids |
| `completed` | bool | no | Task Completed Or Pending |
| `assignedTo` | Vec<String> | no | Assigned User Ids |
| `query` | String | no | Search Value |
| `limit` | f64 | no | Limit To Api |
| `skip` | f64 | no | Number Of Tasks To Skip |
| `businessId` | String | no | Bussiness Id |

### `TwilioSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `sid` | String | **yes** | SID provided by Twilio |
| `authToken` | String | **yes** | Auth token provided by Twilio |

### `UpdateCustomFieldsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | — |
| `placeholder` | String | no | — |
| `acceptedFormat` | Vec<String> | no | — |
| `isMultipleFile` | bool | no | — |
| `maxNumberOfFiles` | f64 | no | — |
| `textBoxListOptions` | Vec<JSON> | no | — |
| `position` | f64 | no | — |
| `model` | String — `contact`, `opportunity` | no | Model of the custom field you want to update |

### `UpdateLocationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | The name for the sub-account/location |
| `phone` | String | no | The phone number of the business for which sub-account is created |
| `companyId` | String | **yes** | Company/Agency Id |
| `address` | String | no | The address of the business for which sub-account is created |
| `city` | String | no | The city where the business is located for which sub-account is created |
| `state` | String | no | The state in which the business operates for which sub-account is created |
| `country` | String — 247 values ([shared](shared-enums.md)) | no | The country in which the business is present for which sub-account is created |
| `postalCode` | String | no | The postal code of the business for which sub-account is created |
| `website` | String | no | The website of the business for which sub-account is created |
| `timezone` | String | no | The timezone of the business for which sub-account is created |
| `prospectInfo` | [`ProspectInfoDto`](#prospectinfodto) | no | — |
| `settings` | [`SettingsSchema`](#settingsschema) | no | The default settings for location |
| `social` | [`SocialSchema`](#socialschema) | no | The social media links for location |
| `twilio` | [`TwilioSchema`](#twilioschema) | no | (DEPRECATED) The twilio credentials for location |
| `mailgun` | [`MailgunSchema`](#mailgunschema) | no | The mailgun credentials for location |
| `snapshot` | [`SnapshotPutSchema`](#snapshotputschema) | no | The snapshot to be updated in the location. |

### `UpdatePermissionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `permissions` | Vec<String (enum)> | **yes** | Permission plan values to apply for the sub-account |

### `customValuesDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | — |
| `value` | String | **yes** | — |

### `tagBody`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Tag name |

### `textBoxListOptionsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `label` | String | no | — |
| `prefillValue` | String | no | — |

