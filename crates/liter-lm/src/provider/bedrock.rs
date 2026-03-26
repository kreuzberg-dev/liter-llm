use std::borrow::Cow;

#[cfg(feature = "bedrock")]
use crate::error::LiterLmError;
use crate::error::Result;
use crate::provider::Provider;

/// Default AWS region for Bedrock when none is specified.
const DEFAULT_REGION: &str = "us-east-1";

/// AWS Bedrock provider.
///
/// Differences from the OpenAI-compatible baseline:
/// - Routes `bedrock/` prefixed model names to the Bedrock runtime endpoint.
/// - The model prefix is stripped before the model ID is sent in the request.
/// - When the `bedrock` feature is enabled, every request is signed with
///   AWS Signature Version 4 using credentials from the environment
///   (`AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, `AWS_SESSION_TOKEN`).
/// - When the `bedrock` feature is disabled, the provider is usable with a
///   `base_url` override (e.g. in tests against a mock server) without any
///   signing.
///
/// # Region resolution
///
/// The region is resolved in priority order:
/// 1. Explicit value passed to [`BedrockProvider::with_region`].
/// 2. `AWS_DEFAULT_REGION` environment variable.
/// 3. `AWS_REGION` environment variable.
/// 4. Hard-coded default: `us-east-1`.
///
/// # Configuration
///
/// ```rust,ignore
/// let config = ClientConfigBuilder::new("unused-for-sigv4")
///     .build();
/// let client = DefaultClient::new(config, Some("bedrock/anthropic.claude-3-sonnet-20240229-v1:0"))?;
/// ```
pub struct BedrockProvider {
    #[allow(dead_code)] // used by region() accessor and in sigv4_sign
    region: String,
    /// Cached base URL: `https://bedrock-runtime.{region}.amazonaws.com`.
    base_url: String,
}

impl BedrockProvider {
    /// Construct with the given AWS region.
    #[must_use]
    pub fn new(region: impl Into<String>) -> Self {
        let region = region.into();
        let base_url = format!("https://bedrock-runtime.{region}.amazonaws.com");
        Self { region, base_url }
    }

    /// Construct using region from the environment, falling back to `us-east-1`.
    ///
    /// Reads `AWS_DEFAULT_REGION` then `AWS_REGION`.
    #[must_use]
    pub fn from_env() -> Self {
        let region = std::env::var("AWS_DEFAULT_REGION")
            .or_else(|_| std::env::var("AWS_REGION"))
            .unwrap_or_else(|_| DEFAULT_REGION.to_owned());
        Self::new(region)
    }

    /// Return the AWS region this provider is configured for.
    #[must_use]
    #[allow(dead_code)] // useful for consumers of the library
    pub fn region(&self) -> &str {
        &self.region
    }
}

impl Provider for BedrockProvider {
    fn name(&self) -> &str {
        "bedrock"
    }

