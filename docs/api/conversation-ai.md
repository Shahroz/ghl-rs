# `conversation-ai`

**12** operations / **28** models in API v2 · **12** operations / **28** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `conversation-ai` cargo feature on `ghl-sdk`, then call any of the 12 generated methods on `ghl.conversation_ai()`:

```toml
ghl-sdk = { version = "0.4", features = ["conversation-ai"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `POST` | `/conversation-ai/agents` | Create an Agent | `create_an_agent()` | `conversation-ai.post_conversation_ai_agents` |
| `GET` | `/conversation-ai/agents/search` | Search Agents | `search_agents()` | `conversation-ai.get_conversation_ai_agents_search` |
| `DELETE` | `/conversation-ai/agents/{agentId}` | Delete Agent | `delete_agent()` | `conversation-ai.delete_conversation_ai_agents_by_agentId` |
| `GET` | `/conversation-ai/agents/{agentId}` | Get Agent | `get_agent()` | `conversation-ai.get_conversation_ai_agents_by_agentId` |
| `PUT` | `/conversation-ai/agents/{agentId}` | Update Agent | `update_agent()` | `conversation-ai.put_conversation_ai_agents_by_agentId` |
| `POST` | `/conversation-ai/agents/{agentId}/actions` | Attach Action to Agent | `attach_action_to_agent()` | `conversation-ai.post_conversation_ai_agents_by_agentId_actions` |
| `GET` | `/conversation-ai/agents/{agentId}/actions/list` | List Actions for an Agent | `list_actions_for_an_agent()` | `conversation-ai.get_conversation_ai_agents_by_agentId_actions_list` |
| `DELETE` | `/conversation-ai/agents/{agentId}/actions/{actionId}` | Remove Action from Agent | `remove_action_from_agent()` | `conversation-ai.delete_conversation_ai_agents_by_agentId_actions_by_actionId` |
| `GET` | `/conversation-ai/agents/{agentId}/actions/{actionId}` | Get Action by ID | `get_action_by_id()` | `conversation-ai.get_conversation_ai_agents_by_agentId_actions_by_actionId` |
| `PUT` | `/conversation-ai/agents/{agentId}/actions/{actionId}` | Update Action | `update_action()` | `conversation-ai.put_conversation_ai_agents_by_agentId_actions_by_actionId` |
| `PATCH` | `/conversation-ai/agents/{agentId}/followup-settings` | Update Followup Settings | `update_followup_settings()` | `conversation-ai.patch_conversation_ai_agents_by_agentId_followup_settings` |
| `GET` | `/conversation-ai/generations` | Get the generation details | `get_the_generation_details()` | `conversation-ai.get_conversation_ai_generations` |

### Endpoint details — v2

#### `POST /conversation-ai/agents`

**Create an Agent**

Creates a new AI agent for the location. The agent will be created with the specified configuration including name, role, actions, and behavior settings.

Operation id: `conversation-ai.post_conversation_ai_agents` · `Version: 2021-04-15` · Scopes: `conversation-ai.write`

*Request body*: [`CreateEmployeeDto`](#createemployeedto)

*Response*: [`EmployeeResponseDTO`](#employeeresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversation_ai().create_an_agent(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversation-ai.post_conversation_ai_agents",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /conversation-ai/agents/search`

**Search Agents**

Searches for AI agents based on various criteria including name, status, and configuration. Supports advanced filtering and full-text search capabilities.

Operation id: `conversation-ai.get_conversation_ai_agents_search` · `Version: 2021-04-15` · Scopes: `conversation-ai.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `startAfter` | string | no | Start after is the agent id to start after, Serving as skip, send empty when first page |
| `limit` | number | no | Records per page |
| `query` | string | no | query to search on agent name, must be provided in lowercase |

*Response*: [`SearchEmployeeResponseDTO`](#searchemployeeresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::conversation_ai::SearchAgentsParams;

let params = SearchAgentsParams::new();
let out = ghl.conversation_ai().search_agents(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversation-ai.get_conversation_ai_agents_search"
  }
}
```

</details>

#### `DELETE /conversation-ai/agents/{agentId}`

**Delete Agent**

Deletes an AI agent permanently. This action cannot be undone. All associated configurations and conversation history will be removed.

Operation id: `conversation-ai.delete_conversation_ai_agents_by_agentId` · `Version: 2021-04-15` · Scopes: `conversation-ai.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | Conversations AI agent id |

*Response*: [`DeleteEmployeeResponseDTO`](#deleteemployeeresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversation_ai().delete_agent(&agentId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversation-ai.delete_conversation_ai_agents_by_agentId",
    "path_params": {
      "agentId": "<agentId>"
    }
  }
}
```

</details>

#### `GET /conversation-ai/agents/{agentId}`

**Get Agent**

Retrieves a specific AI agent by its ID. Returns the complete agent configuration including name, status, actions, and settings.

Operation id: `conversation-ai.get_conversation_ai_agents_by_agentId` · `Version: 2021-04-15` · Scopes: `conversation-ai.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | Conversations AI agent id |

*Response*: [`EmployeeResponseDTO`](#employeeresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversation_ai().get_agent(&agentId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversation-ai.get_conversation_ai_agents_by_agentId",
    "path_params": {
      "agentId": "<agentId>"
    }
  }
}
```

</details>

#### `PUT /conversation-ai/agents/{agentId}`

**Update Agent**

Updates an existing AI agent's configuration. All fields in the agent configuration can be updated including name, status, actions, and behavior settings.

Operation id: `conversation-ai.put_conversation_ai_agents_by_agentId` · `Version: 2021-04-15` · Scopes: `conversation-ai.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | Conversations AI agent id |

*Request body*: [`UpdateEmployeeDto`](#updateemployeedto)

*Response*: [`EmployeeResponseDTO`](#employeeresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversation_ai().update_agent(&agentId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversation-ai.put_conversation_ai_agents_by_agentId",
    "path_params": {
      "agentId": "<agentId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /conversation-ai/agents/{agentId}/actions`

**Attach Action to Agent**

Creates and attach a new action for an AI agent. Actions define specific tasks or behaviors that the agent can perform, such as booking appointments, sending follow-ups, or collecting information.

Operation id: `conversation-ai.post_conversation_ai_agents_by_agentId_actions` · `Version: 2021-04-15` · Scopes: `conversation-ai.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | — |

*Request body*: [`CreateActionDTO`](#createactiondto)

*Response*: [`createActionResponseDTO`](#createactionresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversation_ai().attach_action_to_agent(&agentId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversation-ai.post_conversation_ai_agents_by_agentId_actions",
    "path_params": {
      "agentId": "<agentId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /conversation-ai/agents/{agentId}/actions/list`

**List Actions for an Agent**

List for actions for an agent

Operation id: `conversation-ai.get_conversation_ai_agents_by_agentId_actions_list` · `Version: 2021-04-15` · Scopes: `conversation-ai.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | — |

*Response*: [`fetchActionsForEmployeeResponseDTO`](#fetchactionsforemployeeresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversation_ai().list_actions_for_an_agent(&agentId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversation-ai.get_conversation_ai_agents_by_agentId_actions_list",
    "path_params": {
      "agentId": "<agentId>"
    }
  }
}
```

