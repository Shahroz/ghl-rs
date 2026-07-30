# `marketplace`

**9** operations / **13** models in API v2 · **9** operations / **13** models in API v3

## How to call it

No hand-written service yet — reach these endpoints two ways:

**From Rust** (typed body via [`ghl-models`](https://docs.rs/ghl-models)):

```rust,ignore
// cargo add ghl-models --features marketplace
use ghl_models::v2::marketplace::*;

let body = serde_json::to_value(/* a Create…Dto from above */)?;
let out = ghl.request_raw("POST", "/path/", &[], Some(&body), None).await?;
```

**From an AI agent** (MCP meta-tools):

```json
{
  "name": "ghl_search_operations",
  "arguments": {
    "query": "",
    "module": "marketplace"
  }
}
```

## Endpoints — API v2

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `DELETE` | `/marketplace/app/{appId}/installations` | Uninstall an application | `marketplace.delete_marketplace_app_by_appId_installations` |
| `GET` | `/marketplace/app/{appId}/installations` | Get Installer Details | `marketplace.get_marketplace_app_by_appId_installations` |
| `GET` | `/marketplace/app/{appId}/rebilling-config/location/{locationId}` | Get rebilling config for an app subscription and usage plans | `marketplace.get_marketplace_app_by_appId_rebilling_config_location_by_locationId` |
| `GET` | `/marketplace/billing/charges` | Get all wallet charges | `marketplace.get_marketplace_billing_charges` |
| `POST` | `/marketplace/billing/charges` | Create a new wallet charge | `marketplace.post_marketplace_billing_charges` |
| `GET` | `/marketplace/billing/charges/has-funds` | Check if account has sufficient funds | `marketplace.get_marketplace_billing_charges_has_funds` |
| `DELETE` | `/marketplace/billing/charges/{chargeId}` | Delete a wallet charge | `marketplace.delete_marketplace_billing_charges_by_chargeId` |
| `GET` | `/marketplace/billing/charges/{chargeId}` | Get specific wallet charge details | `marketplace.get_marketplace_billing_charges_by_chargeId` |
| `POST` | `/marketplace/external-auth/migration` | Migrate external authentication connection | `marketplace.post_marketplace_external_auth_migration` |

### Endpoint details — v2

#### `DELETE /marketplace/app/{appId}/installations`

**Uninstall an application**

Uninstalls an application from your company or a specific location. This will remove the application`s access and stop all its functionalities

Operation id: `marketplace.delete_marketplace_app_by_appId_installations` · `Version: 2021-07-28` · Scopes: `oauth.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `appId` | string | **yes** | The application id which is to be uninstalled. |

*Request body*: [`DeleteIntegrationBodyDto`](#deleteintegrationbodydto)

*Response*: [`DeleteIntegrationResponse`](#deleteintegrationresponse)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "marketplace.delete_marketplace_app_by_appId_installations",
    "path_params": {
      "appId": "<appId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /marketplace/app/{appId}/installations`

**Get Installer Details**

Fetches installer details for the authenticated user. This endpoint returns information about the company, location, user, and installation details associated with the current OAuth token.

Operation id: `marketplace.get_marketplace_app_by_appId_installations` · `Version: 2021-07-28` · Scopes: `marketplace-installer-details.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `appId` | string | **yes** | ID of the app to get installer details |

*Response*: [`GetInstallerDetailsResponseDTO`](#getinstallerdetailsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "marketplace.get_marketplace_app_by_appId_installations",
    "path_params": {
      "appId": "<appId>"
    }
  }
}
```

</details>

#### `GET /marketplace/app/{appId}/rebilling-config/location/{locationId}`

**Get rebilling config for an app subscription and usage plans**

Get rebilling config for an app subscription and usage plans for the authenticated sub-account. This endpoint returns the subscription and usage plans for an app.

Operation id: `marketplace.get_marketplace_app_by_appId_rebilling_config_location_by_locationId` · `Version: 2021-07-28` · Scopes: `oauth.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `appId` | string | **yes** | ID of the app to get rebilling config |
| `locationId` | string | **yes** | ID of the Sub-Account location to get rebilling config for |

*Response*: [`GetRebillingConfigResponseDTO`](#getrebillingconfigresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "marketplace.get_marketplace_app_by_appId_rebilling_config_location_by_locationId",
    "path_params": {
      "appId": "<appId>",
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /marketplace/billing/charges`

**Get all wallet charges**

Operation id: `marketplace.get_marketplace_billing_charges` · Scopes: `charges.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `meterId` | string | no | Billing Meter ID (you can find this on your app's pricing page on the developer portal) |
| `eventId` | string | no | Event ID / Transaction ID |
| `userId` | string | no | Filter results by User ID that your server passed via API when the charge was created |
| `startDate` | string | no | Filter results AFTER a specific date. Use this in combination with endDate to filter results in a specific time window. |
| `endDate` | string | no | Filter results BEFORE a specific date. Use this in combination with startDate to filter results in a specific time window. |
| `skip` | number | no | Number of records to skip |
| `limit` | number | no | Maximum number of records to return |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "marketplace.get_marketplace_billing_charges"
  }
}
```

</details>

#### `POST /marketplace/billing/charges`

**Create a new wallet charge**

Operation id: `marketplace.post_marketplace_billing_charges` · Scopes: `charges.write`

*Request body*: [`RaiseChargeBodyDTO`](#raisechargebodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "marketplace.post_marketplace_billing_charges",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /marketplace/billing/charges/has-funds`

**Check if account has sufficient funds**

Operation id: `marketplace.get_marketplace_billing_charges_has_funds` · Scopes: `charges.readonly`

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "marketplace.get_marketplace_billing_charges_has_funds"
  }
}
```

</details>

#### `DELETE /marketplace/billing/charges/{chargeId}`

**Delete a wallet charge**

Operation id: `marketplace.delete_marketplace_billing_charges_by_chargeId` · Scopes: `charges.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `chargeId` | string | **yes** | ID of the charge to delete |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "marketplace.delete_marketplace_billing_charges_by_chargeId",
    "path_params": {
      "chargeId": "<chargeId>"
    }
  }
}
```

</details>

#### `GET /marketplace/billing/charges/{chargeId}`

**Get specific wallet charge details**

Operation id: `marketplace.get_marketplace_billing_charges_by_chargeId` · Scopes: `charges.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `chargeId` | string | **yes** | ID of the charge to retrieve |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "marketplace.get_marketplace_billing_charges_by_chargeId",
    "path_params": {
      "chargeId": "<chargeId>"
    }
  }
}
```

</details>

#### `POST /marketplace/external-auth/migration`

**Migrate external authentication connection**

Migrates an external authentication connection credentials (basic or oauth2) for a specific app and location. This endpoint validates the app configuration, stores credentials safely in CRM's native encrypted storage. With this the lifecycle of the token is managed by CRM.

Operation id: `marketplace.post_marketplace_external_auth_migration` · `Version: 2021-07-28` · Scopes: `marketplace-external-auth-migration.write`

*Request body*: [`MigrateConnectionDto`](#migrateconnectiondto)

*Response*: [`MigrateConnectionResponseDto`](#migrateconnectionresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "marketplace.post_marketplace_external_auth_migration",
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
| `DELETE` | `/marketplace/app/{appId}/installations` | Uninstall an application | `v3:marketplace.delete_marketplace_app_by_appId_installations` |
| `GET` | `/marketplace/app/{appId}/installations` | Get Installer Details | `v3:marketplace.get_marketplace_app_by_appId_installations` |
| `GET` | `/marketplace/app/{appId}/rebilling-config/location/{locationId}` | Get rebilling config for an app subscription and usage plans | `v3:marketplace.get_marketplace_app_by_appId_rebilling_config_location_by_locationId` |
| `GET` | `/marketplace/billing/charges` | Get all wallet charges | `v3:marketplace.get_marketplace_billing_charges` |
| `POST` | `/marketplace/billing/charges` | Create a new wallet charge | `v3:marketplace.post_marketplace_billing_charges` |
| `GET` | `/marketplace/billing/charges/has-funds` | Check if account has sufficient funds | `v3:marketplace.get_marketplace_billing_charges_has_funds` |
| `DELETE` | `/marketplace/billing/charges/{chargeId}` | Delete a wallet charge | `v3:marketplace.delete_marketplace_billing_charges_by_chargeId` |
| `GET` | `/marketplace/billing/charges/{chargeId}` | Get specific wallet charge details | `v3:marketplace.get_marketplace_billing_charges_by_chargeId` |
| `POST` | `/marketplace/external-auth/migration` | Migrate external authentication connection | `v3:marketplace.post_marketplace_external_auth_migration` |

### Endpoint details — v3

#### `DELETE /marketplace/app/{appId}/installations`

**Uninstall an application**

Uninstalls an application from your company or a specific location. This will remove the application`s access and stop all its functionalities

