//! ghl-mcp: MCP server for GoHighLevel CRM, over stdio.

mod server;

use anyhow::Context;
use clap::Parser;
use ghl_sdk::Ghl;
use rmcp::transport::stdio;
use rmcp::ServiceExt;

use crate::server::GhlServer;

/// MCP server for GoHighLevel CRM.
///
/// Every option can be set by flag or environment variable; flags win.
#[derive(Parser, Debug)]
#[command(name = "ghl-mcp", version, about)]
struct Cli {
    /// Private Integration Token (pit-…) — the simplest way to authenticate.
    #[arg(long, env = "GHL_PIT_TOKEN", hide_env_values = true)]
    pit_token: Option<String>,

    /// OAuth access token (alternative to --pit-token).
    #[arg(long, env = "GHL_ACCESS_TOKEN", hide_env_values = true)]
    access_token: Option<String>,

    /// Default location (sub-account) id used when a tool call omits location_id.
    #[arg(long, env = "GHL_LOCATION_ID")]
    location_id: Option<String>,

    /// API base URL override (proxies, testing).
    #[arg(long, env = "GHL_BASE_URL")]
    base_url: Option<String>,

    /// Enable destructive tools (contact deletion). Off by default.
    #[arg(long, env = "GHL_ALLOW_DESTRUCTIVE", default_value_t = false)]
    allow_destructive: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // stdout carries the MCP protocol — all logging must go to stderr.
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();

    let mut builder = Ghl::builder();
    if let Some(url) = cli.base_url {
        builder = builder.base_url(url);
    }
    builder = match (cli.pit_token, cli.access_token) {
        (Some(pit), _) => builder.private_integration_token(pit),
        (None, Some(token)) => builder.access_token(token),
        (None, None) => anyhow::bail!(
            "no credentials: set GHL_PIT_TOKEN (or GHL_ACCESS_TOKEN), \
             or pass --pit-token / --access-token"
        ),
    };
    let ghl = builder
        .build()
        .context("failed to build GoHighLevel client")?;

    if cli.allow_destructive {
        tracing::warn!("destructive tools ENABLED (--allow-destructive)");
    }
    tracing::info!(
        default_location = cli.location_id.as_deref().unwrap_or("<none>"),
        "starting ghl-mcp on stdio"
    );

    let service = GhlServer::new(ghl, cli.location_id, cli.allow_destructive)
        .serve(stdio())
        .await
        .context("failed to start MCP server")?;
    service.waiting().await?;
    Ok(())
}
