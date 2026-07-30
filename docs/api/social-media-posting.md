# `social-media-posting`

**40** operations / **107** models in API v2

## How to call it

**Every endpoint has a typed Rust method.** Enable the `social-media-posting` cargo feature on `ghl-sdk`, then call any of the 40 generated methods on `ghl.social_media_posting()` (v2):

```toml
ghl-sdk = { version = "0.5", features = ["social-media-posting"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/social-media-posting/oauth/facebook/start` | Starts OAuth For Facebook Account | `starts_o_auth_for_facebook_account()` | `social-media-posting.get_social_media_posting_oauth_facebook_start` |
| `GET` | `/social-media-posting/oauth/google/start` | Starts OAuth For Google Account | `starts_o_auth_for_google_account()` | `social-media-posting.get_social_media_posting_oauth_google_start` |
| `GET` | `/social-media-posting/oauth/instagram/start` | Starts OAuth For Instagram Account | `starts_o_auth_for_instagram_account()` | `social-media-posting.get_social_media_posting_oauth_instagram_start` |
| `GET` | `/social-media-posting/oauth/linkedin/start` | Starts OAuth For LinkedIn Account | `starts_o_auth_for_linked_in_account()` | `social-media-posting.get_social_media_posting_oauth_linkedin_start` |
| `GET` | `/social-media-posting/oauth/tiktok-business/start` | Starts OAuth For Tiktok Business Account | `starts_o_auth_for_tiktok_business_account()` | `social-media-posting.get_social_media_posting_oauth_tiktok_business_start` |
| `GET` | `/social-media-posting/oauth/tiktok/start` | Starts OAuth For Tiktok Account | `starts_o_auth_for_tiktok_account()` | `social-media-posting.get_social_media_posting_oauth_tiktok_start` |
| `GET` | `/social-media-posting/oauth/twitter/start` | Starts OAuth For Twitter Account | `starts_o_auth_for_twitter_account()` | `social-media-posting.get_social_media_posting_oauth_twitter_start` |
| `GET` | `/social-media-posting/oauth/{locationId}/facebook/accounts/{accountId}` | Get facebook pages | `get_facebook_pages()` | `social-media-posting.get_social_media_posting_oauth_by_locationId_facebook_accounts_by_accountId` |
| `POST` | `/social-media-posting/oauth/{locationId}/facebook/accounts/{accountId}` | Attach facebook pages | `attach_facebook_pages()` | `social-media-posting.post_social_media_posting_oauth_by_locationId_facebook_accounts_by_accountId` |
| `GET` | `/social-media-posting/oauth/{locationId}/google/locations/{accountId}` | Get google business locations | `get_google_business_locations()` | `social-media-posting.get_social_media_posting_oauth_by_locationId_google_locations_by_accountId` |
| `POST` | `/social-media-posting/oauth/{locationId}/google/locations/{accountId}` | Set google business locations | `set_google_business_locations()` | `social-media-posting.post_social_media_posting_oauth_by_locationId_google_locations_by_accountId` |
| `GET` | `/social-media-posting/oauth/{locationId}/instagram/accounts/{accountId}` | Get Instagram Professional Accounts | `get_instagram_professional_accounts()` | `social-media-posting.get_social_media_posting_oauth_by_locationId_instagram_accounts_by_accountId` |
| `POST` | `/social-media-posting/oauth/{locationId}/instagram/accounts/{accountId}` | Attach Instagram Professional Accounts | `attach_instagram_professional_accounts()` | `social-media-posting.post_social_media_posting_oauth_by_locationId_instagram_accounts_by_accountId` |
| `GET` | `/social-media-posting/oauth/{locationId}/linkedin/accounts/{accountId}` | Get Linkedin pages and profile | `get_linkedin_pages_and_profile()` | `social-media-posting.get_social_media_posting_oauth_by_locationId_linkedin_accounts_by_accountId` |
| `POST` | `/social-media-posting/oauth/{locationId}/linkedin/accounts/{accountId}` | Attach linkedin pages and profile | `attach_linkedin_pages_and_profile()` | `social-media-posting.post_social_media_posting_oauth_by_locationId_linkedin_accounts_by_accountId` |
| `GET` | `/social-media-posting/oauth/{locationId}/tiktok-business/accounts/{accountId}` | Get Tiktok Business profile | `get_tiktok_business_profile()` | `social-media-posting.get_social_media_posting_oauth_by_locationId_tiktok_business_accounts_by_accountId` |
| `GET` | `/social-media-posting/oauth/{locationId}/tiktok/accounts/{accountId}` | Get Tiktok profile | `get_tiktok_profile()` | `social-media-posting.get_social_media_posting_oauth_by_locationId_tiktok_accounts_by_accountId` |
| `POST` | `/social-media-posting/oauth/{locationId}/tiktok/accounts/{accountId}` | Attach Tiktok profile | `attach_tiktok_profile()` | `social-media-posting.post_social_media_posting_oauth_by_locationId_tiktok_accounts_by_accountId` |
| `GET` | `/social-media-posting/oauth/{locationId}/twitter/accounts/{accountId}` | Get Twitter profile | `get_twitter_profile()` | `social-media-posting.get_social_media_posting_oauth_by_locationId_twitter_accounts_by_accountId` |
| `POST` | `/social-media-posting/oauth/{locationId}/twitter/accounts/{accountId}` | Attach Twitter profile | `attach_twitter_profile()` | `social-media-posting.post_social_media_posting_oauth_by_locationId_twitter_accounts_by_accountId` |
| `POST` | `/social-media-posting/statistics` | Get Social Media Statistics | `get_social_media_statistics()` | `social-media-posting.post_social_media_posting_statistics` |
| `GET` | `/social-media-posting/{locationId}/accounts` | Get Accounts | `get_accounts()` | `social-media-posting.get_social_media_posting_by_locationId_accounts` |
| `DELETE` | `/social-media-posting/{locationId}/accounts/{id}` | Delete Account | `delete_account()` | `social-media-posting.delete_social_media_posting_by_locationId_accounts_by_id` |
| `GET` | `/social-media-posting/{locationId}/categories` | Get categories by location id | `get_categories_by_location_id()` | `social-media-posting.get_social_media_posting_by_locationId_categories` |
| `GET` | `/social-media-posting/{locationId}/categories/{id}` | Get categories by id | `get_categories_by_id()` | `social-media-posting.get_social_media_posting_by_locationId_categories_by_id` |
| `GET` | `/social-media-posting/{locationId}/csv` | Get Upload Status | `get_upload_status()` | `social-media-posting.get_social_media_posting_by_locationId_csv` |
| `POST` | `/social-media-posting/{locationId}/csv` | Upload CSV | `upload_csv()` | `social-media-posting.post_social_media_posting_by_locationId_csv` |
| `DELETE` | `/social-media-posting/{locationId}/csv/{csvId}/post/{postId}` | Delete CSV Post | `delete_csv_post()` | `social-media-posting.delete_social_media_posting_by_locationId_csv_by_csvId_post_by_postId` |
| `DELETE` | `/social-media-posting/{locationId}/csv/{id}` | Delete CSV | `delete_csv()` | `social-media-posting.delete_social_media_posting_by_locationId_csv_by_id` |
| `GET` | `/social-media-posting/{locationId}/csv/{id}` | Get CSV Post | `get_csv_post()` | `social-media-posting.get_social_media_posting_by_locationId_csv_by_id` |
| `PATCH` | `/social-media-posting/{locationId}/csv/{id}` | Start CSV Finalize | `start_csv_finalize()` | `social-media-posting.patch_social_media_posting_by_locationId_csv_by_id` |
| `POST` | `/social-media-posting/{locationId}/posts` | Create post | `create_post()` | `social-media-posting.post_social_media_posting_by_locationId_posts` |
| `POST` | `/social-media-posting/{locationId}/posts/bulk-delete` | Bulk Delete Social Planner Posts | `bulk_delete_social_planner_posts()` | `social-media-posting.post_social_media_posting_by_locationId_posts_bulk_delete` |
| `POST` | `/social-media-posting/{locationId}/posts/list` | Get posts | `get_posts()` | `social-media-posting.post_social_media_posting_by_locationId_posts_list` |
| `DELETE` | `/social-media-posting/{locationId}/posts/{id}` | Delete Post | `delete_post()` | `social-media-posting.delete_social_media_posting_by_locationId_posts_by_id` |
| `GET` | `/social-media-posting/{locationId}/posts/{id}` | Get post | `get_post()` | `social-media-posting.get_social_media_posting_by_locationId_posts_by_id` |
| `PUT` | `/social-media-posting/{locationId}/posts/{id}` | Edit post | `edit_post()` | `social-media-posting.put_social_media_posting_by_locationId_posts_by_id` |
| `POST` | `/social-media-posting/{locationId}/set-accounts` | Set Accounts | `set_accounts()` | `social-media-posting.post_social_media_posting_by_locationId_set_accounts` |
| `GET` | `/social-media-posting/{locationId}/tags` | Get tags by location id | `get_tags_by_location_id()` | `social-media-posting.get_social_media_posting_by_locationId_tags` |
| `POST` | `/social-media-posting/{locationId}/tags/details` | Get tags by ids | `get_tags_by_ids()` | `social-media-posting.post_social_media_posting_by_locationId_tags_details` |

