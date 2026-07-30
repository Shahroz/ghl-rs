# `store`

**18** operations / **40** models in API v2 · **18** operations / **40** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `store` cargo feature on `ghl-sdk`, then call any of the 18 generated methods on `ghl.store()`:

```toml
ghl-sdk = { version = "0.4", features = ["store"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/store/shipping-carrier` | List Shipping Carriers | `list_shipping_carriers()` | `store.get_store_shipping_carrier` |
| `POST` | `/store/shipping-carrier` | Create Shipping Carrier | `create_shipping_carrier()` | `store.post_store_shipping_carrier` |
| `DELETE` | `/store/shipping-carrier/{shippingCarrierId}` | Delete shipping carrier | `delete_shipping_carrier()` | `store.delete_store_shipping_carrier_by_shippingCarrierId` |
| `GET` | `/store/shipping-carrier/{shippingCarrierId}` | Get Shipping Carrier | `get_shipping_carrier()` | `store.get_store_shipping_carrier_by_shippingCarrierId` |
| `PUT` | `/store/shipping-carrier/{shippingCarrierId}` | Update Shipping Carrier | `update_shipping_carrier()` | `store.put_store_shipping_carrier_by_shippingCarrierId` |
| `GET` | `/store/shipping-zone` | List Shipping Zones | `list_shipping_zones()` | `store.get_store_shipping_zone` |
| `POST` | `/store/shipping-zone` | Create Shipping Zone | `create_shipping_zone()` | `store.post_store_shipping_zone` |
| `POST` | `/store/shipping-zone/shipping-rates` | Get available shipping rates | `get_available_shipping_rates()` | `store.post_store_shipping_zone_shipping_rates` |
| `DELETE` | `/store/shipping-zone/{shippingZoneId}` | Delete shipping zone | `delete_shipping_zone()` | `store.delete_store_shipping_zone_by_shippingZoneId` |
| `GET` | `/store/shipping-zone/{shippingZoneId}` | Get Shipping Zone | `get_shipping_zone()` | `store.get_store_shipping_zone_by_shippingZoneId` |
| `PUT` | `/store/shipping-zone/{shippingZoneId}` | Update Shipping Zone | `update_shipping_zone()` | `store.put_store_shipping_zone_by_shippingZoneId` |
| `GET` | `/store/shipping-zone/{shippingZoneId}/shipping-rate` | List Shipping Rates | `list_shipping_rates()` | `store.get_store_shipping_zone_by_shippingZoneId_shipping_rate` |
| `POST` | `/store/shipping-zone/{shippingZoneId}/shipping-rate` | Create Shipping Rate | `create_shipping_rate()` | `store.post_store_shipping_zone_by_shippingZoneId_shipping_rate` |
| `DELETE` | `/store/shipping-zone/{shippingZoneId}/shipping-rate/{shippingRateId}` | Delete shipping rate | `delete_shipping_rate()` | `store.delete_store_shipping_zone_by_shippingZoneId_shipping_rate_by_shippingRateId` |
| `GET` | `/store/shipping-zone/{shippingZoneId}/shipping-rate/{shippingRateId}` | Get Shipping Rate | `get_shipping_rate()` | `store.get_store_shipping_zone_by_shippingZoneId_shipping_rate_by_shippingRateId` |
| `PUT` | `/store/shipping-zone/{shippingZoneId}/shipping-rate/{shippingRateId}` | Update Shipping Rate | `update_shipping_rate()` | `store.put_store_shipping_zone_by_shippingZoneId_shipping_rate_by_shippingRateId` |
| `GET` | `/store/store-setting` | Get Store Settings | `get_store_settings()` | `store.get_store_store_setting` |
| `POST` | `/store/store-setting` | Create/Update Store Settings | `create_update_store_settings()` | `store.post_store_store_setting` |

### Endpoint details — v2

#### `GET /store/shipping-carrier`

**List Shipping Carriers**

The "List Shipping Carrier" API allows to retrieve a list of shipping carrier.

