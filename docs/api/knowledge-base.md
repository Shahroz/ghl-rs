# `knowledge-base`

**14** operations / **38** models in API v2 · **14** operations / **38** models in API v3

## How to call it

No hand-written service yet — reach these endpoints two ways:

**From Rust** (typed body via [`ghl-models`](https://docs.rs/ghl-models)):

```rust,ignore
// cargo add ghl-models --features knowledge-base
use ghl_models::v2::knowledge_base::*;

let body = serde_json::to_value(/* a Create…Dto from above */)?;
let out = ghl.request_raw("POST", "/path/", &[], Some(&body), None).await?;
```

**From an AI agent** (MCP meta-tools):

```json
{
  "name": "ghl_search_operations",
  "arguments": {
    "query": "",
    "module": "knowledge-base"
  }
}
```

## Endpoints — API v2

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/knowledge-bases/` | Get all knowledge bases for a location by location Id (paginated) | `knowledge-base.get_knowledge_bases` |
| `POST` | `/knowledge-bases/` | Create a new knowledge base (max 15 knowledge bases per location) | `knowledge-base.post_knowledge_bases` |
| `DELETE` | `/knowledge-bases/crawler` | Delete trained pages | `knowledge-base.delete_knowledge_bases_crawler` |
| `GET` | `/knowledge-bases/crawler` | Get all trained page links by knowledge base | `knowledge-base.get_knowledge_bases_crawler` |
| `POST` | `/knowledge-bases/crawler` | Start crawling and discover pages for training | `knowledge-base.post_knowledge_bases_crawler` |
| `GET` | `/knowledge-bases/crawler/status` | Get crawling status for the latest operation | `knowledge-base.get_knowledge_bases_crawler_status` |
| `POST` | `/knowledge-bases/crawler/train` | Train discovered website pages and ingest into the knowledge base | `knowledge-base.post_knowledge_bases_crawler_train` |
| `GET` | `/knowledge-bases/faqs` | Get all FAQs by knowledge base with pagination support | `knowledge-base.get_knowledge_bases_faqs` |
| `POST` | `/knowledge-bases/faqs` | Create a new FAQ inside knowledge base | `knowledge-base.post_knowledge_bases_faqs` |
| `DELETE` | `/knowledge-bases/faqs/{id}` | Delete an existing knowledge base FAQ | `knowledge-base.delete_knowledge_bases_faqs_by_id` |
| `PUT` | `/knowledge-bases/faqs/{id}` | Update an existing knowledge base FAQ | `knowledge-base.put_knowledge_bases_faqs_by_id` |
| `PUT` | `/knowledge-bases/{id}` | Update a knowledge base | `knowledge-base.put_knowledge_bases_by_id` |
| `DELETE` | `/knowledge-bases/{knowledgeBaseId}` | Delete a knowledge base | `knowledge-base.delete_knowledge_bases_by_knowledgeBaseId` |
| `GET` | `/knowledge-bases/{knowledgeBaseId}` | Get knowledge base by ID | `knowledge-base.get_knowledge_bases_by_knowledgeBaseId` |

### Endpoint details — v2

#### `GET /knowledge-bases/`

**Get all knowledge bases for a location by location Id (paginated)**

Operation id: `knowledge-base.get_knowledge_bases` · `Version: 2021-04-15`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `query` | string | no | search query for knowledge base name |
| `limit` | number | no | Maximum number of knowledge bases to return |
| `lastKnowledgeBaseId` | string | no | ID of the last knowledge base from the previous page (for pagination) |

*Response*: [`GetAllKnowledgeBasesPaginatedResponseDTO`](#getallknowledgebasespaginatedresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "knowledge-base.get_knowledge_bases",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /knowledge-bases/`

**Create a new knowledge base (max 15 knowledge bases per location)**

Operation id: `knowledge-base.post_knowledge_bases` · `Version: 2021-04-15`

*Request body*: [`CreateKnowledgeBaseDTO`](#createknowledgebasedto)

*Response*: [`CreateKnowledgeBaseResponseDTO`](#createknowledgebaseresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "knowledge-base.post_knowledge_bases",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /knowledge-bases/crawler`

**Delete trained pages**

Operation id: `knowledge-base.delete_knowledge_bases_crawler` · `Version: 2021-04-15`

*Request body*: [`DeleteWebsiteUrlRequestDTO`](#deletewebsiteurlrequestdto)

*Response*: [`DeleteWebsiteUrlResponseDTO`](#deletewebsiteurlresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "knowledge-base.delete_knowledge_bases_crawler",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /knowledge-bases/crawler`

**Get all trained page links by knowledge base**

Operation id: `knowledge-base.get_knowledge_bases_crawler` · `Version: 2021-04-15`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `knowledgeBaseId` | string | **yes** | knowledge base ID as string |
| `locationId` | string | **yes** | location ID as string |
| `page` | number | no | Page number |
| `pageLength` | number | no | Records per page |
| `query` | string | no | query to filter on url links |

*Response*: [`GetAllUrlsByKnowledgeBaseResponseDTO`](#getallurlsbyknowledgebaseresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "knowledge-base.get_knowledge_bases_crawler",
    "query": {
      "knowledgeBaseId": "<knowledgeBaseId>",
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /knowledge-bases/crawler`

**Start crawling and discover pages for training**

Operation id: `knowledge-base.post_knowledge_bases_crawler` · `Version: 2021-04-15`

*Request body*: [`DiscoverWebsiteRequestDTO`](#discoverwebsiterequestdto)

*Response*: [`DiscoverWebsiteResponseDTO`](#discoverwebsiteresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "knowledge-base.post_knowledge_bases_crawler",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /knowledge-bases/crawler/status`

**Get crawling status for the latest operation**

Operation id: `knowledge-base.get_knowledge_bases_crawler_status` · `Version: 2021-04-15`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID as string |
| `operationId` | string | **yes** | operation id as string |
| `knowledgeBaseId` | string | **yes** | knowledge base id |

*Response*: [`CrawlingStatusResponseDTO`](#crawlingstatusresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "knowledge-base.get_knowledge_bases_crawler_status",
    "query": {
      "locationId": "<locationId>",
      "operationId": "<operationId>",
      "knowledgeBaseId": "<knowledgeBaseId>"
    }
  }
}
```

</details>

#### `POST /knowledge-bases/crawler/train`

**Train discovered website pages and ingest into the knowledge base**

Operation id: `knowledge-base.post_knowledge_bases_crawler_train` · `Version: 2021-04-15`

*Request body*: [`TrainDiscoveredUrlsDTO`](#traindiscoveredurlsdto)

*Response*: [`TrainDiscoveredUrlsResponseDTO`](#traindiscoveredurlsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "knowledge-base.post_knowledge_bases_crawler_train",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /knowledge-bases/faqs`

**Get all FAQs by knowledge base with pagination support**

Retrieves FAQs for a knowledge base. Supports pagination using limit and lastFaqId parameters.

Operation id: `knowledge-base.get_knowledge_bases_faqs` · `Version: 2021-04-15`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `knowledgeBaseId` | string | **yes** | knowledge base ID as string |
| `locationId` | string | **yes** | location ID as string |
| `limit` | number | no | Limit the number of FAQs returned |
| `lastFaqId` | string | no | Last FAQ ID for pagination (cursor-based) |

*Response*: [`ListFaqsResponseDTO`](#listfaqsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "knowledge-base.get_knowledge_bases_faqs",
    "query": {
      "knowledgeBaseId": "<knowledgeBaseId>",
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /knowledge-bases/faqs`

**Create a new FAQ inside knowledge base**

Operation id: `knowledge-base.post_knowledge_bases_faqs` · `Version: 2021-04-15`

*Request body*: [`AddFaqDTO`](#addfaqdto)

*Response*: [`CreateFaqResponseDTO`](#createfaqresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "knowledge-base.post_knowledge_bases_faqs",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /knowledge-bases/faqs/{id}`

**Delete an existing knowledge base FAQ**

Operation id: `knowledge-base.delete_knowledge_bases_faqs_by_id` · `Version: 2021-04-15`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | faq ID as string |

*Response*: [`DeleteFaqResponseDTO`](#deletefaqresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "knowledge-base.delete_knowledge_bases_faqs_by_id",
    "path_params": {
      "id": "<id>"
    }
  }
}
```

</details>

#### `PUT /knowledge-bases/faqs/{id}`

**Update an existing knowledge base FAQ**

Operation id: `knowledge-base.put_knowledge_bases_faqs_by_id` · `Version: 2021-04-15`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | faq ID as string |

*Request body*: [`UpdateFaqBodyDTO`](#updatefaqbodydto)

*Response*: [`UpdateFaqResponseDTO`](#updatefaqresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "knowledge-base.put_knowledge_bases_faqs_by_id",
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

#### `PUT /knowledge-bases/{id}`

**Update a knowledge base**

Operation id: `knowledge-base.put_knowledge_bases_by_id` · `Version: 2021-04-15`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | — |

*Request body*: [`UpdateKnowledgeBaseDTO`](#updateknowledgebasedto)

*Response*: [`UpdateKnowledgeBaseResponseDTO`](#updateknowledgebaseresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "knowledge-base.put_knowledge_bases_by_id",
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

#### `DELETE /knowledge-bases/{knowledgeBaseId}`

**Delete a knowledge base**

Operation id: `knowledge-base.delete_knowledge_bases_by_knowledgeBaseId` · `Version: 2021-04-15`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `knowledgeBaseId` | string | **yes** | — |

*Response*: [`DeleteKnowledgeBaseResponseDTO`](#deleteknowledgebaseresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "knowledge-base.delete_knowledge_bases_by_knowledgeBaseId",
    "path_params": {
      "knowledgeBaseId": "<knowledgeBaseId>"
    }
  }
}
```

</details>

#### `GET /knowledge-bases/{knowledgeBaseId}`

**Get knowledge base by ID**

Operation id: `knowledge-base.get_knowledge_bases_by_knowledgeBaseId` · `Version: 2021-04-15`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `knowledgeBaseId` | string | **yes** | — |

*Response*: [`GetKnowledgeBaseByIdResponseDTO`](#getknowledgebasebyidresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "knowledge-base.get_knowledge_bases_by_knowledgeBaseId",
    "path_params": {
      "knowledgeBaseId": "<knowledgeBaseId>"
    }
  }
}
```

</details>

## Endpoints — API v3

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/knowledge-bases/` | Get all knowledge bases for a location by location Id (paginated) | `v3:knowledge-base.get_knowledge_bases` |
| `POST` | `/knowledge-bases/` | Create a new knowledge base (max 15 knowledge bases per location) | `v3:knowledge-base.post_knowledge_bases` |
| `DELETE` | `/knowledge-bases/crawler` | Delete trained pages | `v3:knowledge-base.delete_knowledge_bases_crawler` |
| `GET` | `/knowledge-bases/crawler` | Get all trained page links by knowledge base | `v3:knowledge-base.get_knowledge_bases_crawler` |
| `POST` | `/knowledge-bases/crawler` | Start crawling and discover pages for training | `v3:knowledge-base.post_knowledge_bases_crawler` |
| `GET` | `/knowledge-bases/crawler/status` | Get crawling status for the latest operation | `v3:knowledge-base.get_knowledge_bases_crawler_status` |
| `POST` | `/knowledge-bases/crawler/train` | Train discovered website pages and ingest into the knowledge base | `v3:knowledge-base.post_knowledge_bases_crawler_train` |
| `GET` | `/knowledge-bases/faqs` | Get all FAQs by knowledge base with pagination support | `v3:knowledge-base.get_knowledge_bases_faqs` |
| `POST` | `/knowledge-bases/faqs` | Create a new FAQ inside knowledge base | `v3:knowledge-base.post_knowledge_bases_faqs` |
| `DELETE` | `/knowledge-bases/faqs/{id}` | Delete an existing knowledge base FAQ | `v3:knowledge-base.delete_knowledge_bases_faqs_by_id` |
| `PUT` | `/knowledge-bases/faqs/{id}` | Update an existing knowledge base FAQ | `v3:knowledge-base.put_knowledge_bases_faqs_by_id` |
| `PUT` | `/knowledge-bases/{id}` | Update a knowledge base | `v3:knowledge-base.put_knowledge_bases_by_id` |
| `DELETE` | `/knowledge-bases/{knowledgeBaseId}` | Delete a knowledge base | `v3:knowledge-base.delete_knowledge_bases_by_knowledgeBaseId` |
| `GET` | `/knowledge-bases/{knowledgeBaseId}` | Get knowledge base by ID | `v3:knowledge-base.get_knowledge_bases_by_knowledgeBaseId` |

### Endpoint details — v3

#### `GET /knowledge-bases/`

**Get all knowledge bases for a location by location Id (paginated)**

Operation id: `v3:knowledge-base.get_knowledge_bases` · `Version: v3`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `query` | string | no | search query for knowledge base name |
| `limit` | number | no | Maximum number of knowledge bases to return |
| `lastKnowledgeBaseId` | string | no | ID of the last knowledge base from the previous page (for pagination) |

*Response*: [`GetAllKnowledgeBasesPaginatedResponseDTO`](#getallknowledgebasespaginatedresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:knowledge-base.get_knowledge_bases",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /knowledge-bases/`

**Create a new knowledge base (max 15 knowledge bases per location)**

Operation id: `v3:knowledge-base.post_knowledge_bases` · `Version: v3`

*Request body*: [`CreateKnowledgeBaseDTO`](#createknowledgebasedto)

*Response*: [`CreateKnowledgeBaseResponseDTO`](#createknowledgebaseresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:knowledge-base.post_knowledge_bases",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /knowledge-bases/crawler`

**Delete trained pages**

Operation id: `v3:knowledge-base.delete_knowledge_bases_crawler` · `Version: v3`

*Request body*: [`DeleteWebsiteUrlRequestDTO`](#deletewebsiteurlrequestdto)

*Response*: [`DeleteWebsiteUrlResponseDTO`](#deletewebsiteurlresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:knowledge-base.delete_knowledge_bases_crawler",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /knowledge-bases/crawler`

**Get all trained page links by knowledge base**

Operation id: `v3:knowledge-base.get_knowledge_bases_crawler` · `Version: v3`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `knowledgeBaseId` | string | **yes** | knowledge base ID as string |
| `locationId` | string | **yes** | location ID as string |
| `page` | number | no | Page number |
| `pageLength` | number | no | Records per page |
| `query` | string | no | query to filter on url links |

*Response*: [`GetAllUrlsByKnowledgeBaseResponseDTO`](#getallurlsbyknowledgebaseresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:knowledge-base.get_knowledge_bases_crawler",
    "query": {
      "knowledgeBaseId": "<knowledgeBaseId>",
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /knowledge-bases/crawler`

**Start crawling and discover pages for training**

Operation id: `v3:knowledge-base.post_knowledge_bases_crawler` · `Version: v3`

*Request body*: [`DiscoverWebsiteRequestDTO`](#discoverwebsiterequestdto)

*Response*: [`DiscoverWebsiteResponseDTO`](#discoverwebsiteresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:knowledge-base.post_knowledge_bases_crawler",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /knowledge-bases/crawler/status`

**Get crawling status for the latest operation**

Operation id: `v3:knowledge-base.get_knowledge_bases_crawler_status` · `Version: v3`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID as string |
| `operationId` | string | **yes** | operation id as string |
| `knowledgeBaseId` | string | **yes** | knowledge base id |

*Response*: [`CrawlingStatusResponseDTO`](#crawlingstatusresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:knowledge-base.get_knowledge_bases_crawler_status",
    "query": {
      "locationId": "<locationId>",
      "operationId": "<operationId>",
      "knowledgeBaseId": "<knowledgeBaseId>"
    }
  }
}
```

</details>

#### `POST /knowledge-bases/crawler/train`

**Train discovered website pages and ingest into the knowledge base**

Operation id: `v3:knowledge-base.post_knowledge_bases_crawler_train` · `Version: v3`

*Request body*: [`TrainDiscoveredUrlsDTO`](#traindiscoveredurlsdto)

*Response*: [`TrainDiscoveredUrlsResponseDTO`](#traindiscoveredurlsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:knowledge-base.post_knowledge_bases_crawler_train",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /knowledge-bases/faqs`

**Get all FAQs by knowledge base with pagination support**

Retrieves FAQs for a knowledge base. Supports pagination using limit and lastFaqId parameters.

Operation id: `v3:knowledge-base.get_knowledge_bases_faqs` · `Version: v3`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `knowledgeBaseId` | string | **yes** | knowledge base ID as string |
| `locationId` | string | **yes** | location ID as string |
| `limit` | number | no | Limit the number of FAQs returned |
| `lastFaqId` | string | no | Last FAQ ID for pagination (cursor-based) |

*Response*: [`ListFaqsResponseDTO`](#listfaqsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:knowledge-base.get_knowledge_bases_faqs",
    "query": {
      "knowledgeBaseId": "<knowledgeBaseId>",
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /knowledge-bases/faqs`

**Create a new FAQ inside knowledge base**

Operation id: `v3:knowledge-base.post_knowledge_bases_faqs` · `Version: v3`

*Request body*: [`AddFaqDTO`](#addfaqdto)

*Response*: [`CreateFaqResponseDTO`](#createfaqresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:knowledge-base.post_knowledge_bases_faqs",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /knowledge-bases/faqs/{id}`

**Delete an existing knowledge base FAQ**

Operation id: `v3:knowledge-base.delete_knowledge_bases_faqs_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | faq ID as string |

*Response*: [`DeleteFaqResponseDTO`](#deletefaqresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:knowledge-base.delete_knowledge_bases_faqs_by_id",
    "path_params": {
      "id": "<id>"
    }
  }
}
```

</details>

#### `PUT /knowledge-bases/faqs/{id}`

**Update an existing knowledge base FAQ**

Operation id: `v3:knowledge-base.put_knowledge_bases_faqs_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | faq ID as string |

*Request body*: [`UpdateFaqBodyDTO`](#updatefaqbodydto)

*Response*: [`UpdateFaqResponseDTO`](#updatefaqresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:knowledge-base.put_knowledge_bases_faqs_by_id",
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

#### `PUT /knowledge-bases/{id}`

**Update a knowledge base**

Operation id: `v3:knowledge-base.put_knowledge_bases_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | — |

*Request body*: [`UpdateKnowledgeBaseDTO`](#updateknowledgebasedto)

*Response*: [`UpdateKnowledgeBaseResponseDTO`](#updateknowledgebaseresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:knowledge-base.put_knowledge_bases_by_id",
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

#### `DELETE /knowledge-bases/{knowledgeBaseId}`

**Delete a knowledge base**

Operation id: `v3:knowledge-base.delete_knowledge_bases_by_knowledgeBaseId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `knowledgeBaseId` | string | **yes** | — |

*Response*: [`DeleteKnowledgeBaseResponseDTO`](#deleteknowledgebaseresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:knowledge-base.delete_knowledge_bases_by_knowledgeBaseId",
    "path_params": {
      "knowledgeBaseId": "<knowledgeBaseId>"
    }
  }
}
```

</details>

#### `GET /knowledge-bases/{knowledgeBaseId}`

**Get knowledge base by ID**

Operation id: `v3:knowledge-base.get_knowledge_bases_by_knowledgeBaseId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `knowledgeBaseId` | string | **yes** | — |

*Response*: [`GetKnowledgeBaseByIdResponseDTO`](#getknowledgebasebyidresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:knowledge-base.get_knowledge_bases_by_knowledgeBaseId",
    "path_params": {
      "knowledgeBaseId": "<knowledgeBaseId>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::knowledge_base::*` (enable the `knowledge-base` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/knowledge_base/).

### `AddFaqDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | location ID as string |
| `question` | String | **yes** | faq question as a string |
| `answer` | String | **yes** | faq answer as a string |
| `knowledgeBaseId` | String | **yes** | knowledge base ID as string |

### `BadRequestDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | — |
| `message` | String | no | — |

### `CrawledUrlDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the URL |
| `url` | String | **yes** | The actual URL that was crawled |
| `title` | String | **yes** | Title of the webpage |
| `status` | String — `Pending`, `Processing`, `Successful`, `Failed`, `Existing`, `Restricted`, `Cancelled`, `Aborted`, `Training` | **yes** | Current processing status of the URL |
| `locationId` | String | **yes** | Location ID associated with this URL |
| `knowledgeBaseId` | String | **yes** | Knowledge base ID this URL belongs to |
| `content` | String | **yes** | URL to the stored content file |
| `contentEditedByUser` | bool | **yes** | Whether the content was edited by user |
| `updatedAt` | String | **yes** | Last updated timestamp |

### `CrawlingAggregateDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String — `Pending`, `Processing`, `Successful`, `Failed`, `Existing`, `Restricted`, `Cancelled`, `Aborted`, `Training` | **yes** | Status grouping identifier |
| `records` | Vec<CrawlingRecordDTO> | **yes** | Array of records for this status |

### `CrawlingRecordDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | String | **yes** | URL being crawled |
| `id` | String | **yes** | Unique record identifier |
| `title` | String | no | Page title (for successful/pending records) |
| `error` | [`ErrorDetailsDTO`](#errordetailsdto) | no | Error details (for failed records) |

### `CrawlingStatusDataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `aggregate` | Vec<CrawlingAggregateDTO> | **yes** | Aggregated crawling results by status |
| `operationDetails` | [`OperationDetailsDTO`](#operationdetailsdto) | **yes** | Detailed operation information |

### `CrawlingStatusResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Indicates if the operation was successful |
| `data` | [`CrawlingStatusDataDTO`](#crawlingstatusdatadto) | **yes** | Detailed crawling status data |

### `CreateFaqResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status of the operation |
| `faq` | [`FaqResponseDTO`](#faqresponsedto) | **yes** | Created FAQ details |

### `CreateKnowledgeBaseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | — |
| `description` | String | no | — |
| `locationId` | String | **yes** | — |

### `CreateKnowledgeBaseResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status of the operation |
| `data` | [`KnowledgeBaseDataDTO`](#knowledgebasedatadto) | **yes** | Created knowledge base details |

### `DeleteFaqResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status of the delete operation |

### `DeleteKnowledgeBaseResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |

### `DeleteWebsiteUrlRequestDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `knowledgeBaseId` | String | **yes** | knowledge base ID as string |
| `locationId` | String | **yes** | location ID as string |
| `urlIds` | Vec<String> | **yes** | List of trained urls ids ( fetched from the Get all trained page links by knowledge base endpoint) |

### `DeleteWebsiteUrlResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Indicates if the operation was successful |

### `DiscoverWebsiteDataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `operationId` | String | **yes** | Operation ID for tracking the discovery process |
| `status` | String — `Pending`, `Processing`, `Successful`, `Failed`, `Existing`, `Restricted`, `Cancelled`, `Aborted`, `Training` | **yes** | Current status of the website discovery operation |
| `url` | String | **yes** | The URL being discovered/crawled |

### `DiscoverWebsiteRequestDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID as string |
| `url` | String | **yes** | Website URL as string |
| `option` | String — `Exact`, `Path`, `Domain` | **yes** | Mode as string |
| `knowledgeBaseId` | String | **yes** | knowledge base ID as string |

### `DiscoverWebsiteResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Indicates if the operation was successful |
| `data` | [`DiscoverWebsiteDataDTO`](#discoverwebsitedatadto) | **yes** | Data containing operation details |

### `ErrorDetailsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `stack` | String | **yes** | Error stack trace |
| `response` | String | **yes** | Error response message |
| `status` | f64 | **yes** | HTTP status code |
| `options` | JSON | no | Additional options (nullable) |
| `message` | String | **yes** | Error message |
| `name` | String | **yes** | Error name/type |

### `FaqResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | FAQ ID as string |
| `question` | String | **yes** | FAQ question |
| `questionLowerCase` | String | **yes** | FAQ question in lowercase |
| `answer` | String | **yes** | FAQ answer |
| `knowledgeBaseId` | String | **yes** | Knowledge base ID |
| `locationId` | String | **yes** | Location ID |
| `trainedUrlId` | String | **yes** | Trained URL ID |
| `deleted` | bool | **yes** | Whether the FAQ is deleted |
| `createdAt` | String | **yes** | Date when FAQ was created |
| `updatedAt` | String | **yes** | Date when FAQ was last updated |

### `GetAllKnowledgeBasesPaginatedDataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `knowledgeBases` | Vec<KnowledgeBaseListItemDTO> | **yes** | Array of knowledge bases |
| `activeCount` | f64 | **yes** | Total count of all active knowledge bases |
| `hasMore` | bool | **yes** | Whether there are more knowledge bases available |
| `lastKnowledgeBaseId` | String | no | ID of the last knowledge base in this page (use for next page request) |

### `GetAllKnowledgeBasesPaginatedResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status of the operation |
| `data` | [`GetAllKnowledgeBasesPaginatedDataDTO`](#getallknowledgebasespaginateddatadto) | **yes** | Paginated knowledge bases data |

### `GetAllUrlsByKnowledgeBaseResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `count` | f64 | **yes** | Total count of URLs in the knowledge base |
| `urls` | Vec<CrawledUrlDTO> | **yes** | Array of crawled URLs with their details |

### `GetKnowledgeBaseByIdDataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Knowledge base ID |
| `name` | String | **yes** | Knowledge base name |
| `nameLowerCase` | String | **yes** | Knowledge base name in lowercase |
| `locationId` | String | **yes** | Location ID |
| `deleted` | bool | **yes** | Whether the knowledge base is deleted |
| `createdAt` | String | **yes** | Date when knowledge base was created |
| `updatedAt` | String | **yes** | Date when knowledge base was last updated |
| `kbMetadata` | [`KnowledgeBaseMetadataDTO`](#knowledgebasemetadatadto) | **yes** | Knowledge base metadata with content counts |
| `isDefault` | bool | no | Whether the knowledge base is default or not |

### `GetKnowledgeBaseByIdResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status of the operation |
| `data` | [`GetKnowledgeBaseByIdDataDTO`](#getknowledgebasebyiddatadto) | **yes** | Knowledge base details |

### `InternalServerErrorDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | — |
| `message` | String | no | — |

### `KnowledgeBaseDataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Knowledge base ID |
| `name` | String | **yes** | Knowledge base name |
| `nameLowerCase` | String | **yes** | Knowledge base name in lowercase |
| `locationId` | String | **yes** | Location ID |
| `kbMetadata` | JSON | **yes** | Knowledge base metadata |
| `deleted` | bool | **yes** | Whether the knowledge base is deleted |
| `createdAt` | String | **yes** | Date when knowledge base was created |
| `updatedAt` | String | **yes** | Date when knowledge base was last updated |

### `KnowledgeBaseListItemDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Knowledge base ID |
| `name` | String | **yes** | Knowledge base name |
| `createdAt` | String | **yes** | Date when knowledge base was created |

### `KnowledgeBaseMetadataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `faqs` | f64 | **yes** | Number of FAQs in the knowledge base |
| `urls` | f64 | **yes** | Number of URLs in the knowledge base |
| `richText` | f64 | **yes** | Number of rich text documents in the knowledge base |
| `files` | f64 | **yes** | Number of files in the knowledge base |
| `webSearches` | f64 | **yes** | Number of web searche configs in the knowledge base |
| `tables` | f64 | **yes** | Number of tables in the knowledge base |

### `ListFaqsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `count` | f64 | **yes** | Total count of all FAQs in the knowledge base |
| `faqs` | Vec<FaqResponseDTO> | **yes** | Array of FAQ objects |
| `lastFaqId` | String | no | Last FAQ ID for pagination (use as lastFaqId in next request) |
| `hasMore` | bool | no | Whether there are more FAQs available |

### `OperationDetailsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `discoveredUrlsCount` | f64 | **yes** | Number of URLs discovered |
| `trainedUrlsCount` | f64 | **yes** | Number of URLs successfully trained |
| `_id` | String | **yes** | Operation unique identifier |
| `locationId` | String | **yes** | Associated location ID |
| `status` | String — `Pending`, `Processing`, `Successful`, `Failed`, `Existing`, `Restricted`, `Cancelled`, `Aborted`, `Training` | **yes** | Current operation status |
| `url` | String | **yes** | Base URL being crawled |
| `mode` | String — `Exact`, `Path`, `Domain` | **yes** | Crawling mode used |
| `knowledgeBaseId` | String | **yes** | Knowledge base ID |
| `createdAt` | String | **yes** | Operation creation timestamp |
| `updatedAt` | String | **yes** | Last update timestamp |
| `__v` | f64 | **yes** | Version field |
| `robotsFileData` | String | no | Robots.txt file content |

### `TrainDiscoveredUrlsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID as string |
| `urlIds` | Vec<String> | **yes** | List of Object ids of the discovered urls |
| `knowledgeBaseId` | String | **yes** | knowledge base id |
| `operationId` | String | **yes** | operation id as string |

### `TrainDiscoveredUrlsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Indicates if the operation was successful |

### `UnauthorizedDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | — |
| `message` | String | no | — |
| `error` | String | no | — |

### `UnprocessableDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | — |
| `message` | Vec<String> | no | — |
| `error` | String | no | — |

### `UpdateFaqBodyDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `question` | String | **yes** | faq question as a string |
| `answer` | String | **yes** | faq answer as a string |

### `UpdateFaqResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status of the update operation |

### `UpdateKnowledgeBaseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | field to update the name of the knowledge base |
| `description` | String | no | field to update the description of the knowledge base |

### `UpdateKnowledgeBaseResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |

## Data models — API v3

In Rust: `ghl_models::v3::knowledge_base::*` (enable the `knowledge-base` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/knowledge_base/).

### `AddFaqDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | location ID as string |
| `question` | String | **yes** | faq question as a string |
| `answer` | String | **yes** | faq answer as a string |
| `knowledgeBaseId` | String | **yes** | knowledge base ID as string |

### `BadRequestDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | — |
| `message` | String | no | — |

### `CrawledUrlDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the URL |
| `url` | String | **yes** | The actual URL that was crawled |
| `title` | String | **yes** | Title of the webpage |
| `status` | String — `Pending`, `Processing`, `Successful`, `Failed`, `Existing`, `Restricted`, `Cancelled`, `Aborted`, `Training` | **yes** | Current processing status of the URL |
| `locationId` | String | **yes** | Location ID associated with this URL |
| `knowledgeBaseId` | String | **yes** | Knowledge base ID this URL belongs to |
| `content` | String | **yes** | URL to the stored content file |
| `contentEditedByUser` | bool | **yes** | Whether the content was edited by user |
| `updatedAt` | String | **yes** | Last updated timestamp |

### `CrawlingAggregateDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String — `Pending`, `Processing`, `Successful`, `Failed`, `Existing`, `Restricted`, `Cancelled`, `Aborted`, `Training` | **yes** | Status grouping identifier |
| `records` | Vec<CrawlingRecordDTO> | **yes** | Array of records for this status |

### `CrawlingRecordDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | String | **yes** | URL being crawled |
| `id` | String | **yes** | Unique record identifier |
| `title` | String | no | Page title (for successful/pending records) |
| `error` | [`ErrorDetailsDTO`](#errordetailsdto) | no | Error details (for failed records) |

### `CrawlingStatusDataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `aggregate` | Vec<CrawlingAggregateDTO> | **yes** | Aggregated crawling results by status |
| `operationDetails` | [`OperationDetailsDTO`](#operationdetailsdto) | **yes** | Detailed operation information |

### `CrawlingStatusResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Indicates if the operation was successful |
| `data` | [`CrawlingStatusDataDTO`](#crawlingstatusdatadto) | **yes** | Detailed crawling status data |

### `CreateFaqResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status of the operation |
| `faq` | [`FaqResponseDTO`](#faqresponsedto) | **yes** | Created FAQ details |

### `CreateKnowledgeBaseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | — |
| `description` | String | no | — |
| `locationId` | String | **yes** | — |

### `CreateKnowledgeBaseResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status of the operation |
| `data` | [`KnowledgeBaseDataDTO`](#knowledgebasedatadto) | **yes** | Created knowledge base details |

### `DeleteFaqResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status of the delete operation |

### `DeleteKnowledgeBaseResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |

### `DeleteWebsiteUrlRequestDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `knowledgeBaseId` | String | **yes** | knowledge base ID as string |
| `locationId` | String | **yes** | location ID as string |
| `urlIds` | Vec<String> | **yes** | List of trained urls ids ( fetched from the Get all trained page links by knowledge base endpoint) |

### `DeleteWebsiteUrlResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Indicates if the operation was successful |

### `DiscoverWebsiteDataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `operationId` | String | **yes** | Operation ID for tracking the discovery process |
| `status` | String — `Pending`, `Processing`, `Successful`, `Failed`, `Existing`, `Restricted`, `Cancelled`, `Aborted`, `Training` | **yes** | Current status of the website discovery operation |
| `url` | String | **yes** | The URL being discovered/crawled |

### `DiscoverWebsiteRequestDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID as string |
| `url` | String | **yes** | Website URL as string |
| `option` | String — `Exact`, `Path`, `Domain` | **yes** | Mode as string |
| `knowledgeBaseId` | String | **yes** | knowledge base ID as string |

### `DiscoverWebsiteResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Indicates if the operation was successful |
| `data` | [`DiscoverWebsiteDataDTO`](#discoverwebsitedatadto) | **yes** | Data containing operation details |

### `ErrorDetailsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `stack` | String | **yes** | Error stack trace |
| `response` | String | **yes** | Error response message |
| `status` | f64 | **yes** | HTTP status code |
| `options` | JSON | no | Additional options (nullable) |
| `message` | String | **yes** | Error message |
| `name` | String | **yes** | Error name/type |

### `FaqResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | FAQ ID as string |
| `question` | String | **yes** | FAQ question |
| `questionLowerCase` | String | **yes** | FAQ question in lowercase |
| `answer` | String | **yes** | FAQ answer |
| `knowledgeBaseId` | String | **yes** | Knowledge base ID |
| `locationId` | String | **yes** | Location ID |
| `trainedUrlId` | String | **yes** | Trained URL ID |
| `deleted` | bool | **yes** | Whether the FAQ is deleted |
| `createdAt` | String | **yes** | Date when FAQ was created |
| `updatedAt` | String | **yes** | Date when FAQ was last updated |

### `GetAllKnowledgeBasesPaginatedDataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `knowledgeBases` | Vec<KnowledgeBaseListItemDTO> | **yes** | Array of knowledge bases |
| `activeCount` | f64 | **yes** | Total count of all active knowledge bases |
| `hasMore` | bool | **yes** | Whether there are more knowledge bases available |
| `lastKnowledgeBaseId` | String | no | ID of the last knowledge base in this page (use for next page request) |

### `GetAllKnowledgeBasesPaginatedResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status of the operation |
| `data` | [`GetAllKnowledgeBasesPaginatedDataDTO`](#getallknowledgebasespaginateddatadto) | **yes** | Paginated knowledge bases data |

### `GetAllUrlsByKnowledgeBaseResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `count` | f64 | **yes** | Total count of URLs in the knowledge base |
| `urls` | Vec<CrawledUrlDTO> | **yes** | Array of crawled URLs with their details |

### `GetKnowledgeBaseByIdDataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Knowledge base ID |
| `name` | String | **yes** | Knowledge base name |
| `nameLowerCase` | String | **yes** | Knowledge base name in lowercase |
| `locationId` | String | **yes** | Location ID |
| `deleted` | bool | **yes** | Whether the knowledge base is deleted |
| `createdAt` | String | **yes** | Date when knowledge base was created |
| `updatedAt` | String | **yes** | Date when knowledge base was last updated |
| `kbMetadata` | [`KnowledgeBaseMetadataDTO`](#knowledgebasemetadatadto) | **yes** | Knowledge base metadata with content counts |
| `isDefault` | bool | no | Whether the knowledge base is default or not |

### `GetKnowledgeBaseByIdResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status of the operation |
| `data` | [`GetKnowledgeBaseByIdDataDTO`](#getknowledgebasebyiddatadto) | **yes** | Knowledge base details |

### `InternalServerErrorDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | — |
| `message` | String | no | — |

### `KnowledgeBaseDataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Knowledge base ID |
| `name` | String | **yes** | Knowledge base name |
| `nameLowerCase` | String | **yes** | Knowledge base name in lowercase |
| `locationId` | String | **yes** | Location ID |
| `kbMetadata` | JSON | **yes** | Knowledge base metadata |
| `deleted` | bool | **yes** | Whether the knowledge base is deleted |
| `createdAt` | String | **yes** | Date when knowledge base was created |
| `updatedAt` | String | **yes** | Date when knowledge base was last updated |

### `KnowledgeBaseListItemDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Knowledge base ID |
| `name` | String | **yes** | Knowledge base name |
| `createdAt` | String | **yes** | Date when knowledge base was created |

### `KnowledgeBaseMetadataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `faqs` | f64 | **yes** | Number of FAQs in the knowledge base |
| `urls` | f64 | **yes** | Number of URLs in the knowledge base |
| `richText` | f64 | **yes** | Number of rich text documents in the knowledge base |
| `files` | f64 | **yes** | Number of files in the knowledge base |
| `webSearches` | f64 | **yes** | Number of web searche configs in the knowledge base |
| `tables` | f64 | **yes** | Number of tables in the knowledge base |

### `ListFaqsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `count` | f64 | **yes** | Total count of all FAQs in the knowledge base |
| `faqs` | Vec<FaqResponseDTO> | **yes** | Array of FAQ objects |
| `lastFaqId` | String | no | Last FAQ ID for pagination (use as lastFaqId in next request) |
| `hasMore` | bool | no | Whether there are more FAQs available |

### `OperationDetailsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `discoveredUrlsCount` | f64 | **yes** | Number of URLs discovered |
| `trainedUrlsCount` | f64 | **yes** | Number of URLs successfully trained |
| `_id` | String | **yes** | Operation unique identifier |
| `locationId` | String | **yes** | Associated location ID |
| `status` | String — `Pending`, `Processing`, `Successful`, `Failed`, `Existing`, `Restricted`, `Cancelled`, `Aborted`, `Training` | **yes** | Current operation status |
| `url` | String | **yes** | Base URL being crawled |
| `mode` | String — `Exact`, `Path`, `Domain` | **yes** | Crawling mode used |
| `knowledgeBaseId` | String | **yes** | Knowledge base ID |
| `createdAt` | String | **yes** | Operation creation timestamp |
| `updatedAt` | String | **yes** | Last update timestamp |
| `__v` | f64 | **yes** | Version field |
| `robotsFileData` | String | no | Robots.txt file content |

### `TrainDiscoveredUrlsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID as string |
| `urlIds` | Vec<String> | **yes** | List of Object ids of the discovered urls |
| `knowledgeBaseId` | String | **yes** | knowledge base id |
| `operationId` | String | **yes** | operation id as string |

### `TrainDiscoveredUrlsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Indicates if the operation was successful |

### `UnauthorizedDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | — |
| `message` | String | no | — |
| `error` | String | no | — |

### `UnprocessableDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | — |
| `message` | Vec<String> | no | — |
| `error` | String | no | — |

### `UpdateFaqBodyDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `question` | String | **yes** | faq question as a string |
| `answer` | String | **yes** | faq answer as a string |

### `UpdateFaqResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status of the update operation |

### `UpdateKnowledgeBaseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | field to update the name of the knowledge base |
| `description` | String | no | field to update the description of the knowledge base |

### `UpdateKnowledgeBaseResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |

