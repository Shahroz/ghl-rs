# `objects`

**9** operations / **20** models in API v2 · **9** operations / **20** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `objects` cargo feature on `ghl-sdk`, then call any of the 9 generated methods on `ghl.objects()`:

```toml
ghl-sdk = { version = "0.4", features = ["objects"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/objects/` | Get all objects for a location | `get_all_objects_for_a_location()` | `objects.get_objects` |
| `POST` | `/objects/` | Create Custom Object | `create_custom_object()` | `objects.post_objects` |
| `GET` | `/objects/{key}` | Get Object Schema by key / id | `get_object_schema_by_key_id()` | `objects.get_objects_by_key` |
| `PUT` | `/objects/{key}` | Update Object Schema By Key / Id | `update_object_schema_by_key_id()` | `objects.put_objects_by_key` |
| `POST` | `/objects/{schemaKey}/records` | Create Record | `create_record()` | `objects.post_objects_by_schemaKey_records` |
| `POST` | `/objects/{schemaKey}/records/search` | Search Object Records | `search_object_records()` | `objects.post_objects_by_schemaKey_records_search` |
| `DELETE` | `/objects/{schemaKey}/records/{id}` | Delete Record | `delete_record()` | `objects.delete_objects_by_schemaKey_records_by_id` |
| `GET` | `/objects/{schemaKey}/records/{id}` | Get Record By Id | `get_record_by_id()` | `objects.get_objects_by_schemaKey_records_by_id` |
| `PUT` | `/objects/{schemaKey}/records/{id}` | Update Record | `update_record()` | `objects.put_objects_by_schemaKey_records_by_id` |

### Endpoint details — v2

#### `GET /objects/`

**Get all objects for a location**

Get all objects for a location. Supported Objects are contact, opportunity, business and custom objects.To understand objects and records, please have a look at the documentation here : https://doc.clickup.com/8631005/d/h/87cpx-277156/93bf0c2e23177b0

Operation id: `objects.get_objects` · `Version: 2021-07-28` · Scopes: `objects/schema.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | location id |

*Response*: [`CustomObjectListResponseDTO`](#customobjectlistresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::objects::GetAllObjectsForALocationParams;

let params = GetAllObjectsForALocationParams::new("locationId");
let out = ghl.objects().get_all_objects_for_a_location(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "objects.get_objects",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /objects/`

**Create Custom Object**

Allows you to create a custom object schema. To understand objects and records, please have a look at the documentation here : https://doc.clickup.com/8631005/d/h/87cpx-277156/93bf0c2e23177b0

Operation id: `objects.post_objects` · `Version: 2021-07-28` · Scopes: `objects/schema.write`