    /// Base URL for the Bedrock runtime service.
    ///
    /// When a `base_url` override is set in [`ClientConfig`] (as in tests),
    /// the override takes precedence and this value is never used.
    fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Bedrock uses SigV4 signing rather than a static authorization header.
    ///
    /// Returns `None` so the HTTP layer skips adding an `Authorization` header.
    /// Actual signing headers are injected by [`BedrockProvider::signing_headers`]
    /// when the `bedrock` feature is enabled.
    fn auth_header<'a>(&'a self, _api_key: &'a str) -> Option<(Cow<'static, str>, Cow<'a, str>)> {
        None
    }

    fn matches_model(&self, model: &str) -> bool {
        model.starts_with("bedrock/")
    }

    fn strip_model_prefix<'m>(&self, model: &'m str) -> &'m str {
        model.strip_prefix("bedrock/").unwrap_or(model)
    }

    /// Validate that the provider is usable in the current environment.
    ///
    /// When the `bedrock` feature is enabled, checks that AWS credentials are
    /// available in the environment (`AWS_ACCESS_KEY_ID` at minimum).  Without
    /// credentials, every real Bedrock request will be rejected with a 403.
    ///
    /// When the `bedrock` feature is disabled (e.g. in tests with `base_url`
    /// override), validation is skipped so callers can connect to a mock server
    /// without real AWS credentials.
    fn validate(&self) -> Result<()> {
        #[cfg(feature = "bedrock")]
        {
            if std::env::var("AWS_ACCESS_KEY_ID").is_err() {
                return Err(LiterLmError::BadRequest {
                    message: "AWS Bedrock requires AWS credentials. \
                              Set AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY (and optionally \
                              AWS_SESSION_TOKEN) in the environment."
                        .into(),
                });
            }
        }
        Ok(())
    }

    /// Compute AWS SigV4 signing headers for the request.
    ///
    /// When the `bedrock` feature is enabled, derives the `Authorization`,
    /// `x-amz-date`, and (when a session token is present) `x-amz-security-token`
    /// headers from the current request parameters and AWS credentials.
    ///
    /// When the `bedrock` feature is disabled, returns an empty vector so
    /// requests work against override base-URLs (e.g. mock servers in tests).
    fn signing_headers(&self, method: &str, url: &str, body: &[u8]) -> Vec<(String, String)> {
        #[cfg(feature = "bedrock")]
        {
            sigv4_sign(method, url, body, &self.region).unwrap_or_default()
        }

        #[cfg(not(feature = "bedrock"))]
        {
            let _ = (method, url, body);
            vec![]
        }
    }
}

/// Compute AWS SigV4 signing headers using the `aws-sigv4` crate.
///
/// Reads credentials from the standard AWS environment variables:
/// - `AWS_ACCESS_KEY_ID` (required)
/// - `AWS_SECRET_ACCESS_KEY` (required)
/// - `AWS_SESSION_TOKEN` (optional, for temporary credentials)
///
/// Returns a vector of `(header-name, header-value)` pairs to inject into the
/// outgoing HTTP request.
#[cfg(feature = "bedrock")]
fn sigv4_sign(method: &str, url: &str, body: &[u8], region: &str) -> Result<Vec<(String, String)>> {
    use aws_credential_types::Credentials;
    use aws_sigv4::http_request::{SignableBody, SignableRequest, SigningSettings, sign};
    use aws_sigv4::sign::v4::SigningParams;

    let access_key = std::env::var("AWS_ACCESS_KEY_ID").map_err(|_| LiterLmError::BadRequest {
        message: "AWS_ACCESS_KEY_ID environment variable is required for Bedrock requests".into(),
    })?;
    let secret_key = std::env::var("AWS_SECRET_ACCESS_KEY").map_err(|_| LiterLmError::BadRequest {
        message: "AWS_SECRET_ACCESS_KEY environment variable is required for Bedrock requests".into(),
    })?;
    let session_token = std::env::var("AWS_SESSION_TOKEN").ok();

    let credentials = Credentials::new(
        access_key,
        secret_key,
        session_token,
        None, // expiry
        "env",
    );

    let identity = credentials.into();

    let signing_settings = SigningSettings::default();
    let now = std::time::SystemTime::now();

    let params = SigningParams::builder()
        .identity(&identity)
        .region(region)
        .name("bedrock")
        .time(now)
        .settings(signing_settings)
        .build()
        .map_err(|e| LiterLmError::BadRequest {
            message: format!("failed to build SigV4 signing params: {e}"),
        })?;

    // Build a signable request from the method, URL, and body.
    let signable = SignableRequest::new(
        method,
        url,
        std::iter::empty::<(&str, &str)>(),
        SignableBody::Bytes(body),
    )
    .map_err(|e| LiterLmError::BadRequest {
        message: format!("failed to create signable request: {e}"),
    })?;

    let signing_output = sign(signable, &params.into()).map_err(|e| LiterLmError::BadRequest {
        message: format!("SigV4 signing failed: {e}"),
    })?;

    let instructions = signing_output.output();
    let signed_headers: Vec<(String, String)> = instructions
        .headers()
        .map(|(name, value)| (name.to_owned(), value.to_owned()))
        .collect();

    Ok(signed_headers)
}
