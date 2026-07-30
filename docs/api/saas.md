# `saas`

**25** operations / **25** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `saas` cargo feature on `ghl-sdk`, then call any of the 25 generated methods on `ghl.v3().saas()` (v3):

```toml
ghl-sdk = { version = "0.5", features = ["saas"] }
```


## Endpoints — API v3

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/saas-api/public-api/agency-plans/{companyId}` | Get Agency Plans | `get_agency_plans()` | `v3:saas.get_saas_api_public_api_agency_plans_by_companyId` |
| `POST` | `/saas-api/public-api/bulk-disable-saas/{companyId}` | Disable SaaS for locations | `disable_saa_s_for_locations()` | `v3:saas.post_saas_api_public_api_bulk_disable_saas_by_companyId` |
| `POST` | `/saas-api/public-api/bulk-enable-saas/{companyId}` | Bulk Enable SaaS | `bulk_enable_saa_s()` | `v3:saas.post_saas_api_public_api_bulk_enable_saas_by_companyId` |
| `GET` | `/saas-api/public-api/companies/{companyId}/locations/{locationId}/wallet-balance` | Get Location Wallet Balance | `get_location_wallet_balance()` | `v3:saas.get_saas_api_public_api_companies_by_companyId_locations_by_locationId_wallet_balance` |
| `POST` | `/saas-api/public-api/companies/{companyId}/locations/{locationId}/wallet-balance/complimentary-credits` | Update Location Wallet Balance | `update_location_wallet_balance()` | `v3:saas.post_saas_api_public_api_companies_by_companyId_locations_by_locationId_wallet_balance_complimentary_credits` |
| `POST` | `/saas-api/public-api/enable-saas/{locationId}` | Enable SaaS for Sub-Account (Formerly Location) | `enable_saa_s_for_sub_account_formerly_location()` | `v3:saas.post_saas_api_public_api_enable_saas_by_locationId` |
| `GET` | `/saas-api/public-api/get-saas-subscription/{locationId}` | Get Location Subscription Details | `get_location_subscription_details()` | `v3:saas.get_saas_api_public_api_get_saas_subscription_by_locationId` |
| `GET` | `/saas-api/public-api/locations` | Get locations by stripeId with companyId | `get_locations_by_stripe_id_with_company_id()` | `v3:saas.get_saas_api_public_api_locations` |
| `POST` | `/saas-api/public-api/pause/{locationId}` | Pause location | `pause_location()` | `v3:saas.post_saas_api_public_api_pause_by_locationId` |
| `GET` | `/saas-api/public-api/saas-locations/{companyId}` | Get SaaS Locations | `get_saa_s_locations()` | `v3:saas.get_saas_api_public_api_saas_locations_by_companyId` |
| `GET` | `/saas-api/public-api/saas-plan/{planId}` | Get SaaS Plan | `get_saa_s_plan()` | `v3:saas.get_saas_api_public_api_saas_plan_by_planId` |
| `POST` | `/saas-api/public-api/update-rebilling/{companyId}` | Update Rebilling | `update_rebilling()` | `v3:saas.post_saas_api_public_api_update_rebilling_by_companyId` |
| `PUT` | `/saas-api/public-api/update-saas-subscription/{locationId}` | Update SaaS subscription | `update_saa_s_subscription()` | `v3:saas.put_saas_api_public_api_update_saas_subscription_by_locationId` |
| `GET` | `/saas/agency-plans/{companyId}` | Get Agency Plans | `get_agency_plans_op()` | `v3:saas.get_saas_agency_plans_by_companyId` |
| `POST` | `/saas/allow-attach-rebilling/{locationId}` | Allow Attach Rebilling | `allow_attach_rebilling()` | `v3:saas.post_saas_allow_attach_rebilling_by_locationId` |
| `POST` | `/saas/bulk-disable-saas/{companyId}` | Disable SaaS for locations | `disable_saa_s_for_locations_op()` | `v3:saas.post_saas_bulk_disable_saas_by_companyId` |
| `POST` | `/saas/bulk-enable-saas/{companyId}` | Bulk Enable SaaS | `bulk_enable_saa_s_op()` | `v3:saas.post_saas_bulk_enable_saas_by_companyId` |
| `POST` | `/saas/enable-saas/{locationId}` | Enable SaaS for Sub-Account (Formerly Location) | `enable_saa_s_for_sub_account_formerly_location_op()` | `v3:saas.post_saas_enable_saas_by_locationId` |
| `GET` | `/saas/get-saas-subscription/{locationId}` | Get Location Subscription Details | `get_location_subscription_details_op()` | `v3:saas.get_saas_get_saas_subscription_by_locationId` |
| `GET` | `/saas/locations` | Get locations by stripeId with companyId | `get_locations_by_stripe_id_with_company_id_op()` | `v3:saas.get_saas_locations` |
| `POST` | `/saas/pause/{locationId}` | Pause location | `pause_location_op()` | `v3:saas.post_saas_pause_by_locationId` |
| `GET` | `/saas/saas-locations/{companyId}` | Get SaaS Locations | `get_saa_s_locations_op()` | `v3:saas.get_saas_saas_locations_by_companyId` |
| `GET` | `/saas/saas-plan/{planId}` | Get SaaS Plan | `get_saa_s_plan_op()` | `v3:saas.get_saas_saas_plan_by_planId` |
| `POST` | `/saas/update-rebilling/{companyId}` | Update Rebilling | `update_rebilling_op()` | `v3:saas.post_saas_update_rebilling_by_companyId` |
| `PUT` | `/saas/update-saas-subscription/{locationId}` | Update SaaS subscription | `update_saa_s_subscription_op()` | `v3:saas.put_saas_update_saas_subscription_by_locationId` |

### Endpoint details — v3

#### `GET /saas-api/public-api/agency-plans/{companyId}`

**Get Agency Plans**

Fetch all agency subscription plans for a given company ID

Operation id: `v3:saas.get_saas_api_public_api_agency_plans_by_companyId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | Company ID to get agency plans for |

