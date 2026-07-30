# `agent-studio`

**11** operations / **14** models in API v2 · **11** operations / **14** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `agent-studio` cargo feature on `ghl-sdk`, then call any of the 11 generated methods on `ghl.agent_studio()`:

```toml
ghl-sdk = { version = "0.4", features = ["agent-studio"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/agent-studio/agent` | List Agents | `list_agents()` | `agent-studio.get_agent_studio_agent` |
| `POST` | `/agent-studio/agent` | Create Agent | `create_agent()` | `agent-studio.post_agent_studio_agent` |
| `PATCH` | `/agent-studio/agent/versions/{versionId}` | Update Agent | `update_agent()` | `agent-studio.patch_agent_studio_agent_versions_by_versionId` |
| `POST` | `/agent-studio/agent/versions/{versionId}/publish` | Promote to Production | `promote_to_production()` | `agent-studio.post_agent_studio_agent_versions_by_versionId_publish` |
| `DELETE` | `/agent-studio/agent/{agentId}` | Delete Agent | `delete_agent()` | `agent-studio.delete_agent_studio_agent_by_agentId` |
| `GET` | `/agent-studio/agent/{agentId}` | Get Agent | `get_agent()` | `agent-studio.get_agent_studio_agent_by_agentId` |
| `PATCH` | `/agent-studio/agent/{agentId}` | Update Agent Metadata | `update_agent_metadata()` | `agent-studio.patch_agent_studio_agent_by_agentId` |
| `POST` | `/agent-studio/agent/{agentId}/execute` | Execute Agent | `execute_agent()` | `agent-studio.post_agent_studio_agent_by_agentId_execute` |
| `GET` | `/agent-studio/public-api/agents` | List Agents (Deprecated) | `list_agents_deprecated()` | `agent-studio.get_agent_studio_public_api_agents` |
| `GET` | `/agent-studio/public-api/agents/{agentId}` | Get Agent (Deprecated) | `get_agent_deprecated()` | `agent-studio.get_agent_studio_public_api_agents_by_agentId` |
| `POST` | `/agent-studio/public-api/agents/{agentId}/execute` | Execute Agent (Deprecated) | `execute_agent_deprecated()` | `agent-studio.post_agent_studio_public_api_agents_by_agentId_execute` |

### Endpoint details — v2

#### `GET /agent-studio/agent`

**List Agents**

Lists all active agents for the specified location. locationId is required parameter to ensure optimal performance. Supports pagination using limit and offset. Optionally filter by isPublished=true to return only agents with a published production version.

Operation id: `agent-studio.get_agent_studio_agent` · `Version: 2021-04-15` · Scopes: `agent-studio.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `isPublished` | string | no | Optional filter to return only agents with a published production version |
| `limit` | string | **yes** | — |
| `offset` | string | **yes** | — |
| `source` | string | no | — |

*Response*: [`GetPublishedAgentsResponseDTO`](#getpublishedagentsresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::agent_studio::ListAgentsParams;

let params = ListAgentsParams::new("locationId", "limit", "offset");
let out = ghl.agent_studio().list_agents(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "agent-studio.get_agent_studio_agent",
    "query": {
      "locationId": "<locationId>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `POST /agent-studio/agent`

**Create Agent**

Creates a new agent with staging version. The agent will be created with an initial staging version that can later be promoted to production.

Operation id: `agent-studio.post_agent_studio_agent` · `Version: 2021-04-15` · Scopes: `agent-studio.write`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `source` | string | no | — |

*Request body*: [`CreatePublicAgentDTO`](#createpublicagentdto)

*Response*: [`CreatePublicAgentResponseDTO`](#createpublicagentresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::agent_studio::CreateAgentParams;

let params = CreateAgentParams::new();
let out = ghl.agent_studio().create_agent(&params, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "agent-studio.post_agent_studio_agent",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PATCH /agent-studio/agent/versions/{versionId}`

**Update Agent**

Updates a specific agent version by versionId. Supports updating nodes, edges, variables, and configuration.

Operation id: `agent-studio.patch_agent_studio_agent_versions_by_versionId` · `Version: 2021-04-15` · Scopes: `agent-studio.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `versionId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `source` | string | no | — |

