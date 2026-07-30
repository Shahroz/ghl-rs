# `emails`

**5** operations / **12** models in API v2 · **18** operations / **67** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `emails` cargo feature on `ghl-sdk`, then call any of the 5 generated methods on `ghl.emails()`:

```toml
ghl-sdk = { version = "0.4", features = ["emails"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/emails/builder` | Fetch email templates | `fetch_email_templates()` | `emails.get_emails_builder` |
| `POST` | `/emails/builder` | Create a new template | `create_a_new_template()` | `emails.post_emails_builder` |
| `POST` | `/emails/builder/data` | Update a template | `update_a_template()` | `emails.post_emails_builder_data` |
| `DELETE` | `/emails/builder/{locationId}/{templateId}` | Delete a template | `delete_a_template()` | `emails.delete_emails_builder_by_locationId_by_templateId` |
| `GET` | `/emails/schedule` | Get Campaigns | `get_campaigns()` | `emails.get_emails_schedule` |

### Endpoint details — v2

#### `GET /emails/builder`

**Fetch email templates**

Fetch email templates by location id

Operation id: `emails.get_emails_builder` · `Version: 2021-07-28` · Scopes: `emails/builder.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `limit` | string | no | — |
| `offset` | string | no | — |
| `search` | string | no | — |
| `sortByDate` | string | no | — |
| `archived` | string | no | — |
| `builderVersion` | enum: `1`, `2` | no | — |
| `name` | string | no | — |
| `parentId` | string | no | — |
| `originId` | string | no | — |
| `templatesOnly` | string | no | — |

*Response*: [`FetchBuilderSuccesfulResponseDto`](#fetchbuildersuccesfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::emails::FetchEmailTemplatesParams;

let params = FetchEmailTemplatesParams::new("locationId");
let out = ghl.emails().fetch_email_templates(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "emails.get_emails_builder",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /emails/builder`

**Create a new template**

Operation id: `emails.post_emails_builder` · `Version: 2021-07-28` · Scopes: `emails/builder.write`