*Rust*:

```rust,ignore
let out = ghl.v3().saas().get_agency_plans(&companyId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.get_saas_api_public_api_agency_plans_by_companyId",
    "path_params": {
      "companyId": "<companyId>"
    }
  }
}
```

</details>

#### `POST /saas-api/public-api/bulk-disable-saas/{companyId}`

**Disable SaaS for locations**

Disable SaaS for locations for given locationIds

Operation id: `v3:saas.post_saas_api_public_api_bulk_disable_saas_by_companyId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | Company ID to disable SaaS for |

*Request body*: [`BulkDisableSaasDto`](#bulkdisablesaasdto)

*Response*: [`BulkDisableSaasResponseDto`](#bulkdisablesaasresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().saas().disable_saa_s_for_locations(&companyId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.post_saas_api_public_api_bulk_disable_saas_by_companyId",
    "path_params": {
      "companyId": "<companyId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /saas-api/public-api/bulk-enable-saas/{companyId}`

**Bulk Enable SaaS**

Enable SaaS mode for multiple locations with support for both SaaS v1 and v2

Operation id: `v3:saas.post_saas_api_public_api_bulk_enable_saas_by_companyId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | Company ID to enable SaaS for |

*Request body*: [`BulkEnableSaasRequestDto`](#bulkenablesaasrequestdto)

*Response*: [`BulkEnableSaasResponseDto`](#bulkenablesaasresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().saas().bulk_enable_saa_s(&companyId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.post_saas_api_public_api_bulk_enable_saas_by_companyId",
    "path_params": {
      "companyId": "<companyId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /saas-api/public-api/companies/{companyId}/locations/{locationId}/wallet-balance`

**Get Location Wallet Balance**

Fetch the wallet balance for a specific location. Returns a resource object with balance details.

Operation id: `v3:saas.get_saas_api_public_api_companies_by_companyId_locations_by_locationId_wallet_balance` · `Version: v3` · Scopes: `saas/company.read`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | Company ID that owns the location |
| `locationId` | string | **yes** | Location ID to get wallet balance for |

