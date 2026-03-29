use std::fmt;
use std::time::Duration;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use liter_llm::error::{ApiError, ErrorResponse, LiterLlmError};

/// An HTTP-aware error that serialises to an OpenAI-compatible JSON body.
///
/// `ProxyError` carries the HTTP status code, the structured [`ErrorResponse`]
/// payload, and an optional `Retry-After` duration so that [`IntoResponse`] can
/// produce the correct wire representation — including headers — in a single
/// step.
#[derive(Debug)]
pub struct ProxyError {
    status: StatusCode,
    body: ErrorResponse,
    retry_after: Option<Duration>,
}

impl ProxyError {
    /// Create a `ProxyError` from a status code and an error type / message
    /// pair.
    fn new(status: StatusCode, error_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            status,
            body: ErrorResponse {
                error: ApiError {
                    message: message.into(),
                    error_type: error_type.into(),
                    param: None,
                    code: None,
                },
            },
            retry_after: None,
        }
    }

    /// 401 Unauthorized.
    pub fn authentication(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, "Authentication", message)
    }

    /// 404 Not Found.
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, "NotFound", message)
    }

    /// 400 Bad Request.
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, "BadRequest", message)
    }

    /// 500 Internal Server Error.
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "InternalError", message)
    }

    /// 503 Service Unavailable.
    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::new(StatusCode::SERVICE_UNAVAILABLE, "ServiceUnavailable", message)
    }

    /// 403 Forbidden.
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, "Forbidden", message)
    }

    /// 429 Too Many Requests.
    pub fn rate_limited(message: impl Into<String>) -> Self {
        Self::new(StatusCode::TOO_MANY_REQUESTS, "RateLimited", message)
    }
}

impl fmt::Display for ProxyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.body.error.message)
    }
}

impl std::error::Error for ProxyError {}

impl IntoResponse for ProxyError {
    fn into_response(self) -> Response {
        let mut response = (self.status, Json(self.body)).into_response();
        if let Some(duration) = self.retry_after
            && let Ok(value) = duration.as_secs().to_string().parse()
        {
            response.headers_mut().insert("retry-after", value);
        }
        response
    }
}

