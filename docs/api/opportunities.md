# `opportunities`

**12** operations / **25** models in API v2 · **12** operations / **26** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `opportunities` cargo feature on `ghl-sdk`, then call any of the 24 generated methods on `ghl.opportunities()` (v2) or `ghl.v3().opportunities()` (v3):

```toml
ghl-sdk = { version = "0.5", features = ["opportunities"] }
```

This module also has hand-written ergonomic helpers on the same `ghl.opportunities()`: `pipelines()`, `create()`, `get()`, `update()`, `update_status()`, `delete()`, `search()` (envelope unwrapping, paginated `Stream`s).

MCP tools: `ghl_list_pipelines`, `ghl_search_opportunities`, `ghl_get_opportunity`, `ghl_create_opportunity`, `ghl_move_opportunity`.


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `POST` | `/opportunities/` | Create Opportunity | `create_opportunity()` | `opportunities.post_opportunities` |
| `GET` | `/opportunities/lost-reason` | Get lost reason | `get_lost_reason()` | `opportunities.get_opportunities_lost_reason` |
| `GET` | `/opportunities/pipelines` | Get Pipelines | `get_pipelines()` | `opportunities.get_opportunities_pipelines` |
| `GET` | `/opportunities/search` | Search Opportunity | `search_opportunity()` | `opportunities.get_opportunities_search` |
| `POST` | `/opportunities/search` | Search Opportunities | `search_opportunities()` | `opportunities.post_opportunities_search` |
| `POST` | `/opportunities/upsert` | Upsert Opportunity | `upsert_opportunity()` | `opportunities.post_opportunities_upsert` |
| `DELETE` | `/opportunities/{id}` | Delete Opportunity | `delete_opportunity()` | `opportunities.delete_opportunities_by_id` |
| `GET` | `/opportunities/{id}` | Get Opportunity | `get_opportunity()` | `opportunities.get_opportunities_by_id` |
| `PUT` | `/opportunities/{id}` | Update Opportunity | `update_opportunity()` | `opportunities.put_opportunities_by_id` |
| `DELETE` | `/opportunities/{id}/followers` | Remove Followers | `remove_followers()` | `opportunities.delete_opportunities_by_id_followers` |
| `POST` | `/opportunities/{id}/followers` | Add Followers | `add_followers()` | `opportunities.post_opportunities_by_id_followers` |
| `PUT` | `/opportunities/{id}/status` | Update Opportunity Status | `update_opportunity_status()` | `opportunities.put_opportunities_by_id_status` |

### Endpoint details — v2

#### `POST /opportunities/`

**Create Opportunity**

Operation id: `opportunities.post_opportunities` · `Version: 2021-07-28` · Scopes: `opportunities.write`