*Response*: [`LocationWalletBalanceDto`](#locationwalletbalancedto)

*Rust*:

```rust,ignore
let out = ghl.v3().saas().get_location_wallet_balance(&companyId, &locationId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.get_saas_api_public_api_companies_by_companyId_locations_by_locationId_wallet_balance",
    "path_params": {
      "companyId": "<companyId>",
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /saas-api/public-api/companies/{companyId}/locations/{locationId}/wallet-balance/complimentary-credits`

**Update Location Wallet Balance**

Update the wallet balance or complimentary credit settings for a specific location. Supports partial updates via updateMask field (AIP-134 compliant).

Operation id: `v3:saas.post_saas_api_public_api_companies_by_companyId_locations_by_locationId_wallet_balance_complimentary_credits` · `Version: v3` · Scopes: `saas/company.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | Company ID that owns the location |
| `locationId` | string | **yes** | Location ID to update wallet balance for |

*Request body*: [`ComplimentaryCreditDTO`](#complimentarycreditdto)

*Response*: [`LocationWalletBalanceDto`](#locationwalletbalancedto)

*Rust*:

```rust,ignore
let out = ghl.v3().saas().update_location_wallet_balance(&companyId, &locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.post_saas_api_public_api_companies_by_companyId_locations_by_locationId_wallet_balance_complimentary_credits",
    "path_params": {
      "companyId": "<companyId>",
      "locationId": "<locationId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /saas-api/public-api/enable-saas/{locationId}`

**Enable SaaS for Sub-Account (Formerly Location)**

<div> <p>Enable SaaS for Sub-Account (Formerly Location) based on the data provided</p> <div> <span> :::info This feature is only available on Agency Pro ($497) plan. ::: </span> </div> </div>

Operation id: `v3:saas.post_saas_api_public_api_enable_saas_by_locationId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID to enable SaaS for |

*Request body*: [`EnableSaasDto`](#enablesaasdto)

*Response*: [`EnableSaasResponseDto`](#enablesaasresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().saas().enable_saa_s_for_sub_account_formerly_location(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.post_saas_api_public_api_enable_saas_by_locationId",
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

#### `GET /saas-api/public-api/get-saas-subscription/{locationId}`

**Get Location Subscription Details**

Fetch subscription details for a specific location from location metadata

Operation id: `v3:saas.get_saas_api_public_api_get_saas_subscription_by_locationId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID to get subscription details for |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | Company ID to filter subscription details |

*Response*: [`LocationSubscriptionResponseDto`](#locationsubscriptionresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::saas::GetLocationSubscriptionDetailsParams;

let params = GetLocationSubscriptionDetailsParams::new("companyId");
let out = ghl.v3().saas().get_location_subscription_details(&locationId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.get_saas_api_public_api_get_saas_subscription_by_locationId",
    "path_params": {
      "locationId": "<locationId>"
    },
    "query": {
      "companyId": "<companyId>"
    }
  }
}
```

</details>

#### `GET /saas-api/public-api/locations`

**Get locations by stripeId with companyId**

Get locations by stripeCustomerId or stripeSubscriptionId with companyId

Operation id: `v3:saas.get_saas_api_public_api_locations` · `Version: v3`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `customerId` | string | no | Stripe customer ID to find locations for |
| `subscriptionId` | string | no | Stripe subscription ID to find locations for |
| `companyId` | string | **yes** | Company ID to filter locations |

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::saas::GetLocationsByStripeIdWithCompanyIdParams;

let params = GetLocationsByStripeIdWithCompanyIdParams::new("companyId");
let out = ghl.v3().saas().get_locations_by_stripe_id_with_company_id(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.get_saas_api_public_api_locations",
    "query": {
      "companyId": "<companyId>"
    }
  }
}
```

</details>

#### `POST /saas-api/public-api/pause/{locationId}`

**Pause location**

Pause Sub account for given locationId

Operation id: `v3:saas.post_saas_api_public_api_pause_by_locationId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID to pause/unpause |

*Request body*: [`PauseLocationDto`](#pauselocationdto)

*Rust*:

```rust,ignore
let out = ghl.v3().saas().pause_location(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.post_saas_api_public_api_pause_by_locationId",
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

#### `GET /saas-api/public-api/saas-locations/{companyId}`

**Get SaaS Locations**

Fetch all SaaS-activated locations for a company with pagination

Operation id: `v3:saas.get_saas_api_public_api_saas_locations_by_companyId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | Company ID to get SaaS locations for |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | number | no | Page number for pagination |

*Response*: [`GetSaasLocationsResponseDto`](#getsaaslocationsresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::saas::GetSaaSLocationsParams;

let params = GetSaaSLocationsParams::new();
let out = ghl.v3().saas().get_saa_s_locations(&companyId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.get_saas_api_public_api_saas_locations_by_companyId",
    "path_params": {
      "companyId": "<companyId>"
    }
  }
}
```

</details>

#### `GET /saas-api/public-api/saas-plan/{planId}`

**Get SaaS Plan**

Fetch a specific SaaS plan by plan ID

Operation id: `v3:saas.get_saas_api_public_api_saas_plan_by_planId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `planId` | string | **yes** | Plan ID to get SaaS plan details for |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | Company ID to filter SaaS plan |

*Response*: [`SaasPlanResponseDto`](#saasplanresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::saas::GetSaaSPlanParams;

let params = GetSaaSPlanParams::new("companyId");
let out = ghl.v3().saas().get_saa_s_plan(&planId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.get_saas_api_public_api_saas_plan_by_planId",
    "path_params": {
      "planId": "<planId>"
    },
    "query": {
      "companyId": "<companyId>"
    }
  }
}
```

</details>

#### `POST /saas-api/public-api/update-rebilling/{companyId}`

**Update Rebilling**

Bulk update rebilling for given locationIds

Operation id: `v3:saas.post_saas_api_public_api_update_rebilling_by_companyId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | Company ID to update rebilling for |

*Request body*: [`UpdateRebillingDto`](#updaterebillingdto)

*Response*: [`UpdateRebillingResponseDto`](#updaterebillingresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().saas().update_rebilling(&companyId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.post_saas_api_public_api_update_rebilling_by_companyId",
    "path_params": {
      "companyId": "<companyId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /saas-api/public-api/update-saas-subscription/{locationId}`

**Update SaaS subscription**

Update SaaS subscription for given locationId and customerId

Operation id: `v3:saas.put_saas_api_public_api_update_saas_subscription_by_locationId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID to update subscription for |

*Request body*: [`UpdateSubscriptionDto`](#updatesubscriptiondto)

*Rust*:

```rust,ignore
let out = ghl.v3().saas().update_saa_s_subscription(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.put_saas_api_public_api_update_saas_subscription_by_locationId",
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

#### `GET /saas/agency-plans/{companyId}`

**Get Agency Plans**

Fetch all agency subscription plans for a given company ID

Operation id: `v3:saas.get_saas_agency_plans_by_companyId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | — |

*Rust*:

```rust,ignore
let out = ghl.v3().saas().get_agency_plans_op(&companyId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.get_saas_agency_plans_by_companyId",
    "path_params": {
      "companyId": "<companyId>"
    }
  }
}
```

</details>

#### `POST /saas/allow-attach-rebilling/{locationId}`

**Allow Attach Rebilling**

Marks a SaaS sub-account as awaiting rebilling attach and optionally stores the rebilling configuration that should be applied when the rebilling config is created. Sets payment_pending on the sub-account. Only allowed when the sub-account is in setup_pending state.

Operation id: `v3:saas.post_saas_allow_attach_rebilling_by_locationId` · `Version: v3` · Scopes: `saas/company.read`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID (Sub-account) to allow attach rebilling for |

*Request body*: [`AllowAttachRebillingDto`](#allowattachrebillingdto)

*Response*: [`AllowAttachRebillingResponseDto`](#allowattachrebillingresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().saas().allow_attach_rebilling(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.post_saas_allow_attach_rebilling_by_locationId",
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

#### `POST /saas/bulk-disable-saas/{companyId}`

**Disable SaaS for locations**

Disable SaaS for locations for given locationIds

Operation id: `v3:saas.post_saas_bulk_disable_saas_by_companyId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | — |

*Request body*: [`BulkDisableSaasDto`](#bulkdisablesaasdto)

*Rust*:

```rust,ignore
let out = ghl.v3().saas().disable_saa_s_for_locations_op(&companyId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.post_saas_bulk_disable_saas_by_companyId",
    "path_params": {
      "companyId": "<companyId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /saas/bulk-enable-saas/{companyId}`

**Bulk Enable SaaS**

Enable SaaS mode for multiple locations with support for both SaaS v1 and v2

Operation id: `v3:saas.post_saas_bulk_enable_saas_by_companyId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | — |

*Request body*: [`BulkEnableSaasRequestDto`](#bulkenablesaasrequestdto)

*Rust*:

```rust,ignore
let out = ghl.v3().saas().bulk_enable_saa_s_op(&companyId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.post_saas_bulk_enable_saas_by_companyId",
    "path_params": {
      "companyId": "<companyId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /saas/enable-saas/{locationId}`

**Enable SaaS for Sub-Account (Formerly Location)**

<div> <p>Enable SaaS for Sub-Account (Formerly Location) based on the data provided</p> <div> <span> :::info This feature is only available on Agency Pro ($497) plan. ::: </span> </div> </div>

Operation id: `v3:saas.post_saas_enable_saas_by_locationId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Request body*: [`EnableSaasDto`](#enablesaasdto)

*Rust*:

```rust,ignore
let out = ghl.v3().saas().enable_saa_s_for_sub_account_formerly_location_op(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.post_saas_enable_saas_by_locationId",
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

#### `GET /saas/get-saas-subscription/{locationId}`

**Get Location Subscription Details**

Fetch subscription details for a specific location from location metadata

Operation id: `v3:saas.get_saas_get_saas_subscription_by_locationId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | — |

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::saas::GetLocationSubscriptionDetailsOpParams;

let params = GetLocationSubscriptionDetailsOpParams::new("companyId");
let out = ghl.v3().saas().get_location_subscription_details_op(&locationId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.get_saas_get_saas_subscription_by_locationId",
    "path_params": {
      "locationId": "<locationId>"
    },
    "query": {
      "companyId": "<companyId>"
    }
  }
}
```

</details>

#### `GET /saas/locations`

**Get locations by stripeId with companyId**

Get locations by stripeCustomerId or stripeSubscriptionId with companyId

Operation id: `v3:saas.get_saas_locations` · `Version: v3`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `customerId` | string | **yes** | — |
| `subscriptionId` | string | **yes** | — |
| `companyId` | string | **yes** | — |

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::saas::GetLocationsByStripeIdWithCompanyIdOpParams;

let params = GetLocationsByStripeIdWithCompanyIdOpParams::new("customerId", "subscriptionId", "companyId");
let out = ghl.v3().saas().get_locations_by_stripe_id_with_company_id_op(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.get_saas_locations",
    "query": {
      "customerId": "<customerId>",
      "subscriptionId": "<subscriptionId>",
      "companyId": "<companyId>"
    }
  }
}
```

</details>

#### `POST /saas/pause/{locationId}`

**Pause location**

Pause Sub account for given locationId

Operation id: `v3:saas.post_saas_pause_by_locationId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Request body*: [`PauseLocationDto`](#pauselocationdto)

*Rust*:

```rust,ignore
let out = ghl.v3().saas().pause_location_op(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.post_saas_pause_by_locationId",
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

#### `GET /saas/saas-locations/{companyId}`

**Get SaaS Locations**

Fetch all SaaS-activated locations for a company with pagination

Operation id: `v3:saas.get_saas_saas_locations_by_companyId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `page` | number | **yes** | — |

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::saas::GetSaaSLocationsOpParams;

let params = GetSaaSLocationsOpParams::new("page");
let out = ghl.v3().saas().get_saa_s_locations_op(&companyId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.get_saas_saas_locations_by_companyId",
    "path_params": {
      "companyId": "<companyId>"
    },
    "query": {
      "page": "<page>"
    }
  }
}
```

</details>

#### `GET /saas/saas-plan/{planId}`

**Get SaaS Plan**

Fetch a specific SaaS plan by plan ID

Operation id: `v3:saas.get_saas_saas_plan_by_planId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `planId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | — |

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::saas::GetSaaSPlanOpParams;

let params = GetSaaSPlanOpParams::new("companyId");
let out = ghl.v3().saas().get_saa_s_plan_op(&planId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.get_saas_saas_plan_by_planId",
    "path_params": {
      "planId": "<planId>"
    },
    "query": {
      "companyId": "<companyId>"
    }
  }
}
```

</details>

#### `POST /saas/update-rebilling/{companyId}`

**Update Rebilling**

Bulk update rebilling for given locationIds

Operation id: `v3:saas.post_saas_update_rebilling_by_companyId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | — |

*Request body*: [`UpdateRebillingDto`](#updaterebillingdto)

*Rust*:

```rust,ignore
let out = ghl.v3().saas().update_rebilling_op(&companyId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.post_saas_update_rebilling_by_companyId",
    "path_params": {
      "companyId": "<companyId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /saas/update-saas-subscription/{locationId}`

**Update SaaS subscription**

Update SaaS subscription for given locationId and customerId

Operation id: `v3:saas.put_saas_update_saas_subscription_by_locationId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Request body*: [`UpdateSubscriptionDto`](#updatesubscriptiondto)

*Rust*:

```rust,ignore
let out = ghl.v3().saas().update_saa_s_subscription_op(&locationId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:saas.put_saas_update_saas_subscription_by_locationId",
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

In Rust: `ghl_models::v3::saas::*` (enable the `saas` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/saas/).

### `AgencyPlanResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `planId` | String | **yes** | Unique identifier for the plan |
| `title` | String | **yes** | Title of the plan |
| `description` | String | **yes** | Description of the plan |
| `saasProducts` | Vec<String> | **yes** | Array of SaaS products included in the plan |
| `addOns` | Vec<String> | no | Array of add-ons included in the plan |
| `planLevel` | f64 | **yes** | Level of the plan (0-4) |
| `trialPeriod` | f64 | **yes** | Trial period in days |
| `userLimit` | f64 | no | User limit for the plan |
| `contactLimit` | f64 | no | Contact limit for the plan |
| `prices` | Vec<JSON> | **yes** | Pricing information for the plan |
| `categoryId` | String | no | Category ID for the plan |
| `snapshotId` | String | no | Snapshot ID for the plan |
| `productId` | String | no | Product ID for the plan |
| `isSaaSV2` | bool | **yes** | Indicates if this is a SaaS V2 plan |
| `providerLocationId` | String | no | Provider location ID |
| `createdAt` | String | **yes** | Creation timestamp |
| `updatedAt` | String | **yes** | Last update timestamp |

### `AllowAttachRebillingDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `companyId` | String | **yes** | Company ID owning the location |
| `attachedRebillingConfig` | JSON | no | Map of rebilling product code to its config. When provided, this gets stored on the sub-account so it can be applied when the rebilling config is created. Omit to only mark the sub-account as awaiting… |

### `AllowAttachRebillingResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Indicates if the allow attach rebilling operation succeeded |
| `locationId` | String | **yes** | Location ID the rebilling config was attached to |
| `attachedRebillingConfig` | JSON | **yes** | Stored rebilling configuration on the location. Markup is the internal percentage value converted from the request multiplier (e.g. 4 -> 300%, 3 -> 200%). |

### `AttachedRebillingProductConfigDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | Enable rebilling for the product |
| `markup` | f64 | **yes** | Additional value to be added in terms of percentage |
| `price` | f64 | no | Product price override |

### `BadRequestDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | Status code |
| `message` | String | no | Error message |

### `BulkDisableSaasDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationIds` | Vec<String> | **yes** | Location IDs |

### `BulkDisableSaasResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | JSON | **yes** | Response data from the bulk disable SaaS operation |

### `BulkEnableSaasActionPayloadDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `priceId` | String | no | Price ID for the SaaS plan |
| `stripeAccountId` | String | no | Stripe account ID |
| `saasPlanId` | String | **yes** | SaaS plan ID |
| `providerLocationId` | String | no | Provider location ID |

### `BulkEnableSaasRequestDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationIds` | Vec<String> | **yes** | Array of location IDs to enable SaaS for |
| `isSaaSV2` | bool | **yes** | Indicates if the SaaS is V2 |
| `actionPayload` | [`BulkEnableSaasActionPayloadDto`](#bulkenablesaasactionpayloaddto) | **yes** | Action payload for the bulk enable SaaS operation |

### `BulkEnableSaasResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Indicates if the bulk enable SaaS operation was successful |
| `message` | String | **yes** | Message indicating the bulk enable SaaS operation |
| `bulkActionUrl` | String | no | URL for the bulk enable SaaS operation |

### `ComplimentaryCreditDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `complimentaryCreditsAmount` | f64 | no | Credit amount to be added |

### `EnableSaasDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `stripeAccountId` | String | no | Stripe account id(Required only for SaaS V1) |
| `name` | String | no | Name of the stripe customer(Required only for SaaS V1) |
| `email` | String | no | Email of the stripe customer(Required only for SaaS V1) |
| `stripeCustomerId` | String | no | Stripe customer id if exists(Required only for SaaS V1) |
| `companyId` | String | **yes** | — |
| `isSaaSV2` | bool | **yes** | Denotes if it is a saas v2 or v1 sub-account |
| `contactId` | String | no | Agency subaccount used for payment provider integration(Required Only for SaaS V2) |
| `providerLocationId` | String | no | Agency Subaccount ID |
| `description` | String | no | Description |
| `saasPlanId` | String | no | Required only while pre-configuring saas subscription |
| `priceId` | String | no | Required only while pre-configuring saas subscription |

### `EnableSaasResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | JSON | **yes** | Response data from the enable SaaS operation |

### `GetSaasLocationsResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locations` | Vec<SaasLocationDto> | **yes** | Array of SaaS locations |
| `pagination` | JSON | **yes** | — |

### `InternalServerErrorDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | Status code |
| `message` | String | no | Error message |

### `LocationSubscriptionResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `isSaaSV2` | bool | **yes** | Indicates if the SaaS is V2 |
| `companyId` | String | **yes** | Company ID |
| `saasMode` | String | no | SaaS mode |
| `subscriptionId` | String | no | Subscription ID |
| `customerId` | String | no | Customer ID |
| `productId` | String | no | Product ID |
| `priceId` | String | no | Price ID |
| `saasPlanId` | String | no | SaaS plan ID |
| `subscriptionStatus` | String | no | Subscription status |

### `LocationWalletBalanceDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `walletId` | String | **yes** | Wallet Id |
| `balance` | f64 | **yes** | Current wallet balance |
| `complimentaryCredits` | f64 | **yes** | Complimentary credits amount |

### `PauseLocationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `paused` | bool | **yes** | Paused |
| `companyId` | String | **yes** | Company ID |

### `ResourceNotFoundDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | Status code |
| `message` | String | no | Error message |

### `SaasLocationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID |
| `companyId` | String | **yes** | Company ID |
| `saasMode` | String | **yes** | SaaS mode |
| `subscriptionId` | String | **yes** | Subscription ID |
| `customerId` | String | no | Customer ID |
| `name` | String | no | Name |
| `email` | String | no | Email |
| `providerLocationId` | String | no | Provider location ID |
| `isSaaSV2` | bool | no | Indicates if the SaaS is V2 |
| `subscriptionInfo` | JSON | no | Subscription information |

### `SaasPlanResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `planId` | String | **yes** | Unique identifier for the SaaS plan |
| `companyId` | String | **yes** | Company ID associated with the SaaS plan |
| `title` | String | **yes** | Title of the SaaS plan |
| `description` | String | **yes** | Description of the SaaS plan |
| `saasProducts` | Vec<String> | **yes** | Array of SaaS products included in the plan |
| `addOns` | Vec<String> | no | Array of add-ons included in the plan |
| `planLevel` | f64 | **yes** | Level of the plan (0-4) |
| `trialPeriod` | f64 | **yes** | Trial period in days |
| `setupFee` | f64 | no | Setup fee for the plan |
| `userLimit` | f64 | no | User limit for the plan |
| `contactLimit` | f64 | no | Contact limit for the plan |
| `prices` | Vec<JSON> | **yes** | Prices for the plan |
| `categoryId` | String | no | Category ID for the plan |
| `snapshotId` | String | no | Snapshot ID for the plan |
| `providerLocationId` | String | no | Provider location ID |
| `productId` | String | no | Product ID for the plan |
| `isSaaSV2` | bool | **yes** | Indicates if this is a SaaS V2 plan |
| `createdAt` | String | **yes** | Creation timestamp |
| `updatedAt` | String | **yes** | Last update timestamp |

### `UnauthorizedDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | Status code |
| `message` | String | no | Error message |
| `error` | String | no | Error message |

### `UpdateRebillingDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `product` | String — `contentAI`, `workflow_premium_actions`, `workflow_ai`, `conversationAI`, `EmailNotification`, `whatsApp`, `reviewsAI`, `VERIFIED_CALLER_ID`, `WALLET_SALES_TAX`, `NOTIFICATION_SMS`, `EmailSmtp`, `EmailVerification`, `autoCompleteAddress`, `funnelAI`, `domainPurchase`, `Phone`, `Email` | **yes** | The product to update rebilling for |
| `locationIds` | Vec<String> | **yes** | Array of location IDs to update rebilling for |
| `config` | JSON | **yes** | Configuration for rebilling settings |

### `UpdateRebillingResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Indicates if the rebilling update was successful |

### `UpdateSubscriptionDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `subscriptionId` | String | **yes** | Subscription ID |
| `customerId` | String | **yes** | Customer ID |
| `companyId` | String | **yes** | Company ID |