### Endpoint details — v2

#### `GET /social-media-posting/oauth/facebook/start`

**Starts OAuth For Facebook Account**

Open the API in a window with appropriate params and headers instead of using the Curl. User is navigated to Facebook login OAuth screen. On successful login, listen on window object for message where event listener returns data in its callback function. ### Sample code to listen to event data: window.addEventListener('message', function(e) { if (e.data && e.data.page === 'social_media_posting') { const { actionType, page, platform, placement, accountId, reconnectAccounts } = e.data } }, false) …

Operation id: `social-media-posting.get_social_media_posting_oauth_facebook_start` · `Version: 2021-07-28`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Account Location Id |
| `userId` | string | **yes** | User ID |
| `page` | string | no | Facebook Page |
| `reconnect` | string | no | Reconnect boolean as string |

*Rust*:

```rust,ignore
use ghl_sdk::services::social_media_posting::StartsOAuthForFacebookAccountParams;

let params = StartsOAuthForFacebookAccountParams::new("locationId", "userId");
let out = ghl.social_media_posting().starts_o_auth_for_facebook_account(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_oauth_facebook_start",
    "query": {
      "locationId": "<locationId>",
      "userId": "<userId>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/oauth/google/start`

**Starts OAuth For Google Account**

Open the API in a window with appropriate params and headers instead of using the Curl. User is navigated to Google login OAuth screen. On successful login, listen on window object for message where event listener returns data in its callback function. ### Sample code to listen to event data: window.addEventListener('message', function(e) { if (e.data && e.data.page === 'social_media_posting') { const { actionType, page, platform, placement, accountId, reconnectAccounts } = e.data } }, false) ##…

Operation id: `social-media-posting.get_social_media_posting_oauth_google_start` · `Version: 2021-07-28` · Scopes: `socialplanner/oauth.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `userId` | string | **yes** | User Id |
| `page` | string | no | Page |
| `reconnect` | string | no | Reconnect |

*Rust*:

```rust,ignore
use ghl_sdk::services::social_media_posting::StartsOAuthForGoogleAccountParams;

