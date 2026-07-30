# `affiliate-manager`

**4** operations / **13** models in API v2 · **4** operations / **13** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `affiliate-manager` cargo feature on `ghl-sdk`, then call any of the 4 generated methods on `ghl.affiliate_manager()`:

```toml
ghl-sdk = { version = "0.4", features = ["affiliate-manager"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/affiliate-manager/{locationId}/affiliates` | List Affiliates | `list_affiliates()` | `affiliate-manager.get_affiliate_manager_by_locationId_affiliates` |
| `GET` | `/affiliate-manager/{locationId}/affiliates/{affiliateId}` | Get Affiliate | `get_affiliate()` | `affiliate-manager.get_affiliate_manager_by_locationId_affiliates_by_affiliateId` |
| `GET` | `/affiliate-manager/{locationId}/commissions` | List Commissions | `list_commissions()` | `affiliate-manager.get_affiliate_manager_by_locationId_commissions` |
| `GET` | `/affiliate-manager/{locationId}/payouts` | List Payouts | `list_payouts()` | `affiliate-manager.get_affiliate_manager_by_locationId_payouts` |

### Endpoint details — v2

#### `GET /affiliate-manager/{locationId}/affiliates`

**List Affiliates**

Retrieve the list of affiliates for a location.

Operation id: `affiliate-manager.get_affiliate_manager_by_locationId_affiliates` · `Version: 2021-07-28` · Scopes: `affiliate-manager.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `query` | string | no | — |
| `active` | string | no | — |
| `campaignId` | string | no | — |
| `skip` | number | no | — |
| `limit` | number | no | Maximum number of records to return. Maximum allowed value is 100. |
| `fromDate` | string | no | — |
| `toDate` | string | no | — |

*Response*: [`ListAffiliatesResponseDto`](#listaffiliatesresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::affiliate_manager::ListAffiliatesParams;

let params = ListAffiliatesParams::new();
let out = ghl.affiliate_manager().list_affiliates(&locationId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "affiliate-manager.get_affiliate_manager_by_locationId_affiliates",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /affiliate-manager/{locationId}/affiliates/{affiliateId}`

**Get Affiliate**

Retrieve a single affiliate by id for a location.

Operation id: `affiliate-manager.get_affiliate_manager_by_locationId_affiliates_by_affiliateId` · `Version: 2021-07-28` · Scopes: `affiliate-manager.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `affiliateId` | string | **yes** | Affiliate Id |

*Response*: [`GetAffiliateResponseDto`](#getaffiliateresponsedto)

*Rust*:

```rust,ignore
let out = ghl.affiliate_manager().get_affiliate(&locationId, &affiliateId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "affiliate-manager.get_affiliate_manager_by_locationId_affiliates_by_affiliateId",
    "path_params": {
      "locationId": "<locationId>",
      "affiliateId": "<affiliateId>"
    }
  }
}
```

</details>

#### `GET /affiliate-manager/{locationId}/commissions`

**List Commissions**

Retrieve the list of commissions for a location.

Operation id: `affiliate-manager.get_affiliate_manager_by_locationId_commissions` · `Version: 2021-07-28` · Scopes: `affiliate-manager.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `campaignId` | string | no | Campaign Id |
| `affiliateId` | string | no | Affiliate Id |
| `status` | string | no | Status |
| `query` | string | no | Query |
| `skip` | number | no | — |
| `limit` | number | no | Maximum number of records to return. Maximum allowed value is 100. |
| `fromDate` | string | no | — |
| `toDate` | string | no | — |

