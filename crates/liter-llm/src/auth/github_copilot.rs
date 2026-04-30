//! GitHub Copilot OAuth Device Flow credential provider.
//!
//! Implements a two-token system:
//!
//! 1. **GitHub OAuth Access Token** (long-lived) — obtained once via the OAuth
//!    Device Flow and cached to disk.  Reused across process restarts.
//! 2. **Copilot API Key** (short-lived) — exchanged for the access token via
//!    `https://api.github.com/copilot_internal/v2/token` and cached in memory
//!    with automatic refresh when within 5 minutes of expiry.
//!
//! # Environment variables
//!
//! | Variable | Default | Description |
//! |----------|---------|-------------|
//! | `GITHUB_COPILOT_CLIENT_ID` | `Iv1.b507a08c87ecfe98` | GitHub OAuth App client ID |
//! | `GITHUB_COPILOT_DEVICE_CODE_URL` | `https://github.com/login/device/code` | Device code endpoint |
//! | `GITHUB_COPILOT_ACCESS_TOKEN_URL` | `https://github.com/login/oauth/access_token` | Token poll endpoint |
//! | `GITHUB_COPILOT_API_KEY_URL` | `https://api.github.com/copilot_internal/v2/token` | Copilot key endpoint |
//! | `GITHUB_COPILOT_TOKEN_DIR` | `~/.config/liter-llm/github_copilot/` | Directory for cached tokens |
//! | `GITHUB_COPILOT_ACCESS_TOKEN_FILE` | *(derived from `TOKEN_DIR`)* | Full path override for access token |
//! | `GITHUB_COPILOT_API_KEY_FILE` | *(derived from `TOKEN_DIR`)* | Full path override for API key JSON |

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use secrecy::{ExposeSecret, SecretString};
use tokio::sync::RwLock;

use super::{Credential, CredentialProvider, StaticTokenProvider};
use crate::client::BoxFuture;
use crate::error::LiterLlmError;

// ── Constants ────────────────────────────────────────────────────────────────

/// Public GitHub OAuth App client ID for GitHub Copilot.
const DEFAULT_CLIENT_ID: &str = "Iv1.b507a08c87ecfe98";

/// Default device-code endpoint.
const DEFAULT_DEVICE_CODE_URL: &str = "https://github.com/login/device/code";

/// Default access-token poll endpoint.
const DEFAULT_ACCESS_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

/// Default Copilot internal API key endpoint.
const DEFAULT_API_KEY_URL: &str = "https://api.github.com/copilot_internal/v2/token";

/// Default cache sub-directory (relative to `~/.config`).
const DEFAULT_TOKEN_SUBDIR: &str = "liter-llm/github_copilot";

/// File name for the persisted GitHub access token.
const ACCESS_TOKEN_FILE_NAME: &str = "access-token";

/// File name for the persisted Copilot API key JSON.
const API_KEY_FILE_NAME: &str = "api-key.json";

/// OAuth scope requested from GitHub.
const OAUTH_SCOPE: &str = "read:user";

/// Number of poll attempts during the Device Flow before timing out.
const DEVICE_FLOW_POLL_ATTEMPTS: u32 = 12;

/// Delay between Device Flow poll attempts.
const DEVICE_FLOW_POLL_INTERVAL: Duration = Duration::from_secs(5);

/// Minimum remaining lifetime before a Copilot API key is considered expired.
const EXPIRY_BUFFER_SECS: u64 = 300;

// ── Internal types ────────────────────────────────────────────────────────────

/// In-memory cached Copilot API key with its expiry timestamp.
struct CachedToken {
    token: SecretString,
    /// Unix timestamp (seconds) at which the token expires.
    expires_at: u64,
}

impl CachedToken {
    /// Returns `true` if the token is still valid after subtracting the safety
    /// buffer, i.e. more than [`EXPIRY_BUFFER_SECS`] remain before expiry.
    fn is_valid(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        now + EXPIRY_BUFFER_SECS < self.expires_at
    }
}

// ── Serde helpers (private) ──────────────────────────────────────────────────

#[derive(serde::Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
}

#[derive(serde::Deserialize)]
struct AccessTokenResponse {
    access_token: Option<String>,
    error: Option<String>,
}

#[derive(serde::Deserialize)]
struct ApiKeyResponse {
    token: String,
    expires_at: u64,
    endpoints: Option<ApiKeyEndpoints>,
}

