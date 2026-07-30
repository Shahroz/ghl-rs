# `invoices`

**42** operations / **104** models in API v2 · **42** operations / **104** models in API v3

## How to call it

No hand-written service yet — reach these endpoints two ways:

**From Rust** (typed body via [`ghl-models`](https://docs.rs/ghl-models)):

```rust,ignore
// cargo add ghl-models --features invoices
use ghl_models::v2::invoices::*;

let body = serde_json::to_value(/* a Create…Dto from above */)?;
let out = ghl.request_raw("POST", "/path/", &[], Some(&body), None).await?;
```

**From an AI agent** (MCP meta-tools):

```json
{
  "name": "ghl_search_operations",
  "arguments": {
    "query": "",
    "module": "invoices"
  }
}
```

## Endpoints — API v2

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/invoices/` | List invoices | `invoices.get_invoices` |
| `POST` | `/invoices/` | Create Invoice | `invoices.post_invoices` |
| `POST` | `/invoices/estimate` | Create New Estimate | `invoices.post_invoices_estimate` |
| `GET` | `/invoices/estimate/list` | List Estimates | `invoices.get_invoices_estimate_list` |
| `GET` | `/invoices/estimate/number/generate` | Generate Estimate Number | `invoices.get_invoices_estimate_number_generate` |
| `PATCH` | `/invoices/estimate/stats/last-visited-at` | Update estimate last visited at | `invoices.patch_invoices_estimate_stats_last_visited_at` |
| `GET` | `/invoices/estimate/template` | List Estimate Templates | `invoices.get_invoices_estimate_template` |
| `POST` | `/invoices/estimate/template` | Create Estimate Template | `invoices.post_invoices_estimate_template` |
| `GET` | `/invoices/estimate/template/preview` | Preview Estimate Template | `invoices.get_invoices_estimate_template_preview` |
| `DELETE` | `/invoices/estimate/template/{templateId}` | Delete Estimate Template | `invoices.delete_invoices_estimate_template_by_templateId` |
| `PUT` | `/invoices/estimate/template/{templateId}` | Update Estimate Template | `invoices.put_invoices_estimate_template_by_templateId` |
| `DELETE` | `/invoices/estimate/{estimateId}` | Delete Estimate | `invoices.delete_invoices_estimate_by_estimateId` |
| `PUT` | `/invoices/estimate/{estimateId}` | Update Estimate | `invoices.put_invoices_estimate_by_estimateId` |
| `POST` | `/invoices/estimate/{estimateId}/invoice` | Create Invoice from Estimate | `invoices.post_invoices_estimate_by_estimateId_invoice` |
| `POST` | `/invoices/estimate/{estimateId}/send` | Send Estimate | `invoices.post_invoices_estimate_by_estimateId_send` |
| `GET` | `/invoices/generate-invoice-number` | Generate Invoice Number | `invoices.get_invoices_generate_invoice_number` |
| `GET` | `/invoices/schedule` | List schedules | `invoices.get_invoices_schedule` |
| `POST` | `/invoices/schedule` | Create Invoice Schedule | `invoices.post_invoices_schedule` |
| `DELETE` | `/invoices/schedule/{scheduleId}` | Delete schedule | `invoices.delete_invoices_schedule_by_scheduleId` |
| `GET` | `/invoices/schedule/{scheduleId}` | Get an schedule | `invoices.get_invoices_schedule_by_scheduleId` |
| `PUT` | `/invoices/schedule/{scheduleId}` | Update schedule | `invoices.put_invoices_schedule_by_scheduleId` |
| `POST` | `/invoices/schedule/{scheduleId}/auto-payment` | Manage Auto payment for an schedule invoice | `invoices.post_invoices_schedule_by_scheduleId_auto_payment` |
| `POST` | `/invoices/schedule/{scheduleId}/cancel` | Cancel an scheduled invoice | `invoices.post_invoices_schedule_by_scheduleId_cancel` |
| `POST` | `/invoices/schedule/{scheduleId}/schedule` | Schedule an schedule invoice | `invoices.post_invoices_schedule_by_scheduleId_schedule` |
| `POST` | `/invoices/schedule/{scheduleId}/updateAndSchedule` | Update scheduled recurring invoice | `invoices.post_invoices_schedule_by_scheduleId_updateAndSchedule` |
| `GET` | `/invoices/settings` | Get Invoice Settings | `invoices.get_invoices_settings` |
| `PATCH` | `/invoices/stats/last-visited-at` | Update invoice last visited at | `invoices.patch_invoices_stats_last_visited_at` |
| `GET` | `/invoices/template` | List templates | `invoices.get_invoices_template` |
| `POST` | `/invoices/template` | Create template | `invoices.post_invoices_template` |
| `DELETE` | `/invoices/template/{templateId}` | Delete template | `invoices.delete_invoices_template_by_templateId` |
| `GET` | `/invoices/template/{templateId}` | Get an template | `invoices.get_invoices_template_by_templateId` |
| `PUT` | `/invoices/template/{templateId}` | Update template | `invoices.put_invoices_template_by_templateId` |
| `PATCH` | `/invoices/template/{templateId}/late-fees-configuration` | Update template late fees configuration | `invoices.patch_invoices_template_by_templateId_late_fees_configuration` |
| `PATCH` | `/invoices/template/{templateId}/payment-methods-configuration` | Update template late fees configuration | `invoices.patch_invoices_template_by_templateId_payment_methods_configuration` |
| `POST` | `/invoices/text2pay` | Create & Send | `invoices.post_invoices_text2pay` |
| `DELETE` | `/invoices/{invoiceId}` | Delete invoice | `invoices.delete_invoices_by_invoiceId` |
| `GET` | `/invoices/{invoiceId}` | Get invoice | `invoices.get_invoices_by_invoiceId` |
| `PUT` | `/invoices/{invoiceId}` | Update invoice | `invoices.put_invoices_by_invoiceId` |
| `PATCH` | `/invoices/{invoiceId}/late-fees-configuration` | Update invoice late fees configuration | `invoices.patch_invoices_by_invoiceId_late_fees_configuration` |
| `POST` | `/invoices/{invoiceId}/record-payment` | Record a manual payment for an invoice | `invoices.post_invoices_by_invoiceId_record_payment` |
| `POST` | `/invoices/{invoiceId}/send` | Send invoice | `invoices.post_invoices_by_invoiceId_send` |
| `POST` | `/invoices/{invoiceId}/void` | Void invoice | `invoices.post_invoices_by_invoiceId_void` |

### Endpoint details — v2

#### `GET /invoices/`

**List invoices**

API to get list of invoices

Operation id: `invoices.get_invoices` · `Version: 2021-07-28` · Scopes: `invoices.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | location Id / company Id based on altType |
| `altType` | enum: `location` | **yes** | Alt Type |
| `status` | string | no | status to be filtered |
| `startAt` | string | no | startAt in YYYY-MM-DD format |
| `endAt` | string | no | endAt in YYYY-MM-DD format |
| `search` | string | no | To search for an invoice by id / name / email / phoneNo |
| `paymentMode` | enum: `default`, `live`, `test` | no | payment mode |
| `contactId` | string | no | Contact ID for the invoice |
| `limit` | string | **yes** | Limit the number of items to return |
| `offset` | string | **yes** | Number of items to skip |
| `sortField` | enum: `issueDate` | no | The field on which sorting should be applied |
| `sortOrder` | enum: `ascend`, `descend` | no | The order of sort which should be applied for the sortField |

*Response*: [`ListInvoicesResponseDto`](#listinvoicesresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.get_invoices",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `POST /invoices/`

**Create Invoice**

API to create an invoice

Operation id: `invoices.post_invoices` · `Version: 2021-07-28` · Scopes: `invoices.write`

*Request body*: [`CreateInvoiceDto`](#createinvoicedto)

*Response*: [`CreateInvoiceResponseDto`](#createinvoiceresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.post_invoices",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/estimate`

**Create New Estimate**

Create a new estimate with the provided details

Operation id: `invoices.post_invoices_estimate` · `Version: 2021-07-28` · Scopes: `invoices/estimate.write`

*Request body*: [`CreateEstimatesDto`](#createestimatesdto)

*Response*: [`EstimateResponseDto`](#estimateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.post_invoices_estimate",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /invoices/estimate/list`

**List Estimates**

Get a paginated list of estimates

Operation id: `invoices.get_invoices_estimate_list` · `Version: 2021-07-28` · Scopes: `invoices/estimate.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `startAt` | string | no | startAt in YYYY-MM-DD format |
| `endAt` | string | no | endAt in YYYY-MM-DD format |
| `search` | string | no | search text for estimates name |
| `status` | enum: `all`, `draft`, `sent`, `accepted`, `declined`, `invoiced`, `viewed` | no | estimate status |
| `contactId` | string | no | Contact ID for the estimate |
| `limit` | string | **yes** | Limit the number of items to return |
| `offset` | string | **yes** | Number of items to skip |

*Response*: [`ListEstimatesResponseDTO`](#listestimatesresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.get_invoices_estimate_list",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `GET /invoices/estimate/number/generate`

**Generate Estimate Number**

Get the next estimate number for the given location

Operation id: `invoices.get_invoices_estimate_number_generate` · `Version: 2021-07-28` · Scopes: `invoices/estimate.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`GenerateEstimateNumberResponse`](#generateestimatenumberresponse)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.get_invoices_estimate_number_generate",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `PATCH /invoices/estimate/stats/last-visited-at`

**Update estimate last visited at**

API to update estimate last visited at by estimate id

Operation id: `invoices.patch_invoices_estimate_stats_last_visited_at` · `Version: 2021-07-28`

*Request body*: [`EstimateIdParam`](#estimateidparam)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.patch_invoices_estimate_stats_last_visited_at",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /invoices/estimate/template`

**List Estimate Templates**

Get a list of estimate templates or a specific template by ID

Operation id: `invoices.get_invoices_estimate_template` · `Version: 2021-07-28` · Scopes: `invoices/estimate.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `search` | string | no | To search for an estimate template by id / name |
| `limit` | string | **yes** | Limit the number of items to return |
| `offset` | string | **yes** | Number of items to skip |

*Response*: [`ListEstimateTemplateResponseDTO`](#listestimatetemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.get_invoices_estimate_template",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `POST /invoices/estimate/template`

**Create Estimate Template**

Create a new estimate template

Operation id: `invoices.post_invoices_estimate_template` · `Version: 2021-07-28` · Scopes: `invoices/estimate.write`

*Request body*: [`EstimateTemplatesDto`](#estimatetemplatesdto)

*Response*: [`EstimateTemplateResponseDTO`](#estimatetemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.post_invoices_estimate_template",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /invoices/estimate/template/preview`

**Preview Estimate Template**

Get a preview of an estimate template

Operation id: `invoices.get_invoices_estimate_template_preview` · `Version: 2021-07-28` · Scopes: `invoices/estimate.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `templateId` | string | **yes** | Template Id |

*Response*: [`EstimateTemplateResponseDTO`](#estimatetemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.get_invoices_estimate_template_preview",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>",
      "templateId": "<templateId>"
    }
  }
}
```

</details>

#### `DELETE /invoices/estimate/template/{templateId}`

**Delete Estimate Template**

Delete an existing estimate template

Operation id: `invoices.delete_invoices_estimate_template_by_templateId` · `Version: 2021-07-28` · Scopes: `invoices/estimate.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `templateId` | string | **yes** | Template Id |

*Request body*: [`AltDto`](#altdto)

*Response*: [`EstimateTemplateResponseDTO`](#estimatetemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.delete_invoices_estimate_template_by_templateId",
    "path_params": {
      "templateId": "<templateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /invoices/estimate/template/{templateId}`

**Update Estimate Template**

Update an existing estimate template

Operation id: `invoices.put_invoices_estimate_template_by_templateId` · `Version: 2021-07-28` · Scopes: `invoices/estimate.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `templateId` | string | **yes** | Template Id |

*Request body*: [`EstimateTemplatesDto`](#estimatetemplatesdto)

*Response*: [`EstimateTemplateResponseDTO`](#estimatetemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.put_invoices_estimate_template_by_templateId",
    "path_params": {
      "templateId": "<templateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /invoices/estimate/{estimateId}`

**Delete Estimate**

Delete an existing estimate

Operation id: `invoices.delete_invoices_estimate_by_estimateId` · `Version: 2021-07-28` · Scopes: `invoices/estimate.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `estimateId` | string | **yes** | Estimate Id |

*Request body*: [`AltDto`](#altdto)

*Response*: [`EstimateResponseDto`](#estimateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.delete_invoices_estimate_by_estimateId",
    "path_params": {
      "estimateId": "<estimateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /invoices/estimate/{estimateId}`

**Update Estimate**

Update an existing estimate with new details

Operation id: `invoices.put_invoices_estimate_by_estimateId` · `Version: 2021-07-28` · Scopes: `invoices/estimate.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `estimateId` | string | **yes** | Estimate Id |

*Request body*: [`UpdateEstimateDto`](#updateestimatedto)

*Response*: [`EstimateResponseDto`](#estimateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.put_invoices_estimate_by_estimateId",
    "path_params": {
      "estimateId": "<estimateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/estimate/{estimateId}/invoice`

**Create Invoice from Estimate**

Create a new invoice from an existing estimate

Operation id: `invoices.post_invoices_estimate_by_estimateId_invoice` · `Version: 2021-07-28` · Scopes: `invoices/estimate.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `estimateId` | string | **yes** | Estimate Id |

*Request body*: [`CreateInvoiceFromEstimateDto`](#createinvoicefromestimatedto)

*Response*: [`CreateInvoiceFromEstimateResponseDTO`](#createinvoicefromestimateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.post_invoices_estimate_by_estimateId_invoice",
    "path_params": {
      "estimateId": "<estimateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/estimate/{estimateId}/send`

**Send Estimate**

API to send estimate by estimate id

Operation id: `invoices.post_invoices_estimate_by_estimateId_send` · `Version: 2021-07-28` · Scopes: `invoices/estimate.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `estimateId` | string | **yes** | Estimate Id |

*Request body*: [`SendEstimateDto`](#sendestimatedto)

*Response*: [`EstimateResponseDto`](#estimateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.post_invoices_estimate_by_estimateId_send",
    "path_params": {
      "estimateId": "<estimateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /invoices/generate-invoice-number`

**Generate Invoice Number**

Get the next invoice number for the given location

Operation id: `invoices.get_invoices_generate_invoice_number` · `Version: 2021-07-28` · Scopes: `invoices.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`GenerateInvoiceNumberResponseDto`](#generateinvoicenumberresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.get_invoices_generate_invoice_number",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /invoices/schedule`

**List schedules**

API to get list of schedules

Operation id: `invoices.get_invoices_schedule` · `Version: 2021-07-28` · Scopes: `invoices/schedule.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | location Id / company Id based on altType |
| `altType` | enum: `location` | **yes** | Alt Type |
| `status` | string | no | status to be filtered |
| `startAt` | string | no | startAt in YYYY-MM-DD format |
| `endAt` | string | no | endAt in YYYY-MM-DD format |
| `search` | string | no | To search for an invoice by id / name / email / phoneNo |
| `paymentMode` | enum: `default`, `live`, `test` | no | payment mode |
| `limit` | string | **yes** | Limit the number of items to return |
| `offset` | string | **yes** | Number of items to skip |

*Response*: [`ListSchedulesResponseDto`](#listschedulesresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.get_invoices_schedule",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `POST /invoices/schedule`

**Create Invoice Schedule**

API to create an invoice Schedule

Operation id: `invoices.post_invoices_schedule` · `Version: 2021-07-28` · Scopes: `invoices/schedule.write`

*Request body*: [`CreateInvoiceScheduleDto`](#createinvoicescheduledto)

*Response*: [`CreateInvoiceScheduleResponseDto`](#createinvoicescheduleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.post_invoices_schedule",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /invoices/schedule/{scheduleId}`

**Delete schedule**

API to delete an schedule by schedule id

Operation id: `invoices.delete_invoices_schedule_by_scheduleId` · `Version: 2021-07-28` · Scopes: `invoices/schedule.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `scheduleId` | string | **yes** | Schedule Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | location Id / company Id based on altType |
| `altType` | enum: `location` | **yes** | Alt Type |

*Response*: [`DeleteInvoiceScheduleResponseDto`](#deleteinvoicescheduleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.delete_invoices_schedule_by_scheduleId",
    "path_params": {
      "scheduleId": "<scheduleId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /invoices/schedule/{scheduleId}`

**Get an schedule**

API to get an schedule by schedule id

Operation id: `invoices.get_invoices_schedule_by_scheduleId` · `Version: 2021-07-28` · Scopes: `invoices/schedule.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `scheduleId` | string | **yes** | Schedule Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | location Id / company Id based on altType |
| `altType` | enum: `location` | **yes** | Alt Type |

*Response*: [`GetScheduleResponseDto`](#getscheduleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.get_invoices_schedule_by_scheduleId",
    "path_params": {
      "scheduleId": "<scheduleId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `PUT /invoices/schedule/{scheduleId}`

**Update schedule**

API to update an schedule by schedule id

Operation id: `invoices.put_invoices_schedule_by_scheduleId` · `Version: 2021-07-28` · Scopes: `invoices/schedule.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `scheduleId` | string | **yes** | Schedule Id |

*Request body*: [`UpdateInvoiceScheduleDto`](#updateinvoicescheduledto)

*Response*: [`UpdateInvoiceScheduleResponseDto`](#updateinvoicescheduleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.put_invoices_schedule_by_scheduleId",
    "path_params": {
      "scheduleId": "<scheduleId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/schedule/{scheduleId}/auto-payment`

**Manage Auto payment for an schedule invoice**

API to manage auto payment for a schedule

Operation id: `invoices.post_invoices_schedule_by_scheduleId_auto_payment` · `Version: 2021-07-28` · Scopes: `invoices/schedule.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `scheduleId` | string | **yes** | Schedule Id |

*Request body*: [`AutoPaymentScheduleDto`](#autopaymentscheduledto)

*Response*: [`AutoPaymentInvoiceScheduleResponseDto`](#autopaymentinvoicescheduleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.post_invoices_schedule_by_scheduleId_auto_payment",
    "path_params": {
      "scheduleId": "<scheduleId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/schedule/{scheduleId}/cancel`

**Cancel an scheduled invoice**

API to cancel a scheduled invoice by schedule id

Operation id: `invoices.post_invoices_schedule_by_scheduleId_cancel` · `Version: 2021-07-28` · Scopes: `invoices/schedule.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `scheduleId` | string | **yes** | Schedule Id |

*Request body*: [`CancelInvoiceScheduleDto`](#cancelinvoicescheduledto)

*Response*: [`CancelInvoiceScheduleResponseDto`](#cancelinvoicescheduleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.post_invoices_schedule_by_scheduleId_cancel",
    "path_params": {
      "scheduleId": "<scheduleId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/schedule/{scheduleId}/schedule`

**Schedule an schedule invoice**

API to schedule an schedule invoice to start sending to the customer

Operation id: `invoices.post_invoices_schedule_by_scheduleId_schedule` · `Version: 2021-07-28` · Scopes: `invoices/schedule.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `scheduleId` | string | **yes** | Schedule Id |

*Request body*: [`ScheduleInvoiceScheduleDto`](#scheduleinvoicescheduledto)

*Response*: [`ScheduleInvoiceScheduleResponseDto`](#scheduleinvoicescheduleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.post_invoices_schedule_by_scheduleId_schedule",
    "path_params": {
      "scheduleId": "<scheduleId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/schedule/{scheduleId}/updateAndSchedule`

**Update scheduled recurring invoice**

API to update scheduled recurring invoice

Operation id: `invoices.post_invoices_schedule_by_scheduleId_updateAndSchedule` · `Version: 2021-07-28` · Scopes: `invoices/schedule.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `scheduleId` | string | **yes** | Schedule Id |

*Response*: [`UpdateAndScheduleInvoiceScheduleResponseDto`](#updateandscheduleinvoicescheduleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.post_invoices_schedule_by_scheduleId_updateAndSchedule",
    "path_params": {
      "scheduleId": "<scheduleId>"
    }
  }
}
```

</details>

#### `GET /invoices/settings`

**Get Invoice Settings**

Get the invoice settings for the given location

Operation id: `invoices.get_invoices_settings` · `Version: 2021-07-28` · Scopes: `invoices.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`GetInvoiceSettingsResponseDto`](#getinvoicesettingsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.get_invoices_settings",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `PATCH /invoices/stats/last-visited-at`

**Update invoice last visited at**

API to update invoice last visited at by invoice id

Operation id: `invoices.patch_invoices_stats_last_visited_at` · `Version: 2021-07-28`

*Request body*: [`PatchInvoiceStatsLastViewedDto`](#patchinvoicestatslastvieweddto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.patch_invoices_stats_last_visited_at",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /invoices/template`

**List templates**

API to get list of templates

Operation id: `invoices.get_invoices_template` · `Version: 2021-07-28` · Scopes: `invoices/template.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | location Id / company Id based on altType |
| `altType` | enum: `location` | **yes** | Alt Type |
| `status` | string | no | status to be filtered |
| `startAt` | string | no | startAt in YYYY-MM-DD format |
| `endAt` | string | no | endAt in YYYY-MM-DD format |
| `search` | string | no | To search for an invoice by id / name / email / phoneNo |
| `paymentMode` | enum: `default`, `live`, `test` | no | payment mode |
| `limit` | string | **yes** | Limit the number of items to return |
| `offset` | string | **yes** | Number of items to skip |

*Response*: [`ListTemplatesResponseDto`](#listtemplatesresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.get_invoices_template",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `POST /invoices/template`

**Create template**

API to create a template

Operation id: `invoices.post_invoices_template` · `Version: 2021-07-28` · Scopes: `invoices/template.write`

*Request body*: [`CreateInvoiceTemplateDto`](#createinvoicetemplatedto)

*Response*: [`CreateInvoiceTemplateResponseDto`](#createinvoicetemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.post_invoices_template",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /invoices/template/{templateId}`

**Delete template**

API to update an template by template id

Operation id: `invoices.delete_invoices_template_by_templateId` · `Version: 2021-07-28` · Scopes: `invoices/template.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `templateId` | string | **yes** | Template Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | location Id / company Id based on altType |
| `altType` | enum: `location` | **yes** | Alt Type |

*Response*: [`DeleteInvoiceTemplateResponseDto`](#deleteinvoicetemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.delete_invoices_template_by_templateId",
    "path_params": {
      "templateId": "<templateId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /invoices/template/{templateId}`

**Get an template**

API to get an template by template id

Operation id: `invoices.get_invoices_template_by_templateId` · `Version: 2021-07-28` · Scopes: `invoices/template.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `templateId` | string | **yes** | Template Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | location Id / company Id based on altType |
| `altType` | enum: `location` | **yes** | Alt Type |

*Response*: [`GetTemplateResponseDto`](#gettemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.get_invoices_template_by_templateId",
    "path_params": {
      "templateId": "<templateId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `PUT /invoices/template/{templateId}`

**Update template**

API to update an template by template id

Operation id: `invoices.put_invoices_template_by_templateId` · `Version: 2021-07-28` · Scopes: `invoices/template.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `templateId` | string | **yes** | Template Id |

*Request body*: [`UpdateInvoiceTemplateDto`](#updateinvoicetemplatedto)

*Response*: [`UpdateInvoiceTemplateResponseDto`](#updateinvoicetemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.put_invoices_template_by_templateId",
    "path_params": {
      "templateId": "<templateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PATCH /invoices/template/{templateId}/late-fees-configuration`

**Update template late fees configuration**

API to update template late fees configuration by template id

Operation id: `invoices.patch_invoices_template_by_templateId_late_fees_configuration` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `templateId` | string | **yes** | Template Id |

*Request body*: [`UpdateInvoiceLateFeesConfigurationDto`](#updateinvoicelatefeesconfigurationdto)

*Response*: [`UpdateInvoiceTemplateResponseDto`](#updateinvoicetemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.patch_invoices_template_by_templateId_late_fees_configuration",
    "path_params": {
      "templateId": "<templateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PATCH /invoices/template/{templateId}/payment-methods-configuration`

**Update template late fees configuration**

API to update template late fees configuration by template id

Operation id: `invoices.patch_invoices_template_by_templateId_payment_methods_configuration` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `templateId` | string | **yes** | Template Id |

*Request body*: [`UpdatePaymentMethodsConfigurationDto`](#updatepaymentmethodsconfigurationdto)

*Response*: [`UpdateInvoiceTemplateResponseDto`](#updateinvoicetemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.patch_invoices_template_by_templateId_payment_methods_configuration",
    "path_params": {
      "templateId": "<templateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/text2pay`

**Create & Send**

API to create or update a text2pay invoice

Operation id: `invoices.post_invoices_text2pay` · `Version: 2021-07-28` · Scopes: `invoices.write`

*Request body*: [`Text2PayDto`](#text2paydto)

*Response*: [`Text2PayInvoiceResponseDto`](#text2payinvoiceresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.post_invoices_text2pay",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /invoices/{invoiceId}`

**Delete invoice**

API to delete invoice by invoice id

Operation id: `invoices.delete_invoices_by_invoiceId` · `Version: 2021-07-28` · Scopes: `invoices.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `invoiceId` | string | **yes** | Invoice Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | location Id / company Id based on altType |
| `altType` | enum: `location` | **yes** | Alt Type |

*Response*: [`DeleteInvoiceResponseDto`](#deleteinvoiceresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.delete_invoices_by_invoiceId",
    "path_params": {
      "invoiceId": "<invoiceId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /invoices/{invoiceId}`

**Get invoice**

API to get invoice by invoice id

Operation id: `invoices.get_invoices_by_invoiceId` · `Version: 2021-07-28` · Scopes: `invoices.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `invoiceId` | string | **yes** | Invoice Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | location Id / company Id based on altType |
| `altType` | enum: `location` | **yes** | Alt Type |

*Response*: [`GetInvoiceResponseDto`](#getinvoiceresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.get_invoices_by_invoiceId",
    "path_params": {
      "invoiceId": "<invoiceId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `PUT /invoices/{invoiceId}`

**Update invoice**

API to update invoice by invoice id

Operation id: `invoices.put_invoices_by_invoiceId` · `Version: 2021-07-28` · Scopes: `invoices.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `invoiceId` | string | **yes** | Invoice Id |

*Request body*: [`UpdateInvoiceDto`](#updateinvoicedto)

*Response*: [`UpdateInvoiceResponseDto`](#updateinvoiceresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.put_invoices_by_invoiceId",
    "path_params": {
      "invoiceId": "<invoiceId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PATCH /invoices/{invoiceId}/late-fees-configuration`

**Update invoice late fees configuration**

API to update invoice late fees configuration by invoice id

Operation id: `invoices.patch_invoices_by_invoiceId_late_fees_configuration` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `invoiceId` | string | **yes** | Invoice Id |

*Request body*: [`UpdateInvoiceLateFeesConfigurationDto`](#updateinvoicelatefeesconfigurationdto)

*Response*: [`UpdateInvoiceResponseDto`](#updateinvoiceresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.patch_invoices_by_invoiceId_late_fees_configuration",
    "path_params": {
      "invoiceId": "<invoiceId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/{invoiceId}/record-payment`

**Record a manual payment for an invoice**

API to record manual payment for an invoice by invoice id

Operation id: `invoices.post_invoices_by_invoiceId_record_payment` · `Version: 2021-07-28` · Scopes: `invoices.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `invoiceId` | string | **yes** | Invoice Id |

*Request body*: [`RecordPaymentDto`](#recordpaymentdto)

*Response*: [`RecordPaymentResponseDto`](#recordpaymentresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.post_invoices_by_invoiceId_record_payment",
    "path_params": {
      "invoiceId": "<invoiceId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/{invoiceId}/send`

**Send invoice**

API to send invoice by invoice id

Operation id: `invoices.post_invoices_by_invoiceId_send` · `Version: 2021-07-28` · Scopes: `invoices.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `invoiceId` | string | **yes** | Invoice Id |

*Request body*: [`SendInvoiceDto`](#sendinvoicedto)

*Response*: [`SendInvoicesResponseDto`](#sendinvoicesresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.post_invoices_by_invoiceId_send",
    "path_params": {
      "invoiceId": "<invoiceId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/{invoiceId}/void`

**Void invoice**

API to delete invoice by invoice id

Operation id: `invoices.post_invoices_by_invoiceId_void` · `Version: 2021-07-28` · Scopes: `invoices.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `invoiceId` | string | **yes** | Invoice Id |

*Request body*: [`VoidInvoiceDto`](#voidinvoicedto)

*Response*: [`VoidInvoiceResponseDto`](#voidinvoiceresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "invoices.post_invoices_by_invoiceId_void",
    "path_params": {
      "invoiceId": "<invoiceId>"
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
| `GET` | `/invoices/` | List invoices | `v3:invoices.get_invoices` |
| `POST` | `/invoices/` | Create Invoice | `v3:invoices.post_invoices` |
| `POST` | `/invoices/estimate` | Create New Estimate | `v3:invoices.post_invoices_estimate` |
| `GET` | `/invoices/estimate/list` | List Estimates | `v3:invoices.get_invoices_estimate_list` |
| `GET` | `/invoices/estimate/number/generate` | Generate Estimate Number | `v3:invoices.get_invoices_estimate_number_generate` |
| `PATCH` | `/invoices/estimate/stats/last-visited-at` | Update estimate last visited at | `v3:invoices.patch_invoices_estimate_stats_last_visited_at` |
| `GET` | `/invoices/estimate/template` | List Estimate Templates | `v3:invoices.get_invoices_estimate_template` |
| `POST` | `/invoices/estimate/template` | Create Estimate Template | `v3:invoices.post_invoices_estimate_template` |
| `GET` | `/invoices/estimate/template/preview` | Preview Estimate Template | `v3:invoices.get_invoices_estimate_template_preview` |
| `DELETE` | `/invoices/estimate/template/{templateId}` | Delete Estimate Template | `v3:invoices.delete_invoices_estimate_template_by_templateId` |
| `PUT` | `/invoices/estimate/template/{templateId}` | Update Estimate Template | `v3:invoices.put_invoices_estimate_template_by_templateId` |
| `DELETE` | `/invoices/estimate/{estimateId}` | Delete Estimate | `v3:invoices.delete_invoices_estimate_by_estimateId` |
| `PUT` | `/invoices/estimate/{estimateId}` | Update Estimate | `v3:invoices.put_invoices_estimate_by_estimateId` |
| `POST` | `/invoices/estimate/{estimateId}/invoice` | Create Invoice from Estimate | `v3:invoices.post_invoices_estimate_by_estimateId_invoice` |
| `POST` | `/invoices/estimate/{estimateId}/send` | Send Estimate | `v3:invoices.post_invoices_estimate_by_estimateId_send` |
| `GET` | `/invoices/generate-invoice-number` | Generate Invoice Number | `v3:invoices.get_invoices_generate_invoice_number` |
| `GET` | `/invoices/schedule` | List schedules | `v3:invoices.get_invoices_schedule` |
| `POST` | `/invoices/schedule` | Create Invoice Schedule | `v3:invoices.post_invoices_schedule` |
| `DELETE` | `/invoices/schedule/{scheduleId}` | Delete schedule | `v3:invoices.delete_invoices_schedule_by_scheduleId` |
| `GET` | `/invoices/schedule/{scheduleId}` | Get an schedule | `v3:invoices.get_invoices_schedule_by_scheduleId` |
| `PUT` | `/invoices/schedule/{scheduleId}` | Update schedule | `v3:invoices.put_invoices_schedule_by_scheduleId` |
| `POST` | `/invoices/schedule/{scheduleId}/auto-payment` | Manage Auto payment for an schedule invoice | `v3:invoices.post_invoices_schedule_by_scheduleId_auto_payment` |
| `POST` | `/invoices/schedule/{scheduleId}/cancel` | Cancel an scheduled invoice | `v3:invoices.post_invoices_schedule_by_scheduleId_cancel` |
| `POST` | `/invoices/schedule/{scheduleId}/schedule` | Schedule an schedule invoice | `v3:invoices.post_invoices_schedule_by_scheduleId_schedule` |
| `POST` | `/invoices/schedule/{scheduleId}/updateAndSchedule` | Update scheduled recurring invoice | `v3:invoices.post_invoices_schedule_by_scheduleId_updateAndSchedule` |
| `GET` | `/invoices/settings` | Get Invoice Settings | `v3:invoices.get_invoices_settings` |
| `PATCH` | `/invoices/stats/last-visited-at` | Update invoice last visited at | `v3:invoices.patch_invoices_stats_last_visited_at` |
| `GET` | `/invoices/template` | List templates | `v3:invoices.get_invoices_template` |
| `POST` | `/invoices/template` | Create template | `v3:invoices.post_invoices_template` |
| `DELETE` | `/invoices/template/{templateId}` | Delete template | `v3:invoices.delete_invoices_template_by_templateId` |
| `GET` | `/invoices/template/{templateId}` | Get an template | `v3:invoices.get_invoices_template_by_templateId` |
| `PUT` | `/invoices/template/{templateId}` | Update template | `v3:invoices.put_invoices_template_by_templateId` |
| `PATCH` | `/invoices/template/{templateId}/late-fees-configuration` | Update template late fees configuration | `v3:invoices.patch_invoices_template_by_templateId_late_fees_configuration` |
| `PATCH` | `/invoices/template/{templateId}/payment-methods-configuration` | Update template late fees configuration | `v3:invoices.patch_invoices_template_by_templateId_payment_methods_configuration` |
| `POST` | `/invoices/text2pay` | Create & Send | `v3:invoices.post_invoices_text2pay` |
| `DELETE` | `/invoices/{invoiceId}` | Delete invoice | `v3:invoices.delete_invoices_by_invoiceId` |
| `GET` | `/invoices/{invoiceId}` | Get invoice | `v3:invoices.get_invoices_by_invoiceId` |
| `PUT` | `/invoices/{invoiceId}` | Update invoice | `v3:invoices.put_invoices_by_invoiceId` |
| `PATCH` | `/invoices/{invoiceId}/late-fees-configuration` | Update invoice late fees configuration | `v3:invoices.patch_invoices_by_invoiceId_late_fees_configuration` |
| `POST` | `/invoices/{invoiceId}/record-payment` | Record a manual payment for an invoice | `v3:invoices.post_invoices_by_invoiceId_record_payment` |
| `POST` | `/invoices/{invoiceId}/send` | Send invoice | `v3:invoices.post_invoices_by_invoiceId_send` |
| `POST` | `/invoices/{invoiceId}/void` | Void invoice | `v3:invoices.post_invoices_by_invoiceId_void` |

### Endpoint details — v3

#### `GET /invoices/`

**List invoices**

API to get list of invoices

Operation id: `v3:invoices.get_invoices` · `Version: v3` · Scopes: `invoices.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | location Id / company Id based on altType |
| `altType` | enum: `location` | **yes** | Alt Type |
| `status` | string | no | status to be filtered |
| `startAt` | string | no | startAt in YYYY-MM-DD format |
| `endAt` | string | no | endAt in YYYY-MM-DD format |
| `search` | string | no | To search for an invoice by id / name / email / phoneNo |
| `paymentMode` | enum: `default`, `live`, `test` | no | payment mode |
| `contactId` | string | no | Contact ID for the invoice |
| `limit` | string | **yes** | Limit the number of items to return |
| `offset` | string | **yes** | Number of items to skip |
| `sortField` | enum: `issueDate` | no | The field on which sorting should be applied |
| `sortOrder` | enum: `ascend`, `descend` | no | The order of sort which should be applied for the sortField |

*Response*: [`ListInvoicesResponseDto`](#listinvoicesresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.get_invoices",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `POST /invoices/`

**Create Invoice**

API to create an invoice

Operation id: `v3:invoices.post_invoices` · `Version: v3` · Scopes: `invoices.write`

*Request body*: [`CreateInvoiceDto`](#createinvoicedto)

*Response*: [`CreateInvoiceResponseDto`](#createinvoiceresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.post_invoices",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/estimate`

**Create New Estimate**

Create a new estimate with the provided details

Operation id: `v3:invoices.post_invoices_estimate` · `Version: v3` · Scopes: `invoices/estimate.write`

*Request body*: [`CreateEstimatesDto`](#createestimatesdto)

*Response*: [`EstimateResponseDto`](#estimateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.post_invoices_estimate",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /invoices/estimate/list`

**List Estimates**

Get a paginated list of estimates

Operation id: `v3:invoices.get_invoices_estimate_list` · `Version: v3` · Scopes: `invoices/estimate.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `startAt` | string | no | startAt in YYYY-MM-DD format |
| `endAt` | string | no | endAt in YYYY-MM-DD format |
| `search` | string | no | search text for estimates name |
| `status` | enum: `all`, `draft`, `sent`, `accepted`, `declined`, `invoiced`, `viewed` | no | estimate status |
| `contactId` | string | no | Contact ID for the estimate |
| `limit` | string | **yes** | Limit the number of items to return |
| `offset` | string | **yes** | Number of items to skip |

*Response*: [`ListEstimatesResponseDTO`](#listestimatesresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.get_invoices_estimate_list",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `GET /invoices/estimate/number/generate`

**Generate Estimate Number**

Get the next estimate number for the given location

Operation id: `v3:invoices.get_invoices_estimate_number_generate` · `Version: v3` · Scopes: `invoices/estimate.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`GenerateEstimateNumberResponse`](#generateestimatenumberresponse)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.get_invoices_estimate_number_generate",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `PATCH /invoices/estimate/stats/last-visited-at`

**Update estimate last visited at**

API to update estimate last visited at by estimate id

Operation id: `v3:invoices.patch_invoices_estimate_stats_last_visited_at` · `Version: v3`

*Request body*: [`EstimateIdParam`](#estimateidparam)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.patch_invoices_estimate_stats_last_visited_at",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /invoices/estimate/template`

**List Estimate Templates**

Get a list of estimate templates or a specific template by ID

Operation id: `v3:invoices.get_invoices_estimate_template` · `Version: v3` · Scopes: `invoices/estimate.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `search` | string | no | To search for an estimate template by id / name |
| `limit` | string | **yes** | Limit the number of items to return |
| `offset` | string | **yes** | Number of items to skip |

*Response*: [`ListEstimateTemplateResponseDTO`](#listestimatetemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.get_invoices_estimate_template",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `POST /invoices/estimate/template`

**Create Estimate Template**

Create a new estimate template

Operation id: `v3:invoices.post_invoices_estimate_template` · `Version: v3` · Scopes: `invoices/estimate.write`

*Request body*: [`EstimateTemplatesDto`](#estimatetemplatesdto)

*Response*: [`EstimateTemplateResponseDTO`](#estimatetemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.post_invoices_estimate_template",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /invoices/estimate/template/preview`

**Preview Estimate Template**

Get a preview of an estimate template

Operation id: `v3:invoices.get_invoices_estimate_template_preview` · `Version: v3` · Scopes: `invoices/estimate.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `templateId` | string | **yes** | Template Id |

*Response*: [`EstimateTemplateResponseDTO`](#estimatetemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.get_invoices_estimate_template_preview",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>",
      "templateId": "<templateId>"
    }
  }
}
```

</details>

#### `DELETE /invoices/estimate/template/{templateId}`

**Delete Estimate Template**

Delete an existing estimate template

Operation id: `v3:invoices.delete_invoices_estimate_template_by_templateId` · `Version: v3` · Scopes: `invoices/estimate.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `templateId` | string | **yes** | Template Id |

*Request body*: [`AltDto`](#altdto)

*Response*: [`EstimateTemplateResponseDTO`](#estimatetemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.delete_invoices_estimate_template_by_templateId",
    "path_params": {
      "templateId": "<templateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /invoices/estimate/template/{templateId}`

**Update Estimate Template**

Update an existing estimate template

Operation id: `v3:invoices.put_invoices_estimate_template_by_templateId` · `Version: v3` · Scopes: `invoices/estimate.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `templateId` | string | **yes** | Template Id |

*Request body*: [`EstimateTemplatesDto`](#estimatetemplatesdto)

*Response*: [`EstimateTemplateResponseDTO`](#estimatetemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.put_invoices_estimate_template_by_templateId",
    "path_params": {
      "templateId": "<templateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /invoices/estimate/{estimateId}`

**Delete Estimate**

Delete an existing estimate

Operation id: `v3:invoices.delete_invoices_estimate_by_estimateId` · `Version: v3` · Scopes: `invoices/estimate.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `estimateId` | string | **yes** | Estimate Id |

*Request body*: [`AltDto`](#altdto)

*Response*: [`EstimateResponseDto`](#estimateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.delete_invoices_estimate_by_estimateId",
    "path_params": {
      "estimateId": "<estimateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /invoices/estimate/{estimateId}`

**Update Estimate**

Update an existing estimate with new details

Operation id: `v3:invoices.put_invoices_estimate_by_estimateId` · `Version: v3` · Scopes: `invoices/estimate.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `estimateId` | string | **yes** | Estimate Id |

*Request body*: [`UpdateEstimateDto`](#updateestimatedto)

*Response*: [`EstimateResponseDto`](#estimateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.put_invoices_estimate_by_estimateId",
    "path_params": {
      "estimateId": "<estimateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/estimate/{estimateId}/invoice`

**Create Invoice from Estimate**

Create a new invoice from an existing estimate

Operation id: `v3:invoices.post_invoices_estimate_by_estimateId_invoice` · `Version: v3` · Scopes: `invoices/estimate.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `estimateId` | string | **yes** | Estimate Id |

*Request body*: [`CreateInvoiceFromEstimateDto`](#createinvoicefromestimatedto)

*Response*: [`CreateInvoiceFromEstimateResponseDTO`](#createinvoicefromestimateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.post_invoices_estimate_by_estimateId_invoice",
    "path_params": {
      "estimateId": "<estimateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/estimate/{estimateId}/send`

**Send Estimate**

API to send estimate by estimate id

Operation id: `v3:invoices.post_invoices_estimate_by_estimateId_send` · `Version: v3` · Scopes: `invoices/estimate.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `estimateId` | string | **yes** | Estimate Id |

*Request body*: [`SendEstimateDto`](#sendestimatedto)

*Response*: [`EstimateResponseDto`](#estimateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.post_invoices_estimate_by_estimateId_send",
    "path_params": {
      "estimateId": "<estimateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /invoices/generate-invoice-number`

**Generate Invoice Number**

Get the next invoice number for the given location

Operation id: `v3:invoices.get_invoices_generate_invoice_number` · `Version: v3` · Scopes: `invoices.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`GenerateInvoiceNumberResponseDto`](#generateinvoicenumberresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.get_invoices_generate_invoice_number",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /invoices/schedule`

**List schedules**

API to get list of schedules

Operation id: `v3:invoices.get_invoices_schedule` · `Version: v3` · Scopes: `invoices/schedule.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | location Id / company Id based on altType |
| `altType` | enum: `location` | **yes** | Alt Type |
| `status` | string | no | status to be filtered |
| `startAt` | string | no | startAt in YYYY-MM-DD format |
| `endAt` | string | no | endAt in YYYY-MM-DD format |
| `search` | string | no | To search for an invoice by id / name / email / phoneNo |
| `paymentMode` | enum: `default`, `live`, `test` | no | payment mode |
| `limit` | string | **yes** | Limit the number of items to return |
| `offset` | string | **yes** | Number of items to skip |

*Response*: [`ListSchedulesResponseDto`](#listschedulesresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.get_invoices_schedule",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `POST /invoices/schedule`

**Create Invoice Schedule**

API to create an invoice Schedule

Operation id: `v3:invoices.post_invoices_schedule` · `Version: v3` · Scopes: `invoices/schedule.write`

*Request body*: [`CreateInvoiceScheduleDto`](#createinvoicescheduledto)

*Response*: [`CreateInvoiceScheduleResponseDto`](#createinvoicescheduleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.post_invoices_schedule",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /invoices/schedule/{scheduleId}`

**Delete schedule**

API to delete an schedule by schedule id

Operation id: `v3:invoices.delete_invoices_schedule_by_scheduleId` · `Version: v3` · Scopes: `invoices/schedule.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `scheduleId` | string | **yes** | Schedule Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | location Id / company Id based on altType |
| `altType` | enum: `location` | **yes** | Alt Type |

*Response*: [`DeleteInvoiceScheduleResponseDto`](#deleteinvoicescheduleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.delete_invoices_schedule_by_scheduleId",
    "path_params": {
      "scheduleId": "<scheduleId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /invoices/schedule/{scheduleId}`

**Get an schedule**

API to get an schedule by schedule id

Operation id: `v3:invoices.get_invoices_schedule_by_scheduleId` · `Version: v3` · Scopes: `invoices/schedule.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `scheduleId` | string | **yes** | Schedule Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | location Id / company Id based on altType |
| `altType` | enum: `location` | **yes** | Alt Type |

*Response*: [`GetScheduleResponseDto`](#getscheduleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.get_invoices_schedule_by_scheduleId",
    "path_params": {
      "scheduleId": "<scheduleId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `PUT /invoices/schedule/{scheduleId}`

**Update schedule**

API to update an schedule by schedule id

Operation id: `v3:invoices.put_invoices_schedule_by_scheduleId` · `Version: v3` · Scopes: `invoices/schedule.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `scheduleId` | string | **yes** | Schedule Id |

*Request body*: [`UpdateInvoiceScheduleDto`](#updateinvoicescheduledto)

*Response*: [`UpdateInvoiceScheduleResponseDto`](#updateinvoicescheduleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.put_invoices_schedule_by_scheduleId",
    "path_params": {
      "scheduleId": "<scheduleId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/schedule/{scheduleId}/auto-payment`

**Manage Auto payment for an schedule invoice**

API to manage auto payment for a schedule

Operation id: `v3:invoices.post_invoices_schedule_by_scheduleId_auto_payment` · `Version: v3` · Scopes: `invoices/schedule.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `scheduleId` | string | **yes** | Schedule Id |

*Request body*: [`AutoPaymentScheduleDto`](#autopaymentscheduledto)

*Response*: [`AutoPaymentInvoiceScheduleResponseDto`](#autopaymentinvoicescheduleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.post_invoices_schedule_by_scheduleId_auto_payment",
    "path_params": {
      "scheduleId": "<scheduleId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/schedule/{scheduleId}/cancel`

**Cancel an scheduled invoice**

API to cancel a scheduled invoice by schedule id

Operation id: `v3:invoices.post_invoices_schedule_by_scheduleId_cancel` · `Version: v3` · Scopes: `invoices/schedule.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `scheduleId` | string | **yes** | Schedule Id |

*Request body*: [`CancelInvoiceScheduleDto`](#cancelinvoicescheduledto)

*Response*: [`CancelInvoiceScheduleResponseDto`](#cancelinvoicescheduleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.post_invoices_schedule_by_scheduleId_cancel",
    "path_params": {
      "scheduleId": "<scheduleId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/schedule/{scheduleId}/schedule`

**Schedule an schedule invoice**

API to schedule an schedule invoice to start sending to the customer

Operation id: `v3:invoices.post_invoices_schedule_by_scheduleId_schedule` · `Version: v3` · Scopes: `invoices/schedule.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `scheduleId` | string | **yes** | Schedule Id |

*Request body*: [`ScheduleInvoiceScheduleDto`](#scheduleinvoicescheduledto)

*Response*: [`ScheduleInvoiceScheduleResponseDto`](#scheduleinvoicescheduleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.post_invoices_schedule_by_scheduleId_schedule",
    "path_params": {
      "scheduleId": "<scheduleId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/schedule/{scheduleId}/updateAndSchedule`

**Update scheduled recurring invoice**

API to update scheduled recurring invoice

Operation id: `v3:invoices.post_invoices_schedule_by_scheduleId_updateAndSchedule` · `Version: v3` · Scopes: `invoices/schedule.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `scheduleId` | string | **yes** | Schedule Id |

*Response*: [`UpdateAndScheduleInvoiceScheduleResponseDto`](#updateandscheduleinvoicescheduleresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.post_invoices_schedule_by_scheduleId_updateAndSchedule",
    "path_params": {
      "scheduleId": "<scheduleId>"
    }
  }
}
```

</details>

#### `GET /invoices/settings`

**Get Invoice Settings**

Get the invoice settings for the given location

Operation id: `v3:invoices.get_invoices_settings` · `Version: v3` · Scopes: `invoices.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`GetInvoiceSettingsResponseDto`](#getinvoicesettingsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.get_invoices_settings",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `PATCH /invoices/stats/last-visited-at`

**Update invoice last visited at**

API to update invoice last visited at by invoice id

Operation id: `v3:invoices.patch_invoices_stats_last_visited_at` · `Version: v3`

*Request body*: [`PatchInvoiceStatsLastViewedDto`](#patchinvoicestatslastvieweddto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.patch_invoices_stats_last_visited_at",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /invoices/template`

**List templates**

API to get list of templates

Operation id: `v3:invoices.get_invoices_template` · `Version: v3` · Scopes: `invoices/template.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | location Id / company Id based on altType |
| `altType` | enum: `location` | **yes** | Alt Type |
| `status` | string | no | status to be filtered |
| `startAt` | string | no | startAt in YYYY-MM-DD format |
| `endAt` | string | no | endAt in YYYY-MM-DD format |
| `search` | string | no | To search for an invoice by id / name / email / phoneNo |
| `paymentMode` | enum: `default`, `live`, `test` | no | payment mode |
| `limit` | string | **yes** | Limit the number of items to return |
| `offset` | string | **yes** | Number of items to skip |

*Response*: [`ListTemplatesResponseDto`](#listtemplatesresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.get_invoices_template",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `POST /invoices/template`

**Create template**

API to create a template

Operation id: `v3:invoices.post_invoices_template` · `Version: v3` · Scopes: `invoices/template.write`

*Request body*: [`CreateInvoiceTemplateDto`](#createinvoicetemplatedto)

*Response*: [`CreateInvoiceTemplateResponseDto`](#createinvoicetemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.post_invoices_template",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /invoices/template/{templateId}`

**Delete template**

API to update an template by template id

Operation id: `v3:invoices.delete_invoices_template_by_templateId` · `Version: v3` · Scopes: `invoices/template.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `templateId` | string | **yes** | Template Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | location Id / company Id based on altType |
| `altType` | enum: `location` | **yes** | Alt Type |

*Response*: [`DeleteInvoiceTemplateResponseDto`](#deleteinvoicetemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.delete_invoices_template_by_templateId",
    "path_params": {
      "templateId": "<templateId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /invoices/template/{templateId}`

**Get an template**

API to get an template by template id

Operation id: `v3:invoices.get_invoices_template_by_templateId` · `Version: v3` · Scopes: `invoices/template.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `templateId` | string | **yes** | Template Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | location Id / company Id based on altType |
| `altType` | enum: `location` | **yes** | Alt Type |

*Response*: [`GetTemplateResponseDto`](#gettemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.get_invoices_template_by_templateId",
    "path_params": {
      "templateId": "<templateId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `PUT /invoices/template/{templateId}`

**Update template**

API to update an template by template id

Operation id: `v3:invoices.put_invoices_template_by_templateId` · `Version: v3` · Scopes: `invoices/template.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `templateId` | string | **yes** | Template Id |

*Request body*: [`UpdateInvoiceTemplateDto`](#updateinvoicetemplatedto)

*Response*: [`UpdateInvoiceTemplateResponseDto`](#updateinvoicetemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.put_invoices_template_by_templateId",
    "path_params": {
      "templateId": "<templateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PATCH /invoices/template/{templateId}/late-fees-configuration`

**Update template late fees configuration**

API to update template late fees configuration by template id

Operation id: `v3:invoices.patch_invoices_template_by_templateId_late_fees_configuration` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `templateId` | string | **yes** | Template Id |

*Request body*: [`UpdateInvoiceLateFeesConfigurationDto`](#updateinvoicelatefeesconfigurationdto)

*Response*: [`UpdateInvoiceTemplateResponseDto`](#updateinvoicetemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.patch_invoices_template_by_templateId_late_fees_configuration",
    "path_params": {
      "templateId": "<templateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PATCH /invoices/template/{templateId}/payment-methods-configuration`

**Update template late fees configuration**

API to update template late fees configuration by template id

Operation id: `v3:invoices.patch_invoices_template_by_templateId_payment_methods_configuration` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `templateId` | string | **yes** | Template Id |

*Request body*: [`UpdatePaymentMethodsConfigurationDto`](#updatepaymentmethodsconfigurationdto)

*Response*: [`UpdateInvoiceTemplateResponseDto`](#updateinvoicetemplateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.patch_invoices_template_by_templateId_payment_methods_configuration",
    "path_params": {
      "templateId": "<templateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/text2pay`

**Create & Send**

API to create or update a text2pay invoice

Operation id: `v3:invoices.post_invoices_text2pay` · `Version: v3` · Scopes: `invoices.write`

*Request body*: [`Text2PayDto`](#text2paydto)

*Response*: [`Text2PayInvoiceResponseDto`](#text2payinvoiceresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.post_invoices_text2pay",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /invoices/{invoiceId}`

**Delete invoice**

API to delete invoice by invoice id

Operation id: `v3:invoices.delete_invoices_by_invoiceId` · `Version: v3` · Scopes: `invoices.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `invoiceId` | string | **yes** | Invoice Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | location Id / company Id based on altType |
| `altType` | enum: `location` | **yes** | Alt Type |

*Response*: [`DeleteInvoiceResponseDto`](#deleteinvoiceresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.delete_invoices_by_invoiceId",
    "path_params": {
      "invoiceId": "<invoiceId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /invoices/{invoiceId}`

**Get invoice**

API to get invoice by invoice id

Operation id: `v3:invoices.get_invoices_by_invoiceId` · `Version: v3` · Scopes: `invoices.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `invoiceId` | string | **yes** | Invoice Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | location Id / company Id based on altType |
| `altType` | enum: `location` | **yes** | Alt Type |

*Response*: [`GetInvoiceResponseDto`](#getinvoiceresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.get_invoices_by_invoiceId",
    "path_params": {
      "invoiceId": "<invoiceId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `PUT /invoices/{invoiceId}`

**Update invoice**

API to update invoice by invoice id

Operation id: `v3:invoices.put_invoices_by_invoiceId` · `Version: v3` · Scopes: `invoices.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `invoiceId` | string | **yes** | Invoice Id |

*Request body*: [`UpdateInvoiceDto`](#updateinvoicedto)

*Response*: [`UpdateInvoiceResponseDto`](#updateinvoiceresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.put_invoices_by_invoiceId",
    "path_params": {
      "invoiceId": "<invoiceId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PATCH /invoices/{invoiceId}/late-fees-configuration`

**Update invoice late fees configuration**

API to update invoice late fees configuration by invoice id

Operation id: `v3:invoices.patch_invoices_by_invoiceId_late_fees_configuration` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `invoiceId` | string | **yes** | Invoice Id |

*Request body*: [`UpdateInvoiceLateFeesConfigurationDto`](#updateinvoicelatefeesconfigurationdto)

*Response*: [`UpdateInvoiceResponseDto`](#updateinvoiceresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.patch_invoices_by_invoiceId_late_fees_configuration",
    "path_params": {
      "invoiceId": "<invoiceId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/{invoiceId}/record-payment`

**Record a manual payment for an invoice**

API to record manual payment for an invoice by invoice id

Operation id: `v3:invoices.post_invoices_by_invoiceId_record_payment` · `Version: v3` · Scopes: `invoices.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `invoiceId` | string | **yes** | Invoice Id |

*Request body*: [`RecordPaymentDto`](#recordpaymentdto)

*Response*: [`RecordPaymentResponseDto`](#recordpaymentresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.post_invoices_by_invoiceId_record_payment",
    "path_params": {
      "invoiceId": "<invoiceId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/{invoiceId}/send`

**Send invoice**

API to send invoice by invoice id

Operation id: `v3:invoices.post_invoices_by_invoiceId_send` · `Version: v3` · Scopes: `invoices.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `invoiceId` | string | **yes** | Invoice Id |

*Request body*: [`SendInvoiceDto`](#sendinvoicedto)

*Response*: [`SendInvoicesResponseDto`](#sendinvoicesresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.post_invoices_by_invoiceId_send",
    "path_params": {
      "invoiceId": "<invoiceId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /invoices/{invoiceId}/void`

**Void invoice**

API to delete invoice by invoice id

Operation id: `v3:invoices.post_invoices_by_invoiceId_void` · `Version: v3` · Scopes: `invoices.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `invoiceId` | string | **yes** | Invoice Id |

*Request body*: [`VoidInvoiceDto`](#voidinvoicedto)

*Response*: [`VoidInvoiceResponseDto`](#voidinvoiceresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:invoices.post_invoices_by_invoiceId_void",
    "path_params": {
      "invoiceId": "<invoiceId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::invoices::*` (enable the `invoices` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/invoices/).

### `AdditionalEmailsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `email` | String | **yes** | — |

### `Address`

| Field | Type | Required | Description |
|---|---|---|---|
| `addressLine1` | String | no | — |
| `addressLine2` | String | no | — |
| `city` | String | no | — |
| `state` | String | no | — |
| `countryCode` | String — 247 values ([shared](shared-enums.md)) | no | — |
| `postalCode` | String | no | — |

### `AddressDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `addressLine1` | String | no | Address Line 1 |
| `addressLine2` | String | no | Address Line 2 |
| `city` | String | no | City |
| `state` | String | no | State |
| `countryCode` | String | no | Country Code |
| `postalCode` | String | no | Postal Code |

### `AltDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |

### `AttachmentsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Id of the file selected |
| `name` | String | **yes** | Name of the file |
| `url` | String | **yes** | URL of the file |
| `type` | String | **yes** | Type of the file |
| `size` | f64 | **yes** | Size of the file |

### `AutoInvoice`

_No fields defined in the spec._

### `AutoInvoicingDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | Enable Auto Invoice |
| `directPayments` | bool | no | Direct Payments |

### `AutoPaymentDetailsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enable` | bool | **yes** | — |
| `type` | String | no | — |
| `paymentMethodId` | String | no | — |
| `customerId` | String | no | — |
| `card` | [`CardDto`](#carddto) | no | — |
| `usBankAccount` | [`USBankAccountDto`](#usbankaccountdto) | no | — |
| `sepaDirectDebit` | [`SepaDirectDebitDTO`](#sepadirectdebitdto) | no | — |
| `bacsDirectDebit` | [`BacsDirectDebitDTO`](#bacsdirectdebitdto) | no | — |
| `becsDirectDebit` | [`BecsDirectDebitDTO`](#becsdirectdebitdto) | no | — |
| `cardId` | String | no | — |
| `provider` | JSON | no | — |

### `AutoPaymentInvoiceScheduleResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Schedule Id |
| `status` | JSON | **yes** | Schedule Status |
| `liveMode` | bool | **yes** | Live Mode |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | no | — |
| `invoices` | Vec<DefaultInvoiceResponseDto> | **yes** | List of invoices |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | Business Details |
| `currency` | String | **yes** | Currency |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact Details |
| `discount` | [`DiscountDto`](#discountdto) | no | Discount |
| `items` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `termsNotes` | String | **yes** | Terms notes |
| `compiledTermsNotes` | String | **yes** | Compiled terms notes |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `AutoPaymentScheduleDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `id` | String | **yes** | — |
| `autoPayment` | [`AutoPaymentDetailsDto`](#autopaymentdetailsdto) | **yes** | auto-payment configuration |

### `BacsDirectDebitDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `sort_code` | String | **yes** | — |
| `last4` | String | **yes** | — |

### `BecsDirectDebitDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `bsb_number` | String | **yes** | — |
| `last4` | String | **yes** | — |

### `BusinessDetails`

_No fields defined in the spec._

### `BusinessDetailsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `logoUrl` | String | no | Business Logo URL |
| `name` | String | no | Business Name |
| `phoneNo` | String | no | Business Phone Number |
| `address` | [`AddressDto`](#addressdto) | no | Business Address |
| `website` | String | no | Business Website Link |
| `customValues` | Vec<String> | no | Custom Values |

### `CancelInvoiceScheduleDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |

### `CancelInvoiceScheduleResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Schedule Id |
| `status` | JSON | **yes** | Schedule Status |
| `liveMode` | bool | **yes** | Live Mode |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | no | — |
| `invoices` | Vec<DefaultInvoiceResponseDto> | **yes** | List of invoices |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | Business Details |
| `currency` | String | **yes** | Currency |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact Details |
| `discount` | [`DiscountDto`](#discountdto) | no | Discount |
| `items` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `termsNotes` | String | **yes** | Terms notes |
| `compiledTermsNotes` | String | **yes** | Compiled terms notes |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `CardDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `brand` | String | **yes** | — |
| `last4` | String | **yes** | — |

### `ChequeDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `number` | String | **yes** | check number |

### `ContactDetails`

_No fields defined in the spec._

### `ContactDetailsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Contact ID |
| `name` | String | **yes** | Contact Name |
| `phoneNo` | String | **yes** | Contact Phone Number |
| `email` | String | **yes** | Contact Email |
| `additionalEmails` | Vec<AdditionalEmailsDto> | no | Secondary email addresses for the contact to be saved |
| `companyName` | String | no | Contact Company Name |
| `address` | [`AddressDto`](#addressdto) | no | — |
| `customFields` | Vec<String> | no | Custom Values |

### `CreateEstimatesDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Estimate Name |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | — |
| `currency` | String | **yes** | Currency code |
| `items` | Vec<EstimateLineItemDto> | **yes** | An array of items for the estimate. |
| `liveMode` | bool | no | livemode for estimate |
| `discount` | [`DiscountDto`](#discountdto) | **yes** | — |
| `termsNotes` | String | no | Terms notes, Also supports HTML markups |
| `title` | String | no | Title for the estimate |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact information to send the estimate to |
| `estimateNumber` | f64 | no | Estimate Number, if not specified will take in the next valid estimate number |
| `issueDate` | String | no | issue date estimate |
| `expiryDate` | String | no | expiry date estimate |
| `sentTo` | [`SentToDto`](#senttodto) | no | Email and sent to details for the estimate |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Estimate |
| `meta` | JSON | no | Meta data for the estimate |
| `sendEstimateDetails` | [`SendEstimateDto`](#sendestimatedto) | no | When sending estimate directly while saving |
| `frequencySettings` | [`FrequencySettingsDto`](#frequencysettingsdto) | **yes** | frequency settings for the estimate |
| `estimateNumberPrefix` | String | no | Prefix for the estimate number |
| `userId` | String | no | User Id |
| `attachments` | Vec<AttachmentsDto> | no | attachments for the invoice |
| `autoInvoice` | [`AutoInvoicingDto`](#autoinvoicingdto) | no | Auto invoice for the estimate |
| `miscellaneousCharges` | [`ProcessingFeeDto`](#processingfeedto) | no | miscellaneous charges for the estimate |
| `paymentScheduleConfig` | [`PaymentScheduleConfigDto`](#paymentscheduleconfigdto) | no | Payment Schedule Config for the estimate |

### `CreateInvoiceDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `name` | String | **yes** | Invoice Name |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | — |
| `currency` | String | **yes** | Currency code |
| `items` | Vec<InvoiceItemDto> | **yes** | An array of items for the invoice. |
| `discount` | [`DiscountDto`](#discountdto) | **yes** | — |
| `termsNotes` | String | no | Terms notes, Also supports HTML markups |
| `title` | String | no | Title for the invoice |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact information to send the invoice to |
| `invoiceNumber` | String | no | Invoice Number |
| `issueDate` | String | **yes** | Issue date in YYYY-MM-DD format |
| `dueDate` | String | no | Due date in YYYY-MM-DD format |
| `sentTo` | [`SentToDto`](#senttodto) | **yes** | — |
| `liveMode` | bool | **yes** | — |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `paymentSchedule` | [`PaymentScheduleDto`](#paymentscheduledto) | no | split invoice into payment schedule summing up to full invoice amount |
| `lateFeesConfiguration` | [`LateFeesConfigurationDto`](#latefeesconfigurationdto) | no | late fees configuration |
| `tipsConfiguration` | [`TipsConfigurationDto`](#tipsconfigurationdto) | no | tips configuration for the invoice |
| `invoiceNumberPrefix` | String | no | prefix for invoice number |
| `paymentMethods` | [`PaymentMethodDto`](#paymentmethoddto) | no | Payment Methods for Invoices |
| `attachments` | Vec<AttachmentsDto> | no | attachments for the invoice |
| `miscellaneousCharges` | [`ProcessingFeeDto`](#processingfeedto) | no | miscellaneous charges for the invoice |

### `CreateInvoiceFromEstimateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `markAsInvoiced` | bool | **yes** | Mark Estimate as Invoiced |
| `version` | String — `v1`, `v2` | no | Version of the update request |

### `CreateInvoiceFromEstimateResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `estimate` | [`EstimateResponseDto`](#estimateresponsedto) | **yes** | Estimate details |
| `invoice` | [`DefaultInvoiceResponseDto`](#defaultinvoiceresponsedto) | **yes** | Invoice details |

### `CreateInvoiceResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Invoice Id |
| `status` | String — `draft`, `sent`, `payment_processing`, `paid`, `void`, `partially_paid` | **yes** | Invoice Status |
| `liveMode` | bool | **yes** | Live Mode |
| `amountPaid` | f64 | **yes** | Amount Paid |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `businessDetails` | JSON | **yes** | Business Details |
| `invoiceNumber` | f64 | **yes** | Invoice Number |
| `currency` | String | **yes** | Currency |
| `contactDetails` | JSON | **yes** | Contact Details |
| `issueDate` | String | **yes** | Issue date in YYYY-MM-DD format |
| `dueDate` | String | **yes** | Due date in YYYY-MM-DD format |
| `discount` | JSON | no | Discount |
| `invoiceItems` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `amountDue` | f64 | **yes** | Total Amount Due |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `automaticTaxesCalculated` | bool | no | Is Automatic taxes calculated for the Invoice items |
| `paymentSchedule` | JSON | no | split invoice into payment schedule summing up to full invoice amount |

### `CreateInvoiceScheduleDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `name` | String | **yes** | — |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | — |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | **yes** | — |
| `liveMode` | bool | **yes** | — |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | — |
| `currency` | String | **yes** | — |
| `items` | Vec<InvoiceItemDto> | **yes** | — |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `discount` | [`DiscountDto`](#discountdto) | **yes** | — |
| `termsNotes` | String | no | — |
| `title` | String | no | — |
| `tipsConfiguration` | [`TipsConfigurationDto`](#tipsconfigurationdto) | no | Configuration for tips on invoices |
| `lateFeesConfiguration` | [`LateFeesConfigurationDto`](#latefeesconfigurationdto) | no | Late fees configuration for the invoices |
| `invoiceNumberPrefix` | String | no | prefix for invoice number |
| `paymentMethods` | [`PaymentMethodDto`](#paymentmethoddto) | no | Payment Methods for Invoices |
| `attachments` | Vec<AttachmentsDto> | no | attachments for the invoice |
| `miscellaneousCharges` | [`ProcessingFeeDto`](#processingfeedto) | no | miscellaneous charges for the invoice |

### `CreateInvoiceScheduleResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Schedule Id |
| `status` | JSON | **yes** | Schedule Status |
| `liveMode` | bool | **yes** | Live Mode |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | no | — |
| `invoices` | Vec<DefaultInvoiceResponseDto> | **yes** | List of invoices |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | Business Details |
| `currency` | String | **yes** | Currency |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact Details |
| `discount` | [`DiscountDto`](#discountdto) | no | Discount |
| `items` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `termsNotes` | String | **yes** | Terms notes |
| `compiledTermsNotes` | String | **yes** | Compiled terms notes |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `CreateInvoiceTemplateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `internal` | bool | no | — |
| `name` | String | **yes** | Name of the template |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | — |
| `currency` | String | **yes** | — |
| `items` | Vec<InvoiceItemDto> | **yes** | — |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `discount` | [`DiscountDto`](#discountdto) | no | — |
| `termsNotes` | String | no | — |
| `title` | String | no | Template title |
| `tipsConfiguration` | [`TipsConfigurationDto`](#tipsconfigurationdto) | no | Configuration for tips on invoices |
| `lateFeesConfiguration` | [`LateFeesConfigurationDto`](#latefeesconfigurationdto) | no | Late fees configuration for the invoices |
| `invoiceNumberPrefix` | String | no | prefix for invoice number |
| `paymentMethods` | [`PaymentMethodDto`](#paymentmethoddto) | no | Payment Methods for Invoices |
| `attachments` | Vec<String> | no | attachments for the invoice |
| `miscellaneousCharges` | [`ProcessingFeeDto`](#processingfeedto) | no | miscellaneous charges for the invoice |

### `CreateInvoiceTemplateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Template Id |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the Template |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | Business Details |
| `currency` | String | **yes** | Currency |
| `discount` | [`DiscountDto`](#discountdto) | no | Discount |
| `items` | Vec<String> | **yes** | Invoice Items |
| `invoiceNumberPrefix` | String | no | prefix for invoice number |
| `total` | f64 | **yes** | Total Amount |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `CustomNotificationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `customerSendInvoice` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `teamPaymentSuccess` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `customerPaymentSuccess` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `teamAutoPaymentSuccess` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `customerAutoPaymentSuccess` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `teamPaymentFailure` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `customerPaymentFailure` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `teamAutoPaymentFailure` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `customerAutoPaymentFailure` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `customerAutoPaymentInfo` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `customerAutoPaymentAmountChanged` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `teamAutoPaymentSkip` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `teamRecurringSendInvoiceFailed` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `customerSendEstimate` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `teamEstimateAccepted` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `teamEstimateDeclined` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |

### `CustomNotificationItemDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | Flag indicating if the notification is enabled or not |
| `emailTemplate` | String | **yes** | Template to be used for sending email |
| `smsTemplate` | String | **yes** | Template to be used for sending sms |
| `fromName` | String | no | Name to be used while sending email |
| `fromEmail` | String | no | Email address to be used for sending email |
| `emailSubject` | String | no | Subject of email which is sent out |
| `defaultEmailTemplateId` | String | no | Default email TemplateId to be used for sending via email |

### `CustomRRuleOptionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `intervalType` | String — `yearly`, `monthly`, `weekly`, `daily`, `hourly`, `minutely`, `secondly` | **yes** | — |
| `interval` | f64 | **yes** | — |
| `startDate` | String | **yes** | Start date in YYYY-MM-DD format |
| `startTime` | String | no | Start time in HH:mm:ss format |
| `endDate` | String | no | End date in YYYY-MM-DD format |
| `endTime` | String | no | End time in HH:mm:ss format |
| `dayOfMonth` | f64 | no | -1, 1, 2, 3, ..., 27, 28 |
| `dayOfWeek` | String — `mo`, `tu`, `we`, `th`, `fr`, `sa`, `su` | no | — |
| `numOfWeek` | f64 | no | -1, 1, 2, 3, 4 |
| `monthOfYear` | String — `jan`, `feb`, `mar`, `apr`, `may`, `jun`, `jul`, `aug`, `sep`, `oct`, `nov`, `dec` | no | — |
| `count` | f64 | no | Max number of task executions |
| `daysBefore` | f64 | no | Execute task number of days before |
| `useStartAsPrimaryUserAccepted` | bool | no | Start as primary user accepted date |
| `endType` | String | no | End type like after, by, count |

### `DefaultInvoiceResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Invoice Id |
| `status` | String — `draft`, `sent`, `payment_processing`, `paid`, `void`, `partially_paid` | **yes** | Invoice Status |
| `liveMode` | bool | **yes** | Live Mode |
| `amountPaid` | f64 | **yes** | Amount Paid |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `businessDetails` | JSON | **yes** | Business Details |
| `invoiceNumber` | f64 | **yes** | Invoice Number |
| `currency` | String | **yes** | Currency |
| `contactDetails` | JSON | **yes** | Contact Details |
| `issueDate` | String | **yes** | Issue date in YYYY-MM-DD format |
| `dueDate` | String | **yes** | Due date in YYYY-MM-DD format |
| `discount` | JSON | no | Discount |
| `invoiceItems` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `amountDue` | f64 | **yes** | Total Amount Due |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `automaticTaxesCalculated` | bool | no | Is Automatic taxes calculated for the Invoice items |
| `paymentSchedule` | JSON | no | split invoice into payment schedule summing up to full invoice amount |

### `DeleteInvoiceResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Invoice Id |
| `status` | String — `draft`, `sent`, `payment_processing`, `paid`, `void`, `partially_paid` | **yes** | Invoice Status |
| `liveMode` | bool | **yes** | Live Mode |
| `amountPaid` | f64 | **yes** | Amount Paid |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `businessDetails` | JSON | **yes** | Business Details |
| `invoiceNumber` | f64 | **yes** | Invoice Number |
| `currency` | String | **yes** | Currency |
| `contactDetails` | JSON | **yes** | Contact Details |
| `issueDate` | String | **yes** | Issue date in YYYY-MM-DD format |
| `dueDate` | String | **yes** | Due date in YYYY-MM-DD format |
| `discount` | JSON | no | Discount |
| `invoiceItems` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `amountDue` | f64 | **yes** | Total Amount Due |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `automaticTaxesCalculated` | bool | no | Is Automatic taxes calculated for the Invoice items |
| `paymentSchedule` | JSON | no | split invoice into payment schedule summing up to full invoice amount |

### `DeleteInvoiceScheduleResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | success |

### `DeleteInvoiceTemplateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | success |

### `DiscountDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `value` | f64 | no | Discount Value |
| `type` | String — `percentage`, `fixed` | **yes** | Discount type |
| `validOnProductIds` | Vec<String> | no | Product Ids on which discount is applicable |

### `EstimateIdParam`

| Field | Type | Required | Description |
|---|---|---|---|
| `estimateId` | String | **yes** | Estimate Id |

### `EstimateLineItemDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Invoice Item Name |
| `description` | String | no | Invoice descriptions |
| `productId` | String | no | Product Id |
| `priceId` | String | no | Price Id |
| `currency` | String | **yes** | Currency |
| `amount` | f64 | **yes** | Product amount |
| `qty` | f64 | **yes** | Product Quantity |
| `taxes` | Vec<ItemTaxDto> | no | Tax |
| `automaticTaxCategoryId` | String | no | Tax category id for calculating automatic tax |
| `isSetupFeeItem` | bool | no | Setupfee item, only created when 1st invoice of recurring schedule is generated |
| `type` | String — `one_time`, `recurring` | no | Price type of the item |
| `taxInclusive` | bool | no | true if item amount is tax inclusive |
| `attachments` | Vec<String> | no | Attachments for the line item |

### `EstimateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `_id` | String | **yes** | Unique identifier |
| `liveMode` | bool | **yes** | Indicates if it is in live mode |
| `deleted` | bool | **yes** | Indicates if deleted |
| `name` | String | **yes** | Name |
| `currency` | String | **yes** | Currency code |
| `businessDetails` | [`BusinessDetails`](#businessdetails) | **yes** | Business details associated with the estimate |
| `items` | Vec<Vec<JSON>> | **yes** | An array of items |
| `discount` | [`DiscountDto`](#discountdto) | **yes** | Discount details for the estimate template |
| `title` | String | no | Title |
| `estimateNumberPrefix` | String | no | Estimate number prefix |
| `attachments` | Vec<AttachmentsDto> | no | Attachments |
| `updatedBy` | String | no | User Id of who last updated |
| `total` | f64 | **yes** | Total amount |
| `createdAt` | String | **yes** | Timestamp when created |
| `updatedAt` | String | **yes** | Timestamp when last updated |
| `__v` | f64 | **yes** | Version number |
| `automaticTaxesEnabled` | bool | **yes** | Indicates if automatic taxes are enabled for this estimate |
| `termsNotes` | String | no | Terms and conditions for the estimate, supports HTML markup |
| `companyId` | String | **yes** | Company identifier associated with the estimate |
| `contactDetails` | [`ContactDetails`](#contactdetails) | **yes** | Contact details for the estimate |
| `issueDate` | String | **yes** | Date when the estimate was issued |
| `expiryDate` | String | **yes** | Date when the estimate expires |
| `sentBy` | String | no | User who sent the estimate |
| `automaticTaxesCalculated` | bool | **yes** | Indicates if automatic taxes were calculated |
| `meta` | JSON | **yes** | Additional metadata associated with the estimate |
| `estimateActionHistory` | Vec<String> | **yes** | History of actions taken on the estimate |
| `sentTo` | [`SentTo`](#sentto) | **yes** | Recipient details for the estimate |
| `frequencySettings` | [`FrequencySettingsDto`](#frequencysettingsdto) | **yes** | Frequency settings for recurring estimates |
| `lastVisitedAt` | String | **yes** | Timestamp when the estimate was last visited |
| `totalamountInUSD` | f64 | **yes** | Total amount in USD |
| `autoInvoice` | [`AutoInvoice`](#autoinvoice) | no | Auto-invoice settings for the estimate |
| `traceId` | String | **yes** | Trace ID for logging and debugging |

### `EstimateTemplateResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `_id` | String | **yes** | Unique identifier |
| `liveMode` | bool | **yes** | Indicates if it is in live mode |
| `deleted` | bool | **yes** | Indicates if deleted |
| `name` | String | **yes** | Name |
| `currency` | String | **yes** | Currency code |
| `businessDetails` | [`BusinessDetails`](#businessdetails) | **yes** | Business details associated with the estimate |
| `items` | Vec<Vec<JSON>> | **yes** | An array of items |
| `discount` | [`DiscountDto`](#discountdto) | **yes** | Discount details for the estimate template |
| `title` | String | no | Title |
| `estimateNumberPrefix` | String | no | Estimate number prefix |
| `attachments` | Vec<AttachmentsDto> | no | Attachments |
| `updatedBy` | String | no | User Id of who last updated |
| `total` | f64 | **yes** | Total amount |
| `createdAt` | String | **yes** | Timestamp when created |
| `updatedAt` | String | **yes** | Timestamp when last updated |
| `__v` | f64 | **yes** | Version number |
| `automaticTaxesEnabled` | bool | **yes** | Indicates if automatic taxes are enabled for this estimate |
| `termsNotes` | String | no | Terms and conditions for the estimate, supports HTML markup |

### `EstimateTemplatesDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Estimate Name |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | — |
| `currency` | String | **yes** | Currency code |
| `items` | Vec<Vec<JSON>> | **yes** | An array of items for the estimate. |
| `liveMode` | bool | no | livemode for estimate |
| `discount` | [`DiscountDto`](#discountdto) | **yes** | — |
| `termsNotes` | String | no | Terms notes, Also supports HTML markups |
| `title` | String | no | Title for the estimate |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Estimate |
| `meta` | JSON | no | Meta data for the estimate |
| `sendEstimateDetails` | [`SendEstimateDto`](#sendestimatedto) | no | When sending estimate directly while saving |
| `estimateNumberPrefix` | String | no | Prefix for the estimate number |
| `attachments` | Vec<AttachmentsDto> | no | attachments for the invoice |
| `miscellaneousCharges` | [`ProcessingFeeDto`](#processingfeedto) | no | miscellaneous charges for the estimate |

### `FrequencySettingsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | enabled for the frequency settings |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | **yes** | schedule setting for the estimate |

### `GenerateEstimateNumberResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `estimateNumber` | f64 | **yes** | — |
| `traceId` | String | **yes** | — |

### `GenerateInvoiceNumberResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `invoiceNumber` | f64 | no | Invoice Number |

### `GetInvoiceResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Invoice Id |
| `status` | String — `draft`, `sent`, `payment_processing`, `paid`, `void`, `partially_paid` | **yes** | Invoice Status |
| `liveMode` | bool | **yes** | Live Mode |
| `amountPaid` | f64 | **yes** | Amount Paid |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `businessDetails` | JSON | **yes** | Business Details |
| `invoiceNumber` | f64 | **yes** | Invoice Number |
| `currency` | String | **yes** | Currency |
| `contactDetails` | JSON | **yes** | Contact Details |
| `issueDate` | String | **yes** | Issue date in YYYY-MM-DD format |
| `dueDate` | String | **yes** | Due date in YYYY-MM-DD format |
| `discount` | JSON | no | Discount |
| `invoiceItems` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `amountDue` | f64 | **yes** | Total Amount Due |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `automaticTaxesCalculated` | bool | no | Is Automatic taxes calculated for the Invoice items |
| `paymentSchedule` | JSON | no | split invoice into payment schedule summing up to full invoice amount |
| `totalSummary` | [`TotalSummaryDto`](#totalsummarydto) | **yes** | — |
| `remindersConfiguration` | [`RemindersConfigurationDto`](#remindersconfigurationdto) | no | Reminders Configuration |

### `GetInvoiceSettingsResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | no | Sub-Account Id |
| `altType` | String — `location` | no | Alt Type |
| `termsNote` | String | no | Terms and conditions for invoices |
| `estimatesTermsNote` | String | no | Terms and conditions for estimates |
| `title` | String | no | Title for invoices |
| `estimatesTitle` | String | no | Title for estimates |
| `invoiceNumberPrefix` | String | no | Prefix for invoice numbers |
| `estimateNumberPrefix` | String | no | Prefix for estimate numbers |
| `dueAfterXDays` | f64 | no | Number of days after which invoice is due |
| `estimatesExpireAfterXDays` | f64 | no | Number of days after which estimate expires |
| `minimumPercentagePartialPayment` | f64 | no | Minimum percentage for partial payment |
| `customFields` | Vec<String> | no | Custom fields array |
| `customNotification` | [`CustomNotificationDto`](#customnotificationdto) | no | Custom notification settings |
| `businessDetails` | [`InvoiceSettingsBusinessDetailsDto`](#invoicesettingsbusinessdetailsdto) | no | Business details |
| `senderConfiguration` | [`InvoiceSettingsSenderConfigurationDto`](#invoicesettingssenderconfigurationdto) | no | Sender configuration |
| `productSettings` | [`InvoiceProductSettingsDto`](#invoiceproductsettingsdto) | no | Product settings |
| `reminderSettings` | [`ReminderSettingsDto`](#remindersettingsdto) | no | Reminder settings |
| `lateFeesConfiguration` | [`LateFeesConfigurationDto`](#latefeesconfigurationdto) | no | Late fees configuration |
| `tipsConfiguration` | [`TipsConfigurationDto`](#tipsconfigurationdto) | no | Tips configuration |
| `paymentMethods` | [`PaymentMethodDto`](#paymentmethoddto) | no | Payment methods configuration |

### `GetScheduleResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Schedule Id |
| `status` | JSON | **yes** | Schedule Status |
| `liveMode` | bool | **yes** | Live Mode |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | no | — |
| `invoices` | Vec<DefaultInvoiceResponseDto> | **yes** | List of invoices |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | Business Details |
| `currency` | String | **yes** | Currency |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact Details |
| `discount` | [`DiscountDto`](#discountdto) | no | Discount |
| `items` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `termsNotes` | String | **yes** | Terms notes |
| `compiledTermsNotes` | String | **yes** | Compiled terms notes |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `GetTemplateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Template Id |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the Template |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | Business Details |
| `currency` | String | **yes** | Currency |
| `discount` | [`DiscountDto`](#discountdto) | no | Discount |
| `items` | Vec<String> | **yes** | Invoice Items |
| `invoiceNumberPrefix` | String | no | prefix for invoice number |
| `total` | f64 | **yes** | Total Amount |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `InvoiceItemDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Invoice Item Name |
| `description` | String | no | Invoice descriptions |
| `productId` | String | no | Product Id |
| `priceId` | String | no | Price Id |
| `currency` | String | **yes** | Currency |
| `amount` | f64 | **yes** | Product amount |
| `qty` | f64 | **yes** | Product Quantity |
| `taxes` | Vec<ItemTaxDto> | no | Tax |
| `automaticTaxCategoryId` | String | no | Tax category id for calculating automatic tax |
| `isSetupFeeItem` | bool | no | Setupfee item, only created when 1st invoice of recurring schedule is generated |
| `type` | String — `one_time`, `recurring` | no | Price type of the item |
| `taxInclusive` | bool | no | true if item amount is tax inclusive |

### `InvoiceProductSettingsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enableImportProductDescription` | bool | no | Flag indicating if the product description import is enabled or not |
| `descriptionOptional` | bool | no | Flag indicating if the product description is optional or not |

### `InvoiceSettingsBusinessDetailsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `logoUrl` | String | no | — |
| `name` | String | **yes** | — |
| `phoneNo` | String | no | — |
| `address` | [`Address`](#address) | no | — |
| `website` | String | no | — |
| `customValues` | Vec<String> | no | — |

### `InvoiceSettingsSenderConfigurationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `fromName` | String | no | Sender name to be used while sending email notification |
| `fromEmail` | String | no | Email id to be used while sending email notification |

### `ItemTaxDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | — |
| `name` | String | **yes** | — |
| `rate` | f64 | **yes** | — |
| `calculation` | String — `exclusive` | no | — |
| `description` | String | no | — |
| `taxId` | String | no | — |

### `LateFeesConfigurationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enable` | bool | **yes** | Enable late fees |
| `value` | f64 | **yes** | Late Fees Value |
| `type` | String — `fixed`, `percentage` | **yes** | Late Fees Type |
| `frequency` | [`LateFeesFrequencyDto`](#latefeesfrequencydto) | **yes** | Late Fees Frequency |
| `grace` | [`LateFeesGraceDto`](#latefeesgracedto) | no | Late Fees Grace |
| `maxLateFees` | [`LateFeesMaxFeesDto`](#latefeesmaxfeesdto) | no | Max late fees payable |

### `LateFeesFrequencyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `intervalCount` | f64 | **yes** | Late fees interval count |
| `interval` | String — `minute`, `hour`, `day`, `week`, `month`, `one_time` | **yes** | Late fees interval |

### `LateFeesGraceDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `intervalCount` | f64 | **yes** | Late fees grace interval count |
| `interval` | String — `day` | **yes** | Late fees grace interval |

### `LateFeesMaxFeesDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `fixed` | **yes** | — |
| `value` | f64 | **yes** | Max late fees to pay |

### `ListEstimateTemplateResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<String> | **yes** | List of estimate templates |
| `totalCount` | f64 | **yes** | Total number of estimate templates available |
| `traceId` | String | **yes** | Unique identifier for tracing the request |

### `ListEstimatesResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `estimates` | Vec<String> | **yes** | List of estimates |
| `total` | f64 | **yes** | Total number of estimates |
| `traceId` | String | **yes** | Unique identifier for tracing the request |

### `ListInvoicesResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `invoices` | Vec<GetInvoiceResponseDto> | **yes** | — |
| `total` | f64 | **yes** | Total number of invoices |

### `ListSchedulesResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `schedules` | Vec<GetScheduleResponseDto> | **yes** | — |
| `total` | f64 | **yes** | Total number of Schedules |

### `ListTemplatesResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<GetTemplateResponseDto> | **yes** | — |
| `totalCount` | f64 | **yes** | Total number of Templates |

### `OldCreateInvoiceDTO`

_No fields defined in the spec._

### `PatchInvoiceStatsLastViewedDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `invoiceId` | String | **yes** | Invoice Id |

### `PaymentMethodDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `stripe` | [`StripePaymentMethodDto`](#stripepaymentmethoddto) | **yes** | Payment Method |

### `PaymentScheduleConfigDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `fixed`, `percentage` | **yes** | Payment Schedule Type |
| `dateConfig` | [`PaymentScheduleDateConfigDto`](#paymentscheduledateconfigdto) | **yes** | Due date type configuration |
| `schedules` | Vec<Vec<JSON>> | **yes** | Payment Schedule Items |

### `PaymentScheduleDateConfigDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `depositDateType` | String — `estimate_accepted`, `custom` | **yes** | Deposit date type |
| `scheduleDateType` | String — `regular_interval`, `custom` | **yes** | Payment Schedule Date Type |

### `PaymentScheduleDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `fixed`, `percentage` | **yes** | Payment schedule type |
| `schedules` | Vec<String> | **yes** | payment schedule item |

### `ProcessingFeeDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `charges` | Vec<Vec<JSON>> | **yes** | charges for the processing fee |
| `collectedMiscellaneousCharges` | f64 | no | collected miscellaneous charges |
| `paidCharges` | Vec<ProcessingFeePaidChargeDto> | no | paid miscellaneous charges |

### `ProcessingFeePaidChargeDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | name of the processing fee |
| `charge` | f64 | **yes** | charge for the processing fee |
| `amount` | f64 | **yes** | amount of the processing fee |
| `_id` | String | **yes** | id of the processing fee |

### `RecordPaymentDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `mode` | String — `cash`, `card`, `cheque`, `bank_transfer`, `other` | **yes** | manual payment method |
| `card` | [`CardDto`](#carddto) | **yes** | — |
| `cheque` | [`ChequeDto`](#chequedto) | **yes** | — |
| `notes` | String | **yes** | Any note to be recorded with the transaction |
| `amount` | f64 | no | Amount to be paid against the invoice. |
| `meta` | JSON | no | — |
| `paymentScheduleIds` | Vec<String> | no | Payment Schedule Ids to be recorded against the invoice. |
| `fulfilledAt` | String | no | Updated At to be recorded against the invoice. |

### `RecordPaymentResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | status |
| `invoice` | [`DefaultInvoiceResponseDto`](#defaultinvoiceresponsedto) | **yes** | — |

### `ReminderDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | Flag indicating if the reminder is enabled or not |
| `emailTemplate` | String | **yes** | Email template to be used for sending reminders |
| `smsTemplate` | String | **yes** | SMS template to be used for sending reminders |
| `emailSubject` | String | **yes** | Subject of the reminder |
| `reminderId` | String | **yes** | Unique identifier for the reminder |
| `reminderName` | String | **yes** | Name of the reminder |
| `reminderTime` | String — `before`, `after` | **yes** | Time condition for the reminder |
| `intervalType` | String — `yearly`, `monthly`, `weekly`, `daily`, `hourly`, `minutely`, `secondly` | **yes** | Interval type for the reminder |
| `maxReminders` | f64 | **yes** | Maximum number of reminders that can be sent |
| `reminderInvoiceCondition` | String — `invoice_sent`, `invoice_overdue` | **yes** | Condition for sending the reminder |
| `reminderNumber` | f64 | **yes** | frequency gap of the reminder to exeucte |
| `startTime` | String | no | Business Hour Start Time |
| `endTime` | String | no | Business Hour End Time |
| `timezone` | String | no | Timezone at which reminder will be sent |

### `ReminderExecutionDetailsList`

_No fields defined in the spec._

### `ReminderSettingsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `defaultEmailTemplateId` | String | **yes** | default template Id of reminder |
| `reminders` | Vec<ReminderDto> | **yes** | List of reminders |

### `RemindersConfigurationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `reminderExecutionDetailsList` | [`ReminderExecutionDetailsList`](#reminderexecutiondetailslist) | **yes** | List of reminders |
| `reminderSettings` | [`ReminderSettingsDto`](#remindersettingsdto) | **yes** | Reminder settings |

### `ScheduleInvoiceScheduleDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `liveMode` | bool | **yes** | — |
| `autoPayment` | [`AutoPaymentDetailsDto`](#autopaymentdetailsdto) | no | auto-payment configuration |

### `ScheduleInvoiceScheduleResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Schedule Id |
| `status` | JSON | **yes** | Schedule Status |
| `liveMode` | bool | **yes** | Live Mode |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | no | — |
| `invoices` | Vec<DefaultInvoiceResponseDto> | **yes** | List of invoices |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | Business Details |
| `currency` | String | **yes** | Currency |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact Details |
| `discount` | [`DiscountDto`](#discountdto) | no | Discount |
| `items` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `termsNotes` | String | **yes** | Terms notes |
| `compiledTermsNotes` | String | **yes** | Compiled terms notes |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `ScheduleOptionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `executeAt` | String | no | — |
| `rrule` | [`CustomRRuleOptionsDto`](#customrruleoptionsdto) | no | — |

### `SendEstimateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `action` | String — `sms_and_email`, `send_manually`, `email`, `sms` | **yes** | — |
| `liveMode` | bool | **yes** | livemode for estimate |
| `userId` | String | **yes** | Please ensure that the UserId corresponds to an authorized personnel, either by an employee ID or agency ID, to access this location. This account will serve as the primary channel for all future comm… |
| `sentFrom` | [`InvoiceSettingsSenderConfigurationDto`](#invoicesettingssenderconfigurationdto) | no | sender details for invoice, valid only if invoice is not sent manually |
| `estimateName` | String | no | estimate name |

### `SendInvoiceDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `userId` | String | **yes** | Please ensure that the UserId corresponds to an authorized personnel, either by an employee ID or agency ID, to access this location. This account will serve as the primary channel for all future comm… |
| `action` | String — `sms_and_email`, `send_manually`, `email`, `sms` | **yes** | — |
| `liveMode` | bool | **yes** | — |
| `sentFrom` | [`InvoiceSettingsSenderConfigurationDto`](#invoicesettingssenderconfigurationdto) | no | sender details for invoice, valid only if invoice is not sent manually |
| `autoPayment` | [`AutoPaymentDetailsDto`](#autopaymentdetailsdto) | no | auto-payment configuration |

### `SendInvoicesResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `invoice` | [`DefaultInvoiceResponseDto`](#defaultinvoiceresponsedto) | **yes** | — |
| `smsData` | JSON | **yes** | — |
| `emailData` | JSON | **yes** | — |

### `SentTo`

_No fields defined in the spec._

### `SentToDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `email` | Vec<String> | **yes** | Email Address |
| `emailCc` | Vec<String> | no | cc to be kept in any sent out emails |
| `emailBcc` | Vec<String> | no | bcc to be kept in any sent out emails |
| `phoneNo` | Vec<String> | no | Contact Phone Number |

### `SepaDirectDebitDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `bank_code` | String | **yes** | — |
| `last4` | String | **yes** | — |
| `branch_code` | String | **yes** | — |

### `StripePaymentMethodDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enableBankDebitOnly` | bool | **yes** | Enable Bank Debit Only |

### `Text2PayDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `name` | String | **yes** | Invoice Name |
| `currency` | String | **yes** | Currency code |
| `items` | Vec<InvoiceItemDto> | **yes** | An array of items for the invoice. |
| `termsNotes` | String | no | Terms notes, Also supports HTML markups |
| `title` | String | no | Title for the invoice |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact information to send the invoice to |
| `invoiceNumber` | String | no | Invoice Number |
| `issueDate` | String | **yes** | Issue date in YYYY-MM-DD format |
| `dueDate` | String | no | Due date in YYYY-MM-DD format |
| `sentTo` | [`SentToDto`](#senttodto) | **yes** | — |
| `liveMode` | bool | **yes** | — |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `paymentSchedule` | [`PaymentScheduleDto`](#paymentscheduledto) | no | split invoice into payment schedule summing up to full invoice amount |
| `lateFeesConfiguration` | [`LateFeesConfigurationDto`](#latefeesconfigurationdto) | no | late fees configuration |
| `tipsConfiguration` | [`TipsConfigurationDto`](#tipsconfigurationdto) | no | tips configuration for the invoice |
| `invoiceNumberPrefix` | String | no | prefix for invoice number |
| `paymentMethods` | [`PaymentMethodDto`](#paymentmethoddto) | no | Payment Methods for Invoices |
| `attachments` | Vec<AttachmentsDto> | no | attachments for the invoice |
| `miscellaneousCharges` | [`ProcessingFeeDto`](#processingfeedto) | no | miscellaneous charges for the invoice |
| `id` | String | no | id of invoice to update. If skipped, a new invoice will be created |
| `includeTermsNote` | bool | no | include terms & notes with receipts |
| `action` | String — `draft`, `send` | **yes** | create invoice in draft mode or send mode |
| `userId` | String | **yes** | id of user generating invoice |
| `discount` | [`DiscountDto`](#discountdto) | no | — |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | no | — |

### `Text2PayInvoiceResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `invoice` | [`DefaultInvoiceResponseDto`](#defaultinvoiceresponsedto) | **yes** | — |
| `invoiceUrl` | String | **yes** | preview url of generated invoice |

### `TipsConfigurationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `tipsPercentage` | Vec<String> | **yes** | Percentage of tips allowed |
| `tipsEnabled` | bool | **yes** | Tips enabled status |

### `TotalSummaryDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `subTotal` | f64 | **yes** | subTotal |
| `discount` | f64 | **yes** | discount |
| `tax` | f64 | **yes** | tax |

### `USBankAccountDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `bank_name` | String | **yes** | — |
| `last4` | String | **yes** | — |

### `UpdateAndScheduleInvoiceScheduleResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Schedule Id |
| `status` | JSON | **yes** | Schedule Status |
| `liveMode` | bool | **yes** | Live Mode |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | no | — |
| `invoices` | Vec<DefaultInvoiceResponseDto> | **yes** | List of invoices |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | Business Details |
| `currency` | String | **yes** | Currency |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact Details |
| `discount` | [`DiscountDto`](#discountdto) | no | Discount |
| `items` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `termsNotes` | String | **yes** | Terms notes |
| `compiledTermsNotes` | String | **yes** | Compiled terms notes |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `UpdateEstimateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Estimate Name |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | — |
| `currency` | String | **yes** | Currency code |
| `items` | Vec<EstimateLineItemDto> | **yes** | An array of items for the estimate. |
| `liveMode` | bool | no | livemode for estimate |
| `discount` | [`DiscountDto`](#discountdto) | **yes** | — |
| `termsNotes` | String | no | Terms notes, Also supports HTML markups |
| `title` | String | no | Title for the estimate |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact information to send the estimate to |
| `estimateNumber` | f64 | no | Estimate Number, if not specified will take in the next valid estimate number |
| `issueDate` | String | no | issue date estimate |
| `expiryDate` | String | no | expiry date estimate |
| `sentTo` | [`SentToDto`](#senttodto) | no | Email and sent to details for the estimate |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Estimate |
| `meta` | JSON | no | Meta data for the estimate |
| `sendEstimateDetails` | [`SendEstimateDto`](#sendestimatedto) | no | When sending estimate directly while saving |
| `frequencySettings` | [`FrequencySettingsDto`](#frequencysettingsdto) | **yes** | frequency settings for the estimate |
| `estimateNumberPrefix` | String | no | Prefix for the estimate number |
| `userId` | String | no | User Id |
| `attachments` | Vec<AttachmentsDto> | no | attachments for the invoice |
| `autoInvoice` | [`AutoInvoicingDto`](#autoinvoicingdto) | no | Auto invoice for the estimate |
| `miscellaneousCharges` | [`ProcessingFeeDto`](#processingfeedto) | no | miscellaneous charges for the estimate |
| `paymentScheduleConfig` | [`PaymentScheduleConfigDto`](#paymentscheduleconfigdto) | no | Payment Schedule Config for the estimate |
| `estimateStatus` | String — `all`, `draft`, `sent`, `accepted`, `declined`, `invoiced`, `viewed` | no | Estimate Status |

### `UpdateInvoiceDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `name` | String | **yes** | Name to be updated |
| `title` | String | no | Title for the invoice |
| `currency` | String | **yes** | Currency |
| `description` | String | no | Description |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | no | Business details which need to be updated |
| `invoiceNumber` | String | no | Invoice Number |
| `contactId` | String | no | Id of the contact which you need to send the invoice |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | no | — |
| `termsNotes` | String | no | Terms notes, Also supports HTML markups |
| `discount` | [`DiscountDto`](#discountdto) | no | — |
| `invoiceItems` | Vec<InvoiceItemDto> | **yes** | — |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `liveMode` | bool | no | Payment mode |
| `issueDate` | String | **yes** | Issue date in YYYY-MM-DD format |
| `dueDate` | String | **yes** | Due date in YYYY-MM-DD format |
| `paymentSchedule` | [`PaymentScheduleDto`](#paymentscheduledto) | no | split invoice into payment schedule summing up to full invoice amount |
| `tipsConfiguration` | [`TipsConfigurationDto`](#tipsconfigurationdto) | no | tips configuration for the invoice |
| `xeroDetails` | JSON | no | — |
| `invoiceNumberPrefix` | String | no | prefix for invoice number |
| `paymentMethods` | [`PaymentMethodDto`](#paymentmethoddto) | no | Payment Methods for Invoices |
| `attachments` | Vec<AttachmentsDto> | no | attachments for the invoice |
| `miscellaneousCharges` | [`ProcessingFeeDto`](#processingfeedto) | no | miscellaneous charges for the invoice |

### `UpdateInvoiceLateFeesConfigurationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `lateFeesConfiguration` | [`LateFeesConfigurationDto`](#latefeesconfigurationdto) | **yes** | late fees configuration |

### `UpdateInvoiceResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Invoice Id |
| `status` | String — `draft`, `sent`, `payment_processing`, `paid`, `void`, `partially_paid` | **yes** | Invoice Status |
| `liveMode` | bool | **yes** | Live Mode |
| `amountPaid` | f64 | **yes** | Amount Paid |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `businessDetails` | JSON | **yes** | Business Details |
| `invoiceNumber` | f64 | **yes** | Invoice Number |
| `currency` | String | **yes** | Currency |
| `contactDetails` | JSON | **yes** | Contact Details |
| `issueDate` | String | **yes** | Issue date in YYYY-MM-DD format |
| `dueDate` | String | **yes** | Due date in YYYY-MM-DD format |
| `discount` | JSON | no | Discount |
| `invoiceItems` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `amountDue` | f64 | **yes** | Total Amount Due |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `automaticTaxesCalculated` | bool | no | Is Automatic taxes calculated for the Invoice items |
| `paymentSchedule` | JSON | no | split invoice into payment schedule summing up to full invoice amount |

### `UpdateInvoiceScheduleDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `name` | String | **yes** | — |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | — |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | **yes** | — |
| `liveMode` | bool | **yes** | — |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | — |
| `currency` | String | **yes** | — |
| `items` | Vec<InvoiceItemDto> | **yes** | — |
| `discount` | [`DiscountDto`](#discountdto) | **yes** | — |
| `termsNotes` | String | no | — |
| `title` | String | no | — |
| `attachments` | Vec<AttachmentsDto> | no | attachments for the invoice |
| `miscellaneousCharges` | [`ProcessingFeeDto`](#processingfeedto) | no | miscellaneous charges for the invoice |

### `UpdateInvoiceScheduleResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Schedule Id |
| `status` | JSON | **yes** | Schedule Status |
| `liveMode` | bool | **yes** | Live Mode |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | no | — |
| `invoices` | Vec<DefaultInvoiceResponseDto> | **yes** | List of invoices |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | Business Details |
| `currency` | String | **yes** | Currency |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact Details |
| `discount` | [`DiscountDto`](#discountdto) | no | Discount |
| `items` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `termsNotes` | String | **yes** | Terms notes |
| `compiledTermsNotes` | String | **yes** | Compiled terms notes |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `UpdateInvoiceTemplateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `internal` | bool | no | — |
| `name` | String | **yes** | Name of the template |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | — |
| `currency` | String | **yes** | — |
| `items` | Vec<InvoiceItemDto> | **yes** | — |
| `discount` | [`DiscountDto`](#discountdto) | no | — |
| `termsNotes` | String | no | — |
| `title` | String | no | Template title |
| `miscellaneousCharges` | [`ProcessingFeeDto`](#processingfeedto) | no | miscellaneous charges for the invoice |

### `UpdateInvoiceTemplateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Template Id |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the Template |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | Business Details |
| `currency` | String | **yes** | Currency |
| `discount` | [`DiscountDto`](#discountdto) | no | Discount |
| `items` | Vec<String> | **yes** | Invoice Items |
| `invoiceNumberPrefix` | String | no | prefix for invoice number |
| `total` | f64 | **yes** | Total Amount |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `UpdatePaymentMethodsConfigurationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `paymentMethods` | [`PaymentMethodDto`](#paymentmethoddto) | no | Payment Methods for Invoices |

### `VoidInvoiceDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |

### `VoidInvoiceResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Invoice Id |
| `status` | String — `draft`, `sent`, `payment_processing`, `paid`, `void`, `partially_paid` | **yes** | Invoice Status |
| `liveMode` | bool | **yes** | Live Mode |
| `amountPaid` | f64 | **yes** | Amount Paid |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `businessDetails` | JSON | **yes** | Business Details |
| `invoiceNumber` | f64 | **yes** | Invoice Number |
| `currency` | String | **yes** | Currency |
| `contactDetails` | JSON | **yes** | Contact Details |
| `issueDate` | String | **yes** | Issue date in YYYY-MM-DD format |
| `dueDate` | String | **yes** | Due date in YYYY-MM-DD format |
| `discount` | JSON | no | Discount |
| `invoiceItems` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `amountDue` | f64 | **yes** | Total Amount Due |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `automaticTaxesCalculated` | bool | no | Is Automatic taxes calculated for the Invoice items |
| `paymentSchedule` | JSON | no | split invoice into payment schedule summing up to full invoice amount |

## Data models — API v3

In Rust: `ghl_models::v3::invoices::*` (enable the `invoices` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/invoices/).

### `AdditionalEmailsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `email` | String | **yes** | — |

### `Address`

| Field | Type | Required | Description |
|---|---|---|---|
| `addressLine1` | String | no | — |
| `addressLine2` | String | no | — |
| `city` | String | no | — |
| `state` | String | no | — |
| `countryCode` | String — 247 values ([shared](shared-enums.md)) | no | — |
| `postalCode` | String | no | — |

### `AddressDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `addressLine1` | String | no | Address Line 1 |
| `addressLine2` | String | no | Address Line 2 |
| `city` | String | no | City |
| `state` | String | no | State |
| `countryCode` | String | no | Country Code |
| `postalCode` | String | no | Postal Code |

### `AltDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |

### `AttachmentsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Id of the file selected |
| `name` | String | **yes** | Name of the file |
| `url` | String | **yes** | URL of the file |
| `type` | String | **yes** | Type of the file |
| `size` | f64 | **yes** | Size of the file |

### `AutoInvoice`

_No fields defined in the spec._

### `AutoInvoicingDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | Enable Auto Invoice |
| `directPayments` | bool | no | Direct Payments |

### `AutoPaymentDetailsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enable` | bool | **yes** | — |
| `type` | String | no | — |
| `paymentMethodId` | String | no | — |
| `customerId` | String | no | — |
| `card` | [`CardDto`](#carddto) | no | — |
| `usBankAccount` | [`USBankAccountDto`](#usbankaccountdto) | no | — |
| `sepaDirectDebit` | [`SepaDirectDebitDTO`](#sepadirectdebitdto) | no | — |
| `bacsDirectDebit` | [`BacsDirectDebitDTO`](#bacsdirectdebitdto) | no | — |
| `becsDirectDebit` | [`BecsDirectDebitDTO`](#becsdirectdebitdto) | no | — |
| `cardId` | String | no | — |
| `provider` | JSON | no | — |

### `AutoPaymentInvoiceScheduleResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Schedule Id |
| `status` | JSON | **yes** | Schedule Status |
| `liveMode` | bool | **yes** | Live Mode |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | no | — |
| `invoices` | Vec<DefaultInvoiceResponseDto> | **yes** | List of invoices |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | Business Details |
| `currency` | String | **yes** | Currency |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact Details |
| `discount` | [`DiscountDto`](#discountdto) | no | Discount |
| `items` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `termsNotes` | String | **yes** | Terms notes |
| `compiledTermsNotes` | String | **yes** | Compiled terms notes |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `AutoPaymentScheduleDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `id` | String | **yes** | — |
| `autoPayment` | [`AutoPaymentDetailsDto`](#autopaymentdetailsdto) | **yes** | auto-payment configuration |

### `BacsDirectDebitDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `sort_code` | String | **yes** | — |
| `last4` | String | **yes** | — |

### `BecsDirectDebitDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `bsb_number` | String | **yes** | — |
| `last4` | String | **yes** | — |

### `BusinessDetails`

_No fields defined in the spec._

### `BusinessDetailsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `logoUrl` | String | no | Business Logo URL |
| `name` | String | no | Business Name |
| `phoneNo` | String | no | Business Phone Number |
| `address` | [`AddressDto`](#addressdto) | no | Business Address |
| `website` | String | no | Business Website Link |
| `customValues` | Vec<String> | no | Custom Values |

### `CancelInvoiceScheduleDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |

### `CancelInvoiceScheduleResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Schedule Id |
| `status` | JSON | **yes** | Schedule Status |
| `liveMode` | bool | **yes** | Live Mode |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | no | — |
| `invoices` | Vec<DefaultInvoiceResponseDto> | **yes** | List of invoices |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | Business Details |
| `currency` | String | **yes** | Currency |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact Details |
| `discount` | [`DiscountDto`](#discountdto) | no | Discount |
| `items` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `termsNotes` | String | **yes** | Terms notes |
| `compiledTermsNotes` | String | **yes** | Compiled terms notes |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `CardDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `brand` | String | **yes** | — |
| `last4` | String | **yes** | — |

### `ChequeDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `number` | String | **yes** | check number |

### `ContactDetails`

_No fields defined in the spec._

### `ContactDetailsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Contact ID |
| `name` | String | **yes** | Contact Name |
| `phoneNo` | String | **yes** | Contact Phone Number |
| `email` | String | **yes** | Contact Email |
| `additionalEmails` | Vec<AdditionalEmailsDto> | no | Secondary email addresses for the contact to be saved |
| `companyName` | String | no | Contact Company Name |
| `address` | [`AddressDto`](#addressdto) | no | — |
| `customFields` | Vec<String> | no | Custom Values |

### `CreateEstimatesDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Estimate Name |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | — |
| `currency` | String | **yes** | Currency code |
| `items` | Vec<EstimateLineItemDto> | **yes** | An array of items for the estimate. |
| `liveMode` | bool | no | livemode for estimate |
| `discount` | [`DiscountDto`](#discountdto) | **yes** | — |
| `termsNotes` | String | no | Terms notes, Also supports HTML markups |
| `title` | String | no | Title for the estimate |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact information to send the estimate to |
| `estimateNumber` | f64 | no | Estimate Number, if not specified will take in the next valid estimate number |
| `issueDate` | String | no | issue date estimate |
| `expiryDate` | String | no | expiry date estimate |
| `sentTo` | [`SentToDto`](#senttodto) | no | Email and sent to details for the estimate |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Estimate |
| `meta` | JSON | no | Meta data for the estimate |
| `sendEstimateDetails` | [`SendEstimateDto`](#sendestimatedto) | no | When sending estimate directly while saving |
| `frequencySettings` | [`FrequencySettingsDto`](#frequencysettingsdto) | **yes** | frequency settings for the estimate |
| `estimateNumberPrefix` | String | no | Prefix for the estimate number |
| `userId` | String | no | User Id |
| `attachments` | Vec<AttachmentsDto> | no | attachments for the invoice |
| `autoInvoice` | [`AutoInvoicingDto`](#autoinvoicingdto) | no | Auto invoice for the estimate |
| `miscellaneousCharges` | [`ProcessingFeeDto`](#processingfeedto) | no | miscellaneous charges for the estimate |
| `paymentScheduleConfig` | [`PaymentScheduleConfigDto`](#paymentscheduleconfigdto) | no | Payment Schedule Config for the estimate |

### `CreateInvoiceDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `name` | String | **yes** | Invoice Name |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | — |
| `currency` | String | **yes** | Currency code |
| `items` | Vec<InvoiceItemDto> | **yes** | An array of items for the invoice. |
| `discount` | [`DiscountDto`](#discountdto) | **yes** | — |
| `termsNotes` | String | no | Terms notes, Also supports HTML markups |
| `title` | String | no | Title for the invoice |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact information to send the invoice to |
| `invoiceNumber` | String | no | Invoice Number |
| `issueDate` | String | **yes** | Issue date in YYYY-MM-DD format |
| `dueDate` | String | no | Due date in YYYY-MM-DD format |
| `sentTo` | [`SentToDto`](#senttodto) | **yes** | — |
| `liveMode` | bool | **yes** | — |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `paymentSchedule` | [`PaymentScheduleDto`](#paymentscheduledto) | no | split invoice into payment schedule summing up to full invoice amount |
| `lateFeesConfiguration` | [`LateFeesConfigurationDto`](#latefeesconfigurationdto) | no | late fees configuration |
| `tipsConfiguration` | [`TipsConfigurationDto`](#tipsconfigurationdto) | no | tips configuration for the invoice |
| `invoiceNumberPrefix` | String | no | prefix for invoice number |
| `paymentMethods` | [`PaymentMethodDto`](#paymentmethoddto) | no | Payment Methods for Invoices |
| `attachments` | Vec<AttachmentsDto> | no | attachments for the invoice |
| `miscellaneousCharges` | [`ProcessingFeeDto`](#processingfeedto) | no | miscellaneous charges for the invoice |

### `CreateInvoiceFromEstimateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `markAsInvoiced` | bool | **yes** | Mark Estimate as Invoiced |
| `version` | String — `v1`, `v2` | no | Version of the update request |

### `CreateInvoiceFromEstimateResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `estimate` | [`EstimateResponseDto`](#estimateresponsedto) | **yes** | Estimate details |
| `invoice` | [`DefaultInvoiceResponseDto`](#defaultinvoiceresponsedto) | **yes** | Invoice details |

### `CreateInvoiceResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Invoice Id |
| `status` | String — `draft`, `sent`, `payment_processing`, `paid`, `void`, `partially_paid` | **yes** | Invoice Status |
| `liveMode` | bool | **yes** | Live Mode |
| `amountPaid` | f64 | **yes** | Amount Paid |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `businessDetails` | JSON | **yes** | Business Details |
| `invoiceNumber` | f64 | **yes** | Invoice Number |
| `currency` | String | **yes** | Currency |
| `contactDetails` | JSON | **yes** | Contact Details |
| `issueDate` | String | **yes** | Issue date in YYYY-MM-DD format |
| `dueDate` | String | **yes** | Due date in YYYY-MM-DD format |
| `discount` | JSON | no | Discount |
| `invoiceItems` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `amountDue` | f64 | **yes** | Total Amount Due |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `automaticTaxesCalculated` | bool | no | Is Automatic taxes calculated for the Invoice items |
| `paymentSchedule` | JSON | no | split invoice into payment schedule summing up to full invoice amount |

### `CreateInvoiceScheduleDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `name` | String | **yes** | — |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | — |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | **yes** | — |
| `liveMode` | bool | **yes** | — |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | — |
| `currency` | String | **yes** | — |
| `items` | Vec<InvoiceItemDto> | **yes** | — |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `discount` | [`DiscountDto`](#discountdto) | **yes** | — |
| `termsNotes` | String | no | — |
| `title` | String | no | — |
| `tipsConfiguration` | [`TipsConfigurationDto`](#tipsconfigurationdto) | no | Configuration for tips on invoices |
| `lateFeesConfiguration` | [`LateFeesConfigurationDto`](#latefeesconfigurationdto) | no | Late fees configuration for the invoices |
| `invoiceNumberPrefix` | String | no | prefix for invoice number |
| `paymentMethods` | [`PaymentMethodDto`](#paymentmethoddto) | no | Payment Methods for Invoices |
| `attachments` | Vec<AttachmentsDto> | no | attachments for the invoice |
| `miscellaneousCharges` | [`ProcessingFeeDto`](#processingfeedto) | no | miscellaneous charges for the invoice |

### `CreateInvoiceScheduleResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Schedule Id |
| `status` | JSON | **yes** | Schedule Status |
| `liveMode` | bool | **yes** | Live Mode |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | no | — |
| `invoices` | Vec<DefaultInvoiceResponseDto> | **yes** | List of invoices |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | Business Details |
| `currency` | String | **yes** | Currency |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact Details |
| `discount` | [`DiscountDto`](#discountdto) | no | Discount |
| `items` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `termsNotes` | String | **yes** | Terms notes |
| `compiledTermsNotes` | String | **yes** | Compiled terms notes |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `CreateInvoiceTemplateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `internal` | bool | no | — |
| `name` | String | **yes** | Name of the template |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | — |
| `currency` | String | **yes** | — |
| `items` | Vec<InvoiceItemDto> | **yes** | — |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `discount` | [`DiscountDto`](#discountdto) | no | — |
| `termsNotes` | String | no | — |
| `title` | String | no | Template title |
| `tipsConfiguration` | [`TipsConfigurationDto`](#tipsconfigurationdto) | no | Configuration for tips on invoices |
| `lateFeesConfiguration` | [`LateFeesConfigurationDto`](#latefeesconfigurationdto) | no | Late fees configuration for the invoices |
| `invoiceNumberPrefix` | String | no | prefix for invoice number |
| `paymentMethods` | [`PaymentMethodDto`](#paymentmethoddto) | no | Payment Methods for Invoices |
| `attachments` | Vec<String> | no | attachments for the invoice |
| `miscellaneousCharges` | [`ProcessingFeeDto`](#processingfeedto) | no | miscellaneous charges for the invoice |

### `CreateInvoiceTemplateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Template Id |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the Template |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | Business Details |
| `currency` | String | **yes** | Currency |
| `discount` | [`DiscountDto`](#discountdto) | no | Discount |
| `items` | Vec<String> | **yes** | Invoice Items |
| `invoiceNumberPrefix` | String | no | prefix for invoice number |
| `total` | f64 | **yes** | Total Amount |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `CustomNotificationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `customerSendInvoice` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `teamPaymentSuccess` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `customerPaymentSuccess` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `teamAutoPaymentSuccess` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `customerAutoPaymentSuccess` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `teamPaymentFailure` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `customerPaymentFailure` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `teamAutoPaymentFailure` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `customerAutoPaymentFailure` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `customerAutoPaymentInfo` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `customerAutoPaymentAmountChanged` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `teamAutoPaymentSkip` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `teamRecurringSendInvoiceFailed` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `customerSendEstimate` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `teamEstimateAccepted` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |
| `teamEstimateDeclined` | [`CustomNotificationItemDto`](#customnotificationitemdto) | **yes** | — |

### `CustomNotificationItemDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | Flag indicating if the notification is enabled or not |
| `emailTemplate` | String | **yes** | Template to be used for sending email |
| `smsTemplate` | String | **yes** | Template to be used for sending sms |
| `fromName` | String | no | Name to be used while sending email |
| `fromEmail` | String | no | Email address to be used for sending email |
| `emailSubject` | String | no | Subject of email which is sent out |
| `defaultEmailTemplateId` | String | no | Default email TemplateId to be used for sending via email |

### `CustomRRuleOptionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `intervalType` | String — `yearly`, `monthly`, `weekly`, `daily`, `hourly`, `minutely`, `secondly` | **yes** | — |
| `interval` | f64 | **yes** | — |
| `startDate` | String | **yes** | Start date in YYYY-MM-DD format |
| `startTime` | String | no | Start time in HH:mm:ss format |
| `endDate` | String | no | End date in YYYY-MM-DD format |
| `endTime` | String | no | End time in HH:mm:ss format |
| `dayOfMonth` | f64 | no | -1, 1, 2, 3, ..., 27, 28 |
| `dayOfWeek` | String — `mo`, `tu`, `we`, `th`, `fr`, `sa`, `su` | no | — |
| `numOfWeek` | f64 | no | -1, 1, 2, 3, 4 |
| `monthOfYear` | String — `jan`, `feb`, `mar`, `apr`, `may`, `jun`, `jul`, `aug`, `sep`, `oct`, `nov`, `dec` | no | — |
| `count` | f64 | no | Max number of task executions |
| `daysBefore` | f64 | no | Execute task number of days before |
| `useStartAsPrimaryUserAccepted` | bool | no | Start as primary user accepted date |
| `endType` | String | no | End type like after, by, count |

### `DefaultInvoiceResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Invoice Id |
| `status` | String — `draft`, `sent`, `payment_processing`, `paid`, `void`, `partially_paid` | **yes** | Invoice Status |
| `liveMode` | bool | **yes** | Live Mode |
| `amountPaid` | f64 | **yes** | Amount Paid |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `businessDetails` | JSON | **yes** | Business Details |
| `invoiceNumber` | f64 | **yes** | Invoice Number |
| `currency` | String | **yes** | Currency |
| `contactDetails` | JSON | **yes** | Contact Details |
| `issueDate` | String | **yes** | Issue date in YYYY-MM-DD format |
| `dueDate` | String | **yes** | Due date in YYYY-MM-DD format |
| `discount` | JSON | no | Discount |
| `invoiceItems` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `amountDue` | f64 | **yes** | Total Amount Due |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `automaticTaxesCalculated` | bool | no | Is Automatic taxes calculated for the Invoice items |
| `paymentSchedule` | JSON | no | split invoice into payment schedule summing up to full invoice amount |

### `DeleteInvoiceResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Invoice Id |
| `status` | String — `draft`, `sent`, `payment_processing`, `paid`, `void`, `partially_paid` | **yes** | Invoice Status |
| `liveMode` | bool | **yes** | Live Mode |
| `amountPaid` | f64 | **yes** | Amount Paid |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `businessDetails` | JSON | **yes** | Business Details |
| `invoiceNumber` | f64 | **yes** | Invoice Number |
| `currency` | String | **yes** | Currency |
| `contactDetails` | JSON | **yes** | Contact Details |
| `issueDate` | String | **yes** | Issue date in YYYY-MM-DD format |
| `dueDate` | String | **yes** | Due date in YYYY-MM-DD format |
| `discount` | JSON | no | Discount |
| `invoiceItems` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `amountDue` | f64 | **yes** | Total Amount Due |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `automaticTaxesCalculated` | bool | no | Is Automatic taxes calculated for the Invoice items |
| `paymentSchedule` | JSON | no | split invoice into payment schedule summing up to full invoice amount |

### `DeleteInvoiceScheduleResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | success |

### `DeleteInvoiceTemplateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | success |

### `DiscountDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `value` | f64 | no | Discount Value |
| `type` | String — `percentage`, `fixed` | **yes** | Discount type |
| `validOnProductIds` | Vec<String> | no | Product Ids on which discount is applicable |

### `EstimateIdParam`

| Field | Type | Required | Description |
|---|---|---|---|
| `estimateId` | String | **yes** | Estimate Id |

### `EstimateLineItemDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Invoice Item Name |
| `description` | String | no | Invoice descriptions |
| `productId` | String | no | Product Id |
| `priceId` | String | no | Price Id |
| `currency` | String | **yes** | Currency |
| `amount` | f64 | **yes** | Product amount |
| `qty` | f64 | **yes** | Product Quantity |
| `taxes` | Vec<ItemTaxDto> | no | Tax |
| `automaticTaxCategoryId` | String | no | Tax category id for calculating automatic tax |
| `isSetupFeeItem` | bool | no | Setupfee item, only created when 1st invoice of recurring schedule is generated |
| `type` | String — `one_time`, `recurring` | no | Price type of the item |
| `taxInclusive` | bool | no | true if item amount is tax inclusive |
| `attachments` | Vec<String> | no | Attachments for the line item |

### `EstimateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `_id` | String | **yes** | Unique identifier |
| `liveMode` | bool | **yes** | Indicates if it is in live mode |
| `deleted` | bool | **yes** | Indicates if deleted |
| `name` | String | **yes** | Name |
| `currency` | String | **yes** | Currency code |
| `businessDetails` | [`BusinessDetails`](#businessdetails) | **yes** | Business details associated with the estimate |
| `items` | Vec<Vec<JSON>> | **yes** | An array of items |
| `discount` | [`DiscountDto`](#discountdto) | **yes** | Discount details for the estimate template |
| `title` | String | no | Title |
| `estimateNumberPrefix` | String | no | Estimate number prefix |
| `attachments` | Vec<AttachmentsDto> | no | Attachments |
| `updatedBy` | String | no | User Id of who last updated |
| `total` | f64 | **yes** | Total amount |
| `createdAt` | String | **yes** | Timestamp when created |
| `updatedAt` | String | **yes** | Timestamp when last updated |
| `__v` | f64 | **yes** | Version number |
| `automaticTaxesEnabled` | bool | **yes** | Indicates if automatic taxes are enabled for this estimate |
| `termsNotes` | String | no | Terms and conditions for the estimate, supports HTML markup |
| `companyId` | String | **yes** | Company identifier associated with the estimate |
| `contactDetails` | [`ContactDetails`](#contactdetails) | **yes** | Contact details for the estimate |
| `issueDate` | String | **yes** | Date when the estimate was issued |
| `expiryDate` | String | **yes** | Date when the estimate expires |
| `sentBy` | String | no | User who sent the estimate |
| `automaticTaxesCalculated` | bool | **yes** | Indicates if automatic taxes were calculated |
| `meta` | JSON | **yes** | Additional metadata associated with the estimate |
| `estimateActionHistory` | Vec<String> | **yes** | History of actions taken on the estimate |
| `sentTo` | [`SentTo`](#sentto) | **yes** | Recipient details for the estimate |
| `frequencySettings` | [`FrequencySettingsDto`](#frequencysettingsdto) | **yes** | Frequency settings for recurring estimates |
| `lastVisitedAt` | String | **yes** | Timestamp when the estimate was last visited |
| `totalamountInUSD` | f64 | **yes** | Total amount in USD |
| `autoInvoice` | [`AutoInvoice`](#autoinvoice) | no | Auto-invoice settings for the estimate |
| `traceId` | String | **yes** | Trace ID for logging and debugging |

### `EstimateTemplateResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `_id` | String | **yes** | Unique identifier |
| `liveMode` | bool | **yes** | Indicates if it is in live mode |
| `deleted` | bool | **yes** | Indicates if deleted |
| `name` | String | **yes** | Name |
| `currency` | String | **yes** | Currency code |
| `businessDetails` | [`BusinessDetails`](#businessdetails) | **yes** | Business details associated with the estimate |
| `items` | Vec<Vec<JSON>> | **yes** | An array of items |
| `discount` | [`DiscountDto`](#discountdto) | **yes** | Discount details for the estimate template |
| `title` | String | no | Title |
| `estimateNumberPrefix` | String | no | Estimate number prefix |
| `attachments` | Vec<AttachmentsDto> | no | Attachments |
| `updatedBy` | String | no | User Id of who last updated |
| `total` | f64 | **yes** | Total amount |
| `createdAt` | String | **yes** | Timestamp when created |
| `updatedAt` | String | **yes** | Timestamp when last updated |
| `__v` | f64 | **yes** | Version number |
| `automaticTaxesEnabled` | bool | **yes** | Indicates if automatic taxes are enabled for this estimate |
| `termsNotes` | String | no | Terms and conditions for the estimate, supports HTML markup |

### `EstimateTemplatesDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Estimate Name |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | — |
| `currency` | String | **yes** | Currency code |
| `items` | Vec<Vec<JSON>> | **yes** | An array of items for the estimate. |
| `liveMode` | bool | no | livemode for estimate |
| `discount` | [`DiscountDto`](#discountdto) | **yes** | — |
| `termsNotes` | String | no | Terms notes, Also supports HTML markups |
| `title` | String | no | Title for the estimate |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Estimate |
| `meta` | JSON | no | Meta data for the estimate |
| `sendEstimateDetails` | [`SendEstimateDto`](#sendestimatedto) | no | When sending estimate directly while saving |
| `estimateNumberPrefix` | String | no | Prefix for the estimate number |
| `attachments` | Vec<AttachmentsDto> | no | attachments for the invoice |
| `miscellaneousCharges` | [`ProcessingFeeDto`](#processingfeedto) | no | miscellaneous charges for the estimate |

### `FrequencySettingsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | enabled for the frequency settings |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | **yes** | schedule setting for the estimate |

### `GenerateEstimateNumberResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `estimateNumber` | f64 | **yes** | — |
| `traceId` | String | **yes** | — |

### `GenerateInvoiceNumberResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `invoiceNumber` | f64 | no | Invoice Number |

### `GetInvoiceResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Invoice Id |
| `status` | String — `draft`, `sent`, `payment_processing`, `paid`, `void`, `partially_paid` | **yes** | Invoice Status |
| `liveMode` | bool | **yes** | Live Mode |
| `amountPaid` | f64 | **yes** | Amount Paid |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `businessDetails` | JSON | **yes** | Business Details |
| `invoiceNumber` | f64 | **yes** | Invoice Number |
| `currency` | String | **yes** | Currency |
| `contactDetails` | JSON | **yes** | Contact Details |
| `issueDate` | String | **yes** | Issue date in YYYY-MM-DD format |
| `dueDate` | String | **yes** | Due date in YYYY-MM-DD format |
| `discount` | JSON | no | Discount |
| `invoiceItems` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `amountDue` | f64 | **yes** | Total Amount Due |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `automaticTaxesCalculated` | bool | no | Is Automatic taxes calculated for the Invoice items |
| `paymentSchedule` | JSON | no | split invoice into payment schedule summing up to full invoice amount |
| `totalSummary` | [`TotalSummaryDto`](#totalsummarydto) | **yes** | — |
| `remindersConfiguration` | [`RemindersConfigurationDto`](#remindersconfigurationdto) | no | Reminders Configuration |

### `GetInvoiceSettingsResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | no | Sub-Account Id |
| `altType` | String — `location` | no | Alt Type |
| `termsNote` | String | no | Terms and conditions for invoices |
| `estimatesTermsNote` | String | no | Terms and conditions for estimates |
| `title` | String | no | Title for invoices |
| `estimatesTitle` | String | no | Title for estimates |
| `invoiceNumberPrefix` | String | no | Prefix for invoice numbers |
| `estimateNumberPrefix` | String | no | Prefix for estimate numbers |
| `dueAfterXDays` | f64 | no | Number of days after which invoice is due |
| `estimatesExpireAfterXDays` | f64 | no | Number of days after which estimate expires |
| `minimumPercentagePartialPayment` | f64 | no | Minimum percentage for partial payment |
| `customFields` | Vec<String> | no | Custom fields array |
| `customNotification` | [`CustomNotificationDto`](#customnotificationdto) | no | Custom notification settings |
| `businessDetails` | [`InvoiceSettingsBusinessDetailsDto`](#invoicesettingsbusinessdetailsdto) | no | Business details |
| `senderConfiguration` | [`InvoiceSettingsSenderConfigurationDto`](#invoicesettingssenderconfigurationdto) | no | Sender configuration |
| `productSettings` | [`InvoiceProductSettingsDto`](#invoiceproductsettingsdto) | no | Product settings |
| `reminderSettings` | [`ReminderSettingsDto`](#remindersettingsdto) | no | Reminder settings |
| `lateFeesConfiguration` | [`LateFeesConfigurationDto`](#latefeesconfigurationdto) | no | Late fees configuration |
| `tipsConfiguration` | [`TipsConfigurationDto`](#tipsconfigurationdto) | no | Tips configuration |
| `paymentMethods` | [`PaymentMethodDto`](#paymentmethoddto) | no | Payment methods configuration |

### `GetScheduleResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Schedule Id |
| `status` | JSON | **yes** | Schedule Status |
| `liveMode` | bool | **yes** | Live Mode |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | no | — |
| `invoices` | Vec<DefaultInvoiceResponseDto> | **yes** | List of invoices |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | Business Details |
| `currency` | String | **yes** | Currency |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact Details |
| `discount` | [`DiscountDto`](#discountdto) | no | Discount |
| `items` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `termsNotes` | String | **yes** | Terms notes |
| `compiledTermsNotes` | String | **yes** | Compiled terms notes |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `GetTemplateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Template Id |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the Template |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | Business Details |
| `currency` | String | **yes** | Currency |
| `discount` | [`DiscountDto`](#discountdto) | no | Discount |
| `items` | Vec<String> | **yes** | Invoice Items |
| `invoiceNumberPrefix` | String | no | prefix for invoice number |
| `total` | f64 | **yes** | Total Amount |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `InvoiceItemDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Invoice Item Name |
| `description` | String | no | Invoice descriptions |
| `productId` | String | no | Product Id |
| `priceId` | String | no | Price Id |
| `currency` | String | **yes** | Currency |
| `amount` | f64 | **yes** | Product amount |
| `qty` | f64 | **yes** | Product Quantity |
| `taxes` | Vec<ItemTaxDto> | no | Tax |
| `automaticTaxCategoryId` | String | no | Tax category id for calculating automatic tax |
| `isSetupFeeItem` | bool | no | Setupfee item, only created when 1st invoice of recurring schedule is generated |
| `type` | String — `one_time`, `recurring` | no | Price type of the item |
| `taxInclusive` | bool | no | true if item amount is tax inclusive |

### `InvoiceProductSettingsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enableImportProductDescription` | bool | no | Flag indicating if the product description import is enabled or not |
| `descriptionOptional` | bool | no | Flag indicating if the product description is optional or not |

### `InvoiceSettingsBusinessDetailsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `logoUrl` | String | no | — |
| `name` | String | **yes** | — |
| `phoneNo` | String | no | — |
| `address` | [`Address`](#address) | no | — |
| `website` | String | no | — |
| `customValues` | Vec<String> | no | — |

### `InvoiceSettingsSenderConfigurationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `fromName` | String | no | Sender name to be used while sending email notification |
| `fromEmail` | String | no | Email id to be used while sending email notification |

### `ItemTaxDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | — |
| `name` | String | **yes** | — |
| `rate` | f64 | **yes** | — |
| `calculation` | String — `exclusive` | no | — |
| `description` | String | no | — |
| `taxId` | String | no | — |

### `LateFeesConfigurationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enable` | bool | **yes** | Enable late fees |
| `value` | f64 | **yes** | Late Fees Value |
| `type` | String — `fixed`, `percentage` | **yes** | Late Fees Type |
| `frequency` | [`LateFeesFrequencyDto`](#latefeesfrequencydto) | **yes** | Late Fees Frequency |
| `grace` | [`LateFeesGraceDto`](#latefeesgracedto) | no | Late Fees Grace |
| `maxLateFees` | [`LateFeesMaxFeesDto`](#latefeesmaxfeesdto) | no | Max late fees payable |

### `LateFeesFrequencyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `intervalCount` | f64 | **yes** | Late fees interval count |
| `interval` | String — `minute`, `hour`, `day`, `week`, `month`, `one_time` | **yes** | Late fees interval |

### `LateFeesGraceDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `intervalCount` | f64 | **yes** | Late fees grace interval count |
| `interval` | String — `day` | **yes** | Late fees grace interval |

### `LateFeesMaxFeesDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `fixed` | **yes** | — |
| `value` | f64 | **yes** | Max late fees to pay |

### `ListEstimateTemplateResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<String> | **yes** | List of estimate templates |
| `totalCount` | f64 | **yes** | Total number of estimate templates available |
| `traceId` | String | **yes** | Unique identifier for tracing the request |

### `ListEstimatesResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `estimates` | Vec<String> | **yes** | List of estimates |
| `total` | f64 | **yes** | Total number of estimates |
| `traceId` | String | **yes** | Unique identifier for tracing the request |

### `ListInvoicesResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `invoices` | Vec<GetInvoiceResponseDto> | **yes** | — |
| `total` | f64 | **yes** | Total number of invoices |

### `ListSchedulesResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `schedules` | Vec<GetScheduleResponseDto> | **yes** | — |
| `total` | f64 | **yes** | Total number of Schedules |

### `ListTemplatesResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<GetTemplateResponseDto> | **yes** | — |
| `totalCount` | f64 | **yes** | Total number of Templates |

### `OldCreateInvoiceDTO`

_No fields defined in the spec._

### `PatchInvoiceStatsLastViewedDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `invoiceId` | String | **yes** | Invoice Id |

### `PaymentMethodDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `stripe` | [`StripePaymentMethodDto`](#stripepaymentmethoddto) | **yes** | Payment Method |

### `PaymentScheduleConfigDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `fixed`, `percentage` | **yes** | Payment Schedule Type |
| `dateConfig` | [`PaymentScheduleDateConfigDto`](#paymentscheduledateconfigdto) | **yes** | Due date type configuration |
| `schedules` | Vec<Vec<JSON>> | **yes** | Payment Schedule Items |

### `PaymentScheduleDateConfigDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `depositDateType` | String — `estimate_accepted`, `custom` | **yes** | Deposit date type |
| `scheduleDateType` | String — `regular_interval`, `custom` | **yes** | Payment Schedule Date Type |

### `PaymentScheduleDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `fixed`, `percentage` | **yes** | Payment schedule type |
| `schedules` | Vec<String> | **yes** | payment schedule item |

### `ProcessingFeeDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `charges` | Vec<Vec<JSON>> | **yes** | charges for the processing fee |
| `collectedMiscellaneousCharges` | f64 | no | collected miscellaneous charges |
| `paidCharges` | Vec<ProcessingFeePaidChargeDto> | no | paid miscellaneous charges |

### `ProcessingFeePaidChargeDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | name of the processing fee |
| `charge` | f64 | **yes** | charge for the processing fee |
| `amount` | f64 | **yes** | amount of the processing fee |
| `_id` | String | **yes** | id of the processing fee |

### `RecordPaymentDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `mode` | String — `cash`, `card`, `cheque`, `bank_transfer`, `other` | **yes** | manual payment method |
| `card` | [`CardDto`](#carddto) | **yes** | — |
| `cheque` | [`ChequeDto`](#chequedto) | **yes** | — |
| `notes` | String | **yes** | Any note to be recorded with the transaction |
| `amount` | f64 | no | Amount to be paid against the invoice. |
| `meta` | JSON | no | — |
| `paymentScheduleIds` | Vec<String> | no | Payment Schedule Ids to be recorded against the invoice. |
| `fulfilledAt` | String | no | Updated At to be recorded against the invoice. |

### `RecordPaymentResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | status |
| `invoice` | [`DefaultInvoiceResponseDto`](#defaultinvoiceresponsedto) | **yes** | — |

### `ReminderDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | Flag indicating if the reminder is enabled or not |
| `emailTemplate` | String | **yes** | Email template to be used for sending reminders |
| `smsTemplate` | String | **yes** | SMS template to be used for sending reminders |
| `emailSubject` | String | **yes** | Subject of the reminder |
| `reminderId` | String | **yes** | Unique identifier for the reminder |
| `reminderName` | String | **yes** | Name of the reminder |
| `reminderTime` | String — `before`, `after` | **yes** | Time condition for the reminder |
| `intervalType` | String — `yearly`, `monthly`, `weekly`, `daily`, `hourly`, `minutely`, `secondly` | **yes** | Interval type for the reminder |
| `maxReminders` | f64 | **yes** | Maximum number of reminders that can be sent |
| `reminderInvoiceCondition` | String — `invoice_sent`, `invoice_overdue` | **yes** | Condition for sending the reminder |
| `reminderNumber` | f64 | **yes** | frequency gap of the reminder to exeucte |
| `startTime` | String | no | Business Hour Start Time |
| `endTime` | String | no | Business Hour End Time |
| `timezone` | String | no | Timezone at which reminder will be sent |

### `ReminderExecutionDetailsList`

_No fields defined in the spec._

### `ReminderSettingsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `defaultEmailTemplateId` | String | **yes** | default template Id of reminder |
| `reminders` | Vec<ReminderDto> | **yes** | List of reminders |

### `RemindersConfigurationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `reminderExecutionDetailsList` | [`ReminderExecutionDetailsList`](#reminderexecutiondetailslist) | **yes** | List of reminders |
| `reminderSettings` | [`ReminderSettingsDto`](#remindersettingsdto) | **yes** | Reminder settings |

### `ScheduleInvoiceScheduleDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `liveMode` | bool | **yes** | — |
| `autoPayment` | [`AutoPaymentDetailsDto`](#autopaymentdetailsdto) | no | auto-payment configuration |

### `ScheduleInvoiceScheduleResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Schedule Id |
| `status` | JSON | **yes** | Schedule Status |
| `liveMode` | bool | **yes** | Live Mode |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | no | — |
| `invoices` | Vec<DefaultInvoiceResponseDto> | **yes** | List of invoices |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | Business Details |
| `currency` | String | **yes** | Currency |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact Details |
| `discount` | [`DiscountDto`](#discountdto) | no | Discount |
| `items` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `termsNotes` | String | **yes** | Terms notes |
| `compiledTermsNotes` | String | **yes** | Compiled terms notes |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `ScheduleOptionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `executeAt` | String | no | — |
| `rrule` | [`CustomRRuleOptionsDto`](#customrruleoptionsdto) | no | — |

### `SendEstimateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `action` | String — `sms_and_email`, `send_manually`, `email`, `sms` | **yes** | — |
| `liveMode` | bool | **yes** | livemode for estimate |
| `userId` | String | **yes** | Please ensure that the UserId corresponds to an authorized personnel, either by an employee ID or agency ID, to access this location. This account will serve as the primary channel for all future comm… |
| `sentFrom` | [`InvoiceSettingsSenderConfigurationDto`](#invoicesettingssenderconfigurationdto) | no | sender details for invoice, valid only if invoice is not sent manually |
| `estimateName` | String | no | estimate name |

### `SendInvoiceDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `userId` | String | **yes** | Please ensure that the UserId corresponds to an authorized personnel, either by an employee ID or agency ID, to access this location. This account will serve as the primary channel for all future comm… |
| `action` | String — `sms_and_email`, `send_manually`, `email`, `sms` | **yes** | — |
| `liveMode` | bool | **yes** | — |
| `sentFrom` | [`InvoiceSettingsSenderConfigurationDto`](#invoicesettingssenderconfigurationdto) | no | sender details for invoice, valid only if invoice is not sent manually |
| `autoPayment` | [`AutoPaymentDetailsDto`](#autopaymentdetailsdto) | no | auto-payment configuration |

### `SendInvoicesResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `invoice` | [`DefaultInvoiceResponseDto`](#defaultinvoiceresponsedto) | **yes** | — |
| `smsData` | JSON | **yes** | — |
| `emailData` | JSON | **yes** | — |

### `SentTo`

_No fields defined in the spec._

### `SentToDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `email` | Vec<String> | **yes** | Email Address |
| `emailCc` | Vec<String> | no | cc to be kept in any sent out emails |
| `emailBcc` | Vec<String> | no | bcc to be kept in any sent out emails |
| `phoneNo` | Vec<String> | no | Contact Phone Number |

### `SepaDirectDebitDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `bank_code` | String | **yes** | — |
| `last4` | String | **yes** | — |
| `branch_code` | String | **yes** | — |

### `StripePaymentMethodDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enableBankDebitOnly` | bool | **yes** | Enable Bank Debit Only |

### `Text2PayDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `name` | String | **yes** | Invoice Name |
| `currency` | String | **yes** | Currency code |
| `items` | Vec<InvoiceItemDto> | **yes** | An array of items for the invoice. |
| `termsNotes` | String | no | Terms notes, Also supports HTML markups |
| `title` | String | no | Title for the invoice |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact information to send the invoice to |
| `invoiceNumber` | String | no | Invoice Number |
| `issueDate` | String | **yes** | Issue date in YYYY-MM-DD format |
| `dueDate` | String | no | Due date in YYYY-MM-DD format |
| `sentTo` | [`SentToDto`](#senttodto) | **yes** | — |
| `liveMode` | bool | **yes** | — |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `paymentSchedule` | [`PaymentScheduleDto`](#paymentscheduledto) | no | split invoice into payment schedule summing up to full invoice amount |
| `lateFeesConfiguration` | [`LateFeesConfigurationDto`](#latefeesconfigurationdto) | no | late fees configuration |
| `tipsConfiguration` | [`TipsConfigurationDto`](#tipsconfigurationdto) | no | tips configuration for the invoice |
| `invoiceNumberPrefix` | String | no | prefix for invoice number |
| `paymentMethods` | [`PaymentMethodDto`](#paymentmethoddto) | no | Payment Methods for Invoices |
| `attachments` | Vec<AttachmentsDto> | no | attachments for the invoice |
| `miscellaneousCharges` | [`ProcessingFeeDto`](#processingfeedto) | no | miscellaneous charges for the invoice |
| `id` | String | no | id of invoice to update. If skipped, a new invoice will be created |
| `includeTermsNote` | bool | no | include terms & notes with receipts |
| `action` | String — `draft`, `send` | **yes** | create invoice in draft mode or send mode |
| `userId` | String | **yes** | id of user generating invoice |
| `discount` | [`DiscountDto`](#discountdto) | no | — |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | no | — |

### `Text2PayInvoiceResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `invoice` | [`DefaultInvoiceResponseDto`](#defaultinvoiceresponsedto) | **yes** | — |
| `invoiceUrl` | String | **yes** | preview url of generated invoice |

### `TipsConfigurationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `tipsPercentage` | Vec<String> | **yes** | Percentage of tips allowed |
| `tipsEnabled` | bool | **yes** | Tips enabled status |

### `TotalSummaryDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `subTotal` | f64 | **yes** | subTotal |
| `discount` | f64 | **yes** | discount |
| `tax` | f64 | **yes** | tax |

### `USBankAccountDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `bank_name` | String | **yes** | — |
| `last4` | String | **yes** | — |

### `UpdateAndScheduleInvoiceScheduleResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Schedule Id |
| `status` | JSON | **yes** | Schedule Status |
| `liveMode` | bool | **yes** | Live Mode |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | no | — |
| `invoices` | Vec<DefaultInvoiceResponseDto> | **yes** | List of invoices |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | Business Details |
| `currency` | String | **yes** | Currency |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact Details |
| `discount` | [`DiscountDto`](#discountdto) | no | Discount |
| `items` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `termsNotes` | String | **yes** | Terms notes |
| `compiledTermsNotes` | String | **yes** | Compiled terms notes |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `UpdateEstimateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Estimate Name |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | — |
| `currency` | String | **yes** | Currency code |
| `items` | Vec<EstimateLineItemDto> | **yes** | An array of items for the estimate. |
| `liveMode` | bool | no | livemode for estimate |
| `discount` | [`DiscountDto`](#discountdto) | **yes** | — |
| `termsNotes` | String | no | Terms notes, Also supports HTML markups |
| `title` | String | no | Title for the estimate |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact information to send the estimate to |
| `estimateNumber` | f64 | no | Estimate Number, if not specified will take in the next valid estimate number |
| `issueDate` | String | no | issue date estimate |
| `expiryDate` | String | no | expiry date estimate |
| `sentTo` | [`SentToDto`](#senttodto) | no | Email and sent to details for the estimate |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Estimate |
| `meta` | JSON | no | Meta data for the estimate |
| `sendEstimateDetails` | [`SendEstimateDto`](#sendestimatedto) | no | When sending estimate directly while saving |
| `frequencySettings` | [`FrequencySettingsDto`](#frequencysettingsdto) | **yes** | frequency settings for the estimate |
| `estimateNumberPrefix` | String | no | Prefix for the estimate number |
| `userId` | String | no | User Id |
| `attachments` | Vec<AttachmentsDto> | no | attachments for the invoice |
| `autoInvoice` | [`AutoInvoicingDto`](#autoinvoicingdto) | no | Auto invoice for the estimate |
| `miscellaneousCharges` | [`ProcessingFeeDto`](#processingfeedto) | no | miscellaneous charges for the estimate |
| `paymentScheduleConfig` | [`PaymentScheduleConfigDto`](#paymentscheduleconfigdto) | no | Payment Schedule Config for the estimate |
| `estimateStatus` | String — `all`, `draft`, `sent`, `accepted`, `declined`, `invoiced`, `viewed` | no | Estimate Status |

### `UpdateInvoiceDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `name` | String | **yes** | Name to be updated |
| `title` | String | no | Title for the invoice |
| `currency` | String | **yes** | Currency |
| `description` | String | no | Description |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | no | Business details which need to be updated |
| `invoiceNumber` | String | no | Invoice Number |
| `contactId` | String | no | Id of the contact which you need to send the invoice |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | no | — |
| `termsNotes` | String | no | Terms notes, Also supports HTML markups |
| `discount` | [`DiscountDto`](#discountdto) | no | — |
| `invoiceItems` | Vec<InvoiceItemDto> | **yes** | — |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `liveMode` | bool | no | Payment mode |
| `issueDate` | String | **yes** | Issue date in YYYY-MM-DD format |
| `dueDate` | String | **yes** | Due date in YYYY-MM-DD format |
| `paymentSchedule` | [`PaymentScheduleDto`](#paymentscheduledto) | no | split invoice into payment schedule summing up to full invoice amount |
| `tipsConfiguration` | [`TipsConfigurationDto`](#tipsconfigurationdto) | no | tips configuration for the invoice |
| `xeroDetails` | JSON | no | — |
| `invoiceNumberPrefix` | String | no | prefix for invoice number |
| `paymentMethods` | [`PaymentMethodDto`](#paymentmethoddto) | no | Payment Methods for Invoices |
| `attachments` | Vec<AttachmentsDto> | no | attachments for the invoice |
| `miscellaneousCharges` | [`ProcessingFeeDto`](#processingfeedto) | no | miscellaneous charges for the invoice |

### `UpdateInvoiceLateFeesConfigurationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `lateFeesConfiguration` | [`LateFeesConfigurationDto`](#latefeesconfigurationdto) | **yes** | late fees configuration |

### `UpdateInvoiceResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Invoice Id |
| `status` | String — `draft`, `sent`, `payment_processing`, `paid`, `void`, `partially_paid` | **yes** | Invoice Status |
| `liveMode` | bool | **yes** | Live Mode |
| `amountPaid` | f64 | **yes** | Amount Paid |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `businessDetails` | JSON | **yes** | Business Details |
| `invoiceNumber` | f64 | **yes** | Invoice Number |
| `currency` | String | **yes** | Currency |
| `contactDetails` | JSON | **yes** | Contact Details |
| `issueDate` | String | **yes** | Issue date in YYYY-MM-DD format |
| `dueDate` | String | **yes** | Due date in YYYY-MM-DD format |
| `discount` | JSON | no | Discount |
| `invoiceItems` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `amountDue` | f64 | **yes** | Total Amount Due |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `automaticTaxesCalculated` | bool | no | Is Automatic taxes calculated for the Invoice items |
| `paymentSchedule` | JSON | no | split invoice into payment schedule summing up to full invoice amount |

### `UpdateInvoiceScheduleDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `name` | String | **yes** | — |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | — |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | **yes** | — |
| `liveMode` | bool | **yes** | — |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | — |
| `currency` | String | **yes** | — |
| `items` | Vec<InvoiceItemDto> | **yes** | — |
| `discount` | [`DiscountDto`](#discountdto) | **yes** | — |
| `termsNotes` | String | no | — |
| `title` | String | no | — |
| `attachments` | Vec<AttachmentsDto> | no | attachments for the invoice |
| `miscellaneousCharges` | [`ProcessingFeeDto`](#processingfeedto) | no | miscellaneous charges for the invoice |

### `UpdateInvoiceScheduleResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Schedule Id |
| `status` | JSON | **yes** | Schedule Status |
| `liveMode` | bool | **yes** | Live Mode |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | no | — |
| `invoices` | Vec<DefaultInvoiceResponseDto> | **yes** | List of invoices |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | Business Details |
| `currency` | String | **yes** | Currency |
| `contactDetails` | [`ContactDetailsDto`](#contactdetailsdto) | **yes** | Contact Details |
| `discount` | [`DiscountDto`](#discountdto) | no | Discount |
| `items` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `termsNotes` | String | **yes** | Terms notes |
| `compiledTermsNotes` | String | **yes** | Compiled terms notes |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `UpdateInvoiceTemplateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `internal` | bool | no | — |
| `name` | String | **yes** | Name of the template |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | — |
| `currency` | String | **yes** | — |
| `items` | Vec<InvoiceItemDto> | **yes** | — |
| `discount` | [`DiscountDto`](#discountdto) | no | — |
| `termsNotes` | String | no | — |
| `title` | String | no | Template title |
| `miscellaneousCharges` | [`ProcessingFeeDto`](#processingfeedto) | no | miscellaneous charges for the invoice |

### `UpdateInvoiceTemplateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Template Id |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the Template |
| `businessDetails` | [`BusinessDetailsDto`](#businessdetailsdto) | **yes** | Business Details |
| `currency` | String | **yes** | Currency |
| `discount` | [`DiscountDto`](#discountdto) | no | Discount |
| `items` | Vec<String> | **yes** | Invoice Items |
| `invoiceNumberPrefix` | String | no | prefix for invoice number |
| `total` | f64 | **yes** | Total Amount |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `UpdatePaymentMethodsConfigurationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `paymentMethods` | [`PaymentMethodDto`](#paymentmethoddto) | no | Payment Methods for Invoices |

### `VoidInvoiceDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |

### `VoidInvoiceResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Invoice Id |
| `status` | String — `draft`, `sent`, `payment_processing`, `paid`, `void`, `partially_paid` | **yes** | Invoice Status |
| `liveMode` | bool | **yes** | Live Mode |
| `amountPaid` | f64 | **yes** | Amount Paid |
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the invoice |
| `businessDetails` | JSON | **yes** | Business Details |
| `invoiceNumber` | f64 | **yes** | Invoice Number |
| `currency` | String | **yes** | Currency |
| `contactDetails` | JSON | **yes** | Contact Details |
| `issueDate` | String | **yes** | Issue date in YYYY-MM-DD format |
| `dueDate` | String | **yes** | Due date in YYYY-MM-DD format |
| `discount` | JSON | no | Discount |
| `invoiceItems` | Vec<String> | **yes** | Invoice Items |
| `total` | f64 | **yes** | Total Amount |
| `title` | String | **yes** | Title |
| `amountDue` | f64 | **yes** | Total Amount Due |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |
| `automaticTaxesEnabled` | bool | no | Automatic taxes enabled for the Invoice |
| `automaticTaxesCalculated` | bool | no | Is Automatic taxes calculated for the Invoice items |
| `paymentSchedule` | JSON | no | split invoice into payment schedule summing up to full invoice amount |

