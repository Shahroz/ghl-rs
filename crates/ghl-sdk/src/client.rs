//! The core HTTP client: request building, auth, retries, and rate-limit tracking.

use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Method, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::auth::Auth;
use crate::contacts::ContactsService;
use crate::error::{Error, Result};
use crate::locations::LocationsService;
use crate::opportunities::OpportunitiesService;

/// Production API base URL.
pub const DEFAULT_BASE_URL: &str = "https://services.leadconnectorhq.com";

/// The `Version` header value used by most API 2.0 modules.
pub const API_VERSION: &str = "2021-07-28";

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_MAX_RETRIES: u32 = 3;
const BACKOFF_BASE: Duration = Duration::from_millis(500);
const BACKOFF_CAP: Duration = Duration::from_secs(8);

/// Async client for the GoHighLevel API 2.0.
///
/// Cloning is cheap (`Arc` internally); share one client across tasks.
///
/// ```no_run
/// # async fn demo() -> Result<(), ghl_sdk::Error> {
/// // From the environment (GHL_PIT_TOKEN or GHL_ACCESS_TOKEN, optional GHL_BASE_URL):
/// let ghl = ghl_sdk::Ghl::from_env()?;
/// // Or explicitly:
/// let ghl = ghl_sdk::Ghl::builder()
///     .private_integration_token("pit-…")
///     .build()?;
/// # Ok(()) }
/// ```
#[derive(Clone)]
pub struct Ghl {
    inner: Arc<Inner>,
}

struct Inner {
    http: reqwest::Client,
    base_url: String,
    auth: Auth,
    max_retries: u32,
    /// Last-seen `X-RateLimit-Remaining` (burst window); -1 = unknown.
    rate_remaining: AtomicI64,
    /// Last-seen `X-RateLimit-Daily-Remaining`; -1 = unknown.
    rate_daily_remaining: AtomicI64,
}

impl std::fmt::Debug for Ghl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ghl")
            .field("base_url", &self.inner.base_url)
            .field("auth", &self.inner.auth)
            .finish_non_exhaustive()
    }
}

/// Builder for [`Ghl`].
#[derive(Default)]
pub struct GhlBuilder {
    base_url: Option<String>,
    auth: Option<Auth>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
}

impl GhlBuilder {
    /// Override the API base URL (e.g. for tests or a proxy).
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Authenticate with a Private Integration Token (`pit-…`).
    pub fn private_integration_token(mut self, token: impl Into<String>) -> Self {
        self.auth = Some(Auth::private_integration(token));
        self
    }

    /// Authenticate with a pre-obtained OAuth access token (no refresh).
    pub fn access_token(mut self, token: impl Into<String>) -> Self {
        self.auth = Some(Auth::access_token(token));
        self
    }

    /// Authenticate with any [`Auth`] variant (e.g. full OAuth with refresh).
    pub fn auth(mut self, auth: Auth) -> Self {
        self.auth = Some(auth);
        self
    }

    /// Per-request timeout (default 30s).
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Maximum retry attempts after the initial request (default 3).
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = Some(retries);
        self
    }

    /// Finalize the client. Errors if no credentials were provided.
    pub fn build(self) -> Result<Ghl> {
        let auth = self.auth.ok_or_else(|| {
            Error::Config(
                "no credentials configured — call `.private_integration_token(…)`, \
                 `.access_token(…)`, or `.auth(…)`"
                    .into(),
            )
        })?;
        let base_url = self
            .base_url
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_owned())
            .trim_end_matches('/')
            .to_owned();
        let http = reqwest::Client::builder()
            .timeout(self.timeout.unwrap_or(DEFAULT_TIMEOUT))
            .user_agent(concat!("ghl-sdk/", env!("CARGO_PKG_VERSION")))
            .build()?;
        Ok(Ghl {
            inner: Arc::new(Inner {
                http,
                base_url,
                auth,
                max_retries: self.max_retries.unwrap_or(DEFAULT_MAX_RETRIES),
                rate_remaining: AtomicI64::new(-1),
                rate_daily_remaining: AtomicI64::new(-1),
            }),
        })
    }
}

/// A snapshot of the most recently observed rate-limit headers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateStatus {
    /// Remaining requests in the current burst window (100 req / 10s), if known.
    pub burst_remaining: Option<i64>,
    /// Remaining requests today (200k/day), if known.
    pub daily_remaining: Option<i64>,
}

