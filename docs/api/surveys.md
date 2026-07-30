# `surveys`

**2** operations / **9** models in API v2 · **2** operations / **9** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `surveys` cargo feature on `ghl-sdk`, then call any of the 2 generated methods on `ghl.surveys()`:

```toml
ghl-sdk = { version = "0.4", features = ["surveys"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/surveys/` | Get Surveys | `get_surveys()` | `surveys.get_surveys` |
| `GET` | `/surveys/submissions` | Get Surveys Submissions | `get_surveys_submissions()` | `surveys.get_surveys_submissions` |

### Endpoint details — v2

#### `GET /surveys/`

**Get Surveys**

Operation id: `surveys.get_surveys` · `Version: 2021-07-28` · Scopes: `surveys.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `skip` | number | no | — |
| `limit` | number | no | Limit Per Page records count. will allow maximum up to 50 and default will be 10 |
| `type` | string | no | — |

*Response*: [`GetSurveysSuccessfulResponseDto`](#getsurveyssuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::surveys::GetSurveysParams;

let params = GetSurveysParams::new("locationId");
let out = ghl.surveys().get_surveys(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "surveys.get_surveys",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /surveys/submissions`

**Get Surveys Submissions**

Operation id: `surveys.get_surveys_submissions` · `Version: 2021-07-28` · Scopes: `surveys.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `page` | number | no | Page No. By default it will be 1 |
| `limit` | number | no | Limit Per Page records count. will allow maximum up to 100 and default will be 20 |
| `surveyId` | string | no | Filter submission by survey id |
| `q` | string | no | Filter by contactId, name, email or phone no. |
| `startAt` | string | no | Get submission by starting of this date. By default it will be same date of last month(YYYY-MM-DD). |
| `endAt` | string | no | Get submission by ending of this date. By default it will be current date(YYYY-MM-DD). |

*Response*: [`GetSurveysSubmissionSuccessfulResponseDto`](#getsurveyssubmissionsuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::surveys::GetSurveysSubmissionsParams;

let params = GetSurveysSubmissionsParams::new("locationId");
let out = ghl.surveys().get_surveys_submissions(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "surveys.get_surveys_submissions",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

## Endpoints — API v3

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/surveys/` | Get Surveys | `v3:surveys.get_surveys` |
| `GET` | `/surveys/submissions` | Get Surveys Submissions | `v3:surveys.get_surveys_submissions` |

### Endpoint details — v3

#### `GET /surveys/`

**Get Surveys**

Operation id: `v3:surveys.get_surveys` · `Version: v3` · Scopes: `surveys.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `skip` | number | no | — |
| `limit` | number | no | Limit Per Page records count. will allow maximum up to 50 and default will be 10 |
| `type` | string | no | — |

*Response*: [`GetSurveysSuccessfulResponseDto`](#getsurveyssuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:surveys.get_surveys",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /surveys/submissions`

**Get Surveys Submissions**

Operation id: `v3:surveys.get_surveys_submissions` · `Version: v3` · Scopes: `surveys.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `page` | number | no | Page No. By default it will be 1 |
| `limit` | number | no | Limit Per Page records count. will allow maximum up to 100 and default will be 20 |
| `surveyId` | string | no | Filter submission by survey id |
| `q` | string | no | Filter by contactId, name, email or phone no. |
| `startAt` | string | no | Get submission by starting of this date. By default it will be same date of last month(YYYY-MM-DD). |
| `endAt` | string | no | Get submission by ending of this date. By default it will be current date(YYYY-MM-DD). |

*Response*: [`GetSurveysSubmissionSuccessfulResponseDto`](#getsurveyssubmissionsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:surveys.get_surveys_submissions",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::surveys::*` (enable the `surveys` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/surveys/).

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

### `GetSurveysSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `locationId` | String | no | — |

### `GetSurveysSubmissionSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `submissions` | Vec<SubmissionSchema> | no | — |
| `meta` | [`metaSchema`](#metaschema) | no | — |

### `GetSurveysSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `surveys` | Vec<GetSurveysSchema> | no | — |
| `total` | f64 | no | Number of surveys |

### `PageDetailsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | String | no | — |
| `title` | String | no | — |

### `SubmissionSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `contactId` | String | no | — |
| `createdAt` | String | no | — |
| `surveyId` | String | no | — |
| `name` | String | no | — |
| `email` | String | no | — |
| `others` | [`othersSchema`](#othersschema) | no | — |

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

In Rust: `ghl_models::v3::surveys::*` (enable the `surveys` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/surveys/).

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

### `GetSurveysSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `locationId` | String | no | — |

### `GetSurveysSubmissionSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `submissions` | Vec<SubmissionSchema> | no | — |
| `meta` | [`metaSchema`](#metaschema) | no | — |

### `GetSurveysSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `surveys` | Vec<GetSurveysSchema> | no | — |
| `total` | f64 | no | Number of surveys |

### `PageDetailsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | String | no | — |
| `title` | String | no | — |

### `SubmissionSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `contactId` | String | no | — |
| `createdAt` | String | no | — |
| `surveyId` | String | no | — |
| `name` | String | no | — |
| `email` | String | no | — |
| `others` | [`othersSchema`](#othersschema) | no | — |

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

