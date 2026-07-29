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

| Tool | Effect |
|---|---|
| `ghl_search_contacts` | Search/list contacts (paginated via `next_cursor`) — read-only |
| `ghl_get_contact` | Full contact by id — read-only |
| `ghl_create_contact` | Create a contact (requires email or phone) |
| `ghl_update_contact` | Update provided fields only |
| `ghl_delete_contact` | **Destructive** — disabled unless `--allow-destructive` |
| `ghl_list_locations` | Discover sub-accounts visible to the credential — read-only |
| `ghl_rate_status` | Remaining API rate budget — read-only, no API call |

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
