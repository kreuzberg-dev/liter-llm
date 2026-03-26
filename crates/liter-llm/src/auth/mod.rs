#[cfg(feature = "azure-auth")]
pub mod azure_ad;
#[cfg(feature = "bedrock-auth")]
pub mod bedrock_sts;
#[cfg(feature = "vertex-auth")]
pub mod vertex_oauth;

use std::sync::Arc;

use secrecy::SecretString;

use crate::client::BoxFuture;

/// Dynamic credential provider for providers that use token-based auth
/// (Azure AD, Vertex OAuth2) or refreshable credentials (AWS STS).
///
/// Implementations handle caching, refresh, and expiry internally.
/// The client calls `resolve()` before each request when a credential
/// provider is configured.
pub trait CredentialProvider: Send + Sync {
    /// Retrieve a valid credential.
    ///
    /// Implementations should cache credentials and only refresh when
    /// expired or about to expire.
    fn resolve(&self) -> BoxFuture<'_, Credential>;
}

/// Blanket implementation so `Arc<dyn CredentialProvider>` is itself a
/// `CredentialProvider`, making it convenient to share providers across
/// clients.
impl CredentialProvider for Arc<dyn CredentialProvider> {
    fn resolve(&self) -> BoxFuture<'_, Credential> {
        (**self).resolve()
    }
}

/// A resolved credential ready for use in request authentication.
#[derive(Debug, Clone)]
pub enum Credential {
    /// Bearer token (Azure AD, Vertex OAuth2, generic OIDC).
    BearerToken(SecretString),
    /// AWS credentials for SigV4 signing.
    AwsCredentials {
        access_key_id: SecretString,
        secret_access_key: SecretString,
        session_token: Option<SecretString>,
    },
}

/// A static credential provider that always returns the same bearer token.
/// Useful for testing or when tokens are managed externally.
pub struct StaticTokenProvider {
    token: SecretString,
}

impl StaticTokenProvider {
    #[must_use]
    pub fn new(token: SecretString) -> Self {
        Self { token }
    }
}

impl CredentialProvider for StaticTokenProvider {
    fn resolve(&self) -> BoxFuture<'_, Credential> {
        let token = self.token.clone();
        Box::pin(async move { Ok(Credential::BearerToken(token)) })
    }
}
