# `links`

**6** operations / **6** models in API v2 · **6** operations / **6** models in API v3

## How to call it

No hand-written service yet — reach these endpoints two ways:

**From Rust** (typed body via [`ghl-models`](https://docs.rs/ghl-models)):

```rust,ignore
// cargo add ghl-models --features links
use ghl_models::v2::links::*;

let body = serde_json::to_value(/* a Create…Dto from above */)?;
let out = ghl.request_raw("POST", "/path/", &[], Some(&body), None).await?;
```

**From an AI agent** (MCP meta-tools):

```json
{
  "name": "ghl_search_operations",
  "arguments": {
    "query": "",
    "module": "links"
  }
}
```

## Endpoints — API v2

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/links/` | Get Links | `links.get_links` |
| `POST` | `/links/` | Create Link | `links.post_links` |
| `GET` | `/links/id/{linkId}` | Get Link by ID | `links.get_links_id_by_linkId` |
| `GET` | `/links/search` | Search Trigger Links | `links.get_links_search` |
| `DELETE` | `/links/{linkId}` | Delete Link | `links.delete_links_by_linkId` |
| `PUT` | `/links/{linkId}` | Update Link | `links.put_links_by_linkId` |

### Endpoint details — v2

#### `GET /links/`

**Get Links**

Operation id: `links.get_links` · `Version: 2021-07-28` · Scopes: `links.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Response*: [`GetLinksSuccessfulResponseDto`](#getlinkssuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "links.get_links",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /links/`

**Create Link**

Operation id: `links.post_links` · `Version: 2021-07-28` · Scopes: `links.write`

*Request body*: [`LinksDto`](#linksdto)

*Response*: [`GetLinkSuccessfulResponseDto`](#getlinksuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "links.post_links",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /links/id/{linkId}`

**Get Link by ID**

Get a single link by its ID

Operation id: `links.get_links_id_by_linkId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `linkId` | string | **yes** | Link Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Response*: [`GetLinkSuccessfulResponseDto`](#getlinksuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "links.get_links_id_by_linkId",
    "path_params": {
      "linkId": "<linkId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /links/search`

**Search Trigger Links**

Get list of links by searching

Operation id: `links.get_links_search` · `Version: 2021-04-15`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `query` | string | no | Search query as a string |
| `skip` | number | no | Numbers of query results to skip |
| `limit` | number | no | Limit on number of search results |

*Response*: [`GetLinksSuccessfulResponseDto`](#getlinkssuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "links.get_links_search",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `DELETE /links/{linkId}`

**Delete Link**

Operation id: `links.delete_links_by_linkId` · `Version: 2021-07-28` · Scopes: `links.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `linkId` | string | **yes** | Link Id |

*Response*: [`DeleteLinksSuccessfulResponseDto`](#deletelinkssuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "links.delete_links_by_linkId",
    "path_params": {
      "linkId": "<linkId>"
    }
  }
}
```

</details>

#### `PUT /links/{linkId}`

**Update Link**

Operation id: `links.put_links_by_linkId` · `Version: 2021-07-28` · Scopes: `links.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `linkId` | string | **yes** | Link Id |

*Request body*: [`LinkUpdateDto`](#linkupdatedto)

*Response*: [`GetLinkSuccessfulResponseDto`](#getlinksuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "links.put_links_by_linkId",
    "path_params": {
      "linkId": "<linkId>"
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
| `GET` | `/links/` | Get Links | `v3:links.get_links` |
| `POST` | `/links/` | Create Link | `v3:links.post_links` |
| `GET` | `/links/id/{linkId}` | Get Link by ID | `v3:links.get_links_id_by_linkId` |
| `GET` | `/links/search` | Search Trigger Links | `v3:links.get_links_search` |
| `DELETE` | `/links/{linkId}` | Delete Link | `v3:links.delete_links_by_linkId` |
| `PUT` | `/links/{linkId}` | Update Link | `v3:links.put_links_by_linkId` |

### Endpoint details — v3

#### `GET /links/`

**Get Links**

Operation id: `v3:links.get_links` · `Version: v3` · Scopes: `links.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID of the business profile |

*Response*: [`GetLinksSuccessfulResponseDto`](#getlinkssuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:links.get_links",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /links/`

**Create Link**

Operation id: `v3:links.post_links` · `Version: v3` · Scopes: `links.write`

*Request body*: [`LinksDto`](#linksdto)

*Response*: [`GetLinkSuccessfulResponseDto`](#getlinksuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:links.post_links",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /links/id/{linkId}`

**Get Link by ID**

Get a single link by its ID

Operation id: `v3:links.get_links_id_by_linkId` · `Version: v3` · Scopes: `links.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `linkId` | string | **yes** | Link Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Response*: [`GetLinkSuccessfulResponseDto`](#getlinksuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:links.get_links_id_by_linkId",
    "path_params": {
      "linkId": "<linkId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /links/search`

**Search Trigger Links**

Get list of links by searching

Operation id: `v3:links.get_links_search` · `Version: v3` · Scopes: `links.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `query` | string | no | Search query as a string |
| `skip` | number | no | Numbers of query results to skip |
| `limit` | number | no | Limit on number of search results |

*Response*: [`GetLinksSuccessfulResponseDto`](#getlinkssuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:links.get_links_search",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `DELETE /links/{linkId}`

**Delete Link**

Operation id: `v3:links.delete_links_by_linkId` · `Version: v3` · Scopes: `links.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `linkId` | string | **yes** | Link Id |

*Response*: [`DeleteLinksSuccessfulResponseDto`](#deletelinkssuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:links.delete_links_by_linkId",
    "path_params": {
      "linkId": "<linkId>"
    }
  }
}
```

</details>

#### `PUT /links/{linkId}`

**Update Link**

Operation id: `v3:links.put_links_by_linkId` · `Version: v3` · Scopes: `links.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `linkId` | string | **yes** | Link Id |

*Request body*: [`LinkUpdateDto`](#linkupdatedto)

*Response*: [`GetLinkSuccessfulResponseDto`](#getlinksuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:links.put_links_by_linkId",
    "path_params": {
      "linkId": "<linkId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::links::*` (enable the `links` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/links/).

### `DeleteLinksSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeded` | bool | no | — |

### `GetLinkSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `link` | [`LinkSchema`](#linkschema) | no | — |

### `GetLinksSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `links` | Vec<LinkSchema> | no | — |

### `LinkSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `redirectTo` | String | no | — |
| `fieldKey` | String | no | — |
| `locationId` | String | no | — |

### `LinkUpdateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | — |
| `redirectTo` | String | **yes** | — |

### `LinksDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | — |
| `name` | String | **yes** | — |
| `redirectTo` | String | **yes** | — |

## Data models — API v3

In Rust: `ghl_models::v3::links::*` (enable the `links` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/links/).

### `DeleteLinksSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeded` | bool | no | Indicates whether the link was successfully deleted (legacy field, misspelled). Use `succeeded` with x-api-version: v3. |
| `succeeded` | bool | no | Indicates whether the link was successfully deleted. |

### `GetLinkSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `link` | [`LinkSchema`](#linkschema) | no | The trigger link object |

### `GetLinksSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `links` | Vec<LinkSchema> | no | List of trigger links |

### `LinkSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Unique identifier of the trigger link |
| `name` | String | no | Display name of the trigger link |
| `redirectTo` | String | no | URL or variable to redirect to when the trigger link is clicked |
| `fieldKey` | String | no | Template variable key used to reference this trigger link |
| `locationId` | String | no | Location ID this trigger link belongs to |

### `LinkUpdateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Display name of the trigger link |
| `redirectTo` | String | **yes** | URL or variable to redirect to when the trigger link is clicked |

### `LinksDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID of the business profile |
| `name` | String | **yes** | Display name of the trigger link |
| `redirectTo` | String | **yes** | URL or variable to redirect to when the trigger link is clicked |

