# `funnels`

**7** operations / **10** models in API v2 · **7** operations / **10** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `funnels` cargo feature on `ghl-sdk`, then call any of the 7 generated methods on `ghl.funnels()`:

```toml
ghl-sdk = { version = "0.4", features = ["funnels"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/funnels/funnel/list` | Fetch List of Funnels | `fetch_list_of_funnels()` | `funnels.get_funnels_funnel_list` |
| `POST` | `/funnels/lookup/redirect` | Create Redirect | `create_redirect()` | `funnels.post_funnels_lookup_redirect` |
| `GET` | `/funnels/lookup/redirect/list` | Fetch List of Redirects | `fetch_list_of_redirects()` | `funnels.get_funnels_lookup_redirect_list` |
| `DELETE` | `/funnels/lookup/redirect/{id}` | Delete Redirect By Id | `delete_redirect_by_id()` | `funnels.delete_funnels_lookup_redirect_by_id` |
| `PATCH` | `/funnels/lookup/redirect/{id}` | Update Redirect By Id | `update_redirect_by_id()` | `funnels.patch_funnels_lookup_redirect_by_id` |
| `GET` | `/funnels/page` | Fetch list of funnel pages | `fetch_list_of_funnel_pages()` | `funnels.get_funnels_page` |
| `GET` | `/funnels/page/count` | Fetch count of funnel pages | `fetch_count_of_funnel_pages()` | `funnels.get_funnels_page_count` |

### Endpoint details — v2

#### `GET /funnels/funnel/list`

**Fetch List of Funnels**

Retrieves a list of all funnels based on the given query parameters.

Operation id: `funnels.get_funnels_funnel_list`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `type` | string | no | — |
| `category` | string | no | — |
| `offset` | string | no | — |
| `limit` | string | no | — |
| `parentId` | string | no | — |
| `name` | string | no | — |

