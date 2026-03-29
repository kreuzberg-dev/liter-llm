use axum::Extension;
use axum::Json;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use tower::Service;

use liter_llm::tower::types::{LlmRequest, LlmResponse};
use liter_llm::types::ChatCompletionRequest;

use crate::auth::KeyContext;
use crate::error::ProxyError;
use crate::state::AppState;
use crate::streaming;

/// POST /v1/chat/completions
///
/// Accepts an OpenAI-compatible chat completion request, checks model access
/// for the authenticated key, and dispatches to the appropriate Tower service.
/// When `"stream": true` is present in the request body the response is
/// returned as an SSE stream; otherwise a single JSON body is returned.
#[utoipa::path(
    post,
    path = "/v1/chat/completions",
    tag = "chat",
    request_body(content_type = "application/json", description = "Chat completion request"),
    responses(
        (status = 200, description = "Chat completion response"),
        (status = 400, description = "Bad request", body = crate::openapi::ProxyErrorBody),
        (status = 422, description = "Unprocessable entity", body = crate::openapi::ProxyErrorBody),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 429, description = "Rate limited", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 415, description = "Unsupported media type", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn chat_completions(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
    Json(body): Json<serde_json::Value>,
) -> Result<Response, ProxyError> {
    // Peek at the `stream` flag and `model` from the raw JSON before
    // deserializing into the typed request (the `stream` field is `pub(crate)`
    // in liter-llm).
    let is_stream = body.get("stream").and_then(serde_json::Value::as_bool).unwrap_or(false);

    let model = body
        .get("model")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| ProxyError::bad_request("missing 'model' field"))?
        .to_owned();

    // Check model access early, before deserializing the full request.
    if !key_ctx.can_access_model(&model) {
        return Err(ProxyError::forbidden(format!(
            "key '{}' is not allowed to access model '{model}'",
            key_ctx.key_id
        )));
    }

    // `body` must be consumed here; `model` is already an owned String above.
    let req: ChatCompletionRequest =
        serde_json::from_value(body.clone()).map_err(|e| ProxyError::bad_request(e.to_string()))?;

    let llm_req = if is_stream {
        LlmRequest::ChatStream(req)
    } else {
        LlmRequest::Chat(req)
    };

    let mut svc = state.service_pool.get_service(&model)?;
    let resp = svc.call(llm_req).await?;

    match resp {
        LlmResponse::ChatStream(stream) => Ok(streaming::sse_response(stream)),
        LlmResponse::Chat(completion) => Ok(Json(completion).into_response()),
        other => Err(ProxyError::internal(format!("unexpected response variant: {other:?}"))),
    }
}
