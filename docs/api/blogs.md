# `blogs`

**7** operations / **13** models in API v2 · **7** operations / **13** models in API v3

## How to call it

**Every endpoint has a typed Rust method.** Enable the `blogs` cargo feature on `ghl-sdk`, then call any of the 7 generated methods on `ghl.blogs()`:

```toml
ghl-sdk = { version = "0.4", features = ["blogs"] }
```


## Endpoints — API v2

| Method | Path | Summary | Rust method | Operation id |
|---|---|---|---|---|
| `GET` | `/blogs/authors` | Get all authors | `get_all_authors()` | `blogs.get_blogs_authors` |
| `GET` | `/blogs/categories` | Get all categories | `get_all_categories()` | `blogs.get_blogs_categories` |
| `POST` | `/blogs/posts` | Create Blog Post | `create_blog_post()` | `blogs.post_blogs_posts` |
| `GET` | `/blogs/posts/all` | Get Blog posts by Blog ID | `get_blog_posts_by_blog_id()` | `blogs.get_blogs_posts_all` |
| `GET` | `/blogs/posts/url-slug-exists` | Check url slug | `check_url_slug()` | `blogs.get_blogs_posts_url_slug_exists` |
| `PUT` | `/blogs/posts/{postId}` | Update Blog Post | `update_blog_post()` | `blogs.put_blogs_posts_by_postId` |
| `GET` | `/blogs/site/all` | Get Blogs by Location ID | `get_blogs_by_location_id()` | `blogs.get_blogs_site_all` |

### Endpoint details — v2

#### `GET /blogs/authors`

**Get all authors**

The "Get all authors" Api return the blog authors for a given location ID. Please use "blogs/author.readonly"

Operation id: `blogs.get_blogs_authors` · `Version: 2021-07-28` · Scopes: `blogs/author.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `limit` | number | **yes** | Number of authors to show in the listing |
| `offset` | number | **yes** | Number of authors to skip in listing |

*Response*: [`AuthorsResponseDTO`](#authorsresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::blogs::GetAllAuthorsParams;

let params = GetAllAuthorsParams::new("locationId", "limit", "offset");
let out = ghl.blogs().get_all_authors(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "blogs.get_blogs_authors",
    "query": {
      "locationId": "<locationId>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `GET /blogs/categories`

**Get all categories**

The "Get all categories" Api return the blog categoies for a given location ID. Please use "blogs/category.readonly"

Operation id: `blogs.get_blogs_categories` · `Version: 2021-07-28` · Scopes: `blogs/category.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `limit` | number | **yes** | Number of categories to show in the listing |
| `offset` | number | **yes** | Number of categories to skip in listing |

