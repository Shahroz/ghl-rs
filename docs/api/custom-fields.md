# `custom-fields`

**8** operations / **10** models in API v2 · **8** operations / **10** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `custom-fields` cargo feature on `ghl-sdk`, then call any of the 8 generated methods on `ghl.custom_fields()`:

```toml
ghl-sdk = { version = "0.4", features = ["custom-fields"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `POST` | `/custom-fields/` | Create Custom Field | `create_custom_field()` | `custom-fields.post_custom_fields` |
| `POST` | `/custom-fields/folder` | Create Custom Field Folder | `create_custom_field_folder()` | `custom-fields.post_custom_fields_folder` |
| `DELETE` | `/custom-fields/folder/{id}` | Delete Custom Field Folder | `delete_custom_field_folder()` | `custom-fields.delete_custom_fields_folder_by_id` |
| `PUT` | `/custom-fields/folder/{id}` | Update Custom Field Folder Name | `update_custom_field_folder_name()` | `custom-fields.put_custom_fields_folder_by_id` |
| `GET` | `/custom-fields/object-key/{objectKey}` | Get Custom Fields By Object Key | `get_custom_fields_by_object_key()` | `custom-fields.get_custom_fields_object_key_by_objectKey` |
| `DELETE` | `/custom-fields/{id}` | Delete Custom Field By Id | `delete_custom_field_by_id()` | `custom-fields.delete_custom_fields_by_id` |
| `GET` | `/custom-fields/{id}` | Get Custom Field / Folder By Id | `get_custom_field_folder_by_id()` | `custom-fields.get_custom_fields_by_id` |
| `PUT` | `/custom-fields/{id}` | Update Custom Field By Id | `update_custom_field_by_id()` | `custom-fields.put_custom_fields_by_id` |

### Endpoint details — v2

#### `POST /custom-fields/`

**Create Custom Field**

<div> <p> Create Custom Field </p> <div> <span style= "display: inline-block; width: 25px; height: 25px; background-color: yellow; color: black; font-weight: bold; font-size: 24px; text-align: center; line-height: 22px; border: 2px solid black; border-radius: 10%; margin-right: 10px;"> ! </span> <span> <strong> Only supports Custom Objects and Company (Business) today. Will be extended to other Standard Objects in the future. </strong> </span> </div> </div>

Operation id: `custom-fields.post_custom_fields` · `Version: 2021-07-28` · Scopes: `locations/customFields.write`

*Request body*: [`CreateCustomFieldsDTO`](#createcustomfieldsdto)

*Response*: [`CustomFieldSuccessfulResponseDto`](#customfieldsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.custom_fields().create_custom_field(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "custom-fields.post_custom_fields",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /custom-fields/folder`

**Create Custom Field Folder**

<div> <p> Create Custom Field Folder </p> <div> <span style= "display: inline-block; width: 25px; height: 25px; background-color: yellow; color: black; font-weight: bold; font-size: 24px; text-align: center; line-height: 22px; border: 2px solid black; border-radius: 10%; margin-right: 10px;"> ! </span> <span> <strong> Only supports Custom Objects and Company (Business) today. Will be extended to other Standard Objects in the future. </strong> </span> </div> </div>

Operation id: `custom-fields.post_custom_fields_folder` · `Version: 2021-07-28` · Scopes: `locations/customFields.write`