*Response*: [`FunnelListResponseDTO`](#funnellistresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::funnels::FetchListOfFunnelsParams;

let params = FetchListOfFunnelsParams::new("locationId");
let out = ghl.funnels().fetch_list_of_funnels(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "funnels.get_funnels_funnel_list",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /funnels/lookup/redirect`

**Create Redirect**

The "Create Redirect" API Allows adding a new url redirect to the system. Use this endpoint to create a url redirect with the specified details. Ensure that the required information is provided in the request payload.

Operation id: `funnels.post_funnels_lookup_redirect` · `Version: 2021-07-28` · Scopes: `funnels/redirect.write`

*Request body*: [`CreateRedirectParams`](#createredirectparams)

*Response*: [`CreateRedirectResponseDTO`](#createredirectresponsedto)

*Rust*:

```rust,ignore
let out = ghl.funnels().create_redirect(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "funnels.post_funnels_lookup_redirect",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /funnels/lookup/redirect/list`

**Fetch List of Redirects**

Retrieves a list of all URL redirects based on the given query parameters.

Operation id: `funnels.get_funnels_lookup_redirect_list` · `Version: 2021-07-28` · Scopes: `funnels/redirect.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `limit` | number | **yes** | — |
| `offset` | number | **yes** | — |
| `search` | string | no | — |

*Response*: [`RedirectListResponseDTO`](#redirectlistresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::funnels::FetchListOfRedirectsParams;

let params = FetchListOfRedirectsParams::new("locationId", "limit", "offset");
let out = ghl.funnels().fetch_list_of_redirects(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "funnels.get_funnels_lookup_redirect_list",
    "query": {
      "locationId": "<locationId>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `DELETE /funnels/lookup/redirect/{id}`

**Delete Redirect By Id**

The "Delete Redirect By Id" API Allows deletion of a URL redirect from the system using its unique identifier. Use this endpoint to delete a URL redirect with the specified ID using details provided in the request payload.

Operation id: `funnels.delete_funnels_lookup_redirect_by_id` · `Version: 2021-07-28` · Scopes: `funnels/redirect.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Response*: [`DeleteRedirectResponseDTO`](#deleteredirectresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::funnels::DeleteRedirectByIdParams;

let params = DeleteRedirectByIdParams::new("locationId");
let out = ghl.funnels().delete_redirect_by_id(&id, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "funnels.delete_funnels_lookup_redirect_by_id",
    "path_params": {
      "id": "<id>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PATCH /funnels/lookup/redirect/{id}`

**Update Redirect By Id**

The "Update Redirect By Id" API Allows updating an existing URL redirect in the system. Use this endpoint to modify a URL redirect with the specified ID using details provided in the request payload.

Operation id: `funnels.patch_funnels_lookup_redirect_by_id` · `Version: 2021-07-28` · Scopes: `funnels/redirect.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | — |

*Request body*: [`UpdateRedirectParams`](#updateredirectparams)

*Response*: [`UpdateRedirectResponseDTO`](#updateredirectresponsedto)

*Rust*:

```rust,ignore
let out = ghl.funnels().update_redirect_by_id(&id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "funnels.patch_funnels_lookup_redirect_by_id",
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

#### `GET /funnels/page`

**Fetch list of funnel pages**

Retrieves a list of all funnel pages based on the given query parameters.

Operation id: `funnels.get_funnels_page`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `funnelId` | string | **yes** | — |
| `name` | string | no | — |
| `limit` | number | **yes** | — |
| `offset` | number | **yes** | — |

*Response*: [`FunnelPageResponseDTO`](#funnelpageresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::funnels::FetchListOfFunnelPagesParams;

let params = FetchListOfFunnelPagesParams::new("locationId", "funnelId", "limit", "offset");
let out = ghl.funnels().fetch_list_of_funnel_pages(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "funnels.get_funnels_page",
    "query": {
      "locationId": "<locationId>",
      "funnelId": "<funnelId>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `GET /funnels/page/count`

**Fetch count of funnel pages**

Retrieves count of all funnel pages based on the given query parameters.

Operation id: `funnels.get_funnels_page_count`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `funnelId` | string | **yes** | — |
| `name` | string | no | — |

*Response*: [`FunnelPageCountResponseDTO`](#funnelpagecountresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::funnels::FetchCountOfFunnelPagesParams;

let params = FetchCountOfFunnelPagesParams::new("locationId", "funnelId");
let out = ghl.funnels().fetch_count_of_funnel_pages(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "funnels.get_funnels_page_count",
    "query": {
      "locationId": "<locationId>",
      "funnelId": "<funnelId>"
    }
  }
}
```

</details>

## Endpoints — API v3

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/funnels/funnel/list` | Fetch List of Funnels | `v3:funnels.get_funnels_funnel_list` |
| `POST` | `/funnels/lookup/redirect` | Create Redirect | `v3:funnels.post_funnels_lookup_redirect` |
| `GET` | `/funnels/lookup/redirect/list` | Fetch List of Redirects | `v3:funnels.get_funnels_lookup_redirect_list` |
| `DELETE` | `/funnels/lookup/redirect/{id}` | Delete Redirect By Id | `v3:funnels.delete_funnels_lookup_redirect_by_id` |
| `PATCH` | `/funnels/lookup/redirect/{id}` | Update Redirect By Id | `v3:funnels.patch_funnels_lookup_redirect_by_id` |
| `GET` | `/funnels/page` | Fetch list of funnel pages | `v3:funnels.get_funnels_page` |
| `GET` | `/funnels/page/count` | Fetch count of funnel pages | `v3:funnels.get_funnels_page_count` |

### Endpoint details — v3

#### `GET /funnels/funnel/list`

**Fetch List of Funnels**

Retrieves a list of all funnels based on the given query parameters.

Operation id: `v3:funnels.get_funnels_funnel_list` · `Version: v3`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `type` | string | no | — |
| `category` | string | no | — |
| `offset` | string | no | — |
| `limit` | string | no | — |
| `parentId` | string | no | — |
| `name` | string | no | — |

*Response*: [`FunnelListResponseDTO`](#funnellistresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:funnels.get_funnels_funnel_list",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /funnels/lookup/redirect`

**Create Redirect**

The "Create Redirect" API Allows adding a new url redirect to the system. Use this endpoint to create a url redirect with the specified details. Ensure that the required information is provided in the request payload.

Operation id: `v3:funnels.post_funnels_lookup_redirect` · `Version: v3` · Scopes: `funnels/redirect.write`

*Request body*: [`CreateRedirectParams`](#createredirectparams)

*Response*: [`CreateRedirectResponseDTO`](#createredirectresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:funnels.post_funnels_lookup_redirect",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /funnels/lookup/redirect/list`

**Fetch List of Redirects**

Retrieves a list of all URL redirects based on the given query parameters.

Operation id: `v3:funnels.get_funnels_lookup_redirect_list` · `Version: v3` · Scopes: `funnels/redirect.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `limit` | number | **yes** | — |
| `offset` | number | **yes** | — |
| `search` | string | no | — |

*Response*: [`RedirectListResponseDTO`](#redirectlistresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:funnels.get_funnels_lookup_redirect_list",
    "query": {
      "locationId": "<locationId>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `DELETE /funnels/lookup/redirect/{id}`

**Delete Redirect By Id**

The "Delete Redirect By Id" API Allows deletion of a URL redirect from the system using its unique identifier. Use this endpoint to delete a URL redirect with the specified ID using details provided in the request payload.

Operation id: `v3:funnels.delete_funnels_lookup_redirect_by_id` · `Version: v3` · Scopes: `funnels/redirect.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Response*: [`DeleteRedirectResponseDTO`](#deleteredirectresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:funnels.delete_funnels_lookup_redirect_by_id",
    "path_params": {
      "id": "<id>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PATCH /funnels/lookup/redirect/{id}`

**Update Redirect By Id**

The "Update Redirect By Id" API Allows updating an existing URL redirect in the system. Use this endpoint to modify a URL redirect with the specified ID using details provided in the request payload.

Operation id: `v3:funnels.patch_funnels_lookup_redirect_by_id` · `Version: v3` · Scopes: `funnels/redirect.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | — |

*Request body*: [`UpdateRedirectParams`](#updateredirectparams)

*Response*: [`UpdateRedirectResponseDTO`](#updateredirectresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:funnels.patch_funnels_lookup_redirect_by_id",
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

#### `GET /funnels/page`

**Fetch list of funnel pages**

Retrieves a list of all funnel pages based on the given query parameters.

Operation id: `v3:funnels.get_funnels_page` · `Version: v3`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `funnelId` | string | **yes** | — |
| `name` | string | no | — |
| `limit` | number | **yes** | — |
| `offset` | number | **yes** | — |

*Response*: [`FunnelPageResponseDTO`](#funnelpageresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:funnels.get_funnels_page",
    "query": {
      "locationId": "<locationId>",
      "funnelId": "<funnelId>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `GET /funnels/page/count`

**Fetch count of funnel pages**

Retrieves count of all funnel pages based on the given query parameters.

Operation id: `v3:funnels.get_funnels_page_count` · `Version: v3`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `funnelId` | string | **yes** | — |
| `name` | string | no | — |

*Response*: [`FunnelPageCountResponseDTO`](#funnelpagecountresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:funnels.get_funnels_page_count",
    "query": {
      "locationId": "<locationId>",
      "funnelId": "<funnelId>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::funnels::*` (enable the `funnels` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/funnels/).

### `CreateRedirectParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | — |
| `domain` | String | **yes** | — |
| `path` | String | **yes** | — |
| `target` | String | **yes** | — |
| `action` | String — `funnel`, `website`, `url`, `all` | **yes** | — |

### `CreateRedirectResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | [`RedirectResponseDTO`](#redirectresponsedto) | **yes** | Data containing details of the created redirect |

### `DeleteRedirectResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | JSON | **yes** | Status of the delete operation |

### `FunnelListResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `funnels` | JSON | **yes** | — |
| `count` | f64 | **yes** | — |
| `traceId` | String | **yes** | — |

### `FunnelPageCountResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `count` | f64 | **yes** | — |

### `FunnelPageResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | — |
| `locationId` | String | **yes** | — |
| `funnelId` | String | **yes** | — |
| `name` | String | **yes** | — |
| `stepId` | String | **yes** | — |
| `deleted` | String | **yes** | — |
| `updatedAt` | String | **yes** | — |

### `RedirectListResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | JSON | **yes** | Object containing the count of redirects and an array of redirect data |

### `RedirectResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier of the redirect |
| `locationId` | String | **yes** | Identifier of the location associated with the redirect |
| `domain` | String | **yes** | Domain where the redirect occurs |
| `path` | String | **yes** | Original path that will be redirected |
| `pathLowercase` | String | **yes** | Lowercase version of the original path |
| `type` | String | **yes** | Type of redirect (e.g., Permanent, Temporary) |
| `target` | String | **yes** | Target URL to which the original path will be redirected |
| `action` | String | **yes** | Action performed by the redirect |

### `UpdateRedirectParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `target` | String | **yes** | — |
| `action` | String — `funnel`, `website`, `url`, `all` | **yes** | — |
| `locationId` | String | **yes** | — |

### `UpdateRedirectResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | [`RedirectResponseDTO`](#redirectresponsedto) | **yes** | Data containing details of the updated redirect |

## Data models — API v3

In Rust: `ghl_models::v3::funnels::*` (enable the `funnels` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/funnels/).

### `CreateRedirectParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | — |
| `domain` | String | **yes** | — |
| `path` | String | **yes** | — |
| `target` | String | **yes** | — |
| `action` | String — `funnel`, `website`, `url`, `all` | **yes** | — |

### `CreateRedirectResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | [`RedirectResponseDTO`](#redirectresponsedto) | **yes** | Data containing details of the created redirect |

### `DeleteRedirectResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | JSON | **yes** | Status of the delete operation |

### `FunnelListResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `funnels` | JSON | **yes** | — |
| `count` | f64 | **yes** | — |
| `traceId` | String | **yes** | — |

### `FunnelPageCountResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `count` | f64 | **yes** | — |

### `FunnelPageResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | — |
| `locationId` | String | **yes** | — |
| `funnelId` | String | **yes** | — |
| `name` | String | **yes** | — |
| `stepId` | String | **yes** | — |
| `deleted` | String | **yes** | — |
| `updatedAt` | String | **yes** | — |

### `RedirectListResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | JSON | **yes** | Object containing the count of redirects and an array of redirect data |

### `RedirectResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier of the redirect |
| `locationId` | String | **yes** | Identifier of the location associated with the redirect |
| `domain` | String | **yes** | Domain where the redirect occurs |
| `path` | String | **yes** | Original path that will be redirected |
| `pathLowercase` | String | **yes** | Lowercase version of the original path |
| `type` | String | **yes** | Type of redirect (e.g., Permanent, Temporary) |
| `target` | String | **yes** | Target URL to which the original path will be redirected |
| `action` | String | **yes** | Action performed by the redirect |

### `UpdateRedirectParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `target` | String | **yes** | — |
| `action` | String — `funnel`, `website`, `url`, `all` | **yes** | — |
| `locationId` | String | **yes** | — |

### `UpdateRedirectResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | [`RedirectResponseDTO`](#redirectresponsedto) | **yes** | Data containing details of the updated redirect |

