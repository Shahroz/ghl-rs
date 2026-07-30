# `payments`

**23** operations / **55** models in API v2 · **23** operations / **55** models in API v3

## How to call it

No hand-written service yet — reach these endpoints two ways:

**From Rust** (typed body via [`ghl-models`](https://docs.rs/ghl-models)):

```rust,ignore
// cargo add ghl-models --features payments
use ghl_models::v2::payments::*;

let body = serde_json::to_value(/* a Create…Dto from above */)?;
let out = ghl.request_raw("POST", "/path/", &[], Some(&body), None).await?;
```

**From an AI agent** (MCP meta-tools):

```json
{
  "name": "ghl_search_operations",
  "arguments": {
    "query": "",
    "module": "payments"
  }
}
```

## Endpoints — API v2

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `DELETE` | `/payments/coupon` | Delete Coupon | `payments.delete_payments_coupon` |
| `GET` | `/payments/coupon` | Fetch Coupon | `payments.get_payments_coupon` |
| `POST` | `/payments/coupon` | Create Coupon | `payments.post_payments_coupon` |
| `PUT` | `/payments/coupon` | Update Coupon | `payments.put_payments_coupon` |
| `GET` | `/payments/coupon/list` | List Coupons | `payments.get_payments_coupon_list` |
| `PUT` | `/payments/custom-provider/capabilities` | Custom-provider marketplace app update capabilities | `payments.put_payments_custom_provider_capabilities` |
| `GET` | `/payments/custom-provider/connect` | Fetch given provider config | `payments.get_payments_custom_provider_connect` |
| `POST` | `/payments/custom-provider/connect` | Create new provider config | `payments.post_payments_custom_provider_connect` |
| `POST` | `/payments/custom-provider/disconnect` | Disconnect existing provider config | `payments.post_payments_custom_provider_disconnect` |
| `DELETE` | `/payments/custom-provider/provider` | Deleting an existing integration | `payments.delete_payments_custom_provider_provider` |
| `POST` | `/payments/custom-provider/provider` | Create new integration | `payments.post_payments_custom_provider_provider` |
| `GET` | `/payments/integrations/provider/whitelabel` | List White-label Integration Providers | `payments.get_payments_integrations_provider_whitelabel` |
| `POST` | `/payments/integrations/provider/whitelabel` | Create White-label Integration Provider | `payments.post_payments_integrations_provider_whitelabel` |
| `GET` | `/payments/orders` | List Orders | `payments.get_payments_orders` |
| `GET` | `/payments/orders/{orderId}` | Get Order by ID | `payments.get_payments_orders_by_orderId` |
| `GET` | `/payments/orders/{orderId}/fulfillments` | List fulfillment | `payments.get_payments_orders_by_orderId_fulfillments` |
| `POST` | `/payments/orders/{orderId}/fulfillments` | Create order fulfillment | `payments.post_payments_orders_by_orderId_fulfillments` |
| `GET` | `/payments/orders/{orderId}/notes` | List Order Notes | `payments.get_payments_orders_by_orderId_notes` |
| `POST` | `/payments/orders/{orderId}/record-payment` | Record Order Payment | `payments.post_payments_orders_by_orderId_record_payment` |
| `GET` | `/payments/subscriptions` | List Subscriptions | `payments.get_payments_subscriptions` |
| `GET` | `/payments/subscriptions/{subscriptionId}` | Get Subscription by ID | `payments.get_payments_subscriptions_by_subscriptionId` |
| `GET` | `/payments/transactions` | List Transactions | `payments.get_payments_transactions` |
| `GET` | `/payments/transactions/{transactionId}` | Get Transaction by ID | `payments.get_payments_transactions_by_transactionId` |

### Endpoint details — v2

#### `DELETE /payments/coupon`

**Delete Coupon**

The "Delete Coupon" API allows you to permanently remove a coupon from your system using its unique identifier. Use this endpoint to discontinue promotional offers or clean up unused coupons. Note that this action cannot be undone.

Operation id: `payments.delete_payments_coupon` · `Version: 2021-07-28` · Scopes: `payments/coupons.write`

*Request body*: [`DeleteCouponParams`](#deletecouponparams)

*Response*: [`DeleteCouponResponseDto`](#deletecouponresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.delete_payments_coupon",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /payments/coupon`

**Fetch Coupon**

The "Get Coupon Details" API enables you to retrieve comprehensive information about a specific coupon using either its unique identifier or promotional code. Use this endpoint to view coupon parameters, usage statistics, validity periods, and other promotional details.

Operation id: `payments.get_payments_coupon` · `Version: 2021-07-28` · Scopes: `payments/coupons.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id |
| `altType` | enum: `location` | **yes** | Alt Type |
| `id` | string | **yes** | Coupon id |
| `code` | string | **yes** | Coupon code |

*Response*: [`CreateCouponResponseDto`](#createcouponresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.get_payments_coupon",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>",
      "id": "<id>",
      "code": "<code>"
    }
  }
}
```

</details>

#### `POST /payments/coupon`

**Create Coupon**

The "Create Coupon" API allows you to create a new promotional coupon with customizable parameters such as discount amount, validity period, usage limits, and applicable products. Use this endpoint to set up promotional offers and special discounts for your customers.

Operation id: `payments.post_payments_coupon` · `Version: 2021-07-28` · Scopes: `payments/coupons.write`

*Request body*: [`CreateCouponParams`](#createcouponparams)

*Response*: [`CreateCouponResponseDto`](#createcouponresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.post_payments_coupon",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /payments/coupon`

**Update Coupon**

The "Update Coupon" API enables you to modify existing coupon details such as discount values, validity periods, usage limits, and other promotional parameters. Use this endpoint to adjust or extend promotional offers for your customers.

Operation id: `payments.put_payments_coupon` · `Version: 2021-07-28` · Scopes: `payments/coupons.write`

*Request body*: [`UpdateCouponParams`](#updatecouponparams)

*Response*: [`CreateCouponResponseDto`](#createcouponresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.put_payments_coupon",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /payments/coupon/list`

**List Coupons**

The "List Coupons" API allows you to retrieve a list of all coupons available in your location. Use this endpoint to view all promotional offers and special discounts for your customers.

Operation id: `payments.get_payments_coupon_list` · `Version: 2021-07-28` · Scopes: `payments/coupons.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id |
| `altType` | enum: `location` | **yes** | Alt Type |
| `limit` | number | no | Maximum number of coupons to return |
| `offset` | number | no | Number of coupons to skip for pagination |
| `status` | enum: `scheduled`, `active`, `expired` | no | Filter coupons by status |
| `search` | string | no | Search term to filter coupons by name or code |

*Response*: [`ListCouponsResponseDto`](#listcouponsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.get_payments_coupon_list",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `PUT /payments/custom-provider/capabilities`

**Custom-provider marketplace app update capabilities**

Toggle capabilities for the marketplace app tied to the OAuth client

Operation id: `payments.put_payments_custom_provider_capabilities` · `Version: 2021-07-28` · Scopes: `payments/custom-provider.write`

*Request body*: [`UpdateCustomProviderCapabilitiesDto`](#updatecustomprovidercapabilitiesdto)

*Response*: [`UpdateCustomProviderCapabilitiesResponseSchema`](#updatecustomprovidercapabilitiesresponseschema)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.put_payments_custom_provider_capabilities",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /payments/custom-provider/connect`

**Fetch given provider config**

API for fetching an existing payment config for given location

Operation id: `payments.get_payments_custom_provider_connect` · `Version: 2021-07-28` · Scopes: `payments/custom-provider.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location id |

*Response*: [`GetCustomProvidersResponseSchema`](#getcustomprovidersresponseschema)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.get_payments_custom_provider_connect",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /payments/custom-provider/connect`

**Create new provider config**

API to create a new payment config for given location

Operation id: `payments.post_payments_custom_provider_connect` · `Version: 2021-07-28` · Scopes: `payments/custom-provider.write`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location id |

*Request body*: [`ConnectCustomProvidersConfigDto`](#connectcustomprovidersconfigdto)

*Response*: [`ConnectCustomProvidersResponseSchema`](#connectcustomprovidersresponseschema)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.post_payments_custom_provider_connect",
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

#### `POST /payments/custom-provider/disconnect`

**Disconnect existing provider config**

API to disconnect an existing payment config for given location

Operation id: `payments.post_payments_custom_provider_disconnect` · `Version: 2021-07-28` · Scopes: `payments/custom-provider.write`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location id |

*Request body*: [`DeleteCustomProvidersConfigDto`](#deletecustomprovidersconfigdto)

*Response*: [`DisconnectCustomProvidersResponseSchema`](#disconnectcustomprovidersresponseschema)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.post_payments_custom_provider_disconnect",
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

#### `DELETE /payments/custom-provider/provider`

**Deleting an existing integration**

API to delete an association for an app and location

Operation id: `payments.delete_payments_custom_provider_provider` · `Version: 2021-07-28` · Scopes: `payments/custom-provider.write`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location id |

*Response*: [`DeleteCustomProvidersResponseSchema`](#deletecustomprovidersresponseschema)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.delete_payments_custom_provider_provider",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /payments/custom-provider/provider`

**Create new integration**

API to create a new association for an app and location

Operation id: `payments.post_payments_custom_provider_provider` · `Version: 2021-07-28` · Scopes: `payments/custom-provider.write`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location id |

*Request body*: [`CreateCustomProvidersDto`](#createcustomprovidersdto)

*Response*: [`CreateCustomProvidersResponseSchema`](#createcustomprovidersresponseschema)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.post_payments_custom_provider_provider",
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

#### `GET /payments/integrations/provider/whitelabel`

**List White-label Integration Providers**

The "List White-label Integration Providers" API allows to retrieve a paginated list of integration providers. Customize your results by filtering whitelabel integration providers(which are built directly on top of Authorize.net or NMI) based on name or paginate through the list using the provided query parameters. This endpoint provides a straightforward way to explore and retrieve integration provider information.

Operation id: `payments.get_payments_integrations_provider_whitelabel` · `Version: 2021-07-28` · Scopes: `payments/integration.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | location Id / company Id based on altType |
| `altType` | enum: `location` | **yes** | Alt Type |
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |

*Response*: [`ListWhitelabelIntegrationProviderResponseDto`](#listwhitelabelintegrationproviderresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.get_payments_integrations_provider_whitelabel",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `POST /payments/integrations/provider/whitelabel`

**Create White-label Integration Provider**

The "Create White-label Integration Provider" API allows adding a new payment provider integration to the system which is built on top of Authorize.net or NMI. Use this endpoint to create a integration provider with the specified details. Ensure that the required information is provided in the request payload. This endpoint can be only invoked using marketplace-app token

Operation id: `payments.post_payments_integrations_provider_whitelabel` · `Version: 2021-07-28` · Scopes: `payments/integration.write`

*Request body*: [`CreateWhiteLabelIntegrationProviderDto`](#createwhitelabelintegrationproviderdto)

*Response*: [`CreateWhitelabelIntegrationResponseDto`](#createwhitelabelintegrationresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.post_payments_integrations_provider_whitelabel",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /payments/orders`

**List Orders**

The "List Orders" API allows to retrieve a paginated list of orders. Customize your results by filtering orders based on name, alt type, order status, payment mode, date range, type of source, contact, funnel products or paginate through the list using the provided query parameters. This endpoint provides a straightforward way to explore and retrieve order information.

Operation id: `payments.get_payments_orders` · `Version: 2021-07-28` · Scopes: `payments/orders.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | no | LocationId is the id of the sub-account. |
| `altId` | string | **yes** | AltId is the unique identifier e.g: location id. |
| `status` | string | no | Order status. |
| `paymentStatus` | enum: `paid`, `unpaid`, `refunded`, `partially_paid` | no | Payment Status of the Order |
| `paymentMode` | string | no | Mode of payment. |
| `startAt` | string | no | Starting interval of orders. |
| `endAt` | string | no | Closing interval of orders. |
| `search` | string | no | The name of the order for searching. |
| `contactId` | string | no | Contact id for filtering of orders. |
| `funnelProductIds` | string | no | Funnel product ids separated by comma. |
| `sourceId` | string | no | Source id |
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |

*Response*: [`ListOrdersResponseDto`](#listordersresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.get_payments_orders",
    "query": {
      "altId": "<altId>"
    }
  }
}
```

</details>

#### `GET /payments/orders/{orderId}`

**Get Order by ID**

The "Get Order by ID" API allows to retrieve information for a specific order using its unique identifier. Use this endpoint to fetch details for a single order based on the provided order ID.

Operation id: `payments.get_payments_orders_by_orderId` · `Version: 2021-07-28` · Scopes: `payments/orders.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `orderId` | string | **yes** | ID of the order that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | no | LocationId is the id of the sub-account. |
| `altId` | string | **yes** | AltId is the unique identifier e.g: location id. |

*Response*: [`GetOrderResponseSchema`](#getorderresponseschema)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.get_payments_orders_by_orderId",
    "path_params": {
      "orderId": "<orderId>"
    },
    "query": {
      "altId": "<altId>"
    }
  }
}
```

</details>

#### `GET /payments/orders/{orderId}/fulfillments`

**List fulfillment**

List all fulfillment history of an order

Operation id: `payments.get_payments_orders_by_orderId_fulfillments` · `Version: 2021-07-28` · Scopes: `payments/orders.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `orderId` | string | **yes** | ID of the order that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`ListFulfillmentResponseDto`](#listfulfillmentresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.get_payments_orders_by_orderId_fulfillments",
    "path_params": {
      "orderId": "<orderId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `POST /payments/orders/{orderId}/fulfillments`

**Create order fulfillment**

The "Order Fulfillment" API facilitates the process of fulfilling an order.

Operation id: `payments.post_payments_orders_by_orderId_fulfillments` · `Version: 2021-07-28` · Scopes: `payments/orders.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `orderId` | string | **yes** | ID of the order that needs to be returned |

*Request body*: [`CreateFulfillmentDto`](#createfulfillmentdto)

*Response*: [`CreateFulfillmentResponseDto`](#createfulfillmentresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.post_payments_orders_by_orderId_fulfillments",
    "path_params": {
      "orderId": "<orderId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /payments/orders/{orderId}/notes`

**List Order Notes**

List all notes of an order

Operation id: `payments.get_payments_orders_by_orderId_notes` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `orderId` | string | **yes** | ID of the order that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.get_payments_orders_by_orderId_notes",
    "path_params": {
      "orderId": "<orderId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `POST /payments/orders/{orderId}/record-payment`

**Record Order Payment**

The "Record Order Payment" API allows to record a payment for an order. Use this endpoint to record payment for an order and update the order status to "Paid".

Operation id: `payments.post_payments_orders_by_orderId_record_payment` · `Version: 2021-07-28` · Scopes: `payments/orders.collectPayment`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `orderId` | string | **yes** | Order ID |

*Request body*: [`PostRecordOrderPaymentBody`](#postrecordorderpaymentbody)

*Response*: [`PostRecordOrderPaymentResponse`](#postrecordorderpaymentresponse)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.post_payments_orders_by_orderId_record_payment",
    "path_params": {
      "orderId": "<orderId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /payments/subscriptions`

**List Subscriptions**

The "List Subscriptions" API allows to retrieve a paginated list of subscriptions. Customize your results by filtering subscriptions based on name, alt type, subscription status, payment mode, date range, type of source, contact, subscription id, entity id, contact or paginate through the list using the provided query parameters. This endpoint provides a straightforward way to explore and retrieve subscription information.

Operation id: `payments.get_payments_subscriptions` · `Version: 2021-07-28` · Scopes: `payments/subscriptions.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | AltId is the unique identifier e.g: location id. |
| `altType` | enum: `location` | **yes** | AltType is the type of identifier. |
| `entityId` | string | no | Entity id for filtering of subscriptions. |
| `paymentMode` | string | no | Mode of payment. |
| `startAt` | string | no | Starting interval of subscriptions. |
| `endAt` | string | no | Closing interval of subscriptions. |
| `entitySourceType` | string | no | Source of the subscriptions. |
| `search` | string | no | The name of the subscription for searching. |
| `contactId` | string | no | Contact ID for the subscription |
| `id` | string | no | Subscription id for filtering of subscriptions. |
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |
| `getPaymentsCollectedCount` | boolean | no | Get the total payments collected for the subscription. |

*Response*: [`ListSubscriptionResponseDto`](#listsubscriptionresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.get_payments_subscriptions",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /payments/subscriptions/{subscriptionId}`

**Get Subscription by ID**

The "Get Subscription by ID" API allows to retrieve information for a specific subscription using its unique identifier. Use this endpoint to fetch details for a single subscription based on the provided subscription ID.

Operation id: `payments.get_payments_subscriptions_by_subscriptionId` · `Version: 2021-07-28` · Scopes: `payments/subscriptions.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `subscriptionId` | string | **yes** | ID of the subscription that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | AltId is the unique identifier e.g: location id. |
| `altType` | enum: `location` | **yes** | AltType is the type of identifier. |

*Response*: [`GetSubscriptionResponseSchema`](#getsubscriptionresponseschema)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.get_payments_subscriptions_by_subscriptionId",
    "path_params": {
      "subscriptionId": "<subscriptionId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /payments/transactions`

**List Transactions**

The "List Transactions" API allows to retrieve a paginated list of transactions. Customize your results by filtering transactions based on name, alt type, transaction status, payment mode, date range, type of source, contact, subscription id, entity id or paginate through the list using the provided query parameters. This endpoint provides a straightforward way to explore and retrieve transaction information.

Operation id: `payments.get_payments_transactions` · `Version: 2021-07-28` · Scopes: `payments/transactions.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | no | LocationId is the id of the sub-account. |
| `altId` | string | **yes** | AltId is the unique identifier e.g: location id. |
| `altType` | string | **yes** | AltType is the type of identifier. |
| `paymentMode` | string | no | Mode of payment. |
| `startAt` | string | no | Starting interval of transactions. |
| `endAt` | string | no | Closing interval of transactions. |
| `entitySourceType` | string | no | Source of the transactions. |
| `entitySourceSubType` | string | no | Source sub-type of the transactions. |
| `search` | string | no | The name of the transaction for searching. |
| `subscriptionId` | string | no | Subscription id for filtering of transactions. |
| `entityId` | string | no | Entity id for filtering of transactions. |
| `contactId` | string | no | Contact id for filtering of transactions. |
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |

*Response*: [`ListTxnsResponseDto`](#listtxnsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.get_payments_transactions",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /payments/transactions/{transactionId}`

**Get Transaction by ID**

The "Get Transaction by ID" API allows to retrieve information for a specific transaction using its unique identifier. Use this endpoint to fetch details for a single transaction based on the provided transaction ID.

Operation id: `payments.get_payments_transactions_by_transactionId` · `Version: 2021-07-28` · Scopes: `payments/transactions.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `transactionId` | string | **yes** | ID of the transaction that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | no | LocationId is the id of the sub-account. |
| `altId` | string | **yes** | AltId is the unique identifier e.g: location id. |
| `altType` | string | **yes** | AltType is the type of identifier. |

*Response*: [`GetTxnResponseSchema`](#gettxnresponseschema)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "payments.get_payments_transactions_by_transactionId",
    "path_params": {
      "transactionId": "<transactionId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

## Endpoints — API v3

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `DELETE` | `/payments/coupon` | Delete Coupon | `v3:payments.delete_payments_coupon` |
| `GET` | `/payments/coupon` | Fetch Coupon | `v3:payments.get_payments_coupon` |
| `POST` | `/payments/coupon` | Create Coupon | `v3:payments.post_payments_coupon` |
| `PUT` | `/payments/coupon` | Update Coupon | `v3:payments.put_payments_coupon` |
| `GET` | `/payments/coupon/list` | List Coupons | `v3:payments.get_payments_coupon_list` |
| `PUT` | `/payments/custom-provider/capabilities` | Custom-provider marketplace app update capabilities | `v3:payments.put_payments_custom_provider_capabilities` |
| `GET` | `/payments/custom-provider/connect` | Fetch given provider config | `v3:payments.get_payments_custom_provider_connect` |
| `POST` | `/payments/custom-provider/connect` | Create new provider config | `v3:payments.post_payments_custom_provider_connect` |
| `POST` | `/payments/custom-provider/disconnect` | Disconnect existing provider config | `v3:payments.post_payments_custom_provider_disconnect` |
| `DELETE` | `/payments/custom-provider/provider` | Deleting an existing integration | `v3:payments.delete_payments_custom_provider_provider` |
| `POST` | `/payments/custom-provider/provider` | Create new integration | `v3:payments.post_payments_custom_provider_provider` |
| `GET` | `/payments/integrations/provider/whitelabel` | List White-label Integration Providers | `v3:payments.get_payments_integrations_provider_whitelabel` |
| `POST` | `/payments/integrations/provider/whitelabel` | Create White-label Integration Provider | `v3:payments.post_payments_integrations_provider_whitelabel` |
| `GET` | `/payments/orders` | List Orders | `v3:payments.get_payments_orders` |
| `GET` | `/payments/orders/{orderId}` | Get Order by ID | `v3:payments.get_payments_orders_by_orderId` |
| `GET` | `/payments/orders/{orderId}/fulfillments` | List fulfillment | `v3:payments.get_payments_orders_by_orderId_fulfillments` |
| `POST` | `/payments/orders/{orderId}/fulfillments` | Create order fulfillment | `v3:payments.post_payments_orders_by_orderId_fulfillments` |
| `GET` | `/payments/orders/{orderId}/notes` | List Order Notes | `v3:payments.get_payments_orders_by_orderId_notes` |
| `POST` | `/payments/orders/{orderId}/record-payment` | Record Order Payment | `v3:payments.post_payments_orders_by_orderId_record_payment` |
| `GET` | `/payments/subscriptions` | List Subscriptions | `v3:payments.get_payments_subscriptions` |
| `GET` | `/payments/subscriptions/{subscriptionId}` | Get Subscription by ID | `v3:payments.get_payments_subscriptions_by_subscriptionId` |
| `GET` | `/payments/transactions` | List Transactions | `v3:payments.get_payments_transactions` |
| `GET` | `/payments/transactions/{transactionId}` | Get Transaction by ID | `v3:payments.get_payments_transactions_by_transactionId` |

### Endpoint details — v3

#### `DELETE /payments/coupon`

**Delete Coupon**

The "Delete Coupon" API allows you to permanently remove a coupon from your system using its unique identifier. Use this endpoint to discontinue promotional offers or clean up unused coupons. Note that this action cannot be undone.

Operation id: `v3:payments.delete_payments_coupon` · `Version: v3` · Scopes: `payments/coupons.write`

*Request body*: [`DeleteCouponParams`](#deletecouponparams)

*Response*: [`DeleteCouponResponseDto`](#deletecouponresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.delete_payments_coupon",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /payments/coupon`

**Fetch Coupon**

The "Get Coupon Details" API enables you to retrieve comprehensive information about a specific coupon using either its unique identifier or promotional code. Use this endpoint to view coupon parameters, usage statistics, validity periods, and other promotional details.

Operation id: `v3:payments.get_payments_coupon` · `Version: v3` · Scopes: `payments/coupons.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id |
| `altType` | enum: `location` | **yes** | Alt Type |
| `id` | string | **yes** | Coupon id |
| `code` | string | **yes** | Coupon code |

*Response*: [`CreateCouponResponseDto`](#createcouponresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.get_payments_coupon",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>",
      "id": "<id>",
      "code": "<code>"
    }
  }
}
```

</details>

#### `POST /payments/coupon`

**Create Coupon**

The "Create Coupon" API allows you to create a new promotional coupon with customizable parameters such as discount amount, validity period, usage limits, and applicable products. Use this endpoint to set up promotional offers and special discounts for your customers.

Operation id: `v3:payments.post_payments_coupon` · `Version: v3` · Scopes: `payments/coupons.write`

*Request body*: [`CreateCouponParams`](#createcouponparams)

*Response*: [`CreateCouponResponseDto`](#createcouponresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.post_payments_coupon",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `PUT /payments/coupon`

**Update Coupon**

The "Update Coupon" API enables you to modify existing coupon details such as discount values, validity periods, usage limits, and other promotional parameters. Use this endpoint to adjust or extend promotional offers for your customers.

Operation id: `v3:payments.put_payments_coupon` · `Version: v3` · Scopes: `payments/coupons.write`

*Request body*: [`UpdateCouponParams`](#updatecouponparams)

*Response*: [`CreateCouponResponseDto`](#createcouponresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.put_payments_coupon",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /payments/coupon/list`

**List Coupons**

The "List Coupons" API allows you to retrieve a list of all coupons available in your location. Use this endpoint to view all promotional offers and special discounts for your customers.

Operation id: `v3:payments.get_payments_coupon_list` · `Version: v3` · Scopes: `payments/coupons.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id |
| `altType` | enum: `location` | **yes** | Alt Type |
| `limit` | number | no | Maximum number of coupons to return |
| `offset` | number | no | Number of coupons to skip for pagination |
| `status` | enum: `scheduled`, `active`, `expired` | no | Filter coupons by status |
| `search` | string | no | Search term to filter coupons by name or code |

*Response*: [`ListCouponsResponseDto`](#listcouponsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.get_payments_coupon_list",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `PUT /payments/custom-provider/capabilities`

**Custom-provider marketplace app update capabilities**

Toggle capabilities for the marketplace app tied to the OAuth client

Operation id: `v3:payments.put_payments_custom_provider_capabilities` · `Version: v3` · Scopes: `payments/custom-provider.write`

*Request body*: [`UpdateCustomProviderCapabilitiesDto`](#updatecustomprovidercapabilitiesdto)

*Response*: [`UpdateCustomProviderCapabilitiesResponseSchema`](#updatecustomprovidercapabilitiesresponseschema)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.put_payments_custom_provider_capabilities",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /payments/custom-provider/connect`

**Fetch given provider config**

API for fetching an existing payment config for given location

Operation id: `v3:payments.get_payments_custom_provider_connect` · `Version: v3` · Scopes: `payments/custom-provider.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location id |

*Response*: [`GetCustomProvidersResponseSchema`](#getcustomprovidersresponseschema)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.get_payments_custom_provider_connect",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /payments/custom-provider/connect`

**Create new provider config**

API to create a new payment config for given location

Operation id: `v3:payments.post_payments_custom_provider_connect` · `Version: v3` · Scopes: `payments/custom-provider.write`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location id |

*Request body*: [`ConnectCustomProvidersConfigDto`](#connectcustomprovidersconfigdto)

*Response*: [`ConnectCustomProvidersResponseSchema`](#connectcustomprovidersresponseschema)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.post_payments_custom_provider_connect",
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

#### `POST /payments/custom-provider/disconnect`

**Disconnect existing provider config**

API to disconnect an existing payment config for given location

Operation id: `v3:payments.post_payments_custom_provider_disconnect` · `Version: v3` · Scopes: `payments/custom-provider.write`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location id |

*Request body*: [`DeleteCustomProvidersConfigDto`](#deletecustomprovidersconfigdto)

*Response*: [`DisconnectCustomProvidersResponseSchema`](#disconnectcustomprovidersresponseschema)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.post_payments_custom_provider_disconnect",
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

#### `DELETE /payments/custom-provider/provider`

**Deleting an existing integration**

API to delete an association for an app and location

Operation id: `v3:payments.delete_payments_custom_provider_provider` · `Version: v3` · Scopes: `payments/custom-provider.write`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location id |

*Response*: [`DeleteCustomProvidersResponseSchema`](#deletecustomprovidersresponseschema)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.delete_payments_custom_provider_provider",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /payments/custom-provider/provider`

**Create new integration**

API to create a new association for an app and location

Operation id: `v3:payments.post_payments_custom_provider_provider` · `Version: v3` · Scopes: `payments/custom-provider.write`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location id |

*Request body*: [`CreateCustomProvidersDto`](#createcustomprovidersdto)

*Response*: [`CreateCustomProvidersResponseSchema`](#createcustomprovidersresponseschema)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.post_payments_custom_provider_provider",
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

#### `GET /payments/integrations/provider/whitelabel`

**List White-label Integration Providers**

The "List White-label Integration Providers" API allows to retrieve a paginated list of integration providers. Customize your results by filtering whitelabel integration providers(which are built directly on top of Authorize.net or NMI) based on name or paginate through the list using the provided query parameters. This endpoint provides a straightforward way to explore and retrieve integration provider information.

Operation id: `v3:payments.get_payments_integrations_provider_whitelabel` · `Version: v3` · Scopes: `payments/integration.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | location Id / company Id based on altType |
| `altType` | enum: `location` | **yes** | Alt Type |
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |

*Response*: [`ListWhitelabelIntegrationProviderResponseDto`](#listwhitelabelintegrationproviderresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.get_payments_integrations_provider_whitelabel",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `POST /payments/integrations/provider/whitelabel`

**Create White-label Integration Provider**

The "Create White-label Integration Provider" API allows adding a new payment provider integration to the system which is built on top of Authorize.net or NMI. Use this endpoint to create a integration provider with the specified details. Ensure that the required information is provided in the request payload. This endpoint can be only invoked using marketplace-app token

Operation id: `v3:payments.post_payments_integrations_provider_whitelabel` · `Version: v3` · Scopes: `payments/integration.write`

*Request body*: [`CreateWhiteLabelIntegrationProviderDto`](#createwhitelabelintegrationproviderdto)

*Response*: [`CreateWhitelabelIntegrationResponseDto`](#createwhitelabelintegrationresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.post_payments_integrations_provider_whitelabel",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /payments/orders`

**List Orders**

The "List Orders" API allows to retrieve a paginated list of orders. Customize your results by filtering orders based on name, alt type, order status, payment mode, date range, type of source, contact, funnel products or paginate through the list using the provided query parameters. This endpoint provides a straightforward way to explore and retrieve order information.

Operation id: `v3:payments.get_payments_orders` · `Version: v3` · Scopes: `payments/orders.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | no | LocationId is the id of the sub-account. |
| `altId` | string | **yes** | AltId is the unique identifier e.g: location id. |
| `status` | string | no | Order status. |
| `paymentStatus` | enum: `paid`, `unpaid`, `refunded`, `partially_paid` | no | Payment Status of the Order |
| `paymentMode` | string | no | Mode of payment. |
| `startAt` | string | no | Starting interval of orders. |
| `endAt` | string | no | Closing interval of orders. |
| `search` | string | no | The name of the order for searching. |
| `contactId` | string | no | Contact id for filtering of orders. |
| `funnelProductIds` | string | no | Funnel product ids separated by comma. |
| `sourceId` | string | no | Source id |
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |

*Response*: [`ListOrdersResponseDto`](#listordersresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.get_payments_orders",
    "query": {
      "altId": "<altId>"
    }
  }
}
```

</details>

#### `GET /payments/orders/{orderId}`

**Get Order by ID**

The "Get Order by ID" API allows to retrieve information for a specific order using its unique identifier. Use this endpoint to fetch details for a single order based on the provided order ID.

Operation id: `v3:payments.get_payments_orders_by_orderId` · `Version: v3` · Scopes: `payments/orders.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `orderId` | string | **yes** | ID of the order that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | no | LocationId is the id of the sub-account. |
| `altId` | string | **yes** | AltId is the unique identifier e.g: location id. |

*Response*: [`GetOrderResponseSchema`](#getorderresponseschema)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.get_payments_orders_by_orderId",
    "path_params": {
      "orderId": "<orderId>"
    },
    "query": {
      "altId": "<altId>"
    }
  }
}
```

</details>

#### `GET /payments/orders/{orderId}/fulfillments`

**List fulfillment**

List all fulfillment history of an order

Operation id: `v3:payments.get_payments_orders_by_orderId_fulfillments` · `Version: v3` · Scopes: `payments/orders.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `orderId` | string | **yes** | ID of the order that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

*Response*: [`ListFulfillmentResponseDto`](#listfulfillmentresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.get_payments_orders_by_orderId_fulfillments",
    "path_params": {
      "orderId": "<orderId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `POST /payments/orders/{orderId}/fulfillments`

**Create order fulfillment**

The "Order Fulfillment" API facilitates the process of fulfilling an order.

Operation id: `v3:payments.post_payments_orders_by_orderId_fulfillments` · `Version: v3` · Scopes: `payments/orders.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `orderId` | string | **yes** | ID of the order that needs to be returned |

*Request body*: [`CreateFulfillmentDto`](#createfulfillmentdto)

*Response*: [`CreateFulfillmentResponseDto`](#createfulfillmentresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.post_payments_orders_by_orderId_fulfillments",
    "path_params": {
      "orderId": "<orderId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /payments/orders/{orderId}/notes`

**List Order Notes**

List all notes of an order

Operation id: `v3:payments.get_payments_orders_by_orderId_notes` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `orderId` | string | **yes** | ID of the order that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.get_payments_orders_by_orderId_notes",
    "path_params": {
      "orderId": "<orderId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `POST /payments/orders/{orderId}/record-payment`

**Record Order Payment**

The "Record Order Payment" API allows to record a payment for an order. Use this endpoint to record payment for an order and update the order status to "Paid".

Operation id: `v3:payments.post_payments_orders_by_orderId_record_payment` · `Version: v3` · Scopes: `payments/orders.collectPayment`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `orderId` | string | **yes** | Order ID |

*Request body*: [`PostRecordOrderPaymentBody`](#postrecordorderpaymentbody)

*Response*: [`PostRecordOrderPaymentResponse`](#postrecordorderpaymentresponse)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.post_payments_orders_by_orderId_record_payment",
    "path_params": {
      "orderId": "<orderId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /payments/subscriptions`

**List Subscriptions**

The "List Subscriptions" API allows to retrieve a paginated list of subscriptions. Customize your results by filtering subscriptions based on name, alt type, subscription status, payment mode, date range, type of source, contact, subscription id, entity id, contact or paginate through the list using the provided query parameters. This endpoint provides a straightforward way to explore and retrieve subscription information.

Operation id: `v3:payments.get_payments_subscriptions` · `Version: v3` · Scopes: `payments/subscriptions.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | AltId is the unique identifier e.g: location id. |
| `altType` | enum: `location` | **yes** | AltType is the type of identifier. |
| `entityId` | string | no | Entity id for filtering of subscriptions. |
| `paymentMode` | string | no | Mode of payment. |
| `startAt` | string | no | Starting interval of subscriptions. |
| `endAt` | string | no | Closing interval of subscriptions. |
| `entitySourceType` | string | no | Source of the subscriptions. |
| `search` | string | no | The name of the subscription for searching. |
| `contactId` | string | no | Contact ID for the subscription |
| `id` | string | no | Subscription id for filtering of subscriptions. |
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |
| `getPaymentsCollectedCount` | boolean | no | Get the total payments collected for the subscription. |

*Response*: [`ListSubscriptionResponseDto`](#listsubscriptionresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.get_payments_subscriptions",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /payments/subscriptions/{subscriptionId}`

**Get Subscription by ID**

The "Get Subscription by ID" API allows to retrieve information for a specific subscription using its unique identifier. Use this endpoint to fetch details for a single subscription based on the provided subscription ID.

Operation id: `v3:payments.get_payments_subscriptions_by_subscriptionId` · `Version: v3` · Scopes: `payments/subscriptions.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `subscriptionId` | string | **yes** | ID of the subscription that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | AltId is the unique identifier e.g: location id. |
| `altType` | enum: `location` | **yes** | AltType is the type of identifier. |

*Response*: [`GetSubscriptionResponseSchema`](#getsubscriptionresponseschema)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.get_payments_subscriptions_by_subscriptionId",
    "path_params": {
      "subscriptionId": "<subscriptionId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /payments/transactions`

**List Transactions**

The "List Transactions" API allows to retrieve a paginated list of transactions. Customize your results by filtering transactions based on name, alt type, transaction status, payment mode, date range, type of source, contact, subscription id, entity id or paginate through the list using the provided query parameters. This endpoint provides a straightforward way to explore and retrieve transaction information.

Operation id: `v3:payments.get_payments_transactions` · `Version: v3` · Scopes: `payments/transactions.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | no | LocationId is the id of the sub-account. |
| `altId` | string | **yes** | AltId is the unique identifier e.g: location id. |
| `altType` | string | **yes** | AltType is the type of identifier. |
| `paymentMode` | string | no | Mode of payment. |
| `startAt` | string | no | Starting interval of transactions. |
| `endAt` | string | no | Closing interval of transactions. |
| `entitySourceType` | string | no | Source of the transactions. |
| `entitySourceSubType` | string | no | Source sub-type of the transactions. |
| `search` | string | no | The name of the transaction for searching. |
| `subscriptionId` | string | no | Subscription id for filtering of transactions. |
| `entityId` | string | no | Entity id for filtering of transactions. |
| `contactId` | string | no | Contact id for filtering of transactions. |
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |

*Response*: [`ListTxnsResponseDto`](#listtxnsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.get_payments_transactions",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /payments/transactions/{transactionId}`

**Get Transaction by ID**

The "Get Transaction by ID" API allows to retrieve information for a specific transaction using its unique identifier. Use this endpoint to fetch details for a single transaction based on the provided transaction ID.

Operation id: `v3:payments.get_payments_transactions_by_transactionId` · `Version: v3` · Scopes: `payments/transactions.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `transactionId` | string | **yes** | ID of the transaction that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | no | LocationId is the id of the sub-account. |
| `altId` | string | **yes** | AltId is the unique identifier e.g: location id. |
| `altType` | string | **yes** | AltType is the type of identifier. |

*Response*: [`GetTxnResponseSchema`](#gettxnresponseschema)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:payments.get_payments_transactions_by_transactionId",
    "path_params": {
      "transactionId": "<transactionId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::payments::*` (enable the `payments` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/payments/).

### `AmountSummary`

| Field | Type | Required | Description |
|---|---|---|---|
| `subtotal` | f64 | **yes** | Order sub-total value. |
| `discount` | f64 | no | Discount value on order. |

### `ApplyToFuturePaymentsConfig`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `forever`, `fixed` | **yes** | Type of the config |
| `duration` | f64 | **yes** | Duration the coupon to be applied in a subscription |
| `durationType` | String — `months` | **yes** | Type of the duration |

### `ApplyToFuturePaymentsConfigDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `forever`, `fixed` | **yes** | Type of future payments configuration |
| `duration` | f64 | no | Duration value for fixed type configurations |
| `durationType` | String | no | Duration type for fixed configurations (e.g. months) |

### `CardDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `visa`, `mastercard`, `other` | **yes** | — |
| `last4` | String | **yes** | Last 4 digit of the card |

### `ChequeDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `number` | String | **yes** | check number |

### `ConnectCustomProvidersConfigDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `live` | [`CustomProviderKeys`](#customproviderkeys) | **yes** | Live config containing api-key and publishable key for live payments |
| `test` | [`CustomProviderKeys`](#customproviderkeys) | **yes** | Test config containing api-key and publishable-key for test payments |

### `ConnectCustomProvidersResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | The name of the custom provider |
| `description` | String | **yes** | Description of payment gateway. Shown on the payments integrations page as subtext |
| `paymentsUrl` | String | **yes** | This url will be loaded in iFrame to start a payment session. |
| `queryUrl` | String | **yes** | The url used for querying payments related events. Ex. verify, refund, subscription etc. |
| `imageUrl` | String | **yes** | Public image url for logo of the payment gateway displayed on the payments integrations page. |
| `_id` | String | **yes** | The unique identifier for the custom provider. |
| `locationId` | String | **yes** | Location id |
| `marketplaceAppId` | String | **yes** | The application id of marketplace |
| `paymentProvider` | JSON | no | Payment provider details. |
| `deleted` | bool | **yes** | Whether the config is deleted or not. true represents config is deleted |
| `createdAt` | String | **yes** | The creation timestamp of the custom provider. |
| `updatedAt` | String | **yes** | The last update timestamp of the custom provider. |
| `traceId` | String | no | Trace id of the custom provider. |

### `CouponDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Unique MongoDB identifier for the coupon |
| `usageCount` | f64 | **yes** | Number of times the coupon has been used |
| `limitPerCustomer` | f64 | **yes** | Maximum number of times a customer can use this coupon (0 for unlimited) |
| `altId` | String | **yes** | Location Id |
| `altType` | String | **yes** | Type of entity |
| `name` | String | **yes** | Display name of the coupon |
| `code` | String | **yes** | Redemption code for the coupon |
| `discountType` | String — `percentage`, `amount` | **yes** | Type of discount (percentage or amount) |
| `discountValue` | f64 | **yes** | Value of the discount (percentage or fixed amount) |
| `status` | String — `scheduled`, `active`, `expired` | **yes** | Current status of the coupon |
| `startDate` | String | **yes** | Date when the coupon becomes active |
| `endDate` | String | no | End date when the coupon expires |
| `applyToFuturePayments` | bool | **yes** | Indicates if the coupon applies to future recurring payments |
| `applyToFuturePaymentsConfig` | [`ApplyToFuturePaymentsConfigDto`](#applytofuturepaymentsconfigdto) | **yes** | Configuration for how the coupon applies to future payments |
| `userId` | String | no | User ID associated with the coupon (if applicable) |
| `createdAt` | String | **yes** | Creation timestamp |
| `updatedAt` | String | **yes** | Last update timestamp |

### `CreateCouponParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id |
| `altType` | String — `location` | **yes** | Alt Type |
| `name` | String | **yes** | Coupon Name |
| `code` | String | **yes** | Coupon Code |
| `discountType` | String — `percentage`, `amount` | **yes** | Discount Type |
| `discountValue` | f64 | **yes** | Discount Value |
| `startDate` | String | **yes** | Start date in YYYY-MM-DDTHH:mm:ssZ format |
| `endDate` | String | no | End date in YYYY-MM-DDTHH:mm:ssZ format |
| `usageLimit` | f64 | no | Max number of times coupon can be used |
| `productIds` | Vec<String> | no | Product Ids |
| `applyToFuturePayments` | bool | no | Is Coupon applicable on upcoming subscription transactions |
| `applyToFuturePaymentsConfig` | [`ApplyToFuturePaymentsConfig`](#applytofuturepaymentsconfig) | no | If coupon is applicable on upcoming subscription transactions, how many months should it be applicable for a subscription |
| `limitPerCustomer` | bool | no | Limits whether a coupon can be redeemed only once per customer. |

### `CreateCouponResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Unique MongoDB identifier for the coupon |
| `usageCount` | f64 | **yes** | Number of times the coupon has been used |
| `limitPerCustomer` | f64 | **yes** | Maximum number of times a customer can use this coupon (0 for unlimited) |
| `altId` | String | **yes** | Location Id |
| `altType` | String | **yes** | Type of entity |
| `name` | String | **yes** | Display name of the coupon |
| `code` | String | **yes** | Redemption code for the coupon |
| `discountType` | String — `percentage`, `amount` | **yes** | Type of discount (percentage or amount) |
| `discountValue` | f64 | **yes** | Value of the discount (percentage or fixed amount) |
| `status` | String — `scheduled`, `active`, `expired` | **yes** | Current status of the coupon |
| `startDate` | String | **yes** | Date when the coupon becomes active |
| `endDate` | String | no | End date when the coupon expires |
| `applyToFuturePayments` | bool | **yes** | Indicates if the coupon applies to future recurring payments |
| `applyToFuturePaymentsConfig` | [`ApplyToFuturePaymentsConfigDto`](#applytofuturepaymentsconfigdto) | **yes** | Configuration for how the coupon applies to future payments |
| `userId` | String | no | User ID associated with the coupon (if applicable) |
| `createdAt` | String | **yes** | Creation timestamp |
| `updatedAt` | String | **yes** | Last update timestamp |
| `traceId` | String | **yes** | Unique identifier for tracing this API request |

### `CreateCustomProvidersDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | The name of the custom provider |
| `description` | String | **yes** | Description of payment gateway. Shown on the payments integrations page as subtext |
| `paymentsUrl` | String | **yes** | This url will be loaded in iFrame to start a payment session. |
| `queryUrl` | String | **yes** | The url used for querying payments related events. Ex. verify, refund, subscription etc. |
| `imageUrl` | String | **yes** | Public image url for logo of the payment gateway displayed on the payments integrations page. |
| `supportsSubscriptionSchedule` | bool | **yes** | Whether the config supports subscription schedule or not. true represents config supports subscription schedule |

### `CreateCustomProvidersResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | The name of the custom provider |
| `description` | String | **yes** | Description of payment gateway. Shown on the payments integrations page as subtext |
| `paymentsUrl` | String | **yes** | This url will be loaded in iFrame to start a payment session. |
| `queryUrl` | String | **yes** | The url used for querying payments related events. Ex. verify, refund, subscription etc. |
| `imageUrl` | String | **yes** | Public image url for logo of the payment gateway displayed on the payments integrations page. |
| `_id` | String | **yes** | The unique identifier for the custom provider. |
| `locationId` | String | **yes** | Location id |
| `marketplaceAppId` | String | **yes** | The application id of marketplace |
| `paymentProvider` | JSON | no | Payment provider details. |
| `deleted` | bool | **yes** | Whether the config is deleted or not. true represents config is deleted |
| `createdAt` | String | **yes** | The creation timestamp of the custom provider. |
| `updatedAt` | String | **yes** | The last update timestamp of the custom provider. |
| `traceId` | String | no | Trace id of the custom provider. |

### `CreateFulfillmentDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `trackings` | Vec<FulfillmentTracking> | **yes** | Fulfillment tracking information |
| `items` | Vec<FulfillmentItems> | **yes** | Fulfilled items |
| `notifyCustomer` | bool | **yes** | Need to send a notification to customer |

### `CreateFulfillmentResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `data` | [`FulfillmentSchema`](#fulfillmentschema) | **yes** | fulfillment data |

### `CreateWhiteLabelIntegrationProviderDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `uniqueName` | String | **yes** | A unique name given to the integration provider, uniqueName must start and end with a character. Only lowercase characters and hyphens (-) are supported |
| `title` | String | **yes** | The title or name of the integration provider. |
| `provider` | String — `authorize-net`, `nmi` | **yes** | The type of payment provider associated with the integration provider. |
| `description` | String | **yes** | A brief description providing additional information about the integration provider. |
| `imageUrl` | String | **yes** | The URL to an image representing the integration provider. The imageUrl should start with "https://" and ensure that this URL is publicly accessible. |

### `CreateWhitelabelIntegrationResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier of the integration provider. |
| `altId` | String | **yes** | The altId / locationId of the integration provider. |
| `altType` | String | **yes** | The altType of the integration provider. |
| `title` | String | **yes** | The title or name of the integration provider. |
| `route` | String | **yes** | The route name associated with the integration provider. |
| `provider` | String | **yes** | The payment provider associated with the integration provider. |
| `description` | String | **yes** | A brief description providing additional information about the integration provider. |
| `imageUrl` | String | **yes** | The URL to an image representing the integration provider. |
| `createdAt` | String | **yes** | The timestamp when the integration provider was created. |
| `updatedAt` | String | **yes** | The timestamp when the integration provider was last updated. |

### `CustomProviderKeys`

| Field | Type | Required | Description |
|---|---|---|---|
| `apiKey` | String | **yes** | Api-key for custom payment provider config |
| `publishableKey` | String | **yes** | Publishable-key for custom payment provider config |

### `CustomRRuleOptionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `intervalType` | String — `yearly`, `monthly`, `weekly`, `daily`, `hourly`, `minutely`, `secondly` | **yes** | — |
| `interval` | f64 | **yes** | — |
| `startDate` | String | **yes** | Start date in YYYY-MM-DD format |
| `startTime` | String | no | Start time in HH:mm:ss format |
| `endDate` | String | no | End date in YYYY-MM-DD format |
| `endTime` | String | no | End time in HH:mm:ss format |
| `dayOfMonth` | f64 | no | -1, 1, 2, 3, ..., 27, 28 |
| `dayOfWeek` | String — `mo`, `tu`, `we`, `th`, `fr`, `sa`, `su` | no | — |
| `numOfWeek` | f64 | no | -1, 1, 2, 3, 4 |
| `monthOfYear` | String — `jan`, `feb`, `mar`, `apr`, `may`, `jun`, `jul`, `aug`, `sep`, `oct`, `nov`, `dec` | no | — |
| `count` | f64 | no | Max number of task executions |
| `daysBefore` | f64 | no | Execute task number of days before |

### `DefaultPriceResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier for the price. |
| `membershipOffers` | Vec<MembershipOfferDto> | no | An array of membership offers associated with the price. |
| `variantOptionIds` | Vec<String> | no | An array of variant option IDs associated with the price. |
| `locationId` | String | no | The unique identifier for the location. |
| `product` | String | no | The unique identifier for the associated product. |
| `userId` | String | no | The unique identifier for the user. |
| `name` | String | **yes** | The name of the price. |
| `type` | String — `one_time`, `recurring` | **yes** | The type of the price (e.g., one_time). |
| `currency` | String | **yes** | The currency code for the price. |
| `amount` | f64 | **yes** | The amount of the price. |
| `recurring` | [`RecurringDto`](#recurringdto) | no | The recurring details of the price (if type is recurring). |
| `createdAt` | String | no | The creation timestamp of the price. |
| `updatedAt` | String | no | The last update timestamp of the price. |
| `compareAtPrice` | f64 | no | The compare-at price for comparison purposes. |
| `trackInventory` | bool | no | Indicates whether inventory tracking is enabled. |
| `availableQuantity` | f64 | no | Available inventory stock quantity |
| `allowOutOfStockPurchases` | bool | no | Continue selling when out of stock |

### `DefaultProductResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier for the product. |
| `description` | String | no | product description |
| `variants` | Vec<ProductVariantDto> | no | An array of variants for the product. |
| `locationId` | String | **yes** | The unique identifier for the location. |
| `name` | String | **yes** | The name of the product. |
| `productType` | String | **yes** | The type of the product (e.g., PHYSICAL). |
| `availableInStore` | bool | no | Indicates whether the product is available in-store. |
| `createdAt` | String | **yes** | The creation timestamp of the product. |
| `updatedAt` | String | **yes** | The last update timestamp of the product. |
| `statementDescriptor` | String | no | The statement descriptor for the product. |
| `image` | String | no | The URL for the product image. |
| `collectionIds` | Vec<String> | no | An array of category Ids for the product |
| `isTaxesEnabled` | bool | no | The field indicates whether taxes are enabled for the product or not. |
| `taxes` | Vec<String> | no | An array of ids of Taxes attached to the Product. If the expand query includes tax, the taxes will be of type `ProductTaxDto`. Please refer to the `ProductTaxDto` for additional details. |
| `automaticTaxCategoryId` | String | no | Tax category ID for Automatic taxes calculation. |
| `label` | [`ProductLabelDto`](#productlabeldto) | no | The Product label details |
| `slug` | String | no | The slug of the product by which the product will be navigated |

### `DeleteCouponParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id |
| `altType` | String — `location` | **yes** | Alt Type |
| `id` | String | **yes** | Coupon Id |

### `DeleteCouponResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Indicates whether the delete was successful |
| `traceId` | String | **yes** | Unique identifier for tracing this API request |

### `DeleteCustomProvidersConfigDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `liveMode` | bool | **yes** | Whether the config is for test mode or live mode. true represents config is for live payments |

### `DeleteCustomProvidersResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Whether the custom provider config is disconnect or not. true represents config is disconnect |

### `DisconnectCustomProvidersResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Whether the custom provider config is disconnect or not. true represents config is disconnect |

### `FulfilledItem`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The id of product price |
| `name` | String | **yes** | Name |
| `product` | [`DefaultProductResponseDto`](#defaultproductresponsedto) | **yes** | Product details |
| `price` | [`DefaultPriceResponseDto`](#defaultpriceresponsedto) | **yes** | Price details |
| `qty` | f64 | **yes** | The no of quantity of the current fulfilled item |

### `FulfillmentItems`

| Field | Type | Required | Description |
|---|---|---|---|
| `priceId` | String | **yes** | The id of product price |
| `qty` | f64 | **yes** | The no of quantity of the item |

### `FulfillmentSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `trackings` | Vec<FulfillmentTracking> | **yes** | Fulfillment tracking information |
| `_id` | String | **yes** | The unique identifier for the fulfillment item. |
| `items` | Vec<FulfilledItem> | **yes** | Fulfilled items |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `FulfillmentTracking`

| Field | Type | Required | Description |
|---|---|---|---|
| `trackingNumber` | String | no | Tracking number provided by the shipping carrier |
| `shippingCarrier` | String | no | Shipping carrier name |
| `trackingUrl` | String | no | Tracking URL |

### `GetCustomProvidersResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | The name of the custom provider |
| `description` | String | **yes** | Description of payment gateway. Shown on the payments integrations page as subtext |
| `paymentsUrl` | String | **yes** | This url will be loaded in iFrame to start a payment session. |
| `queryUrl` | String | **yes** | The url used for querying payments related events. Ex. verify, refund, subscription etc. |
| `imageUrl` | String | **yes** | Public image url for logo of the payment gateway displayed on the payments integrations page. |
| `_id` | String | **yes** | The unique identifier for the custom provider. |
| `locationId` | String | **yes** | Location id |
| `marketplaceAppId` | String | **yes** | The application id of marketplace |
| `paymentProvider` | JSON | no | Payment provider details. |
| `deleted` | bool | **yes** | Whether the config is deleted or not. true represents config is deleted |
| `createdAt` | String | **yes** | The creation timestamp of the custom provider. |
| `updatedAt` | String | **yes** | The last update timestamp of the custom provider. |
| `traceId` | String | no | Trace id of the custom provider. |

### `GetOrderResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier for the order. |
| `altId` | String | **yes** | AltId is the unique identifier eg: location id. |
| `altType` | String | **yes** | AltType is the type of identifier. |
| `contactId` | String | no | Contact id corresponding to the order. |
| `currency` | String | no | Currency in which order was created. |
| `amount` | f64 | no | Order value. |
| `status` | String | **yes** | The status of the order (e.g., completed). |
| `liveMode` | bool | no | Order is in live / test mode. |
| `createdAt` | String | **yes** | The creation timestamp of the order. |
| `updatedAt` | String | **yes** | The last update timestamp of the order. |
| `fulfillmentStatus` | String | no | Fulfillment status of the order. |
| `contactSnapshot` | JSON | no | Contact details of the order. |
| `amountSummary` | [`AmountSummary`](#amountsummary) | no | Amount details of the order. |
| `source` | [`OrderSource`](#ordersource) | no | Source details of the order. |
| `items` | Vec<String> | no | Item details of the order. |
| `coupon` | JSON | no | Coupon details of the order. |
| `trackingId` | String | no | Tracking id of the order. |
| `fingerprint` | String | no | Fingerprint id of the order. |
| `meta` | JSON | no | Meta details of the order. |
| `markAsTest` | bool | no | Is test order. |
| `traceId` | String | no | Trace id of the order. |
| `automaticTaxesCalculated` | bool | no | Automatic taxes applied for the Order |
| `taxCalculationProvider` | JSON | no | Provider name for automatic tax calculation |
| `createdBy` | String | no | User ID who created the order. |

### `GetSubscriptionResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier for the subscription. |
| `altType` | JSON | **yes** | AltType is the type of identifier. |
| `altId` | String | **yes** | AltId is the unique identifier eg: location id. |
| `contactId` | String | no | Contact id corresponding to the subscription. |
| `contactSnapshot` | JSON | no | Contact details of the subscriber. |
| `coupon` | JSON | no | Coupon details of the subscription. |
| `currency` | String | no | Currency in which subscription was made. |
| `amount` | f64 | no | Subscription value. |
| `status` | JSON | no | Subscription status. |
| `liveMode` | bool | no | Subscription is in live / test mode. |
| `entityType` | String | no | Entity type of subscription (eg: order). |
| `entityId` | String | no | Entity id for the subscription. e.g: order id |
| `entitySource` | [`OrderSource`](#ordersource) | no | Entity source details for the subscription. |
| `subscriptionId` | String | no | Subscription id for subscription. |
| `subscriptionSnapshot` | JSON | no | Snapshot of subscription. |
| `paymentProvider` | JSON | no | Payment provider details for the subscription. |
| `ipAddress` | String | no | Ip address from where subscription was initiated. |
| `createdAt` | String | **yes** | The creation timestamp of the subscription. |
| `updatedAt` | String | **yes** | The last update timestamp of the subscription. |
| `meta` | JSON | no | Meta details of the subscription. |
| `markAsTest` | bool | no | Is test subscription. |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | no | Scedule details for the subscription. |
| `autoPayment` | JSON | no | Auto payment details of the subscription. |
| `recurringProduct` | JSON | no | Recurring product details of the subscription. |
| `canceledAt` | String | no | Cancellation timestamp of the subscription. |
| `canceledBy` | String | no | User id who cancelled the subscription. |
| `traceId` | String | no | Trace id of the subscription. |
| `createdBy` | String | no | User ID who created the subscription. |

### `GetTxnResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier for the transaction. |
| `altType` | String | **yes** | AltType is the type of identifier. |
| `altId` | String | **yes** | AltId is the unique identifier eg: location id. |
| `contactId` | String | no | Contact id corresponding to the transaction. |
| `contactSnapshot` | JSON | no | Contact details of the transaction. |
| `currency` | String | no | Currency in which transaction was made. |
| `amount` | f64 | no | Transaction value. |
| `status` | JSON | no | Transaction status. |
| `liveMode` | bool | no | Transaction is in live / test mode. |
| `createdAt` | String | **yes** | The creation timestamp of the transaction. |
| `updatedAt` | String | **yes** | The last update timestamp of the transaction. |
| `entityType` | String | no | Entity type of transaction (eg: order). |
| `entityId` | String | no | Entity id for the transaction. e.g: order id |
| `entitySource` | [`OrderSource`](#ordersource) | no | Entity source details for the transaction. |
| `chargeId` | String | no | Charge id for transaction. |
| `chargeSnapshot` | JSON | no | Charge snapshot of transaction. |
| `invoiceId` | String | no | Invoice id for the transaction. |
| `subscriptionId` | String | no | Subscription id for transaction. |
| `paymentProvider` | JSON | no | Payment provider details of the transaction. |
| `ipAddress` | String | no | Ip address from where transaction was initiated. |
| `meta` | JSON | no | Meta details of the transaction. |
| `markAsTest` | bool | no | Is test transaction. |
| `isParent` | bool | no | Is parent transaction. |
| `amountRefunded` | f64 | no | Transaction amount refunded. |
| `receiptId` | String | no | Receipt id for transaction. |
| `qboSynced` | bool | no | Is transaction qbo synced. |
| `qboResponse` | JSON | no | Qbo details of the transaction. |
| `traceId` | String | no | Trace id of the transaction. |
| `mergedFromContactId` | String | no | ID of the contact that was merged from. |
| `createdBy` | String | no | User ID who created the transaction. |

### `IntegrationProviderSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier of the integration provider. |
| `altId` | String | **yes** | The altId / locationId of the integration provider. |
| `altType` | String | **yes** | The altType of the integration provider. |
| `title` | String | **yes** | The title or name of the integration provider. |
| `route` | String | **yes** | The route name associated with the integration provider. |
| `provider` | String | **yes** | The payment provider associated with the integration provider. |
| `description` | String | **yes** | A brief description providing additional information about the integration provider. |
| `imageUrl` | String | **yes** | The URL to an image representing the integration provider. |
| `createdAt` | String | **yes** | The timestamp when the integration provider was created. |
| `updatedAt` | String | **yes** | The timestamp when the integration provider was last updated. |

### `ListCouponsResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<CouponDto> | **yes** | Array of coupon objects |
| `totalCount` | f64 | **yes** | Total number of coupons matching the query criteria |
| `traceId` | String | **yes** | Unique identifier for tracing this API request |

### `ListFulfillmentResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `data` | Vec<FulfillmentSchema> | **yes** | An array of fulfilled items |

### `ListOrdersResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<OrderResponseSchema> | **yes** | An array of orders |
| `totalCount` | f64 | **yes** | total orders count |

### `ListSubscriptionResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<SubscriptionResponseSchema> | **yes** | An array of subscriptions |
| `totalCount` | f64 | **yes** | total subscriptions count |

### `ListTxnsResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<TxnResponseSchema> | **yes** | An array of transactions |
| `totalCount` | f64 | **yes** | total transactions count |

### `ListWhitelabelIntegrationProviderResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `providers` | [`IntegrationProviderSchema`](#integrationproviderschema) | **yes** | list of integration provider. |

### `MembershipOfferDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `label` | String | **yes** | Membership offer label |
| `value` | String | **yes** | Membership offer label |
| `_id` | String | **yes** | The unique identifier for the membership offer. |

### `OrderResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier for the order. |
| `altId` | String | **yes** | AltId is the unique identifier eg: location id. |
| `altType` | String | **yes** | AltType is the type of identifier. |
| `contactId` | String | no | Contact id corresponding to the order. |
| `contactName` | String | no | Contact name corresponding to the order. |
| `contactEmail` | String | no | Contact email corresponding to the order. |
| `currency` | String | no | Currency in which order was created. |
| `amount` | f64 | no | Order value. |
| `subtotal` | f64 | no | Order sub-total value. |
| `discount` | f64 | no | Discount value on order. |
| `status` | String | **yes** | The status of the order (e.g., completed). |
| `liveMode` | bool | no | Order is in live / test mode. |
| `totalProducts` | f64 | no | Total products in an order. |
| `sourceType` | String | **yes** | Source type of order (eg: funnel). |
| `sourceName` | String | no | Source name for the order. |
| `sourceId` | String | no | Source id for the order. |
| `sourceMeta` | JSON | no | Meta content for the source of order. |
| `couponCode` | String | no | Coupon code for the order. |
| `createdAt` | String | **yes** | The creation timestamp of the order. |
| `updatedAt` | String | **yes** | The last update timestamp of the order. |
| `sourceSubType` | String | no | Source sub-type for the order. |
| `fulfillmentStatus` | String | no | Fulfillment status of the order. |
| `onetimeProducts` | f64 | no | Total one time products in an order. |
| `recurringProducts` | f64 | no | Total recurring time products in an order. |
| `createdBy` | String | no | User ID who created the order. |

### `OrderSource`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `funnel`, `website`, `invoice`, `calendar`, `text2Pay`, `document_contracts`, `membership`, `mobile_app`, `communities`, `point_of_sale`, `manual`, `form`, `survey`, `payment_link`, `external` | **yes** | — |
| `subType` | String — `one_step_order_form`, `two_step_order_form`, `upsell`, `tap_to_pay`, `card_payment`, `store`, `contact_view`, `email_campaign`, `payments_dashboard`, `shopify`, `subscription_view`, `store_upsell`, `woocommerce`, `service`, `meeting`, `imported_csv`, `qr_code`, `saas_one_time`, `saas_subscription` | no | — |
| `id` | String | **yes** | — |
| `name` | String | no | — |
| `meta` | JSON | no | — |

### `PostRecordOrderPaymentBody`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `mode` | String — `cash`, `card`, `cheque`, `bank_transfer`, `other` | **yes** | manual payment method |
| `card` | [`CardDto`](#carddto) | no | Details of Card if used for payment |
| `cheque` | [`ChequeDto`](#chequedto) | no | Details of the Cheque if used for payment |
| `notes` | String | no | Any note to be recorded with the transaction |
| `amount` | f64 | no | Amount to be paid against the invoice. |
| `meta` | JSON | no | Meta data to be recorded with the transaction |
| `isPartialPayment` | bool | no | Indicates if the order is intended to be a partial payment. |

### `PostRecordOrderPaymentResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status of the request |

### `ProductLabelDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | The content for the product label. |
| `startDate` | String | no | Start date in YYYY-MM-DDTHH:mm:ssZ format |
| `endDate` | String | no | Start date in YYYY-MM-DDTHH:mm:ssZ format |

### `ProductVariantDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | A unique identifier for the variant. |
| `name` | String | **yes** | The name of the variant. |
| `options` | Vec<ProductVariantOptionDto> | **yes** | An array of options for the variant. |

### `ProductVariantOptionDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | The unique identifier for the option. |
| `name` | String | **yes** | The name of the option. |

### `RecurringDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `interval` | String — `day`, `month`, `week`, `year` | **yes** | The interval at which the recurring event occurs. |
| `intervalCount` | f64 | **yes** | The number of intervals between each occurrence of the event. |

### `ScheduleOptionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `executeAt` | String | no | — |
| `rrule` | [`CustomRRuleOptionsDto`](#customrruleoptionsdto) | no | — |

### `SubscriptionResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier for the subscription. |
| `altId` | String | **yes** | AltId is the unique identifier eg: location id. |
| `altType` | String — `location` | **yes** | AltType is the type of identifier. |
| `contactId` | String | no | Contact id corresponding to the subscription. |
| `contactName` | String | no | Contact name corresponding to the subscription. |
| `contactEmail` | String | no | Contact email corresponding to the subscription. |
| `currency` | String | no | Currency in which subscription occurred. |
| `amount` | f64 | no | Subscription value. |
| `status` | JSON | **yes** | The status of the subscription (e.g., succeeded). |
| `liveMode` | bool | no | Subscription is in live / test mode. |
| `entityType` | String | no | Entity type of subscription (eg: order). |
| `entityId` | String | no | Entity id for the subscription. e.g: order id |
| `entitySourceType` | String | **yes** | Entity source type of subscription (eg: funnel). |
| `entitySourceName` | String | no | Entity source name for the subscription. |
| `entitySourceId` | String | no | Entity source id for the subscription. |
| `entitySourceMeta` | JSON | no | Meta content for the entity source of subscription. |
| `subscriptionId` | String | no | Subscription id for subscription. |
| `subscriptionSnapshot` | JSON | no | Snapshot of subscription. |
| `paymentProviderType` | String | no | Payment provider for subscription. |
| `paymentProviderConnectedAccount` | String | no | Payment provider connected account id for subscription. |
| `ipAddress` | String | no | Ip address from where subscription was initiated. |
| `createdAt` | String | **yes** | The creation timestamp of the subscription. |
| `updatedAt` | String | **yes** | The update timestamp of the subscription. |
| `createdBy` | String | no | User ID who created the subscription. |

### `TxnResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier for the transaction. |
| `altId` | String | **yes** | AltId is the unique identifier eg: location id. |
| `altType` | String | **yes** | AltType is the type of identifier. |
| `contactId` | String | no | Contact id corresponding to the transaction. |
| `mergedFromContactId` | String | no | ID of the contact that was merged from. |
| `contactName` | String | no | Contact name corresponding to the transaction. |
| `contactEmail` | String | no | Contact email corresponding to the transaction. |
| `currency` | String | no | Currency in which transaction occurred. |
| `amount` | f64 | no | Transaction value. |
| `status` | JSON | **yes** | The status of the transaction (e.g., succeeded). |
| `liveMode` | bool | no | Transaction is in live / test mode. |
| `entityType` | String | no | Entity type of transaction (eg: order). |
| `entityId` | String | no | Entity id for the transaction. e.g: order id |
| `entitySourceType` | String | **yes** | Entity source type of transaction (eg: funnel). |
| `entitySourceSubType` | String | no | Entity source sub-type of the transactions. |
| `entitySourceName` | String | no | Entity source name for the transaction. |
| `entitySourceId` | String | no | Entity source id for the transaction. |
| `entitySourceMeta` | JSON | no | Meta content for the entity source of transaction. |
| `subscriptionId` | String | no | Subscription id for transaction. |
| `chargeId` | String | no | Charge id for transaction. |
| `chargeSnapshot` | JSON | no | Charge snapshot of transaction. |
| `paymentProviderType` | String | no | Payment provider for transaction. |
| `paymentProviderConnectedAccount` | String | no | Payment provider account id for transaction. |
| `ipAddress` | String | no | Ip address from where transaction was initiated. |
| `createdAt` | String | **yes** | The creation timestamp of the transaction. |
| `updatedAt` | String | **yes** | The update timestamp of the transaction. |
| `amountRefunded` | f64 | no | Transaction amount refunded. |
| `paymentMethod` | JSON | no | Transaction payment method details. |
| `fulfilledAt` | String | **yes** | The charged timestamp of the transaction. |
| `createdBy` | String | no | User ID who created the transaction. |

### `UpdateCouponParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id |
| `altType` | String — `location` | **yes** | Alt Type |
| `name` | String | **yes** | Coupon Name |
| `code` | String | **yes** | Coupon Code |
| `discountType` | String — `percentage`, `amount` | **yes** | Discount Type |
| `discountValue` | f64 | **yes** | Discount Value |
| `startDate` | String | **yes** | Start date in YYYY-MM-DDTHH:mm:ssZ format |
| `endDate` | String | no | End date in YYYY-MM-DDTHH:mm:ssZ format |
| `usageLimit` | f64 | no | Max number of times coupon can be used |
| `productIds` | Vec<String> | no | Product Ids |
| `applyToFuturePayments` | bool | no | Is Coupon applicable on upcoming subscription transactions |
| `applyToFuturePaymentsConfig` | [`ApplyToFuturePaymentsConfig`](#applytofuturepaymentsconfig) | no | If coupon is applicable on upcoming subscription transactions, how many months should it be applicable for a subscription |
| `limitPerCustomer` | bool | no | Limits whether a coupon can be redeemed only once per customer. |
| `id` | String | **yes** | Coupon Id |

### `UpdateCustomProviderCapabilitiesDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `supportsSubscriptionSchedules` | bool | **yes** | Whether the marketplace app supports subscription schedules or not |
| `companyId` | String | no | Company id. Mandatory if locationId is not provided |
| `locationId` | String | no | Location / Sub-account id. Mandatory if companyId is not provided |

### `UpdateCustomProviderCapabilitiesResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Whether the custom provider capabilities are updated or not. true represents capabilities are updated |

## Data models — API v3

In Rust: `ghl_models::v3::payments::*` (enable the `payments` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/payments/).

### `AmountSummary`

| Field | Type | Required | Description |
|---|---|---|---|
| `subtotal` | f64 | **yes** | Order sub-total value. |
| `discount` | f64 | no | Discount value on order. |

### `ApplyToFuturePaymentsConfig`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `forever`, `fixed` | **yes** | Type of the config |
| `duration` | f64 | **yes** | Duration the coupon to be applied in a subscription |
| `durationType` | String — `months` | **yes** | Type of the duration |

### `ApplyToFuturePaymentsConfigDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `forever`, `fixed` | **yes** | Type of future payments configuration |
| `duration` | f64 | no | Duration value for fixed type configurations |
| `durationType` | String | no | Duration type for fixed configurations (e.g. months) |

### `CardDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `visa`, `mastercard`, `other` | **yes** | — |
| `last4` | String | **yes** | Last 4 digit of the card |

### `ChequeDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `number` | String | **yes** | check number |

### `ConnectCustomProvidersConfigDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `live` | [`CustomProviderKeys`](#customproviderkeys) | **yes** | Live config containing api-key and publishable key for live payments |
| `test` | [`CustomProviderKeys`](#customproviderkeys) | **yes** | Test config containing api-key and publishable-key for test payments |

### `ConnectCustomProvidersResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | The name of the custom provider |
| `description` | String | **yes** | Description of payment gateway. Shown on the payments integrations page as subtext |
| `paymentsUrl` | String | **yes** | This url will be loaded in iFrame to start a payment session. |
| `queryUrl` | String | **yes** | The url used for querying payments related events. Ex. verify, refund, subscription etc. |
| `imageUrl` | String | **yes** | Public image url for logo of the payment gateway displayed on the payments integrations page. |
| `_id` | String | **yes** | The unique identifier for the custom provider. |
| `locationId` | String | **yes** | Location id |
| `marketplaceAppId` | String | **yes** | The application id of marketplace |
| `paymentProvider` | JSON | no | Payment provider details. |
| `deleted` | bool | **yes** | Whether the config is deleted or not. true represents config is deleted |
| `createdAt` | String | **yes** | The creation timestamp of the custom provider. |
| `updatedAt` | String | **yes** | The last update timestamp of the custom provider. |
| `traceId` | String | no | Trace id of the custom provider. |

### `CouponDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Unique MongoDB identifier for the coupon |
| `usageCount` | f64 | **yes** | Number of times the coupon has been used |
| `limitPerCustomer` | f64 | **yes** | Maximum number of times a customer can use this coupon (0 for unlimited) |
| `altId` | String | **yes** | Location Id |
| `altType` | String | **yes** | Type of entity |
| `name` | String | **yes** | Display name of the coupon |
| `code` | String | **yes** | Redemption code for the coupon |
| `discountType` | String — `percentage`, `amount` | **yes** | Type of discount (percentage or amount) |
| `discountValue` | f64 | **yes** | Value of the discount (percentage or fixed amount) |
| `status` | String — `scheduled`, `active`, `expired` | **yes** | Current status of the coupon |
| `startDate` | String | **yes** | Date when the coupon becomes active |
| `endDate` | String | no | End date when the coupon expires |
| `applyToFuturePayments` | bool | **yes** | Indicates if the coupon applies to future recurring payments |
| `applyToFuturePaymentsConfig` | [`ApplyToFuturePaymentsConfigDto`](#applytofuturepaymentsconfigdto) | **yes** | Configuration for how the coupon applies to future payments |
| `productIds` | Vec<String> | no | Product Ids |
| `priceIds` | Vec<String> | no | Price Ids |
| `variantIds` | Vec<String> | no | Variant Ids |
| `userId` | String | no | User ID associated with the coupon (if applicable) |
| `createdAt` | String | **yes** | Creation timestamp |
| `updatedAt` | String | **yes** | Last update timestamp |

### `CreateCouponParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id |
| `altType` | String — `location` | **yes** | Alt Type |
| `name` | String | **yes** | Coupon Name |
| `code` | String | **yes** | Coupon Code |
| `discountType` | String — `percentage`, `amount` | **yes** | Discount Type |
| `discountValue` | f64 | **yes** | Discount Value |
| `startDate` | String | **yes** | Start date in YYYY-MM-DDTHH:mm:ssZ format |
| `endDate` | String | no | End date in YYYY-MM-DDTHH:mm:ssZ format |
| `usageLimit` | f64 | no | Max number of times coupon can be used |
| `productIds` | Vec<String> | no | Product Ids |
| `priceIds` | Vec<String> | no | Price Ids |
| `variantIds` | Vec<String> | no | Variant Ids |
| `applyToFuturePayments` | bool | no | Is Coupon applicable on upcoming subscription transactions |
| `applyToFuturePaymentsConfig` | [`ApplyToFuturePaymentsConfig`](#applytofuturepaymentsconfig) | no | If coupon is applicable on upcoming subscription transactions, how many months should it be applicable for a subscription |
| `limitPerCustomer` | bool | no | Limits whether a coupon can be redeemed only once per customer. |

### `CreateCouponResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Unique MongoDB identifier for the coupon |
| `usageCount` | f64 | **yes** | Number of times the coupon has been used |
| `limitPerCustomer` | f64 | **yes** | Maximum number of times a customer can use this coupon (0 for unlimited) |
| `altId` | String | **yes** | Location Id |
| `altType` | String | **yes** | Type of entity |
| `name` | String | **yes** | Display name of the coupon |
| `code` | String | **yes** | Redemption code for the coupon |
| `discountType` | String — `percentage`, `amount` | **yes** | Type of discount (percentage or amount) |
| `discountValue` | f64 | **yes** | Value of the discount (percentage or fixed amount) |
| `status` | String — `scheduled`, `active`, `expired` | **yes** | Current status of the coupon |
| `startDate` | String | **yes** | Date when the coupon becomes active |
| `endDate` | String | no | End date when the coupon expires |
| `applyToFuturePayments` | bool | **yes** | Indicates if the coupon applies to future recurring payments |
| `applyToFuturePaymentsConfig` | [`ApplyToFuturePaymentsConfigDto`](#applytofuturepaymentsconfigdto) | **yes** | Configuration for how the coupon applies to future payments |
| `userId` | String | no | User ID associated with the coupon (if applicable) |
| `createdAt` | String | **yes** | Creation timestamp |
| `updatedAt` | String | **yes** | Last update timestamp |
| `traceId` | String | **yes** | Unique identifier for tracing this API request |

### `CreateCustomProvidersDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | The name of the custom provider |
| `description` | String | **yes** | Description of payment gateway. Shown on the payments integrations page as subtext |
| `paymentsUrl` | String | **yes** | This url will be loaded in iFrame to start a payment session. |
| `queryUrl` | String | **yes** | The url used for querying payments related events. Ex. verify, refund, subscription etc. |
| `imageUrl` | String | **yes** | Public image url for logo of the payment gateway displayed on the payments integrations page. |
| `supportsSubscriptionSchedule` | bool | **yes** | Whether the config supports subscription schedule or not. true represents config supports subscription schedule |

### `CreateCustomProvidersResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | The name of the custom provider |
| `description` | String | **yes** | Description of payment gateway. Shown on the payments integrations page as subtext |
| `paymentsUrl` | String | **yes** | This url will be loaded in iFrame to start a payment session. |
| `queryUrl` | String | **yes** | The url used for querying payments related events. Ex. verify, refund, subscription etc. |
| `imageUrl` | String | **yes** | Public image url for logo of the payment gateway displayed on the payments integrations page. |
| `_id` | String | **yes** | The unique identifier for the custom provider. |
| `locationId` | String | **yes** | Location id |
| `marketplaceAppId` | String | **yes** | The application id of marketplace |
| `paymentProvider` | JSON | no | Payment provider details. |
| `deleted` | bool | **yes** | Whether the config is deleted or not. true represents config is deleted |
| `createdAt` | String | **yes** | The creation timestamp of the custom provider. |
| `updatedAt` | String | **yes** | The last update timestamp of the custom provider. |
| `traceId` | String | no | Trace id of the custom provider. |

### `CreateFulfillmentDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `trackings` | Vec<FulfillmentTracking> | **yes** | Fulfillment tracking information |
| `items` | Vec<FulfillmentItems> | **yes** | Fulfilled items |
| `notifyCustomer` | bool | **yes** | Need to send a notification to customer |

### `CreateFulfillmentResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `data` | [`FulfillmentSchema`](#fulfillmentschema) | **yes** | fulfillment data |

### `CreateWhiteLabelIntegrationProviderDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `uniqueName` | String | **yes** | A unique name given to the integration provider, uniqueName must start and end with a character. Only lowercase characters and hyphens (-) are supported |
| `title` | String | **yes** | The title or name of the integration provider. |
| `provider` | String — `authorize-net`, `nmi` | **yes** | The type of payment provider associated with the integration provider. |
| `description` | String | **yes** | A brief description providing additional information about the integration provider. |
| `imageUrl` | String | **yes** | The URL to an image representing the integration provider. The imageUrl should start with "https://" and ensure that this URL is publicly accessible. |

### `CreateWhitelabelIntegrationResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier of the integration provider. |
| `altId` | String | **yes** | The altId / locationId of the integration provider. |
| `altType` | String | **yes** | The altType of the integration provider. |
| `title` | String | **yes** | The title or name of the integration provider. |
| `route` | String | **yes** | The route name associated with the integration provider. |
| `provider` | String | **yes** | The payment provider associated with the integration provider. |
| `description` | String | **yes** | A brief description providing additional information about the integration provider. |
| `imageUrl` | String | **yes** | The URL to an image representing the integration provider. |
| `createdAt` | String | **yes** | The timestamp when the integration provider was created. |
| `updatedAt` | String | **yes** | The timestamp when the integration provider was last updated. |

### `CustomProviderKeys`

| Field | Type | Required | Description |
|---|---|---|---|
| `apiKey` | String | **yes** | Api-key for custom payment provider config |
| `publishableKey` | String | **yes** | Publishable-key for custom payment provider config |

### `CustomRRuleOptionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `intervalType` | String — `yearly`, `monthly`, `weekly`, `daily`, `hourly`, `minutely`, `secondly` | **yes** | — |
| `interval` | f64 | **yes** | — |
| `startDate` | String | **yes** | Start date in YYYY-MM-DD format |
| `startTime` | String | no | Start time in HH:mm:ss format |
| `endDate` | String | no | End date in YYYY-MM-DD format |
| `endTime` | String | no | End time in HH:mm:ss format |
| `dayOfMonth` | f64 | no | -1, 1, 2, 3, ..., 27, 28 |
| `dayOfWeek` | String — `mo`, `tu`, `we`, `th`, `fr`, `sa`, `su` | no | — |
| `numOfWeek` | f64 | no | -1, 1, 2, 3, 4 |
| `monthOfYear` | String — `jan`, `feb`, `mar`, `apr`, `may`, `jun`, `jul`, `aug`, `sep`, `oct`, `nov`, `dec` | no | — |
| `count` | f64 | no | Max number of task executions |
| `daysBefore` | f64 | no | Execute task number of days before |

### `DefaultPriceResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier for the price. |
| `membershipOffers` | Vec<MembershipOfferDto> | no | An array of membership offers associated with the price. |
| `variantOptionIds` | Vec<String> | no | An array of variant option IDs associated with the price. |
| `locationId` | String | no | The unique identifier for the location. |
| `product` | String | no | The unique identifier for the associated product. |
| `userId` | String | no | The unique identifier for the user. |
| `name` | String | **yes** | The name of the price. |
| `type` | String — `one_time`, `recurring` | **yes** | The type of the price (e.g., one_time). |
| `currency` | String | **yes** | The currency code for the price. |
| `amount` | f64 | **yes** | The amount of the price. |
| `recurring` | [`RecurringDto`](#recurringdto) | no | The recurring details of the price (if type is recurring). |
| `createdAt` | String | no | The creation timestamp of the price. |
| `updatedAt` | String | no | The last update timestamp of the price. |
| `compareAtPrice` | f64 | no | The compare-at price for comparison purposes. |
| `trackInventory` | bool | no | Indicates whether inventory tracking is enabled. |
| `availableQuantity` | f64 | no | Available inventory stock quantity |
| `allowOutOfStockPurchases` | bool | no | Continue selling when out of stock |

### `DefaultProductResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier for the product. |
| `description` | String | no | product description |
| `variants` | Vec<ProductVariantDto> | no | An array of variants for the product. |
| `locationId` | String | **yes** | The unique identifier for the location. |
| `name` | String | **yes** | The name of the product. |
| `productType` | String | **yes** | The type of the product (e.g., PHYSICAL). |
| `availableInStore` | bool | no | Indicates whether the product is available in-store. |
| `createdAt` | String | **yes** | The creation timestamp of the product. |
| `updatedAt` | String | **yes** | The last update timestamp of the product. |
| `statementDescriptor` | String | no | The statement descriptor for the product. |
| `image` | String | no | The URL for the product image. |
| `collectionIds` | Vec<String> | no | An array of category Ids for the product |
| `isTaxesEnabled` | bool | no | The field indicates whether taxes are enabled for the product or not. |
| `taxes` | Vec<String> | no | An array of ids of Taxes attached to the Product. If the expand query includes tax, the taxes will be of type `ProductTaxDto`. Please refer to the `ProductTaxDto` for additional details. |
| `automaticTaxCategoryId` | String | no | Tax category ID for Automatic taxes calculation. |
| `label` | [`ProductLabelDto`](#productlabeldto) | no | The Product label details |
| `slug` | String | no | The slug of the product by which the product will be navigated |

### `DeleteCouponParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id |
| `altType` | String — `location` | **yes** | Alt Type |
| `id` | String | **yes** | Coupon Id |

### `DeleteCouponResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Indicates whether the delete was successful |
| `traceId` | String | **yes** | Unique identifier for tracing this API request |

### `DeleteCustomProvidersConfigDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `liveMode` | bool | **yes** | Whether the config is for test mode or live mode. true represents config is for live payments |

### `DeleteCustomProvidersResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Whether the custom provider config is disconnect or not. true represents config is disconnect |

### `DisconnectCustomProvidersResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Whether the custom provider config is disconnect or not. true represents config is disconnect |

### `FulfilledItem`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The id of product price |
| `name` | String | **yes** | Name |
| `product` | [`DefaultProductResponseDto`](#defaultproductresponsedto) | **yes** | Product details |
| `price` | [`DefaultPriceResponseDto`](#defaultpriceresponsedto) | **yes** | Price details |
| `qty` | f64 | **yes** | The no of quantity of the current fulfilled item |

### `FulfillmentItems`

| Field | Type | Required | Description |
|---|---|---|---|
| `priceId` | String | **yes** | The id of product price |
| `qty` | f64 | **yes** | The no of quantity of the item |

### `FulfillmentSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `trackings` | Vec<FulfillmentTracking> | **yes** | Fulfillment tracking information |
| `_id` | String | **yes** | The unique identifier for the fulfillment item. |
| `items` | Vec<FulfilledItem> | **yes** | Fulfilled items |
| `createdAt` | String | **yes** | created at |
| `updatedAt` | String | **yes** | updated at |

### `FulfillmentTracking`

| Field | Type | Required | Description |
|---|---|---|---|
| `trackingNumber` | String | no | Tracking number provided by the shipping carrier |
| `shippingCarrier` | String | no | Shipping carrier name |
| `trackingUrl` | String | no | Tracking URL |

### `GetCustomProvidersResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | The name of the custom provider |
| `description` | String | **yes** | Description of payment gateway. Shown on the payments integrations page as subtext |
| `paymentsUrl` | String | **yes** | This url will be loaded in iFrame to start a payment session. |
| `queryUrl` | String | **yes** | The url used for querying payments related events. Ex. verify, refund, subscription etc. |
| `imageUrl` | String | **yes** | Public image url for logo of the payment gateway displayed on the payments integrations page. |
| `_id` | String | **yes** | The unique identifier for the custom provider. |
| `locationId` | String | **yes** | Location id |
| `marketplaceAppId` | String | **yes** | The application id of marketplace |
| `paymentProvider` | JSON | no | Payment provider details. |
| `deleted` | bool | **yes** | Whether the config is deleted or not. true represents config is deleted |
| `createdAt` | String | **yes** | The creation timestamp of the custom provider. |
| `updatedAt` | String | **yes** | The last update timestamp of the custom provider. |
| `traceId` | String | no | Trace id of the custom provider. |

### `GetOrderResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier for the order. |
| `altId` | String | **yes** | AltId is the unique identifier eg: location id. |
| `altType` | String | **yes** | AltType is the type of identifier. |
| `contactId` | String | no | Contact id corresponding to the order. |
| `currency` | String | no | Currency in which order was created. |
| `amount` | f64 | no | Order value. |
| `status` | String | **yes** | The status of the order (e.g., completed). |
| `liveMode` | bool | no | Order is in live / test mode. |
| `createdAt` | String | **yes** | The creation timestamp of the order. |
| `updatedAt` | String | **yes** | The last update timestamp of the order. |
| `fulfillmentStatus` | String | no | Fulfillment status of the order. |
| `contactSnapshot` | JSON | no | Contact details of the order. |
| `amountSummary` | [`AmountSummary`](#amountsummary) | no | Amount details of the order. |
| `source` | [`OrderSource`](#ordersource) | no | Source details of the order. |
| `items` | Vec<String> | no | Item details of the order. |
| `coupon` | JSON | no | Coupon details of the order. |
| `trackingId` | String | no | Tracking id of the order. |
| `fingerprint` | String | no | Fingerprint id of the order. |
| `meta` | JSON | no | Meta details of the order. |
| `markAsTest` | bool | no | Is test order. |
| `traceId` | String | no | Trace id of the order. |
| `automaticTaxesCalculated` | bool | no | Automatic taxes applied for the Order |
| `taxCalculationProvider` | JSON | no | Provider name for automatic tax calculation |
| `createdBy` | String | no | User ID who created the order. |

### `GetSubscriptionResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier for the subscription. |
| `altType` | JSON | **yes** | AltType is the type of identifier. |
| `altId` | String | **yes** | AltId is the unique identifier eg: location id. |
| `contactId` | String | no | Contact id corresponding to the subscription. |
| `contactSnapshot` | JSON | no | Contact details of the subscriber. |
| `coupon` | JSON | no | Coupon details of the subscription. |
| `currency` | String | no | Currency in which subscription was made. |
| `amount` | f64 | no | Subscription value. |
| `status` | JSON | no | Subscription status. |
| `liveMode` | bool | no | Subscription is in live / test mode. |
| `entityType` | String | no | Entity type of subscription (eg: order). |
| `entityId` | String | no | Entity id for the subscription. e.g: order id |
| `entitySource` | [`OrderSource`](#ordersource) | no | Entity source details for the subscription. |
| `subscriptionId` | String | no | Subscription id for subscription. |
| `subscriptionSnapshot` | JSON | no | Snapshot of subscription. |
| `paymentProvider` | JSON | no | Payment provider details for the subscription. |
| `ipAddress` | String | no | Ip address from where subscription was initiated. |
| `createdAt` | String | **yes** | The creation timestamp of the subscription. |
| `updatedAt` | String | **yes** | The last update timestamp of the subscription. |
| `meta` | JSON | no | Meta details of the subscription. |
| `markAsTest` | bool | no | Is test subscription. |
| `schedule` | [`ScheduleOptionsDto`](#scheduleoptionsdto) | no | Scedule details for the subscription. |
| `autoPayment` | JSON | no | Auto payment details of the subscription. |
| `recurringProduct` | JSON | no | Recurring product details of the subscription. |
| `canceledAt` | String | no | Cancellation timestamp of the subscription. |
| `canceledBy` | String | no | User id who cancelled the subscription. |
| `traceId` | String | no | Trace id of the subscription. |
| `createdBy` | String | no | User ID who created the subscription. |

### `GetTxnResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier for the transaction. |
| `altType` | String | **yes** | AltType is the type of identifier. |
| `altId` | String | **yes** | AltId is the unique identifier eg: location id. |
| `contactId` | String | no | Contact id corresponding to the transaction. |
| `contactSnapshot` | JSON | no | Contact details of the transaction. |
| `currency` | String | no | Currency in which transaction was made. |
| `amount` | f64 | no | Transaction value. |
| `status` | JSON | no | Transaction status. |
| `liveMode` | bool | no | Transaction is in live / test mode. |
| `createdAt` | String | **yes** | The creation timestamp of the transaction. |
| `updatedAt` | String | **yes** | The last update timestamp of the transaction. |
| `entityType` | String | no | Entity type of transaction (eg: order). |
| `entityId` | String | no | Entity id for the transaction. e.g: order id |
| `entitySource` | [`OrderSource`](#ordersource) | no | Entity source details for the transaction. |
| `chargeId` | String | no | Charge id for transaction. |
| `chargeSnapshot` | JSON | no | Charge snapshot of transaction. |
| `invoiceId` | String | no | Invoice id for the transaction. |
| `subscriptionId` | String | no | Subscription id for transaction. |
| `paymentProvider` | JSON | no | Payment provider details of the transaction. |
| `ipAddress` | String | no | Ip address from where transaction was initiated. |
| `meta` | JSON | no | Meta details of the transaction. |
| `markAsTest` | bool | no | Is test transaction. |
| `isParent` | bool | no | Is parent transaction. |
| `amountRefunded` | f64 | no | Transaction amount refunded. |
| `receiptId` | String | no | Receipt id for transaction. |
| `qboSynced` | bool | no | Is transaction qbo synced. |
| `qboResponse` | JSON | no | Qbo details of the transaction. |
| `traceId` | String | no | Trace id of the transaction. |
| `mergedFromContactId` | String | no | ID of the contact that was merged from. |
| `createdBy` | String | no | User ID who created the transaction. |

### `IntegrationProviderSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier of the integration provider. |
| `altId` | String | **yes** | The altId / locationId of the integration provider. |
| `altType` | String | **yes** | The altType of the integration provider. |
| `title` | String | **yes** | The title or name of the integration provider. |
| `route` | String | **yes** | The route name associated with the integration provider. |
| `provider` | String | **yes** | The payment provider associated with the integration provider. |
| `description` | String | **yes** | A brief description providing additional information about the integration provider. |
| `imageUrl` | String | **yes** | The URL to an image representing the integration provider. |
| `createdAt` | String | **yes** | The timestamp when the integration provider was created. |
| `updatedAt` | String | **yes** | The timestamp when the integration provider was last updated. |

### `ListCouponsResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<CouponDto> | **yes** | Array of coupon objects |
| `totalCount` | f64 | **yes** | Total number of coupons matching the query criteria |
| `traceId` | String | **yes** | Unique identifier for tracing this API request |

### `ListFulfillmentResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `data` | Vec<FulfillmentSchema> | **yes** | An array of fulfilled items |

### `ListOrdersResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<OrderResponseSchema> | **yes** | An array of orders |
| `totalCount` | f64 | **yes** | total orders count |

### `ListSubscriptionResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<SubscriptionResponseSchema> | **yes** | An array of subscriptions |
| `totalCount` | f64 | **yes** | total subscriptions count |

### `ListTxnsResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<TxnResponseSchema> | **yes** | An array of transactions |
| `totalCount` | f64 | **yes** | total transactions count |

### `ListWhitelabelIntegrationProviderResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `providers` | [`IntegrationProviderSchema`](#integrationproviderschema) | **yes** | list of integration provider. |

### `MembershipOfferDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `label` | String | **yes** | Membership offer label |
| `value` | String | **yes** | Membership offer label |
| `_id` | String | **yes** | The unique identifier for the membership offer. |

### `OrderResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier for the order. |
| `altId` | String | **yes** | AltId is the unique identifier eg: location id. |
| `altType` | String | **yes** | AltType is the type of identifier. |
| `contactId` | String | no | Contact id corresponding to the order. |
| `contactName` | String | no | Contact name corresponding to the order. |
| `contactEmail` | String | no | Contact email corresponding to the order. |
| `currency` | String | no | Currency in which order was created. |
| `amount` | f64 | no | Order value. |
| `subtotal` | f64 | no | Order sub-total value. |
| `discount` | f64 | no | Discount value on order. |
| `status` | String | **yes** | The status of the order (e.g., completed). |
| `liveMode` | bool | no | Order is in live / test mode. |
| `totalProducts` | f64 | no | Total products in an order. |
| `sourceType` | String | **yes** | Source type of order (eg: funnel). |
| `sourceName` | String | no | Source name for the order. |
| `sourceId` | String | no | Source id for the order. |
| `sourceMeta` | JSON | no | Meta content for the source of order. |
| `couponCode` | String | no | Coupon code for the order. |
| `createdAt` | String | **yes** | The creation timestamp of the order. |
| `updatedAt` | String | **yes** | The last update timestamp of the order. |
| `sourceSubType` | String | no | Source sub-type for the order. |
| `fulfillmentStatus` | String | no | Fulfillment status of the order. |
| `onetimeProducts` | f64 | no | Total one time products in an order. |
| `recurringProducts` | f64 | no | Total recurring time products in an order. |
| `createdBy` | String | no | User ID who created the order. |

### `OrderSource`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `funnel`, `website`, `invoice`, `calendar`, `text2Pay`, `document_contracts`, `membership`, `mobile_app`, `communities`, `point_of_sale`, `manual`, `form`, `survey`, `payment_link`, `external` | **yes** | — |
| `subType` | String — `one_step_order_form`, `two_step_order_form`, `upsell`, `tap_to_pay`, `card_payment`, `store`, `contact_view`, `email_campaign`, `payments_dashboard`, `shopify`, `subscription_view`, `store_upsell`, `woocommerce`, `service`, `meeting`, `imported_csv`, `qr_code`, `saas_one_time`, `saas_subscription` | no | — |
| `id` | String | **yes** | — |
| `name` | String | no | — |
| `meta` | JSON | no | — |

### `PostRecordOrderPaymentBody`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | location Id / company Id based on altType |
| `altType` | String — `location` | **yes** | Alt Type |
| `mode` | String — `cash`, `card`, `cheque`, `bank_transfer`, `other` | **yes** | manual payment method |
| `card` | [`CardDto`](#carddto) | no | Details of Card if used for payment |
| `cheque` | [`ChequeDto`](#chequedto) | no | Details of the Cheque if used for payment |
| `notes` | String | no | Any note to be recorded with the transaction |
| `amount` | f64 | no | Amount to be paid against the invoice. |
| `meta` | JSON | no | Meta data to be recorded with the transaction |
| `isPartialPayment` | bool | no | Indicates if the order is intended to be a partial payment. |

### `PostRecordOrderPaymentResponse`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Success status of the request |

### `ProductLabelDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | The content for the product label. |
| `startDate` | String | no | Start date in YYYY-MM-DDTHH:mm:ssZ format |
| `endDate` | String | no | Start date in YYYY-MM-DDTHH:mm:ssZ format |

### `ProductVariantDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | A unique identifier for the variant. |
| `name` | String | **yes** | The name of the variant. |
| `options` | Vec<ProductVariantOptionDto> | **yes** | An array of options for the variant. |

### `ProductVariantOptionDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | The unique identifier for the option. |
| `name` | String | **yes** | The name of the option. |

### `RecurringDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `interval` | String — `day`, `month`, `week`, `year` | **yes** | The interval at which the recurring event occurs. |
| `intervalCount` | f64 | **yes** | The number of intervals between each occurrence of the event. |

### `ScheduleOptionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `executeAt` | String | no | — |
| `rrule` | [`CustomRRuleOptionsDto`](#customrruleoptionsdto) | no | — |

### `SubscriptionResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier for the subscription. |
| `altId` | String | **yes** | AltId is the unique identifier eg: location id. |
| `altType` | String — `location` | **yes** | AltType is the type of identifier. |
| `contactId` | String | no | Contact id corresponding to the subscription. |
| `contactName` | String | no | Contact name corresponding to the subscription. |
| `contactEmail` | String | no | Contact email corresponding to the subscription. |
| `currency` | String | no | Currency in which subscription occurred. |
| `amount` | f64 | no | Subscription value. |
| `status` | JSON | **yes** | The status of the subscription (e.g., succeeded). |
| `liveMode` | bool | no | Subscription is in live / test mode. |
| `entityType` | String | no | Entity type of subscription (eg: order). |
| `entityId` | String | no | Entity id for the subscription. e.g: order id |
| `entitySourceType` | String | **yes** | Entity source type of subscription (eg: funnel). |
| `entitySourceName` | String | no | Entity source name for the subscription. |
| `entitySourceId` | String | no | Entity source id for the subscription. |
| `entitySourceMeta` | JSON | no | Meta content for the entity source of subscription. |
| `subscriptionId` | String | no | Subscription id for subscription. |
| `subscriptionSnapshot` | JSON | no | Snapshot of subscription. |
| `paymentProviderType` | String | no | Payment provider for subscription. |
| `paymentProviderConnectedAccount` | String | no | Payment provider connected account id for subscription. |
| `ipAddress` | String | no | Ip address from where subscription was initiated. |
| `createdAt` | String | **yes** | The creation timestamp of the subscription. |
| `updatedAt` | String | **yes** | The update timestamp of the subscription. |
| `createdBy` | String | no | User ID who created the subscription. |

### `TxnResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier for the transaction. |
| `altId` | String | **yes** | AltId is the unique identifier eg: location id. |
| `altType` | String | **yes** | AltType is the type of identifier. |
| `contactId` | String | no | Contact id corresponding to the transaction. |
| `mergedFromContactId` | String | no | ID of the contact that was merged from. |
| `contactName` | String | no | Contact name corresponding to the transaction. |
| `contactEmail` | String | no | Contact email corresponding to the transaction. |
| `currency` | String | no | Currency in which transaction occurred. |
| `amount` | f64 | no | Transaction value. |
| `status` | JSON | **yes** | The status of the transaction (e.g., succeeded). |
| `liveMode` | bool | no | Transaction is in live / test mode. |
| `entityType` | String | no | Entity type of transaction (eg: order). |
| `entityId` | String | no | Entity id for the transaction. e.g: order id |
| `entitySourceType` | String | **yes** | Entity source type of transaction (eg: funnel). |
| `entitySourceSubType` | String | no | Entity source sub-type of the transactions. |
| `entitySourceName` | String | no | Entity source name for the transaction. |
| `entitySourceId` | String | no | Entity source id for the transaction. |
| `entitySourceMeta` | JSON | no | Meta content for the entity source of transaction. |
| `subscriptionId` | String | no | Subscription id for transaction. |
| `chargeId` | String | no | Charge id for transaction. |
| `chargeSnapshot` | JSON | no | Charge snapshot of transaction. |
| `paymentProviderType` | String | no | Payment provider for transaction. |
| `paymentProviderConnectedAccount` | String | no | Payment provider account id for transaction. |
| `ipAddress` | String | no | Ip address from where transaction was initiated. |
| `createdAt` | String | **yes** | The creation timestamp of the transaction. |
| `updatedAt` | String | **yes** | The update timestamp of the transaction. |
| `amountRefunded` | f64 | no | Transaction amount refunded. |
| `paymentMethod` | JSON | no | Transaction payment method details. |
| `fulfilledAt` | String | **yes** | The charged timestamp of the transaction. |
| `createdBy` | String | no | User ID who created the transaction. |

### `UpdateCouponParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id |
| `altType` | String — `location` | **yes** | Alt Type |
| `name` | String | **yes** | Coupon Name |
| `code` | String | **yes** | Coupon Code |
| `discountType` | String — `percentage`, `amount` | **yes** | Discount Type |
| `discountValue` | f64 | **yes** | Discount Value |
| `startDate` | String | **yes** | Start date in YYYY-MM-DDTHH:mm:ssZ format |
| `endDate` | String | no | End date in YYYY-MM-DDTHH:mm:ssZ format |
| `usageLimit` | f64 | no | Max number of times coupon can be used |
| `productIds` | Vec<String> | no | Product Ids |
| `priceIds` | Vec<String> | no | Price Ids |
| `variantIds` | Vec<String> | no | Variant Ids |
| `applyToFuturePayments` | bool | no | Is Coupon applicable on upcoming subscription transactions |
| `applyToFuturePaymentsConfig` | [`ApplyToFuturePaymentsConfig`](#applytofuturepaymentsconfig) | no | If coupon is applicable on upcoming subscription transactions, how many months should it be applicable for a subscription |
| `limitPerCustomer` | bool | no | Limits whether a coupon can be redeemed only once per customer. |
| `id` | String | **yes** | Coupon Id |

### `UpdateCustomProviderCapabilitiesDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `supportsSubscriptionSchedules` | bool | **yes** | Whether the marketplace app supports subscription schedules or not |
| `companyId` | String | no | Company id. Mandatory if locationId is not provided |
| `locationId` | String | no | Location / Sub-account id. Mandatory if companyId is not provided |

### `UpdateCustomProviderCapabilitiesResponseSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Whether the custom provider capabilities are updated or not. true represents capabilities are updated |

