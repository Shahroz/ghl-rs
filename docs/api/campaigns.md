# `campaigns`

**1** operations / **2** models in API v2 · **1** operations / **2** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `campaigns` cargo feature on `ghl-sdk`, then call any of the 2 generated methods on `ghl.campaigns()` (v2) or `ghl.v3().campaigns()` (v3):

```toml
ghl-sdk = { version = "0.5", features = ["campaigns"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/campaigns/` | Get Campaigns | `get_campaigns()` | `campaigns.get_campaigns` |

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

*Rust*:

```rust,ignore
use ghl_sdk::services::campaigns::GetCampaignsParams;

let params = GetCampaignsParams::new("locationId");
let out = ghl.campaigns().get_campaigns(&params).await?;
```

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

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/campaigns/` | Get Campaigns | `get_campaigns()` | `v3:campaigns.get_campaigns` |

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

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::campaigns::GetCampaignsParams;

let params = GetCampaignsParams::new("locationId");
let out = ghl.v3().campaigns().get_campaigns(&params).await?;
```

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

