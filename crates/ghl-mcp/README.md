# ghl-mcp

MCP server for [GoHighLevel](https://www.gohighlevel.com) CRM — a **single static binary** built on the official [`rmcp`](https://crates.io/crates/rmcp) SDK and [`ghl-sdk`](https://crates.io/crates/ghl-sdk). No Node runtime, rate-limit-aware, destructive actions gated off by default.

## Install & run

```sh
cargo install ghl-mcp
```

Claude Desktop / Claude Code / any MCP host:

```json
{
  "mcpServers": {
    "gohighlevel": {
      "command": "ghl-mcp",
      "env": {
        "GHL_PIT_TOKEN": "pit-…",
        "GHL_LOCATION_ID": "your-location-id"
      }
    }
  }
}
```

Create the `pit-…` token in your sub-account under **Settings → Private Integrations** with contacts + locations scopes.

## Tools

21 tools: dedicated typed tools for the busiest modules, plus meta-tools that reach **every** GoHighLevel endpoint.

### Contacts
| Tool | Effect |
|---|---|
| `ghl_search_contacts` | Search/list contacts (paginated via `next_cursor`) — read-only |
| `ghl_get_contact` | Full contact by id — read-only |
| `ghl_create_contact` | Create a contact (requires email or phone) |
| `ghl_update_contact` | Update provided fields only |
| `ghl_delete_contact` | **Gated** — needs `--allow-destructive` |

### Opportunities
| Tool | Effect |
|---|---|
| `ghl_list_pipelines` | Pipelines + stage ids for a location — read-only |
| `ghl_search_opportunities` | Search deals by pipeline/status/text (paginated) — read-only |
| `ghl_get_opportunity` | Full opportunity by id — read-only |
| `ghl_create_opportunity` | Create a deal in a pipeline |
| `ghl_move_opportunity` | Change a deal's stage and/or status |

### Conversations & calendars
| Tool | Effect |
|---|---|
| `ghl_search_conversations` | Threads with last-message preview and unread counts — read-only |
| `ghl_get_messages` | Messages in a thread, newest first — read-only |
| `ghl_send_message` | Send SMS/email/channel message — **gated** (reaches a real person) |
| `ghl_list_calendars` | Bookable calendars — read-only |
| `ghl_get_free_slots` | Available slots in a date range — read-only |
| `ghl_book_appointment` | Book an appointment — **gated** |

### Whole-API access (meta-tools)
| Tool | Effect |
|---|---|
| `ghl_search_operations` | Find any of **1,203 operations across 45 modules**, API v2 **and** v3 — read-only |
| `ghl_describe_operation` | Parameters, body fields, and required scopes for an operation — read-only |
| `ghl_execute_operation` | Call any endpoint by id; writes are **gated** — read-only GETs always work |

### Utility
| Tool | Effect |
|---|---|
| `ghl_list_locations` | Discover sub-accounts visible to the credential — read-only |
| `ghl_rate_status` | Remaining API rate budget — read-only, no API call |

The operations catalog is generated from [HighLevel's official OpenAPI specs](https://github.com/GoHighLevel/highlevel-api-docs) and embedded in the binary, so invoices, payments, workflows, forms, products, social planner, ad publishing, voice AI, SaaS, custom objects — everything — is callable without waiting for typed coverage.

**Both API versions.** v2 operation ids look like `invoices.get_invoices`; v3 ids are prefixed `v3:` (e.g. `v3:social-planner.post_posts`). Each carries its own `Version` header, so the right one is always sent — including the V3 `ad-publishing` module, which unusually declares `2021-07-28` rather than `v3`. Pass `api_version` to `ghl_search_operations` to filter; v2 wins ties by default since it's the stable API.

For typed request/response structs in your own Rust code, see [`ghl-models`](https://crates.io/crates/ghl-models) (2,417 generated DTOs).

## Configuration

Flags win over environment variables.

| Env var | Flag | Purpose |
|---|---|---|
| `GHL_PIT_TOKEN` | `--pit-token` | Private Integration Token |
| `GHL_ACCESS_TOKEN` | `--access-token` | OAuth access token (alternative) |
| `GHL_LOCATION_ID` | `--location-id` | Default location for tool calls |
| `GHL_BASE_URL` | `--base-url` | API base override |
| `GHL_ALLOW_DESTRUCTIVE` | `--allow-destructive` | Enable deletion tools |
| `RUST_LOG` | — | Log filter (stderr only; stdout is the MCP channel) |

License: MIT or Apache-2.0. *Not affiliated with HighLevel Inc.*
