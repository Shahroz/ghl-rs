# `products`

**27** operations / **64** models in API v2 · **27** operations / **64** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `products` cargo feature on `ghl-sdk`, then call any of the 54 generated methods on `ghl.products()` (v2) or `ghl.v3().products()` (v3):

```toml
ghl-sdk = { version = "0.5", features = ["products"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/products/` | List Products | `list_products()` | `products.get_products` |
| `POST` | `/products/` | Create Product | `create_product()` | `products.post_products` |
| `POST` | `/products/bulk-update` | Bulk Update Products | `bulk_update_products()` | `products.post_products_bulk_update` |
| `POST` | `/products/bulk-update/edit` | Bulk Edit Products and Prices | `bulk_edit_products_and_prices()` | `products.post_products_bulk_update_edit` |
| `GET` | `/products/collections` | Fetch Product Collections | `fetch_product_collections()` | `products.get_products_collections` |
| `POST` | `/products/collections` | Create Product Collection | `create_product_collection()` | `products.post_products_collections` |
| `DELETE` | `/products/collections/{collectionId}` | Delete Product Collection | `delete_product_collection()` | `products.delete_products_collections_by_collectionId` |
| `GET` | `/products/collections/{collectionId}` | Get Details about individual product collection | `get_details_about_individual_product_collection()` | `products.get_products_collections_by_collectionId` |
| `PUT` | `/products/collections/{collectionId}` | Update Product Collection | `update_product_collection()` | `products.put_products_collections_by_collectionId` |
| `GET` | `/products/inventory` | List Inventory | `list_inventory()` | `products.get_products_inventory` |
| `POST` | `/products/inventory` | Update Inventory | `update_inventory()` | `products.post_products_inventory` |
| `GET` | `/products/reviews` | Fetch Product Reviews | `fetch_product_reviews()` | `products.get_products_reviews` |
| `POST` | `/products/reviews/bulk-update` | Update Product Reviews | `update_product_reviews()` | `products.post_products_reviews_bulk_update` |
| `GET` | `/products/reviews/count` | Fetch Review Count as per status | `fetch_review_count_as_per_status()` | `products.get_products_reviews_count` |
| `DELETE` | `/products/reviews/{reviewId}` | Delete Product Review | `delete_product_review()` | `products.delete_products_reviews_by_reviewId` |
| `PUT` | `/products/reviews/{reviewId}` | Update Product Reviews | `update_product_reviews_op()` | `products.put_products_reviews_by_reviewId` |
| `POST` | `/products/store/{storeId}` | Action to include/exclude the product in store | `action_to_include_exclude_the_product_in_store()` | `products.post_products_store_by_storeId` |
| `POST` | `/products/store/{storeId}/priority` | Update product display priorities in store | `update_product_display_priorities_in_store()` | `products.post_products_store_by_storeId_priority` |
| `GET` | `/products/store/{storeId}/stats` | Fetch Product Store Stats | `fetch_product_store_stats()` | `products.get_products_store_by_storeId_stats` |
| `DELETE` | `/products/{productId}` | Delete Product by ID | `delete_product_by_id()` | `products.delete_products_by_productId` |
| `GET` | `/products/{productId}` | Get Product by ID | `get_product_by_id()` | `products.get_products_by_productId` |
| `PUT` | `/products/{productId}` | Update Product by ID | `update_product_by_id()` | `products.put_products_by_productId` |
| `GET` | `/products/{productId}/price` | List Prices for a Product | `list_prices_for_a_product()` | `products.get_products_by_productId_price` |
| `POST` | `/products/{productId}/price` | Create Price for a Product | `create_price_for_a_product()` | `products.post_products_by_productId_price` |
| `DELETE` | `/products/{productId}/price/{priceId}` | Delete Price by ID for a Product | `delete_price_by_id_for_a_product()` | `products.delete_products_by_productId_price_by_priceId` |
| `GET` | `/products/{productId}/price/{priceId}` | Get Price by ID for a Product | `get_price_by_id_for_a_product()` | `products.get_products_by_productId_price_by_priceId` |
| `PUT` | `/products/{productId}/price/{priceId}` | Update Price by ID for a Product | `update_price_by_id_for_a_product()` | `products.put_products_by_productId_price_by_priceId` |

### Endpoint details — v2

#### `GET /products/`

**List Products**

The "List Products" API allows to retrieve a paginated list of products. Customize your results by filtering products based on name or paginate through the list using the provided query parameters. This endpoint provides a straightforward way to explore and retrieve product information.

Operation id: `products.get_products` · `Version: 2021-07-28` · Scopes: `products.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |
| `locationId` | string | **yes** | LocationId is the id of the sub-account |
| `search` | string | no | The name of the product for searching. |
| `collectionIds` | string | no | Filter by product category Ids. Supports comma separated values |
| `collectionSlug` | string | no | The slug value of the collection by which the collection would be searched |
| `expand` | array | no | Name of an entity whose data has to be fetched along with product. Possible entities are tax, stripe and paypal. If not mentioned, only ID will be returned in c… |
| `productIds` | array | no | List of product ids to be fetched. |
| `storeId` | string | no | fetch and project products based on the storeId |
| `includedInStore` | boolean | no | Separate products by which are included in the store and which are not |
| `availableInStore` | boolean | no | If the product is included in the online store |
| `sortOrder` | enum: `asc`, `desc` | no | The order of sort which should be applied for the date |

*Response*: [`ListProductsResponseDto`](#listproductsresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::products::ListProductsParams;

let params = ListProductsParams::new("locationId");
let out = ghl.products().list_products(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.get_products",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /products/`

**Create Product**

The "Create Product" API allows adding a new product to the system. Use this endpoint to create a product with the specified details. Ensure that the required information is provided in the request payload.

Operation id: `products.post_products` · `Version: 2021-07-28` · Scopes: `products.write`