Operation id: `v3:marketplace.delete_marketplace_app_by_appId_installations` · `Version: v3` · Scopes: `oauth.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `appId` | string | **yes** | The application id which is to be uninstalled. |

*Request body*: [`DeleteIntegrationBodyDto`](#deleteintegrationbodydto)

*Response*: [`DeleteIntegrationResponse`](#deleteintegrationresponse)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:marketplace.delete_marketplace_app_by_appId_installations",
    "path_params": {
      "appId": "<appId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /marketplace/app/{appId}/installations`

**Get Installer Details**

Fetches installer details for the authenticated user. This endpoint returns information about the company, location, user, and installation details associated with the current OAuth token.

Operation id: `v3:marketplace.get_marketplace_app_by_appId_installations` · `Version: v3` · Scopes: `marketplace-installer-details.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `appId` | string | **yes** | ID of the app to get installer details |

*Response*: [`GetInstallerDetailsResponseDTO`](#getinstallerdetailsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:marketplace.get_marketplace_app_by_appId_installations",
    "path_params": {
      "appId": "<appId>"
    }
  }
}
```

</details>

#### `GET /marketplace/app/{appId}/rebilling-config/location/{locationId}`

**Get rebilling config for an app subscription and usage plans**

Get rebilling config for an app subscription and usage plans for the authenticated sub-account. This endpoint returns the subscription and usage plans for an app.