impl From<LiterLlmError> for ProxyError {
    fn from(err: LiterLlmError) -> Self {
        let error_type = err.error_type().to_owned();
        let message = err.to_string();

        // Extract retry_after before we lose access to the variant fields.
        let retry_after = if let LiterLlmError::RateLimited { retry_after, .. } = &err {
            *retry_after
        } else {
            None
        };

        let status = match &err {
            LiterLlmError::Authentication { .. } => StatusCode::UNAUTHORIZED,
            LiterLlmError::RateLimited { .. } => StatusCode::TOO_MANY_REQUESTS,
            LiterLlmError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            LiterLlmError::ContextWindowExceeded { .. } => StatusCode::BAD_REQUEST,
            LiterLlmError::ContentPolicy { .. } => StatusCode::BAD_REQUEST,
            LiterLlmError::NotFound { .. } => StatusCode::NOT_FOUND,
            LiterLlmError::BudgetExceeded { .. } => StatusCode::TOO_MANY_REQUESTS,
            LiterLlmError::HookRejected { .. } => StatusCode::FORBIDDEN,
            LiterLlmError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            LiterLlmError::ServiceUnavailable { .. } => StatusCode::SERVICE_UNAVAILABLE,
            LiterLlmError::ServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            LiterLlmError::Network(_) => StatusCode::BAD_GATEWAY,
            LiterLlmError::Streaming { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            LiterLlmError::EndpointNotSupported { .. } => StatusCode::NOT_IMPLEMENTED,
            LiterLlmError::InvalidHeader { .. } => StatusCode::BAD_REQUEST,
            LiterLlmError::Serialization(_) => StatusCode::BAD_REQUEST,
            LiterLlmError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            // LiterLlmError is #[non_exhaustive]; treat unknown future variants
            // as internal server errors.
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status,
            body: ErrorResponse {
                error: ApiError {
                    message,
                    error_type,
                    param: None,
                    code: None,
                },
            },
            retry_after,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use axum::body::Body;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use http_body_util::BodyExt;
    use liter_llm::error::{ErrorResponse, LiterLlmError};

    use super::ProxyError;

    /// Helper: convert a `ProxyError` into a response and extract status + JSON
    /// body.
    async fn extract(err: ProxyError) -> (StatusCode, ErrorResponse) {
        let response = err.into_response();
        let status = response.status();
        let bytes = Body::new(response.into_body()).collect().await.unwrap().to_bytes();
        let body: ErrorResponse = serde_json::from_slice(&bytes).unwrap();
        (status, body)
    }

    // ── Variant -> HTTP status mapping ───────────────────────────────────

    #[tokio::test]
    async fn authentication_maps_to_401() {
        let err: ProxyError = LiterLlmError::Authentication {
            message: "bad key".into(),
        }
        .into();
        let (status, body) = extract(err).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(body.error.error_type, "Authentication");
    }

    #[tokio::test]
    async fn rate_limited_maps_to_429() {
        let err: ProxyError = LiterLlmError::RateLimited {
            message: "slow down".into(),
            retry_after: None,
        }
        .into();
        let (status, _) = extract(err).await;
        assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    }

    #[tokio::test]
    async fn bad_request_maps_to_400() {
        let err: ProxyError = LiterLlmError::BadRequest {
            message: "invalid".into(),
        }
        .into();
        let (status, _) = extract(err).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn context_window_exceeded_maps_to_400() {
        let err: ProxyError = LiterLlmError::ContextWindowExceeded {
            message: "too long".into(),
        }
        .into();
        let (status, body) = extract(err).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(body.error.error_type, "ContextWindowExceeded");
    }

    #[tokio::test]
    async fn content_policy_maps_to_400() {
        let err: ProxyError = LiterLlmError::ContentPolicy {
            message: "violation".into(),
        }
        .into();
        let (status, _) = extract(err).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn not_found_maps_to_404() {
        let err: ProxyError = LiterLlmError::NotFound { message: "gone".into() }.into();
        let (status, _) = extract(err).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn budget_exceeded_maps_to_429() {
        let err: ProxyError = LiterLlmError::BudgetExceeded {
            message: "over budget".into(),
            model: None,
        }
        .into();
        let (status, _) = extract(err).await;
        assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    }

    #[tokio::test]
    async fn hook_rejected_maps_to_403() {
        let err: ProxyError = LiterLlmError::HookRejected {
            message: "denied".into(),
        }
        .into();
        let (status, _) = extract(err).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn timeout_maps_to_504() {
        let err: ProxyError = LiterLlmError::Timeout.into();
        let (status, _) = extract(err).await;
        assert_eq!(status, StatusCode::GATEWAY_TIMEOUT);
    }

    #[tokio::test]
    async fn service_unavailable_maps_to_503() {
        let err: ProxyError = LiterLlmError::ServiceUnavailable { message: "down".into() }.into();
        let (status, _) = extract(err).await;
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn server_error_maps_to_500() {
        let err: ProxyError = LiterLlmError::ServerError { message: "boom".into() }.into();
        let (status, _) = extract(err).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn streaming_maps_to_500() {
        let err: ProxyError = LiterLlmError::Streaming {
            message: "broke".into(),
        }
        .into();
        let (status, _) = extract(err).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn endpoint_not_supported_maps_to_501() {
        let err: ProxyError = LiterLlmError::EndpointNotSupported {
            endpoint: "images".into(),
            provider: "test".into(),
        }
        .into();
        let (status, _) = extract(err).await;
        assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
    }

    #[tokio::test]
    async fn invalid_header_maps_to_400() {
        let err: ProxyError = LiterLlmError::InvalidHeader {
            name: "x-bad".into(),
            reason: "nope".into(),
        }
        .into();
        let (status, _) = extract(err).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn serialization_maps_to_400() {
        let json_err = serde_json::from_str::<String>("not json").unwrap_err();
        let err: ProxyError = LiterLlmError::Serialization(json_err).into();
        let (status, _) = extract(err).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn internal_error_maps_to_500() {
        let err: ProxyError = LiterLlmError::InternalError { message: "bug".into() }.into();
        let (status, _) = extract(err).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    // ── IntoResponse produces valid JSON ─────────────────────────────────

    #[tokio::test]
    async fn into_response_produces_valid_json_with_correct_fields() {
        let err: ProxyError = LiterLlmError::Authentication {
            message: "invalid api key".into(),
        }
        .into();
        let (status, body) = extract(err).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(body.error.error_type, "Authentication");
        assert!(body.error.message.contains("invalid api key"));
    }

    // ── Constructor methods ──────────────────────────────────────────────

    #[tokio::test]
    async fn constructor_authentication() {
        let (status, body) = extract(ProxyError::authentication("no token")).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(body.error.error_type, "Authentication");
        assert_eq!(body.error.message, "no token");
    }

    #[tokio::test]
    async fn constructor_not_found() {
        let (status, _) = extract(ProxyError::not_found("missing")).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn constructor_bad_request() {
        let (status, _) = extract(ProxyError::bad_request("oops")).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn constructor_internal() {
        let (status, _) = extract(ProxyError::internal("bug")).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn constructor_forbidden() {
        let (status, _) = extract(ProxyError::forbidden("nope")).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn constructor_rate_limited() {
        let (status, _) = extract(ProxyError::rate_limited("slow")).await;
        assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    }

    // ── Retry-After header ───────────────────────────────────────────────

    #[tokio::test]
    async fn rate_limited_with_retry_after_includes_header() {
        let err: ProxyError = LiterLlmError::RateLimited {
            message: "slow down".into(),
            retry_after: Some(Duration::from_secs(30)),
        }
        .into();
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
        let retry = response
            .headers()
            .get("retry-after")
            .expect("retry-after header must be present");
        assert_eq!(retry.to_str().unwrap(), "30");
    }

    #[tokio::test]
    async fn rate_limited_without_retry_after_omits_header() {
        let err: ProxyError = LiterLlmError::RateLimited {
            message: "slow down".into(),
            retry_after: None,
        }
        .into();
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
        assert!(response.headers().get("retry-after").is_none());
    }

    // ── Display impl ─────────────────────────────────────────────────────

    #[test]
    fn display_delegates_to_body_message() {
        let err = ProxyError::authentication("bad api key");
        assert_eq!(err.to_string(), "bad api key");
    }

    #[test]
    fn display_from_core_error() {
        let err: ProxyError = LiterLlmError::NotFound {
            message: "model gone".into(),
        }
        .into();
        assert!(err.to_string().contains("model gone"));
    }
}