*Request body*: [`CreateDto`](#createdto)

*Response*: [`GetPostOpportunitySuccessfulResponseDto`](#getpostopportunitysuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.opportunities().create_opportunity(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "opportunities.post_opportunities",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /opportunities/lost-reason`

**Get lost reason**

Operation id: `opportunities.get_opportunities_lost_reason` · `Version: 2021-07-28` · Scopes: `opportunities.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `name` | string | no | lost reason name |
| `deleted` | boolean | no | deleted |
| `query` | string | no | search query |
| `skip` | number | no | skip |
| `limit` | number | no | limit |
| `getCount` | boolean | no | get count |

*Response*: [`LostReasonsResponseSchema`](#lostreasonsresponseschema)

*Rust*:

```rust,ignore
use ghl_sdk::services::opportunities::GetLostReasonParams;

let params = GetLostReasonParams::new("locationId");
let out = ghl.opportunities().get_lost_reason(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "opportunities.get_opportunities_lost_reason",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /opportunities/pipelines`

**Get Pipelines**

Operation id: `opportunities.get_opportunities_pipelines` · `Version: 2021-07-28` · Scopes: `opportunities.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Response*: [`GetPipelinesSuccessfulResponseDto`](#getpipelinessuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::opportunities::GetPipelinesParams;

let params = GetPipelinesParams::new("locationId");
let out = ghl.opportunities().get_pipelines(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "opportunities.get_opportunities_pipelines",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /opportunities/search`

**Search Opportunity**

Operation id: `opportunities.get_opportunities_search` · `Version: 2021-07-28` · Scopes: `opportunities.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | — |
| `location_id` | string | **yes** | Location Id |
| `pipeline_id` | string | no | Pipeline Id |
| `pipeline_stage_id` | string | no | stage Id |
| `contact_id` | string | no | Contact Id |
| `status` | enum: `open`, `won`, `lost`, `abandoned`, `all` | no | — |
| `assigned_to` | string | no | — |
| `campaignId` | string | no | Campaign Id |
| `id` | string | no | Opportunity Id |
| `order` | string | no | — |
| `endDate` | string | no | End date |
| `startAfter` | string | no | Start After |
| `startAfterId` | string | no | Start After Id |
| `date` | string | no | Start date |
| `country` | string | no | — |
| `page` | number | no | — |
| `limit` | number | no | Limit Per Page records count. will allow maximum up to 100 and default will be 20 |
| `getTasks` | boolean | no | get Tasks in contact |
| `getNotes` | boolean | no | get Notes in contact |
| `getCalendarEvents` | boolean | no | get Calender event in contact |

*Response*: [`SearchSuccessfulResponseDto`](#searchsuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::opportunities::SearchOpportunityParams;

let params = SearchOpportunityParams::new("location_id");
let out = ghl.opportunities().search_opportunity(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "opportunities.get_opportunities_search",
    "query": {
      "location_id": "<location_id>"
    }
  }
}
```

</details>

#### `POST /opportunities/search`

**Search Opportunities**

Search Opportunities based on combinations of advanced filters. Documentation Link - https://doc.clickup.com/8631005/d/h/87cpx-424216/7bf11bc9b94f80f

Operation id: `opportunities.post_opportunities_search` · `Version: 2021-07-28` · Scopes: `opportunities.readonly`

*Request body*: [`OpportunitySearchBodyDTO`](#opportunitysearchbodydto)

*Response*: [`PostSearchSuccessfulResponseDto`](#postsearchsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.opportunities().search_opportunities(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "opportunities.post_opportunities_search",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /opportunities/upsert`

**Upsert Opportunity**

Operation id: `opportunities.post_opportunities_upsert` · `Version: 2021-07-28` · Scopes: `opportunities.write`

*Request body*: [`UpsertOpportunityDto`](#upsertopportunitydto)

*Response*: [`UpsertOpportunitySuccessfulResponseDto`](#upsertopportunitysuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.opportunities().upsert_opportunity(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "opportunities.post_opportunities_upsert",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /opportunities/{id}`

**Delete Opportunity**

Operation id: `opportunities.delete_opportunities_by_id` · `Version: 2021-07-28` · Scopes: `opportunities.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Opportunity Id |

*Response*: [`DeleteUpdateOpportunitySuccessfulResponseDto`](#deleteupdateopportunitysuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.opportunities().delete_opportunity(&id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "opportunities.delete_opportunities_by_id",
    "path_params": {
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /opportunities/{id}`

**Get Opportunity**

Operation id: `opportunities.get_opportunities_by_id` · `Version: 2021-07-28` · Scopes: `opportunities.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Opportunity Id |

*Response*: [`GetPostOpportunitySuccessfulResponseDto`](#getpostopportunitysuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.opportunities().get_opportunity(&id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "opportunities.get_opportunities_by_id",
    "path_params": {
      "id": "<id>"
    }
  }
}
```

</details>

#### `PUT /opportunities/{id}`

**Update Opportunity**

Operation id: `opportunities.put_opportunities_by_id` · `Version: 2021-07-28` · Scopes: `opportunities.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Opportunity Id |

*Request body*: [`UpdateOpportunityDto`](#updateopportunitydto)

*Response*: [`GetPostOpportunitySuccessfulResponseDto`](#getpostopportunitysuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.opportunities().update_opportunity(&id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "opportunities.put_opportunities_by_id",
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

#### `DELETE /opportunities/{id}/followers`

**Remove Followers**

Allows removal of one or all followers from an opportunity.

Operation id: `opportunities.delete_opportunities_by_id_followers` · `Version: 2021-07-28` · Scopes: `opportunities.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Opportunity Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `isRemoveAllFollowers` | boolean | no | — |

*Request body*: [`FollowersDTO`](#followersdto)

*Response*: [`DeleteFollowersSuccessfulResponseDto`](#deletefollowerssuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::opportunities::RemoveFollowersParams;

let params = RemoveFollowersParams::new();
let out = ghl.opportunities().remove_followers(&id, &params, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "opportunities.delete_opportunities_by_id_followers",
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

#### `POST /opportunities/{id}/followers`

**Add Followers**

Operation id: `opportunities.post_opportunities_by_id_followers` · `Version: 2021-07-28` · Scopes: `opportunities.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Opportunity Id |

*Request body*: [`FollowersDTO`](#followersdto)

*Response*: [`CreateAddFollowersSuccessfulResponseDto`](#createaddfollowerssuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.opportunities().add_followers(&id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "opportunities.post_opportunities_by_id_followers",
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

#### `PUT /opportunities/{id}/status`

**Update Opportunity Status**

Operation id: `opportunities.put_opportunities_by_id_status` · `Version: 2021-07-28` · Scopes: `opportunities.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Opportunity Id |

*Request body*: [`UpdateStatusDto`](#updatestatusdto)

*Response*: [`DeleteUpdateOpportunitySuccessfulResponseDto`](#deleteupdateopportunitysuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.opportunities().update_opportunity_status(&id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "opportunities.put_opportunities_by_id_status",
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

## Endpoints — API v3

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `POST` | `/opportunities/` | Create Opportunity | `create_opportunity()` | `v3:opportunities.post_opportunities` |
| `GET` | `/opportunities/lost-reason` | Get lost reason | `get_lost_reason()` | `v3:opportunities.get_opportunities_lost_reason` |
| `GET` | `/opportunities/pipelines` | Get Pipelines | `get_pipelines()` | `v3:opportunities.get_opportunities_pipelines` |
| `GET` | `/opportunities/search` | Search Opportunity | `search_opportunity()` | `v3:opportunities.get_opportunities_search` |
| `POST` | `/opportunities/search` | Search Opportunities | `search_opportunities()` | `v3:opportunities.post_opportunities_search` |
| `POST` | `/opportunities/upsert` | Upsert Opportunity | `upsert_opportunity()` | `v3:opportunities.post_opportunities_upsert` |
| `DELETE` | `/opportunities/{id}` | Delete Opportunity | `delete_opportunity()` | `v3:opportunities.delete_opportunities_by_id` |
| `GET` | `/opportunities/{id}` | Get Opportunity | `get_opportunity()` | `v3:opportunities.get_opportunities_by_id` |
| `PUT` | `/opportunities/{id}` | Update Opportunity | `update_opportunity()` | `v3:opportunities.put_opportunities_by_id` |
| `DELETE` | `/opportunities/{id}/followers` | Remove Followers | `remove_followers()` | `v3:opportunities.delete_opportunities_by_id_followers` |
| `POST` | `/opportunities/{id}/followers` | Add Followers | `add_followers()` | `v3:opportunities.post_opportunities_by_id_followers` |
| `PUT` | `/opportunities/{id}/status` | Update Opportunity Status | `update_opportunity_status()` | `v3:opportunities.put_opportunities_by_id_status` |

### Endpoint details — v3

#### `POST /opportunities/`

**Create Opportunity**

Operation id: `v3:opportunities.post_opportunities` · `Version: v3` · Scopes: `opportunities.write`

*Request body*: [`CreateDtoV3`](#createdtov3)

*Response*: [`GetPostOpportunitySuccessfulResponseDto`](#getpostopportunitysuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().opportunities().create_opportunity(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:opportunities.post_opportunities",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /opportunities/lost-reason`

**Get lost reason**

Operation id: `v3:opportunities.get_opportunities_lost_reason` · `Version: v3` · Scopes: `opportunities.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Identifier of the location (sub-account) |
| `name` | string | no | lost reason name |
| `deleted` | boolean | no | deleted |
| `query` | string | no | search query |
| `skip` | number | no | skip |
| `limit` | number | no | limit |
| `getCount` | boolean | no | get count |

*Response*: [`LostReasonsResponseSchema`](#lostreasonsresponseschema)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::opportunities::GetLostReasonParams;

let params = GetLostReasonParams::new("locationId");
let out = ghl.v3().opportunities().get_lost_reason(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:opportunities.get_opportunities_lost_reason",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /opportunities/pipelines`

**Get Pipelines**

Operation id: `v3:opportunities.get_opportunities_pipelines` · `Version: v3` · Scopes: `opportunities.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Identifier of the location (sub-account) to retrieve pipelines for |

*Response*: [`GetPipelinesSuccessfulResponseDto`](#getpipelinessuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::opportunities::GetPipelinesParams;

let params = GetPipelinesParams::new("locationId");
let out = ghl.v3().opportunities().get_pipelines(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:opportunities.get_opportunities_pipelines",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /opportunities/search`

**Search Opportunity**

Operation id: `v3:opportunities.get_opportunities_search` · `Version: v3` · Scopes: `opportunities.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `q` | string | no | Search query (max 75 characters) |
| `status` | enum: `open`, `won`, `lost`, `abandoned`, `all` | no | Filter by opportunity status |
| `campaignId` | string | no | Campaign Id |
| `id` | string | no | Opportunity Id |
| `order` | string | no | Sort order for results (e.g. added_asc, added_desc, name_asc, name_desc) |
| `endDate` | string | no | End date |
| `startAfter` | string | no | Start After |
| `startAfterId` | string | no | Start After Id |
| `date` | string | no | Start date |
| `country` | string | no | Filter by country code (ISO 3166-1 alpha-2) |
| `page` | number | no | Page number for pagination |
| `limit` | number | no | Limit Per Page records count. will allow maximum up to 100 and default will be 20 |
| `getTasks` | boolean | no | get Tasks in contact |
| `getNotes` | boolean | no | get Notes in contact |
| `getCalendarEvents` | boolean | no | get Calender event in contact |
| `locationId` | string | **yes** | Location Id |
| `pipelineId` | string | no | Pipeline Id |
| `pipelineStageId` | string | no | Stage Id |
| `contactId` | string | no | Contact Id |
| `assignedTo` | string | no | Filter by assigned user identifier |

*Response*: [`SearchSuccessfulResponseDto`](#searchsuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::opportunities::SearchOpportunityParams;

let params = SearchOpportunityParams::new("locationId");
let out = ghl.v3().opportunities().search_opportunity(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:opportunities.get_opportunities_search",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /opportunities/search`

**Search Opportunities**

Search Opportunities based on combinations of advanced filters. Documentation Link - https://doc.clickup.com/8631005/d/h/87cpx-424216/7bf11bc9b94f80f

Operation id: `v3:opportunities.post_opportunities_search` · `Version: v3` · Scopes: `opportunities.readonly`

*Request body*: [`OpportunitySearchBodyDTO`](#opportunitysearchbodydto)

*Response*: [`PostSearchSuccessfulResponseDto`](#postsearchsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().opportunities().search_opportunities(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:opportunities.post_opportunities_search",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /opportunities/upsert`

**Upsert Opportunity**

Operation id: `v3:opportunities.post_opportunities_upsert` · `Version: v3` · Scopes: `opportunities.write`

*Request body*: [`UpsertOpportunityDto`](#upsertopportunitydto)

*Response*: [`UpsertOpportunitySuccessfulResponseDto`](#upsertopportunitysuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().opportunities().upsert_opportunity(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:opportunities.post_opportunities_upsert",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /opportunities/{id}`

**Delete Opportunity**

Operation id: `v3:opportunities.delete_opportunities_by_id` · `Version: v3` · Scopes: `opportunities.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Opportunity Id |

*Response*: [`DeleteUpdateOpportunitySuccessfulResponseDto`](#deleteupdateopportunitysuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().opportunities().delete_opportunity(&id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:opportunities.delete_opportunities_by_id",
    "path_params": {
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /opportunities/{id}`

**Get Opportunity**

Operation id: `v3:opportunities.get_opportunities_by_id` · `Version: v3` · Scopes: `opportunities.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Opportunity Id |

*Response*: [`GetPostOpportunitySuccessfulResponseDto`](#getpostopportunitysuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().opportunities().get_opportunity(&id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:opportunities.get_opportunities_by_id",
    "path_params": {
      "id": "<id>"
    }
  }
}
```

</details>

#### `PUT /opportunities/{id}`

**Update Opportunity**

Operation id: `v3:opportunities.put_opportunities_by_id` · `Version: v3` · Scopes: `opportunities.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Opportunity Id |

*Request body*: [`UpdateOpportunityDtoV3`](#updateopportunitydtov3)

*Response*: [`GetPostOpportunitySuccessfulResponseDto`](#getpostopportunitysuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().opportunities().update_opportunity(&id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:opportunities.put_opportunities_by_id",
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

#### `DELETE /opportunities/{id}/followers`

**Remove Followers**

Allows removal of one or all followers from an opportunity.

Operation id: `v3:opportunities.delete_opportunities_by_id_followers` · `Version: v3` · Scopes: `opportunities.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Opportunity Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `isRemoveAllFollowers` | boolean | no | Set to true to remove all followers from the opportunity |

*Request body*: [`FollowersDTO`](#followersdto)

*Response*: [`DeleteFollowersSuccessfulResponseDto`](#deletefollowerssuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::opportunities::RemoveFollowersParams;

let params = RemoveFollowersParams::new();
let out = ghl.v3().opportunities().remove_followers(&id, &params, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:opportunities.delete_opportunities_by_id_followers",
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

#### `POST /opportunities/{id}/followers`

**Add Followers**

Operation id: `v3:opportunities.post_opportunities_by_id_followers` · `Version: v3` · Scopes: `opportunities.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Opportunity Id |

*Request body*: [`FollowersDTO`](#followersdto)

*Response*: [`CreateAddFollowersSuccessfulResponseDto`](#createaddfollowerssuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().opportunities().add_followers(&id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:opportunities.post_opportunities_by_id_followers",
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

#### `PUT /opportunities/{id}/status`

**Update Opportunity Status**

Operation id: `v3:opportunities.put_opportunities_by_id_status` · `Version: v3` · Scopes: `opportunities.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Opportunity Id |

*Request body*: [`UpdateStatusDto`](#updatestatusdto)

*Response*: [`DeleteUpdateOpportunitySuccessfulResponseDto`](#deleteupdateopportunitysuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().opportunities().update_opportunity_status(&id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:opportunities.put_opportunities_by_id_status",
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

## Data models — API v2

In Rust: `ghl_models::v2::opportunities::*` (enable the `opportunities` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/opportunities/).

### `AdditionalDetailsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `notes` | bool | **yes** | — |
| `tasks` | bool | **yes** | — |
| `calendarEvents` | bool | **yes** | — |
| `unReadConversations` | bool | **yes** | — |

### `CreateAddFollowersSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `followers` | Vec<String> | no | — |
| `followersAdded` | Vec<String> | no | — |

### `CreateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `pipelineId` | String | **yes** | pipeline Id |
| `locationId` | String | **yes** | — |
| `name` | String | **yes** | — |
| `pipelineStageId` | String | no | — |
| `status` | String — `open`, `won`, `lost`, `abandoned`, `all` | **yes** | — |
| `contactId` | String | **yes** | — |
| `monetaryValue` | f64 | no | — |
| `assignedTo` | String | no | — |
| `customFields` | Vec<JSON> | no | Add custom fields to opportunities. |

### `CustomFieldResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | — |
| `fieldValue` | JSON | **yes** | The value of the custom field |

### `DeleteFollowersSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `followers` | Vec<String> | no | — |
| `followersRemoved` | Vec<String> | no | — |

### `DeleteUpdateOpportunitySuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeded` | bool | no | — |

### `FollowersDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `followers` | Vec<String> | **yes** | — |

### `GetPipelinesSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `pipelines` | Vec<PipelinesResponseSchema> | no | — |

### `GetPostOpportunitySuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `opportunity` | [`SearchOpportunitiesResponseSchema`](#searchopportunitiesresponseschema) | no | — |

### `LostReasonResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | lost reason id |
| `name` | String | no | lost reason name |
| `locationId` | String | no | location id |
| `updatedAt` | String | no | updated at |
| `createdAt` | String | no | created at |

### `LostReasonsResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `lostReasons` | Vec<LostReasonResponseSchema> | no | — |
| `total` | f64 | no | — |

### `OpportunitySearchBodyDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location Id |
| `query` | String | **yes** | — |
| `limit` | f64 | **yes** | — |
| `page` | f64 | **yes** | — |
| `searchAfter` | Vec<String> | **yes** | — |
| `additionalDetails` | [`AdditionalDetailsDTO`](#additionaldetailsdto) | **yes** | — |

### `PipelinesResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `stages` | Vec<Vec<JSON>> | no | — |
| `showInFunnel` | bool | no | — |
| `showInPieChart` | bool | no | — |
| `locationId` | String | no | — |
| `colorRenderMode` | String — `dot`, `bg-tint`, `none` | no | How pipeline/stage colors are rendered |

### `PostSearchSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `opportunities` | Vec<SearchOpportunitiesResponseSchema> | no | — |
| `total` | f64 | **yes** | — |
| `aggregations` | JSON | no | — |

### `SearchMetaResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `total` | f64 | no | — |
| `nextPageUrl` | String | no | — |
| `startAfterId` | String | no | — |
| `startAfter` | f64 | no | — |
| `currentPage` | f64 | no | — |
| `nextPage` | f64 | no | — |
| `prevPage` | f64 | no | — |

### `SearchOpportunitiesContactResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `companyName` | String | no | — |
| `email` | String | no | — |
| `phone` | String | no | — |
| `tags` | Vec<String> | no | — |

### `SearchOpportunitiesResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `monetaryValue` | f64 | no | — |
| `pipelineId` | String | no | — |
| `pipelineStageId` | String | no | — |
| `assignedTo` | String | no | — |
| `status` | String | no | — |
| `source` | String | no | — |
| `lastStatusChangeAt` | String | no | — |
| `lastStageChangeAt` | String | no | — |
| `lastActionDate` | String | no | — |
| `indexVersion` | String | no | — |
| `createdAt` | String | no | — |
| `updatedAt` | String | no | — |
| `contactId` | String | no | — |
| `locationId` | String | no | — |
| `contact` | [`SearchOpportunitiesContactResponseSchema`](#searchopportunitiescontactresponseschema) | no | — |
| `notes` | Vec<Vec<JSON>> | no | — |
| `tasks` | Vec<Vec<JSON>> | no | — |
| `calendarEvents` | Vec<Vec<JSON>> | no | — |
| `lostReasonId` | String | no | — |
| `customFields` | Vec<CustomFieldResponseSchema> | no | — |
| `followers` | Vec<Vec<JSON>> | no | — |
| `externalObjectId` | String | no | — |

### `SearchSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `opportunities` | Vec<SearchOpportunitiesResponseSchema> | no | — |
| `meta` | [`SearchMetaResponseSchema`](#searchmetaresponseschema) | no | — |
| `aggregations` | JSON | no | — |

### `UpdateOpportunityDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `pipelineId` | String | no | pipeline Id |
| `name` | String | no | — |
| `pipelineStageId` | String | no | — |
| `status` | String — `open`, `won`, `lost`, `abandoned`, `all` | no | — |
| `monetaryValue` | f64 | no | — |
| `assignedTo` | String | no | — |
| `customFields` | Vec<JSON> | no | Update custom fields to opportunities. |

### `UpdateStatusDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | String — `open`, `won`, `lost`, `abandoned`, `all` | **yes** | — |
| `lostReasonId` | String | no | lost reason Id |

### `UpsertOpportunityDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | opportunityId |
| `pipelineId` | String | **yes** | pipeline Id |
| `locationId` | String | **yes** | locationId |
| `followers` | Vec<String> | **yes** | contactId |
| `isRemoveAllFollowers` | bool | **yes** | isRemoveAllFollowers |
| `followersActionType` | String — `add`, `remove` | **yes** | followers action type |
| `name` | String | no | name |
| `status` | String — `open`, `won`, `lost`, `abandoned`, `all` | no | — |
| `pipelineStageId` | String | no | — |
| `monetaryValue` | JSON | no | — |
| `assignedTo` | String | no | — |
| `lostReasonId` | String | no | lost reason Id |

### `UpsertOpportunitySuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `opportunity` | JSON | **yes** | Updated / New Opportunity |
| `new` | bool | **yes** | — |

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

In Rust: `ghl_models::v3::opportunities::*` (enable the `opportunities` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/opportunities/).

### `AdditionalDetailsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `notes` | bool | **yes** | Include notes in the response |
| `tasks` | bool | **yes** | Include tasks in the response |
| `calendarEvents` | bool | **yes** | Include calendar events in the response |
| `unReadConversations` | bool | **yes** | Include unread conversations count in the response |

### `CreateAddFollowersSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `followers` | Vec<String> | no | Current list of all follower user IDs after the operation |
| `followersAdded` | Vec<String> | no | User IDs that were successfully added as followers |

### `CreateDtoV3`

| Field | Type | Required | Description |
|---|---|---|---|
| `pipelineId` | String | **yes** | pipeline Id |
| `locationId` | String | **yes** | Identifier of the location (sub-account) |
| `name` | String | **yes** | Name of the opportunity |
| `pipelineStageId` | String | no | Identifier of the pipeline stage |
| `status` | String — `open`, `won`, `lost`, `abandoned`, `all` | **yes** | Current status of the opportunity |
| `contactId` | String | **yes** | Identifier of the contact linked to the opportunity |
| `monetaryValue` | f64 | no | Monetary value of the opportunity |
| `forecastExpectedCloseDate` | String | no | Expected close date. Supported formats: YYYY/MM/DD, MM/DD/YYYY, YYYY-MM-DD, MM-DD-YYYY, YYYY.MM.DD, MM.DD.YYYY, or ISO 8601 |
| `forecastProbability` | f64 | no | Forecast probability |
| `assignedTo` | String | no | Identifier of the user the opportunity is assigned to |
| `customFields` | Vec<JSON> | no | Add custom fields to opportunities. |

### `CustomFieldResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier of the custom field |
| `fieldValue` | JSON | **yes** | The value of the custom field |

### `DeleteFollowersSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `followers` | Vec<String> | no | Current list of all follower user IDs after the operation |
| `followersRemoved` | Vec<String> | no | User IDs that were successfully removed as followers |

### `DeleteUpdateOpportunitySuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeded` | bool | no | Indicates whether the operation was successful. Deprecated — use `success` instead. |
| `success` | bool | no | Indicates whether the operation was successful |

### `FollowersDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `followers` | Vec<String> | **yes** | Array of user IDs to add or remove as followers (max 10) |

### `GetPipelinesSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `pipelines` | Vec<PipelinesResponseSchema> | no | List of pipelines for the location |

### `GetPostOpportunitySuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `opportunity` | [`SearchOpportunitiesResponseSchema`](#searchopportunitiesresponseschema) | no | The created or retrieved opportunity object |

### `LostReasonResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | lost reason id |
| `name` | String | no | lost reason name |
| `locationId` | String | no | location id |
| `updatedAt` | String | no | updated at |
| `createdAt` | String | no | created at |

### `LostReasonsResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `lostReasons` | Vec<LostReasonResponseSchema> | no | List of lost reasons for the location |
| `total` | f64 | no | Total number of lost reasons matching the query |

### `OpportunitySearchBodyDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location Id |
| `query` | String | **yes** | Full-text search query string (max 75 characters) |
| `limit` | f64 | **yes** | Maximum number of results to return per page |
| `page` | f64 | **yes** | Page number (0-indexed) |
| `searchAfter` | Vec<String> | **yes** | Search-after cursor values for deep pagination |
| `additionalDetails` | [`AdditionalDetailsDTO`](#additionaldetailsdto) | **yes** | Flags to include additional related entities in the response |

### `PipelinesResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Unique identifier of the pipeline |
| `name` | String | no | Name of the pipeline |
| `stages` | Vec<Vec<JSON>> | no | Stages belonging to this pipeline |
| `showInFunnel` | bool | no | Whether the pipeline is shown in the funnel view |
| `showInPieChart` | bool | no | Whether the pipeline is shown in the pie chart view |
| `locationId` | String | no | Identifier of the location (sub-account) this pipeline belongs to |
| `useOpportunityProbability` | bool | no | Whether stage-level win probability is enabled for this pipeline |
| `colorRenderMode` | String — `dot`, `bg-tint`, `none` | no | How pipeline/stage colors are rendered |

### `PostSearchSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `opportunities` | Vec<SearchOpportunitiesResponseSchema> | no | List of opportunities matching the search criteria |
| `total` | f64 | **yes** | Total number of opportunities matching the query |
| `stageAggregations` | Vec<StageAggregationResponseDto> | no | Per-stage totals when pipeline filter is present |
| `aggregations` | JSON | no | Aggregation results keyed by aggregation name |

### `SearchMetaResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `total` | f64 | no | Total number of opportunities matching the query |
| `nextPageUrl` | String | no | URL to retrieve the next page of results |
| `startAfterId` | String | no | Cursor id to use for pagination (startAfterId param) |
| `startAfter` | f64 | no | Cursor timestamp to use for pagination (startAfter param) |
| `currentPage` | f64 | no | Current page number |
| `nextPage` | f64 | no | Next page number |
| `prevPage` | f64 | no | Previous page number |

### `SearchOpportunitiesContactResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Unique identifier of the contact |
| `name` | String | no | Full name of the contact |
| `companyName` | String | no | Company name associated with the contact |
| `email` | String | no | Email address of the contact |
| `phone` | String | no | Phone number of the contact |
| `tags` | Vec<String> | no | Tags associated with the contact |

### `SearchOpportunitiesResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Unique identifier of the opportunity |
| `name` | String | no | Name of the opportunity |
| `monetaryValue` | f64 | no | Monetary value of the opportunity |
| `pipelineId` | String | no | Identifier of the pipeline the opportunity belongs to |
| `pipelineStageId` | String | no | Identifier of the pipeline stage the opportunity is in |
| `assignedTo` | String | no | Identifier of the user the opportunity is assigned to |
| `status` | String | no | Current status of the opportunity |
| `source` | String | no | Source of the opportunity |
| `lastStatusChangeAt` | String | no | ISO 8601 timestamp of the last status change |
| `lastStageChangeAt` | String | no | ISO 8601 timestamp of the last stage change |
| `lastActionDate` | String | no | ISO 8601 timestamp of the last action on the opportunity |
| `indexVersion` | String | no | Index version of the opportunity record |
| `createdAt` | String | no | ISO 8601 timestamp when the opportunity was created |
| `updatedAt` | String | no | ISO 8601 timestamp when the opportunity was last updated |
| `forecastExpectedCloseDate` | String | no | Expected close date for the forecast (YYYY-MM-DD) |
| `forecastOriginalCloseDate` | String | no | Original forecast close date before any slippage (YYYY-MM-DD) |
| `forecastSlippageCount` | f64 | no | Number of times the close date has slipped |
| `forecastDaysSlipped` | f64 | no | Total days the close date has slipped |
| `forecastLastSlippedAt` | String | no | ISO 8601 timestamp of the last close-date slip |
| `forecastProbability` | f64 | no | Forecast win probability percentage (0–100) |
| `effectiveProbability` | f64 | no | Effective win probability after stage and forecast adjustments (0–100) |
| `contactId` | String | no | Identifier of the contact linked to the opportunity |
| `locationId` | String | no | Identifier of the location (sub-account) the opportunity belongs to |
| `contact` | [`SearchOpportunitiesContactResponseSchema`](#searchopportunitiescontactresponseschema) | no | Contact details associated with the opportunity |
| `notes` | Vec<Vec<JSON>> | no | Notes attached to the opportunity |
| `tasks` | Vec<Vec<JSON>> | no | Tasks attached to the opportunity |
| `calendarEvents` | Vec<Vec<JSON>> | no | Calendar events attached to the opportunity |
| `lostReasonId` | String | no | Identifier of the lost reason if the opportunity was marked lost |
| `customFields` | Vec<CustomFieldResponseSchema> | no | Custom fields associated with the opportunity |
| `followers` | Vec<Vec<JSON>> | no | User IDs following this opportunity |
| `externalObjectId` | String | no | External object identifier for integrations |

### `SearchSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `opportunities` | Vec<SearchOpportunitiesResponseSchema> | no | List of opportunities matching the search criteria |
| `meta` | [`SearchMetaResponseSchema`](#searchmetaresponseschema) | no | Pagination metadata for the result set |
| `aggregations` | JSON | no | Aggregation results keyed by aggregation name |

### `StageAggregationResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `pipelineStageId` | String | **yes** | Identifier of the pipeline stage being aggregated |
| `totalCount` | f64 | **yes** | Total number of opportunities in this stage |
| `totalValue` | f64 | **yes** | Total monetary value of all opportunities in this stage |
| `weightedValue` | f64 | **yes** | Probability-weighted total value of opportunities in this stage |
| `openValue` | f64 | **yes** | Total value of open opportunities in this stage |
| `openWeightedValue` | f64 | **yes** | Probability-weighted value of open opportunities in this stage |
| `wonValue` | f64 | **yes** | Total value of won opportunities in this stage |

### `UpdateOpportunityDtoV3`

| Field | Type | Required | Description |
|---|---|---|---|
| `pipelineId` | String | no | pipeline Id |
| `name` | String | no | Name of the opportunity |
| `pipelineStageId` | String | no | Identifier of the pipeline stage |
| `status` | String — `open`, `won`, `lost`, `abandoned`, `all` | no | Current status of the opportunity |
| `monetaryValue` | f64 | no | Monetary value of the opportunity |
| `forecastExpectedCloseDate` | String | no | Expected close date. Supported formats: YYYY/MM/DD, MM/DD/YYYY, YYYY-MM-DD, MM-DD-YYYY, YYYY.MM.DD, MM.DD.YYYY, or ISO 8601 |
| `forecastProbability` | f64 | no | Forecast probability |
| `assignedTo` | String | no | Identifier of the user the opportunity is assigned to |
| `customFields` | Vec<JSON> | no | Update custom fields to opportunities. |

### `UpdateStatusDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | String — `open`, `won`, `lost`, `abandoned`, `all` | **yes** | New status for the opportunity |
| `lostReasonId` | String | no | lost reason Id |

### `UpsertOpportunityDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | opportunityId |
| `pipelineId` | String | **yes** | pipeline Id |
| `locationId` | String | **yes** | locationId |
| `followers` | Vec<String> | **yes** | contactId |
| `isRemoveAllFollowers` | bool | **yes** | isRemoveAllFollowers |
| `followersActionType` | String — `add`, `remove` | **yes** | followers action type |
| `name` | String | no | name |
| `status` | String — `open`, `won`, `lost`, `abandoned`, `all` | no | Current status of the opportunity |
| `pipelineStageId` | String | no | Identifier of the pipeline stage |
| `monetaryValue` | JSON | no | Monetary value of the opportunity |
| `forecastExpectedCloseDate` | String | no | Expected close date. Supported formats: YYYY/MM/DD, MM/DD/YYYY, YYYY-MM-DD, MM-DD-YYYY, YYYY.MM.DD, MM.DD.YYYY, or ISO 8601 |
| `forecastProbability` | f64 | no | Forecast probability |
| `assignedTo` | String | no | Identifier of the user the opportunity is assigned to |
| `lostReasonId` | String | no | lost reason Id |

### `UpsertOpportunitySuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `opportunity` | JSON | **yes** | Updated / New Opportunity |
| `new` | bool | **yes** | Indicates whether the opportunity was newly created (true) or updated (false) |

### `customFieldsInputArraySchemaV3`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Pass either `id` or `key` of custom field |
| `key` | String | no | Pass either `id` or `key` of custom field |
| `fieldValue` | Vec<String> | no | Value of the custom field |

### `customFieldsInputObjectSchemaV3`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Pass either `id` or `key` of custom field |
| `key` | String | no | Pass either `id` or `key` of custom field |
| `fieldValue` | JSON | no | Value of the custom field |

### `customFieldsInputStringSchemaV3`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Pass either `id` or `key` of custom field |
| `key` | String | no | Pass either `id` or `key` of custom field |
| `fieldValue` | String | no | Value of the custom field |

