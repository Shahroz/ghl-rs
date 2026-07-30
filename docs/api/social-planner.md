# `social-planner`

**45** operations / **228** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `social-planner` cargo feature on `ghl-sdk`, then call any of the 45 generated methods on `ghl.v3().social_planner()` (v3):

```toml
ghl-sdk = { version = "0.5", features = ["social-planner"] }
```


## Endpoints — API v3

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `POST` | `/social-media-posting/category/queues` | Create a new category queue | `create_a_new_category_queue()` | `v3:social-planner.post_social_media_posting_category_queues` |
| `GET` | `/social-media-posting/category/queues/available-categories` | Get all categories with their queue status | `get_all_categories_with_their_queue_status()` | `v3:social-planner.get_social_media_posting_category_queues_available_categories` |
| `POST` | `/social-media-posting/category/queues/list` | Fetch category queues for a location | `fetch_category_queues_for_a_location()` | `v3:social-planner.post_social_media_posting_category_queues_list` |
| `POST` | `/social-media-posting/category/queues/list/calendar` | Get scheduled posts calendar view | `get_scheduled_posts_calendar_view()` | `v3:social-planner.post_social_media_posting_category_queues_list_calendar` |
| `DELETE` | `/social-media-posting/category/queues/{postId}/active-post` | Delete an active post and schedule the next one | `delete_an_active_post_and_schedule_the_next_one()` | `v3:social-planner.delete_social_media_posting_category_queues_by_postId_active_post` |
| `GET` | `/social-media-posting/category/queues/{queueId}` | Fetch a category queue by ID | `fetch_a_category_queue_by_id()` | `v3:social-planner.get_social_media_posting_category_queues_by_queueId` |
| `PUT` | `/social-media-posting/category/queues/{queueId}` | Update queue settings or status | `update_queue_settings_or_status()` | `v3:social-planner.put_social_media_posting_category_queues_by_queueId` |
| `POST` | `/social-media-posting/category/queues/{queueId}/create/item` | Create a new item in the queue | `create_a_new_item_in_the_queue()` | `v3:social-planner.post_social_media_posting_category_queues_by_queueId_create_item` |
| `POST` | `/social-media-posting/category/queues/{queueId}/edit/calendar` | Fetch calendar view for an edit session | `fetch_calendar_view_for_an_edit_session()` | `v3:social-planner.post_social_media_posting_category_queues_by_queueId_edit_calendar` |
| `POST` | `/social-media-posting/category/queues/{queueId}/edit/discard` | Discard edit session changes | `discard_edit_session_changes()` | `v3:social-planner.post_social_media_posting_category_queues_by_queueId_edit_discard` |
| `POST` | `/social-media-posting/category/queues/{queueId}/edit/save` | Save edit session changes | `save_edit_session_changes()` | `v3:social-planner.post_social_media_posting_category_queues_by_queueId_edit_save` |
| `POST` | `/social-media-posting/category/queues/{queueId}/edit/start` | Start or resume an edit session | `start_or_resume_an_edit_session()` | `v3:social-planner.post_social_media_posting_category_queues_by_queueId_edit_start` |
| `POST` | `/social-media-posting/category/queues/{queueId}/items` | Fetch items from a queue | `fetch_items_from_a_queue()` | `v3:social-planner.post_social_media_posting_category_queues_by_queueId_items` |
| `DELETE` | `/social-media-posting/category/queues/{queueId}/items/{itemId}` | Delete an item from a queue | `delete_an_item_from_a_queue()` | `v3:social-planner.delete_social_media_posting_category_queues_by_queueId_items_by_itemId` |
| `PUT` | `/social-media-posting/category/queues/{queueId}/items/{itemId}` | Update an item in a queue | `update_an_item_in_a_queue()` | `v3:social-planner.put_social_media_posting_category_queues_by_queueId_items_by_itemId` |
| `POST` | `/social-media-posting/category/queues/{queueId}/items/{itemId}/clone` | Clone a queue item | `clone_a_queue_item()` | `v3:social-planner.post_social_media_posting_category_queues_by_queueId_items_by_itemId_clone` |
| `PUT` | `/social-media-posting/category/queues/{queueId}/items/{itemId}/reset` | Reset an item in a queue | `reset_an_item_in_a_queue()` | `v3:social-planner.put_social_media_posting_category_queues_by_queueId_items_by_itemId_reset` |
| `POST` | `/social-media-posting/category/queues/{queueId}/slots` | Fetch slot information for queue items | `fetch_slot_information_for_queue_items()` | `v3:social-planner.post_social_media_posting_category_queues_by_queueId_slots` |
| `POST` | `/social-media-posting/comments/{platform}` | Create a comment or reply | `create_a_comment_or_reply()` | `v3:social-planner.post_social_media_posting_comments_by_platform` |
| `POST` | `/social-media-posting/comments/{platform}/list` | List comments for a post or thread | `list_comments_for_a_post_or_thread()` | `v3:social-planner.post_social_media_posting_comments_by_platform_list` |
| `DELETE` | `/social-media-posting/comments/{platform}/{id}/like` | Unlike a comment | `unlike_a_comment()` | `v3:social-planner.delete_social_media_posting_comments_by_platform_by_id_like` |
| `POST` | `/social-media-posting/comments/{platform}/{id}/like` | Like a comment | `like_a_comment()` | `v3:social-planner.post_social_media_posting_comments_by_platform_by_id_like` |
| `GET` | `/social-media-posting/oauth/{locationId}/{platform}/accounts/{accountId}` | Get Available Accounts (Step 2 of 3) | `get_available_accounts_step_2_of_3()` | `v3:social-planner.get_social_media_posting_oauth_by_locationId_by_platform_accounts_by_accountId` |
| `POST` | `/social-media-posting/oauth/{locationId}/{platform}/accounts/{accountId}` | Connect Account (Step 3 of 3) | `connect_account_step_3_of_3()` | `v3:social-planner.post_social_media_posting_oauth_by_locationId_by_platform_accounts_by_accountId` |
| `GET` | `/social-media-posting/oauth/{platform}/start` | Start OAuth Flow (Step 1 of 3) | `start_o_auth_flow_step_1_of_3()` | `v3:social-planner.get_social_media_posting_oauth_by_platform_start` |
| `POST` | `/social-media-posting/statistics` | Get Social Media Statistics | `get_social_media_statistics()` | `v3:social-planner.post_social_media_posting_statistics` |
| `GET` | `/social-media-posting/{locationId}/accounts` | Get Accounts | `get_accounts()` | `v3:social-planner.get_social_media_posting_by_locationId_accounts` |
| `DELETE` | `/social-media-posting/{locationId}/accounts/{id}` | Delete Account | `delete_account()` | `v3:social-planner.delete_social_media_posting_by_locationId_accounts_by_id` |
| `GET` | `/social-media-posting/{locationId}/categories` | Get categories by location id | `get_categories_by_location_id()` | `v3:social-planner.get_social_media_posting_by_locationId_categories` |
| `GET` | `/social-media-posting/{locationId}/categories/{id}` | Get categories by id | `get_categories_by_id()` | `v3:social-planner.get_social_media_posting_by_locationId_categories_by_id` |
| `GET` | `/social-media-posting/{locationId}/csv` | Get Upload Status | `get_upload_status()` | `v3:social-planner.get_social_media_posting_by_locationId_csv` |
| `POST` | `/social-media-posting/{locationId}/csv` | Upload CSV | `upload_csv()` | `v3:social-planner.post_social_media_posting_by_locationId_csv` |
| `DELETE` | `/social-media-posting/{locationId}/csv/{csvId}/post/{postId}` | Delete CSV Post | `delete_csv_post()` | `v3:social-planner.delete_social_media_posting_by_locationId_csv_by_csvId_post_by_postId` |
| `DELETE` | `/social-media-posting/{locationId}/csv/{id}` | Delete CSV | `delete_csv()` | `v3:social-planner.delete_social_media_posting_by_locationId_csv_by_id` |
| `GET` | `/social-media-posting/{locationId}/csv/{id}` | Get CSV Post | `get_csv_post()` | `v3:social-planner.get_social_media_posting_by_locationId_csv_by_id` |
| `PATCH` | `/social-media-posting/{locationId}/csv/{id}` | Start CSV Finalize | `start_csv_finalize()` | `v3:social-planner.patch_social_media_posting_by_locationId_csv_by_id` |
| `POST` | `/social-media-posting/{locationId}/posts` | Create post | `create_post()` | `v3:social-planner.post_social_media_posting_by_locationId_posts` |
| `POST` | `/social-media-posting/{locationId}/posts/bulk-delete` | Bulk Delete Social Planner Posts | `bulk_delete_social_planner_posts()` | `v3:social-planner.post_social_media_posting_by_locationId_posts_bulk_delete` |
| `POST` | `/social-media-posting/{locationId}/posts/list` | Get posts | `get_posts()` | `v3:social-planner.post_social_media_posting_by_locationId_posts_list` |
| `DELETE` | `/social-media-posting/{locationId}/posts/{id}` | Delete Post | `delete_post()` | `v3:social-planner.delete_social_media_posting_by_locationId_posts_by_id` |
| `GET` | `/social-media-posting/{locationId}/posts/{id}` | Get post | `get_post()` | `v3:social-planner.get_social_media_posting_by_locationId_posts_by_id` |
| `PUT` | `/social-media-posting/{locationId}/posts/{id}` | Edit post | `edit_post()` | `v3:social-planner.put_social_media_posting_by_locationId_posts_by_id` |
| `POST` | `/social-media-posting/{locationId}/set-accounts` | Set Accounts | `set_accounts()` | `v3:social-planner.post_social_media_posting_by_locationId_set_accounts` |
| `GET` | `/social-media-posting/{locationId}/tags` | Get tags by location id | `get_tags_by_location_id()` | `v3:social-planner.get_social_media_posting_by_locationId_tags` |
| `POST` | `/social-media-posting/{locationId}/tags/details` | Get tags by ids | `get_tags_by_ids()` | `v3:social-planner.post_social_media_posting_by_locationId_tags_details` |

### Endpoint details — v3

#### `POST /social-media-posting/category/queues`

**Create a new category queue**

Creates a queue in draft status for a category. Published posts are auto-added. Use update endpoint to activate.

Operation id: `v3:social-planner.post_social_media_posting_category_queues` · `Version: v3` · Scopes: `socialplanner/category.write`

