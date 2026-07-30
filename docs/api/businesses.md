# `businesses`

**5** operations / **8** models in API v2 · **5** operations / **8** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `businesses` cargo feature on `ghl-sdk`, then call any of the 5 generated methods on `ghl.businesses()`:

```toml
ghl-sdk = { version = "0.4", features = ["businesses"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/businesses/` | Get Businesses by Location | `get_businesses_by_location()` | `businesses.get_businesses` |
| `POST` | `/businesses/` | Create Business | `create_business()` | `businesses.post_businesses` |
| `DELETE` | `/businesses/{businessId}` | Delete Business | `delete_business()` | `businesses.delete_businesses_by_businessId` |
| `GET` | `/businesses/{businessId}` | Get Business | `get_business()` | `businesses.get_businesses_by_businessId` |
| `PUT` | `/businesses/{businessId}` | Update Business | `update_business()` | `businesses.put_businesses_by_businessId` |

### Endpoint details — v2

#### `GET /businesses/`

**Get Businesses by Location**

Operation id: `businesses.get_businesses` · `Version: 2021-07-28` · Scopes: `businesses.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `limit` | string | no | — |
| `skip` | string | no | — |

*Response*: [`GetBusinessByLocationResponseDto`](#getbusinessbylocationresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::businesses::GetBusinessesByLocationParams;

let params = GetBusinessesByLocationParams::new("locationId");
let out = ghl.businesses().get_businesses_by_location(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "businesses.get_businesses",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /businesses/`

**Create Business**

Operation id: `businesses.post_businesses` · `Version: 2021-07-28` · Scopes: `businesses.write`

