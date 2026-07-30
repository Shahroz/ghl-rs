# `associations`

**10** operations / **5** models in API v2 · **10** operations / **5** models in API v3

## How to call it

No hand-written service yet — reach these endpoints two ways:

**From Rust** (typed body via [`ghl-models`](https://docs.rs/ghl-models)):

```rust,ignore
// cargo add ghl-models --features associations
use ghl_models::v2::associations::*;

let body = serde_json::to_value(/* a Create…Dto from above */)?;
let out = ghl.request_raw("POST", "/path/", &[], Some(&body), None).await?;
```

**From an AI agent** (MCP meta-tools):

```json
{
  "name": "ghl_search_operations",
  "arguments": {
    "query": "",
    "module": "associations"
  }
}
```

## Endpoints — API v2

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/associations/` | Get all associations for a sub-account / location | `associations.get_associations` |
| `POST` | `/associations/` | Create Association | `associations.post_associations` |
| `GET` | `/associations/key/{key_name}` | Get association key by key name | `associations.get_associations_key_by_key_name` |
| `GET` | `/associations/objectKey/{objectKey}` | Get association by object keys | `associations.get_associations_objectKey_by_objectKey` |
| `POST` | `/associations/relations` | Create Relation for you associated entities. | `associations.post_associations_relations` |
| `GET` | `/associations/relations/{recordId}` | Get all relations By record Id | `associations.get_associations_relations_by_recordId` |
| `DELETE` | `/associations/relations/{relationId}` | Delete Relation | `associations.delete_associations_relations_by_relationId` |
| `DELETE` | `/associations/{associationId}` | Delete Association | `associations.delete_associations_by_associationId` |
| `GET` | `/associations/{associationId}` | Get association by ID | `associations.get_associations_by_associationId` |
| `PUT` | `/associations/{associationId}` | Update Association By Id | `associations.put_associations_by_associationId` |

### Endpoint details — v2

#### `GET /associations/`

**Get all associations for a sub-account / location**

Get all Associations

Operation id: `associations.get_associations` · `Version: 2021-07-28` · Scopes: `associations.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `skip` | number | **yes** | — |
| `limit` | number | **yes** | — |

*Response*: [`GetPostSuccessfulResponseDto`](#getpostsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "associations.get_associations",
    "query": {
      "locationId": "<locationId>",
      "skip": "<skip>",
      "limit": "<limit>"
    }
  }
}
```

</details>

#### `POST /associations/`

**Create Association**

Allow you to create contact - contact , contact - custom objects associations, will add more in the future.Documentation Link - https://doc.clickup.com/8631005/d/h/87cpx-293776/cd0f4122abc04d3

Operation id: `associations.post_associations` · `Version: 2021-07-28` · Scopes: `associations.write`

*Request body*: [`createAssociationReqDto`](#createassociationreqdto)

*Response*: [`GetPostSuccessfulResponseDto`](#getpostsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "associations.post_associations",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /associations/key/{key_name}`

**Get association key by key name**

Using this api you can get standard / user defined association by key

Operation id: `associations.get_associations_key_by_key_name` · `Version: 2021-07-28` · Scopes: `associations.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `key_name` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Response*: [`GetPostSuccessfulResponseDto`](#getpostsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "associations.get_associations_key_by_key_name",
    "path_params": {
      "key_name": "<key_name>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /associations/objectKey/{objectKey}`

**Get association by object keys**

Get association by object keys like contacts, custom objects and opportunities. Documentation Link - https://doc.clickup.com/8631005/d/h/87cpx-293776/cd0f4122abc04d3

Operation id: `associations.get_associations_objectKey_by_objectKey` · `Version: 2021-07-28` · Scopes: `associations.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `objectKey` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | no | — |

*Response*: [`GetPostSuccessfulResponseDto`](#getpostsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "associations.get_associations_objectKey_by_objectKey",
    "path_params": {
      "objectKey": "<objectKey>"
    }
  }
}
```

</details>

#### `POST /associations/relations`

**Create Relation for you associated entities.**

Create Relation.Documentation Link - https://doc.clickup.com/8631005/d/h/87cpx-293776/cd0f4122abc04d3

Operation id: `associations.post_associations_relations` · `Version: 2021-07-28` · Scopes: `associations/relation.write`

*Request body*: [`createRelationReqDto`](#createrelationreqdto)

*Response*: [`GetPostSuccessfulResponseDto`](#getpostsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "associations.post_associations_relations",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /associations/relations/{recordId}`

**Get all relations By record Id**

Get all relations by record Id

Operation id: `associations.get_associations_relations_by_recordId` · `Version: 2021-07-28` · Scopes: `associations/relation.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `recordId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Your Sub Account's ID |
| `skip` | number | **yes** | — |
| `limit` | number | **yes** | — |
| `associationIds` | array | no | Association Ids |

*Response*: [`GetPostSuccessfulResponseDto`](#getpostsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "associations.get_associations_relations_by_recordId",
    "path_params": {
      "recordId": "<recordId>"
    },
    "query": {
      "locationId": "<locationId>",
      "skip": "<skip>",
      "limit": "<limit>"
    }
  }
}
```

</details>

#### `DELETE /associations/relations/{relationId}`

**Delete Relation**

Operation id: `associations.delete_associations_relations_by_relationId` · `Version: 2021-07-28` · Scopes: `associations/relation.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `relationId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Your Sub Account's ID |

*Response*: [`GetPostSuccessfulResponseDto`](#getpostsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "associations.delete_associations_relations_by_relationId",
    "path_params": {
      "relationId": "<relationId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `DELETE /associations/{associationId}`

**Delete Association**

Delete USER_DEFINED Association By Id, deleting an association will also all the relations for that association

Operation id: `associations.delete_associations_by_associationId` · `Version: 2021-07-28` · Scopes: `associations.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `associationId` | string | **yes** | — |

*Response*: [`DeleteAssociationsResponseDTO`](#deleteassociationsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "associations.delete_associations_by_associationId",
    "path_params": {
      "associationId": "<associationId>"
    }
  }
}
```

</details>

#### `GET /associations/{associationId}`

**Get association by ID**

Using this api you can get SYSTEM_DEFINED / USER_DEFINED association by id

Operation id: `associations.get_associations_by_associationId` · `Version: 2021-07-28` · Scopes: `associations.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `associationId` | string | **yes** | — |

*Response*: [`GetPostSuccessfulResponseDto`](#getpostsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "associations.get_associations_by_associationId",
    "path_params": {
      "associationId": "<associationId>"
    }
  }
}
```

</details>

#### `PUT /associations/{associationId}`

**Update Association By Id**

Update Association , Allows you to update labels of an associations. Documentation Link - https://doc.clickup.com/8631005/d/h/87cpx-293776/cd0f4122abc04d3

Operation id: `associations.put_associations_by_associationId` · `Version: 2021-07-28` · Scopes: `associations.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `associationId` | string | **yes** | — |

