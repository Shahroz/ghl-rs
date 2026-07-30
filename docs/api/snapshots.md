# `snapshots`

**4** operations / **8** models in API v2 · **4** operations / **8** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `snapshots` cargo feature on `ghl-sdk`, then call any of the 8 generated methods on `ghl.snapshots()` (v2) or `ghl.v3().snapshots()` (v3):

```toml
ghl-sdk = { version = "0.5", features = ["snapshots"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/snapshots/` | Get Snapshots | `get_snapshots()` | `snapshots.get_snapshots` |
| `POST` | `/snapshots/share/link` | Create Snapshot Share Link | `create_snapshot_share_link()` | `snapshots.post_snapshots_share_link` |
| `GET` | `/snapshots/snapshot-status/{snapshotId}` | Get Snapshot Push between Dates | `get_snapshot_push_between_dates()` | `snapshots.get_snapshots_snapshot_status_by_snapshotId` |
| `GET` | `/snapshots/snapshot-status/{snapshotId}/location/{locationId}` | Get Last Snapshot Push | `get_last_snapshot_push()` | `snapshots.get_snapshots_snapshot_status_by_snapshotId_location_by_locationId` |

### Endpoint details — v2

#### `GET /snapshots/`

**Get Snapshots**

Get a list of all own and imported Snapshots

Operation id: `snapshots.get_snapshots` · `Version: 2021-07-28`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | Company Id |

*Response*: [`GetSnapshotsSuccessfulResponseDto`](#getsnapshotssuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::snapshots::GetSnapshotsParams;

let params = GetSnapshotsParams::new("companyId");
let out = ghl.snapshots().get_snapshots(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "snapshots.get_snapshots",
    "query": {
      "companyId": "<companyId>"
    }
  }
}
```

</details>

#### `POST /snapshots/share/link`

**Create Snapshot Share Link**

Create a share link for snapshot

Operation id: `snapshots.post_snapshots_share_link` · `Version: 2021-07-28`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | — |

*Request body*: [`CreateSnapshotShareLinkRequestDTO`](#createsnapshotsharelinkrequestdto)

*Response*: [`CreateSnapshotShareLinkSuccessfulResponseDTO`](#createsnapshotsharelinksuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::snapshots::CreateSnapshotShareLinkParams;

let params = CreateSnapshotShareLinkParams::new("companyId");
let out = ghl.snapshots().create_snapshot_share_link(&params, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "snapshots.post_snapshots_share_link",
    "query": {
      "companyId": "<companyId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /snapshots/snapshot-status/{snapshotId}`

**Get Snapshot Push between Dates**

Get list of sub-accounts snapshot pushed in time period

Operation id: `snapshots.get_snapshots_snapshot_status_by_snapshotId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `snapshotId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | — |
| `from` | string | **yes** | — |
| `to` | string | **yes** | — |
| `lastDoc` | string | **yes** | Id for last document till what you want to skip |
| `limit` | string | **yes** | — |

*Response*: [`GetSnapshotPushStatusSuccessfulResponseDTO`](#getsnapshotpushstatussuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::snapshots::GetSnapshotPushBetweenDatesParams;

let params = GetSnapshotPushBetweenDatesParams::new("companyId", "from", "to", "lastDoc", "limit");
let out = ghl.snapshots().get_snapshot_push_between_dates(&snapshotId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "snapshots.get_snapshots_snapshot_status_by_snapshotId",
    "path_params": {
      "snapshotId": "<snapshotId>"
    },
    "query": {
      "companyId": "<companyId>",
      "from": "<from>",
      "to": "<to>",
      "lastDoc": "<lastDoc>",
      "limit": "<limit>"
    }
  }
}
```

</details>

#### `GET /snapshots/snapshot-status/{snapshotId}/location/{locationId}`

**Get Last Snapshot Push**

Get Latest Snapshot Push Status for a location id

Operation id: `snapshots.get_snapshots_snapshot_status_by_snapshotId_location_by_locationId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `snapshotId` | string | **yes** | — |
| `locationId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | — |

*Response*: [`GetLatestSnapshotPushStatusSuccessfulResponseDTO`](#getlatestsnapshotpushstatussuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::snapshots::GetLastSnapshotPushParams;

let params = GetLastSnapshotPushParams::new("companyId");
let out = ghl.snapshots().get_last_snapshot_push(&snapshotId, &locationId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "snapshots.get_snapshots_snapshot_status_by_snapshotId_location_by_locationId",
    "path_params": {
      "snapshotId": "<snapshotId>",
      "locationId": "<locationId>"
    },
    "query": {
      "companyId": "<companyId>"
    }
  }
}
```

