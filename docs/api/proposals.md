# `proposals`

**4** operations / **21** models in API v2 · **4** operations / **21** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `proposals` cargo feature on `ghl-sdk`, then call any of the 8 generated methods on `ghl.proposals()` (v2) or `ghl.v3().proposals()` (v3):

```toml
ghl-sdk = { version = "0.5", features = ["proposals"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/proposals/document` | List documents | `list_documents()` | `proposals.get_proposals_document` |
| `POST` | `/proposals/document/send` | Send document | `send_document()` | `proposals.post_proposals_document_send` |
| `GET` | `/proposals/templates` | List templates | `list_templates()` | `proposals.get_proposals_templates` |
| `POST` | `/proposals/templates/send` | Send template | `send_template()` | `proposals.post_proposals_templates_send` |

### Endpoint details — v2

#### `GET /proposals/document`

**List documents**

List documents for a location

Operation id: `proposals.get_proposals_document` · `Version: 2021-07-28`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `status` | enum: `draft`, `sent`, `viewed`, `completed`, `accepted` | no | Document status, pass as comma separated values |
| `paymentStatus` | enum: `waiting_for_payment`, `paid`, `no_payment` | no | Payment status, pass as comma separated values |
| `limit` | number | no | Limit to fetch number of records |
| `skip` | number | no | Skip number of records |
| `query` | string | no | Search string |
| `dateFrom` | string | no | Date start from (ISO 8601), dateFrom & DateTo must be provided together |
| `dateTo` | string | no | Date to (ISO 8601), dateFrom & DateTo must be provided together |