Operation id: `v3:marketplace.get_marketplace_app_by_appId_rebilling_config_location_by_locationId` · `Version: v3` · Scopes: `oauth.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `appId` | string | **yes** | ID of the app to get rebilling config |
| `locationId` | string | **yes** | ID of the Sub-Account location to get rebilling config for |

*Response*: [`GetRebillingConfigResponseDTO`](#getrebillingconfigresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:marketplace.get_marketplace_app_by_appId_rebilling_config_location_by_locationId",
    "path_params": {
      "appId": "<appId>",
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /marketplace/billing/charges`

**Get all wallet charges**

Operation id: `v3:marketplace.get_marketplace_billing_charges` · `Version: v3` · Scopes: `charges.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `meterId` | string | no | Billing Meter ID (you can find this on your app's pricing page on the developer portal) |
| `eventId` | string | no | Event ID / Transaction ID |
| `userId` | string | no | Filter results by User ID that your server passed via API when the charge was created |
| `startDate` | string | no | Filter results AFTER a specific date. Use this in combination with endDate to filter results in a specific time window. |
| `endDate` | string | no | Filter results BEFORE a specific date. Use this in combination with startDate to filter results in a specific time window. |
| `skip` | number | no | Number of records to skip |
| `limit` | number | no | Maximum number of records to return |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:marketplace.get_marketplace_billing_charges"
  }
}
```

</details>

#### `POST /marketplace/billing/charges`

**Create a new wallet charge**

Operation id: `v3:marketplace.post_marketplace_billing_charges` · `Version: v3` · Scopes: `charges.write`

