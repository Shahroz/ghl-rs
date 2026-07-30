# `courses`

**1** operations / **10** models in API v2 · **1** operations / **10** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `courses` cargo feature on `ghl-sdk`, then call any of the 2 generated methods on `ghl.courses()` (v2) or `ghl.v3().courses()` (v3):

```toml
ghl-sdk = { version = "0.5", features = ["courses"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `POST` | `/courses/courses-exporter/public/import` | Import Courses | `import_courses()` | `courses.post_courses_courses_exporter_public_import` |

### Endpoint details — v2

#### `POST /courses/courses-exporter/public/import`

**Import Courses**

Import Courses through public channels

Operation id: `courses.post_courses_courses_exporter_public_import` · `Version: 2021-07-28`

*Request body*: [`PublicExporterPayload`](#publicexporterpayload)

*Rust*:

```rust,ignore
let out = ghl.courses().import_courses(&body).await?;
```

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

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `POST` | `/courses/courses-exporter/public/import` | Import Courses | `import_courses()` | `v3:courses.post_courses_courses_exporter_public_import` |

### Endpoint details — v3

#### `POST /courses/courses-exporter/public/import`

**Import Courses**

Import Courses through public channels

Operation id: `v3:courses.post_courses_courses_exporter_public_import` · `Version: v3`

*Request body*: [`PublicExporterPayload`](#publicexporterpayload)

*Rust*:

```rust,ignore
let out = ghl.v3().courses().import_courses(&body).await?;
```

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