*Response*: [`CategoriesResponseDTO`](#categoriesresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::blogs::GetAllCategoriesParams;

let params = GetAllCategoriesParams::new("locationId", "limit", "offset");
let out = ghl.blogs().get_all_categories(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "blogs.get_blogs_categories",
    "query": {
      "locationId": "<locationId>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `POST /blogs/posts`

**Create Blog Post**

The "Create Blog Post" API allows you create blog post for any given blog site. Please use blogs/post.write

Operation id: `blogs.post_blogs_posts` · `Version: 2021-07-28` · Scopes: `blogs/post.write`

*Request body*: [`CreateBlogPostParams`](#createblogpostparams)

*Response*: [`BlogPostCreateResponseWrapperDTO`](#blogpostcreateresponsewrapperdto)

*Rust*:

```rust,ignore
let out = ghl.blogs().create_blog_post(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "blogs.post_blogs_posts",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /blogs/posts/all`

**Get Blog posts by Blog ID**

The "Get Blog posts by Blog ID" API allows you get blog posts for any given blog site using blog ID.Please use blogs/posts.readonly

Operation id: `blogs.get_blogs_posts_all` · `Version: 2021-07-28` · Scopes: `blogs/posts.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `blogId` | string | **yes** | — |
| `limit` | number | **yes** | — |
| `offset` | number | **yes** | — |
| `searchTerm` | string | no | search for any post by name |
| `status` | enum: `PUBLISHED`, `SCHEDULED`, `ARCHIVED`, `DRAFT` | no | — |

*Response*: [`BlogPostGetResponseWrapperDTO`](#blogpostgetresponsewrapperdto)

*Rust*:

```rust,ignore
use ghl_sdk::services::blogs::GetBlogPostsByBlogIdParams;

let params = GetBlogPostsByBlogIdParams::new("locationId", "blogId", "limit", "offset");
let out = ghl.blogs().get_blog_posts_by_blog_id(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "blogs.get_blogs_posts_all",
    "query": {
      "locationId": "<locationId>",
      "blogId": "<blogId>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `GET /blogs/posts/url-slug-exists`

**Check url slug**

The "Check url slug" API allows check the blog slug validation which is needed before publishing any blog post. Please use blogs/check-slug.readonly. you can find the POST ID from the post edit url.

Operation id: `blogs.get_blogs_posts_url_slug_exists` · `Version: 2021-07-28` · Scopes: `blogs/check-slug.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `urlSlug` | string | **yes** | — |
| `locationId` | string | **yes** | — |
| `postId` | string | no | — |

*Response*: [`UrlSlugCheckResponseDTO`](#urlslugcheckresponsedto)

*Rust*:

```rust,ignore
use ghl_sdk::services::blogs::CheckUrlSlugParams;

let params = CheckUrlSlugParams::new("urlSlug", "locationId");
let out = ghl.blogs().check_url_slug(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "blogs.get_blogs_posts_url_slug_exists",
    "query": {
      "urlSlug": "<urlSlug>",
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /blogs/posts/{postId}`

**Update Blog Post**

The "Update Blog Post" API allows you update blog post for any given blog site. Please use blogs/post-update.write

Operation id: `blogs.put_blogs_posts_by_postId` · `Version: 2021-07-28` · Scopes: `blogs/post-update.write`

*Request body*: [`UpdateBlogPostParams`](#updateblogpostparams)

*Response*: [`BlogPostUpdateResponseWrapperDTO`](#blogpostupdateresponsewrapperdto)

*Rust*:

```rust,ignore
let out = ghl.blogs().update_blog_post(&body).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "blogs.put_blogs_posts_by_postId",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /blogs/site/all`

**Get Blogs by Location ID**

The "Get Blogs by Location ID" API allows you get blogs using Location ID.Please use blogs/list.readonly

Operation id: `blogs.get_blogs_site_all` · `Version: 2021-07-28` · Scopes: `blogs/list.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `skip` | number | **yes** | — |
| `limit` | number | **yes** | — |
| `searchTerm` | string | no | search for any post by name |

*Response*: [`BlogGetResponseWrapperDTO`](#bloggetresponsewrapperdto)

*Rust*:

```rust,ignore
use ghl_sdk::services::blogs::GetBlogsByLocationIdParams;

let params = GetBlogsByLocationIdParams::new("locationId", "skip", "limit");
let out = ghl.blogs().get_blogs_by_location_id(&params).await?;
```

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "blogs.get_blogs_site_all",
    "query": {
      "locationId": "<locationId>",
      "skip": "<skip>",
      "limit": "<limit>"
    }
  }
}
```

</details>

## Endpoints — API v3

| Method | Path | Summary | Operation id |
|---|---|---|---|
| `GET` | `/blogs/authors` | Get all authors | `v3:blogs.get_blogs_authors` |
| `GET` | `/blogs/categories` | Get all categories | `v3:blogs.get_blogs_categories` |
| `POST` | `/blogs/posts` | Create Blog Post | `v3:blogs.post_blogs_posts` |
| `GET` | `/blogs/posts/all` | Get Blog posts by Blog ID | `v3:blogs.get_blogs_posts_all` |
| `GET` | `/blogs/posts/url-slug-exists` | Check url slug | `v3:blogs.get_blogs_posts_url_slug_exists` |
| `PUT` | `/blogs/posts/{postId}` | Update Blog Post | `v3:blogs.put_blogs_posts_by_postId` |
| `GET` | `/blogs/site/all` | Get Blogs by Location ID | `v3:blogs.get_blogs_site_all` |

### Endpoint details — v3

#### `GET /blogs/authors`

**Get all authors**

The "Get all authors" Api return the blog authors for a given location ID. Please use "blogs/author.readonly"

Operation id: `v3:blogs.get_blogs_authors` · `Version: v3` · Scopes: `blogs/author.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | Location Id |
| `limit` | number | **yes** | Number of authors to show in the listing |
| `offset` | number | **yes** | Number of authors to skip in listing |

*Response*: [`AuthorsResponseDTO`](#authorsresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:blogs.get_blogs_authors",
    "query": {
      "locationId": "<locationId>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `GET /blogs/categories`

**Get all categories**

The "Get all categories" Api return the blog categoies for a given location ID. Please use "blogs/category.readonly"

Operation id: `v3:blogs.get_blogs_categories` · `Version: v3` · Scopes: `blogs/category.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `limit` | number | **yes** | Number of categories to show in the listing |
| `offset` | number | **yes** | Number of categories to skip in listing |

*Response*: [`CategoriesResponseDTO`](#categoriesresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:blogs.get_blogs_categories",
    "query": {
      "locationId": "<locationId>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `POST /blogs/posts`

**Create Blog Post**

The "Create Blog Post" API allows you create blog post for any given blog site. Please use blogs/post.write

Operation id: `v3:blogs.post_blogs_posts` · `Version: v3` · Scopes: `blogs/post.write`

*Request body*: [`CreateBlogPostParams`](#createblogpostparams)

*Response*: [`BlogPostCreateResponseWrapperDTO`](#blogpostcreateresponsewrapperdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:blogs.post_blogs_posts",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /blogs/posts/all`

**Get Blog posts by Blog ID**

The "Get Blog posts by Blog ID" API allows you get blog posts for any given blog site using blog ID.Please use blogs/posts.readonly

Operation id: `v3:blogs.get_blogs_posts_all` · `Version: v3` · Scopes: `blogs/posts.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `blogId` | string | **yes** | — |
| `limit` | number | **yes** | — |
| `offset` | number | **yes** | — |
| `searchTerm` | string | no | search for any post by name |
| `status` | enum: `ALL`, `PUBLISHED`, `SCHEDULED`, `ARCHIVED`, `DRAFT` | no | — |

*Response*: [`BlogPostGetResponseWrapperDTO`](#blogpostgetresponsewrapperdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:blogs.get_blogs_posts_all",
    "query": {
      "locationId": "<locationId>",
      "blogId": "<blogId>",
      "limit": "<limit>",
      "offset": "<offset>"
    }
  }
}
```

</details>

#### `GET /blogs/posts/url-slug-exists`

**Check url slug**

The "Check url slug" API allows check the blog slug validation which is needed before publishing any blog post. Please use blogs/check-slug.readonly. you can find the POST ID from the post edit url.

Operation id: `v3:blogs.get_blogs_posts_url_slug_exists` · `Version: v3` · Scopes: `blogs/check-slug.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `urlSlug` | string | **yes** | — |
| `locationId` | string | **yes** | — |
| `postId` | string | no | — |

*Response*: [`UrlSlugCheckResponseDTO`](#urlslugcheckresponsedto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:blogs.get_blogs_posts_url_slug_exists",
    "query": {
      "urlSlug": "<urlSlug>",
      "locationId": "<locationId>"
    }
  }
}
```

</details>

#### `PUT /blogs/posts/{postId}`

**Update Blog Post**

The "Update Blog Post" API allows you update blog post for any given blog site. Please use blogs/post-update.write

Operation id: `v3:blogs.put_blogs_posts_by_postId` · `Version: v3` · Scopes: `blogs/post-update.write`

*Request body*: [`UpdateBlogPostParams`](#updateblogpostparams)

*Response*: [`BlogPostUpdateResponseWrapperDTO`](#blogpostupdateresponsewrapperdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:blogs.put_blogs_posts_by_postId",
    "body": {
      "<field>": "<value>"
    }
  }
}
```

</details>

#### `GET /blogs/site/all`

**Get Blogs by Location ID**

The "Get Blogs by Location ID" API allows you get blogs using Location ID.Please use blogs/list.readonly

Operation id: `v3:blogs.get_blogs_site_all` · `Version: v3` · Scopes: `blogs/list.readonly`

*Query parameters*

| Name | Type | Required | Description |
|---|---|---|---|
| `locationId` | string | **yes** | — |
| `skip` | number | **yes** | — |
| `limit` | number | **yes** | — |
| `searchTerm` | string | no | search for any post by name |

*Response*: [`BlogGetResponseWrapperDTO`](#bloggetresponsewrapperdto)

<details><summary>MCP call</summary>

```json
{
  "name": "ghl_execute_operation",
  "arguments": {
    "operation_id": "v3:blogs.get_blogs_site_all",
    "query": {
      "locationId": "<locationId>",
      "skip": "<skip>",
      "limit": "<limit>"
    }
  }
}
```

</details>

## Data models — API v2

In Rust: `ghl_models::v2::blogs::*` (enable the `blogs` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v2/blogs/).

### `AuthorResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | — |
| `name` | String | **yes** | — |
| `locationId` | String | **yes** | — |
| `updatedAt` | String | **yes** | — |
| `canonicalLink` | String | **yes** | — |

### `AuthorsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `authors` | Vec<AuthorResponseDTO> | **yes** | Array of authors |

### `BlogGetResponseWrapperDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<BlogResponseDTO> | **yes** | Object containing response data of blog |

### `BlogPostCreateResponseWrapperDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | [`BlogPostResponseDTO`](#blogpostresponsedto) | **yes** | Object containing response data of blog post create. |

### `BlogPostGetResponseWrapperDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `blogs` | Vec<BlogPostResponseDTO> | **yes** | Object containing response data of blog posts |

### `BlogPostResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `categories` | Vec<String> | **yes** | Array of category IDs associated with the blog post |
| `tags` | Vec<String> | no | Array of tags associated with the blog post |
| `archived` | bool | **yes** | Indicates whether the blog post is archived |
| `_id` | String | **yes** | Unique identifier of the blog post |
| `title` | String | **yes** | Title of the blog post |
| `description` | String | **yes** | Description of the blog post |
| `imageUrl` | String | **yes** | URL of the image associated with the blog post |
| `status` | String | **yes** | Publication status of the blog post |
| `imageAltText` | String | **yes** | Alternative text for the blog post image |
| `urlSlug` | String | **yes** | URL slug for the blog post |
| `canonicalLink` | String | no | Canonical link of the blog post |
| `author` | String | no | Identifier of the author of the blog post |
| `publishedAt` | String | **yes** | Timestamp when the blog post was published |
| `updatedAt` | String | **yes** | Timestamp when the blog post was last updated |

### `BlogPostUpdateResponseWrapperDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `updatedBlogPost` | [`BlogPostResponseDTO`](#blogpostresponsedto) | **yes** | Object containing response data of blog post update |

### `BlogResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Unique identifier of the blog |
| `name` | String | **yes** | Name of the blog |

### `CategoriesResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `categories` | Vec<CategoryResponseDTO> | **yes** | Array of categories |

### `CategoryResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | — |
| `label` | String | no | — |
| `locationId` | String | **yes** | — |
| `updatedAt` | String | **yes** | — |
| `canonicalLink` | String | **yes** | — |
| `urlSlug` | String | **yes** | — |

### `CreateBlogPostParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | — |
| `locationId` | String | **yes** | — |
| `blogId` | String | **yes** | You can find the blog id from blog site dashboard link |
| `imageUrl` | String | **yes** | — |
| `description` | String | **yes** | — |
| `rawHTML` | String | **yes** | — |
| `status` | String — `DRAFT`, `PUBLISHED`, `SCHEDULED`, `ARCHIVED` | **yes** | — |
| `imageAltText` | String | **yes** | — |
| `categories` | Vec<String> | **yes** | This needs to be array of category ids, which you can get from the category get api call. |
| `tags` | Vec<String> | no | — |
| `author` | String | **yes** | This needs to be author id, which you can get from the author get api call. |
| `urlSlug` | String | **yes** | — |
| `canonicalLink` | String | no | — |
| `publishedAt` | String | **yes** | Provide ISO timestamp |

### `UpdateBlogPostParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | — |
| `locationId` | String | **yes** | — |
| `blogId` | String | **yes** | You can find the blog id from blog site dashboard link |
| `imageUrl` | String | **yes** | — |
| `description` | String | **yes** | — |
| `rawHTML` | String | **yes** | — |
| `status` | String — `DRAFT`, `PUBLISHED`, `SCHEDULED`, `ARCHIVED` | **yes** | — |
| `imageAltText` | String | **yes** | — |
| `categories` | Vec<String> | **yes** | This needs to be array of category ids, which you can get from the category get api call. |
| `tags` | Vec<String> | no | — |
| `author` | String | **yes** | This needs to be author id, which you can get from the author get api call. |
| `urlSlug` | String | **yes** | — |
| `canonicalLink` | String | no | — |
| `publishedAt` | String | **yes** | Provide ISO timestamp |

### `UrlSlugCheckResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `exists` | bool | **yes** | Indicates whether the url slug exists or not |

## Data models — API v3

In Rust: `ghl_models::v3::blogs::*` (enable the `blogs` feature). Full field docs on [docs.rs](https://docs.rs/ghl-models/latest/ghl_models/v3/blogs/).

### `AuthorResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | — |
| `name` | String | **yes** | — |
| `locationId` | String | **yes** | — |
| `updatedAt` | String | **yes** | — |
| `canonicalLink` | String | **yes** | — |

### `AuthorsResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `authors` | Vec<AuthorResponseDTO> | **yes** | Array of authors |

### `BlogGetResponseWrapperDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | Vec<BlogResponseDTO> | **yes** | Object containing response data of blog |

### `BlogPostCreateResponseWrapperDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `data` | [`BlogPostResponseDTO`](#blogpostresponsedto) | **yes** | Object containing response data of blog post create. |

### `BlogPostGetResponseWrapperDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `blogs` | Vec<BlogPostResponseDTO> | **yes** | Object containing response data of blog posts |

### `BlogPostResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `categories` | Vec<String> | **yes** | Array of category IDs associated with the blog post |
| `tags` | Vec<String> | no | Array of tags associated with the blog post |
| `archived` | bool | **yes** | Indicates whether the blog post is archived |
| `_id` | String | **yes** | Unique identifier of the blog post |
| `title` | String | **yes** | Title of the blog post |
| `description` | String | **yes** | Description of the blog post |
| `imageUrl` | String | **yes** | URL of the image associated with the blog post |
| `status` | String | **yes** | Publication status of the blog post |
| `imageAltText` | String | **yes** | Alternative text for the blog post image |
| `urlSlug` | String | **yes** | URL slug for the blog post |
| `canonicalLink` | String | no | Canonical link of the blog post |
| `author` | String | no | Identifier of the author of the blog post |
| `publishedAt` | String | **yes** | Timestamp when the blog post was published |
| `updatedAt` | String | **yes** | Timestamp when the blog post was last updated |

### `BlogPostUpdateResponseWrapperDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `updatedBlogPost` | [`BlogPostResponseDTO`](#blogpostresponsedto) | **yes** | Object containing response data of blog post update |

### `BlogResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | Unique identifier of the blog |
| `name` | String | **yes** | Name of the blog |

### `CategoriesResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `categories` | Vec<CategoryResponseDTO> | **yes** | Array of categories |

### `CategoryResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `_id` | String | **yes** | — |
| `label` | String | no | — |
| `locationId` | String | **yes** | — |
| `updatedAt` | String | **yes** | — |
| `canonicalLink` | String | **yes** | — |
| `urlSlug` | String | **yes** | — |

### `CreateBlogPostParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | — |
| `locationId` | String | **yes** | — |
| `blogId` | String | **yes** | You can find the blog id from blog site dashboard link |
| `imageUrl` | String | **yes** | — |
| `description` | String | **yes** | — |
| `rawHTML` | String | **yes** | — |
| `status` | String — `DRAFT`, `PUBLISHED`, `SCHEDULED`, `ARCHIVED` | **yes** | — |
| `imageAltText` | String | **yes** | — |
| `categories` | Vec<String> | **yes** | This needs to be array of category ids, which you can get from the category get api call. |
| `tags` | Vec<String> | no | — |
| `author` | String | **yes** | This needs to be author id, which you can get from the author get api call. |
| `urlSlug` | String | **yes** | — |
| `canonicalLink` | String | no | — |
| `publishedAt` | String | **yes** | Provide ISO timestamp |

### `UpdateBlogPostParams`

| Field | Type | Required | Description |
|---|---|---|---|
| `title` | String | **yes** | — |
| `locationId` | String | **yes** | — |
| `blogId` | String | **yes** | You can find the blog id from blog site dashboard link |
| `imageUrl` | String | **yes** | — |
| `description` | String | **yes** | — |
| `rawHTML` | String | **yes** | — |
| `status` | String — `DRAFT`, `PUBLISHED`, `SCHEDULED`, `ARCHIVED` | **yes** | — |
| `imageAltText` | String | **yes** | — |
| `categories` | Vec<String> | **yes** | This needs to be array of category ids, which you can get from the category get api call. |
| `tags` | Vec<String> | no | — |
| `author` | String | **yes** | This needs to be author id, which you can get from the author get api call. |
| `urlSlug` | String | **yes** | — |
| `wordCount` | f64 | **yes** | — |
| `canonicalLink` | String | no | — |
| `publishedAt` | String | **yes** | Provide ISO timestamp |

### `UrlSlugCheckResponseDTO`

| Field | Type | Required | Description |
|---|---|---|---|
| `exists` | bool | **yes** | Indicates whether the url slug exists or not |