*Request body*: [`RaiseChargeBodyDTO`](#raisechargebodydto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:marketplace.post_marketplace_billing_charges",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /marketplace/billing/charges/has-funds`

**Check if account has sufficient funds**

Operation id: `v3:marketplace.get_marketplace_billing_charges_has_funds` · `Version: v3` · Scopes: `charges.readonly`

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:marketplace.get_marketplace_billing_charges_has_funds"
  }
}
```

</details>

#### `DELETE /marketplace/billing/charges/{chargeId}`

**Delete a wallet charge**

Operation id: `v3:marketplace.delete_marketplace_billing_charges_by_chargeId` · `Version: v3` · Scopes: `charges.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `chargeId` | string | **yes** | ID of the charge to delete |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:marketplace.delete_marketplace_billing_charges_by_chargeId",
    "path_params": {
      "chargeId": "<chargeId>"
    }
  }
}
```

</details>

#### `GET /marketplace/billing/charges/{chargeId}`

**Get specific wallet charge details**

Operation id: `v3:marketplace.get_marketplace_billing_charges_by_chargeId` · `Version: v3` · Scopes: `charges.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `chargeId` | string | **yes** | ID of the charge to retrieve |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:marketplace.get_marketplace_billing_charges_by_chargeId",
    "path_params": {
      "chargeId": "<chargeId>"
    }
  }
}
```

</details>

#### `POST /marketplace/external-auth/migration`

**Migrate external authentication connection**

Migrates an external authentication connection credentials (basic or oauth2) for a specific app and location. This endpoint validates the app configuration, stores credentials safely in CRM's native encrypted storage. With this the lifecycle of the token is managed by CRM.

Operation id: `v3:marketplace.post_marketplace_external_auth_migration` · `Version: v3` · Scopes: `marketplace-external-auth-migration.write`

*Request body*: [`MigrateConnectionDto`](#migrateconnectiondto)

*Response*: [`MigrateConnectionResponseDto`](#migrateconnectionresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:marketplace.post_marketplace_external_auth_migration",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::marketplace::*` (enable the `marketplace` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/marketplace/).

### `DeleteIntegrationBodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `companyId` | String | no | The company id from which the application is to be uninstalled. If you pass agency token, then companyId is required. It will uninstall application from agency as well as all sub-accounts. |
| `locationId` | String | no | The location id from which the application is to be uninstalled. If you pass location token, then locationId is required. It will uninstall application from that location only. |
| `reason` | String | no | The reason for uninstalling the application. Reason is required if you are uninstalling the application as a developer. |

### `DeleteIntegrationResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | The status of the uninstallation of the application |

### `GetInstallerDetailsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `installationDetails` | [`InstallerDetailsDTO`](#installerdetailsdto) | **yes** | Installation details |

### `GetRebillingConfigResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `plans` | [`PlansDTO`](#plansdto) | **yes** | The rebilling plans configuration |

