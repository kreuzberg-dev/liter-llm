use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Error response from an OpenAI-compatible API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ApiError,
}

/// Inner error object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    #[serde(default)]
    pub param: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
}

/// All errors that can occur when using `liter-lm`.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum LiterLmError {
    #[error("authentication failed: {message}")]
    Authentication { message: String },

    #[error("rate limited: {message}")]
    RateLimited {
        message: String,
        retry_after: Option<Duration>,
    },

    #[error("bad request: {message}")]
    BadRequest { message: String },

    #[error("context window exceeded: {message}")]
    ContextWindowExceeded { message: String },

    #[error("content policy violation: {message}")]
    ContentPolicy { message: String },

    #[error("not found: {message}")]
    NotFound { message: String },

    #[error("server error: {message}")]
    ServerError { message: String },

    #[error("service unavailable: {message}")]
    ServiceUnavailable { message: String },

    #[error("request timeout")]
    Timeout,

    #[cfg(feature = "native-http")]
    #[error(transparent)]
    Network(#[from] reqwest::Error),

    #[error("streaming error: {message}")]
    Streaming { message: String },

    #[error("provider {provider} does not support {endpoint}")]
    EndpointNotSupported { endpoint: String, provider: String },

    #[error("invalid header {name:?}: {reason}")]
    InvalidHeader { name: String, reason: String },

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl LiterLmError {
    /// Create from an HTTP status code, an API error response body, and an
    /// optional `Retry-After` duration already parsed from the response header.
    ///
    /// The `retry_after` value is forwarded into [`LiterLmError::RateLimited`]
    /// so callers can honour the server-requested delay without re-parsing the
    /// header.
    pub fn from_status(status: u16, body: &str, retry_after: Option<Duration>) -> Self {
        let message = serde_json::from_str::<ErrorResponse>(body)
            .map(|r| r.error.message)
            .unwrap_or_else(|_| body.to_string());

        match status {
            401 | 403 => Self::Authentication { message },
            429 => Self::RateLimited { message, retry_after },
            400 => {
                if message.contains("context_length_exceeded")
                    || message.contains("context window")
                    || message.contains("maximum context length")
                {
                    Self::ContextWindowExceeded { message }
                } else if message.contains("content_policy") || message.contains("content_filter") {
                    Self::ContentPolicy { message }
                } else {
                    Self::BadRequest { message }
                }
            }
            404 => Self::NotFound { message },
            500 => Self::ServerError { message },
            502..=504 => Self::ServiceUnavailable { message },
            _ => Self::ServerError { message },
        }
    }
}

pub type Result<T> = std::result::Result<T, LiterLmError>;
