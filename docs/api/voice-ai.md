# `voice-ai`

**11** operations / **35** models in API v2 · **11** operations / **35** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `voice-ai` cargo feature on `ghl-sdk`, then call any of the 11 generated methods on `ghl.voice_ai()`:

```toml
ghl-sdk = { version = "0.4", features = ["voice-ai"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `POST` | `/voice-ai/actions` | Create Agent Action | `create_agent_action()` | `voice-ai.post_voice_ai_actions` |
| `DELETE` | `/voice-ai/actions/{actionId}` | Delete Agent Action | `delete_agent_action()` | `voice-ai.delete_voice_ai_actions_by_actionId` |
| `GET` | `/voice-ai/actions/{actionId}` | Get Agent Action | `get_agent_action()` | `voice-ai.get_voice_ai_actions_by_actionId` |
| `PUT` | `/voice-ai/actions/{actionId}` | Update Agent Action | `update_agent_action()` | `voice-ai.put_voice_ai_actions_by_actionId` |
| `GET` | `/voice-ai/agents` | List Agents | `list_agents()` | `voice-ai.get_voice_ai_agents` |
| `POST` | `/voice-ai/agents` | Create Agent | `create_agent()` | `voice-ai.post_voice_ai_agents` |
| `DELETE` | `/voice-ai/agents/{agentId}` | Delete Agent | `delete_agent()` | `voice-ai.delete_voice_ai_agents_by_agentId` |
| `GET` | `/voice-ai/agents/{agentId}` | Get Agent | `get_agent()` | `voice-ai.get_voice_ai_agents_by_agentId` |
| `PATCH` | `/voice-ai/agents/{agentId}` | Patch Agent | `patch_agent()` | `voice-ai.patch_voice_ai_agents_by_agentId` |
| `GET` | `/voice-ai/dashboard/call-logs` | List Call Logs | `list_call_logs()` | `voice-ai.get_voice_ai_dashboard_call_logs` |
| `GET` | `/voice-ai/dashboard/call-logs/{callId}` | Get Call Log | `get_call_log()` | `voice-ai.get_voice_ai_dashboard_call_logs_by_callId` |

### Endpoint details — v2

#### `POST /voice-ai/actions`

**Create Agent Action**

Create a new action for a voice AI agent. Actions define specific behaviors and capabilities for the agent during calls.

Operation id: `voice-ai.post_voice_ai_actions` · `Version: 2021-04-15` · Scopes: `voice-ai-agent-goals.write`

*Request body*: [`CreateSingleActionDTO`](#createsingleactiondto)

*Response*: [`CreateActionResponseDTO`](#createactionresponsedto)

*Rust*:

```rust,ignore
let out = ghl.voice_ai().create_agent_action(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "voice-ai.post_voice_ai_actions",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /voice-ai/actions/{actionId}`

**Delete Agent Action**

Delete an existing action from a voice AI agent. This permanently removes the action and its configuration.

Operation id: `voice-ai.delete_voice_ai_actions_by_actionId` · `Version: 2021-04-15` · Scopes: `voice-ai-agent-goals.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `actionId` | string | **yes** | Unique identifier for the action |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `agentId` | string | **yes** | Agent ID the action is attached to |

*Rust*:

```rust,ignore
use ghl_sdk::services::voice_ai::DeleteAgentActionParams;

let params = DeleteAgentActionParams::new("locationId", "agentId");
let out = ghl.voice_ai().delete_agent_action(&actionId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "voice-ai.delete_voice_ai_actions_by_actionId",
    "path_params": {
      "actionId": "<actionId>"
    },
    "query": {
      "locationId": "<locationId>",
      "agentId": "<agentId>"
    }
  }
}
```

</details>

#### `GET /voice-ai/actions/{actionId}`

**Get Agent Action**

Retrieve details of a specific action by its ID. Returns the action configuration including actionParameters.

Operation id: `voice-ai.get_voice_ai_actions_by_actionId` · `Version: 2021-04-15` · Scopes: `voice-ai-agent-goals.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `actionId` | string | **yes** | Unique identifier for the action |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Response*: [`GetActionResponseDTO`](#getactionresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::voice_ai::GetAgentActionParams;

let params = GetAgentActionParams::new("locationId");
let out = ghl.voice_ai().get_agent_action(&actionId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "voice-ai.get_voice_ai_actions_by_actionId",
    "path_params": {
      "actionId": "<actionId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /voice-ai/actions/{actionId}`

**Update Agent Action**

Update an existing action for a voice AI agent. Modifies the behavior and configuration of an agent action.

Operation id: `voice-ai.put_voice_ai_actions_by_actionId` · `Version: 2021-04-15` · Scopes: `voice-ai-agent-goals.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `actionId` | string | **yes** | Unique identifier for the action |

*Request body*: [`UpdateSingleActionDTO`](#updatesingleactiondto)

*Response*: [`UpdateActionResponseDTO`](#updateactionresponsedto)

*Rust*:

```rust,ignore
let out = ghl.voice_ai().update_agent_action(&actionId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "voice-ai.put_voice_ai_actions_by_actionId",
    "path_params": {
      "actionId": "<actionId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /voice-ai/agents`

**List Agents**

Retrieve a paginated list of agents for given location.

Operation id: `voice-ai.get_voice_ai_agents` · `Version: 2021-04-15` · Scopes: `voice-ai-agents.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | number | no | Page number starting from 1 |
| `pageSize` | number | no | Number of items per page |
| `locationId` | string | **yes** | Location ID |
| `query` | string | no | Query |

*Response*: [`GetAgentsResponseDTO`](#getagentsresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::voice_ai::ListAgentsParams;

let params = ListAgentsParams::new("locationId");
let out = ghl.voice_ai().list_agents(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "voice-ai.get_voice_ai_agents",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /voice-ai/agents`

**Create Agent**

Create a new voice AI agent configuration and settings

Operation id: `voice-ai.post_voice_ai_agents` · `Version: 2021-04-15` · Scopes: `voice-ai-agents.write`

*Request body*: [`AgentCreationRequestDTO`](#agentcreationrequestdto)

*Response*: [`CreateAgentResponseDTO`](#createagentresponsedto)

*Rust*:

```rust,ignore
let out = ghl.voice_ai().create_agent(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "voice-ai.post_voice_ai_agents",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /voice-ai/agents/{agentId}`

**Delete Agent**

Delete a voice AI agent and all its configurations

Operation id: `voice-ai.delete_voice_ai_agents_by_agentId` · `Version: 2021-04-15` · Scopes: `voice-ai-agents.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | Unique agent identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Rust*:

```rust,ignore
use ghl_sdk::services::voice_ai::DeleteAgentParams;

let params = DeleteAgentParams::new("locationId");
let out = ghl.voice_ai().delete_agent(&agentId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "voice-ai.delete_voice_ai_agents_by_agentId",
    "path_params": {
      "agentId": "<agentId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /voice-ai/agents/{agentId}`

**Get Agent**

Retrieve detailed configuration and settings for a specific voice AI agent

Operation id: `voice-ai.get_voice_ai_agents_by_agentId` · `Version: 2021-04-15` · Scopes: `voice-ai-agents.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | Unique agent identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Response*: [`GetAgentResponseDTO`](#getagentresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::voice_ai::GetAgentParams;

let params = GetAgentParams::new("locationId");
let out = ghl.voice_ai().get_agent(&agentId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "voice-ai.get_voice_ai_agents_by_agentId",
    "path_params": {
      "agentId": "<agentId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PATCH /voice-ai/agents/{agentId}`

**Patch Agent**

Partially update an existing voice AI agent

Operation id: `voice-ai.patch_voice_ai_agents_by_agentId` · `Version: 2021-04-15` · Scopes: `voice-ai-agents.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | Unique agent identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Request body*: [`PatchAgentDTO`](#patchagentdto)

*Response*: [`PatchAgentResponseDTO`](#patchagentresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::voice_ai::PatchAgentParams;

let params = PatchAgentParams::new("locationId");
let out = ghl.voice_ai().patch_agent(&agentId, &params, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "voice-ai.patch_voice_ai_agents_by_agentId",
    "path_params": {
      "agentId": "<agentId>"
    },
    "query": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /voice-ai/dashboard/call-logs`

**List Call Logs**

Returns call logs for Voice AI agents scoped to a location. Supports filtering by agent, contact, call type, action types, and date range (interpreted in the provided IANA timezone). Also supports sorting and 1-based pagination.

Operation id: `voice-ai.get_voice_ai_dashboard_call_logs` · `Version: 2021-04-15` · Scopes: `voice-ai-dashboard.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier. Filters results to this location. |
| `agentId` | string | no | Agent identifier. When provided, returns logs for this agent only. |
| `contactId` | string | no | Contact IDs (comma-separated) to filter by. |
| `callType` | enum: `LIVE`, `TRIAL` | no | Call type filter. |
| `startDate` | number | no | Start date filter (Unix timestamp). Must be less than endDate. Both startDate and endDate must be provided together. |
| `endDate` | number | no | End date filter (Unix timestamp). Must be greater than startDate. Both startDate and endDate must be provided together. |
| `actionType` | enum: `CALL_TRANSFER`, `DATA_EXTRACTION`, `IN_CALL_DATA_EXTRACTION`, `WORKFLOW_TRIGGER`, `SMS`, `APPOINTMENT_BOOKING`, `CUSTOM_ACTION`, `KNOWLEDGE_BASE` | no | Action type filter for call logs (comma-separated ACTION_TYPE values) |
| `sortBy` | enum: `duration`, `createdAt` | no | Field to sort by. Defaults to newest if omitted. |
| `sort` | enum: `ascend`, `descend` | no | Sort direction. Applies only when sortBy is provided. |
| `page` | number | no | Page number (1-based). |
| `pageSize` | number | no | Page size (max 50). |

*Response*: [`CallLogsResponseDTO`](#calllogsresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::voice_ai::ListCallLogsParams;

let params = ListCallLogsParams::new("locationId");
let out = ghl.voice_ai().list_call_logs(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "voice-ai.get_voice_ai_dashboard_call_logs",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /voice-ai/dashboard/call-logs/{callId}`

**Get Call Log**

Returns a call log by callId.

Operation id: `voice-ai.get_voice_ai_dashboard_call_logs_by_callId` · `Version: 2021-04-15` · Scopes: `voice-ai-dashboard.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `callId` | string | **yes** | Call ID |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Response*: [`CallLogDTO`](#calllogdto)

*Rust*:

```rust,ignore
use ghl_sdk::services::voice_ai::GetCallLogParams;

let params = GetCallLogParams::new("locationId");
let out = ghl.voice_ai().get_call_log(&callId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "voice-ai.get_voice_ai_dashboard_call_logs_by_callId",
    "path_params": {
      "callId": "<callId>"
    },
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
| `POST` | `/voice-ai/actions` | Create Agent Action | `v3:voice-ai.post_voice_ai_actions` |
| `DELETE` | `/voice-ai/actions/{actionId}` | Delete Agent Action | `v3:voice-ai.delete_voice_ai_actions_by_actionId` |
| `GET` | `/voice-ai/actions/{actionId}` | Get Agent Action | `v3:voice-ai.get_voice_ai_actions_by_actionId` |
| `PUT` | `/voice-ai/actions/{actionId}` | Update Agent Action | `v3:voice-ai.put_voice_ai_actions_by_actionId` |
| `GET` | `/voice-ai/agents` | List Agents | `v3:voice-ai.get_voice_ai_agents` |
| `POST` | `/voice-ai/agents` | Create Agent | `v3:voice-ai.post_voice_ai_agents` |
| `DELETE` | `/voice-ai/agents/{agentId}` | Delete Agent | `v3:voice-ai.delete_voice_ai_agents_by_agentId` |
| `GET` | `/voice-ai/agents/{agentId}` | Get Agent | `v3:voice-ai.get_voice_ai_agents_by_agentId` |
| `PATCH` | `/voice-ai/agents/{agentId}` | Patch Agent | `v3:voice-ai.patch_voice_ai_agents_by_agentId` |
| `GET` | `/voice-ai/dashboard/call-logs` | List Call Logs | `v3:voice-ai.get_voice_ai_dashboard_call_logs` |
| `GET` | `/voice-ai/dashboard/call-logs/{callId}` | Get Call Log | `v3:voice-ai.get_voice_ai_dashboard_call_logs_by_callId` |

### Endpoint details — v3

#### `POST /voice-ai/actions`

**Create Agent Action**

Create a new action for a voice AI agent. Actions define specific behaviors and capabilities for the agent during calls.

Operation id: `v3:voice-ai.post_voice_ai_actions` · `Version: v3` · Scopes: `voice-ai-agent-goals.write`

*Request body*: [`CreateSingleActionDTO`](#createsingleactiondto)

*Response*: [`CreateActionResponseDTO`](#createactionresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:voice-ai.post_voice_ai_actions",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /voice-ai/actions/{actionId}`

**Delete Agent Action**

Delete an existing action from a voice AI agent. This permanently removes the action and its configuration.

Operation id: `v3:voice-ai.delete_voice_ai_actions_by_actionId` · `Version: v3` · Scopes: `voice-ai-agent-goals.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `actionId` | string | **yes** | Unique identifier for the action |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `agentId` | string | **yes** | Agent ID the action is attached to |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:voice-ai.delete_voice_ai_actions_by_actionId",
    "path_params": {
      "actionId": "<actionId>"
    },
    "query": {
      "locationId": "<locationId>",
      "agentId": "<agentId>"
    }
  }
}
```

</details>

#### `GET /voice-ai/actions/{actionId}`

**Get Agent Action**

Retrieve details of a specific action by its ID. Returns the action configuration including actionParameters.

Operation id: `v3:voice-ai.get_voice_ai_actions_by_actionId` · `Version: v3` · Scopes: `voice-ai-agent-goals.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `actionId` | string | **yes** | Unique identifier for the action |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Response*: [`GetActionResponseDTO`](#getactionresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:voice-ai.get_voice_ai_actions_by_actionId",
    "path_params": {
      "actionId": "<actionId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /voice-ai/actions/{actionId}`

**Update Agent Action**

Update an existing action for a voice AI agent. Modifies the behavior and configuration of an agent action.

Operation id: `v3:voice-ai.put_voice_ai_actions_by_actionId` · `Version: v3` · Scopes: `voice-ai-agent-goals.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `actionId` | string | **yes** | Unique identifier for the action |

*Request body*: [`UpdateSingleActionDTO`](#updatesingleactiondto)

*Response*: [`UpdateActionResponseDTO`](#updateactionresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:voice-ai.put_voice_ai_actions_by_actionId",
    "path_params": {
      "actionId": "<actionId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /voice-ai/agents`

**List Agents**

Retrieve a paginated list of agents for given location.

Operation id: `v3:voice-ai.get_voice_ai_agents` · `Version: v3` · Scopes: `voice-ai-agents.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | number | no | Page number starting from 1 |
| `pageSize` | number | no | Number of items per page |
| `locationId` | string | **yes** | Location ID |
| `query` | string | no | Query |

*Response*: [`GetAgentsResponseDTO`](#getagentsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:voice-ai.get_voice_ai_agents",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /voice-ai/agents`

**Create Agent**

Create a new voice AI agent configuration and settings

Operation id: `v3:voice-ai.post_voice_ai_agents` · `Version: v3` · Scopes: `voice-ai-agents.write`

*Request body*: [`AgentCreationRequestDTO`](#agentcreationrequestdto)

*Response*: [`CreateAgentResponseDTO`](#createagentresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:voice-ai.post_voice_ai_agents",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /voice-ai/agents/{agentId}`

**Delete Agent**

Delete a voice AI agent and all its configurations

Operation id: `v3:voice-ai.delete_voice_ai_agents_by_agentId` · `Version: v3` · Scopes: `voice-ai-agents.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | Unique agent identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:voice-ai.delete_voice_ai_agents_by_agentId",
    "path_params": {
      "agentId": "<agentId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /voice-ai/agents/{agentId}`

**Get Agent**

Retrieve detailed configuration and settings for a specific voice AI agent

Operation id: `v3:voice-ai.get_voice_ai_agents_by_agentId` · `Version: v3` · Scopes: `voice-ai-agents.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | Unique agent identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Response*: [`GetAgentResponseDTO`](#getagentresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:voice-ai.get_voice_ai_agents_by_agentId",
    "path_params": {
      "agentId": "<agentId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PATCH /voice-ai/agents/{agentId}`

**Patch Agent**

Partially update an existing voice AI agent

Operation id: `v3:voice-ai.patch_voice_ai_agents_by_agentId` · `Version: v3` · Scopes: `voice-ai-agents.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | Unique agent identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Request body*: [`PatchAgentDTO`](#patchagentdto)

*Response*: [`PatchAgentResponseDTO`](#patchagentresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:voice-ai.patch_voice_ai_agents_by_agentId",
    "path_params": {
      "agentId": "<agentId>"
    },
    "query": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /voice-ai/dashboard/call-logs`

**List Call Logs**

Returns call logs for Voice AI agents scoped to a location. Supports filtering by agent, contact, call type, action types, and date range (interpreted in the provided IANA timezone). Also supports sorting and 1-based pagination.

Operation id: `v3:voice-ai.get_voice_ai_dashboard_call_logs` · `Version: v3` · Scopes: `voice-ai-dashboard.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier. Filters results to this location. |
| `agentId` | string | no | Agent identifier. When provided, returns logs for this agent only. |
| `contactId` | string | no | Contact IDs (comma-separated) to filter by. |
| `callType` | enum: `LIVE`, `TRIAL` | no | Call type filter. |
| `startDate` | number | no | Start date filter (Unix timestamp). Must be less than endDate. Both startDate and endDate must be provided together. |
| `endDate` | number | no | End date filter (Unix timestamp). Must be greater than startDate. Both startDate and endDate must be provided together. |
| `actionType` | enum: `CALL_TRANSFER`, `DATA_EXTRACTION`, `IN_CALL_DATA_EXTRACTION`, `WORKFLOW_TRIGGER`, `SMS`, `APPOINTMENT_BOOKING`, `CUSTOM_ACTION`, `KNOWLEDGE_BASE` | no | Action type filter for call logs (comma-separated ACTION_TYPE values) |
| `sortBy` | enum: `duration`, `createdAt` | no | Field to sort by. Defaults to newest if omitted. |
| `sort` | enum: `ascend`, `descend` | no | Sort direction. Applies only when sortBy is provided. |
| `page` | number | no | Page number (1-based). |
| `pageSize` | number | no | Page size (max 50). |

*Response*: [`CallLogsResponseDTO`](#calllogsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:voice-ai.get_voice_ai_dashboard_call_logs",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /voice-ai/dashboard/call-logs/{callId}`

**Get Call Log**

Returns a call log by callId.

Operation id: `v3:voice-ai.get_voice_ai_dashboard_call_logs_by_callId` · `Version: v3` · Scopes: `voice-ai-dashboard.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `callId` | string | **yes** | Call ID |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Response*: [`CallLogDTO`](#calllogdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:voice-ai.get_voice_ai_dashboard_call_logs_by_callId",
    "path_params": {
      "callId": "<callId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::voice_ai::*` (enable the `voice-ai` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/voice_ai/).

### `AgentActionResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for this action |
| `actionType` | String — `CALL_TRANSFER`, `DATA_EXTRACTION`, `IN_CALL_DATA_EXTRACTION`, `WORKFLOW_TRIGGER`, `SMS`, `APPOINTMENT_BOOKING`, `CUSTOM_ACTION`, `KNOWLEDGE_BASE` | **yes** | Type of action |
| `name` | String | **yes** | Human-readable name for this action |
| `actionParameters` | JSON | **yes** | Action parameters - structure varies by actionType |

### `AgentCreationRequestDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Unique identifier for the location where this agent will operate |
| `agentName` | String | no | Display name for the voice AI agent, between 1-40 characters. Default: "My Agent {random 3 digit number}" |
| `businessName` | String | no | Name of the business this agent represents. Default: Uses location name |
| `welcomeMessage` | String | no | Initial greeting spoken when the agent answers calls. Default: Auto generated |
| `agentPrompt` | String | no | Custom instructions defining the agent's behavior and personality. Default: Basic prompt generated automatically |
| `voiceId` | String | no | Identifier for the speech synthesis voice from available voice options. Default: Auto generated |
| `language` | [`VoiceAILanguage`](#voiceailanguage) | no | — |
| `patienceLevel` | [`PatienceLevel`](#patiencelevel) | no | — |
| `maxCallDuration` | f64 | no | Maximum call duration in seconds, between 180-900 (3-15 minutes). Default: 300 seconds (5 minutes) |
| `sendUserIdleReminders` | bool | no | Enables automatic reminders when callers are silent. Default: true |
| `reminderAfterIdleTimeSeconds` | f64 | no | Seconds to wait before sending idle reminders, between 1-20. Default: 8 seconds |
| `inboundNumber` | String | no | Phone number for receiving inbound calls to this agent. Default: null |
| `numberPoolId` | String | no | Identifier for the number pool managing phone number allocation. Default: null |
| `callEndWorkflowIds` | Vec<String> | no | Array of workflow IDs to trigger automatically when calls end. Default: [] |
| `sendPostCallNotificationTo` | [`SendPostCallNotificationDTO`](#sendpostcallnotificationdto) | no | Configuration for post-call email notifications to various recipients. Default: [] |
| `agentWorkingHours` | Vec<AgentWorkingHoursDTO> | no | Time intervals defining when the agent accepts calls, organized by day of week. Default: [] (available 24/7) |
| `timezone` | String | no | IANA timezone identifier affecting working hours and scheduling. Default: Location timezone |
| `isAgentAsBackupDisabled` | bool | no | Prevents this agent from being used as a fallback option. Default: false (Available as backup agent) |
| `translation` | [`TranslationDTO`](#translationdto) | no | Language translation settings including enablement flag and target language code. Rules: (1) translation.enabled can only be true if the agent's language is not en-US; (2) when enabled, translation.la… |

### `AgentWorkingHoursDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `dayOfTheWeek` | String — `1`, `2`, `3`, `4`, `5`, `6`, `7` | **yes** | Day of the week for this working hours configuration (Monday=1 to Sunday=7) |
| `intervals` | Vec<IntervalDTO> | **yes** | Array of time intervals when the agent is available on this day |

### `AppointmentBookingActionParameters`

| Field | Type | Required | Description |
|---|---|---|---|
| `calendarId` | String | **yes** | Calendar ID to book appointments in |
| `daysOfOfferingDates` | f64 | **yes** | Number of days ahead to offer booking dates |
| `slotsPerDay` | f64 | **yes** | Number of available slots per day |
| `hoursBetweenSlots` | f64 | **yes** | Hours between available slots |

### `CallActionSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `actionId` | String | no | Action ID reference |
| `actionType` | String — `CALL_TRANSFER`, `DATA_EXTRACTION`, `IN_CALL_DATA_EXTRACTION`, `WORKFLOW_TRIGGER`, `SMS`, `APPOINTMENT_BOOKING`, `CUSTOM_ACTION`, `KNOWLEDGE_BASE` | **yes** | Action type |
| `actionName` | String | **yes** | Action name |
| `actionParameters` | JSON | no | Action parameters - structure varies by actionType |
| `executedAt` | String | no | When the action was executed |
| `triggerReceivedAt` | String | no | When the trigger was received |

### `CallLogDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the call |
| `contactId` | String | no | Associated contact ID |
| `agentId` | String | **yes** | Agent ID associated with the call |
| `isAgentDeleted` | bool | **yes** | Whether the agent is deleted |
| `fromNumber` | String | no | Caller phone number |
| `createdAt` | String | **yes** | Timestamp when the call was created |
| `duration` | f64 | **yes** | Call duration in seconds |
| `trialCall` | bool | **yes** | Whether this call was a trial call |
| `executedCallActions` | Vec<CallActionSchema> | **yes** | Actions performed during the call. Note: The APPOINTMENT_BOOKING action will only be visible in executedCallActions from Sep 9th 2025. |
| `summary` | String | **yes** | Call summary |
| `transcript` | String | **yes** | Call transcript |
| `translation` | [`TranslationSchema`](#translationschema) | no | Transcript translation details |
| `extractedData` | [`ExtractedDataSchema`](#extracteddataschema) | no | Dynamic data extracted from the call based on agent configuration |
| `messageId` | String | no | Message identifier associated with the call |

### `CallLogsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `total` | f64 | **yes** | Total number of items |
| `page` | f64 | **yes** | Page number starting from 1 |
| `pageSize` | f64 | **yes** | Number of items per page |
| `callLogs` | Vec<CallLogDTO> | **yes** | Array of call logs |

### `CallTransferActionParameters`

| Field | Type | Required | Description |
|---|---|---|---|
| `triggerPrompt` | String | **yes** | When to trigger this action during the call |
| `transferToType` | String — `number` | **yes** | Type of transfer destination (currently only "number" is supported) |
| `transferToValue` | String | **yes** | Phone number to transfer to. Must start with +, include country code, contain only numbers, and be 11-16 characters long (e.g., +12345678901). |
| `triggerMessage` | String | **yes** | Message to tell the caller before transferring |
| `hearWhisperMessage` | bool | no | Whether to play whisper message to the receiving party |

### `CreateActionResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the created action |
| `actionType` | String — `CALL_TRANSFER`, `DATA_EXTRACTION`, `IN_CALL_DATA_EXTRACTION`, `WORKFLOW_TRIGGER`, `SMS`, `APPOINTMENT_BOOKING`, `CUSTOM_ACTION`, `KNOWLEDGE_BASE` | **yes** | Type of action |
| `name` | String | **yes** | Human-readable name for this action |
| `actionParameters` | JSON | **yes** | Action parameters - structure varies by actionType |

### `CreateAgentResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the created agent |
| `locationId` | String | **yes** | Unique identifier for the location where this agent operates |
| `agentName` | String | **yes** | Display name of the voice AI agent |
| `businessName` | String | **yes** | Name of the business this agent represents |
| `welcomeMessage` | String | **yes** | Greeting message spoken when the agent answers calls |
| `agentPrompt` | String | **yes** | Custom instructions defining the agent's behavior |
| `voiceId` | String | **yes** | Identifier for the speech synthesis voice being used |
| `language` | String | **yes** | Language code for the agent's speech and understanding |
| `patienceLevel` | String | **yes** | Current tolerance level for caller response delays |
| `maxCallDuration` | f64 | **yes** | Maximum call duration in seconds, between 180-900 |
| `sendUserIdleReminders` | bool | **yes** | Indicates whether automatic idle reminders are enabled |
| `reminderAfterIdleTimeSeconds` | f64 | **yes** | Seconds to wait before sending idle reminders, between 1-20 |
| `inboundNumber` | String | no | Phone number for receiving inbound calls |
| `numberPoolId` | String | no | Identifier for the number pool managing this agent's phone allocation |
| `callEndWorkflowIds` | Vec<String> | no | Array of workflow IDs triggered automatically when calls end |
| `sendPostCallNotificationTo` | [`SendPostCallNotificationSchema`](#sendpostcallnotificationschema) | no | Current post-call notification settings including recipient configuration |
| `agentWorkingHours` | Vec<AgentWorkingHoursDTO> | no | Time intervals when the agent accepts calls, organized by day of week |
| `timezone` | String | **yes** | IANA timezone identifier for working hours and scheduling |
| `isAgentAsBackupDisabled` | bool | **yes** | Indicates whether this agent is excluded from backup scenarios |
| `translation` | [`TranslationSchema`](#translationschema) | no | Current language translation settings including enablement status and target language |

### `CreateSingleActionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `agentId` | String | **yes** | Agent ID to attach the action to |
| `locationId` | String | **yes** | Location ID |
| `actionType` | String — `CALL_TRANSFER`, `DATA_EXTRACTION`, `IN_CALL_DATA_EXTRACTION`, `WORKFLOW_TRIGGER`, `SMS`, `APPOINTMENT_BOOKING`, `CUSTOM_ACTION`, `KNOWLEDGE_BASE` | **yes** | Type of action |
| `name` | String | **yes** | Human-readable name for this action |
| `actionParameters` | JSON | **yes** | Action parameters - structure varies by actionType |

### `CustomActionApiDetailsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | String | **yes** | API endpoint URL |
| `method` | String — `POST`, `GET` | **yes** | HTTP method |
| `authenticationRequired` | bool | no | Whether authentication is required |
| `authenticationValue` | String | no | Authentication token or API key (required if authenticationRequired is true) |
| `headers` | Vec<CustomActionHeaderDTO> | no | HTTP headers to include |
| `parameters` | Vec<CustomActionParameterDTO> | no | API parameters to send |

### `CustomActionHeaderDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `key` | String | **yes** | HTTP header name |
| `value` | String | **yes** | HTTP header value |

### `CustomActionParameterDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Parameter name |
| `description` | String | no | Parameter description |
| `type` | String | no | Parameter type |
| `example` | String | no | Example parameter value |

### `CustomActionParameters`

| Field | Type | Required | Description |
|---|---|---|---|
| `triggerPrompt` | String | **yes** | When to call the custom API |
| `triggerMessage` | String | **yes** | Message to tell the caller |
| `apiDetails` | [`CustomActionApiDetailsDTO`](#customactionapidetailsdto) | **yes** | API endpoint configuration |
| `selectedPaths` | Vec<String> | no | Selected response paths to extract from API response. Required: at least 1 value if the method is GET. Should be empty if the method is POST. |

### `DataExtractionActionParameters`

| Field | Type | Required | Description |
|---|---|---|---|
| `contactFieldId` | String | **yes** | ID of the contact field to be updated with the extracted data |
| `description` | String | **yes** | Description of what data to extract |
| `examples` | Vec<String> | **yes** | Example values to help Agent understand the expected format. At least one example is required, maximum 5 examples allowed. |
| `overwriteExistingValue` | bool | no | Whether to overwrite existing field value if already set, default is false |

### `ExtractedDataSchema`

_No fields defined in the spec._

### `GetActionResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the action |
| `actionType` | String — `CALL_TRANSFER`, `DATA_EXTRACTION`, `IN_CALL_DATA_EXTRACTION`, `WORKFLOW_TRIGGER`, `SMS`, `APPOINTMENT_BOOKING`, `CUSTOM_ACTION`, `KNOWLEDGE_BASE` | **yes** | Type of action |
| `name` | String | **yes** | Human-readable name for this action |
| `actionParameters` | JSON | **yes** | Action parameters - structure varies by actionType |

### `GetAgentResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the created agent |
| `locationId` | String | **yes** | Unique identifier for the location where this agent operates |
| `agentName` | String | **yes** | Display name of the voice AI agent |
| `businessName` | String | **yes** | Name of the business this agent represents |
| `welcomeMessage` | String | **yes** | Greeting message spoken when the agent answers calls |
| `agentPrompt` | String | **yes** | Custom instructions defining the agent's behavior |
| `voiceId` | String | **yes** | Identifier for the speech synthesis voice being used |
| `language` | String | **yes** | Language code for the agent's speech and understanding |
| `patienceLevel` | String | **yes** | Current tolerance level for caller response delays |
| `maxCallDuration` | f64 | **yes** | Maximum call duration in seconds, between 180-900 |
| `sendUserIdleReminders` | bool | **yes** | Indicates whether automatic idle reminders are enabled |
| `reminderAfterIdleTimeSeconds` | f64 | **yes** | Seconds to wait before sending idle reminders, between 1-20 |
| `inboundNumber` | String | no | Phone number for receiving inbound calls |
| `numberPoolId` | String | no | Identifier for the number pool managing this agent's phone allocation |
| `callEndWorkflowIds` | Vec<String> | no | Array of workflow IDs triggered automatically when calls end |
| `sendPostCallNotificationTo` | [`SendPostCallNotificationSchema`](#sendpostcallnotificationschema) | no | Current post-call notification settings including recipient configuration |
| `agentWorkingHours` | Vec<AgentWorkingHoursDTO> | no | Time intervals when the agent accepts calls, organized by day of week |
| `timezone` | String | **yes** | IANA timezone identifier for working hours and scheduling |
| `isAgentAsBackupDisabled` | bool | **yes** | Indicates whether this agent is excluded from backup scenarios |
| `translation` | [`TranslationSchema`](#translationschema) | no | Current language translation settings including enablement status and target language |
| `actions` | Vec<AgentActionResponseDTO> | **yes** | Raw actions configured for this agent with complete actionParameters structure |

### `GetAgentsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `total` | f64 | **yes** | Total number of items |
| `page` | f64 | **yes** | Page number starting from 1 |
| `pageSize` | f64 | **yes** | Number of items per page |
| `agents` | Vec<GetAgentResponseDTO> | **yes** | — |

### `InCallDataExtractionActionParameters`

| Field | Type | Required | Description |
|---|---|---|---|
| `contactFieldId` | String | **yes** | ID of the contact field to be updated with the extracted data |
| `description` | String | **yes** | Description of what data to extract |
| `examples` | Vec<String> | **yes** | Example values to help Agent understand the expected format. At least one example is required, maximum 5 examples allowed. |
| `overwriteExistingValue` | bool | no | Whether to overwrite existing field value if already set, default is false |

### `IntervalDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `startHour` | f64 | **yes** | Starting hour of the working interval in 24-hour format (0-23) |
| `endHour` | f64 | **yes** | Ending hour of the working interval in 24-hour format (0-23) |
| `startMinute` | f64 | **yes** | Starting minute of the working interval (0-59) |
| `endMinute` | f64 | **yes** | Ending minute of the working interval (0-59) |

### `KnowledgeBaseParameters`

| Field | Type | Required | Description |
|---|---|---|---|
| `triggerPrompt` | String | **yes** | When to query the knowledge base |
| `knowledgeBaseId` | String | **yes** | Knowledge base ID to query |

### `PatchAgentDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `agentName` | String | no | Display name for the voice AI agent, between 1-40 characters. Default: "My Agent {random 3 digit number}" |
| `businessName` | String | no | Name of the business this agent represents. Default: Uses location name |
| `welcomeMessage` | String | no | Initial greeting spoken when the agent answers calls. Default: Auto generated |
| `agentPrompt` | String | no | Custom instructions defining the agent's behavior and personality. Default: Basic prompt generated automatically |
| `voiceId` | String | no | Identifier for the speech synthesis voice from available voice options. Default: Auto generated |
| `language` | [`VoiceAILanguage`](#voiceailanguage) | no | — |
| `patienceLevel` | [`PatienceLevel`](#patiencelevel) | no | — |
| `maxCallDuration` | f64 | no | Maximum call duration in seconds, between 180-900 (3-15 minutes). Default: 300 seconds (5 minutes) |
| `sendUserIdleReminders` | bool | no | Enables automatic reminders when callers are silent. Default: true |
| `reminderAfterIdleTimeSeconds` | f64 | no | Seconds to wait before sending idle reminders, between 1-20. Default: 8 seconds |
| `inboundNumber` | String | no | Phone number for receiving inbound calls to this agent. Default: null |
| `numberPoolId` | String | no | Identifier for the number pool managing phone number allocation. Default: null |
| `callEndWorkflowIds` | Vec<String> | no | Array of workflow IDs to trigger automatically when calls end. Default: [] |
| `sendPostCallNotificationTo` | [`SendPostCallNotificationDTO`](#sendpostcallnotificationdto) | no | Configuration for post-call email notifications to various recipients. Default: [] |
| `agentWorkingHours` | Vec<AgentWorkingHoursDTO> | no | Time intervals defining when the agent accepts calls, organized by day of week. Default: [] (available 24/7) |
| `timezone` | String | no | IANA timezone identifier affecting working hours and scheduling. Default: Location timezone |
| `isAgentAsBackupDisabled` | bool | no | Prevents this agent from being used as a fallback option. Default: false (Available as backup agent) |
| `translation` | [`TranslationDTO`](#translationdto) | no | Language translation settings including enablement flag and target language code. Rules: (1) translation.enabled can only be true if the agent's language is not en-US; (2) when enabled, translation.la… |

### `PatchAgentResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the created agent |
| `locationId` | String | **yes** | Unique identifier for the location where this agent operates |
| `agentName` | String | **yes** | Display name of the voice AI agent |
| `businessName` | String | **yes** | Name of the business this agent represents |
| `welcomeMessage` | String | **yes** | Greeting message spoken when the agent answers calls |
| `agentPrompt` | String | **yes** | Custom instructions defining the agent's behavior |
| `voiceId` | String | **yes** | Identifier for the speech synthesis voice being used |
| `language` | String | **yes** | Language code for the agent's speech and understanding |
| `patienceLevel` | String | **yes** | Current tolerance level for caller response delays |
| `maxCallDuration` | f64 | **yes** | Maximum call duration in seconds, between 180-900 |
| `sendUserIdleReminders` | bool | **yes** | Indicates whether automatic idle reminders are enabled |
| `reminderAfterIdleTimeSeconds` | f64 | **yes** | Seconds to wait before sending idle reminders, between 1-20 |
| `inboundNumber` | String | no | Phone number for receiving inbound calls |
| `numberPoolId` | String | no | Identifier for the number pool managing this agent's phone allocation |
| `callEndWorkflowIds` | Vec<String> | no | Array of workflow IDs triggered automatically when calls end |
| `sendPostCallNotificationTo` | [`SendPostCallNotificationSchema`](#sendpostcallnotificationschema) | no | Current post-call notification settings including recipient configuration |
| `agentWorkingHours` | Vec<AgentWorkingHoursDTO> | no | Time intervals when the agent accepts calls, organized by day of week |
| `timezone` | String | **yes** | IANA timezone identifier for working hours and scheduling |
| `isAgentAsBackupDisabled` | bool | **yes** | Indicates whether this agent is excluded from backup scenarios |
| `translation` | [`TranslationSchema`](#translationschema) | no | Current language translation settings including enablement status and target language |

### `PatienceLevel`

Tolerance level for caller response delays. Default: "high"

String enum. Allowed values: `low`, `medium`, `high`

### `SMSParameters`

| Field | Type | Required | Description |
|---|---|---|---|
| `triggerPrompt` | String | **yes** | When to send the SMS |
| `triggerMessage` | String | **yes** | Message to tell the caller |
| `messageBody` | String | **yes** | SMS message content to send |

### `SendPostCallNotificationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `admins` | bool | **yes** | Enables post-call notifications to all admin users in the location. Default: true |
| `allUsers` | bool | **yes** | Enables post-call notifications to all users in the location. Default: false |
| `contactAssignedUser` | bool | **yes** | Enables post-call notifications to the user assigned to the contact. Default: false |
| `specificUsers` | Vec<String> | **yes** | Array of specific user IDs to receive post-call notifications. Default: [] |
| `customEmails` | Vec<String> | **yes** | Array of custom email addresses to receive post-call notifications. Default: [] |

### `SendPostCallNotificationSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `admins` | bool | no | Send notifications to admins |
| `allUsers` | bool | no | Send notifications to all users |
| `contactAssignedUser` | bool | no | Send notifications to contact assigned user |
| `specificUsers` | Vec<String> | no | Specific user IDs to notify |
| `customEmails` | Vec<String> | no | Custom email addresses to notify |

### `TranslationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | Enables language translation for agent conversations. Default: false |
| `language` | String | no | Target language code for translation (e.g., "es" for Spanish, "fr" for French). |

### `TranslationSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | no | Whether translation is enabled |
| `language` | String | no | Translation language code |

### `UpdateActionResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the created action |
| `actionType` | String — `CALL_TRANSFER`, `DATA_EXTRACTION`, `IN_CALL_DATA_EXTRACTION`, `WORKFLOW_TRIGGER`, `SMS`, `APPOINTMENT_BOOKING`, `CUSTOM_ACTION`, `KNOWLEDGE_BASE` | **yes** | Type of action |
| `name` | String | **yes** | Human-readable name for this action |
| `actionParameters` | JSON | **yes** | Action parameters - structure varies by actionType |

### `UpdateSingleActionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `agentId` | String | **yes** | Agent ID to attach the action to |
| `locationId` | String | **yes** | Location ID |
| `actionType` | String — `CALL_TRANSFER`, `DATA_EXTRACTION`, `IN_CALL_DATA_EXTRACTION`, `WORKFLOW_TRIGGER`, `SMS`, `APPOINTMENT_BOOKING`, `CUSTOM_ACTION`, `KNOWLEDGE_BASE` | **yes** | Type of action |
| `name` | String | **yes** | Human-readable name for this action |
| `actionParameters` | JSON | **yes** | Action parameters - structure varies by actionType |

### `VoiceAILanguage`

Language code for the agent's speech and understanding. Default: "en-US"

String enum. Allowed values: `en-US`, `pt-BR`, `es`, `fr`, `de`, `it`, `nl-NL`, `multi`

### `WorkflowTriggerParameters`

| Field | Type | Required | Description |
|---|---|---|---|
| `triggerPrompt` | String | **yes** | When to trigger this workflow |
| `triggerMessage` | String | **yes** | Message to tell the caller |
| `workflowId` | String | **yes** | Workflow ID to trigger |

## Data models — API v3

In Rust: `ghl_models::v3::voice_ai::*` (enable the `voice-ai` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/voice_ai/).

### `AgentActionResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for this action |
| `actionType` | String — `CALL_TRANSFER`, `DATA_EXTRACTION`, `IN_CALL_DATA_EXTRACTION`, `WORKFLOW_TRIGGER`, `SMS`, `APPOINTMENT_BOOKING`, `CUSTOM_ACTION`, `KNOWLEDGE_BASE` | **yes** | Type of action |
| `name` | String | **yes** | Human-readable name for this action |
| `actionParameters` | JSON | **yes** | Action parameters - structure varies by actionType |

### `AgentCreationRequestDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Unique identifier for the location where this agent will operate |
| `agentName` | String | no | Display name for the voice AI agent, between 1-40 characters. Default: "My Agent {random 3 digit number}" |
| `businessName` | String | no | Name of the business this agent represents. Default: Uses location name |
| `welcomeMessage` | String | no | Initial greeting spoken when the agent answers calls. Default: Auto generated |
| `agentPrompt` | String | no | Custom instructions defining the agent's behavior and personality. Default: Basic prompt generated automatically |
| `voiceId` | String | no | Identifier for the speech synthesis voice from available voice options. Default: Auto generated |
| `language` | [`VoiceAILanguage`](#voiceailanguage) | no | — |
| `patienceLevel` | [`PatienceLevel`](#patiencelevel) | no | — |
| `maxCallDuration` | f64 | no | Maximum call duration in seconds, between 180-900 (3-15 minutes). Default: 300 seconds (5 minutes) |
| `sendUserIdleReminders` | bool | no | Enables automatic reminders when callers are silent. Default: true |
| `reminderAfterIdleTimeSeconds` | f64 | no | Seconds to wait before sending idle reminders, between 1-20. Default: 8 seconds |
| `inboundNumber` | String | no | Phone number for receiving inbound calls to this agent. Default: null |
| `numberPoolId` | String | no | Identifier for the number pool managing phone number allocation. Default: null |
| `callEndWorkflowIds` | Vec<String> | no | Array of workflow IDs to trigger automatically when calls end. Default: [] |
| `sendPostCallNotificationTo` | [`SendPostCallNotificationDTO`](#sendpostcallnotificationdto) | no | Configuration for post-call email notifications to various recipients. Default: [] |
| `agentWorkingHours` | Vec<AgentWorkingHoursDTO> | no | Time intervals defining when the agent accepts calls, organized by day of week. Default: [] (available 24/7) |
| `timezone` | String | no | IANA timezone identifier affecting working hours and scheduling. Default: Location timezone |
| `isAgentAsBackupDisabled` | bool | no | Prevents this agent from being used as a fallback option. Default: false (Available as backup agent) |
| `translation` | [`TranslationDTO`](#translationdto) | no | Language translation settings including enablement flag and target language code. Rules: (1) translation.enabled can only be true if the agent's language is not en-US; (2) when enabled, translation.la… |

### `AgentWorkingHoursDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `dayOfTheWeek` | String — `1`, `2`, `3`, `4`, `5`, `6`, `7` | **yes** | Day of the week for this working hours configuration (Monday=1 to Sunday=7) |
| `intervals` | Vec<IntervalDTO> | **yes** | Array of time intervals when the agent is available on this day |

### `AppointmentBookingActionParameters`

| Field | Type | Required | Description |
|---|---|---|---|
| `calendarId` | String | **yes** | Calendar ID to book appointments in |
| `daysOfOfferingDates` | f64 | **yes** | Number of days ahead to offer booking dates |
| `slotsPerDay` | f64 | **yes** | Number of available slots per day |
| `hoursBetweenSlots` | f64 | **yes** | Hours between available slots |

### `CallActionSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `actionId` | String | no | Action ID reference |
| `actionType` | String — `CALL_TRANSFER`, `DATA_EXTRACTION`, `IN_CALL_DATA_EXTRACTION`, `WORKFLOW_TRIGGER`, `SMS`, `APPOINTMENT_BOOKING`, `CUSTOM_ACTION`, `KNOWLEDGE_BASE` | **yes** | Action type |
| `actionName` | String | **yes** | Action name |
| `actionParameters` | JSON | no | Action parameters - structure varies by actionType |
| `executedAt` | String | no | When the action was executed |
| `triggerReceivedAt` | String | no | When the trigger was received |

### `CallLogDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the call |
| `contactId` | String | no | Associated contact ID |
| `agentId` | String | **yes** | Agent ID associated with the call |
| `isAgentDeleted` | bool | **yes** | Whether the agent is deleted |
| `fromNumber` | String | no | Caller phone number |
| `createdAt` | String | **yes** | Timestamp when the call was created |
| `duration` | f64 | **yes** | Call duration in seconds |
| `trialCall` | bool | **yes** | Whether this call was a trial call |
| `executedCallActions` | Vec<CallActionSchema> | **yes** | Actions performed during the call. Note: The APPOINTMENT_BOOKING action will only be visible in executedCallActions from Sep 9th 2025. |
| `summary` | String | **yes** | Call summary |
| `transcript` | String | **yes** | Call transcript |
| `translation` | [`TranslationSchema`](#translationschema) | no | Transcript translation details |
| `extractedData` | [`ExtractedDataSchema`](#extracteddataschema) | no | Dynamic data extracted from the call based on agent configuration |
| `messageId` | String | no | Message identifier associated with the call |

### `CallLogsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `total` | f64 | **yes** | Total number of items |
| `page` | f64 | **yes** | Page number starting from 1 |
| `pageSize` | f64 | **yes** | Number of items per page |
| `callLogs` | Vec<CallLogDTO> | **yes** | Array of call logs |

### `CallTransferActionParameters`

| Field | Type | Required | Description |
|---|---|---|---|
| `triggerPrompt` | String | **yes** | When to trigger this action during the call |
| `transferToType` | String — `number` | **yes** | Type of transfer destination (currently only "number" is supported) |
| `transferToValue` | String | **yes** | Phone number to transfer to. Must start with +, include country code, contain only numbers, and be 11-16 characters long (e.g., +12345678901). |
| `triggerMessage` | String | **yes** | Message to tell the caller before transferring |
| `hearWhisperMessage` | bool | no | Whether to play whisper message to the receiving party |

### `CreateActionResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the created action |
| `actionType` | String — `CALL_TRANSFER`, `DATA_EXTRACTION`, `IN_CALL_DATA_EXTRACTION`, `WORKFLOW_TRIGGER`, `SMS`, `APPOINTMENT_BOOKING`, `CUSTOM_ACTION`, `KNOWLEDGE_BASE` | **yes** | Type of action |
| `name` | String | **yes** | Human-readable name for this action |
| `actionParameters` | JSON | **yes** | Action parameters - structure varies by actionType |

### `CreateAgentResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the created agent |
| `locationId` | String | **yes** | Unique identifier for the location where this agent operates |
| `agentName` | String | **yes** | Display name of the voice AI agent |
| `businessName` | String | **yes** | Name of the business this agent represents |
| `welcomeMessage` | String | **yes** | Greeting message spoken when the agent answers calls |
| `agentPrompt` | String | **yes** | Custom instructions defining the agent's behavior |
| `voiceId` | String | **yes** | Identifier for the speech synthesis voice being used |
| `language` | String | **yes** | Language code for the agent's speech and understanding |
| `patienceLevel` | String | **yes** | Current tolerance level for caller response delays |
| `maxCallDuration` | f64 | **yes** | Maximum call duration in seconds, between 180-900 |
| `sendUserIdleReminders` | bool | **yes** | Indicates whether automatic idle reminders are enabled |
| `reminderAfterIdleTimeSeconds` | f64 | **yes** | Seconds to wait before sending idle reminders, between 1-20 |
| `inboundNumber` | String | no | Phone number for receiving inbound calls |
| `numberPoolId` | String | no | Identifier for the number pool managing this agent's phone allocation |
| `callEndWorkflowIds` | Vec<String> | no | Array of workflow IDs triggered automatically when calls end |
| `sendPostCallNotificationTo` | [`SendPostCallNotificationSchema`](#sendpostcallnotificationschema) | no | Current post-call notification settings including recipient configuration |
| `agentWorkingHours` | Vec<AgentWorkingHoursDTO> | no | Time intervals when the agent accepts calls, organized by day of week |
| `timezone` | String | **yes** | IANA timezone identifier for working hours and scheduling |
| `isAgentAsBackupDisabled` | bool | **yes** | Indicates whether this agent is excluded from backup scenarios |
| `translation` | [`TranslationSchema`](#translationschema) | no | Current language translation settings including enablement status and target language |

### `CreateSingleActionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `agentId` | String | **yes** | Agent ID to attach the action to |
| `locationId` | String | **yes** | Location ID |
| `actionType` | String — `CALL_TRANSFER`, `DATA_EXTRACTION`, `IN_CALL_DATA_EXTRACTION`, `WORKFLOW_TRIGGER`, `SMS`, `APPOINTMENT_BOOKING`, `CUSTOM_ACTION`, `KNOWLEDGE_BASE` | **yes** | Type of action |
| `name` | String | **yes** | Human-readable name for this action |
| `actionParameters` | JSON | **yes** | Action parameters - structure varies by actionType |

### `CustomActionApiDetailsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | String | **yes** | API endpoint URL |
| `method` | String — `POST`, `GET` | **yes** | HTTP method |
| `authenticationRequired` | bool | no | Whether authentication is required |
| `authenticationValue` | String | no | Authentication token or API key (required if authenticationRequired is true) |
| `headers` | Vec<CustomActionHeaderDTO> | no | HTTP headers to include |
| `parameters` | Vec<CustomActionParameterDTO> | no | API parameters to send |

### `CustomActionHeaderDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `key` | String | **yes** | HTTP header name |
| `value` | String | **yes** | HTTP header value |

### `CustomActionParameterDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Parameter name |
| `description` | String | no | Parameter description |
| `type` | String | no | Parameter type |
| `example` | String | no | Example parameter value |

### `CustomActionParameters`

| Field | Type | Required | Description |
|---|---|---|---|
| `triggerPrompt` | String | **yes** | When to call the custom API |
| `triggerMessage` | String | **yes** | Message to tell the caller |
| `apiDetails` | [`CustomActionApiDetailsDTO`](#customactionapidetailsdto) | **yes** | API endpoint configuration |
| `selectedPaths` | Vec<String> | no | Selected response paths to extract from API response. Required: at least 1 value if the method is GET. Should be empty if the method is POST. |

### `DataExtractionActionParameters`

| Field | Type | Required | Description |
|---|---|---|---|
| `contactFieldId` | String | **yes** | ID of the contact field to be updated with the extracted data |
| `description` | String | **yes** | Description of what data to extract |
| `examples` | Vec<String> | **yes** | Example values to help Agent understand the expected format. At least one example is required, maximum 5 examples allowed. |
| `overwriteExistingValue` | bool | no | Whether to overwrite existing field value if already set, default is false |

### `ExtractedDataSchema`

_No fields defined in the spec._

### `GetActionResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the action |
| `actionType` | String — `CALL_TRANSFER`, `DATA_EXTRACTION`, `IN_CALL_DATA_EXTRACTION`, `WORKFLOW_TRIGGER`, `SMS`, `APPOINTMENT_BOOKING`, `CUSTOM_ACTION`, `KNOWLEDGE_BASE` | **yes** | Type of action |
| `name` | String | **yes** | Human-readable name for this action |
| `actionParameters` | JSON | **yes** | Action parameters - structure varies by actionType |

### `GetAgentResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the created agent |
| `locationId` | String | **yes** | Unique identifier for the location where this agent operates |
| `agentName` | String | **yes** | Display name of the voice AI agent |
| `businessName` | String | **yes** | Name of the business this agent represents |
| `welcomeMessage` | String | **yes** | Greeting message spoken when the agent answers calls |
| `agentPrompt` | String | **yes** | Custom instructions defining the agent's behavior |
| `voiceId` | String | **yes** | Identifier for the speech synthesis voice being used |
| `language` | String | **yes** | Language code for the agent's speech and understanding |
| `patienceLevel` | String | **yes** | Current tolerance level for caller response delays |
| `maxCallDuration` | f64 | **yes** | Maximum call duration in seconds, between 180-900 |
| `sendUserIdleReminders` | bool | **yes** | Indicates whether automatic idle reminders are enabled |
| `reminderAfterIdleTimeSeconds` | f64 | **yes** | Seconds to wait before sending idle reminders, between 1-20 |
| `inboundNumber` | String | no | Phone number for receiving inbound calls |
| `numberPoolId` | String | no | Identifier for the number pool managing this agent's phone allocation |
| `callEndWorkflowIds` | Vec<String> | no | Array of workflow IDs triggered automatically when calls end |
| `sendPostCallNotificationTo` | [`SendPostCallNotificationSchema`](#sendpostcallnotificationschema) | no | Current post-call notification settings including recipient configuration |
| `agentWorkingHours` | Vec<AgentWorkingHoursDTO> | no | Time intervals when the agent accepts calls, organized by day of week |
| `timezone` | String | **yes** | IANA timezone identifier for working hours and scheduling |
| `isAgentAsBackupDisabled` | bool | **yes** | Indicates whether this agent is excluded from backup scenarios |
| `translation` | [`TranslationSchema`](#translationschema) | no | Current language translation settings including enablement status and target language |
| `actions` | Vec<AgentActionResponseDTO> | **yes** | Raw actions configured for this agent with complete actionParameters structure |

### `GetAgentsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `total` | f64 | **yes** | Total number of items |
| `page` | f64 | **yes** | Page number starting from 1 |
| `pageSize` | f64 | **yes** | Number of items per page |
| `agents` | Vec<GetAgentResponseDTO> | **yes** | — |

### `InCallDataExtractionActionParameters`

| Field | Type | Required | Description |
|---|---|---|---|
| `contactFieldId` | String | **yes** | ID of the contact field to be updated with the extracted data |
| `description` | String | **yes** | Description of what data to extract |
| `examples` | Vec<String> | **yes** | Example values to help Agent understand the expected format. At least one example is required, maximum 5 examples allowed. |
| `overwriteExistingValue` | bool | no | Whether to overwrite existing field value if already set, default is false |

### `IntervalDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `startHour` | f64 | **yes** | Starting hour of the working interval in 24-hour format (0-23) |
| `endHour` | f64 | **yes** | Ending hour of the working interval in 24-hour format (0-23) |
| `startMinute` | f64 | **yes** | Starting minute of the working interval (0-59) |
| `endMinute` | f64 | **yes** | Ending minute of the working interval (0-59) |

### `KnowledgeBaseParameters`

| Field | Type | Required | Description |
|---|---|---|---|
| `triggerPrompt` | String | **yes** | When to query the knowledge base |
| `knowledgeBaseId` | String | **yes** | Knowledge base ID to query |

### `PatchAgentDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `agentName` | String | no | Display name for the voice AI agent, between 1-40 characters. Default: "My Agent {random 3 digit number}" |
| `businessName` | String | no | Name of the business this agent represents. Default: Uses location name |
| `welcomeMessage` | String | no | Initial greeting spoken when the agent answers calls. Default: Auto generated |
| `agentPrompt` | String | no | Custom instructions defining the agent's behavior and personality. Default: Basic prompt generated automatically |
| `voiceId` | String | no | Identifier for the speech synthesis voice from available voice options. Default: Auto generated |
| `language` | [`VoiceAILanguage`](#voiceailanguage) | no | — |
| `patienceLevel` | [`PatienceLevel`](#patiencelevel) | no | — |
| `maxCallDuration` | f64 | no | Maximum call duration in seconds, between 180-900 (3-15 minutes). Default: 300 seconds (5 minutes) |
| `sendUserIdleReminders` | bool | no | Enables automatic reminders when callers are silent. Default: true |
| `reminderAfterIdleTimeSeconds` | f64 | no | Seconds to wait before sending idle reminders, between 1-20. Default: 8 seconds |
| `inboundNumber` | String | no | Phone number for receiving inbound calls to this agent. Default: null |
| `numberPoolId` | String | no | Identifier for the number pool managing phone number allocation. Default: null |
| `callEndWorkflowIds` | Vec<String> | no | Array of workflow IDs to trigger automatically when calls end. Default: [] |
| `sendPostCallNotificationTo` | [`SendPostCallNotificationDTO`](#sendpostcallnotificationdto) | no | Configuration for post-call email notifications to various recipients. Default: [] |
| `agentWorkingHours` | Vec<AgentWorkingHoursDTO> | no | Time intervals defining when the agent accepts calls, organized by day of week. Default: [] (available 24/7) |
| `timezone` | String | no | IANA timezone identifier affecting working hours and scheduling. Default: Location timezone |
| `isAgentAsBackupDisabled` | bool | no | Prevents this agent from being used as a fallback option. Default: false (Available as backup agent) |
| `translation` | [`TranslationDTO`](#translationdto) | no | Language translation settings including enablement flag and target language code. Rules: (1) translation.enabled can only be true if the agent's language is not en-US; (2) when enabled, translation.la… |

### `PatchAgentResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the created agent |
| `locationId` | String | **yes** | Unique identifier for the location where this agent operates |
| `agentName` | String | **yes** | Display name of the voice AI agent |
| `businessName` | String | **yes** | Name of the business this agent represents |
| `welcomeMessage` | String | **yes** | Greeting message spoken when the agent answers calls |
| `agentPrompt` | String | **yes** | Custom instructions defining the agent's behavior |
| `voiceId` | String | **yes** | Identifier for the speech synthesis voice being used |
| `language` | String | **yes** | Language code for the agent's speech and understanding |
| `patienceLevel` | String | **yes** | Current tolerance level for caller response delays |
| `maxCallDuration` | f64 | **yes** | Maximum call duration in seconds, between 180-900 |
| `sendUserIdleReminders` | bool | **yes** | Indicates whether automatic idle reminders are enabled |
| `reminderAfterIdleTimeSeconds` | f64 | **yes** | Seconds to wait before sending idle reminders, between 1-20 |
| `inboundNumber` | String | no | Phone number for receiving inbound calls |
| `numberPoolId` | String | no | Identifier for the number pool managing this agent's phone allocation |
| `callEndWorkflowIds` | Vec<String> | no | Array of workflow IDs triggered automatically when calls end |
| `sendPostCallNotificationTo` | [`SendPostCallNotificationSchema`](#sendpostcallnotificationschema) | no | Current post-call notification settings including recipient configuration |
| `agentWorkingHours` | Vec<AgentWorkingHoursDTO> | no | Time intervals when the agent accepts calls, organized by day of week |
| `timezone` | String | **yes** | IANA timezone identifier for working hours and scheduling |
| `isAgentAsBackupDisabled` | bool | **yes** | Indicates whether this agent is excluded from backup scenarios |
| `translation` | [`TranslationSchema`](#translationschema) | no | Current language translation settings including enablement status and target language |

### `PatienceLevel`

Tolerance level for caller response delays. Default: "high"

String enum. Allowed values: `low`, `medium`, `high`

### `SMSParameters`

| Field | Type | Required | Description |
|---|---|---|---|
| `triggerPrompt` | String | **yes** | When to send the SMS |
| `triggerMessage` | String | **yes** | Message to tell the caller |
| `messageBody` | String | **yes** | SMS message content to send |

### `SendPostCallNotificationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `admins` | bool | **yes** | Enables post-call notifications to all admin users in the location. Default: true |
| `allUsers` | bool | **yes** | Enables post-call notifications to all users in the location. Default: false |
| `contactAssignedUser` | bool | **yes** | Enables post-call notifications to the user assigned to the contact. Default: false |
| `specificUsers` | Vec<String> | **yes** | Array of specific user IDs to receive post-call notifications. Default: [] |
| `customEmails` | Vec<String> | **yes** | Array of custom email addresses to receive post-call notifications. Default: [] |

### `SendPostCallNotificationSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `admins` | bool | no | Send notifications to admins |
| `allUsers` | bool | no | Send notifications to all users |
| `contactAssignedUser` | bool | no | Send notifications to contact assigned user |
| `specificUsers` | Vec<String> | no | Specific user IDs to notify |
| `customEmails` | Vec<String> | no | Custom email addresses to notify |

### `TranslationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | Enables language translation for agent conversations. Default: false |
| `language` | String | no | Target language code for translation (e.g., "es" for Spanish, "fr" for French). |

### `TranslationSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | no | Whether translation is enabled |
| `language` | String | no | Translation language code |

### `UpdateActionResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the created action |
| `actionType` | String — `CALL_TRANSFER`, `DATA_EXTRACTION`, `IN_CALL_DATA_EXTRACTION`, `WORKFLOW_TRIGGER`, `SMS`, `APPOINTMENT_BOOKING`, `CUSTOM_ACTION`, `KNOWLEDGE_BASE` | **yes** | Type of action |
| `name` | String | **yes** | Human-readable name for this action |
| `actionParameters` | JSON | **yes** | Action parameters - structure varies by actionType |

### `UpdateSingleActionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `agentId` | String | **yes** | Agent ID to attach the action to |
| `locationId` | String | **yes** | Location ID |
| `actionType` | String — `CALL_TRANSFER`, `DATA_EXTRACTION`, `IN_CALL_DATA_EXTRACTION`, `WORKFLOW_TRIGGER`, `SMS`, `APPOINTMENT_BOOKING`, `CUSTOM_ACTION`, `KNOWLEDGE_BASE` | **yes** | Type of action |
| `name` | String | **yes** | Human-readable name for this action |
| `actionParameters` | JSON | **yes** | Action parameters - structure varies by actionType |

### `VoiceAILanguage`

Language code for the agent's speech and understanding. Default: "en-US"

String enum. Allowed values: `en-US`, `pt-BR`, `es`, `fr`, `de`, `it`, `nl-NL`, `multi`

### `WorkflowTriggerParameters`

| Field | Type | Required | Description |
|---|---|---|---|
| `triggerPrompt` | String | **yes** | When to trigger this workflow |
| `triggerMessage` | String | **yes** | Message to tell the caller |
| `workflowId` | String | **yes** | Workflow ID to trigger |