</details>

#### `DELETE /conversation-ai/agents/{agentId}/actions/{actionId}`

**Remove Action from Agent**

Permanently deletes an action. This will remove the action from all associated agents and cannot be undone.

Operation id: `conversation-ai.delete_conversation_ai_agents_by_agentId_actions_by_actionId` · `Version: 2021-04-15` · Scopes: `conversation-ai.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `actionId` | string | **yes** | The unique identifier of the action ID Attached to the agent |
| `agentId` | string | **yes** | — |

*Response*: [`deleteActionResponseDTO`](#deleteactionresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversation_ai().remove_action_from_agent(&actionId, &agentId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversation-ai.delete_conversation_ai_agents_by_agentId_actions_by_actionId",
    "path_params": {
      "actionId": "<actionId>",
      "agentId": "<agentId>"
    }
  }
}
```

</details>

#### `GET /conversation-ai/agents/{agentId}/actions/{actionId}`

**Get Action by ID**

Retrieves detailed information about a specific action using its unique identifier. Returns the action configuration, associated agents, and performance metrics.

Operation id: `conversation-ai.get_conversation_ai_agents_by_agentId_actions_by_actionId` · `Version: 2021-04-15` · Scopes: `conversation-ai.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `actionId` | string | **yes** | The unique identifier of the action ID Attached to the agent |
| `agentId` | string | **yes** | — |

*Response*: [`fetchActionDetailsResponseDTO`](#fetchactiondetailsresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversation_ai().get_action_by_id(&actionId, &agentId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversation-ai.get_conversation_ai_agents_by_agentId_actions_by_actionId",
    "path_params": {
      "actionId": "<actionId>",
      "agentId": "<agentId>"
    }
  }
}
```

</details>

#### `PUT /conversation-ai/agents/{agentId}/actions/{actionId}`

**Update Action**

Updates an existing action's configuration. This includes modifying the action name, description, trigger conditions, and behavior settings.

Operation id: `conversation-ai.put_conversation_ai_agents_by_agentId_actions_by_actionId` · `Version: 2021-04-15` · Scopes: `conversation-ai.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `actionId` | string | **yes** | The unique identifier of the action ID Attached to the agent |
| `agentId` | string | **yes** | — |

*Request body*: [`CreateActionDTO`](#createactiondto)

*Response*: [`updateActionResponseDTO`](#updateactionresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversation_ai().update_action(&actionId, &agentId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversation-ai.put_conversation_ai_agents_by_agentId_actions_by_actionId",
    "path_params": {
      "actionId": "<actionId>",
      "agentId": "<agentId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PATCH /conversation-ai/agents/{agentId}/followup-settings`

**Update Followup Settings**

Update the followup settings for an action