impl Ghl {
    /// Start configuring a client.
    pub fn builder() -> GhlBuilder {
        GhlBuilder::default()
    }

    /// Build a client from environment variables.
    ///
    /// - `GHL_PIT_TOKEN` — Private Integration Token, **or**
    /// - `GHL_ACCESS_TOKEN` — OAuth access token
    /// - `GHL_BASE_URL` — optional base-URL override
    pub fn from_env() -> Result<Self> {
        let mut builder = Ghl::builder();
        if let Ok(url) = std::env::var("GHL_BASE_URL") {
            builder = builder.base_url(url);
        }
        if let Ok(token) = std::env::var("GHL_PIT_TOKEN") {
            builder = builder.private_integration_token(token);
        } else if let Ok(token) = std::env::var("GHL_ACCESS_TOKEN") {
            builder = builder.access_token(token);
        } else {
            return Err(Error::Config(
                "set GHL_PIT_TOKEN (or GHL_ACCESS_TOKEN) in the environment, \
                 or use `Ghl::builder()` to pass credentials as parameters"
                    .into(),
            ));
        }
        builder.build()
    }

    /// Contacts API.
    pub fn contacts(&self) -> ContactsService {
        ContactsService::new(self.clone())
    }

    /// Locations (sub-accounts) API.
    pub fn locations(&self) -> LocationsService {
        LocationsService::new(self.clone())
    }

    /// Opportunities (pipeline deals) API.
    pub fn opportunities(&self) -> OpportunitiesService {
        OpportunitiesService::new(self.clone())
    }

    /// The most recently observed rate-limit headroom.
    pub fn rate_status(&self) -> RateStatus {
        let read = |a: &AtomicI64| {
            let v = a.load(Ordering::Relaxed);
            (v >= 0).then_some(v)
        };
        RateStatus {
            burst_remaining: read(&self.inner.rate_remaining),
            daily_remaining: read(&self.inner.rate_daily_remaining),
        }
    }

