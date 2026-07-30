# `phone-system`

**4** operations / **6** models in API v2 · **4** operations / **8** models in API v3

## How to call it

No hand-written service yet — reach these endpoints two ways:

**From Rust** (typed body via [`ghl-models`](https://docs.rs/ghl-models)):

```rust,ignore
// cargo add ghl-models --features phone-system
use ghl_models::v2::phone_system::*;

let body = serde_json::to_value(/* a Create…Dto from above */)?;
let out = ghl.request_raw("POST", "/path/", &[], Some(&body), None).await?;
```

**From an AI agent** (MCP meta-tools):

```json
{
  "name": "ghl_search_operations",
  "arguments": {
    "query": "",
    "module": "phone-system"
  }
}
```

## Endpoints — API v2

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/phone-system/number-pools` | List Number Pools | `phone-system.get_phone_system_number_pools` |
| `GET` | `/phone-system/numbers/location/{locationId}` | List active numbers | `phone-system.get_phone_system_numbers_location_by_locationId` |
| `GET` | `/phone-system/numbers/location/{locationId}/available` | List available phone numbers | `phone-system.get_phone_system_numbers_location_by_locationId_available` |
| `POST` | `/phone-system/numbers/location/{locationId}/purchase` | Purchase a phone number | `phone-system.post_phone_system_numbers_location_by_locationId_purchase` |

### Endpoint details — v2

#### `GET /phone-system/number-pools`

**List Number Pools**

Get list of number pools

Operation id: `phone-system.get_phone_system_number_pools` · `Version: 2021-07-28` · Scopes: `numberpools.read`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | no | Location ID to filter pools |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "phone-system.get_phone_system_number_pools"
  }
}
```

</details>

#### `GET /phone-system/numbers/location/{locationId}`

**List active numbers**

Retrieve a paginated list of active phone numbers for a specific location. Supports filtering, pagination, and optional exclusion of number pool assignments.

Operation id: `phone-system.get_phone_system_numbers_location_by_locationId` · `Version: 2021-07-28` · Scopes: `phonenumbers.read`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | The unique identifier of the location |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `pageSize` | number | no | How many resources to return in each list page. The default is 50, and the maximum is 1000. |
| `page` | number | no | The page index for pagination. The default is 0. |
| `searchFilter` | string | no | Filter numbers by phone number pattern. Supports partial matching (e.g., "+91" to find all Indian numbers). |
| `skipNumberPool` | boolean | no | Whether to exclude numbers that are assigned to number pools. Default is true. |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "phone-system.get_phone_system_numbers_location_by_locationId",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /phone-system/numbers/location/{locationId}/available`

**List available phone numbers**

Search for available phone numbers to purchase for a specific location. Supports filtering by number pattern, type, and capabilities.

Operation id: `phone-system.get_phone_system_numbers_location_by_locationId_available` · `Version: 2021-07-28` · Scopes: `phonenumbers.read`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | The unique identifier of the location |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `countryCode` | string | **yes** | ISO 3166-1 alpha-2 country code for which to search available numbers |
| `numberTypes` | string | no | Comma-separated list of phone number types to search for (e.g. local, tollFree, mobile) |
| `firstPart` | string | no | Filter numbers that begin with this digit pattern |
| `lastPart` | string | no | Filter numbers that end with this digit pattern |
| `anywhere` | string | no | Filter numbers that contain this digit pattern anywhere |
| `smsEnabled` | boolean | no | Filter for numbers with SMS capability |
| `mmsEnabled` | boolean | no | Filter for numbers with MMS capability |
| `voiceEnabled` | boolean | no | Filter for numbers with voice capability |

*Response*: [`AvailableNumbersResponseDto`](#availablenumbersresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "phone-system.get_phone_system_numbers_location_by_locationId_available",
    "path_params": {
      "locationId": "<locationId>"
    },
    "query": {
      "countryCode": "<countryCode>"
    }
  }
}
```

</details>

#### `POST /phone-system/numbers/location/{locationId}/purchase`

**Purchase a phone number**

Purchase a phone number for a specific location.

Operation id: `phone-system.post_phone_system_numbers_location_by_locationId_purchase` · `Version: 2021-07-28` · Scopes: `phonenumbers.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | The unique identifier of the location |

*Request body*: [`PurchasePhoneNumberBodyDto`](#purchasephonenumberbodydto)

*Response*: [`TwilioAccountResponseDto`](#twilioaccountresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "phone-system.post_phone_system_numbers_location_by_locationId_purchase",
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

## Endpoints — API v3

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/phone-system/number-pools` | List number pools | `v3:phone-system.get_phone_system_number_pools` |
| `GET` | `/phone-system/numbers/location/{locationId}` | List active numbers | `v3:phone-system.get_phone_system_numbers_location_by_locationId` |
| `GET` | `/phone-system/numbers/location/{locationId}/available` | List available phone numbers | `v3:phone-system.get_phone_system_numbers_location_by_locationId_available` |
| `POST` | `/phone-system/numbers/location/{locationId}/purchase` | Purchase number for location | `v3:phone-system.post_phone_system_numbers_location_by_locationId_purchase` |

### Endpoint details — v3

#### `GET /phone-system/number-pools`

**List number pools**

Returns number pools for the location. Requires locationId as a query parameter.

Operation id: `v3:phone-system.get_phone_system_number_pools` · `Version: v3` · Scopes: `numberpools.read`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID to scope the number pool list |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:phone-system.get_phone_system_number_pools",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /phone-system/numbers/location/{locationId}`

**List active numbers**

List active numbers. With `version: v3`, the HTTP 200 body is the standard success envelope (`status`, `data`, `message`, `statusCode`). The v3 list payload is under `data`; `isUnderGhl` is renamed to `isUnderLc` per AIP naming convention.

Operation id: `v3:phone-system.get_phone_system_numbers_location_by_locationId` · `Version: v3` · Scopes: `phonenumbers.read`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID as string |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `pageSize` | number | no | How many resources to return in each list page. The default is 50, and the maximum is 1000. |
| `page` | number | no | The page index. The default is 0. |
| `searchFilter` | string | no | Number search Filter |
| `skipNumberPool` | boolean | no | When true, exclude numbers assigned to number pools from the list. |
| `includeRcsSenderIds` | boolean | no | Include RCS Sender IDs |

*Response*: [`ListNumbersV3Http200ResponseDto`](#listnumbersv3http200responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:phone-system.get_phone_system_numbers_location_by_locationId",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /phone-system/numbers/location/{locationId}/available`

**List available phone numbers**

Search Twilio inventory for purchasable phone numbers in a country for the given location.

Operation id: `v3:phone-system.get_phone_system_numbers_location_by_locationId_available` · `Version: v3` · Scopes: `phonenumbers.read`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID as string |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `firstPart` | string | **yes** | firstPart is the beginning of the phone number |
| `lastPart` | string | **yes** | lastPart is the ending of the phone number |
| `anywhere` | string | **yes** | anywhere are the numbers required anywhere in phone number |
| `numberTypes` | array | **yes** | comma separated types of phone number required |
| `smsEnabled` | boolean | **yes** | requested phone numbers should have sms functionality |
| `mmsEnabled` | boolean | **yes** | requested phone numbers should have mms functionality |
| `voiceEnabled` | boolean | **yes** | requested phone numbers should have voice functionality |
| `countryCode` | string | **yes** | country for which the phone numbers are being requested |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:phone-system.get_phone_system_numbers_location_by_locationId_available",
    "path_params": {
      "locationId": "<locationId>"
    },
    "query": {
      "firstPart": "<firstPart>",
      "lastPart": "<lastPart>",
      "anywhere": "<anywhere>",
      "numberTypes": "<numberTypes>",
      "smsEnabled": "<smsEnabled>",
      "mmsEnabled": "<mmsEnabled>",
      "voiceEnabled": "<voiceEnabled>",
      "countryCode": "<countryCode>"
    }
  }
}
```

</details>

#### `POST /phone-system/numbers/location/{locationId}/purchase`

**Purchase number for location**

Purchase number for location. With `version: v3`, the HTTP 201 body is the standard success envelope (`status`, `data`, `message`, `statusCode`). The v3 purchase fields live under `data`: `number`, `locationId`, `id`, and `underLcAccount` (renamed from under_ghl_account).

Operation id: `v3:phone-system.post_phone_system_numbers_location_by_locationId_purchase` · `Version: v3` · Scopes: `phonenumbers.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID as string |

*Request body*: [`PurchasePhoneNumberBodyDto`](#purchasephonenumberbodydto)

*Response*: [`PurchaseNumberForLocationV3Http201ResponseDto`](#purchasenumberforlocationv3http201responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:phone-system.post_phone_system_numbers_location_by_locationId_purchase",
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

In Rust: `ghl_models::v2::phone_system::*` (enable the `phone-system` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/phone_system/).

### `AvailableNumbersResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `fingerprintId` | String | **yes** | Unique fingerprint ID for this search result, required when purchasing one of the returned numbers |
| `numbers` | Vec<AvailablePhoneNumberDto> | **yes** | List of available phone numbers matching the search criteria |

### `AvailablePhoneNumberDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `phoneNumber` | String | **yes** | E.164 formatted phone number |
| `friendlyName` | String | **yes** | Human-readable formatted phone number |
| `isoCountry` | String | **yes** | ISO 3166-1 alpha-2 country code |
| `lata` | String | no | Local Access and Transport Area code |
| `locality` | String | no | City or locality of the number |
| `rateCenter` | String | no | Rate center of the number |
| `latitude` | String | no | Latitude coordinate of the number's location |
| `longitude` | String | no | Longitude coordinate of the number's location |
| `region` | String | no | State or region abbreviation |
| `postalCode` | String | no | Postal code of the number |
| `addressRequirements` | String — `none`, `any`, `local`, `foreign` | **yes** | Address requirements for purchasing this number |
| `beta` | bool | **yes** | Whether this is a beta number |
| `capabilities` | JSON | **yes** | Communication capabilities supported by this number |
| `price` | JSON | no | Pricing information for this number |

### `DetailedPhoneNumberDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `phoneNumber` | String | **yes** | E.164 formatted phone number |
| `friendlyName` | String | no | Human-readable name assigned to the number |
| `sid` | String | **yes** | Phone number SID (unique identifier) |
| `countryCode` | String | **yes** | ISO 3166-1 alpha-2 country code |
| `capabilities` | JSON | **yes** | Communication capabilities supported by this number |
| `type` | String — `local`, `toll-free`, `mobile`, `national` | **yes** | Type of phone number (local, toll-free, mobile, etc.) |
| `isDefaultNumber` | bool | **yes** | Whether this is the default outbound number for the location |
| `linkedUser` | String | no | User ID of the user assigned to this number |
| `linkedRingAllUsers` | Vec<String> | **yes** | Array of user IDs that should ring when this number is called |
| `inboundCallService` | JSON | no | Configuration for inbound call handling service |
| `forwardingNumber` | String | no | Phone number to forward calls to |
| `isGroupConversationEnabled` | bool | **yes** | Whether group conversations are enabled for this number (US/CA numbers with SMS/MMS only) |
| `addressSid` | String | no | Address SID for compliance purposes |
| `bundleSid` | String | no | Bundle SID for regulatory compliance |
| `dateAdded` | String | no | When the number was originally purchased/added |
| `dateUpdated` | String | no | When the number configuration was last updated |
| `origin` | String — `twilio`, `hosted`, `ported` | no | Source or origin of the phone number |

### `NumberPoolDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Unique identifier for the number pool |
| `name` | String | no | Human-readable name of the number pool |
| `locationId` | String | no | Location ID this pool belongs to |
| `numbers` | Vec<JSON> | no | Phone numbers in this pool |
| `forwardingNumber` | String | no | Number to forward calls to |
| `whisper` | bool | no | Whether whisper is enabled |
| `whisperMessage` | String | no | Message played during whisper |
| `callRecording` | bool | no | Whether call recording is enabled |
| `isActive` | bool | no | Whether the number pool is active |
| `inboundCallService` | JSON | no | Inbound call service configuration |

### `PurchasePhoneNumberBodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `phoneNumber` | String | **yes** | The phone number to purchase |
| `countryCode` | String | no | ISO 3166-1 alpha-2 country code of the number |
| `numberType` | String — `local`, `tollFree`, `mobile` | no | Type of phone number |
| `addressSid` | String | no | Twilio address SID for compliance |
| `bundleSid` | String | no | Twilio bundle SID for regulatory compliance |
| `locality` | String | no | Locality where the number is being purchased |
| `region` | String | no | Region where the number is being purchased |
| `fingerprintId` | String | no | Unique request ID for idempotency (fingerprint of the purchase request) |
| `skipLocationKYC` | bool | no | Skip location-level KYC verification if agency-level compliance has already been verified |

### `TwilioAccountResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier of the Twilio account record |
| `account_sid` | String | **yes** | Twilio Account SID |
| `under_ghl_account` | bool | **yes** | Whether this location is under a GHL-managed Twilio account |
| `validate_sms` | bool | **yes** | Whether SMS validation is enabled |
| `location_id` | String | **yes** | The location ID this Twilio account belongs to |
| `migration_status` | String | no | Current migration status of the account |
| `migration_numbers` | Vec<String> | no | List of numbers being migrated |
| `assigned_to_numbers` | JSON | no | Map of phone numbers to assigned user IDs |
| `numbers` | JSON | **yes** | Map of phone numbers to their service type (e.g. 'conversation') |
| `number_name` | JSON | no | Map of phone numbers to their friendly names |
| `new_account_sid` | String | no | New account SID if the account has been migrated to new credentials |

## Data models — API v3

In Rust: `ghl_models::v3::phone_system::*` (enable the `phone-system` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/phone_system/).

### `ListNumberItemResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `phoneNumber` | String | **yes** | Phone number in E.164 format |
| `friendlyName` | String | no | Human-friendly label for the number |
| `sid` | String | no | Provider number identifier |
| `countryCode` | String | no | ISO country code for the number |
| `capabilities` | [`NumberCapabilitiesDto`](#numbercapabilitiesdto) | no | Phone number capabilities |
| `type` | String | no | Phone number type |
| `isDefaultNumber` | bool | no | Whether this is the default outbound number |
| `linkedUser` | String | no | Linked user ID if the number is assigned |
| `linkedRingAllUsers` | Vec<String> | no | Ring-all user IDs linked to the number |
| `inboundCallService` | JSON | no | Inbound call service metadata |
| `forwardingNumber` | String | no | Forwarding number in E.164 format |
| `isGroupConversationEnabled` | bool | no | Whether group conversations are enabled |
| `addressSid` | String | no | Address SID used for regulated number purchases |
| `bundleSid` | String | no | Bundle SID used for regulated number purchases |
| `dateAdded` | JSON | no | Date the number was added |
| `dateUpdated` | JSON | no | Date the number was last updated |
| `dateCreated` | JSON | no | Legacy created-at field returned by some providers |
| `origin` | String | no | Provider origin for the number |

### `ListNumbersV3Http200ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | String — `success` | **yes** | Outcome indicator from the shared success helper. |
| `data` | [`ListNumbersV3ResponseDto`](#listnumbersv3responsedto) | **yes** | V3 list payload: numbers, pagination fields, isUnderLc (renamed from isUnderGhl), etc. |
| `message` | String | **yes** | Human-readable success message. |
| `statusCode` | f64 | **yes** | HTTP status echoed in the response body. |

### `ListNumbersV3ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `numbers` | Vec<ListNumberItemResponseDto> | **yes** | Active numbers available for the location |
| `isUnderLc` | bool | no | Whether the account is managed under LC. Renamed from isUnderGhl. |
| `pageSize` | f64 | no | The page size requested |
| `page` | f64 | no | The zero-based page index requested |
| `accountStatus` | String | no | Twilio account status for the location |
| `rcsSenderIds` | Vec<RcsSenderIdResponseDto> | no | Optional RCS sender IDs returned with the number list |
| `total` | f64 | no | Total number of active numbers when available |

### `NumberCapabilitiesDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `voice` | bool | no | Whether the number supports voice calls |
| `sms` | bool | no | Whether the number supports SMS |
| `mms` | bool | no | Whether the number supports MMS |
| `fax` | bool | no | Whether the number supports fax |

### `PurchaseNumberForLocationV3Http201ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | String — `success` | **yes** | Outcome indicator from the shared success helper. |
| `data` | [`PurchaseNumberForLocationV3ResponseDto`](#purchasenumberforlocationv3responsedto) | **yes** | V3 purchase payload: purchased number, location, Twilio account id, and underLcAccount. |
| `message` | String | **yes** | Human-readable success message. |
| `statusCode` | f64 | **yes** | HTTP status echoed in the response body. |

### `PurchaseNumberForLocationV3ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `number` | String | **yes** | E.164 phone number that was purchased (from the request body) |
| `locationId` | String | **yes** | Location that owns the Twilio / numbers account |
| `id` | String | **yes** | Twilio account document identifier |
| `underLcAccount` | bool | **yes** | Whether the account is managed under LC. Renamed from under_ghl_account in the legacy document. |

### `PurchasePhoneNumberBodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `phoneNumber` | String | **yes** | phoneNumber to purchase |
| `addressSid` | String | **yes** | addressSid twilio address id |
| `bundleSid` | String | **yes** | bundleSid twilio bundle id |
| `countryCode` | String | **yes** | country for which the phone numbers are being requested |
| `numberType` | JSON | **yes** | type of phone number to be purchased |
| `paymentIntentId` | String | **yes** | stripe payment intent id |
| `stripeAccountId` | String | **yes** | stripe account id |
| `paymentMethodId` | String | **yes** | stripe registered payment method id |
| `locality` | String | **yes** | locality of the user in which number is being purchased |
| `region` | String | **yes** | region of the user in which number is being purchased |
| `fingerprintId` | String | **yes** | fingerprintId is request id which is unique for every purchase number request |
| `skipLocationKYC` | bool | **yes** | Skip location-level KYC verification if agency-level compliance has already been verified |

### `RcsSenderIdResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `number` | String | **yes** | RCS sender ID |
| `numberType` | String | **yes** | Entry type |
| `friendlyName` | String | no | Human-friendly label for the sender ID |
| `rcsMeta` | JSON | no | RCS sender metadata |