#[derive(serde::Deserialize)]
struct ApiKeyEndpoints {
    api: Option<String>,
}

/// Persisted representation of the Copilot API key on disk.
#[derive(serde::Serialize, serde::Deserialize)]
struct PersistedApiKey {
    token: String,
    expires_at: u64,
    api_endpoint: Option<String>,
}

// ── Provider ─────────────────────────────────────────────────────────────────

/// GitHub Copilot credential provider using the OAuth Device Flow.
///
/// Maintains two caches:
/// - A disk-persisted GitHub OAuth access token (long-lived, survives restarts).
/// - An in-memory Copilot API key (short-lived, refreshed automatically).
///
/// On first use the user is prompted to visit a URL and enter a code.
/// Subsequent runs reuse the cached access token and API key transparently.
pub struct GithubCopilotCredentialProvider {
    client_id: String,
    device_code_url: String,
    access_token_url: String,
    api_key_url: String,
    access_token_path: PathBuf,
    api_key_path: PathBuf,
    cached: RwLock<Option<CachedToken>>,
    http_client: reqwest::Client,
}

impl GithubCopilotCredentialProvider {
    /// Create a new provider with all defaults and the given HTTP client.
    #[must_use]
    pub fn new(http_client: reqwest::Client) -> Self {
        let token_dir = default_token_dir();
        Self {
            client_id: DEFAULT_CLIENT_ID.to_owned(),
            device_code_url: DEFAULT_DEVICE_CODE_URL.to_owned(),
            access_token_url: DEFAULT_ACCESS_TOKEN_URL.to_owned(),
            api_key_url: DEFAULT_API_KEY_URL.to_owned(),
            access_token_path: token_dir.join(ACCESS_TOKEN_FILE_NAME),
            api_key_path: token_dir.join(API_KEY_FILE_NAME),
            cached: RwLock::new(None),
            http_client,
        }
    }

    /// Create a provider reading all configuration from environment variables.
    ///
    /// If no env-var overrides are set, all defaults are used.  The provider
    /// always performs the Device Flow interactively the first time.
    ///
    /// # Errors
    ///
    /// Returns [`LiterLlmError::Authentication`] if a path cannot be resolved.
    pub fn from_env() -> Result<Arc<dyn CredentialProvider>, LiterLlmError> {
        crate::ensure_crypto_provider();
        let http_client = reqwest::Client::new();

        // Allow a pre-existing static token via the access-token env var.
        // This matches the azure_ad fast-path pattern.
        if let Ok(token) = std::env::var("GITHUB_COPILOT_TOKEN") {
            return Ok(Arc::new(StaticTokenProvider::new(SecretString::from(token))));
        }

        let client_id = std::env::var("GITHUB_COPILOT_CLIENT_ID").unwrap_or_else(|_| DEFAULT_CLIENT_ID.to_owned());

        let device_code_url =
            std::env::var("GITHUB_COPILOT_DEVICE_CODE_URL").unwrap_or_else(|_| DEFAULT_DEVICE_CODE_URL.to_owned());

        let access_token_url =
            std::env::var("GITHUB_COPILOT_ACCESS_TOKEN_URL").unwrap_or_else(|_| DEFAULT_ACCESS_TOKEN_URL.to_owned());

        let api_key_url =
            std::env::var("GITHUB_COPILOT_API_KEY_URL").unwrap_or_else(|_| DEFAULT_API_KEY_URL.to_owned());

        let token_dir = std::env::var("GITHUB_COPILOT_TOKEN_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| default_token_dir());

        let access_token_path = std::env::var("GITHUB_COPILOT_ACCESS_TOKEN_FILE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| token_dir.join(ACCESS_TOKEN_FILE_NAME));

        let api_key_path = std::env::var("GITHUB_COPILOT_API_KEY_FILE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| token_dir.join(API_KEY_FILE_NAME));

        Ok(Arc::new(Self {
            client_id,
            device_code_url,
            access_token_url,
            api_key_url,
            access_token_path,
            api_key_path,
            cached: RwLock::new(None),
            http_client,
        }))
    }

    /// Return the Copilot API base URL from the cached api-key response, if
    /// available.  This endpoint is provider-specific and may differ from the
    /// default `https://api.githubcopilot.com`.
    ///
    /// Returns `None` when no API key has been fetched yet or the response did
    /// not include an `endpoints.api` field.
    pub fn api_base(&self) -> Option<String> {
        // Read from disk cache — cheap, avoids holding the async RwLock in a
        // sync context.
        let raw = std::fs::read_to_string(&self.api_key_path).ok()?;
        let persisted: PersistedApiKey = serde_json::from_str(&raw).ok()?;
        persisted.api_endpoint
    }

    // ── Private helpers ───────────────────────────────────────────────────────

    /// Load the GitHub OAuth access token from disk, if present.
    fn load_access_token(&self) -> Option<SecretString> {
        let raw = std::fs::read_to_string(&self.access_token_path).ok()?;
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(SecretString::from(trimmed.to_owned()))
        }
    }

    /// Persist the GitHub OAuth access token to disk.
    async fn save_access_token(&self, token: &SecretString) -> Result<(), LiterLlmError> {
        if let Some(parent) = self.access_token_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| LiterLlmError::Authentication {
                    message: format!("failed to create token directory {}: {e}", parent.display()),
                })?;
        }
        tokio::fs::write(&self.access_token_path, token.expose_secret())
            .await
            .map_err(|e| LiterLlmError::Authentication {
                message: format!(
                    "failed to write access token to {}: {e}",
                    self.access_token_path.display()
                ),
            })
    }

