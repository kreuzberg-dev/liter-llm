---
description: "Configure Azure AD, AWS Bedrock STS, and Vertex AI OAuth2 authentication for liter-llm."
---

# Authentication

liter-llm supports three enterprise credential flows in addition to plain API keys: Azure AD client credentials, AWS STS web identity (for Bedrock), and Google Vertex AI service-account OAuth2. Each provider implements the `CredentialProvider` trait, which resolves a fresh credential before each request and caches it until five minutes before expiry.

Feature flags gate each provider at compile time:

| Feature flag   | Provider         | Credential type                              |
| -------------- | ---------------- | -------------------------------------------- |
| `azure-auth`   | Azure OpenAI     | Bearer token (OAuth2 client credentials)     |
| `bedrock-auth` | AWS Bedrock      | AWS SigV4 credentials (STS web identity)     |
| `vertex-auth`  | Google Vertex AI | Bearer token (service-account JWT assertion) |

Enable the relevant flag when adding liter-llm to `Cargo.toml`:

```toml
[dependencies]
liter-llm = { version = "...", features = ["azure-auth", "bedrock-auth", "vertex-auth"] }
```

## Credential resolution

The `CredentialProvider::resolve()` method returns a `Credential` enum:

```rust
pub enum Credential {
    BearerToken(SecretString),
    AwsCredentials {
        access_key_id: SecretString,
        secret_access_key: SecretString,
        session_token: Option<SecretString>,
    },
}
```

All three built-in providers implement read-lock caching with a write-lock refresh. Concurrent callers never trigger redundant token requests.

## Azure AD (client credentials)

`AzureAdCredentialProvider` exchanges a client secret for a bearer token via the Microsoft Identity Platform v2.0 token endpoint. Tokens are cached and refreshed five minutes before expiry.

**Token endpoint:** `https://login.microsoftonline.com/{tenant}/oauth2/v2.0/token`

**Default scope:** `https://cognitiveservices.azure.com/.default` (Azure OpenAI)

### Environment variables

| Variable              | Required | Description                                                               |
| --------------------- | -------- | ------------------------------------------------------------------------- |
| `AZURE_TENANT_ID`     | yes      | Azure AD tenant ID.                                                       |
| `AZURE_CLIENT_ID`     | yes      | Application (client) ID.                                                  |
| `AZURE_CLIENT_SECRET` | yes      | Client secret value.                                                      |
| `AZURE_AD_TOKEN`      | no       | Static bearer token. When set, skips the OAuth flow entirely.             |
| `AZURE_AD_SCOPE`      | no       | OAuth2 scope. Defaults to `https://cognitiveservices.azure.com/.default`. |

`AZURE_AD_TOKEN` takes precedence. If it is set, none of the other three variables are read.

### Usage

```rust
use liter_llm::auth::azure_ad::AzureAdCredentialProvider;

// From environment variables:
let provider = AzureAdCredentialProvider::from_env()?;

// Explicit credentials:
use secrecy::SecretString;
let provider = AzureAdCredentialProvider::new(
    "my-tenant-id",
    "my-client-id",
    SecretString::from("my-client-secret".to_owned()),
);

// Custom scope:
let provider = AzureAdCredentialProvider::new(tenant, client_id, secret)
    .with_scope("https://custom.scope/.default");
```

Set the provider on the client config and point `base_url` at your Azure OpenAI deployment endpoint:

```rust
use liter_llm::{ClientConfig, client::DefaultClient};

let config = ClientConfig::builder()
    .base_url("https://<resource>.openai.azure.com/openai/deployments/<deployment>")
    .credential_provider(provider)
    .build()?;

let client = DefaultClient::new(config, None)?;
```

## AWS Bedrock (STS web identity)

`WebIdentityCredentialProvider` reads an OIDC JWT from a file, calls the STS `AssumeRoleWithWebIdentity` endpoint, and returns temporary AWS credentials for SigV4 signing. This is the standard flow for EKS pods using IAM Roles for Service Accounts (IRSA).

Temporary credentials have a default lifetime of 3,600 seconds and are refreshed five minutes before expiry.

!!! note "SigV4 signing"
SigV4 request signing is handled by the Bedrock provider crate (`crates/liter-llm/src/provider/bedrock.rs`), not by this credential provider. `WebIdentityCredentialProvider` only supplies the `access_key_id`, `secret_access_key`, and `session_token`.

### Environment variables