*Request body*: [`CreateCustomObjectSchemaDTO`](#createcustomobjectschemadto)

*Response*: [`CustomObjectResponseDTO`](#customobjectresponsedto)

*Rust*:

```rust,ignore
let out = ghl.objects().create_custom_object(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "objects.post_objects",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /objects/{key}`

**Get Object Schema by key / id**

Retrieve Object Schema by key or ID. This will return the schema of the custom object, including all its fields and properties. Supported objects include contact, opportunity, business and custom objects.To understand objects and records, please have a look the documentation here : https://doc.clickup.com/8631005/d/h/87cpx-277156/93bf0c2e23177b0

Operation id: `objects.get_objects_by_key` · `Version: 2021-07-28` · Scopes: `objects/schema.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `key` | string | **yes** | key of the custom or standard object. For custom objects, the key must include the prefix “custom_objects.”. This key can be found on the Object Details page un… |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | location id of the sub account |
| `fetchProperties` | string | no | Fetch Properties , Fetches all the standard / custom fields of the object when set to true |

*Response*: [`CustomObjectByIdResponseDTO`](#customobjectbyidresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::objects::GetObjectSchemaByKeyIdParams;

let params = GetObjectSchemaByKeyIdParams::new("locationId");
let out = ghl.objects().get_object_schema_by_key_id(&key, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "objects.get_objects_by_key",
    "path_params": {
      "key": "<key>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /objects/{key}`

**Update Object Schema By Key / Id**

Update Custom Object Schema or standard object's like contact, opportunity, business searchable fields. To understand objects and records, please have a look at the documentation here : https://doc.clickup.com/8631005/d/h/87cpx-277156/93bf0c2e23177b0

Operation id: `objects.put_objects_by_key` · `Version: 2021-07-28` · Scopes: `objects/schema.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `key` | string | **yes** | key of the custom or standard object. For custom objects, the key must include the prefix “custom_objects.”. This key can be found on the Object Details page un… |

*Request body*: [`UpdateCustomObjectSchemaDTO`](#updatecustomobjectschemadto)

*Response*: [`CustomObjectResponseDTO`](#customobjectresponsedto)

*Rust*:

```rust,ignore
let out = ghl.objects().update_object_schema_by_key_id(&key, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "objects.put_objects_by_key",
    "path_params": {
      "key": "<key>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /objects/{schemaKey}/records`

**Create Record**

Create a Custom Object Record. Supported Objects business and custom objects. Documentation Link - https://doc.clickup.com/8631005/d/h/87cpx-277156/93bf0c2e23177b0/87cpx-376296

Operation id: `objects.post_objects_by_schemaKey_records` · `Version: 2021-07-28` · Scopes: `objects/record.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `schemaKey` | string | **yes** | The key of the Custom Object / Standard Object Schema. For custom objects, the key must include the “custom_objects.” prefix, while standard objects use their r… |

*Request body*: [`CreateCustomObjectRecordDto`](#createcustomobjectrecorddto)

*Response*: [`RecordByIdResponseDTO`](#recordbyidresponsedto)

*Rust*:

```rust,ignore
let out = ghl.objects().create_record(&schemaKey, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "objects.post_objects_by_schemaKey_records",
    "path_params": {
      "schemaKey": "<schemaKey>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /objects/{schemaKey}/records/search`

**Search Object Records**

Supported Objects are custom objects and standard objects like "business". Documentation Link - https://doc.clickup.com/8631005/d/h/87cpx-277156/93bf0c2e23177b0/87cpx-379336

Operation id: `objects.post_objects_by_schemaKey_records_search` · `Version: 2021-07-28` · Scopes: `objects/record.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `schemaKey` | string | **yes** | custom object key |

*Request body*: [`SearchRecordsBody`](#searchrecordsbody)

*Response*: [`SearchRecordResponseDTO`](#searchrecordresponsedto)

*Rust*:

```rust,ignore
let out = ghl.objects().search_object_records(&schemaKey, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "objects.post_objects_by_schemaKey_records_search",
    "path_params": {
      "schemaKey": "<schemaKey>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /objects/{schemaKey}/records/{id}`

**Delete Record**

Delete Record By Id . Supported Objects are business and custom objects.

Operation id: `objects.delete_objects_by_schemaKey_records_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `schemaKey` | string | **yes** | The key of the Custom Object / Standard Object Schema. For custom objects, the key must include the “custom_objects.” prefix, while standard objects use their r… |
| `id` | string | **yes** | id of the record to be updated. Available on the Record details page under the 3 dots or in the url |

*Response*: [`ObjectRecordDeleteResponseDTO`](#objectrecorddeleteresponsedto)

*Rust*:

```rust,ignore
let out = ghl.objects().delete_record(&schemaKey, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "objects.delete_objects_by_schemaKey_records_by_id",
    "path_params": {
      "schemaKey": "<schemaKey>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /objects/{schemaKey}/records/{id}`

**Get Record By Id**

Allows you to get a Standard Object like business and custom object record by Id

Operation id: `objects.get_objects_by_schemaKey_records_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `schemaKey` | string | **yes** | The key of the Custom Object / Standard Object Schema. For custom objects, the key must include the “custom_objects.” prefix, while standard objects use their r… |
| `id` | string | **yes** | id of the record to be updated. Available on the Record details page under the 3 dots or in the url |

*Response*: [`RecordByIdResponseDTO`](#recordbyidresponsedto)

*Rust*:

```rust,ignore
let out = ghl.objects().get_record_by_id(&schemaKey, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "objects.get_objects_by_schemaKey_records_by_id",
    "path_params": {
      "schemaKey": "<schemaKey>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `PUT /objects/{schemaKey}/records/{id}`

**Update Record**

Update a Custom Object Record by Id. Supported Objects are business and custom objects. Documentation Link - https://doc.clickup.com/8631005/d/h/87cpx-277156/93bf0c2e23177b0/87cpx-376296

Operation id: `objects.put_objects_by_schemaKey_records_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `schemaKey` | string | **yes** | The key of the Custom Object / Standard Object Schema. For custom objects, the key must include the “custom_objects.” prefix, while standard objects use their r… |
| `id` | string | **yes** | id of the record to be updated. Available on the Record details page under the 3 dots or in the url |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Request body*: [`UpdateCustomObjectRecordDto`](#updatecustomobjectrecorddto)

*Response*: [`RecordByIdResponseDTO`](#recordbyidresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::objects::UpdateRecordParams;

let params = UpdateRecordParams::new("locationId");
let out = ghl.objects().update_record(&schemaKey, &id, &params, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "objects.put_objects_by_schemaKey_records_by_id",
    "path_params": {
      "schemaKey": "<schemaKey>",
      "id": "<id>"
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

## Endpoints — API v3

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/objects/` | Get all objects for a location | `v3:objects.get_objects` |
| `POST` | `/objects/` | Create Custom Object | `v3:objects.post_objects` |
| `GET` | `/objects/{key}` | Get Object Schema by key / id | `v3:objects.get_objects_by_key` |
| `PUT` | `/objects/{key}` | Update Object Schema By Key / Id | `v3:objects.put_objects_by_key` |
| `POST` | `/objects/{schemaKey}/records` | Create Record | `v3:objects.post_objects_by_schemaKey_records` |
| `POST` | `/objects/{schemaKey}/records/search` | Search Object Records | `v3:objects.post_objects_by_schemaKey_records_search` |
| `DELETE` | `/objects/{schemaKey}/records/{id}` | Delete Record | `v3:objects.delete_objects_by_schemaKey_records_by_id` |
| `GET` | `/objects/{schemaKey}/records/{id}` | Get Record By Id | `v3:objects.get_objects_by_schemaKey_records_by_id` |
| `PUT` | `/objects/{schemaKey}/records/{id}` | Update Record | `v3:objects.put_objects_by_schemaKey_records_by_id` |

### Endpoint details — v3

#### `GET /objects/`

**Get all objects for a location**

Get all objects for a location. Supported Objects are contact, opportunity, business and custom objects.To understand objects and records, please have a look at the documentation here : https://doc.clickup.com/8631005/d/h/87cpx-277156/93bf0c2e23177b0

Operation id: `v3:objects.get_objects` · `Version: v3` · Scopes: `objects/schema.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | location id |

*Response*: [`CustomObjectListResponseDTO`](#customobjectlistresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:objects.get_objects",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /objects/`

**Create Custom Object**

Allows you to create a custom object schema. To understand objects and records, please have a look at the documentation here : https://doc.clickup.com/8631005/d/h/87cpx-277156/93bf0c2e23177b0

Operation id: `v3:objects.post_objects` · `Version: v3` · Scopes: `objects/schema.write`

*Request body*: [`CreateCustomObjectSchemaDTO`](#createcustomobjectschemadto)

*Response*: [`CustomObjectResponseDTO`](#customobjectresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:objects.post_objects",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /objects/{key}`

**Get Object Schema by key / id**

Retrieve Object Schema by key or ID. This will return the schema of the custom object, including all its fields and properties. Supported objects include contact, opportunity, business and custom objects.To understand objects and records, please have a look the documentation here : https://doc.clickup.com/8631005/d/h/87cpx-277156/93bf0c2e23177b0

Operation id: `v3:objects.get_objects_by_key` · `Version: v3` · Scopes: `objects/schema.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `key` | string | **yes** | key of the custom or standard object. For custom objects, the key must include the prefix “custom_objects.”. This key can be found on the Object Details page un… |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | location id of the sub account |
| `fetchProperties` | string | no | Fetch Properties , Fetches all the standard / custom fields of the object when set to true |

*Response*: [`CustomObjectByIdResponseDTO`](#customobjectbyidresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:objects.get_objects_by_key",
    "path_params": {
      "key": "<key>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /objects/{key}`

**Update Object Schema By Key / Id**

Update Custom Object Schema or standard object's like contact, opportunity, business searchable fields. To understand objects and records, please have a look at the documentation here : https://doc.clickup.com/8631005/d/h/87cpx-277156/93bf0c2e23177b0

Operation id: `v3:objects.put_objects_by_key` · `Version: v3` · Scopes: `objects/schema.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `key` | string | **yes** | key of the custom or standard object. For custom objects, the key must include the prefix “custom_objects.”. This key can be found on the Object Details page un… |

*Request body*: [`UpdateCustomObjectSchemaDTO`](#updatecustomobjectschemadto)

*Response*: [`CustomObjectResponseDTO`](#customobjectresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:objects.put_objects_by_key",
    "path_params": {
      "key": "<key>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /objects/{schemaKey}/records`

**Create Record**

Create a Custom Object Record. Supported Objects business and custom objects. Documentation Link - https://doc.clickup.com/8631005/d/h/87cpx-277156/93bf0c2e23177b0/87cpx-376296

Operation id: `v3:objects.post_objects_by_schemaKey_records` · `Version: v3` · Scopes: `objects/record.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `schemaKey` | string | **yes** | The key of the Custom Object / Standard Object Schema. For custom objects, the key must include the “custom_objects.” prefix, while standard objects use their r… |

*Request body*: [`CreateCustomObjectRecordDto`](#createcustomobjectrecorddto)

*Response*: [`RecordByIdResponseDTO`](#recordbyidresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:objects.post_objects_by_schemaKey_records",
    "path_params": {
      "schemaKey": "<schemaKey>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /objects/{schemaKey}/records/search`

**Search Object Records**

Supported Objects are custom objects and standard objects like "business". Documentation Link - https://doc.clickup.com/8631005/d/h/87cpx-277156/93bf0c2e23177b0/87cpx-379336

Operation id: `v3:objects.post_objects_by_schemaKey_records_search` · `Version: v3` · Scopes: `objects/record.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `schemaKey` | string | **yes** | custom object key |

*Request body*: [`SearchRecordsBody`](#searchrecordsbody)

*Response*: [`SearchRecordResponseDTO`](#searchrecordresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:objects.post_objects_by_schemaKey_records_search",
    "path_params": {
      "schemaKey": "<schemaKey>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /objects/{schemaKey}/records/{id}`

**Delete Record**

Delete Record By Id . Supported Objects are business and custom objects.

Operation id: `v3:objects.delete_objects_by_schemaKey_records_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `schemaKey` | string | **yes** | The key of the Custom Object / Standard Object Schema. For custom objects, the key must include the “custom_objects.” prefix, while standard objects use their r… |
| `id` | string | **yes** | id of the record to be updated. Available on the Record details page under the 3 dots or in the url |

*Response*: [`ObjectRecordDeleteResponseDTO`](#objectrecorddeleteresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:objects.delete_objects_by_schemaKey_records_by_id",
    "path_params": {
      "schemaKey": "<schemaKey>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /objects/{schemaKey}/records/{id}`

**Get Record By Id**

Allows you to get a Standard Object like business and custom object record by Id

Operation id: `v3:objects.get_objects_by_schemaKey_records_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `schemaKey` | string | **yes** | The key of the Custom Object / Standard Object Schema. For custom objects, the key must include the “custom_objects.” prefix, while standard objects use their r… |
| `id` | string | **yes** | id of the record to be updated. Available on the Record details page under the 3 dots or in the url |

*Response*: [`RecordByIdResponseDTO`](#recordbyidresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:objects.get_objects_by_schemaKey_records_by_id",
    "path_params": {
      "schemaKey": "<schemaKey>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `PUT /objects/{schemaKey}/records/{id}`

**Update Record**

Update a Custom Object Record by Id. Supported Objects are business and custom objects. Documentation Link - https://doc.clickup.com/8631005/d/h/87cpx-277156/93bf0c2e23177b0/87cpx-376296

Operation id: `v3:objects.put_objects_by_schemaKey_records_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `schemaKey` | string | **yes** | The key of the Custom Object / Standard Object Schema. For custom objects, the key must include the “custom_objects.” prefix, while standard objects use their r… |
| `id` | string | **yes** | id of the record to be updated. Available on the Record details page under the 3 dots or in the url |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Request body*: [`UpdateCustomObjectRecordDto`](#updatecustomobjectrecorddto)

*Response*: [`RecordByIdResponseDTO`](#recordbyidresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:objects.put_objects_by_schemaKey_records_by_id",
    "path_params": {
      "schemaKey": "<schemaKey>",
      "id": "<id>"
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

## Data models — API v2

In Rust: `ghl_models::v2::objects::*` (enable the `objects` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/objects/).

### `CreateCustomObjectRecordDto`

_No fields defined in the spec._

### `CreateCustomObjectSchemaDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `labels` | [`CustomObjectLabelDto`](#customobjectlabeldto) | **yes** | This is what your custom object will be called. These labels will be used to display your custom object on the UI |
| `key` | String | **yes** | key that would be used to refer the Custom Object internally (lowercase + underscore_separated). 'custom_objects.' would be added as prefix by default |
| `description` | String | no | Pet Object`s description |
| `locationId` | String | **yes** | Location Id |
| `primaryDisplayPropertyDetails` | [`CustomObjectDisplayPropertyDetails`](#customobjectdisplaypropertydetails) | **yes** | Primary property which will be displayed on the record page |

### `CreatedByResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `channel` | String | **yes** | Creation Channel |
| `createdAt` | String | **yes** | Created At |
| `source` | String | **yes** | From where the record was created |
| `sourceId` | String | **yes** | User/Resource Id |

### `CustomObjectByIdResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `object` | [`ICustomObjectSchema`](#icustomobjectschema) | no | — |
| `cache` | bool | **yes** | Is the response served from cache |
| `fields` | Vec<ICustomField> | no | — |

### `CustomObjectDisplayPropertyDetails`

| Field | Type | Required | Description |
|---|---|---|---|
| `key` | String | **yes** | key that would be used to refer the custom field internally (lowercase + underscore_separated). 'custom_objects.{{objectKey}}' would be added as prefix by default is not passed |
| `name` | String | **yes** | Name of the Primary property name which will be displayed on the record page |
| `dataType` | String | **yes** | Primary property data Type (it can either be TEXT or NUMERICAL type) |

### `CustomObjectLabelDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `singular` | String | **yes** | Singular name of the custom object |
| `plural` | String | **yes** | Plural name of the custom object |

### `CustomObjectLabelUpdateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `singular` | String | no | Singular name of the custom object |
| `plural` | String | no | Plural name of the custom object |

### `CustomObjectListResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `objects` | Vec<ICustomObjectSchema> | no | — |

### `CustomObjectResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `object` | [`ICustomObjectSchema`](#icustomobjectschema) | no | — |

### `ICustomField`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location Id |
| `name` | String | no | Field name |
| `description` | String | no | Description of the field |
| `placeholder` | String | no | Placeholder text for the field |
| `showInForms` | bool | **yes** | Whether the field should be shown in forms |
| `options` | Vec<OptionDTO> | no | Options for the field (Optional, valid only for SINGLE_OPTIONS, MULTIPLE_OPTIONS, RADIO, CHECKBOX, TEXTBOX_LIST type) |
| `acceptedFormats` | String — `.pdf`, `.docx`, `.doc`, `.jpg`, `.jpeg`, `.png`, `.gif`, `.csv`, `.xlsx`, `.xls`, `all` | no | Allowed file formats for uploads. Options include: .pdf, .docx, .doc, .jpg, .jpeg, .png, .gif, .csv, .xlsx, .xls, all |
| `id` | String | **yes** | Unique identifier of the object |
| `objectKey` | String | **yes** | The key for your custom / standard object. This key uniquely identifies the custom object. Example: "custom_object.pet" for a custom object related to pets. |
| `dataType` | String — `TEXT`, `LARGE_TEXT`, `NUMERICAL`, `PHONE`, `MONETORY`, `CHECKBOX`, `SINGLE_OPTIONS`, `MULTIPLE_OPTIONS`, `DATE`, `TEXTBOX_LIST`, `FILE_UPLOAD`, `RADIO` | **yes** | Type of field that you are trying to create |
| `parentId` | String | **yes** | ID of the parent folder |
| `fieldKey` | String | **yes** | Field key. For Custom Object it's formatted as "custom_object.{objectKey}.{fieldKey}". "custom_object" is a fixed prefix, "{objectKey}" is your custom object's identifier, and "{fieldName}" is the uni… |
| `allowCustomOption` | bool | no | Determines if users can add a custom option value different from the predefined options in records for RADIO type fields. A custom value added in one record does not automatically become an option and… |
| `maxFileLimit` | f64 | no | Maximum file limit for uploads |
| `dateAdded` | String | **yes** | Date and time when the object was added |
| `dateUpdated` | String | **yes** | Date and time when the object was last updated |

### `ICustomObjectSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | id of the custom / standard object schema |
| `standard` | bool | **yes** | false in case of custom objects and true in case of standard objects like contacts and opportunities |
| `key` | String | **yes** | key that would be used to refer the custom / standard Object internally (lowercase + underscore_separated). For custom objects, 'custom_objects.' would be added as prefix by default |
| `labels` | [`CustomObjectLabelDto`](#customobjectlabeldto) | **yes** | This is what your custom / standard object will be called. These labels will be used to display your custom object on the UI |
| `description` | String | no | Custom / Standard Object Descriptions for example , Pet Object`s description |
| `locationId` | String | **yes** | location's id |
| `primaryDisplayProperty` | String | **yes** | Primary property for the custom / standard Object. This would be used as primary data when rendering the UI. 'custom_objects.{{object_key}} or business.{{object_key}} (for company)' would be added as … |
| `dateAdded` | String | **yes** | Date and time when the object was added |
| `dateUpdated` | String | **yes** | Date and time when the object was last updated |
| `type` | JSON | no | Object`s Type |

### `IRecordSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | id of the record |
| `owner` | Vec<String> | **yes** | Owner (User's id). Limited to 1 for now. Only Supported with custom objects |
| `followers` | Vec<String> | **yes** | Follower (User's ids). Limited to 10 for now |
| `properties` | String | **yes** | Properties of the record |
| `dateAdded` | String | **yes** | Date and time when the object was added |
| `dateUpdated` | String | **yes** | Date and time when the object was last updated |

### `ObjectRecordDeleteResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | id of the deleted object |
| `success` | bool | no | boolean that defines if the operation was a success or not |

### `OptionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `key` | String | **yes** | Key of the option (Included in Create and Response, excluded in Update) |
| `label` | String | **yes** | Value of the option |
| `url` | String | no | URL associated with the option (Optional, valid only for RADIO type) |

### `RecordByIdResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `record` | [`IRecordSchema`](#irecordschema) | no | — |

### `RecordResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | id of the record |
| `owner` | Vec<String> | **yes** | Owner (User's id). Limited to 1 for now . Only supported for custom objects for now |
| `followers` | Vec<String> | **yes** | Follower (User's ids). Limited to 10 and supported for custom objects for now |
| `properties` | String | **yes** | Properties of the record |
| `createdAt` | String | **yes** | Date and time when the object was added |
| `updatedAt` | String | **yes** | Date and time when the object was last updated |
| `locationId` | String | **yes** | Location Id |
| `objectId` | String | **yes** | ObjectId Id |
| `objectKey` | String | **yes** | ObjectId key |
| `createdBy` | [`CreatedByResponseDTO`](#createdbyresponsedto) | **yes** | Created By Meta |
| `lastUpdatedBy` | [`CreatedByResponseDTO`](#createdbyresponsedto) | **yes** | Last Updated By Meta |
| `searchAfter` | Vec<f64> | **yes** | — |

### `SearchRecordResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `records` | Vec<RecordResponseDTO> | no | Records |
| `total` | f64 | **yes** | Total Number of records |

### `SearchRecordsBody`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location Id |
| `page` | f64 | **yes** | Page |
| `pageLimit` | f64 | **yes** | Page Limit |
| `query` | String | **yes** | Pass this query parameter to search using your searchable properties. For example, if you have a custom object called “Pets” and have configured “name” as a searchable property, you can pass name:Budd… |
| `searchAfter` | Vec<String> | **yes** | — |

### `UpdateCustomObjectRecordDto`

_No fields defined in the spec._

### `UpdateCustomObjectSchemaDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `labels` | [`CustomObjectLabelUpdateDto`](#customobjectlabelupdatedto) | no | This is how your custom object will be displayed |
| `description` | String | no | Pet Object`s description |
| `locationId` | String | **yes** | location id |
| `searchableProperties` | Vec<String> | **yes** | Searchable Fields: Provide the field key of your object that you want to search on, using the format (custom_object.<object_name>.<field_key>). |

## Data models — API v3

In Rust: `ghl_models::v3::objects::*` (enable the `objects` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/objects/).

### `CreateCustomObjectRecordDto`

_No fields defined in the spec._

### `CreateCustomObjectSchemaDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `labels` | [`CustomObjectLabelDto`](#customobjectlabeldto) | **yes** | This is what your custom object will be called. These labels will be used to display your custom object on the UI |
| `key` | String | **yes** | key that would be used to refer the Custom Object internally (lowercase + underscore_separated). 'custom_objects.' would be added as prefix by default |
| `description` | String | no | Pet Object`s description |
| `locationId` | String | **yes** | Location Id |
| `primaryDisplayPropertyDetails` | [`CustomObjectDisplayPropertyDetails`](#customobjectdisplaypropertydetails) | **yes** | Primary property which will be displayed on the record page |

### `CreatedByResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `channel` | String | **yes** | Creation Channel |
| `createdAt` | String | **yes** | Created At |
| `source` | String | **yes** | From where the record was created |
| `sourceId` | String | **yes** | User/Resource Id |

### `CustomObjectByIdResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `object` | [`ICustomObjectSchema`](#icustomobjectschema) | no | — |
| `cache` | bool | **yes** | Is the response served from cache |
| `fields` | Vec<ICustomField> | no | — |

### `CustomObjectDisplayPropertyDetails`

| Field | Type | Required | Description |
|---|---|---|---|
| `key` | String | **yes** | key that would be used to refer the custom field internally (lowercase + underscore_separated). 'custom_objects.{{objectKey}}' would be added as prefix by default is not passed |
| `name` | String | **yes** | Name of the Primary property name which will be displayed on the record page |
| `dataType` | String | **yes** | Primary property data Type (it can either be TEXT or NUMERICAL type) |

### `CustomObjectLabelDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `singular` | String | **yes** | Singular name of the custom object |
| `plural` | String | **yes** | Plural name of the custom object |

### `CustomObjectLabelUpdateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `singular` | String | no | Singular name of the custom object |
| `plural` | String | no | Plural name of the custom object |

### `CustomObjectListResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `objects` | Vec<ICustomObjectSchema> | no | — |

### `CustomObjectResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `object` | [`ICustomObjectSchema`](#icustomobjectschema) | no | — |

### `ICustomField`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location Id |
| `name` | String | no | Field name |
| `description` | String | no | Description of the field |
| `placeholder` | String | no | Placeholder text for the field |
| `showInForms` | bool | **yes** | Whether the field should be shown in forms |
| `options` | Vec<OptionDTO> | no | Options for the field (Optional, valid only for SINGLE_OPTIONS, MULTIPLE_OPTIONS, RADIO, CHECKBOX, TEXTBOX_LIST type) |
| `acceptedFormats` | String — `.pdf`, `.docx`, `.doc`, `.jpg`, `.jpeg`, `.png`, `.gif`, `.csv`, `.xlsx`, `.xls`, `all` | no | Allowed file formats for uploads. Options include: .pdf, .docx, .doc, .jpg, .jpeg, .png, .gif, .csv, .xlsx, .xls, all |
| `id` | String | **yes** | Unique identifier of the object |
| `objectKey` | String | **yes** | The key for your custom / standard object. This key uniquely identifies the custom object. Example: "custom_object.pet" for a custom object related to pets. |
| `dataType` | String — `TEXT`, `LARGE_TEXT`, `NUMERICAL`, `PHONE`, `MONETORY`, `CHECKBOX`, `SINGLE_OPTIONS`, `MULTIPLE_OPTIONS`, `DATE`, `TEXTBOX_LIST`, `FILE_UPLOAD`, `RADIO` | **yes** | Type of field that you are trying to create |
| `parentId` | String | **yes** | ID of the parent folder |
| `fieldKey` | String | **yes** | Field key. For Custom Object it's formatted as "custom_object.{objectKey}.{fieldKey}". "custom_object" is a fixed prefix, "{objectKey}" is your custom object's identifier, and "{fieldName}" is the uni… |
| `allowCustomOption` | bool | no | Determines if users can add a custom option value different from the predefined options in records for RADIO type fields. A custom value added in one record does not automatically become an option and… |
| `maxFileLimit` | f64 | no | Maximum file limit for uploads |
| `dateAdded` | String | **yes** | Date and time when the object was added |
| `dateUpdated` | String | **yes** | Date and time when the object was last updated |

### `ICustomObjectSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | id of the custom / standard object schema |
| `standard` | bool | **yes** | false in case of custom objects and true in case of standard objects like contacts and opportunities |
| `key` | String | **yes** | key that would be used to refer the custom / standard Object internally (lowercase + underscore_separated). For custom objects, 'custom_objects.' would be added as prefix by default |
| `labels` | [`CustomObjectLabelDto`](#customobjectlabeldto) | **yes** | This is what your custom / standard object will be called. These labels will be used to display your custom object on the UI |
| `description` | String | no | Custom / Standard Object Descriptions for example , Pet Object`s description |
| `locationId` | String | **yes** | location's id |
| `primaryDisplayProperty` | String | **yes** | Primary property for the custom / standard Object. This would be used as primary data when rendering the UI. 'custom_objects.{{object_key}} or business.{{object_key}} (for company)' would be added as … |
| `dateAdded` | String | **yes** | Date and time when the object was added |
| `dateUpdated` | String | **yes** | Date and time when the object was last updated |
| `type` | JSON | no | Object`s Type |

### `IRecordSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | id of the record |
| `owner` | Vec<String> | **yes** | Owner (User's id). Limited to 1 for now. Only Supported with custom objects |
| `followers` | Vec<String> | **yes** | Follower (User's ids). Limited to 10 for now |
| `properties` | String | **yes** | Properties of the record |
| `dateAdded` | String | **yes** | Date and time when the object was added |
| `dateUpdated` | String | **yes** | Date and time when the object was last updated |

### `ObjectRecordDeleteResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | id of the deleted object |
| `success` | bool | no | boolean that defines if the operation was a success or not |

### `OptionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `key` | String | **yes** | Key of the option (Included in Create and Response, excluded in Update) |
| `label` | String | **yes** | Value of the option |
| `url` | String | no | URL associated with the option (Optional, valid only for RADIO type) |

### `RecordByIdResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `record` | [`IRecordSchema`](#irecordschema) | no | — |

### `RecordResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | id of the record |
| `owner` | Vec<String> | **yes** | Owner (User's id). Limited to 1 for now . Only supported for custom objects for now |
| `followers` | Vec<String> | **yes** | Follower (User's ids). Limited to 10 and supported for custom objects for now |
| `properties` | String | **yes** | Properties of the record |
| `createdAt` | String | **yes** | Date and time when the object was added |
| `updatedAt` | String | **yes** | Date and time when the object was last updated |
| `locationId` | String | **yes** | Location Id |
| `objectId` | String | **yes** | ObjectId Id |
| `objectKey` | String | **yes** | ObjectId key |
| `createdBy` | [`CreatedByResponseDTO`](#createdbyresponsedto) | **yes** | Created By Meta |
| `lastUpdatedBy` | [`CreatedByResponseDTO`](#createdbyresponsedto) | **yes** | Last Updated By Meta |
| `searchAfter` | Vec<f64> | **yes** | — |

### `SearchRecordResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `records` | Vec<RecordResponseDTO> | no | Records |
| `total` | f64 | **yes** | Total Number of records |

### `SearchRecordsBody`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location Id |
| `page` | f64 | **yes** | Page |
| `pageLimit` | f64 | **yes** | Page Limit |
| `query` | String | **yes** | Pass this query parameter to search using your searchable properties. For example, if you have a custom object called “Pets” and have configured “name” as a searchable property, you can pass name:Budd… |
| `searchAfter` | Vec<String> | **yes** | — |

### `UpdateCustomObjectRecordDto`

_No fields defined in the spec._

### `UpdateCustomObjectSchemaDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `labels` | [`CustomObjectLabelUpdateDto`](#customobjectlabelupdatedto) | no | This is how your custom object will be displayed |
| `description` | String | no | Pet Object`s description |
| `locationId` | String | **yes** | location id |
| `searchableProperties` | Vec<String> | **yes** | Searchable Fields: Provide the field key of your object that you want to search on, using the format (custom_object.<object_name>.<field_key>). |

