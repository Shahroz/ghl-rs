# `ad-publishing`

**95** operations / **108** models in API v3

## How to call it

No hand-written service yet — reach these endpoints two ways:

**From Rust** (typed body via [`ghl-models`](https://docs.rs/ghl-models)):

```rust,ignore
// cargo add ghl-models --features ad-publishing
use ghl_models::v2::ad_publishing::*;

let body = serde_json::to_value(/* a Create…Dto from above */)?;
let out = ghl.request_raw("POST", "/path/", &[], Some(&body), None).await?;
```

**From an AI agent** (MCP meta-tools):

```json
{
  "name": "ghl_search_operations",
  "arguments": {
    "query": "",
    "module": "ad-publishing"
  }
}
```

## Endpoints — API v3

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/ad-publishing/facebook/ad-accounts` | Get ad accounts | `v3:ad-publishing.get_ad_publishing_facebook_ad_accounts` |
| `DELETE` | `/ad-publishing/facebook/ad-accounts/{adAccountId}` | Delete ad account | `v3:ad-publishing.delete_ad_publishing_facebook_ad_accounts_by_adAccountId` |
| `GET` | `/ad-publishing/facebook/ad-accounts/{adAccountId}` | Get ad account details | `v3:ad-publishing.get_ad_publishing_facebook_ad_accounts_by_adAccountId` |
| `PUT` | `/ad-publishing/facebook/ads` | Upsert ad | `v3:ad-publishing.put_ad_publishing_facebook_ads` |
| `DELETE` | `/ad-publishing/facebook/ads/{adId}` | Delete ad | `v3:ad-publishing.delete_ad_publishing_facebook_ads_by_adId` |
| `POST` | `/ad-publishing/facebook/ads/{adId}/duplicate` | Duplicate ad | `v3:ad-publishing.post_ad_publishing_facebook_ads_by_adId_duplicate` |
| `POST` | `/ad-publishing/facebook/ads/{adId}/pause` | Pause ad | `v3:ad-publishing.post_ad_publishing_facebook_ads_by_adId_pause` |
| `POST` | `/ad-publishing/facebook/ads/{adId}/resume` | Resume ad | `v3:ad-publishing.post_ad_publishing_facebook_ads_by_adId_resume` |
| `PUT` | `/ad-publishing/facebook/adsets` | Upsert adset | `v3:ad-publishing.put_ad_publishing_facebook_adsets` |
| `DELETE` | `/ad-publishing/facebook/adsets/{adSetId}` | Delete ad set | `v3:ad-publishing.delete_ad_publishing_facebook_adsets_by_adSetId` |
| `POST` | `/ad-publishing/facebook/adsets/{adSetId}/duplicate` | Duplicate ad set | `v3:ad-publishing.post_ad_publishing_facebook_adsets_by_adSetId_duplicate` |
| `POST` | `/ad-publishing/facebook/adsets/{adSetId}/pause` | Pause ad set | `v3:ad-publishing.post_ad_publishing_facebook_adsets_by_adSetId_pause` |
| `POST` | `/ad-publishing/facebook/adsets/{adSetId}/resume` | Resume ad set | `v3:ad-publishing.post_ad_publishing_facebook_adsets_by_adSetId_resume` |
| `GET` | `/ad-publishing/facebook/campaign/{campaignId}` | Get campaign with linked entities | `v3:ad-publishing.get_ad_publishing_facebook_campaign_by_campaignId` |
| `PUT` | `/ad-publishing/facebook/campaigns` | Upsert campaign | `v3:ad-publishing.put_ad_publishing_facebook_campaigns` |
| `DELETE` | `/ad-publishing/facebook/campaigns/{campaignId}` | Delete campaign | `v3:ad-publishing.delete_ad_publishing_facebook_campaigns_by_campaignId` |
| `POST` | `/ad-publishing/facebook/campaigns/{campaignId}/duplicate` | Duplicate campaign | `v3:ad-publishing.post_ad_publishing_facebook_campaigns_by_campaignId_duplicate` |
| `POST` | `/ad-publishing/facebook/campaigns/{campaignId}/pause` | Pause campaign | `v3:ad-publishing.post_ad_publishing_facebook_campaigns_by_campaignId_pause` |
| `POST` | `/ad-publishing/facebook/campaigns/{campaignId}/publish` | Publish campaign | `v3:ad-publishing.post_ad_publishing_facebook_campaigns_by_campaignId_publish` |
| `GET` | `/ad-publishing/facebook/campaigns/{campaignId}/publishing-progress` | Get campaign publishing progress | `v3:ad-publishing.get_ad_publishing_facebook_campaigns_by_campaignId_publishing_progress` |
| `POST` | `/ad-publishing/facebook/campaigns/{campaignId}/resume` | Resume campaign | `v3:ad-publishing.post_ad_publishing_facebook_campaigns_by_campaignId_resume` |
| `GET` | `/ad-publishing/facebook/conversation-forms` | Get conversation forms | `v3:ad-publishing.get_ad_publishing_facebook_conversation_forms` |
| `POST` | `/ad-publishing/facebook/conversation-forms` | Create conversation form | `v3:ad-publishing.post_ad_publishing_facebook_conversation_forms` |
| `GET` | `/ad-publishing/facebook/custom-audience` | Get custom audiences | `v3:ad-publishing.get_ad_publishing_facebook_custom_audience` |
| `DELETE` | `/ad-publishing/facebook/custom-audience/{audienceId}` | Delete custom audience | `v3:ad-publishing.delete_ad_publishing_facebook_custom_audience_by_audienceId` |
| `GET` | `/ad-publishing/facebook/custom-audience/{audienceId}` | Get custom audience by ID | `v3:ad-publishing.get_ad_publishing_facebook_custom_audience_by_audienceId` |
| `PUT` | `/ad-publishing/facebook/custom-audience/{audienceId}` | Update custom audience | `v3:ad-publishing.put_ad_publishing_facebook_custom_audience_by_audienceId` |
| `DELETE` | `/ad-publishing/facebook/custom-audience/{audienceId}/member` | Remove custom audience member | `v3:ad-publishing.delete_ad_publishing_facebook_custom_audience_by_audienceId_member` |
| `PUT` | `/ad-publishing/facebook/custom-audience/{audienceId}/member` | Add custom audience member | `v3:ad-publishing.put_ad_publishing_facebook_custom_audience_by_audienceId_member` |
| `PUT` | `/ad-publishing/facebook/custom-audience/{audienceId}/member/batch` | Batch update audience members | `v3:ad-publishing.put_ad_publishing_facebook_custom_audience_by_audienceId_member_batch` |
| `GET` | `/ad-publishing/facebook/entity` | Get entities | `v3:ad-publishing.get_ad_publishing_facebook_entity` |
| `DELETE` | `/ad-publishing/facebook/integration` | Delete Facebook integration | `v3:ad-publishing.delete_ad_publishing_facebook_integration` |
| `GET` | `/ad-publishing/facebook/integration` | Get Facebook integration | `v3:ad-publishing.get_ad_publishing_facebook_integration` |
| `POST` | `/ad-publishing/facebook/integration` | Create Facebook integration | `v3:ad-publishing.post_ad_publishing_facebook_integration` |
| `GET` | `/ad-publishing/facebook/lead-form/{leadFormId}` | Get lead form by ID | `v3:ad-publishing.get_ad_publishing_facebook_lead_form_by_leadFormId` |
| `GET` | `/ad-publishing/facebook/me` | Get current Facebook user | `v3:ad-publishing.get_ad_publishing_facebook_me` |
| `DELETE` | `/ad-publishing/facebook/page` | Delete page connection | `v3:ad-publishing.delete_ad_publishing_facebook_page` |
| `PUT` | `/ad-publishing/facebook/page/default` | Set default page | `v3:ad-publishing.put_ad_publishing_facebook_page_default` |
| `GET` | `/ad-publishing/facebook/page/{pageId}/forms` | Get page lead forms | `v3:ad-publishing.get_ad_publishing_facebook_page_by_pageId_forms` |
| `POST` | `/ad-publishing/facebook/page/{pageId}/forms` | Create page lead form | `v3:ad-publishing.post_ad_publishing_facebook_page_by_pageId_forms` |
| `GET` | `/ad-publishing/facebook/page/{pageId}/instagram` | Get Instagram accounts for page | `v3:ad-publishing.get_ad_publishing_facebook_page_by_pageId_instagram` |
| `GET` | `/ad-publishing/facebook/pages` | Get Facebook pages | `v3:ad-publishing.get_ad_publishing_facebook_pages` |
| `GET` | `/ad-publishing/facebook/pixels` | Get conversion pixels | `v3:ad-publishing.get_ad_publishing_facebook_pixels` |
| `PUT` | `/ad-publishing/facebook/pixels` | Upsert conversion pixel | `v3:ad-publishing.put_ad_publishing_facebook_pixels` |
| `GET` | `/ad-publishing/facebook/reporting` | Get reporting data | `v3:ad-publishing.get_ad_publishing_facebook_reporting` |
| `GET` | `/ad-publishing/facebook/reporting/campaign/{campaignId}` | Get campaign reporting | `v3:ad-publishing.get_ad_publishing_facebook_reporting_campaign_by_campaignId` |
| `GET` | `/ad-publishing/facebook/reporting/list` | Get reporting list | `v3:ad-publishing.get_ad_publishing_facebook_reporting_list` |
| `GET` | `/ad-publishing/facebook/targeting/search` | Search targeting options | `v3:ad-publishing.get_ad_publishing_facebook_targeting_search` |
| `GET` | `/ad-publishing/google/ad-accounts` | Get Google ad accounts | `v3:ad-publishing.get_ad_publishing_google_ad_accounts` |
| `DELETE` | `/ad-publishing/google/ad-accounts/{adAccountId}` | Delete ad account | `v3:ad-publishing.delete_ad_publishing_google_ad_accounts_by_adAccountId` |
| `GET` | `/ad-publishing/google/ad-accounts/{adAccountId}` | Get ad account details | `v3:ad-publishing.get_ad_publishing_google_ad_accounts_by_adAccountId` |
| `PUT` | `/ad-publishing/google/ads` | Upsert Google campaign | `v3:ad-publishing.put_ad_publishing_google_ads` |
| `GET` | `/ad-publishing/google/ads/{adId}` | Get Google campaign by ID | `v3:ad-publishing.get_ad_publishing_google_ads_by_adId` |
| `POST` | `/ad-publishing/google/ads/{adId}/publish` | Publish ad | `v3:ad-publishing.post_ad_publishing_google_ads_by_adId_publish` |
| `GET` | `/ad-publishing/google/assets` | Get assets | `v3:ad-publishing.get_ad_publishing_google_assets` |
| `POST` | `/ad-publishing/google/assets` | Upsert assets | `v3:ad-publishing.post_ad_publishing_google_assets` |
| `GET` | `/ad-publishing/google/audiences` | Get audiences | `v3:ad-publishing.get_ad_publishing_google_audiences` |
| `PUT` | `/ad-publishing/google/audiences` | Upsert audience | `v3:ad-publishing.put_ad_publishing_google_audiences` |
| `GET` | `/ad-publishing/google/audiences/{audienceId}` | Get audience by ID | `v3:ad-publishing.get_ad_publishing_google_audiences_by_audienceId` |
| `GET` | `/ad-publishing/google/conversion-goals` | Get conversion goals | `v3:ad-publishing.get_ad_publishing_google_conversion_goals` |
| `GET` | `/ad-publishing/google/conversions` | Get conversions | `v3:ad-publishing.get_ad_publishing_google_conversions` |
| `PUT` | `/ad-publishing/google/conversions` | Upsert conversion | `v3:ad-publishing.put_ad_publishing_google_conversions` |
| `DELETE` | `/ad-publishing/google/conversions/{conversionId}` | Delete conversion | `v3:ad-publishing.delete_ad_publishing_google_conversions_by_conversionId` |
| `GET` | `/ad-publishing/google/conversions/{conversionId}` | Get conversion by ID | `v3:ad-publishing.get_ad_publishing_google_conversions_by_conversionId` |
| `GET` | `/ad-publishing/google/entity` | Get entities | `v3:ad-publishing.get_ad_publishing_google_entity` |
| `GET` | `/ad-publishing/google/integration` | Get Google integration | `v3:ad-publishing.get_ad_publishing_google_integration` |
| `POST` | `/ad-publishing/google/integration` | Create Google integration | `v3:ad-publishing.post_ad_publishing_google_integration` |
| `POST` | `/ad-publishing/google/keyword-ideas` | Get keyword ideas | `v3:ad-publishing.post_ad_publishing_google_keyword_ideas` |
| `GET` | `/ad-publishing/google/me` | Get current Google user | `v3:ad-publishing.get_ad_publishing_google_me` |
| `GET` | `/ad-publishing/google/reporting` | Get reporting data | `v3:ad-publishing.get_ad_publishing_google_reporting` |
| `GET` | `/ad-publishing/google/reporting/campaign/{campaignId}` | Get campaign reporting | `v3:ad-publishing.get_ad_publishing_google_reporting_campaign_by_campaignId` |
| `GET` | `/ad-publishing/google/reporting/list` | Get reporting list | `v3:ad-publishing.get_ad_publishing_google_reporting_list` |
| `GET` | `/ad-publishing/google/segments` | Get segments | `v3:ad-publishing.get_ad_publishing_google_segments` |
| `PUT` | `/ad-publishing/google/segments` | Upsert segment | `v3:ad-publishing.put_ad_publishing_google_segments` |
| `POST` | `/ad-publishing/google/segments/offline-user-list-job` | Create offline user list job | `v3:ad-publishing.post_ad_publishing_google_segments_offline_user_list_job` |
| `DELETE` | `/ad-publishing/google/segments/{segmentId}` | Delete segment | `v3:ad-publishing.delete_ad_publishing_google_segments_by_segmentId` |
| `GET` | `/ad-publishing/google/segments/{segmentId}` | Get segment by ID | `v3:ad-publishing.get_ad_publishing_google_segments_by_segmentId` |
| `GET` | `/ad-publishing/google/target-interests` | Get target interests | `v3:ad-publishing.get_ad_publishing_google_target_interests` |
| `GET` | `/ad-publishing/google/targeting/search` | Search targeting options | `v3:ad-publishing.get_ad_publishing_google_targeting_search` |
| `DELETE` | `/ad-publishing/linkedin/ad-account` | Delete ad account | `v3:ad-publishing.delete_ad_publishing_linkedin_ad_account` |
| `GET` | `/ad-publishing/linkedin/ad-account` | Get ad account details | `v3:ad-publishing.get_ad_publishing_linkedin_ad_account` |
| `GET` | `/ad-publishing/linkedin/ad-accounts` | Get LinkedIn ad accounts | `v3:ad-publishing.get_ad_publishing_linkedin_ad_accounts` |
| `PUT` | `/ad-publishing/linkedin/ads` | Upsert ad campaign group | `v3:ad-publishing.put_ad_publishing_linkedin_ads` |
| `GET` | `/ad-publishing/linkedin/ads/{adId}` | Get ad campaign group | `v3:ad-publishing.get_ad_publishing_linkedin_ads_by_adId` |
| `POST` | `/ad-publishing/linkedin/ads/{adId}/publish` | Publish ad campaign group | `v3:ad-publishing.post_ad_publishing_linkedin_ads_by_adId_publish` |
| `GET` | `/ad-publishing/linkedin/integration` | Get LinkedIn integration | `v3:ad-publishing.get_ad_publishing_linkedin_integration` |
| `POST` | `/ad-publishing/linkedin/integration` | Create LinkedIn integration | `v3:ad-publishing.post_ad_publishing_linkedin_integration` |
| `GET` | `/ad-publishing/linkedin/me` | Get current LinkedIn user | `v3:ad-publishing.get_ad_publishing_linkedin_me` |
| `GET` | `/ad-publishing/linkedin/reporting` | Get ad analytics | `v3:ad-publishing.get_ad_publishing_linkedin_reporting` |
| `GET` | `/ad-publishing/linkedin/reporting/campaign-group/{campaignGroupId}` | Get campaign group reporting | `v3:ad-publishing.get_ad_publishing_linkedin_reporting_campaign_group_by_campaignGroupId` |
| `GET` | `/ad-publishing/linkedin/reporting/list` | Get reporting list | `v3:ad-publishing.get_ad_publishing_linkedin_reporting_list` |
| `GET` | `/ad-publishing/linkedin/targeting/search` | Search targeting options | `v3:ad-publishing.get_ad_publishing_linkedin_targeting_search` |
| `POST` | `/ad-publishing/linkedin/{accountId}/form` | Create lead form | `v3:ad-publishing.post_ad_publishing_linkedin_by_accountId_form` |
| `GET` | `/ad-publishing/linkedin/{accountId}/forms` | Get lead forms | `v3:ad-publishing.get_ad_publishing_linkedin_by_accountId_forms` |
| `PATCH` | `/ad-publishing/linkedin/{adId}/status` | Update ad status | `v3:ad-publishing.patch_ad_publishing_linkedin_by_adId_status` |

### Endpoint details — v3

#### `GET /ad-publishing/facebook/ad-accounts`

**Get ad accounts**

Retrieve Facebook ad accounts available for the connected user

Operation id: `v3:ad-publishing.get_ad_publishing_facebook_ad_accounts` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `type` | string | no | Account source type |
| `next` | string | no | Pagination cursor |
| `fetchAll` | string | no | Fetch all accounts |
| `limit` | string | no | Results page limit |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_facebook_ad_accounts",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `DELETE /ad-publishing/facebook/ad-accounts/{adAccountId}`

**Delete ad account**

Remove a Facebook ad account connection from a location

Operation id: `v3:ad-publishing.delete_ad_publishing_facebook_ad_accounts_by_adAccountId` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `adAccountId` | string | **yes** | Ad account identifier |

*Request body*: [`LocationIdBodyDTO`](#locationidbodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.delete_ad_publishing_facebook_ad_accounts_by_adAccountId",
    "path_params": {
      "adAccountId": "<adAccountId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/facebook/ad-accounts/{adAccountId}`

**Get ad account details**

Retrieve details of a specific Facebook ad account

Operation id: `v3:ad-publishing.get_ad_publishing_facebook_ad_accounts_by_adAccountId` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `adAccountId` | string | **yes** | Ad account identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_facebook_ad_accounts_by_adAccountId",
    "path_params": {
      "adAccountId": "<adAccountId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /ad-publishing/facebook/ads`

**Upsert ad**

Create or update a Facebook ad

Operation id: `v3:ad-publishing.put_ad_publishing_facebook_ads` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Request body*: [`UpsertAdDTO`](#upsertaddto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.put_ad_publishing_facebook_ads",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /ad-publishing/facebook/ads/{adId}`

**Delete ad**

Delete a Facebook ad by ID

Operation id: `v3:ad-publishing.delete_ad_publishing_facebook_ads_by_adId` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `adId` | string | **yes** | Ad identifier |

*Request body*: [`LocationIdBodyDTO`](#locationidbodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.delete_ad_publishing_facebook_ads_by_adId",
    "path_params": {
      "adId": "<adId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/facebook/ads/{adId}/duplicate`

**Duplicate ad**

Duplicate an existing Facebook ad

Operation id: `v3:ad-publishing.post_ad_publishing_facebook_ads_by_adId_duplicate` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `adId` | string | **yes** | Ad identifier |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_facebook_ads_by_adId_duplicate",
    "path_params": {
      "adId": "<adId>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/facebook/ads/{adId}/pause`

**Pause ad**

Pause a running Facebook ad

Operation id: `v3:ad-publishing.post_ad_publishing_facebook_ads_by_adId_pause` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `adId` | string | **yes** | Ad identifier |

*Request body*: [`LocationIdBodyDTO`](#locationidbodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_facebook_ads_by_adId_pause",
    "path_params": {
      "adId": "<adId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/facebook/ads/{adId}/resume`

**Resume ad**

Resume a paused Facebook ad

Operation id: `v3:ad-publishing.post_ad_publishing_facebook_ads_by_adId_resume` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `adId` | string | **yes** | Ad identifier |

*Request body*: [`LocationIdBodyDTO`](#locationidbodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_facebook_ads_by_adId_resume",
    "path_params": {
      "adId": "<adId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /ad-publishing/facebook/adsets`

**Upsert adset**

Create or update a Facebook ad set

Operation id: `v3:ad-publishing.put_ad_publishing_facebook_adsets` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Request body*: [`UpsertAdsetDTO`](#upsertadsetdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.put_ad_publishing_facebook_adsets",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /ad-publishing/facebook/adsets/{adSetId}`

**Delete ad set**

Delete a Facebook ad set by ID

Operation id: `v3:ad-publishing.delete_ad_publishing_facebook_adsets_by_adSetId` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `adSetId` | string | **yes** | Ad set identifier |

*Request body*: [`LocationIdBodyDTO`](#locationidbodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.delete_ad_publishing_facebook_adsets_by_adSetId",
    "path_params": {
      "adSetId": "<adSetId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/facebook/adsets/{adSetId}/duplicate`

**Duplicate ad set**

Duplicate an existing Facebook ad set

Operation id: `v3:ad-publishing.post_ad_publishing_facebook_adsets_by_adSetId_duplicate` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `adSetId` | string | **yes** | Ad set identifier |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_facebook_adsets_by_adSetId_duplicate",
    "path_params": {
      "adSetId": "<adSetId>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/facebook/adsets/{adSetId}/pause`

**Pause ad set**

Pause a running Facebook ad set

Operation id: `v3:ad-publishing.post_ad_publishing_facebook_adsets_by_adSetId_pause` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `adSetId` | string | **yes** | Ad set identifier |

*Request body*: [`LocationIdBodyDTO`](#locationidbodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_facebook_adsets_by_adSetId_pause",
    "path_params": {
      "adSetId": "<adSetId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/facebook/adsets/{adSetId}/resume`

**Resume ad set**

Resume a paused Facebook ad set

Operation id: `v3:ad-publishing.post_ad_publishing_facebook_adsets_by_adSetId_resume` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `adSetId` | string | **yes** | Ad set identifier |

*Request body*: [`LocationIdBodyDTO`](#locationidbodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_facebook_adsets_by_adSetId_resume",
    "path_params": {
      "adSetId": "<adSetId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/facebook/campaign/{campaignId}`

**Get campaign with linked entities**

Retrieve a Facebook campaign with its linked adsets and ads

Operation id: `v3:ad-publishing.get_ad_publishing_facebook_campaign_by_campaignId` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `campaignId` | string | **yes** | Campaign identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `fields` | string | no | Comma-separated field names |
| `source` | string | no | Campaign data source |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_facebook_campaign_by_campaignId",
    "path_params": {
      "campaignId": "<campaignId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /ad-publishing/facebook/campaigns`

**Upsert campaign**

Create or update a Facebook campaign

Operation id: `v3:ad-publishing.put_ad_publishing_facebook_campaigns` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Request body*: [`UpsertCampaignDTO`](#upsertcampaigndto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.put_ad_publishing_facebook_campaigns",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /ad-publishing/facebook/campaigns/{campaignId}`

**Delete campaign**

Delete a Facebook campaign by ID

Operation id: `v3:ad-publishing.delete_ad_publishing_facebook_campaigns_by_campaignId` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `campaignId` | string | **yes** | Campaign identifier |

*Request body*: [`LocationIdBodyDTO`](#locationidbodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.delete_ad_publishing_facebook_campaigns_by_campaignId",
    "path_params": {
      "campaignId": "<campaignId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/facebook/campaigns/{campaignId}/duplicate`

**Duplicate campaign**

Duplicate an existing Facebook campaign

Operation id: `v3:ad-publishing.post_ad_publishing_facebook_campaigns_by_campaignId_duplicate` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `campaignId` | string | **yes** | Campaign identifier |

*Request body*: [`LocationIdBodyDTO`](#locationidbodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_facebook_campaigns_by_campaignId_duplicate",
    "path_params": {
      "campaignId": "<campaignId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/facebook/campaigns/{campaignId}/pause`

**Pause campaign**

Pause a running Facebook campaign

Operation id: `v3:ad-publishing.post_ad_publishing_facebook_campaigns_by_campaignId_pause` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `campaignId` | string | **yes** | Campaign identifier |

*Request body*: [`LocationIdBodyDTO`](#locationidbodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_facebook_campaigns_by_campaignId_pause",
    "path_params": {
      "campaignId": "<campaignId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/facebook/campaigns/{campaignId}/publish`

**Publish campaign**

Publish a Facebook campaign and push it live to Facebook

Operation id: `v3:ad-publishing.post_ad_publishing_facebook_campaigns_by_campaignId_publish` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `campaignId` | string | **yes** | Campaign identifier |

*Request body*: [`PublishAdDTO`](#publishaddto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_facebook_campaigns_by_campaignId_publish",
    "path_params": {
      "campaignId": "<campaignId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/facebook/campaigns/{campaignId}/publishing-progress`

**Get campaign publishing progress**

Returns Redis-backed publish progress for a campaign while it is publishing to Meta. Used by the validation funnel UI to poll step counts and completion state.

Operation id: `v3:ad-publishing.get_ad_publishing_facebook_campaigns_by_campaignId_publishing_progress` · `Version: v3` · Scopes: `adPublishing.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `campaignId` | string | **yes** | Campaign identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

*Response*: [`PublishingProgressResponseDTO`](#publishingprogressresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_facebook_campaigns_by_campaignId_publishing_progress",
    "path_params": {
      "campaignId": "<campaignId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/facebook/campaigns/{campaignId}/resume`

**Resume campaign**

Resume a paused Facebook campaign

Operation id: `v3:ad-publishing.post_ad_publishing_facebook_campaigns_by_campaignId_resume` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `campaignId` | string | **yes** | Campaign identifier |

*Request body*: [`LocationIdBodyDTO`](#locationidbodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_facebook_campaigns_by_campaignId_resume",
    "path_params": {
      "campaignId": "<campaignId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/facebook/conversation-forms`

**Get conversation forms**

Retrieve Facebook conversation lead forms for a location

Operation id: `v3:ad-publishing.get_ad_publishing_facebook_conversation_forms` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_facebook_conversation_forms",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/facebook/conversation-forms`

**Create conversation form**

Create a new Facebook conversation lead form

Operation id: `v3:ad-publishing.post_ad_publishing_facebook_conversation_forms` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Request body*: [`CreateConversationFormDTO`](#createconversationformdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_facebook_conversation_forms",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/facebook/custom-audience`

**Get custom audiences**

Retrieve Facebook custom audiences for a location

Operation id: `v3:ad-publishing.get_ad_publishing_facebook_custom_audience` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `type` | enum: `lookalike`, `custom`, `all` | **yes** | Audience list type |
| `source` | enum: `ad_manager`, `integration` | no | Audience data source |
| `adAccountId` | string | **yes** | Ad account identifier |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_facebook_custom_audience",
    "query": {
      "locationId": "<locationId>",
      "type": "<type>",
      "adAccountId": "<adAccountId>"
    }
  }
}
```

</details>

#### `DELETE /ad-publishing/facebook/custom-audience/{audienceId}`

**Delete custom audience**

Delete a Facebook custom audience by ID

Operation id: `v3:ad-publishing.delete_ad_publishing_facebook_custom_audience_by_audienceId` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `audienceId` | string | **yes** | Custom audience identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.delete_ad_publishing_facebook_custom_audience_by_audienceId",
    "path_params": {
      "audienceId": "<audienceId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/facebook/custom-audience/{audienceId}`

**Get custom audience by ID**

Retrieve a specific Facebook custom audience by its ID

Operation id: `v3:ad-publishing.get_ad_publishing_facebook_custom_audience_by_audienceId` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `audienceId` | string | **yes** | Custom audience identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_facebook_custom_audience_by_audienceId",
    "path_params": {
      "audienceId": "<audienceId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /ad-publishing/facebook/custom-audience/{audienceId}`

**Update custom audience**

Update name or description of a Facebook custom audience

Operation id: `v3:ad-publishing.put_ad_publishing_facebook_custom_audience_by_audienceId` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `audienceId` | string | **yes** | Custom audience identifier |

*Request body*: [`FbUpdateAudienceBodyDTO`](#fbupdateaudiencebodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.put_ad_publishing_facebook_custom_audience_by_audienceId",
    "path_params": {
      "audienceId": "<audienceId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /ad-publishing/facebook/custom-audience/{audienceId}/member`

**Remove custom audience member**

Remove a member from a Facebook custom audience

Operation id: `v3:ad-publishing.delete_ad_publishing_facebook_custom_audience_by_audienceId_member` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `audienceId` | string | **yes** | Custom audience identifier |

*Request body*: [`UpdateCustomAudienceDTO`](#updatecustomaudiencedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.delete_ad_publishing_facebook_custom_audience_by_audienceId_member",
    "path_params": {
      "audienceId": "<audienceId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /ad-publishing/facebook/custom-audience/{audienceId}/member`

**Add custom audience member**

Add a member to a Facebook custom audience

Operation id: `v3:ad-publishing.put_ad_publishing_facebook_custom_audience_by_audienceId_member` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `audienceId` | string | **yes** | Custom audience identifier |

*Request body*: [`UpdateCustomAudienceDTO`](#updatecustomaudiencedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.put_ad_publishing_facebook_custom_audience_by_audienceId_member",
    "path_params": {
      "audienceId": "<audienceId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /ad-publishing/facebook/custom-audience/{audienceId}/member/batch`

**Batch update audience members**

Add or remove members in bulk from a Facebook custom audience via CSV or smart lists

Operation id: `v3:ad-publishing.put_ad_publishing_facebook_custom_audience_by_audienceId_member_batch` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `audienceId` | string | **yes** | Custom audience identifier |

*Request body*: [`UpdateCustomAudienceBatchDTO`](#updatecustomaudiencebatchdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.put_ad_publishing_facebook_custom_audience_by_audienceId_member_batch",
    "path_params": {
      "audienceId": "<audienceId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/facebook/entity`

**Get entities**

Retrieve Facebook campaigns, adsets, or ads based on entity type

Operation id: `v3:ad-publishing.get_ad_publishing_facebook_entity` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `type` | enum: `AD_MANAGER`, `INTEGRATION` | **yes** | Integration source type |
| `next` | string | no | Pagination cursor |
| `fetchAll` | string | no | Fetch all entities |
| `campaignId` | string | no | Campaign identifier |
| `adSetId` | string | no | Ad set identifier |
| `entityType` | enum: `CAMPAIGN`, `ADSET`, `AD` | **yes** | Entity type to fetch |
| `searchId` | string | no | Search identifier |
| `selectedAdAccountId` | string | no | Selected ad account ID |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_facebook_entity",
    "query": {
      "locationId": "<locationId>",
      "type": "<type>",
      "entityType": "<entityType>"
    }
  }
}
```

</details>

#### `DELETE /ad-publishing/facebook/integration`

**Delete Facebook integration**

Remove the Facebook ad integration from a location

Operation id: `v3:ad-publishing.delete_ad_publishing_facebook_integration` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Request body*: [`LocationIdBodyDTO`](#locationidbodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.delete_ad_publishing_facebook_integration",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/facebook/integration`

**Get Facebook integration**

Retrieve the Facebook ad integration details for a location

Operation id: `v3:ad-publishing.get_ad_publishing_facebook_integration` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_facebook_integration",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/facebook/integration`

**Create Facebook integration**

Create a Facebook ad integration for a location with page and ad account

Operation id: `v3:ad-publishing.post_ad_publishing_facebook_integration` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Request body*: [`CreateIntegrationDTO`](#createintegrationdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_facebook_integration",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/facebook/lead-form/{leadFormId}`

**Get lead form by ID**

Retrieve a specific Facebook lead form by its ID

Operation id: `v3:ad-publishing.get_ad_publishing_facebook_lead_form_by_leadFormId` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `leadFormId` | string | **yes** | Lead form identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_facebook_lead_form_by_leadFormId",
    "path_params": {
      "leadFormId": "<leadFormId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/facebook/me`

**Get current Facebook user**

Retrieve the authenticated Facebook user profile for a location

Operation id: `v3:ad-publishing.get_ad_publishing_facebook_me` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_facebook_me",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `DELETE /ad-publishing/facebook/page`

**Delete page connection**

Remove a Facebook page connection from a location

Operation id: `v3:ad-publishing.delete_ad_publishing_facebook_page` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `pageId` | string | **yes** | Facebook page ID |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.delete_ad_publishing_facebook_page",
    "query": {
      "locationId": "<locationId>",
      "pageId": "<pageId>"
    }
  }
}
```

</details>

#### `PUT /ad-publishing/facebook/page/default`

**Set default page**

Set the default Facebook page for a location

Operation id: `v3:ad-publishing.put_ad_publishing_facebook_page_default` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

*Request body*: [`FbSetDefaultPageBodyDTO`](#fbsetdefaultpagebodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.put_ad_publishing_facebook_page_default",
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

#### `GET /ad-publishing/facebook/page/{pageId}/forms`

**Get page lead forms**

Retrieve lead gen forms for a specific Facebook page

Operation id: `v3:ad-publishing.get_ad_publishing_facebook_page_by_pageId_forms` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `pageId` | string | **yes** | Facebook page identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_facebook_page_by_pageId_forms",
    "path_params": {
      "pageId": "<pageId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/facebook/page/{pageId}/forms`

**Create page lead form**

Create a new lead gen form on a Facebook page

Operation id: `v3:ad-publishing.post_ad_publishing_facebook_page_by_pageId_forms` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `pageId` | string | **yes** | Facebook page identifier |

*Request body*: [`CreateLeadFormDTO`](#createleadformdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_facebook_page_by_pageId_forms",
    "path_params": {
      "pageId": "<pageId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/facebook/page/{pageId}/instagram`

**Get Instagram accounts for page**

Retrieve Instagram accounts linked to a specific Facebook page

Operation id: `v3:ad-publishing.get_ad_publishing_facebook_page_by_pageId_instagram` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `pageId` | string | **yes** | Facebook page identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `type` | enum: `INTEGRATION`, `AD_MANAGER` | no | Integration type |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_facebook_page_by_pageId_instagram",
    "path_params": {
      "pageId": "<pageId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/facebook/pages`

**Get Facebook pages**

Retrieve Facebook pages associated with the connected account

Operation id: `v3:ad-publishing.get_ad_publishing_facebook_pages` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `fetchExisting` | string | no | Fetch existing pages flag |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_facebook_pages",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/facebook/pixels`

**Get conversion pixels**

Retrieve Facebook conversion pixels for a location

Operation id: `v3:ad-publishing.get_ad_publishing_facebook_pixels` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `channel` | string | no | Channel type |
| `pageId` | string | no | Facebook page ID |
| `igUserId` | string | no | Instagram user ID |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_facebook_pixels",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /ad-publishing/facebook/pixels`

**Upsert conversion pixel**

Create or update a Facebook conversion pixel configuration

Operation id: `v3:ad-publishing.put_ad_publishing_facebook_pixels` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Request body*: [`UpsertConversionPixelDTO`](#upsertconversionpixeldto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.put_ad_publishing_facebook_pixels",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/facebook/reporting`

**Get reporting data**

Retrieve aggregated Facebook ad reporting metrics for a location

Operation id: `v3:ad-publishing.get_ad_publishing_facebook_reporting` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `fields` | array | **yes** | Reporting fields. Pass as comma-separated values on the wire (e.g. ?fields=impressions,clicks). |
| `groupBy` | enum: `day`, `week`, `month` | **yes** | Time grouping interval |
| `startDate` | string | **yes** | Report start date |
| `endDate` | string | **yes** | Report end date |
| `type` | enum: `AD_MANAGER`, `INTEGRATION` | **yes** | Integration source type |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_facebook_reporting",
    "query": {
      "locationId": "<locationId>",
      "fields": "<fields>",
      "groupBy": "<groupBy>",
      "startDate": "<startDate>",
      "endDate": "<endDate>",
      "type": "<type>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/facebook/reporting/campaign/{campaignId}`

**Get campaign reporting**

Retrieve reporting metrics for a specific Facebook campaign

Operation id: `v3:ad-publishing.get_ad_publishing_facebook_reporting_campaign_by_campaignId` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `campaignId` | string | **yes** | Campaign identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `startDate` | string | **yes** | Report start date |
| `endDate` | string | **yes** | Report end date |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_facebook_reporting_campaign_by_campaignId",
    "path_params": {
      "campaignId": "<campaignId>"
    },
    "query": {
      "locationId": "<locationId>",
      "startDate": "<startDate>",
      "endDate": "<endDate>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/facebook/reporting/list`

**Get reporting list**

Retrieve a list of Facebook campaigns, adsets, or ads with reporting data

Operation id: `v3:ad-publishing.get_ad_publishing_facebook_reporting_list` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `listType` | string | **yes** | Reporting list type |
| `startDate` | string | **yes** | Report start date |
| `endDate` | string | **yes** | Report end date |
| `campaignId` | string | no | Campaign identifier (required when listType is adsets or ads) |
| `type` | enum: `AD_MANAGER`, `INTEGRATION` | **yes** | Integration source type |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_facebook_reporting_list",
    "query": {
      "locationId": "<locationId>",
      "listType": "<listType>",
      "startDate": "<startDate>",
      "endDate": "<endDate>",
      "type": "<type>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/facebook/targeting/search`

**Search targeting options**

Search Facebook geo-locations and interests for ad targeting

Operation id: `v3:ad-publishing.get_ad_publishing_facebook_targeting_search` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | no | Location identifier |
| `type` | string | **yes** | Targeting search type |
| `query` | string | **yes** | Search query string |
| `searchType` | string | no | Specific search subtype |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_facebook_targeting_search",
    "query": {
      "type": "<type>",
      "query": "<query>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/google/ad-accounts`

**Get Google ad accounts**

Retrieve Google Ads accounts available for the connected user

Operation id: `v3:ad-publishing.get_ad_publishing_google_ad_accounts` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `type` | enum: `INTEGRATION`, `AD_MANAGER` | no | Account type |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_google_ad_accounts",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `DELETE /ad-publishing/google/ad-accounts/{adAccountId}`

**Delete ad account**

Remove a Google Ads account connection from a location

Operation id: `v3:ad-publishing.delete_ad_publishing_google_ad_accounts_by_adAccountId` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `adAccountId` | string | **yes** | Ad account identifier |

*Request body*: [`LocationIdBodyDTO`](#locationidbodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.delete_ad_publishing_google_ad_accounts_by_adAccountId",
    "path_params": {
      "adAccountId": "<adAccountId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/google/ad-accounts/{adAccountId}`

**Get ad account details**

Retrieve details of a specific Google Ads account

Operation id: `v3:ad-publishing.get_ad_publishing_google_ad_accounts_by_adAccountId` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `adAccountId` | string | **yes** | Ad account identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_google_ad_accounts_by_adAccountId",
    "path_params": {
      "adAccountId": "<adAccountId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /ad-publishing/google/ads`

**Upsert Google campaign**

Create or update a full Google Ads campaign structure

Operation id: `v3:ad-publishing.put_ad_publishing_google_ads` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Request body*: [`CampaignDTO`](#campaigndto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.put_ad_publishing_google_ads",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/google/ads/{adId}`

**Get Google campaign by ID**

Retrieve a specific Google Ads campaign by ID

Operation id: `v3:ad-publishing.get_ad_publishing_google_ads_by_adId` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `adId` | string | **yes** | Ad identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_google_ads_by_adId",
    "path_params": {
      "adId": "<adId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/google/ads/{adId}/publish`

**Publish ad**

Publish a Google ad and push it live

Operation id: `v3:ad-publishing.post_ad_publishing_google_ads_by_adId_publish` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `adId` | string | **yes** | Ad identifier |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_google_ads_by_adId_publish",
    "path_params": {
      "adId": "<adId>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/google/assets`

**Get assets**

Retrieve Google Ads creative assets for a location

Operation id: `v3:ad-publishing.get_ad_publishing_google_assets` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `type` | enum: `CALL`, `SITELINK`, `LEAD_FORM`, `IMAGE`, `TEXT` | **yes** | Asset type to retrieve |
| `id` | string | no | Asset identifier |
| `advertiserOnly` | string | no | Advertiser only flag |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_google_assets",
    "query": {
      "locationId": "<locationId>",
      "type": "<type>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/google/assets`

**Upsert assets**

Create or update Google Ads creative assets

Operation id: `v3:ad-publishing.post_ad_publishing_google_assets` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Request body*: [`UpsertAssetsDTO`](#upsertassetsdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_google_assets",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/google/audiences`

**Get audiences**

Retrieve Google Ads combined audiences for a location

Operation id: `v3:ad-publishing.get_ad_publishing_google_audiences` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_google_audiences",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /ad-publishing/google/audiences`

**Upsert audience**

Create or update a Google Ads combined audience

Operation id: `v3:ad-publishing.put_ad_publishing_google_audiences` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Request body*: [`UpsertAudienceDTO`](#upsertaudiencedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.put_ad_publishing_google_audiences",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/google/audiences/{audienceId}`

**Get audience by ID**

Retrieve a specific Google Ads combined audience by ID

Operation id: `v3:ad-publishing.get_ad_publishing_google_audiences_by_audienceId` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `audienceId` | string | **yes** | Audience identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_google_audiences_by_audienceId",
    "path_params": {
      "audienceId": "<audienceId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/google/conversion-goals`

**Get conversion goals**

Retrieve Google Ads conversion goals for a location

Operation id: `v3:ad-publishing.get_ad_publishing_google_conversion_goals` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_google_conversion_goals",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/google/conversions`

**Get conversions**

Retrieve Google Ads conversion actions for a location

Operation id: `v3:ad-publishing.get_ad_publishing_google_conversions` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `type` | enum: `AD_MANAGER`, `AD_WORDS` | no | Integration type |
| `conversionType` | string | no | Conversion type |
| `category` | string | no | Conversion category |
| `startDate` | string | no | Filter start date |
| `endDate` | string | no | Filter end date |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_google_conversions",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /ad-publishing/google/conversions`

**Upsert conversion**

Create or update a Google Ads conversion action

Operation id: `v3:ad-publishing.put_ad_publishing_google_conversions` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Request body*: [`UpsertConversionDTO`](#upsertconversiondto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.put_ad_publishing_google_conversions",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /ad-publishing/google/conversions/{conversionId}`

**Delete conversion**

Delete a Google Ads conversion action by ID

Operation id: `v3:ad-publishing.delete_ad_publishing_google_conversions_by_conversionId` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `conversionId` | string | **yes** | Conversion identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.delete_ad_publishing_google_conversions_by_conversionId",
    "path_params": {
      "conversionId": "<conversionId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/google/conversions/{conversionId}`

**Get conversion by ID**

Retrieve a specific Google Ads conversion action by ID

Operation id: `v3:ad-publishing.get_ad_publishing_google_conversions_by_conversionId` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `conversionId` | string | **yes** | Conversion identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_google_conversions_by_conversionId",
    "path_params": {
      "conversionId": "<conversionId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/google/entity`

**Get entities**

Retrieve Google campaigns, ad groups, or ads based on entity type

Operation id: `v3:ad-publishing.get_ad_publishing_google_entity` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `type` | enum: `AD_MANAGER`, `INTEGRATION` | **yes** | Integration type |
| `campaignId` | string | no | Campaign identifier |
| `adGroupId` | string | no | Ad group identifier |
| `entityType` | enum: `CAMPAIGN`, `ADGROUP`, `AD` | **yes** | Entity type |
| `searchId` | string | no | Search identifier |
| `startDate` | string | no | Filter start date |
| `endDate` | string | no | Filter end date |
| `selectedAdAccountId` | string | no | Selected ad account ID |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_google_entity",
    "query": {
      "locationId": "<locationId>",
      "type": "<type>",
      "entityType": "<entityType>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/google/integration`

**Get Google integration**

Retrieve the Google Ads integration details for a location

Operation id: `v3:ad-publishing.get_ad_publishing_google_integration` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_google_integration",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/google/integration`

**Create Google integration**

Create a Google Ads integration for a location

Operation id: `v3:ad-publishing.post_ad_publishing_google_integration` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Request body*: [`CreateGoogleIntegrationDTO`](#creategoogleintegrationdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_google_integration",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/google/keyword-ideas`

**Get keyword ideas**

Retrieve keyword suggestions for Google Ads campaigns

Operation id: `v3:ad-publishing.post_ad_publishing_google_keyword_ideas` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

*Request body*: [`KeywordSuggestionDTO`](#keywordsuggestiondto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_google_keyword_ideas",
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

#### `GET /ad-publishing/google/me`

**Get current Google user**

Retrieve the authenticated Google user info for a location

Operation id: `v3:ad-publishing.get_ad_publishing_google_me` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_google_me",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/google/reporting`

**Get reporting data**

Retrieve aggregated Google Ads reporting metrics for a location

Operation id: `v3:ad-publishing.get_ad_publishing_google_reporting` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `fields` | array | **yes** | Reporting fields. Pass as comma-separated values on the wire (e.g. ?fields=impressions,clicks). |
| `groupBy` | enum: `date`, `week`, `month` | no | Group by period |
| `startDate` | string | **yes** | Report start date |
| `endDate` | string | **yes** | Report end date |
| `type` | enum: `AD_MANAGER`, `INTEGRATION` | **yes** | Integration type |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_google_reporting",
    "query": {
      "locationId": "<locationId>",
      "fields": "<fields>",
      "startDate": "<startDate>",
      "endDate": "<endDate>",
      "type": "<type>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/google/reporting/campaign/{campaignId}`

**Get campaign reporting**

Retrieve reporting metrics for a specific Google campaign

Operation id: `v3:ad-publishing.get_ad_publishing_google_reporting_campaign_by_campaignId` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `campaignId` | string | **yes** | Campaign identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `startDate` | string | **yes** | Report start date |
| `endDate` | string | **yes** | Report end date |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_google_reporting_campaign_by_campaignId",
    "path_params": {
      "campaignId": "<campaignId>"
    },
    "query": {
      "locationId": "<locationId>",
      "startDate": "<startDate>",
      "endDate": "<endDate>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/google/reporting/list`

**Get reporting list**

Retrieve a list of Google campaigns or ad groups with reporting data

Operation id: `v3:ad-publishing.get_ad_publishing_google_reporting_list` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `listType` | string | **yes** | Report list type |
| `startDate` | string | **yes** | Report start date |
| `endDate` | string | **yes** | Report end date |
| `campaignId` | string | no | Campaign identifier (required when listType is adGroups, ads, or keywords) |
| `type` | enum: `AD_MANAGER`, `INTEGRATION` | **yes** | Integration type |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_google_reporting_list",
    "query": {
      "locationId": "<locationId>",
      "listType": "<listType>",
      "startDate": "<startDate>",
      "endDate": "<endDate>",
      "type": "<type>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/google/segments`

**Get segments**

Retrieve Google Ads audience segments for a location

Operation id: `v3:ad-publishing.get_ad_publishing_google_segments` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `type` | string | no | Segment type |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_google_segments",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /ad-publishing/google/segments`

**Upsert segment**

Create or update a Google Ads audience segment

Operation id: `v3:ad-publishing.put_ad_publishing_google_segments` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `type` | enum: `CUSTOM_SEGMENTS`, `WEBSITE_VISITOR`, `CUSTOMER_MATCH`, `LOOKALIKE` | **yes** | Segment type |

*Request body*: [`UpsertSegmentDTO`](#upsertsegmentdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.put_ad_publishing_google_segments",
    "query": {
      "locationId": "<locationId>",
      "type": "<type>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/google/segments/offline-user-list-job`

**Create offline user list job**

Create a job to upload users to a Google customer match list

Operation id: `v3:ad-publishing.post_ad_publishing_google_segments_offline_user_list_job` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Request body*: [`CreateOfflineUserListJobDTO`](#createofflineuserlistjobdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_google_segments_offline_user_list_job",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /ad-publishing/google/segments/{segmentId}`

**Delete segment**

Delete a Google Ads audience segment by ID

Operation id: `v3:ad-publishing.delete_ad_publishing_google_segments_by_segmentId` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `segmentId` | string | **yes** | Segment identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `type` | enum: `CUSTOM_SEGMENTS`, `DATA_SEGMENTS` | **yes** | Segment type |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.delete_ad_publishing_google_segments_by_segmentId",
    "path_params": {
      "segmentId": "<segmentId>"
    },
    "query": {
      "locationId": "<locationId>",
      "type": "<type>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/google/segments/{segmentId}`

**Get segment by ID**

Retrieve a specific Google Ads audience segment by ID

Operation id: `v3:ad-publishing.get_ad_publishing_google_segments_by_segmentId` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `segmentId` | string | **yes** | Segment identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `type` | enum: `CUSTOM_SEGMENTS`, `DATA_SEGMENTS` | **yes** | Segment type |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_google_segments_by_segmentId",
    "path_params": {
      "segmentId": "<segmentId>"
    },
    "query": {
      "locationId": "<locationId>",
      "type": "<type>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/google/target-interests`

**Get target interests**

Retrieve affinity and in-market audience options for Google Ads targeting

Operation id: `v3:ad-publishing.get_ad_publishing_google_target_interests` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `type` | enum: `AFFINITY`, `IN_MARKET` | **yes** | Interest type |
| `advertisingChannelType` | string | **yes** | Channel type |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_google_target_interests",
    "query": {
      "locationId": "<locationId>",
      "type": "<type>",
      "advertisingChannelType": "<advertisingChannelType>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/google/targeting/search`

**Search targeting options**

Search Google geo-locations for ad targeting

Operation id: `v3:ad-publishing.get_ad_publishing_google_targeting_search` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `type` | string | **yes** | Search type |
| `query` | string | no | Search query |
| `locationId` | string | **yes** | Location identifier |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_google_targeting_search",
    "query": {
      "type": "<type>",
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `DELETE /ad-publishing/linkedin/ad-account`

**Delete ad account**

Remove a LinkedIn ad account connection from a location

Operation id: `v3:ad-publishing.delete_ad_publishing_linkedin_ad_account` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `adAccountId` | string | **yes** | Ad account identifier |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.delete_ad_publishing_linkedin_ad_account",
    "query": {
      "locationId": "<locationId>",
      "adAccountId": "<adAccountId>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/linkedin/ad-account`

**Get ad account details**

Retrieve details of a specific LinkedIn ad account

Operation id: `v3:ad-publishing.get_ad_publishing_linkedin_ad_account` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `adAccountId` | string | **yes** | Ad account identifier |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_linkedin_ad_account",
    "query": {
      "locationId": "<locationId>",
      "adAccountId": "<adAccountId>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/linkedin/ad-accounts`

**Get LinkedIn ad accounts**

Retrieve LinkedIn Ads accounts available for the connected user

Operation id: `v3:ad-publishing.get_ad_publishing_linkedin_ad_accounts` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_linkedin_ad_accounts",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /ad-publishing/linkedin/ads`

**Upsert ad campaign group**

Create or update a LinkedIn ad campaign group with campaigns and ads

Operation id: `v3:ad-publishing.put_ad_publishing_linkedin_ads` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Request body*: [`AdCampaignGroupDataDTO`](#adcampaigngroupdatadto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.put_ad_publishing_linkedin_ads",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/linkedin/ads/{adId}`

**Get ad campaign group**

Retrieve a LinkedIn ad campaign group by ID

Operation id: `v3:ad-publishing.get_ad_publishing_linkedin_ads_by_adId` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `adId` | string | **yes** | Ad identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_linkedin_ads_by_adId",
    "path_params": {
      "adId": "<adId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/linkedin/ads/{adId}/publish`

**Publish ad campaign group**

Publish a LinkedIn ad campaign group and push it live

Operation id: `v3:ad-publishing.post_ad_publishing_linkedin_ads_by_adId_publish` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `adId` | string | **yes** | Ad identifier |

*Request body*: [`LocationIdBodyDTO`](#locationidbodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_linkedin_ads_by_adId_publish",
    "path_params": {
      "adId": "<adId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/linkedin/integration`

**Get LinkedIn integration**

Retrieve the LinkedIn Ads integration details for a location

Operation id: `v3:ad-publishing.get_ad_publishing_linkedin_integration` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_linkedin_integration",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/linkedin/integration`

**Create LinkedIn integration**

Create a LinkedIn Ads integration for a location with ad account details

Operation id: `v3:ad-publishing.post_ad_publishing_linkedin_integration` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Request body*: [`CreateLinkedinIntegrationDTO`](#createlinkedinintegrationdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_linkedin_integration",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/linkedin/me`

**Get current LinkedIn user**

Retrieve the authenticated LinkedIn user info for a location

Operation id: `v3:ad-publishing.get_ad_publishing_linkedin_me` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_linkedin_me",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/linkedin/reporting`

**Get ad analytics**

Retrieve LinkedIn Ads analytics data with configurable pivot and time grouping

Operation id: `v3:ad-publishing.get_ad_publishing_linkedin_reporting` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `pivot` | enum: `ACCOUNT`, `CAMPAIGN`, `CAMPAIGN_GROUP`, `CREATIVE` | no | Analytics pivot type |
| `groupBy` | enum: `day`, `month`, `year` | no | Time granularity for analytics |
| `startDate` | string | **yes** | Start date in yyyy-mm-dd format |
| `endDate` | string | **yes** | End date in yyyy-mm-dd format |
| `entityUrns` | string | no | Comma-separated list of entity URNs |
| `fields` | array | no | Reporting fields. Pass as comma-separated values on the wire (e.g. ?fields=impressions,clicks). |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_linkedin_reporting",
    "query": {
      "locationId": "<locationId>",
      "startDate": "<startDate>",
      "endDate": "<endDate>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/linkedin/reporting/campaign-group/{campaignGroupId}`

**Get campaign group reporting**

Retrieve reporting metrics for a specific LinkedIn campaign group

Operation id: `v3:ad-publishing.get_ad_publishing_linkedin_reporting_campaign_group_by_campaignGroupId` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `campaignGroupId` | string | **yes** | Campaign group identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `startDate` | string | **yes** | Start date in yyyy-mm-dd format |
| `endDate` | string | **yes** | End date in yyyy-mm-dd format |
| `fields` | array | no | Reporting fields. Pass as comma-separated values on the wire (e.g. ?fields=impressions,clicks). |
| `campaignGroupId` | string | no | Campaign group ID |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_linkedin_reporting_campaign_group_by_campaignGroupId",
    "path_params": {
      "campaignGroupId": "<campaignGroupId>"
    },
    "query": {
      "locationId": "<locationId>",
      "startDate": "<startDate>",
      "endDate": "<endDate>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/linkedin/reporting/list`

**Get reporting list**

Retrieve a list of LinkedIn campaigns or campaign groups with reporting data

Operation id: `v3:ad-publishing.get_ad_publishing_linkedin_reporting_list` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `listType` | string | **yes** | List type |
| `campaignId` | string | **yes** | Campaign ID |
| `campaignGroupId` | string | **yes** | Campaign group ID |
| `startDate` | string | **yes** | Start date in yyyy-mm-dd format |
| `endDate` | string | **yes** | End date in yyyy-mm-dd format |
| `fields` | array | no | Reporting fields. Pass as comma-separated values on the wire (e.g. ?fields=impressions,clicks). |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_linkedin_reporting_list",
    "query": {
      "locationId": "<locationId>",
      "listType": "<listType>",
      "campaignId": "<campaignId>",
      "campaignGroupId": "<campaignGroupId>",
      "startDate": "<startDate>",
      "endDate": "<endDate>"
    }
  }
}
```

</details>

#### `GET /ad-publishing/linkedin/targeting/search`

**Search targeting options**

Search LinkedIn targeting facets such as locations, industries, and job titles

Operation id: `v3:ad-publishing.get_ad_publishing_linkedin_targeting_search` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `facet` | string | **yes** | Targeting facet |
| `query` | string | no | Search query |
| `q` | string | no | Query parameter |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_linkedin_targeting_search",
    "query": {
      "locationId": "<locationId>",
      "facet": "<facet>"
    }
  }
}
```

</details>

#### `POST /ad-publishing/linkedin/{accountId}/form`

**Create lead form**

Create a new LinkedIn lead gen form for an ad account

Operation id: `v3:ad-publishing.post_ad_publishing_linkedin_by_accountId_form` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

*Request body*: [`LinkedInCreateLeadFormBodyDTO`](#linkedincreateleadformbodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.post_ad_publishing_linkedin_by_accountId_form",
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

#### `GET /ad-publishing/linkedin/{accountId}/forms`

**Get lead forms**

Retrieve LinkedIn lead gen forms for an ad account

Operation id: `v3:ad-publishing.get_ad_publishing_linkedin_by_accountId_forms` · `Version: 2021-07-28` · Scopes: `adPublishing.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `accountId` | string | **yes** | Account identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.get_ad_publishing_linkedin_by_accountId_forms",
    "path_params": {
      "accountId": "<accountId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PATCH /ad-publishing/linkedin/{adId}/status`

**Update ad status**

Pause or resume a LinkedIn ad, campaign, or ad group

Operation id: `v3:ad-publishing.patch_ad_publishing_linkedin_by_adId_status` · `Version: 2021-07-28` · Scopes: `adPublishing.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `adId` | string | **yes** | Ad identifier |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location identifier |
| `isDraft` | boolean | no | Is draft |

*Request body*: [`LinkedInUpdateAdStatusBodyDTO`](#linkedinupdateadstatusbodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:ad-publishing.patch_ad_publishing_linkedin_by_adId_status",
    "path_params": {
      "adId": "<adId>"
    },
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

## Data models — API v3

In Rust: `ghl_models::v3::ad_publishing::*` (enable the `ad-publishing` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/ad_publishing/).

### `AdCampaignDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Internal campaign identifier |
| `locale` | [`LocaleDTO`](#localedto) | no | Campaign locale |
| `name` | String | no | Campaign name |
| `publishingStatus` | String — `DRAFT`, `SCHEDULED`, `PUBLISHED`, `PUBLISHING`, `FAILED`, `IN_REVIEW`, `PAUSED`, `ARCHIVED`, `WITH_ISSUES`, `REJECTED` | no | Publishing status |
| `mediaType` | String — `STANDARD_UPDATE`, `SINGLE_VIDEO`, `CAROUSEL` | no | Creative media type for the campaign |
| `audience` | [`AudienceDTO`](#audiencedto) | no | Campaign audience targeting |
| `unitCost` | [`UnitCostDTO`](#unitcostdto) | no | Bid unit cost |
| `campaignType` | String | no | LinkedIn campaign type |
| `adCampaignGroupId` | String | no | Parent campaign group identifier |
| `adCampaignId` | String | no | LinkedIn campaign resource ID |
| `ads` | Vec<LinkedInAdDTO> | no | Ads in the campaign |
| `linkedInError` | String | no | LinkedIn API error message |
| `meta` | JSON | no | Additional campaign metadata |

### `AdCampaignGroupDataDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Internal ID |
| `locationId` | String | **yes** | Location ID |
| `budget` | [`LinkedInBudgetDTO`](#linkedinbudgetdto) | no | Campaign group budget |
| `adCampaigns` | Vec<AdCampaignDTO> | no | Child ad campaigns |
| `adBudgetOptimization` | String — `MAXIMUM_DELIVERY`, `COST_CAP` | no | Ad budget optimization mode |
| `objectiveType` | String — `LEAD_GENERATION`, `WEBSITE_VISIT` | no | Campaign group objective |
| `name` | String | no | Campaign group name |
| `adCampaignGroupId` | String | no | LinkedIn campaign group resource ID |
| `publishingStatus` | String — `DRAFT`, `SCHEDULED`, `PUBLISHED`, `PUBLISHING`, `FAILED`, `IN_REVIEW`, `PAUSED`, `ARCHIVED`, `WITH_ISSUES`, `REJECTED` | no | Publishing status |
| `linkedInAdAccountId` | String | no | LinkedIn ad account identifier |
| `unpublishedChanges` | bool | no | Whether the campaign group has unpublished changes |
| `meta` | JSON | no | Additional metadata |
| `linkedInError` | String | no | LinkedIn API error message |

### `AdScheduleTargetDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `startMinute` | String — `ZERO`, `FIFTEEN`, `THIRTY`, `FORTY_FIVE` | **yes** | Minute mark the schedule starts at |
| `endMinute` | String — `ZERO`, `FIFTEEN`, `THIRTY`, `FORTY_FIVE` | **yes** | Minute mark the schedule ends at |
| `dayOfWeek` | String — `MONDAY`, `TUESDAY`, `WEDNESDAY`, `THURSDAY`, `FRIDAY`, `SATURDAY`, `SUNDAY` | **yes** | Day of the week for this schedule |
| `startHour` | f64 | **yes** | Start hour in 24h format (0-23) |
| `endHour` | f64 | **yes** | End hour in 24h format (0-23) |

### `AudienceCustomAudienceItemDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Custom audience ID |
| `name` | String | **yes** | Custom audience name |

### `AudienceDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `geoLocations` | Vec<GeoLocationDTO> | no | Geographic location targets |
| `targetAudience` | [`TargetAudienceDTO`](#targetaudiencedto) | no | Target audience attribute selections |

### `AudienceDimensionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `isAgeUnknown` | bool | no | Include unknown age |
| `ageRanges` | Vec<String> | no | Age range filters |
| `genders` | Vec<String> | no | Gender targets |
| `parentalStatuses` | Vec<String> | no | Parental status targets |
| `audienceSegments` | [`AudienceSegmentsDTO`](#audiencesegmentsdto) | no | Audience segment references used for targeting |

### `AudienceInterestDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Interest ID |
| `name` | String | **yes** | Interest name |
| `type` | String | no | Interest category type (defaults to "interests" if omitted) |

### `AudienceLocaleDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Locale display name |
| `key` | f64 | **yes** | Facebook locale key |

### `AudienceLocationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `key` | String | **yes** | Facebook location key |
| `name` | String | **yes** | Location display name |
| `type` | String — `country`, `city`, `region`, `country_group`, `geo_market`, `large_geo_area`, `medium_geo_area`, `small_geo_area`, `subcity`, `neighborhood`, `zip`, `address` | **yes** | Geographic location type |
| `selectionType` | String — `include`, `exclude` | **yes** | Whether the location is included or excluded from targeting |
| `radius` | f64 | no | Targeting radius around the location (for city/address types) |
| `radiusUnit` | String — `km`, `mi` | no | Unit for the targeting radius |
| `geometry` | [`AudienceLocationGeometry`](#audiencelocationgeometry) | no | Geometry data for address-based targeting |

### `AudienceLocationGeometry`

| Field | Type | Required | Description |
|---|---|---|---|
| `location` | JSON | **yes** | Geographic coordinates |
| `locationType` | String | **yes** | Geocoding result type |

### `AudiencePlacementsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `facebook` | Vec<String> | no | Facebook placement positions |
| `instagram` | Vec<String> | no | Instagram placement positions |
| `messenger` | Vec<String> | no | Messenger placement positions |

### `AudienceSegmentsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `customAudiences` | Vec<String> | no | Resource names of custom audience segments |
| `userLists` | Vec<String> | no | Resource names of user lists (remarketing lists, customer match lists, etc.) |
| `userInterests` | Vec<String> | no | Resource names of user interest segments (in-market or affinity audiences) |

### `Budget`

| Field | Type | Required | Description |
|---|---|---|---|
| `budgetType` | String — `DAILY`, `LIFETIME` | **yes** | Budget type |
| `amount` | f64 | **yes** | Budget amount |
| `scheduleStartDate` | String | no | Schedule start date |
| `scheduleEndDate` | String | no | Schedule end date |

### `CallAssetPayloadDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `phoneNumber` | String | **yes** | Phone number for call ads |
| `countryCode` | String | **yes** | Two-letter ISO country code |
| `callConversionAction` | String | no | Call conversion action resource name |
| `adScheduleTargets` | Vec<AdScheduleTargetDTO> | no | Ad schedule targets restricting when the call asset is shown |
| `resourceName` | String | no | Google Ads resource name for an existing call asset |

### `CampaignDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Campaign identifier |
| `name` | String | **yes** | Campaign name |
| `locationId` | String | **yes** | Location identifier |
| `advertisingChannelType` | String — `SEARCH`, `DISCOVERY`, `DISPLAY`, `HOTEL`, `LOCAL`, `MULTI_CHANNEL`, `PERFORMANCE_MAX`, `DEMAND_GEN` | **yes** | Advertising channel |
| `advertisingChannelSubType` | String — `DEMAND_GEN` | no | Channel sub type |
| `goalType` | String — `WEBSITE_TRAFFIC`, `LEAD` | no | Goal type |
| `budget` | [`GoogleBudgetDTO`](#googlebudgetdto) | no | Campaign budget |
| `audience` | [`GoogleCampaignAudienceDTO`](#googlecampaignaudiencedto) | no | Campaign audience targeting |
| `networkSettings` | [`GoogleNetworkSettingsDTO`](#googlenetworksettingsdto) | no | Network settings |
| `biddingStrategy` | [`GoogleBiddingStrategyDTO`](#googlebiddingstrategydto) | no | Bidding strategy config |
| `assets` | [`GoogleAssetsDTO`](#googleassetsdto) | no | Campaign assets |
| `isEuPoliticalAds` | bool | no | EU political ads flag |
| `adGroups` | Vec<GoogleAdGroupDTO> | no | Campaign ad groups |
| `campaignGoal` | [`GoogleCampaignGoalDTO`](#googlecampaigngoaldto) | no | Campaign goal config |
| `adSchedule` | Vec<GoogleAdScheduleDTO> | no | Ad schedule rules |
| `publishingStatus` | String — `DRAFT`, `SCHEDULED`, `PUBLISHED`, `PUBLISHING`, `FAILED`, `IN_REVIEW`, `PAUSED`, `ARCHIVED`, `WITH_ISSUES`, `REJECTED` | no | Publishing status |
| `googleAdAccountId` | String | no | Google Ad account identifier |
| `unpublishedChanges` | bool | no | Whether the campaign has unpublished changes |
| `maximumCpc` | f64 | no | Maximum CPC bid in micros |
| `googleCampaignId` | String | no | Google Ads campaign resource ID |
| `source` | String | no | Traffic source |
| `advancedOptions` | JSON | no | Advanced options |

### `ConsentDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `checkRequired` | bool | **yes** | Whether consent checkbox is required |
| `id` | f64 | **yes** | Consent identifier |
| `consent` | [`LocalizedStringDTO`](#localizedstringdto) | **yes** | Consent text |

### `ConversionValueSettings`

| Field | Type | Required | Description |
|---|---|---|---|
| `defaultValue` | f64 | **yes** | Default monetary value assigned to each conversion |
| `defaultCurrencyCode` | String | **yes** | ISO 4217 currency code for the default value |
| `alwaysUseDefaultValue` | bool | **yes** | When true, always uses the default value even if a transaction-specific value is provided |

### `CreateConversationFormDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location identifier |
| `name` | String | **yes** | Conversation form name |
| `text` | String | **yes** | Welcome message text |
| `questions` | Vec<WelcomeMessageQuestion> | **yes** | Quick-reply questions shown in the welcome message of the conversation form |

### `CreateGoogleIntegrationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location identifier |
| `adAccountId` | String | **yes** | Ad account identifier |
| `mccId` | String | **yes** | MCC identifier |

### `CreateIntegrationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location identifier |
| `pageId` | String | **yes** | Facebook page ID |
| `adAccountId` | String | no | Ad account identifier |

### `CreateLeadFormDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `MORE_VOLUME`, `HIGHER_INTENT` | **yes** | Lead form type |
| `name` | String | **yes** | Lead form name |
| `locationId` | String | **yes** | Location identifier |
| `greetingCard` | [`GreetingCard`](#greetingcard) | no | Greeting card config |
| `questions` | Vec<FormQuestion> | **yes** | List of questions displayed on the lead form. Required (non-empty) when `isDraft` is false or omitted; optional for drafts. |
| `questionPageHeadline` | String | no | Question page headline |
| `privacyPolicyLink` | String | **yes** | Privacy policy URL. Required when `isDraft` is false or omitted; optional for drafts. |
| `privacyPolicyText` | String | no | Privacy policy text |
| `customDisclaimer` | [`CustomDisclaimer`](#customdisclaimer) | no | Custom disclaimer config |
| `thankYouPage` | [`ThankYouPage`](#thankyoupage) | **yes** | Thank you page config. Required when `isDraft` is false or omitted; optional for drafts. |
| `isDraft` | bool | no | If the form is a draft, set to true |
| `draftFormId` | String | no | Draft form ID |
| `locale` | String | no | Locale |

### `CreateLinkedinIntegrationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location identifier |
| `adAccountId` | String | **yes** | Ad account identifier |
| `adAccountName` | String | **yes** | Ad account name |
| `currencyCode` | String | **yes** | Currency code |
| `organizationId` | String | **yes** | Organization identifier |

### `CreateOfflineUserListJobDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location identifier |
| `smartListIds` | Vec<String> | no | Smart list IDs |
| `csvPath` | String | no | CSV file path |
| `userListId` | String | no | User list identifier |
| `isDynamic` | bool | no | Dynamic list flag |

### `CreationLocaleDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `country` | String | **yes** | Country code |
| `language` | String | **yes** | Language code |

### `CustomDisclaimer`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | Disclaimer title |
| `body` | String | **yes** | Disclaimer body text |
| `checkboxes` | Vec<CustomDisclaimerCheckbox> | no | Consent checkboxes the user must agree to before submitting the form |

### `CustomDisclaimerCheckbox`

| Field | Type | Required | Description |
|---|---|---|---|
| `isRequired` | bool | **yes** | Checkbox required flag |
| `text` | String | **yes** | Checkbox text label |
| `key` | String | **yes** | Checkbox unique key |

### `CustomQuestionFieldDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `customQuestionText` | String | **yes** | Custom question text shown to the user |
| `singleChoiceAnswers` | Vec<String> | **yes** | Answer choices for the custom question |

### `FacebookAudienceDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `geoLocations` | Vec<AudienceLocationDTO> | **yes** | Geographic locations to target or exclude |
| `locales` | Vec<AudienceLocaleDTO> | no | Language locales to target |
| `placements` | [`AudiencePlacementsDTO`](#audienceplacementsdto) | no | Ad placement positions per platform (only used when placementType is "manual") |
| `placementType` | String — `auto`, `manual` | no | Placement strategy — "auto" lets Facebook choose, "manual" uses the placements config |
| `lookalike` | Vec<AudienceCustomAudienceItemDTO> | no | Lookalike audiences to target |
| `retargeting` | Vec<AudienceCustomAudienceItemDTO> | no | Retargeting custom audiences to target |
| `interests` | Vec<AudienceInterestDTO> | no | Interest-based targeting criteria |
| `ageMin` | f64 | no | Minimum age for targeting |
| `ageMax` | f64 | no | Maximum age for targeting |
| `genders` | Vec<f64> | no | Gender targeting (1 = male, 2 = female) |

### `FbSetDefaultPageBodyDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `pageId` | String | **yes** | Facebook page identifier |

### `FbUpdateAudienceBodyDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location identifier |
| `name` | String | **yes** | Audience name |
| `description` | String | no | Audience description |

### `FlexibleRuleUserListDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `inclusiveRuleOperator` | String | no | Operator for combining inclusive operands |
| `inclusiveOperands` | Vec<RuleOperandDTO> | **yes** | Inclusive rule operands |
| `exclusiveOperands` | Vec<RuleOperandDTO> | **yes** | Exclusive rule operands |

### `FormQuestion`

| Field | Type | Required | Description |
|---|---|---|---|
| `label` | String | no | Question label text shown to the user |
| `key` | String | **yes** | Question key |
| `type` | String — `CUSTOM`, `CITY`, `COMPANY_NAME`, `COUNTRY`, `DATE_OF_BIRTH`, `EMAIL`, `FIRST_NAME`, `FULL_NAME`, `GENDER`, `JOB_TITLE`, `LAST_NAME`, `MARITIAL_STATUS`, `MILITARY_STATUS`, `PHONE`, `POST_CODE`, `RELATIONSHIP_STATUS`, `STATE`, `STREET_ADDRESS`, `WORK_EMAIL`, `WORK_PHONE_NUMBER`, `ZIP`, `SHORT_ANSWER` | **yes** | Question input type — use a prefilled type for standard fields or CUSTOM / SHORT_ANSWER for freeform questions |
| `options` | Vec<FormQuestionOption> | no | Answer options for multiple-choice questions (only applies to CUSTOM type) |

### `FormQuestionOption`

| Field | Type | Required | Description |
|---|---|---|---|
| `key` | String | **yes** | Option key |
| `value` | String | **yes** | Option value |

### `GeoAddressComponentDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `longName` | String | no | Full name of the address component |
| `shortName` | String | no | Abbreviated name of the address component |
| `types` | Vec<String> | no | Address component types |

### `GeoGeometryDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `location` | [`GeoLatLngDTO`](#geolatlngdto) | no | Location coordinates |
| `locationType` | String | no | Location type (e.g. APPROXIMATE) |
| `viewport` | [`GeoViewportDTO`](#geoviewportdto) | no | Viewport bounding box |

### `GeoLatLngDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `lat` | f64 | no | Latitude |
| `lng` | f64 | no | Longitude |

### `GeoLocationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Location display name |
| `urn` | String | **yes** | Location URN |
| `facetUrn` | String | **yes** | Facet URN |
| `selectionType` | String — `include`, `exclude` | **yes** | Selection type |

### `GeoViewportDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `northeast` | [`GeoLatLngDTO`](#geolatlngdto) | no | Northeast corner of the viewport |
| `southwest` | [`GeoLatLngDTO`](#geolatlngdto) | no | Southwest corner of the viewport |

### `GoogleAdContentDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Ad identifier |
| `name` | String | no | Ad name |
| `mediaType` | String — `IMAGE`, `VIDEO`, `CAROUSEL` | no | Media type |
| `headlines` | Vec<String> | no | Ad headlines |
| `longHeadlines` | Vec<String> | no | Long headlines |
| `descriptions` | Vec<String> | no | Ad descriptions |
| `finalUrl` | String | no | Final URL |
| `path1` | String | no | Display path 1 |
| `path2` | String | no | Display path 2 |
| `isDeleted` | bool | no | Whether the ad is soft-deleted |
| `adError` | String | no | Ad-level error message from Google |
| `publishingStatus` | String — `DRAFT`, `SCHEDULED`, `PUBLISHED`, `PUBLISHING`, `FAILED`, `IN_REVIEW`, `PAUSED`, `ARCHIVED`, `WITH_ISSUES`, `REJECTED` | no | Ad publishing status |
| `adId` | String | no | Internal ad identifier |
| `adCampaignId` | String | no | Ad campaign identifier |
| `adGroupId` | String | no | Ad group identifier |
| `googleAdId` | String | no | Google Ads ad resource ID |
| `media` | Vec<GoogleMediaDTO> | no | Ad media items |
| `callToActionLabel` | String — `AUTOMATED`, `LEARN_MORE`, `GET_QUOTE`, `APPLY_NOW`, `SIGN_UP`, `CONTACT_US`, `SUBSCRIBE`, `DOWNLOAD`, `BOOK_NOW`, `SHOP_NOW`, `BUY_NOW`, `DONATE_NOW`, `ORDER_NOW`, `PLAY_NOW`, `SEE_MORE` | no | Call to action label |
| `businessName` | String | no | Business name |
| `youtubeVideoLinks` | Vec<GoogleYouTubeVideoLinkDTO> | no | YouTube video links |
| `carouselCards` | Vec<GoogleCarouselCardDTO> | no | Carousel cards |
| `placements` | Vec<String (enum)> | no | Channel placements |
| `customChannels` | bool | no | Custom channels flag |

### `GoogleAdGroupAudienceDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `geoLocations` | Vec<GoogleGeoLocationDTO> | no | Geo-location targeting |
| `locales` | Vec<GoogleLocaleDTO> | no | Language/locale targeting |

### `GoogleAdGroupDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Ad group identifier |
| `adGroupId` | String | no | Google ad group identifier |
| `name` | String | no | Ad group name |
| `adCampaignId` | String | no | Ad campaign identifier |
| `adContent` | Vec<GoogleAdContentDTO> | no | Ad content items |
| `keywords` | [`GoogleKeywordsDTO`](#googlekeywordsdto) | no | Keyword targeting |
| `publishingStatus` | String — `DRAFT`, `SCHEDULED`, `PUBLISHED`, `PUBLISHING`, `FAILED`, `IN_REVIEW`, `PAUSED`, `ARCHIVED`, `WITH_ISSUES`, `REJECTED` | no | Ad group publishing status |
| `adGroupError` | String | no | Ad group-level error from Google |
| `googleAdGroupId` | String | no | Google Ads ad group resource ID |
| `customChannels` | bool | no | Custom channels flag |
| `selectedChannels` | Vec<String (enum)> | no | Selected channel placements |
| `googleAudienceId` | String | no | Google audience resource ID |
| `audience` | [`GoogleAdGroupAudienceDTO`](#googleadgroupaudiencedto) | no | Ad group audience targeting |

### `GoogleAdScheduleDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `dayOfWeek` | String — `FRIDAY`, `MONDAY`, `SATURDAY`, `SUNDAY`, `THURSDAY`, `TUESDAY`, `UNKNOWN`, `UNSPECIFIED`, `WEDNESDAY`, `ALL_DAYS`, `MONDAY_TO_FRIDAY`, `SATURDAY_AND_SUNDAY` | **yes** | Day of week |
| `from` | String | **yes** | Start time (HH:MM) |
| `to` | String | **yes** | End time (HH:MM) |

### `GoogleAssetImageDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | String | **yes** | Image URL |
| `resourceName` | String | no | Google Ads resource name |
| `name` | String | no | Asset name |
| `error` | String | no | Error message if asset upload failed |

### `GoogleAssetsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `calls` | Vec<String> | no | Call extension asset resource names |
| `sitelinks` | Vec<String> | no | Sitelink asset resource names |
| `leadForm` | String | no | Lead form asset resource name |
| `images` | Vec<GoogleAssetImageDTO> | no | Image assets |
| `businessLogo` | [`GoogleAssetImageDTO`](#googleassetimagedto) | no | Business logo asset |

### `GoogleBiddingStrategyDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String | no | Bidding strategy type |
| `value` | f64 | no | Bid value in micros |

### `GoogleBudgetDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `budgetType` | String — `DAILY`, `LIFETIME` | no | Budget type |
| `amount` | f64 | no | Budget amount in micros |
| `scheduleStartDate` | String | no | Schedule start date |
| `scheduleEndDate` | String | no | Schedule end date |

### `GoogleCampaignAudienceDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `geoLocations` | Vec<GoogleGeoLocationDTO> | no | Geo-location targeting |
| `locales` | Vec<GoogleLocaleDTO> | no | Language/locale targeting |
| `gender` | Vec<GoogleDemographicTargetDTO> | no | Gender targeting |
| `ageRange` | Vec<GoogleDemographicTargetDTO> | no | Age range targeting |
| `segments` | Vec<GoogleSegmentTargetDTO> | no | Audience segment targeting |
| `targetInterests` | [`GoogleTargetInterestsDTO`](#googletargetinterestsdto) | no | Interest-based targeting |

### `GoogleCampaignGoalDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `CONVERSIONS`, `CLICK`, `YOUTUBE_ENGAGEMENT` | **yes** | Campaign goal type |
| `value` | String | no | Goal value (e.g. conversion action resource name) |
| `isCustomConversionGoal` | bool | no | Whether this is a custom conversion goal |

### `GoogleCarouselCardDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `headline` | String | no | Card headline |
| `finalUrl` | String | no | Card final URL |
| `callToActionLabel` | String | no | Call to action label |
| `media` | Vec<GoogleMediaDTO> | no | Card media items |

### `GoogleDemographicTargetDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `enum` | String | **yes** | Demographic enum value |
| `negative` | bool | **yes** | Whether this is a negative target |

### `GoogleGeoLocationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `key` | String | no | Geo target constant resource name |
| `id` | String | no | Location identifier (place_id) |
| `name` | String | no | Location display name |
| `countryName` | String | no | Country name |
| `type` | String | no | Location type (city, region, country, address, etc.) |
| `radius` | f64 | no | Radius for proximity targeting |
| `radiusUnit` | String — `km`, `mi` | no | Radius unit |
| `selectionType` | String — `include`, `exclude` | no | Include or exclude this location |
| `resourceName` | String | no | Google Ads resource name |
| `placeId` | String | no | Google place ID |
| `formattedAddress` | String | no | Full formatted address string |
| `geometry` | [`GeoGeometryDTO`](#geogeometrydto) | no | Geometry data from Google Geocoding API |
| `addressComponents` | Vec<GeoAddressComponentDTO> | no | Address components from Google Geocoding API |

### `GoogleKeywordItemDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `keyword` | String | **yes** | Keyword text |
| `matchType` | String | **yes** | Match type (BROAD, PHRASE, EXACT) |

### `GoogleKeywordsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `positives` | Vec<GoogleKeywordItemDTO> | no | Positive keywords |
| `negatives` | Vec<GoogleKeywordItemDTO> | no | Negative keywords |

### `GoogleLocaleDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Language display name |
| `key` | String | no | Language key |
| `id` | String | no | Language identifier |
| `resourceName` | String | no | Language resource name |

### `GoogleMediaDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `IMAGE` | no | Media type |
| `src` | String | no | Media source URL |
| `isLogo` | bool | no | Is logo flag |
| `error` | String | no | Error message if media failed |
| `url` | String | no | Public URL of the media |
| `imageType` | String | no | Image type classification |

### `GoogleNetworkSettingsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `targetSearchNetwork` | bool | **yes** | Target Google Search Network |
| `targetContentNetwork` | bool | **yes** | Target Google Display Network |

### `GoogleSegmentTargetDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String | **yes** | Segment type |
| `id` | String | **yes** | Segment identifier |

### `GoogleTargetInterestsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `affinity` | Vec<String> | no | Affinity audience IDs |
| `inMarket` | Vec<String> | no | In-market audience IDs |

### `GoogleYouTubeVideoLinkDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `youtubeVideoId` | String | **yes** | YouTube video ID |

### `GreetingCard`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | Greeting card title |
| `style` | String | **yes** | Greeting card style |
| `content` | Vec<String> | **yes** | Greeting card content |

### `HiddenFieldDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Field name |
| `value` | String | **yes** | Field value |

### `KeywordSuggestionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | String | **yes** | Target URL |
| `languageCode` | String | no | Language code |
| `locations` | Vec<String> | no | Target locations |
| `keywords` | Vec<String> | no | Seed keywords |

### `LeadFormAssetPayloadDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `resourceName` | String | no | Google Ads resource name for an existing lead form asset |
| `headline` | String | **yes** | Lead form headline |
| `description` | String | **yes** | Lead form description |
| `businessName` | String | **yes** | Business name shown on the form |
| `privacyPolicyUrl` | String | **yes** | Privacy policy URL |
| `fields` | Vec<LeadFormFieldDTO> | **yes** | Form fields to collect user input |
| `callToActionType` | String — `LEARN_MORE`, `GET_QUOTE`, `APPLY_NOW`, `SIGN_UP`, `CONTACT_US`, `SUBSCRIBE`, `DOWNLOAD`, `BOOK_NOW`, `GET_OFFER`, `REGISTER`, `GET_INFO`, `REQUEST_DEMO`, `JOIN_NOW`, `GET_STARTED`, `VISIT_SITE` | **yes** | Call to action button type |
| `callToActionDescription` | String | no | Description text for the CTA button |
| `backgroundImageAsset` | String | no | Background image asset resource name |
| `desiredIntent` | String — `LOW_INTENT`, `HIGH_INTENT` | no | Desired lead intent level |
| `customQuestionFields` | Vec<CustomQuestionFieldDTO> | no | Custom question fields appended after standard fields |
| `postSubmitHeadline` | String | no | Headline shown after form submission |
| `postSubmitDescription` | String | no | Description shown after form submission |
| `postSubmitCallToActionType` | String — `VISIT_SITE`, `DOWNLOAD`, `LEARN_MORE`, `SHOP_NOW` | no | Post-submit CTA button type |
| `finalUrls` | String | no | Final URL shown after form submission |

### `LeadFormContentDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `questions` | Vec<LeadFormQuestionDTO> | **yes** | Form questions |
| `description` | [`LocalizedStringDTO`](#localizedstringdto) | no | Form description |
| `headline` | [`LocalizedStringDTO`](#localizedstringdto) | **yes** | Form headline |
| `postSubmissionInfo` | [`PostSubmissionInfoDTO`](#postsubmissioninfodto) | **yes** | Post-submission info |
| `legalInfo` | [`LegalInfoDTO`](#legalinfodto) | **yes** | Legal information |

### `LeadFormFieldDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `inputType` | String | **yes** | Field input type from Google Ads LeadFormFieldUserInputType |
| `singleChoiceAnswers` | Vec<String> | no | Single-choice answer options for the field |

### `LeadFormQuestionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `question` | [`LocalizedStringDTO`](#localizedstringdto) | **yes** | Question text |
| `name` | String | **yes** | Question field name |
| `questionDetails` | [`QuestionDetailsDTO`](#questiondetailsdto) | **yes** | Question type details |
| `predefinedField` | String | no | Predefined field identifier |

### `LegalInfoDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `consents` | Vec<ConsentDTO> | **yes** | Consent entries |
| `privacyPolicyUrl` | String | **yes** | Privacy policy URL |
| `legalDisclaimer` | [`LocalizedStringDTO`](#localizedstringdto) | no | Legal disclaimer text |

### `LinkedInAdDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Ad identifier |
| `name` | String | no | Ad name |
| `introductoryText` | String | no | Introductory ad copy shown above the creative |
| `destinationUrl` | String | no | Click-through destination URL |
| `callToActionLabel` | String | no | Call to action label |
| `destinationFormId` | String | no | Destination lead-gen form ID |
| `contentReferenceString` | String | no | Content reference URN for boosted posts |
| `media` | Vec<LinkedInMediaDTO> | no | Ad creative media |
| `adCampaignId` | String | no | Parent ad campaign identifier |
| `adId` | String | no | LinkedIn ad resource ID |
| `headline` | String | no | Ad headline |
| `publishingStatus` | String — `DRAFT`, `SCHEDULED`, `PUBLISHED`, `PUBLISHING`, `FAILED`, `IN_REVIEW`, `PAUSED`, `ARCHIVED`, `WITH_ISSUES`, `REJECTED` | no | Ad publishing status |
| `adCampaignGroupId` | String | no | Parent campaign group identifier |
| `description` | String | no | Ad description |
| `meta` | JSON | no | Additional ad metadata |
| `linkedInError` | String | no | LinkedIn API error message |

### `LinkedInBudgetDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `budgetType` | String — `DAILY`, `LIFETIME` | no | LinkedIn campaign budget type |
| `amount` | f64 | no | Budget amount (currency minor units) |
| `scheduleStartDate` | String | no | Schedule start date (ISO 8601) |
| `scheduleEndDate` | String | no | Schedule end date (ISO 8601) |

### `LinkedInCreateLeadFormBodyDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `owner` | [`SponsoredAccountOwnerDTO`](#sponsoredaccountownerdto) | **yes** | Form owner |
| `creationLocale` | [`CreationLocaleDTO`](#creationlocaledto) | **yes** | Creation locale |
| `name` | String | **yes** | Form name |
| `state` | String — `PUBLISHED` | **yes** | Form state |
| `content` | [`LeadFormContentDTO`](#leadformcontentdto) | **yes** | Form content |
| `hiddenFields` | Vec<HiddenFieldDTO> | no | Hidden fields |

### `LinkedInMediaDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `video`, `image` | no | Media type |
| `src` | String | no | Media source URL |
| `frames` | Vec<String> | no | Video frame URLs |
| `selectedPoster` | f64 | no | Selected poster frame index |
| `thumbnailUrl` | String | no | Thumbnail URL |
| `name` | String | no | Media name |
| `headline` | String | no | Media headline |
| `destinationUrl` | String | no | Click-through destination URL |
| `fileSizeBytes` | f64 | no | File size in bytes |

### `LinkedInUpdateAdStatusBodyDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `operationType` | String — `PAUSED`, `ARCHIVED`, `RESUME` | **yes** | Update operation |
| `type` | String — `adGroup`, `adCampaign`, `ad` | **yes** | Ad object type |

### `LocaleDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `country` | String | **yes** | Country code |
| `language` | String | **yes** | Language code |

### `LocalizedStringDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `localized` | JSON | **yes** | Locale-keyed string map |

### `LocationIdBodyDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location identifier |

### `MediaDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `src` | String | **yes** | Media source URL |
| `thumbnailUrl` | String | no | Thumbnail URL (required when type is video) |
| `selectedPoster` | f64 | no | Selected poster index (required when type is video) |
| `type` | String — `image`, `video` | **yes** | Media content type |
| `name` | String | no | Media file name |
| `headline` | String | no | Media headline |
| `description` | String | no | Media description |
| `link` | String | no | Media destination link |

### `MemberDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `memberType` | String — `KEYWORD`, `URL`, `APP` | **yes** | Member type |
| `keyword` | String | no | Keyword value |
| `url` | String | no | URL value |
| `app` | String | no | App identifier |

### `MultipleChoiceOptionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | f64 | **yes** | Option ID |
| `text` | [`LocalizedStringDTO`](#localizedstringdto) | **yes** | Option text |

### `MultipleChoiceQuestionDetailsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `options` | Vec<MultipleChoiceOptionDTO> | **yes** | Choice options |

### `PostSubmissionCallToActionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `callToActionTarget` | [`PostSubmissionCallToActionTargetDTO`](#postsubmissioncalltoactiontargetdto) | **yes** | Call to action target |
| `callToActionLabel` | String — `VISIT_COMPANY_WEBSITE`, `DOWNLOAD_NOW`, `TRY_NOW`, `VIEW_NOW`, `LEARN_MORE` | **yes** | Call to action label |

### `PostSubmissionCallToActionTargetDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `landingPageUrl` | String | **yes** | Landing page URL |

### `PostSubmissionInfoDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | [`LocalizedStringDTO`](#localizedstringdto) | **yes** | Thank-you message |
| `callToAction` | [`PostSubmissionCallToActionDTO`](#postsubmissioncalltoactiondto) | **yes** | Post-submission call to action |

### `PublishAdDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location identifier |

### `PublishingProgressResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `campaignId` | String | **yes** | Campaign identifier |
| `publishingStatus` | String — `DRAFT`, `SCHEDULED`, `PUBLISHED`, `PUBLISHING`, `FAILED`, `IN_REVIEW`, `PAUSED`, `ARCHIVED`, `WITH_ISSUES`, `REJECTED` | **yes** | Current campaign publishing status in ad-publishing |
| `total` | f64 | **yes** | Total publish steps tracked in Redis (campaign + ad sets + ads) |
| `processed` | f64 | **yes** | Number of publish steps completed so far |
| `isComplete` | bool | **yes** | Whether publishing is finished (Redis complete/failed, processed >= total, or status is no longer PUBLISHING) |
| `hasFailed` | bool | **yes** | Whether publishing failed (Redis failed status or campaign FAILED) |

### `QuestionDetailsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `textQuestionDetails` | JSON | no | Text question details (empty object for text questions) |
| `multipleChoiceQuestionDetails` | [`MultipleChoiceQuestionDetailsDTO`](#multiplechoicequestiondetailsdto) | no | Multiple choice question details |

### `RuleBasedUserListDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `prepopulationStatus` | String — `REQUESTED` | no | Prepopulation status |
| `flexibleRuleUserList` | [`FlexibleRuleUserListDTO`](#flexibleruleuserlistdto) | **yes** | Flexible rule user list configuration |

### `RuleDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `ruleItemGroups` | Vec<RuleItemGroupDTO> | **yes** | List of rule item groups |

### `RuleItemDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String — `url__`, `referrer__` | **yes** | Rule item name |
| `stringRuleItem` | [`StringRuleItemDTO`](#stringruleitemdto) | **yes** | String rule item condition |

### `RuleItemGroupDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `ruleItems` | Vec<RuleItemDTO> | **yes** | List of rule items |

### `RuleOperandDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `lookbackWindowDays` | f64 | **yes** | Lookback window in days |
| `rule` | [`RuleDTO`](#ruledto) | **yes** | Rule definition |

### `SelectedAttributeDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `urn` | String | **yes** | Targeting attribute URN |
| `name` | String | **yes** | Display name |
| `categoryName` | String | **yes** | Category name |
| `facet` | String | **yes** | Facet identifier |

### `SitelinkAssetPayloadDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `resourceName` | String | no | Google Ads resource name for an existing sitelink asset |
| `linkText` | String | **yes** | Sitelink display text |
| `finalUrls` | String | **yes** | Final landing page URL |
| `description1` | String | no | First description line |
| `description2` | String | no | Second description line |
| `startDate` | String | no | Start date for the sitelink (YYYY-MM-DD) |
| `endDate` | String | no | End date for the sitelink (YYYY-MM-DD) |
| `adScheduleTargets` | Vec<AdScheduleTargetDTO> | no | Ad schedule targets restricting when the sitelink is shown |

### `SponsoredAccountOwnerDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `sponsoredAccount` | String | **yes** | Sponsored account URN |

### `StringRuleItemDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `operator` | String | **yes** | Rule operator |
| `value` | String | **yes** | Rule value |

### `TargetAudienceDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `include` | Vec<Vec<SelectedAttributeDTO>> | no | Included targeting attributes (groups of ANDed attributes, ORed together) |
| `exclude` | Vec<Vec<SelectedAttributeDTO>> | no | Excluded targeting attributes |

### `ThankYouPage`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | Thank you page title |
| `body` | String | **yes** | Thank you page body |
| `buttonText` | String | **yes** | Button text label |
| `buttonType` | String | **yes** | Button action type |
| `buttonLink` | String | no | Button destination link |
| `businessPhone` | String | no | Business phone number |
| `countryCode` | String | no | Phone country code |

### `UnitCostDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `amount` | f64 | **yes** | Bid amount in currency minor units |

### `UpdateCustomAudienceBatchDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location identifier |
| `csvPath` | String | no | CSV file path |
| `operationType` | String | **yes** | Batch operation type |
| `smartlistIds` | Vec<String> | no | Smartlist IDs array |
| `dynamicAudience` | String | no | Dynamic audience flag |

### `UpdateCustomAudienceDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location identifier |
| `contactId` | String | **yes** | Contact identifier |
| `fbAdAccountId` | String | no | Facebook ad account ID |

### `UpsertAdDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Ad identifier |
| `locationId` | String | **yes** | Location identifier |
| `name` | String | no | Ad name |
| `primaryText` | String | no | Ad primary text |
| `headline` | String | no | Ad headline text |
| `description` | String | no | Ad description text |
| `imageUrl` | String | no | Ad image URL |
| `mediaType` | String — `SINGLE`, `CAROUSEL` | no | Ad media type |
| `media` | Vec<MediaDTO> | no | Media items (images or videos) attached to the ad creative |
| `multiAdvertiserAds` | bool | no | Enable multi-advertiser ads |
| `campaignId` | String | **yes** | Parent campaign ID |
| `adsetId` | String | **yes** | Parent ad set ID |
| `cta` | String | no | Call to action type |
| `conversationFormId` | String | no | Conversation form ID |
| `destinationLink` | String | no | Destination link URL |
| `destinationFormId` | String | no | Destination form ID |

### `UpsertAdsetDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Ad set identifier |
| `locationId` | String | **yes** | Location identifier |
| `name` | String | no | Ad set name |
| `pageId` | String | no | Facebook page ID |
| `instagramActorId` | String | no | Instagram actor ID |
| `messagingPlatforms` | Vec<String (enum)> | no | Messaging platforms |
| `whatsappNumber` | String | no | WhatsApp phone number |
| `audience` | [`FacebookAudienceDTO`](#facebookaudiencedto) | no | Targeting audience configuration including geo-locations, locales, placements, and custom audiences |
| `budget` | [`Budget`](#budget) | no | Ad set budget config |
| `conversionLocation` | String | no | Conversion location |
| `customEventType` | String | no | Custom event type |
| `pixelId` | String | no | Conversion pixel ID |
| `campaignId` | String | **yes** | Parent campaign ID |

### `UpsertAssetsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location identifier |
| `type` | String — `CALL`, `SITELINK`, `LEAD_FORM` | **yes** | Asset type to create or update |
| `payload` | JSON | **yes** | Asset payload — shape depends on the type field: CallAssetPayload (CALL), SitelinkAssetPayload (SITELINK), or LeadFormAssetPayload (LEAD_FORM) |

### `UpsertAudienceDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location identifier |
| `resourceName` | String | no | Audience resource name |
| `name` | String | **yes** | Audience name |
| `dimensions` | [`AudienceDimensionDTO`](#audiencedimensiondto) | no | Audience dimensions |
| `exclusionDimension` | [`AudienceDimensionDTO`](#audiencedimensiondto) | no | Exclusion dimensions |

### `UpsertCampaignDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Campaign identifier |
| `locationId` | String | **yes** | Location identifier |
| `name` | String | no | Campaign name |
| `objective` | String — `OUTCOME_LEADS`, `OUTCOME_TRAFFIC`, `OUTCOME_ENGAGEMENT`, `OUTCOME_SALES` | no | Campaign objective |
| `specialAdCategories` | Vec<String (enum)> | no | Special ad categories |
| `source` | String | no | Campaign data source |

### `UpsertConversionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location identifier |
| `conversionId` | String | no | Conversion identifier |
| `name` | String | **yes** | Conversion name |
| `type` | String — `UPLOAD_CLICKS`, `UPLOAD_CALLS`, `WEBPAGE`, `LEAD_FORM_SUBMIT` | **yes** | Conversion type |
| `category` | String | **yes** | Conversion category |
| `valueSettings` | [`ConversionValueSettings`](#conversionvaluesettings) | **yes** | Value settings that control how monetary value is attributed to conversions |
| `countingType` | String — `ONE_PER_CLICK`, `MANY_PER_CLICK` | **yes** | How conversions are counted per interaction |
| `attributionModel` | String — `GOOGLE_SEARCH_ATTRIBUTION_DATA_DRIVEN`, `GOOGLE_ADS_LAST_CLICK` | **yes** | Attribution model used to credit conversions |
| `clickThroughWindow` | f64 | **yes** | Click-through conversion window in days |

### `UpsertConversionPixelDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location identifier |
| `conversionPixelId` | String | no | Conversion pixel ID |
| `name` | String | no | Pixel name |
| `igUserId` | String | no | Instagram user ID |
| `type` | String | **yes** | Pixel event type |

### `UpsertSegmentDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Segment name |
| `description` | String | no | Segment description |
| `members` | Vec<MemberDTO> | no | Segment members — keywords, URLs, or apps that define the custom segment |
| `status` | String | no | Segment status |
| `type` | String | no | Segment type |
| `id` | String | no | Segment identifier |
| `membershipStatus` | String | no | Membership status |
| `ruleBasedUserList` | [`RuleBasedUserListDTO`](#rulebaseduserlistdto) | no | Rule-based user list config |
| `membershipLifeSpan` | f64 | no | Membership life span |
| `seedUserListIds` | Vec<String> | no | Seed user list IDs |
| `countryCodes` | Vec<String> | no | Country codes |
| `expansionLevel` | String — `BALANCED`, `BROAD`, `NARROW` | no | Expansion level |

### `WelcomeMessageQuestion`

| Field | Type | Required | Description |
|---|---|---|---|
| `question` | String | **yes** | Question title text |
| `response` | String | no | Auto-response message |

