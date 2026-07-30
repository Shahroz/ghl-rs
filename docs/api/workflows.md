# `workflows`

**1** operations / **2** models in API v2 · **1** operations / **2** models in API v3

## How to call it

No hand-written service yet — reach these endpoints two ways:

**From Rust** (typed body via [`ghl-models`](https://docs.rs/ghl-models)):

```rust,ignore
// cargo add ghl-models --features workflows
use ghl_models::v2::workflows::*;

let body = serde_json::to_value(/* a Create…Dto from above */)?;
let out = ghl.request_raw("POST", "/path/", &[], Some(&body), None).await?;
```

**From an AI agent** (MCP meta-tools):

```json
{
  "name": "ghl_search_operations",
  "arguments": {
    "query": "",
    "module": "workflows"
  }
}
```

## Endpoints — API v2

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/workflows/` | Get Workflow | `workflows.get_workflows` |

### Endpoint details — v2

#### `GET /workflows/`

**Get Workflow**

Operation id: `workflows.get_workflows` · `Version: 2021-07-28` · Scopes: `workflows.readonly`

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

