# `workflows`

**1** operations / **2** models in API v2 · **1** operations / **2** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `workflows` cargo feature on `ghl-sdk`, then call any of the 1 generated methods on `ghl.workflows()`:

```toml
ghl-sdk = { version = "0.4", features = ["workflows"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/workflows/` | Get Workflow | `get_workflow()` | `workflows.get_workflows` |

### Endpoint details — v2

#### `GET /workflows/`

**Get Workflow**

Operation id: `workflows.get_workflows` · `Version: 2021-07-28` · Scopes: `workflows.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Response*: [`GetWorkflowSuccessfulResponseDto`](#getworkflowsuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::workflows::GetWorkflowParams;

let params = GetWorkflowParams::new("locationId");
let out = ghl.workflows().get_workflow(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "workflows.get_workflows",
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
| `GET` | `/workflows/` | Get Workflow | `v3:workflows.get_workflows` |

### Endpoint details — v3

#### `GET /workflows/`

**Get Workflow**

Operation id: `v3:workflows.get_workflows` · `Version: v3` · Scopes: `workflows.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Response*: [`GetWorkflowSuccessfulResponseDto`](#getworkflowsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:workflows.get_workflows",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::workflows::*` (enable the `workflows` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/workflows/).

### `GetWorkflowSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `workflows` | Vec<WorkflowSchema> | no | — |

### `WorkflowSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `status` | String | no | — |
| `version` | f64 | no | — |
| `createdAt` | String | no | — |
| `updatedAt` | String | no | — |
| `locationId` | String | no | — |

## Data models — API v3

In Rust: `ghl_models::v3::workflows::*` (enable the `workflows` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/workflows/).

### `GetWorkflowSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `workflows` | Vec<WorkflowSchema> | no | — |

### `WorkflowSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `status` | String | no | — |
| `version` | f64 | no | — |
| `createdAt` | String | no | — |
| `updatedAt` | String | no | — |
| `locationId` | String | no | — |

