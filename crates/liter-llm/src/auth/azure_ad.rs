//! Azure AD OAuth2 credential provider (client-credentials flow).
//!
//! Exchanges client credentials for a bearer token via the Microsoft Identity
//! Platform v2.0 token endpoint.  Tokens are cached and refreshed automatically
//! when they are within 5 minutes of expiry.
//!
//! # Environment variables
//!
//! | Variable | Description |
//! |----------|-------------|
//! | `AZURE_TENANT_ID` | Azure AD tenant ID |
//! | `AZURE_CLIENT_ID` | Application (client) ID |
//! | `AZURE_CLIENT_SECRET` | Client secret value |
//! | `AZURE_AD_TOKEN` | Static bearer token (skips OAuth flow) |
//! | `AZURE_AD_SCOPE` | OAuth scope (defaults to `https://cognitiveservices.azure.com/.default`) |

use std::sync::Arc;
use std::time::Instant;

use secrecy::{ExposeSecret, SecretString};
use tokio::sync::RwLock;

use super::{Credential, CredentialProvider, StaticTokenProvider};
use crate::client::BoxFuture;
use crate::error::LiterLlmError;

/// Default OAuth2 scope for Azure Cognitive Services (including Azure OpenAI).
const DEFAULT_SCOPE: &str = "https://cognitiveservices.azure.com/.default";

/// Minimum remaining lifetime before a cached token is considered expired.
const EXPIRY_BUFFER_SECS: u64 = 300;

/// Cached token and its acquisition timestamp + lifetime.
struct CachedToken {
    token: SecretString,
    acquired_at: Instant,
    expires_in_secs: u64,
}

impl CachedToken {
    /// Returns `true` if the token is still valid with the safety buffer.
    fn is_valid(&self) -> bool {
        let elapsed = self.acquired_at.elapsed().as_secs();
        elapsed + EXPIRY_BUFFER_SECS < self.expires_in_secs
    }
}

/// Azure AD OAuth2 credential provider using the client-credentials grant.
///
/// Obtains bearer tokens from `https://login.microsoftonline.com/{tenant}/oauth2/v2.0/token`
/// and caches them until they are within 5 minutes of expiry.
pub struct AzureAdCredentialProvider {
    tenant_id: String,
    client_id: String,
    client_secret: SecretString,
    scope: String,
    cached: RwLock<Option<CachedToken>>,
    http_client: reqwest::Client,
}

impl AzureAdCredentialProvider {
    /// Create a new provider with explicit credentials.
    ///
    /// Uses the default scope `https://cognitiveservices.azure.com/.default`.
    #[must_use]
    pub fn new(tenant_id: impl Into<String>, client_id: impl Into<String>, client_secret: SecretString) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            client_id: client_id.into(),
            client_secret,
            scope: DEFAULT_SCOPE.to_owned(),
            cached: RwLock::new(None),
            http_client: reqwest::Client::new(),
        }
    }

    /// Override the OAuth2 scope (default: `https://cognitiveservices.azure.com/.default`).
    #[must_use]
    pub fn with_scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = scope.into();
        self
    }

    /// Override the HTTP client used for token requests.
    #[must_use]
    pub fn with_http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = client;
        self
    }

    /// Create a provider from environment variables.
    ///
    /// If `AZURE_AD_TOKEN` is set, returns a [`StaticTokenProvider`] instead
    /// (no OAuth flow needed).
    ///
    /// # Errors
    ///
    /// Returns [`LiterLlmError::Authentication`] if required environment
    /// variables are missing.
    pub fn from_env() -> Result<Arc<dyn CredentialProvider>, LiterLlmError> {
        // Fast path: static token from environment.
        if let Ok(token) = std::env::var("AZURE_AD_TOKEN") {
            return Ok(Arc::new(StaticTokenProvider::new(SecretString::from(token))));
        }

        let tenant_id = env_var_required("AZURE_TENANT_ID")?;
        let client_id = env_var_required("AZURE_CLIENT_ID")?;
        let client_secret = SecretString::from(env_var_required("AZURE_CLIENT_SECRET")?);

        let mut provider = Self::new(tenant_id, client_id, client_secret);

        if let Ok(scope) = std::env::var("AZURE_AD_SCOPE") {
            provider.scope = scope;
        }

        Ok(Arc::new(provider))
    }

    /// Exchange client credentials for an access token.
    async fn fetch_token(&self) -> Result<CachedToken, LiterLlmError> {
        let url = format!("https://login.microsoftonline.com/{}/oauth2/v2.0/token", self.tenant_id);

        let resp = self
            .http_client
            .post(&url)
            .form(&[
                ("grant_type", "client_credentials"),
                ("client_id", &self.client_id),
                ("client_secret", self.client_secret.expose_secret()),
                ("scope", &self.scope),
            ])
            .send()
            .await
            .map_err(|e| LiterLlmError::Authentication {
                message: format!("Azure AD token request failed: {e}"),
            })?;

        let status = resp.status();
        let body = resp.text().await.map_err(|e| LiterLlmError::Authentication {
            message: format!("Azure AD token response unreadable: {e}"),
        })?;

        if !status.is_success() {
            return Err(LiterLlmError::Authentication {
                message: format!("Azure AD token request returned {status}: {body}"),
            });
        }

        let parsed: TokenResponse = serde_json::from_str(&body).map_err(|e| LiterLlmError::Authentication {
            message: format!("Azure AD token response parse error: {e}"),
        })?;

        Ok(CachedToken {
            token: SecretString::from(parsed.access_token),
            acquired_at: Instant::now(),
            expires_in_secs: parsed.expires_in,
        })
    }
}