*Request body*: [`CreateCategoryQueueDTO`](#createcategoryqueuedto)

*Response*: [`WrappedCreateCategoryQueueResponseDTO`](#wrappedcreatecategoryqueueresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().create_a_new_category_queue(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_category_queues",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/category/queues/available-categories`

**Get all categories with their queue status**

Returns categories with status: "available" (no queue), "in_queue" (active/paused queue), or "draft" (queue in draft).

Operation id: `v3:social-planner.get_social_media_posting_category_queues_available_categories` · `Version: v3` · Scopes: `socialplanner/category.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `skip` | string | no | Number of items to skip |
| `limit` | string | no | Maximum number of items to return |
| `q` | string | no | Search query |

*Response*: [`WrappedFetchAvailableCategoriesResponseDTO`](#wrappedfetchavailablecategoriesresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::social_planner::GetAllCategoriesWithTheirQueueStatusParams;

let params = GetAllCategoriesWithTheirQueueStatusParams::new("locationId");
let out = ghl.v3().social_planner().get_all_categories_with_their_queue_status(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.get_social_media_posting_category_queues_available_categories",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/category/queues/list`

**Fetch category queues for a location**

Retrieves a paginated list of all category queues for a given location, excluding any that have been marked as deleted.

Operation id: `v3:social-planner.post_social_media_posting_category_queues_list` · `Version: v3` · Scopes: `socialplanner/category.readonly`

*Request body*: [`FetchCategoryQueuesDTO`](#fetchcategoryqueuesdto)

*Response*: [`WrappedFetchCategoryQueuesResponseDTO`](#wrappedfetchcategoryqueuesresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().fetch_category_queues_for_a_location(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_category_queues_list",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/category/queues/list/calendar`

**Get scheduled posts calendar view**

Returns scheduled posts from active queues within a date range. Supports filtering by categories and accounts.

Operation id: `v3:social-planner.post_social_media_posting_category_queues_list_calendar` · `Version: v3` · Scopes: `socialplanner/category.readonly`

*Request body*: [`CalendarListDTO`](#calendarlistdto)

*Response*: [`WrappedFetchCalendarListResponseDTO`](#wrappedfetchcalendarlistresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().get_scheduled_posts_calendar_view(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_category_queues_list_calendar",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /social-media-posting/category/queues/{postId}/active-post`

**Delete an active post and schedule the next one**

Deletes a post that is currently scheduled and automatically triggers the scheduling of the next available post in the queue.

Operation id: `v3:social-planner.delete_social_media_posting_category_queues_by_postId_active_post` · `Version: v3` · Scopes: `socialplanner/category.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `postId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Response*: [`WrappedDeleteActivePostResponseDTO`](#wrappeddeleteactivepostresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::social_planner::DeleteAnActivePostAndScheduleTheNextOneParams;

let params = DeleteAnActivePostAndScheduleTheNextOneParams::new("locationId");
let out = ghl.v3().social_planner().delete_an_active_post_and_schedule_the_next_one(&postId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.delete_social_media_posting_category_queues_by_postId_active_post",
    "path_params": {
      "postId": "<postId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/category/queues/{queueId}`

**Fetch a category queue by ID**

Retrieves the details of a single category queue by its unique ID. The response includes a count of posts within the queue that have errors.

Operation id: `v3:social-planner.get_social_media_posting_category_queues_by_queueId` · `Version: v3` · Scopes: `socialplanner/category.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `queueId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Response*: [`WrappedFetchQueueByIdResponseDTO`](#wrappedfetchqueuebyidresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::social_planner::FetchACategoryQueueByIdParams;

let params = FetchACategoryQueueByIdParams::new("locationId");
let out = ghl.v3().social_planner().fetch_a_category_queue_by_id(&queueId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.get_social_media_posting_category_queues_by_queueId",
    "path_params": {
      "queueId": "<queueId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /social-media-posting/category/queues/{queueId}`

**Update queue settings or status**

Updates queue status (active/paused/deleted), time slots, or skip dates.

Operation id: `v3:social-planner.put_social_media_posting_category_queues_by_queueId` · `Version: v3` · Scopes: `socialplanner/category.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `queueId` | string | **yes** | — |

*Request body*: [`UpdateCategoryQueueDTO`](#updatecategoryqueuedto)

*Response*: [`WrappedUpdateCategoryQueueResponseDTO`](#wrappedupdatecategoryqueueresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().update_queue_settings_or_status(&queueId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.put_social_media_posting_category_queues_by_queueId",
    "path_params": {
      "queueId": "<queueId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/category/queues/{queueId}/create/item`

**Create a new item in the queue**

Adds a new post item to a queue. Use sessionId for edit session or directToQueue for immediate addition.

Operation id: `v3:social-planner.post_social_media_posting_category_queues_by_queueId_create_item` · `Version: v3` · Scopes: `socialplanner/category.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `queueId` | string | **yes** | — |

*Request body*: [`CreateQueueItemDTO`](#createqueueitemdto)

*Response*: [`WrappedCreateQueueItemResponseDTO`](#wrappedcreatequeueitemresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().create_a_new_item_in_the_queue(&queueId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_category_queues_by_queueId_create_item",
    "path_params": {
      "queueId": "<queueId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/category/queues/{queueId}/edit/calendar`

**Fetch calendar view for an edit session**

Retrieves a calendar preview of scheduled posts based on draft items within an edit session. This shows how posts would be scheduled if changes were saved.

Operation id: `v3:social-planner.post_social_media_posting_category_queues_by_queueId_edit_calendar` · `Version: v3` · Scopes: `socialplanner/category.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `queueId` | string | **yes** | — |

*Request body*: [`EditSessionCalendarDTO`](#editsessioncalendardto)

*Response*: [`WrappedEditSessionCalendarResponseDTO`](#wrappededitsessioncalendarresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().fetch_calendar_view_for_an_edit_session(&queueId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_category_queues_by_queueId_edit_calendar",
    "path_params": {
      "queueId": "<queueId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/category/queues/{queueId}/edit/discard`

**Discard edit session changes**

Cancels the edit session and deletes all staged changes without affecting the live queue.

Operation id: `v3:social-planner.post_social_media_posting_category_queues_by_queueId_edit_discard` · `Version: v3` · Scopes: `socialplanner/category.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `queueId` | string | **yes** | — |

*Request body*: [`DiscardEditSessionDTO`](#discardeditsessiondto)

*Response*: [`WrappedDiscardEditSessionResponseDTO`](#wrappeddiscardeditsessionresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().discard_edit_session_changes(&queueId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_category_queues_by_queueId_edit_discard",
    "path_params": {
      "queueId": "<queueId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/category/queues/{queueId}/edit/save`

**Save edit session changes**

Applies all staged changes to the live queue and closes the edit session.

Operation id: `v3:social-planner.post_social_media_posting_category_queues_by_queueId_edit_save` · `Version: v3` · Scopes: `socialplanner/category.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `queueId` | string | **yes** | — |

*Request body*: [`SaveEditSessionDTO`](#saveeditsessiondto)

*Response*: [`WrappedSaveEditSessionResponseDTO`](#wrappedsaveeditsessionresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().save_edit_session_changes(&queueId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_category_queues_by_queueId_edit_save",
    "path_params": {
      "queueId": "<queueId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/category/queues/{queueId}/edit/start`

**Start or resume an edit session**

Creates a draft copy of queue items for editing. Changes are staged until saved or discarded.

Operation id: `v3:social-planner.post_social_media_posting_category_queues_by_queueId_edit_start` · `Version: v3` · Scopes: `socialplanner/category.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `queueId` | string | **yes** | — |

*Request body*: [`StartEditSessionDTO`](#starteditsessiondto)

*Response*: [`WrappedStartEditSessionResponseDTO`](#wrappedstarteditsessionresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().start_or_resume_an_edit_session(&queueId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_category_queues_by_queueId_edit_start",
    "path_params": {
      "queueId": "<queueId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/category/queues/{queueId}/items`

**Fetch items from a queue**

Returns paginated queue items. Pass sessionId to get draft items from an edit session instead of live items.

Operation id: `v3:social-planner.post_social_media_posting_category_queues_by_queueId_items` · `Version: v3` · Scopes: `socialplanner/category.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `queueId` | string | **yes** | — |

*Request body*: [`FetchQueueItemsDTO`](#fetchqueueitemsdto)

*Response*: [`WrappedFetchQueueItemsResponseDTO`](#wrappedfetchqueueitemsresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().fetch_items_from_a_queue(&queueId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_category_queues_by_queueId_items",
    "path_params": {
      "queueId": "<queueId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /social-media-posting/category/queues/{queueId}/items/{itemId}`

**Delete an item from a queue**

Deletes an item from a specific category queue.

Operation id: `v3:social-planner.delete_social_media_posting_category_queues_by_queueId_items_by_itemId` · `Version: v3` · Scopes: `socialplanner/category.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `queueId` | string | **yes** | — |
| `itemId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `sessionId` | string | no | Edit session ID |

*Response*: [`WrappedGeneralSuccessResponseDTO`](#wrappedgeneralsuccessresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::social_planner::DeleteAnItemFromAQueueParams;

let params = DeleteAnItemFromAQueueParams::new("locationId");
let out = ghl.v3().social_planner().delete_an_item_from_a_queue(&queueId, &itemId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.delete_social_media_posting_category_queues_by_queueId_items_by_itemId",
    "path_params": {
      "queueId": "<queueId>",
      "itemId": "<itemId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /social-media-posting/category/queues/{queueId}/items/{itemId}`

**Update an item in a queue**

Updates the content or variations of a specific item within a category queue.

Operation id: `v3:social-planner.put_social_media_posting_category_queues_by_queueId_items_by_itemId` · `Version: v3` · Scopes: `socialplanner/category.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `queueId` | string | **yes** | — |
| `itemId` | string | **yes** | — |

*Request body*: [`UpdateQueueItemDTO`](#updatequeueitemdto)

*Response*: [`WrappedUpdateQueueItemResponseDTO`](#wrappedupdatequeueitemresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().update_an_item_in_a_queue(&queueId, &itemId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.put_social_media_posting_category_queues_by_queueId_items_by_itemId",
    "path_params": {
      "queueId": "<queueId>",
      "itemId": "<itemId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/category/queues/{queueId}/items/{itemId}/clone`

**Clone a queue item**

Duplicates an existing queue item at a specified order position. Requires an active edit session.

Operation id: `v3:social-planner.post_social_media_posting_category_queues_by_queueId_items_by_itemId_clone` · `Version: v3` · Scopes: `socialplanner/category.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `queueId` | string | **yes** | — |
| `itemId` | string | **yes** | — |

*Request body*: [`CloneQueueItemDTO`](#clonequeueitemdto)

*Response*: [`WrappedCloneQueueItemResponseDTO`](#wrappedclonequeueitemresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().clone_a_queue_item(&queueId, &itemId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_category_queues_by_queueId_items_by_itemId_clone",
    "path_params": {
      "queueId": "<queueId>",
      "itemId": "<itemId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /social-media-posting/category/queues/{queueId}/items/{itemId}/reset`

**Reset an item in a queue**

Resets a specific queue item to its original state, discarding any modifications made.

Operation id: `v3:social-planner.put_social_media_posting_category_queues_by_queueId_items_by_itemId_reset` · `Version: v3` · Scopes: `socialplanner/category.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `queueId` | string | **yes** | — |
| `itemId` | string | **yes** | — |

*Request body*: [`ResetQueueItemDTO`](#resetqueueitemdto)

*Response*: [`WrappedResetQueueItemResponseDTO`](#wrappedresetqueueitemresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().reset_an_item_in_a_queue(&queueId, &itemId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.put_social_media_posting_category_queues_by_queueId_items_by_itemId_reset",
    "path_params": {
      "queueId": "<queueId>",
      "itemId": "<itemId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/category/queues/{queueId}/slots`

**Fetch slot information for queue items**

Returns paginated slot information (scheduledDateTime, isSkipped) for queue items. Pass sessionId to get slots for draft items, or omit for live items. Call this after mutations to refresh slot data.

Operation id: `v3:social-planner.post_social_media_posting_category_queues_by_queueId_slots` · `Version: v3` · Scopes: `socialplanner/category.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `queueId` | string | **yes** | — |

*Request body*: [`FetchSlotsDTO`](#fetchslotsdto)

*Response*: [`WrappedFetchSlotsResponseDTO`](#wrappedfetchslotsresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().fetch_slot_information_for_queue_items(&queueId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_category_queues_by_queueId_slots",
    "path_params": {
      "queueId": "<queueId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/comments/{platform}`

**Create a comment or reply**

Create a top-level comment on a post (`isParentThread: true`, `parentId` = postId) or a reply to an existing comment (`isParentThread: false`, `parentId` = commentId). Per-platform content max length: Facebook 8000, Instagram 2200, Linkedin 3000, Community 8000, Tiktok 150, Bluesky 300, Youtube 10000, Threads 500. **Optional-field platform support:** - `attachments` — supported on **Facebook only**. Ignored on Instagram, LinkedIn, TikTok, Bluesky, Community (Community processes the field but ext…

Operation id: `v3:social-planner.post_social_media_posting_comments_by_platform` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `platform` | enum: `facebook`, `instagram`, `linkedin`, `community`, `tiktok`, `bluesky`, `youtube`, `threads` | **yes** | Supported Comments Platforms |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Request body*: [`CommentsCreateBodyDTO`](#commentscreatebodydto)

*Response*: [`CommentsCreateResponseDTO`](#commentscreateresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::social_planner::CreateACommentOrReplyParams;

let params = CreateACommentOrReplyParams::new("locationId");
let out = ghl.v3().social_planner().create_a_comment_or_reply(&platform, &params, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_comments_by_platform",
    "path_params": {
      "platform": "<platform>"
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

#### `POST /social-media-posting/comments/{platform}/list`

**List comments for a post or thread**

Paginated list of comments scoped to a post (`parentId` = postId) or a comment thread (`parentId` = commentId). Use `skip`/`limit` for pagination, `sortBy` for ordering, `originIds` to filter by connected account, and `search` for keyword search.

Operation id: `v3:social-planner.post_social_media_posting_comments_by_platform_list` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `platform` | enum: `facebook`, `instagram`, `linkedin`, `community`, `tiktok`, `bluesky`, `youtube`, `threads` | **yes** | Supported Comments Platforms |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Request body*: [`CommentsGetListBodyDTO`](#commentsgetlistbodydto)

*Response*: [`CommentsGetListResponseDTO`](#commentsgetlistresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::social_planner::ListCommentsForAPostOrThreadParams;

let params = ListCommentsForAPostOrThreadParams::new("locationId");
let out = ghl.v3().social_planner().list_comments_for_a_post_or_thread(&platform, &params, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_comments_by_platform_list",
    "path_params": {
      "platform": "<platform>"
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

#### `DELETE /social-media-posting/comments/{platform}/{id}/like`

**Unlike a comment**

Remove a like from a comment by its **Highlevel** comment ID (the `_id` returned by the list-comments endpoint — not the native platform ID). Works for any comment level — top-level comments, replies, and replies-to-replies. **Supported platforms:** Facebook, LinkedIn, Community, TikTok, Bluesky. Instagram is not supported (passing `instagram` returns 400).

Operation id: `v3:social-planner.delete_social_media_posting_comments_by_platform_by_id_like` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `platform` | enum: `facebook`, `linkedin`, `community`, `tiktok`, `bluesky` | **yes** | Platform that supports liking / unliking comments (Instagram is not supported) |
| `id` | string | **yes** | Highlevel comment ID — the `_id` returned by the list-comments endpoint (`POST /comments/{platform}/list`). Not the native platform comment ID. Works for any co… |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Response*: [`DeleteLikeResponseDTO`](#deletelikeresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::social_planner::UnlikeACommentParams;

let params = UnlikeACommentParams::new("locationId");
let out = ghl.v3().social_planner().unlike_a_comment(&platform, &id, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.delete_social_media_posting_comments_by_platform_by_id_like",
    "path_params": {
      "platform": "<platform>",
      "id": "<id>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/comments/{platform}/{id}/like`

**Like a comment**

Like a comment by its **Highlevel** comment ID (the `_id` returned by the list-comments endpoint — not the native platform ID). Works for any comment level — top-level comments, replies, and replies-to-replies. **Supported platforms:** Facebook, LinkedIn, Community, TikTok, Bluesky. Instagram is not supported (passing `instagram` returns 400).

Operation id: `v3:social-planner.post_social_media_posting_comments_by_platform_by_id_like` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `platform` | enum: `facebook`, `linkedin`, `community`, `tiktok`, `bluesky` | **yes** | Platform that supports liking / unliking comments (Instagram is not supported) |
| `id` | string | **yes** | Highlevel comment ID — the `_id` returned by the list-comments endpoint (`POST /comments/{platform}/list`). Not the native platform comment ID. Works for any co… |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Response*: [`CommentsLikeResponseDTO`](#commentslikeresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::social_planner::LikeACommentParams;

let params = LikeACommentParams::new("locationId");
let out = ghl.v3().social_planner().like_a_comment(&platform, &id, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_comments_by_platform_by_id_like",
    "path_params": {
      "platform": "<platform>",
      "id": "<id>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/oauth/{locationId}/{platform}/accounts/{accountId}`

**Get Available Accounts (Step 2 of 3)**

## OAuth Connection Flow - Step 2: Get Available Accounts After completing OAuth authentication (Step 1), use this endpoint to retrieve the list of available pages, channels, or locations that can be connected. ### OAuth Flow Position 1. **Start OAuth** → User authenticates, returns `accountId` 2. **Get Accounts** (this endpoint) → Lists available pages/channels to connect 3. **Attach Account** → Connect the selected account ### What This Returns The response varies by platform: \| Platform \| R…

Operation id: `v3:social-planner.get_social_media_posting_oauth_by_locationId_by_platform_accounts_by_accountId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Account Location Id |
| `platform` | enum: `google`, `facebook`, `instagram`, `linkedin`, `tiktok`, `tiktok-business`, `youtube`, `pinterest`, `threads`, `bluesky` | **yes** | Social media platform |
| `accountId` | string | **yes** | The OAuth Account ID received from Step 1 (Start OAuth) via the window message event |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `search` | string | no | Search term to filter accounts/pages by name. Useful when the user has many pages to choose from. |

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::social_planner::GetAvailableAccountsStep2Of3Params;

let params = GetAvailableAccountsStep2Of3Params::new();
let out = ghl.v3().social_planner().get_available_accounts_step_2_of_3(&locationId, &platform, &accountId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.get_social_media_posting_oauth_by_locationId_by_platform_accounts_by_accountId",
    "path_params": {
      "locationId": "<locationId>",
      "platform": "<platform>",
      "accountId": "<accountId>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/oauth/{locationId}/{platform}/accounts/{accountId}`

**Connect Account (Step 3 of 3)**

## OAuth Connection Flow - Step 3: Connect the Account This is the final step in the OAuth flow. After retrieving available accounts (Step 2), use this endpoint to connect the selected account to your location. ### OAuth Flow Summary 1. **Start OAuth** → User authenticates with platform 2. **Get Accounts** → Retrieved available pages/channels 3. **Attach Account** (this endpoint) → Connect the selected account ### Request Body by Platform The request body structure varies depending on the platfo…

Operation id: `v3:social-planner.post_social_media_posting_oauth_by_locationId_by_platform_accounts_by_accountId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | The Location ID where you want to connect this social account |
| `platform` | enum: `google`, `facebook`, `instagram`, `linkedin`, `tiktok`, `youtube`, `pinterest`, `threads`, `bluesky` | **yes** | Social media platform (must match the platform used in Steps 1 and 2) |
| `accountId` | string | **yes** | The OAuth Account ID received from Step 1 (same as used in Step 2) |

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().connect_account_step_3_of_3(&locationId, &platform, &accountId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_oauth_by_locationId_by_platform_accounts_by_accountId",
    "path_params": {
      "locationId": "<locationId>",
      "platform": "<platform>",
      "accountId": "<accountId>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/oauth/{platform}/start`

**Start OAuth Flow (Step 1 of 3)**

## OAuth Connection Flow - Step 1: Initiate OAuth This is the first step in the 3-step OAuth flow to connect a social media account: 1. **Start OAuth** (this endpoint) → User authenticates with the platform 2. **Get Accounts** → Retrieve available pages/channels to connect 3. **Attach Account** → Connect the selected account to your location ### How to Use Open this API in a browser window (not via cURL) with the required query parameters. The user will be redirected to the platform's OAuth logi…

Operation id: `v3:social-planner.get_social_media_posting_oauth_by_platform_start` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `platform` | enum: `google`, `facebook`, `instagram`, `linkedin`, `tiktok`, `tiktok-business`, `youtube`, `pinterest`, `threads`, `bluesky` | **yes** | Social media platform to connect. Each platform has specific account types: - **google**: Google Business Profile locations - **facebook**: Facebook Pages - **i… |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `userId` | string | **yes** | User Id |
| `page` | string | no | Page |
| `reconnect` | string | no | Reconnect |

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::social_planner::StartOAuthFlowStep1Of3Params;

let params = StartOAuthFlowStep1Of3Params::new("locationId", "userId");
let out = ghl.v3().social_planner().start_o_auth_flow_step_1_of_3(&platform, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.get_social_media_posting_oauth_by_platform_start",
    "path_params": {
      "platform": "<platform>"
    },
    "query": {
      "locationId": "<locationId>",
      "userId": "<userId>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/statistics`

**Get Social Media Statistics**

Retrieve analytics data for multiple social media accounts. Supports custom date ranges for both the current period and a comparison period. If no date ranges are provided, defaults to the last 7 days (excluding today) with comparison to the previous 7 days.

Operation id: `v3:social-planner.post_social_media_posting_statistics` · `Version: v3` · Scopes: `socialplanner/statistics.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Request body fields*: `profileIds`**\***, `platforms`, `currentRange`, `prevRange`  (**\*** = required)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::social_planner::GetSocialMediaStatisticsParams;

let params = GetSocialMediaStatisticsParams::new("locationId");
let out = ghl.v3().social_planner().get_social_media_statistics(&params, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_statistics",
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

Operation id: `v3:social-planner.get_social_media_posting_by_locationId_accounts` · `Version: v3` · Scopes: `socialplanner/account.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Response*: [`AccountsListResponseDTO`](#accountslistresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().get_accounts(&locationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.get_social_media_posting_by_locationId_accounts",
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

Operation id: `v3:social-planner.delete_social_media_posting_by_locationId_accounts_by_id` · `Version: v3`

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
use ghl_sdk::services::v3::social_planner::DeleteAccountParams;

let params = DeleteAccountParams::new();
let out = ghl.v3().social_planner().delete_account(&locationId, &id, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.delete_social_media_posting_by_locationId_accounts_by_id",
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

Retrieve all categories for a specific location with optional search and pagination

Operation id: `v3:social-planner.get_social_media_posting_by_locationId_categories` · `Version: v3` · Scopes: `socialplanner/category.readonly`

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
use ghl_sdk::services::v3::social_planner::GetCategoriesByLocationIdParams;

let params = GetCategoriesByLocationIdParams::new();
let out = ghl.v3().social_planner().get_categories_by_location_id(&locationId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.get_social_media_posting_by_locationId_categories",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /social-media-posting/{locationId}/categories/{id}`

**Get categories by id**

Retrieve a specific category by its ID

Operation id: `v3:social-planner.get_social_media_posting_by_locationId_categories_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `id` | string | **yes** | Category Id |
| `locationId` | string | **yes** | Location Id |

*Response*: [`GetByIdResponseDTO`](#getbyidresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().get_categories_by_id(&id, &locationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.get_social_media_posting_by_locationId_categories_by_id",
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

Get the status of all CSV imports for a location

Operation id: `v3:social-planner.get_social_media_posting_by_locationId_csv` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `skip` | string | no | Number of records to skip |
| `limit` | string | no | Maximum number of records to return |
| `includeUsers` | string | no | Include user data in response |
| `isFromTemplate` | string | no | Filter CSVs imported from template library |
| `userId` | string | **yes** | User ID |

*Response*: [`GetUploadStatusResponseDTO`](#getuploadstatusresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::social_planner::GetUploadStatusParams;

let params = GetUploadStatusParams::new("userId");
let out = ghl.v3().social_planner().get_upload_status(&locationId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.get_social_media_posting_by_locationId_csv",
    "path_params": {
      "locationId": "<locationId>"
    },
    "query": {
      "userId": "<userId>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/{locationId}/csv`

**Upload CSV**

Upload a CSV file containing social media posts for bulk scheduling

Operation id: `v3:social-planner.post_social_media_posting_by_locationId_csv` · `Version: v3` · Scopes: `socialplanner/csv.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Response*: [`UploadFileResponseDTO`](#uploadfileresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().upload_csv(&locationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_by_locationId_csv",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `DELETE /social-media-posting/{locationId}/csv/{csvId}/post/{postId}`

**Delete CSV Post**

Delete a specific post from a CSV import

Operation id: `v3:social-planner.delete_social_media_posting_by_locationId_csv_by_csvId_post_by_postId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `postId` | string | **yes** | CSV Post Id |
| `csvId` | string | **yes** | CSV Id |

*Response*: [`DeletePostResponseDTO`](#deletepostresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().delete_csv_post(&locationId, &postId, &csvId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.delete_social_media_posting_by_locationId_csv_by_csvId_post_by_postId",
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

Delete a CSV import and all its associated posts

Operation id: `v3:social-planner.delete_social_media_posting_by_locationId_csv_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | CSV Id |

*Response*: [`DeleteCsvResponseDTO`](#deletecsvresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().delete_csv(&locationId, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.delete_social_media_posting_by_locationId_csv_by_id",
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

Get details of a specific CSV import including its posts

Operation id: `v3:social-planner.get_social_media_posting_by_locationId_csv_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | CSV Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `skip` | string | no | Number of records to skip |
| `limit` | string | no | Maximum number of records to return |

*Response*: [`GetCsvPostResponseDTO`](#getcsvpostresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::social_planner::GetCsvPostParams;

let params = GetCsvPostParams::new();
let out = ghl.v3().social_planner().get_csv_post(&locationId, &id, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.get_social_media_posting_by_locationId_csv_by_id",
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

Finalize a CSV import and schedule all posts for publishing

Operation id: `v3:social-planner.patch_social_media_posting_by_locationId_csv_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | CSV Id |

*Request body*: [`CSVDefaultDTO`](#csvdefaultdto)

*Response*: [`CsvPostStatusResponseDTO`](#csvpoststatusresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().start_csv_finalize(&locationId, &id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.patch_social_media_posting_by_locationId_csv_by_id",
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

Operation id: `v3:social-planner.post_social_media_posting_by_locationId_posts` · `Version: v3` · Scopes: `socialplanner/post.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`CreatePostDTO`](#createpostdto)

*Response*: [`CreatePostSuccessfulResponseDTO`](#createpostsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().create_post(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_by_locationId_posts",
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

Deletes multiple posts based on the provided list of post IDs. This operation is useful for clearing up large numbers of posts efficiently. Note: 1.The maximum number of posts that can be deleted in a single request is '50'. 2.However, It will only get deleted in CRM database but still it is recommended to be cautious of this operation.

Operation id: `v3:social-planner.post_social_media_posting_by_locationId_posts_bulk_delete` · `Version: v3`

*Request body*: [`DeletePostsDto`](#deletepostsdto)

*Response*: [`BulkDeleteResponseDto`](#bulkdeleteresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().bulk_delete_social_planner_posts(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_by_locationId_posts_bulk_delete",
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

Operation id: `v3:social-planner.post_social_media_posting_by_locationId_posts_list` · `Version: v3` · Scopes: `socialplanner/post.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`SearchPostDTO`](#searchpostdto)

*Response*: [`PostSuccessfulResponseDTO`](#postsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().get_posts(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_by_locationId_posts_list",
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

Operation id: `v3:social-planner.delete_social_media_posting_by_locationId_posts_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Post Id |

*Response*: [`DeletePostSuccessfulResponseDTO`](#deletepostsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().delete_post(&locationId, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.delete_social_media_posting_by_locationId_posts_by_id",
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

Operation id: `v3:social-planner.get_social_media_posting_by_locationId_posts_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Post Id |

*Response*: [`GetPostSuccessfulResponseDTO`](#getpostsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().get_post(&locationId, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.get_social_media_posting_by_locationId_posts_by_id",
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

Operation id: `v3:social-planner.put_social_media_posting_by_locationId_posts_by_id` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `id` | string | **yes** | Post Id |

*Request body*: [`CreatePostDTO`](#createpostdto)

*Response*: [`UpdatePostSuccessfulResponseDTO`](#updatepostsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().edit_post(&locationId, &id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.put_social_media_posting_by_locationId_posts_by_id",
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

Set social media accounts for a CSV import to publish posts to

Operation id: `v3:social-planner.post_social_media_posting_by_locationId_set_accounts` · `Version: v3` · Scopes: `socialplanner/csv.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`SetAccountsDTO`](#setaccountsdto)

*Response*: [`SetAccountsResponseDTO`](#setaccountsresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().set_accounts(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_by_locationId_set_accounts",
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

Retrieve all tags for a specific location with optional search and pagination

Operation id: `v3:social-planner.get_social_media_posting_by_locationId_tags` · `Version: v3`

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
use ghl_sdk::services::v3::social_planner::GetTagsByLocationIdParams;

let params = GetTagsByLocationIdParams::new();
let out = ghl.v3().social_planner().get_tags_by_location_id(&locationId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.get_social_media_posting_by_locationId_tags",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /social-media-posting/{locationId}/tags/details`

**Get tags by ids**

Retrieve specific tags by their IDs

Operation id: `v3:social-planner.post_social_media_posting_by_locationId_tags_details` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Request body*: [`UpdateTagDTO`](#updatetagdto)

*Response*: [`GetTagsByIdResponseDTO`](#gettagsbyidresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().social_planner().get_tags_by_ids(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:social-planner.post_social_media_posting_by_locationId_tags_details",
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

## Data models — API v3

In Rust: `ghl_models::v3::social_planner::*` (enable the `social-planner` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/social_planner/).

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
| `accounts` | Vec<GetAccountSchema> | no | Array of connected social media accounts |
| `groups` | Vec<GetGroupSchema> | no | Array of account groups |

### `AttachFBAccountDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `page` | **yes** | Type of Facebook account (must be page) |
| `originId` | String | **yes** | Original Facebook platform identifier |
| `name` | String | **yes** | Name of the Facebook page or account |
| `avatar` | String | **yes** | Avatar or profile picture URL |

### `AttachGMBLocationAccountDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Account name identifier |
| `accountName` | String | **yes** | Display name of the account |
| `type` | String | **yes** | Type of the account |
| `verificationState` | String | **yes** | State of account verification |
| `vettedState` | String | **yes** | Vetting state of the account |

### `AttachGMBLocationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `location` | [`AttachGMBLocationLocationDTO`](#attachgmblocationlocationdto) | **yes** | — |
| `account` | [`AttachGMBLocationAccountDTO`](#attachgmblocationaccountdto) | **yes** | — |

### `AttachGMBLocationLocationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Location name |
| `storeCode` | String | no | Store code |
| `title` | String | **yes** | Location title |
| `storefrontAddress` | JSON | no | Storefront address details |
| `metadata` | JSON | no | Additional metadata |
| `maxLocation` | bool | no | Whether this is a max location |
| `isVerified` | bool | no | Whether the location is verified |
| `isConnected` | bool | no | Whether the location is connected |

### `AttachIGAccountDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `originId` | String | no | Original platform-specific account identifier |
| `name` | String | no | Display name of the account |
| `avatar` | String | no | Avatar or profile picture URL |
| `pageId` | String | **yes** | Facebook page ID associated with the Instagram account |

### `AttachLinkedinAccountDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `page`, `group`, `profile`, `location`, `business` | **yes** | Type of LinkedIn account (must be one of: page, profile) |
| `originId` | String | **yes** | Original LinkedIn platform identifier |
| `name` | String | **yes** | Name of the LinkedIn page or profile |
| `avatar` | String | **yes** | Avatar or profile picture URL |
| `urn` | String | **yes** | LinkedIn URN (Uniform Resource Name) identifier |

### `AttachPinterestAccountDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `originId` | String | **yes** | Origin ID |
| `name` | String | no | Name |
| `avatar` | String | no | Avatar URL |
| `verified` | bool | no | Verification status |
| `username` | String | no | Username |
| `websiteUrl` | String | no | Website URL |
| `companyId` | String | no | Company ID |
| `type` | String — `profile` | no | Account type must be one of the following values: profile |
| `originAccountType` | String | no | Origin account type |

### `AttachThreadsAccountDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `profile` | **yes** | Account type |
| `originId` | String | **yes** | Origin ID |
| `name` | String | **yes** | Account name |
| `avatar` | String | **yes** | Avatar URL |

### `AttachTiktokAccountDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `page`, `group`, `profile`, `location`, `business` | **yes** | Type of TikTok account |
| `originId` | String | **yes** | Original platform-specific account identifier |
| `name` | String | **yes** | Display name of the account |
| `avatar` | String | **yes** | Avatar or profile picture URL |
| `verified` | bool | no | Indicates if the TikTok account is verified |
| `username` | String | no | Username or handle of the TikTok account |

### `AttachTwitterAccountDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `originId` | String | no | Original Twitter platform identifier |
| `name` | String | no | Name of the Twitter account |
| `username` | String | no | Username or handle of the Twitter account |
| `avatar` | String | no | Avatar or profile picture URL |
| `protected` | bool | no | Indicates if the Twitter account is protected (private) |
| `verified` | bool | no | Indicates if the Twitter account is verified |
| `companyId` | String | no | Company ID |

### `AttachYoutubeAccountResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `profile` | **yes** | Account type |
| `originId` | String | **yes** | Origin ID |
| `name` | String | **yes** | Name |
| `avatar` | String | **yes** | Avatar URL |
| `verified` | bool | no | Verification status |
| `username` | String | no | Username |

### `AttachmentDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | String | **yes** | URL of the attachment |
| `type` | String — `image` | **yes** | Type of the attachment |

### `AvailableCategoryDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `deleted` | bool | no | Indicates if deleted |
| `_id` | String | no | Category ID |
| `name` | String | no | Category name |
| `locationId` | String | no | Location ID |
| `primaryColor` | String | no | Primary color (hex) |
| `secondaryColor` | String | no | Secondary color (hex) |
| `createdBy` | String | no | Creator user ID |
| `createdAt` | String | no | Creation timestamp |
| `updatedAt` | String | no | Last update timestamp |
| `publishedPostsCount` | f64 | no | Published posts count |
| `status` | String — `available`, `in_queue`, `draft` | no | Status: available (no queue), in_queue (active/paused), or draft |
| `queueDetails` | [`AvailableCategoryQueueDetailsDTO`](#availablecategoryqueuedetailsdto) | no | Queue details (present when in_queue or draft) |

### `AvailableCategoryQueueDetailsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `queueId` | String | no | Queue ID |
| `prioritizeNewContent` | bool | no | Prioritize new content over older content |
| `enableFuturePosts` | bool | no | Enable posting future content |

### `BaseOAuthAccountSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Account Name |
| `originId` | String | **yes** | Origin ID |
| `avatar` | String | no | Account Avatar URL |

### `BlueskyPostSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `shortenedLinks` | Vec<String> | no | Shortened links for the post (auto-generated). |
| `replyTo` | String | no | Bluesky AT Protocol URI of a post to reply to. **Format:** `at://did:plc:{user-id}/app.bsky.feed.post/{post-id}` **Use Case:** Create a reply thread to an existing Bluesky post. |
| `quotePost` | String | no | Bluesky AT Protocol URI of a post to quote. **Format:** `at://did:plc:{user-id}/app.bsky.feed.post/{post-id}` **Use Case:** Quote-post another user's post with your commentary. |
| `language` | String | no | ISO 639-1 language code for the post content. **Examples:** `en` (English), `es` (Spanish), `fr` (French), `de` (German) |
| `externalLink` | String | no | External URL to embed as a link card in the post. |
| `externalLinkTitle` | String | no | Title for the external link card. Displayed prominently in the embed. |
| `externalLinkDescription` | String | no | Description for the external link card. Brief summary displayed below the title. |

### `BulkDeletePostSuccessfulResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `deletedCount` | f64 | no | Number of posts successfully deleted |

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
| `userId` | String | **yes** | User ID |

### `CSVErrorResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `code` | String — `CSV_EMPTY_FILE`, `CSV_INVALID_FORMAT`, `CSV_HEADERS_NOT_MATCHING`, `CSV_MISSING_HEADERS`, `CSV_MISSING_REQUIRED_ROWS`, `CSV_UNSUPPORTED_EXTENSION`, `CSV_NO_VALID_POST`, `CSV_PROCESSING_ERROR` | **yes** | Error code for CSV processing errors |
| `message` | String | **yes** | Error message describing the CSV validation error |
| `fileType` | String — `CSV`, `XLSX` | no | File type detected |
| `csvFileType` | String — `basic`, `advance` | no | CSV file type |
| `missingHeaders` | String | no | Comma-separated list of missing headers from the file |

### `CSVFileRequiredBadRequestDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | f64 | **yes** | HTTP Status |
| `options` | JSON | no | Options |
| `message` | String | **yes** | Error message |
| `name` | String | **yes** | Exception name |
| `error` | String | **yes** | Error type |
| `statusCode` | f64 | **yes** | HTTP Status Code |
| `traceId` | String | no | Trace ID for debugging |

### `CSVImportSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | CSV Id |
| `locationId` | String | no | Location Id |
| `fileName` | String | no | File Name |
| `accountIds` | Vec<String> | no | Account Ids |
| `file` | String | no | File path |
| `status` | String — `pending`, `in_progress`, `completed`, `failed`, `in_review`, `importing`, `deleted` | no | CSV import status |
| `count` | f64 | no | Posts count |
| `createdBy` | String | no | Created By Id |
| `traceId` | String | no | Trace Id |
| `originId` | String | no | Origin Id |
| `approver` | String | no | Approver Id |
| `createdAt` | String | no | Date Created |
| `csvFileType` | String — `basic`, `advance` | no | CSV file type |
| `mediaOptimization` | bool | no | Media optimization flag |
| `applyWatermark` | bool | no | Apply watermark flag |
| `channel` | String | no | Channel |
| `updatedAt` | String | no | Date Updated |

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
| `instagramError` | String | no | Instagram media error. It can be one of the following errors: imageSize, imageType, videoType, videoDuration, videoSize, videoAspectRatio, videoWidthHeight, audioCodec, audioCodecChannels, videoCodec,… |
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
| `scheduleDate` | String | no | Schedule date for the post in ISO format |
| `summary` | String | no | Post content/summary |
| `followUpComment` | String | no | Follow-up comment to be posted immediately after the main post is published. **Supported Platforms:** Facebook, Instagram, LinkedIn, YouTube **NOT Supported:** TikTok, Google My Business (GMB), Pinter… |
| `type` | String | no | Post type - post, story, or reel |
| `tiktokPostDetails` | [`TiktokPostSchema`](#tiktokpostschema) | no | Tiktok Post Details |
| `gmbPostDetails` | [`GMBPostSchema`](#gmbpostschema) | no | GMB Post Details |
| `errorMessage` | String | no | Error Description |
| `csvFileType` | String — `basic`, `advance` | no | CSV file type |
| `mediaOptimization` | bool | no | Media optimization flag |
| `applyWatermark` | bool | no | Apply watermark flag |
| `status` | String — `pending`, `accepted`, `rejected`, `deleted` | no | Post status |
| `updatedAt` | String | no | Date Updated |

### `CSVResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `csv` | [`CsvResponse`](#csvresponse) | no | CSV Data |

### `CalendarListDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `startDate` | String | **yes** | Start Date in ISO format |
| `endDate` | String | **yes** | End Date in ISO format |
| `categoryIds` | Vec<String> | no | Category Id |
| `accountIds` | Vec<String> | no | Filter by Account IDs. If not provided or empty, returns all posts. |

### `CategoryInfoDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | Category ID |
| `name` | String | no | Name of the category |
| `primaryColor` | String | no | Primary color of the category |
| `secondaryColor` | String | no | Secondary color of the category |
| `deleted` | bool | no | Indicates if the category is deleted |
| `locationId` | String | no | Location ID |
| `createdBy` | String | no | ID of the user who created the category |
| `createdAt` | String | no | Creation timestamp |
| `updatedAt` | String | no | Last update timestamp |

### `CategoryQueueDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | Queue ID |
| `locationId` | String | no | Location ID |
| `categoryId` | String | no | Category ID |
| `timeSlots` | Vec<TimeSlotDTO> | no | Time slots for scheduling posts |
| `enableFuturePosts` | bool | no | Enable posting future content |
| `prioritizeNewContent` | bool | no | Prioritize new content over older content |
| `currentOrder` | f64 | no | Current order number in the queue |
| `status` | String — `active`, `paused`, `draft` | no | Status of the queue. Possible values: active, paused, draft. |
| `startDate` | String | no | Start date of the queue |
| `skipDateTime` | Vec<String> | no | Dates/times to skip posting |
| `currentPostId` | String | no | ID of the currently scheduled post |
| `totalPosts` | f64 | no | Total number of posts in the queue |
| `lastScheduledTime` | String | no | Timestamp of the last scheduled post |
| `createdBy` | String | no | ID of the user who created the queue |
| `createdAt` | String | no | Creation timestamp |
| `updatedAt` | String | no | Last update timestamp |

### `CategoryQueueWithCategoryDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | Queue ID |
| `locationId` | String | no | Location ID |
| `categoryId` | String | no | Category ID |
| `timeSlots` | Vec<TimeSlotDTO> | no | Time slots for scheduling posts |
| `enableFuturePosts` | bool | no | Enable posting future content |
| `prioritizeNewContent` | bool | no | Prioritize new content over older content |
| `currentOrder` | f64 | no | Current order number in the queue |
| `status` | String — `active`, `paused`, `draft` | no | Status of the queue. Possible values: active, paused, draft. |
| `startDate` | String | no | Start date of the queue |
| `skipDateTime` | Vec<String> | no | Dates/times to skip posting |
| `currentPostId` | String | no | ID of the currently scheduled post |
| `totalPosts` | f64 | no | Total number of posts in the queue |
| `lastScheduledTime` | String | no | Timestamp of the last scheduled post |
| `createdBy` | String | no | ID of the user who created the queue |
| `createdAt` | String | no | Creation timestamp |
| `updatedAt` | String | no | Last update timestamp |
| `category` | [`CategoryInfoDTO`](#categoryinfodto) | no | The category associated with the queue. |

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

### `CloneQueueItemDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `sessionId` | String | **yes** | Edit session ID |
| `order` | f64 | **yes** | Order for the cloned item (typically between source and next item) |

### `CloneQueueItemResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | no | A message indicating the result of the operation. |
| `queueItem` | [`CreatedQueueItemWithVariationsDTO`](#createdqueueitemwithvariationsdto) | no | The cloned queue item |

### `CommentAttachmentDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String | no | Attachment MIME type or platform-specific type |
| `url` | String | no | Attachment URL |
| `thumbnail` | String | no | Thumbnail URL |
| `videoUrl` | String | no | Video URL (when attachment is a video) |

### `CommentAuthorDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Platform author ID |
| `name` | String | no | Author display name |
| `profilePic` | String | no | Author profile picture URL |

### `CommentItemDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Highlevel comment ID |
| `platform` | String | **yes** | Platform the comment was posted on |
| `platformCommentId` | String | no | Native platform comment ID |
| `platformParentId` | String | no | Native platform parent ID (the post or comment this is a reply to) |
| `platformPostId` | String | no | Native platform post ID |
| `postId` | String | **yes** | Highlevel post ID |
| `originId` | String | **yes** | Connected account / page ID on the native platform |
| `isParentThread` | bool | no | True if this comment is a top-level comment on the post; false if it is a reply to another comment |
| `isPost` | bool | **yes** | True if this record represents the root post (not a comment) |
| `content` | String | no | Comment content. May be empty or missing for attachment-only comments. |
| `attachments` | Vec<CommentAttachmentDTO> | no | Attachments on the comment |
| `author` | [`CommentAuthorDTO`](#commentauthordto) | no | Author of the comment. May be partial or missing for some sync paths. |
| `level` | f64 | no | Comment depth (0 = post, 1 = comment, 2 = reply) |
| `likeCount` | f64 | **yes** | Number of likes on the comment |
| `reactionCount` | f64 | **yes** | Number of reactions on the comment |
| `replyCount` | f64 | **yes** | Number of replies to the comment |
| `shareCount` | f64 | **yes** | Number of shares of the comment |
| `repostCount` | f64 | **yes** | Number of reposts of the comment (platform-specific) |
| `quoteCount` | f64 | **yes** | Number of quote posts (platform-specific) |
| `previewLink` | String | no | Direct link to view the comment on the native platform |
| `isRead` | bool | **yes** | Whether the comment has been read |
| `isDeleted` | bool | **yes** | Whether the comment was deleted |
| `isEdited` | bool | **yes** | Whether the comment was edited |
| `publishedAt` | String | no | Time the comment was published on the native platform. May be missing for legacy or webhook-synced records. |
| `createdAt` | String | no | Time the comment record was created in Highlevel |
| `updatedAt` | String | no | Time the comment record was last updated in Highlevel |

### `CommentsCreateBodyDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `parentId` | String | **yes** | For top-level comments (`isParentThread: true`): pass the post ID returned by the posts API. For replies (`isParentThread: false`): pass the parent comment ID returned by the list-comments API. In bot… |
| `isParentThread` | bool | **yes** | Set `true` to create a top-level comment on a post (parentId = post ID). Set `false` to create a reply to an existing comment (parentId = comment ID). |
| `content` | String | **yes** | Content of the comment. Per-platform max length: Facebook 8000, Instagram 2200, Linkedin 3000, Community 8000, Tiktok 150, Bluesky 300, Youtube 10000, Threads 500. |
| `attachments` | Vec<AttachmentDTO> | no | Attachments for the comment (max 1 image). **Supported on:** Facebook only. **Not supported on:** Instagram, LinkedIn, TikTok, Bluesky, Community — the field is accepted by the API but the attachment … |
| `mentions` | Vec<MentionsDTO> | no | Mentions for the comment. **Supported on:** Facebook, LinkedIn, Community. **Ignored on:** Instagram, TikTok, Bluesky — the field is accepted but mentions are not rendered on these platforms. |
| `notifyAllGroupMembers` | bool | no | When `true`, all members of the Community group receive a push/in-app notification about this comment — equivalent to an `@everyone` broadcast. **Supported on:** Community only. Ignored on all other p… |

### `CommentsCreateResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`CommentItemDTO`](#commentitemdto) | **yes** | The created comment |

### `CommentsGetListBodyDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `fromDate` | String | no | Start of the published-date window (ISO 8601). If provided, `toDate` is also required, and `fromDate` must be ≤ `toDate`. Omit both to disable date filtering. |
| `toDate` | String | no | End of the published-date window (ISO 8601). If provided, `fromDate` is also required. |
| `originIds` | Vec<String> | **yes** | Origin IDs of connected accounts to filter by |
| `sortBy` | String — `top`, `latest` | no | Sort by top comments or latest comments |
| `search` | String | no | Search |
| `skip` | f64 | no | Pagination offset — number of comments to skip (zero-based). Must be ≥ 0. |
| `limit` | f64 | no | Pagination page size — number of comments to return. Must be between 1 and 100. |
| `parentId` | String | no | Parent ID — pass the Highlevel post ID (for replies under a specific post) or the Highlevel comment ID (for replies under a specific comment). Omit to list all top-level comments for the location filt… |

### `CommentsGetListResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`CommentsGetListResultsDTO`](#commentsgetlistresultsdto) | **yes** | Comments and pagination metadata |

### `CommentsGetListResultsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `comments` | Vec<CommentItemDTO> | **yes** | List of comments |
| `meta` | [`CommentsListMetaDTO`](#commentslistmetadto) | **yes** | Pagination metadata |

### `CommentsLikeResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |

### `CommentsListMetaDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `total` | f64 | **yes** | Total comments matching the query |
| `totalUnread` | f64 | no | Total unread comments matching the query |
| `skip` | f64 | **yes** | Pagination skip |
| `limit` | f64 | **yes** | Pagination limit |
| `hasMore` | bool | **yes** | True if more pages exist beyond this batch |

### `CreateCategoryQueueDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `categoryId` | String | **yes** | Category ID |
| `timeSlots` | Vec<TimeSlotDTO> | **yes** | — |
| `enableFuturePosts` | bool | no | Enable Future Posts. Defaults to false. |
| `prioritizeNewContent` | bool | no | Prioritize New Content. Defaults to false. |
| `userId` | String | **yes** | User id |

### `CreateCategoryQueueResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | no | A message indicating the result of the operation. |
| `queue` | [`CreatedCategoryQueueDTO`](#createdcategoryqueuedto) | no | The newly created queue. |

### `CreatePostDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `accountIds` | Vec<String> | **yes** | Account IDs for the post. Each account ID identifies a connected social media account. **Get IDs from:** [Get Accounts API](./get-account) — use the `id` field from each account. **Validations:** - Re… |
| `summary` | String | no | Post content/caption text. Character limits vary by platform. **Custom Values & Hashtags:** - You can include custom values/variables in the content (e.g., `{{contact.name}}`) - Hashtags: Use `#hashta… |
| `media` | Vec<PostMediaSchema> | no | Post Media Data The limitations of media as per the platforms is provided through the reference link in API description |
| `status` | String — `draft`, `scheduled`, `in_review`, `published`, `in_progress`, `pending`, `failed`, `notification_sent`, `deleted` | no | Post status indicating the current state of the post. **Available Status Values:** - `draft` - Post saved as draft, not yet ready for publishing - `scheduled` - Post scheduled for future publishing (r… |
| `scheduleDate` | String | no | Schedule Date |
| `selectedBestTime` | String | no | Selected Best Time slot for scheduling |
| `createdBy` | String | no | User ID of the creator who is creating/managing the post. Must be a valid MongoDB ObjectId. **Get User IDs from:** [Get User API](../users/get-user) — use the `id` field from the user object. **Valida… |
| `followUpComment` | String | no | Follow-up comment to be posted immediately after the main post is published. **Supported Platforms:** Facebook, Instagram, LinkedIn, YouTube **NOT Supported:** TikTok, Google My Business (GMB), Pinter… |
| `ogTagsDetails` | [`OgTagsSchema`](#ogtagsschema) | no | Og Tags Meta Data |
| `type` | String — `post`, `story`, `reel` | **yes** | Type of post to create. Determines the format and platform requirements. **Available Types:** - `post` - Standard feed post (all platforms) - `story` - Temporary 24-hour story (Instagram, Facebook) - … |
| `postApprovalDetails` | [`PostApprovalSchema`](#postapprovalschema) | no | Post Approval Details |
| `scheduleTimeUpdated` | bool | no | Flag indicating if the schedule datetime was manually updated. Used for tracking rescheduled posts. |
| `tags` | Vec<String> | no | Array of Tag IDs to associate with the post for organization and filtering. **Get Tag IDs from:** [Get Tags API](./social-planner/get-tags-location-id) — use the `_id` field from each tag. **Validatio… |
| `categoryId` | String | no | Category ID to organize the post. Categories help group related posts. **Get Category IDs from:** [Get Categories API](./social-planner/get-categories-location-id) — use the `_id` field. **Validation:… |
| `applyWatermark` | bool | no | Apply watermark to media in this post. **Note:** Watermarks are applied to images only. Videos are not watermarked. |
| `tiktokPostDetails` | [`TiktokPostSchema`](#tiktokpostschema) | no | Tiktok Post Details |
| `gmbPostDetails` | [`GMBPostSchema`](#gmbpostschema) | no | GMB Post Details |
| `userId` | String | **yes** | User ID of the user creating/managing the post. Required for OAuth channel posts (non-draft). |
| `linkedinPostDetails` | [`LinkedinPostSchema`](#linkedinpostschema) | no | LinkedIn-specific post configuration. **Key Fields:** - `postAsPdf`: Set to `true` to post images as a PDF carousel document - `pdfTitle`: Title for the PDF document (max 100 characters) **Limits:** -… |
| `pinterestPostDetails` | [`PinterestPostSchema`](#pinterestpostschema) | no | Pinterest-specific post configuration. Required when posting to Pinterest accounts. **Required Fields:** - `boardIds`: Object mapping account OAuth IDs to Pinterest board IDs **Optional Fields:** - `t… |
| `facebookPostDetails` | [`FacebookPostSchema`](#facebookpostschema) | no | Facebook-specific post configuration. **Key Fields:** - `type`: Post type (`post`, `story`, `reel`) **Restrictions:** - Facebook Groups do NOT support Reels - Reels require exactly 1 video - Stories d… |
| `instagramPostDetails` | [`InstagramPostSchema`](#instagrampostschema) | no | Instagram-specific post configuration. **Key Fields:** - `type`: Post type (`post`, `story`, `reel`) - `collaborators`: Map of account IDs to Instagram usernames for collaboration invites (max 3 per a… |
| `youtubePostDetails` | [`YoutubePostSchema`](#youtubepostschema) | no | YouTube-specific post configuration. **Key Fields:** - `title`: Video title (max 100 characters) - `type`: Video type (`video` for regular videos, `short` for YouTube Shorts) - `privacyLevel`: Video v… |

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

### `CreateQueueItemDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `sessionId` | String | no | Edit session ID |
| `modifiedPostPayload` | [`QueueModifiedPostDTO`](#queuemodifiedpostdto) | no | New post details |
| `order` | JSON | no | Order for the new item in the queue (cyclic-aware). Accepts: - A number: explicit order value calculated by FE as midpoint between adjacent items - "top": place at cyclic top (first to be scheduled ne… |
| `variations` | Vec<VariationInputDTO> | no | Variations |
| `primaryImage` | String | no | Primary media URL (image) for the post. Falls back to modifiedPostPayload.primaryImage if not set. |
| `directToQueue` | bool | no | When true, creates the queue item directly without requiring an edit session, even for active/paused queues. The order field is ignored and the item position is determined by the queue's prioritizeNew… |

### `CreateQueueItemResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | no | A message indicating the result of the operation. |
| `queueItem` | [`CreatedQueueItemWithVariationsDTO`](#createdqueueitemwithvariationsdto) | no | The newly created queue item |

### `CreatedCategoryQueueDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | Queue ID |
| `locationId` | String | no | Location ID |
| `categoryId` | String | no | Category ID |
| `timeSlots` | Vec<CreatedTimeSlotDTO> | no | Time slots for scheduling posts |
| `enableFuturePosts` | bool | no | Enable posting future content |
| `prioritizeNewContent` | bool | no | Prioritize new content over older content |
| `status` | String — `active`, `paused`, `draft` | no | Status of the queue. Always "draft" for a new queue. |
| `startDate` | String | no | Start date of the queue |
| `skipDateTime` | Vec<String> | no | Dates/times to skip posting. Always empty for a new queue. |
| `totalPosts` | f64 | no | Total number of posts in the queue. Always 0 for a new queue. |
| `lastScheduledTime` | String | no | Timestamp of the last scheduled post. Always null for a new queue. |
| `createdBy` | String | no | ID of the user who created the queue |
| `createdAt` | String | no | Creation timestamp |
| `updatedAt` | String | no | Last update timestamp |

### `CreatedQueueItemWithVariationsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | The unique identifier of the queue item. |
| `order` | f64 | no | The order of the item in the queue. |
| `variations` | Vec<VariationDTO> | no | A list of content variations for the post. |
| `primaryImage` | String | no | The primary image URL for the post. |
| `lastScheduledTime` | String | no | Timestamp of the last scheduled post. Always null for a new queue. |
| `queueId` | String | no | The ID of the queue this post belongs to |
| `postId` | String | no | The ID of the original post, if any. |
| `modifiedPostPayload` | [`GetModifiedPayloadFormattedSchema`](#getmodifiedpayloadformattedschema) | no | The formatted post data modified from original post. |
| `parentPostId` | String | no | The ID of the parent post before splitting to individual social posts. |
| `errors` | Vec<String> | no | List of errors associated with the queue item. Possible values: INVALID_USER_ID, PIXABAY_MEDIA. |
| `currentVariation` | f64 | no | The index of the current variation being used for this post |
| `createdAt` | String | no | Creation timestamp |
| `updatedAt` | String | no | Last update timestamp |
| `deleted` | bool | no | Indicates if the item is deleted |
| `locationId` | String | no | Location ID |

### `CreatedTimeSlotDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | Time slot ID |
| `dayOfWeek` | f64 | no | Day of the week (0=Sunday, 1=Monday, ...) |
| `time` | String | no | Time of the day (HH:mm format) |

### `CsvPostStatusResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |

### `CsvResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | no | Location Id |
| `fileName` | String | no | Name of the CSV file |
| `accountIds` | Vec<String> | no | Account Ids |
| `file` | String | no | File path of the CSV |
| `status` | String — `pending`, `in_progress`, `completed`, `failed`, `in_review`, `importing`, `deleted` | no | CSV import status |
| `count` | f64 | no | Number of posts in the CSV |
| `createdBy` | String | no | User Id who created the CSV import |
| `traceId` | String | no | Trace Id for debugging |
| `originId` | String | no | Origin Id for tracking source |
| `approver` | String | no | Approver User Id |
| `csvFileType` | String — `basic`, `advance` | no | CSV file type |
| `mediaOptimization` | bool | no | Media optimization flag |
| `applyWatermark` | bool | no | Apply watermark flag |
| `updatedAt` | String | no | Date Updated |

### `DateSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `year` | f64 | **yes** | Year component of the date |
| `month` | f64 | **yes** | Month component of the date (1-12) |
| `day` | f64 | **yes** | Day component of the date (1-31) |

### `DeleteAccountResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | no | Location Id |
| `id` | String | no | Id |

### `DeleteActivePostResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | no | A message indicating the result of the operation. |

### `DeleteCsvResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`CSVResponseSchema`](#csvresponseschema) | no | Requested Results |

### `DeleteLikeResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |

### `DeletePostCsvSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | CSV Id |
| `csvFileType` | String — `basic`, `advance` | no | CSV file type |
| `mediaOptimization` | bool | no | Media optimization flag |
| `applyWatermark` | bool | no | Apply watermark flag |
| `status` | String — `pending`, `in_progress`, `completed`, `failed`, `in_review`, `importing`, `deleted` | no | CSV import status |
| `updatedAt` | String | no | Date Updated |

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
| `csv` | [`DeletePostCsvSchema`](#deletepostcsvschema) | no | CSV Data |

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
| `postId` | String | no | Platform-specific post identifier |

### `DeletePostsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `postIds` | Vec<String> | no | Requested Results |

### `DiscardEditSessionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `sessionId` | String | **yes** | Edit session ID |
| `keepInDraft` | bool | no | If true, keeps the queue in DRAFT state after saving instead of automatically activating it. Only applicable when the queue is currently in DRAFT status. |

### `DiscardEditSessionResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | no | A message indicating the result of the operation. |

### `EditSessionCalendarDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `sessionId` | String | **yes** | Edit session ID |
| `startDate` | String | **yes** | Start Date in ISO format |
| `endDate` | String | **yes** | End Date in ISO format |
| `accountIds` | Vec<String> | no | Filter by Account IDs. If not provided or empty, returns all posts. |

### `EditSessionCalendarResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | no | — |
| `scheduledPosts` | Vec<EditSessionScheduledPostDTO> | no | — |
| `total` | f64 | no | Total number of scheduled posts returned |
| `timezone` | String | no | The timezone used for scheduling, e.g., "Asia/Calcutta" |

### `EditSessionScheduledPostDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `scheduledDateTime` | String | no | The date and time the post is scheduled to be published |
| `post` | [`GetPostFormattedSchema`](#getpostformattedschema) | no | The formatted post data. |
| `queueItemId` | String | no | The unique identifier of the queue item. |
| `queueId` | String | no | The ID of the queue this post belongs to |
| `order` | f64 | no | The order of the item in the queue. |
| `variations` | Vec<VariationDTO> | no | A list of content variations for the post. |
| `primaryImage` | String | no | The primary image URL for the post. |
| `errors` | Vec<String> | no | List of errors associated with the queue item. Possible values: INVALID_USER_ID, PIXABAY_MEDIA. |
| `category` | [`CategoryInfoDTO`](#categoryinfodto) | no | The category associated with this post |
| `currentVariation` | f64 | no | The index of the current variation being used for this post |
| `timezone` | String | no | The timezone in which the post is scheduled |
| `isDraft` | bool | no | Indicates this is a draft item from an edit session |
| `originalItemId` | String | no | Original queue item ID if this draft was created from an existing item |

### `EndDateSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `endDate` | [`DateSchema`](#dateschema) | no | End Date |
| `endTime` | [`TimeSchema`](#timeschema) | no | End Time |

### `FacebookPageSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Facebook page identifier |
| `name` | String | no | Name of the Facebook page |
| `avatar` | String | no | Avatar or profile picture URL of the page |
| `isOwned` | bool | no | Indicates if the user owns this page |
| `isConnected` | bool | no | Indicates if the page is currently connected |

### `FacebookPostSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `post`, `story`, `reel` | **yes** | Facebook post format type. **Available Types:** - `post` - Standard feed post (images, videos, text) - `story` - 24-hour temporary story - `reel` - Short-form vertical video **Restrictions:** - Reels:… |
| `textFormatPresetId` | String | no | Facebook background preset ID for text-only feed posts. **Facebook `post` only** — not `story` or `reel`. Ignored when media is attached; `metaLink` is omitted on publish. **Validations** — request re… |

### `FetchAvailableCategoriesResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | no | — |
| `categories` | Vec<AvailableCategoryDTO> | no | List of categories with queue status |
| `meta` | [`MetaDTO`](#metadto) | no | — |

### `FetchCalendarListResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | no | — |
| `scheduledPosts` | Vec<ScheduledPostDTO> | no | — |
| `total` | f64 | no | Total number of scheduled posts returned |
| `timezone` | String | no | The timezone used for scheduling, e.g., "Asia/Calcutta" |

### `FetchCategoryQueuesDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `skip` | f64 | no | Number of items to skip |
| `limit` | f64 | no | Maximum number of items to return |

### `FetchCategoryQueuesResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | no | — |
| `queues` | Vec<CategoryQueueWithCategoryDTO> | no | — |
| `meta` | [`MetaDTO`](#metadto) | no | — |

### `FetchQueueByIdResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | no | A message indicating the result of the operation. |
| `queue` | [`CategoryQueueWithCategoryDTO`](#categoryqueuewithcategorydto) | no | The fetched queue along with its category metadata. |

### `FetchQueueItemsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `sessionId` | String | no | Edit session ID |
| `skip` | f64 | no | Number of items to skip |
| `limit` | f64 | no | Maximum number of items to return |
| `errorFilter` | bool | no | To return only queue items with errors |
| `itemId` | String | no | Item ID to center the response around. When provided, the response will position this item in the center with items above and below based on limit. The skip parameter is ignored when itemId is provide… |

### `FetchQueueItemsMetaDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `count` | String | no | Total count of items |
| `skip` | f64 | no | Number of items skipped (offset from start) |
| `limit` | f64 | no | Maximum number of items returned |

### `FetchQueueItemsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | no | — |
| `items` | Vec<QueueItemDTO> | no | — |
| `meta` | [`FetchQueueItemsMetaDTO`](#fetchqueueitemsmetadto) | no | — |

### `FetchSlotsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | The location ID |
| `sessionId` | String | no | Session ID for edit mode. If not provided, calculates slots for live items. |
| `skip` | f64 | no | Number of items to skip |
| `limit` | f64 | no | Number of items to return |

### `FetchSlotsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | no | — |
| `slots` | Vec<UpdatedSlotInfoDTO> | no | Slot information for items in the requested range |
| `total` | f64 | no | Total number of items in the queue |
| `skip` | f64 | no | Number of items skipped |
| `limit` | f64 | no | Number of items returned |
| `timezone` | String | no | Timezone used for slot calculations |

### `FormatedApprovalDetails`

| Field | Type | Required | Description |
|---|---|---|---|
| `approver` | String | no | User ID of the designated approver. **Note:** The approver will receive a notification when a post is submitted for review. |
| `requesterNote` | String | no | Note from the post creator to the approver explaining the post or requesting specific feedback. |
| `approverNote` | String | no | Note from the approver to the post creator with feedback or approval comments. |
| `approvalStatus` | String — `pending`, `approved`, `rejected`, `not_required` | no | Current approval status of the post. **Available Values:** - `pending` - Awaiting approver review - `approved` - Approved and ready for publishing - `rejected` - Rejected by approver (needs revision) … |
| `approverUser` | [`PostUserSchema`](#postuserschema) | no | Approver User Details |

### `GMBPostSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `gmbEventType` | String — `STANDARD`, `EVENT`, `OFFER` | **yes** | Google My Business post type. **Available Types:** - `STANDARD` - Regular update post (What's New) - `EVENT` - Event announcement with dates and title - `OFFER` - Promotional offer with coupon and red… |
| `title` | String | no | Event title. Required when `gmbEventType` is `EVENT`. **Max length:** 58 characters |
| `offerTitle` | String | no | Offer title. Required when `gmbEventType` is `OFFER`. |
| `startDate` | [`StartDateSchema`](#startdateschema) | no | Start date and time for EVENT or OFFER posts. **Required:** When `gmbEventType` is `EVENT` or `OFFER`. **Structure:** - `startDate`: { year, month, day } - `startTime`: { hours, minutes, seconds } |
| `endDate` | [`EndDateSchema`](#enddateschema) | no | End date and time for EVENT or OFFER posts. **Required:** When `gmbEventType` is `EVENT` or `OFFER`. **Validation:** Must be after `startDate`. **Structure:** - `endDate`: { year, month, day } - `endT… |
| `termsConditions` | String | no | URL to terms and conditions page. Required for OFFER posts. |
| `url` | String | no | Call-to-action URL. Required when `actionType` is set (except `none` and `call`). **Required for:** STANDARD and EVENT posts with actionType other than `none` or `call`. |
| `couponCode` | String | no | Promotional coupon code. Required for OFFER posts. |
| `redeemOnlineUrl` | String | no | URL where customers can redeem the offer online. Required for OFFER posts. |
| `actionType` | String — `none`, `order`, `book`, `shop`, `learn_more`, `call`, `sign_up` | no | Call-to-action button type for the post. **Available Actions:** - `none` - No action button - `order` - Order online - `book` - Book appointment - `shop` - Shop now - `learn_more` - Learn more - `call… |

### `GeneralSuccessResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | no | A message indicating the result of the operation. |
| `updatedSlots` | Vec<UpdatedSlotInfoDTO> | no | Updated slot information for items affected by the operation |
| `totalPostsChanged` | f64 | no | Number of unique posts that had their slots changed |

### `GetAccountSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Unique identifier for the connected account |
| `oauthId` | String | no | OAuth provider account identifier |
| `profileId` | String | no | Profile identifier from the social media platform |
| `name` | String | no | Display name of the account |
| `platform` | String | no | platform must be one of the following values: google, facebook, instagram, linkedin, tiktok |
| `type` | String | no | Type of account (e.g., location, page, profile) |
| `expire` | String | no | Token expiration date and time |
| `isExpired` | bool | no | Indicates if the account token has expired |
| `meta` | JSON | no | Additional metadata for the account |

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
| `count` | f64 | no | Total count of posts in CSV |
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
| `accountIds` | Vec<String> | **yes** | Array of account IDs belonging to this group |

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

### `GetModifiedPayloadFormattedSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | MongoDB document ID of the post |
| `source` | String — `composer`, `csv`, `recurring`, `review`, `rss`, `template-library`, `category-queue`, `native` | no | source must be one of the following values: composer, recurring, csv |
| `locationId` | String | **yes** | Location Id |
| `displayDate` | String | no | Display date for the post |
| `createdAt` | String | no | Date when the post was created |
| `updatedAt` | String | no | Date when the post was last updated |
| `accountId` | String | no | Account Id |
| `error` | String | **yes** | Error |
| `postId` | String | no | Platform-specific post identifier |
| `publishedAt` | String | no | Date when the post was published |
| `thumbnail` | String | no | Post-level cover image (thumbnail) URL. For posts that contain a video, this is the resolved cover image used for the first video in `media[]`. It is set automatically from the `thumbnail` provided on… |
| `accountIds` | Vec<String> | no | Account Ids |
| `summary` | String | no | Content text of the post |
| `media` | Vec<PostMediaSchema> | no | Post Media Data The limitations of media as per the platforms is provided through the reference link in API description |
| `status` | JSON | no | Status must be one of the following values: in_progress, draft, failed, published, scheduled, in_review, notification_sent, deleted |
| `createdBy` | String | no | User ID who created the post |
| `type` | JSON | **yes** | Post Type must be one of the following values: - post, story, reel |
| `tags` | Vec<String> | no | Tag Ids |
| `ogTagsDetails` | [`OgTagsSchema`](#ogtagsschema) | no | Og Tags Meta Data |
| `postApprovalDetails` | [`FormatedApprovalDetails`](#formatedapprovaldetails) | no | Post Approval Details |
| `tiktokPostDetails` | [`TiktokPostSchema`](#tiktokpostschema) | no | Tiktok Post Details |
| `gmbPostDetails` | [`GMBPostSchema`](#gmbpostschema) | no | GMB Post Details |
| `user` | [`PostUserSchema`](#postuserschema) | no | User |
| `linkedinPostDetails` | [`LinkedinPostSchema`](#linkedinpostschema) | no | Linkedin Post Details |
| `pinterestPostDetails` | [`PinterestPostSchema`](#pinterestpostschema) | no | Pinterest Post Details |
| `facebookPostDetails` | [`FacebookPostSchema`](#facebookpostschema) | no | Facebook Post Details |
| `instagramPostDetails` | [`InstagramPostSchema`](#instagrampostschema) | no | Instagram Post Details |
| `youtubePostDetails` | [`YoutubePostSchema`](#youtubepostschema) | no | Youtube Post Details |
| `mediaOptimization` | bool | no | Pass this parameter to optimize the image media |

### `GetPinterestAccountSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `profile` | Vec<PinterestProfileSchema> | no | Pinterest Profile |

### `GetPinterestAccountsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`GetPinterestAccountSchema`](#getpinterestaccountschema) | no | Requested Results |

### `GetPostFormattedSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | MongoDB document ID of the post |
| `source` | String — `composer`, `csv`, `recurring`, `review`, `rss` | no | source must be one of the following values: composer, recurring, csv |
| `locationId` | String | **yes** | Location Id |
| `platform` | String | no | platform must be one of the following values: google, facebook, instagram, linkedin, twitter, tiktok |
| `thumbnail` | String | no | Post-level cover image (thumbnail) URL. For posts that contain a video, this is the resolved cover image used for the first video in `media[]`. It is set automatically from the `thumbnail` provided on… |
| `displayDate` | String | no | Display date for the post |
| `createdAt` | String | no | Date when the post was created |
| `updatedAt` | String | no | Date when the post was last updated |
| `accountId` | String | no | Account Id |
| `error` | String | **yes** | Error |
| `postId` | String | no | Platform-specific post identifier |
| `publishedAt` | String | no | Date when the post was published |
| `accountIds` | Vec<String> | no | Account Ids |
| `summary` | String | no | Content text of the post |
| `media` | Vec<PostMediaSchema> | no | Post Media Data The limitations of media as per the platforms is provided through the reference link in API description |
| `status` | JSON | no | Status must be one of the following values: in_progress, draft, failed, published, scheduled, in_review, notification_sent, deleted |
| `createdBy` | String | no | User ID who created the post |
| `type` | JSON | **yes** | Post Type must be one of the following values: - post, story, reel |
| `tags` | Vec<String> | no | Tag Ids |
| `ogTagsDetails` | [`OgTagsSchema`](#ogtagsschema) | no | Og Tags Meta Data |
| `postApprovalDetails` | [`FormatedApprovalDetails`](#formatedapprovaldetails) | no | Post Approval Details |
| `tiktokPostDetails` | [`TiktokPostSchema`](#tiktokpostschema) | no | Tiktok Post Details |
| `gmbPostDetails` | [`GMBPostSchema`](#gmbpostschema) | no | GMB Post Details |
| `blueskyPostDetails` | [`BlueskyPostSchema`](#blueskypostschema) | no | Bluesky Post Details |
| `user` | [`PostUserSchema`](#postuserschema) | no | User |
| `linkedinPostDetails` | [`LinkedinPostSchema`](#linkedinpostschema) | no | Linkedin Post Details |
| `pinterestPostDetails` | [`PinterestPostSchema`](#pinterestpostschema) | no | Pinterest Post Details |
| `facebookPostDetails` | [`FacebookPostSchema`](#facebookpostschema) | no | Facebook Post Details |
| `instagramPostDetails` | [`InstagramPostSchema`](#instagrampostschema) | no | Instagram Post Details |
| `youtubePostDetails` | [`YoutubePostSchema`](#youtubepostschema) | no | Youtube Post Details |
| `mediaOptimization` | bool | no | Pass this parameter to optimize the image media |
| `insights` | [`PostInsightsSchema`](#postinsightsschema) | no | Aggregated engagement metrics for the published post. Populated asynchronously by the insights sync workers for supported platforms (Facebook, Instagram, LinkedIn, YouTube). Absent on posts that have … |

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
| `csvs` | Vec<CSVImportSchema> | **yes** | CSV Data |
| `count` | f64 | **yes** | Total count of CSV records |

### `GetYouTubeAccountsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`GetYoutubeAccountSchema`](#getyoutubeaccountschema) | no | Requested Results |

### `GetYoutubeAccountSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `profile` | Vec<YoutubeProfileSchema> | no | Youtube Profile |

### `GoogleAccountsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Google Business account identifier |
| `accountName` | String | no | Display name of the Google Business account |
| `type` | String | no | Type of Google Business account |
| `verificationState` | String | no | Verification state of the account |
| `vettedState` | String | no | Vetted state of the account by Google |

### `GoogleLocationSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Google Business account identifier |
| `storeCode` | String | no | Store code or location identifier |
| `title` | String | no | Business location title or name |
| `metadata` | JSON | no | Meta data not related to User |
| `storefrontAddress` | JSON | no | Store front address |
| `relationshipData` | JSON | no | All locations and chain related to this one |
| `maxLocation` | bool | no | Indicates if location limit has been reached |
| `isVerified` | bool | no | Indicates if the location is verified by Google |
| `isConnected` | bool | no | Indicates if the location is currently connected |

### `IOgTagsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | String | no | Tag url |
| `ogDescription` | String | no | Tag description |
| `ogImage` | [`OgImageSchema`](#ogimageschema) | no | OG Image data |
| `ogTitle` | String | no | Tag Title |
| `ogUrl` | String | no | Tag Url |
| `ogSiteName` | String | no | Site Name |
| `error` | String | no | Og Tag Error |

### `InstagramAccountSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Instagram account identifier |
| `name` | String | no | Display name of the Instagram account |
| `avatar` | String | no | Avatar or profile picture URL |
| `pageId` | String | no | Facebook page ID associated with the Instagram account |
| `isConnected` | bool | no | Indicates if the account is currently connected |

### `InstagramPostSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `post`, `story`, `reel` | **yes** | Instagram post format type. **Available Types:** - `post` - Standard feed post (images, videos, carousels) - `story` - 24-hour temporary story - `reel` - Short-form vertical video (up to 90 seconds) *… |
| `collaborators` | JSON | no | Object mapping account IDs to arrays of associated usernames for collaboration. Only allowed for type "post" and "reels" |
| `showOnFeed` | bool | no | Show Reel on profile grid/feed. **✅ Applies to:** Reels only - `true` - Reel appears on your profile grid - `false` - Reel only appears in Reels tab |
| `publishViaPushNotification` | bool | no | Send Instagram Story via Push Notification instead of direct posting. Only applicable for Story type. |
| `publisherNote` | String | no | Note to the publisher for manual posting guidance. Only used when publishViaPushNotification is true. |

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

### `LinkedinPollDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `question` | String | **yes** | Question for the poll. Max length: 140 characters. |
| `options` | Vec<LinkedinPollOptionDto> | **yes** | Poll options. Minimum 2, maximum 4. Each option text max 30 characters. Option texts must be unique. |
| `settings` | [`LinkedinPollSettingsDto`](#linkedinpollsettingsdto) | **yes** | Poll settings (duration). |

### `LinkedinPollOptionDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `text` | String | **yes** | Text describing the option. Max length: 30 characters. |

### `LinkedinPollSettingsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `duration` | String — `ONE_DAY`, `THREE_DAYS`, `SEVEN_DAYS`, `FOURTEEN_DAYS` | **yes** | Duration the poll stays open for votes. |

### `LinkedinPostSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `pdfTitle` | String | **yes** | Title for the PDF document carousel. Displayed as the document name on LinkedIn. **Max length:** 100 characters **Tip:** Use a descriptive title that explains the document content. |
| `postAsPdf` | bool | **yes** | Post images as a PDF document carousel. **Limits:** - Max 300 pages/images - Max PDF size: 100 MB |
| `poll` | [`LinkedinPollDto`](#linkedinpolldto) | no | Publish a LinkedIn poll post. **Required fields when `poll` is supplied:** - `question` (max 140 characters) - `options`: 2 to 4 entries, each `text` ≤ 30 characters; option texts must be unique - `se… |

### `LocationAndAccountDeleteResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`DeleteAccountResponseSchema`](#deleteaccountresponseschema) | no | Requested Results |

### `MentionsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Mention name |
| `id` | String | **yes** | Mention ID |
| `offset` | f64 | **yes** | Mention offset |
| `length` | f64 | **yes** | Mention length |
| `slug` | String | no | Mention slug for community profile link |

### `MetaDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `count` | String | no | Total count of items |

### `OgImageSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | String | no | Image url |
| `width` | f64 | no | Image width |
| `height` | f64 | no | Image height |
| `type` | String | no | Image Type |

### `OgTagsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `metaLink` | JSON | no | The canonical URL of the content. |
| `metaImage` | JSON | no | URL of the content's primary image. |
| `ogTitle` | JSON | no | The title of the content. |

### `OgTagsInputDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `metaLink` | JSON | no | The canonical URL of the content for link preview. |
| `metaImage` | JSON | no | URL of the image to display in link preview. |
| `ogTitle` | JSON | no | Title to display in link preview. |

### `OgTagsSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `metaImage` | String | no | Preview image URL for the shared link. **Best Practices:** - Use high-quality images (1200x630px recommended) - Ensure the image is publicly accessible **Auto-fetch:** Use the Get Metatags API to fetc… |
| `metaLink` | String | no | URL of the webpage being shared. This is the destination when users click the link preview. |
| `ogTitle` | String | no | Custom title for the link preview. Overrides the page's og:title meta tag. **Tip:** Keep titles concise (50-60 characters) for best display across platforms. |
| `ogDescription` | String | no | Custom description for the link preview. Overrides the page's og:description meta tag. **Tip:** Keep descriptions under 155 characters for optimal display. |

### `PinterestBoardSelection`

Per-account Pinterest board selection. Each entry binds one connected Pinterest account to a list of board IDs the pin should publish to. Each selected board produces an independent child post tracked separately for success/failure.

| Field | Type | Required | Description |
|---|---|---|---|
| `accountId` | String | **yes** | Connected Pinterest account ID. Must match one of the accounts referenced in the post's `userIds`. |
| `boards` | Vec<String> | **yes** | Pinterest board IDs to publish to on this account. Each board produces an independent child post. Capped at 25 boards per account. |

### `PinterestOAuthAccountSchema`

_No fields defined in the spec._

### `PinterestPostSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | no | Pin title displayed on Pinterest. **Max length:** 100 characters **Best Practices:** - Include relevant keywords - Be descriptive and engaging |
| `link` | String | no | Destination URL for the pin. Users clicking the pin will be directed to this URL. **Max length:** 2048 characters **Best Practices:** - Use direct links to relevant content - Track with UTM parameters… |
| `boardIds` | JSON | no | **DEPRECATED — use `pinterestBoards` instead.** Will be removed on July 31, 2026. Legacy mapping of Pinterest account OAuth IDs to a single board ID: `{ accountOAuthId: "boardId" }` For multi-board po… |
| `pinterestBoards` | Vec<PinterestBoardSelection> | no | Per-account Pinterest board selection. Each entry binds one connected Pinterest account to a list of boards on that account. Each board produces an independent child post tracked separately for succes… |
| `shortenedLinks` | Vec<String> | no | Shortened links for the post (auto-generated). |

### `PinterestProfileSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Id |
| `name` | String | no | Name of account |
| `username` | String | no | Username of account |
| `avatar` | String | no | Avatar of profile account |
| `isConnected` | bool | no | Is connected |
| `type` | String | no | Pinterest Account Type |
| `websiteUrl` | String | no | Pinterest Account website Url |

### `PostApprovalSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `approver` | String | no | User ID of the designated approver. **Note:** The approver will receive a notification when a post is submitted for review. |
| `requesterNote` | String | no | Note from the post creator to the approver explaining the post or requesting specific feedback. |
| `approverNote` | String | no | Note from the approver to the post creator with feedback or approval comments. |
| `approvalStatus` | String — `pending`, `approved`, `rejected`, `not_required` | no | Current approval status of the post. **Available Values:** - `pending` - Awaiting approver review - `approved` - Approved and ready for publishing - `rejected` - Rejected by approver (needs revision) … |

### `PostCreateRequest`

| Field | Type | Required | Description |
|---|---|---|---|
| `accountIds` | Vec<String> | no | Account Ids |
| `summary` | String | no | Post Content The limitations of content as per the platforms is provided through the reference link in API description |
| `media` | Vec<PostMediaSchema> | no | Post Media Data The limitations of media as per the platforms is provided through the reference link in API description |
| `status` | JSON | no | Status must be one of the following values: in_progress, draft, failed, published, scheduled, in_review, notification_sent, deleted |
| `scheduleDate` | String | no | Schedule Date |
| `createdBy` | String | no | User ID of the creator who is creating/managing the post. Must be a valid MongoDB ObjectId. **Get User IDs from:** [Get User API](../users/get-user) — use the `id` field from the user object. **Valida… |
| `followUpComment` | String | no | Follow-up comment to be posted immediately after the main post is published. **Supported Platforms:** Facebook, Instagram, LinkedIn, YouTube **NOT Supported:** TikTok, Google My Business (GMB), Pinter… |
| `ogTagsDetails` | [`OgTagsSchema`](#ogtagsschema) | no | Og Tags Meta Data |
| `type` | JSON | **yes** | Post Type must be one of the following values: - post, story, reel |
| `postApprovalDetails` | [`PostApprovalSchema`](#postapprovalschema) | no | Post Approval Details |
| `scheduleTimeUpdated` | bool | no | if schedule datetime is updated |
| `tags` | Vec<String> | no | Array of Tag Value |
| `categoryId` | String | no | Category Id |
| `tiktokPostDetails` | [`TiktokPostSchema`](#tiktokpostschema) | no | Tiktok Post Details |
| `gmbPostDetails` | [`GMBPostSchema`](#gmbpostschema) | no | GMB Post Details |
| `userId` | String | no | User ID |

### `PostInsightsSchema`

Aggregated engagement metrics for a published post.

| Field | Type | Required | Description |
|---|---|---|---|
| `like` | f64 | no | Total number of likes (or platform-equivalent reactions) on the post. |
| `share` | f64 | no | Total number of shares/reposts of the post. |
| `comment` | f64 | no | Total number of comments on the post. |

### `PostMediaSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `url` | String | **yes** | Public URL of the media file. Must be a valid, accessible HTTPS URL. |
| `caption` | String | no | Alt text or caption for the media. Used for accessibility and SEO. |
| `originalUrl` | String | no | Original media URL before any processing (watermarking/optimization). Set automatically by the system. |
| `watermarkedUrl` | String | no | URL of the media after watermarking. Set automatically when watermark is applied. |
| `type` | String — `image/jpeg`, `image/jpg`, `image/png`, `image/gif`, `video/mp4`, `video/mov`, `video/webm` | **yes** | MIME type of the media file. See Platform Limitations Guide for platform-specific format support. |
| `thumbnail` | String | no | Cover image URL for a video media item. **Scope** - Applies to the **first video** in `media[]`. Values supplied on subsequent video items are ignored. - Has no effect on image-only media items. **Res… |
| `id` | String | no | Unique identifier for the media item. Used for tracking and referencing. |
| `optimizedUrl` | String | no | URL of the optimized/compressed media. Set automatically when media optimization is enabled. **Enable Optimization:** Set `mediaOptimization: true` in the post request. |
| `optimizedType` | String | no | MIME type of the optimized media. May differ from original if format conversion occurred. |
| `isModified` | bool | no | Flag indicating if the media was modified (watermarked, optimized, or processed). |
| `altText` | String | no | Alt text for accessibility. Supported on Instagram, Threads, Pinterest, Bluesky, and LinkedIn image posts (ignored for video and other platforms). Auto-truncated per platform: Pinterest 500, Instagram… |

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
| `count` | f64 | no | Total count of posts |

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

### `QueueItemDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | The unique identifier of the queue item. |
| `order` | f64 | no | The order of the item in the queue. |
| `variations` | Vec<VariationDTO> | no | A list of content variations for the post. |
| `primaryImage` | String | no | The primary image URL for the post. |
| `postId` | String | no | The ID of the original post, if any. |
| `post` | [`GetPostFormattedSchema`](#getpostformattedschema) | no | The formatted post data. |
| `errors` | Vec<String> | no | List of errors associated with the queue item. Possible values: INVALID_USER_ID, PIXABAY_MEDIA. |
| `scheduledDateTime` | String | no | The calculated date/time when this item is scheduled to be posted |
| `scheduledVariationIndex` | f64 | no | The variation index that will be used when this item is posted |
| `isSkipped` | bool | no | Indicates if this time slot is skipped and the post will not be published at this time |

### `QueueItemWithVariationsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | The unique identifier of the queue item. |
| `order` | f64 | no | The order of the item in the queue. |
| `variations` | Vec<VariationDTO> | no | A list of content variations for the post. |
| `primaryImage` | String | no | The primary image URL for the post. |
| `postId` | String | no | The ID of the original post, if any. |
| `post` | [`GetPostFormattedSchema`](#getpostformattedschema) | no | The formatted post data. |
| `errors` | Vec<String> | no | List of errors associated with the queue item. Possible values: INVALID_USER_ID, PIXABAY_MEDIA. |
| `scheduledDateTime` | String | no | The calculated date/time when this item is scheduled to be posted |
| `scheduledVariationIndex` | f64 | no | The variation index that will be used when this item is posted |
| `isSkipped` | bool | no | Indicates if this time slot is skipped and the post will not be published at this time |
| `currentVariation` | f64 | no | The index of the current variation being used for this post |

### `QueueModifiedPostDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `accountIds` | Vec<String> | no | Account Ids (OAuth only) |
| `summary` | String | no | Post Content |
| `media` | Vec<PostMediaSchema> | no | Post Media |
| `status` | JSON | no | Post status indicating the current state of the post. **Available Status Values:** - `draft` - Post saved as draft, not yet ready for publishing - `scheduled` - Post scheduled for future publishing (r… |
| `scheduleDate` | String | no | Schedule Date |
| `selectedBestTime` | String | no | Selected Best Time slot for scheduling |
| `createdBy` | String | no | Created By |
| `followUpComment` | String | no | Follow-up comment to be posted immediately after the main post is published. **Supported Platforms:** Facebook, Instagram, LinkedIn, YouTube **NOT Supported:** TikTok, Google My Business (GMB), Pinter… |
| `ogTagsDetails` | [`OgTagsSchema`](#ogtagsschema) | no | Og Tags Meta Data |
| `type` | JSON | no | Post Type must be one of the following values: - post, story, reel |
| `postApprovalDetails` | [`PostApprovalSchema`](#postapprovalschema) | no | Post Approval Details |
| `scheduleTimeUpdated` | bool | no | Flag indicating if the schedule datetime was manually updated. Used for tracking rescheduled posts. |
| `tags` | Vec<String> | no | Array of Tag IDs to associate with the post for organization and filtering. **Get Tag IDs from:** [Get Tags API](./social-planner/get-tags-location-id) — use the `_id` field from each tag. **Validatio… |
| `categoryId` | String | no | Category ID to organize the post. Categories help group related posts. **Get Category IDs from:** [Get Categories API](./social-planner/get-categories-location-id) — use the `_id` field. **Validation:… |
| `applyWatermark` | bool | no | Apply watermark to media in this post. **Note:** Watermarks are applied to images only. Videos are not watermarked. |
| `tiktokPostDetails` | [`TiktokPostSchema`](#tiktokpostschema) | no | Tiktok Post Details |
| `gmbPostDetails` | [`GMBPostSchema`](#gmbpostschema) | no | GMB Post Details |
| `userId` | String | no | User ID |
| `linkedinPostDetails` | [`LinkedinPostSchema`](#linkedinpostschema) | no | Linkedin Post Details |
| `pinterestPostDetails` | [`PinterestPostSchema`](#pinterestpostschema) | no | Pinterest Post Details |
| `facebookPostDetails` | [`FacebookPostSchema`](#facebookpostschema) | no | Facebook Post Details |
| `instagramPostDetails` | [`InstagramPostSchema`](#instagrampostschema) | no | Instagram Post Details |
| `youtubePostDetails` | [`YoutubePostSchema`](#youtubepostschema) | no | Youtube Post Details |
| `locationId` | String | no | Location Id |

### `ResetQueueItemDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `sessionId` | String | no | Edit session ID |

### `ResetQueueItemResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | no | A message indicating the result of the operation. Examples: "Queue item reset successfully", "Dummy queue item deleted successfully". |
| `queueItem` | [`QueueItemWithVariationsDTO`](#queueitemwithvariationsdto) | no | The reset queue item, including its current variation. |

### `SaveEditSessionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `sessionId` | String | **yes** | Edit session ID |
| `keepInDraft` | bool | no | If true, keeps the queue in DRAFT state after saving instead of automatically activating it. Only applicable when the queue is currently in DRAFT status. |

### `SaveEditSessionResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | no | A message indicating the result of the operation. |
| `updatedSlots` | Vec<UpdatedSlotInfoDTO> | no | Updated slot information for all items after saving changes |
| `totalPostsChanged` | f64 | no | Number of unique posts that had their slots changed |

### `ScheduledPostDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `scheduledDateTime` | String | no | The date and time the post is scheduled to be published |
| `post` | [`GetPostFormattedSchema`](#getpostformattedschema) | no | The formatted post data. |
| `queueItemId` | String | no | The unique identifier of the queue item. |
| `queueId` | String | no | The ID of the queue this post belongs to |
| `order` | f64 | no | The order of the item in the queue. |
| `variations` | Vec<VariationDTO> | no | A list of content variations for the post. |
| `primaryImage` | String | no | The primary image URL for the post. |
| `errors` | Vec<String> | no | List of errors associated with the queue item. Possible values: INVALID_USER_ID, PIXABAY_MEDIA. |
| `category` | [`CategoryInfoDTO`](#categoryinfodto) | no | The category associated with this post |
| `currentVariation` | f64 | no | The index of the current variation being used for this post |
| `timezone` | String | no | The timezone in which the post is scheduled |

### `SearchPostDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String | no | type must be one of the following values: recent, all, scheduled, draft, failed, in_review, published, in_progress, pending and deleted |
| `accounts` | String | no | List of account Ids separated by comma as a string |
| `skip` | String | **yes** | Number of records to skip for pagination |
| `limit` | String | **yes** | Maximum number of records to return |
| `fromDate` | String | **yes** | From Date |
| `toDate` | String | **yes** | To Date |
| `includeUsers` | String | **yes** | Include User Data |
| `postType` | JSON | no | Post Type must be one of the following values: - post, story, reel |

### `SetAccountsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `accountIds` | Vec<String> | **yes** | Account Ids |
| `filePath` | String | **yes** | File path |
| `rowsCount` | f64 | **yes** | Entries Count. rowsCount must be between 1 and number of posts in CSV |
| `fileName` | String | **yes** | Name of file |
| `approver` | String | no | Approver User Id |
| `userId` | String | **yes** | User ID |
| `csvFileType` | String — `basic`, `advance` | no | CSV file type - determines the format of the CSV file being imported |

### `SetAccountsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`SetAccountsResultSchema`](#setaccountsresultschema) | no | Requested Results |

### `SetAccountsResultSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `csvId` | String | **yes** | CSV Id |

### `SetAccountsUnprocessableDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | f64 | **yes** | HTTP Status |
| `options` | JSON | no | Options |
| `message` | Vec<String> | **yes** | Validation error messages |
| `name` | String | **yes** | Exception name |
| `error` | String | **yes** | Error type |
| `statusCode` | f64 | **yes** | HTTP Status Code |
| `traceId` | String | no | Trace ID for debugging |

### `SocialGoogleMediaAccountSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | MongoDB document ID of the social media account |
| `oAuthId` | String | no | OAuth provider account identifier |
| `oldId` | String | no | Legacy account identifier for backward compatibility |
| `locationId` | String | no | Location ID associated with this account |
| `originId` | String | no | Original platform-specific account identifier |
| `platform` | JSON | no | Social media platform name |
| `type` | JSON | no | Type of account (e.g., location, page, profile) |
| `name` | String | no | Display name of the account |
| `avatar` | String | no | Avatar or profile picture URL |
| `meta` | JSON | no | Additional metadata for the account |
| `active` | bool | no | Indicates if the account is currently active |
| `deleted` | bool | no | Indicates if the account has been deleted |
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
| `_id` | String | no | MongoDB document ID of the social media account |
| `oAuthId` | String | no | OAuth provider account identifier |
| `oldId` | String | no | Legacy account identifier for backward compatibility |
| `locationId` | String | no | Location ID associated with this account |
| `originId` | String | no | Original platform-specific account identifier |
| `platform` | JSON | no | Social media platform name |
| `type` | JSON | no | type value must be page |
| `name` | String | no | Display name of the account |
| `avatar` | String | no | Avatar or profile picture URL |
| `meta` | JSON | no | Additional metadata for the account |
| `active` | bool | no | Indicates if the account is currently active |
| `deleted` | bool | no | Indicates if the account has been deleted |
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
| `_id` | String | no | MongoDB document ID of the social media account |
| `oAuthId` | String | no | OAuth provider account identifier |
| `oldId` | String | no | Legacy account identifier for backward compatibility |
| `locationId` | String | no | Location ID associated with this account |
| `originId` | String | no | Original platform-specific account identifier |
| `platform` | JSON | no | Social media platform name |
| `type` | JSON | no | Type of account (e.g., location, page, profile) |
| `name` | String | no | Display name of the account |
| `avatar` | String | no | Avatar or profile picture URL |
| `meta` | JSON | no | Additional metadata for the account |
| `active` | bool | no | Indicates if the account is currently active |
| `deleted` | bool | no | Indicates if the account has been deleted |
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
| `_id` | String | no | MongoDB document ID of the social media account |
| `oAuthId` | String | no | OAuth provider account identifier |
| `oldId` | String | no | Legacy account identifier for backward compatibility |
| `locationId` | String | no | Location ID associated with this account |
| `originId` | String | no | Original platform-specific account identifier |
| `platform` | JSON | no | Social media platform name |
| `type` | JSON | no | type must be one of the following values: page, profile |
| `name` | String | no | Display name of the account |
| `avatar` | String | no | Avatar or profile picture URL |
| `meta` | JSON | no | Additional metadata for the account |
| `active` | bool | no | Indicates if the account is currently active |
| `deleted` | bool | no | Indicates if the account has been deleted |
| `createdAt` | String | no | created date |
| `updatedAt` | String | no | updated date |

### `SocialMediaPinterestAccountResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`PinterestOAuthAccountSchema`](#pinterestoauthaccountschema) | no | Requested Results |

### `SocialMediaTagSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `tag` | String | no | Tag Name |
| `locationId` | String | no | Location Id |
| `_id` | String | no | MongoDB document ID |
| `createdBy` | String | no | Created By User Id |
| `deleted` | bool | no | Deleted boolean value |
| `createdAt` | String | no | Date when the record was created |
| `updatedAt` | String | no | Date when the record was last updated |

### `SocialMediaThreadsAccountResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`SocialMediaThreadsAccountSchema`](#socialmediathreadsaccountschema) | no | Requested Results |

### `SocialMediaThreadsAccountSchema`

_No fields defined in the spec._

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
| `_id` | String | no | MongoDB document ID of the social media account |
| `oAuthId` | String | no | OAuth provider account identifier |
| `oldId` | String | no | Legacy account identifier for backward compatibility |
| `locationId` | String | no | Location ID associated with this account |
| `originId` | String | no | Original platform-specific account identifier |
| `platform` | JSON | no | Social media platform name |
| `type` | JSON | no | type must be one of the following values: profile, business |
| `name` | String | no | Display name of the account |
| `avatar` | String | no | Avatar or profile picture URL |
| `meta` | JSON | no | Additional metadata for the account |
| `active` | bool | no | Indicates if the account is currently active |
| `deleted` | bool | no | Indicates if the account has been deleted |
| `createdAt` | String | no | created date |
| `updatedAt` | String | no | updated date |

### `SocialMediaTiktokBusinessAccountResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`TikTokOAuthAccountSchema`](#tiktokoauthaccountschema) | no | Requested Results |

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
| `_id` | String | no | MongoDB document ID of the social media account |
| `oAuthId` | String | no | OAuth provider account identifier |
| `oldId` | String | no | Legacy account identifier for backward compatibility |
| `locationId` | String | no | Location ID associated with this account |
| `originId` | String | no | Original platform-specific account identifier |
| `platform` | JSON | no | Social media platform name |
| `type` | JSON | no | Type of account (e.g., location, page, profile) |
| `name` | String | no | Display name of the account |
| `avatar` | String | no | Avatar or profile picture URL |
| `meta` | JSON | no | Additional metadata for the account |
| `active` | bool | no | Indicates if the account is currently active |
| `deleted` | bool | no | Indicates if the account has been deleted |
| `createdAt` | String | no | created date |
| `updatedAt` | String | no | updated date |

### `SocialMediaYouTubeAccountResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |
| `results` | [`YouTubeOAuthAccountSchema`](#youtubeoauthaccountschema) | no | Requested Results |

### `StartDateSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `startDate` | [`DateSchema`](#dateschema) | no | Start Date |
| `startTime` | [`TimeSchema`](#timeschema) | no | Start Time |

### `StartEditSessionDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |

### `StartEditSessionResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | no | A message indicating the result of the operation. |
| `sessionId` | String | no | The ID of the edit session. |
| `itemCount` | f64 | no | Number of items staged for editing. |

### `TikTokOAuthAccountSchema`

_No fields defined in the spec._

### `TiktokPostSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `privacyLevel` | String — `PUBLIC_TO_EVERYONE`, `MUTUAL_FOLLOW_FRIENDS`, `SELF_ONLY` | **yes** | Privacy level controlling who can view the video. **Available Values:** - `PUBLIC_TO_EVERYONE` - Anyone can view (default) - `MUTUAL_FOLLOW_FRIENDS` - Only mutual followers can view - `SELF_ONLY` - On… |
| `promoteOtherBrand` | bool | no | Indicates if the video promotes a third-party brand or product. **Required:** Must be `true` if `videoDisclosure` is enabled and you're promoting another brand. |
| `enableComment` | bool | no | Allow users to comment on the video. Default is determined by account settings. |
| `enableDuet` | bool | no | Allow users to create Duet videos with your content. **Duet:** Side-by-side video featuring your content and the creator's reaction/addition. |
| `enableStitch` | bool | no | Allow users to create Stitch videos with your content. **Stitch:** Clips up to 5 seconds of your video that creators can use in their own videos. |
| `videoDisclosure` | bool | no | Enable branded content disclosure. Required when video is promotional content. **Validations:** - Cannot be `true` if `privacyLevel` is `SELF_ONLY` - If enabled, at least one of `promoteYourBrand` or … |
| `promoteYourBrand` | bool | no | Indicates if the video promotes your own brand or product. **Required:** Must be `true` if `videoDisclosure` is enabled and you're promoting your own brand. |

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
| `hours` | f64 | **yes** | Hour component of the time (0-23) |
| `minutes` | f64 | **yes** | Minute component of the time (0-59) |
| `seconds` | f64 | **yes** | Second component of the time (0-59) |

### `TimeSlotDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `dayOfWeek` | f64 | **yes** | Day of the week (0-6) |
| `time` | String | **yes** | Time in HH:mm format |

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

### `UpdateCategoryQueueDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `skipLegacyWatermark` | bool | no | Skip legacy watermark cleanup when rescheduling posts |
| `status` | JSON | no | Status of the Queue |
| `skipDateTime` | String | no | Skip Date Time in ISO format |
| `timeSlots` | Vec<TimeSlotDTO> | no | — |
| `enableFuturePosts` | bool | no | Enable posting future content. Automatically Queue any New Posts Created in this Category. |
| `prioritizeNewContent` | bool | no | Prioritize new content over older content. When true, new items added via directToQueue will be placed at the top of the queue. |

### `UpdateCategoryQueueResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | no | A message indicating the result of the operation. Examples: "Queue updated successfully.", "Queue paused successfully.", "Queue activated successfully.", "Queue deleted successfully." |
| `queue` | [`CategoryQueueDTO`](#categoryqueuedto) | no | The updated queue. |

### `UpdatePostSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success or Failure |
| `statusCode` | f64 | **yes** | Status Code |
| `message` | String | **yes** | Message |

### `UpdateQueueItemDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `sessionId` | String | no | Edit session ID |
| `modifiedPostPayload` | [`QueueModifiedPostDTO`](#queuemodifiedpostdto) | no | Modifications to the original post |
| `newOrder` | JSON | no | New order value or position keyword (cyclic-aware). Accepts: - A number: explicit order value calculated by FE as midpoint between adjacent items - "top": place at cyclic top (first to be scheduled ne… |
| `variations` | Vec<VariationInputDTO> | no | Variations |
| `primaryImage` | String | no | Primary media URL (image) |

### `UpdateQueueItemResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | no | A message indicating the result of the operation. |
| `queueItem` | [`CreatedQueueItemWithVariationsDTO`](#createdqueueitemwithvariationsdto) | no | The updated queue item. |
| `updatedSlots` | Vec<UpdatedSlotInfoDTO> | no | Updated slot information for items affected by reorder operation |
| `totalPostsChanged` | f64 | no | Number of unique posts that had their slots changed |

### `UpdateTagDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `tagIds` | Vec<String> | **yes** | Array of Tag Ids |

### `UpdatedSlotInfoDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `itemId` | String | no | The ID of the queue item |
| `scheduledDateTime` | String | no | The updated scheduled date/time for this item |
| `isSkipped` | bool | no | Indicates if this time slot is skipped |

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
| `filePath` | String | no | File path of uploaded CSV |
| `rowsCount` | f64 | no | Number of rows in the CSV |
| `fileName` | String | no | Name of the uploaded file |
| `fileSize` | f64 | no | Size of the file in bytes |
| `csvFileType` | String — `basic`, `advance` | no | CSV file type |

### `VariationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | The ID of the variation. |
| `content` | String | no | The text content of the variation. |
| `mentions` | Vec<JSON> | no | Platform-specific mentions within the content (e.g., @username references). |
| `ogTags` | [`OgTagsDTO`](#ogtagsdto) | no | Open Graph tags for link previews. |

### `VariationInputDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `content` | String | no | The text content of the variation. |
| `mentions` | Vec<JSON> | no | Platform-specific mentions within the content (e.g., @username references). |
| `ogTags` | [`OgTagsInputDTO`](#ogtagsinputdto) | no | Open Graph tags for link previews. |

### `WrappedCloneQueueItemResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |
| `statusCode` | f64 | **yes** | — |
| `results` | [`CloneQueueItemResponseDTO`](#clonequeueitemresponsedto) | **yes** | — |
| `traceId` | String | no | — |

### `WrappedCreateCategoryQueueResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |
| `statusCode` | f64 | **yes** | — |
| `results` | [`CreateCategoryQueueResponseDTO`](#createcategoryqueueresponsedto) | **yes** | — |
| `traceId` | String | no | — |

### `WrappedCreateQueueItemResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |
| `statusCode` | f64 | **yes** | — |
| `results` | [`CreateQueueItemResponseDTO`](#createqueueitemresponsedto) | **yes** | — |
| `traceId` | String | no | — |

### `WrappedDeleteActivePostResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |
| `statusCode` | f64 | **yes** | — |
| `results` | [`DeleteActivePostResponseDTO`](#deleteactivepostresponsedto) | **yes** | — |
| `traceId` | String | no | — |

### `WrappedDiscardEditSessionResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |
| `statusCode` | f64 | **yes** | — |
| `results` | [`DiscardEditSessionResponseDTO`](#discardeditsessionresponsedto) | **yes** | — |
| `traceId` | String | no | — |

### `WrappedEditSessionCalendarResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |
| `statusCode` | f64 | **yes** | — |
| `results` | [`EditSessionCalendarResponseDTO`](#editsessioncalendarresponsedto) | **yes** | — |
| `traceId` | String | no | — |

### `WrappedFetchAvailableCategoriesResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |
| `statusCode` | f64 | **yes** | — |
| `results` | [`FetchAvailableCategoriesResponseDTO`](#fetchavailablecategoriesresponsedto) | **yes** | — |
| `traceId` | String | no | — |

### `WrappedFetchCalendarListResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |
| `statusCode` | f64 | **yes** | — |
| `results` | [`FetchCalendarListResponseDTO`](#fetchcalendarlistresponsedto) | **yes** | — |
| `traceId` | String | no | — |

### `WrappedFetchCategoryQueuesResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |
| `statusCode` | f64 | **yes** | — |
| `results` | [`FetchCategoryQueuesResponseDTO`](#fetchcategoryqueuesresponsedto) | **yes** | — |
| `traceId` | String | no | — |

### `WrappedFetchQueueByIdResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |
| `statusCode` | f64 | **yes** | — |
| `results` | [`FetchQueueByIdResponseDTO`](#fetchqueuebyidresponsedto) | **yes** | — |
| `traceId` | String | no | — |

### `WrappedFetchQueueItemsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |
| `statusCode` | f64 | **yes** | — |
| `results` | [`FetchQueueItemsResponseDTO`](#fetchqueueitemsresponsedto) | **yes** | — |
| `traceId` | String | no | — |

### `WrappedFetchSlotsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |
| `statusCode` | f64 | **yes** | — |
| `results` | [`FetchSlotsResponseDTO`](#fetchslotsresponsedto) | **yes** | — |
| `traceId` | String | no | — |

### `WrappedGeneralSuccessResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |
| `statusCode` | f64 | **yes** | — |
| `results` | [`GeneralSuccessResponseDTO`](#generalsuccessresponsedto) | **yes** | — |
| `traceId` | String | no | — |

### `WrappedResetQueueItemResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |
| `statusCode` | f64 | **yes** | — |
| `results` | [`ResetQueueItemResponseDTO`](#resetqueueitemresponsedto) | **yes** | — |
| `traceId` | String | no | — |

### `WrappedSaveEditSessionResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |
| `statusCode` | f64 | **yes** | — |
| `results` | [`SaveEditSessionResponseDTO`](#saveeditsessionresponsedto) | **yes** | — |
| `traceId` | String | no | — |

### `WrappedStartEditSessionResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |
| `statusCode` | f64 | **yes** | — |
| `results` | [`StartEditSessionResponseDTO`](#starteditsessionresponsedto) | **yes** | — |
| `traceId` | String | no | — |

### `WrappedUpdateCategoryQueueResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |
| `statusCode` | f64 | **yes** | — |
| `results` | [`UpdateCategoryQueueResponseDTO`](#updatecategoryqueueresponsedto) | **yes** | — |
| `traceId` | String | no | — |

### `WrappedUpdateQueueItemResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | — |
| `statusCode` | f64 | **yes** | — |
| `results` | [`UpdateQueueItemResponseDTO`](#updatequeueitemresponsedto) | **yes** | — |
| `traceId` | String | no | — |

### `YouTubeOAuthAccountSchema`

_No fields defined in the spec._

### `YoutubePostSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | no | Video title displayed on YouTube. **Max length:** 100 characters **Best Practices:** - Include relevant keywords - Be descriptive but concise - Avoid clickbait |
| `privacyLevel` | String — `private`, `public`, `unlisted` | no | Video visibility/privacy setting. **Available Values:** - `public` - Anyone can search and view - `unlisted` - Only people with the link can view - `private` - Only you can view |
| `type` | String — `video`, `short` | **yes** | YouTube video format type. **Available Types:** - `video` - Standard YouTube video (landscape, any duration) - `short` - YouTube Shorts (vertical, up to 60 seconds) **Required field.** |

### `YoutubeProfileSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Id |
| `name` | String | no | Name of account |
| `username` | String | no | Username of account |
| `avatar` | String | no | Avatar of profile account |
| `verified` | bool | no | Is verified |
| `isConnected` | bool | no | Is connected |
| `type` | String | no | Youtube Account Type |