### `InstallerDetailsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `companyId` | String | **yes** | Company ID |
| `locationId` | String | no | Location ID (if applicable) |
| `companyName` | String | **yes** | Company name |
| `relationshipNumber` | String | **yes** | Company relationship number |
| `companyEmail` | String | no | Company email. Will be null for sub-account installations due to PII concerns. |
| `companyOwnerFullName` | String | no | Company owner full name. Will be null for sub-account installations due to PII concerns. |
| `userId` | String | **yes** | User ID who installed the app |
| `isWhitelabelCompany` | bool | **yes** | Whether the company is a whitelabel company |
| `companyPlan` | String | no | Company plan. Will be null for sub-account installations due to business sensitivity. |
| `companyHighLevelPlan` | String | no | Company plan. Will be null for sub-account installations due to business sensitivity. |
| `marketplaceAppPlanId` | String | no | Marketplace app plan ID for paid apps |
| `whitelabelDetails` | [`WhitelabelDetailsDTO`](#whitelabeldetailsdto) | no | Whitelabel details (only present if isWhitelabelCompany is true) |

### `InternalServerErrorDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | HTTP status code |
| `message` | String | no | Error message describing the internal server error |

### `MigrateConnectionDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `oauth2`, `basic` | **yes** | Type of authentication - basic or oauth2 |
| `locationId` | String | **yes** | Location ID |
| `appId` | String | **yes** | App ID |
| `appVersionId` | String | **yes** | App Version ID |
| `accountId` | String | **yes** | Connection identifier |
| `apiKey` | String | no | API Key (supported when type is basic) |
| `basicCredentials` | JSON | no | Basic auth credentials as key/value pairs (supported when type is basic). Keys are validated against the app version externalAuthConfig.fields. |
| `accessToken` | String | no | Access token (required when type is oauth2) |
| `refreshToken` | String | no | Refresh token (required when type is oauth2) |
| `expiryIn` | f64 | no | Access token expiry time in milliseconds (optional for oauth2) |
| `expiryAt` | f64 | no | Timestamp for access token expiry (optional for oauth2) |
| `scopes` | Vec<String> | no | OAuth2 scopes (optional for oauth2) |
| `displayName` | String | no | Display name for the connection (optional, defaults to accountId) |
| `isDefault` | bool | no | Whether this is the default connection for the location (optional, defaults to false) |

### `MigrateConnectionResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Indicates if the migration was successful |
| `identifier` | String | **yes** | Unique identifier for the migrated connection |
| `message` | String | no | Message describing the result |

### `PlansDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `subscription` | Vec<SubscriptionPlanDTO> | **yes** | Subscription plans |
| `usage` | Vec<UsagePlanDTO> | **yes** | Usage-based plans |

### `RaiseChargeBodyDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `appId` | String | **yes** | App ID of the App |
| `meterId` | String | **yes** | Billing Meter ID (you can find this on your app's pricing page) |
| `eventId` | String | **yes** | Event ID / Transaction ID on your server's side. This will help you maintain the reference of the event/transaction on your end that you charged the customer for. |
| `userId` | String | no | User ID |
| `locationId` | String | **yes** | ID of the Sub-Account to be charged |
| `companyId` | String | **yes** | ID of the Agency the Sub-account belongs to |
| `description` | String | **yes** | Description of the charge |
| `price` | f64 | no | Price per unit to charge |
| `units` | f64 | **yes** | Number of units to charge |
| `eventTime` | String | no | The timestamp when the event/transaction was performed. If blank, the billing timestamp will be set as the event time. ISO8601 Format. |

### `SubscriptionPlanDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `resellingAmount` | f64 | **yes** | The reselling amount |
| `baseAmount` | f64 | **yes** | The base amount |
| `planId` | String | **yes** | The plan id |
| `features` | Vec<String> | **yes** | The features |
| `paymentType` | String | **yes** | The payment time |
| `name` | String | **yes** | The plan name |
| `paymentTime` | String | **yes** | The payment time |

### `UsagePlanDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `productType` | String | **yes** | The product type |
| `productName` | String | **yes** | The product name |
| `usageUnit` | String | **yes** | The usage unit for the meter |
| `meterId` | String | **yes** | The meter id |
| `meterName` | String | **yes** | The meter name |
| `fixedPricePerUnit` | f64 | **yes** | The fixed price per unit, applicable for fixed price type |
| `priceType` | String — `fixed`, `dynamic` | **yes** | The price type |
| `minPricePerUnit` | String | **yes** | The min price per unit, applicable for dynamic price type |
| `maxPricePerUnit` | String | **yes** | The max price per unit, applicable for dynamic price type |
| `executionLimitPerCycle` | f64 | **yes** | The execution limit per cycle |

### `WhitelabelDetailsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `domain` | String | **yes** | Domain of the whitelabel company |
| `logoUrl` | String | **yes** | Logo URL of the whitelabel company |

## Data models — API v3

In Rust: `ghl_models::v3::marketplace::*` (enable the `marketplace` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/marketplace/).

### `DeleteIntegrationBodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `companyId` | String | no | The company id from which the application is to be uninstalled. If you pass agency token, then companyId is required. It will uninstall application from agency as well as all sub-accounts. |
| `locationId` | String | no | The location id from which the application is to be uninstalled. If you pass location token, then locationId is required. It will uninstall application from that location only. |
| `reason` | String | no | The reason for uninstalling the application. Reason is required if you are uninstalling the application as a developer. |

### `DeleteIntegrationResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | The status of the uninstallation of the application |

### `GetInstallerDetailsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `installationDetails` | [`InstallerDetailsDTO`](#installerdetailsdto) | **yes** | Installation details |

### `GetRebillingConfigResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `plans` | [`PlansDTO`](#plansdto) | **yes** | The rebilling plans configuration |

### `InstallerDetailsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `companyId` | String | **yes** | Company ID |
| `locationId` | String | no | Location ID (if applicable) |
| `companyName` | String | **yes** | Company name |
| `relationshipNumber` | String | **yes** | Company relationship number |
| `companyEmail` | String | no | Company email. Will be null for sub-account installations due to PII concerns. |
| `companyOwnerFullName` | String | no | Company owner full name. Will be null for sub-account installations due to PII concerns. |
| `userId` | String | **yes** | User ID who installed the app |
| `isWhitelabelCompany` | bool | **yes** | Whether the company is a whitelabel company |
| `companyPlan` | String | no | Company plan. Will be null for sub-account installations due to business sensitivity. |
| `companyHighLevelPlan` | String | no | Company plan. Will be null for sub-account installations due to business sensitivity. |
| `marketplaceAppPlanId` | String | no | Marketplace app plan ID for paid apps |
| `whitelabelDetails` | [`WhitelabelDetailsDTO`](#whitelabeldetailsdto) | no | Whitelabel details (only present if isWhitelabelCompany is true) |

### `InternalServerErrorDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | HTTP status code |
| `message` | String | no | Error message describing the internal server error |

### `MigrateConnectionDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `oauth2`, `basic` | **yes** | Type of authentication - basic or oauth2 |
| `locationId` | String | **yes** | Location ID |
| `appId` | String | **yes** | App ID |
| `appVersionId` | String | **yes** | App Version ID |
| `accountId` | String | **yes** | Connection identifier |
| `apiKey` | String | no | API Key (supported when type is basic) |
| `basicCredentials` | JSON | no | Basic auth credentials as key/value pairs (supported when type is basic). Keys are validated against the app version externalAuthConfig.fields. |
| `accessToken` | String | no | Access token (required when type is oauth2) |
| `refreshToken` | String | no | Refresh token (required when type is oauth2) |
| `expiryIn` | f64 | no | Access token expiry time in milliseconds (optional for oauth2) |
| `expiryAt` | f64 | no | Timestamp for access token expiry (optional for oauth2) |
| `scopes` | Vec<String> | no | OAuth2 scopes (optional for oauth2) |
| `displayName` | String | no | Display name for the connection (optional, defaults to accountId) |
| `isDefault` | bool | no | Whether this is the default connection for the location (optional, defaults to false) |

### `MigrateConnectionResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Indicates if the migration was successful |
| `identifier` | String | **yes** | Unique identifier for the migrated connection |
| `message` | String | no | Message describing the result |

### `PlansDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `subscription` | Vec<SubscriptionPlanDTO> | **yes** | Subscription plans |
| `usage` | Vec<UsagePlanDTO> | **yes** | Usage-based plans |

### `RaiseChargeBodyDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `appId` | String | **yes** | App ID of the App |
| `meterId` | String | **yes** | Billing Meter ID (you can find this on your app's pricing page) |
| `eventId` | String | **yes** | Event ID / Transaction ID on your server's side. This will help you maintain the reference of the event/transaction on your end that you charged the customer for. |
| `userId` | String | no | User ID |
| `locationId` | String | **yes** | ID of the Sub-Account to be charged |
| `companyId` | String | **yes** | ID of the Agency the Sub-account belongs to |
| `description` | String | **yes** | Description of the charge |
| `price` | f64 | no | Price per unit to charge |
| `units` | f64 | **yes** | Number of units to charge |
| `eventTime` | String | no | The timestamp when the event/transaction was performed. If blank, the billing timestamp will be set as the event time. ISO8601 Format. |

### `SubscriptionPlanDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `resellingAmount` | f64 | **yes** | The reselling amount |
| `baseAmount` | f64 | **yes** | The base amount |
| `planId` | String | **yes** | The plan id |
| `features` | Vec<String> | **yes** | The features |
| `paymentType` | String | **yes** | The payment time |
| `name` | String | **yes** | The plan name |
| `paymentTime` | String | **yes** | The payment time |

### `UsagePlanDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `productType` | String | **yes** | The product type |
| `productName` | String | **yes** | The product name |
| `usageUnit` | String | **yes** | The usage unit for the meter |
| `meterId` | String | **yes** | The meter id |
| `meterName` | String | **yes** | The meter name |
| `fixedPricePerUnit` | f64 | **yes** | The fixed price per unit, applicable for fixed price type |
| `priceType` | String — `fixed`, `dynamic` | **yes** | The price type |
| `minPricePerUnit` | String | **yes** | The min price per unit, applicable for dynamic price type |
| `maxPricePerUnit` | String | **yes** | The max price per unit, applicable for dynamic price type |
| `executionLimitPerCycle` | f64 | **yes** | The execution limit per cycle |

### `WhitelabelDetailsDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `domain` | String | **yes** | Domain of the whitelabel company |
| `logoUrl` | String | **yes** | Logo URL of the whitelabel company |

