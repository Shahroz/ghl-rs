# `email-isv`

**1** operations / **4** models in API v2 · **1** operations / **4** models in API v3

## How to call it

No hand-written service yet — reach these endpoints two ways:

**From Rust** (typed body via [`ghl-models`](https://docs.rs/ghl-models)):

```rust,ignore
// cargo add ghl-models --features email-isv
use ghl_models::v2::email_isv::*;

let body = serde_json::to_value(/* a Create…Dto from above */)?;
let out = ghl.request_raw("POST", "/path/", &[], Some(&body), None).await?;
```

**From an AI agent** (MCP meta-tools):

```json
{
  "name": "ghl_search_operations",
  "arguments": {
    "query": "",
    "module": "email-isv"
  }
}
```

## Endpoints — API v2

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `POST` | `/email/verify` | Email Verification | `email-isv.post_email_verify` |

### Endpoint details — v2

#### `POST /email/verify`

**Email Verification**

Verify Email

Operation id: `email-isv.post_email_verify` · `Version: 2021-07-28`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id, The email verification charges will be deducted from this location (if rebilling is enabled) / company wallet |

*Request body*: [`VerificationBodyDto`](#verificationbodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "email-isv.post_email_verify",
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
| `POST` | `/email/verify` | Email Verification | `v3:email-isv.post_email_verify` |

### Endpoint details — v3

#### `POST /email/verify`

**Email Verification**

Verify Email

Operation id: `v3:email-isv.post_email_verify` · `Version: v3` · Scopes: `lc-email.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id, The email verification charges will be deducted from this location (if rebilling is enabled) / company wallet |

*Request body*: [`VerificationBodyDto`](#verificationbodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:email-isv.post_email_verify",
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

In Rust: `ghl_models::v2::email_isv::*` (enable the `email-isv` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/email_isv/).

### `EmailNotVerifiedResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `verified` | bool | **yes** | Email verification not processed |
| `message` | String | no | Email verification failure message |
| `address` | String | no | Email address |

### `EmailVerifiedResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `reason` | Vec<String> | no | Reason for email verification failure |
| `result` | String — `deliverable`, `undeliverable`, `do_not_send`, `unknown`, `catch_all` | **yes** | Email verification result |
| `risk` | String — `high`, `low`, `medium`, `unknown` | **yes** | Risk level of email sending to bounce |
| `address` | String | **yes** | Email address |
| `leadconnectorRecomendation` | [`LeadConnectorRecomandationDto`](#leadconnectorrecomandationdto) | **yes** | Lead Connector email verification recomendation |

### `LeadConnectorRecomandationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `isEmailValid` | bool | no | Email verification status |

### `VerificationBodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `email`, `contact` | **yes** | Email Verification type |
| `verify` | String | **yes** | Email Verification recepient (email address / contactId) |

## Data models — API v3

In Rust: `ghl_models::v3::email_isv::*` (enable the `email-isv` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/email_isv/).

### `EmailNotVerifiedResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `verified` | bool | **yes** | Email verification not processed |
| `message` | String | no | Email verification failure message |
| `address` | String | no | Email address |

### `EmailVerifiedV3ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `reason` | Vec<String> | no | Reason for email verification failure |
| `result` | String — `deliverable`, `undeliverable`, `do_not_send`, `unknown`, `catch_all` | **yes** | Email verification result |
| `risk` | String — `high`, `low`, `medium`, `unknown` | **yes** | Risk level of email sending to bounce |
| `address` | String | **yes** | Email address |
| `leadConnectorRecommendation` | [`LeadConnectorRecommendationDto`](#leadconnectorrecommendationdto) | no | Lead Connector email verification recommendation |

### `LeadConnectorRecommendationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `isEmailValid` | bool | no | Email verification status |

### `VerificationBodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `email`, `contact` | **yes** | Email Verification type |
| `verify` | String | **yes** | Email Verification recepient (email address / contactId) |

