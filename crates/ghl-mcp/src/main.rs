//! ghl-mcp: MCP server for GoHighLevel CRM.
//!
//! Two transports:
//! - **stdio** (default) — what Claude Desktop, Claude Code and most MCP hosts
//!   launch directly.
//! - **Streamable HTTP** (`--http <addr>`) — for running one shared server, e.g.
//!   a container serving several agents.

mod operations;
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

    /// Enable destructive tools (writes, deletes, sending messages, booking).
    /// Off by default.
    #[arg(long, env = "GHL_ALLOW_DESTRUCTIVE", default_value_t = false)]
    allow_destructive: bool,

    /// Serve Streamable HTTP on this address instead of stdio,
    /// e.g. `127.0.0.1:8000`. The MCP endpoint is `/mcp`.
    ///
    /// The listener has **no authentication of its own** — it hands every caller
    /// the credentials above. Bind it to localhost, or put it behind a proxy that
    /// authenticates, and never expose it to the internet directly.
    #[arg(long, env = "GHL_HTTP_ADDR")]
    http: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // stdout carries the MCP protocol on stdio — all logging must go to stderr.
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

    let location = cli.location_id;
    let destructive = cli.allow_destructive;

    match cli.http {
        Some(addr) => serve_http(ghl, location, destructive, &addr).await,
        None => {
            tracing::info!(
                default_location = location.as_deref().unwrap_or("<none>"),
                "starting ghl-mcp on stdio"
            );
            let service = GhlServer::new(ghl, location, destructive)
                .serve(stdio())
                .await
                .context("failed to start MCP server")?;
            service.waiting().await?;
            Ok(())
        }
    }
}

/// Serve MCP over Streamable HTTP at `/mcp`.
async fn serve_http(
    ghl: Ghl,
    location: Option<String>,
    destructive: bool,
    addr: &str,
) -> anyhow::Result<()> {
    use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
    use rmcp::transport::streamable_http_server::{
        StreamableHttpServerConfig, StreamableHttpService,
    };

    // Stateless: the factory runs per request, so the handler holds only cheap
    // clones of the (Arc-backed) client.
    let service = StreamableHttpService::new(
        move || Ok(GhlServer::new(ghl.clone(), location.clone(), destructive)),
        LocalSessionManager::default().into(),
        StreamableHttpServerConfig::default()
            .with_legacy_session_mode(false)
            .with_json_response(true),
    );

    let app = axum::Router::new().nest_service("/mcp", service);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind {addr}"))?;

    tracing::warn!(
        %addr,
        "serving MCP over HTTP at /mcp — this endpoint is UNAUTHENTICATED and \
         exposes the configured GoHighLevel credentials to any caller; bind to \
         localhost or front it with an authenticating proxy"
    );
    axum::serve(listener, app)
        .await
        .context("HTTP server failed")?;
    Ok(())
}
