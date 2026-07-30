# `medias`

**7** operations / **10** models in API v2 · **7** operations / **10** models in API v3

## How to call it

No hand-written service yet — reach these endpoints two ways:

**From Rust** (typed body via [`ghl-models`](https://docs.rs/ghl-models)):

```rust,ignore
// cargo add ghl-models --features medias
use ghl_models::v2::medias::*;

let body = serde_json::to_value(/* a Create…Dto from above */)?;
let out = ghl.request_raw("POST", "/path/", &[], Some(&body), None).await?;
```

**From an AI agent** (MCP meta-tools):

```json
{
  "name": "ghl_search_operations",
  "arguments": {
    "query": "",
    "module": "medias"
  }
}
```

## Endpoints — API v2

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `PUT` | `/medias/delete-files` | Bulk Delete / Trash Files or Folders | `medias.put_medias_delete_files` |
| `GET` | `/medias/files` | Get List of Files/ Folders | `medias.get_medias_files` |
| `POST` | `/medias/folder` | Create Folder | `medias.post_medias_folder` |
| `PUT` | `/medias/update-files` | Bulk Update Files/ Folders | `medias.put_medias_update_files` |
| `POST` | `/medias/upload-file` | Upload File into Media Storage | `medias.post_medias_upload_file` |
| `DELETE` | `/medias/{id}` | Delete File or Folder | `medias.delete_medias_by_id` |
| `POST` | `/medias/{id}` | Update File/ Folder | `medias.post_medias_by_id` |

### Endpoint details — v2

#### `PUT /medias/delete-files`

**Bulk Delete / Trash Files or Folders**

Soft-deletes or trashes multiple files and folders in a single request

Operation id: `medias.put_medias_delete_files` · `Version: 2021-07-28`

*Request body*: [`DeleteMediaObjectsBodyParams`](#deletemediaobjectsbodyparams)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "medias.put_medias_delete_files",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /medias/files`

**Get List of Files/ Folders**

Fetches list of files and folders from the media storage

Operation id: `medias.get_medias_files` · `Version: 2021-07-28` · Scopes: `medias.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `offset` | string | no | Number of files to skip in listing |
| `limit` | string | no | Number of files to show in the listing |
| `sortBy` | string | **yes** | Field to sorting the file listing by |
| `sortOrder` | string | **yes** | Direction in which file needs to be sorted |
| `type` | string | **yes** | Type |
| `query` | string | no | Query text |
| `altType` | enum: `location` | **yes** | AltType |
| `altId` | string | **yes** | location Id |
| `parentId` | string | no | parent id or folder id |
| `fetchAll` | string | no | Fetch all files or folders |

*Response*: [`GetFilesResponseDTO`](#getfilesresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "medias.get_medias_files",
    "query": {
      "sortBy": "<sortBy>",
      "sortOrder": "<sortOrder>",
      "type": "<type>",
      "altType": "<altType>",
      "altId": "<altId>"
    }
  }
}
```

</details>

#### `POST /medias/folder`

**Create Folder**

Creates a new folder in the media storage

Operation id: `medias.post_medias_folder` · `Version: 2021-07-28`

*Request body*: [`CreateFolderParams`](#createfolderparams)

*Response*: [`FolderDTO`](#folderdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "medias.post_medias_folder",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /medias/update-files`

**Bulk Update Files/ Folders**

Updates metadata or status of multiple files and folders

Operation id: `medias.put_medias_update_files` · `Version: 2021-07-28`

*Request body*: [`UpdateMediaObjects`](#updatemediaobjects)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "medias.put_medias_update_files",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /medias/upload-file`

**Upload File into Media Storage**

If hosted is set to true then fileUrl is required. Else file is required. If adding a file, maximum allowed is 25 MB

Operation id: `medias.post_medias_upload_file` · `Version: 2021-07-28` · Scopes: `medias.write`

*Response*: [`UploadFileResponseDTO`](#uploadfileresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "medias.post_medias_upload_file"
  }
}
```

</details>

#### `DELETE /medias/{id}`

**Delete File or Folder**

Deletes specific file or folder from the media storage

Operation id: `medias.delete_medias_by_id` · `Version: 2021-07-28` · Scopes: `medias.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altType` | enum: `location` | **yes** | AltType |
| `altId` | string | **yes** | location Id |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "medias.delete_medias_by_id",
    "path_params": {
      "id": "<id>"
    },
    "query": {
      "altType": "<altType>",
      "altId": "<altId>"
    }
  }
}
```

</details>

#### `POST /medias/{id}`

**Update File/ Folder**

Updates a single file or folder by ID

Operation id: `medias.post_medias_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Unique identifier of the file or folder to update |

*Request body*: [`UpdateObject`](#updateobject)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "medias.post_medias_by_id",
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
| `PUT` | `/medias/delete-files` | Bulk Delete / Trash Files or Folders | `v3:medias.put_medias_delete_files` |
| `GET` | `/medias/files` | Get List of Files/ Folders | `v3:medias.get_medias_files` |
| `POST` | `/medias/folder` | Create Folder | `v3:medias.post_medias_folder` |
| `PUT` | `/medias/update-files` | Bulk Update Files/ Folders | `v3:medias.put_medias_update_files` |
| `POST` | `/medias/upload-file` | Upload File into Media Storage | `v3:medias.post_medias_upload_file` |
| `DELETE` | `/medias/{id}` | Delete File or Folder | `v3:medias.delete_medias_by_id` |
| `POST` | `/medias/{id}` | Update File/ Folder | `v3:medias.post_medias_by_id` |

### Endpoint details — v3

#### `PUT /medias/delete-files`

**Bulk Delete / Trash Files or Folders**

Soft-deletes or trashes multiple files and folders in a single request

Operation id: `v3:medias.put_medias_delete_files` · `Version: v3`

*Request body*: [`DeleteMediaObjectsBodyParams`](#deletemediaobjectsbodyparams)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:medias.put_medias_delete_files",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /medias/files`

**Get List of Files/ Folders**

Fetches list of files and folders from the media storage

Operation id: `v3:medias.get_medias_files` · `Version: v3` · Scopes: `medias.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `offset` | string | no | Number of files to skip in listing |
| `limit` | string | no | Number of files to show in the listing |
| `sortBy` | string | **yes** | Field to sorting the file listing by |
| `sortOrder` | string | **yes** | Direction in which file needs to be sorted |
| `type` | string | **yes** | Type |
| `query` | string | no | Query text |
| `altType` | enum: `location` | **yes** | AltType |
| `altId` | string | **yes** | location Id |
| `parentId` | string | no | parent id or folder id |
| `fetchAll` | string | no | Fetch all files or folders |

*Response*: [`GetFilesResponseDTO`](#getfilesresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:medias.get_medias_files",
    "query": {
      "sortBy": "<sortBy>",
      "sortOrder": "<sortOrder>",
      "type": "<type>",
      "altType": "<altType>",
      "altId": "<altId>"
    }
  }
}
```

</details>

#### `POST /medias/folder`

**Create Folder**

Creates a new folder in the media storage

Operation id: `v3:medias.post_medias_folder` · `Version: v3`

*Request body*: [`CreateFolderParams`](#createfolderparams)

*Response*: [`FolderDTO`](#folderdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:medias.post_medias_folder",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /medias/update-files`

**Bulk Update Files/ Folders**

Updates metadata or status of multiple files and folders

Operation id: `v3:medias.put_medias_update_files` · `Version: v3`

*Request body*: [`UpdateMediaObjects`](#updatemediaobjects)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:medias.put_medias_update_files",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /medias/upload-file`

**Upload File into Media Storage**

If hosted is set to true then fileUrl is required. Else file is required. If adding a file, maximum allowed is 25 MB. For video files, the maximum allowed size is 500 MB.

Operation id: `v3:medias.post_medias_upload_file` · `Version: v3` · Scopes: `medias.write`

*Response*: [`UploadFileResponseDTO`](#uploadfileresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:medias.post_medias_upload_file"
  }
}
```

</details>

#### `DELETE /medias/{id}`

**Delete File or Folder**

Deletes specific file or folder from the media storage

Operation id: `v3:medias.delete_medias_by_id` · `Version: v3` · Scopes: `medias.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altType` | enum: `location` | **yes** | AltType |
| `altId` | string | **yes** | location Id |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:medias.delete_medias_by_id",
    "path_params": {
      "id": "<id>"
    },
    "query": {
      "altType": "<altType>",
      "altId": "<altId>"
    }
  }
}
```

</details>

#### `POST /medias/{id}`

**Update File/ Folder**

Updates a single file or folder by ID

Operation id: `v3:medias.post_medias_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Unique identifier of the file or folder to update |

*Request body*: [`UpdateObject`](#updateobject)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:medias.post_medias_by_id",
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

In Rust: `ghl_models::v2::medias::*` (enable the `medias` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/medias/).

### `CreateFolderParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id |
| `altType` | String — `location` | **yes** | Type of entity (location only) |
| `name` | String | **yes** | Name of the folder to be created |
| `parentId` | String | no | ID of the parent folder (optional) |

### `DeleteMediaObjectItem`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Unique identifier of the file or folder to be deleted |

### `DeleteMediaObjectsBodyParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `filesToBeDeleted` | Vec<DeleteMediaObjectItem> | **yes** | Array of file objects to be deleted or trashed |
| `altType` | String — `location` | **yes** | Type of entity that owns the files |
| `altId` | String | **yes** | Location identifier |
| `status` | String — `deleted`, `trashed` | **yes** | Status to set for the files (deleted or trashed) |

### `FolderDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location identifier that owns this folder |
| `altType` | String — `location` | **yes** | Type of entity that owns the folder |
| `name` | String | **yes** | Name of the folder |
| `parentId` | String | no | ID of the parent folder (null for root folders) |
| `type` | String | **yes** | Type of the object (always 'folder' for folders) |
| `deleted` | bool | no | Whether the folder has been deleted |
| `pendingUpload` | bool | no | Whether there are pending uploads to this folder |
| `category` | String | no | Primary category of content stored in the folder |
| `subCategory` | String | no | Sub-category of content stored in the folder |
| `isPrivate` | bool | no | Whether the folder is private and not publicly accessible |
| `relocatedFolder` | bool | no | Whether the folder has been moved from its original location |
| `migrationCompleted` | bool | no | Whether the data migration process has been completed for this folder |
| `appFolder` | bool | no | Whether this is a system-generated application folder |
| `isEssential` | bool | no | Whether the folder is essential and should not be deleted |
| `status` | String | no | Current status of the folder |
| `lastUpdatedBy` | String | no | ID of the user who last updated the folder |

### `GetFilesResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `files` | Vec<String> | **yes** | Array of File Objects |

### `MoveOrDeleteObjectParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `altType` | String | **yes** | — |
| `altId` | String | **yes** | — |
| `_id` | String | **yes** | — |

### `UpdateMediaObject`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier of the file or folder to be updated |
| `name` | String | no | New name for the file or folder |

### `UpdateMediaObjects`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location identifier |
| `altType` | String — `location` | **yes** | Type of entity that owns the files |
| `filesToBeUpdated` | Vec<UpdateMediaObject> | **yes** | Array of file objects to be updated |

### `UpdateObject`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | New name for the file or folder |
| `altType` | String — `location` | **yes** | Type of entity that owns the file or folder |
| `altId` | String | **yes** | Location identifier that owns the file or folder |

### `UploadFileResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `fileId` | String | **yes** | ID of the uploaded file |
| `url` | String | **yes** | Google Cloud Storage URL of the uploaded file |

## Data models — API v3

In Rust: `ghl_models::v3::medias::*` (enable the `medias` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/medias/).

### `CreateFolderParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id |
| `altType` | String — `location` | **yes** | Type of entity (location only) |
| `name` | String | **yes** | Name of the folder to be created |
| `parentId` | String | no | ID of the parent folder (optional) |

### `DeleteMediaObjectItem`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Unique identifier of the file or folder to be deleted |

### `DeleteMediaObjectsBodyParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `filesToBeDeleted` | Vec<DeleteMediaObjectItem> | **yes** | Array of file objects to be deleted or trashed |
| `altType` | String — `location` | **yes** | Type of entity that owns the files |
| `altId` | String | **yes** | Location identifier |
| `status` | String — `deleted`, `trashed` | **yes** | Status to set for the files (deleted or trashed) |

### `FolderDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location identifier that owns this folder |
| `altType` | String — `location` | **yes** | Type of entity that owns the folder |
| `name` | String | **yes** | Name of the folder |
| `parentId` | String | no | ID of the parent folder (null for root folders) |
| `type` | String | **yes** | Type of the object (always 'folder' for folders) |
| `deleted` | bool | no | Whether the folder has been deleted |
| `pendingUpload` | bool | no | Whether there are pending uploads to this folder |
| `category` | String | no | Primary category of content stored in the folder |
| `subCategory` | String | no | Sub-category of content stored in the folder |
| `isPrivate` | bool | no | Whether the folder is private and not publicly accessible |
| `relocatedFolder` | bool | no | Whether the folder has been moved from its original location |
| `migrationCompleted` | bool | no | Whether the data migration process has been completed for this folder |
| `appFolder` | bool | no | Whether this is a system-generated application folder |
| `isEssential` | bool | no | Whether the folder is essential and should not be deleted |
| `status` | String | no | Current status of the folder |
| `lastUpdatedBy` | String | no | ID of the user who last updated the folder |

### `GetFilesResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `files` | Vec<String> | **yes** | Array of File Objects |

### `MoveOrDeleteObjectParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `altType` | String | **yes** | — |
| `altId` | String | **yes** | — |
| `_id` | String | **yes** | — |

### `UpdateMediaObject`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier of the file or folder to be updated |
| `name` | String | no | New name for the file or folder |

### `UpdateMediaObjects`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location identifier |
| `altType` | String — `location` | **yes** | Type of entity that owns the files |
| `filesToBeUpdated` | Vec<UpdateMediaObject> | **yes** | Array of file objects to be updated |

### `UpdateObject`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | New name for the file or folder |
| `altType` | String — `location` | **yes** | Type of entity that owns the file or folder |
| `altId` | String | **yes** | Location identifier that owns the file or folder |

### `UploadFileResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `fileId` | String | **yes** | ID of the uploaded file |
| `url` | String | **yes** | Google Cloud Storage URL of the uploaded file |