*Response*: [`GetCommissionListResponseDto`](#getcommissionlistresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::affiliate_manager::ListCommissionsParams;

let params = ListCommissionsParams::new();
let out = ghl.affiliate_manager().list_commissions(&locationId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "affiliate-manager.get_affiliate_manager_by_locationId_commissions",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /affiliate-manager/{locationId}/payouts`

**List Payouts**

Retrieve the list of payouts for a location.

Operation id: `affiliate-manager.get_affiliate_manager_by_locationId_payouts` · `Version: 2021-07-28` · Scopes: `affiliate-manager.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `status` | string | no | Payout status |
| `query` | string | no | query |
| `affiliateId` | string | no | Affiliate Id |
| `campaignId` | string | no | Campaign Id |
| `skip` | number | no | — |
| `limit` | number | no | — |
| `start` | string | no | — |
| `end` | string | no | — |

*Response*: [`GetPayoutListResponseDto`](#getpayoutlistresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::affiliate_manager::ListPayoutsParams;

let params = ListPayoutsParams::new();
let out = ghl.affiliate_manager().list_payouts(&locationId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "affiliate-manager.get_affiliate_manager_by_locationId_payouts",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

## Endpoints — API v3

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/affiliate-manager/{locationId}/affiliates` | List Affiliates | `v3:affiliate-manager.get_affiliate_manager_by_locationId_affiliates` |
| `GET` | `/affiliate-manager/{locationId}/affiliates/{affiliateId}` | Get Affiliate | `v3:affiliate-manager.get_affiliate_manager_by_locationId_affiliates_by_affiliateId` |
| `GET` | `/affiliate-manager/{locationId}/commissions` | List Commissions | `v3:affiliate-manager.get_affiliate_manager_by_locationId_commissions` |
| `GET` | `/affiliate-manager/{locationId}/payouts` | List Payouts | `v3:affiliate-manager.get_affiliate_manager_by_locationId_payouts` |

### Endpoint details — v3

#### `GET /affiliate-manager/{locationId}/affiliates`

**List Affiliates**

Retrieve the list of affiliates for a location.

Operation id: `v3:affiliate-manager.get_affiliate_manager_by_locationId_affiliates` · `Version: v3` · Scopes: `affiliate-manager.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `query` | string | no | — |
| `active` | string | no | — |
| `campaignId` | string | no | — |
| `skip` | number | no | — |
| `limit` | number | no | Maximum number of records to return. Maximum allowed value is 100. |
| `fromDate` | string | no | — |
| `toDate` | string | no | — |

*Response*: [`ListAffiliatesResponseDto`](#listaffiliatesresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:affiliate-manager.get_affiliate_manager_by_locationId_affiliates",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /affiliate-manager/{locationId}/affiliates/{affiliateId}`

**Get Affiliate**

Retrieve a single affiliate by id for a location.

Operation id: `v3:affiliate-manager.get_affiliate_manager_by_locationId_affiliates_by_affiliateId` · `Version: v3` · Scopes: `affiliate-manager.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `affiliateId` | string | **yes** | Affiliate Id |

*Response*: [`GetAffiliateResponseDto`](#getaffiliateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:affiliate-manager.get_affiliate_manager_by_locationId_affiliates_by_affiliateId",
    "path_params": {
      "locationId": "<locationId>",
      "affiliateId": "<affiliateId>"
    }
  }
}
```

</details>

#### `GET /affiliate-manager/{locationId}/commissions`

**List Commissions**

Retrieve the list of commissions for a location.

Operation id: `v3:affiliate-manager.get_affiliate_manager_by_locationId_commissions` · `Version: v3` · Scopes: `affiliate-manager.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `campaignId` | string | no | Campaign Id |
| `affiliateId` | string | no | Affiliate Id |
| `status` | string | no | Status |
| `query` | string | no | Query |
| `skip` | number | no | — |
| `limit` | number | no | Maximum number of records to return. Maximum allowed value is 100. |
| `fromDate` | string | no | — |
| `toDate` | string | no | — |

*Response*: [`GetCommissionListResponseDto`](#getcommissionlistresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:affiliate-manager.get_affiliate_manager_by_locationId_commissions",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /affiliate-manager/{locationId}/payouts`

**List Payouts**

Retrieve the list of payouts for a location.

Operation id: `v3:affiliate-manager.get_affiliate_manager_by_locationId_payouts` · `Version: v3` · Scopes: `affiliate-manager.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `status` | string | no | Payout status |
| `query` | string | no | query |
| `affiliateId` | string | no | Affiliate Id |
| `campaignId` | string | no | Campaign Id |
| `skip` | number | no | — |
| `limit` | number | no | — |
| `start` | string | no | — |
| `end` | string | no | — |

*Response*: [`GetPayoutListResponseDto`](#getpayoutlistresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:affiliate-manager.get_affiliate_manager_by_locationId_payouts",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::affiliate_manager::*` (enable the `affiliate-manager` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/affiliate_manager/).

### `AffiliateListMetaResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `count` | f64 | **yes** | Total affiliates matching the applied filters |

### `CommissionAffiliateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | Affiliate id |
| `name` | String | no | Affiliate display name |
| `email` | String | no | Affiliate email |

### `CommissionCampaignResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Campaign id |
| `name` | String | no | Campaign name |
| `liveMode` | bool | no | Whether the campaign is in live mode |

### `CommissionCustomerResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | Customer id |
| `firstName` | String | no | Customer first name |
| `lastName` | String | no | Customer last name |
| `email` | String | no | Customer email |
| `type` | String | no | Customer type |

### `CommissionListItemResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Commission id |
| `productId` | String | no | Product id |
| `productName` | String | no | Product name |
| `qty` | f64 | no | Quantity |
| `productCommission` | f64 | no | Product commission amount |
| `commissionAmount` | f64 | no | Commission amount |
| `amount` | f64 | no | Base amount |
| `unitDiscount` | f64 | no | Unit discount |
| `campaignName` | String | no | Campaign name |
| `commission` | f64 | no | Commission percentage or value |
| `commissionType` | String | no | Commission type |
| `transactionAt` | String | no | Transaction time |
| `transactionId` | String | no | Transaction id |
| `affiliateId` | String | no | Affiliate id |
| `payoutId` | String | no | Payout id |
| `status` | String | no | Commission status |
| `currency` | String | no | Currency |
| `isTrial` | bool | no | Whether the item is a trial commission |
| `customer` | [`CommissionCustomerResponseDto`](#commissioncustomerresponsedto) | no | Customer details |
| `createdAt` | String | no | Created at |
| `eventId` | String | no | Event id |
| `campaign` | [`CommissionCampaignResponseDto`](#commissioncampaignresponsedto) | no | Campaign details |
| `affiliate` | [`CommissionAffiliateResponseDto`](#commissionaffiliateresponsedto) | no | Affiliate details |
| `dueAt` | String | no | Due date |
| `liveMode` | bool | no | Whether the commission is in live mode |
| `tier` | f64 | no | Commission tier |

### `CommissionListMetaResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `count` | f64 | **yes** | Total commissions matching the filters |

### `GetAffiliateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Affiliate id |
| `firstName` | String | no | Affiliate first name |
| `lastName` | String | no | Affiliate last name |
| `phone` | String | no | Affiliate phone number |
| `deleted` | bool | no | Whether the affiliate is deleted |
| `locationId` | String | **yes** | Location id |
| `active` | bool | no | Whether the affiliate is active |
| `address` | String | no | Affiliate address |
| `avatar` | String | no | Affiliate avatar URL |
| `createdAt` | String | no | Created at timestamp |
| `createdBy` | JSON | no | Created by audit info |
| `facebookUrl` | String | no | Facebook URL |
| `instagramUrl` | String | no | Instagram URL |
| `linkedInUrl` | String | no | LinkedIn URL |
| `twitterUrl` | String | no | Twitter URL |
| `youtubeUrl` | String | no | YouTube URL |
| `websiteUrl` | String | no | Website URL |
| `contactId` | String | no | Contact id associated with the affiliate |
| `campaignIds` | Vec<String> | no | Campaign ids |
| `vatId` | String | no | VAT ID |
| `updatedAt` | String | no | Updated at timestamp |
| `w8Form` | String | no | W8 form URL |
| `w9Form` | String | no | W9 form URL |
| `lastUpdatedBy` | JSON | no | Last updated by audit info |
| `email` | String | **yes** | Affiliate email |
| `revenue` | f64 | no | Affiliate revenue |
| `customer` | f64 | no | Customer count |
| `lead` | f64 | no | Lead count |
| `droppedCustomer` | f64 | no | Dropped customer count |
| `clickCount` | f64 | no | Click count |
| `paid` | f64 | no | Paid amount |
| `currency` | String | no | Currency code |
| `owned` | f64 | no | Owned amount |

### `GetCommissionListResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `commissions` | Vec<CommissionListItemResponseDto> | **yes** | Commission list |
| `meta` | [`CommissionListMetaResponseDto`](#commissionlistmetaresponsedto) | no | Pagination metadata |

### `GetPayoutListResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `payouts` | Vec<PayoutListItemResponseDto> | **yes** | Payout list |
| `meta` | [`PayoutListMetaResponseDto`](#payoutlistmetaresponsedto) | no | Pagination metadata |

### `ListAffiliatesResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `affiliates` | Vec<OAuthAffiliateListItemResponseDto> | **yes** | Affiliate list |
| `meta` | [`AffiliateListMetaResponseDto`](#affiliatelistmetaresponsedto) | **yes** | Pagination metadata |

### `OAuthAffiliateListItemResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Affiliate id |
| `firstName` | String | no | Affiliate first name |
| `lastName` | String | no | Affiliate last name |
| `phone` | String | no | Affiliate phone number |
| `deleted` | bool | no | Whether the affiliate is deleted |
| `locationId` | String | **yes** | Location id |
| `active` | bool | no | Whether the affiliate is active |
| `address` | String | no | Affiliate address |
| `avatar` | String | no | Affiliate avatar URL |
| `createdAt` | String | no | Created at timestamp |
| `createdBy` | JSON | no | Created by audit info |
| `facebookUrl` | String | no | Facebook URL |
| `instagramUrl` | String | no | Instagram URL |
| `linkedInUrl` | String | no | LinkedIn URL |
| `twitterUrl` | String | no | Twitter URL |
| `youtubeUrl` | String | no | YouTube URL |
| `websiteUrl` | String | no | Website URL |
| `contactId` | String | no | Contact id associated with the affiliate |
| `campaignIds` | Vec<String> | no | Campaign ids |
| `vatId` | String | no | VAT ID |
| `updatedAt` | String | no | Updated at timestamp |
| `w8Form` | String | no | W8 form URL |
| `w9Form` | String | no | W9 form URL |
| `lastUpdatedBy` | JSON | no | Last updated by audit info |
| `email` | String | **yes** | Affiliate email |
| `revenue` | f64 | no | Affiliate revenue |
| `customer` | f64 | no | Customer count |
| `lead` | f64 | no | Lead count |
| `droppedCustomer` | f64 | no | Dropped customer count |
| `clickCount` | f64 | no | Click count |
| `paid` | f64 | no | Paid amount |
| `currency` | String | no | Currency code |
| `owned` | f64 | no | Owned amount |

### `PayoutListItemResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Payout id |
| `locationId` | String | **yes** | Location id |
| `affiliateId` | String | **yes** | Affiliate id |
| `campaignId` | String | no | Campaign id |
| `currency` | String | **yes** | Payout currency |
| `amount` | f64 | **yes** | Payout amount |
| `status` | String | no | Payout status |
| `payoutMonth` | String | no | Payout month |
| `dueAt` | String | no | Payout due date |
| `paidAt` | String | no | Payout paid date |
| `paidMeta` | JSON | no | Payout metadata |
| `paidMethod` | String | no | Payout paid method |
| `altId` | String | no | Alternate id |
| `deleted` | bool | no | Whether the payout is deleted |
| `isMigrated` | bool | no | Whether the payout is migrated |
| `createdAt` | String | no | Created at timestamp |
| `updatedAt` | String | no | Updated at timestamp |
| `campaign` | String | no | Campaign name |
| `affiliateName` | String | no | Affiliate display name |
| `affiliateEmail` | String | no | Affiliate email |
| `payoutMethod` | String | no | Primary payout method |
| `affiliate` | [`OAuthAffiliateListItemResponseDto`](#oauthaffiliatelistitemresponsedto) | no | Affiliate details |

### `PayoutListMetaResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `count` | f64 | **yes** | Total payouts matching the filters |

## Data models — API v3

In Rust: `ghl_models::v3::affiliate_manager::*` (enable the `affiliate-manager` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/affiliate_manager/).

### `AffiliateListMetaResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `count` | f64 | **yes** | Total affiliates matching the applied filters |

### `CommissionAffiliateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | Affiliate id |
| `name` | String | no | Affiliate display name |
| `email` | String | no | Affiliate email |

### `CommissionCampaignResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Campaign id |
| `name` | String | no | Campaign name |
| `liveMode` | bool | no | Whether the campaign is in live mode |

### `CommissionCustomerResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | no | Customer id |
| `firstName` | String | no | Customer first name |
| `lastName` | String | no | Customer last name |
| `email` | String | no | Customer email |
| `type` | String | no | Customer type |

### `CommissionListItemResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Commission id |
| `productId` | String | no | Product id |
| `productName` | String | no | Product name |
| `qty` | f64 | no | Quantity |
| `productCommission` | f64 | no | Product commission amount |
| `commissionAmount` | f64 | no | Commission amount |
| `amount` | f64 | no | Base amount |
| `unitDiscount` | f64 | no | Unit discount |
| `campaignName` | String | no | Campaign name |
| `commission` | f64 | no | Commission percentage or value |
| `commissionType` | String | no | Commission type |
| `transactionAt` | String | no | Transaction time |
| `transactionId` | String | no | Transaction id |
| `affiliateId` | String | no | Affiliate id |
| `payoutId` | String | no | Payout id |
| `status` | String | no | Commission status |
| `currency` | String | no | Currency |
| `isTrial` | bool | no | Whether the item is a trial commission |
| `customer` | [`CommissionCustomerResponseDto`](#commissioncustomerresponsedto) | no | Customer details |
| `createdAt` | String | no | Created at |
| `eventId` | String | no | Event id |
| `campaign` | [`CommissionCampaignResponseDto`](#commissioncampaignresponsedto) | no | Campaign details |
| `affiliate` | [`CommissionAffiliateResponseDto`](#commissionaffiliateresponsedto) | no | Affiliate details |
| `dueAt` | String | no | Due date |
| `liveMode` | bool | no | Whether the commission is in live mode |
| `tier` | f64 | no | Commission tier |

### `CommissionListMetaResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `count` | f64 | **yes** | Total commissions matching the filters |

### `GetAffiliateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Affiliate id |
| `firstName` | String | no | Affiliate first name |
| `lastName` | String | no | Affiliate last name |
| `phone` | String | no | Affiliate phone number |
| `deleted` | bool | no | Whether the affiliate is deleted |
| `locationId` | String | **yes** | Location id |
| `active` | bool | no | Whether the affiliate is active |
| `address` | String | no | Affiliate address |
| `avatar` | String | no | Affiliate avatar URL |
| `createdAt` | String | no | Created at timestamp |
| `createdBy` | JSON | no | Created by audit info |
| `facebookUrl` | String | no | Facebook URL |
| `instagramUrl` | String | no | Instagram URL |
| `linkedInUrl` | String | no | LinkedIn URL |
| `twitterUrl` | String | no | Twitter URL |
| `youtubeUrl` | String | no | YouTube URL |
| `websiteUrl` | String | no | Website URL |
| `contactId` | String | no | Contact id associated with the affiliate |
| `campaignIds` | Vec<String> | no | Campaign ids |
| `vatId` | String | no | VAT ID |
| `updatedAt` | String | no | Updated at timestamp |
| `w8Form` | String | no | W8 form URL |
| `w9Form` | String | no | W9 form URL |
| `lastUpdatedBy` | JSON | no | Last updated by audit info |
| `email` | String | **yes** | Affiliate email |
| `revenue` | f64 | no | Affiliate revenue |
| `customer` | f64 | no | Customer count |
| `lead` | f64 | no | Lead count |
| `droppedCustomer` | f64 | no | Dropped customer count |
| `clickCount` | f64 | no | Click count |
| `paid` | f64 | no | Paid amount |
| `currency` | String | no | Currency code |
| `owned` | f64 | no | Owned amount |

### `GetCommissionListResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `commissions` | Vec<CommissionListItemResponseDto> | **yes** | Commission list |
| `meta` | [`CommissionListMetaResponseDto`](#commissionlistmetaresponsedto) | no | Pagination metadata |

### `GetPayoutListResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `payouts` | Vec<PayoutListItemResponseDto> | **yes** | Payout list |
| `meta` | [`PayoutListMetaResponseDto`](#payoutlistmetaresponsedto) | no | Pagination metadata |

### `ListAffiliatesResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `affiliates` | Vec<OAuthAffiliateListItemResponseDto> | **yes** | Affiliate list |
| `meta` | [`AffiliateListMetaResponseDto`](#affiliatelistmetaresponsedto) | **yes** | Pagination metadata |

### `OAuthAffiliateListItemResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Affiliate id |
| `firstName` | String | no | Affiliate first name |
| `lastName` | String | no | Affiliate last name |
| `phone` | String | no | Affiliate phone number |
| `deleted` | bool | no | Whether the affiliate is deleted |
| `locationId` | String | **yes** | Location id |
| `active` | bool | no | Whether the affiliate is active |
| `address` | String | no | Affiliate address |
| `avatar` | String | no | Affiliate avatar URL |
| `createdAt` | String | no | Created at timestamp |
| `createdBy` | JSON | no | Created by audit info |
| `facebookUrl` | String | no | Facebook URL |
| `instagramUrl` | String | no | Instagram URL |
| `linkedInUrl` | String | no | LinkedIn URL |
| `twitterUrl` | String | no | Twitter URL |
| `youtubeUrl` | String | no | YouTube URL |
| `websiteUrl` | String | no | Website URL |
| `contactId` | String | no | Contact id associated with the affiliate |
| `campaignIds` | Vec<String> | no | Campaign ids |
| `vatId` | String | no | VAT ID |
| `updatedAt` | String | no | Updated at timestamp |
| `w8Form` | String | no | W8 form URL |
| `w9Form` | String | no | W9 form URL |
| `lastUpdatedBy` | JSON | no | Last updated by audit info |
| `email` | String | **yes** | Affiliate email |
| `revenue` | f64 | no | Affiliate revenue |
| `customer` | f64 | no | Customer count |
| `lead` | f64 | no | Lead count |
| `droppedCustomer` | f64 | no | Dropped customer count |
| `clickCount` | f64 | no | Click count |
| `paid` | f64 | no | Paid amount |
| `currency` | String | no | Currency code |
| `owned` | f64 | no | Owned amount |

### `PayoutListItemResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Payout id |
| `locationId` | String | **yes** | Location id |
| `affiliateId` | String | **yes** | Affiliate id |
| `campaignId` | String | no | Campaign id |
| `currency` | String | **yes** | Payout currency |
| `amount` | f64 | **yes** | Payout amount |
| `status` | String | no | Payout status |
| `payoutMonth` | String | no | Payout month |
| `dueAt` | String | no | Payout due date |
| `paidAt` | String | no | Payout paid date |
| `paidMeta` | JSON | no | Payout metadata |
| `paidMethod` | String | no | Payout paid method |
| `altId` | String | no | Alternate id |
| `deleted` | bool | no | Whether the payout is deleted |
| `isMigrated` | bool | no | Whether the payout is migrated |
| `createdAt` | String | no | Created at timestamp |
| `updatedAt` | String | no | Updated at timestamp |
| `campaign` | String | no | Campaign name |
| `affiliateName` | String | no | Affiliate display name |
| `affiliateEmail` | String | no | Affiliate email |
| `payoutMethod` | String | no | Primary payout method |
| `affiliate` | [`OAuthAffiliateListItemResponseDto`](#oauthaffiliatelistitemresponsedto) | no | Affiliate details |

### `PayoutListMetaResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `count` | f64 | **yes** | Total payouts matching the filters |