*Request body*: [`UpdateAssociationReqDto`](#updateassociationreqdto)

*Response*: [`GetPostSuccessfulResponseDto`](#getpostsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "associations.put_associations_by_associationId",
    "path_params": {
      "associationId": "<associationId>"
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
| `GET` | `/associations/` | Get all associations for a sub-account / location | `v3:associations.get_associations` |
| `POST` | `/associations/` | Create Association | `v3:associations.post_associations` |
| `GET` | `/associations/key/{key_name}` | Get association key by key name | `v3:associations.get_associations_key_by_key_name` |
| `GET` | `/associations/objectKey/{objectKey}` | Get association by object keys | `v3:associations.get_associations_objectKey_by_objectKey` |
| `POST` | `/associations/relations` | Create Relation for you associated entities. | `v3:associations.post_associations_relations` |
| `GET` | `/associations/relations/{recordId}` | Get all relations By record Id | `v3:associations.get_associations_relations_by_recordId` |
| `DELETE` | `/associations/relations/{relationId}` | Delete Relation | `v3:associations.delete_associations_relations_by_relationId` |
| `DELETE` | `/associations/{associationId}` | Delete Association | `v3:associations.delete_associations_by_associationId` |
| `GET` | `/associations/{associationId}` | Get association by ID | `v3:associations.get_associations_by_associationId` |
| `PUT` | `/associations/{associationId}` | Update Association By Id | `v3:associations.put_associations_by_associationId` |

### Endpoint details — v3

#### `GET /associations/`

**Get all associations for a sub-account / location**

Get all Associations

Operation id: `v3:associations.get_associations` · `Version: v3` · Scopes: `associations.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `skip` | number | **yes** | — |
| `limit` | number | **yes** | — |

*Response*: [`GetPostSuccessfulResponseDto`](#getpostsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:associations.get_associations",
    "query": {
      "locationId": "<locationId>",
      "skip": "<skip>",
      "limit": "<limit>"
    }
  }
}
```

</details>

#### `POST /associations/`

**Create Association**

Allow you to create contact - contact , contact - custom objects associations, will add more in the future.Documentation Link - https://doc.clickup.com/8631005/d/h/87cpx-293776/cd0f4122abc04d3

Operation id: `v3:associations.post_associations` · `Version: v3` · Scopes: `associations.write`

*Request body*: [`createAssociationReqDto`](#createassociationreqdto)

*Response*: [`GetPostSuccessfulResponseDto`](#getpostsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:associations.post_associations",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /associations/key/{key_name}`

**Get association key by key name**

Using this api you can get standard / user defined association by key

Operation id: `v3:associations.get_associations_key_by_key_name` · `Version: v3` · Scopes: `associations.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `key_name` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Response*: [`GetPostSuccessfulResponseDto`](#getpostsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:associations.get_associations_key_by_key_name",
    "path_params": {
      "key_name": "<key_name>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /associations/objectKey/{objectKey}`

**Get association by object keys**

Get association by object keys like contacts, custom objects and opportunities. Documentation Link - https://doc.clickup.com/8631005/d/h/87cpx-293776/cd0f4122abc04d3

Operation id: `v3:associations.get_associations_objectKey_by_objectKey` · `Version: v3` · Scopes: `associations.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `objectKey` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | no | — |

*Response*: [`GetPostSuccessfulResponseDto`](#getpostsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:associations.get_associations_objectKey_by_objectKey",
    "path_params": {
      "objectKey": "<objectKey>"
    }
  }
}
```

</details>

#### `POST /associations/relations`

**Create Relation for you associated entities.**

Create Relation.Documentation Link - https://doc.clickup.com/8631005/d/h/87cpx-293776/cd0f4122abc04d3

Operation id: `v3:associations.post_associations_relations` · `Version: v3` · Scopes: `associations/relation.write`

*Request body*: [`createRelationReqDto`](#createrelationreqdto)

*Response*: [`GetPostSuccessfulResponseDto`](#getpostsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:associations.post_associations_relations",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /associations/relations/{recordId}`

**Get all relations By record Id**

Get all relations by record Id

Operation id: `v3:associations.get_associations_relations_by_recordId` · `Version: v3` · Scopes: `associations/relation.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `recordId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Your Sub Account's ID |
| `skip` | number | **yes** | — |
| `limit` | number | **yes** | — |
| `associationIds` | array | no | Association Ids |

*Response*: [`GetPostSuccessfulResponseDto`](#getpostsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:associations.get_associations_relations_by_recordId",
    "path_params": {
      "recordId": "<recordId>"
    },
    "query": {
      "locationId": "<locationId>",
      "skip": "<skip>",
      "limit": "<limit>"
    }
  }
}
```

</details>

#### `DELETE /associations/relations/{relationId}`

**Delete Relation**

Operation id: `v3:associations.delete_associations_relations_by_relationId` · `Version: v3` · Scopes: `associations/relation.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `relationId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Your Sub Account's ID |

*Response*: [`GetPostSuccessfulResponseDto`](#getpostsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:associations.delete_associations_relations_by_relationId",
    "path_params": {
      "relationId": "<relationId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `DELETE /associations/{associationId}`

**Delete Association**

Delete USER_DEFINED Association By Id, deleting an association will also all the relations for that association

Operation id: `v3:associations.delete_associations_by_associationId` · `Version: v3` · Scopes: `associations.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `associationId` | string | **yes** | — |

*Response*: [`DeleteAssociationsResponseDTO`](#deleteassociationsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:associations.delete_associations_by_associationId",
    "path_params": {
      "associationId": "<associationId>"
    }
  }
}
```

</details>

#### `GET /associations/{associationId}`

**Get association by ID**

Using this api you can get SYSTEM_DEFINED / USER_DEFINED association by id

Operation id: `v3:associations.get_associations_by_associationId` · `Version: v3` · Scopes: `associations.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `associationId` | string | **yes** | — |

*Response*: [`GetPostSuccessfulResponseDto`](#getpostsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:associations.get_associations_by_associationId",
    "path_params": {
      "associationId": "<associationId>"
    }
  }
}
```

</details>

#### `PUT /associations/{associationId}`

**Update Association By Id**

Update Association , Allows you to update labels of an associations. Documentation Link - https://doc.clickup.com/8631005/d/h/87cpx-293776/cd0f4122abc04d3

Operation id: `v3:associations.put_associations_by_associationId` · `Version: v3` · Scopes: `associations.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `associationId` | string | **yes** | — |

*Request body*: [`UpdateAssociationReqDto`](#updateassociationreqdto)

*Response*: [`GetPostSuccessfulResponseDto`](#getpostsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:associations.put_associations_by_associationId",
    "path_params": {
      "associationId": "<associationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::associations::*` (enable the `associations` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/associations/).

### `DeleteAssociationsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `deleted` | bool | **yes** | Deletion status |
| `id` | String | **yes** | Association Id |
| `message` | String | **yes** | — |

### `GetPostSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | — |
| `id` | String | **yes** | — |
| `key` | String | **yes** | First Objects Association Label (custom_objects.children) |
| `firstObjectLabel` | JSON | **yes** | First Objects Association Label (custom_objects.children) |
| `firstObjectKey` | JSON | **yes** | First Objects Key |
| `secondObjectLabel` | JSON | **yes** | Second Object Association Label (contact) |
| `secondObjectKey` | JSON | **yes** | Second Objects Key |
| `associationType` | JSON | **yes** | Association Type can be USER_DEFINED or SYSTEM_DEFINED |

### `UpdateAssociationReqDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `firstObjectLabel` | JSON | **yes** | — |
| `secondObjectLabel` | JSON | **yes** | — |

### `createAssociationReqDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | — |
| `key` | String | **yes** | Association's Unique key |
| `firstObjectLabel` | JSON | **yes** | First Objects Association Label (custom_objects.children) |
| `firstObjectKey` | JSON | **yes** | First Objects Key |
| `secondObjectLabel` | JSON | **yes** | Second Object Association Label (contact) |
| `secondObjectKey` | JSON | **yes** | Second Objects Key |

### `createRelationReqDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Your Sub Account's ID |
| `associationId` | String | **yes** | Association's Id |
| `firstRecordId` | String | **yes** | First Record's Id. For instance, if you have an association between a contact and a custom object, and you specify the contact as the first object while creating the association, then your firstRecord… |
| `secondRecordId` | String | **yes** | Second Record's Id.For instance, if you have an association between a contact and a custom object, and you specify the custom object as the second entity while creating the association, then your seco… |

## Data models — API v3

In Rust: `ghl_models::v3::associations::*` (enable the `associations` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/associations/).

### `DeleteAssociationsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `deleted` | bool | **yes** | Deletion status |
| `id` | String | **yes** | Association Id |
| `message` | String | **yes** | — |

### `GetPostSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | — |
| `id` | String | **yes** | — |
| `key` | String | **yes** | First Objects Association Label (custom_objects.children) |
| `firstObjectLabel` | JSON | **yes** | First Objects Association Label (custom_objects.children) |
| `firstObjectKey` | JSON | **yes** | First Objects Key |
| `secondObjectLabel` | JSON | **yes** | Second Object Association Label (contact) |
| `secondObjectKey` | JSON | **yes** | Second Objects Key |
| `associationType` | JSON | **yes** | Association Type can be USER_DEFINED or SYSTEM_DEFINED |

### `UpdateAssociationReqDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `firstObjectLabel` | JSON | **yes** | — |
| `secondObjectLabel` | JSON | **yes** | — |

### `createAssociationReqDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | — |
| `key` | String | **yes** | Association's Unique key |
| `firstObjectLabel` | JSON | **yes** | First Objects Association Label (custom_objects.children) |
| `firstObjectKey` | JSON | **yes** | First Objects Key |
| `secondObjectLabel` | JSON | **yes** | Second Object Association Label (contact) |
| `secondObjectKey` | JSON | **yes** | Second Objects Key |

### `createRelationReqDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Your Sub Account's ID |
| `associationId` | String | **yes** | Association's Id |
| `firstRecordId` | String | **yes** | First Record's Id. For instance, if you have an association between a contact and a custom object, and you specify the contact as the first object while creating the association, then your firstRecord… |
| `secondRecordId` | String | **yes** | Second Record's Id.For instance, if you have an association between a contact and a custom object, and you specify the custom object as the second entity while creating the association, then your seco… |