let params = StartsOAuthForGoogleAccountParams::new("locationId", "userId");
let out = ghl.social_media_posting().starts_o_auth_for_google_account(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_oauth_google_start",
    "query": {
      "locationId": "<locationId>",
      "userId": "<userId>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/oauth/instagram/start`

**Starts OAuth For Instagram Account**

Open the API in a window with appropriate params and headers instead of using the Curl. User is navigated to Instagram login OAuth screen. On successful login, listen on window object for message where event listener returns data in its callback function. ### Sample code to listen to event data: window.addEventListener('message', function(e) { if (e.data && e.data.page === 'social_media_posting') { const { actionType, page, platform, placement, accountId, reconnectAccounts } = e.data } }, false)…

Operation id: `social-media-posting.get_social_media_posting_oauth_instagram_start` · `Version: 2021-07-28` · Scopes: `socialplanner/oauth.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `userId` | string | **yes** | User Id |
| `page` | string | no | Page |
| `reconnect` | string | no | Reconnect |

*Rust*:

```rust,ignore
use ghl_sdk::services::social_media_posting::StartsOAuthForInstagramAccountParams;

let params = StartsOAuthForInstagramAccountParams::new("locationId", "userId");
let out = ghl.social_media_posting().starts_o_auth_for_instagram_account(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_oauth_instagram_start",
    "query": {
      "locationId": "<locationId>",
      "userId": "<userId>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/oauth/linkedin/start`

**Starts OAuth For LinkedIn Account**

Open the API in a window with appropriate params and headers instead of using the Curl. User is navigated to LinkedIn login OAuth screen. On successful login, listen on window object for message where event listener returns data in its callback function. ### Sample code to listen to event data: window.addEventListener('message', function(e) { if (e.data && e.data.page === 'social_media_posting') { const { actionType, page, platform, placement, accountId, reconnectAccounts } = e.data } }, false) …

Operation id: `social-media-posting.get_social_media_posting_oauth_linkedin_start` · `Version: 2021-07-28`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `userId` | string | **yes** | User Id |
| `page` | string | no | Page |
| `reconnect` | string | no | Reconnect |

*Rust*:

```rust,ignore
use ghl_sdk::services::social_media_posting::StartsOAuthForLinkedInAccountParams;

let params = StartsOAuthForLinkedInAccountParams::new("locationId", "userId");
let out = ghl.social_media_posting().starts_o_auth_for_linked_in_account(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_oauth_linkedin_start",
    "query": {
      "locationId": "<locationId>",
      "userId": "<userId>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/oauth/tiktok-business/start`

**Starts OAuth For Tiktok Business Account**

Open the API in a window with appropriate params and headers instead of using the Curl. User is navigated to Tiktok-Business login OAuth screen. On successful login, listen on window object for message where event listener returns data in its callback function. ### Sample code to listen to event data: window.addEventListener('message', function(e) { if (e.data && e.data.page === 'social_media_posting') { const { actionType, page, platform, placement, accountId, reconnectAccounts } = e.data } }, …

Operation id: `social-media-posting.get_social_media_posting_oauth_tiktok_business_start` · `Version: 2021-07-28` · Scopes: `socialplanner/oauth.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `userId` | string | **yes** | User Id |
| `page` | string | no | Page |
| `reconnect` | string | no | Reconnect |

*Rust*:

```rust,ignore
use ghl_sdk::services::social_media_posting::StartsOAuthForTiktokBusinessAccountParams;

let params = StartsOAuthForTiktokBusinessAccountParams::new("locationId", "userId");
let out = ghl.social_media_posting().starts_o_auth_for_tiktok_business_account(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_oauth_tiktok_business_start",
    "query": {
      "locationId": "<locationId>",
      "userId": "<userId>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/oauth/tiktok/start`

**Starts OAuth For Tiktok Account**

Open the API in a window with appropriate params and headers instead of using the Curl. User is navigated to Tiktok login OAuth screen. On successful login, listen on window object for message where event listener returns data in its callback function. ### Sample code to listen to event data: window.addEventListener('message', function(e) { if (e.data && e.data.page === 'social_media_posting') { const { actionType, page, platform, placement, accountId, reconnectAccounts } = e.data } }, false) ##…

Operation id: `social-media-posting.get_social_media_posting_oauth_tiktok_start` · `Version: 2021-07-28` · Scopes: `socialplanner/oauth.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `userId` | string | **yes** | User Id |
| `page` | string | no | Page |
| `reconnect` | string | no | Reconnect |

*Rust*:

```rust,ignore
use ghl_sdk::services::social_media_posting::StartsOAuthForTiktokAccountParams;

let params = StartsOAuthForTiktokAccountParams::new("locationId", "userId");
let out = ghl.social_media_posting().starts_o_auth_for_tiktok_account(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_oauth_tiktok_start",
    "query": {
      "locationId": "<locationId>",
      "userId": "<userId>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/oauth/twitter/start`

**Starts OAuth For Twitter Account**

<div><div> <span style= "display: inline-block; width: 25px; height: 25px; background-color: red; color: black; font-weight: bold; font-size: 24px; text-align: center; line-height: 20px; border: 2px solid black; border-radius: 20%; margin-right: 10px;"> ! </span> <span><strong>As of December 4, 2024, X (formerly Twitter) is no longer supported. We apologise for any inconvenience.</strong></span> </div></div>

Operation id: `social-media-posting.get_social_media_posting_oauth_twitter_start` · `Version: 2021-07-28` · Scopes: `socialplanner/oauth.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `userId` | string | **yes** | User Id |
| `page` | string | no | Page |
| `reconnect` | string | no | Reconnect |

*Rust*:

```rust,ignore
use ghl_sdk::services::social_media_posting::StartsOAuthForTwitterAccountParams;

let params = StartsOAuthForTwitterAccountParams::new("locationId", "userId");
let out = ghl.social_media_posting().starts_o_auth_for_twitter_account(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_oauth_twitter_start",
    "query": {
      "locationId": "<locationId>",
      "userId": "<userId>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/oauth/{locationId}/facebook/accounts/{accountId}`

**Get facebook pages**

Operation id: `social-media-posting.get_social_media_posting_oauth_by_locationId_facebook_accounts_by_accountId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Account Location Id |
| `accountId` | string | **yes** | Account Id |

*Response*: [`GetFacebookAccountsResponseDTO`](#getfacebookaccountsresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().get_facebook_pages(&locationId, &accountId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_oauth_by_locationId_facebook_accounts_by_accountId",
    "path_params": {
      "locationId": "<locationId>",
      "accountId": "<accountId>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/oauth/{locationId}/facebook/accounts/{accountId}`

**Attach facebook pages**

Operation id: `social-media-posting.post_social_media_posting_oauth_by_locationId_facebook_accounts_by_accountId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Account Location Id |
| `accountId` | string | **yes** | Account Id |

*Request body*: [`AttachFBAccountDTO`](#attachfbaccountdto)

*Response*: [`SocialMediaFBAccountResponseDTO`](#socialmediafbaccountresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().attach_facebook_pages(&locationId, &accountId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.post_social_media_posting_oauth_by_locationId_facebook_accounts_by_accountId",
    "path_params": {
      "locationId": "<locationId>",
      "accountId": "<accountId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/oauth/{locationId}/google/locations/{accountId}`

**Get google business locations**

Operation id: `social-media-posting.get_social_media_posting_oauth_by_locationId_google_locations_by_accountId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Account Location Id |
| `accountId` | string | **yes** | Account Id |

*Response*: [`GetGoogleLocationResponseDTO`](#getgooglelocationresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().get_google_business_locations(&locationId, &accountId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_oauth_by_locationId_google_locations_by_accountId",
    "path_params": {
      "locationId": "<locationId>",
      "accountId": "<accountId>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/oauth/{locationId}/google/locations/{accountId}`

**Set google business locations**

Operation id: `social-media-posting.post_social_media_posting_oauth_by_locationId_google_locations_by_accountId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Account Location Id |
| `accountId` | string | **yes** | Account Id |

*Request body*: [`AttachGMBLocationDTO`](#attachgmblocationdto)

*Response*: [`SocialMediaGmbAccountResponseDTO`](#socialmediagmbaccountresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().set_google_business_locations(&locationId, &accountId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.post_social_media_posting_oauth_by_locationId_google_locations_by_accountId",
    "path_params": {
      "locationId": "<locationId>",
      "accountId": "<accountId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/oauth/{locationId}/instagram/accounts/{accountId}`

**Get Instagram Professional Accounts**

Operation id: `social-media-posting.get_social_media_posting_oauth_by_locationId_instagram_accounts_by_accountId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Account Location Id |
| `accountId` | string | **yes** | Account Id |

*Response*: [`GetInstagramAccountsResponseDTO`](#getinstagramaccountsresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().get_instagram_professional_accounts(&locationId, &accountId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_oauth_by_locationId_instagram_accounts_by_accountId",
    "path_params": {
      "locationId": "<locationId>",
      "accountId": "<accountId>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/oauth/{locationId}/instagram/accounts/{accountId}`

**Attach Instagram Professional Accounts**

Operation id: `social-media-posting.post_social_media_posting_oauth_by_locationId_instagram_accounts_by_accountId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Account Location Id |
| `accountId` | string | **yes** | Account Id |

*Request body*: [`AttachIGAccountDTO`](#attachigaccountdto)

*Response*: [`SocialMediaInstagramAccountResponseDTO`](#socialmediainstagramaccountresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().attach_instagram_professional_accounts(&locationId, &accountId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.post_social_media_posting_oauth_by_locationId_instagram_accounts_by_accountId",
    "path_params": {
      "locationId": "<locationId>",
      "accountId": "<accountId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/oauth/{locationId}/linkedin/accounts/{accountId}`

**Get Linkedin pages and profile**

Operation id: `social-media-posting.get_social_media_posting_oauth_by_locationId_linkedin_accounts_by_accountId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Account Location Id |
| `accountId` | string | **yes** | Account Id |

*Response*: [`GetLinkedInAccountsResponseDTO`](#getlinkedinaccountsresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().get_linkedin_pages_and_profile(&locationId, &accountId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_oauth_by_locationId_linkedin_accounts_by_accountId",
    "path_params": {
      "locationId": "<locationId>",
      "accountId": "<accountId>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/oauth/{locationId}/linkedin/accounts/{accountId}`

**Attach linkedin pages and profile**

Operation id: `social-media-posting.post_social_media_posting_oauth_by_locationId_linkedin_accounts_by_accountId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Account Location Id |
| `accountId` | string | **yes** | Account Id |

*Request body*: [`AttachLinkedinAccountDTO`](#attachlinkedinaccountdto)

*Response*: [`SocialMediaLinkedInAccountResponseDTO`](#socialmedialinkedinaccountresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().attach_linkedin_pages_and_profile(&locationId, &accountId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.post_social_media_posting_oauth_by_locationId_linkedin_accounts_by_accountId",
    "path_params": {
      "locationId": "<locationId>",
      "accountId": "<accountId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/oauth/{locationId}/tiktok-business/accounts/{accountId}`

**Get Tiktok Business profile**

Operation id: `social-media-posting.get_social_media_posting_oauth_by_locationId_tiktok_business_accounts_by_accountId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Account Location Id |
| `accountId` | string | **yes** | Account Id |

*Response*: [`GetTiktokBusinessAccountResponseDTO`](#gettiktokbusinessaccountresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().get_tiktok_business_profile(&locationId, &accountId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_oauth_by_locationId_tiktok_business_accounts_by_accountId",
    "path_params": {
      "locationId": "<locationId>",
      "accountId": "<accountId>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/oauth/{locationId}/tiktok/accounts/{accountId}`

**Get Tiktok profile**

Operation id: `social-media-posting.get_social_media_posting_oauth_by_locationId_tiktok_accounts_by_accountId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Account Location Id |
| `accountId` | string | **yes** | Account Id |

*Response*: [`GetTiktokAccountResponseDTO`](#gettiktokaccountresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().get_tiktok_profile(&locationId, &accountId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_oauth_by_locationId_tiktok_accounts_by_accountId",
    "path_params": {
      "locationId": "<locationId>",
      "accountId": "<accountId>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/oauth/{locationId}/tiktok/accounts/{accountId}`

**Attach Tiktok profile**

Operation id: `social-media-posting.post_social_media_posting_oauth_by_locationId_tiktok_accounts_by_accountId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Account Location Id |
| `accountId` | string | **yes** | Account Id |

*Request body*: [`AttachTiktokAccountDTO`](#attachtiktokaccountdto)

*Response*: [`SocialMediaTiktokAccountResponseDTO`](#socialmediatiktokaccountresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().attach_tiktok_profile(&locationId, &accountId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.post_social_media_posting_oauth_by_locationId_tiktok_accounts_by_accountId",
    "path_params": {
      "locationId": "<locationId>",
      "accountId": "<accountId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/oauth/{locationId}/twitter/accounts/{accountId}`

**Get Twitter profile**

<div><div> <span style= "display: inline-block; width: 25px; height: 25px; background-color: red; color: black; font-weight: bold; font-size: 24px; text-align: center; line-height: 20px; border: 2px solid black; border-radius: 20%; margin-right: 10px;"> ! </span> <span><strong>As of December 4, 2024, X (formerly Twitter) is no longer supported. We apologise for any inconvenience.</strong></span> </div></div>

Operation id: `social-media-posting.get_social_media_posting_oauth_by_locationId_twitter_accounts_by_accountId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Account Location Id |
| `accountId` | string | **yes** | Account Id |

*Response*: [`GetTwitterAccountsResponseDTO`](#gettwitteraccountsresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().get_twitter_profile(&locationId, &accountId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_oauth_by_locationId_twitter_accounts_by_accountId",
    "path_params": {
      "locationId": "<locationId>",
      "accountId": "<accountId>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/oauth/{locationId}/twitter/accounts/{accountId}`

**Attach Twitter profile**

<div><div> <span style= "display: inline-block; width: 25px; height: 25px; background-color: red; color: black; font-weight: bold; font-size: 24px; text-align: center; line-height: 20px; border: 2px solid black; border-radius: 20%; margin-right: 10px;"> ! </span> <span><strong>As of December 4, 2024, X (formerly Twitter) is no longer supported. We apologise for any inconvenience.</strong></span> </div></div>

Operation id: `social-media-posting.post_social_media_posting_oauth_by_locationId_twitter_accounts_by_accountId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Account Location Id |
| `accountId` | string | **yes** | Account Id |

*Request body*: [`AttachTwitterAccountDTO`](#attachtwitteraccountdto)

*Response*: [`SocialMediaTwitterAccountResponseDTO`](#socialmediatwitteraccountresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().attach_twitter_profile(&locationId, &accountId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.post_social_media_posting_oauth_by_locationId_twitter_accounts_by_accountId",
    "path_params": {
      "locationId": "<locationId>",
      "accountId": "<accountId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/statistics`

**Get Social Media Statistics**

Retrieve analytics data for multiple social media accounts. Provides metrics for the last 7 days with comparison to the previous 7 days. Supports filtering by platforms and specific connected accounts.

Operation id: `social-media-posting.post_social_media_posting_statistics` · `Version: 2021-07-28`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Request body fields*: `profileIds`**\***, `platforms`  (**\*** = required)

*Rust*:

```rust,ignore
use ghl_sdk::services::social_media_posting::GetSocialMediaStatisticsParams;

let params = GetSocialMediaStatisticsParams::new("locationId");
let out = ghl.social_media_posting().get_social_media_statistics(&params, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.post_social_media_posting_statistics",
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

#### `GET /social-media-posting/{locationId}/accounts`

**Get Accounts**

Get list of accounts and groups

Operation id: `social-media-posting.get_social_media_posting_by_locationId_accounts` · `Version: 2021-07-28` · Scopes: `socialplanner/account.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Response*: [`AccountsListResponseDTO`](#accountslistresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().get_accounts(&locationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_by_locationId_accounts",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `DELETE /social-media-posting/{locationId}/accounts/{id}`

**Delete Account**

Delete account and account from group

Operation id: `social-media-posting.delete_social_media_posting_by_locationId_accounts_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | no | Company ID |
| `userId` | string | no | User ID |

*Response*: [`LocationAndAccountDeleteResponseDTO`](#locationandaccountdeleteresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::social_media_posting::DeleteAccountParams;

let params = DeleteAccountParams::new();
let out = ghl.social_media_posting().delete_account(&locationId, &id, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.delete_social_media_posting_by_locationId_accounts_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/{locationId}/categories`

**Get categories by location id**

Operation id: `social-media-posting.get_social_media_posting_by_locationId_categories` · `Version: 2021-07-28` · Scopes: `socialplanner/category.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `searchText` | string | no | Search text string |
| `limit` | string | no | Limit |
| `skip` | string | no | Skip |

*Response*: [`GetByLocationIdResponseDTO`](#getbylocationidresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::social_media_posting::GetCategoriesByLocationIdParams;

let params = GetCategoriesByLocationIdParams::new();
let out = ghl.social_media_posting().get_categories_by_location_id(&locationId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_by_locationId_categories",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/{locationId}/categories/{id}`

**Get categories by id**

Operation id: `social-media-posting.get_social_media_posting_by_locationId_categories_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Category Id |
| `locationId` | string | **yes** | Location Id |

*Response*: [`GetByIdResponseDTO`](#getbyidresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().get_categories_by_id(&id, &locationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_by_locationId_categories_by_id",
    "path_params": {
      "id": "<id>",
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/{locationId}/csv`

**Get Upload Status**

Operation id: `social-media-posting.get_social_media_posting_by_locationId_csv` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `skip` | string | no | — |
| `limit` | string | no | — |
| `includeUsers` | string | no | — |
| `userId` | string | no | User ID |

*Response*: [`GetUploadStatusResponseDTO`](#getuploadstatusresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::social_media_posting::GetUploadStatusParams;

let params = GetUploadStatusParams::new();
let out = ghl.social_media_posting().get_upload_status(&locationId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_by_locationId_csv",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/{locationId}/csv`

**Upload CSV**

Operation id: `social-media-posting.post_social_media_posting_by_locationId_csv` · `Version: 2021-07-28` · Scopes: `socialplanner/csv.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Response*: [`UploadFileResponseDTO`](#uploadfileresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().upload_csv(&locationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.post_social_media_posting_by_locationId_csv",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `DELETE /social-media-posting/{locationId}/csv/{csvId}/post/{postId}`

**Delete CSV Post**

Operation id: `social-media-posting.delete_social_media_posting_by_locationId_csv_by_csvId_post_by_postId` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `postId` | string | **yes** | CSV Post Id |
| `csvId` | string | **yes** | CSV Id |

*Response*: [`DeletePostResponseDTO`](#deletepostresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().delete_csv_post(&locationId, &postId, &csvId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.delete_social_media_posting_by_locationId_csv_by_csvId_post_by_postId",
    "path_params": {
      "locationId": "<locationId>",
      "postId": "<postId>",
      "csvId": "<csvId>"
    }
  }
}
```

</details>

#### `DELETE /social-media-posting/{locationId}/csv/{id}`

**Delete CSV**

Operation id: `social-media-posting.delete_social_media_posting_by_locationId_csv_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `id` | string | **yes** | CSV Id |

*Response*: [`DeleteCsvResponseDTO`](#deletecsvresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().delete_csv(&locationId, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.delete_social_media_posting_by_locationId_csv_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/{locationId}/csv/{id}`

**Get CSV Post**

Operation id: `social-media-posting.get_social_media_posting_by_locationId_csv_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `id` | string | **yes** | CSV Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `skip` | string | no | — |
| `limit` | string | no | — |

*Response*: [`GetCsvPostResponseDTO`](#getcsvpostresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::social_media_posting::GetCsvPostParams;

let params = GetCsvPostParams::new();
let out = ghl.social_media_posting().get_csv_post(&locationId, &id, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_by_locationId_csv_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `PATCH /social-media-posting/{locationId}/csv/{id}`

**Start CSV Finalize**

Operation id: `social-media-posting.patch_social_media_posting_by_locationId_csv_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `id` | string | **yes** | CSV Id |

*Request body*: [`CSVDefaultDTO`](#csvdefaultdto)

*Response*: [`CsvPostStatusResponseDTO`](#csvpoststatusresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().start_csv_finalize(&locationId, &id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.patch_social_media_posting_by_locationId_csv_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/{locationId}/posts`

**Create post**

Create posts for all supported platforms. It is possible to create customized posts per channel by using the same platform account IDs in a request and hitting the create post API multiple times with different summaries and account IDs per platform. The content and media limitations, as well as platform rate limiters corresponding to the respective platforms, are provided in the following reference link: Link: [Platform Limitations](https://help.leadconnectorhq.com/support/solutions/articles/480…

Operation id: `social-media-posting.post_social_media_posting_by_locationId_posts` · `Version: 2021-07-28` · Scopes: `socialplanner/post.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`CreatePostDTO`](#createpostdto)

*Response*: [`CreatePostSuccessfulResponseDTO`](#createpostsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().create_post(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.post_social_media_posting_by_locationId_posts",
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

#### `POST /social-media-posting/{locationId}/posts/bulk-delete`

**Bulk Delete Social Planner Posts**

Deletes multiple posts based on the provided list of post IDs. This operation is useful for clearing up large numbers of posts efficiently. Note: 1.The maximum number of posts that can be deleted in a single request is '50'. 2.However, It will only get deleted in Highlevel database but still it is recommended to be cautious of this operation.

Operation id: `social-media-posting.post_social_media_posting_by_locationId_posts_bulk_delete` · `Version: 2021-07-28`

*Request body*: [`DeletePostsDto`](#deletepostsdto)

*Response*: [`BulkDeleteResponseDto`](#bulkdeleteresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().bulk_delete_social_planner_posts(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.post_social_media_posting_by_locationId_posts_bulk_delete",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/{locationId}/posts/list`

**Get posts**

Get Posts

Operation id: `social-media-posting.post_social_media_posting_by_locationId_posts_list` · `Version: 2021-07-28` · Scopes: `socialplanner/post.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`SearchPostDTO`](#searchpostdto)

*Response*: [`PostSuccessfulResponseDTO`](#postsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().get_posts(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.post_social_media_posting_by_locationId_posts_list",
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

#### `DELETE /social-media-posting/{locationId}/posts/{id}`

**Delete Post**

Operation id: `social-media-posting.delete_social_media_posting_by_locationId_posts_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Post Id |

*Response*: [`DeletePostSuccessfulResponseDTO`](#deletepostsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().delete_post(&locationId, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.delete_social_media_posting_by_locationId_posts_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/{locationId}/posts/{id}`

**Get post**

Operation id: `social-media-posting.get_social_media_posting_by_locationId_posts_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Post Id |

*Response*: [`GetPostSuccessfulResponseDTO`](#getpostsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().get_post(&locationId, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_by_locationId_posts_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `PUT /social-media-posting/{locationId}/posts/{id}`

**Edit post**

Create posts for all supported platforms. It is possible to create customized posts per channel by using the same platform account IDs in a request and hitting the create post API multiple times with different summaries and account IDs per platform. The content and media limitations, as well as platform rate limiters corresponding to the respective platforms, are provided in the following reference link: Link: [Platform Limitations](https://help.leadconnectorhq.com/support/solutions/articles/480…

Operation id: `social-media-posting.put_social_media_posting_by_locationId_posts_by_id` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Post Id |

*Request body*: [`PostCreateRequest`](#postcreaterequest)

*Response*: [`UpdatePostSuccessfulResponseDTO`](#updatepostsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().edit_post(&locationId, &id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.put_social_media_posting_by_locationId_posts_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/{locationId}/set-accounts`

**Set Accounts**

Operation id: `social-media-posting.post_social_media_posting_by_locationId_set_accounts` · `Version: 2021-07-28` · Scopes: `socialplanner/csv.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`SetAccountsDTO`](#setaccountsdto)

*Response*: [`SetAccountsResponseDTO`](#setaccountsresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().set_accounts(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.post_social_media_posting_by_locationId_set_accounts",
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

#### `GET /social-media-posting/{locationId}/tags`

**Get tags by location id**

Operation id: `social-media-posting.get_social_media_posting_by_locationId_tags` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `searchText` | string | no | Search text string |
| `limit` | string | no | Limit |
| `skip` | string | no | Skip |

*Response*: [`GetTagsByLocationIdResponseDTO`](#gettagsbylocationidresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::social_media_posting::GetTagsByLocationIdParams;

let params = GetTagsByLocationIdParams::new();
let out = ghl.social_media_posting().get_tags_by_location_id(&locationId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.get_social_media_posting_by_locationId_tags",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/{locationId}/tags/details`

**Get tags by ids**

Operation id: `social-media-posting.post_social_media_posting_by_locationId_tags_details` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`UpdateTagDTO`](#updatetagdto)

*Response*: [`GetTagsByIdResponseDTO`](#gettagsbyidresponsedto)

*Rust*:

```rust,ignore
let out = ghl.social_media_posting().get_tags_by_ids(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "social-media-posting.post_social_media_posting_by_locationId_tags_details",
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

## Data models — API v2

In Rust: `ghl_models::v2::social_media_posting::*` (enable the `social-media-posting` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/social_media_posting/).

### `AccountsListResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`AccountsListResponseSchema`](#accountslistresponseschema) | no | Requested Results |

### `AccountsListResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `accounts` | Vec<GetAccountSchema> | no | — |
| `groups` | Vec<GetGroupSchema> | no | — |

### `AttachFBAccountDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | JSON | no | — |
| `originId` | String | no | — |
| `name` | String | no | — |
| `avatar` | String | no | — |
| `companyId` | String | no | Company ID |

### `AttachGMBLocationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `location` | JSON | no | — |
| `account` | JSON | no | — |
| `companyId` | String | no | Company ID |

### `AttachIGAccountDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `originId` | String | no | — |
| `name` | String | no | — |
| `avatar` | String | no | — |
| `pageId` | String | **yes** | — |
| `companyId` | String | no | Company ID |

### `AttachLinkedinAccountDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `page`, `group`, `profile`, `location`, `business` | no | — |
| `originId` | String | no | — |
| `name` | String | no | — |
| `avatar` | String | no | — |
| `urn` | String | no | — |
| `companyId` | String | no | Company ID |

### `AttachTiktokAccountDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `page`, `group`, `profile`, `location`, `business` | no | — |
| `originId` | String | no | — |
| `name` | String | no | — |
| `avatar` | String | no | — |
| `verified` | bool | no | — |
| `username` | String | no | — |
| `companyId` | String | no | Company ID |

### `AttachTwitterAccountDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `originId` | String | no | — |
| `name` | String | no | — |
| `username` | String | no | — |
| `avatar` | String | no | — |
| `protected` | bool | no | — |
| `verified` | bool | no | — |
| `companyId` | String | no | Company ID |

### `BulkDeletePostSuccessfulResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `deletedCount` | f64 | no | — |

### `BulkDeleteResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | JSON | **yes** | Message and deleted count |

### `CSVDefaultDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `userId` | String | no | User ID |

### `CSVImportSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Csv Id |
| `locationId` | String | no | locationId |
| `fileName` | String | no | File Name |
| `accountIds` | Vec<String> | no | Account Ids |
| `file` | String | no | File path |
| `status` | String | no | status must be one of the following values: pending, in_progress, completed, failed, in_review, importing, deleted |
| `count` | f64 | no | Posts count |
| `createdBy` | String | no | Created By Id |
| `traceId` | String | no | Trace Id |
| `originId` | String | no | Origin Id |
| `approver` | String | no | Approver Id |
| `createdAt` | String | no | Date Created |

### `CSVMediaResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | String | no | Media Url |
| `type` | String | no | Media Type |
| `size` | f64 | no | Media Size |
| `width` | f64 | no | Media Width |
| `height` | f64 | no | Media Height |
| `aspectRatio` | f64 | no | Media Aspect Ratio |
| `duration` | f64 | no | Media Aspect Ratio |
| `format` | String | no | Media format |
| `videoCodecName` | String | no | Video Codec |
| `frameRate` | f64 | no | Video Frame Rate |
| `audioCodecName` | String | no | Audio Codec |
| `audioChannels` | f64 | no | Audio Channel |
| `displayAspectRatio` | String | no | Display Aspect Ratio |
| `frames` | Vec<String> | no | List of frames |
| `selectedPoster` | f64 | no | Selected Poster |
| `error` | String | no | Error |
| `instagramError` | String | no | Instagram media error. It can we one of the following errors: imageSize, imageType, imageAspectRatio, videoType, videoDuration, videoSize, videoAspectRatio, videoWidthHeight, audioCodec, audioCodecCha… |
| `gmbError` | String | no | GMB media error. It can be one of the following errors: imageSize, imageDimension, imageType |
| `facebookError` | String | no | Facebook media error. It can be one of the following errors: imageSize, imageType, videoDuration, videoSize |
| `linkedinError` | String | no | LinkedIn media error. It can be one of the following errors: imageSize, imageType, videoType, videoDuration, videoSize |
| `twitterError` | String | no | Twitter media error. It can be one of the following errors: imageSize, videoType, videoDuration, videoSize |
| `tiktokError` | String | no | Tiktok media error. It can be one of the following errors: videoType, videoDuration, videoSize, videoWidthHeight, videoCodec, videoFrameRate |
| `tiktokBusinessError` | String | no | Tikok Business media error. It can be one of the following errors: videoType, videoDuration, videoSize, videoWidthHeight, videoCodec, videoFrameRate |
| `invalidError` | String | no | Media error. It can be one of the following values: imageSize, imageWidth |

### `CSVPostSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `accountIds` | Vec<String> | no | Account Ids |
| `link` | [`IOgTagsSchema`](#iogtagsschema) | no | OG Tag |
| `medias` | Vec<CSVMediaResponseSchema> | no | Post Media List |
| `scheduleDate` | String | no | — |
| `summary` | String | no | — |
| `followUpComment` | String | no | — |
| `type` | JSON | no | — |
| `tiktokPostDetails` | [`TiktokPostSchema`](#tiktokpostschema) | no | Tiktok Post Details |
| `gmbPostDetails` | [`GMBPostSchema`](#gmbpostschema) | no | GMB Post Details |
| `errorMessage` | String | no | Error Description |

### `CSVResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `csv` | [`CsvResponse`](#csvresponse) | no | CSV Data |

### `CategorySchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Category Name |
| `primaryColor` | String | no | Color For Category |
| `secondaryColor` | String | no | Secondary Color |
| `locationId` | String | no | Location ID |
| `_id` | String | no | ID |
| `createdBy` | String | no | Created By User |
| `deleted` | bool | **yes** | Deleted Value |
| `createdAt` | String | no | — |
| `updatedAt` | String | no | — |

### `CreatePostDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `accountIds` | Vec<String> | **yes** | Account Ids |
| `summary` | String | no | Post Content The limitations of content as per the platforms is provided through the reference link in API description. The summary will be trimmed based on the limit |
| `media` | Vec<PostMediaSchema> | no | Post Media Data The limitations of media as per the platforms is provided through the reference link in API description |
| `status` | JSON | no | Status must be one of the following values: null, in_progress, draft, failed, published, scheduled, in_review, notification_sent, deleted |
| `scheduleDate` | String | no | Schedule Date |
| `createdBy` | String | no | Created By |
| `followUpComment` | String | no | Follow Up Comment on platform. It is not allowed on Tiktok and GMB accounts and there is a limit of 280 charecters for twitter account |
| `ogTagsDetails` | [`OgTagsSchema`](#ogtagsschema) | no | Og Tags Meta Data |
| `type` | JSON | **yes** | Post Type must be one of the following values: - post, story, reel |
| `postApprovalDetails` | [`PostApprovalSchema`](#postapprovalschema) | no | Post Approval Details |
| `scheduleTimeUpdated` | bool | no | if schedule datetime is updated |
| `tags` | Vec<String> | no | Array of Tag Value |
| `categoryId` | String | no | Category Id |
| `tiktokPostDetails` | [`TiktokPostSchema`](#tiktokpostschema) | no | Tiktok Post Details |
| `gmbPostDetails` | [`GMBPostSchema`](#gmbpostschema) | no | GMB Post Details |
| `userId` | String | **yes** | User ID |

### `CreatePostSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`CreatePostSuccessfulResponseSchema`](#createpostsuccessfulresponseschema) | no | Requested Results |

### `CreatePostSuccessfulResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `post` | [`GetPostFormattedSchema`](#getpostformattedschema) | no | Post Data |

### `CsvPostStatusResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |

### `CsvResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | no | — |
| `fileName` | String | no | — |
| `accountIds` | Vec<String> | no | Account Ids |
| `file` | String | no | — |
| `status` | JSON | no | status must be one of the following values: pending, in_progress, completed, failed, in_review, importing, deleted |
| `count` | f64 | no | — |
| `createdBy` | String | no | — |
| `traceId` | String | no | — |
| `originId` | String | no | — |
| `approver` | String | no | — |

### `DateSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `year` | f64 | **yes** | — |
| `month` | f64 | **yes** | — |
| `day` | f64 | **yes** | — |

### `DeleteAccountResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | no | Location Id |
| `id` | String | no | Id |

### `DeleteCsvResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`CSVResponseSchema`](#csvresponseschema) | no | Requested Results |

### `DeletePostResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`DeletePostResponseSchema`](#deletepostresponseschema) | no | Requested Results |

### `DeletePostResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `postId` | String | **yes** | Post Id |

### `DeletePostSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`DeletePostSuccessfulResponseSchema`](#deletepostsuccessfulresponseschema) | no | Requested Results |

### `DeletePostSuccessfulResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `postId` | String | no | — |

### `DeletePostsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `postIds` | Vec<String> | no | Requested Results |

### `EndDateSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `endDate` | [`DateSchema`](#dateschema) | no | End Date |
| `endTime` | [`TimeSchema`](#timeschema) | no | End Time |

### `FacebookPageSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `avatar` | String | no | — |
| `isOwned` | bool | no | — |
| `isConnected` | bool | no | — |

### `FormatedApprovalDetails`

| Field | Type | Required | Description |
|---|---|---|---|
| `approver` | String | no | Approver |
| `requesterNote` | String | no | Requester Notes |
| `approverNote` | String | no | Approver Notes |
| `approvalStatus` | JSON | no | Approval Status must be one of the following values: pending, approved, rejected, not_required |
| `approverUser` | [`PostUserSchema`](#postuserschema) | no | Approver User Details |

### `GMBPostSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `gmbEventType` | String | no | gmbEventType must be one of the following values: STANDARD, EVENT, OFFER |
| `title` | String | no | Title |
| `offerTitle` | String | no | Offer Title |
| `startDate` | [`StartDateSchema`](#startdateschema) | no | Start Date |
| `endDate` | [`EndDateSchema`](#enddateschema) | no | End Date |
| `termsConditions` | String | no | Terms Condition Url |
| `url` | String | no | Url |
| `couponCode` | String | no | Coupon Code |
| `redeemOnlineUrl` | String | no | Redeem Online Url |
| `actionType` | JSON | no | Action Type must be one of the following values: none, order, book, shop, learn_more, call, sign_up |

### `GetAccountSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `oauthId` | String | no | — |
| `profileId` | String | no | — |
| `name` | String | no | — |
| `platform` | String | no | platform must be one of the following values: google, facebook, instagram, linkedin, twitter, tiktok |
| `type` | String | no | — |
| `expire` | String | no | — |
| `isExpired` | bool | no | — |
| `meta` | JSON | no | — |

### `GetByIdResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`GetCategorySchema`](#getcategoryschema) | no | Requested Results |

### `GetByIdResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Category Name |
| `primaryColor` | String | no | Color For Category |
| `secondaryColor` | String | no | Secondary Color |
| `locationId` | String | no | Location ID |
| `_id` | String | no | ID |
| `createdBy` | String | no | Created By User |
| `deleted` | bool | **yes** | Deleted Value |
| `message` | String | no | Message |
| `createdAt` | String | no | — |
| `updatedAt` | String | no | — |

### `GetByLocationIdResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`GetByLocationIdResponseSchema`](#getbylocationidresponseschema) | no | Requested Results |

### `GetByLocationIdResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `count` | f64 | **yes** | Count |
| `categories` | Vec<CategorySchema> | **yes** | Meta Data |

### `GetCategorySchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `category` | [`GetByIdResponseSchema`](#getbyidresponseschema) | no | Category Schema |

### `GetCsvPostResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`GetCsvPostResponseSchema`](#getcsvpostresponseschema) | no | Requested Results |

### `GetCsvPostResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `csv` | [`CSVImportSchema`](#csvimportschema) | no | CSV Data |
| `count` | f64 | no | — |
| `posts` | Vec<CSVPostSchema> | no | CSV Posts |

### `GetFacebookAccountsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`GetFacebookAccountsSchema`](#getfacebookaccountsschema) | no | Requested Results |

### `GetFacebookAccountsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `pages` | Vec<FacebookPageSchema> | no | Facebook Pages Details |

### `GetGoogleLocationAccountSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `locations` | [`GetGoogleLocationSchema`](#getgooglelocationschema) | no | Locations |

### `GetGoogleLocationResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`GetGoogleLocationAccountSchema`](#getgooglelocationaccountschema) | no | Requested Results |

### `GetGoogleLocationSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `location` | [`GoogleLocationSchema`](#googlelocationschema) | no | Google Location Details |
| `account` | [`GoogleAccountsSchema`](#googleaccountsschema) | no | Google Account Details |

### `GetGroupSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Group Id |
| `name` | String | **yes** | name of group |
| `accountIds` | Vec<String> | **yes** | — |

### `GetInstagramAccountsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`GetInstagramAccountsSchema`](#getinstagramaccountsschema) | no | Requested Results |

### `GetInstagramAccountsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `accounts` | Vec<InstagramAccountSchema> | no | Instagram Account Details |

### `GetLinkedInAccountSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `pages` | Vec<LinkedInPageSchema> | no | LinkedIn Pages |
| `profile` | Vec<LinkedInProfileSchema> | no | LinkedIn Profile Details |

### `GetLinkedInAccountsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`GetLinkedInAccountSchema`](#getlinkedinaccountschema) | no | Requested Results |

### `GetPostFormattedSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | — |
| `source` | String — `composer`, `csv`, `recurring`, `review`, `rss` | no | source must be one of the following values: composer, recurring, csv |
| `locationId` | String | **yes** | Location Id |
| `platform` | String | no | platform must be one of the following values: google, facebook, instagram, linkedin, twitter, tiktok |
| `displayDate` | String | no | — |
| `createdAt` | String | no | — |
| `updatedAt` | String | no | — |
| `accountId` | String | no | Account Id |
| `error` | String | **yes** | Error |
| `postId` | String | no | — |
| `publishedAt` | String | no | — |
| `accountIds` | Vec<String> | no | Account Ids |
| `summary` | String | no | — |
| `media` | Vec<PostMediaSchema> | no | Post Media Data The limitations of media as per the platforms is provided through the reference link in API description |
| `status` | JSON | no | Status must be one of the following values: in_progress, draft, failed, published, scheduled, in_review, notification_sent, deleted |
| `createdBy` | String | no | — |
| `type` | JSON | **yes** | Post Type must be one of the following values: - post, story, reel |
| `tags` | Vec<String> | no | Tag Ids |
| `ogTagsDetails` | [`OgTagsSchema`](#ogtagsschema) | no | Og Tags Meta Data |
| `postApprovalDetails` | [`FormatedApprovalDetails`](#formatedapprovaldetails) | no | Post Approval Details |
| `tiktokPostDetails` | [`TiktokPostSchema`](#tiktokpostschema) | no | Tiktok Post Details |
| `gmbPostDetails` | [`GMBPostSchema`](#gmbpostschema) | no | GMB Post Details |
| `user` | [`PostUserSchema`](#postuserschema) | no | User |

### `GetPostSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`GetPostSuccessfulResponseSchema`](#getpostsuccessfulresponseschema) | no | Requested Results |

### `GetPostSuccessfulResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `post` | [`GetPostFormattedSchema`](#getpostformattedschema) | no | Post Data |

### `GetTagsByIdResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`GetTagsByIdResponseSchema`](#gettagsbyidresponseschema) | no | Requested Results |

### `GetTagsByIdResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `tags` | Vec<SocialMediaTagSchema> | **yes** | Social Media Tag Data |
| `count` | f64 | no | Count |

### `GetTagsByLocationIdResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`GetTagsByLocationIdResponseSchema`](#gettagsbylocationidresponseschema) | no | Requested Results |

### `GetTagsByLocationIdResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `tags` | Vec<SocialMediaTagSchema> | no | Tags Data |
| `count` | f64 | no | Count |

### `GetTiktokAccountResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`GetTiktokAccountSchema`](#gettiktokaccountschema) | no | Requested Results |

### `GetTiktokAccountSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `profile` | Vec<TiktokProfileSchema> | no | Tiktok Business Account |

### `GetTiktokBusinessAccountResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`GetTiktokBusinessAccountSchema`](#gettiktokbusinessaccountschema) | no | Requested Results |

### `GetTiktokBusinessAccountSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `profile` | Vec<TiktokProfileSchema> | no | Tiktok Profile |

### `GetTwitterAccountsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`GetTwitterAccountsSchema`](#gettwitteraccountsschema) | no | Requested Results |

### `GetTwitterAccountsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `profile` | Vec<TwitterProfileSchema> | no | Twitter Profile Details |

### `GetUploadStatusResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`GetUploadStatusResponseSchema`](#getuploadstatusresponseschema) | no | Requested Results |

### `GetUploadStatusResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `csvs` | [`CSVImportSchema`](#csvimportschema) | **yes** | CSV Data |
| `count` | f64 | **yes** | — |

### `GoogleAccountsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | — |
| `accountName` | String | no | — |
| `type` | String | no | — |
| `verificationState` | String | no | — |
| `vettedState` | String | no | — |

### `GoogleLocationSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | — |
| `storeCode` | String | no | — |
| `title` | String | no | — |
| `metadata` | JSON | no | Meta data not related to User |
| `storefrontAddress` | JSON | no | Store front address |
| `relationshipData` | JSON | no | All locations and chain related to this one |
| `maxLocation` | bool | no | — |
| `isVerified` | bool | no | — |
| `isConnected` | bool | no | — |

### `IOgTagsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | String | no | Tag url |
| `ogDescription` | String | no | Tag description |
| `ogImage` | [`OgImageSchema`](#ogimageschema) | no | Tag description |
| `ogTitle` | String | no | Tag Title |
| `ogUrl` | String | no | Tag Url |
| `ogSiteName` | String | no | Site Name |
| `error` | String | no | Og Tag Error |

### `InstagramAccountSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `avatar` | String | no | — |
| `pageId` | String | no | — |
| `isConnected` | bool | no | — |

### `LinkedInPageSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Page ID |
| `name` | String | no | LinkedIn Page Name |
| `avatar` | String | no | Profile Avatar url |
| `urn` | String | no | URN |
| `isConnected` | bool | no | is connected to app |

### `LinkedInProfileSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Id |
| `name` | String | no | Name of profile |
| `avatar` | String | no | Profile avatar |
| `urn` | String | no | URN |
| `isConnected` | bool | no | is connected to app |

### `LocationAndAccountDeleteResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`DeleteAccountResponseSchema`](#deleteaccountresponseschema) | no | Requested Results |

### `OgImageSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | String | no | Image url |
| `width` | f64 | no | Image width |
| `height` | f64 | no | Image height |
| `type` | String | no | Image Type |

### `OgTagsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `metaImage` | String | no | Meta Image |
| `metaLink` | String | no | Meta Link |

### `PostApprovalSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `approver` | String | no | Approver |
| `requesterNote` | String | no | Requester Notes |
| `approverNote` | String | no | Approver Notes |
| `approvalStatus` | JSON | no | Approval Status must be one of the following values: pending, approved, rejected, not_required |

### `PostCreateRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `accountIds` | Vec<String> | no | Account Ids |
| `summary` | String | no | Post Content The limitations of content as per the platforms is provided through the reference link in API description |
| `media` | Vec<PostMediaSchema> | no | Post Media Data The limitations of media as per the platforms is provided through the reference link in API description |
| `status` | JSON | no | Status must be one of the following values: in_progress, draft, failed, published, scheduled, in_review, notification_sent, deleted |
| `scheduleDate` | String | no | Schedule Date |
| `createdBy` | String | no | Created By |
| `followUpComment` | String | no | Follow Up Comment on platform. It is not allowed on Tiktok and GMB accounts and there is a limit of 280 charecters for twitter account |
| `ogTagsDetails` | [`OgTagsSchema`](#ogtagsschema) | no | Og Tags Meta Data |
| `type` | JSON | **yes** | Post Type must be one of the following values: - post, story, reel |
| `postApprovalDetails` | [`PostApprovalSchema`](#postapprovalschema) | no | Post Approval Details |
| `scheduleTimeUpdated` | bool | no | if schedule datetime is updated |
| `tags` | Vec<String> | no | Array of Tag Value |
| `categoryId` | String | no | Category Id |
| `tiktokPostDetails` | [`TiktokPostSchema`](#tiktokpostschema) | no | Tiktok Post Details |
| `gmbPostDetails` | [`GMBPostSchema`](#gmbpostschema) | no | GMB Post Details |
| `userId` | String | no | User ID |

### `PostMediaSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | String | **yes** | — |
| `caption` | String | no | — |
| `type` | String | no | — |
| `thumbnail` | String | no | — |
| `defaultThumb` | String | no | — |
| `id` | String | no | — |

### `PostSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`PostSuccessfulResponseSchema`](#postsuccessfulresponseschema) | no | Requested Results |

### `PostSuccessfulResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `posts` | Vec<GetPostFormattedSchema> | no | Post Data |
| `count` | f64 | no | — |

### `PostUserSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | User Id |
| `title` | String | **yes** | Title |
| `firstName` | String | **yes** | First name |
| `lastName` | String | **yes** | Last name |
| `profilePhoto` | String | **yes** | Profile photo |
| `phone` | String | **yes** | Phone number |
| `email` | String | **yes** | Email Id |

### `SearchPostDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String | no | type must be one of the following values: recent, all, scheduled, draft, failed, in_review, published, in_progress and deleted |
| `accounts` | String | no | List of account Ids seperated by comma as a string |
| `skip` | String | **yes** | — |
| `limit` | String | **yes** | — |
| `fromDate` | String | **yes** | From Date |
| `toDate` | String | **yes** | To Date |
| `includeUsers` | String | **yes** | Include User Data |
| `postType` | JSON | no | Post Type must be one of the following values: - post, story, reel |

### `SetAccountsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `accountIds` | Vec<String> | **yes** | Account Ids |
| `filePath` | String | **yes** | File path |
| `rowsCount` | f64 | **yes** | Entires Count. rowcCount must be between 1 and number of posts in CSV |
| `fileName` | String | **yes** | Name of file |
| `approver` | String | no | — |
| `userId` | String | no | User ID |

### `SetAccountsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |

### `SocialGoogleMediaAccountSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | — |
| `oAuthId` | String | no | — |
| `oldId` | String | no | — |
| `locationId` | String | no | — |
| `originId` | String | no | — |
| `platform` | JSON | no | — |
| `type` | JSON | no | — |
| `name` | String | no | — |
| `avatar` | String | no | — |
| `meta` | JSON | no | — |
| `active` | bool | no | — |
| `deleted` | bool | no | — |
| `createdAt` | String | no | created date |
| `updatedAt` | String | no | updated date |

### `SocialMediaFBAccountResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`SocialMediaFacebookAccountSchema`](#socialmediafacebookaccountschema) | no | Requested Results |

### `SocialMediaFacebookAccountSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | — |
| `oAuthId` | String | no | — |
| `oldId` | String | no | — |
| `locationId` | String | no | — |
| `originId` | String | no | — |
| `platform` | JSON | no | — |
| `type` | JSON | no | type value must be page |
| `name` | String | no | — |
| `avatar` | String | no | — |
| `meta` | JSON | no | — |
| `active` | bool | no | — |
| `deleted` | bool | no | — |
| `createdAt` | String | no | created date |
| `updatedAt` | String | no | updated date |

### `SocialMediaGmbAccountResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`SocialGoogleMediaAccountSchema`](#socialgooglemediaaccountschema) | no | Requested Results |

### `SocialMediaInstagramAccountResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`SocialMediaInstagramAccountSchema`](#socialmediainstagramaccountschema) | no | Requested Results |

### `SocialMediaInstagramAccountSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | — |
| `oAuthId` | String | no | — |
| `oldId` | String | no | — |
| `locationId` | String | no | — |
| `originId` | String | no | — |
| `platform` | JSON | no | — |
| `type` | JSON | no | — |
| `name` | String | no | — |
| `avatar` | String | no | — |
| `meta` | JSON | no | — |
| `active` | bool | no | — |
| `deleted` | bool | no | — |
| `createdAt` | String | no | created date |
| `updatedAt` | String | no | updated date |

### `SocialMediaLinkedInAccountResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`SocialMediaLinkedInAccountSchema`](#socialmedialinkedinaccountschema) | no | Requested Results |

### `SocialMediaLinkedInAccountSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | — |
| `oAuthId` | String | no | — |
| `oldId` | String | no | — |
| `locationId` | String | no | — |
| `originId` | String | no | — |
| `platform` | JSON | no | — |
| `type` | JSON | no | type must be one of the following values: page, profile |
| `name` | String | no | — |
| `avatar` | String | no | — |
| `meta` | JSON | no | — |
| `active` | bool | no | — |
| `deleted` | bool | no | — |
| `createdAt` | String | no | created date |
| `updatedAt` | String | no | updated date |

### `SocialMediaTagSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `tag` | String | no | Tag Name |
| `locationId` | String | no | Location Id |
| `_id` | String | no | ID |
| `createdBy` | String | no | Created By User Id |
| `deleted` | bool | no | Deleted boolean value |
| `createdAt` | String | no | — |
| `updatedAt` | String | no | — |

### `SocialMediaTiktokAccountResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`SocialMediaTiktokAccountSchema`](#socialmediatiktokaccountschema) | no | Requested Results |

### `SocialMediaTiktokAccountSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | — |
| `oAuthId` | String | no | — |
| `oldId` | String | no | — |
| `locationId` | String | no | — |
| `originId` | String | no | — |
| `platform` | JSON | no | — |
| `type` | JSON | no | type must be one of the following values: profile, business |
| `name` | String | no | — |
| `avatar` | String | no | — |
| `meta` | JSON | no | — |
| `active` | bool | no | — |
| `deleted` | bool | no | — |
| `createdAt` | String | no | created date |
| `updatedAt` | String | no | updated date |

### `SocialMediaTwitterAccountResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`SocialMediaTwitterAccountSchema`](#socialmediatwitteraccountschema) | no | Requested Results |

### `SocialMediaTwitterAccountSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | — |
| `oAuthId` | String | no | — |
| `oldId` | String | no | — |
| `locationId` | String | no | — |
| `originId` | String | no | — |
| `platform` | JSON | no | — |
| `type` | JSON | no | — |
| `name` | String | no | — |
| `avatar` | String | no | — |
| `meta` | JSON | no | — |
| `active` | bool | no | — |
| `deleted` | bool | no | — |
| `createdAt` | String | no | created date |
| `updatedAt` | String | no | updated date |

### `StartDateSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `startDate` | [`DateSchema`](#dateschema) | no | Start Date |
| `startTime` | [`TimeSchema`](#timeschema) | no | Start Time |

### `TiktokPostSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `privacyLevel` | JSON | no | privacy level is an enum and must be one of the following values: PUBLIC_TO_EVERYONE, MUTUAL_FOLLOW_FRIENDS, SELF_ONLY |
| `promoteOtherBrand` | bool | no | promote other brand |
| `enableComment` | bool | no | enable comment |
| `enableDuet` | bool | no | enable duet |
| `enableStitch` | bool | no | enable stitch |
| `videoDisclosure` | bool | no | video disclosure |
| `promoteYourBrand` | bool | no | promote your brand |

### `TiktokProfileSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Id |
| `name` | String | no | Name of account |
| `username` | String | no | Username of account |
| `avatar` | String | no | Avatar of profile account |
| `verified` | bool | no | Is verified |
| `isConnected` | bool | no | Is connected |
| `type` | JSON | no | Tiktok Account Type must be one of the following values: business, profile |

### `TimeSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `hours` | f64 | **yes** | — |
| `minutes` | f64 | **yes** | — |
| `seconds` | f64 | **yes** | — |

### `TwitterProfileSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | ID of profile |
| `name` | String | no | Name of profile |
| `username` | String | no | Username of profile |
| `avatar` | String | no | Avatar of profile |
| `protected` | bool | no | Is protected |
| `verified` | bool | no | Is verified |
| `isConnected` | bool | no | Is connected |

### `UpdatePostSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |

### `UpdateTagDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `tagIds` | Vec<String> | **yes** | Array of Tag Ids |

### `UploadCSVDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `file` | String | no | — |

### `UploadFileResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`UploadFileResponseSchema`](#uploadfileresponseschema) | no | Requested Results |

### `UploadFileResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `filePath` | String | no | — |
| `rowsCount` | f64 | no | — |
| `fileName` | String | no | — |

