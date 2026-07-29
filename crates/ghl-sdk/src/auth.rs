//! Authentication: Private Integration Tokens and OAuth 2.0 with automatic refresh.

use std::sync::Arc;
use std::time::{Duration, SystemTime};

use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use tokio::sync::{Mutex, RwLock};

use crate::error::{Error, Result};

/// Refresh the access token this long before it actually expires.
const EXPIRY_SKEW: Duration = Duration::from_secs(60);

/// How the client authenticates against the API.
///
/// Pick one of:
/// - [`Auth::private_integration`] — a `pit-…` token created in a sub-account's
///   *Settings → Private Integrations*. The simplest option for internal tools.
/// - [`Auth::access_token`] — a raw OAuth access token you obtained yourself
///   (no refresh handling; the token is used as-is).
/// - [`Auth::oauth`] — full OAuth 2.0 with automatic refresh via a [`TokenStore`].
#[derive(Clone)]
pub enum Auth {
    /// Private Integration Token (`pit-…`).
    PrivateIntegration(SecretString),
    /// A raw bearer token supplied by the caller; never refreshed.
    AccessToken(SecretString),
    /// OAuth 2.0 marketplace-app credentials with automatic refresh.
    OAuth(OAuthAuth),
}

impl std::fmt::Debug for Auth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Never print token material.
        match self {
            Auth::PrivateIntegration(_) => f.write_str("Auth::PrivateIntegration(REDACTED)"),
            Auth::AccessToken(_) => f.write_str("Auth::AccessToken(REDACTED)"),
            Auth::OAuth(_) => f.write_str("Auth::OAuth(REDACTED)"),
        }
    }
}

impl Auth {
    /// Authenticate with a Private Integration Token (`pit-…`).
    pub fn private_integration(token: impl Into<String>) -> Self {
        Auth::PrivateIntegration(SecretString::from(token.into()))
    }

    /// Authenticate with a pre-obtained OAuth access token (no refresh).
    pub fn access_token(token: impl Into<String>) -> Self {
        Auth::AccessToken(SecretString::from(token.into()))
    }

    /// Authenticate with OAuth 2.0 credentials and automatic token refresh.
    pub fn oauth(config: OAuthConfig, store: Arc<dyn TokenStore>) -> Self {
        Auth::OAuth(OAuthAuth::new(config, store))
    }

    /// Resolve the current bearer token, refreshing if needed.
    pub(crate) async fn bearer(&self, http: &reqwest::Client, base_url: &str) -> Result<String> {
        match self {
            Auth::PrivateIntegration(t) | Auth::AccessToken(t) => Ok(t.expose_secret().to_owned()),
            Auth::OAuth(oauth) => oauth.bearer(http, base_url).await,
        }
    }
}

/// Whether the OAuth token belongs to a sub-account (location) or an agency (company).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserType {
    /// A sub-account token.
    Location,
    /// An agency-level token (can be exchanged for location tokens).
    Company,
}

impl UserType {
    fn as_str(self) -> &'static str {
        match self {
            UserType::Location => "Location",
            UserType::Company => "Company",
        }
    }
}

/// OAuth 2.0 app credentials (from the GoHighLevel Marketplace developer portal).
#[derive(Clone)]
#[allow(missing_docs)] // fields match the OAuth token request parameters
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: SecretString,
    pub user_type: UserType,
}

impl OAuthConfig {
    /// Build an OAuth config from app credentials.
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        user_type: UserType,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: SecretString::from(client_secret.into()),
            user_type,
        }
    }
}

/// An access/refresh token pair with its expiry instant.
#[derive(Clone)]
pub struct TokenSet {
    access_token: SecretString,
    refresh_token: SecretString,
    expires_at: SystemTime,
}

impl TokenSet {
    /// Build a token set (e.g. from an authorization-code exchange you ran yourself).
    pub fn new(
        access_token: impl Into<String>,
        refresh_token: impl Into<String>,
        expires_at: SystemTime,
    ) -> Self {
        Self {
            access_token: SecretString::from(access_token.into()),
            refresh_token: SecretString::from(refresh_token.into()),
            expires_at,
        }
    }

    /// The current access token (handle with care — this exposes the secret).
    pub fn access_token(&self) -> &str {
        self.access_token.expose_secret()
    }

    /// The current refresh token (handle with care — this exposes the secret).
    pub fn refresh_token(&self) -> &str {
        self.refresh_token.expose_secret()
    }

    /// When the access token expires.
    pub fn expires_at(&self) -> SystemTime {
        self.expires_at
    }

