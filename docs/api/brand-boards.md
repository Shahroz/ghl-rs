# `brand-boards`

**5** operations / **11** models in API v2 · **11** operations / **24** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `brand-boards` cargo feature on `ghl-sdk`, then call any of the 5 generated methods on `ghl.brand_boards()`:

```toml
ghl-sdk = { version = "0.4", features = ["brand-boards"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `POST` | `/brand-boards/` | Create a new brand board | `create_a_new_brand_board()` | `brand-boards.post_brand_boards` |
| `GET` | `/brand-boards/{locationId}` | Get Brand Boards | `get_brand_boards()` | `brand-boards.get_brand_boards_by_locationId` |
| `DELETE` | `/brand-boards/{locationId}/{id}` | Delete a Brand Board | `delete_a_brand_board()` | `brand-boards.delete_brand_boards_by_locationId_by_id` |
| `GET` | `/brand-boards/{locationId}/{id}` | Get Brand Board | `get_brand_board()` | `brand-boards.get_brand_boards_by_locationId_by_id` |
| `PATCH` | `/brand-boards/{locationId}/{id}` | Update a Brand Board | `update_a_brand_board()` | `brand-boards.patch_brand_boards_by_locationId_by_id` |

### Endpoint details — v2

#### `POST /brand-boards/`

**Create a new brand board**

Creates a new brand board with logos, colors, and fonts

Operation id: `brand-boards.post_brand_boards` · `Version: 2021-07-28` · Scopes: `brand-boards/design-kit.write`

*Request body*: [`CreateBrandBoardParam`](#createbrandboardparam)

*Response*: [`GetBrandBoardSuccessDTO`](#getbrandboardsuccessdto)

*Rust*:

```rust,ignore
let out = ghl.brand_boards().create_a_new_brand_board(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "brand-boards.post_brand_boards",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /brand-boards/{locationId}`

**Get Brand Boards**

Retrieves all Brand Boards for a specific location

Operation id: `brand-boards.get_brand_boards_by_locationId` · `Version: 2021-07-28` · Scopes: `brand-boards/design-kit.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | number | no | Maximum number of brand boards to return |
| `offset` | number | no | Number of brand boards to skip for pagination |
| `search` | string | no | Search term to filter brand boards by name |
| `deleted` | boolean | no | Include deleted brand boards in results |

*Response*: [`GetBrandBoardsByLocationSuccessDTO`](#getbrandboardsbylocationsuccessdto)

*Rust*:

```rust,ignore
use ghl_sdk::services::brand_boards::GetBrandBoardsParams;

let params = GetBrandBoardsParams::new();
let out = ghl.brand_boards().get_brand_boards(&locationId, &params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "brand-boards.get_brand_boards_by_locationId",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `DELETE /brand-boards/{locationId}/{id}`

**Delete a Brand Board**

Deletes a Brand Board

Operation id: `brand-boards.delete_brand_boards_by_locationId_by_id` · `Version: 2021-07-28` · Scopes: `brand-boards/design-kit.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID where the brand board exists |
| `id` | string | **yes** | Brand board ID to update, retrieve, or delete |

*Response*: [`GetBrandBoardSuccessDTO`](#getbrandboardsuccessdto)

*Rust*:

```rust,ignore
let out = ghl.brand_boards().delete_a_brand_board(&locationId, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "brand-boards.delete_brand_boards_by_locationId_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /brand-boards/{locationId}/{id}`

**Get Brand Board**

Retrieves a specific Brand Board by its ID

Operation id: `brand-boards.get_brand_boards_by_locationId_by_id` · `Version: 2021-07-28` · Scopes: `brand-boards/design-kit.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID where the brand board exists |
| `id` | string | **yes** | Brand board ID to update, retrieve, or delete |

*Response*: [`GetBrandBoardSuccessDTO`](#getbrandboardsuccessdto)

*Rust*:

```rust,ignore
let out = ghl.brand_boards().get_brand_board(&locationId, &id).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "brand-boards.get_brand_boards_by_locationId_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `PATCH /brand-boards/{locationId}/{id}`

**Update a Brand Board**

Updates an existing Brand Board

Operation id: `brand-boards.patch_brand_boards_by_locationId_by_id` · `Version: 2021-07-28` · Scopes: `brand-boards/design-kit.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID where the brand board exists |
| `id` | string | **yes** | Brand board ID to update, retrieve, or delete |

*Request body*: [`UpdateBrandBoardBody`](#updatebrandboardbody)

*Response*: [`GetBrandBoardSuccessDTO`](#getbrandboardsuccessdto)

*Rust*:

```rust,ignore
let out = ghl.brand_boards().update_a_brand_board(&locationId, &id, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "brand-boards.patch_brand_boards_by_locationId_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
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
| `POST` | `/brand-boards/` | Create a new brand board | `v3:brand-boards.post_brand_boards` |
| `GET` | `/brand-boards/locations/{locationId}/brand-voices` | List Brand Voices | `v3:brand-boards.get_brand_boards_locations_by_locationId_brand_voices` |
| `POST` | `/brand-boards/locations/{locationId}/brand-voices` | Create Brand Voice | `v3:brand-boards.post_brand_boards_locations_by_locationId_brand_voices` |
| `DELETE` | `/brand-boards/locations/{locationId}/brand-voices/{brandVoiceId}` | Delete Brand Voice | `v3:brand-boards.delete_brand_boards_locations_by_locationId_brand_voices_by_brandVoiceId` |
| `GET` | `/brand-boards/locations/{locationId}/brand-voices/{brandVoiceId}` | Get Brand Voice | `v3:brand-boards.get_brand_boards_locations_by_locationId_brand_voices_by_brandVoiceId` |
| `PATCH` | `/brand-boards/locations/{locationId}/brand-voices/{brandVoiceId}` | Update Brand Voice | `v3:brand-boards.patch_brand_boards_locations_by_locationId_brand_voices_by_brandVoiceId` |
| `POST` | `/brand-boards/locations/{locationId}/brand-voices/{brandVoiceId}/default` | Set Default Brand Voice | `v3:brand-boards.post_brand_boards_locations_by_locationId_brand_voices_by_brandVoiceId_default` |
| `GET` | `/brand-boards/{locationId}` | Get Brand Boards | `v3:brand-boards.get_brand_boards_by_locationId` |
| `DELETE` | `/brand-boards/{locationId}/{id}` | Delete a Brand Board | `v3:brand-boards.delete_brand_boards_by_locationId_by_id` |
| `GET` | `/brand-boards/{locationId}/{id}` | Get Brand Board | `v3:brand-boards.get_brand_boards_by_locationId_by_id` |
| `PATCH` | `/brand-boards/{locationId}/{id}` | Update a Brand Board | `v3:brand-boards.patch_brand_boards_by_locationId_by_id` |

### Endpoint details — v3

#### `POST /brand-boards/`

**Create a new brand board**

Creates a new brand board with logos, colors, and fonts

Operation id: `v3:brand-boards.post_brand_boards` · `Version: v3` · Scopes: `brand-boards/design-kit.write`

*Request body*: [`CreateBrandBoardParam`](#createbrandboardparam)

*Response*: [`GetBrandBoardSuccessDTO`](#getbrandboardsuccessdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:brand-boards.post_brand_boards",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /brand-boards/locations/{locationId}/brand-voices`

**List Brand Voices**

Get list of brand voices for a location

Operation id: `v3:brand-boards.get_brand_boards_locations_by_locationId_brand_voices` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | number | no | Number of brand voices to return. Defaults to 10, minimum is 1, maximum is 20 |
| `offset` | number | no | Number of brand voices to skip for pagination. Defaults to 0, minimum is 0 |
| `search` | string | no | Search text for brand voice name |
| `deleted` | boolean | no | Whether to return deleted brand voices. Defaults to false |

*Response*: [`ListBrandVoicesPublicV1ResponseDto`](#listbrandvoicespublicv1responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:brand-boards.get_brand_boards_locations_by_locationId_brand_voices",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `POST /brand-boards/locations/{locationId}/brand-voices`

**Create Brand Voice**

Create a brand voice for a location

Operation id: `v3:brand-boards.post_brand_boards_locations_by_locationId_brand_voices` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |

*Request body*: [`CreateBrandVoicePublicV1BodyDto`](#createbrandvoicepublicv1bodydto)

*Response*: [`CreateBrandVoicePublicV1ResponseDto`](#createbrandvoicepublicv1responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:brand-boards.post_brand_boards_locations_by_locationId_brand_voices",
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

#### `DELETE /brand-boards/locations/{locationId}/brand-voices/{brandVoiceId}`

**Delete Brand Voice**

Delete a brand voice by ID

Operation id: `v3:brand-boards.delete_brand_boards_locations_by_locationId_brand_voices_by_brandVoiceId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `brandVoiceId` | string | **yes** | Brand voice ID |

*Response*: [`DeleteBrandVoicePublicV1ResponseDto`](#deletebrandvoicepublicv1responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:brand-boards.delete_brand_boards_locations_by_locationId_brand_voices_by_brandVoiceId",
    "path_params": {
      "locationId": "<locationId>",
      "brandVoiceId": "<brandVoiceId>"
    }
  }
}
```

</details>

#### `GET /brand-boards/locations/{locationId}/brand-voices/{brandVoiceId}`

**Get Brand Voice**

Get a brand voice by ID

Operation id: `v3:brand-boards.get_brand_boards_locations_by_locationId_brand_voices_by_brandVoiceId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `brandVoiceId` | string | **yes** | Brand voice ID |

*Response*: [`GetBrandVoicePublicV1ResponseDto`](#getbrandvoicepublicv1responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:brand-boards.get_brand_boards_locations_by_locationId_brand_voices_by_brandVoiceId",
    "path_params": {
      "locationId": "<locationId>",
      "brandVoiceId": "<brandVoiceId>"
    }
  }
}
```

</details>

#### `PATCH /brand-boards/locations/{locationId}/brand-voices/{brandVoiceId}`

**Update Brand Voice**

Update a brand voice by ID

Operation id: `v3:brand-boards.patch_brand_boards_locations_by_locationId_brand_voices_by_brandVoiceId` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `brandVoiceId` | string | **yes** | Brand voice ID |

*Request body*: [`UpdateBrandVoicePublicV1BodyDto`](#updatebrandvoicepublicv1bodydto)

*Response*: [`UpdateBrandVoicePublicV1ResponseDto`](#updatebrandvoicepublicv1responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:brand-boards.patch_brand_boards_locations_by_locationId_brand_voices_by_brandVoiceId",
    "path_params": {
      "locationId": "<locationId>",
      "brandVoiceId": "<brandVoiceId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `POST /brand-boards/locations/{locationId}/brand-voices/{brandVoiceId}/default`

**Set Default Brand Voice**

Set a brand voice as the default for a location. The previous default will be unset.

Operation id: `v3:brand-boards.post_brand_boards_locations_by_locationId_brand_voices_by_brandVoiceId_default` · `Version: v3`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID |
| `brandVoiceId` | string | **yes** | Brand voice ID |

*Response*: [`SetDefaultBrandVoicePublicV1ResponseDto`](#setdefaultbrandvoicepublicv1responsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:brand-boards.post_brand_boards_locations_by_locationId_brand_voices_by_brandVoiceId_default",
    "path_params": {
      "locationId": "<locationId>",
      "brandVoiceId": "<brandVoiceId>"
    }
  }
}
```

</details>

#### `GET /brand-boards/{locationId}`

**Get Brand Boards**

Retrieves all Brand Boards for a specific location

Operation id: `v3:brand-boards.get_brand_boards_by_locationId` · `Version: v3` · Scopes: `brand-boards/design-kit.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID where the brand boards exist |

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `limit` | number | no | Maximum number of brand boards to return |
| `offset` | number | no | Number of brand boards to skip for pagination |
| `search` | string | no | Search term to filter brand boards by name |
| `deleted` | boolean | no | Include deleted brand boards in results |

*Response*: [`GetBrandBoardsByLocationSuccessDTO`](#getbrandboardsbylocationsuccessdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:brand-boards.get_brand_boards_by_locationId",
    "path_params": {
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `DELETE /brand-boards/{locationId}/{id}`

**Delete a Brand Board**

Deletes a Brand Board

Operation id: `v3:brand-boards.delete_brand_boards_by_locationId_by_id` · `Version: v3` · Scopes: `brand-boards/design-kit.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID where the brand board exists |
| `id` | string | **yes** | Brand board ID to update, retrieve, or delete |

*Response*: [`GetBrandBoardSuccessDTO`](#getbrandboardsuccessdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:brand-boards.delete_brand_boards_by_locationId_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `GET /brand-boards/{locationId}/{id}`

**Get Brand Board**

Retrieves a specific Brand Board by its ID

Operation id: `v3:brand-boards.get_brand_boards_by_locationId_by_id` · `Version: v3` · Scopes: `brand-boards/design-kit.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID where the brand board exists |
| `id` | string | **yes** | Brand board ID to update, retrieve, or delete |

*Response*: [`GetBrandBoardSuccessDTO`](#getbrandboardsuccessdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:brand-boards.get_brand_boards_by_locationId_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    }
  }
}
```

</details>

#### `PATCH /brand-boards/{locationId}/{id}`

**Update a Brand Board**

Updates an existing Brand Board

Operation id: `v3:brand-boards.patch_brand_boards_by_locationId_by_id` · `Version: v3` · Scopes: `brand-boards/design-kit.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location ID where the brand board exists |
| `id` | string | **yes** | Brand board ID to update, retrieve, or delete |

*Request body*: [`UpdateBrandBoardBody`](#updatebrandboardbody)

*Response*: [`GetBrandBoardSuccessDTO`](#getbrandboardsuccessdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:brand-boards.patch_brand_boards_by_locationId_by_id",
    "path_params": {
      "locationId": "<locationId>",
      "id": "<id>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::brand_boards::*` (enable the `brand-boards` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/brand_boards/).

### `BrandBoardListItemDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Brand board ID |
| `name` | String | **yes** | Brand board name |
| `updatedAt` | String | **yes** | Last update timestamp |
| `default` | bool | no | Whether this is the default brand board for the location |
| `meta` | [`MetaData`](#metadata) | no | Metadata about the brand board |

### `Color`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the color |
| `hexa` | String | **yes** | Color in HEXA format (with alpha) |
| `rgba` | String | **yes** | Color in RGBA format |
| `hex` | String | **yes** | Color in HEX format |
| `rgb` | String | **yes** | Color in RGB format |
| `label` | String | **yes** | Display label for the color |

### `CreateBrandBoardParam`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID where the brand board will be created |
| `name` | String | **yes** | Name of the brand board |
| `logos` | Vec<Logo> | no | Array of logos for the brand board |
| `colors` | Vec<Color> | no | Array of colors for the brand board |
| `fonts` | Vec<Font> | no | Array of fonts for the brand board |
| `default` | bool | no | Set as the default brand board for this location |
| `brandBoardId` | String | no | Source brand board ID to copy from (creates a new brand board based on this template) |
| `parentId` | String | no | Parent folder ID in media library for organizing brand boards |
| `type` | String — `template`, `blank`, `snapshot` | no | Source type indicating how the brand board was created |

### `Font`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the font |
| `font` | String | **yes** | Font family name |
| `fallback` | String | **yes** | Fallback font family |
| `label` | String | **yes** | Display label for the font |

### `GetBrandBoardSuccessDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Brand board ID |
| `locationId` | String | **yes** | Location ID |
| `name` | String | **yes** | Brand board name |
| `logos` | Vec<Logo> | no | Array of logos |
| `colors` | Vec<Color> | no | Array of brand colors |
| `fonts` | Vec<Font> | no | Array of brand fonts |
| `default` | bool | **yes** | Whether this is the default brand board for the location |
| `deleted` | bool | **yes** | Whether the brand board has been soft deleted |
| `parentId` | String | no | Parent folder ID in media library |
| `folderId` | String | no | Media library folder ID for this brand board |
| `originId` | String | no | Original brand board ID if cloned from snapshot |
| `meta` | [`MetaData`](#metadata) | no | Metadata about the brand board |
| `createdAt` | String | no | Creation timestamp |
| `updatedAt` | String | no | Last update timestamp |

### `GetBrandBoardsByLocationSuccessDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `brandBoards` | Vec<BrandBoardListItemDTO> | **yes** | Array of brand boards for the location |
| `totalCount` | f64 | **yes** | Total number of brand boards matching the query |

### `InvalidLocationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | — |
| `message` | String | no | — |

### `Logo`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Unique identifier for the logo |
| `url` | String | **yes** | Public URL of the logo image. Used for uploading to the brand board folder in media library |
| `label` | String | **yes** | Display label for the logo (e.g., Primary, Secondary) |
| `path` | String | **yes** | Storage path of the logo in the media library |

### `MetaData`

| Field | Type | Required | Description |
|---|---|---|---|
| `updatedBy` | String | no | User ID who last updated the brand board |
| `lastAction` | String | no | Last action performed on the brand board |
| `sourceId` | String | no | Source brand board ID if created from a template |
| `sourceType` | String — `template`, `blank`, `snapshot` | no | How the brand board was created |

### `NotFoundDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | — |
| `message` | String | no | — |
| `error` | String | no | — |

### `UpdateBrandBoardBody`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Name of the brand board |
| `logos` | Vec<Logo> | no | Array of logos for the brand board |
| `colors` | Vec<Color> | no | Array of colors for the brand board |
| `fonts` | Vec<Font> | no | Array of fonts for the brand board |
| `default` | bool | no | Set as the default brand board for this location |
| `parentId` | String | no | Parent folder ID in media library (reserved for future use) |

## Data models — API v3

In Rust: `ghl_models::v3::brand_boards::*` (enable the `brand-boards` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/brand_boards/).

### `BrandBoardListItemDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Brand board ID |
| `name` | String | **yes** | Brand board name |
| `updatedAt` | String | **yes** | Last update timestamp |
| `default` | bool | no | Whether this is the default brand board for the location |
| `meta` | [`MetaData`](#metadata) | no | Metadata about the brand board |

### `BrandVoiceAnswersDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `brandName` | String | **yes** | Brand Name |
| `toneOfVoice` | String | **yes** | Tone of Voice |
| `targetAudience` | String | **yes** | Target Audience |
| `customerPainPoints` | String | **yes** | Customer Pain Points |
| `businessType` | String | no | Business Type |
| `companyWebsite` | String | no | Company Website |
| `companyEmail` | String | no | Company Email |
| `companyAddress` | String | no | Company Address |
| `phone` | JSON | no | Phone Information |
| `businessHours` | String | no | Business Hours |
| `brandPromise` | String | no | Brand Promise |
| `brandValues` | String | no | Brand Values |
| `brandPurpose` | String | no | Brand Purpose |
| `competitiveAdvantage` | String | no | Competitive Advantage |
| `risksOfInaction` | String | no | Risks of Inaction |
| `uniqueSellingProposition` | String | no | Unique Selling Proposition |
| `callToAction` | String | no | Call to Action |

### `BrandVoiceAnswersPublicV1Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `brandName` | String | no | Brand Name |
| `toneOfVoice` | String | no | Tone of Voice |
| `targetAudience` | String | no | Target Audience |
| `customerPainPoints` | String | no | Customer Pain Points |
| `businessType` | String | no | Business Type |
| `companyWebsite` | String | no | Company Website |
| `companyEmail` | String | no | Company Email |
| `companyAddress` | String | no | Company Address |
| `phone` | JSON | no | Phone Information |
| `businessHours` | String | no | Business Hours |
| `brandPromise` | String | no | Brand Promise |
| `brandValues` | String | no | Brand Values |
| `brandPurpose` | String | no | Brand Purpose |
| `competitiveAdvantage` | String | no | Competitive Advantage |
| `risksOfInaction` | String | no | Risks of Inaction |
| `uniqueSellingProposition` | String | no | Unique Selling Proposition |
| `callToAction` | String | no | Call to Action |

### `BrandVoicePublicV1Dto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Brand voice ID |
| `name` | String | **yes** | Brand voice name |
| `isDefault` | bool | **yes** | Whether this is the default brand voice |
| `createdAt` | String | **yes** | Creation timestamp |
| `updatedAt` | String | **yes** | Last update timestamp |

### `Color`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Unique identifier for the color |
| `hexa` | String | **yes** | Color in 8-digit hexadecimal notation with alpha channel |
| `rgba` | String | **yes** | Color with red, green, blue, and alpha channel values |
| `hex` | String | **yes** | Color in HEX format |
| `rgb` | String | **yes** | Color in RGB format |
| `label` | String | **yes** | Display label for the color |

### `CreateBrandBoardParam`

| Field | Type | Required | Description |
|---|---|---|---|
| `locationId` | String | **yes** | Location ID where the brand board will be created |
| `name` | String | **yes** | Name of the brand board |
| `logos` | Vec<Logo> | no | Array of logos for the brand board |
| `colors` | Vec<Color> | no | Array of colors for the brand board |
| `fonts` | Vec<Font> | no | Array of fonts for the brand board |
| `default` | bool | no | Set as the default brand board for this location |
| `brandBoardId` | String | no | Source brand board ID to copy from (creates a new brand board based on this template) |
| `parentId` | String | no | Parent folder ID in media library for organizing brand boards |
| `type` | String — `template`, `blank`, `snapshot`, `url` | no | Source type indicating how the brand board was created |
| `url` | String | no | Website URL to extract design kit from (colors, fonts, logos) |

### `CreateBrandVoicePublicV1BodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Name |
| `type` | String — `manual`, `url`, `description` | **yes** | Creation type. "manual" creates with provided custom answers, "url" generates answers from a website, "description" generates answers from a text description |
| `url` | String | no | Website URL to generate brand voice from. Required when type is "url" |
| `description` | String | no | Company description to generate brand voice from. Required when type is "description", optional when type is "url" |
| `answers` | [`BrandVoiceAnswersDto`](#brandvoiceanswersdto) | no | Brand voice answers. Required when type is "manual" |

### `CreateBrandVoicePublicV1ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Brand voice ID |
| `name` | String | **yes** | Brand voice name |
| `isDefault` | bool | **yes** | Whether this is the default brand voice |
| `createdAt` | String | **yes** | Creation timestamp |
| `updatedAt` | String | **yes** | Last update timestamp |
| `locationId` | String | **yes** | Location ID |
| `deleted` | bool | **yes** | Whether the brand voice has been soft deleted |
| `answers` | [`BrandVoiceAnswersPublicV1Dto`](#brandvoiceanswerspublicv1dto) | no | Brand voice answers |
| `traceId` | String | no | Trace ID of request |

### `DeleteBrandVoicePublicV1ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `deleted` | bool | **yes** | Whether the brand voice is deleted |
| `traceId` | String | no | Trace ID of request |

### `Font`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Unique identifier for the font |
| `font` | String | **yes** | Font family name |
| `fallback` | String | **yes** | Fallback font family |
| `label` | String | **yes** | Display label for the font |

### `GetBrandBoardSuccessDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Brand board ID |
| `locationId` | String | **yes** | Location ID |
| `name` | String | **yes** | Brand board name |
| `logos` | Vec<Logo> | no | Array of logos |
| `colors` | Vec<Color> | no | Array of brand colors |
| `fonts` | Vec<Font> | no | Array of brand fonts |
| `default` | bool | **yes** | Whether this is the default brand board for the location |
| `deleted` | bool | **yes** | Whether the brand board has been soft deleted |
| `parentId` | String | no | Parent folder ID in media library |
| `folderId` | String | no | Media library folder ID for this brand board |
| `originId` | String | no | Original brand board ID if cloned from snapshot |
| `meta` | [`MetaData`](#metadata) | no | Metadata about the brand board |
| `missingAssets` | [`MissingAssets`](#missingassets) | no | Assets that used fallbacks/defaults (only returned when creating from URL) |
| `createdAt` | String | no | Creation timestamp |
| `updatedAt` | String | no | Last update timestamp |

### `GetBrandBoardsByLocationSuccessDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `brandBoards` | Vec<BrandBoardListItemDTO> | **yes** | Array of brand boards for the location |
| `totalCount` | f64 | **yes** | Total number of brand boards matching the query |

### `GetBrandVoicePublicV1ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Brand voice ID |
| `name` | String | **yes** | Brand voice name |
| `isDefault` | bool | **yes** | Whether this is the default brand voice |
| `createdAt` | String | **yes** | Creation timestamp |
| `updatedAt` | String | **yes** | Last update timestamp |
| `locationId` | String | **yes** | Location ID |
| `deleted` | bool | **yes** | Whether the brand voice has been soft deleted |
| `answers` | [`BrandVoiceAnswersPublicV1Dto`](#brandvoiceanswerspublicv1dto) | no | Brand voice answers |
| `traceId` | String | no | Trace ID of request |

### `InvalidLocationDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | HTTP status code for invalid location access |
| `message` | String | no | Error message describing the location access failure |

### `ListBrandVoicesPublicV1ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `items` | Vec<BrandVoicePublicV1Dto> | **yes** | List of brand voices |
| `total` | f64 | **yes** | Total count of brand voices |
| `traceId` | String | no | Trace ID of request |

### `Logo`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Unique identifier for the logo |
| `url` | String | **yes** | Public URL of the logo image. Used for uploading to the brand board folder in media library |
| `label` | String | **yes** | Display label for the logo (e.g., Primary, Secondary) |
| `path` | String | **yes** | Storage path of the logo in the media library |

### `MetaData`

| Field | Type | Required | Description |
|---|---|---|---|
| `updatedBy` | String | no | User ID who last updated the brand board |
| `lastAction` | String | no | Last action performed on the brand board |
| `sourceId` | String | no | Source brand board ID if created from a template |
| `sourceType` | String — `template`, `blank`, `snapshot`, `url` | no | How the brand board was created |

### `MissingAssets`

| Field | Type | Required | Description |
|---|---|---|---|
| `logos` | Vec<String> | **yes** | Logo labels that used fallbacks |
| `fonts` | Vec<String> | **yes** | Font labels that used defaults |
| `colors` | Vec<String> | **yes** | Color labels that used defaults |

### `NotFoundDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `statusCode` | f64 | no | HTTP status code for not found |
| `message` | String | no | Error message describing the not found failure |
| `error` | String | no | Error type identifier |

### `SetDefaultBrandVoicePublicV1ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | **yes** | Whether the operation was successful |
| `brandVoiceId` | String | **yes** | Brand voice ID that was set as default |
| `traceId` | String | no | Trace ID of request |

### `UpdateBrandBoardBody`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Name of the brand board |
| `logos` | Vec<Logo> | no | Array of logos for the brand board |
| `colors` | Vec<Color> | no | Array of colors for the brand board |
| `fonts` | Vec<Font> | no | Array of fonts for the brand board |
| `default` | bool | no | Set as the default brand board for this location |
| `parentId` | String | no | Parent folder ID in media library (reserved for future use) |

### `UpdateBrandVoiceAnswersDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `brandName` | String | no | Brand Name |
| `toneOfVoice` | String | no | Tone of Voice |
| `targetAudience` | String | no | Target Audience |
| `customerPainPoints` | String | no | Customer Pain Points |
| `businessType` | String | no | Business Type |
| `companyWebsite` | String | no | Company Website |
| `companyEmail` | String | no | Company Email |
| `companyAddress` | String | no | Company Address |
| `phone` | JSON | no | Phone Information |
| `businessHours` | String | no | Business Hours |
| `brandPromise` | String | no | Brand Promise |
| `brandValues` | String | no | Brand Values |
| `brandPurpose` | String | no | Brand Purpose |
| `competitiveAdvantage` | String | no | Competitive Advantage |
| `risksOfInaction` | String | no | Risks of Inaction |
| `uniqueSellingProposition` | String | no | Unique Selling Proposition |
| `callToAction` | String | no | Call to Action |

### `UpdateBrandVoicePublicV1BodyDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Name |
| `answers` | [`UpdateBrandVoiceAnswersDto`](#updatebrandvoiceanswersdto) | no | Updated answers |

### `UpdateBrandVoicePublicV1ResponseDto`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | **yes** | Brand voice ID |
| `name` | String | **yes** | Brand voice name |
| `isDefault` | bool | **yes** | Whether this is the default brand voice |
| `createdAt` | String | **yes** | Creation timestamp |
| `updatedAt` | String | **yes** | Last update timestamp |
| `locationId` | String | **yes** | Location ID |
| `deleted` | bool | **yes** | Whether the brand voice has been soft deleted |
| `answers` | [`BrandVoiceAnswersPublicV1Dto`](#brandvoiceanswerspublicv1dto) | no | Brand voice answers |
| `traceId` | String | no | Trace ID of request |

