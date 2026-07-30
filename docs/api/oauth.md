# `oauth`

**3** operations / **6** models in API v2 · **3** operations / **12** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `oauth` cargo feature on `ghl-sdk`, then call any of the 3 generated methods on `ghl.oauth()`:

```toml
ghl-sdk = { version = "0.4", features = ["oauth"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/oauth/installedLocations` | Get Location where app is installed | `get_location_where_app_is_installed()` | `oauth.get_oauth_installedLocations` |
| `POST` | `/oauth/locationToken` | Get Location Access Token from Agency Token | `get_location_access_token_from_agency_token()` | `oauth.post_oauth_locationToken` |
| `POST` | `/oauth/token` | Get Access Token | `get_access_token()` | `oauth.post_oauth_token` |

### Endpoint details — v2

#### `GET /oauth/installedLocations`

**Get Location where app is installed**

This API allows you fetch location where app is installed upon

Operation id: `oauth.get_oauth_installedLocations` · `Version: 2021-07-28` · Scopes: `oauth.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `skip` | string | no | Parameter to skip the number installed locations |
| `limit` | string | no | Parameter to limit the number installed locations |
| `query` | string | no | Parameter to search for the installed location by name |
| `isInstalled` | boolean | no | Filters out location which are installed for specified app under the specified company |
| `companyId` | string | **yes** | Parameter to search by the companyId |
| `appId` | string | **yes** | Parameter to search by the appId |
| `versionId` | string | no | VersionId of the app |
| `onTrial` | boolean | no | Filters out locations which are installed for specified app in trial mode |
| `planId` | string | no | Filters out location which are installed for specified app under the specified planId |
| `locationId` | string | no | locationId |

*Response*: [`GetInstalledLocationsSuccessfulResponseDto`](#getinstalledlocationssuccessfulresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::oauth::GetLocationWhereAppIsInstalledParams;

let params = GetLocationWhereAppIsInstalledParams::new("companyId", "appId");
let out = ghl.oauth().get_location_where_app_is_installed(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "oauth.get_oauth_installedLocations",
    "query": {
      "companyId": "<companyId>",
      "appId": "<appId>"
    }
  }
}
```

</details>

#### `POST /oauth/locationToken`

**Get Location Access Token from Agency Token**

This API allows you to generate locationAccessToken from AgencyAccessToken

Operation id: `oauth.post_oauth_locationToken` · `Version: 2021-07-28` · Scopes: `oauth.write`

*Response*: [`GetLocationAccessTokenSuccessfulResponseDto`](#getlocationaccesstokensuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.oauth().get_location_access_token_from_agency_token().await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "oauth.post_oauth_locationToken"
  }
}
```

</details>

#### `POST /oauth/token`

**Get Access Token**

Use Access Tokens to access GoHighLevel resources on behalf of an authenticated location/company.

Operation id: `oauth.post_oauth_token`

*Response*: [`GetAccessCodeSuccessfulResponseDto`](#getaccesscodesuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.oauth().get_access_token().await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "oauth.post_oauth_token"
  }
}
```

</details>

## Endpoints — API v3

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/oauth/installed-locations` | Get Location where app is installed | `v3:oauth.get_oauth_installed_locations` |
| `POST` | `/oauth/location-token` | Get Location Access Token from Agency Token | `v3:oauth.post_oauth_location_token` |
| `POST` | `/oauth/token` | Get Access Token | `v3:oauth.post_oauth_token` |

### Endpoint details — v3

#### `GET /oauth/installed-locations`

**Get Location where app is installed**

This API allows you fetch location where app is installed upon

Operation id: `v3:oauth.get_oauth_installed_locations` · `Version: v3` · Scopes: `oauth.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `pageSize` | number | no | Max items per page (1-100). Replaces legacy `limit` parameter per AIP-158. |
| `pageToken` | string | no | Opaque token returned in a previous response to fetch the next page. Replaces legacy `skip` parameter per AIP-158. |
| `query` | string | no | Parameter to search for the installed location by name |
| `isInstalled` | boolean | no | Filters out location which are installed for specified app under the specified company |
| `restrictToUserLocations` | boolean | no | When true, restricts the list to locations the current user has access to (for restricted agency admins and account admins). When false or omitted, no user-base… |
| `companyId` | string | **yes** | Parameter to search by the companyId |
| `appId` | string | **yes** | Parameter to search by the appId |
| `versionId` | string | no | VersionId of the app |
| `onTrial` | boolean | no | Filters out locations which are installed for specified app in trial mode |
| `planId` | string | no | Filters out location which are installed for specified app under the specified planId |
| `locationId` | string | no | locationId |