impl CredentialProvider for AzureAdCredentialProvider {
    fn resolve(&self) -> BoxFuture<'_, Credential> {
        Box::pin(async move {
            // Fast path: read lock to check cache.
            {
                let guard = self.cached.read().await;
                if let Some(ref cached) = *guard
                    && cached.is_valid()
                {
                    return Ok(Credential::BearerToken(cached.token.clone()));
                }
            }

            // Slow path: write lock to refresh.
            let mut guard = self.cached.write().await;

            // Double-check after acquiring write lock (another task may have refreshed).
            if let Some(ref cached) = *guard
                && cached.is_valid()
            {
                return Ok(Credential::BearerToken(cached.token.clone()));
            }

            let fresh = self.fetch_token().await?;
            let token = fresh.token.clone();
            *guard = Some(fresh);

            Ok(Credential::BearerToken(token))
        })
    }
}

/// Minimal deserialization of the Azure AD token response.
#[derive(serde::Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

/// Read a required environment variable, returning an auth error if missing.
fn env_var_required(name: &str) -> Result<String, LiterLlmError> {
    std::env::var(name).map_err(|_| LiterLlmError::Authentication {
        message: format!("missing required environment variable: {name}"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cached_token_validity() {
        let cached = CachedToken {
            token: SecretString::from("test-token".to_owned()),
            acquired_at: Instant::now(),
            expires_in_secs: 3600,
        };
        assert!(cached.is_valid());
    }

    #[test]
    fn cached_token_expired() {
        let cached = CachedToken {
            token: SecretString::from("test-token".to_owned()),
            // A token with zero lifetime is immediately expired (no Duration subtraction
            // needed, which avoids panics on Windows where Instant uptime may be < 1h).
            acquired_at: Instant::now(),
            expires_in_secs: 0,
        };
        assert!(!cached.is_valid());
    }

    #[test]
    fn cached_token_within_buffer() {
        let cached = CachedToken {
            token: SecretString::from("test-token".to_owned()),
            // 200s lifetime is within the 300s expiry buffer, so the token is invalid.
            acquired_at: Instant::now(),
            expires_in_secs: 200,
        };
        assert!(!cached.is_valid());
    }

    #[test]
    fn default_scope() {
        let provider = AzureAdCredentialProvider::new("tenant", "client", SecretString::from("secret".to_owned()));
        assert_eq!(provider.scope, DEFAULT_SCOPE);
    }

    #[test]
    fn with_scope_override() {
        let provider = AzureAdCredentialProvider::new("tenant", "client", SecretString::from("secret".to_owned()))
            .with_scope("https://custom.scope/.default");
        assert_eq!(provider.scope, "https://custom.scope/.default");
    }

    #[tokio::test]
    #[ignore] // Requires network access and valid Azure AD credentials.
    async fn live_azure_ad_token_exchange() {
        let Ok(provider) = AzureAdCredentialProvider::from_env() else {
            return; // Skip when Azure AD credentials are not configured.
        };
        let credential = provider.resolve().await.expect("token exchange failed");
        assert!(matches!(credential, Credential::BearerToken(_)));
    }
}