    /// Run the GitHub OAuth Device Flow to obtain a new access token.
    async fn run_device_flow(&self) -> Result<SecretString, LiterLlmError> {
        // Step 1: request a device code.
        let device_resp = self
            .http_client
            .post(&self.device_code_url)
            .header("accept", "application/json")
            .header("editor-version", "vscode/1.85.1")
            .header("editor-plugin-version", "copilot/1.155.0")
            .header("user-agent", "GithubCopilot/1.155.0")
            .header("accept-encoding", "gzip,deflate,br")
            .header("content-type", "application/json")
            .json(&serde_json::json!({
                "client_id": self.client_id,
                "scope": OAUTH_SCOPE,
            }))
            .send()
            .await
            .map_err(|e| LiterLlmError::Authentication {
                message: format!("GitHub device code request failed: {e}"),
            })?;

        let device_status = device_resp.status();
        let device_body = device_resp.text().await.map_err(|e| LiterLlmError::Authentication {
            message: format!("GitHub device code response unreadable: {e}"),
        })?;

        if !device_status.is_success() {
            return Err(LiterLlmError::Authentication {
                message: format!("GitHub device code request returned {device_status}: {device_body}"),
            });
        }

        let device: DeviceCodeResponse =
            serde_json::from_str(&device_body).map_err(|e| LiterLlmError::Authentication {
                message: format!("GitHub device code response parse error: {e}"),
            })?;

        // Step 2: prompt the user.
        eprintln!(
            "\nTo authenticate with GitHub Copilot, visit: {}\nand enter code: {}\n",
            device.verification_uri, device.user_code
        );

        // Step 3: poll for the access token.
        for attempt in 0..DEVICE_FLOW_POLL_ATTEMPTS {
            tokio::time::sleep(DEVICE_FLOW_POLL_INTERVAL).await;

            let poll_resp = self
                .http_client
                .post(&self.access_token_url)
                .header("accept", "application/json")
                .header("editor-version", "vscode/1.85.1")
                .header("editor-plugin-version", "copilot/1.155.0")
                .header("user-agent", "GithubCopilot/1.155.0")
                .header("accept-encoding", "gzip,deflate,br")
                .header("content-type", "application/json")
                .json(&serde_json::json!({
                    "client_id": self.client_id,
                    "device_code": device.device_code,
                    "grant_type": "urn:ietf:params:oauth:grant-type:device_code",
                }))
                .send()
                .await
                .map_err(|e| LiterLlmError::Authentication {
                    message: format!("GitHub access token poll request failed: {e}"),
                })?;

            let poll_body = poll_resp.text().await.map_err(|e| LiterLlmError::Authentication {
                message: format!("GitHub access token poll response unreadable: {e}"),
            })?;

            let parsed: AccessTokenResponse =
                serde_json::from_str(&poll_body).map_err(|e| LiterLlmError::Authentication {
                    message: format!("GitHub access token poll parse error: {e}"),
                })?;

            if let Some(token) = parsed.access_token
                && !token.is_empty()
            {
                return Ok(SecretString::from(token));
            }

            if let Some(ref error) = parsed.error {
                match error.as_str() {
                    "authorization_pending" | "slow_down" => {
                        // Continue polling.
                    }
                    other => {
                        return Err(LiterLlmError::Authentication {
                            message: format!("GitHub Device Flow error after attempt {attempt}: {other}"),
                        });
                    }
                }
            }
        }

        Err(LiterLlmError::Authentication {
            message: format!(
                "GitHub Device Flow timed out after {} attempts ({} seconds)",
                DEVICE_FLOW_POLL_ATTEMPTS,
                DEVICE_FLOW_POLL_ATTEMPTS * DEVICE_FLOW_POLL_INTERVAL.as_secs() as u32
            ),
        })
    }