| Variable                      | Required | Description                                                       |
| ----------------------------- | -------- | ----------------------------------------------------------------- |
| `AWS_ROLE_ARN`                | yes      | ARN of the IAM role to assume.                                    |
| `AWS_WEB_IDENTITY_TOKEN_FILE` | yes      | Path to the file containing the OIDC JWT.                         |
| `AWS_ROLE_SESSION_NAME`       | no       | Session name. Defaults to `liter-llm-session`.                    |
| `AWS_REGION`                  | no       | AWS region. Falls back to `AWS_DEFAULT_REGION`, then `us-east-1`. |
| `AWS_DEFAULT_REGION`          | no       | Fallback region when `AWS_REGION` is not set.                     |

### Usage

```rust
use liter_llm::auth::bedrock_sts::WebIdentityCredentialProvider;

// From standard AWS environment variables (typical for EKS IRSA):
let provider = WebIdentityCredentialProvider::from_env()?;

// Explicit parameters:
let provider = WebIdentityCredentialProvider::new(
    "arn:aws:iam::123456789012:role/MyRole",
    "/var/run/secrets/token",
    "my-session",
    "us-east-1",
);
```

```rust
use liter_llm::{ClientConfig, client::DefaultClient};

let config = ClientConfig::builder()
    .base_url("https://bedrock-runtime.us-east-1.amazonaws.com")
    .credential_provider(provider)
    .build()?;

let client = DefaultClient::new(config, None)?;
```

## Google Vertex AI (service-account JWT)

`VertexOAuthCredentialProvider` creates a self-signed RS256 JWT from a Google service account key, then exchanges it at `https://oauth2.googleapis.com/token` for an access token (two-legged OAuth / JWT assertion flow). Tokens have a one-hour lifetime and are refreshed five minutes before expiry.

**Default scope:** `https://www.googleapis.com/auth/cloud-platform`

### Environment variables

| Variable                         | Required | Description                                                                 |
| -------------------------------- | -------- | --------------------------------------------------------------------------- |
| `GOOGLE_APPLICATION_CREDENTIALS` | yes      | Path to the service account JSON key file.                                  |
| `VERTEX_AI_SCOPE`                | no       | OAuth2 scope. Defaults to `https://www.googleapis.com/auth/cloud-platform`. |

### Usage

```rust
use liter_llm::auth::vertex_oauth::VertexOAuthCredentialProvider;

// From environment variable (standard ADC path):
let provider = VertexOAuthCredentialProvider::from_env()?;

// From a key file path directly:
use std::path::Path;
let provider = VertexOAuthCredentialProvider::from_key_file(
    Path::new("/path/to/service-account.json")
)?;

// From an already-parsed JSON value:
let json: serde_json::Value = serde_json::from_str(&key_file_contents)?;
let provider = VertexOAuthCredentialProvider::from_service_account_json(&json)?;
```

```rust
use liter_llm::{ClientConfig, client::DefaultClient};

let config = ClientConfig::builder()
    .base_url("https://us-central1-aiplatform.googleapis.com/v1/projects/my-project/locations/us-central1/publishers/google")
    .credential_provider(provider)
    .build()?;

let client = DefaultClient::new(config, None)?;
```

## Static token provider

For cases where token management is handled externally (short-lived tokens injected by a sidecar, tokens fetched from a secrets manager), use `StaticTokenProvider`:

```rust
use liter_llm::auth::{StaticTokenProvider, CredentialProvider};
use secrecy::SecretString;

let provider = StaticTokenProvider::new(SecretString::from(token_string));
```

`StaticTokenProvider` always returns the same bearer token without any caching or refresh logic.

## Proxy configuration

When using the proxy server, credential providers are configured per `[[models]]` entry. The proxy's built-in providers read the same environment variables listed above; no Rust code is required.

```toml
[[models]]
name = "gpt-4o"
provider_model = "azure/gpt-4o"
api_key = "${AZURE_AD_TOKEN}"   # static token

[[models]]
name = "claude-3-opus"
provider_model = "bedrock/anthropic.claude-3-opus-20240229-v1:0"
# AWS credentials read from AWS_ROLE_ARN + AWS_WEB_IDENTITY_TOKEN_FILE

[[models]]
name = "gemini-1.5-pro"
provider_model = "vertex/google/gemini-1.5-pro-001"
# Google credentials read from GOOGLE_APPLICATION_CREDENTIALS
```

See [Proxy Configuration](../server/proxy-configuration.md) for the full `[[models]]` field reference.