*Request body*: [`CreateFolder`](#createfolder)

*Response*: [`ICustomFieldFolder`](#icustomfieldfolder)

*Rust*:

```rust,ignore
let out = ghl.custom_fields().create_custom_field_folder(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "custom-fields.post_custom_fields_folder",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /custom-fields/folder/{id}`

**Delete Custom Field Folder**

<div> <p> Create Custom Field Folder </p> <div> <span style= "display: inline-block; width: 25px; height: 25px; background-color: yellow; color: black; font-weight: bold; font-size: 24px; text-align: center; line-height: 22px; border: 2px solid black; border-radius: 10%; margin-right: 10px;"> ! </span> <span> <strong> Only supports Custom Objects and Company (Business) today. Will be extended to other Standard Objects in the future. </strong> </span> </div> </div>

Operation id: `custom-fields.delete_custom_fields_folder_by_id` · `Version: 2021-07-28` · Scopes: `locations/customFields.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Response*: [`CustomFolderDeleteResponseDto`](#customfolderdeleteresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::custom_fields::DeleteCustomFieldFolderParams;

let params = DeleteCustomFieldFolderParams::new("locationId");
let out = ghl.custom_fields().delete_custom_field_folder(&id, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "custom-fields.delete_custom_fields_folder_by_id",
    "path_params": {
      "id": "<id>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /custom-fields/folder/{id}`

**Update Custom Field Folder Name**

<div> <p> Create Custom Field Folder </p> <div> <span style= "display: inline-block; width: 25px; height: 25px; background-color: yellow; color: black; font-weight: bold; font-size: 24px; text-align: center; line-height: 22px; border: 2px solid black; border-radius: 10%; margin-right: 10px;"> ! </span> <span> <strong> Only supports Custom Objects and Company (Business) today. Will be extended to other Standard Objects in the future. </strong> </span> </div> </div>

Operation id: `custom-fields.put_custom_fields_folder_by_id` · `Version: 2021-07-28` · Scopes: `locations/customFields.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | — |

*Request body*: [`UpdateFolder`](#updatefolder)

*Response*: [`ICustomFieldFolder`](#icustomfieldfolder)

*Rust*:

```rust,ignore
let out = ghl.custom_fields().update_custom_field_folder_name(&id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "custom-fields.put_custom_fields_folder_by_id",
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

#### `GET /custom-fields/object-key/{objectKey}`

**Get Custom Fields By Object Key**

<div> <p> Get Custom Fields By Object Key</p> <div> <span style= "display: inline-block; width: 25px; height: 25px; background-color: yellow; color: black; font-weight: bold; font-size: 24px; text-align: center; line-height: 22px; border: 2px solid black; border-radius: 10%; margin-right: 10px;"> ! </span> <span> <strong> Only supports Custom Objects and Company (Business) today. Will be extended to other Standard Objects in the future. </strong> </span> </div> </div>

Operation id: `custom-fields.get_custom_fields_object_key_by_objectKey` · `Version: 2021-07-28` · Scopes: `locations/customFields.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `objectKey` | string | **yes** | key of the Object. Must include "custom_objects." prefix for custom objects. Available on the Custom Objects details Page under settings |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Response*: [`CustomFieldsResponseDTO`](#customfieldsresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::custom_fields::GetCustomFieldsByObjectKeyParams;

let params = GetCustomFieldsByObjectKeyParams::new("locationId");
let out = ghl.custom_fields().get_custom_fields_by_object_key(&objectKey, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "custom-fields.get_custom_fields_object_key_by_objectKey",
    "path_params": {
      "objectKey": "<objectKey>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `DELETE /custom-fields/{id}`

**Delete Custom Field By Id**

<div> <p> Delete Custom Field By Id </p> <div> <span style= "display: inline-block; width: 25px; height: 25px; background-color: yellow; color: black; font-weight: bold; font-size: 24px; text-align: center; line-height: 22px; border: 2px solid black; border-radius: 10%; margin-right: 10px;"> ! </span> <span> <strong> Only supports Custom Objects and Company (Business) today. Will be extended to other Standard Objects in the future. </strong> </span> </div> </div>

Operation id: `custom-fields.delete_custom_fields_by_id` · `Version: 2021-07-28` · Scopes: `locations/customFields.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | — |

*Response*: [`CustomFolderDeleteResponseDto`](#customfolderdeleteresponsedto)

*Rust*:

```rust,ignore
let out = ghl.custom_fields().delete_custom_field_by_id(&id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "custom-fields.delete_custom_fields_by_id",
    "path_params": {
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /custom-fields/{id}`

**Get Custom Field / Folder By Id**

<div> <p> Get Custom Field / Folder By Id.</p> <div> <span style= "display: inline-block; width: 25px; height: 25px; background-color: yellow; color: black; font-weight: bold; font-size: 24px; text-align: center; line-height: 22px; border: 2px solid black; border-radius: 10%; margin-right: 10px;"> ! </span> <span> <strong> Only supports Custom Objects and Company (Business) today. Will be extended to other Standard Objects in the future. </strong> </span> </div> </div>

Operation id: `custom-fields.get_custom_fields_by_id` · `Version: 2021-07-28` · Scopes: `locations/customFields.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | — |

*Response*: [`CustomFieldSuccessfulResponseDto`](#customfieldsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.custom_fields().get_custom_field_folder_by_id(&id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "custom-fields.get_custom_fields_by_id",
    "path_params": {
      "id": "<id>"
    }
  }
}
```

</details>

#### `PUT /custom-fields/{id}`

**Update Custom Field By Id**

<div> <p> Update Custom Field By Id </p> <div> <span style= "display: inline-block; width: 25px; height: 25px; background-color: yellow; color: black; font-weight: bold; font-size: 24px; text-align: center; line-height: 22px; border: 2px solid black; border-radius: 10%; margin-right: 10px;"> ! </span> <span> <strong> Only supports Custom Objects and Company (Business) today. Will be extended to other Standard Objects in the future. </strong> </span> </div> </div>

Operation id: `custom-fields.put_custom_fields_by_id` · `Version: 2021-07-28` · Scopes: `locations/customFields.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | — |

*Request body*: [`UpdateCustomFieldsDTO`](#updatecustomfieldsdto)

*Response*: [`CustomFieldSuccessfulResponseDto`](#customfieldsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.custom_fields().update_custom_field_by_id(&id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "custom-fields.put_custom_fields_by_id",
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

## Endpoints — API v3

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `POST` | `/custom-fields/` | Create Custom Field | `v3:custom-fields.post_custom_fields` |
| `POST` | `/custom-fields/folder` | Create Custom Field Folder | `v3:custom-fields.post_custom_fields_folder` |
| `DELETE` | `/custom-fields/folder/{id}` | Delete Custom Field Folder | `v3:custom-fields.delete_custom_fields_folder_by_id` |
| `PUT` | `/custom-fields/folder/{id}` | Update Custom Field Folder Name | `v3:custom-fields.put_custom_fields_folder_by_id` |
| `GET` | `/custom-fields/object-key/{objectKey}` | Get Custom Fields By Object Key | `v3:custom-fields.get_custom_fields_object_key_by_objectKey` |
| `DELETE` | `/custom-fields/{id}` | Delete Custom Field By Id | `v3:custom-fields.delete_custom_fields_by_id` |
| `GET` | `/custom-fields/{id}` | Get Custom Field / Folder By Id | `v3:custom-fields.get_custom_fields_by_id` |
| `PUT` | `/custom-fields/{id}` | Update Custom Field By Id | `v3:custom-fields.put_custom_fields_by_id` |

### Endpoint details — v3

#### `POST /custom-fields/`

**Create Custom Field**

<div> <p> Create Custom Field </p> </div> :::info Only supports Custom Objects and Company (Business) today. Will be extended to other Standard Objects in the future. :::

Operation id: `v3:custom-fields.post_custom_fields` · `Version: v3` · Scopes: `locations/customFields.write`

*Request body*: [`CreateCustomFieldsDTO`](#createcustomfieldsdto)

*Response*: [`CustomFieldSuccessfulResponseDto`](#customfieldsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:custom-fields.post_custom_fields",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /custom-fields/folder`

**Create Custom Field Folder**

<div> <p> Create Custom Field Folder </p> </div> :::info Only supports Custom Objects and Company (Business) today. Will be extended to other Standard Objects in the future. :::

Operation id: `v3:custom-fields.post_custom_fields_folder` · `Version: v3` · Scopes: `locations/customFields.write`

*Request body*: [`CreateFolder`](#createfolder)

*Response*: [`ICustomFieldFolder`](#icustomfieldfolder)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:custom-fields.post_custom_fields_folder",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /custom-fields/folder/{id}`

**Delete Custom Field Folder**

<div> <p> Create Custom Field Folder </p> </div> :::info Only supports Custom Objects and Company (Business) today. Will be extended to other Standard Objects in the future. :::

Operation id: `v3:custom-fields.delete_custom_fields_folder_by_id` · `Version: v3` · Scopes: `locations/customFields.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Response*: [`CustomFolderDeleteResponseDto`](#customfolderdeleteresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:custom-fields.delete_custom_fields_folder_by_id",
    "path_params": {
      "id": "<id>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /custom-fields/folder/{id}`

**Update Custom Field Folder Name**

<div> <p> Create Custom Field Folder </p> </div> :::info Only supports Custom Objects and Company (Business) today. Will be extended to other Standard Objects in the future. :::

Operation id: `v3:custom-fields.put_custom_fields_folder_by_id` · `Version: v3` · Scopes: `locations/customFields.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | — |

*Request body*: [`UpdateFolder`](#updatefolder)

*Response*: [`ICustomFieldFolder`](#icustomfieldfolder)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:custom-fields.put_custom_fields_folder_by_id",
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

#### `GET /custom-fields/object-key/{objectKey}`

**Get Custom Fields By Object Key**

<div> <p> Get Custom Fields By Object Key </p> </div> :::info Only supports Custom Objects and Company (Business) today. Will be extended to other Standard Objects in the future. :::

Operation id: `v3:custom-fields.get_custom_fields_object_key_by_objectKey` · `Version: v3` · Scopes: `locations/customFields.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `objectKey` | string | **yes** | key of the Object. Must include "custom_objects." prefix for custom objects. Available on the Custom Objects details Page under settings |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Response*: [`CustomFieldsResponseDTO`](#customfieldsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:custom-fields.get_custom_fields_object_key_by_objectKey",
    "path_params": {
      "objectKey": "<objectKey>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `DELETE /custom-fields/{id}`

**Delete Custom Field By Id**

<div> <p> Delete Custom Field By Id </p> </div> :::info Only supports Custom Objects and Company (Business) today. Will be extended to other Standard Objects in the future. :::

Operation id: `v3:custom-fields.delete_custom_fields_by_id` · `Version: v3` · Scopes: `locations/customFields.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | — |

*Response*: [`CustomFolderDeleteResponseDto`](#customfolderdeleteresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:custom-fields.delete_custom_fields_by_id",
    "path_params": {
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /custom-fields/{id}`

**Get Custom Field / Folder By Id**

<div> <p> Get Custom Field / Folder By Id.</p> </div> :::info Only supports Custom Objects and Company (Business) today. Will be extended to other Standard Objects in the future. :::

Operation id: `v3:custom-fields.get_custom_fields_by_id` · `Version: v3` · Scopes: `locations/customFields.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | — |

*Response*: [`CustomFieldSuccessfulResponseDto`](#customfieldsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:custom-fields.get_custom_fields_by_id",
    "path_params": {
      "id": "<id>"
    }
  }
}
```

</details>

#### `PUT /custom-fields/{id}`

**Update Custom Field By Id**

<div> <p> Update Custom Field By Id </p> </div> :::info Only supports Custom Objects and Company (Business) today. Will be extended to other Standard Objects in the future. :::

Operation id: `v3:custom-fields.put_custom_fields_by_id` · `Version: v3` · Scopes: `locations/customFields.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | — |

*Request body*: [`UpdateCustomFieldsDTO`](#updatecustomfieldsdto)

*Response*: [`CustomFieldSuccessfulResponseDto`](#customfieldsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:custom-fields.put_custom_fields_by_id",
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

## Data models — API v2

In Rust: `ghl_models::v2::custom_fields::*` (enable the `custom-fields` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/custom_fields/).

### `CreateCustomFieldsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location Id |
| `name` | String | no | Field name |
| `description` | String | no | Description of the field |
| `placeholder` | String | no | Placeholder text for the field |
| `showInForms` | bool | **yes** | Whether the field should be shown in forms |
| `options` | Vec<OptionDTO> | no | Options for the field (Optional, valid only for SINGLE_OPTIONS, MULTIPLE_OPTIONS, RADIO, CHECKBOX, TEXTBOX_LIST type) |
| `acceptedFormats` | String — `.pdf`, `.docx`, `.doc`, `.jpg`, `.jpeg`, `.png`, `.gif`, `.csv`, `.xlsx`, `.xls`, `all` | no | Allowed file formats for uploads. Options include: .pdf, .docx, .doc, .jpg, .jpeg, .png, .gif, .csv, .xlsx, .xls, all |
| `dataType` | String — `TEXT`, `LARGE_TEXT`, `NUMERICAL`, `PHONE`, `MONETORY`, `CHECKBOX`, `SINGLE_OPTIONS`, `MULTIPLE_OPTIONS`, `DATE`, `TEXTBOX_LIST`, `FILE_UPLOAD`, `RADIO`, `EMAIL` | **yes** | Type of field that you are trying to create |
| `fieldKey` | String | **yes** | Field key. For Custom Object it's formatted as "custom_object.{objectKey}.{fieldKey}". "custom_object" is a fixed prefix, "{objectKey}" is your custom object's identifier, and "{fieldKey}" is the uniq… |
| `objectKey` | String | **yes** | The key for your custom object. This key uniquely identifies the custom object. Example: "custom_object.pet" for a custom object related to pets. |
| `maxFileLimit` | f64 | no | Maximum file limit for uploads. Applicable only for fields with a data type of FILE_UPLOAD. |
| `allowCustomOption` | bool | no | Determines if users can add a custom option value different from the predefined options in records for RADIO type fields. A custom value added in one record does not automatically become an option and… |
| `parentId` | String | **yes** | ID of the parent folder |

### `CreateFolder`

| Field | Type | Required | Description |
|---|---|---|---|
| `objectKey` | String | **yes** | The key for your custom object. This key uniquely identifies the custom object. Example: "custom_object.pet" for a custom object related to pets. |
| `name` | String | **yes** | Field name |
| `locationId` | String | **yes** | Location Id |

### `CustomFieldSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `field` | [`ICustomField`](#icustomfield) | no | — |

### `CustomFieldsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `fields` | Vec<ICustomField> | no | Custom Fields for the object. |
| `folders` | Vec<ICustomField> | no | Custom Fields folder for the object. |

### `CustomFolderDeleteResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeded` | bool | **yes** | — |
| `id` | String | **yes** | — |
| `key` | String | **yes** | — |

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

### `ICustomFieldFolder`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier of the object |
| `objectKey` | String | **yes** | The key for your custom object. This key uniquely identifies the custom object. Example: "custom_object.pet" for a custom object related to pets. |
| `locationId` | String | **yes** | Location Id |
| `name` | String | **yes** | Field name |

### `OptionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `key` | String | **yes** | Key of the option (Included in Create and Response, excluded in Update) |
| `label` | String | **yes** | Value of the option |
| `url` | String | no | URL associated with the option (Optional, valid only for RADIO type) |

### `UpdateCustomFieldsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location Id |
| `name` | String | no | Field name |
| `description` | String | no | Description of the field |
| `placeholder` | String | no | Placeholder text for the field |
| `showInForms` | bool | **yes** | Whether the field should be shown in forms |
| `options` | Vec<OptionDTO> | no | Options for the field. Important: Providing options will completely replace the existing options array. You must include all existing options alongside any new options you wish to add. Removal of opti… |
| `acceptedFormats` | String — `.pdf`, `.docx`, `.doc`, `.jpg`, `.jpeg`, `.png`, `.gif`, `.csv`, `.xlsx`, `.xls`, `all` | no | Allowed file formats for uploads. Options include: .pdf, .docx, .doc, .jpg, .jpeg, .png, .gif, .csv, .xlsx, .xls, all |
| `maxFileLimit` | f64 | no | Maximum file limit for uploads. Applicable only for fields with a data type of FILE_UPLOAD. |

### `UpdateFolder`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Field name |
| `locationId` | String | **yes** | Location Id |

## Data models — API v3

In Rust: `ghl_models::v3::custom_fields::*` (enable the `custom-fields` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/custom_fields/).

### `CreateCustomFieldsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location Id |
| `name` | String | no | Field name |
| `description` | String | no | Description of the field |
| `placeholder` | String | no | Placeholder text for the field |
| `showInForms` | bool | **yes** | Whether the field should be shown in forms |
| `options` | Vec<OptionDTO> | no | Options for the field (Optional, valid only for SINGLE_OPTIONS, MULTIPLE_OPTIONS, RADIO, CHECKBOX, TEXTBOX_LIST type) |
| `acceptedFormats` | String — `.pdf`, `.docx`, `.doc`, `.jpg`, `.jpeg`, `.png`, `.gif`, `.csv`, `.xlsx`, `.xls`, `all` | no | Allowed file formats for uploads. Options include: .pdf, .docx, .doc, .jpg, .jpeg, .png, .gif, .csv, .xlsx, .xls, all |
| `dataType` | String — `TEXT`, `LARGE_TEXT`, `NUMERICAL`, `PHONE`, `MONETORY`, `CHECKBOX`, `SINGLE_OPTIONS`, `MULTIPLE_OPTIONS`, `DATE`, `TEXTBOX_LIST`, `FILE_UPLOAD`, `RADIO`, `EMAIL` | **yes** | Type of field that you are trying to create |
| `fieldKey` | String | **yes** | Field key. For Custom Object it's formatted as "custom_object.{objectKey}.{fieldKey}". "custom_object" is a fixed prefix, "{objectKey}" is your custom object's identifier, and "{fieldKey}" is the uniq… |
| `objectKey` | String | **yes** | The key for your custom object. This key uniquely identifies the custom object. Example: "custom_object.pet" for a custom object related to pets. |
| `maxFileLimit` | f64 | no | Maximum file limit for uploads. Applicable only for fields with a data type of FILE_UPLOAD. |
| `allowCustomOption` | bool | no | Determines if users can add a custom option value different from the predefined options in records for RADIO type fields. A custom value added in one record does not automatically become an option and… |
| `parentId` | String | **yes** | ID of the parent folder |

### `CreateFolder`

| Field | Type | Required | Description |
|---|---|---|---|
| `objectKey` | String | **yes** | The key for your custom object. This key uniquely identifies the custom object. Example: "custom_object.pet" for a custom object related to pets. |
| `name` | String | **yes** | Field name |
| `locationId` | String | **yes** | Location Id |

### `CustomFieldSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `field` | [`ICustomField`](#icustomfield) | no | — |

### `CustomFieldsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `fields` | Vec<ICustomField> | no | Custom Fields for the object. |
| `folders` | Vec<ICustomField> | no | Custom Fields folder for the object. |

### `CustomFolderDeleteResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeded` | bool | **yes** | — |
| `id` | String | **yes** | — |
| `key` | String | **yes** | — |

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

### `ICustomFieldFolder`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier of the object |
| `objectKey` | String | **yes** | The key for your custom object. This key uniquely identifies the custom object. Example: "custom_object.pet" for a custom object related to pets. |
| `locationId` | String | **yes** | Location Id |
| `name` | String | **yes** | Field name |

### `OptionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `key` | String | **yes** | Key of the option (Included in Create and Response, excluded in Update) |
| `label` | String | **yes** | Value of the option |
| `url` | String | no | URL associated with the option (Optional, valid only for RADIO type) |

### `UpdateCustomFieldsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location Id |
| `name` | String | no | Field name |
| `description` | String | no | Description of the field |
| `placeholder` | String | no | Placeholder text for the field |
| `showInForms` | bool | **yes** | Whether the field should be shown in forms |
| `options` | Vec<OptionDTO> | no | Options for the field. Important: Providing options will completely replace the existing options array. You must include all existing options alongside any new options you wish to add. Removal of opti… |
| `acceptedFormats` | String — `.pdf`, `.docx`, `.doc`, `.jpg`, `.jpeg`, `.png`, `.gif`, `.csv`, `.xlsx`, `.xls`, `all` | no | Allowed file formats for uploads. Options include: .pdf, .docx, .doc, .jpg, .jpeg, .png, .gif, .csv, .xlsx, .xls, all |
| `maxFileLimit` | f64 | no | Maximum file limit for uploads. Applicable only for fields with a data type of FILE_UPLOAD. |

### `UpdateFolder`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Field name |
| `locationId` | String | **yes** | Location Id |