*Request body*: [`CreateBuilderDto`](#createbuilderdto)

*Response*: [`CreateBuilderSuccesfulResponseDto`](#createbuildersuccesfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.emails().create_a_new_template(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "emails.post_emails_builder",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /emails/builder/data`

**Update a template**

Operation id: `emails.post_emails_builder_data` · `Version: 2021-07-28` · Scopes: `emails/builder.write`

*Request body*: [`SaveBuilderDataDto`](#savebuilderdatadto)

*Response*: [`BuilderUpdateSuccessfulDTO`](#builderupdatesuccessfuldto)

*Rust*:

```rust,ignore
let out = ghl.emails().update_a_template(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "emails.post_emails_builder_data",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /emails/builder/{locationId}/{templateId}`

**Delete a template**

Operation id: `emails.delete_emails_builder_by_locationId_by_templateId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `templateId` | string | **yes** | — |

*Response*: [`DeleteBuilderSuccesfulResponseDto`](#deletebuildersuccesfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.emails().delete_a_template(&locationId, &templateId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "emails.delete_emails_builder_by_locationId_by_templateId",
    "path_params": {
      "locationId": "<locationId>",
      "templateId": "<templateId>"
    }
  }
}
```

</details>

#### `GET /emails/schedule`

**Get Campaigns**

Operation id: `emails.get_emails_schedule` · `Version: 2021-07-28` · Scopes: `emails/schedule.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID to fetch campaigns from |
| `limit` | number | no | Maximum number of campaigns to return. Defaults to 10, maximum is 100 |
| `offset` | number | no | Number of campaigns to skip for pagination |
| `status` | enum: `active`, `pause`, `complete`, `cancelled`, `retry`, `draft`, `resend-scheduled` | no | Filter by schedule status |
| `emailStatus` | enum: `all`, `not-started`, `paused`, `cancelled`, `processing`, `resumed`, `next-drip`, `complete`, `success`, `error`, `waiting`, `queued`, `queueing`, `reading`, `scheduled` | no | Filter by email delivery status |
| `name` | string | no | Filter campaigns by name |
| `parentId` | string | no | Filter campaigns by parent folder ID |
| `limitedFields` | boolean | no | When true, returns only essential campaign fields like id, templateDataDownloadUrl, updatedAt, type, templateType, templateId, downloadUrl and isPlainText. When… |
| `archived` | boolean | no | Filter archived campaigns |
| `campaignsOnly` | boolean | no | Return only campaigns, excluding folders |
| `showStats` | boolean | no | When true, returns campaign statistics including delivered count, opened count, clicked count and revenue if available for the campaign. When false, returns cam… |

*Response*: [`ScheduleFetchSuccessfulDTO`](#schedulefetchsuccessfuldto)

*Rust*:

```rust,ignore
use ghl_sdk::services::emails::GetCampaignsParams;

let params = GetCampaignsParams::new("locationId");
let out = ghl.emails().get_campaigns(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "emails.get_emails_schedule",
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
| `GET` | `/emails/locations/{locationId}/campaigns/bulk-actions` | List Bulk Action Campaigns | `v3:emails.get_emails_locations_by_locationId_campaigns_bulk_actions` |
| `GET` | `/emails/locations/{locationId}/campaigns/bulk-actions/{campaignId}` | Get Bulk Action Campaign by ID | `v3:emails.get_emails_locations_by_locationId_campaigns_bulk_actions_by_campaignId` |
| `GET` | `/emails/locations/{locationId}/campaigns/emails` | List Email Campaigns | `v3:emails.get_emails_locations_by_locationId_campaigns_emails` |
| `POST` | `/emails/locations/{locationId}/campaigns/emails` | Create Email Campaign | `v3:emails.post_emails_locations_by_locationId_campaigns_emails` |
| `DELETE` | `/emails/locations/{locationId}/campaigns/emails/{campaignId}` | Delete Campaign | `v3:emails.delete_emails_locations_by_locationId_campaigns_emails_by_campaignId` |
| `GET` | `/emails/locations/{locationId}/campaigns/emails/{campaignId}` | Get Email Campaign by ID | `v3:emails.get_emails_locations_by_locationId_campaigns_emails_by_campaignId` |
| `PATCH` | `/emails/locations/{locationId}/campaigns/emails/{campaignId}` | Update Email Campaign | `v3:emails.patch_emails_locations_by_locationId_campaigns_emails_by_campaignId` |
| `POST` | `/emails/locations/{locationId}/campaigns/emails/{campaignId}/schedule` | Schedule Campaign | `v3:emails.post_emails_locations_by_locationId_campaigns_emails_by_campaignId_schedule` |
| `GET` | `/emails/locations/{locationId}/campaigns/stats/{source}/{sourceId}` | Get Campaign Statistics | `v3:emails.get_emails_locations_by_locationId_campaigns_stats_by_source_by_sourceId` |
| `GET` | `/emails/locations/{locationId}/campaigns/workflows` | List Workflow Campaigns | `v3:emails.get_emails_locations_by_locationId_campaigns_workflows` |
| `GET` | `/emails/locations/{locationId}/campaigns/workflows/{campaignId}` | Get Workflow Campaign by ID | `v3:emails.get_emails_locations_by_locationId_campaigns_workflows_by_campaignId` |
| `GET` | `/emails/locations/{locationId}/templates` | List templates | `v3:emails.get_emails_locations_by_locationId_templates` |
| `POST` | `/emails/locations/{locationId}/templates` | Create an email template | `v3:emails.post_emails_locations_by_locationId_templates` |
| `POST` | `/emails/locations/{locationId}/templates/folders` | Create a template folder | `v3:emails.post_emails_locations_by_locationId_templates_folders` |
| `POST` | `/emails/locations/{locationId}/templates/import` | Import an email template | `v3:emails.post_emails_locations_by_locationId_templates_import` |
| `DELETE` | `/emails/locations/{locationId}/templates/{templateId}` | Delete a template | `v3:emails.delete_emails_locations_by_locationId_templates_by_templateId` |
| `GET` | `/emails/locations/{locationId}/templates/{templateId}` | Get Email Template by ID | `v3:emails.get_emails_locations_by_locationId_templates_by_templateId` |
| `PATCH` | `/emails/locations/{locationId}/templates/{templateId}` | Update an email template | `v3:emails.patch_emails_locations_by_locationId_templates_by_templateId` |

### Endpoint details — v3

#### `GET /emails/locations/{locationId}/campaigns/bulk-actions`

**List Bulk Action Campaigns**

Get list of bulk action campaigns for a location

Operation id: `v3:emails.get_emails_locations_by_locationId_campaigns_bulk_actions` · `Version: v3` · Scopes: `emails/campaigns.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | number | no | Number of campaigns to return. Defaults to 10, minimum is 1, maximum is 20 |
| `offset` | number | no | Number of campaigns to skip for pagination. Defaults to 0, minimum is 0 |
| `search` | string | no | Search query to filter campaigns. |
| `dateFrom` | string | no | Filter by start date (ISO 8601 format) |
| `dateTo` | string | no | Filter by end date (ISO 8601 format) |
| `status` | enum: `processing`, `scheduled`, `paused`, `complete`, `cancelled` | no | Filter by status |

*Response*: [`ListBulkActionCampaignsPublicV2ResponseDto`](#listbulkactioncampaignspublicv2responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:emails.get_emails_locations_by_locationId_campaigns_bulk_actions",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /emails/locations/{locationId}/campaigns/bulk-actions/{campaignId}`

**Get Bulk Action Campaign by ID**

Get a single bulk action campaign by its ID

Operation id: `v3:emails.get_emails_locations_by_locationId_campaigns_bulk_actions_by_campaignId` · `Version: v3` · Scopes: `emails/campaigns.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `campaignId` | string | **yes** | Campaign ID |

*Response*: [`GetBulkActionCampaignPublicV2ResponseDto`](#getbulkactioncampaignpublicv2responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:emails.get_emails_locations_by_locationId_campaigns_bulk_actions_by_campaignId",
    "path_params": {
      "locationId": "<locationId>",
      "campaignId": "<campaignId>"
    }
  }
}
```

</details>

#### `GET /emails/locations/{locationId}/campaigns/emails`

**List Email Campaigns**

Get list of email campaigns for a location

Operation id: `v3:emails.get_emails_locations_by_locationId_campaigns_emails` · `Version: v3` · Scopes: `emails/campaigns.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | number | no | Number of campaigns to return. Defaults to 10, minimum is 1, maximum is 20 |
| `offset` | number | no | Number of campaigns to skip for pagination. Defaults to 0, minimum is 0 |
| `search` | string | no | Search text for campaign name |
| `status` | enum: `all`, `sent`, `failed`, `archived`, `draft`, `processing`, `scheduled`, `cancelled`, `paused` | no | Filter by campaign status |

*Response*: [`ListEmailCampaignsPublicV2ResponseDto`](#listemailcampaignspublicv2responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:emails.get_emails_locations_by_locationId_campaigns_emails",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /emails/locations/{locationId}/campaigns/emails`

**Create Email Campaign**

Create a new email campaign

Operation id: `v3:emails.post_emails_locations_by_locationId_campaigns_emails` · `Version: v3` · Scopes: `emails/campaigns.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Request body*: [`CreateEmailCampaignPublicV2BodyDto`](#createemailcampaignpublicv2bodydto)

*Response*: [`CreateEmailCampaignPublicV2ResponseDto`](#createemailcampaignpublicv2responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:emails.post_emails_locations_by_locationId_campaigns_emails",
    "path_params": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /emails/locations/{locationId}/campaigns/emails/{campaignId}`

**Delete Campaign**

Delete a campaign

Operation id: `v3:emails.delete_emails_locations_by_locationId_campaigns_emails_by_campaignId` · `Version: v3` · Scopes: `emails/campaigns.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `campaignId` | string | **yes** | Campaign ID |

*Response*: [`DeleteCampaignPublicV2ResponseDto`](#deletecampaignpublicv2responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:emails.delete_emails_locations_by_locationId_campaigns_emails_by_campaignId",
    "path_params": {
      "locationId": "<locationId>",
      "campaignId": "<campaignId>"
    }
  }
}
```

</details>

#### `GET /emails/locations/{locationId}/campaigns/emails/{campaignId}`

**Get Email Campaign by ID**

Get a single email campaign by its ID

Operation id: `v3:emails.get_emails_locations_by_locationId_campaigns_emails_by_campaignId` · `Version: v3` · Scopes: `emails/campaigns.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `campaignId` | string | **yes** | Campaign ID |

*Response*: [`GetEmailCampaignPublicV2ResponseDto`](#getemailcampaignpublicv2responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:emails.get_emails_locations_by_locationId_campaigns_emails_by_campaignId",
    "path_params": {
      "locationId": "<locationId>",
      "campaignId": "<campaignId>"
    }
  }
}
```

</details>

#### `PATCH /emails/locations/{locationId}/campaigns/emails/{campaignId}`

**Update Email Campaign**

Update an email campaign draft

Operation id: `v3:emails.patch_emails_locations_by_locationId_campaigns_emails_by_campaignId` · `Version: v3` · Scopes: `emails/campaigns.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `campaignId` | string | **yes** | Campaign ID |

*Request body*: [`UpdateEmailCampaignPublicV2BodyDto`](#updateemailcampaignpublicv2bodydto)

*Response*: [`UpdateEmailCampaignPublicV2ResponseDto`](#updateemailcampaignpublicv2responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:emails.patch_emails_locations_by_locationId_campaigns_emails_by_campaignId",
    "path_params": {
      "locationId": "<locationId>",
      "campaignId": "<campaignId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /emails/locations/{locationId}/campaigns/emails/{campaignId}/schedule`

**Schedule Campaign**

Schedule or start an email campaign. The campaign must be in draft, cancelled, or paused status.

Operation id: `v3:emails.post_emails_locations_by_locationId_campaigns_emails_by_campaignId_schedule` · `Version: v3` · Scopes: `emails/campaigns.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `campaignId` | string | **yes** | Campaign ID |

*Request body*: [`ScheduleCampaignPublicV2BodyDto`](#schedulecampaignpublicv2bodydto)

*Response*: [`ScheduleCampaignPublicV2ResponseDto`](#schedulecampaignpublicv2responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:emails.post_emails_locations_by_locationId_campaigns_emails_by_campaignId_schedule",
    "path_params": {
      "locationId": "<locationId>",
      "campaignId": "<campaignId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /emails/locations/{locationId}/campaigns/stats/{source}/{sourceId}`

**Get Campaign Statistics**

Get statistics for email campaigns, workflows, or bulk actions

Operation id: `v3:emails.get_emails_locations_by_locationId_campaigns_stats_by_source_by_sourceId` · `Version: v3` · Scopes: `emails/stats.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `source` | enum: `email-campaigns`, `workflow-campaigns`, `bulk-actions` | **yes** | Source type: email-campaigns, workflow-campaigns, or bulk-actions |
| `sourceId` | string | **yes** | Source ID of the email campaign, workflow campaign, or bulk action |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `subSourceId` | string | no | Workflow action ID. Only valid when source is `workflow-campaigns` |

*Response*: [`GetCampaignStatsPublicV2ResponseDto`](#getcampaignstatspublicv2responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:emails.get_emails_locations_by_locationId_campaigns_stats_by_source_by_sourceId",
    "path_params": {
      "locationId": "<locationId>",
      "source": "<source>",
      "sourceId": "<sourceId>"
    }
  }
}
```

</details>

#### `GET /emails/locations/{locationId}/campaigns/workflows`

**List Workflow Campaigns**

Get list of workflow campaigns for a location

Operation id: `v3:emails.get_emails_locations_by_locationId_campaigns_workflows` · `Version: v3` · Scopes: `emails/campaigns.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | number | no | Number of campaigns to return. Defaults to 10, minimum is 1, maximum is 20 |
| `offset` | number | no | Number of items to skip for pagination. Defaults to 0, minimum is 0 |
| `search` | string | no | Search query to filter campaigns. |
| `status` | enum: `published`, `draft` | no | Filter by campaign status |

*Response*: [`ListWorkflowCampaignsPublicV2ResponseDto`](#listworkflowcampaignspublicv2responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:emails.get_emails_locations_by_locationId_campaigns_workflows",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /emails/locations/{locationId}/campaigns/workflows/{campaignId}`

**Get Workflow Campaign by ID**

Get a single workflow campaign by its ID

Operation id: `v3:emails.get_emails_locations_by_locationId_campaigns_workflows_by_campaignId` · `Version: v3` · Scopes: `emails/campaigns.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `campaignId` | string | **yes** | Campaign ID |

*Response*: [`GetWorkflowCampaignPublicV2ResponseDto`](#getworkflowcampaignpublicv2responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:emails.get_emails_locations_by_locationId_campaigns_workflows_by_campaignId",
    "path_params": {
      "locationId": "<locationId>",
      "campaignId": "<campaignId>"
    }
  }
}
```

</details>

#### `GET /emails/locations/{locationId}/templates`

**List templates**

Get list of templates by location

Operation id: `v3:emails.get_emails_locations_by_locationId_templates` · `Version: v3` · Scopes: `emails/templates.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | number | no | Number of templates to return |
| `offset` | number | no | Number of templates to skip |
| `search` | string | no | Search by template name |
| `sortBy` | enum: `updatedAt` | no | Field to sort by |
| `sortOrder` | enum: `asc`, `desc` | no | Sort direction |
| `archived` | boolean | no | Return archived templates |
| `folderId` | string | no | Folder to list templates from. Use 'root' for top-level listing. |
| `include` | enum: `all`, `templates`, `folders` | no | Whether to include templates, folders, or both in the response. `templates` will return only templates, `folders` will return only folders, and `all` will retur… |

*Response*: [`ListTemplatesPublicV2ResponseDto`](#listtemplatespublicv2responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:emails.get_emails_locations_by_locationId_templates",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /emails/locations/{locationId}/templates`

**Create an email template**

Create a new email template

Operation id: `v3:emails.post_emails_locations_by_locationId_templates` · `Version: v3` · Scopes: `emails/templates.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Request body*: [`CreateTemplatePublicV2BodyDto`](#createtemplatepublicv2bodydto)

*Response*: [`CreateTemplatePublicV2ResponseDto`](#createtemplatepublicv2responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:emails.post_emails_locations_by_locationId_templates",
    "path_params": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /emails/locations/{locationId}/templates/folders`

**Create a template folder**

Create a new template folder

Operation id: `v3:emails.post_emails_locations_by_locationId_templates_folders` · `Version: v3` · Scopes: `emails/templates.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Request body*: [`CreateTemplateFolderPublicV2BodyDto`](#createtemplatefolderpublicv2bodydto)

*Response*: [`CreateTemplateFolderPublicV2ResponseDto`](#createtemplatefolderpublicv2responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:emails.post_emails_locations_by_locationId_templates_folders",
    "path_params": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /emails/locations/{locationId}/templates/import`

**Import an email template**

Import a template from a provider URL

Operation id: `v3:emails.post_emails_locations_by_locationId_templates_import` · `Version: v3` · Scopes: `emails/templates.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Request body*: [`ImportTemplatePublicV2BodyDto`](#importtemplatepublicv2bodydto)

*Response*: [`ImportTemplatePublicV2ResponseDto`](#importtemplatepublicv2responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:emails.post_emails_locations_by_locationId_templates_import",
    "path_params": {
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /emails/locations/{locationId}/templates/{templateId}`

**Delete a template**

Operation id: `v3:emails.delete_emails_locations_by_locationId_templates_by_templateId` · `Version: v3` · Scopes: `emails/templates.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `templateId` | string | **yes** | Template ID |

*Response*: [`DeleteTemplatePublicV2ResponseDto`](#deletetemplatepublicv2responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:emails.delete_emails_locations_by_locationId_templates_by_templateId",
    "path_params": {
      "locationId": "<locationId>",
      "templateId": "<templateId>"
    }
  }
}
```

</details>

#### `GET /emails/locations/{locationId}/templates/{templateId}`

**Get Email Template by ID**

Get a single email template by its ID

Operation id: `v3:emails.get_emails_locations_by_locationId_templates_by_templateId` · `Version: v3` · Scopes: `emails/templates.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `templateId` | string | **yes** | Template ID |

*Response*: [`GetTemplatePublicV2ResponseDto`](#gettemplatepublicv2responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:emails.get_emails_locations_by_locationId_templates_by_templateId",
    "path_params": {
      "locationId": "<locationId>",
      "templateId": "<templateId>"
    }
  }
}
```

</details>

#### `PATCH /emails/locations/{locationId}/templates/{templateId}`

**Update an email template**

Update email template

Operation id: `v3:emails.patch_emails_locations_by_locationId_templates_by_templateId` · `Version: v3` · Scopes: `emails/templates.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `templateId` | string | **yes** | Template ID |

*Request body*: [`UpdateTemplatePublicV2BodyDto`](#updatetemplatepublicv2bodydto)

*Response*: [`UpdateTemplatePublicV2ResponseDto`](#updatetemplatepublicv2responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:emails.patch_emails_locations_by_locationId_templates_by_templateId",
    "path_params": {
      "locationId": "<locationId>",
      "templateId": "<templateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::emails::*` (enable the `emails` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/emails/).

### `BuilderUpdateSuccessfulDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `ok` | String | no | ok |
| `traceId` | String | no | trace id |
| `previewUrl` | String | no | preview url |
| `templateDownloadUrl` | String | no | template data download url |

### `CreateBuilderDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | — |
| `title` | String | no | — |
| `type` | String — `html`, `folder`, `import`, `builder`, `blank` | **yes** | — |
| `updatedBy` | String | no | — |
| `builderVersion` | String — `1`, `2` | no | — |
| `name` | String | no | — |
| `parentId` | String | no | — |
| `templateDataUrl` | String | no | — |
| `importProvider` | String — `mailchimp`, `active_campaign`, `kajabi` | **yes** | — |
| `importURL` | String | no | — |
| `templateSource` | String | no | — |
| `isPlainText` | bool | no | — |

### `CreateBuilderSuccesfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `redirect` | String | **yes** | template id |
| `traceId` | String | **yes** | trace id |

### `DeleteBuilderSuccesfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `ok` | String | no | ok |
| `traceId` | String | no | trace id |

### `FetchBuilderSuccesfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | template name |
| `updatedBy` | String | no | updated by |
| `isPlainText` | bool | no | plain text based template |
| `lastUpdated` | String | no | last updated |
| `dateAdded` | String | no | date added |
| `previewUrl` | String | no | preview url |
| `id` | String | no | id |
| `version` | String | no | version |
| `templateType` | String | no | type |

### `IBuilderJsonMapper`

| Field | Type | Required | Description |
|---|---|---|---|
| `elements` | Vec<String> | **yes** | — |
| `attrs` | JSON | **yes** | — |
| `templateSettings` | [`TemplateSettings`](#templatesettings) | **yes** | — |

### `InvalidLocationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | — |
| `message` | String | no | — |

### `NotFoundDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | — |
| `message` | String | no | — |
| `error` | String | no | — |

### `SaveBuilderDataDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | — |
| `templateId` | String | **yes** | — |
| `updatedBy` | String | **yes** | — |
| `dnd` | [`IBuilderJsonMapper`](#ibuilderjsonmapper) | **yes** | — |
| `html` | String | **yes** | — |
| `editorType` | String — `html`, `builder` | **yes** | — |
| `previewText` | String | no | — |
| `isPlainText` | bool | no | — |

### `ScheduleDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | — |
| `repeatAfter` | String | **yes** | — |
| `id` | String | **yes** | — |
| `parentId` | String | **yes** | — |
| `childCount` | f64 | **yes** | — |
| `campaignType` | String | **yes** | — |
| `bulkActionVersion` | String | **yes** | — |
| `_id` | String | **yes** | — |
| `status` | String | **yes** | — |
| `sendDays` | Vec<String> | **yes** | — |
| `deleted` | bool | **yes** | — |
| `migrated` | bool | **yes** | — |
| `archived` | bool | **yes** | — |
| `hasTracking` | bool | **yes** | — |
| `isPlainText` | bool | **yes** | — |
| `hasUtmTracking` | bool | **yes** | — |
| `enableResendToUnopened` | bool | **yes** | — |
| `locationId` | String | **yes** | — |
| `templateId` | String | **yes** | — |
| `templateType` | String | **yes** | — |
| `createdAt` | String | **yes** | — |
| `updatedAt` | String | **yes** | — |
| `__v` | f64 | **yes** | — |
| `documentId` | String | **yes** | — |
| `downloadUrl` | String | **yes** | — |
| `templateDataDownloadUrl` | String | **yes** | — |
| `child` | Vec<String> | **yes** | — |

### `ScheduleFetchSuccessfulDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `schedules` | Vec<ScheduleDto> | **yes** | The list of campaigns |
| `total` | Vec<String> | **yes** | The total number of campaigns |
| `traceId` | String | **yes** | Trace Id |

### `TemplateSettings`

_No fields defined in the spec._

## Data models — API v3

In Rust: `ghl_models::v3::emails::*` (enable the `emails` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/emails/).

### `BuilderAttributePublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Attribute name |
| `default` | JSON | no | Attribute default value |
| `unit` | String | no | Attribute unit |

### `BuilderCustomFlagsPublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `layoutId` | JSON | no | Layout ID |
| `theme` | String | no | Theme name |
| `socialElementType` | String — `icon`, `icon-text`, `text` | no | Social element rendering type |

### `BuilderEditorContentPublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `elements` | Vec<BuilderElementNodePublicV2Dto> | no | Builder elements |
| `attrs` | JSON | no | Builder attributes map keyed by element ID |
| `templateSettings` | JSON | no | Template-level settings map keyed by setting group |

### `BuilderElementNodePublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Element ID |
| `tagName` | String | **yes** | Tag name |
| `children` | Vec<BuilderElementNodePublicV2Dto> | no | Child elements |

### `BuilderNodeAttrsPublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `tagName` | String | **yes** | Tag name |
| `attributes` | Vec<BuilderAttributePublicV2Dto> | **yes** | Element attributes |
| `content` | String | no | Element content |
| `customFlags` | [`BuilderCustomFlagsPublicV2Dto`](#buildercustomflagspublicv2dto) | no | Custom flags |

### `BuilderUpdateSuccessfulDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `ok` | String | no | ok |
| `traceId` | String | no | trace id |
| `previewUrl` | String | no | preview url |
| `templateDownloadUrl` | String | no | template data download url |
| `versionId` | String | no | version id of the saved template |

### `BulkActionCampaignDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Campaign ID |
| `name` | String | **yes** | Campaign name |
| `status` | String — `processing`, `scheduled`, `paused`, `complete`, `cancelled` | **yes** | Campaign status |
| `scheduleType` | String | **yes** | Schedule type (NOW or SCHEDULED) |
| `createdBy` | String | **yes** | User who created the campaign |
| `deleted` | bool | **yes** | Whether the campaign is deleted |
| `createdAt` | String | **yes** | Created at timestamp |
| `updatedAt` | String | **yes** | Last updated timestamp |
| `completedAt` | String | no | Processing completion timestamp |
| `emailMetadata` | [`BulkActionCampaignEmailDetailsDto`](#bulkactioncampaignemaildetailsdto) | no | Email metadata |

### `BulkActionCampaignEmailDetailsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `subject` | String | no | Email subject line |
| `from` | String | no | Sender (name and email) |
| `name` | String | no | Sender name |
| `templateId` | String | no | Email template ID |

### `BulkActionCampaignPublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Campaign ID |
| `source` | String | no | Source of the campaign |
| `sourceId` | String | no | Source ID of the campaign |
| `name` | String | no | Campaign name |
| `status` | String — `processing`, `scheduled`, `paused`, `complete`, `cancelled` | **yes** | Campaign status |
| `scheduleType` | String — `NOW`, `SCHEDULED`, `DRIP` | no | Schedule type (NOW, SCHEDULED, or DRIP) |
| `deleted` | bool | **yes** | Whether the campaign is deleted |
| `createdAt` | String | **yes** | Created at timestamp |
| `updatedAt` | String | **yes** | Last updated timestamp |
| `completedAt` | String | no | Processing completion timestamp |
| `emailMetadata` | [`BulkActionCampaignEmailDetailsDto`](#bulkactioncampaignemaildetailsdto) | no | Email metadata |

### `CreateBuilderDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | — |
| `title` | String | no | — |
| `type` | String — `html`, `folder`, `import`, `builder`, `blank`, `ai_template`, `vibe-editor` | **yes** | — |
| `updatedBy` | String | no | — |
| `builderVersion` | String — `1`, `2` | no | — |
| `name` | String | no | — |
| `parentId` | String | no | — |
| `templateDataUrl` | String | no | — |
| `importProvider` | String — `mailchimp`, `active_campaign`, `kajabi`, `other`, `import_with_email_ai` | **yes** | — |
| `importURL` | String | no | — |
| `templateSource` | String | no | — |
| `isPlainText` | bool | no | — |
| `subjectLine` | String | no | — |
| `fromName` | String | no | — |
| `fromEmail` | String | no | — |
| `previewText` | String | no | — |

### `CreateBuilderSuccesfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `redirect` | String | **yes** | template id |
| `traceId` | String | **yes** | trace id |

### `CreateEmailCampaignPublicV2BodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Campaign name |
| `editorType` | String — `html`, `text` | **yes** | Editor type for the campaign content. Use `html` for code-editor campaigns or `text` for plain-text campaigns. |
| `templateId` | String | no | Existing template ID to create the campaign from. Omit this field to create a blank campaign. |
| `editorContent` | String | no | Optional initial editor content to persist immediately after campaign creation. Provide HTML or plain-text string content. |
| `parentFolderId` | String | no | Parent folder ID |
| `timeZone` | String | **yes** | Timezone for the campaign |
| `userId` | String | **yes** | ID of the user performing this action |
| `userName` | String | no | Name of the user performing this action |

### `CreateEmailCampaignPublicV2ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Campaign ID |
| `source` | String | no | Source of the campaign |
| `sourceId` | String | no | Source ID of the campaign |
| `name` | String | no | Campaign name |
| `status` | String — `all`, `sent`, `failed`, `archived`, `draft`, `processing`, `scheduled`, `cancelled`, `paused` | no | Campaign status |
| `campaignType` | String | no | Campaign type |
| `campaignCategory` | String | no | Campaign category |
| `variations` | Vec<EmailCampaignVariationPublicV2Dto> | no | AB test variation identifiers (available only for AB test campaigns) |
| `deleted` | bool | **yes** | Whether the campaign is deleted |
| `createdAt` | String | **yes** | Created at timestamp |
| `updatedAt` | String | **yes** | Last updated timestamp |
| `traceId` | String | no | Trace ID of request |

### `CreateTemplateFolderPublicV2BodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Folder name |
| `userId` | String | no | ID of the user performing this action |

### `CreateTemplateFolderPublicV2ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Folder ID |
| `name` | String | **yes** | Folder name |
| `createdAt` | String | no | Created timestamp |
| `updatedAt` | String | no | Updated timestamp |
| `traceId` | String | no | Trace ID of request |

### `CreateTemplatePublicV2BodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Template name |
| `editorType` | String — `html`, `text` | **yes** | Editor type for the new template. Use `html` for code-editor templates or `text` for plain-text templates. |
| `editorContent` | String | no | Optional initial editor content. Provide HTML or plain-text string content. |
| `parentFolderId` | String | no | Parent folder ID |
| `subjectLine` | String | no | Email subject line |
| `fromName` | String | no | Sender name |
| `fromEmail` | String | no | Sender email address |
| `previewText` | String | no | Preview text |
| `userId` | String | no | ID of the user performing this action |

### `CreateTemplatePublicV2ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Template ID |
| `name` | String | **yes** | Template name |
| `editorType` | String — `html`, `text` | **yes** | Editor type |
| `isPlainText` | bool | **yes** | Whether template is plain text |
| `parentFolderId` | String | no | Parent folder ID |
| `fromName` | String | no | Sender name |
| `fromEmail` | String | no | Sender email address |
| `subjectLine` | String | no | Email subject line |
| `previewText` | String | no | Preview text |
| `previewUrl` | String | no | Preview URL |
| `createdAt` | String | no | Created timestamp |
| `updatedAt` | String | no | Updated timestamp |
| `traceId` | String | no | Trace ID of request |

### `DeleteBuilderSuccesfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `ok` | String | no | ok |
| `traceId` | String | no | trace id |

### `DeleteCampaignPublicV2ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `deleted` | bool | **yes** | Whether the campaign was deleted successfully |
| `traceId` | String | no | Trace ID of the request |

### `DeleteTemplatePublicV2ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `deleted` | bool | **yes** | Whether the template was deleted successfully |
| `traceId` | String | no | Trace ID of the request |

### `EmailCampaignPublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Campaign ID |
| `source` | String | no | Source of the campaign |
| `sourceId` | String | no | Source ID of the campaign |
| `name` | String | no | Campaign name |
| `status` | String — `all`, `sent`, `failed`, `archived`, `draft`, `processing`, `scheduled`, `cancelled`, `paused` | no | Campaign status |
| `campaignType` | String | no | Campaign type |
| `campaignCategory` | String | no | Campaign category |
| `variations` | Vec<EmailCampaignVariationPublicV2Dto> | no | AB test variation identifiers (available only for AB test campaigns) |
| `deleted` | bool | **yes** | Whether the campaign is deleted |
| `createdAt` | String | **yes** | Created at timestamp |
| `updatedAt` | String | **yes** | Last updated timestamp |

### `EmailCampaignVariationPublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `sourceId` | String | **yes** | Variation source ID for stats lookup |
| `isWinner` | bool | **yes** | Whether this is the winning variation |

### `EmailStatsNumbersDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `delivered` | f64 | **yes** | Emails delivered |
| `opened` | f64 | **yes** | Emails opened |
| `clicked` | f64 | **yes** | Links clicked |
| `unsubscribed` | f64 | **yes** | Unsubscribes |
| `complained` | f64 | **yes** | Spam complaints |
| `permanentFail` | f64 | **yes** | Hard bounces |
| `temporaryFail` | f64 | **yes** | Soft bounces |
| `rejected` | f64 | **yes** | Rejected emails |
| `failed` | f64 | **yes** | Failed emails |
| `replied` | f64 | **yes** | Replies received |

### `EmailStatsNumbersPublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `sent` | f64 | **yes** | Total emails sent (delivered + accepted + bounced) |
| `accepted` | f64 | **yes** | Emails accepted by the mail server |
| `delivered` | f64 | **yes** | Emails delivered to inbox |
| `opened` | f64 | **yes** | Emails opened |
| `clicked` | f64 | **yes** | Links clicked |
| `unsubscribed` | f64 | **yes** | Unsubscribes |
| `complained` | f64 | **yes** | Spam complaints |
| `permanentFail` | f64 | **yes** | Hard bounces |
| `temporaryFail` | f64 | **yes** | Soft bounces |
| `rejected` | f64 | **yes** | Rejected emails |
| `failed` | f64 | **yes** | Failed emails |
| `replied` | f64 | **yes** | Replies received |
| `openRate` | f64 | **yes** | Open rate as percentage of delivered |
| `clickRate` | f64 | **yes** | Click rate as percentage of delivered |
| `unsubscribeRate` | f64 | **yes** | Unsubscribe rate as percentage of delivered |
| `complaintRate` | f64 | **yes** | Complaint rate as percentage of delivered |
| `bounceRate` | f64 | **yes** | Bounce rate as percentage of sent |
| `replyRate` | f64 | **yes** | Reply rate as percentage of delivered |

### `FetchBuilderSuccesfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | template name |
| `updatedBy` | String | no | updated by |
| `isPlainText` | bool | no | plain text based template |
| `lastUpdated` | String | no | last updated |
| `dateAdded` | String | no | date added |
| `previewUrl` | String | no | preview url |
| `id` | String | no | id |
| `version` | String | no | version |
| `templateType` | String | no | type |

### `GetBulkActionCampaignPublicV2ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Campaign ID |
| `source` | String | no | Source of the campaign |
| `sourceId` | String | no | Source ID of the campaign |
| `name` | String | no | Campaign name |
| `status` | String — `processing`, `scheduled`, `paused`, `complete`, `cancelled` | **yes** | Campaign status |
| `scheduleType` | String — `NOW`, `SCHEDULED`, `DRIP` | no | Schedule type (NOW, SCHEDULED, or DRIP) |
| `fromName` | String | no | Sender name |
| `fromEmail` | String | no | Sender email address |
| `subject` | String | no | Email subject line |
| `replyToAddress` | String | no | Reply-to email address |
| `previewText` | String | no | Preview text |
| `editorType` | String — `html`, `builder`, `text` | no | Editor type for this campaign |
| `isPlainText` | bool | no | Whether the campaign uses plain text |
| `editorContentUrl` | String | no | URL to fetch the rendered campaign content as HTML. Issue a GET against this URL to retrieve the body. |
| `deleted` | bool | **yes** | Whether the campaign is deleted |
| `createdAt` | String | **yes** | Created at timestamp |
| `updatedAt` | String | **yes** | Last updated timestamp |
| `completedAt` | String | no | Processing completion timestamp |
| `traceId` | String | no | Trace ID of the request |

### `GetBulkActionCampaignsResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `campaigns` | Vec<BulkActionCampaignDto> | **yes** | List of bulk action campaigns |
| `total` | f64 | **yes** | Total count of bulk action campaigns |

### `GetCampaignStatsPublicV2ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `source` | String — `email-campaigns`, `workflow-campaigns`, `bulk-actions` | **yes** | Source type |
| `sourceId` | String | **yes** | Source ID |
| `subSourceId` | String | no | Workflow action ID |
| `stats` | [`EmailStatsNumbersPublicV2Dto`](#emailstatsnumberspublicv2dto) | **yes** | Email performance metrics |
| `traceId` | String | no | Trace ID of the request |

### `GetCampaignStatsResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `source` | String — `email-campaigns`, `workflow-campaigns`, `bulk-actions` | **yes** | Source type |
| `sourceId` | String | **yes** | Source ID |
| `subSourceId` | String | no | Workflow action ID |
| `stats` | [`EmailStatsNumbersDto`](#emailstatsnumbersdto) | **yes** | Email performance metrics |

### `GetEmailCampaignPublicV2ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Campaign ID |
| `source` | String | no | Source of the campaign |
| `sourceId` | String | no | Source ID of the campaign |
| `name` | String | no | Campaign name |
| `status` | String — `all`, `sent`, `failed`, `archived`, `draft`, `processing`, `scheduled`, `cancelled`, `paused` | no | Campaign status |
| `campaignType` | String | no | Campaign delivery type |
| `campaignCategory` | String | no | Campaign category |
| `variations` | Vec<EmailCampaignVariationPublicV2Dto> | no | AB test variation identifiers (available only for AB test campaigns) |
| `editorType` | String — `html`, `builder`, `text` | no | Original editor type the campaign was created with |
| `isPlainText` | bool | no | Whether the campaign uses plain text |
| `editorContentUrl` | String | no | URL to fetch the rendered campaign content as HTML. Issue a GET against this URL to retrieve the body. |
| `fromName` | String | no | Sender name |
| `fromEmail` | String | no | Sender email address |
| `subject` | String | no | Email subject line |
| `replyToAddress` | String | no | Reply-to email address |
| `previewText` | String | no | Preview text |
| `deleted` | bool | **yes** | Whether the campaign is deleted |
| `createdAt` | String | **yes** | Created at timestamp |
| `updatedAt` | String | **yes** | Last updated timestamp |
| `traceId` | String | no | Trace ID of the request |

### `GetTemplatePublicV2ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Template ID |
| `name` | String | **yes** | Template name |
| `editorType` | String — `html`, `builder`, `text` | **yes** | Editor type |
| `isPlainText` | bool | **yes** | Whether template is plain text |
| `parentFolderId` | String | no | Parent folder ID |
| `fromName` | String | no | Sender name |
| `fromEmail` | String | no | Sender email address |
| `subject` | String | no | Email subject line |
| `previewText` | String | no | Preview text |
| `editorContentUrl` | String | no | URL to fetch the rendered template content as HTML. Issue a GET against this URL to retrieve the body. |
| `deleted` | bool | **yes** | Whether the template is deleted |
| `createdAt` | String | no | Created timestamp |
| `updatedAt` | String | no | Updated timestamp |
| `traceId` | String | no | Trace ID of request |

### `GetWorkflowCampaignPublicV2ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Campaign ID |
| `name` | String | no | Campaign name |
| `status` | String — `published`, `draft` | no | Campaign status |
| `source` | String | no | Source of the campaign |
| `sourceId` | String | no | Source ID of the campaign |
| `subSources` | Vec<WorkflowCampaignSubSourcePublicV2Dto> | no | Sub-sources (email-sending steps) within this workflow. Each entry's `id` can be passed as the `subSourceId` query parameter to the campaign stats endpoint to retrieve per-step stats. |
| `deleted` | bool | no | Whether the campaign is deleted |
| `createdAt` | String | **yes** | Created at timestamp |
| `updatedAt` | String | **yes** | Updated at timestamp |
| `traceId` | String | no | Trace ID of the request |

### `GetWorkflowCampaignsPublicResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `campaigns` | Vec<WorkflowCampaignPublicDto> | **yes** | List of workflow campaigns |
| `total` | f64 | **yes** | Total count of campaigns |

### `IBuilderJsonMapper`

| Field | Type | Required | Description |
|---|---|---|---|
| `elements` | Vec<String> | **yes** | Array of VNode elements representing the email structure |
| `attrs` | JSON | **yes** | Object mapping element IDs to their attributes and styles |
| `templateSettings` | [`TemplateSettings`](#templatesettings) | **yes** | Template-level settings and configuration |

### `ImportTemplatePublicV2BodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `importProvider` | String — `mailchimp`, `active_campaign` | **yes** | Import provider (URL-based providers only) |
| `importUrl` | String | **yes** | Public import URL |
| `name` | String | no | Template name |
| `parentFolderId` | String | no | Parent folder ID |
| `userId` | String | no | ID of the user performing this action |

### `ImportTemplatePublicV2ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Template ID |
| `name` | String | **yes** | Template name |
| `editorType` | String — `html`, `text` | **yes** | Editor type |
| `isPlainText` | bool | **yes** | Whether template is plain text |
| `parentFolderId` | String | no | Parent folder ID |
| `fromName` | String | no | Sender name |
| `fromEmail` | String | no | Sender email address |
| `subjectLine` | String | no | Email subject line |
| `previewText` | String | no | Preview text |
| `previewUrl` | String | no | Preview URL |
| `createdAt` | String | no | Created timestamp |
| `updatedAt` | String | no | Updated timestamp |
| `traceId` | String | no | Trace ID of request |

### `InvalidLocationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | HTTP status code for invalid location access |
| `message` | String | no | Error message describing the location access failure |

### `ListBulkActionCampaignsPublicV2ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `campaigns` | Vec<BulkActionCampaignPublicV2Dto> | **yes** | List of bulk action campaigns |
| `total` | f64 | **yes** | Total count of bulk action campaigns |
| `traceId` | String | no | Trace ID of the request |

### `ListEmailCampaignsPublicV2ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `campaigns` | Vec<EmailCampaignPublicV2Dto> | **yes** | List of email campaigns |
| `total` | f64 | **yes** | Total count of email campaigns |
| `traceId` | String | no | Trace ID of the request |

### `ListTemplatesPublicV2ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `items` | Vec<TemplateListItemPublicV2Dto> | **yes** | List of template and folder resources |
| `total` | f64 | **yes** | Total count of templates and folders |
| `traceId` | String | no | Trace ID of the request |

### `ListWorkflowCampaignsPublicV2ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `campaigns` | Vec<WorkflowCampaignPublicV2Dto> | **yes** | List of workflow campaigns |
| `total` | f64 | **yes** | Total count of campaigns |
| `traceId` | String | no | Trace ID of the request |

### `NotFoundDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | HTTP status code for not found |
| `message` | String | no | Error message describing the not found failure |
| `error` | String | no | Error type identifier |

### `SaveBuilderDataDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | — |
| `templateId` | String | **yes** | — |
| `updatedBy` | String | **yes** | — |
| `dnd` | [`IBuilderJsonMapper`](#ibuilderjsonmapper) | **yes** | — |
| `html` | String | **yes** | — |
| `editorType` | String — `html`, `builder` | **yes** | — |
| `previewText` | String | no | — |
| `isPlainText` | bool | no | — |
| `usedEmailAI` | bool | no | Whether Email AI was used |

### `ScheduleCampaignABTestConfigPublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `testType` | String — `emailContent`, `subjectLine` | **yes** | What is being tested |
| `testDuration` | f64 | **yes** | Seconds to run the test before picking a winner |
| `variationCount` | f64 | **yes** | Number of variations |
| `testSize` | f64 | **yes** | Percentage of contacts in the test group (0-100) |
| `winningCriteria` | String — `openRate`, `clickRate` | **yes** | How to pick the winner |
| `variations` | Vec<ScheduleCampaignABTestVariationPublicV2Dto> | **yes** | A/B test variations |

### `ScheduleCampaignABTestVariationPublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `subject` | String | no | Subject line for this variation |
| `documentId` | String | no | Template/document ID for this variation |

### `ScheduleCampaignBatchConfigPublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `batchSize` | f64 | **yes** | Number of contacts to process per batch |
| `interval` | f64 | **yes** | Delay between batches |
| `intervalUnit` | String — `minutes`, `hours`, `days` | **yes** | Unit for the interval |
| `skipDays` | Vec<String (enum)> | no | Days to skip sending |
| `windowStart` | String | no | Earliest time to send batches |
| `windowEnd` | String | no | Latest time to send batches |

### `ScheduleCampaignEmailMetaPublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `subject` | String | **yes** | Email subject line |
| `fromName` | String | **yes** | Sender display name |
| `fromEmail` | String | **yes** | Sender email address |
| `replyToAddress` | String | no | Reply-to email address |
| `previewText` | String | no | Preview text shown in inbox after subject |
| `attachments` | Vec<String> | no | Attachment download URLs |

### `ScheduleCampaignPublicV2BodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `scheduleType` | String — `immediate`, `scheduled`, `batch`, `rss`, `smart_send` | **yes** | How to schedule the campaign |
| `timeZone` | String | **yes** | IANA timezone |
| `userId` | String | **yes** | ID of the user performing this action |
| `userName` | String | no | Name of the user performing this action |
| `emailMeta` | [`ScheduleCampaignEmailMetaPublicV2Dto`](#schedulecampaignemailmetapublicv2dto) | **yes** | Email subject, sender, and content metadata |
| `recipients` | [`ScheduleCampaignRecipientsPublicV2Dto`](#schedulecampaignrecipientspublicv2dto) | **yes** | Who receives the email. Must provide either contactIds or filter. |
| `sendDays` | Vec<String (enum)> | no | Days of the week to allow sending. Used for batch and RSS scheduleTypes. |
| `scheduleConfig` | [`ScheduleCampaignScheduleConfigPublicV2Dto`](#schedulecampaignscheduleconfigpublicv2dto) | no | Schedule configuration for immediate, scheduled, batch, and smart_send types. Required when scheduleType is not rss. |
| `rssConfig` | [`ScheduleCampaignRssConfigPublicV2Dto`](#schedulecampaignrssconfigpublicv2dto) | no | RSS feed configuration. Required when scheduleType is rss. |
| `abTestConfig` | [`ScheduleCampaignABTestConfigPublicV2Dto`](#schedulecampaignabtestconfigpublicv2dto) | no | A/B test configuration. Can be combined with any scheduleType except rss. |

### `ScheduleCampaignPublicV2ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `campaignId` | String | **yes** | Campaign ID |
| `sourceId` | String | **yes** | Source ID for fetching campaign statistics |
| `traceId` | String | no | Trace ID of the request |

### `ScheduleCampaignRecipientsPublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `contact`, `tag`, `segment` | **yes** | Recipient selection type |
| `contactIds` | Vec<String> | no | Contact IDs to send to. Required when type is contact. |
| `tagIds` | Vec<String> | no | Tag IDs to filter recipients by. Required when type is tag. |
| `segment` | String — `engaged_last_7_days`, `engaged_last_30_days`, `engaged_last_60_days`, `engaged_last_5_campaigns`, `unengaged_last_5_campaigns` | no | Segment type for pre-built segments. Required when type is segment. |
| `freezeList` | bool | no | Freeze the contact list at schedule time — new matching contacts will not be added later |

### `ScheduleCampaignResendPublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | no | Enable resend to contacts who did not open |
| `waitHours` | f64 | no | Hours to wait before resending. Required when enabled is true. |
| `subject` | String | no | Override subject line for the resend email |

### `ScheduleCampaignRssConfigPublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | RSS schedule name |
| `rssFeedURL` | String | **yes** | RSS feed URL |
| `repeatAfter` | String — `every_day`, `every_week`, `every_month` | **yes** | How often to check the feed |
| `repeatAfterTime` | String | **yes** | Time of day to execute |
| `rssFeedLimit` | f64 | no | Max number of RSS items per email |
| `startAtDay` | String — `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday`, `Sunday` | no | Day of week for weekly RSS |
| `startAtMonthDay` | String | no | Day of month for monthly RSS |
| `firstExecutionDate` | String | no | Override first execution date/time |

### `ScheduleCampaignScheduleConfigPublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `sendAt` | String | no | Date/time to send. Required for scheduled, batch, and smart_send. Ignored for immediate. |
| `batch` | [`ScheduleCampaignBatchConfigPublicV2Dto`](#schedulecampaignbatchconfigpublicv2dto) | no | Batch/drip configuration. Required when scheduleType is batch. Ignored otherwise. |
| `tracking` | [`ScheduleCampaignTrackingPublicV2Dto`](#schedulecampaigntrackingpublicv2dto) | no | Click and UTM tracking options |
| `resend` | [`ScheduleCampaignResendPublicV2Dto`](#schedulecampaignresendpublicv2dto) | no | Auto-resend to contacts who did not open |
| `emailPreferenceId` | String | no | Email preference type ID for categorizing this campaign |

### `ScheduleCampaignTrackingPublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `clickTracking` | bool | no | Enable click tracking on links |
| `utmTracking` | bool | no | Enable UTM parameters on links |

### `ScheduleDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | — |
| `repeatAfter` | String | **yes** | — |
| `id` | String | **yes** | — |
| `parentId` | String | **yes** | — |
| `childCount` | f64 | **yes** | — |
| `campaignType` | String | **yes** | — |
| `bulkActionVersion` | String | **yes** | — |
| `_id` | String | **yes** | — |
| `status` | String | **yes** | — |
| `sendDays` | Vec<String> | **yes** | — |
| `deleted` | bool | **yes** | — |
| `migrated` | bool | **yes** | — |
| `archived` | bool | **yes** | — |
| `hasTracking` | bool | **yes** | — |
| `isPlainText` | bool | **yes** | — |
| `hasUtmTracking` | bool | **yes** | — |
| `enableResendToUnopened` | bool | **yes** | — |
| `locationId` | String | **yes** | — |
| `templateId` | String | **yes** | — |
| `templateType` | String | **yes** | — |
| `createdAt` | String | **yes** | — |
| `updatedAt` | String | **yes** | — |
| `__v` | f64 | **yes** | — |
| `documentId` | String | **yes** | — |
| `downloadUrl` | String | **yes** | — |
| `templateDataDownloadUrl` | String | **yes** | — |
| `child` | Vec<String> | **yes** | — |

### `ScheduleFetchSuccessfulDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `schedules` | Vec<ScheduleDto> | **yes** | The list of campaigns |
| `total` | Vec<String> | **yes** | The total number of campaigns |
| `traceId` | String | **yes** | Trace Id |

### `TemplateListItemPublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Resource ID |
| `name` | String | **yes** | Resource name |
| `type` | String — `template`, `folder` | **yes** | Resource type |
| `isPlainText` | bool | no | Whether template is plain text |
| `updatedAt` | String | no | Last updated timestamp |
| `createdAt` | String | no | Created timestamp |
| `previewUrl` | String | no | Preview URL |
| `editorType` | String — `html`, `builder`, `text` | no | Editor type for template resources |
| `childCount` | f64 | no | Children count for folder resources |
| `hasChildren` | bool | no | Whether folder has child resources |
| `parentFolderId` | String | no | Parent folder ID |

### `TemplateSettings`

_No fields defined in the spec._

### `UpdateEmailCampaignPublicV2BodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Campaign name |
| `editorContent` | String | no | Editor content to update. Required only when updating campaign content, and must be provided together with editorType. Provide HTML or plain-text string content. |
| `editorType` | String — `html`, `text` | no | Editor type for campaign content. Required only when updating campaign content, and must be provided together with editorContent. |
| `userId` | String | no | ID of the user performing this action |

### `UpdateEmailCampaignPublicV2ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Campaign ID |
| `source` | String | no | Source of the campaign |
| `sourceId` | String | no | Source ID of the campaign |
| `name` | String | no | Campaign name |
| `status` | String — `all`, `sent`, `failed`, `archived`, `draft`, `processing`, `scheduled`, `cancelled`, `paused` | no | Campaign status |
| `campaignType` | String | no | Campaign type |
| `campaignCategory` | String | no | Campaign category |
| `variations` | Vec<EmailCampaignVariationPublicV2Dto> | no | AB test variation identifiers (available only for AB test campaigns) |
| `deleted` | bool | **yes** | Whether the campaign is deleted |
| `createdAt` | String | **yes** | Created at timestamp |
| `updatedAt` | String | **yes** | Last updated timestamp |
| `traceId` | String | no | Trace ID of request |

### `UpdateEmailTemplateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID where the template belongs |
| `updatedBy` | String | no | User ID who is updating the template |
| `editorContent` | JSON | no | Editor content - can be HTML string, plain text string, or DND builder object depending on editorType. When editorType is "html" or "text", this should be a string. When editorType is "builder", this … |
| `editorType` | String — `html`, `builder`, `text` | no | Type of editor content: "html" for HTML content, "text" for plain text content, "builder" for drag-and-drop builder content. Must be provided together with editorContent. |
| `previewText` | String | no | Preview text shown in email clients before opening |
| `subjectLine` | String | no | Email subject line |
| `fromName` | String | no | Sender name displayed in email |
| `fromEmail` | String | no | Sender email address |
| `name` | String | no | Template name |
| `archived` | bool | no | Whether the template is archived |
| `fieldDefaults` | JSON | no | Field-level default values for custom variables in template fields (fromName, subjectLine, previewText) |

### `UpdateEmailTemplateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `ok` | bool | **yes** | Indicates if the update was successful |
| `id` | String | **yes** | Unique template identifier |
| `name` | String | **yes** | Template name |
| `archived` | bool | **yes** | Whether the template is archived |
| `builderVersion` | String | **yes** | Builder version used for the template |
| `fromName` | String | **yes** | Sender name displayed in email |
| `fromEmail` | String | **yes** | Sender email address |
| `subjectLine` | String | **yes** | Email subject line |
| `previewText` | String | **yes** | Preview text shown in email clients |
| `previewUrl` | String | **yes** | URL to preview the rendered template |
| `type` | String — `html`, `builder` | **yes** | Type of template editor used |
| `lastUpdated` | String | **yes** | Timestamp of last update |
| `createdAt` | String | **yes** | Timestamp when template was created |
| `isPlainText` | bool | **yes** | Whether the template contains plain text content (true) or HTML content (false) |
| `fieldDefaults` | JSON | no | Field-level default values for custom variables in template fields |

### `UpdateTemplatePublicV2BodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Template name |
| `editorContent` | String | no | Editor content to update. Required only when updating template content, and must be provided together with editorType. Provide HTML or plain-text string content. |
| `editorType` | String — `html`, `text` | no | Type of editor content. Required only when updating template content, and must be provided together with editorContent. |
| `previewText` | String | no | Preview text |
| `subjectLine` | String | no | Email subject line |
| `fromName` | String | no | Sender name |
| `fromEmail` | String | no | Sender email address |
| `archived` | bool | no | Whether template is archived |
| `parentFolderId` | String | no | Parent folder ID. Pass `null` to move template to the root level. |
| `userId` | String | no | ID of the user performing this action |

### `UpdateTemplatePublicV2ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Template ID |
| `name` | String | **yes** | Template name |
| `archived` | bool | **yes** | Whether template is archived |
| `fromName` | String | **yes** | Sender name |
| `fromEmail` | String | **yes** | Sender email address |
| `subjectLine` | String | **yes** | Email subject line |
| `previewText` | String | **yes** | Preview text |
| `previewUrl` | String | **yes** | Preview URL |
| `editorType` | String — `html`, `text` | no | Template type |
| `isPlainText` | bool | no | Whether template is plain text |
| `parentFolderId` | String | no | Parent folder ID |
| `updatedAt` | String | no | Last updated timestamp |
| `createdAt` | String | no | Created timestamp |
| `traceId` | String | no | Trace ID of request |

### `WorkflowCampaignPublicDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Campaign ID |
| `name` | String | **yes** | Campaign name |
| `status` | String — `published`, `draft` | **yes** | Campaign status |
| `sourceId` | String | **yes** | Source ID |
| `deleted` | bool | **yes** | Whether the campaign is deleted |
| `createdAt` | String | **yes** | Created at timestamp |
| `updatedAt` | String | **yes** | Updated at timestamp |

### `WorkflowCampaignPublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Campaign ID |
| `name` | String | no | Campaign name |
| `status` | String — `published`, `draft` | no | Campaign status |
| `source` | String | no | Source of the campaign |
| `sourceId` | String | no | Source ID of the campaign |
| `deleted` | bool | no | Whether the campaign is deleted |
| `createdAt` | String | **yes** | Created at timestamp |
| `updatedAt` | String | **yes** | Updated at timestamp |

### `WorkflowCampaignSubSourcePublicV2Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Sub-source identifier (workflow step). Pass this value as the `subSourceId` query parameter on the campaign stats endpoint to retrieve stats scoped to this step. |
| `name` | String | no | Workflow step name |
| `subject` | String | no | Email subject line |
| `fromName` | String | no | Sender name |
| `fromEmail` | String | no | Sender email address |
| `previewText` | String | no | Preview text |
| `editorType` | String — `html`, `builder`, `text` | no | Editor type for this step |
| `isPlainText` | bool | no | Whether this step uses plain text |
| `editorContentUrl` | String | no | URL to fetch the rendered step content as HTML. Issue a GET against this URL to retrieve the body. |
| `createdAt` | String | no | Timestamp when this step was added to the workflow |
| `updatedAt` | String | no | Timestamp when this step was last updated |

