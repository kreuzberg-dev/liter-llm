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

/// All errors that can occur when using `liter-llm`.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum LiterLlmError {
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

    /// A catch-all for errors that occur during streaming response processing.
    ///
    /// This variant covers multiple sub-conditions including UTF-8 decoding
    /// failures, CRC/checksum mismatches (AWS EventStream), JSON parse errors
    /// in individual SSE chunks, and buffer overflow conditions.  The `message`
    /// field contains a human-readable description of the specific failure.
    #[error("streaming error: {message}")]
    Streaming { message: String },

    #[error("provider {provider} does not support {endpoint}")]
    EndpointNotSupported { endpoint: String, provider: String },

    #[error("invalid header {name:?}: {reason}")]
    InvalidHeader { name: String, reason: String },

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("budget exceeded: {message}")]
    BudgetExceeded { message: String, model: Option<String> },

    #[error("hook rejected: {message}")]
    HookRejected { message: String },

    /// An internal logic error (e.g. unexpected Tower response variant).
    ///
    /// This should never surface in normal operation — if it does, it
    /// indicates a bug in the library.
    #[error("internal error: {message}")]
    InternalError { message: String },
}

impl LiterLlmError {
    /// Returns `true` for errors that are worth retrying on a different service
    /// or deployment (transient failures).
    ///
    /// Used by [`crate::tower::fallback::FallbackService`] and
    /// [`crate::tower::router::Router`] to decide whether to route to an
    /// alternative endpoint.
    #[must_use]
    pub fn is_transient(&self) -> bool {
        match self {
            Self::RateLimited { .. } | Self::ServiceUnavailable { .. } | Self::Timeout | Self::ServerError { .. } => {
                true
            }
            #[cfg(feature = "native-http")]
            Self::Network(_) => true,
            _ => false,
        }
    }

    /// Return the OpenTelemetry `error.type` string for this error variant.
    ///
    /// Used by the tracing middleware to record the `error.type` span attribute
    /// on failed requests per the GenAI semantic conventions.
    #[must_use]
    pub fn error_type(&self) -> &'static str {
        match self {
            Self::Authentication { .. } => "Authentication",
            Self::RateLimited { .. } => "RateLimited",
            Self::BadRequest { .. } => "BadRequest",
            Self::ContextWindowExceeded { .. } => "ContextWindowExceeded",
            Self::ContentPolicy { .. } => "ContentPolicy",
            Self::NotFound { .. } => "NotFound",
            Self::ServerError { .. } => "ServerError",
            Self::ServiceUnavailable { .. } => "ServiceUnavailable",
            Self::Timeout => "Timeout",
            #[cfg(feature = "native-http")]
            Self::Network(_) => "Network",
            Self::Streaming { .. } => "Streaming",
            Self::EndpointNotSupported { .. } => "EndpointNotSupported",
            Self::InvalidHeader { .. } => "InvalidHeader",
            Self::Serialization(_) => "Serialization",
            Self::BudgetExceeded { .. } => "BudgetExceeded",
            Self::HookRejected { .. } => "HookRejected",
            Self::InternalError { .. } => "InternalError",
        }
    }

    /// Create from an HTTP status code, an API error response body, and an
    /// optional `Retry-After` duration already parsed from the response header.
    ///
    /// The `retry_after` value is forwarded into [`LiterLlmError::RateLimited`]
    /// so callers can honour the server-requested delay without re-parsing the
    /// header.
    pub fn from_status(status: u16, body: &str, retry_after: Option<Duration>) -> Self {
        let parsed = serde_json::from_str::<ErrorResponse>(body).ok();
        let code = parsed.as_ref().and_then(|r| r.error.code.clone());
        let message = parsed.map(|r| r.error.message).unwrap_or_else(|| body.to_string());

        match status {
            401 | 403 => Self::Authentication { message },
            429 => Self::RateLimited { message, retry_after },
            400 | 422 => {
                // Check the structured `code` field first — it is more reliable
                // than substring matching on the human-readable message.
                if code.as_deref() == Some("context_length_exceeded") {
                    Self::ContextWindowExceeded { message }
                } else if code.as_deref() == Some("content_policy_violation")
                    || code.as_deref() == Some("content_filter")
                {
                    Self::ContentPolicy { message }
                }
                // Fall back to message-based heuristics for providers that do not
                // populate the `code` field.
                else if message.contains("context_length_exceeded")
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
            405 | 413 => Self::BadRequest { message },
            408 => Self::Timeout,
            500 => Self::ServerError { message },
            502..=504 => Self::ServiceUnavailable { message },
            // Map remaining 4xx codes to BadRequest (client errors) and
            // everything else (5xx, unknown) to ServerError.
            400..=499 => Self::BadRequest { message },
            _ => Self::ServerError { message },
        }
    }
}

pub type Result<T> = std::result::Result<T, LiterLlmError>;