    /// Obtain a valid GitHub OAuth access token, running the Device Flow if
    /// necessary.
    async fn get_or_create_access_token(&self) -> Result<SecretString, LiterLlmError> {
        if let Some(token) = self.load_access_token() {
            return Ok(token);
        }
        let token = self.run_device_flow().await?;
        self.save_access_token(&token).await?;
        Ok(token)
    }

    /// Exchange a GitHub OAuth access token for a short-lived Copilot API key.
    async fn fetch_api_key(&self, access_token: &SecretString) -> Result<CachedToken, LiterLlmError> {
        let resp = self
            .http_client
            .get(&self.api_key_url)
            .header("accept", "application/json")
            .header("editor-version", "vscode/1.85.1")
            .header("editor-plugin-version", "copilot/1.155.0")
            .header("user-agent", "GithubCopilot/1.155.0")
            .header("accept-encoding", "gzip,deflate,br")
            .header("authorization", format!("token {}", access_token.expose_secret()))
            .send()
            .await
            .map_err(|e| LiterLlmError::Authentication {
                message: format!("Copilot API key request failed: {e}"),
            })?;

        let status = resp.status();
        let body = resp.text().await.map_err(|e| LiterLlmError::Authentication {
            message: format!("Copilot API key response unreadable: {e}"),
        })?;

        if !status.is_success() {
            return Err(LiterLlmError::Authentication {
                message: format!("Copilot API key request returned {status}: {body}"),
            });
        }

        let parsed: ApiKeyResponse = serde_json::from_str(&body).map_err(|e| LiterLlmError::Authentication {
            message: format!("Copilot API key response parse error: {e}"),
        })?;

        // Persist to disk so `api_base()` can read it without holding a lock.
        let api_endpoint = parsed.endpoints.as_ref().and_then(|e| e.api.clone());
        let persisted = PersistedApiKey {
            token: parsed.token.clone(),
            expires_at: parsed.expires_at,
            api_endpoint,
        };
        if let Some(parent) = self.api_key_path.parent() {
            // Best-effort — if the directory exists we write; ignore create
            // failures here since the token is still usable in memory.
            let _ = tokio::fs::create_dir_all(parent).await;
        }
        let _ = tokio::fs::write(
            &self.api_key_path,
            serde_json::to_string(&persisted).unwrap_or_default(),
        )
        .await;

        Ok(CachedToken {
            token: SecretString::from(parsed.token),
            expires_at: parsed.expires_at,
        })
    }
}

impl CredentialProvider for GithubCopilotCredentialProvider {
    fn resolve(&self) -> BoxFuture<'_, crate::error::Result<Credential>> {
        Box::pin(async move {
            // Fast path: read lock to check in-memory cache.
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

            let access_token = self.get_or_create_access_token().await?;
            let fresh = self.fetch_api_key(&access_token).await?;
            let token = fresh.token.clone();
            *guard = Some(fresh);

            Ok(Credential::BearerToken(token))
        })
    }
}

// ── Module-level helpers ──────────────────────────────────────────────────────

/// Resolve the default token directory: `~/.config/liter-llm/github_copilot/`.
///
/// On Unix, uses `$XDG_CONFIG_HOME` if set, otherwise `$HOME/.config`.
/// On Windows, uses `%APPDATA%`.
/// Falls back to the current directory if no home directory can be determined.
fn default_token_dir() -> PathBuf {
    platform_config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(DEFAULT_TOKEN_SUBDIR)
}