*Request body*: [`CreateProductDto`](#createproductdto)

*Response*: [`CreateProductResponseDto`](#createproductresponsedto)

*Rust*:

```rust,ignore
let out = ghl.products().create_product(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.post_products",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /products/bulk-update`

**Bulk Update Products**

API to bulk update products (price, availability, collections, delete)

Operation id: `products.post_products_bulk_update` · `Version: 2021-07-28` · Scopes: `products.write`

*Request body*: [`BulkUpdateDto`](#bulkupdatedto)

*Response*: [`BulkUpdateResponseDto`](#bulkupdateresponsedto)

*Rust*:

```rust,ignore
let out = ghl.products().bulk_update_products(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.post_products_bulk_update",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /products/bulk-update/edit`

**Bulk Edit Products and Prices**

API to bulk edit products and their associated prices (max 30 entities)

Operation id: `products.post_products_bulk_update_edit` · `Version: 2021-07-28`

*Request body*: [`BulkEditRequestDto`](#bulkeditrequestdto)

*Response*: [`BulkEditResponseDto`](#bulkeditresponsedto)

*Rust*:

```rust,ignore
let out = ghl.products().bulk_edit_products_and_prices(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.post_products_bulk_update_edit",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /products/collections`

**Fetch Product Collections**

Internal API to fetch the Product Collections

Operation id: `products.get_products_collections` · `Version: 2021-07-28` · Scopes: `products/collection.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |
| `altId` | string | **yes** | Location Id |
| `altType` | enum: `location` | **yes** | The type of alt. For now it is only LOCATION |
| `collectionIds` | string | no | Ids of the collections separated by comma(,) for search purposes |
| `name` | string | no | Query to search collection based on names |

*Response*: [`ListCollectionResponseDto`](#listcollectionresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::products::FetchProductCollectionsParams;

let params = FetchProductCollectionsParams::new("altId", "altType");
let out = ghl.products().fetch_product_collections(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.get_products_collections",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `POST /products/collections`

**Create Product Collection**

Create a new Product Collection for a specific location

Operation id: `products.post_products_collections` · `Version: 2021-07-28` · Scopes: `products/collection.write`

*Request body*: [`CreateProductCollectionsDto`](#createproductcollectionsdto)

*Response*: [`CreateCollectionResponseDto`](#createcollectionresponsedto)

*Rust*:

```rust,ignore
let out = ghl.products().create_product_collection(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.post_products_collections",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /products/collections/{collectionId}`

**Delete Product Collection**

Delete specific product collection with Id :collectionId

Operation id: `products.delete_products_collections_by_collectionId` · `Version: 2021-07-28` · Scopes: `products/collection.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `collectionId` | string | **yes** | MongoId of the collection |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id |
| `altType` | enum: `location` | **yes** | The type of alt. For now it is only LOCATION |

*Response*: [`DeleteProductCollectionResponseDto`](#deleteproductcollectionresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::products::DeleteProductCollectionParams;

let params = DeleteProductCollectionParams::new("altId", "altType");
let out = ghl.products().delete_product_collection(&collectionId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.delete_products_collections_by_collectionId",
    "path_params": {
      "collectionId": "<collectionId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /products/collections/{collectionId}`

**Get Details about individual product collection**

Operation id: `products.get_products_collections_by_collectionId` · `Version: 2021-07-28` · Scopes: `products/collection.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `collectionId` | string | **yes** | Collection Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id |

*Response*: [`DefaultCollectionResponseDto`](#defaultcollectionresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::products::GetDetailsAboutIndividualProductCollectionParams;

let params = GetDetailsAboutIndividualProductCollectionParams::new("altId");
let out = ghl.products().get_details_about_individual_product_collection(&collectionId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.get_products_collections_by_collectionId",
    "path_params": {
      "collectionId": "<collectionId>"
    },
    "query": {
      "altId": "<altId>"
    }
  }
}
```

</details>

#### `PUT /products/collections/{collectionId}`

**Update Product Collection**

Update a specific product collection with Id :collectionId

Operation id: `products.put_products_collections_by_collectionId` · `Version: 2021-07-28` · Scopes: `products/collection.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `collectionId` | string | **yes** | MongoId of the collection |

*Request body*: [`UpdateProductCollectionsDto`](#updateproductcollectionsdto)

*Response*: [`UpdateProductCollectionResponseDto`](#updateproductcollectionresponsedto)

*Rust*:

```rust,ignore
let out = ghl.products().update_product_collection(&collectionId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.put_products_collections_by_collectionId",
    "path_params": {
      "collectionId": "<collectionId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /products/inventory`

**List Inventory**

The "List Inventory API allows the user to retrieve a paginated list of inventory items. Use this endpoint to fetch details for multiple items in the inventory based on the provided query parameters.

Operation id: `products.get_products_inventory` · `Version: 2021-07-28` · Scopes: `products/prices.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `search` | string | no | Search string for Variant Search |

*Response*: [`GetInventoryResponseDto`](#getinventoryresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::products::ListInventoryParams;

let params = ListInventoryParams::new("altId", "altType");
let out = ghl.products().list_inventory(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.get_products_inventory",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `POST /products/inventory`

**Update Inventory**

The Update Inventory API allows the user to bulk update the inventory for multiple items. Use this endpoint to update the available quantity and out-of-stock purchase settings for multiple items in the inventory.

Operation id: `products.post_products_inventory` · `Version: 2021-07-28` · Scopes: `products/prices.write`

*Request body*: [`UpdateInventoryDto`](#updateinventorydto)

*Response*: [`UpdateInventoryResponseDto`](#updateinventoryresponsedto)

*Rust*:

```rust,ignore
let out = ghl.products().update_inventory(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.post_products_inventory",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /products/reviews`

**Fetch Product Reviews**

API to fetch the Product Reviews

Operation id: `products.get_products_reviews` · `Version: 2021-07-28` · Scopes: `products.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |
| `sortField` | enum: `createdAt`, `rating` | no | The field upon which the sort should be applied |
| `sortOrder` | enum: `asc`, `desc` | no | The order of sort which should be applied for the sortField |
| `rating` | number | no | Key to filter the ratings |
| `startDate` | string | no | The start date for filtering reviews |
| `endDate` | string | no | The end date for filtering reviews |
| `productId` | string | no | Comma-separated list of product IDs |
| `storeId` | string | no | Comma-separated list of store IDs |

*Response*: [`ListProductReviewsResponseDto`](#listproductreviewsresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::products::FetchProductReviewsParams;

let params = FetchProductReviewsParams::new("altId", "altType");
let out = ghl.products().fetch_product_reviews(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.get_products_reviews",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `POST /products/reviews/bulk-update`

**Update Product Reviews**

Update one or multiple product reviews: status, reply, etc.

Operation id: `products.post_products_reviews_bulk_update` · `Version: 2021-07-28` · Scopes: `products.write`

*Request body*: [`UpdateProductReviewsDto`](#updateproductreviewsdto)

*Response*: [`UpdateProductReviewsResponseDto`](#updateproductreviewsresponsedto)

*Rust*:

```rust,ignore
let out = ghl.products().update_product_reviews(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.post_products_reviews_bulk_update",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /products/reviews/count`

**Fetch Review Count as per status**

API to fetch the Review Count as per status

Operation id: `products.get_products_reviews_count` · `Version: 2021-07-28` · Scopes: `products.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `rating` | number | no | Key to filter the ratings |
| `startDate` | string | no | The start date for filtering reviews |
| `endDate` | string | no | The end date for filtering reviews |
| `productId` | string | no | Comma-separated list of product IDs |
| `storeId` | string | no | Comma-separated list of store IDs |

*Response*: [`CountReviewsByStatusResponseDto`](#countreviewsbystatusresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::products::FetchReviewCountAsPerStatusParams;

let params = FetchReviewCountAsPerStatusParams::new("altId", "altType");
let out = ghl.products().fetch_review_count_as_per_status(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.get_products_reviews_count",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `DELETE /products/reviews/{reviewId}`

**Delete Product Review**

Delete specific product review

Operation id: `products.delete_products_reviews_by_reviewId` · `Version: 2021-07-28` · Scopes: `products.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `reviewId` | string | **yes** | Review Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `productId` | string | **yes** | Product Id of the product |

*Response*: [`DeleteProductReviewResponseDto`](#deleteproductreviewresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::products::DeleteProductReviewParams;

let params = DeleteProductReviewParams::new("altId", "altType", "productId");
let out = ghl.products().delete_product_review(&reviewId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.delete_products_reviews_by_reviewId",
    "path_params": {
      "reviewId": "<reviewId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>",
      "productId": "<productId>"
    }
  }
}
```

</details>

#### `PUT /products/reviews/{reviewId}`

**Update Product Reviews**

Update status, reply, etc of a particular review

Operation id: `products.put_products_reviews_by_reviewId` · `Version: 2021-07-28` · Scopes: `products.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `reviewId` | string | **yes** | Review Id |

*Request body*: [`UpdateProductReviewDto`](#updateproductreviewdto)

*Response*: [`UpdateProductReviewsResponseDto`](#updateproductreviewsresponsedto)

*Rust*:

```rust,ignore
let out = ghl.products().update_product_reviews_op(&reviewId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.put_products_reviews_by_reviewId",
    "path_params": {
      "reviewId": "<reviewId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /products/store/{storeId}`

**Action to include/exclude the product in store**

API to update the status of products in a particular store

Operation id: `products.post_products_store_by_storeId` · `Version: 2021-07-28` · Scopes: `products.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `storeId` | string | **yes** | Products related to the store |

*Request body*: [`UpdateProductStoreDto`](#updateproductstoredto)

*Response*: [`UpdateProductStoreResponseDto`](#updateproductstoreresponsedto)

*Rust*:

```rust,ignore
let out = ghl.products().action_to_include_exclude_the_product_in_store(&storeId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.post_products_store_by_storeId",
    "path_params": {
      "storeId": "<storeId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /products/store/{storeId}/priority`

**Update product display priorities in store**

API to set the display priority of products in a store

Operation id: `products.post_products_store_by_storeId_priority` · `Version: 2021-07-28`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `storeId` | string | **yes** | Products related to the store |

*Request body*: [`UpdateDisplayPriorityBodyDto`](#updatedisplayprioritybodydto)

*Rust*:

```rust,ignore
let out = ghl.products().update_product_display_priorities_in_store(&storeId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.post_products_store_by_storeId_priority",
    "path_params": {
      "storeId": "<storeId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /products/store/{storeId}/stats`

**Fetch Product Store Stats**

API to fetch the total number of products, included in the store, and excluded from the store and other stats

Operation id: `products.get_products_store_by_storeId_stats` · `Version: 2021-07-28` · Scopes: `products.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `storeId` | string | **yes** | Products related to the store |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `search` | string | no | The name of the product for searching. |
| `collectionIds` | string | no | Filter by product collection Ids. Supports comma separated values |

*Response*: [`GetProductStatsResponseDto`](#getproductstatsresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::products::FetchProductStoreStatsParams;

let params = FetchProductStoreStatsParams::new("altId", "altType");
let out = ghl.products().fetch_product_store_stats(&storeId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.get_products_store_by_storeId_stats",
    "path_params": {
      "storeId": "<storeId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `DELETE /products/{productId}`

**Delete Product by ID**

The "Delete Product by ID" API allows deleting a specific product using its unique identifier. Use this endpoint to remove a product from the system.

Operation id: `products.delete_products_by_productId` · `Version: 2021-07-28` · Scopes: `products.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `productId` | string | **yes** | ID or the slug of the product that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | location Id |
| `sendWishlistStatus` | boolean | no | Parameter which will decide whether to show the wishlisting status of products |

*Response*: [`DeleteProductResponseDto`](#deleteproductresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::products::DeleteProductByIdParams;

let params = DeleteProductByIdParams::new("locationId");
let out = ghl.products().delete_product_by_id(&productId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.delete_products_by_productId",
    "path_params": {
      "productId": "<productId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /products/{productId}`

**Get Product by ID**

The "Get Product by ID" API allows to retrieve information for a specific product using its unique identifier. Use this endpoint to fetch details for a single product based on the provided product ID.

Operation id: `products.get_products_by_productId` · `Version: 2021-07-28` · Scopes: `products.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `productId` | string | **yes** | ID or the slug of the product that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | location Id |
| `sendWishlistStatus` | boolean | no | Parameter which will decide whether to show the wishlisting status of products |

*Response*: [`GetProductResponseDto`](#getproductresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::products::GetProductByIdParams;

let params = GetProductByIdParams::new("locationId");
let out = ghl.products().get_product_by_id(&productId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.get_products_by_productId",
    "path_params": {
      "productId": "<productId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /products/{productId}`

**Update Product by ID**

The "Update Product by ID" API allows modifying information for a specific product using its unique identifier. Use this endpoint to update details for a single product based on the provided product ID.

Operation id: `products.put_products_by_productId` · `Version: 2021-07-28` · Scopes: `products.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `productId` | string | **yes** | ID or the slug of the product that needs to be returned |

*Request body*: [`UpdateProductDto`](#updateproductdto)

*Response*: [`UpdateProductResponseDto`](#updateproductresponsedto)

*Rust*:

```rust,ignore
let out = ghl.products().update_product_by_id(&productId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.put_products_by_productId",
    "path_params": {
      "productId": "<productId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /products/{productId}/price`

**List Prices for a Product**

The "List Prices for a Product" API allows retrieving a paginated list of prices associated with a specific product. Customize your results by filtering prices or paginate through the list using the provided query parameters.

Operation id: `products.get_products_by_productId_price` · `Version: 2021-07-28` · Scopes: `products/prices.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `productId` | string | **yes** | ID of the product that needs to be used |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |
| `locationId` | string | **yes** | The unique identifier for the location. |
| `ids` | string | no | To filter the response only with the given price ids, Please provide with comma separated |

*Response*: [`ListPricesResponseDto`](#listpricesresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::products::ListPricesForAProductParams;

let params = ListPricesForAProductParams::new("locationId");
let out = ghl.products().list_prices_for_a_product(&productId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.get_products_by_productId_price",
    "path_params": {
      "productId": "<productId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /products/{productId}/price`

**Create Price for a Product**

The "Create Price for a Product" API allows adding a new price associated with a specific product to the system. Use this endpoint to create a price with the specified details for a particular product. Ensure that the required information is provided in the request payload.

Operation id: `products.post_products_by_productId_price` · `Version: 2021-07-28` · Scopes: `products/prices.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `productId` | string | **yes** | ID of the product that needs to be used |

*Request body*: [`CreatePriceDto`](#createpricedto)

*Response*: [`CreatePriceResponseDto`](#createpriceresponsedto)

*Rust*:

```rust,ignore
let out = ghl.products().create_price_for_a_product(&productId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.post_products_by_productId_price",
    "path_params": {
      "productId": "<productId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /products/{productId}/price/{priceId}`

**Delete Price by ID for a Product**

The "Delete Price by ID for a Product" API allows deleting a specific price associated with a particular product using its unique identifier. Use this endpoint to remove a price from the system.

Operation id: `products.delete_products_by_productId_price_by_priceId` · `Version: 2021-07-28` · Scopes: `products/prices.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `productId` | string | **yes** | ID of the product that needs to be used |
| `priceId` | string | **yes** | ID of the price that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | location Id |

*Response*: [`DeletePriceResponseDto`](#deletepriceresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::products::DeletePriceByIdForAProductParams;

let params = DeletePriceByIdForAProductParams::new("locationId");
let out = ghl.products().delete_price_by_id_for_a_product(&productId, &priceId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.delete_products_by_productId_price_by_priceId",
    "path_params": {
      "productId": "<productId>",
      "priceId": "<priceId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /products/{productId}/price/{priceId}`

**Get Price by ID for a Product**

The "Get Price by ID for a Product" API allows retrieving information for a specific price associated with a particular product using its unique identifier. Use this endpoint to fetch details for a single price based on the provided price ID and product ID.

Operation id: `products.get_products_by_productId_price_by_priceId` · `Version: 2021-07-28` · Scopes: `products/prices.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `productId` | string | **yes** | ID of the product that needs to be used |
| `priceId` | string | **yes** | ID of the price that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | location Id |

*Response*: [`GetPriceResponseDto`](#getpriceresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::products::GetPriceByIdForAProductParams;

let params = GetPriceByIdForAProductParams::new("locationId");
let out = ghl.products().get_price_by_id_for_a_product(&productId, &priceId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.get_products_by_productId_price_by_priceId",
    "path_params": {
      "productId": "<productId>",
      "priceId": "<priceId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /products/{productId}/price/{priceId}`

**Update Price by ID for a Product**

The "Update Price by ID for a Product" API allows modifying information for a specific price associated with a particular product using its unique identifier. Use this endpoint to update details for a single price based on the provided price ID and product ID.

Operation id: `products.put_products_by_productId_price_by_priceId` · `Version: 2021-07-28` · Scopes: `products/prices.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `productId` | string | **yes** | ID of the product that needs to be used |
| `priceId` | string | **yes** | ID of the price that needs to be returned |

*Request body*: [`UpdatePriceDto`](#updatepricedto)

*Response*: [`UpdatePriceResponseDto`](#updatepriceresponsedto)

*Rust*:

```rust,ignore
let out = ghl.products().update_price_by_id_for_a_product(&productId, &priceId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "products.put_products_by_productId_price_by_priceId",
    "path_params": {
      "productId": "<productId>",
      "priceId": "<priceId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

## Endpoints — API v3

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/products/` | List Products | `list_products()` | `v3:products.get_products` |
| `POST` | `/products/` | Create Product | `create_product()` | `v3:products.post_products` |
| `POST` | `/products/bulk-update` | Bulk Update Products | `bulk_update_products()` | `v3:products.post_products_bulk_update` |
| `POST` | `/products/bulk-update/edit` | Bulk Edit Products and Prices | `bulk_edit_products_and_prices()` | `v3:products.post_products_bulk_update_edit` |
| `GET` | `/products/collections` | Fetch Product Collections | `fetch_product_collections()` | `v3:products.get_products_collections` |
| `POST` | `/products/collections` | Create Product Collection | `create_product_collection()` | `v3:products.post_products_collections` |
| `DELETE` | `/products/collections/{collectionId}` | Delete Product Collection | `delete_product_collection()` | `v3:products.delete_products_collections_by_collectionId` |
| `GET` | `/products/collections/{collectionId}` | Get Details about individual product collection | `get_details_about_individual_product_collection()` | `v3:products.get_products_collections_by_collectionId` |
| `PUT` | `/products/collections/{collectionId}` | Update Product Collection | `update_product_collection()` | `v3:products.put_products_collections_by_collectionId` |
| `GET` | `/products/inventory` | List Inventory | `list_inventory()` | `v3:products.get_products_inventory` |
| `POST` | `/products/inventory` | Update Inventory | `update_inventory()` | `v3:products.post_products_inventory` |
| `GET` | `/products/reviews` | Fetch Product Reviews | `fetch_product_reviews()` | `v3:products.get_products_reviews` |
| `POST` | `/products/reviews/bulk-update` | Update Product Reviews | `update_product_reviews()` | `v3:products.post_products_reviews_bulk_update` |
| `GET` | `/products/reviews/count` | Fetch Review Count as per status | `fetch_review_count_as_per_status()` | `v3:products.get_products_reviews_count` |
| `DELETE` | `/products/reviews/{reviewId}` | Delete Product Review | `delete_product_review()` | `v3:products.delete_products_reviews_by_reviewId` |
| `PUT` | `/products/reviews/{reviewId}` | Update Product Reviews | `update_product_reviews_op()` | `v3:products.put_products_reviews_by_reviewId` |
| `POST` | `/products/store/{storeId}` | Action to include/exclude the product in store | `action_to_include_exclude_the_product_in_store()` | `v3:products.post_products_store_by_storeId` |
| `POST` | `/products/store/{storeId}/priority` | Update product display priorities in store | `update_product_display_priorities_in_store()` | `v3:products.post_products_store_by_storeId_priority` |
| `GET` | `/products/store/{storeId}/stats` | Fetch Product Store Stats | `fetch_product_store_stats()` | `v3:products.get_products_store_by_storeId_stats` |
| `DELETE` | `/products/{productId}` | Delete Product by ID | `delete_product_by_id()` | `v3:products.delete_products_by_productId` |
| `GET` | `/products/{productId}` | Get Product by ID | `get_product_by_id()` | `v3:products.get_products_by_productId` |
| `PUT` | `/products/{productId}` | Update Product by ID | `update_product_by_id()` | `v3:products.put_products_by_productId` |
| `GET` | `/products/{productId}/price` | List Prices for a Product | `list_prices_for_a_product()` | `v3:products.get_products_by_productId_price` |
| `POST` | `/products/{productId}/price` | Create Price for a Product | `create_price_for_a_product()` | `v3:products.post_products_by_productId_price` |
| `DELETE` | `/products/{productId}/price/{priceId}` | Delete Price by ID for a Product | `delete_price_by_id_for_a_product()` | `v3:products.delete_products_by_productId_price_by_priceId` |
| `GET` | `/products/{productId}/price/{priceId}` | Get Price by ID for a Product | `get_price_by_id_for_a_product()` | `v3:products.get_products_by_productId_price_by_priceId` |
| `PUT` | `/products/{productId}/price/{priceId}` | Update Price by ID for a Product | `update_price_by_id_for_a_product()` | `v3:products.put_products_by_productId_price_by_priceId` |

### Endpoint details — v3

#### `GET /products/`

**List Products**

The "List Products" API allows to retrieve a paginated list of products. Customize your results by filtering products based on name or paginate through the list using the provided query parameters. This endpoint provides a straightforward way to explore and retrieve product information.

Operation id: `v3:products.get_products` · `Version: v3` · Scopes: `products.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |
| `locationId` | string | **yes** | LocationId is the id of the sub-account |
| `search` | string | no | The name of the product for searching. |
| `collectionIds` | string | no | Filter by product category Ids. Supports comma separated values |
| `collectionSlug` | string | no | The slug value of the collection by which the collection would be searched |
| `expand` | array | no | Name of an entity whose data has to be fetched along with product. Possible entities are tax, stripe and paypal. If not mentioned, only ID will be returned in c… |
| `productIds` | array | no | List of product ids to be fetched. |
| `storeId` | string | no | fetch and project products based on the storeId |
| `includedInStore` | boolean | no | Separate products by which are included in the store and which are not |
| `availableInStore` | boolean | no | If the product is included in the online store |
| `sortOrder` | enum: `asc`, `desc` | no | The order of sort which should be applied for the date |

*Response*: [`ListProductsResponseDto`](#listproductsresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::products::ListProductsParams;

let params = ListProductsParams::new("locationId");
let out = ghl.v3().products().list_products(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.get_products",
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /products/`

**Create Product**

The "Create Product" API allows adding a new product to the system. Use this endpoint to create a product with the specified details. Ensure that the required information is provided in the request payload.

Operation id: `v3:products.post_products` · `Version: v3` · Scopes: `products.write`

*Request body*: [`CreateProductDto`](#createproductdto)

*Response*: [`CreateProductResponseDto`](#createproductresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().products().create_product(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.post_products",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /products/bulk-update`

**Bulk Update Products**

API to bulk update products (price, availability, collections, delete)

Operation id: `v3:products.post_products_bulk_update` · `Version: v3` · Scopes: `products.write`

*Request body*: [`BulkUpdateDto`](#bulkupdatedto)

*Response*: [`BulkUpdateResponseDto`](#bulkupdateresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().products().bulk_update_products(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.post_products_bulk_update",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /products/bulk-update/edit`

**Bulk Edit Products and Prices**

API to bulk edit products and their associated prices (max 30 entities)

Operation id: `v3:products.post_products_bulk_update_edit` · `Version: v3`

*Request body*: [`BulkEditRequestDto`](#bulkeditrequestdto)

*Response*: [`BulkEditResponseDto`](#bulkeditresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().products().bulk_edit_products_and_prices(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.post_products_bulk_update_edit",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /products/collections`

**Fetch Product Collections**

Internal API to fetch the Product Collections

Operation id: `v3:products.get_products_collections` · `Version: v3` · Scopes: `products/collection.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |
| `altId` | string | **yes** | Location Id |
| `altType` | enum: `location` | **yes** | The type of alt. For now it is only LOCATION |
| `collectionIds` | string | no | Ids of the collections separated by comma(,) for search purposes |
| `name` | string | no | Query to search collection based on names |

*Response*: [`ListCollectionResponseDto`](#listcollectionresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::products::FetchProductCollectionsParams;

let params = FetchProductCollectionsParams::new("altId", "altType");
let out = ghl.v3().products().fetch_product_collections(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.get_products_collections",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `POST /products/collections`

**Create Product Collection**

Create a new Product Collection for a specific location

Operation id: `v3:products.post_products_collections` · `Version: v3` · Scopes: `products/collection.write`

*Request body*: [`CreateProductCollectionsDto`](#createproductcollectionsdto)

*Response*: [`CreateCollectionResponseDto`](#createcollectionresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().products().create_product_collection(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.post_products_collections",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /products/collections/{collectionId}`

**Delete Product Collection**

Delete specific product collection with Id :collectionId

Operation id: `v3:products.delete_products_collections_by_collectionId` · `Version: v3` · Scopes: `products/collection.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `collectionId` | string | **yes** | MongoId of the collection |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id |
| `altType` | enum: `location` | **yes** | The type of alt. For now it is only LOCATION |

*Response*: [`DeleteProductCollectionResponseDto`](#deleteproductcollectionresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::products::DeleteProductCollectionParams;

let params = DeleteProductCollectionParams::new("altId", "altType");
let out = ghl.v3().products().delete_product_collection(&collectionId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.delete_products_collections_by_collectionId",
    "path_params": {
      "collectionId": "<collectionId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `GET /products/collections/{collectionId}`

**Get Details about individual product collection**

Operation id: `v3:products.get_products_collections_by_collectionId` · `Version: v3` · Scopes: `products/collection.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `collectionId` | string | **yes** | Collection Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id |

*Response*: [`DefaultCollectionResponseDto`](#defaultcollectionresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::products::GetDetailsAboutIndividualProductCollectionParams;

let params = GetDetailsAboutIndividualProductCollectionParams::new("altId");
let out = ghl.v3().products().get_details_about_individual_product_collection(&collectionId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.get_products_collections_by_collectionId",
    "path_params": {
      "collectionId": "<collectionId>"
    },
    "query": {
      "altId": "<altId>"
    }
  }
}
```

</details>

#### `PUT /products/collections/{collectionId}`

**Update Product Collection**

Update a specific product collection with Id :collectionId

Operation id: `v3:products.put_products_collections_by_collectionId` · `Version: v3` · Scopes: `products/collection.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `collectionId` | string | **yes** | MongoId of the collection |

*Request body*: [`UpdateProductCollectionsDto`](#updateproductcollectionsdto)

*Response*: [`UpdateProductCollectionResponseDto`](#updateproductcollectionresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().products().update_product_collection(&collectionId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.put_products_collections_by_collectionId",
    "path_params": {
      "collectionId": "<collectionId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /products/inventory`

**List Inventory**

The "List Inventory API allows the user to retrieve a paginated list of inventory items. Use this endpoint to fetch details for multiple items in the inventory based on the provided query parameters.

Operation id: `v3:products.get_products_inventory` · `Version: v3` · Scopes: `products/prices.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `search` | string | no | Search string for Variant Search |

*Response*: [`GetInventoryResponseDto`](#getinventoryresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::products::ListInventoryParams;

let params = ListInventoryParams::new("altId", "altType");
let out = ghl.v3().products().list_inventory(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.get_products_inventory",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `POST /products/inventory`

**Update Inventory**

The Update Inventory API allows the user to bulk update the inventory for multiple items. Use this endpoint to update the available quantity and out-of-stock purchase settings for multiple items in the inventory.

Operation id: `v3:products.post_products_inventory` · `Version: v3` · Scopes: `products/prices.write`

*Request body*: [`UpdateInventoryDto`](#updateinventorydto)

*Response*: [`UpdateInventoryResponseDto`](#updateinventoryresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().products().update_inventory(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.post_products_inventory",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /products/reviews`

**Fetch Product Reviews**

API to fetch the Product Reviews

Operation id: `v3:products.get_products_reviews` · `Version: v3` · Scopes: `products.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |
| `sortField` | enum: `createdAt`, `rating` | no | The field upon which the sort should be applied |
| `sortOrder` | enum: `asc`, `desc` | no | The order of sort which should be applied for the sortField |
| `rating` | number | no | Key to filter the ratings |
| `startDate` | string | no | The start date for filtering reviews |
| `endDate` | string | no | The end date for filtering reviews |
| `productId` | string | no | Comma-separated list of product IDs |
| `storeId` | string | no | Comma-separated list of store IDs |

*Response*: [`ListProductReviewsResponseDto`](#listproductreviewsresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::products::FetchProductReviewsParams;

let params = FetchProductReviewsParams::new("altId", "altType");
let out = ghl.v3().products().fetch_product_reviews(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.get_products_reviews",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `POST /products/reviews/bulk-update`

**Update Product Reviews**

Update one or multiple product reviews: status, reply, etc.

Operation id: `v3:products.post_products_reviews_bulk_update` · `Version: v3` · Scopes: `products.write`

*Request body*: [`UpdateProductReviewsDto`](#updateproductreviewsdto)

*Response*: [`UpdateProductReviewsResponseDto`](#updateproductreviewsresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().products().update_product_reviews(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.post_products_reviews_bulk_update",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /products/reviews/count`

**Fetch Review Count as per status**

API to fetch the Review Count as per status

Operation id: `v3:products.get_products_reviews_count` · `Version: v3` · Scopes: `products.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `rating` | number | no | Key to filter the ratings |
| `startDate` | string | no | The start date for filtering reviews |
| `endDate` | string | no | The end date for filtering reviews |
| `productId` | string | no | Comma-separated list of product IDs |
| `storeId` | string | no | Comma-separated list of store IDs |

*Response*: [`CountReviewsByStatusResponseDto`](#countreviewsbystatusresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::products::FetchReviewCountAsPerStatusParams;

let params = FetchReviewCountAsPerStatusParams::new("altId", "altType");
let out = ghl.v3().products().fetch_review_count_as_per_status(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.get_products_reviews_count",
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `DELETE /products/reviews/{reviewId}`

**Delete Product Review**

Delete specific product review

Operation id: `v3:products.delete_products_reviews_by_reviewId` · `Version: v3` · Scopes: `products.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `reviewId` | string | **yes** | Review Id |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `productId` | string | **yes** | Product Id of the product |

*Response*: [`DeleteProductReviewResponseDto`](#deleteproductreviewresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::products::DeleteProductReviewParams;

let params = DeleteProductReviewParams::new("altId", "altType", "productId");
let out = ghl.v3().products().delete_product_review(&reviewId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.delete_products_reviews_by_reviewId",
    "path_params": {
      "reviewId": "<reviewId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>",
      "productId": "<productId>"
    }
  }
}
```

</details>

#### `PUT /products/reviews/{reviewId}`

**Update Product Reviews**

Update status, reply, etc of a particular review

Operation id: `v3:products.put_products_reviews_by_reviewId` · `Version: v3` · Scopes: `products.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `reviewId` | string | **yes** | Review Id |

*Request body*: [`UpdateProductReviewDto`](#updateproductreviewdto)

*Response*: [`UpdateProductReviewsResponseDto`](#updateproductreviewsresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().products().update_product_reviews_op(&reviewId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.put_products_reviews_by_reviewId",
    "path_params": {
      "reviewId": "<reviewId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /products/store/{storeId}`

**Action to include/exclude the product in store**

API to update the status of products in a particular store

Operation id: `v3:products.post_products_store_by_storeId` · `Version: v3` · Scopes: `products.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `storeId` | string | **yes** | Products related to the store |

*Request body*: [`UpdateProductStoreDto`](#updateproductstoredto)

*Response*: [`UpdateProductStoreResponseDto`](#updateproductstoreresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().products().action_to_include_exclude_the_product_in_store(&storeId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.post_products_store_by_storeId",
    "path_params": {
      "storeId": "<storeId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /products/store/{storeId}/priority`

**Update product display priorities in store**

API to set the display priority of products in a store

Operation id: `v3:products.post_products_store_by_storeId_priority` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `storeId` | string | **yes** | Products related to the store |

*Request body*: [`UpdateDisplayPriorityBodyDto`](#updatedisplayprioritybodydto)

*Rust*:

```rust,ignore
let out = ghl.v3().products().update_product_display_priorities_in_store(&storeId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.post_products_store_by_storeId_priority",
    "path_params": {
      "storeId": "<storeId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /products/store/{storeId}/stats`

**Fetch Product Store Stats**

API to fetch the total number of products, included in the store, and excluded from the store and other stats

Operation id: `v3:products.get_products_store_by_storeId_stats` · `Version: v3` · Scopes: `products.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `storeId` | string | **yes** | Products related to the store |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `altId` | string | **yes** | Location Id or Agency Id |
| `altType` | enum: `location` | **yes** | — |
| `search` | string | no | The name of the product for searching. |
| `collectionIds` | string | no | Filter by product collection Ids. Supports comma separated values |

*Response*: [`GetProductStatsResponseDto`](#getproductstatsresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::products::FetchProductStoreStatsParams;

let params = FetchProductStoreStatsParams::new("altId", "altType");
let out = ghl.v3().products().fetch_product_store_stats(&storeId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.get_products_store_by_storeId_stats",
    "path_params": {
      "storeId": "<storeId>"
    },
    "query": {
      "altId": "<altId>",
      "altType": "<altType>"
    }
  }
}
```

</details>

#### `DELETE /products/{productId}`

**Delete Product by ID**

The "Delete Product by ID" API allows deleting a specific product using its unique identifier. Use this endpoint to remove a product from the system.

Operation id: `v3:products.delete_products_by_productId` · `Version: v3` · Scopes: `products.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `productId` | string | **yes** | ID or the slug of the product that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | location Id |
| `sendWishlistStatus` | boolean | no | Parameter which will decide whether to show the wishlisting status of products |

*Response*: [`DeleteProductResponseDto`](#deleteproductresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::products::DeleteProductByIdParams;

let params = DeleteProductByIdParams::new("locationId");
let out = ghl.v3().products().delete_product_by_id(&productId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.delete_products_by_productId",
    "path_params": {
      "productId": "<productId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /products/{productId}`

**Get Product by ID**

The "Get Product by ID" API allows to retrieve information for a specific product using its unique identifier. Use this endpoint to fetch details for a single product based on the provided product ID.

Operation id: `v3:products.get_products_by_productId` · `Version: v3` · Scopes: `products.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `productId` | string | **yes** | ID or the slug of the product that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | location Id |
| `sendWishlistStatus` | boolean | no | Parameter which will decide whether to show the wishlisting status of products |

*Response*: [`GetProductResponseDto`](#getproductresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::products::GetProductByIdParams;

let params = GetProductByIdParams::new("locationId");
let out = ghl.v3().products().get_product_by_id(&productId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.get_products_by_productId",
    "path_params": {
      "productId": "<productId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /products/{productId}`

**Update Product by ID**

The "Update Product by ID" API allows modifying information for a specific product using its unique identifier. Use this endpoint to update details for a single product based on the provided product ID.

Operation id: `v3:products.put_products_by_productId` · `Version: v3` · Scopes: `products.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `productId` | string | **yes** | ID or the slug of the product that needs to be returned |

*Request body*: [`UpdateProductDto`](#updateproductdto)

*Response*: [`UpdateProductResponseDto`](#updateproductresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().products().update_product_by_id(&productId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.put_products_by_productId",
    "path_params": {
      "productId": "<productId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /products/{productId}/price`

**List Prices for a Product**

The "List Prices for a Product" API allows retrieving a paginated list of prices associated with a specific product. Customize your results by filtering prices or paginate through the list using the provided query parameters.

Operation id: `v3:products.get_products_by_productId_price` · `Version: v3` · Scopes: `products/prices.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `productId` | string | **yes** | ID of the product that needs to be used |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | number | no | The maximum number of items to be included in a single page of results |
| `offset` | number | no | The starting index of the page, indicating the position from which the results should be retrieved. |
| `locationId` | string | **yes** | The unique identifier for the location. |
| `ids` | string | no | To filter the response only with the given price ids, Please provide with comma separated |

*Response*: [`ListPricesResponseDto`](#listpricesresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::products::ListPricesForAProductParams;

let params = ListPricesForAProductParams::new("locationId");
let out = ghl.v3().products().list_prices_for_a_product(&productId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.get_products_by_productId_price",
    "path_params": {
      "productId": "<productId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /products/{productId}/price`

**Create Price for a Product**

The "Create Price for a Product" API allows adding a new price associated with a specific product to the system. Use this endpoint to create a price with the specified details for a particular product. Ensure that the required information is provided in the request payload.

Operation id: `v3:products.post_products_by_productId_price` · `Version: v3` · Scopes: `products/prices.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `productId` | string | **yes** | ID of the product that needs to be used |

*Request body*: [`CreatePriceDto`](#createpricedto)

*Response*: [`CreatePriceResponseDto`](#createpriceresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().products().create_price_for_a_product(&productId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.post_products_by_productId_price",
    "path_params": {
      "productId": "<productId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /products/{productId}/price/{priceId}`

**Delete Price by ID for a Product**

The "Delete Price by ID for a Product" API allows deleting a specific price associated with a particular product using its unique identifier. Use this endpoint to remove a price from the system.

Operation id: `v3:products.delete_products_by_productId_price_by_priceId` · `Version: v3` · Scopes: `products/prices.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `productId` | string | **yes** | ID of the product that needs to be used |
| `priceId` | string | **yes** | ID of the price that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | location Id |

*Response*: [`DeletePriceResponseDto`](#deletepriceresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::products::DeletePriceByIdForAProductParams;

let params = DeletePriceByIdForAProductParams::new("locationId");
let out = ghl.v3().products().delete_price_by_id_for_a_product(&productId, &priceId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.delete_products_by_productId_price_by_priceId",
    "path_params": {
      "productId": "<productId>",
      "priceId": "<priceId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `GET /products/{productId}/price/{priceId}`

**Get Price by ID for a Product**

The "Get Price by ID for a Product" API allows retrieving information for a specific price associated with a particular product using its unique identifier. Use this endpoint to fetch details for a single price based on the provided price ID and product ID.

Operation id: `v3:products.get_products_by_productId_price_by_priceId` · `Version: v3` · Scopes: `products/prices.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `productId` | string | **yes** | ID of the product that needs to be used |
| `priceId` | string | **yes** | ID of the price that needs to be returned |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | location Id |

*Response*: [`GetPriceResponseDto`](#getpriceresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::products::GetPriceByIdForAProductParams;

let params = GetPriceByIdForAProductParams::new("locationId");
let out = ghl.v3().products().get_price_by_id_for_a_product(&productId, &priceId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.get_products_by_productId_price_by_priceId",
    "path_params": {
      "productId": "<productId>",
      "priceId": "<priceId>"
    },
    "query": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /products/{productId}/price/{priceId}`

**Update Price by ID for a Product**

The "Update Price by ID for a Product" API allows modifying information for a specific price associated with a particular product using its unique identifier. Use this endpoint to update details for a single price based on the provided price ID and product ID.

Operation id: `v3:products.put_products_by_productId_price_by_priceId` · `Version: v3` · Scopes: `products/prices.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `productId` | string | **yes** | ID of the product that needs to be used |
| `priceId` | string | **yes** | ID of the price that needs to be returned |

*Request body*: [`UpdatePriceDto`](#updatepricedto)

*Response*: [`UpdatePriceResponseDto`](#updatepriceresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().products().update_price_by_id_for_a_product(&productId, &priceId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:products.put_products_by_productId_price_by_priceId",
    "path_params": {
      "productId": "<productId>",
      "priceId": "<priceId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::products::*` (enable the `products` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/products/).

### `BulkEditPriceDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Price ID |
| `name` | String | no | Price name |
| `amount` | f64 | no | Price amount |
| `currency` | String | no | Price currency |
| `compareAtPrice` | f64 | no | Compare at price |
| `availableQuantity` | f64 | no | Available quantity |
| `trackInventory` | bool | no | Track inventory |
| `allowOutOfStockPurchases` | bool | no | Allow out of stock purchases |
| `sku` | String | no | SKU |
| `trialPeriod` | f64 | no | Trial period in days |
| `totalCycles` | f64 | no | Total billing cycles |
| `setupFee` | f64 | no | Setup fee |
| `shippingOptions` | [`ShippingOptionsDto`](#shippingoptionsdto) | no | Shipping options |
| `recurring` | [`RecurringDto`](#recurringdto) | no | Recurring details |

### `BulkEditProductDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Product ID |
| `name` | String | no | Product name |
| `description` | String | no | Product description |
| `image` | String | no | Product image |
| `availableInStore` | bool | no | Product availability in store |
| `prices` | Vec<BulkEditPriceDto> | no | Array of price variants for the product |
| `collectionIds` | Vec<String> | no | Collection IDs |
| `isLabelEnabled` | bool | no | Enable product label |
| `isTaxesEnabled` | bool | no | Enable taxes |
| `seo` | [`ProductSEODto`](#productseodto) | no | SEO metadata for the product |
| `slug` | String | no | Product URL slug |
| `automaticTaxCategoryId` | String | no | Automatic tax category ID |
| `taxInclusive` | bool | no | Tax inclusive pricing |
| `taxes` | Vec<JSON> | no | Product taxes |
| `medias` | Vec<JSON> | no | Product media |
| `label` | JSON | no | Product label |

### `BulkEditRequestDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `products` | Vec<BulkEditProductDto> | **yes** | Array of products to update. Note: The total count includes all prices within each product. |

### `BulkEditResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | **yes** | Success message |
| `status` | bool | **yes** | Operation status |
| `updatedCount` | f64 | **yes** | Number of products updated |

### `BulkUpdateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `type` | String — `bulk-update-price`, `bulk-update-availability`, `bulk-update-product-collection`, `bulk-delete-products`, `bulk-update-currency` | **yes** | Type of bulk update operation |
| `productIds` | Vec<String> | **yes** | Array of product IDs |
| `filters` | [`BulkUpdateFilters`](#bulkupdatefilters) | no | Filters to apply when selectAll is true |
| `price` | [`PriceUpdateField`](#priceupdatefield) | no | Price update configuration |
| `compareAtPrice` | [`PriceUpdateField`](#priceupdatefield) | no | Compare at price update configuration |
| `availability` | bool | no | New availability status |
| `collectionIds` | Vec<String> | no | Array of collection IDs |
| `currency` | String | no | Currency code |

### `BulkUpdateFilters`

| Field | Type | Required | Description |
|---|---|---|---|
| `collectionIds` | Vec<String> | no | Filter by collection IDs |
| `productType` | String | no | Filter by product type |
| `availableInStore` | bool | no | Filter by availability status |
| `search` | String | no | Filter by search term |

### `BulkUpdateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |

### `CollectionSEODto`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | no | The title which will be displayed as an SEO format |
| `description` | String | no | The description which would be displayed in preview purposes |

### `CollectionSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier for the collection |
| `altId` | String | **yes** | Location Id to which the collection is associated |
| `name` | String | **yes** | Name of the collection |
| `slug` | String | **yes** | Slug of the collection with which navigation is established. Special Characters and spacing is not allowed and should be unique |
| `image` | String | **yes** | The URL of the image that is going to be displayed as the collection Thumbnail |
| `seo` | [`CollectionSEODto`](#collectionseodto) | **yes** | The information which will be displayed in SEO previews |
| `createdAt` | String | **yes** | Date at which the collection was created |

### `CountReviewsByStatusResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<Vec<JSON>> | **yes** | Array of review status counts |

### `CreateCollectionResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | [`CollectionSchema`](#collectionschema) | **yes** | created Collection |

### `CreatePriceDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | The name of the price. |
| `type` | String — `one_time`, `recurring` | **yes** | The type of the price. |
| `currency` | String | **yes** | The currency of the price. |
| `amount` | f64 | **yes** | The amount of the price. ( min: 0 ) |
| `recurring` | [`RecurringDto`](#recurringdto) | no | The recurring details of the price (if type is recurring). |
| `description` | String | no | A brief description of the price. |
| `membershipOffers` | Vec<MembershipOfferDto> | no | An array of membership offers associated with the price. |
| `trialPeriod` | f64 | no | The trial period duration in days (if applicable). |
| `totalCycles` | f64 | no | The total number of billing cycles for the price. ( min: 1 ) |
| `setupFee` | f64 | no | The setup fee for the price. |
| `variantOptionIds` | Vec<String> | no | An array of variant option IDs associated with the price. |
| `compareAtPrice` | f64 | no | The compare at price for the price. |
| `locationId` | String | **yes** | The unique identifier of the location associated with the price. |
| `userId` | String | no | The unique identifier of the user who created the price. |
| `meta` | [`PriceMetaDto`](#pricemetadto) | no | Additional metadata associated with the price. |
| `trackInventory` | bool | no | Need to track inventory stock quantity |
| `availableQuantity` | f64 | no | Available inventory stock quantity |
| `allowOutOfStockPurchases` | bool | no | Continue selling when out of stock |
| `sku` | String | no | The unique identifier of the SKU associated with the price |
| `shippingOptions` | [`ShippingOptionsDto`](#shippingoptionsdto) | no | Shipping options of the Price |
| `isDigitalProduct` | bool | no | Is the product a digital product |
| `digitalDelivery` | Vec<String> | no | Digital delivery options |

### `CreatePriceResponseDto`

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

### `CreateProductCollectionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id |
| `altType` | String — `location` | **yes** | The type of alt. For now it is only LOCATION |
| `collectionId` | String | no | Unique Identifier of the Product Collection, Mongo Id |
| `name` | String | **yes** | Name of the Product Collection |
| `slug` | String | **yes** | Slug of the Product Collection which helps in navigation |
| `image` | String | no | The URL of the image that is going to be displayed as the collection Thumbnail |
| `seo` | [`CollectionSEODto`](#collectionseodto) | no | The metadata information which will be displayed in SEO previews |

### `CreateProductDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | The name of the product. |
| `locationId` | String | **yes** | The unique identifier for the location. |
| `description` | String | no | A brief description of the product. |
| `productType` | String — `DIGITAL`, `PHYSICAL`, `SERVICE`, `PHYSICAL/DIGITAL` | **yes** | — |
| `image` | String | no | The URL for the product image. |
| `statementDescriptor` | String | no | The statement descriptor for the product. |
| `availableInStore` | bool | no | Indicates whether the product is available in-store. |
| `medias` | Vec<ProductMediaDto> | no | An array of medias for the product. |
| `variants` | Vec<ProductVariantDto> | no | An array of variants for the product. |
| `collectionIds` | Vec<String> | no | An array of category Ids for the product |
| `isTaxesEnabled` | bool | no | Are there any taxes attached to the product. If this is true, taxes array cannot be empty. |
| `taxes` | Vec<String> | no | List of ids of Taxes attached to the Product. If taxes are passed, isTaxesEnabled should be true. |
| `automaticTaxCategoryId` | String | no | Tax category ID for Automatic taxes calculation. |
| `isLabelEnabled` | bool | no | Is the product label enabled. If this is true, label object cannot be empty. |
| `label` | [`ProductLabelDto`](#productlabeldto) | no | Details for Product Label |
| `slug` | String | no | The slug using which the product navigation will be handled |
| `seo` | [`ProductSEODto`](#productseodto) | no | SEO data for the product that will be displayed in the preview |
| `taxInclusive` | bool | no | Whether the taxes should be included in the purchase price |

### `CreateProductResponseDto`

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

### `DefaultCollectionResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | [`ProductCategories`](#productcategories) | **yes** | Collection Data |
| `status` | bool | **yes** | Status of the operation |

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

### `DeletePriceResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | returns true if the price is successfully deleted |

### `DeleteProductCollectionResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |

### `DeleteProductResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | returns true if the product is successfully deleted |

### `DeleteProductReviewResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |

### `GetInventoryResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `inventory` | Vec<InventoryItemDto> | **yes** | List of inventory items |
| `total` | JSON | **yes** | Total count of inventory items |

### `GetPriceResponseDto`

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

### `GetProductResponseDto`

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

### `GetProductStatsResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `totalProducts` | f64 | **yes** | Total number of products |
| `includedInStore` | f64 | **yes** | Number of products included in the store |
| `excludedFromStore` | f64 | **yes** | Number of products excluded from the store |

### `InventoryItemDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier for the price |
| `name` | String | **yes** | Name of the price/variant |
| `availableQuantity` | f64 | **yes** | Available quantity in inventory |
| `sku` | String | **yes** | SKU for the product variant |
| `allowOutOfStockPurchases` | bool | **yes** | Whether out of stock purchases are allowed |
| `product` | String | **yes** | Product ID this price belongs to |
| `updatedAt` | String | **yes** | Last update timestamp |
| `image` | String | no | Product image URL |
| `productName` | String | no | Product name |

### `ListCollectionResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<Vec<JSON>> | **yes** | Array of Collections |
| `total` | f64 | **yes** | The total count of the collections present, which is useful to calculate the pagination |

### `ListPricesResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `prices` | Vec<DefaultPriceResponseDto> | **yes** | An array of prices |
| `total` | f64 | **yes** | — |

### `ListProductReviewsResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<Vec<JSON>> | **yes** | Array of Collections |
| `total` | f64 | **yes** | The total count of the collections present, which is useful to calculate the pagination |

### `ListProductsResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `products` | Vec<DefaultProductResponseDto> | **yes** | An array of products |
| `total` | Vec<ListProductsStats> | **yes** | list products status |

### `ListProductsStats`

| Field | Type | Required | Description |
|---|---|---|---|
| `total` | f64 | **yes** | Total number of products |

### `MembershipOfferDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `label` | String | **yes** | Membership offer label |
| `value` | String | **yes** | Membership offer label |
| `_id` | String | **yes** | The unique identifier for the membership offer. |

### `PriceDimensionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `height` | f64 | **yes** | Height of the price |
| `width` | f64 | **yes** | Width of the price |
| `length` | f64 | **yes** | Length of the price |
| `unit` | String — `cm`, `in`, `m` | **yes** | Unit of the price dimensions |

### `PriceMetaDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `source` | String — `stripe`, `woocommerce`, `shopify` | **yes** | The source of the price. |
| `sourceId` | String | no | The id of the source of the price from where it is imported |
| `stripePriceId` | String | **yes** | The Stripe price ID associated with the price. |
| `internalSource` | String — `agency_plan`, `funnel`, `membership`, `communities`, `gokollab`, `calendar` | **yes** | The internal source of the price. |

### `PriceUpdateField`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `INCREASE_BY_AMOUNT`, `REDUCE_BY_AMOUNT`, `SET_NEW_PRICE`, `INCREASE_BY_PERCENTAGE`, `REDUCE_BY_PERCENTAGE` | **yes** | Type of price update |
| `value` | f64 | **yes** | Value to update (amount or percentage based on type) |
| `roundToWhole` | bool | no | Round to nearest whole number |

### `ProductCategories`

_No fields defined in the spec._

### `ProductLabelDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | The content for the product label. |
| `startDate` | String | no | Start date in YYYY-MM-DDTHH:mm:ssZ format |
| `endDate` | String | no | Start date in YYYY-MM-DDTHH:mm:ssZ format |

### `ProductMediaDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | The unique identifier for the media. |
| `title` | String | no | The title of the media file. |
| `url` | String | **yes** | The URL where the media file is stored. |
| `type` | String — `image`, `video` | **yes** | The type of the media file (e.g., image, video will be supporting soon). |
| `isFeatured` | bool | no | Indicates whether the media is featured. |
| `priceIds` | Vec<Vec<JSON>> | no | Mongo ObjectIds of the prices for which the media is assigned |

### `ProductReviewDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `headline` | String | **yes** | Headline of the Review |
| `comment` | String | **yes** | Detailed Review of the product |
| `user` | [`UserDetailsDto`](#userdetailsdto) | **yes** | User who is giving the review/reply |

### `ProductSEODto`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | no | SEO title |
| `description` | String | no | SEO description |

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

### `ShippingOptionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `weight` | [`WeightOptionsDto`](#weightoptionsdto) | no | Weight options of the product |
| `dimensions` | [`PriceDimensionsDto`](#pricedimensionsdto) | no | Dimensions of the product |

### `UpdateDisplayPriorityBodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `products` | Vec<Vec<JSON>> | **yes** | Array of products with their display priorities |

### `UpdateInventoryDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `items` | Vec<UpdateInventoryItemDto> | **yes** | Array of items to update in the inventory. |

### `UpdateInventoryItemDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `priceId` | String | **yes** | The unique identifier for the price, in MongoDB ID format. |
| `availableQuantity` | f64 | no | The available quantity of the item. |
| `allowOutOfStockPurchases` | bool | no | Whether to continue selling the item when out of stock. |

### `UpdateInventoryResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |

### `UpdatePriceDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | The name of the price. |
| `type` | String — `one_time`, `recurring` | **yes** | The type of the price. |
| `currency` | String | **yes** | The currency of the price. |
| `amount` | f64 | **yes** | The amount of the price. ( min: 0 ) |
| `recurring` | [`RecurringDto`](#recurringdto) | no | The recurring details of the price (if type is recurring). |
| `description` | String | no | A brief description of the price. |
| `membershipOffers` | Vec<MembershipOfferDto> | no | An array of membership offers associated with the price. |
| `trialPeriod` | f64 | no | The trial period duration in days (if applicable). |
| `totalCycles` | f64 | no | The total number of billing cycles for the price. ( min: 1 ) |
| `setupFee` | f64 | no | The setup fee for the price. |
| `variantOptionIds` | Vec<String> | no | An array of variant option IDs associated with the price. |
| `compareAtPrice` | f64 | no | The compare at price for the price. |
| `locationId` | String | **yes** | The unique identifier of the location associated with the price. |
| `userId` | String | no | The unique identifier of the user who created the price. |
| `meta` | [`PriceMetaDto`](#pricemetadto) | no | Additional metadata associated with the price. |
| `trackInventory` | bool | no | Need to track inventory stock quantity |
| `availableQuantity` | f64 | no | Available inventory stock quantity |
| `allowOutOfStockPurchases` | bool | no | Continue selling when out of stock |
| `sku` | String | no | The unique identifier of the SKU associated with the price |
| `shippingOptions` | [`ShippingOptionsDto`](#shippingoptionsdto) | no | Shipping options of the Price |
| `isDigitalProduct` | bool | no | Is the product a digital product |
| `digitalDelivery` | Vec<String> | no | Digital delivery options |

### `UpdatePriceResponseDto`

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

### `UpdateProductCollectionResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |

### `UpdateProductCollectionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id |
| `altType` | String — `location` | **yes** | The type of alt. For now it is only LOCATION |
| `name` | String | no | Name of the Product Collection |
| `slug` | String | no | Slug of the Product Collection which helps in navigation |
| `image` | String | no | The URL of the image that is going to be displayed as the collection Thumbnail |
| `seo` | [`CollectionSEODto`](#collectionseodto) | no | The metadata information which will be displayed in SEO previews |

### `UpdateProductDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | The name of the product. |
| `locationId` | String | **yes** | The unique identifier for the location. |
| `description` | String | no | A brief description of the product. |
| `productType` | String — `DIGITAL`, `PHYSICAL`, `SERVICE`, `PHYSICAL/DIGITAL` | **yes** | — |
| `image` | String | no | The URL for the product image. |
| `statementDescriptor` | String | no | The statement descriptor for the product. |
| `availableInStore` | bool | no | Indicates whether the product is available in-store. |
| `medias` | Vec<ProductMediaDto> | no | An array of medias for the product. |
| `variants` | Vec<ProductVariantDto> | no | An array of variants for the product. |
| `collectionIds` | Vec<String> | no | An array of category Ids for the product |
| `isTaxesEnabled` | bool | no | Are there any taxes attached to the product. If this is true, taxes array cannot be empty. |
| `taxes` | Vec<String> | no | List of ids of Taxes attached to the Product. If taxes are passed, isTaxesEnabled should be true. |
| `automaticTaxCategoryId` | String | no | Tax category ID for Automatic taxes calculation. |
| `isLabelEnabled` | bool | no | Is the product label enabled. If this is true, label object cannot be empty. |
| `label` | [`ProductLabelDto`](#productlabeldto) | no | Details for Product Label |
| `slug` | String | no | The slug using which the product navigation will be handled |
| `seo` | [`ProductSEODto`](#productseodto) | no | SEO data for the product that will be displayed in the preview |
| `taxInclusive` | bool | no | Whether the taxes should be included in the purchase price |
| `prices` | Vec<String> | no | The prices of the product |

### `UpdateProductResponseDto`

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

### `UpdateProductReviewDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `productId` | String | **yes** | Product Id |
| `status` | String | **yes** | Status of the review |
| `reply` | Vec<ProductReviewDto> | no | Reply of the review |
| `rating` | f64 | no | Rating of the product |
| `headline` | String | no | Headline of the Review |
| `detail` | String | no | Detailed Review of the product |

### `UpdateProductReviewObjectDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `reviewId` | String | **yes** | Review Id |
| `productId` | String | **yes** | Product Id |
| `storeId` | String | **yes** | Store Id |

### `UpdateProductReviewsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `reviews` | Vec<UpdateProductReviewObjectDto> | **yes** | Array of Product Reviews |
| `status` | JSON | **yes** | Status of the review |

### `UpdateProductReviewsResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |

### `UpdateProductStoreDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `action` | String — `include`, `exclude` | **yes** | Action to include or exclude the product from the store |
| `productIds` | Vec<String> | **yes** | Array of product IDs |

### `UpdateProductStoreResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |

### `UserDetailsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Name of the customer |
| `email` | String | **yes** | Email of the customer |
| `phone` | String | no | Phone no of the customer |
| `isCustomer` | bool | no | Is the person an admin or customer |

### `WeightOptionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `value` | f64 | **yes** | Actual weight of the product |
| `unit` | String — `kg`, `lb`, `g`, `oz` | **yes** | Weight unit of the product |

## Data models — API v3

In Rust: `ghl_models::v3::products::*` (enable the `products` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/products/).

### `BulkEditPriceDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Price ID |
| `name` | String | no | Price name |
| `amount` | f64 | no | Price amount |
| `currency` | String | no | Price currency |
| `compareAtPrice` | f64 | no | Compare at price |
| `availableQuantity` | f64 | no | Available quantity |
| `trackInventory` | bool | no | Track inventory |
| `allowOutOfStockPurchases` | bool | no | Allow out of stock purchases |
| `sku` | String | no | SKU |
| `trialPeriod` | f64 | no | Trial period in days |
| `totalCycles` | f64 | no | Total billing cycles |
| `setupFee` | f64 | no | Setup fee |
| `shippingOptions` | [`ShippingOptionsDto`](#shippingoptionsdto) | no | Shipping options |
| `recurring` | [`RecurringDto`](#recurringdto) | no | Recurring details |

### `BulkEditProductDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Product ID |
| `name` | String | no | Product name |
| `description` | String | no | Product description |
| `image` | String | no | Product image |
| `availableInStore` | bool | no | Product availability in store |
| `prices` | Vec<BulkEditPriceDto> | no | Array of price variants for the product |
| `collectionIds` | Vec<String> | no | Collection IDs |
| `isLabelEnabled` | bool | no | Enable product label |
| `isTaxesEnabled` | bool | no | Enable taxes |
| `seo` | [`ProductSEODto`](#productseodto) | no | SEO metadata for the product |
| `slug` | String | no | Product URL slug |
| `automaticTaxCategoryId` | String | no | Automatic tax category ID |
| `taxInclusive` | bool | no | Tax inclusive pricing |
| `taxes` | Vec<JSON> | no | Product taxes |
| `medias` | Vec<JSON> | no | Product media |
| `label` | JSON | no | Product label |

### `BulkEditRequestDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `products` | Vec<BulkEditProductDto> | **yes** | Array of products to update. Note: The total count includes all prices within each product. |

### `BulkEditResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `message` | String | **yes** | Success message |
| `status` | bool | **yes** | Operation status |
| `updatedCount` | f64 | **yes** | Number of products updated |

### `BulkUpdateDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `type` | String — `bulk-update-price`, `bulk-update-availability`, `bulk-update-product-collection`, `bulk-delete-products`, `bulk-update-currency` | **yes** | Type of bulk update operation |
| `productIds` | Vec<String> | **yes** | Array of product IDs |
| `filters` | [`BulkUpdateFilters`](#bulkupdatefilters) | no | Filters to apply when selectAll is true |
| `price` | [`PriceUpdateField`](#priceupdatefield) | no | Price update configuration |
| `compareAtPrice` | [`PriceUpdateField`](#priceupdatefield) | no | Compare at price update configuration |
| `availability` | bool | no | New availability status |
| `collectionIds` | Vec<String> | no | Array of collection IDs |
| `currency` | String | no | Currency code |

### `BulkUpdateFilters`

| Field | Type | Required | Description |
|---|---|---|---|
| `collectionIds` | Vec<String> | no | Filter by collection IDs |
| `productType` | String | no | Filter by product type |
| `availableInStore` | bool | no | Filter by availability status |
| `search` | String | no | Filter by search term |

### `BulkUpdateResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |

### `CollectionSEODto`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | no | The title which will be displayed as an SEO format |
| `description` | String | no | The description which would be displayed in preview purposes |

### `CollectionSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier for the collection |
| `altId` | String | **yes** | Location Id to which the collection is associated |
| `name` | String | **yes** | Name of the collection |
| `slug` | String | **yes** | Slug of the collection with which navigation is established. Special Characters and spacing is not allowed and should be unique |
| `image` | String | **yes** | The URL of the image that is going to be displayed as the collection Thumbnail |
| `seo` | [`CollectionSEODto`](#collectionseodto) | **yes** | The information which will be displayed in SEO previews |
| `createdAt` | String | **yes** | Date at which the collection was created |

### `CountReviewsByStatusResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<Vec<JSON>> | **yes** | Array of review status counts |

### `CreateCollectionResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | [`CollectionSchema`](#collectionschema) | **yes** | created Collection |

### `CreatePriceDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | The name of the price. |
| `type` | String — `one_time`, `recurring` | **yes** | The type of the price. |
| `currency` | String | **yes** | The currency of the price. |
| `amount` | f64 | **yes** | The amount of the price. ( min: 0 ) |
| `recurring` | [`RecurringDto`](#recurringdto) | no | The recurring details of the price (if type is recurring). |
| `description` | String | no | A brief description of the price. |
| `membershipOffers` | Vec<MembershipOfferDto> | no | An array of membership offers associated with the price. |
| `trialPeriod` | f64 | no | The trial period duration in days (if applicable). |
| `totalCycles` | f64 | no | The total number of billing cycles for the price. ( min: 1 ) |
| `setupFee` | f64 | no | The setup fee for the price. |
| `variantOptionIds` | Vec<String> | no | An array of variant option IDs associated with the price. |
| `compareAtPrice` | f64 | no | The compare at price for the price. |
| `locationId` | String | **yes** | The unique identifier of the location associated with the price. |
| `userId` | String | no | The unique identifier of the user who created the price. |
| `meta` | [`PriceMetaDto`](#pricemetadto) | no | Additional metadata associated with the price. |
| `trackInventory` | bool | no | Need to track inventory stock quantity |
| `availableQuantity` | f64 | no | Available inventory stock quantity |
| `allowOutOfStockPurchases` | bool | no | Continue selling when out of stock |
| `sku` | String | no | The unique identifier of the SKU associated with the price |
| `shippingOptions` | [`ShippingOptionsDto`](#shippingoptionsdto) | no | Shipping options of the Price |
| `isDigitalProduct` | bool | no | Is the product a digital product |
| `digitalDelivery` | Vec<String> | no | Digital delivery options |

### `CreatePriceResponseDto`

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

### `CreateProductCollectionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id |
| `altType` | String — `location` | **yes** | The type of alt. For now it is only LOCATION |
| `collectionId` | String | no | Unique Identifier of the Product Collection, Mongo Id |
| `name` | String | **yes** | Name of the Product Collection |
| `slug` | String | **yes** | Slug of the Product Collection which helps in navigation |
| `image` | String | no | The URL of the image that is going to be displayed as the collection Thumbnail |
| `seo` | [`CollectionSEODto`](#collectionseodto) | no | The metadata information which will be displayed in SEO previews |

### `CreateProductDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | The name of the product. |
| `locationId` | String | **yes** | The unique identifier for the location. |
| `description` | String | no | A brief description of the product. |
| `productType` | String — `DIGITAL`, `PHYSICAL`, `SERVICE`, `PHYSICAL/DIGITAL` | **yes** | — |
| `image` | String | no | The URL for the product image. |
| `statementDescriptor` | String | no | The statement descriptor for the product. |
| `availableInStore` | bool | no | Indicates whether the product is available in-store. |
| `medias` | Vec<ProductMediaDto> | no | An array of medias for the product. |
| `variants` | Vec<ProductVariantDto> | no | An array of variants for the product. |
| `collectionIds` | Vec<String> | no | An array of category Ids for the product |
| `isTaxesEnabled` | bool | no | Are there any taxes attached to the product. If this is true, taxes array cannot be empty. |
| `taxes` | Vec<String> | no | List of ids of Taxes attached to the Product. If taxes are passed, isTaxesEnabled should be true. |
| `automaticTaxCategoryId` | String | no | Tax category ID for Automatic taxes calculation. |
| `isLabelEnabled` | bool | no | Is the product label enabled. If this is true, label object cannot be empty. |
| `label` | [`ProductLabelDto`](#productlabeldto) | no | Details for Product Label |
| `slug` | String | no | The slug using which the product navigation will be handled |
| `seo` | [`ProductSEODto`](#productseodto) | no | SEO data for the product that will be displayed in the preview |
| `taxInclusive` | bool | no | Whether the taxes should be included in the purchase price |

### `CreateProductResponseDto`

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

### `DefaultCollectionResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | [`ProductCategories`](#productcategories) | **yes** | Collection Data |
| `status` | bool | **yes** | Status of the operation |

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

### `DeletePriceResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | returns true if the price is successfully deleted |

### `DeleteProductCollectionResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |

### `DeleteProductResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | returns true if the product is successfully deleted |

### `DeleteProductReviewResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |

### `GetInventoryResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `inventory` | Vec<InventoryItemDto> | **yes** | List of inventory items |
| `total` | JSON | **yes** | Total count of inventory items |

### `GetPriceResponseDto`

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

### `GetProductResponseDto`

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

### `GetProductStatsResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `totalProducts` | f64 | **yes** | Total number of products |
| `includedInStore` | f64 | **yes** | Number of products included in the store |
| `excludedFromStore` | f64 | **yes** | Number of products excluded from the store |

### `InventoryItemDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | The unique identifier for the price |
| `name` | String | **yes** | Name of the price/variant |
| `availableQuantity` | f64 | **yes** | Available quantity in inventory |
| `sku` | String | **yes** | SKU for the product variant |
| `allowOutOfStockPurchases` | bool | **yes** | Whether out of stock purchases are allowed |
| `product` | String | **yes** | Product ID this price belongs to |
| `updatedAt` | String | **yes** | Last update timestamp |
| `image` | String | no | Product image URL |
| `productName` | String | no | Product name |

### `ListCollectionResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<Vec<JSON>> | **yes** | Array of Collections |
| `total` | f64 | **yes** | The total count of the collections present, which is useful to calculate the pagination |

### `ListPricesResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `prices` | Vec<DefaultPriceResponseDto> | **yes** | An array of prices |
| `total` | f64 | **yes** | — |

### `ListProductReviewsResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<Vec<JSON>> | **yes** | Array of Collections |
| `total` | f64 | **yes** | The total count of the collections present, which is useful to calculate the pagination |

### `ListProductsResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `products` | Vec<DefaultProductResponseDto> | **yes** | An array of products |
| `total` | Vec<ListProductsStats> | **yes** | list products status |

### `ListProductsStats`

| Field | Type | Required | Description |
|---|---|---|---|
| `total` | f64 | **yes** | Total number of products |

### `MembershipOfferDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `label` | String | **yes** | Membership offer label |
| `value` | String | **yes** | Membership offer label |
| `_id` | String | **yes** | The unique identifier for the membership offer. |

### `PriceDimensionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `height` | f64 | **yes** | Height of the price |
| `width` | f64 | **yes** | Width of the price |
| `length` | f64 | **yes** | Length of the price |
| `unit` | String — `cm`, `in`, `m` | **yes** | Unit of the price dimensions |

### `PriceMetaDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `source` | String — `stripe`, `woocommerce`, `shopify` | **yes** | The source of the price. |
| `sourceId` | String | no | The id of the source of the price from where it is imported |
| `stripePriceId` | String | **yes** | The Stripe price ID associated with the price. |
| `internalSource` | String — `agency_plan`, `funnel`, `membership`, `communities`, `gokollab`, `calendar` | **yes** | The internal source of the price. |

### `PriceUpdateField`

| Field | Type | Required | Description |
|---|---|---|---|
| `type` | String — `INCREASE_BY_AMOUNT`, `REDUCE_BY_AMOUNT`, `SET_NEW_PRICE`, `INCREASE_BY_PERCENTAGE`, `REDUCE_BY_PERCENTAGE` | **yes** | Type of price update |
| `value` | f64 | **yes** | Value to update (amount or percentage based on type) |
| `roundToWhole` | bool | no | Round to nearest whole number |

### `ProductCategories`

_No fields defined in the spec._

### `ProductLabelDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | The content for the product label. |
| `startDate` | String | no | Start date in YYYY-MM-DDTHH:mm:ssZ format |
| `endDate` | String | no | Start date in YYYY-MM-DDTHH:mm:ssZ format |

### `ProductMediaDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | The unique identifier for the media. |
| `title` | String | no | The title of the media file. |
| `url` | String | **yes** | The URL where the media file is stored. |
| `type` | String — `image`, `video` | **yes** | The type of the media file (e.g., image, video will be supporting soon). |
| `isFeatured` | bool | no | Indicates whether the media is featured. |
| `priceIds` | Vec<Vec<JSON>> | no | Mongo ObjectIds of the prices for which the media is assigned |

### `ProductReviewDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `headline` | String | **yes** | Headline of the Review |
| `comment` | String | **yes** | Detailed Review of the product |
| `user` | [`UserDetailsDto`](#userdetailsdto) | **yes** | User who is giving the review/reply |

### `ProductSEODto`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | no | SEO title |
| `description` | String | no | SEO description |

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

### `ShippingOptionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `weight` | [`WeightOptionsDto`](#weightoptionsdto) | no | Weight options of the product |
| `dimensions` | [`PriceDimensionsDto`](#pricedimensionsdto) | no | Dimensions of the product |

### `UpdateDisplayPriorityBodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `products` | Vec<Vec<JSON>> | **yes** | Array of products with their display priorities |

### `UpdateInventoryDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `items` | Vec<UpdateInventoryItemDto> | **yes** | Array of items to update in the inventory. |

### `UpdateInventoryItemDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `priceId` | String | **yes** | The unique identifier for the price, in MongoDB ID format. |
| `availableQuantity` | f64 | no | The available quantity of the item. |
| `allowOutOfStockPurchases` | bool | no | Whether to continue selling the item when out of stock. |

### `UpdateInventoryResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |

### `UpdatePriceDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | The name of the price. |
| `type` | String — `one_time`, `recurring` | **yes** | The type of the price. |
| `currency` | String | **yes** | The currency of the price. |
| `amount` | f64 | **yes** | The amount of the price. ( min: 0 ) |
| `recurring` | [`RecurringDto`](#recurringdto) | no | The recurring details of the price (if type is recurring). |
| `description` | String | no | A brief description of the price. |
| `membershipOffers` | Vec<MembershipOfferDto> | no | An array of membership offers associated with the price. |
| `trialPeriod` | f64 | no | The trial period duration in days (if applicable). |
| `totalCycles` | f64 | no | The total number of billing cycles for the price. ( min: 1 ) |
| `setupFee` | f64 | no | The setup fee for the price. |
| `variantOptionIds` | Vec<String> | no | An array of variant option IDs associated with the price. |
| `compareAtPrice` | f64 | no | The compare at price for the price. |
| `locationId` | String | **yes** | The unique identifier of the location associated with the price. |
| `userId` | String | no | The unique identifier of the user who created the price. |
| `meta` | [`PriceMetaDto`](#pricemetadto) | no | Additional metadata associated with the price. |
| `trackInventory` | bool | no | Need to track inventory stock quantity |
| `availableQuantity` | f64 | no | Available inventory stock quantity |
| `allowOutOfStockPurchases` | bool | no | Continue selling when out of stock |
| `sku` | String | no | The unique identifier of the SKU associated with the price |
| `shippingOptions` | [`ShippingOptionsDto`](#shippingoptionsdto) | no | Shipping options of the Price |
| `isDigitalProduct` | bool | no | Is the product a digital product |
| `digitalDelivery` | Vec<String> | no | Digital delivery options |

### `UpdatePriceResponseDto`

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

### `UpdateProductCollectionResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |

### `UpdateProductCollectionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id |
| `altType` | String — `location` | **yes** | The type of alt. For now it is only LOCATION |
| `name` | String | no | Name of the Product Collection |
| `slug` | String | no | Slug of the Product Collection which helps in navigation |
| `image` | String | no | The URL of the image that is going to be displayed as the collection Thumbnail |
| `seo` | [`CollectionSEODto`](#collectionseodto) | no | The metadata information which will be displayed in SEO previews |

### `UpdateProductDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | The name of the product. |
| `locationId` | String | **yes** | The unique identifier for the location. |
| `description` | String | no | A brief description of the product. |
| `productType` | String — `DIGITAL`, `PHYSICAL`, `SERVICE`, `PHYSICAL/DIGITAL` | **yes** | — |
| `image` | String | no | The URL for the product image. |
| `statementDescriptor` | String | no | The statement descriptor for the product. |
| `availableInStore` | bool | no | Indicates whether the product is available in-store. |
| `medias` | Vec<ProductMediaDto> | no | An array of medias for the product. |
| `variants` | Vec<ProductVariantDto> | no | An array of variants for the product. |
| `collectionIds` | Vec<String> | no | An array of category Ids for the product |
| `isTaxesEnabled` | bool | no | Are there any taxes attached to the product. If this is true, taxes array cannot be empty. |
| `taxes` | Vec<String> | no | List of ids of Taxes attached to the Product. If taxes are passed, isTaxesEnabled should be true. |
| `automaticTaxCategoryId` | String | no | Tax category ID for Automatic taxes calculation. |
| `isLabelEnabled` | bool | no | Is the product label enabled. If this is true, label object cannot be empty. |
| `label` | [`ProductLabelDto`](#productlabeldto) | no | Details for Product Label |
| `slug` | String | no | The slug using which the product navigation will be handled |
| `seo` | [`ProductSEODto`](#productseodto) | no | SEO data for the product that will be displayed in the preview |
| `taxInclusive` | bool | no | Whether the taxes should be included in the purchase price |
| `prices` | Vec<String> | no | The prices of the product |

### `UpdateProductResponseDto`

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

### `UpdateProductReviewDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `productId` | String | **yes** | Product Id |
| `status` | String | **yes** | Status of the review |
| `reply` | Vec<ProductReviewDto> | no | Reply of the review |
| `rating` | f64 | no | Rating of the product |
| `headline` | String | no | Headline of the Review |
| `detail` | String | no | Detailed Review of the product |

### `UpdateProductReviewObjectDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `reviewId` | String | **yes** | Review Id |
| `productId` | String | **yes** | Product Id |
| `storeId` | String | **yes** | Store Id |

### `UpdateProductReviewsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `reviews` | Vec<UpdateProductReviewObjectDto> | **yes** | Array of Product Reviews |
| `status` | JSON | **yes** | Status of the review |

### `UpdateProductReviewsResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |

### `UpdateProductStoreDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `altId` | String | **yes** | Location Id or Agency Id |
| `altType` | String — `location` | **yes** | — |
| `action` | String — `include`, `exclude` | **yes** | Action to include or exclude the product from the store |
| `productIds` | Vec<String> | **yes** | Array of product IDs |

### `UpdateProductStoreResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `status` | bool | **yes** | Status of api action |
| `message` | String | no | Success message |

### `UserDetailsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Name of the customer |
| `email` | String | **yes** | Email of the customer |
| `phone` | String | no | Phone no of the customer |
| `isCustomer` | bool | no | Is the person an admin or customer |

### `WeightOptionsDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `value` | f64 | **yes** | Actual weight of the product |
| `unit` | String — `kg`, `lb`, `g`, `oz` | **yes** | Weight unit of the product |