</details>

## Endpoints — API v3

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/snapshots/` | Get Snapshots | `get_snapshots()` | `v3:snapshots.get_snapshots` |
| `POST` | `/snapshots/share/link` | Create Snapshot Share Link | `create_snapshot_share_link()` | `v3:snapshots.post_snapshots_share_link` |
| `GET` | `/snapshots/snapshot-status/{snapshotId}` | Get Snapshot Push between Dates | `get_snapshot_push_between_dates()` | `v3:snapshots.get_snapshots_snapshot_status_by_snapshotId` |
| `GET` | `/snapshots/snapshot-status/{snapshotId}/location/{locationId}` | Get Last Snapshot Push | `get_last_snapshot_push()` | `v3:snapshots.get_snapshots_snapshot_status_by_snapshotId_location_by_locationId` |

### Endpoint details — v3

#### `GET /snapshots/`

**Get Snapshots**

Get a list of all own and imported Snapshots

Operation id: `v3:snapshots.get_snapshots` · `Version: v3`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | Company Id |

*Response*: [`GetSnapshotsSuccessfulResponseDto`](#getsnapshotssuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::snapshots::GetSnapshotsParams;

let params = GetSnapshotsParams::new("companyId");
let out = ghl.v3().snapshots().get_snapshots(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:snapshots.get_snapshots",
    "query": {
      "companyId": "<companyId>"
    }
  }
}
```

</details>

#### `POST /snapshots/share/link`

**Create Snapshot Share Link**

Create a share link for snapshot

Operation id: `v3:snapshots.post_snapshots_share_link` · `Version: v3`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | — |

