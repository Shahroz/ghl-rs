# `custom-menus`

**5** operations / **9** models in API v2 · **5** operations / **9** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `custom-menus` cargo feature on `ghl-sdk`, then call any of the 10 generated methods on `ghl.custom_menus()` (v2) or `ghl.v3().custom_menus()` (v3):

```toml
ghl-sdk = { version = "0.5", features = ["custom-menus"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/custom-menus/` | Get Custom Menu Links | `get_custom_menu_links()` | `custom-menus.get_custom_menus` |
| `POST` | `/custom-menus/` | Create Custom Menu Link | `create_custom_menu_link()` | `custom-menus.post_custom_menus` |
| `DELETE` | `/custom-menus/{customMenuId}` | Delete Custom Menu Link | `delete_custom_menu_link()` | `custom-menus.delete_custom_menus_by_customMenuId` |
| `GET` | `/custom-menus/{customMenuId}` | Get Custom Menu Link | `get_custom_menu_link()` | `custom-menus.get_custom_menus_by_customMenuId` |
| `PUT` | `/custom-menus/{customMenuId}` | Update Custom Menu Link | `update_custom_menu_link()` | `custom-menus.put_custom_menus_by_customMenuId` |

### Endpoint details — v2

#### `GET /custom-menus/`

**Get Custom Menu Links**

Fetches a collection of custom menus based on specified criteria. This endpoint allows clients to retrieve custom menu configurations, which may include menu items, categories, and associated metadata. The response can be tailored using query parameters for filtering, sorting, and pagination.

Operation id: `custom-menus.get_custom_menus` · `Version: 2021-07-28` · Scopes: `custom-menu-link.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | no | Unique identifier of the location |
| `skip` | number | no | Number of items to skip for pagination |
| `limit` | number | no | Maximum number of items to return |
| `query` | string | no | Search query to filter custom menus by name, supports partial \|\| full names |
| `showOnCompany` | boolean | no | Filter to show only agency-level menu links. When omitted, fetches both agency and sub-account menu links. Ignored if locationId is provided |

*Response*: [`GetCustomMenusResponseDTO`](#getcustommenusresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::custom_menus::GetCustomMenuLinksParams;

let params = GetCustomMenuLinksParams::new();
let out = ghl.custom_menus().get_custom_menu_links(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "custom-menus.get_custom_menus"
  }
}
```

</details>

#### `POST /custom-menus/`

**Create Custom Menu Link**

Creates a new custom menu for a company. Requires authentication and proper permissions. For Icon Usage Details please refer to https://doc.clickup.com/8631005/d/h/87cpx-243696/d60fa70db6b92b2

Operation id: `custom-menus.post_custom_menus` · `Version: 2021-07-28` · Scopes: `custom-menu-link.write`