*Request body*: [`CreateBusinessDto`](#createbusinessdto)

*Response*: [`UpdateBusinessResponseDto`](#updatebusinessresponsedto)

*Rust*:

```rust,ignore
let out = ghl.businesses().create_business(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "businesses.post_businesses",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /businesses/{businessId}`

**Delete Business**

Operation id: `businesses.delete_businesses_by_businessId` · `Version: 2021-07-28` · Scopes: `businesses.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `businessId` | string | **yes** | — |

*Response*: [`DeleteBusinessResponseDto`](#deletebusinessresponsedto)

*Rust*:

```rust,ignore
let out = ghl.businesses().delete_business(&businessId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "businesses.delete_businesses_by_businessId",
    "path_params": {
      "businessId": "<businessId>"
    }
  }
}
```

</details>

#### `GET /businesses/{businessId}`

**Get Business**

Operation id: `businesses.get_businesses_by_businessId` · `Version: 2021-07-28` · Scopes: `businesses.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `businessId` | string | **yes** | — |

*Response*: [`GetBusinessByIdResponseDto`](#getbusinessbyidresponsedto)

*Rust*:

```rust,ignore
let out = ghl.businesses().get_business(&businessId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "businesses.get_businesses_by_businessId",
    "path_params": {
      "businessId": "<businessId>"
    }
  }
}
```

</details>

#### `PUT /businesses/{businessId}`

**Update Business**

Operation id: `businesses.put_businesses_by_businessId` · `Version: 2021-07-28` · Scopes: `businesses.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `businessId` | string | **yes** | — |

*Request body*: [`UpdateBusinessDto`](#updatebusinessdto)

*Response*: [`UpdateBusinessResponseDto`](#updatebusinessresponsedto)

*Rust*:

```rust,ignore
let out = ghl.businesses().update_business(&businessId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "businesses.put_businesses_by_businessId",
    "path_params": {
      "businessId": "<businessId>"
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
| `GET` | `/businesses/` | Get Businesses by Location | `v3:businesses.get_businesses` |
| `POST` | `/businesses/` | Create Business | `v3:businesses.post_businesses` |
| `DELETE` | `/businesses/{businessId}` | Delete Business | `v3:businesses.delete_businesses_by_businessId` |
| `GET` | `/businesses/{businessId}` | Get Business | `v3:businesses.get_businesses_by_businessId` |
| `PUT` | `/businesses/{businessId}` | Update Business | `v3:businesses.put_businesses_by_businessId` |

### Endpoint details — v3

#### `GET /businesses/`

**Get Businesses by Location**

Operation id: `v3:businesses.get_businesses` · `Version: v3` · Scopes: `businesses.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `limit` | string | no | — |
| `skip` | string | no | — |

*Response*: [`GetBusinessByLocationResponseDto`](#getbusinessbylocationresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:businesses.get_businesses",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /businesses/`

**Create Business**

Operation id: `v3:businesses.post_businesses` · `Version: v3` · Scopes: `businesses.write`

*Request body*: [`CreateBusinessDto`](#createbusinessdto)

*Response*: [`UpdateBusinessResponseDto`](#updatebusinessresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:businesses.post_businesses",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /businesses/{businessId}`

**Delete Business**

Operation id: `v3:businesses.delete_businesses_by_businessId` · `Version: v3` · Scopes: `businesses.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `businessId` | string | **yes** | — |

*Response*: [`DeleteBusinessResponseDto`](#deletebusinessresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:businesses.delete_businesses_by_businessId",
    "path_params": {
      "businessId": "<businessId>"
    }
  }
}
```

</details>

#### `GET /businesses/{businessId}`

**Get Business**

Operation id: `v3:businesses.get_businesses_by_businessId` · `Version: v3` · Scopes: `businesses.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `businessId` | string | **yes** | — |

*Response*: [`GetBusinessByIdResponseDto`](#getbusinessbyidresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:businesses.get_businesses_by_businessId",
    "path_params": {
      "businessId": "<businessId>"
    }
  }
}
```

</details>

#### `PUT /businesses/{businessId}`

**Update Business**

Operation id: `v3:businesses.put_businesses_by_businessId` · `Version: v3` · Scopes: `businesses.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `businessId` | string | **yes** | — |

*Request body*: [`UpdateBusinessDto`](#updatebusinessdto)

*Response*: [`UpdateBusinessResponseDto`](#updatebusinessresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:businesses.put_businesses_by_businessId",
    "path_params": {
      "businessId": "<businessId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::businesses::*` (enable the `businesses` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/businesses/).

### `BusinessCreatedByOrUpdatedBy`

_No fields defined in the spec._

### `BusinessDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Business Id |
| `name` | String | **yes** | Business Name |
| `phone` | String | no | phone number |
| `email` | String | no | email |
| `website` | String | no | website |
| `address` | String | no | address |
| `city` | String | no | city |
| `description` | String | no | description |
| `state` | String | no | state |
| `postalCode` | String | no | postal code |
| `country` | String | no | country |
| `updatedBy` | [`BusinessCreatedByOrUpdatedBy`](#businesscreatedbyorupdatedby) | no | updated By |
| `locationId` | String | **yes** | locaitonId |
| `createdBy` | [`BusinessCreatedByOrUpdatedBy`](#businesscreatedbyorupdatedby) | no | Created By |
| `createdAt` | String | no | Creation Time |
| `updatedAt` | String | no | Last updation time |

### `CreateBusinessDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | — |
| `locationId` | String | **yes** | — |
| `phone` | String | no | — |
| `email` | String | no | — |
| `website` | String | no | — |
| `address` | String | no | — |
| `city` | String | no | — |
| `postalCode` | String | no | — |
| `state` | String | no | — |
| `country` | String | no | — |
| `description` | String | no | — |

### `DeleteBusinessResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success value |

### `GetBusinessByIdResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `business` | [`BusinessDto`](#businessdto) | **yes** | Business Response |

### `GetBusinessByLocationResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `businesses` | Vec<BusinessDto> | **yes** | Business Response |

### `UpdateBusinessDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | — |
| `phone` | String | no | — |
| `email` | String | no | — |
| `postalCode` | String | no | — |
| `website` | String | no | — |
| `address` | String | no | — |
| `state` | String | no | — |
| `city` | String | no | — |
| `country` | String | no | — |
| `description` | String | no | — |

### `UpdateBusinessResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success Value |
| `buiseness` | [`BusinessDto`](#businessdto) | **yes** | Business Response |

## Data models — API v3

In Rust: `ghl_models::v3::businesses::*` (enable the `businesses` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/businesses/).

### `BusinessCreatedByOrUpdatedBy`

_No fields defined in the spec._

### `BusinessDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Business Id |
| `name` | String | **yes** | Business Name |
| `phone` | String | no | phone number |
| `email` | String | no | email |
| `website` | String | no | website |
| `address` | String | no | address |
| `city` | String | no | city |
| `description` | String | no | description |
| `state` | String | no | state |
| `postalCode` | String | no | postal code |
| `country` | String | no | country |
| `updatedBy` | [`BusinessCreatedByOrUpdatedBy`](#businesscreatedbyorupdatedby) | no | updated By |
| `locationId` | String | **yes** | locaitonId |
| `createdBy` | [`BusinessCreatedByOrUpdatedBy`](#businesscreatedbyorupdatedby) | no | Created By |
| `createdAt` | String | no | Creation Time |
| `updatedAt` | String | no | Last updation time |

### `CreateBusinessDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | — |
| `locationId` | String | **yes** | — |
| `phone` | String | no | — |
| `email` | String | no | — |
| `website` | String | no | — |
| `address` | String | no | — |
| `city` | String | no | — |
| `postalCode` | String | no | — |
| `state` | String | no | — |
| `country` | String | no | — |
| `description` | String | no | — |

### `DeleteBusinessResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success value |

### `GetBusinessByIdResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `business` | [`BusinessDto`](#businessdto) | **yes** | Business Response |

### `GetBusinessByLocationResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `businesses` | Vec<BusinessDto> | **yes** | Business Response |

### `UpdateBusinessDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | — |
| `phone` | String | no | — |
| `email` | String | no | — |
| `postalCode` | String | no | — |
| `website` | String | no | — |
| `address` | String | no | — |
| `state` | String | no | — |
| `city` | String | no | — |
| `country` | String | no | — |
| `description` | String | no | — |

### `UpdateBusinessResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success Value |
| `buiseness` | [`BusinessDto`](#businessdto) | **yes** | Business Response |