Operation id: `conversation-ai.patch_conversation_ai_agents_by_agentId_followup_settings` · `Version: 2021-04-15` · Scopes: `conversation-ai.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | — |

*Request body*: [`UpdateFollowupSettingsDTO`](#updatefollowupsettingsdto)

*Response*: [`updateActionResponseDTO`](#updateactionresponsedto)

*Rust*:

```rust,ignore
let out = ghl.conversation_ai().update_followup_settings(&agentId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversation-ai.patch_conversation_ai_agents_by_agentId_followup_settings",
    "path_params": {
      "agentId": "<agentId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /conversation-ai/generations`

**Get the generation details**

Retrieves detailed information about AI responses including the System Prompt, Conversation history, Knowledge base, website, FAQ chunks, and Rich Text chunks.

Operation id: `conversation-ai.get_conversation_ai_generations` · `Version: 2021-04-15` · Scopes: `conversation-ai.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `messageId` | string | **yes** | Message Id |
| `source` | enum: `conversation`, `workflow` | **yes** | — |

*Response*: [`FetchAIResponseDetailsResponseDTO`](#fetchairesponsedetailsresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::conversation_ai::GetTheGenerationDetailsParams;

let params = GetTheGenerationDetailsParams::new("messageId", "source");
let out = ghl.conversation_ai().get_the_generation_details(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "conversation-ai.get_conversation_ai_generations",
    "query": {
      "messageId": "<messageId>",
      "source": "<source>"
    }
  }
}
```

</details>

## Endpoints — API v3

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `POST` | `/conversation-ai/agents` | Create an Agent | `v3:conversation-ai.post_conversation_ai_agents` |
| `GET` | `/conversation-ai/agents/search` | Search Agents | `v3:conversation-ai.get_conversation_ai_agents_search` |
| `DELETE` | `/conversation-ai/agents/{agentId}` | Delete Agent | `v3:conversation-ai.delete_conversation_ai_agents_by_agentId` |
| `GET` | `/conversation-ai/agents/{agentId}` | Get Agent | `v3:conversation-ai.get_conversation_ai_agents_by_agentId` |
| `PUT` | `/conversation-ai/agents/{agentId}` | Update Agent | `v3:conversation-ai.put_conversation_ai_agents_by_agentId` |
| `POST` | `/conversation-ai/agents/{agentId}/actions` | Attach Action to Agent | `v3:conversation-ai.post_conversation_ai_agents_by_agentId_actions` |
| `GET` | `/conversation-ai/agents/{agentId}/actions/list` | List Actions for an Agent | `v3:conversation-ai.get_conversation_ai_agents_by_agentId_actions_list` |
| `DELETE` | `/conversation-ai/agents/{agentId}/actions/{actionId}` | Remove Action from Agent | `v3:conversation-ai.delete_conversation_ai_agents_by_agentId_actions_by_actionId` |
| `GET` | `/conversation-ai/agents/{agentId}/actions/{actionId}` | Get Action by ID | `v3:conversation-ai.get_conversation_ai_agents_by_agentId_actions_by_actionId` |
| `PUT` | `/conversation-ai/agents/{agentId}/actions/{actionId}` | Update Action | `v3:conversation-ai.put_conversation_ai_agents_by_agentId_actions_by_actionId` |
| `PATCH` | `/conversation-ai/agents/{agentId}/followup-settings` | Update Followup Settings | `v3:conversation-ai.patch_conversation_ai_agents_by_agentId_followup_settings` |
| `GET` | `/conversation-ai/generations` | Get the generation details | `v3:conversation-ai.get_conversation_ai_generations` |

### Endpoint details — v3

#### `POST /conversation-ai/agents`

**Create an Agent**

Creates a new AI agent for the location. The agent will be created with the specified configuration including name, role, actions, and behavior settings.

Operation id: `v3:conversation-ai.post_conversation_ai_agents` · `Version: v3` · Scopes: `conversation-ai.write`

*Request body*: [`CreateEmployeeDto`](#createemployeedto)

*Response*: [`EmployeeResponseDTO`](#employeeresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversation-ai.post_conversation_ai_agents",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /conversation-ai/agents/search`

**Search Agents**

Searches for AI agents based on various criteria including name, status, and configuration. Supports advanced filtering and full-text search capabilities.

Operation id: `v3:conversation-ai.get_conversation_ai_agents_search` · `Version: v3` · Scopes: `conversation-ai.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `startAfter` | string | no | Start after is the agent id to start after, Serving as skip, send empty when first page |
| `limit` | number | no | Records per page |
| `query` | string | no | query to search on agent name, must be provided in lowercase |

*Response*: [`SearchEmployeeResponseDTO`](#searchemployeeresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversation-ai.get_conversation_ai_agents_search"
  }
}
```

</details>

#### `DELETE /conversation-ai/agents/{agentId}`

**Delete Agent**

Deletes an AI agent permanently. This action cannot be undone. All associated configurations and conversation history will be removed.

Operation id: `v3:conversation-ai.delete_conversation_ai_agents_by_agentId` · `Version: v3` · Scopes: `conversation-ai.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | Conversations AI agent id |

*Response*: [`DeleteEmployeeResponseDTO`](#deleteemployeeresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversation-ai.delete_conversation_ai_agents_by_agentId",
    "path_params": {
      "agentId": "<agentId>"
    }
  }
}
```

</details>

#### `GET /conversation-ai/agents/{agentId}`

**Get Agent**

Retrieves a specific AI agent by its ID. Returns the complete agent configuration including name, status, actions, and settings.

Operation id: `v3:conversation-ai.get_conversation_ai_agents_by_agentId` · `Version: v3` · Scopes: `conversation-ai.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | Conversations AI agent id |

*Response*: [`EmployeeResponseDTO`](#employeeresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversation-ai.get_conversation_ai_agents_by_agentId",
    "path_params": {
      "agentId": "<agentId>"
    }
  }
}
```

</details>

#### `PUT /conversation-ai/agents/{agentId}`

**Update Agent**

Updates an existing AI agent's configuration. All fields in the agent configuration can be updated including name, status, actions, and behavior settings.

Operation id: `v3:conversation-ai.put_conversation_ai_agents_by_agentId` · `Version: v3` · Scopes: `conversation-ai.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | Conversations AI agent id |

*Request body*: [`UpdateEmployeeDto`](#updateemployeedto)

*Response*: [`EmployeeResponseDTO`](#employeeresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversation-ai.put_conversation_ai_agents_by_agentId",
    "path_params": {
      "agentId": "<agentId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /conversation-ai/agents/{agentId}/actions`

**Attach Action to Agent**

Creates and attach a new action for an AI agent. Actions define specific tasks or behaviors that the agent can perform, such as booking appointments, sending follow-ups, or collecting information.

Operation id: `v3:conversation-ai.post_conversation_ai_agents_by_agentId_actions` · `Version: v3` · Scopes: `conversation-ai.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | — |

*Request body*: [`CreateActionDTO`](#createactiondto)

*Response*: [`createActionResponseDTO`](#createactionresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversation-ai.post_conversation_ai_agents_by_agentId_actions",
    "path_params": {
      "agentId": "<agentId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /conversation-ai/agents/{agentId}/actions/list`

**List Actions for an Agent**

List for actions for an agent

Operation id: `v3:conversation-ai.get_conversation_ai_agents_by_agentId_actions_list` · `Version: v3` · Scopes: `conversation-ai.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | — |

*Response*: [`fetchActionsForEmployeeResponseDTO`](#fetchactionsforemployeeresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversation-ai.get_conversation_ai_agents_by_agentId_actions_list",
    "path_params": {
      "agentId": "<agentId>"
    }
  }
}
```

</details>

#### `DELETE /conversation-ai/agents/{agentId}/actions/{actionId}`

**Remove Action from Agent**

Permanently deletes an action. This will remove the action from all associated agents and cannot be undone.

Operation id: `v3:conversation-ai.delete_conversation_ai_agents_by_agentId_actions_by_actionId` · `Version: v3` · Scopes: `conversation-ai.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `actionId` | string | **yes** | The unique identifier of the action ID Attached to the agent |
| `agentId` | string | **yes** | — |

*Response*: [`deleteActionResponseDTO`](#deleteactionresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversation-ai.delete_conversation_ai_agents_by_agentId_actions_by_actionId",
    "path_params": {
      "actionId": "<actionId>",
      "agentId": "<agentId>"
    }
  }
}
```

</details>

#### `GET /conversation-ai/agents/{agentId}/actions/{actionId}`

**Get Action by ID**

Retrieves detailed information about a specific action using its unique identifier. Returns the action configuration, associated agents, and performance metrics.

Operation id: `v3:conversation-ai.get_conversation_ai_agents_by_agentId_actions_by_actionId` · `Version: v3` · Scopes: `conversation-ai.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `actionId` | string | **yes** | The unique identifier of the action ID Attached to the agent |
| `agentId` | string | **yes** | — |

*Response*: [`fetchActionDetailsResponseDTO`](#fetchactiondetailsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversation-ai.get_conversation_ai_agents_by_agentId_actions_by_actionId",
    "path_params": {
      "actionId": "<actionId>",
      "agentId": "<agentId>"
    }
  }
}
```

</details>

#### `PUT /conversation-ai/agents/{agentId}/actions/{actionId}`

**Update Action**

Updates an existing action's configuration. This includes modifying the action name, description, trigger conditions, and behavior settings.

Operation id: `v3:conversation-ai.put_conversation_ai_agents_by_agentId_actions_by_actionId` · `Version: v3` · Scopes: `conversation-ai.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `actionId` | string | **yes** | The unique identifier of the action ID Attached to the agent |
| `agentId` | string | **yes** | — |

*Request body*: [`CreateActionDTO`](#createactiondto)

*Response*: [`updateActionResponseDTO`](#updateactionresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversation-ai.put_conversation_ai_agents_by_agentId_actions_by_actionId",
    "path_params": {
      "actionId": "<actionId>",
      "agentId": "<agentId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PATCH /conversation-ai/agents/{agentId}/followup-settings`

**Update Followup Settings**

Update the followup settings for an action

Operation id: `v3:conversation-ai.patch_conversation_ai_agents_by_agentId_followup_settings` · `Version: v3` · Scopes: `conversation-ai.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | — |

*Request body*: [`UpdateFollowupSettingsDTO`](#updatefollowupsettingsdto)

*Response*: [`updateActionResponseDTO`](#updateactionresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversation-ai.patch_conversation_ai_agents_by_agentId_followup_settings",
    "path_params": {
      "agentId": "<agentId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /conversation-ai/generations`

**Get the generation details**

Retrieves detailed information about AI responses including the System Prompt, Conversation history, Knowledge base, website, FAQ chunks, and Rich Text chunks.

Operation id: `v3:conversation-ai.get_conversation_ai_generations` · `Version: v3` · Scopes: `conversation-ai.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `messageId` | string | **yes** | Message Id |
| `source` | enum: `conversation`, `workflow` | **yes** | — |

*Response*: [`FetchAIResponseDetailsResponseDTO`](#fetchairesponsedetailsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:conversation-ai.get_conversation_ai_generations",
    "query": {
      "messageId": "<messageId>",
      "source": "<source>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::conversation_ai::*` (enable the `conversation-ai` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/conversation_ai/).

### `ActionDataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the action |
| `name` | String | **yes** | Name of the action |
| `type` | String — `triggerWorkflow`, `updateContactField`, `appointmentBooking`, `stopBot`, `humanHandOver`, `advancedFollowup`, `transferBot` | **yes** | Type of the action |
| `agentId` | String | no | Agent ID where the action belongs |
| `details` | JSON | **yes** | Action-specific details. The structure depends on the action type. For TRIGGER_WORKFLOW use triggerWorkflowDto, for UPDATE_CONTACT_FIELD use updateContactFieldDto, for APPOINTMENT_BOOKING use appointm… |

### `ActionsIdDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the action. |
| `type` | String — `triggerWorkflow`, `updateContactField`, `appointmentBooking`, `stopBot`, `humanHandOver`, `advancedFollowup`, `transferBot` | **yes** | type of action. |

### `CreateActionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `triggerWorkflow`, `updateContactField`, `appointmentBooking`, `stopBot`, `humanHandOver`, `advancedFollowup`, `transferBot` | **yes** | — |
| `name` | String | **yes** | — |
| `details` | JSON | **yes** | Action-specific details. The structure depends on the action type. For TRIGGER_WORKFLOW use triggerWorkflowDto, for UPDATE_CONTACT_FIELD use updateContactFieldDto, for APPOINTMENT_BOOKING use appointm… |

### `CreateEmployeeDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Name of the agent. |
| `businessName` | String | no | Name of the business the agent represents. |
| `mode` | String — `off`, `suggestive`, `auto-pilot` | no | Mode of operation - OFF, SUGGESTIVE, or AUTO_PILOT |
| `channels` | Vec<String (enum)> | no | Communication channels the agent can operate on |
| `isPrimary` | bool | no | Indicates if this agent is a primary agent. |
| `waitTime` | f64 | no | Wait time before agent responds (max 5 for minutes, 300 for seconds) |
| `waitTimeUnit` | String — `minutes`, `seconds` | no | Unit for wait time - SECONDS or MINUTES |
| `sleepEnabled` | bool | no | Indicates if sleep functionality is enabled. |
| `sleepTime` | f64 | no | Duration of sleep period (required if sleepEnabled is true). Set to null for indefinite sleep. (max 2880 for minutes, 172800 for seconds, 48 for hours) |
| `sleepTimeUnit` | String — `hours`, `minutes`, `seconds` | no | Unit of sleep time - HOURS, MINUTES, or SECONDS (required if sleepEnabled is true). Set to null for indefinite sleep. |
| `personality` | String | **yes** | Personality traits of the agent. |
| `goal` | String | **yes** | The goal of the agent. |
| `instructions` | String | **yes** | Instructions for the agent. |
| `autoPilotMaxMessages` | f64 | no | Maximum number of messages in auto-pilot mode before requiring human intervention. (max: 100, min: 1) |
| `knowledgeBaseIds` | Vec<String> | no | Array of knowledge base IDs associated with this agent. |
| `respondToImages` | bool | no | Allow agent to respond to images |
| `respondToAudio` | bool | no | Allow agent to respond to audio |
| `sleepOnManualMessage` | bool | no | Enable sleep when a manual outbound message is sent. |
| `sleepOnWorkflowMessage` | bool | no | Enable sleep when a workflow outbound message is sent. |

### `DeleteActionDataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | ID of the deleted action |

### `DeleteEmployeeResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Indicates if the agent was deleted successfully. |
| `id` | String | **yes** | Unique identifier of the deleted agent. |

### `EmployeeListItemDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the agent. |
| `name` | String | **yes** | Name of the agent. |
| `businessName` | String | no | Name of the business the agent represents. |
| `mode` | String — `off`, `suggestive`, `auto-pilot` | **yes** | Current operating mode of the agent. |
| `channels` | Vec<String> | **yes** | Communication channels the agent operates on. |
| `waitTime` | f64 | **yes** | Wait time before agent responds. |
| `waitTimeUnit` | String — `minutes`, `seconds` | **yes** | Unit for wait time. |
| `sleepEnabled` | bool | **yes** | Indicates if sleep functionality is enabled. |
| `sleepTime` | f64 | no | Duration of sleep period. |
| `sleepTimeUnit` | String — `hours`, `minutes`, `seconds` | no | Unit of sleep time. |
| `actions` | Vec<JSON> | **yes** | List of actions associated with this agent. |
| `isPrimary` | bool | **yes** | Indicates if this agent is a primary agent. (First agent created for a location is primary by default) |
| `autoPilotMaxMessages` | f64 | **yes** | Maximum number of messages in auto-pilot mode before requiring human intervention. |
| `goal` | JSON | no | Goal configuration for the agent. |
| `knowledgeBaseIds` | Vec<String> | no | Array of knowledge base IDs associated with this agent. |
| `createdAt` | String | **yes** | Timestamp when the agent was created. |
| `updatedAt` | String | **yes** | Timestamp when the agent was last updated. |
| `sleepOnManualMessage` | bool | no | Whether the bot sleeps on manual outbound messages. |
| `sleepOnWorkflowMessage` | bool | no | Whether the bot sleeps on workflow outbound messages. |

### `EmployeeResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the agent. |
| `name` | String | **yes** | Name of the agent. |
| `businessName` | String | no | Name of the business the agent represents. |
| `mode` | String — `off`, `suggestive`, `auto-pilot` | **yes** | Current operating mode of the agent. |
| `channels` | Vec<String (enum)> | **yes** | Communication channels the agent operates on. |
| `waitTime` | f64 | **yes** | Wait time before agent responds. |
| `waitTimeUnit` | String — `minutes`, `seconds` | **yes** | Unit for wait time. |
| `sleepEnabled` | bool | **yes** | Indicates if sleep functionality is enabled. |
| `sleepTime` | f64 | no | Duration of sleep period. |
| `sleepTimeUnit` | String — `hours`, `minutes`, `seconds` | no | Unit of sleep time. |
| `actions` | Vec<ActionsIdDto> | **yes** | List of actions associated with this agent. |
| `isPrimary` | bool | **yes** | Indicates if this agent is a primary agent. |
| `autoPilotMaxMessages` | f64 | **yes** | Maximum number of messages in auto-pilot mode before requiring human intervention. |
| `goal` | String | no | The goal of the agent. |
| `personality` | String | no | Personality traits of the agent. |
| `instructions` | String | no | Instructions for the agent. |
| `knowledgeBaseIds` | Vec<String> | no | Array of knowledge base IDs associated with this agent. |
| `sleepOnManualMessage` | bool | no | Whether the bot sleeps on manual outbound messages. |
| `sleepOnWorkflowMessage` | bool | no | Whether the bot sleeps on workflow outbound messages. |

### `FetchAIResponseDetailsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `prompt` | String | **yes** | The complete prompt used for the AI response. |
| `intent` | String | no | The intent/goal extracted from location prompt. |
| `responseMessage` | String | **yes** | The response message generated by the AI. |
| `faqs` | Vec<JSON> | no | FAQ chunks used in generating the response from fine-tuned data. |
| `website` | Vec<JSON> | no | Website content chunks used in generating the response. |
| `agentId` | String | no | ID of the employee/agent that generated the response. |
| `input` | String | no | The original input message that triggered this response. |
| `actionLogs` | Vec<JSON> | **yes** | List of actions taken during this interaction. |
| `history` | Vec<JSON> | **yes** | Conversation history leading up to this response. |
| `mode` | String | no | Mode of operation during this interaction. |

### `FollowupSequence`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | f64 | **yes** | Unique identifier for this followup step |
| `followupTimeUnit` | String — `days`, `hours`, `minutes` | **yes** | Time unit for followup delay |
| `followupTime` | f64 | **yes** | Time duration before followup (max: 60 minutes, 24 hours, or 180 days depending on unit) |
| `aiEnabledMessage` | bool | no | Whether to use AI to generate the followup message |
| `triggerWorkflow` | bool | no | Whether to trigger a workflow during this followup |
| `customMessage` | String | no | Custom message to send (when aiEnabledMessage is false) |
| `workflowId` | String | no | Workflow ID to trigger (when triggerWorkflow is true) |
| `contactRequested` | bool | no | Whether contact was requested in this followup |

### `FollowupSettings`

| Field | Type | Required | Description |
|---|---|---|---|
| `dynamicChannelSwitching` | bool | **yes** | Whether to dynamically switch channels for followups |
| `followUpHours` | bool | no | Whether to respect working hours for followups |
| `workingHours` | Vec<WorkingHours> | no | Working hours configuration for followups |
| `timezoneToUse` | String — `contact`, `business` | no | Timezone to use for followups, contact or location |

### `Interval`

| Field | Type | Required | Description |
|---|---|---|---|
| `startHour` | f64 | **yes** | Start hour (24-hour format) |
| `startMinute` | f64 | **yes** | Start minute |
| `endHour` | f64 | **yes** | End hour (24-hour format) |
| `endMinute` | f64 | **yes** | End minute |

### `SearchEmployeeResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `agents` | Vec<EmployeeListItemDTO> | **yes** | List of agents matching the search criteria. |
| `totalCount` | f64 | **yes** | Total number of agents in the location (unfiltered count). |
| `count` | f64 | **yes** | Number of agents in the current response (filtered/paginated count). |

### `UpdateEmployeeDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Name of the agent. |
| `businessName` | String | no | Name of the business the agent represents. |
| `mode` | String — `off`, `suggestive`, `auto-pilot` | no | Mode of operation for the agent, required if primary is enabled. |
| `channels` | Vec<String (enum)> | no | Channels the agent can use. |
| `isPrimary` | bool | no | Indicates if this agent is a primary agent. |
| `waitTime` | f64 | no | Wait time before agent responds (max 5 for minutes, 300 for seconds). |
| `waitTimeUnit` | String — `minutes`, `seconds` | no | Unit for wait time - SECONDS or MINUTES |
| `sleepEnabled` | bool | no | Indicates if sleep functionality is enabled. |
| `sleepTime` | f64 | no | Duration of sleep period (required if sleepEnabled is true). Set to null for indefinite sleep. (max 2880 for minutes, 172800 for seconds, 48 for hours) |
| `sleepTimeUnit` | String — `hours`, `minutes`, `seconds` | no | Unit of sleep time - HOURS, MINUTES, or SECONDS (required if sleepEnabled is true). Set to null for indefinite sleep. |
| `personality` | String | no | Personality traits of the agent. |
| `goal` | String | no | The goal of the agent. |
| `instructions` | String | no | Instructions for the agent. |
| `autoPilotMaxMessages` | f64 | **yes** | Maximum number of messages in auto-pilot mode before requiring human intervention. (max: 100, min: 1) |
| `knowledgeBaseIds` | Vec<String> | no | Array of knowledge base IDs associated with this agent. |
| `respondToImages` | bool | no | Allow agent to respond to images |
| `respondToAudio` | bool | no | Allow agent to respond to audio |
| `sleepOnManualMessage` | bool | no | Enable sleep when a manual outbound message is sent. |
| `sleepOnWorkflowMessage` | bool | no | Enable sleep when a workflow outbound message is sent. |

### `UpdateFollowupSettingsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `actionIds` | Vec<String> | **yes** | — |
| `followupSettings` | [`FollowupSettings`](#followupsettings) | **yes** | — |

### `WorkingHours`

| Field | Type | Required | Description |
|---|---|---|---|
| `dayOfTheWeek` | f64 | **yes** | Day of the week (0=Sunday, 1=Monday, etc.) |
| `intervals` | Vec<Interval> | no | Time intervals for this day |

### `advancedFollowupDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | Whether advanced followup is enabled |
| `scenarioId` | String — `contactStoppedReplying`, `contactIsBusy`, `contactRequested` | **yes** | ID of the followup scenario |
| `followupSequence` | Vec<FollowupSequence> | **yes** | Sequence of followup actions to perform |
| `followupSettings` | [`FollowupSettings`](#followupsettings) | no | Additional settings for followup behavior |

### `appointmentBookingDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `actionId` | String | no | Optional action ID reference |
| `calendarId` | String | **yes** | Calendar ID for appointment booking |
| `onlySendLink` | bool | **yes** | If true, only sends the appointment link without booking |
| `triggerWorkflow` | bool | **yes** | Whether to trigger a workflow after booking (cannot be true when onlySendLink is true) |
| `workflowIds` | Vec<String> | no | Workflow IDs to trigger after booking (required when triggerWorkflow is true) |
| `sleepAfterBooking` | bool | **yes** | Whether to put the agent to sleep after booking (cannot be true when onlySendLink is true) |
| `sleepTimeUnit` | String — `days`, `hours`, `minutes` | no | Unit for sleep time (required when sleepAfterBooking is true) |
| `sleepTime` | f64 | no | Sleep duration (required when sleepAfterBooking is true) |
| `transferBot` | bool | **yes** | Whether to transfer to another agent after booking (cannot be true when onlySendLink is true) |
| `transferAgent` | String | no | Agent ID to transfer to (required when transferBot is true) |
| `rescheduleEnabled` | bool | **yes** | Whether to allow appointment rescheduling (cannot be true when onlySendLink is true) |
| `cancelEnabled` | bool | **yes** | Whether to allow appointment cancellation (cannot be true when onlySendLink is true) |

### `createActionResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | [`ActionDataDTO`](#actiondatadto) | **yes** | Created action details |
| `success` | bool | **yes** | Success status of the request |

### `deleteActionResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | [`DeleteActionDataDTO`](#deleteactiondatadto) | **yes** | Deleted action information |
| `success` | bool | **yes** | Success status of the request |

### `fetchActionDetailsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | [`ActionDataDTO`](#actiondatadto) | **yes** | Action details |
| `success` | bool | **yes** | Success status of the request |

### `fetchActionsForEmployeeResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<ActionDataDTO> | **yes** | Grouped actions by type |
| `success` | bool | **yes** | Success status of the request |

### `humanHandOverDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | Whether human handover action is enabled |
| `triggerCondition` | String | **yes** | Condition that triggers human handover |
| `examples` | Vec<String> | no | Example phrases that trigger human handover (required when handoverType is custom or contactRequest) |
| `assignToUserId` | String | no | ID of the user to assign the conversation to |
| `skipAssignToUser` | bool | no | Whether to skip assigning to a specific user |
| `createTask` | bool | no | Whether to create a task when handing over |
| `reactivateEnabled` | bool | **yes** | Whether the agent can be reactivated after handover |
| `sleepTimeUnit` | String — `days`, `hours`, `minutes` | no | Time unit for reactivation delay (required when reactivateEnabled is true) |
| `sleepTime` | f64 | no | Time duration before reactivation (required when reactivateEnabled is true) |
| `finalMessage` | String | **yes** | Final message sent when handing over to human |
| `tags` | Vec<String> | no | Tags to apply during handover |
| `handoverType` | String — `contactRequest`, `lackOfInformation`, `failedToResolveIssue`, `custom` | **yes** | Type of human handover detection |

### `stopBotDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `stopBotDetectionType` | String — `Goodbye`, `Custom` | **yes** | Type of stop bot detection - Goodbye or Custom |
| `stopBotTriggerCondition` | String | **yes** | Condition that triggers stopping the bot |
| `reactivateEnabled` | bool | **yes** | Whether the bot can be reactivated after being stopped |
| `sleepTimeUnit` | String — `days`, `hours`, `minutes` | no | Time unit for reactivation delay (required when reactivateEnabled is true) |
| `sleepTime` | f64 | no | Time duration before reactivation (required when reactivateEnabled is true) |
| `enabled` | bool | **yes** | Whether this action is enabled for the agent |
| `stopBotExamples` | Vec<String> | **yes** | Example phrases that trigger stop bot action (minimum 2 required) |
| `finalMessage` | String | **yes** | Final message sent when stopping the bot |
| `tags` | Vec<String> | no | Tags to apply when stopping the bot |

### `transferBotDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `transferBotType` | String — `Default`, `Custom` | **yes** | Type of transfer - Default or Custom |
| `transferToBot` | String | **yes** | ID of the bot/agent to transfer to |
| `enabled` | bool | **yes** | Whether this transfer action is enabled |
| `transferBotTriggerCondition` | String | no | Condition that triggers the transfer (required for Custom type) |
| `transferBotExamples` | Vec<String> | no | Example phrases that trigger transfer (required for Custom type, minimum 2) |

### `triggerWorkflowDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `workflowIds` | Vec<String> | **yes** | Array of workflow IDs to trigger |
| `triggerCondition` | String | **yes** | Condition that triggers the workflow |
| `triggerMessage` | String | no | Optional message to send when triggering the workflow |

### `updateActionResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | [`ActionDataDTO`](#actiondatadto) | **yes** | Updated action details |
| `success` | bool | **yes** | Success status of the request |

### `updateContactFieldDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `contactFieldId` | String | **yes** | ID of the contact field in Contacts Table |
| `description` | String | **yes** | Description of the contact field in Contacts Table |
| `contactUpdateExamples` | Vec<String> | no | Contact update examples in Contacts Table. Not required when using standard fields, Monetory or Date Custom fields. |

## Data models — API v3

In Rust: `ghl_models::v3::conversation_ai::*` (enable the `conversation-ai` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/conversation_ai/).

### `ActionDataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the action |
| `name` | String | **yes** | Name of the action |
| `type` | String — `triggerWorkflow`, `updateContactField`, `appointmentBooking`, `stopBot`, `humanHandOver`, `advancedFollowup`, `transferBot` | **yes** | Type of the action |
| `agentId` | String | no | Agent ID where the action belongs |
| `details` | JSON | **yes** | Action-specific details. The structure depends on the action type. For TRIGGER_WORKFLOW use triggerWorkflowDto, for UPDATE_CONTACT_FIELD use updateContactFieldDto, for APPOINTMENT_BOOKING use appointm… |

### `ActionsIdDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the action. |
| `type` | String — `triggerWorkflow`, `updateContactField`, `appointmentBooking`, `stopBot`, `humanHandOver`, `advancedFollowup`, `transferBot` | **yes** | type of action. |

### `CreateActionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `triggerWorkflow`, `updateContactField`, `appointmentBooking`, `stopBot`, `humanHandOver`, `advancedFollowup`, `transferBot` | **yes** | — |
| `name` | String | **yes** | — |
| `details` | JSON | **yes** | Action-specific details. The structure depends on the action type. For TRIGGER_WORKFLOW use triggerWorkflowDto, for UPDATE_CONTACT_FIELD use updateContactFieldDto, for APPOINTMENT_BOOKING use appointm… |

### `CreateEmployeeDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Name of the agent. |
| `businessName` | String | no | Name of the business the agent represents. |
| `mode` | String — `off`, `suggestive`, `auto-pilot` | no | Mode of operation - OFF, SUGGESTIVE, or AUTO_PILOT |
| `channels` | Vec<String (enum)> | no | Communication channels the agent can operate on |
| `isPrimary` | bool | no | Indicates if this agent is a primary agent. |
| `waitTime` | f64 | no | Wait time before agent responds (max 5 for minutes, 300 for seconds) |
| `waitTimeUnit` | String — `minutes`, `seconds` | no | Unit for wait time - SECONDS or MINUTES |
| `sleepEnabled` | bool | no | Indicates if sleep functionality is enabled. |
| `sleepTime` | f64 | no | Duration of sleep period (required if sleepEnabled is true). Set to null for indefinite sleep. (max 2880 for minutes, 172800 for seconds, 48 for hours) |
| `sleepTimeUnit` | String — `hours`, `minutes`, `seconds` | no | Unit of sleep time - HOURS, MINUTES, or SECONDS (required if sleepEnabled is true). Set to null for indefinite sleep. |
| `personality` | String | **yes** | Personality traits of the agent. |
| `goal` | String | **yes** | The goal of the agent. |
| `instructions` | String | **yes** | Instructions for the agent. |
| `autoPilotMaxMessages` | f64 | no | Maximum number of messages in auto-pilot mode before requiring human intervention. (max: 100, min: 1) |
| `knowledgeBaseIds` | Vec<String> | no | Array of knowledge base IDs associated with this agent. |
| `respondToImages` | bool | no | Allow agent to respond to images |
| `respondToAudio` | bool | no | Allow agent to respond to audio |
| `sleepOnManualMessage` | bool | no | Enable sleep when a manual outbound message is sent. |
| `sleepOnWorkflowMessage` | bool | no | Enable sleep when a workflow outbound message is sent. |

### `DeleteActionDataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | ID of the deleted action |

### `DeleteEmployeeResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Indicates if the agent was deleted successfully. |
| `id` | String | **yes** | Unique identifier of the deleted agent. |

### `EmployeeListItemDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the agent. |
| `name` | String | **yes** | Name of the agent. |
| `businessName` | String | no | Name of the business the agent represents. |
| `mode` | String — `off`, `suggestive`, `auto-pilot` | **yes** | Current operating mode of the agent. |
| `channels` | Vec<String> | **yes** | Communication channels the agent operates on. |
| `waitTime` | f64 | **yes** | Wait time before agent responds. |
| `waitTimeUnit` | String — `minutes`, `seconds` | **yes** | Unit for wait time. |
| `sleepEnabled` | bool | **yes** | Indicates if sleep functionality is enabled. |
| `sleepTime` | f64 | no | Duration of sleep period. |
| `sleepTimeUnit` | String — `hours`, `minutes`, `seconds` | no | Unit of sleep time. |
| `actions` | Vec<JSON> | **yes** | List of actions associated with this agent. |
| `isPrimary` | bool | **yes** | Indicates if this agent is a primary agent. (First agent created for a location is primary by default) |
| `autoPilotMaxMessages` | f64 | **yes** | Maximum number of messages in auto-pilot mode before requiring human intervention. |
| `goal` | JSON | no | Goal configuration for the agent. |
| `knowledgeBaseIds` | Vec<String> | no | Array of knowledge base IDs associated with this agent. |
| `createdAt` | String | **yes** | Timestamp when the agent was created. |
| `updatedAt` | String | **yes** | Timestamp when the agent was last updated. |
| `sleepOnManualMessage` | bool | no | Whether the bot sleeps on manual outbound messages. |
| `sleepOnWorkflowMessage` | bool | no | Whether the bot sleeps on workflow outbound messages. |

### `EmployeeResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the agent. |
| `name` | String | **yes** | Name of the agent. |
| `businessName` | String | no | Name of the business the agent represents. |
| `mode` | String — `off`, `suggestive`, `auto-pilot` | **yes** | Current operating mode of the agent. |
| `channels` | Vec<String (enum)> | **yes** | Communication channels the agent operates on. |
| `waitTime` | f64 | **yes** | Wait time before agent responds. |
| `waitTimeUnit` | String — `minutes`, `seconds` | **yes** | Unit for wait time. |
| `sleepEnabled` | bool | **yes** | Indicates if sleep functionality is enabled. |
| `sleepTime` | f64 | no | Duration of sleep period. |
| `sleepTimeUnit` | String — `hours`, `minutes`, `seconds` | no | Unit of sleep time. |
| `actions` | Vec<ActionsIdDto> | **yes** | List of actions associated with this agent. |
| `isPrimary` | bool | **yes** | Indicates if this agent is a primary agent. |
| `autoPilotMaxMessages` | f64 | **yes** | Maximum number of messages in auto-pilot mode before requiring human intervention. |
| `goal` | String | no | The goal of the agent. |
| `personality` | String | no | Personality traits of the agent. |
| `instructions` | String | no | Instructions for the agent. |
| `knowledgeBaseIds` | Vec<String> | no | Array of knowledge base IDs associated with this agent. |
| `sleepOnManualMessage` | bool | no | Whether the bot sleeps on manual outbound messages. |
| `sleepOnWorkflowMessage` | bool | no | Whether the bot sleeps on workflow outbound messages. |

### `FetchAIResponseDetailsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `prompt` | String | **yes** | The complete prompt used for the AI response. |
| `intent` | String | no | The intent/goal extracted from location prompt. |
| `responseMessage` | String | **yes** | The response message generated by the AI. |
| `faqs` | Vec<JSON> | no | FAQ chunks used in generating the response from fine-tuned data. |
| `website` | Vec<JSON> | no | Website content chunks used in generating the response. |
| `agentId` | String | no | ID of the employee/agent that generated the response. |
| `input` | String | no | The original input message that triggered this response. |
| `actionLogs` | Vec<JSON> | **yes** | List of actions taken during this interaction. |
| `history` | Vec<JSON> | **yes** | Conversation history leading up to this response. |
| `mode` | String | no | Mode of operation during this interaction. |

### `FollowupSequence`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | f64 | **yes** | Unique identifier for this followup step |
| `followupTimeUnit` | String — `days`, `hours`, `minutes` | **yes** | Time unit for followup delay |
| `followupTime` | f64 | **yes** | Time duration before followup (max: 60 minutes, 24 hours, or 180 days depending on unit) |
| `aiEnabledMessage` | bool | no | Whether to use AI to generate the followup message |
| `triggerWorkflow` | bool | no | Whether to trigger a workflow during this followup |
| `customMessage` | String | no | Custom message to send (when aiEnabledMessage is false) |
| `workflowId` | String | no | Workflow ID to trigger (when triggerWorkflow is true) |
| `contactRequested` | bool | no | Whether contact was requested in this followup |

### `FollowupSettings`

| Field | Type | Required | Description |
|---|---|---|---|
| `dynamicChannelSwitching` | bool | **yes** | Whether to dynamically switch channels for followups |
| `followUpHours` | bool | no | Whether to respect working hours for followups |
| `workingHours` | Vec<WorkingHours> | no | Working hours configuration for followups |
| `timezoneToUse` | String — `contact`, `business` | no | Timezone to use for followups, contact or location |

### `Interval`

| Field | Type | Required | Description |
|---|---|---|---|
| `startHour` | f64 | **yes** | Start hour (24-hour format) |
| `startMinute` | f64 | **yes** | Start minute |
| `endHour` | f64 | **yes** | End hour (24-hour format) |
| `endMinute` | f64 | **yes** | End minute |

### `SearchEmployeeResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `agents` | Vec<EmployeeListItemDTO> | **yes** | List of agents matching the search criteria. |
| `totalCount` | f64 | **yes** | Total number of agents in the location (unfiltered count). |
| `count` | f64 | **yes** | Number of agents in the current response (filtered/paginated count). |

### `UpdateEmployeeDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Name of the agent. |
| `businessName` | String | no | Name of the business the agent represents. |
| `mode` | String — `off`, `suggestive`, `auto-pilot` | no | Mode of operation for the agent, required if primary is enabled. |
| `channels` | Vec<String (enum)> | no | Channels the agent can use. |
| `isPrimary` | bool | no | Indicates if this agent is a primary agent. |
| `waitTime` | f64 | no | Wait time before agent responds (max 5 for minutes, 300 for seconds). |
| `waitTimeUnit` | String — `minutes`, `seconds` | no | Unit for wait time - SECONDS or MINUTES |
| `sleepEnabled` | bool | no | Indicates if sleep functionality is enabled. |
| `sleepTime` | f64 | no | Duration of sleep period (required if sleepEnabled is true). Set to null for indefinite sleep. (max 2880 for minutes, 172800 for seconds, 48 for hours) |
| `sleepTimeUnit` | String — `hours`, `minutes`, `seconds` | no | Unit of sleep time - HOURS, MINUTES, or SECONDS (required if sleepEnabled is true). Set to null for indefinite sleep. |
| `personality` | String | no | Personality traits of the agent. |
| `goal` | String | no | The goal of the agent. |
| `instructions` | String | no | Instructions for the agent. |
| `autoPilotMaxMessages` | f64 | **yes** | Maximum number of messages in auto-pilot mode before requiring human intervention. (max: 100, min: 1) |
| `knowledgeBaseIds` | Vec<String> | no | Array of knowledge base IDs associated with this agent. |
| `respondToImages` | bool | no | Allow agent to respond to images |
| `respondToAudio` | bool | no | Allow agent to respond to audio |
| `sleepOnManualMessage` | bool | no | Enable sleep when a manual outbound message is sent. |
| `sleepOnWorkflowMessage` | bool | no | Enable sleep when a workflow outbound message is sent. |

### `UpdateFollowupSettingsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `actionIds` | Vec<String> | **yes** | — |
| `followupSettings` | [`FollowupSettings`](#followupsettings) | **yes** | — |

### `WorkingHours`

| Field | Type | Required | Description |
|---|---|---|---|
| `dayOfTheWeek` | f64 | **yes** | Day of the week (0=Sunday, 1=Monday, etc.) |
| `intervals` | Vec<Interval> | no | Time intervals for this day |

### `advancedFollowupDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | Whether advanced followup is enabled |
| `scenarioId` | String — `contactStoppedReplying`, `contactIsBusy`, `contactRequested` | **yes** | ID of the followup scenario |
| `followupSequence` | Vec<FollowupSequence> | **yes** | Sequence of followup actions to perform |
| `followupSettings` | [`FollowupSettings`](#followupsettings) | no | Additional settings for followup behavior |

### `appointmentBookingDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `actionId` | String | no | Optional action ID reference |
| `calendarId` | String | **yes** | Calendar ID for appointment booking |
| `onlySendLink` | bool | **yes** | If true, only sends the appointment link without booking |
| `triggerWorkflow` | bool | **yes** | Whether to trigger a workflow after booking (cannot be true when onlySendLink is true) |
| `workflowIds` | Vec<String> | no | Workflow IDs to trigger after booking (required when triggerWorkflow is true) |
| `sleepAfterBooking` | bool | **yes** | Whether to put the agent to sleep after booking (cannot be true when onlySendLink is true) |
| `sleepTimeUnit` | String — `days`, `hours`, `minutes` | no | Unit for sleep time (required when sleepAfterBooking is true) |
| `sleepTime` | f64 | no | Sleep duration (required when sleepAfterBooking is true) |
| `transferBot` | bool | **yes** | Whether to transfer to another agent after booking (cannot be true when onlySendLink is true) |
| `transferAgent` | String | no | Agent ID to transfer to (required when transferBot is true) |
| `rescheduleEnabled` | bool | **yes** | Whether to allow appointment rescheduling (cannot be true when onlySendLink is true) |
| `cancelEnabled` | bool | **yes** | Whether to allow appointment cancellation (cannot be true when onlySendLink is true) |

### `createActionResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | [`ActionDataDTO`](#actiondatadto) | **yes** | Created action details |
| `success` | bool | **yes** | Success status of the request |

### `deleteActionResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | [`DeleteActionDataDTO`](#deleteactiondatadto) | **yes** | Deleted action information |
| `success` | bool | **yes** | Success status of the request |

### `fetchActionDetailsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | [`ActionDataDTO`](#actiondatadto) | **yes** | Action details |
| `success` | bool | **yes** | Success status of the request |

### `fetchActionsForEmployeeResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<ActionDataDTO> | **yes** | Grouped actions by type |
| `success` | bool | **yes** | Success status of the request |

### `humanHandOverDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | Whether human handover action is enabled |
| `triggerCondition` | String | **yes** | Condition that triggers human handover |
| `examples` | Vec<String> | no | Example phrases that trigger human handover (required when handoverType is custom or contactRequest) |
| `assignToUserId` | String | no | ID of the user to assign the conversation to |
| `skipAssignToUser` | bool | no | Whether to skip assigning to a specific user |
| `createTask` | bool | no | Whether to create a task when handing over |
| `reactivateEnabled` | bool | **yes** | Whether the agent can be reactivated after handover |
| `sleepTimeUnit` | String — `days`, `hours`, `minutes` | no | Time unit for reactivation delay (required when reactivateEnabled is true) |
| `sleepTime` | f64 | no | Time duration before reactivation (required when reactivateEnabled is true) |
| `finalMessage` | String | **yes** | Final message sent when handing over to human |
| `tags` | Vec<String> | no | Tags to apply during handover |
| `handoverType` | String — `contactRequest`, `lackOfInformation`, `failedToResolveIssue`, `custom` | **yes** | Type of human handover detection |

### `stopBotDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `stopBotDetectionType` | String — `Goodbye`, `Custom` | **yes** | Type of stop bot detection - Goodbye or Custom |
| `stopBotTriggerCondition` | String | **yes** | Condition that triggers stopping the bot |
| `reactivateEnabled` | bool | **yes** | Whether the bot can be reactivated after being stopped |
| `sleepTimeUnit` | String — `days`, `hours`, `minutes` | no | Time unit for reactivation delay (required when reactivateEnabled is true) |
| `sleepTime` | f64 | no | Time duration before reactivation (required when reactivateEnabled is true) |
| `enabled` | bool | **yes** | Whether this action is enabled for the agent |
| `stopBotExamples` | Vec<String> | **yes** | Example phrases that trigger stop bot action (minimum 2 required) |
| `finalMessage` | String | **yes** | Final message sent when stopping the bot |
| `tags` | Vec<String> | no | Tags to apply when stopping the bot |

### `transferBotDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `transferBotType` | String — `Default`, `Custom` | **yes** | Type of transfer - Default or Custom |
| `transferToBot` | String | **yes** | ID of the bot/agent to transfer to |
| `enabled` | bool | **yes** | Whether this transfer action is enabled |
| `transferBotTriggerCondition` | String | no | Condition that triggers the transfer (required for Custom type) |
| `transferBotExamples` | Vec<String> | no | Example phrases that trigger transfer (required for Custom type, minimum 2) |

### `triggerWorkflowDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `workflowIds` | Vec<String> | **yes** | Array of workflow IDs to trigger |
| `triggerCondition` | String | **yes** | Condition that triggers the workflow |
| `triggerMessage` | String | no | Optional message to send when triggering the workflow |

### `updateActionResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | [`ActionDataDTO`](#actiondatadto) | **yes** | Updated action details |
| `success` | bool | **yes** | Success status of the request |

### `updateContactFieldDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `contactFieldId` | String | **yes** | ID of the contact field in Contacts Table |
| `description` | String | **yes** | Description of the contact field in Contacts Table |
| `contactUpdateExamples` | Vec<String> | no | Contact update examples in Contacts Table. Not required when using standard fields, Monetory or Date Custom fields. |

