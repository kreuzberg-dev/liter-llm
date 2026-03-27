//! AWS STS Web Identity credential provider for Bedrock.
//!
//! Exchanges a web identity token (JWT from a file) for temporary AWS
//! credentials via the STS `AssumeRoleWithWebIdentity` API.  This is the
//! standard authentication flow for EKS pods using IAM Roles for Service
//! Accounts (IRSA) and for other OIDC federation scenarios.
//!
//! Credentials are cached and refreshed automatically when they are within
//! 5 minutes of expiry.
//!
//! # Environment variables
//!
//! | Variable | Description |
//! |----------|-------------|
//! | `AWS_ROLE_ARN` | ARN of the IAM role to assume |
//! | `AWS_WEB_IDENTITY_TOKEN_FILE` | Path to a file containing the OIDC JWT |
//! | `AWS_ROLE_SESSION_NAME` | Session name (defaults to `liter-llm-session`) |
//! | `AWS_REGION` or `AWS_DEFAULT_REGION` | AWS region (defaults to `us-east-1`) |

use std::path::PathBuf;
use std::time::Instant;

use secrecy::SecretString;
use tokio::sync::RwLock;

use super::{Credential, CredentialProvider};
use crate::client::BoxFuture;
use crate::error::LiterLlmError;

/// Default session name when `AWS_ROLE_SESSION_NAME` is not set.
const DEFAULT_SESSION_NAME: &str = "liter-llm-session";

/// Default AWS region when neither `AWS_REGION` nor `AWS_DEFAULT_REGION` is set.
const DEFAULT_REGION: &str = "us-east-1";

/// Minimum remaining lifetime before cached credentials are considered expired.
const EXPIRY_BUFFER_SECS: u64 = 300;

/// Default credential duration in seconds (1 hour).
const DEFAULT_DURATION_SECS: u64 = 3600;

/// Cached temporary credentials.
struct CachedCredentials {
    access_key_id: SecretString,
    secret_access_key: SecretString,
    session_token: SecretString,
    acquired_at: Instant,
    expires_in_secs: u64,
}

impl CachedCredentials {
    /// Returns `true` if the credentials are still valid with the safety buffer.
    fn is_valid(&self) -> bool {
        let elapsed = self.acquired_at.elapsed().as_secs();
        elapsed + EXPIRY_BUFFER_SECS < self.expires_in_secs
    }
}

/// AWS STS Web Identity credential provider.
///
/// Reads a JWT from a token file, sends it to the STS
/// `AssumeRoleWithWebIdentity` endpoint, and returns temporary AWS credentials
/// suitable for SigV4 signing.
pub struct WebIdentityCredentialProvider {
    role_arn: String,
    token_file: PathBuf,
    session_name: String,
    region: String,
    cached: RwLock<Option<CachedCredentials>>,
    http_client: reqwest::Client,
}

impl WebIdentityCredentialProvider {
    /// Create a new provider with explicit parameters.
    #[must_use]
    pub fn new(
        role_arn: impl Into<String>,
        token_file: impl Into<PathBuf>,
        session_name: impl Into<String>,
        region: impl Into<String>,
    ) -> Self {
        Self {
            role_arn: role_arn.into(),
            token_file: token_file.into(),
            session_name: session_name.into(),
            region: region.into(),
            cached: RwLock::new(None),
            http_client: reqwest::Client::new(),
        }
    }

    /// Create a provider from standard AWS environment variables.
    ///
    /// # Errors
    ///
    /// Returns [`LiterLlmError::Authentication`] if `AWS_ROLE_ARN` or
    /// `AWS_WEB_IDENTITY_TOKEN_FILE` are not set.
    pub fn from_env() -> Result<Self, LiterLlmError> {
        let role_arn = env_var_required("AWS_ROLE_ARN")?;
        let token_file = env_var_required("AWS_WEB_IDENTITY_TOKEN_FILE")?;

        let session_name = std::env::var("AWS_ROLE_SESSION_NAME").unwrap_or_else(|_| DEFAULT_SESSION_NAME.to_owned());

        let region = std::env::var("AWS_REGION")
            .or_else(|_| std::env::var("AWS_DEFAULT_REGION"))
            .unwrap_or_else(|_| DEFAULT_REGION.to_owned());

        Ok(Self::new(role_arn, token_file, session_name, region))
    }