Operation id: `store.get_store_shipping_carrier`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`ListShippingCarrierResponseDto`](#listshippingcarrierresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::store::ListShippingCarriersParams;

let params = ListShippingCarriersParams::new("altId", "altType");
let out = ghl.store().list_shipping_carriers(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "store.get_store_shipping_carrier",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `POST /store/shipping-carrier`

**Create Shipping Carrier**

The "Create Shipping Carrier" API allows adding a new shipping carrier.

Operation id: `store.post_store_shipping_carrier`

*Request body*: [`CreateShippingCarrierDto`](#createshippingcarrierdto)

*Response*: [`CreateShippingCarrierResponseDto`](#createshippingcarrierresponsedto)

*Rust*:

```rust,ignore
let out = ghl.store().create_shipping_carrier(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "store.post_store_shipping_carrier",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /store/shipping-carrier/{shippingCarrierId}`

**Delete shipping carrier**

Delete specific shipping carrier with Id :shippingCarrierId

Operation id: `store.delete_store_shipping_carrier_by_shippingCarrierId`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingCarrierId` | string | **yes** | ID of the shipping carrier that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`DeleteShippingCarrierResponseDto`](#deleteshippingcarrierresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::store::DeleteShippingCarrierParams;

let params = DeleteShippingCarrierParams::new("altId", "altType");
let out = ghl.store().delete_shipping_carrier(&shippingCarrierId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "store.delete_store_shipping_carrier_by_shippingCarrierId",
    "path_params": {
      "shippingCarrierId": "<shippingCarrierId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /store/shipping-carrier/{shippingCarrierId}`

**Get Shipping Carrier**

The "List Shipping Carrier" API allows to retrieve a paginated list of shipping carrier.

Operation id: `store.get_store_shipping_carrier_by_shippingCarrierId`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingCarrierId` | string | **yes** | ID of the shipping carrier that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`GetShippingCarrierResponseDto`](#getshippingcarrierresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::store::GetShippingCarrierParams;

let params = GetShippingCarrierParams::new("altId", "altType");
let out = ghl.store().get_shipping_carrier(&shippingCarrierId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "store.get_store_shipping_carrier_by_shippingCarrierId",
    "path_params": {
      "shippingCarrierId": "<shippingCarrierId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `PUT /store/shipping-carrier/{shippingCarrierId}`

**Update Shipping Carrier**

The "update Shipping Carrier" API allows update a shipping carrier to the system.

Operation id: `store.put_store_shipping_carrier_by_shippingCarrierId`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingCarrierId` | string | **yes** | ID of the shipping carrier that needs to be returned |

*Request body*: [`UpdateShippingCarrierDto`](#updateshippingcarrierdto)

*Response*: [`UpdateShippingCarrierResponseDto`](#updateshippingcarrierresponsedto)

*Rust*:

```rust,ignore
let out = ghl.store().update_shipping_carrier(&shippingCarrierId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "store.put_store_shipping_carrier_by_shippingCarrierId",
    "path_params": {
      "shippingCarrierId": "<shippingCarrierId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /store/shipping-zone`

**List Shipping Zones**

The "List Shipping Zone" API allows to retrieve a list of shipping zone.

Operation id: `store.get_store_shipping_zone`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |
| `withShippingRate` | boolean | no | Include shipping rates array |

*Response*: [`ListShippingZoneResponseDto`](#listshippingzoneresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::store::ListShippingZonesParams;

let params = ListShippingZonesParams::new("altId", "altType");
let out = ghl.store().list_shipping_zones(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "store.get_store_shipping_zone",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `POST /store/shipping-zone`

**Create Shipping Zone**

The "Create Shipping Zone" API allows adding a new shipping zone.

Operation id: `store.post_store_shipping_zone`

*Request body*: [`CreateShippingZoneDto`](#createshippingzonedto)

*Response*: [`CreateShippingZoneResponseDto`](#createshippingzoneresponsedto)

*Rust*:

```rust,ignore
let out = ghl.store().create_shipping_zone(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "store.post_store_shipping_zone",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /store/shipping-zone/shipping-rates`

**Get available shipping rates**

This return available shipping rates for country based on order amount

Operation id: `store.post_store_shipping_zone_shipping_rates`

*Request body*: [`GetAvailableShippingRates`](#getavailableshippingrates)

*Response*: [`GetAvailableShippingRatesResponseDto`](#getavailableshippingratesresponsedto)

*Rust*:

```rust,ignore
let out = ghl.store().get_available_shipping_rates(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "store.post_store_shipping_zone_shipping_rates",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /store/shipping-zone/{shippingZoneId}`

**Delete shipping zone**

Delete specific shipping zone with Id :shippingZoneId

Operation id: `store.delete_store_shipping_zone_by_shippingZoneId`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingZoneId` | string | **yes** | ID of the item that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`DeleteShippingZoneResponseDto`](#deleteshippingzoneresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::store::DeleteShippingZoneParams;

let params = DeleteShippingZoneParams::new("altId", "altType");
let out = ghl.store().delete_shipping_zone(&shippingZoneId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "store.delete_store_shipping_zone_by_shippingZoneId",
    "path_params": {
      "shippingZoneId": "<shippingZoneId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /store/shipping-zone/{shippingZoneId}`

**Get Shipping Zone**

The "List Shipping Zone" API allows to retrieve a paginated list of shipping zone.

Operation id: `store.get_store_shipping_zone_by_shippingZoneId`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingZoneId` | string | **yes** | ID of the item that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `withShippingRate` | boolean | no | Include shipping rates array |

*Response*: [`GetShippingZoneResponseDto`](#getshippingzoneresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::store::GetShippingZoneParams;

let params = GetShippingZoneParams::new("altId", "altType");
let out = ghl.store().get_shipping_zone(&shippingZoneId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "store.get_store_shipping_zone_by_shippingZoneId",
    "path_params": {
      "shippingZoneId": "<shippingZoneId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `PUT /store/shipping-zone/{shippingZoneId}`

**Update Shipping Zone**

The "update Shipping Zone" API allows update a shipping zone to the system.

Operation id: `store.put_store_shipping_zone_by_shippingZoneId`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingZoneId` | string | **yes** | ID of the item that needs to be returned |

*Request body*: [`UpdateShippingZoneDto`](#updateshippingzonedto)

*Response*: [`UpdateShippingZoneResponseDto`](#updateshippingzoneresponsedto)

*Rust*:

```rust,ignore
let out = ghl.store().update_shipping_zone(&shippingZoneId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "store.put_store_shipping_zone_by_shippingZoneId",
    "path_params": {
      "shippingZoneId": "<shippingZoneId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /store/shipping-zone/{shippingZoneId}/shipping-rate`

**List Shipping Rates**

The "List Shipping Rate" API allows to retrieve a list of shipping rate.

Operation id: `store.get_store_shipping_zone_by_shippingZoneId_shipping_rate`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingZoneId` | string | **yes** | ID of the item that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |

*Response*: [`ListShippingRateResponseDto`](#listshippingrateresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::store::ListShippingRatesParams;

let params = ListShippingRatesParams::new("altId", "altType");
let out = ghl.store().list_shipping_rates(&shippingZoneId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "store.get_store_shipping_zone_by_shippingZoneId_shipping_rate",
    "path_params": {
      "shippingZoneId": "<shippingZoneId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `POST /store/shipping-zone/{shippingZoneId}/shipping-rate`

**Create Shipping Rate**

The "Create Shipping Rate" API allows adding a new shipping rate.

Operation id: `store.post_store_shipping_zone_by_shippingZoneId_shipping_rate`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingZoneId` | string | **yes** | ID of the item that needs to be returned |

*Request body*: [`CreateShippingRateDto`](#createshippingratedto)

*Response*: [`CreateShippingRateResponseDto`](#createshippingrateresponsedto)

*Rust*:

```rust,ignore
let out = ghl.store().create_shipping_rate(&shippingZoneId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "store.post_store_shipping_zone_by_shippingZoneId_shipping_rate",
    "path_params": {
      "shippingZoneId": "<shippingZoneId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /store/shipping-zone/{shippingZoneId}/shipping-rate/{shippingRateId}`

**Delete shipping rate**

Delete specific shipping rate with Id :shippingRateId

Operation id: `store.delete_store_shipping_zone_by_shippingZoneId_shipping_rate_by_shippingRateId`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingZoneId` | string | **yes** | ID of the shipping zone |
| `shippingRateId` | string | **yes** | ID of the shipping rate that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`DeleteShippingRateResponseDto`](#deleteshippingrateresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::store::DeleteShippingRateParams;

let params = DeleteShippingRateParams::new("altId", "altType");
let out = ghl.store().delete_shipping_rate(&shippingZoneId, &shippingRateId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "store.delete_store_shipping_zone_by_shippingZoneId_shipping_rate_by_shippingRateId",
    "path_params": {
      "shippingZoneId": "<shippingZoneId>",
      "shippingRateId": "<shippingRateId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /store/shipping-zone/{shippingZoneId}/shipping-rate/{shippingRateId}`

**Get Shipping Rate**

The "List Shipping Rate" API allows to retrieve a paginated list of shipping rate.

Operation id: `store.get_store_shipping_zone_by_shippingZoneId_shipping_rate_by_shippingRateId`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingZoneId` | string | **yes** | ID of the shipping zone |
| `shippingRateId` | string | **yes** | ID of the shipping rate that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`GetShippingRateResponseDto`](#getshippingrateresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::store::GetShippingRateParams;

let params = GetShippingRateParams::new("altId", "altType");
let out = ghl.store().get_shipping_rate(&shippingZoneId, &shippingRateId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "store.get_store_shipping_zone_by_shippingZoneId_shipping_rate_by_shippingRateId",
    "path_params": {
      "shippingZoneId": "<shippingZoneId>",
      "shippingRateId": "<shippingRateId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `PUT /store/shipping-zone/{shippingZoneId}/shipping-rate/{shippingRateId}`

**Update Shipping Rate**

The "update Shipping Rate" API allows update a shipping rate to the system.

Operation id: `store.put_store_shipping_zone_by_shippingZoneId_shipping_rate_by_shippingRateId`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingZoneId` | string | **yes** | ID of the shipping zone |
| `shippingRateId` | string | **yes** | ID of the shipping rate that needs to be returned |

*Request body*: [`UpdateShippingRateDto`](#updateshippingratedto)

*Response*: [`UpdateShippingRateResponseDto`](#updateshippingrateresponsedto)

*Rust*:

```rust,ignore
let out = ghl.store().update_shipping_rate(&shippingZoneId, &shippingRateId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "store.put_store_shipping_zone_by_shippingZoneId_shipping_rate_by_shippingRateId",
    "path_params": {
      "shippingZoneId": "<shippingZoneId>",
      "shippingRateId": "<shippingRateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /store/store-setting`

**Get Store Settings**

Get store settings by altId and altType.

Operation id: `store.get_store_store_setting`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`GetStoreSettingResponseDto`](#getstoresettingresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::store::GetStoreSettingsParams;

let params = GetStoreSettingsParams::new("altId", "altType");
let out = ghl.store().get_store_settings(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "store.get_store_store_setting",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `POST /store/store-setting`

**Create/Update Store Settings**

Create or update store settings by altId and altType.

Operation id: `store.post_store_store_setting`

*Request body*: [`CreateStoreSettingDto`](#createstoresettingdto)

*Response*: [`CreateStoreSettingResponseDto`](#createstoresettingresponsedto)

*Rust*:

```rust,ignore
let out = ghl.store().create_update_store_settings(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "store.post_store_store_setting",
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
| `GET` | `/store/shipping-carrier` | List Shipping Carriers | `v3:store.get_store_shipping_carrier` |
| `POST` | `/store/shipping-carrier` | Create Shipping Carrier | `v3:store.post_store_shipping_carrier` |
| `DELETE` | `/store/shipping-carrier/{shippingCarrierId}` | Delete shipping carrier | `v3:store.delete_store_shipping_carrier_by_shippingCarrierId` |
| `GET` | `/store/shipping-carrier/{shippingCarrierId}` | Get Shipping Carrier | `v3:store.get_store_shipping_carrier_by_shippingCarrierId` |
| `PUT` | `/store/shipping-carrier/{shippingCarrierId}` | Update Shipping Carrier | `v3:store.put_store_shipping_carrier_by_shippingCarrierId` |
| `GET` | `/store/shipping-zone` | List Shipping Zones | `v3:store.get_store_shipping_zone` |
| `POST` | `/store/shipping-zone` | Create Shipping Zone | `v3:store.post_store_shipping_zone` |
| `POST` | `/store/shipping-zone/shipping-rates` | Get available shipping rates | `v3:store.post_store_shipping_zone_shipping_rates` |
| `DELETE` | `/store/shipping-zone/{shippingZoneId}` | Delete shipping zone | `v3:store.delete_store_shipping_zone_by_shippingZoneId` |
| `GET` | `/store/shipping-zone/{shippingZoneId}` | Get Shipping Zone | `v3:store.get_store_shipping_zone_by_shippingZoneId` |
| `PUT` | `/store/shipping-zone/{shippingZoneId}` | Update Shipping Zone | `v3:store.put_store_shipping_zone_by_shippingZoneId` |
| `GET` | `/store/shipping-zone/{shippingZoneId}/shipping-rate` | List Shipping Rates | `v3:store.get_store_shipping_zone_by_shippingZoneId_shipping_rate` |
| `POST` | `/store/shipping-zone/{shippingZoneId}/shipping-rate` | Create Shipping Rate | `v3:store.post_store_shipping_zone_by_shippingZoneId_shipping_rate` |
| `DELETE` | `/store/shipping-zone/{shippingZoneId}/shipping-rate/{shippingRateId}` | Delete shipping rate | `v3:store.delete_store_shipping_zone_by_shippingZoneId_shipping_rate_by_shippingRateId` |
| `GET` | `/store/shipping-zone/{shippingZoneId}/shipping-rate/{shippingRateId}` | Get Shipping Rate | `v3:store.get_store_shipping_zone_by_shippingZoneId_shipping_rate_by_shippingRateId` |
| `PUT` | `/store/shipping-zone/{shippingZoneId}/shipping-rate/{shippingRateId}` | Update Shipping Rate | `v3:store.put_store_shipping_zone_by_shippingZoneId_shipping_rate_by_shippingRateId` |
| `GET` | `/store/store-setting` | Get Store Settings | `v3:store.get_store_store_setting` |
| `POST` | `/store/store-setting` | Create/Update Store Settings | `v3:store.post_store_store_setting` |

### Endpoint details — v3

#### `GET /store/shipping-carrier`

**List Shipping Carriers**

The "List Shipping Carrier" API allows to retrieve a list of shipping carrier.

Operation id: `v3:store.get_store_shipping_carrier` · `Version: v3`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`ListShippingCarrierResponseDto`](#listshippingcarrierresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:store.get_store_shipping_carrier",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `POST /store/shipping-carrier`

**Create Shipping Carrier**

The "Create Shipping Carrier" API allows adding a new shipping carrier.

Operation id: `v3:store.post_store_shipping_carrier` · `Version: v3`

*Request body*: [`CreateShippingCarrierDto`](#createshippingcarrierdto)

*Response*: [`CreateShippingCarrierResponseDto`](#createshippingcarrierresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:store.post_store_shipping_carrier",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /store/shipping-carrier/{shippingCarrierId}`

**Delete shipping carrier**

Delete specific shipping carrier with Id :shippingCarrierId

Operation id: `v3:store.delete_store_shipping_carrier_by_shippingCarrierId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingCarrierId` | string | **yes** | ID of the shipping carrier that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`DeleteShippingCarrierResponseDto`](#deleteshippingcarrierresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:store.delete_store_shipping_carrier_by_shippingCarrierId",
    "path_params": {
      "shippingCarrierId": "<shippingCarrierId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /store/shipping-carrier/{shippingCarrierId}`

**Get Shipping Carrier**

The "List Shipping Carrier" API allows to retrieve a paginated list of shipping carrier.

Operation id: `v3:store.get_store_shipping_carrier_by_shippingCarrierId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingCarrierId` | string | **yes** | ID of the shipping carrier that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`GetShippingCarrierResponseDto`](#getshippingcarrierresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:store.get_store_shipping_carrier_by_shippingCarrierId",
    "path_params": {
      "shippingCarrierId": "<shippingCarrierId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `PUT /store/shipping-carrier/{shippingCarrierId}`

**Update Shipping Carrier**

The "update Shipping Carrier" API allows update a shipping carrier to the system.

Operation id: `v3:store.put_store_shipping_carrier_by_shippingCarrierId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingCarrierId` | string | **yes** | ID of the shipping carrier that needs to be returned |

*Request body*: [`UpdateShippingCarrierDto`](#updateshippingcarrierdto)

*Response*: [`UpdateShippingCarrierResponseDto`](#updateshippingcarrierresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:store.put_store_shipping_carrier_by_shippingCarrierId",
    "path_params": {
      "shippingCarrierId": "<shippingCarrierId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /store/shipping-zone`

**List Shipping Zones**

The "List Shipping Zone" API allows to retrieve a list of shipping zone.

Operation id: `v3:store.get_store_shipping_zone` · `Version: v3`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |
| `withShippingRate` | boolean | no | Include shipping rates array |

*Response*: [`ListShippingZoneResponseDto`](#listshippingzoneresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:store.get_store_shipping_zone",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `POST /store/shipping-zone`

**Create Shipping Zone**

The "Create Shipping Zone" API allows adding a new shipping zone.

Operation id: `v3:store.post_store_shipping_zone` · `Version: v3`

*Request body*: [`CreateShippingZoneDto`](#createshippingzonedto)

*Response*: [`CreateShippingZoneResponseDto`](#createshippingzoneresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:store.post_store_shipping_zone",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /store/shipping-zone/shipping-rates`

**Get available shipping rates**

This return available shipping rates for country based on order amount

Operation id: `v3:store.post_store_shipping_zone_shipping_rates` · `Version: v3`

*Request body*: [`GetAvailableShippingRates`](#getavailableshippingrates)

*Response*: [`GetAvailableShippingRatesResponseDto`](#getavailableshippingratesresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:store.post_store_shipping_zone_shipping_rates",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /store/shipping-zone/{shippingZoneId}`

**Delete shipping zone**

Delete specific shipping zone with Id :shippingZoneId

Operation id: `v3:store.delete_store_shipping_zone_by_shippingZoneId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingZoneId` | string | **yes** | ID of the item that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`DeleteShippingZoneResponseDto`](#deleteshippingzoneresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:store.delete_store_shipping_zone_by_shippingZoneId",
    "path_params": {
      "shippingZoneId": "<shippingZoneId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /store/shipping-zone/{shippingZoneId}`

**Get Shipping Zone**

The "List Shipping Zone" API allows to retrieve a paginated list of shipping zone.

Operation id: `v3:store.get_store_shipping_zone_by_shippingZoneId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingZoneId` | string | **yes** | ID of the item that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `withShippingRate` | boolean | no | Include shipping rates array |

*Response*: [`GetShippingZoneResponseDto`](#getshippingzoneresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:store.get_store_shipping_zone_by_shippingZoneId",
    "path_params": {
      "shippingZoneId": "<shippingZoneId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `PUT /store/shipping-zone/{shippingZoneId}`

**Update Shipping Zone**

The "update Shipping Zone" API allows update a shipping zone to the system.

Operation id: `v3:store.put_store_shipping_zone_by_shippingZoneId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingZoneId` | string | **yes** | ID of the item that needs to be returned |

*Request body*: [`UpdateShippingZoneDto`](#updateshippingzonedto)

*Response*: [`UpdateShippingZoneResponseDto`](#updateshippingzoneresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:store.put_store_shipping_zone_by_shippingZoneId",
    "path_params": {
      "shippingZoneId": "<shippingZoneId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /store/shipping-zone/{shippingZoneId}/shipping-rate`

**List Shipping Rates**

The "List Shipping Rate" API allows to retrieve a list of shipping rate.

Operation id: `v3:store.get_store_shipping_zone_by_shippingZoneId_shipping_rate` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingZoneId` | string | **yes** | ID of the item that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |

*Response*: [`ListShippingRateResponseDto`](#listshippingrateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:store.get_store_shipping_zone_by_shippingZoneId_shipping_rate",
    "path_params": {
      "shippingZoneId": "<shippingZoneId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `POST /store/shipping-zone/{shippingZoneId}/shipping-rate`

**Create Shipping Rate**

The "Create Shipping Rate" API allows adding a new shipping rate.

Operation id: `v3:store.post_store_shipping_zone_by_shippingZoneId_shipping_rate` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingZoneId` | string | **yes** | ID of the item that needs to be returned |

*Request body*: [`CreateShippingRateDto`](#createshippingratedto)

*Response*: [`CreateShippingRateResponseDto`](#createshippingrateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:store.post_store_shipping_zone_by_shippingZoneId_shipping_rate",
    "path_params": {
      "shippingZoneId": "<shippingZoneId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /store/shipping-zone/{shippingZoneId}/shipping-rate/{shippingRateId}`

**Delete shipping rate**

Delete specific shipping rate with Id :shippingRateId

Operation id: `v3:store.delete_store_shipping_zone_by_shippingZoneId_shipping_rate_by_shippingRateId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingZoneId` | string | **yes** | ID of the shipping zone |
| `shippingRateId` | string | **yes** | ID of the shipping rate that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`DeleteShippingRateResponseDto`](#deleteshippingrateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:store.delete_store_shipping_zone_by_shippingZoneId_shipping_rate_by_shippingRateId",
    "path_params": {
      "shippingZoneId": "<shippingZoneId>",
      "shippingRateId": "<shippingRateId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /store/shipping-zone/{shippingZoneId}/shipping-rate/{shippingRateId}`

**Get Shipping Rate**

The "List Shipping Rate" API allows to retrieve a paginated list of shipping rate.

Operation id: `v3:store.get_store_shipping_zone_by_shippingZoneId_shipping_rate_by_shippingRateId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingZoneId` | string | **yes** | ID of the shipping zone |
| `shippingRateId` | string | **yes** | ID of the shipping rate that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`GetShippingRateResponseDto`](#getshippingrateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:store.get_store_shipping_zone_by_shippingZoneId_shipping_rate_by_shippingRateId",
    "path_params": {
      "shippingZoneId": "<shippingZoneId>",
      "shippingRateId": "<shippingRateId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `PUT /store/shipping-zone/{shippingZoneId}/shipping-rate/{shippingRateId}`

**Update Shipping Rate**

The "update Shipping Rate" API allows update a shipping rate to the system.

Operation id: `v3:store.put_store_shipping_zone_by_shippingZoneId_shipping_rate_by_shippingRateId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `shippingZoneId` | string | **yes** | ID of the shipping zone |
| `shippingRateId` | string | **yes** | ID of the shipping rate that needs to be returned |

*Request body*: [`UpdateShippingRateDto`](#updateshippingratedto)

*Response*: [`UpdateShippingRateResponseDto`](#updateshippingrateresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:store.put_store_shipping_zone_by_shippingZoneId_shipping_rate_by_shippingRateId",
    "path_params": {
      "shippingZoneId": "<shippingZoneId>",
      "shippingRateId": "<shippingRateId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /store/store-setting`

**Get Store Settings**

Get store settings by altId and altType.

Operation id: `v3:store.get_store_store_setting` · `Version: v3`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`GetStoreSettingResponseDto`](#getstoresettingresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:store.get_store_store_setting",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `POST /store/store-setting`

**Create/Update Store Settings**

Create or update store settings by altId and altType.

Operation id: `v3:store.post_store_store_setting` · `Version: v3`

*Request body*: [`CreateStoreSettingDto`](#createstoresettingdto)

*Response*: [`CreateStoreSettingResponseDto`](#createstoresettingresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:store.post_store_store_setting",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::store::*` (enable the `store` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/store/).

### `AvailableShippingRate`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Name of the shipping zone |
| `description` | String | no | Delivery description |
| `currency` | String | **yes** | The currency of the amount of the rate / handling fee |
| `amount` | f64 | **yes** | The amount of the shipping rate if it is normal rate (0 means free ). Fixed Handling fee if it is a carrier rate (it will add to the carrier rate). |
| `isCarrierRate` | bool | no | is this a carrier rate |
| `shippingCarrierId` | String | **yes** | Shipping carrier id |
| `percentageOfRateFee` | f64 | no | Percentage of rate fee if it is a carrier rate. |
| `shippingCarrierServices` | Vec<ShippingCarrierServiceDto> | no | An array of items |
| `_id` | String | **yes** | The unique identifier for the product. |
| `shippingZoneId` | String | **yes** | The unique identifier for the shipping zone. |

### `ContactAddress`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Name of the customer |
| `companyName` | String | no | Name of the Company |
| `addressLine1` | String | no | Address line 1 of the customer |
| `country` | String — 247 values ([shared](shared-enums.md)) | **yes** | Country code of the customer |
| `state` | String — 771 values ([shared](shared-enums.md)) | no | State code of the customer |
| `city` | String | no | City of the customer |
| `zip` | String | no | Zip code of the customer |
| `phone` | String | no | Phone number of the customer |
| `email` | String | no | Email of the customer |

### `CreateShippingCarrierDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the shipping carrier |
| `callbackUrl` | String | **yes** | The URL endpoint that GHL needs to retrieve shipping rates. This must be a public URL. |
| `services` | Vec<ShippingCarrierServiceDto> | no | An array of available shipping carrier services |
| `allowsMultipleServiceSelection` | bool | no | The seller can choose multiple services while creating shipping rates if this is true. |

### `CreateShippingCarrierResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`ShippingCarrierSchema`](#shippingcarrierschema) | **yes** | Shipping carrier data |

### `CreateShippingRateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the shipping zone |
| `description` | String | no | Delivery description |
| `currency` | String | **yes** | The currency of the amount of the rate / handling fee |
| `amount` | f64 | **yes** | The amount of the shipping rate if it is normal rate (0 means free ). Fixed Handling fee if it is a carrier rate (it will add to the carrier rate). |
| `conditionType` | String — `none`, `price`, `weight` | **yes** | Type of condition to provide the conditional pricing |
| `minCondition` | f64 | **yes** | Minimum condition for applying this price. set 0 or null if there is no minimum |
| `maxCondition` | f64 | **yes** | Maximum condition for applying this price. set 0 or null if there is no maximum |
| `isCarrierRate` | bool | no | is this a carrier rate |
| `shippingCarrierId` | String | **yes** | Shipping carrier id |
| `percentageOfRateFee` | f64 | no | Percentage of rate fee if it is a carrier rate. |
| `shippingCarrierServices` | Vec<ShippingCarrierServiceDto> | no | An array of items |

### `CreateShippingRateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`ShippingRateSchema`](#shippingrateschema) | **yes** | Shipping zone data |

### `CreateShippingZoneDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the shipping zone |
| `countries` | Vec<ShippingZoneCountryDto> | **yes** | List of countries that are available |

### `CreateShippingZoneResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`ShippingZoneSchema`](#shippingzoneschema) | **yes** | Shipping zone data |

### `CreateStoreSettingDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `shippingOrigin` | [`StoreShippingOriginDto`](#storeshippingorigindto) | **yes** | Shipping origin address |
| `storeOrderNotification` | [`StoreOrderNotificationDto`](#storeordernotificationdto) | no | Store order notification email |
| `storeOrderFulfillmentNotification` | [`StoreOrderFulfillmentNotificationDto`](#storeorderfulfillmentnotificationdto) | no | Store order fulfillment notification email |

### `CreateStoreSettingResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`StoreSettingSchema`](#storesettingschema) | **yes** | Shipping carrier data |

### `DeleteShippingCarrierResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |

### `DeleteShippingRateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |

### `DeleteShippingZoneResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |

### `GetAvailableShippingRates`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `country` | String — 247 values ([shared](shared-enums.md)) | **yes** | Country code of the customer |
| `address` | [`ContactAddress`](#contactaddress) | no | Address of the customer |
| `amountAvailable` | String — 247 values ([shared](shared-enums.md)) | no | it will not calculate the order amount form backend if it is true |
| `totalOrderAmount` | f64 | **yes** | The amount of the price. ( min: 0.01 ) |
| `weightAvailable` | bool | no | Flag to pass when the weight is already calculated and should not calculate again |
| `totalOrderWeight` | f64 | **yes** | Estimated weight of the order calculated from the order creation side in kg(s) |
| `source` | [`OrderSource`](#ordersource) | **yes** | Source of the order |
| `products` | Vec<ProductItem> | **yes** | An array of price IDs and quantity |
| `couponCode` | String | no | Coupon code |

### `GetAvailableShippingRatesResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | Vec<AvailableShippingRate> | **yes** | Shipping rate data |

### `GetShippingCarrierResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`ShippingCarrierSchema`](#shippingcarrierschema) | **yes** | Shipping carrier data |

### `GetShippingRateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`ShippingRateSchema`](#shippingrateschema) | **yes** | Shipping zone data |

### `GetShippingZoneResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`ShippingZoneSchema`](#shippingzoneschema) | **yes** | Shipping zone data |

### `GetStoreSettingResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`StoreSettingSchema`](#storesettingschema) | **yes** | Shipping carrier data |

### `ListShippingCarrierResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | Vec<ShippingCarrierSchema> | **yes** | An array of items |

### `ListShippingRateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `total` | f64 | **yes** | Total number of items |
| `data` | Vec<ShippingRateSchema> | **yes** | An array of items |

### `ListShippingZoneResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `total` | f64 | **yes** | Total number of items |
| `data` | Vec<ShippingZoneSchema> | **yes** | An array of items |

### `OrderSource`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `funnel`, `website`, `invoice`, `calendar`, `text2Pay`, `document_contracts`, `membership`, `mobile_app`, `communities`, `point_of_sale`, `manual`, `form`, `survey`, `payment_link`, `external` | **yes** | Source of order |
| `subType` | String — `one_step_order_form`, `two_step_order_form`, `upsell`, `tap_to_pay`, `card_payment`, `store`, `contact_view`, `email_campaign`, `payments_dashboard`, `shopify`, `subscription_view`, `store_upsell`, `woocommerce`, `service`, `meeting`, `imported_csv`, `qr_code` | no | Source subtype of order |

### `ProductItem`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | id of product |
| `qty` | f64 | **yes** | No of quantities |

### `ShippingCarrierSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the shipping carrier |
| `callbackUrl` | String | **yes** | The URL endpoint that GHL needs to retrieve shipping rates. This must be a public URL. |
| `services` | Vec<ShippingCarrierServiceDto> | no | An array of available shipping carrier services |
| `allowsMultipleServiceSelection` | bool | no | The seller can choose multiple services while creating shipping rates if this is true. |
| `_id` | String | **yes** | The unique identifier for the product. |
| `marketplaceAppId` | String | **yes** | The unique identifier for the marketplace app. |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `ShippingCarrierServiceDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Name of the shipping carrier service |
| `value` | String | **yes** | Value of the shipping carrier service |

### `ShippingRateSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the shipping zone |
| `description` | String | no | Delivery description |
| `currency` | String | **yes** | The currency of the amount of the rate / handling fee |
| `amount` | f64 | **yes** | The amount of the shipping rate if it is normal rate (0 means free ). Fixed Handling fee if it is a carrier rate (it will add to the carrier rate). |
| `conditionType` | String — `none`, `price`, `weight` | **yes** | Type of condition to provide the conditional pricing |
| `minCondition` | f64 | **yes** | Minimum condition for applying this price. set 0 or null if there is no minimum |
| `maxCondition` | f64 | **yes** | Maximum condition for applying this price. set 0 or null if there is no maximum |
| `isCarrierRate` | bool | no | is this a carrier rate |
| `shippingCarrierId` | String | **yes** | Shipping carrier id |
| `percentageOfRateFee` | f64 | no | Percentage of rate fee if it is a carrier rate. |
| `shippingCarrierServices` | Vec<ShippingCarrierServiceDto> | no | An array of items |
| `_id` | String | **yes** | The unique identifier for the product. |
| `shippingZoneId` | String | **yes** | The unique identifier for the shipping zone. |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `ShippingZoneCountryDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `code` | String — `['US', 'CA', 'AF', 'AX', 'AL', 'DZ', 'AS', 'AD', 'AO', 'AI', 'AQ', 'AG', 'AR', 'AM', 'AW', 'AU', 'AT', 'AZ', 'BS', 'BH', 'BD', 'BB', 'BY', 'BE', 'BZ', 'BJ', 'BM', 'BT', 'BO', 'BA', 'BW', 'BV', 'BR', 'IO', 'BN', 'BG', 'BF', 'BI', 'KH', 'CM', 'CV', 'KY', 'CF', 'TD', 'CL', 'CN', 'CX', 'CC', 'CO', 'KM', 'CG', 'CD', 'CK', 'CR', 'CI', 'HR', 'CU', 'CY', 'CZ', 'DK', 'DJ', 'DM', 'DO', 'EC', 'EG', 'SV', 'GQ', 'ER', 'EE', 'ET', 'FK', 'FO', 'FJ', 'FI', 'FR', 'GF', 'PF', 'TF', 'GA', 'GM', 'GE', 'DE', 'GH', 'GI', 'GR', 'GL', 'GD', 'GP', 'GU', 'GT', 'GG', 'GN', 'GW', 'GY', 'HT', 'HM', 'VA', 'HN', 'HK', 'HU', 'IS', 'IN', 'ID', 'IR', 'IQ', 'IE', 'IM', 'IL', 'IT', 'JM', 'JP', 'JE', 'JO', 'KZ', 'KE', 'KI', 'KP', 'XK', 'KW', 'KG', 'LA', 'LV', 'LB', 'LS', 'LR', 'LY', 'LI', 'LT', 'LU', 'MO', 'MK', 'MG', 'MW', 'MY', 'MV', 'ML', 'MT', 'MH', 'MQ', 'MR', 'MU', 'YT', 'MX', 'FM', 'MD', 'MC', 'MN', 'ME', 'MS', 'MA', 'MZ', 'MM', 'NA', 'NR', 'NP', 'NL', 'AN', 'NC', 'NZ', 'NI', 'NE', 'NG', 'NU', 'NF', 'MP', 'NO', 'OM', 'PK', 'PW', 'PS', 'PA', 'PG', 'PY', 'PE', 'PH', 'PN', 'PL', 'PT', 'PR', 'QA', 'RE', 'RO', 'RU', 'RW', 'SH', 'KN', 'LC', 'MF', 'PM', 'VC', 'WS', 'SM', 'ST', 'SA', 'SN', 'RS', 'SC', 'SL', 'SG', 'SX', 'SK', 'SI', 'SB', 'SO', 'ZA', 'GS', 'KR', 'ES', 'LK', 'SD', 'SR', 'SJ', 'SZ', 'SE', 'CH', 'SY', 'TW', 'TJ', 'TZ', 'TH', 'TL', 'TG', 'TK', 'TO', 'TT', 'TN', 'TR', 'TM', 'TC', 'TV', 'UG', 'UA', 'AE', 'GB', 'UM', 'UY', 'UZ', 'VU', 'VE', 'VN', 'VG', 'VI', 'WF', 'EH', 'YE', 'ZM', 'ZW']` | **yes** | Country code |
| `states` | Vec<ShippingZoneCountryStateDto> | no | List of states that are available. If states is empty, then all states are available |

### `ShippingZoneCountryStateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `code` | String — 771 values ([shared](shared-enums.md)) | **yes** | State code |

### `ShippingZoneSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the shipping zone |
| `countries` | Vec<ShippingZoneCountryDto> | **yes** | List of countries that are available |
| `_id` | String | **yes** | The unique identifier for the product. |
| `shippingRates` | Vec<ShippingRateSchema> | no | Array of shipping rates under this shipping zone |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `StoreOrderFulfillmentNotificationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | Store order fulfillment notification enabled |
| `subject` | String | **yes** | Store order fulfillment email subject |
| `emailTemplateId` | String | **yes** | Email Template Id |
| `defaultEmailTemplateId` | String | **yes** | Default Email Template Id |

### `StoreOrderNotificationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | Store order notification enabled |
| `subject` | String | **yes** | Store order email subject |
| `emailTemplateId` | String | **yes** | Email Template Id |
| `defaultEmailTemplateId` | String | **yes** | Default Email Template Id |

### `StoreSettingSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `shippingOrigin` | [`StoreShippingOriginDto`](#storeshippingorigindto) | **yes** | Shipping origin address |
| `storeOrderNotification` | [`StoreOrderNotificationDto`](#storeordernotificationdto) | no | Store order notification email |
| `storeOrderFulfillmentNotification` | [`StoreOrderFulfillmentNotificationDto`](#storeorderfulfillmentnotificationdto) | no | Store order fulfillment notification email |
| `_id` | String | **yes** | The unique identifier for the settings. |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `StoreShippingOriginDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Name of the store / company |
| `country` | String — `['US', 'CA', 'AF', 'AX', 'AL', 'DZ', 'AS', 'AD', 'AO', 'AI', 'AQ', 'AG', 'AR', 'AM', 'AW', 'AU', 'AT', 'AZ', 'BS', 'BH', 'BD', 'BB', 'BY', 'BE', 'BZ', 'BJ', 'BM', 'BT', 'BO', 'BA', 'BW', 'BV', 'BR', 'IO', 'BN', 'BG', 'BF', 'BI', 'KH', 'CM', 'CV', 'KY', 'CF', 'TD', 'CL', 'CN', 'CX', 'CC', 'CO', 'KM', 'CG', 'CD', 'CK', 'CR', 'CI', 'HR', 'CU', 'CY', 'CZ', 'DK', 'DJ', 'DM', 'DO', 'EC', 'EG', 'SV', 'GQ', 'ER', 'EE', 'ET', 'FK', 'FO', 'FJ', 'FI', 'FR', 'GF', 'PF', 'TF', 'GA', 'GM', 'GE', 'DE', 'GH', 'GI', 'GR', 'GL', 'GD', 'GP', 'GU', 'GT', 'GG', 'GN', 'GW', 'GY', 'HT', 'HM', 'VA', 'HN', 'HK', 'HU', 'IS', 'IN', 'ID', 'IR', 'IQ', 'IE', 'IM', 'IL', 'IT', 'JM', 'JP', 'JE', 'JO', 'KZ', 'KE', 'KI', 'KP', 'XK', 'KW', 'KG', 'LA', 'LV', 'LB', 'LS', 'LR', 'LY', 'LI', 'LT', 'LU', 'MO', 'MK', 'MG', 'MW', 'MY', 'MV', 'ML', 'MT', 'MH', 'MQ', 'MR', 'MU', 'YT', 'MX', 'FM', 'MD', 'MC', 'MN', 'ME', 'MS', 'MA', 'MZ', 'MM', 'NA', 'NR', 'NP', 'NL', 'AN', 'NC', 'NZ', 'NI', 'NE', 'NG', 'NU', 'NF', 'MP', 'NO', 'OM', 'PK', 'PW', 'PS', 'PA', 'PG', 'PY', 'PE', 'PH', 'PN', 'PL', 'PT', 'PR', 'QA', 'RE', 'RO', 'RU', 'RW', 'SH', 'KN', 'LC', 'MF', 'PM', 'VC', 'WS', 'SM', 'ST', 'SA', 'SN', 'RS', 'SC', 'SL', 'SG', 'SX', 'SK', 'SI', 'SB', 'SO', 'ZA', 'GS', 'KR', 'ES', 'LK', 'SD', 'SR', 'SJ', 'SZ', 'SE', 'CH', 'SY', 'TW', 'TJ', 'TZ', 'TH', 'TL', 'TG', 'TK', 'TO', 'TT', 'TN', 'TR', 'TM', 'TC', 'TV', 'UG', 'UA', 'AE', 'GB', 'UM', 'UY', 'UZ', 'VU', 'VE', 'VN', 'VG', 'VI', 'WF', 'EH', 'YE', 'ZM', 'ZW']` | **yes** | Country code |
| `state` | String — 771 values ([shared](shared-enums.md)) | no | State code |
| `city` | String | **yes** | City name |
| `street1` | String | **yes** | Street address line 1 |
| `street2` | String | no | Street address line 2 |
| `zip` | String | **yes** | Zip code |
| `phone` | String | no | Business Phone Number |
| `email` | String | no | Email |

### `UpdateShippingCarrierDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | no | Location Id or Agency Id |
| `altType` | String — `location` | no | — |
| `name` | String | no | Name of the shipping carrier |
| `callbackUrl` | String | no | The URL endpoint that GHL needs to retrieve shipping rates. This must be a public URL. |
| `services` | Vec<ShippingCarrierServiceDto> | no | An array of available shipping carrier services |
| `allowsMultipleServiceSelection` | bool | no | The seller can choose multiple services while creating shipping rates if this is true. |

### `UpdateShippingCarrierResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`ShippingCarrierSchema`](#shippingcarrierschema) | **yes** | Shipping carrier data |

### `UpdateShippingRateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | no | Location Id or Agency Id |
| `altType` | String — `location` | no | — |
| `name` | String | no | Name of the shipping zone |
| `description` | String | no | Delivery description |
| `currency` | String | no | The currency of the amount of the rate / handling fee |
| `amount` | f64 | no | The amount of the shipping rate if it is normal rate (0 means free ). Fixed Handling fee if it is a carrier rate (it will add to the carrier rate). |
| `conditionType` | String — `none`, `price`, `weight` | no | Type of condition to provide the conditional pricing |
| `minCondition` | f64 | no | Minimum condition for applying this price. set 0 or null if there is no minimum |
| `maxCondition` | f64 | no | Maximum condition for applying this price. set 0 or null if there is no maximum |
| `isCarrierRate` | bool | no | is this a carrier rate |
| `shippingCarrierId` | String | no | Shipping carrier id |
| `percentageOfRateFee` | f64 | no | Percentage of rate fee if it is a carrier rate. |
| `shippingCarrierServices` | Vec<ShippingCarrierServiceDto> | no | An array of items |

### `UpdateShippingRateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`ShippingRateSchema`](#shippingrateschema) | **yes** | Shipping zone data |

### `UpdateShippingZoneDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | no | Location Id or Agency Id |
| `altType` | String — `location` | no | — |
| `name` | String | no | Name of the shipping zone |
| `countries` | Vec<ShippingZoneCountryDto> | no | List of countries that are available |

### `UpdateShippingZoneResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`ShippingZoneSchema`](#shippingzoneschema) | **yes** | Shipping zone data |

## Data models — API v3

In Rust: `ghl_models::v3::store::*` (enable the `store` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/store/).

### `AvailableShippingRate`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Name of the shipping zone |
| `description` | String | no | Delivery description |
| `currency` | String | **yes** | The currency of the amount of the rate / handling fee |
| `amount` | f64 | **yes** | The amount of the shipping rate if it is normal rate (0 means free ). Fixed Handling fee if it is a carrier rate (it will add to the carrier rate). |
| `isCarrierRate` | bool | no | is this a carrier rate |
| `shippingCarrierId` | String | **yes** | Shipping carrier id |
| `percentageOfRateFee` | f64 | no | Percentage of rate fee if it is a carrier rate. |
| `shippingCarrierServices` | Vec<ShippingCarrierServiceDto> | no | An array of items |
| `_id` | String | **yes** | The unique identifier for the product. |
| `shippingZoneId` | String | **yes** | The unique identifier for the shipping zone. |

### `ContactAddress`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Name of the customer |
| `companyName` | String | no | Name of the Company |
| `addressLine1` | String | no | Address line 1 of the customer |
| `country` | String — 247 values ([shared](shared-enums.md)) | **yes** | Country code of the customer |
| `state` | String — 771 values ([shared](shared-enums.md)) | no | State code of the customer |
| `city` | String | no | City of the customer |
| `zip` | String | no | Zip code of the customer |
| `phone` | String | no | Phone number of the customer |
| `email` | String | no | Email of the customer |

### `CreateShippingCarrierDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the shipping carrier |
| `callbackUrl` | String | **yes** | The URL endpoint that CRM needs to retrieve shipping rates. This must be a public URL. |
| `services` | Vec<ShippingCarrierServiceDto> | no | An array of available shipping carrier services |
| `allowsMultipleServiceSelection` | bool | no | The seller can choose multiple services while creating shipping rates if this is true. |

### `CreateShippingCarrierResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`ShippingCarrierSchema`](#shippingcarrierschema) | **yes** | Shipping carrier data |

### `CreateShippingRateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the shipping zone |
| `description` | String | no | Delivery description |
| `currency` | String | **yes** | The currency of the amount of the rate / handling fee |
| `amount` | f64 | **yes** | The amount of the shipping rate if it is normal rate (0 means free ). Fixed Handling fee if it is a carrier rate (it will add to the carrier rate). |
| `conditionType` | String — `none`, `price`, `weight` | **yes** | Type of condition to provide the conditional pricing |
| `minCondition` | f64 | **yes** | Minimum condition for applying this price. set 0 or null if there is no minimum |
| `maxCondition` | f64 | **yes** | Maximum condition for applying this price. set 0 or null if there is no maximum |
| `isCarrierRate` | bool | no | is this a carrier rate |
| `shippingCarrierId` | String | **yes** | Shipping carrier id |
| `percentageOfRateFee` | f64 | no | Percentage of rate fee if it is a carrier rate. |
| `shippingCarrierServices` | Vec<ShippingCarrierServiceDto> | no | An array of items |

### `CreateShippingRateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`ShippingRateSchema`](#shippingrateschema) | **yes** | Shipping zone data |

### `CreateShippingZoneDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the shipping zone |
| `countries` | Vec<ShippingZoneCountryDto> | **yes** | List of countries that are available |

### `CreateShippingZoneResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`ShippingZoneSchema`](#shippingzoneschema) | **yes** | Shipping zone data |

### `CreateStoreSettingDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `shippingOrigin` | [`StoreShippingOriginDto`](#storeshippingorigindto) | **yes** | Shipping origin address |
| `storeOrderNotification` | [`StoreOrderNotificationDto`](#storeordernotificationdto) | no | Store order notification email |
| `storeOrderFulfillmentNotification` | [`StoreOrderFulfillmentNotificationDto`](#storeorderfulfillmentnotificationdto) | no | Store order fulfillment notification email |

### `CreateStoreSettingResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`StoreSettingSchema`](#storesettingschema) | **yes** | Shipping carrier data |

### `DeleteShippingCarrierResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |

### `DeleteShippingRateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |

### `DeleteShippingZoneResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |

### `GetAvailableShippingRates`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `country` | String — 247 values ([shared](shared-enums.md)) | **yes** | Country code of the customer |
| `address` | [`ContactAddress`](#contactaddress) | no | Address of the customer |
| `amountAvailable` | String — 247 values ([shared](shared-enums.md)) | no | it will not calculate the order amount form backend if it is true |
| `totalOrderAmount` | f64 | **yes** | The amount of the price. ( min: 0.01 ) |
| `weightAvailable` | bool | no | Flag to pass when the weight is already calculated and should not calculate again |
| `totalOrderWeight` | f64 | **yes** | Estimated weight of the order calculated from the order creation side in kg(s) |
| `source` | [`OrderSource`](#ordersource) | **yes** | Source of the order |
| `products` | Vec<ProductItem> | **yes** | An array of price IDs and quantity |
| `couponCode` | String | no | Coupon code |

### `GetAvailableShippingRatesResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | Vec<AvailableShippingRate> | **yes** | Shipping rate data |

### `GetShippingCarrierResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`ShippingCarrierSchema`](#shippingcarrierschema) | **yes** | Shipping carrier data |

### `GetShippingRateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`ShippingRateSchema`](#shippingrateschema) | **yes** | Shipping zone data |

### `GetShippingZoneResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`ShippingZoneSchema`](#shippingzoneschema) | **yes** | Shipping zone data |

### `GetStoreSettingResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`StoreSettingSchema`](#storesettingschema) | **yes** | Shipping carrier data |

### `ListShippingCarrierResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | Vec<ShippingCarrierSchema> | **yes** | An array of items |

### `ListShippingRateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `total` | f64 | **yes** | Total number of items |
| `data` | Vec<ShippingRateSchema> | **yes** | An array of items |

### `ListShippingZoneResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `total` | f64 | **yes** | Total number of items |
| `data` | Vec<ShippingZoneSchema> | **yes** | An array of items |

### `OrderSource`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `funnel`, `website`, `invoice`, `calendar`, `text2Pay`, `document_contracts`, `membership`, `mobile_app`, `communities`, `point_of_sale`, `manual`, `form`, `survey`, `payment_link`, `external` | **yes** | Source of order |
| `subType` | String — `one_step_order_form`, `two_step_order_form`, `upsell`, `tap_to_pay`, `card_payment`, `store`, `contact_view`, `email_campaign`, `payments_dashboard`, `shopify`, `subscription_view`, `store_upsell`, `woocommerce`, `service`, `meeting`, `imported_csv`, `qr_code` | no | Source subtype of order |

### `ProductItem`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | id of product |
| `qty` | f64 | **yes** | No of quantities |

### `ShippingCarrierSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the shipping carrier |
| `callbackUrl` | String | **yes** | The URL endpoint that CRM needs to retrieve shipping rates. This must be a public URL. |
| `services` | Vec<ShippingCarrierServiceDto> | no | An array of available shipping carrier services |
| `allowsMultipleServiceSelection` | bool | no | The seller can choose multiple services while creating shipping rates if this is true. |
| `_id` | String | **yes** | The unique identifier for the product. |
| `marketplaceAppId` | String | **yes** | The unique identifier for the marketplace app. |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `ShippingCarrierServiceDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Name of the shipping carrier service |
| `value` | String | **yes** | Value of the shipping carrier service |

### `ShippingRateSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the shipping zone |
| `description` | String | no | Delivery description |
| `currency` | String | **yes** | The currency of the amount of the rate / handling fee |
| `amount` | f64 | **yes** | The amount of the shipping rate if it is normal rate (0 means free ). Fixed Handling fee if it is a carrier rate (it will add to the carrier rate). |
| `conditionType` | String — `none`, `price`, `weight` | **yes** | Type of condition to provide the conditional pricing |
| `minCondition` | f64 | **yes** | Minimum condition for applying this price. set 0 or null if there is no minimum |
| `maxCondition` | f64 | **yes** | Maximum condition for applying this price. set 0 or null if there is no maximum |
| `isCarrierRate` | bool | no | is this a carrier rate |
| `shippingCarrierId` | String | **yes** | Shipping carrier id |
| `percentageOfRateFee` | f64 | no | Percentage of rate fee if it is a carrier rate. |
| `shippingCarrierServices` | Vec<ShippingCarrierServiceDto> | no | An array of items |
| `_id` | String | **yes** | The unique identifier for the product. |
| `shippingZoneId` | String | **yes** | The unique identifier for the shipping zone. |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `ShippingZoneCountryDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `code` | String — `['US', 'CA', 'AF', 'AX', 'AL', 'DZ', 'AS', 'AD', 'AO', 'AI', 'AQ', 'AG', 'AR', 'AM', 'AW', 'AU', 'AT', 'AZ', 'BS', 'BH', 'BD', 'BB', 'BY', 'BE', 'BZ', 'BJ', 'BM', 'BT', 'BO', 'BA', 'BW', 'BV', 'BR', 'IO', 'BN', 'BG', 'BF', 'BI', 'KH', 'CM', 'CV', 'KY', 'CF', 'TD', 'CL', 'CN', 'CX', 'CC', 'CO', 'KM', 'CG', 'CD', 'CK', 'CR', 'CI', 'HR', 'CU', 'CY', 'CZ', 'DK', 'DJ', 'DM', 'DO', 'EC', 'EG', 'SV', 'GQ', 'ER', 'EE', 'ET', 'FK', 'FO', 'FJ', 'FI', 'FR', 'GF', 'PF', 'TF', 'GA', 'GM', 'GE', 'DE', 'GH', 'GI', 'GR', 'GL', 'GD', 'GP', 'GU', 'GT', 'GG', 'GN', 'GW', 'GY', 'HT', 'HM', 'VA', 'HN', 'HK', 'HU', 'IS', 'IN', 'ID', 'IR', 'IQ', 'IE', 'IM', 'IL', 'IT', 'JM', 'JP', 'JE', 'JO', 'KZ', 'KE', 'KI', 'KP', 'XK', 'KW', 'KG', 'LA', 'LV', 'LB', 'LS', 'LR', 'LY', 'LI', 'LT', 'LU', 'MO', 'MK', 'MG', 'MW', 'MY', 'MV', 'ML', 'MT', 'MH', 'MQ', 'MR', 'MU', 'YT', 'MX', 'FM', 'MD', 'MC', 'MN', 'ME', 'MS', 'MA', 'MZ', 'MM', 'NA', 'NR', 'NP', 'NL', 'AN', 'NC', 'NZ', 'NI', 'NE', 'NG', 'NU', 'NF', 'MP', 'NO', 'OM', 'PK', 'PW', 'PS', 'PA', 'PG', 'PY', 'PE', 'PH', 'PN', 'PL', 'PT', 'PR', 'QA', 'RE', 'RO', 'RU', 'RW', 'SH', 'KN', 'LC', 'MF', 'PM', 'VC', 'WS', 'SM', 'ST', 'SA', 'SN', 'RS', 'SC', 'SL', 'SG', 'SX', 'SK', 'SI', 'SB', 'SO', 'ZA', 'GS', 'KR', 'ES', 'LK', 'SD', 'SR', 'SJ', 'SZ', 'SE', 'CH', 'SY', 'TW', 'TJ', 'TZ', 'TH', 'TL', 'TG', 'TK', 'TO', 'TT', 'TN', 'TR', 'TM', 'TC', 'TV', 'UG', 'UA', 'AE', 'GB', 'UM', 'UY', 'UZ', 'VU', 'VE', 'VN', 'VG', 'VI', 'WF', 'EH', 'YE', 'ZM', 'ZW']` | **yes** | Country code |
| `states` | Vec<ShippingZoneCountryStateDto> | no | List of states that are available. If states is empty, then all states are available |

### `ShippingZoneCountryStateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `code` | String — 771 values ([shared](shared-enums.md)) | **yes** | State code |

### `ShippingZoneSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `name` | String | **yes** | Name of the shipping zone |
| `countries` | Vec<ShippingZoneCountryDto> | **yes** | List of countries that are available |
| `_id` | String | **yes** | The unique identifier for the product. |
| `shippingRates` | Vec<ShippingRateSchema> | no | Array of shipping rates under this shipping zone |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `StoreOrderFulfillmentNotificationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | Store order fulfillment notification enabled |
| `subject` | String | **yes** | Store order fulfillment email subject |
| `emailTemplateId` | String | **yes** | Email Template Id |
| `defaultEmailTemplateId` | String | **yes** | Default Email Template Id |

### `StoreOrderNotificationDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `enabled` | bool | **yes** | Store order notification enabled |
| `subject` | String | **yes** | Store order email subject |
| `emailTemplateId` | String | **yes** | Email Template Id |
| `defaultEmailTemplateId` | String | **yes** | Default Email Template Id |

### `StoreSettingSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `shippingOrigin` | [`StoreShippingOriginDto`](#storeshippingorigindto) | **yes** | Shipping origin address |
| `storeOrderNotification` | [`StoreOrderNotificationDto`](#storeordernotificationdto) | no | Store order notification email |
| `storeOrderFulfillmentNotification` | [`StoreOrderFulfillmentNotificationDto`](#storeorderfulfillmentnotificationdto) | no | Store order fulfillment notification email |
| `_id` | String | **yes** | The unique identifier for the settings. |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `StoreShippingOriginDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Name of the store / company |
| `country` | String — `['US', 'CA', 'AF', 'AX', 'AL', 'DZ', 'AS', 'AD', 'AO', 'AI', 'AQ', 'AG', 'AR', 'AM', 'AW', 'AU', 'AT', 'AZ', 'BS', 'BH', 'BD', 'BB', 'BY', 'BE', 'BZ', 'BJ', 'BM', 'BT', 'BO', 'BA', 'BW', 'BV', 'BR', 'IO', 'BN', 'BG', 'BF', 'BI', 'KH', 'CM', 'CV', 'KY', 'CF', 'TD', 'CL', 'CN', 'CX', 'CC', 'CO', 'KM', 'CG', 'CD', 'CK', 'CR', 'CI', 'HR', 'CU', 'CY', 'CZ', 'DK', 'DJ', 'DM', 'DO', 'EC', 'EG', 'SV', 'GQ', 'ER', 'EE', 'ET', 'FK', 'FO', 'FJ', 'FI', 'FR', 'GF', 'PF', 'TF', 'GA', 'GM', 'GE', 'DE', 'GH', 'GI', 'GR', 'GL', 'GD', 'GP', 'GU', 'GT', 'GG', 'GN', 'GW', 'GY', 'HT', 'HM', 'VA', 'HN', 'HK', 'HU', 'IS', 'IN', 'ID', 'IR', 'IQ', 'IE', 'IM', 'IL', 'IT', 'JM', 'JP', 'JE', 'JO', 'KZ', 'KE', 'KI', 'KP', 'XK', 'KW', 'KG', 'LA', 'LV', 'LB', 'LS', 'LR', 'LY', 'LI', 'LT', 'LU', 'MO', 'MK', 'MG', 'MW', 'MY', 'MV', 'ML', 'MT', 'MH', 'MQ', 'MR', 'MU', 'YT', 'MX', 'FM', 'MD', 'MC', 'MN', 'ME', 'MS', 'MA', 'MZ', 'MM', 'NA', 'NR', 'NP', 'NL', 'AN', 'NC', 'NZ', 'NI', 'NE', 'NG', 'NU', 'NF', 'MP', 'NO', 'OM', 'PK', 'PW', 'PS', 'PA', 'PG', 'PY', 'PE', 'PH', 'PN', 'PL', 'PT', 'PR', 'QA', 'RE', 'RO', 'RU', 'RW', 'SH', 'KN', 'LC', 'MF', 'PM', 'VC', 'WS', 'SM', 'ST', 'SA', 'SN', 'RS', 'SC', 'SL', 'SG', 'SX', 'SK', 'SI', 'SB', 'SO', 'ZA', 'GS', 'KR', 'ES', 'LK', 'SD', 'SR', 'SJ', 'SZ', 'SE', 'CH', 'SY', 'TW', 'TJ', 'TZ', 'TH', 'TL', 'TG', 'TK', 'TO', 'TT', 'TN', 'TR', 'TM', 'TC', 'TV', 'UG', 'UA', 'AE', 'GB', 'UM', 'UY', 'UZ', 'VU', 'VE', 'VN', 'VG', 'VI', 'WF', 'EH', 'YE', 'ZM', 'ZW']` | **yes** | Country code |
| `state` | String — 771 values ([shared](shared-enums.md)) | no | State code |
| `city` | String | **yes** | City name |
| `street1` | String | **yes** | Street address line 1 |
| `street2` | String | no | Street address line 2 |
| `zip` | String | **yes** | Zip code |
| `phone` | String | no | Business Phone Number |
| `email` | String | no | Email |

### `UpdateShippingCarrierDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | no | Location Id or Agency Id |
| `altType` | String — `location` | no | — |
| `name` | String | no | Name of the shipping carrier |
| `callbackUrl` | String | no | The URL endpoint that CRM needs to retrieve shipping rates. This must be a public URL. |
| `services` | Vec<ShippingCarrierServiceDto> | no | An array of available shipping carrier services |
| `allowsMultipleServiceSelection` | bool | no | The seller can choose multiple services while creating shipping rates if this is true. |

### `UpdateShippingCarrierResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`ShippingCarrierSchema`](#shippingcarrierschema) | **yes** | Shipping carrier data |

### `UpdateShippingRateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | no | Location Id or Agency Id |
| `altType` | String — `location` | no | — |
| `name` | String | no | Name of the shipping zone |
| `description` | String | no | Delivery description |
| `currency` | String | no | The currency of the amount of the rate / handling fee |
| `amount` | f64 | no | The amount of the shipping rate if it is normal rate (0 means free ). Fixed Handling fee if it is a carrier rate (it will add to the carrier rate). |
| `conditionType` | String — `none`, `price`, `weight` | no | Type of condition to provide the conditional pricing |
| `minCondition` | f64 | no | Minimum condition for applying this price. set 0 or null if there is no minimum |
| `maxCondition` | f64 | no | Maximum condition for applying this price. set 0 or null if there is no maximum |
| `isCarrierRate` | bool | no | is this a carrier rate |
| `shippingCarrierId` | String | no | Shipping carrier id |
| `percentageOfRateFee` | f64 | no | Percentage of rate fee if it is a carrier rate. |
| `shippingCarrierServices` | Vec<ShippingCarrierServiceDto> | no | An array of items |

### `UpdateShippingRateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`ShippingRateSchema`](#shippingrateschema) | **yes** | Shipping zone data |

### `UpdateShippingZoneDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | no | Location Id or Agency Id |
| `altType` | String — `location` | no | — |
| `name` | String | no | Name of the shipping zone |
| `countries` | Vec<ShippingZoneCountryDto> | no | List of countries that are available |

### `UpdateShippingZoneResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |
| `data` | [`ShippingZoneSchema`](#shippingzoneschema) | **yes** | Shipping zone data |