    /// Exchange an **agency (Company) token** for a **location token**
    /// (`POST /oauth/locationToken`) and return a new client scoped to that location.
    ///
    /// This is the multi-tenant primitive: one agency credential, many sub-accounts.
    pub async fn as_location(&self, company_id: &str, location_id: &str) -> Result<Ghl> {
        #[derive(serde::Deserialize)]
        struct LocationTokenResponse {
            access_token: String,
        }

        let bearer = self
            .inner
            .auth
            .bearer(&self.inner.http, &self.inner.base_url)
            .await?;
        let response = self
            .inner
            .http
            .post(format!("{}/oauth/locationToken", self.inner.base_url))
            .header(AUTHORIZATION, format!("Bearer {bearer}"))
            .header("Version", API_VERSION)
            .form(&[("companyId", company_id), ("locationId", location_id)])
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Auth(format!(
                "location token exchange failed ({status}): {body}"
            )));
        }
        let parsed: LocationTokenResponse = response
            .json()
            .await
            .map_err(|e| Error::Auth(format!("unexpected locationToken response: {e}")))?;

        Ghl::builder()
            .base_url(&self.inner.base_url)
            .access_token(parsed.access_token)
            .max_retries(self.inner.max_retries)
            .build()
    }

    // ----- raw escape hatches (for endpoints not yet typed) -----

    /// `GET` any API path and return the raw JSON.
    pub async fn get_raw(&self, path: &str, query: &[(&str, &str)]) -> Result<serde_json::Value> {
        let query: Vec<(String, String)> = query
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect();
        self.send(Method::GET, path, &query, None::<&()>).await
    }

    /// `POST` any API path with a JSON body and return the raw JSON.
    pub async fn post_raw(&self, path: &str, body: &impl Serialize) -> Result<serde_json::Value> {
        self.send(Method::POST, path, &[], Some(body)).await
    }

    // ----- request core -----

    pub(crate) async fn send<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<&impl Serialize>,
    ) -> Result<T> {
        // Serialize the body once so retries don't re-serialize (and can't observe drift).
        let body = match body {
            Some(b) => Some(serde_json::to_value(b).map_err(|source| Error::Decode {
                endpoint: path.to_owned(),
                source,
            })?),
            None => None,
        };

        let url = format!("{}{}", self.inner.base_url, path);
        let idempotent = matches!(
            method,
            Method::GET | Method::PUT | Method::DELETE | Method::HEAD
        );
        let mut attempt: u32 = 0;

        loop {
            let bearer = self
                .inner
                .auth
                .bearer(&self.inner.http, &self.inner.base_url)
                .await?;

            let mut request = self
                .inner
                .http
                .request(method.clone(), &url)
                .header(AUTHORIZATION, format!("Bearer {bearer}"))
                .header("Version", API_VERSION)
                .header(reqwest::header::ACCEPT, "application/json");
            if !query.is_empty() {
                request = request.query(query);
            }
            if let Some(ref b) = body {
                request = request.json(b);
            }

            let outcome = request.send().await;

            match outcome {
                Ok(response) => {
                    self.record_rate_headers(response.headers());
                    let status = response.status();

                    if status.is_success() {
                        let bytes = response.bytes().await?;
                        return serde_json::from_slice(&bytes).map_err(|source| Error::Decode {
                            endpoint: path.to_owned(),
                            source,
                        });
                    }

                    let retry_after = parse_retry_after(response.headers());
                    let request_id = response
                        .headers()
                        .get("x-request-id")
                        .and_then(|v| v.to_str().ok())
                        .map(str::to_owned);
                    let message = read_api_message(response).await;

                    let retryable = status == StatusCode::TOO_MANY_REQUESTS
                        || (idempotent && status.is_server_error());
                    if retryable && attempt < self.inner.max_retries {
                        let delay = retry_after.unwrap_or_else(|| backoff_delay(attempt));
                        tracing::warn!(
                            %status, attempt, delay_ms = delay.as_millis() as u64, path,
                            "GoHighLevel request failed; retrying"
                        );
                        tokio::time::sleep(delay).await;
                        attempt += 1;
                        continue;
                    }

                    return Err(if status == StatusCode::TOO_MANY_REQUESTS {
                        Error::RateLimited { retry_after }
                    } else {
                        Error::Api {
                            status,
                            message,
                            request_id,
                        }
                    });
                }
                Err(err) => {
                    // Connection-level failures: retry idempotent requests only.
                    if idempotent && attempt < self.inner.max_retries && err.status().is_none() {
                        let delay = backoff_delay(attempt);
                        tracing::warn!(
                            error = %err, attempt, delay_ms = delay.as_millis() as u64, path,
                            "transport error; retrying"
                        );
                        tokio::time::sleep(delay).await;
                        attempt += 1;
                        continue;
                    }
                    return Err(err.into());
                }
            }
        }
    }

    fn record_rate_headers(&self, headers: &HeaderMap) {
        let parse =
            |name: &str| -> Option<i64> { headers.get(name)?.to_str().ok()?.trim().parse().ok() };
        if let Some(v) = parse("x-ratelimit-remaining") {
            self.inner.rate_remaining.store(v, Ordering::Relaxed);
        }
        if let Some(v) = parse("x-ratelimit-daily-remaining") {
            self.inner.rate_daily_remaining.store(v, Ordering::Relaxed);
        }
    }
}

/// Exponential backoff with full jitter, capped.
fn backoff_delay(attempt: u32) -> Duration {
    let exp = BACKOFF_BASE.saturating_mul(2u32.saturating_pow(attempt));
    let cap = exp.min(BACKOFF_CAP);
    cap.mul_f64(0.5 + fastrand::f64() * 0.5)
}

fn parse_retry_after(headers: &HeaderMap) -> Option<Duration> {
    let value: &HeaderValue = headers.get(reqwest::header::RETRY_AFTER)?;
    let seconds: u64 = value.to_str().ok()?.trim().parse().ok()?;
    Some(Duration::from_secs(seconds))
}

/// Pull a human-readable message out of a GoHighLevel error body.
/// Bodies are typically `{"message": "…"}` or `{"message": ["…", "…"]}`.
async fn read_api_message(response: reqwest::Response) -> String {
    let text = response.text().await.unwrap_or_default();
    match serde_json::from_str::<serde_json::Value>(&text) {
        Ok(v) => match v.get("message") {
            Some(serde_json::Value::String(s)) => s.clone(),
            Some(serde_json::Value::Array(parts)) => parts
                .iter()
                .filter_map(|p| p.as_str())
                .collect::<Vec<_>>()
                .join("; "),
            _ => text,
        },
        Err(_) => text,
    }
}
