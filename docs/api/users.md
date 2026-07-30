# `users`

**7** operations / **10** models in API v2 · **6** operations / **9** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `users` cargo feature on `ghl-sdk`, then call any of the 7 generated methods on `ghl.users()`:

```toml
ghl-sdk = { version = "0.4", features = ["users"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/users/` | Get User by Location | `get_user_by_location()` | `users.get_users` |
| `POST` | `/users/` | Create User | `create_user()` | `users.post_users` |
| `GET` | `/users/search` | Search Users | `search_users()` | `users.get_users_search` |
| `POST` | `/users/search/filter-by-email` | Filter Users by Email | `filter_users_by_email()` | `users.post_users_search_filter_by_email` |
| `DELETE` | `/users/{userId}` | Delete User | `delete_user()` | `users.delete_users_by_userId` |
| `GET` | `/users/{userId}` | Get User | `get_user()` | `users.get_users_by_userId` |
| `PUT` | `/users/{userId}` | Update User | `update_user()` | `users.put_users_by_userId` |

### Endpoint details — v2

#### `GET /users/`

**Get User by Location**

Deprecated. Use `GET /users/search` instead. Pass `locationId` as a query parameter to filter results by location, along with the required `companyId` and other search filters as needed.

Operation id: `users.get_users` · `Version: 2021-07-28` · Scopes: `users.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Response*: [`LocationSuccessfulResponseDto`](#locationsuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::users::GetUserByLocationParams;

let params = GetUserByLocationParams::new("locationId");
let out = ghl.users().get_user_by_location(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "users.get_users",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /users/`

**Create User**

Operation id: `users.post_users` · `Version: 2021-07-28` · Scopes: `users.write`

