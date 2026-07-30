# `courses`

**1** operations / **10** models in API v2 · **1** operations / **10** models in API v3

## How to call it

No hand-written service yet — reach these endpoints two ways:

**From Rust** (typed body via [`ghl-models`](https://docs.rs/ghl-models)):

```rust,ignore
// cargo add ghl-models --features courses
use ghl_models::v2::courses::*;

let body = serde_json::to_value(/* a Create…Dto from above */)?;
let out = ghl.request_raw("POST", "/path/", &[], Some(&body), None).await?;
```

**From an AI agent** (MCP meta-tools):

```json
{
  "name": "ghl_search_operations",
  "arguments": {
    "query": "",
    "module": "courses"
  }
}
```

## Endpoints — API v2

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `POST` | `/courses/courses-exporter/public/import` | Import Courses | `courses.post_courses_courses_exporter_public_import` |

### Endpoint details — v2

#### `POST /courses/courses-exporter/public/import`

**Import Courses**

Import Courses through public channels

Operation id: `courses.post_courses_courses_exporter_public_import` · `Version: 2021-07-28`

*Request body*: [`PublicExporterPayload`](#publicexporterpayload)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "courses.post_courses_courses_exporter_public_import",
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
| `POST` | `/courses/courses-exporter/public/import` | Import Courses | `v3:courses.post_courses_courses_exporter_public_import` |

### Endpoint details — v3

#### `POST /courses/courses-exporter/public/import`

**Import Courses**

Import Courses through public channels

Operation id: `v3:courses.post_courses_courses_exporter_public_import` · `Version: v3`

*Request body*: [`PublicExporterPayload`](#publicexporterpayload)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:courses.post_courses_courses_exporter_public_import",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::courses::*` (enable the `courses` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/courses/).

### `CategoryInterface`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | — |
| `visibility` | [`visibility`](#visibility) | **yes** | — |
| `thumbnailUrl` | String | no | — |
| `posts` | Vec<PostInterface> | no | — |
| `subCategories` | Vec<SubCategoryInterface> | no | — |

### `InstructorDetails`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | — |
| `description` | String | **yes** | — |

### `PostInterface`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | — |
| `visibility` | [`visibility`](#visibility) | **yes** | — |
| `thumbnailUrl` | String | no | — |
| `contentType` | [`contentType`](#contenttype) | **yes** | — |
| `description` | String | **yes** | — |
| `bucketVideoUrl` | String | no | — |
| `postMaterials` | Vec<PostMaterialInterface> | no | — |

### `PostMaterialInterface`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | — |
| `type` | [`type`](#type) | **yes** | — |
| `url` | String | **yes** | — |

### `ProductInterface`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | — |
| `description` | String | **yes** | — |
| `imageUrl` | String | no | — |
| `categories` | Vec<CategoryInterface> | **yes** | — |
| `instructorDetails` | [`InstructorDetails`](#instructordetails) | no | — |

### `PublicExporterPayload`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | — |
| `userId` | String | no | — |
| `products` | Vec<ProductInterface> | **yes** | — |

### `SubCategoryInterface`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | — |
| `visibility` | [`visibility`](#visibility) | **yes** | — |
| `thumbnailUrl` | String | no | — |
| `posts` | Vec<PostInterface> | no | — |

### `contentType`

String enum. Allowed values: `video`, `assignment`, `quiz`

### `type`

String enum. Allowed values: `pdf`, `image`, `docx`, `pptx`, `xlsx`, `html`, `dotx`, `epub`, `webp`, `gdoc`, `mp3`, `doc`, `txt`, `zip`, `ppt`, `key`, `htm`, `xls`, `odp`, `odt`, `rtf`, `m4a`, `ods`, `mp4`, `ai`, `avi`, `mov`, `wmv`, `mkv`, `wav` …

### `visibility`

String enum. Allowed values: `published`, `draft`

## Data models — API v3

In Rust: `ghl_models::v3::courses::*` (enable the `courses` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/courses/).

### `CategoryInterface`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | — |
| `visibility` | [`visibility`](#visibility) | **yes** | — |
| `thumbnailUrl` | String | no | — |
| `posts` | Vec<PostInterface> | no | — |
| `subCategories` | Vec<SubCategoryInterface> | no | — |

### `InstructorDetails`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | — |
| `description` | String | **yes** | — |

### `PostInterface`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | — |
| `visibility` | [`visibility`](#visibility) | **yes** | — |
| `thumbnailUrl` | String | no | — |
| `contentType` | [`contentType`](#contenttype) | **yes** | — |
| `description` | String | **yes** | — |
| `bucketVideoUrl` | String | no | — |
| `postMaterials` | Vec<PostMaterialInterface> | no | — |

### `PostMaterialInterface`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | — |
| `type` | [`type`](#type) | **yes** | — |
| `url` | String | **yes** | — |

### `ProductInterface`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | — |
| `description` | String | **yes** | — |
| `imageUrl` | String | no | — |
| `categories` | Vec<CategoryInterface> | **yes** | — |
| `instructorDetails` | [`InstructorDetails`](#instructordetails) | no | — |

### `PublicExporterPayload`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | — |
| `userId` | String | no | — |
| `products` | Vec<ProductInterface> | **yes** | — |

### `SubCategoryInterface`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | — |
| `visibility` | [`visibility`](#visibility) | **yes** | — |
| `thumbnailUrl` | String | no | — |
| `posts` | Vec<PostInterface> | no | — |

### `contentType`

String enum. Allowed values: `video`, `assignment`, `quiz`

### `type`

String enum. Allowed values: `pdf`, `image`, `docx`, `pptx`, `xlsx`, `html`, `dotx`, `epub`, `webp`, `gdoc`, `mp3`, `doc`, `txt`, `zip`, `ppt`, `key`, `htm`, `xls`, `odp`, `odt`, `rtf`, `m4a`, `ods`, `mp4`, `ai`, `avi`, `mov`, `wmv`, `mkv`, `wav` …

### `visibility`

String enum. Allowed values: `published`, `draft`