*Request body*: [`CreateCustomMenuDTO`](#createcustommenudto)

*Response*: [`GetSingleCustomMenusSuccessfulResponseDTO`](#getsinglecustommenussuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.custom_menus().create_custom_menu_link(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "custom-menus.post_custom_menus",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /custom-menus/{customMenuId}`

**Delete Custom Menu Link**

Removes a specific custom menu from the system. This operation requires authentication and proper permissions. The custom menu is identified by its unique ID, and the operation is performed within the context of a specific company.

Operation id: `custom-menus.delete_custom_menus_by_customMenuId` · `Version: 2021-07-28` · Scopes: `custom-menu-link.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `customMenuId` | string | **yes** | ID of the custom menu to delete |

*Response*: [`DeleteCustomMenuSuccessfulResponseDTO`](#deletecustommenusuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.custom_menus().delete_custom_menu_link(&customMenuId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "custom-menus.delete_custom_menus_by_customMenuId",
    "path_params": {
      "customMenuId": "<customMenuId>"
    }
  }
}
```

</details>

#### `GET /custom-menus/{customMenuId}`

**Get Custom Menu Link**

Fetches a single custom menus based on id. This endpoint allows clients to retrieve custom menu configurations, which may include menu items, categories, and associated metadata

Operation id: `custom-menus.get_custom_menus_by_customMenuId` · `Version: 2021-07-28` · Scopes: `custom-menu-link.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `customMenuId` | string | **yes** | Unique identifier of the custom menu |

*Response*: [`GetSingleCustomMenusSuccessfulResponseDTO`](#getsinglecustommenussuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.custom_menus().get_custom_menu_link(&customMenuId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "custom-menus.get_custom_menus_by_customMenuId",
    "path_params": {
      "customMenuId": "<customMenuId>"
    }
  }
}
```

</details>

#### `PUT /custom-menus/{customMenuId}`

**Update Custom Menu Link**

Updates an existing custom menu for a given company. Requires authentication and proper permissions.

Operation id: `custom-menus.put_custom_menus_by_customMenuId` · `Version: 2021-07-28` · Scopes: `custom-menu-link.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `customMenuId` | string | **yes** | ID of the custom menu to update |

*Request body*: [`UpdateCustomMenuDTO`](#updatecustommenudto)

*Response*: [`UpdateCustomMenuLinkResponseDTO`](#updatecustommenulinkresponsedto)

*Rust*:

```rust,ignore
let out = ghl.custom_menus().update_custom_menu_link(&customMenuId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "custom-menus.put_custom_menus_by_customMenuId",
    "path_params": {
      "customMenuId": "<customMenuId>"
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
| `GET` | `/custom-menus/` | Get Custom Menu Links | `get_custom_menu_links()` | `v3:custom-menus.get_custom_menus` |
| `POST` | `/custom-menus/` | Create Custom Menu Link | `create_custom_menu_link()` | `v3:custom-menus.post_custom_menus` |
| `DELETE` | `/custom-menus/{customMenuId}` | Delete Custom Menu Link | `delete_custom_menu_link()` | `v3:custom-menus.delete_custom_menus_by_customMenuId` |
| `GET` | `/custom-menus/{customMenuId}` | Get Custom Menu Link | `get_custom_menu_link()` | `v3:custom-menus.get_custom_menus_by_customMenuId` |
| `PUT` | `/custom-menus/{customMenuId}` | Update Custom Menu Link | `update_custom_menu_link()` | `v3:custom-menus.put_custom_menus_by_customMenuId` |

### Endpoint details — v3

#### `GET /custom-menus/`

**Get Custom Menu Links**

Fetches a collection of custom menus based on specified criteria. This endpoint allows clients to retrieve custom menu configurations, which may include menu items, categories, and associated metadata. The response can be tailored using query parameters for filtering, sorting, and pagination.

Operation id: `v3:custom-menus.get_custom_menus` · `Version: v3` · Scopes: `custom-menu-link.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | no | Unique identifier of the location |
| `skip` | number | no | Number of items to skip for pagination |
| `limit` | number | no | Maximum number of items to return |
| `query` | string | no | Search query to filter custom menus by name, supports partial \|\| full names |
| `showOnCompany` | boolean | no | Filter to show only agency-level menu links. When omitted, fetches both agency and sub-account menu links. Ignored if locationId is provided |

*Response*: [`GetCustomMenusResponseDTO`](#getcustommenusresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::v3::custom_menus::GetCustomMenuLinksParams;

let params = GetCustomMenuLinksParams::new();
let out = ghl.v3().custom_menus().get_custom_menu_links(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:custom-menus.get_custom_menus"
  }
}
```

</details>

#### `POST /custom-menus/`

**Create Custom Menu Link**

Creates a new custom menu for a company. Requires authentication and proper permissions. For Icon Usage Details please refer to https://doc.clickup.com/8631005/d/h/87cpx-243696/d60fa70db6b92b2

Operation id: `v3:custom-menus.post_custom_menus` · `Version: v3` · Scopes: `custom-menu-link.write`

*Request body*: [`CreateCustomMenuDTO`](#createcustommenudto)

*Response*: [`GetSingleCustomMenusSuccessfulResponseDTO`](#getsinglecustommenussuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().custom_menus().create_custom_menu_link(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:custom-menus.post_custom_menus",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `DELETE /custom-menus/{customMenuId}`

**Delete Custom Menu Link**

Removes a specific custom menu from the system. This operation requires authentication and proper permissions. The custom menu is identified by its unique ID, and the operation is performed within the context of a specific company.

Operation id: `v3:custom-menus.delete_custom_menus_by_customMenuId` · `Version: v3` · Scopes: `custom-menu-link.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `customMenuId` | string | **yes** | ID of the custom menu to delete |

*Response*: [`DeleteCustomMenuSuccessfulResponseDTO`](#deletecustommenusuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().custom_menus().delete_custom_menu_link(&customMenuId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:custom-menus.delete_custom_menus_by_customMenuId",
    "path_params": {
      "customMenuId": "<customMenuId>"
    }
  }
}
```

</details>

#### `GET /custom-menus/{customMenuId}`

**Get Custom Menu Link**

Fetches a single custom menus based on id. This endpoint allows clients to retrieve custom menu configurations, which may include menu items, categories, and associated metadata

Operation id: `v3:custom-menus.get_custom_menus_by_customMenuId` · `Version: v3` · Scopes: `custom-menu-link.readonly`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `customMenuId` | string | **yes** | Unique identifier of the custom menu |

*Response*: [`GetSingleCustomMenusSuccessfulResponseDTO`](#getsinglecustommenussuccessfulresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().custom_menus().get_custom_menu_link(&customMenuId).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:custom-menus.get_custom_menus_by_customMenuId",
    "path_params": {
      "customMenuId": "<customMenuId>"
    }
  }
}
```

</details>

#### `PUT /custom-menus/{customMenuId}`

**Update Custom Menu Link**

Updates an existing custom menu for a given company. Requires authentication and proper permissions.

Operation id: `v3:custom-menus.put_custom_menus_by_customMenuId` · `Version: v3` · Scopes: `custom-menu-link.write`

*Path parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `customMenuId` | string | **yes** | ID of the custom menu to update |

*Request body*: [`UpdateCustomMenuDTO`](#updatecustommenudto)

*Response*: [`UpdateCustomMenuLinkResponseDTO`](#updatecustommenulinkresponsedto)

*Rust*:

```rust,ignore
let out = ghl.v3().custom_menus().update_custom_menu_link(&customMenuId, &body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:custom-menus.put_custom_menus_by_customMenuId",
    "path_params": {
      "customMenuId": "<customMenuId>"
    },
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::custom_menus::*` (enable the `custom-menus` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/custom_menus/).

### `CreateCustomMenuDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | Title of the custom menu |
| `url` | String | **yes** | URL of the custom menu |
| `icon` | [`IconSchema`](#iconschema) | **yes** | Icon information for the custom menu |
| `showOnCompany` | bool | **yes** | Whether the menu must be displayed on the agency's level |
| `showOnLocation` | bool | **yes** | Whether the menu must be displayed for sub-accounts level |
| `showToAllLocations` | bool | **yes** | Whether the menu must be displayed to all sub-accounts |
| `openMode` | String — `iframe`, `new_tab`, `current_tab` | **yes** | Mode for opening the menu link |
| `locations` | Vec<String> | **yes** | List of sub-account IDs where the menu should be shown. This list is applicable only when showOnLocation is true and showToAllLocations is false |
| `userRole` | String — `all`, `admin`, `user` | **yes** | Which user-roles should the menu be accessible to? |
| `allowCamera` | bool | no | Whether to allow camera access (only for iframe mode) |
| `allowMicrophone` | bool | no | Whether to allow microphone access (only for iframe mode) |

### `CustomMenuSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Unique identifier for the custom menu |
| `icon` | [`IconSchemaOptional`](#iconschemaoptional) | no | Icon information for the menu item |
| `title` | String | no | Title of the custom menu |
| `url` | String | no | URL of the custom menu |
| `order` | f64 | no | Order of the custom menu |
| `showOnCompany` | bool | no | Filter to show only agency-level menu links. When omitted, fetches both agency and sub-account menu links. Ignored if locationId is provided |
| `showOnLocation` | bool | no | Whether the menu must be displayed for sub-accounts level |
| `showToAllLocations` | bool | no | Whether the menu must be displayed to all sub-accounts |
| `locations` | Vec<String> | no | List of sub-account IDs where the menu should be shown. This list is applicable only when showOnLocation is true and showToAllLocations is false |
| `openMode` | String — `iframe`, `new_tab`, `current_tab` | no | Mode for opening the menu link |
| `userRole` | String — `all`, `admin`, `user` | no | Which user-roles should the menu be accessible to? |
| `allowCamera` | bool | no | Indicates if camera access is allowed for this menu |
| `allowMicrophone` | bool | no | Indicates if microphone access is allowed for this menu |

### `DeleteCustomMenuSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | no | Indicates whether the custom menu was successfully deleted |
| `message` | String | no | A message providing additional information about the deletion operation |
| `deletedMenuId` | String | no | The ID of the deleted custom menu |
| `deletedAt` | String | no | Timestamp of when the deletion was performed |

### `GetCustomMenusResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `customMenus` | Vec<CustomMenuSchema> | no | Array of custom menu links |
| `totalLinks` | f64 | no | Total number of custom menu records |

### `GetSingleCustomMenusSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `customMenu` | [`CustomMenuSchema`](#custommenuschema) | no | Single Custom menu link object |

### `IconSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Name of the icon |
| `fontFamily` | String — `fab`, `fas`, `far` | **yes** | Font family of the icon |

### `IconSchemaOptional`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Name of the icon |
| `fontFamily` | String — `fab`, `fas`, `far` | no | Font family of the icon |

### `UpdateCustomMenuDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | no | Title of the custom menu |
| `url` | String | no | URL of the custom menu |
| `icon` | [`IconSchemaOptional`](#iconschemaoptional) | no | Icon information for the custom menu |
| `showOnCompany` | bool | no | Whether the menu must be displayed on the agency's level |
| `showOnLocation` | bool | no | Whether the menu must be displayed for sub-accounts level |
| `showToAllLocations` | bool | no | Whether the menu must be displayed to all sub-accounts |
| `openMode` | String — `iframe`, `new_tab`, `current_tab` | no | Mode for opening the menu link |
| `locations` | Vec<String> | no | List of sub-account IDs where the menu should be shown. This list is applicable only when showOnLocation is true and showToAllLocations is false |
| `userRole` | String — `all`, `admin`, `user` | no | Which user-roles should the menu be accessible to? |
| `allowCamera` | bool | no | Whether to allow camera access (only for iframe mode) |
| `allowMicrophone` | bool | no | Whether to allow microphone access (only for iframe mode) |

### `UpdateCustomMenuLinkResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | no | Status of update |
| `customMenu` | [`CustomMenuSchema`](#custommenuschema) | no | Updated custom menu link |

## Data models — API v3

In Rust: `ghl_models::v3::custom_menus::*` (enable the `custom-menus` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/custom_menus/).

### `CreateCustomMenuDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | Title of the custom menu |
| `url` | String | **yes** | URL of the custom menu |
| `icon` | [`IconSchema`](#iconschema) | **yes** | Icon information for the custom menu |
| `showOnCompany` | bool | **yes** | Whether the menu must be displayed on the agency's level |
| `showOnLocation` | bool | **yes** | Whether the menu must be displayed for sub-accounts level |
| `showToAllLocations` | bool | **yes** | Whether the menu must be displayed to all sub-accounts |
| `openMode` | String — `iframe`, `new_tab`, `current_tab` | **yes** | Mode for opening the menu link |
| `locations` | Vec<String> | **yes** | List of sub-account IDs where the menu should be shown. This list is applicable only when showOnLocation is true and showToAllLocations is false |
| `userRole` | String — `all`, `admin`, `user` | **yes** | Which user-roles should the menu be accessible to? |
| `allowCamera` | bool | no | Whether to allow camera access (only for iframe mode) |
| `allowMicrophone` | bool | no | Whether to allow microphone access (only for iframe mode) |

### `CustomMenuSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | String | no | Unique identifier for the custom menu |
| `icon` | [`IconSchemaOptional`](#iconschemaoptional) | no | Icon information for the menu item |
| `title` | String | no | Title of the custom menu |
| `url` | String | no | URL of the custom menu |
| `order` | f64 | no | Order of the custom menu |
| `showOnCompany` | bool | no | Filter to show only agency-level menu links. When omitted, fetches both agency and sub-account menu links. Ignored if locationId is provided |
| `showOnLocation` | bool | no | Whether the menu must be displayed for sub-accounts level |
| `showToAllLocations` | bool | no | Whether the menu must be displayed to all sub-accounts |
| `locations` | Vec<String> | no | List of sub-account IDs where the menu should be shown. This list is applicable only when showOnLocation is true and showToAllLocations is false |
| `openMode` | String — `iframe`, `new_tab`, `current_tab` | no | Mode for opening the menu link |
| `userRole` | String — `all`, `admin`, `user` | no | Which user-roles should the menu be accessible to? |
| `allowCamera` | bool | no | Indicates if camera access is allowed for this menu |
| `allowMicrophone` | bool | no | Indicates if microphone access is allowed for this menu |

### `DeleteCustomMenuSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | no | Indicates whether the custom menu was successfully deleted |
| `message` | String | no | A message providing additional information about the deletion operation |
| `deletedMenuId` | String | no | The ID of the deleted custom menu |
| `deletedAt` | String | no | Timestamp of when the deletion was performed |

### `GetCustomMenusResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `customMenus` | Vec<CustomMenuSchema> | no | Array of custom menu links |
| `totalLinks` | f64 | no | Total number of custom menu records |

### `GetSingleCustomMenusSuccessfulResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `customMenu` | [`CustomMenuSchema`](#custommenuschema) | no | Single Custom menu link object |

### `IconSchema`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | **yes** | Name of the icon |
| `fontFamily` | String — `fab`, `fas`, `far` | **yes** | Font family of the icon |

### `IconSchemaOptional`

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | String | no | Name of the icon |
| `fontFamily` | String — `fab`, `fas`, `far` | no | Font family of the icon |

### `UpdateCustomMenuDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | no | Title of the custom menu |
| `url` | String | no | URL of the custom menu |
| `icon` | [`IconSchemaOptional`](#iconschemaoptional) | no | Icon information for the custom menu |
| `showOnCompany` | bool | no | Whether the menu must be displayed on the agency's level |
| `showOnLocation` | bool | no | Whether the menu must be displayed for sub-accounts level |
| `showToAllLocations` | bool | no | Whether the menu must be displayed to all sub-accounts |
| `openMode` | String — `iframe`, `new_tab`, `current_tab` | no | Mode for opening the menu link |
| `locations` | Vec<String> | no | List of sub-account IDs where the menu should be shown. This list is applicable only when showOnLocation is true and showToAllLocations is false |
| `userRole` | String — `all`, `admin`, `user` | no | Which user-roles should the menu be accessible to? |
| `allowCamera` | bool | no | Whether to allow camera access (only for iframe mode) |
| `allowMicrophone` | bool | no | Whether to allow microphone access (only for iframe mode) |

### `UpdateCustomMenuLinkResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `success` | bool | no | Status of update |
| `customMenu` | [`CustomMenuSchema`](#custommenuschema) | no | Updated custom menu link |