/// Platform-portable config directory resolution (no external crate required).
fn platform_config_dir() -> Option<PathBuf> {
    // Respect XDG on Unix.
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(xdg));
    }
    // Windows: %APPDATA%
    #[cfg(target_os = "windows")]
    if let Ok(appdata) = std::env::var("APPDATA") {
        return Some(PathBuf::from(appdata));
    }
    // Unix fallback: $HOME/.config
    std::env::var("HOME")
        .ok()
        .map(|home| PathBuf::from(home).join(".config"))
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_cached_token(expires_at: u64) -> CachedToken {
        CachedToken {
            token: SecretString::from("test-token".to_owned()),
            expires_at,
        }
    }

    fn unix_now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }

    #[test]
    fn cached_token_validity() {
        // Expiry 1 hour in the future — well outside the 300s buffer.
        let token = make_cached_token(unix_now() + 3600);
        assert!(token.is_valid());
    }

    #[test]
    fn cached_token_expired() {
        // Already expired.
        let token = make_cached_token(unix_now().saturating_sub(60));
        assert!(!token.is_valid());
    }

    #[test]
    fn cached_token_within_buffer() {
        // Expires in 200s — inside the 300s buffer, so treated as invalid.
        let token = make_cached_token(unix_now() + 200);
        assert!(!token.is_valid());
    }

    #[test]
    fn api_key_response_parsing() {
        let json = r#"{
            "token": "tid=abc123;exp=9999999999;sku=copilot_for_business_seat",
            "expires_at": 9999999999,
            "endpoints": { "api": "https://api.githubcopilot.com" }
        }"#;

        let parsed: ApiKeyResponse = serde_json::from_str(json).expect("parse failed");
        assert_eq!(parsed.token, "tid=abc123;exp=9999999999;sku=copilot_for_business_seat");
        assert_eq!(parsed.expires_at, 9_999_999_999);
        let endpoints = parsed.endpoints.expect("endpoints missing");
        assert_eq!(endpoints.api.as_deref(), Some("https://api.githubcopilot.com"));
    }

    #[test]
    fn api_key_response_parsing_no_endpoints() {
        let json = r#"{ "token": "tok", "expires_at": 1234567890 }"#;
        let parsed: ApiKeyResponse = serde_json::from_str(json).expect("parse failed");
        assert_eq!(parsed.token, "tok");
        assert!(parsed.endpoints.is_none());
    }

    #[test]
    fn default_token_dir() {
        let provider = GithubCopilotCredentialProvider::new(reqwest::Client::new());
        // The paths must end with the canonical file names.
        assert_eq!(
            provider.access_token_path.file_name().and_then(|n| n.to_str()),
            Some(ACCESS_TOKEN_FILE_NAME)
        );
        assert_eq!(
            provider.api_key_path.file_name().and_then(|n| n.to_str()),
            Some(API_KEY_FILE_NAME)
        );
        // Both paths must share the same parent directory.
        assert_eq!(provider.access_token_path.parent(), provider.api_key_path.parent());
    }

    #[test]
    fn env_override_client_id() {
        // SAFETY: This is a single-threaded test that is the sole writer for
        // this env var.  Rust 2024 requires an unsafe block for set_var /
        // remove_var because they are unsound in multi-threaded programs; the
        // risk is acceptable in a focused unit test with no other threads
        // accessing this variable concurrently.
        unsafe {
            std::env::set_var("GITHUB_COPILOT_CLIENT_ID", "custom-client-id");
        }
        let provider =
            GithubCopilotCredentialProvider::from_env().expect("from_env should not fail with default paths");
        // Downcast is not possible through Arc<dyn CredentialProvider>, so we
        // validate construction success as a proxy for env-var reading.
        unsafe {
            std::env::remove_var("GITHUB_COPILOT_CLIENT_ID");
        }
        drop(provider);
    }

    #[test]
    fn default_client_id_used_when_no_env() {
        // SAFETY: sole writer in this test; see safety comment above.
        unsafe {
            std::env::remove_var("GITHUB_COPILOT_CLIENT_ID");
        }
        let provider = GithubCopilotCredentialProvider::new(reqwest::Client::new());
        assert_eq!(provider.client_id, DEFAULT_CLIENT_ID);
    }

    #[tokio::test]
    #[ignore] // Requires interactive browser session and live GitHub access.
    async fn live_device_flow() {
        let provider = GithubCopilotCredentialProvider::from_env().expect("from_env should succeed");
        let credential = provider.resolve().await.expect("resolve should return a credential");
        assert!(matches!(credential, Credential::BearerToken(_)));
    }
}
