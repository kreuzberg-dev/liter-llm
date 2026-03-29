//! Google Vertex AI OAuth2 credential provider (service-account JWT flow).
//!
//! Creates a self-signed JWT from a Google Cloud service account key, then
//! exchanges it for an access token via the Google OAuth2 token endpoint.
//! Tokens are cached and refreshed automatically when within 5 minutes of
//! expiry.
//!
//! # Environment variables
//!
//! | Variable | Description |
//! |----------|-------------|
//! | `GOOGLE_APPLICATION_CREDENTIALS` | Path to service account JSON key file |
//! | `VERTEX_AI_SCOPE` | OAuth scope (defaults to `https://www.googleapis.com/auth/cloud-platform`) |

use std::path::Path;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{Algorithm, EncodingKey, Header};
use secrecy::{ExposeSecret, SecretString};
use tokio::sync::RwLock;

use super::{Credential, CredentialProvider};
use crate::client::BoxFuture;
use crate::error::LiterLlmError;

/// Default OAuth2 scope for Google Cloud Platform / Vertex AI.
const DEFAULT_SCOPE: &str = "https://www.googleapis.com/auth/cloud-platform";

/// Google OAuth2 token endpoint.
const TOKEN_ENDPOINT: &str = "https://oauth2.googleapis.com/token";

/// JWT grant type for the token exchange.
const GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:jwt-bearer";

/// JWT lifetime in seconds (Google allows up to 3600).
const JWT_LIFETIME_SECS: u64 = 3600;

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

/// Google Vertex AI OAuth2 credential provider using the service-account
/// JWT assertion flow (two-legged OAuth).
///
/// Signs a JWT with the service account's private key, then exchanges it
/// at `https://oauth2.googleapis.com/token` for a short-lived access token.
pub struct VertexOAuthCredentialProvider {
    service_account_email: String,
    private_key_pem: SecretString,
    scope: String,
    cached: RwLock<Option<CachedToken>>,
    http_client: reqwest::Client,
}

/// Intentionally redacts the private key to prevent accidental exposure.
impl std::fmt::Debug for VertexOAuthCredentialProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VertexOAuthCredentialProvider")
            .field("service_account_email", &self.service_account_email)
            .field("private_key_pem", &"[redacted]")
            .field("scope", &self.scope)
            .finish_non_exhaustive()
    }
}

impl VertexOAuthCredentialProvider {
    /// Create a provider from a parsed service account JSON object.
    ///
    /// Expects `client_email` and `private_key` fields in the JSON value.
    ///
    /// # Errors
    ///
    /// Returns [`LiterLlmError::Authentication`] if required fields are missing.
    pub fn from_service_account_json(json: &serde_json::Value) -> Result<Self, LiterLlmError> {
        let email = json
            .get("client_email")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| LiterLlmError::Authentication {
                message: "service account JSON missing 'client_email' field".into(),
            })?
            .to_owned();

        let key = json
            .get("private_key")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| LiterLlmError::Authentication {
                message: "service account JSON missing 'private_key' field".into(),
            })?
            .to_owned();

        Ok(Self {
            service_account_email: email,
            private_key_pem: SecretString::from(key),
            scope: DEFAULT_SCOPE.to_owned(),
            cached: RwLock::new(None),
            http_client: reqwest::Client::new(),
        })
    }

    /// Create a provider from a service account JSON key file on disk.
    ///
    /// # Errors
    ///
    /// Returns [`LiterLlmError::Authentication`] if the file cannot be read or
    /// parsed, or if required fields are missing.
    pub fn from_key_file(path: &Path) -> Result<Self, LiterLlmError> {
        let contents = std::fs::read_to_string(path).map_err(|e| LiterLlmError::Authentication {
            message: format!("failed to read service account key file {}: {e}", path.display()),
        })?;

        let json: serde_json::Value = serde_json::from_str(&contents).map_err(|e| LiterLlmError::Authentication {
            message: format!("failed to parse service account key file: {e}"),
        })?;

        Self::from_service_account_json(&json)
    }

    /// Create a provider from the `GOOGLE_APPLICATION_CREDENTIALS` environment
    /// variable, which should point to a service account JSON key file.
    ///
    /// # Errors
    ///
    /// Returns [`LiterLlmError::Authentication`] if the environment variable is
    /// not set, the file cannot be read, or it cannot be parsed.
    pub fn from_env() -> Result<Self, LiterLlmError> {
        let path = std::env::var("GOOGLE_APPLICATION_CREDENTIALS").map_err(|_| LiterLlmError::Authentication {
            message: "missing required environment variable: GOOGLE_APPLICATION_CREDENTIALS".into(),
        })?;

        let mut provider = Self::from_key_file(Path::new(&path))?;

        if let Ok(scope) = std::env::var("VERTEX_AI_SCOPE") {
            provider.scope = scope;
        }

        Ok(provider)
    }

    /// Override the OAuth2 scope (default: `https://www.googleapis.com/auth/cloud-platform`).
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

    /// Build a signed JWT assertion and exchange it for an access token.
    async fn fetch_token(&self) -> Result<CachedToken, LiterLlmError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| LiterLlmError::Authentication {
                message: format!("system clock error: {e}"),
            })?
            .as_secs();

        let claims = JwtClaims {
            iss: &self.service_account_email,
            scope: &self.scope,
            aud: TOKEN_ENDPOINT,
            iat: now,
            exp: now + JWT_LIFETIME_SECS,
        };

        let header = Header::new(Algorithm::RS256);
        let encoding_key = EncodingKey::from_rsa_pem(self.private_key_pem.expose_secret().as_bytes()).map_err(|e| {
            LiterLlmError::Authentication {
                message: format!("invalid RSA private key: {e}"),
            }
        })?;

        let assertion =
            jsonwebtoken::encode(&header, &claims, &encoding_key).map_err(|e| LiterLlmError::Authentication {
                message: format!("JWT signing failed: {e}"),
            })?;

        let resp = self
            .http_client
            .post(TOKEN_ENDPOINT)
            .form(&[("grant_type", GRANT_TYPE), ("assertion", &assertion)])
            .send()
            .await
            .map_err(|e| LiterLlmError::Authentication {
                message: format!("Vertex OAuth token request failed: {e}"),
            })?;

        let status = resp.status();
        let body = resp.text().await.map_err(|e| LiterLlmError::Authentication {
            message: format!("Vertex OAuth token response unreadable: {e}"),
        })?;

        if !status.is_success() {
            return Err(LiterLlmError::Authentication {
                message: format!("Vertex OAuth token request returned {status}: {body}"),
            });
        }

        let parsed: TokenResponse = serde_json::from_str(&body).map_err(|e| LiterLlmError::Authentication {
            message: format!("Vertex OAuth token response parse error: {e}"),
        })?;

        Ok(CachedToken {
            token: SecretString::from(parsed.access_token),
            acquired_at: Instant::now(),
            expires_in_secs: parsed.expires_in,
        })
    }
}

