# `companies`

**1** operations / **3** models in API v2 · **1** operations / **3** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `companies` cargo feature on `ghl-sdk`, then call any of the 1 generated methods on `ghl.companies()`:

```toml
ghl-sdk = { version = "0.4", features = ["companies"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/companies/{companyId}` | Get Company | `get_company()` | `companies.get_companies_by_companyId` |

### Endpoint details — v2

#### `GET /companies/{companyId}`

**Get Company**

Get Comapny

Operation id: `companies.get_companies_by_companyId` · `Version: 2021-07-28` · Scopes: `companies.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | — |

*Response*: [`GetCompanyByIdSuccessfulResponseDto`](#getcompanybyidsuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.companies().get_company(&companyId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "companies.get_companies_by_companyId",
    "path_params": {
      "companyId": "<companyId>"
    }
  }
}
```

</details>

## Endpoints — API v3

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/companies/{companyId}` | Get Company | `v3:companies.get_companies_by_companyId` |

### Endpoint details — v3

#### `GET /companies/{companyId}`

**Get Company**

Get Comapny

Operation id: `v3:companies.get_companies_by_companyId` · `Version: v3` · Scopes: `companies.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `companyId` | string | **yes** | — |

*Response*: [`GetCompanyByIdSuccessfulResponseDto`](#getcompanybyidsuccessfulresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:companies.get_companies_by_companyId",
    "path_params": {
      "companyId": "<companyId>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::companies::*` (enable the `companies` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/companies/).

### `GetCompanyByIdSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `email` | String | no | — |
| `logoUrl` | String | no | — |
| `phone` | String | no | — |
| `website` | String | no | — |
| `domain` | String | no | — |
| `spareDomain` | String | no | — |
| `privacyPolicy` | String | no | — |
| `termsConditions` | String | no | — |
| `address` | String | no | — |
| `city` | String | no | — |
| `postalCode` | String | no | — |
| `country` | String | no | — |
| `state` | String | no | — |
| `timezone` | String | no | — |
| `relationshipNumber` | String | no | — |
| `subdomain` | String | no | — |
| `plan` | f64 | no | — |
| `currency` | String | no | — |
| `customerType` | String | no | — |
| `termsOfServiceVersion` | String | no | — |
| `termsOfServiceAcceptedBy` | String | no | — |
| `twilioTrialMode` | bool | no | — |
| `twilioFreeCredits` | f64 | no | — |
| `termsOfServiceAcceptedDate` | String | no | — |
| `privacyPolicyVersion` | String | no | — |
| `privacyPolicyAcceptedBy` | String | no | — |
| `privacyPolicyAcceptedDate` | String | no | — |
| `affiliatePolicyVersion` | String | no | — |
| `affiliatePolicyAcceptedBy` | String | no | — |
| `affiliatePolicyAcceptedDate` | String | no | — |
| `isReselling` | bool | no | — |
| `onboardingInfo` | [`IOnboardingDto`](#ionboardingdto) | no | — |
| `upgradeEnabledForClients` | bool | no | Flag to set if upgrade plan is enabled |
| `cancelEnabledForClients` | bool | no | Flag to set if cancel plan is enabled |
| `autoSuspendEnabled` | bool | no | Flag to set if auto suspend is enabled |
| `saasSettings` | JSON | no | Saas Settings |
| `stripeConnectId` | String | no | — |
| `enableDepreciatedFeatures` | bool | no | — |
| `premiumUpgraded` | bool | no | If you want to enable / disable Priority Support for any agency. Default value is false. |
| `status` | String | no | — |
| `locationCount` | f64 | no | — |
| `disableEmailService` | bool | no | — |
| `referralId` | String | no | — |
| `isEnterpriseAccount` | bool | no | — |
| `businessNiche` | String | no | The business niche in which the agency is operating |
| `businessCategory` | String | no | Business category |
| `businessAffinityGroup` | String | no | The affinity group of the agency |
| `isSandboxAccount` | bool | no | — |
| `enableNewSubAccountDefaultData` | bool | no | Flag to determine if new sub-accounts should use default data |

### `GetCompanyByIdSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `company` | [`GetCompanyByIdSchema`](#getcompanybyidschema) | no | — |

### `IOnboardingDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `pending` | bool | **yes** | — |
| `haveWebsite` | bool | no | — |
| `websiteUrl` | String | no | — |
| `industryServed` | String | no | — |
| `customerCount` | String | no | — |
| `tools` | Vec<String> | no | — |
| `location` | bool | no | — |
| `conversationDemo` | bool | no | — |
| `locationId` | String | no | — |
| `snapshotId` | String | no | — |
| `planId` | String | no | Selected agency plan unique plan Id |
| `affiliateSignup` | bool | no | Set to true if it is from affiliate |
| `hasJoinedKickoffCall` | bool | no | Set to true if user joined onboarding call |
| `kickoffActionTaken` | bool | no | Set to true if user joined onboarding call |
| `hasJoinedImplementationCall` | bool | no | Set to true if user joined implementation call |
| `version` | String | no | This helps in A/B tracking of onboarding flow |
| `metaData` | JSON | no | metaData for onboarding |

## Data models — API v3

In Rust: `ghl_models::v3::companies::*` (enable the `companies` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/companies/).

### `GetCompanyByIdSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | — |
| `name` | String | no | — |
| `email` | String | no | — |
| `logoUrl` | String | no | — |
| `phone` | String | no | — |
| `website` | String | no | — |
| `domain` | String | no | — |
| `spareDomain` | String | no | — |
| `privacyPolicy` | String | no | — |
| `termsConditions` | String | no | — |
| `address` | String | no | — |
| `city` | String | no | — |
| `postalCode` | String | no | — |
| `country` | String | no | — |
| `state` | String | no | — |
| `timezone` | String | no | — |
| `relationshipNumber` | String | no | — |
| `subdomain` | String | no | — |
| `plan` | f64 | no | — |
| `currency` | String | no | — |
| `customerType` | String | no | — |
| `termsOfServiceVersion` | String | no | — |
| `termsOfServiceAcceptedBy` | String | no | — |
| `twilioTrialMode` | bool | no | — |
| `twilioFreeCredits` | f64 | no | — |
| `termsOfServiceAcceptedDate` | String | no | — |
| `privacyPolicyVersion` | String | no | — |
| `privacyPolicyAcceptedBy` | String | no | — |
| `privacyPolicyAcceptedDate` | String | no | — |
| `affiliatePolicyVersion` | String | no | — |
| `affiliatePolicyAcceptedBy` | String | no | — |
| `affiliatePolicyAcceptedDate` | String | no | — |
| `isReselling` | bool | no | — |
| `onboardingInfo` | [`IOnboardingDto`](#ionboardingdto) | no | — |
| `upgradeEnabledForClients` | bool | no | Flag to set if upgrade plan is enabled |
| `cancelEnabledForClients` | bool | no | Flag to set if cancel plan is enabled |
| `autoSuspendEnabled` | bool | no | Flag to set if auto suspend is enabled |
| `saasSettings` | JSON | no | Saas Settings |
| `stripeConnectId` | String | no | — |
| `enableDepreciatedFeatures` | bool | no | — |
| `premiumUpgraded` | bool | no | If you want to enable / disable Priority Support for any agency. Default value is false. |
| `status` | String | no | — |
| `locationCount` | f64 | no | — |
| `disableEmailService` | bool | no | — |
| `referralId` | String | no | — |
| `isEnterpriseAccount` | bool | no | — |
| `businessNiche` | String | no | The business niche in which the agency is operating |
| `businessCategory` | String | no | Business category |
| `businessAffinityGroup` | String | no | The affinity group of the agency |
| `isSandboxAccount` | bool | no | — |
| `enableNewSubAccountDefaultData` | bool | no | Flag to determine if new sub-accounts should use default data |

### `GetCompanyByIdSuccessfulResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `company` | [`GetCompanyByIdSchema`](#getcompanybyidschema) | no | — |

### `IOnboardingDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `pending` | bool | **yes** | — |
| `haveWebsite` | bool | no | — |
| `websiteUrl` | String | no | — |
| `industryServed` | String | no | — |
| `customerCount` | String | no | — |
| `tools` | Vec<String> | no | — |
| `location` | bool | no | — |
| `conversationDemo` | bool | no | — |
| `locationId` | String | no | — |
| `snapshotId` | String | no | — |
| `planId` | String | no | Selected agency plan unique plan Id |
| `affiliateSignup` | bool | no | Set to true if it is from affiliate |
| `hasJoinedKickoffCall` | bool | no | Set to true if user joined onboarding call |
| `kickoffActionTaken` | bool | no | Set to true if user joined onboarding call |
| `hasJoinedImplementationCall` | bool | no | Set to true if user joined implementation call |
| `version` | String | no | This helps in A/B tracking of onboarding flow |
| `metaData` | JSON | no | metaData for onboarding |