*Request body*: [`CreateSnapshotShareLinkRequestDTO`](#createsnapshotsharelinkrequestdto)

*Response*: [`CreateSnapshotShareLinkSuccessfulResponseDTO`](#createsnapshotsharelinksuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::snapshots::CreateSnapshotShareLinkParams;

let params = CreateSnapshotShareLinkParams::new("companyId");
let out = ghl.v3().snapshots().create_snapshot_share_link(&params, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:snapshots.post_snapshots_share_link",
    "query": {
      "companyId": "<companyId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /snapshots/snapshot-status/{snapshotId}`

**Get Snapshot Push between Dates**

Get list of sub-accounts snapshot pushed in time period

Operation id: `v3:snapshots.get_snapshots_snapshot_status_by_snapshotId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `snapshotId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | — |
| `from` | string | **yes** | Only accepts ISO 8601 format |
| `to` | string | **yes** | Only accepts ISO 8601 format |
| `lastDoc` | string | **yes** | Id for last document till what you want to skip |
| `limit` | string | no | Limit of documents to return. Default is 20 |

*Response*: [`GetSnapshotPushStatusSuccessfulResponseDTO`](#getsnapshotpushstatussuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::snapshots::GetSnapshotPushBetweenDatesParams;

let params = GetSnapshotPushBetweenDatesParams::new("companyId", "from", "to", "lastDoc");
let out = ghl.v3().snapshots().get_snapshot_push_between_dates(&snapshotId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:snapshots.get_snapshots_snapshot_status_by_snapshotId",
    "path_params": {
      "snapshotId": "<snapshotId>"
    },
    "query": {
      "companyId": "<companyId>",
      "from": "<from>",
      "to": "<to>",
      "lastDoc": "<lastDoc>"
    }
  }
}
```

</details>

#### `GET /snapshots/snapshot-status/{snapshotId}/location/{locationId}`

**Get Last Snapshot Push**

Get Latest Snapshot Push Status for a location id

Operation id: `v3:snapshots.get_snapshots_snapshot_status_by_snapshotId_location_by_locationId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `snapshotId` | string | **yes** | — |
| `locationId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | — |

*Response*: [`GetLatestSnapshotPushStatusSuccessfulResponseDTO`](#getlatestsnapshotpushstatussuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::snapshots::GetLastSnapshotPushParams;

let params = GetLastSnapshotPushParams::new("companyId");
let out = ghl.v3().snapshots().get_last_snapshot_push(&snapshotId, &locationId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:snapshots.get_snapshots_snapshot_status_by_snapshotId_location_by_locationId",
    "path_params": {
      "snapshotId": "<snapshotId>",
      "locationId": "<locationId>"
    },
    "query": {
      "companyId": "<companyId>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::snapshots::*` (enable the `snapshots` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/snapshots/).

### `CreateSnapshotShareLinkRequestDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `snapshot_id` | String | **yes** | id for snapshot to be shared |
| `share_type` | String — `link`, `permanent_link`, `agency_link`, `location_link` | **yes** | Type of share link to generate |
| `relationship_number` | String | no | Comma separated Relationship number of Agencies to create agency restricted share link |
| `share_location_id` | String | no | Comma separated Sub-Account ids to create sub-account restricted share link |

### `CreateSnapshotShareLinkSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | id for shared snapshot |
| `shareLink` | String | no | Share Link for snapshot |

### `GetLatestSnapshotPushStatusSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | [`SnapshotStatusSchemaWithAssets`](#snapshotstatusschemawithassets) | no | — |

### `GetSnapshotPushStatusSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<SnapshotStatusSchema> | no | — |

### `GetSnapshotsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `snapshots` | Vec<SnapshotsSchema> | no | — |

### `SnapshotStatusSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Document id |
| `locationId` | String | no | Sub-account id |
| `status` | String | no | Status of snapshot push |
| `dateAdded` | String | no | Timestamp of when snapshot processing starts for sub-account |

### `SnapshotStatusSchemaWithAssets`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Document id |
| `locationId` | String | no | Sub-account id |
| `status` | String | no | Status of snapshot push |
| `completed` | Vec<String> | no | List of completed assets |
| `pending` | Vec<String> | no | List of pending assets |

### `SnapshotsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Snapshot Id. |
| `name` | String | no | Name of the snapshot |
| `type` | String | no | Type of snapshot - own or imported. |

## Data models — API v3

In Rust: `ghl_models::v3::snapshots::*` (enable the `snapshots` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/snapshots/).

### `CreateSnapshotShareLinkRequestDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `snapshot_id` | String | **yes** | id for snapshot to be shared |
| `share_type` | String — `link`, `permanent_link`, `agency_link`, `location_link` | **yes** | Type of share link to generate |
| `relationship_number` | String | no | Comma separated Relationship number of Agencies to create agency restricted share link |
| `share_location_id` | String | no | Comma separated Sub-Account ids to create sub-account restricted share link |

### `CreateSnapshotShareLinkSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | id for shared snapshot |
| `shareLink` | String | no | Share Link for snapshot |

### `GetLatestSnapshotPushStatusSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | [`SnapshotStatusSchemaWithAssets`](#snapshotstatusschemawithassets) | no | — |

### `GetSnapshotPushStatusSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<SnapshotStatusSchema> | no | — |

### `GetSnapshotsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `snapshots` | Vec<SnapshotsSchema> | no | — |

### `SnapshotStatusSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Document id |
| `locationId` | String | no | Sub-account id |
| `status` | String | no | Status of snapshot push |
| `dateAdded` | String | no | Timestamp of when snapshot processing starts for sub-account |

### `SnapshotStatusSchemaWithAssets`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Document id |
| `locationId` | String | no | Sub-account id |
| `status` | String | no | Status of snapshot push |
| `completed` | Vec<String> | no | List of completed assets |
| `pending` | Vec<String> | no | List of pending assets |

### `SnapshotsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Snapshot Id. |
| `name` | String | no | Name of the snapshot |
| `type` | String | no | Type of snapshot - own or imported. |