*Response*: [`DocumentListResponseDto`](#documentlistresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::proposals::ListDocumentsParams;

let params = ListDocumentsParams::new("locationId");
let out = ghl.proposals().list_documents(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "proposals.get_proposals_document",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /proposals/document/send`

**Send document**

Send document to a client

Operation id: `proposals.post_proposals_document_send` · `Version: 2021-07-28`

*Request body*: [`SendDocumentDto`](#senddocumentdto)

*Response*: [`SendDocumentResponseDto`](#senddocumentresponsedto)

*Rust*:

```rust,ignore
let out = ghl.proposals().send_document(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "proposals.post_proposals_document_send",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /proposals/templates`

**List templates**

List document contract templates for a location

Operation id: `proposals.get_proposals_templates` · `Version: 2021-07-28`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `dateFrom` | string | no | Date start from (ISO 8601) |
| `dateTo` | string | no | Date to (ISO 8601) |
| `type` | string | no | Comma-separated template types. Valid values: proposal, estimate, contentLibrary |
| `name` | string | no | Template Name |
| `isPublicDocument` | boolean | no | If the docForm is a DocForm |
| `userId` | string | no | User Id, required when isPublicDocument is true |
| `limit` | string | no | Limit |
| `skip` | string | no | Skip |

*Response*: [`TemplateListPaginationResponseDTO`](#templatelistpaginationresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::proposals::ListTemplatesParams;

let params = ListTemplatesParams::new("locationId");
let out = ghl.proposals().list_templates(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "proposals.get_proposals_templates",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /proposals/templates/send`

**Send template**

Send template to a client

Operation id: `proposals.post_proposals_templates_send` · `Version: 2021-07-28`

*Request body*: [`SendDocumentFromPublicApiBodyDto`](#senddocumentfrompublicapibodydto)

*Response*: [`SendTemplateResponseDto`](#sendtemplateresponsedto)

*Rust*:

```rust,ignore
let out = ghl.proposals().send_template(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "proposals.post_proposals_templates_send",
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
| `GET` | `/proposals/document` | List documents | `list_documents()` | `v3:proposals.get_proposals_document` |
| `POST` | `/proposals/document/send` | Send document | `send_document()` | `v3:proposals.post_proposals_document_send` |
| `GET` | `/proposals/templates` | List templates | `list_templates()` | `v3:proposals.get_proposals_templates` |
| `POST` | `/proposals/templates/send` | Send template | `send_template()` | `v3:proposals.post_proposals_templates_send` |

### Endpoint details — v3

#### `GET /proposals/document`

**List documents**

List documents for a location

Operation id: `v3:proposals.get_proposals_document` · `Version: v3`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `status` | enum: `draft`, `sent`, `viewed`, `completed`, `accepted` | no | Document status, pass as comma separated values |
| `paymentStatus` | enum: `waiting_for_payment`, `paid`, `no_payment` | no | Payment status, pass as comma separated values |
| `limit` | number | no | Limit to fetch number of records |
| `skip` | number | no | Skip number of records |
| `query` | string | no | Search string |
| `dateFrom` | string | no | Date start from (ISO 8601), dateFrom & DateTo must be provided together |
| `dateTo` | string | no | Date to (ISO 8601), dateFrom & DateTo must be provided together |

*Response*: [`DocumentListResponseDto`](#documentlistresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::proposals::ListDocumentsParams;

let params = ListDocumentsParams::new("locationId");
let out = ghl.v3().proposals().list_documents(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:proposals.get_proposals_document",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /proposals/document/send`

**Send document**

Send document to a client

Operation id: `v3:proposals.post_proposals_document_send` · `Version: v3`

*Request body*: [`SendDocumentDto`](#senddocumentdto)

*Response*: [`SendDocumentResponseDto`](#senddocumentresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().proposals().send_document(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:proposals.post_proposals_document_send",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /proposals/templates`

**List templates**

List document contract templates for a location

Operation id: `v3:proposals.get_proposals_templates` · `Version: v3`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `dateFrom` | string | no | Date start from (ISO 8601) |
| `dateTo` | string | no | Date to (ISO 8601) |
| `type` | string | no | Comma-separated template types. Valid values: proposal, estimate, contentLibrary |
| `name` | string | no | Template Name |
| `isPublicDocument` | boolean | no | If the docForm is a DocForm |
| `userId` | string | no | User Id, required when isPublicDocument is true |
| `limit` | string | no | Limit |
| `skip` | string | no | Skip |

*Response*: [`TemplateListPaginationResponseDTO`](#templatelistpaginationresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::proposals::ListTemplatesParams;

let params = ListTemplatesParams::new("locationId");
let out = ghl.v3().proposals().list_templates(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:proposals.get_proposals_templates",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /proposals/templates/send`

**Send template**

Send template to a client

Operation id: `v3:proposals.post_proposals_templates_send` · `Version: v3`

*Request body*: [`SendDocumentFromPublicApiBodyDto`](#senddocumentfrompublicapibodydto)

*Response*: [`SendTemplateResponseDto`](#sendtemplateresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().proposals().send_template(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:proposals.post_proposals_templates_send",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::proposals::*` (enable the `proposals` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/proposals/).

### `BadRequestDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | — |
| `message` | String | no | — |

### `CCRecipientItem`

| Field | Type | Required | Description |
|---|---|---|---|
| `email` | String | **yes** | Email |
| `id` | String | **yes** | Contact ID |
| `imageUrl` | String | **yes** | Contact Image URL |
| `contactName` | String | **yes** | Contact Name |
| `firstName` | String | **yes** | First Name |
| `lastName` | String | **yes** | Last Name |

### `DiscountDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the discount |
| `value` | f64 | **yes** | Discount value (either a percentage or custom amount) |
| `type` | String — `percentage`, `custom_amount` | **yes** | Type of discount |

### `DocumentDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location Id |
| `documentId` | String | **yes** | Document Id |
| `_id` | String | **yes** | Unique identifier |
| `name` | String | **yes** | Name of the document |
| `type` | String | **yes** | Type of the document |
| `deleted` | bool | **yes** | Whether the document is deleted |
| `isExpired` | bool | **yes** | Whether the document is expired |
| `documentRevision` | f64 | **yes** | Number of times document is moved to draft state |
| `fillableFields` | Vec<FillableFieldsDTO> | **yes** | Fillable fields |
| `grandTotal` | [`GrandTotalDto`](#grandtotaldto) | **yes** | Grand total object of the document |
| `locale` | String | **yes** | Locale of the location |
| `status` | Vec<String (enum)> | **yes** | Document status |
| `paymentStatus` | Vec<String (enum)> | **yes** | Payment status |
| `recipients` | Vec<RecipientItem> | **yes** | Recipients |
| `links` | Vec<ProposalEstimateLinksDto> | **yes** | Links for the document if its sent |
| `updatedAt` | String | **yes** | Date start from (ISO 8601) |
| `createdAt` | String | **yes** | Date to (ISO 8601) |

### `DocumentListResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `documents` | Vec<DocumentDto> | **yes** | List of documents |
| `total` | f64 | **yes** | Total records available |
| `whiteLabelBaseUrl` | f64 | no | WhiteLabel url for document |
| `whiteLabelBaseUrlForInvoice` | f64 | no | WhiteLabel url for invoice |

### `ELEMENTS_LOOKUP`

Element type

String enum. Allowed values: `Page`, `Text`, `Image`, `Video`, `Table`, `ProductList`, `PageBreak`, `Signature`, `PaymentDetails`, `TextField`, `DateField`, `InitialsField`, `Checkbox`, `Row`, `Column`

### `EntityReference`

Entity type

String enum. Allowed values: `contacts`, `users`

### `FillableFieldsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `fieldId` | String | **yes** | Field Id |
| `isRequired` | bool | **yes** | Is the field required |
| `hasCompleted` | bool | **yes** | Has the field been completed |
| `recipient` | String | **yes** | Recipient |
| `entityType` | [`EntityReference`](#entityreference) | **yes** | — |
| `id` | String | **yes** | Id |
| `type` | [`ELEMENTS_LOOKUP`](#elements-lookup) | **yes** | — |
| `value` | String | **yes** | Value of the field |

### `GrandTotalDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `amount` | f64 | **yes** | Total amount before discounts |
| `currency` | String | **yes** | Currency of the total amount |
| `discountPercentage` | f64 | **yes** | Total discount percentage applied |
| `discounts` | Vec<DiscountDto> | **yes** | List of applied discounts |

### `NotificationSendSettingDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `templateId` | String | **yes** | — |
| `subject` | String | **yes** | — |

### `NotificationSenderSettingDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `fromEmail` | String | **yes** | — |
| `fromName` | String | **yes** | — |

### `NotificationSettingsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `receive` | [`NotificationSendSettingDto`](#notificationsendsettingdto) | **yes** | — |
| `sender` | [`NotificationSenderSettingDto`](#notificationsendersettingdto) | **yes** | — |

### `ProposalEstimateLinksDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `referenceId` | String | **yes** | Reference ID |
| `documentId` | String | **yes** | Document ID |
| `recipientId` | String | **yes** | Recipient ID |
| `entityName` | String — `contacts`, `users` | **yes** | Entity name that the recipient belongs to |
| `recipientCategory` | String — `recipient`, `cc`, `bcc` | **yes** | Recipient category (recipient, cc, or bcc) |
| `documentRevision` | f64 | **yes** | Document revision number |
| `createdBy` | String | **yes** | Created by user ID |
| `deleted` | bool | **yes** | Whether the document is deleted |

### `RecipientItem`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Recipient Id |
| `firstName` | String | no | Recipient First Name |
| `lastName` | String | no | Recipient Last Name |
| `email` | String | **yes** | Recipient Email |
| `phoneNumber` | String | no | Recipient Phone Number |
| `phone` | String | no | Recipient Phone |
| `hasCompleted` | bool | **yes** | Recipient has completed the document |
| `role` | String — `user`, `signer` | **yes** | Recipient role |
| `isPrimary` | bool | **yes** | Recipient is primary |
| `signingOrder` | f64 | **yes** | Recipient signing order |
| `imgUrl` | String | no | Recipient image url |
| `ip` | String | no | Recipient ip |
| `userAgent` | String | no | Recipient user agent |
| `signedDate` | String | no | Recipient signed date |
| `contactName` | String | no | Recipient contact name |
| `country` | String | no | Recipient country |
| `entityName` | String | no | Recipient entity name |
| `initialsImgUrl` | String | no | Recipient initials image url |
| `lastViewedAt` | String | no | Recipient last viewed date |
| `shareLink` | String | no | Share link |

### `SendDocumentDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location Id |
| `documentId` | String | **yes** | Document Id |
| `documentName` | String | no | Document Name |
| `medium` | String — `link`, `email` | no | Medium to be used for sending the document |
| `ccRecipients` | Vec<CCRecipientItem> | no | CC Recipient |
| `notificationSettings` | [`NotificationSettingsDto`](#notificationsettingsdto) | no | — |
| `sentBy` | String | **yes** | Sent ByUser Id |

### `SendDocumentFromPublicApiBodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `templateId` | String | **yes** | Template Id |
| `userId` | String | **yes** | User Id |
| `sendDocument` | bool | no | Send Document |
| `locationId` | String | **yes** | Location Id |
| `contactId` | String | **yes** | Contact Id |
| `opportunityId` | String | no | Opportunity Id |

### `SendDocumentResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status |
| `links` | Vec<ProposalEstimateLinksDto> | **yes** | Links for all recipients |

### `SendTemplateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status |
| `links` | Vec<ProposalEstimateLinksDto> | **yes** | Links for all recipients |

### `TemplateListPaginationResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<TemplateListResponseDTO> | **yes** | Array of templates |
| `total` | f64 | **yes** | Total number of templates |
| `traceId` | String | no | Trace ID for request tracking |

### `TemplateListResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Template ID |
| `deleted` | bool | **yes** | Whether the template is deleted |
| `version` | f64 | **yes** | Template version |
| `name` | String | **yes** | Template name |
| `locationId` | String | **yes** | Location ID |
| `type` | String — `proposal`, `estimate`, `contentLibrary` | **yes** | Template type |
| `updatedBy` | String | **yes** | User ID who last updated the template |
| `isPublicDocument` | bool | **yes** | Whether the template is a public document |
| `createdAt` | String | **yes** | Template creation date |
| `updatedAt` | String | **yes** | Template last update date |
| `id` | String | **yes** | Template ID (alias for _id) |
| `documentCount` | f64 | no | Document count (only present when isPublicDocument is true) |
| `docFormUrl` | String | no | Document form URL (only present when isPublicDocument is true) |

### `UnauthorizedDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | — |
| `message` | String | no | — |
| `error` | String | no | — |

## Data models — API v3

In Rust: `ghl_models::v3::proposals::*` (enable the `proposals` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/proposals/).

### `BadRequestDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | — |
| `message` | String | no | — |

### `CCRecipientItem`

| Field | Type | Required | Description |
|---|---|---|---|
| `email` | String | **yes** | Email |
| `id` | String | **yes** | Contact ID |
| `imageUrl` | String | **yes** | Contact Image URL |
| `contactName` | String | **yes** | Contact Name |
| `firstName` | String | **yes** | First Name |
| `lastName` | String | **yes** | Last Name |

### `DiscountDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the discount |
| `value` | f64 | **yes** | Discount value (either a percentage or custom amount) |
| `type` | String — `percentage`, `custom_amount` | **yes** | Type of discount |

### `DocumentDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location Id |
| `documentId` | String | **yes** | Document Id |
| `_id` | String | **yes** | Unique identifier |
| `name` | String | **yes** | Name of the document |
| `type` | String | **yes** | Type of the document |
| `deleted` | bool | **yes** | Whether the document is deleted |
| `isExpired` | bool | **yes** | Whether the document is expired |
| `documentRevision` | f64 | **yes** | Number of times document is moved to draft state |
| `fillableFields` | Vec<FillableFieldsDTO> | **yes** | Fillable fields |
| `grandTotal` | [`GrandTotalDto`](#grandtotaldto) | **yes** | Grand total object of the document |
| `locale` | String | **yes** | Locale of the location |
| `status` | Vec<String (enum)> | **yes** | Document status |
| `paymentStatus` | Vec<String (enum)> | **yes** | Payment status |
| `recipients` | Vec<RecipientItem> | **yes** | Recipients |
| `links` | Vec<ProposalEstimateLinksDto> | **yes** | Links for the document if its sent |
| `updatedAt` | String | **yes** | Date start from (ISO 8601) |
| `createdAt` | String | **yes** | Date to (ISO 8601) |

### `DocumentListResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `documents` | Vec<DocumentDto> | **yes** | List of documents |
| `total` | f64 | **yes** | Total records available |
| `whiteLabelBaseUrl` | f64 | no | WhiteLabel url for document |
| `whiteLabelBaseUrlForInvoice` | f64 | no | WhiteLabel url for invoice |

### `ELEMENTS_LOOKUP`

Element type

String enum. Allowed values: `Page`, `Text`, `Image`, `Video`, `Table`, `ProductList`, `PageBreak`, `Signature`, `PaymentDetails`, `TextField`, `DateField`, `InitialsField`, `Checkbox`, `Row`, `Column`

### `EntityReference`

Entity type

String enum. Allowed values: `contacts`, `users`

### `FillableFieldsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `fieldId` | String | **yes** | Field Id |
| `isRequired` | bool | **yes** | Is the field required |
| `hasCompleted` | bool | **yes** | Has the field been completed |
| `recipient` | String | **yes** | Recipient |
| `entityType` | [`EntityReference`](#entityreference) | **yes** | — |
| `id` | String | **yes** | Id |
| `type` | [`ELEMENTS_LOOKUP`](#elements-lookup) | **yes** | — |
| `value` | String | **yes** | Value of the field |

### `GrandTotalDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `amount` | f64 | **yes** | Total amount before discounts |
| `currency` | String | **yes** | Currency of the total amount |
| `discountPercentage` | f64 | **yes** | Total discount percentage applied |
| `discounts` | Vec<DiscountDto> | **yes** | List of applied discounts |

### `NotificationSendSettingDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `templateId` | String | **yes** | — |
| `subject` | String | **yes** | — |

### `NotificationSenderSettingDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `fromEmail` | String | **yes** | — |
| `fromName` | String | **yes** | — |

### `NotificationSettingsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `receive` | [`NotificationSendSettingDto`](#notificationsendsettingdto) | **yes** | — |
| `sender` | [`NotificationSenderSettingDto`](#notificationsendersettingdto) | **yes** | — |

### `ProposalEstimateLinksDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `referenceId` | String | **yes** | Reference ID |
| `documentId` | String | **yes** | Document ID |
| `recipientId` | String | **yes** | Recipient ID |
| `entityName` | String — `contacts`, `users` | **yes** | Entity name that the recipient belongs to |
| `recipientCategory` | String — `recipient`, `cc`, `bcc` | **yes** | Recipient category (recipient, cc, or bcc) |
| `documentRevision` | f64 | **yes** | Document revision number |
| `createdBy` | String | **yes** | Created by user ID |
| `deleted` | bool | **yes** | Whether the document is deleted |

### `RecipientItem`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Recipient Id |
| `firstName` | String | no | Recipient First Name |
| `lastName` | String | no | Recipient Last Name |
| `email` | String | **yes** | Recipient Email |
| `phoneNumber` | String | no | Recipient Phone Number |
| `phone` | String | no | Recipient Phone |
| `hasCompleted` | bool | **yes** | Recipient has completed the document |
| `role` | String — `user`, `signer` | **yes** | Recipient role |
| `isPrimary` | bool | **yes** | Recipient is primary |
| `signingOrder` | f64 | **yes** | Recipient signing order |
| `imgUrl` | String | no | Recipient image url |
| `ip` | String | no | Recipient ip |
| `userAgent` | String | no | Recipient user agent |
| `signedDate` | String | no | Recipient signed date |
| `contactName` | String | no | Recipient contact name |
| `country` | String | no | Recipient country |
| `entityName` | String | no | Recipient entity name |
| `initialsImgUrl` | String | no | Recipient initials image url |
| `lastViewedAt` | String | no | Recipient last viewed date |
| `shareLink` | String | no | Share link |

### `SendDocumentDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location Id |
| `documentId` | String | **yes** | Document Id |
| `documentName` | String | no | Document Name |
| `medium` | String — `link`, `email` | no | Medium to be used for sending the document |
| `ccRecipients` | Vec<CCRecipientItem> | no | CC Recipient |
| `notificationSettings` | [`NotificationSettingsDto`](#notificationsettingsdto) | no | — |
| `sentBy` | String | **yes** | Sent ByUser Id |

### `SendDocumentFromPublicApiBodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `templateId` | String | **yes** | Template Id |
| `userId` | String | **yes** | User Id |
| `sendDocument` | bool | no | Send Document |
| `locationId` | String | **yes** | Location Id |
| `contactId` | String | **yes** | Contact Id |
| `opportunityId` | String | no | Opportunity Id |

### `SendDocumentResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status |
| `links` | Vec<ProposalEstimateLinksDto> | **yes** | Links for all recipients |

### `SendTemplateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status |
| `links` | Vec<ProposalEstimateLinksDto> | **yes** | Links for all recipients |

### `TemplateListPaginationResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<TemplateListResponseDTO> | **yes** | Array of templates |
| `total` | f64 | **yes** | Total number of templates |
| `traceId` | String | no | Trace ID for request tracking |

### `TemplateListResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Template ID |
| `deleted` | bool | **yes** | Whether the template is deleted |
| `version` | f64 | **yes** | Template version |
| `name` | String | **yes** | Template name |
| `locationId` | String | **yes** | Location ID |
| `type` | String — `proposal`, `estimate`, `contentLibrary` | **yes** | Template type |
| `updatedBy` | String | **yes** | User ID who last updated the template |
| `isPublicDocument` | bool | **yes** | Whether the template is a public document |
| `createdAt` | String | **yes** | Template creation date |
| `updatedAt` | String | **yes** | Template last update date |
| `id` | String | **yes** | Template ID (alias for _id) |
| `documentCount` | f64 | no | Document count (only present when isPublicDocument is true) |
| `docFormUrl` | String | no | Document form URL (only present when isPublicDocument is true) |

### `UnauthorizedDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | — |
| `message` | String | no | — |
| `error` | String | no | — |