impl CredentialProvider for VertexOAuthCredentialProvider {
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

            // Double-check after acquiring write lock.
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

/// JWT claims for the Google OAuth2 service-account assertion.
#[derive(serde::Serialize)]
struct JwtClaims<'a> {
    iss: &'a str,
    scope: &'a str,
    aud: &'a str,
    iat: u64,
    exp: u64,
}

/// Minimal deserialization of the Google OAuth2 token response.
#[derive(serde::Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
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
            // Zero lifetime is immediately expired; avoids Duration subtraction panic on Windows.
            acquired_at: Instant::now(),
            expires_in_secs: 0,
        };
        assert!(!cached.is_valid());
    }

    #[test]
    fn from_service_account_json_valid() {
        let json = serde_json::json!({
            "client_email": "test@project.iam.gserviceaccount.com",
            "private_key": "-----BEGIN TEST RSA KEY-----\nMIIE...\n-----END TEST RSA KEY-----\n"
        });
        let provider = VertexOAuthCredentialProvider::from_service_account_json(&json);
        assert!(provider.is_ok());
        let provider = provider.expect("should succeed");
        assert_eq!(provider.service_account_email, "test@project.iam.gserviceaccount.com");
        assert_eq!(provider.scope, DEFAULT_SCOPE);
    }

    #[test]
    fn from_service_account_json_missing_email() {
        let json = serde_json::json!({
            "private_key": "-----BEGIN TEST RSA KEY-----\nMIIE...\n-----END TEST RSA KEY-----\n"
        });
        let err = VertexOAuthCredentialProvider::from_service_account_json(&json).unwrap_err();
        assert!(err.to_string().contains("client_email"));
    }

    #[test]
    fn from_service_account_json_missing_key() {
        let json = serde_json::json!({
            "client_email": "test@project.iam.gserviceaccount.com"
        });
        let err = VertexOAuthCredentialProvider::from_service_account_json(&json).unwrap_err();
        assert!(err.to_string().contains("private_key"));
    }

    #[test]
    fn with_scope_override() {
        let json = serde_json::json!({
            "client_email": "test@project.iam.gserviceaccount.com",
            "private_key": "-----BEGIN TEST RSA KEY-----\nMIIE...\n-----END TEST RSA KEY-----\n"
        });
        let provider = VertexOAuthCredentialProvider::from_service_account_json(&json)
            .expect("should succeed")
            .with_scope("https://custom.scope");
        assert_eq!(provider.scope, "https://custom.scope");
    }

    #[tokio::test]
    #[ignore] // Requires network access and a valid service account key file.
    async fn live_vertex_oauth_token_exchange() {
        let Ok(provider) = VertexOAuthCredentialProvider::from_env() else {
            return; // Skip when Google credentials are not configured.
        };
        let credential = provider.resolve().await.expect("token exchange failed");
        assert!(matches!(credential, Credential::BearerToken(_)));
    }
}