*Response*: [`GetInstalledLocationsV3SuccessfulResponseDto`](#getinstalledlocationsv3successfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:oauth.get_oauth_installed_locations",
    "query": {
      "companyId": "<companyId>",
      "appId": "<appId>"
    }
  }
}
```

</details>

#### `POST /oauth/location-token`

**Get Location Access Token from Agency Token**

This API allows you to generate locationAccessToken from AgencyAccessToken

Operation id: `v3:oauth.post_oauth_location_token` · `Version: v3` · Scopes: `oauth.write`

*Response*: [`GetLocationAccessTokenV3SuccessfulResponseDto`](#getlocationaccesstokenv3successfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:oauth.post_oauth_location_token"
  }
}
```

</details>

#### `POST /oauth/token`

**Get Access Token**

Use Access Tokens to access CRM resources on behalf of an authenticated location/company.

Operation id: `v3:oauth.post_oauth_token` · `Version: v3`

*Request body*: [`GetAccessTokenBodyDto`](#getaccesstokenbodydto)

*Response*: [`GetAccessTokenSuccessfulResponseDto`](#getaccesstokensuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:oauth.post_oauth_token",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::oauth::*` (enable the `oauth` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/oauth/).

### `GetAccessCodeSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `access_token` | String | no | — |
| `token_type` | String | no | — |
| `expires_in` | f64 | no | — |
| `refresh_token` | String | no | — |
| `scope` | String | no | — |
| `userType` | String | no | — |
| `locationId` | String | no | Location ID - Present only for Sub-Account Access Token |
| `companyId` | String | no | Company ID |
| `approvedLocations` | Vec<String> | no | Approved locations to generate location access token |
| `userId` | String | **yes** | USER ID - Represent user id of person who performed installation |
| `planId` | String | no | Plan Id of the subscribed plan in paid apps. |
| `isBulkInstallation` | bool | no | — |
| `installToFutureLocations` | bool | no | Boolean to control if user wants app to be automatically installed to future locations (only for company tokens) |
| `approveAllLocations` | bool | no | Boolean indicating if user approved all locations during bulk installation (only for company tokens) |

### `GetAccessCodebodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `client_id` | String | **yes** | The ID provided by GHL for your integration |
| `client_secret` | String | **yes** | — |
| `grant_type` | String — `authorization_code`, `refresh_token`, `client_credentials` | **yes** | — |
| `code` | String | no | — |
| `refresh_token` | String | no | — |
| `user_type` | String — `Company`, `Location` | no | The type of token to be requested |
| `redirect_uri` | String | no | The redirect URI for your application |

### `GetInstalledLocationsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locations` | Vec<InstalledLocationSchema> | no | — |
| `count` | f64 | no | Total location count under the company |
| `installToFutureLocations` | bool | no | Boolean to control if user wants app to be automatically installed to future locations |

### `GetLocationAccessCodeBodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `companyId` | String | **yes** | Company Id of location you want to request token for |
| `locationId` | String | **yes** | The location ID for which you want to obtain accessToken |

### `GetLocationAccessTokenSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `access_token` | String | no | Location access token which can be used to authenticate & authorize API under following scope |
| `token_type` | String | no | — |
| `expires_in` | f64 | no | Time in seconds remaining for token to expire |
| `scope` | String | no | Scopes the following accessToken have access to |
| `locationId` | String | no | Location ID - Present only for Sub-Account Access Token |
| `planId` | String | no | Plan Id of the subscribed plan in paid apps. |
| `userId` | String | **yes** | USER ID - Represent user id of person who performed installation |
| `appId` | String | no | App ID of the installed application |
| `versionId` | String | no | Version ID of the installed app version |

### `InstalledLocationSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Location ID |
| `name` | String | **yes** | Name of the location |
| `address` | String | **yes** | Address linked to location |
| `isInstalled` | bool | no | Check if the requested app is installed for following location |
| `versionId` | String | no | Version ID of the installed app version for this location |
| `installedAt` | String | no | Timestamp when the app was installed on this location |

## Data models — API v3

In Rust: `ghl_models::v3::oauth::*` (enable the `oauth` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/oauth/).

### `AipErrorBodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `code` | String | **yes** | Machine-readable error code (see AipErrorCode enum in @platform-core/aip-framework) |
| `message` | String | **yes** | Human-readable error message |
| `details` | JSON | no | Additional error context (field name, identifier, etc.) |
| `resolution` | String | no | Suggested resolution for the caller |

### `AipErrorResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `error` | [`AipErrorBodyDto`](#aiperrorbodydto) | **yes** | AIP-compliant error envelope |

### `GetAccessTokenBodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `clientId` | String | **yes** | The ID provided by CRM for your integration |
| `clientSecret` | String | **yes** | The client secret provided by CRM for your integration |
| `grantType` | String — `authorization_code`, `refresh_token`, `client_credentials` | **yes** | The OAuth2 grant type — authorization_code, refresh_token, or client_credentials |
| `code` | String | no | The authorization code received from the authorization endpoint (required for authorization_code grant) |
| `refreshToken` | String | no | The refresh token used to obtain a new access token (required for refresh_token grant) |
| `userType` | String — `Company`, `Location` | no | The type of token to be requested |
| `redirectUri` | String | no | The redirect URI for your application |

### `GetAccessTokenSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `accessToken` | String | no | The OAuth2 access token |
| `tokenType` | String | no | The token type (always Bearer) |
| `expiresIn` | f64 | no | Time in seconds until the access token expires |
| `refreshToken` | String | no | The OAuth2 refresh token used to obtain a new access token |
| `scope` | String | no | Space-separated list of scopes the access token has access to |
| `userType` | String | no | The user type associated with the token (Location or Company) |
| `locationId` | String | no | Location ID - Present only for Sub-Account Access Token |
| `companyId` | String | no | Company ID |
| `approvedLocations` | Vec<String> | no | Approved locations to generate location access token |
| `userId` | String | **yes** | USER ID - Represent user id of person who performed installation |
| `planId` | String | no | Plan Id of the subscribed plan in paid apps. |
| `isBulkInstallation` | bool | no | Indicates whether the installation was performed as a bulk installation |
| `installToFutureLocations` | bool | no | Boolean to control if user wants app to be automatically installed to future locations (only for company tokens) |
| `approveAllLocations` | bool | no | Boolean indicating if user approved all locations during bulk installation (only for company tokens) |

### `GetInstalledLocationsSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locations` | Vec<InstalledLocationSchema> | no | List of locations with their installation status for the requested app |
| `count` | f64 | no | Total location count under the company |
| `installToFutureLocations` | bool | no | Boolean to control if user wants app to be automatically installed to future locations |

### `GetInstalledLocationsV3SuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `items` | Vec<InstalledLocationSchema> | **yes** | List of locations with their installation status for the requested app |
| `pagination` | [`V3PaginationMetaDto`](#v3paginationmetadto) | **yes** | Pagination metadata (AIP-158) |
| `metadata` | [`V3InstalledLocationsListMetadataDto`](#v3installedlocationslistmetadatadto) | no | Query metadata (filters and sort applied) |
| `installToFutureLocations` | bool | no | Boolean to control if user wants app to be automatically installed to future locations |

### `GetLocationAccessCodeBodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `companyId` | String | **yes** | Company Id of location you want to request token for |
| `locationId` | String | **yes** | The location ID for which you want to obtain accessToken |

### `GetLocationAccessTokenSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `access_token` | String | no | Location access token which can be used to authenticate & authorize API under following scope |
| `token_type` | String | no | The token type (always Bearer) |
| `expires_in` | f64 | no | Time in seconds remaining for token to expire |
| `scope` | String | no | Scopes the following accessToken have access to |
| `locationId` | String | no | Location ID - Present only for Sub-Account Access Token |
| `planId` | String | no | Plan Id of the subscribed plan in paid apps. |
| `userId` | String | **yes** | USER ID - Represent user id of person who performed installation |
| `appId` | String | no | App ID of the installed application |
| `versionId` | String | no | Version ID of the installed app version |
| `refresh_token` | String | no | The OAuth2 refresh token used to obtain a new access token for this specific location |

### `GetLocationAccessTokenV3SuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `accessToken` | String | no | Location access token which can be used to authenticate & authorize API under following scope |
| `tokenType` | String | no | The token type (always Bearer) |
| `expiresIn` | f64 | no | Time in seconds remaining for token to expire |
| `scope` | String | no | Scopes the following accessToken have access to |
| `locationId` | String | no | Location ID - Present only for Sub-Account Access Token |
| `planId` | String | no | Plan Id of the subscribed plan in paid apps. |
| `userId` | String | **yes** | USER ID - Represent user id of person who performed installation |
| `appId` | String | no | App ID of the installed application |
| `versionId` | String | no | Version ID of the installed app version |
| `refreshToken` | String | no | The OAuth2 refresh token used to obtain a new access token for this specific location. |

### `InstalledLocationSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Location ID |
| `name` | String | **yes** | Name of the location |
| `address` | String | **yes** | Address linked to location |
| `isInstalled` | bool | no | Check if the requested app is installed for following location |
| `versionId` | String | no | Version ID of the installed app version for this location |
| `installedAt` | String | no | Timestamp when the app was installed on this location |

### `V3InstalledLocationsListMetadataDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `filterApplied` | JSON | no | Filters that were applied to the query |
| `sortApplied` | JSON | no | Sort order that was applied to the query |

### `V3PaginationMetaDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `totalRecords` | f64 | no | Total number of records matching the query across all pages |
| `hasNextPage` | bool | **yes** | True when a next page is available |
| `hasPrevPage` | bool | **yes** | True when a previous page is available |
| `nextPageToken` | String | no | Opaque token to fetch the next page |
| `prevPageToken` | String | no | Opaque token to fetch the previous page |
| `currentPageSize` | f64 | **yes** | Number of items returned in the current page |
| `estimatedTotalRecords` | f64 | no | Estimated total records; present when exact total is unknown |

