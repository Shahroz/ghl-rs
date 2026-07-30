# `campaigns`

**1** operations / **2** models in API v2 · **1** operations / **2** models in API v3

## How to call it

No hand-written service yet — reach these endpoints two ways:

**From Rust** (typed body via [`ghl-models`](https://docs.rs/ghl-models)):

```rust,ignore
// cargo add ghl-models --features campaigns
use ghl_models::v2::campaigns::*;

let body = serde_json::to_value(/* a Create…Dto from above */)?;
let out = ghl.request_raw("POST", "/path/", &[], Some(&body), None).await?;
```

**From an AI agent** (MCP meta-tools):

```json
{
  "name": "ghl_search_operations",
  "arguments": {
    "query": "",
    "module": "campaigns"
  }
}
```

## Endpoints — API v2

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/campaigns/` | Get Campaigns | `campaigns.get_campaigns` |

### Endpoint details — v2

#### `GET /campaigns/`

**Get Campaigns**

Operation id: `campaigns.get_campaigns` · `Version: 2021-07-28` · Scopes: `campaigns.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `status` | string | no | — |

*Response*: [`CampaignsSuccessfulResponseDto`](#campaignssuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "campaigns.get_campaigns",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

## Endpoints — API v3

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/campaigns/` | Get Campaigns | `v3:campaigns.get_campaigns` |

### Endpoint details — v3

#### `GET /campaigns/`

**Get Campaigns**

Operation id: `v3:campaigns.get_campaigns` · `Version: v3` · Scopes: `campaigns.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `status` | string | no | — |

*Response*: [`CampaignsSuccessfulResponseDto`](#campaignssuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:campaigns.get_campaigns",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::campaigns::*` (enable the `campaigns` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/campaigns/).

### `CampaignsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `campaigns` | Vec<campaignsSchema> | no | — |

### `campaignsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `status` | String | no | — |
| `locationId` | String | no | — |

## Data models — API v3

In Rust: `ghl_models::v3::campaigns::*` (enable the `campaigns` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/campaigns/).

### `CampaignsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `campaigns` | Vec<campaignsSchema> | no | — |

### `campaignsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `status` | String | no | — |
| `locationId` | String | no | — |