*Request body*: [`UpdatePublicAgentVersionDTO`](#updatepublicagentversiondto)

*Response*: [`UpdatePublicAgentResponseDTO`](#updatepublicagentresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::agent_studio::UpdateAgentParams;

let params = UpdateAgentParams::new();
let out = ghl.agent_studio().update_agent(&versionId, &params, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "agent-studio.patch_agent_studio_agent_versions_by_versionId",
    "path_params": {
      "versionId": "<versionId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /agent-studio/agent/versions/{versionId}/publish`

**Promote to Production**

Promotes a draft version to production.

Operation id: `agent-studio.post_agent_studio_agent_versions_by_versionId_publish` · `Version: 2021-04-15` · Scopes: `agent-studio.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `versionId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `source` | string | no | — |

*Request body*: [`PromoteAndPublishDTO`](#promoteandpublishdto)

*Response*: [`PromoteAndPublishResponseDTO`](#promoteandpublishresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::agent_studio::PromoteToProductionParams;

let params = PromoteToProductionParams::new();
let out = ghl.agent_studio().promote_to_production(&versionId, &params, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "agent-studio.post_agent_studio_agent_versions_by_versionId_publish",
    "path_params": {
      "versionId": "<versionId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /agent-studio/agent/{agentId}`

**Delete Agent**

Deletes an agent and all its versions.

Operation id: `agent-studio.delete_agent_studio_agent_by_agentId` · `Version: 2021-04-15` · Scopes: `agent-studio.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `source` | string | no | — |

*Response*: [`DeletePublicAgentResponseDTO`](#deletepublicagentresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::agent_studio::DeleteAgentParams;

let params = DeleteAgentParams::new("locationId");
let out = ghl.agent_studio().delete_agent(&agentId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "agent-studio.delete_agent_studio_agent_by_agentId",
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

#### `GET /agent-studio/agent/{agentId}`

**Get Agent**

Gets a specific agent by its ID for the specified location with all its versions. Returns complete agent metadata and all non-deleted versions (draft, staging, production). locationId is required parameter. The agent must have active status.

Operation id: `agent-studio.get_agent_studio_agent_by_agentId` · `Version: 2021-04-15` · Scopes: `agent-studio.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `source` | string | no | — |

*Response*: [`GetAgentByIdResponseDTO`](#getagentbyidresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::agent_studio::GetAgentParams;

let params = GetAgentParams::new("locationId");
let out = ghl.agent_studio().get_agent(&agentId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "agent-studio.get_agent_studio_agent_by_agentId",
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

#### `PATCH /agent-studio/agent/{agentId}`

**Update Agent Metadata**

Updates agent metadata such as name, description, and status.

Operation id: `agent-studio.patch_agent_studio_agent_by_agentId` · `Version: 2021-04-15` · Scopes: `agent-studio.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `source` | string | no | — |

*Request body*: [`UpdatePublicAgentMetadataDTO`](#updatepublicagentmetadatadto)

*Response*: [`UpdatePublicAgentResponseDTO`](#updatepublicagentresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::agent_studio::UpdateAgentMetadataParams;

let params = UpdateAgentMetadataParams::new();
let out = ghl.agent_studio().update_agent_metadata(&agentId, &params, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "agent-studio.patch_agent_studio_agent_by_agentId",
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

#### `POST /agent-studio/agent/{agentId}/execute`

**Execute Agent**

Executes the specified agent and returns a non-streaming JSON response with the complete agent output. The agent must be in active status and belong to the specified location. locationId is required in the request body. **Session Management:** - For the first message in a new session, do not include the `executionId` in the request payload. - The API will return an `executionId` along with the agent response, which uniquely identifies this conversation session. - To continue the conversation wit…

Operation id: `agent-studio.post_agent_studio_agent_by_agentId_execute` · `Version: 2021-04-15` · Scopes: `agent-studio.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `source` | string | no | — |

*Request body*: [`ExecutePublicAgentDTO`](#executepublicagentdto)

*Response*: [`ExecutePublicAgentResponseDTO`](#executepublicagentresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::agent_studio::ExecuteAgentParams;

let params = ExecuteAgentParams::new();
let out = ghl.agent_studio().execute_agent(&agentId, &params, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "agent-studio.post_agent_studio_agent_by_agentId_execute",
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

#### `GET /agent-studio/public-api/agents`

**List Agents (Deprecated)**

**Deprecated endpoint - use GET /agent instead.** Lists all active agents that have a published production version for the specified location. locationId is required parameter. Supports pagination using limit and offset.

Operation id: `agent-studio.get_agent_studio_public_api_agents` · `Version: 2021-04-15` · Scopes: `agent-studio.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `limit` | string | **yes** | — |
| `offset` | string | **yes** | — |
| `source` | string | no | — |

*Response*: [`GetPublishedAgentsResponseDTO`](#getpublishedagentsresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::agent_studio::ListAgentsDeprecatedParams;

let params = ListAgentsDeprecatedParams::new("locationId", "limit", "offset");
let out = ghl.agent_studio().list_agents_deprecated(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "agent-studio.get_agent_studio_public_api_agents",
    "query": {
      "locationId": "<locationId>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `GET /agent-studio/public-api/agents/{agentId}`

**Get Agent (Deprecated)**

**Deprecated endpoint - use GET /agent/:agentId instead.** Gets a specific agent by its ID for the specified location with all its versions. locationId is required parameter. The agent must have active status.

Operation id: `agent-studio.get_agent_studio_public_api_agents_by_agentId` · `Version: 2021-04-15` · Scopes: `agent-studio.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `source` | string | no | — |

*Response*: [`GetAgentByIdResponseDTO`](#getagentbyidresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::agent_studio::GetAgentDeprecatedParams;

let params = GetAgentDeprecatedParams::new("locationId");
let out = ghl.agent_studio().get_agent_deprecated(&agentId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "agent-studio.get_agent_studio_public_api_agents_by_agentId",
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

#### `POST /agent-studio/public-api/agents/{agentId}/execute`

**Execute Agent (Deprecated)**

**Deprecated endpoint - use POST /agent/:agentId/execute instead.** Executes the specified agent and returns a non-streaming JSON response with the complete agent output. The agent must be in active status and belong to the specified location. locationId is required in the request body. **Session Management:** - For the first message in a new session, do not include the `executionId` in the request payload. - The API will return an `executionId` along with the agent response, which uniquely iden…

Operation id: `agent-studio.post_agent_studio_public_api_agents_by_agentId_execute` · `Version: 2021-04-15` · Scopes: `agent-studio.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `source` | string | no | — |

*Request body*: [`ExecutePublicAgentDTO`](#executepublicagentdto)

*Response*: [`ExecutePublicAgentResponseDTO`](#executepublicagentresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::agent_studio::ExecuteAgentDeprecatedParams;

let params = ExecuteAgentDeprecatedParams::new();
let out = ghl.agent_studio().execute_agent_deprecated(&agentId, &params, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "agent-studio.post_agent_studio_public_api_agents_by_agentId_execute",
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

## Endpoints — API v3

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/agent-studio/agent` | List Agents | `v3:agent-studio.get_agent_studio_agent` |
| `POST` | `/agent-studio/agent` | Create Agent | `v3:agent-studio.post_agent_studio_agent` |
| `PATCH` | `/agent-studio/agent/versions/{versionId}` | Update Agent | `v3:agent-studio.patch_agent_studio_agent_versions_by_versionId` |
| `POST` | `/agent-studio/agent/versions/{versionId}/publish` | Promote to Production | `v3:agent-studio.post_agent_studio_agent_versions_by_versionId_publish` |
| `DELETE` | `/agent-studio/agent/{agentId}` | Delete Agent | `v3:agent-studio.delete_agent_studio_agent_by_agentId` |
| `GET` | `/agent-studio/agent/{agentId}` | Get Agent | `v3:agent-studio.get_agent_studio_agent_by_agentId` |
| `PATCH` | `/agent-studio/agent/{agentId}` | Update Agent Metadata | `v3:agent-studio.patch_agent_studio_agent_by_agentId` |
| `POST` | `/agent-studio/agent/{agentId}/execute` | Execute Agent | `v3:agent-studio.post_agent_studio_agent_by_agentId_execute` |
| `GET` | `/agent-studio/public-api/agents` | List Agents (Deprecated) | `v3:agent-studio.get_agent_studio_public_api_agents` |
| `GET` | `/agent-studio/public-api/agents/{agentId}` | Get Agent (Deprecated) | `v3:agent-studio.get_agent_studio_public_api_agents_by_agentId` |
| `POST` | `/agent-studio/public-api/agents/{agentId}/execute` | Execute Agent (Deprecated) | `v3:agent-studio.post_agent_studio_public_api_agents_by_agentId_execute` |

### Endpoint details — v3

#### `GET /agent-studio/agent`

**List Agents**

Lists all active agents for the specified location. locationId is required parameter to ensure optimal performance. Supports pagination using limit and offset. Optionally filter by isPublished=true to return only agents with a published production version.

Operation id: `v3:agent-studio.get_agent_studio_agent` · `Version: v3` · Scopes: `agent-studio.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `isPublished` | string | no | Optional filter to return only agents with a published production version |
| `limit` | string | **yes** | — |
| `offset` | string | **yes** | — |
| `source` | string | no | — |

*Response*: [`GetPublishedAgentsResponseDTO`](#getpublishedagentsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:agent-studio.get_agent_studio_agent",
    "query": {
      "locationId": "<locationId>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `POST /agent-studio/agent`

**Create Agent**

Creates a new agent with staging version. The agent will be created with an initial staging version that can later be promoted to production.

Operation id: `v3:agent-studio.post_agent_studio_agent` · `Version: v3` · Scopes: `agent-studio.write`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `source` | string | no | — |

*Request body*: [`CreatePublicAgentDTO`](#createpublicagentdto)

*Response*: [`CreatePublicAgentResponseDTO`](#createpublicagentresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:agent-studio.post_agent_studio_agent",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PATCH /agent-studio/agent/versions/{versionId}`

**Update Agent**

Updates a specific agent version by versionId. Supports updating nodes, edges, variables, and configuration.

Operation id: `v3:agent-studio.patch_agent_studio_agent_versions_by_versionId` · `Version: v3` · Scopes: `agent-studio.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `versionId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `source` | string | no | — |

*Request body*: [`UpdatePublicAgentVersionDTO`](#updatepublicagentversiondto)

*Response*: [`UpdatePublicAgentResponseDTO`](#updatepublicagentresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:agent-studio.patch_agent_studio_agent_versions_by_versionId",
    "path_params": {
      "versionId": "<versionId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /agent-studio/agent/versions/{versionId}/publish`

**Promote to Production**

Promotes a draft version to production.

Operation id: `v3:agent-studio.post_agent_studio_agent_versions_by_versionId_publish` · `Version: v3` · Scopes: `agent-studio.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `versionId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `source` | string | no | — |

*Request body*: [`PromoteAndPublishDTO`](#promoteandpublishdto)

*Response*: [`PromoteAndPublishResponseDTO`](#promoteandpublishresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:agent-studio.post_agent_studio_agent_versions_by_versionId_publish",
    "path_params": {
      "versionId": "<versionId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /agent-studio/agent/{agentId}`

**Delete Agent**

Deletes an agent and all its versions.

Operation id: `v3:agent-studio.delete_agent_studio_agent_by_agentId` · `Version: v3` · Scopes: `agent-studio.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `source` | string | no | — |

*Response*: [`DeletePublicAgentResponseDTO`](#deletepublicagentresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:agent-studio.delete_agent_studio_agent_by_agentId",
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

#### `GET /agent-studio/agent/{agentId}`

**Get Agent**

Gets a specific agent by its ID for the specified location with all its versions. Returns complete agent metadata and all non-deleted versions (draft, staging, production). locationId is required parameter. The agent must have active status.

Operation id: `v3:agent-studio.get_agent_studio_agent_by_agentId` · `Version: v3` · Scopes: `agent-studio.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `source` | string | no | — |

*Response*: [`GetAgentByIdResponseDTO`](#getagentbyidresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:agent-studio.get_agent_studio_agent_by_agentId",
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

#### `PATCH /agent-studio/agent/{agentId}`

**Update Agent Metadata**

Updates agent metadata such as name, description, and status.

Operation id: `v3:agent-studio.patch_agent_studio_agent_by_agentId` · `Version: v3` · Scopes: `agent-studio.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `source` | string | no | — |

*Request body*: [`UpdatePublicAgentMetadataDTO`](#updatepublicagentmetadatadto)

*Response*: [`UpdatePublicAgentResponseDTO`](#updatepublicagentresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:agent-studio.patch_agent_studio_agent_by_agentId",
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

#### `POST /agent-studio/agent/{agentId}/execute`

**Execute Agent**

Executes the specified agent and returns a non-streaming JSON response with the complete agent output. The agent must be in active status and belong to the specified location. locationId is required in the request body. **Session Management:** - For the first message in a new session, do not include the `executionId` in the request payload. - The API will return an `executionId` along with the agent response, which uniquely identifies this conversation session. - To continue the conversation wit…

Operation id: `v3:agent-studio.post_agent_studio_agent_by_agentId_execute` · `Version: v3` · Scopes: `agent-studio.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `source` | string | no | — |

*Request body*: [`ExecutePublicAgentDTO`](#executepublicagentdto)

*Response*: [`ExecutePublicAgentResponseDTO`](#executepublicagentresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:agent-studio.post_agent_studio_agent_by_agentId_execute",
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

#### `GET /agent-studio/public-api/agents`

**List Agents (Deprecated)**

**Deprecated endpoint - use GET /agent instead.** Lists all active agents that have a published production version for the specified location. locationId is required parameter. Supports pagination using limit and offset.

Operation id: `v3:agent-studio.get_agent_studio_public_api_agents` · `Version: v3` · Scopes: `agent-studio.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `limit` | string | **yes** | — |
| `offset` | string | **yes** | — |
| `source` | string | no | — |

*Response*: [`GetPublishedAgentsResponseDTO`](#getpublishedagentsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:agent-studio.get_agent_studio_public_api_agents",
    "query": {
      "locationId": "<locationId>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `GET /agent-studio/public-api/agents/{agentId}`

**Get Agent (Deprecated)**

**Deprecated endpoint - use GET /agent/:agentId instead.** Gets a specific agent by its ID for the specified location with all its versions. locationId is required parameter. The agent must have active status.

Operation id: `v3:agent-studio.get_agent_studio_public_api_agents_by_agentId` · `Version: v3` · Scopes: `agent-studio.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `source` | string | no | — |

*Response*: [`GetAgentByIdResponseDTO`](#getagentbyidresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:agent-studio.get_agent_studio_public_api_agents_by_agentId",
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

#### `POST /agent-studio/public-api/agents/{agentId}/execute`

**Execute Agent (Deprecated)**

**Deprecated endpoint - use POST /agent/:agentId/execute instead.** Executes the specified agent and returns a non-streaming JSON response with the complete agent output. The agent must be in active status and belong to the specified location. locationId is required in the request body. **Session Management:** - For the first message in a new session, do not include the `executionId` in the request payload. - The API will return an `executionId` along with the agent response, which uniquely iden…

Operation id: `v3:agent-studio.post_agent_studio_public_api_agents_by_agentId_execute` · `Version: v3` · Scopes: `agent-studio.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `agentId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `source` | string | no | — |

*Request body*: [`ExecutePublicAgentDTO`](#executepublicagentdto)

*Response*: [`ExecutePublicAgentResponseDTO`](#executepublicagentresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:agent-studio.post_agent_studio_public_api_agents_by_agentId_execute",
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

## Data models — API v2

In Rust: `ghl_models::v2::agent_studio::*` (enable the `agent-studio` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/agent_studio/).

### `CreatePublicAgentDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `name` | String | no | Name of the agent |
| `description` | String | no | Description of the agent |
| `agencyId` | String | no | Agency ID |
| `authorId` | String | no | Author ID |
| `authorName` | String | no | Author name |
| `authorEmail` | String | no | Author email |
| `status` | String — `active`, `inactive`, `archived` | **yes** | Status of the agent |
| `version` | JSON | **yes** | Version data for the agent including nodes, edges, and configuration |
| `nodes` | Vec<String> | no | Nodes array (deprecated, prefer using version.nodes) |
| `edges` | Vec<String> | no | Edges array (deprecated, prefer using version.edges) |

### `CreatePublicAgentResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status |
| `message` | String | **yes** | Response message |
| `agent` | JSON | **yes** | Created agent data with metadata |
| `versions` | Vec<JSON> | **yes** | Created versions array (initial staging version) |

### `DeletePublicAgentResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status |
| `message` | String | **yes** | Response message |
| `agentId` | String | no | Deleted agent ID |

### `ExecutePublicAgentDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | **yes** | Message to send to the agent |
| `executionId` | String | no | Unique session identifier that maintains conversational context across multiple interactions within the same agent session. Omit this field for the first message in a new session. Include the executio… |
| `inputVariables` | JSON | no | Input variables to pass to the agent. These should match the input variables defined in the agent configuration. |
| `versionId` | String | no | Published version ID to execute. If not provided, the latest published production version will be used. |
| `attachments` | Vec<PublicAttachmentSchema> | no | Attachments for the message |
| `locationId` | String | **yes** | Location ID |
| `contactId` | String | no | Contact ID to associate with this execution. When provided, contact data will be hydrated and made available to the agent. |

### `ExecutePublicAgentResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status |
| `executionId` | String | **yes** | Unique session identifier that maintains conversational context across multiple interactions within the same agent session. Use this ID in subsequent requests to continue the conversation. |
| `interactionId` | String | **yes** | Unique identifier for a single interaction cycle, consisting of one user input and the corresponding agent response. Each message exchange generates a new interactionId. |
| `response` | String | **yes** | Agent response text |
| `type` | String | **yes** | Response type |
| `nextExpectedInput` | String | **yes** | Expected input type for next interaction |
| `goalCompletion` | bool | **yes** | When end node is added in the graph, this will be true if the agent reached the end node in the graph |
| `executionStatus` | String | **yes** | Execution status |
| `flowSwitch` | bool | **yes** | Whether flow was switched |
| `attachments` | Vec<JSON> | **yes** | Response attachments |
| `generativeOutputs` | Vec<JSON> | **yes** | Generated outputs |

### `GetAgentByIdResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status |
| `message` | String | **yes** | Response message |
| `agent` | JSON | **yes** | Agent metadata with all active versions |
| `traceId` | String | no | Request trace ID for debugging |

### `GetPublishedAgentsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status |
| `message` | String | **yes** | Response message |
| `agents` | Vec<JSON> | **yes** | List of agents with metadata |
| `pagination` | JSON | **yes** | Pagination metadata |

### `InternalServerErrorDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | — |
| `message` | String | no | — |

### `PromoteAndPublishDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID for authorization |
| `userId` | String | no | User ID performing the promotion action |
| `userName` | String | no | User name performing the promotion action |
| `userEmail` | String | no | User email performing the promotion action |

### `PromoteAndPublishResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status |
| `message` | String | **yes** | Response message |
| `data` | JSON | **yes** | Result data with production and new draft version details |

### `PublicAttachmentSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String | **yes** | Type of attachment |
| `imageUrl` | String | **yes** | URL of the image attachment |

### `UpdatePublicAgentMetadataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID for authorization (cannot be updated) |
| `name` | String | no | Name of the agent |
| `description` | String | no | Description of the agent |
| `status` | String — `active`, `inactive`, `archived` | no | Status of the agent |

### `UpdatePublicAgentResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status |
| `message` | String | **yes** | Response message |
| `data` | JSON | **yes** | Updated agent or version data |

### `UpdatePublicAgentVersionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID for authorization |
| `versionName` | String | no | Version name |
| `description` | String | no | Description of the version |
| `nodes` | Vec<JSON> | no | Complete array of nodes for the agent workflow. Provide all nodes including unchanged ones. |
| `edges` | Vec<JSON> | no | Complete array of edges connecting the nodes. Provide all edges including unchanged ones. |
| `globalVariables` | Vec<JSON> | no | Global variables accessible throughout the agent workflow |
| `inputVariables` | Vec<JSON> | no | Input variables required from user at execution time |
| `runtimeVariables` | Vec<JSON> | no | Runtime variables generated during agent execution |
| `globalConfig` | JSON | no | Global configuration including prompts and settings |
| `userId` | String | no | User ID performing the update |
| `userName` | String | no | User name performing the update |

## Data models — API v3

In Rust: `ghl_models::v3::agent_studio::*` (enable the `agent-studio` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/agent_studio/).

### `CreatePublicAgentDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `name` | String | no | Name of the agent |
| `description` | String | no | Description of the agent |
| `agencyId` | String | no | Agency ID |
| `authorId` | String | no | Author ID |
| `authorName` | String | no | Author name |
| `authorEmail` | String | no | Author email |
| `status` | String — `active`, `inactive`, `archived` | **yes** | Status of the agent |
| `version` | JSON | **yes** | Version data for the agent including nodes, edges, and configuration |
| `nodes` | Vec<String> | no | Nodes array (deprecated, prefer using version.nodes) |
| `edges` | Vec<String> | no | Edges array (deprecated, prefer using version.edges) |

### `CreatePublicAgentResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status |
| `message` | String | **yes** | Response message |
| `agent` | JSON | **yes** | Created agent data with metadata |
| `versions` | Vec<JSON> | **yes** | Created versions array (initial staging version) |

### `DeletePublicAgentResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status |
| `message` | String | **yes** | Response message |
| `agentId` | String | no | Deleted agent ID |

### `ExecutePublicAgentDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | **yes** | Message to send to the agent |
| `executionId` | String | no | Unique session identifier that maintains conversational context across multiple interactions within the same agent session. Omit this field for the first message in a new session. Include the executio… |
| `inputVariables` | JSON | no | Input variables to pass to the agent. These should match the input variables defined in the agent configuration. |
| `versionId` | String | no | Published version ID to execute. If not provided, the latest published production version will be used. |
| `attachments` | Vec<PublicAttachmentSchema> | no | Attachments for the message |
| `locationId` | String | **yes** | Location ID |
| `contactId` | String | no | Contact ID to associate with this execution. When provided, contact data will be hydrated and made available to the agent. |

### `ExecutePublicAgentResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status |
| `executionId` | String | **yes** | Unique session identifier that maintains conversational context across multiple interactions within the same agent session. Use this ID in subsequent requests to continue the conversation. |
| `interactionId` | String | **yes** | Unique identifier for a single interaction cycle, consisting of one user input and the corresponding agent response. Each message exchange generates a new interactionId. |
| `response` | String | **yes** | Agent response text |
| `type` | String | **yes** | Response type |
| `nextExpectedInput` | String | **yes** | Expected input type for next interaction |
| `goalCompletion` | bool | **yes** | When end node is added in the graph, this will be true if the agent reached the end node in the graph |
| `executionStatus` | String | **yes** | Execution status |
| `flowSwitch` | bool | **yes** | Whether flow was switched |
| `attachments` | Vec<JSON> | **yes** | Response attachments |
| `generativeOutputs` | Vec<JSON> | **yes** | Generated outputs |

### `GetAgentByIdResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status |
| `message` | String | **yes** | Response message |
| `agent` | JSON | **yes** | Agent metadata with all active versions |
| `traceId` | String | no | Request trace ID for debugging |

### `GetPublishedAgentsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status |
| `message` | String | **yes** | Response message |
| `agents` | Vec<JSON> | **yes** | List of agents with metadata |
| `pagination` | JSON | **yes** | Pagination metadata |

### `InternalServerErrorDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | — |
| `message` | String | no | — |

### `PromoteAndPublishDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID for authorization |
| `userId` | String | no | User ID performing the promotion action |
| `userName` | String | no | User name performing the promotion action |
| `userEmail` | String | no | User email performing the promotion action |

### `PromoteAndPublishResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status |
| `message` | String | **yes** | Response message |
| `data` | JSON | **yes** | Result data with production and new draft version details |

### `PublicAttachmentSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String | **yes** | Type of attachment |
| `imageUrl` | String | **yes** | URL of the image attachment |

### `UpdatePublicAgentMetadataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID for authorization (cannot be updated) |
| `name` | String | no | Name of the agent |
| `description` | String | no | Description of the agent |
| `status` | String — `active`, `inactive`, `archived` | no | Status of the agent |

### `UpdatePublicAgentResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status |
| `message` | String | **yes** | Response message |
| `data` | JSON | **yes** | Updated agent or version data |

### `UpdatePublicAgentVersionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID for authorization |
| `versionName` | String | no | Version name |
| `description` | String | no | Description of the version |
| `nodes` | Vec<JSON> | no | Complete array of nodes for the agent workflow. Provide all nodes including unchanged ones. |
| `edges` | Vec<JSON> | no | Complete array of edges connecting the nodes. Provide all edges including unchanged ones. |
| `globalVariables` | Vec<JSON> | no | Global variables accessible throughout the agent workflow |
| `inputVariables` | Vec<JSON> | no | Input variables required from user at execution time |
| `runtimeVariables` | Vec<JSON> | no | Runtime variables generated during agent execution |
| `globalConfig` | JSON | no | Global configuration including prompts and settings |
| `userId` | String | no | User ID performing the update |
| `userName` | String | no | User name performing the update |