*Request body*: [`CreateUserDto`](#createuserdto)

*Response*: [`UserSuccessfulResponseDto`](#usersuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.users().create_user(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "users.post_users",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /users/search`

**Search Users**

Operation id: `users.get_users_search` · `Version: 2021-07-28` · Scopes: `users.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | Company ID in which the search needs to be performed |
| `query` | string | no | The search term for the user is matched based on the user full name, email or phone |
| `skip` | string | no | No of results to be skipped before returning the result |
| `limit` | string | no | No of results to be limited before returning the result |
| `locationId` | string | no | Location ID in which the search needs to be performed |
| `type` | string | no | Type of the users to be filtered in the search |
| `role` | string | no | Role of the users to be filtered in the search |
| `ids` | string | no | List of User IDs to be filtered in the search |
| `sort` | string | no | The field on which sort is applied in which the results need to be sorted. Default is based on the first and last name |
| `sortDirection` | string | no | The direction in which the results need to be sorted |
| `enabled2waySync` | boolean | no | — |

*Response*: [`SearchUserSuccessfulResponseDto`](#searchusersuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::users::SearchUsersParams;

let params = SearchUsersParams::new("companyId");
let out = ghl.users().search_users(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "users.get_users_search",
    "query": {
      "companyId": "<companyId>"
    }
  }
}
```

</details>

#### `POST /users/search/filter-by-email`

**Filter Users by Email**

Filter users by company ID, deleted status, and email array

Operation id: `users.post_users_search_filter_by_email` · `Version: 2021-07-28` · Scopes: `users.readonly`

*Request body*: [`FilterByEmailDto`](#filterbyemaildto)

*Response*: [`SearchUserSuccessfulResponseDto`](#searchusersuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.users().filter_users_by_email(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "users.post_users_search_filter_by_email",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /users/{userId}`

**Delete User**

Operation id: `users.delete_users_by_userId` · `Version: 2021-07-28` · Scopes: `users.write`

*Response*: [`DeleteUserSuccessfulResponseDto`](#deleteusersuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.users().delete_user().await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "users.delete_users_by_userId"
  }
}
```

</details>

#### `GET /users/{userId}`

**Get User**

Operation id: `users.get_users_by_userId` · `Version: 2021-07-28` · Scopes: `users.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `userId` | string | **yes** | User Id |

*Response*: [`UserSuccessfulResponseDto`](#usersuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.users().get_user(&userId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "users.get_users_by_userId",
    "path_params": {
      "userId": "<userId>"
    }
  }
}
```

</details>

#### `PUT /users/{userId}`

**Update User**

Operation id: `users.put_users_by_userId` · `Version: 2021-07-28` · Scopes: `users.write`

*Request body*: [`UpdateUserDto`](#updateuserdto)

*Response*: [`UserSuccessfulResponseDto`](#usersuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.users().update_user(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "users.put_users_by_userId",
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
| `POST` | `/users/` | Create User | `v3:users.post_users` |
| `GET` | `/users/search` | Search Users | `v3:users.get_users_search` |
| `POST` | `/users/search/filter-by-email` | Filter Users by Email | `v3:users.post_users_search_filter_by_email` |
| `DELETE` | `/users/{userId}` | Delete User | `v3:users.delete_users_by_userId` |
| `GET` | `/users/{userId}` | Get User | `v3:users.get_users_by_userId` |
| `PUT` | `/users/{userId}` | Update User | `v3:users.put_users_by_userId` |

### Endpoint details — v3

#### `POST /users/`

**Create User**

Operation id: `v3:users.post_users` · `Version: v3` · Scopes: `users.write`

*Request body*: [`CreateUserDto`](#createuserdto)

*Response*: [`UserSuccessfulResponseDto`](#usersuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:users.post_users",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /users/search`

**Search Users**

Operation id: `v3:users.get_users_search` · `Version: v3` · Scopes: `users.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | Company ID in which the search needs to be performed |
| `query` | string | no | The search term for the user is matched based on the user full name, email or phone |
| `skip` | string | no | No of results to be skipped before returning the result |
| `limit` | string | no | No of results to be limited before returning the result |
| `locationId` | string | no | Location ID in which the search needs to be performed |
| `type` | string | no | Type of the users to be filtered in the search |
| `role` | string | no | Role of the users to be filtered in the search |
| `ids` | string | no | List of User IDs to be filtered in the search |
| `sort` | string | no | The field on which sort is applied in which the results need to be sorted. Default is based on the first and last name |
| `sortDirection` | string | no | The direction in which the results need to be sorted |
| `enabled2waySync` | boolean | no | Filter users by whether 2-way sync is enabled |

*Response*: [`SearchUserSuccessfulResponseDto`](#searchusersuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:users.get_users_search",
    "query": {
      "companyId": "<companyId>"
    }
  }
}
```

</details>

#### `POST /users/search/filter-by-email`

**Filter Users by Email**

Filter users by company ID, deleted status, and email array

Operation id: `v3:users.post_users_search_filter_by_email` · `Version: v3` · Scopes: `users.readonly`

*Request body*: [`FilterByEmailDto`](#filterbyemaildto)

*Response*: [`SearchUserSuccessfulResponseDto`](#searchusersuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:users.post_users_search_filter_by_email",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /users/{userId}`

**Delete User**

Operation id: `v3:users.delete_users_by_userId` · `Version: v3` · Scopes: `users.write`

*Response*: [`DeleteUserSuccessfulResponseV3Dto`](#deleteusersuccessfulresponsev3dto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:users.delete_users_by_userId"
  }
}
```

</details>

#### `GET /users/{userId}`

**Get User**

Operation id: `v3:users.get_users_by_userId` · `Version: v3` · Scopes: `users.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `userId` | string | **yes** | User Id |

*Response*: [`UserSuccessfulResponseDto`](#usersuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:users.get_users_by_userId",
    "path_params": {
      "userId": "<userId>"
    }
  }
}
```

</details>

#### `PUT /users/{userId}`

**Update User**

Operation id: `v3:users.put_users_by_userId` · `Version: v3` · Scopes: `users.write`

*Request body*: [`UpdateUserDto`](#updateuserdto)

*Response*: [`UserSuccessfulResponseDto`](#usersuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:users.put_users_by_userId",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::users::*` (enable the `users` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/users/).

### `CreateUserDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `companyId` | String | **yes** | — |
| `firstName` | String | **yes** | — |
| `lastName` | String | **yes** | — |
| `email` | String | **yes** | — |
| `password` | String | **yes** | — |
| `phone` | String | no | — |
| `type` | String | **yes** | — |
| `role` | String | **yes** | — |
| `locationIds` | Vec<String> | **yes** | — |
| `permissions` | [`PermissionsDto`](#permissionsdto) | no | — |
| `scopes` | Vec<String (enum)> | no | Scopes allowed for users. Only scopes that have been passed will be enabled. Note:- If passed empty all the scopes will be get disabled |
| `scopesAssignedToOnly` | Vec<String (enum)> | no | Assigned Scopes allowed for users. Only scopes that have been passed will be enabled. If passed empty all the assigned scopes will be get disabled |
| `profilePhoto` | String | no | — |
| `twilioPhone` | JSON | no | Per-location inbound Twilio number in E.164 format, keyed by location id (Call and Voicemail Inbound Number for direct Twilio, not LC Phone). Replacement semantics: if you send twilioPhone in the requ… |
| `platformLanguage` | String — `en_US`, `es`, `fr_CA`, `fr_FR`, `nl`, `de`, `pt_PT`, `pt_BR`, `it`, `sv`, `da`, `fi`, `no` | no | Platform language preference for the user |

### `DeleteUserSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeded` | bool | no | — |
| `message` | String | no | — |

### `FilterByEmailDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `companyId` | String | **yes** | Company ID to filter users |
| `emails` | String | **yes** | Comma-separated list of email addresses to filter users |
| `deleted` | bool | no | Filter deleted users |
| `skip` | String | no | No of results to be skipped before returning the result |
| `limit` | String | no | No of results to be limited before returning the result |
| `projection` | String | no | Projection fields to return. Use "all" for all fields, or specify comma-separated field names. Default returns only id and email |

### `LocationSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `users` | Vec<UserSchema> | no | — |

### `PermissionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `campaignsEnabled` | bool | no | — |
| `campaignsReadOnly` | bool | no | — |
| `contactsEnabled` | bool | no | — |
| `workflowsEnabled` | bool | no | — |
| `workflowsReadOnly` | bool | no | — |
| `triggersEnabled` | bool | no | — |
| `funnelsEnabled` | bool | no | — |
| `websitesEnabled` | bool | no | — |
| `opportunitiesEnabled` | bool | no | — |
| `dashboardStatsEnabled` | bool | no | — |
| `bulkRequestsEnabled` | bool | no | — |
| `appointmentsEnabled` | bool | no | — |
| `reviewsEnabled` | bool | no | — |
| `onlineListingsEnabled` | bool | no | — |
| `phoneCallEnabled` | bool | no | — |
| `conversationsEnabled` | bool | no | — |
| `assignedDataOnly` | bool | no | — |
| `adwordsReportingEnabled` | bool | no | — |
| `membershipEnabled` | bool | no | — |
| `facebookAdsReportingEnabled` | bool | no | — |
| `attributionsReportingEnabled` | bool | no | — |
| `settingsEnabled` | bool | no | — |
| `tagsEnabled` | bool | no | — |
| `leadValueEnabled` | bool | no | — |
| `marketingEnabled` | bool | no | — |
| `agentReportingEnabled` | bool | no | — |
| `botService` | bool | no | — |
| `socialPlanner` | bool | no | — |
| `bloggingEnabled` | bool | no | — |
| `invoiceEnabled` | bool | no | — |
| `affiliateManagerEnabled` | bool | no | — |
| `contentAiEnabled` | bool | no | — |
| `refundsEnabled` | bool | no | — |
| `recordPaymentEnabled` | bool | no | — |
| `cancelSubscriptionEnabled` | bool | no | — |
| `paymentsEnabled` | bool | no | — |
| `communitiesEnabled` | bool | no | — |
| `exportPaymentsEnabled` | bool | no | — |

### `RoleSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String | no | — |
| `role` | String | no | — |
| `locationIds` | Vec<String> | no | — |
| `restrictSubAccount` | bool | no | — |

### `SearchUserSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `users` | Vec<UserSchema> | no | — |
| `count` | f64 | no | — |

### `UpdateUserDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `firstName` | String | no | — |
| `lastName` | String | no | — |
| `email` | String | no | Email update is no longer supported due to security reasons. |
| `password` | String | no | — |
| `phone` | String | no | — |
| `type` | String | no | — |
| `role` | String | no | — |
| `companyId` | String | no | Company/Agency Id. Required for Agency Level access |
| `locationIds` | Vec<String> | no | — |
| `permissions` | [`PermissionsDto`](#permissionsdto) | no | — |
| `scopes` | Vec<String (enum)> | no | Scopes allowed for users. Only scopes that have been passed will be enabled. If passed empty all the scopes will be get disabled |
| `scopesAssignedToOnly` | Vec<String (enum)> | no | Assigned Scopes allowed for users. Only scopes that have been passed will be enabled. If passed empty all the assigned scopes will be get disabled |
| `profilePhoto` | String | no | — |
| `twilioPhone` | JSON | no | Per-location inbound Twilio number in E.164 format, keyed by location id (Call and Voicemail Inbound Number for direct Twilio, not LC Phone). Replacement semantics: if you send twilioPhone in the requ… |
| `platformLanguage` | String — `en_US`, `es`, `fr_CA`, `fr_FR`, `nl`, `de`, `pt_PT`, `pt_BR`, `it`, `sv`, `da`, `fi`, `no` | no | Platform language preference for the user |

### `UserSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `firstName` | String | no | — |
| `lastName` | String | no | — |
| `email` | String | no | — |
| `phone` | String | no | — |
| `extension` | String | no | — |
| `permissions` | [`PermissionsDto`](#permissionsdto) | no | — |
| `scopes` | String — 189 values ([shared](shared-enums.md)) | no | — |
| `roles` | [`RoleSchema`](#roleschema) | no | — |
| `deleted` | bool | no | — |
| `lcPhone` | JSON | no | LC Phone Inbound Phone Numbers |
| `platformLanguage` | String — `en_US`, `es`, `fr_CA`, `fr_FR`, `nl`, `de`, `pt_PT`, `pt_BR`, `it`, `sv`, `da`, `fi`, `no` | no | Platform language preference for the user |

### `UserSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `firstName` | String | no | — |
| `lastName` | String | no | — |
| `email` | String | no | — |
| `phone` | String | no | — |
| `extension` | String | no | — |
| `permissions` | [`PermissionsDto`](#permissionsdto) | no | — |
| `scopes` | String — 189 values ([shared](shared-enums.md)) | no | — |
| `roles` | [`RoleSchema`](#roleschema) | no | — |
| `lcPhone` | JSON | no | LC Phone Inbound Phone Numbers |
| `platformLanguage` | String — `en_US`, `es`, `fr_CA`, `fr_FR`, `nl`, `de`, `pt_PT`, `pt_BR`, `it`, `sv`, `da`, `fi`, `no` | no | Platform language preference for the user |

## Data models — API v3

In Rust: `ghl_models::v3::users::*` (enable the `users` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/users/).

### `CreateUserDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `companyId` | String | **yes** | Company/Agency ID to associate the user with |
| `email` | String | **yes** | Email address of the user (used for login) |
| `password` | String | **yes** | Password for the user account. All passwords will be required to meet the following criteria: - Minimum 12 characters - At least one uppercase letter (A–Z) - At least one lowercase letter (a–z) - At l… |
| `phone` | String | no | Phone number of the user in E.164 format |
| `type` | String | **yes** | User account type (account for sub-account users, agency for agency-level users) |
| `role` | String | **yes** | User role within the account (admin or user) |
| `locationIds` | Vec<String> | **yes** | List of location IDs to assign to the user |
| `permissions` | [`PermissionsDto`](#permissionsdto) | no | User permissions controlling access to various features |
| `scopes` | Vec<String (enum)> | no | Scopes allowed for users. Only scopes that have been passed will be enabled. Note:- If passed empty all the scopes will be get disabled |
| `scopesAssignedToOnly` | Vec<String (enum)> | no | Assigned Scopes allowed for users. Only scopes that have been passed will be enabled. If passed empty all the assigned scopes will be get disabled |
| `profilePhoto` | String | no | URL of the user profile photo |
| `twilioPhone` | JSON | no | Per-location inbound Twilio number in E.164 format, keyed by location id (Call and Voicemail Inbound Number for direct Twilio, not LC Phone). Replacement semantics: if you send twilioPhone in the requ… |
| `platformLanguage` | String — `en_US`, `es`, `fr_CA`, `fr_FR`, `nl`, `de`, `pt_PT`, `pt_BR`, `it`, `sv`, `da`, `fi`, `no` | no | Platform language preference for the user |
| `firstName` | String | **yes** | First name of the user |
| `lastName` | String | **yes** | Last name of the user |

### `DeleteUserSuccessfulResponseV3Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `succeeded` | bool | no | Indicates whether the user deletion was queued successfully |
| `message` | String | no | Message describing the result of the deletion request |

### `FilterByEmailDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `companyId` | String | **yes** | Company ID to filter users |
| `emails` | String | **yes** | Comma-separated list of email addresses to filter users |
| `deleted` | bool | no | Filter deleted users |
| `skip` | String | no | No of results to be skipped before returning the result |
| `limit` | String | no | No of results to be limited before returning the result |
| `projection` | String | no | Projection fields to return. Use "all" for all fields, or specify comma-separated field names. Default returns only id and email |

### `PermissionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `campaignsEnabled` | bool | no | Whether campaigns are enabled for this user |
| `campaignsReadOnly` | bool | no | Whether campaigns are in read-only mode for this user |
| `contactsEnabled` | bool | no | Whether contacts are enabled for this user |
| `workflowsEnabled` | bool | no | Whether workflows are enabled for this user |
| `workflowsReadOnly` | bool | no | Whether workflows are in read-only mode for this user |
| `triggersEnabled` | bool | no | Whether triggers are enabled for this user |
| `funnelsEnabled` | bool | no | Whether funnels are enabled for this user |
| `websitesEnabled` | bool | no | Whether websites are enabled for this user |
| `opportunitiesEnabled` | bool | no | Whether opportunities are enabled for this user |
| `dashboardStatsEnabled` | bool | no | Whether dashboard statistics are enabled for this user |
| `bulkRequestsEnabled` | bool | no | Whether bulk requests are enabled for this user |
| `appointmentsEnabled` | bool | no | Whether appointments are enabled for this user |
| `reviewsEnabled` | bool | no | Whether reviews are enabled for this user |
| `onlineListingsEnabled` | bool | no | Whether online listings are enabled for this user |
| `phoneCallEnabled` | bool | no | Whether phone calls are enabled for this user |
| `conversationsEnabled` | bool | no | Whether conversations are enabled for this user |
| `assignedDataOnly` | bool | no | Whether the user can only access data assigned to them |
| `adwordsReportingEnabled` | bool | no | Whether AdWords reporting is enabled for this user |
| `membershipEnabled` | bool | no | Whether membership features are enabled for this user |
| `facebookAdsReportingEnabled` | bool | no | Whether Facebook Ads reporting is enabled for this user |
| `attributionsReportingEnabled` | bool | no | Whether attributions reporting is enabled for this user |
| `settingsEnabled` | bool | no | Whether settings are enabled for this user |
| `tagsEnabled` | bool | no | Whether tags are enabled for this user |
| `leadValueEnabled` | bool | no | Whether lead value features are enabled for this user |
| `marketingEnabled` | bool | no | Whether marketing features are enabled for this user |
| `agentReportingEnabled` | bool | no | Whether agent reporting is enabled for this user |
| `botService` | bool | no | Whether the bot service is enabled for this user |
| `socialPlanner` | bool | no | Whether the social planner is enabled for this user |
| `bloggingEnabled` | bool | no | Whether blogging is enabled for this user |
| `invoiceEnabled` | bool | no | Whether invoices are enabled for this user |
| `affiliateManagerEnabled` | bool | no | Whether the affiliate manager is enabled for this user |
| `contentAiEnabled` | bool | no | Whether Content AI is enabled for this user |
| `refundsEnabled` | bool | no | Whether refunds are enabled for this user |
| `recordPaymentEnabled` | bool | no | Whether recording payments is enabled for this user |
| `cancelSubscriptionEnabled` | bool | no | Whether cancelling subscriptions is enabled for this user |
| `paymentsEnabled` | bool | no | Whether payments are enabled for this user |
| `communitiesEnabled` | bool | no | Whether communities are enabled for this user |
| `exportPaymentsEnabled` | bool | no | Whether exporting payments is enabled for this user |

### `RoleSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String | no | User account type (account for sub-account users, agency for agency-level users) |
| `role` | String | no | User role within the account (admin or user) |
| `locationIds` | Vec<String> | no | List of location IDs the user has access to |
| `restrictSubAccount` | bool | no | Whether the user is restricted to specific sub-accounts only |

### `SearchUserSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `users` | Vec<UserSchema> | no | List of users matching the search criteria |
| `count` | f64 | no | Total number of users matching the search criteria |

### `UpdateUserDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `firstName` | String | no | First name of the user |
| `lastName` | String | no | Last name of the user |
| `email` | String | no | Email update is no longer supported due to security reasons. |
| `password` | String | no | New password for the user account. All passwords will be required to meet the following criteria: - Minimum 12 characters - At least one uppercase letter (A–Z) - At least one lowercase letter (a–z) - … |
| `phone` | String | no | Phone number of the user in E.164 format |
| `type` | String | no | User account type (account for sub-account users, agency for agency-level users) |
| `role` | String | no | User role within the account (admin or user) |
| `companyId` | String | no | Company/Agency Id. Required for Agency Level access |
| `locationIds` | Vec<String> | no | List of sub-account location IDs the user should have access to |
| `permissions` | [`PermissionsDto`](#permissionsdto) | no | User permissions controlling access to various features |
| `scopes` | Vec<String (enum)> | no | Scopes allowed for users. Only scopes that have been passed will be enabled. If passed empty all the scopes will be get disabled |
| `scopesAssignedToOnly` | Vec<String (enum)> | no | Assigned Scopes allowed for users. Only scopes that have been passed will be enabled. If passed empty all the assigned scopes will be get disabled |
| `profilePhoto` | String | no | URL of the user profile photo |
| `twilioPhone` | JSON | no | Per-location inbound Twilio number in E.164 format, keyed by location id (Call and Voicemail Inbound Number for direct Twilio, not LC Phone). Replacement semantics: if you send twilioPhone in the requ… |
| `platformLanguage` | String — `en_US`, `es`, `fr_CA`, `fr_FR`, `nl`, `de`, `pt_PT`, `pt_BR`, `it`, `sv`, `da`, `fi`, `no` | no | Platform language preference for the user |

### `UserSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Unique identifier of the user |
| `name` | String | no | Full name of the user |
| `firstName` | String | no | First name of the user |
| `lastName` | String | no | Last name of the user |
| `email` | String | no | Email address of the user |
| `phone` | String | no | Phone number of the user |
| `extension` | String | no | Phone extension of the user |
| `permissions` | [`PermissionsDto`](#permissionsdto) | no | User permissions controlling access to various features |
| `scopes` | String — 194 values ([shared](shared-enums.md)) | no | List of OAuth scopes granted to this user |
| `roles` | [`RoleSchema`](#roleschema) | no | Role and access configuration for the user |
| `deleted` | bool | no | Whether the user has been deleted |
| `lcPhone` | JSON | no | LC Phone Inbound Phone Numbers |
| `platformLanguage` | String — `en_US`, `es`, `fr_CA`, `fr_FR`, `nl`, `de`, `pt_PT`, `pt_BR`, `it`, `sv`, `da`, `fi`, `no` | no | Platform language preference for the user |

### `UserSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Unique identifier of the user |
| `name` | String | no | Full name of the user |
| `firstName` | String | no | First name of the user |
| `lastName` | String | no | Last name of the user |
| `email` | String | no | Email address of the user |
| `phone` | String | no | Phone number of the user |
| `extension` | String | no | Phone extension of the user |
| `permissions` | [`PermissionsDto`](#permissionsdto) | no | User permissions controlling access to various features |
| `scopes` | String — 194 values ([shared](shared-enums.md)) | no | List of OAuth scopes granted to this user |
| `roles` | [`RoleSchema`](#roleschema) | no | Role and access configuration for the user |
| `lcPhone` | JSON | no | LC Phone Inbound Phone Numbers |
| `platformLanguage` | String — `en_US`, `es`, `fr_CA`, `fr_FR`, `nl`, `de`, `pt_PT`, `pt_BR`, `it`, `sv`, `da`, `fi`, `no` | no | Platform language preference for the user |