    fn is_fresh(&self) -> bool {
        SystemTime::now() + EXPIRY_SKEW < self.expires_at
    }
}

impl std::fmt::Debug for TokenSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenSet")
            .field("access_token", &"REDACTED")
            .field("refresh_token", &"REDACTED")
            .field("expires_at", &self.expires_at)
            .finish()
    }
}

/// Persistence for OAuth tokens (memory, file, Redis, Postgres, …).
///
/// GoHighLevel rotates the refresh token on every use, so implementations
/// **must** persist what [`TokenStore::save`] hands them or the integration
/// will lose its session on restart.
#[async_trait::async_trait]
pub trait TokenStore: Send + Sync {
    /// Load the most recently saved tokens, if any.
    async fn load(&self) -> Result<Option<TokenSet>>;
    /// Persist a freshly rotated token set. Must be durable before returning.
    async fn save(&self, tokens: TokenSet) -> Result<()>;
}

/// In-memory token store — fine for single-process tools; tokens are lost on restart.
#[derive(Default)]
pub struct MemoryTokenStore {
    tokens: RwLock<Option<TokenSet>>,
}

impl MemoryTokenStore {
    /// Seed the store with an initial token set.
    pub fn new(initial: TokenSet) -> Self {
        Self {
            tokens: RwLock::new(Some(initial)),
        }
    }
}

#[async_trait::async_trait]
impl TokenStore for MemoryTokenStore {
    async fn load(&self) -> Result<Option<TokenSet>> {
        Ok(self.tokens.read().await.clone())
    }

    async fn save(&self, tokens: TokenSet) -> Result<()> {
        *self.tokens.write().await = Some(tokens);
        Ok(())
    }
}

/// OAuth state: cached tokens + single-flight refresh.
#[derive(Clone)]
pub struct OAuthAuth {
    config: OAuthConfig,
    store: Arc<dyn TokenStore>,
    cache: Arc<RwLock<Option<TokenSet>>>,
    refresh_lock: Arc<Mutex<()>>,
}

/// Wire shape of `POST /oauth/token` responses.
#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: u64,
}

impl OAuthAuth {
    fn new(config: OAuthConfig, store: Arc<dyn TokenStore>) -> Self {
        Self {
            config,
            store,
            cache: Arc::new(RwLock::new(None)),
            refresh_lock: Arc::new(Mutex::new(())),
        }
    }

    async fn bearer(&self, http: &reqwest::Client, base_url: &str) -> Result<String> {
        // Fast path: cached and fresh.
        if let Some(tokens) = self.cache.read().await.as_ref() {
            if tokens.is_fresh() {
                return Ok(tokens.access_token().to_owned());
            }
        }

        // Slow path: single-flight refresh with a double-check after acquiring the lock.
        let _guard = self.refresh_lock.lock().await;
        if let Some(tokens) = self.cache.read().await.as_ref() {
            if tokens.is_fresh() {
                return Ok(tokens.access_token().to_owned());
            }
        }

        let current = match self.store.load().await? {
            Some(t) => t,
            None => {
                return Err(Error::Auth(
                    "no OAuth tokens in the token store; complete the install flow first \
                     (exchange the authorization code, then `TokenStore::save` the result)"
                        .into(),
                ))
            }
        };
        if current.is_fresh() {
            let token = current.access_token().to_owned();
            *self.cache.write().await = Some(current);
            return Ok(token);
        }

        tracing::debug!("refreshing GoHighLevel OAuth access token");
        let response = http
            .post(format!("{base_url}/oauth/token"))
            .form(&[
                ("client_id", self.config.client_id.as_str()),
                ("client_secret", self.config.client_secret.expose_secret()),
                ("grant_type", "refresh_token"),
                ("refresh_token", current.refresh_token()),
                ("user_type", self.config.user_type.as_str()),
            ])
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Auth(format!(
                "token refresh failed ({status}): {body}"
            )));
        }

        let parsed: TokenResponse = response
            .json()
            .await
            .map_err(|e| Error::Auth(format!("token refresh returned an unexpected body: {e}")))?;
        let tokens = TokenSet::new(
            parsed.access_token,
            parsed.refresh_token,
            SystemTime::now() + Duration::from_secs(parsed.expires_in),
        );

        // Persist first (refresh tokens are single-use), then cache.
        self.store.save(tokens.clone()).await?;
        let access = tokens.access_token().to_owned();
        *self.cache.write().await = Some(tokens);
        Ok(access)
    }
}
