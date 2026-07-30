//! The core HTTP client: request building, auth, retries, and rate-limit tracking.

use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Method, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::auth::Auth;
use crate::calendars::CalendarsService;
use crate::contacts::ContactsService;
use crate::conversations::ConversationsService;
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

    /// Conversations (threads, messages, sending) API.
    pub fn conversations(&self) -> ConversationsService {
        ConversationsService::new(self.clone())
    }

    /// Calendars (calendars, free slots, appointments) API.
    pub fn calendars(&self) -> CalendarsService {
        CalendarsService::new(self.clone())
    }

    /// Typed access to the `ad-manager` API.
    ///
    /// Requires the `ad-manager` cargo feature.
    #[cfg(feature = "ad-manager")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ad-manager")))]
    pub fn ad_manager(&self) -> crate::services::ad_manager::AdManagerService {
        crate::services::ad_manager::AdManagerService::new(self.clone())
    }
    /// Typed access to the `affiliate-manager` API.
    ///
    /// Requires the `affiliate-manager` cargo feature.
    #[cfg(feature = "affiliate-manager")]
    #[cfg_attr(docsrs, doc(cfg(feature = "affiliate-manager")))]
    pub fn affiliate_manager(&self) -> crate::services::affiliate_manager::AffiliateManagerService {
        crate::services::affiliate_manager::AffiliateManagerService::new(self.clone())
    }
    /// Typed access to the `agent-studio` API.
    ///
    /// Requires the `agent-studio` cargo feature.
    #[cfg(feature = "agent-studio")]
    #[cfg_attr(docsrs, doc(cfg(feature = "agent-studio")))]
    pub fn agent_studio(&self) -> crate::services::agent_studio::AgentStudioService {
        crate::services::agent_studio::AgentStudioService::new(self.clone())
    }
    /// Typed access to the `associations` API.
    ///
    /// Requires the `associations` cargo feature.
    #[cfg(feature = "associations")]
    #[cfg_attr(docsrs, doc(cfg(feature = "associations")))]
    pub fn associations(&self) -> crate::services::associations::AssociationsService {
        crate::services::associations::AssociationsService::new(self.clone())
    }
    /// Typed access to the `blogs` API.
    ///
    /// Requires the `blogs` cargo feature.
    #[cfg(feature = "blogs")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blogs")))]
    pub fn blogs(&self) -> crate::services::blogs::BlogsService {
        crate::services::blogs::BlogsService::new(self.clone())
    }
    /// Typed access to the `brand-boards` API.
    ///
    /// Requires the `brand-boards` cargo feature.
    #[cfg(feature = "brand-boards")]
    #[cfg_attr(docsrs, doc(cfg(feature = "brand-boards")))]
    pub fn brand_boards(&self) -> crate::services::brand_boards::BrandBoardsService {
        crate::services::brand_boards::BrandBoardsService::new(self.clone())
    }
    /// Typed access to the `businesses` API.
    ///
    /// Requires the `businesses` cargo feature.
    #[cfg(feature = "businesses")]
    #[cfg_attr(docsrs, doc(cfg(feature = "businesses")))]
    pub fn businesses(&self) -> crate::services::businesses::BusinessesService {
        crate::services::businesses::BusinessesService::new(self.clone())
    }
    /// Typed access to the `campaigns` API.
    ///
    /// Requires the `campaigns` cargo feature.
    #[cfg(feature = "campaigns")]
    #[cfg_attr(docsrs, doc(cfg(feature = "campaigns")))]
    pub fn campaigns(&self) -> crate::services::campaigns::CampaignsService {
        crate::services::campaigns::CampaignsService::new(self.clone())
    }
    /// Typed access to the `companies` API.
    ///
    /// Requires the `companies` cargo feature.
    #[cfg(feature = "companies")]
    #[cfg_attr(docsrs, doc(cfg(feature = "companies")))]
    pub fn companies(&self) -> crate::services::companies::CompaniesService {
        crate::services::companies::CompaniesService::new(self.clone())
    }
    /// Typed access to the `conversation-ai` API.
    ///
    /// Requires the `conversation-ai` cargo feature.
    #[cfg(feature = "conversation-ai")]
    #[cfg_attr(docsrs, doc(cfg(feature = "conversation-ai")))]
    pub fn conversation_ai(&self) -> crate::services::conversation_ai::ConversationAiService {
        crate::services::conversation_ai::ConversationAiService::new(self.clone())
    }
    /// Typed access to the `courses` API.
    ///
    /// Requires the `courses` cargo feature.
    #[cfg(feature = "courses")]
    #[cfg_attr(docsrs, doc(cfg(feature = "courses")))]
    pub fn courses(&self) -> crate::services::courses::CoursesService {
        crate::services::courses::CoursesService::new(self.clone())
    }
    /// Typed access to the `custom-fields` API.
    ///
    /// Requires the `custom-fields` cargo feature.
    #[cfg(feature = "custom-fields")]
    #[cfg_attr(docsrs, doc(cfg(feature = "custom-fields")))]
    pub fn custom_fields(&self) -> crate::services::custom_fields::CustomFieldsService {
        crate::services::custom_fields::CustomFieldsService::new(self.clone())
    }
    /// Typed access to the `custom-menus` API.
    ///
    /// Requires the `custom-menus` cargo feature.
    #[cfg(feature = "custom-menus")]
    #[cfg_attr(docsrs, doc(cfg(feature = "custom-menus")))]
    pub fn custom_menus(&self) -> crate::services::custom_menus::CustomMenusService {
        crate::services::custom_menus::CustomMenusService::new(self.clone())
    }
    /// Typed access to the `email-isv` API.
    ///
    /// Requires the `email-isv` cargo feature.
    #[cfg(feature = "email-isv")]
    #[cfg_attr(docsrs, doc(cfg(feature = "email-isv")))]
    pub fn email_isv(&self) -> crate::services::email_isv::EmailIsvService {
        crate::services::email_isv::EmailIsvService::new(self.clone())
    }
    /// Typed access to the `emails` API.
    ///
    /// Requires the `emails` cargo feature.
    #[cfg(feature = "emails")]
    #[cfg_attr(docsrs, doc(cfg(feature = "emails")))]
    pub fn emails(&self) -> crate::services::emails::EmailsService {
        crate::services::emails::EmailsService::new(self.clone())
    }
    /// Typed access to the `forms` API.
    ///
    /// Requires the `forms` cargo feature.
    #[cfg(feature = "forms")]
    #[cfg_attr(docsrs, doc(cfg(feature = "forms")))]
    pub fn forms(&self) -> crate::services::forms::FormsService {
        crate::services::forms::FormsService::new(self.clone())
    }
    /// Typed access to the `funnels` API.
    ///
    /// Requires the `funnels` cargo feature.
    #[cfg(feature = "funnels")]
    #[cfg_attr(docsrs, doc(cfg(feature = "funnels")))]
    pub fn funnels(&self) -> crate::services::funnels::FunnelsService {
        crate::services::funnels::FunnelsService::new(self.clone())
    }
    /// Typed access to the `invoices` API.
    ///
    /// Requires the `invoices` cargo feature.
    #[cfg(feature = "invoices")]
    #[cfg_attr(docsrs, doc(cfg(feature = "invoices")))]
    pub fn invoices(&self) -> crate::services::invoices::InvoicesService {
        crate::services::invoices::InvoicesService::new(self.clone())
    }
    /// Typed access to the `knowledge-base` API.
    ///
    /// Requires the `knowledge-base` cargo feature.
    #[cfg(feature = "knowledge-base")]
    #[cfg_attr(docsrs, doc(cfg(feature = "knowledge-base")))]
    pub fn knowledge_base(&self) -> crate::services::knowledge_base::KnowledgeBaseService {
        crate::services::knowledge_base::KnowledgeBaseService::new(self.clone())
    }
    /// Typed access to the `links` API.
    ///
    /// Requires the `links` cargo feature.
    #[cfg(feature = "links")]
    #[cfg_attr(docsrs, doc(cfg(feature = "links")))]
    pub fn links(&self) -> crate::services::links::LinksService {
        crate::services::links::LinksService::new(self.clone())
    }
    /// Typed access to the `marketplace` API.
    ///
    /// Requires the `marketplace` cargo feature.
    #[cfg(feature = "marketplace")]
    #[cfg_attr(docsrs, doc(cfg(feature = "marketplace")))]
    pub fn marketplace(&self) -> crate::services::marketplace::MarketplaceService {
        crate::services::marketplace::MarketplaceService::new(self.clone())
    }
    /// Typed access to the `medias` API.
    ///
    /// Requires the `medias` cargo feature.
    #[cfg(feature = "medias")]
    #[cfg_attr(docsrs, doc(cfg(feature = "medias")))]
    pub fn medias(&self) -> crate::services::medias::MediasService {
        crate::services::medias::MediasService::new(self.clone())
    }
    /// Typed access to the `oauth` API.
    ///
    /// Requires the `oauth` cargo feature.
    #[cfg(feature = "oauth")]
    #[cfg_attr(docsrs, doc(cfg(feature = "oauth")))]
    pub fn oauth(&self) -> crate::services::oauth::OauthService {
        crate::services::oauth::OauthService::new(self.clone())
    }
    /// Typed access to the `objects` API.
    ///
    /// Requires the `objects` cargo feature.
    #[cfg(feature = "objects")]
    #[cfg_attr(docsrs, doc(cfg(feature = "objects")))]
    pub fn objects(&self) -> crate::services::objects::ObjectsService {
        crate::services::objects::ObjectsService::new(self.clone())
    }
    /// Typed access to the `payments` API.
    ///
    /// Requires the `payments` cargo feature.
    #[cfg(feature = "payments")]
    #[cfg_attr(docsrs, doc(cfg(feature = "payments")))]
    pub fn payments(&self) -> crate::services::payments::PaymentsService {
        crate::services::payments::PaymentsService::new(self.clone())
    }
    /// Typed access to the `phone-system` API.
    ///
    /// Requires the `phone-system` cargo feature.
    #[cfg(feature = "phone-system")]
    #[cfg_attr(docsrs, doc(cfg(feature = "phone-system")))]
    pub fn phone_system(&self) -> crate::services::phone_system::PhoneSystemService {
        crate::services::phone_system::PhoneSystemService::new(self.clone())
    }
    /// Typed access to the `products` API.
    ///
    /// Requires the `products` cargo feature.
    #[cfg(feature = "products")]
    #[cfg_attr(docsrs, doc(cfg(feature = "products")))]
    pub fn products(&self) -> crate::services::products::ProductsService {
        crate::services::products::ProductsService::new(self.clone())
    }
    /// Typed access to the `proposals` API.
    ///
    /// Requires the `proposals` cargo feature.
    #[cfg(feature = "proposals")]
    #[cfg_attr(docsrs, doc(cfg(feature = "proposals")))]
    pub fn proposals(&self) -> crate::services::proposals::ProposalsService {
        crate::services::proposals::ProposalsService::new(self.clone())
    }
    /// Typed access to the `saas-api` API.
    ///
    /// Requires the `saas-api` cargo feature.
    #[cfg(feature = "saas-api")]
    #[cfg_attr(docsrs, doc(cfg(feature = "saas-api")))]
    pub fn saas_api(&self) -> crate::services::saas_api::SaasApiService {
        crate::services::saas_api::SaasApiService::new(self.clone())
    }
    /// Typed access to the `snapshots` API.
    ///
    /// Requires the `snapshots` cargo feature.
    #[cfg(feature = "snapshots")]
    #[cfg_attr(docsrs, doc(cfg(feature = "snapshots")))]
    pub fn snapshots(&self) -> crate::services::snapshots::SnapshotsService {
        crate::services::snapshots::SnapshotsService::new(self.clone())
    }
    /// Typed access to the `social-media-posting` API.
    ///
    /// Requires the `social-media-posting` cargo feature.
    #[cfg(feature = "social-media-posting")]
    #[cfg_attr(docsrs, doc(cfg(feature = "social-media-posting")))]
    pub fn social_media_posting(
        &self,
    ) -> crate::services::social_media_posting::SocialMediaPostingService {
        crate::services::social_media_posting::SocialMediaPostingService::new(self.clone())
    }
    /// Typed access to the `store` API.
    ///
    /// Requires the `store` cargo feature.
    #[cfg(feature = "store")]
    #[cfg_attr(docsrs, doc(cfg(feature = "store")))]
    pub fn store(&self) -> crate::services::store::StoreService {
        crate::services::store::StoreService::new(self.clone())
    }
    /// Typed access to the `surveys` API.
    ///
    /// Requires the `surveys` cargo feature.
    #[cfg(feature = "surveys")]
    #[cfg_attr(docsrs, doc(cfg(feature = "surveys")))]
    pub fn surveys(&self) -> crate::services::surveys::SurveysService {
        crate::services::surveys::SurveysService::new(self.clone())
    }
    /// Typed access to the `users` API.
    ///
    /// Requires the `users` cargo feature.
    #[cfg(feature = "users")]
    #[cfg_attr(docsrs, doc(cfg(feature = "users")))]
    pub fn users(&self) -> crate::services::users::UsersService {
        crate::services::users::UsersService::new(self.clone())
    }
    /// Typed access to the `voice-ai` API.
    ///
    /// Requires the `voice-ai` cargo feature.
    #[cfg(feature = "voice-ai")]
    #[cfg_attr(docsrs, doc(cfg(feature = "voice-ai")))]
    pub fn voice_ai(&self) -> crate::services::voice_ai::VoiceAiService {
        crate::services::voice_ai::VoiceAiService::new(self.clone())
    }
    /// Typed access to the `workflows` API.
    ///
    /// Requires the `workflows` cargo feature.
    #[cfg(feature = "workflows")]
    #[cfg_attr(docsrs, doc(cfg(feature = "workflows")))]
    pub fn workflows(&self) -> crate::services::workflows::WorkflowsService {
        crate::services::workflows::WorkflowsService::new(self.clone())
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

    /// Call any API endpoint with an arbitrary method, query, and body.
    ///
    /// This is the fully generic escape hatch. Prefer the typed module services
    /// when they cover what you need — they validate inputs and return real
    /// types. `version` overrides the `Version` header for the endpoints that
    /// require a value other than [`API_VERSION`].
    pub async fn request_raw(
        &self,
        method: &str,
        path: &str,
        query: &[(String, String)],
        body: Option<&serde_json::Value>,
        version: Option<&str>,
    ) -> Result<serde_json::Value> {
        let method = Method::from_bytes(method.to_uppercase().as_bytes())
            .map_err(|_| Error::Config(format!("invalid HTTP method `{method}`")))?;
        self.send_versioned(method, path, query, body, version)
            .await
    }

    // ----- request core -----

    pub(crate) async fn send<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<&impl Serialize>,
    ) -> Result<T> {
        self.send_versioned(method, path, query, body, None).await
    }

    pub(crate) async fn send_versioned<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<&impl Serialize>,
        version: Option<&str>,
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
                .header("Version", version.unwrap_or(API_VERSION))
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