    /// Override the HTTP client used for STS requests.
    #[must_use]
    pub fn with_http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = client;
        self
    }

    /// Read the web identity token from the token file and exchange it for
    /// temporary AWS credentials.
    async fn fetch_credentials(&self) -> Result<CachedCredentials, LiterLlmError> {
        let token = tokio::fs::read_to_string(&self.token_file)
            .await
            .map_err(|e| LiterLlmError::Authentication {
                message: format!(
                    "failed to read web identity token file {}: {e}",
                    self.token_file.display()
                ),
            })?;
        let token = token.trim();

        let url = format!("https://sts.{}.amazonaws.com/", self.region);

        let resp = self
            .http_client
            .post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&[
                ("Action", "AssumeRoleWithWebIdentity"),
                ("Version", "2011-06-15"),
                ("RoleArn", &self.role_arn),
                ("RoleSessionName", &self.session_name),
                ("WebIdentityToken", token),
                ("DurationSeconds", &DEFAULT_DURATION_SECS.to_string()),
            ])
            .send()
            .await
            .map_err(|e| LiterLlmError::Authentication {
                message: format!("STS AssumeRoleWithWebIdentity request failed: {e}"),
            })?;

        let status = resp.status();
        let body = resp.text().await.map_err(|e| LiterLlmError::Authentication {
            message: format!("STS response unreadable: {e}"),
        })?;

        if !status.is_success() {
            return Err(LiterLlmError::Authentication {
                message: format!("STS AssumeRoleWithWebIdentity returned {status}: {body}"),
            });
        }

        // Parse the XML response to extract credentials.
        let creds = parse_sts_response(&body)?;

        Ok(CachedCredentials {
            access_key_id: SecretString::from(creds.access_key_id),
            secret_access_key: SecretString::from(creds.secret_access_key),
            session_token: SecretString::from(creds.session_token),
            acquired_at: Instant::now(),
            expires_in_secs: DEFAULT_DURATION_SECS,
        })
    }
}

impl CredentialProvider for WebIdentityCredentialProvider {
    fn resolve(&self) -> BoxFuture<'_, Credential> {
        Box::pin(async move {
            // Fast path: read lock to check cache.
            {
                let guard = self.cached.read().await;
                if let Some(ref cached) = *guard
                    && cached.is_valid()
                {
                    return Ok(Credential::AwsCredentials {
                        access_key_id: cached.access_key_id.clone(),
                        secret_access_key: cached.secret_access_key.clone(),
                        session_token: Some(cached.session_token.clone()),
                    });
                }
            }

            // Slow path: write lock to refresh.
            let mut guard = self.cached.write().await;

            // Double-check after acquiring write lock.
            if let Some(ref cached) = *guard
                && cached.is_valid()
            {
                return Ok(Credential::AwsCredentials {
                    access_key_id: cached.access_key_id.clone(),
                    secret_access_key: cached.secret_access_key.clone(),
                    session_token: Some(cached.session_token.clone()),
                });
            }

            let fresh = self.fetch_credentials().await?;
            let credential = Credential::AwsCredentials {
                access_key_id: fresh.access_key_id.clone(),
                secret_access_key: fresh.secret_access_key.clone(),
                session_token: Some(fresh.session_token.clone()),
            };
            *guard = Some(fresh);

            Ok(credential)
        })
    }
}

/// Parsed STS temporary credentials.
#[derive(Debug)]
struct StsCredentials {
    access_key_id: String,
    secret_access_key: String,
    session_token: String,
}

/// Extract credential fields from the STS XML response using simple string
/// matching.  We avoid pulling in a full XML parser for three fixed elements.
fn parse_sts_response(xml: &str) -> Result<StsCredentials, LiterLlmError> {
    let access_key_id = extract_xml_element(xml, "AccessKeyId")?;
    let secret_access_key = extract_xml_element(xml, "SecretAccessKey")?;
    let session_token = extract_xml_element(xml, "SessionToken")?;

    Ok(StsCredentials {
        access_key_id,
        secret_access_key,
        session_token,
    })
}

