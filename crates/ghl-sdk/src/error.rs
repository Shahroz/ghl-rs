use std::time::Duration;

use reqwest::StatusCode;

/// All errors returned by this crate.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The API returned a non-success status code.
    #[error("GoHighLevel API error ({status}): {message}")]
    Api {
        /// HTTP status returned by the API.
        status: StatusCode,
        /// Human-readable message extracted from the error body.
        message: String,
        /// Request id echoed by the API, when present (useful in support threads).
        request_id: Option<String>,
    },

    /// The API returned 429 and retries were exhausted.
    #[error("rate limited by GoHighLevel API{}", retry_after_hint(.retry_after))]
    RateLimited {
        /// Server-suggested wait before retrying, from the `Retry-After` header.
        retry_after: Option<Duration>,
    },

    /// Authentication problems: missing credentials, failed token refresh, etc.
    #[error("authentication error: {0}")]
    Auth(String),

    /// Network / TLS / protocol-level failures.
    #[error("transport error: {0}")]
    Transport(#[from] reqwest::Error),

    /// The response body could not be decoded into the expected type.
    #[error("failed to decode response from `{endpoint}`: {source}")]
    Decode {
        /// The API path whose response failed to decode.
        endpoint: String,
        /// The underlying deserialization error.
        #[source]
        source: serde_json::Error,
    },

    /// Invalid client configuration.
    #[error("configuration error: {0}")]
    Config(String),
}

fn retry_after_hint(retry_after: &Option<Duration>) -> String {
    match retry_after {
        Some(d) => format!(" (retry after {}s)", d.as_secs()),
        None => String::new(),
    }
}

/// Crate-wide result alias.
pub type Result<T, E = Error> = std::result::Result<T, E>;
