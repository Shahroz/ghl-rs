# `forms`

**3** operations / **9** models in API v2 · **3** operations / **9** models in API v3

## How to call it

No hand-written service yet — reach these endpoints two ways:

**From Rust** (typed body via [`ghl-models`](https://docs.rs/ghl-models)):

```rust,ignore
// cargo add ghl-models --features forms
use ghl_models::v2::forms::*;

let body = serde_json::to_value(/* a Create…Dto from above */)?;
let out = ghl.request_raw("POST", "/path/", &[], Some(&body), None).await?;
```

**From an AI agent** (MCP meta-tools):

```json
{
  "name": "ghl_search_operations",
  "arguments": {
    "query": "",
    "module": "forms"
  }
}
```

## Endpoints — API v2

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/forms/` | Get Forms | `forms.get_forms` |
| `GET` | `/forms/submissions` | Get Forms Submissions | `forms.get_forms_submissions` |
| `POST` | `/forms/upload-custom-files` | Upload files to custom fields | `forms.post_forms_upload_custom_files` |

### Endpoint details — v2

#### `GET /forms/`

**Get Forms**

Operation id: `forms.get_forms` · `Version: 2021-07-28` · Scopes: `forms.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `skip` | number | no | — |
| `limit` | number | no | Limit Per Page records count. will allow maximum up to 50 and default will be 10 |
| `type` | string | no | — |

*Response*: [`FormsSuccessfulResponseDto`](#formssuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "forms.get_forms",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /forms/submissions`

**Get Forms Submissions**

Operation id: `forms.get_forms_submissions` · `Version: 2021-07-28` · Scopes: `forms.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `page` | number | no | Page No. By default it will be 1 |
| `limit` | number | no | Limit Per Page records count. will allow maximum up to 100 and default will be 20 |
| `formId` | string | no | Filter submission by form id |
| `q` | string | no | Filter by contactId, name, email or phone no. |
| `startAt` | string | no | Get submission by starting of this date. By default it will be same date of last month(YYYY-MM-DD). |
| `endAt` | string | no | Get submission by ending of this date. By default it will be current date(YYYY-MM-DD). |

*Response*: [`FormsSubmissionsSuccessfulResponseDto`](#formssubmissionssuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "forms.get_forms_submissions",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /forms/upload-custom-files`

**Upload files to custom fields**

Post the necessary fields for the API to upload files. The files need to be a buffer with the key "< custom_field_id >_< file_id >". Here custom field id is the ID of your custom field and file id is a randomly generated id (or uuid) There is support for multiple file uploads as well. Have multiple fields in the format mentioned. File size is limited to 50 MB. The allowed file types are: <ul><li>PDF</li><li>DOCX</li><li>DOC</li><li>JPG</li><li>JPEG</li><li>PNG</li><li>GIF</li><li>CSV</li><li>XLS…

Operation id: `forms.post_forms_upload_custom_files` · `Version: 2021-07-28` · Scopes: `forms.write`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact ID to upload the file to. |
| `locationId` | string | **yes** | Location ID of the contact. |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "forms.post_forms_upload_custom_files",
    "query": {
      "contactId": "<contactId>",
      "locationId": "<locationId>"
    }
  }
}
```

</details>

## Endpoints — API v3

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/forms/` | Get Forms | `v3:forms.get_forms` |
| `GET` | `/forms/submissions` | Get Forms Submissions | `v3:forms.get_forms_submissions` |
| `POST` | `/forms/upload-custom-files` | Upload files to custom fields | `v3:forms.post_forms_upload_custom_files` |

### Endpoint details — v3

#### `GET /forms/`

**Get Forms**

Operation id: `v3:forms.get_forms` · `Version: v3` · Scopes: `forms.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `skip` | number | no | — |
| `limit` | number | no | Limit Per Page records count. will allow maximum up to 50 and default will be 10 |
| `type` | string | no | — |

*Response*: [`FormsSuccessfulResponseDto`](#formssuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:forms.get_forms",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /forms/submissions`

**Get Forms Submissions**

Operation id: `v3:forms.get_forms_submissions` · `Version: v3` · Scopes: `forms.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `page` | number | no | Page No. By default it will be 1 |
| `limit` | number | no | Limit Per Page records count. will allow maximum up to 100 and default will be 20 |
| `formId` | string | no | Filter submission by form id |
| `q` | string | no | Filter by contactId, name, email or phone no. |
| `startAt` | string | no | Get submission by starting of this date. By default it will be same date of last month(YYYY-MM-DD). |
| `endAt` | string | no | Get submission by ending of this date. By default it will be current date(YYYY-MM-DD). |

*Response*: [`FormsSubmissionsSuccessfulResponseDto`](#formssubmissionssuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:forms.get_forms_submissions",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /forms/upload-custom-files`

**Upload files to custom fields**

Post the necessary fields for the API to upload files. The files need to be a buffer with the key "< custom_field_id >_< file_id >". Here custom field id is the ID of your custom field and file id is a randomly generated id (or uuid) There is support for multiple file uploads as well. Have multiple fields in the format mentioned. File size is limited to 50 MB. The allowed file types are: <ul><li>PDF</li><li>DOCX</li><li>DOC</li><li>JPG</li><li>JPEG</li><li>PNG</li><li>GIF</li><li>CSV</li><li>XLS…

Operation id: `v3:forms.post_forms_upload_custom_files` · `Version: v3` · Scopes: `forms.write`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `contactId` | string | **yes** | Contact ID to upload the file to. |
| `locationId` | string | **yes** | Location ID of the contact. |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:forms.post_forms_upload_custom_files",
    "query": {
      "contactId": "<contactId>",
      "locationId": "<locationId>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::forms::*` (enable the `forms` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/forms/).

### `ContactSessionIds`

| Field | Type | Required | Description |
|---|---|---|---|
| `ids` | Vec<String> | no | — |

### `EventDataSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `fbc` | String | no | — |
| `fbp` | String | no | — |
| `page` | [`PageDetailsSchema`](#pagedetailsschema) | no | — |
| `type` | String | no | — |
| `domain` | String | no | — |
| `medium` | String | no | — |
| `source` | String | no | — |
| `version` | String | no | — |
| `adSource` | String | no | — |
| `mediumId` | String | no | — |
| `parentId` | String | no | — |
| `referrer` | String | no | — |
| `fbEventId` | String | no | — |
| `timestamp` | f64 | no | — |
| `parentName` | String | no | — |
| `fingerprint` | String | no | — |
| `pageVisitType` | String | no | — |
| `contactSessionIds` | [`ContactSessionIds`](#contactsessionids) | no | — |

### `FormsParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `locationId` | String | no | — |

### `FormsSubmissionsSubmissionsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `contactId` | String | no | — |
| `createdAt` | String | no | — |
| `formId` | String | no | — |
| `name` | String | no | — |
| `email` | String | no | — |
| `others` | [`othersSchema`](#othersschema) | no | — |

### `FormsSubmissionsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `submissions` | Vec<FormsSubmissionsSubmissionsSchema> | no | — |
| `meta` | [`metaSchema`](#metaschema) | no | — |

### `FormsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `forms` | Vec<FormsParams> | no | — |
| `total` | f64 | no | Total number of forms |

### `PageDetailsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | String | no | — |
| `title` | String | no | — |

### `metaSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `total` | f64 | no | — |
| `currentPage` | f64 | no | — |
| `nextPage` | f64 | no | — |
| `prevPage` | f64 | no | — |

### `othersSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `__submissions_other_field__` | String | no | — |
| `__custom_field_id__` | String | no | — |
| `eventData` | [`EventDataSchema`](#eventdataschema) | no | — |
| `fieldsOriSequance` | Vec<String> | no | — |

## Data models — API v3

In Rust: `ghl_models::v3::forms::*` (enable the `forms` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/forms/).

### `ContactSessionIds`

| Field | Type | Required | Description |
|---|---|---|---|
| `ids` | Vec<String> | no | — |

### `EventDataSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `fbc` | String | no | — |
| `fbp` | String | no | — |
| `page` | [`PageDetailsSchema`](#pagedetailsschema) | no | — |
| `type` | String | no | — |
| `domain` | String | no | — |
| `medium` | String | no | — |
| `source` | String | no | — |
| `version` | String | no | — |
| `adSource` | String | no | — |
| `mediumId` | String | no | — |
| `parentId` | String | no | — |
| `referrer` | String | no | — |
| `fbEventId` | String | no | — |
| `timestamp` | f64 | no | — |
| `parentName` | String | no | — |
| `fingerprint` | String | no | — |
| `pageVisitType` | String | no | — |
| `contactSessionIds` | [`ContactSessionIds`](#contactsessionids) | no | — |

### `FormsParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `locationId` | String | no | — |

### `FormsSubmissionsSubmissionsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `contactId` | String | no | — |
| `createdAt` | String | no | — |
| `formId` | String | no | — |
| `name` | String | no | — |
| `email` | String | no | — |
| `others` | [`othersSchema`](#othersschema) | no | — |

### `FormsSubmissionsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `submissions` | Vec<FormsSubmissionsSubmissionsSchema> | no | — |
| `meta` | [`metaSchema`](#metaschema) | no | — |

### `FormsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `forms` | Vec<FormsParams> | no | — |
| `total` | f64 | no | Total number of forms |

### `PageDetailsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | String | no | — |
| `title` | String | no | — |

### `metaSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `total` | f64 | no | — |
| `currentPage` | f64 | no | — |
| `nextPage` | f64 | no | — |
| `prevPage` | f64 | no | — |

### `othersSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `__submissions_other_field__` | String | no | — |
| `__custom_field_id__` | String | no | — |
| `eventData` | [`EventDataSchema`](#eventdataschema) | no | — |
| `fieldsOriSequance` | Vec<String> | no | — |