/// Extract the text content of a simple XML element `<tag>value</tag>`.
fn extract_xml_element(xml: &str, tag: &str) -> Result<String, LiterLlmError> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");

    let start = xml.find(&open).ok_or_else(|| LiterLlmError::Authentication {
        message: format!("STS response missing <{tag}> element"),
    })? + open.len();

    let end = xml[start..].find(&close).ok_or_else(|| LiterLlmError::Authentication {
        message: format!("STS response missing </{tag}> element"),
    })? + start;

    Ok(xml[start..end].to_owned())
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
    fn cached_credentials_validity() {
        let cached = CachedCredentials {
            access_key_id: SecretString::from("AKIA...".to_owned()),
            secret_access_key: SecretString::from("secret".to_owned()),
            session_token: SecretString::from("token".to_owned()),
            acquired_at: Instant::now(),
            expires_in_secs: 3600,
        };
        assert!(cached.is_valid());
    }

    #[test]
    fn cached_credentials_expired() {
        let cached = CachedCredentials {
            access_key_id: SecretString::from("AKIA...".to_owned()),
            secret_access_key: SecretString::from("secret".to_owned()),
            session_token: SecretString::from("token".to_owned()),
            // Zero lifetime is immediately expired; avoids Duration subtraction panic on Windows.
            acquired_at: Instant::now(),
            expires_in_secs: 0,
        };
        assert!(!cached.is_valid());
    }

    #[test]
    fn parse_sts_xml_response() {
        let xml = r#"
        <AssumeRoleWithWebIdentityResponse xmlns="https://sts.amazonaws.com/doc/2011-06-15/">
          <AssumeRoleWithWebIdentityResult>
            <Credentials>
              <AccessKeyId>AKIAIOSFODNN7EXAMPLE</AccessKeyId>
              <SecretAccessKey>wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY</SecretAccessKey>
              <SessionToken>FwoGZXIvYXdzEBYaDGlY...</SessionToken>
              <Expiration>2024-01-01T00:00:00Z</Expiration>
            </Credentials>
          </AssumeRoleWithWebIdentityResult>
        </AssumeRoleWithWebIdentityResponse>
        "#;

        let creds = parse_sts_response(xml).expect("should parse");
        assert_eq!(creds.access_key_id, "AKIAIOSFODNN7EXAMPLE");
        assert_eq!(creds.secret_access_key, "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY");
        assert_eq!(creds.session_token, "FwoGZXIvYXdzEBYaDGlY...");
    }

    #[test]
    fn parse_sts_xml_missing_element() {
        let xml = r"<Response><AccessKeyId>AKIA</AccessKeyId></Response>";
        let err = parse_sts_response(xml).unwrap_err();
        assert!(err.to_string().contains("SecretAccessKey"));
    }

    #[test]
    fn extract_xml_element_success() {
        let xml = "<Root><Foo>bar</Foo></Root>";
        assert_eq!(extract_xml_element(xml, "Foo").expect("should work"), "bar");
    }

    #[test]
    fn extract_xml_element_missing_open() {
        let err = extract_xml_element("<Root></Root>", "Missing").unwrap_err();
        assert!(err.to_string().contains("<Missing>"));
    }

    #[test]
    fn constructor_defaults() {
        let provider = WebIdentityCredentialProvider::new(
            "arn:aws:iam::123456789012:role/TestRole",
            "/var/run/secrets/token",
            "test-session",
            "eu-west-1",
        );
        assert_eq!(provider.role_arn, "arn:aws:iam::123456789012:role/TestRole");
        assert_eq!(provider.session_name, "test-session");
        assert_eq!(provider.region, "eu-west-1");
    }

    #[tokio::test]
    #[ignore] // Requires network access and valid AWS OIDC credentials.
    async fn live_sts_web_identity_exchange() {
        let provider = WebIdentityCredentialProvider::from_env().expect("AWS env vars not set");
        let credential = provider.resolve().await.expect("STS exchange failed");
        assert!(matches!(credential, Credential::AwsCredentials { .. }));
    }
}
