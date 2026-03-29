use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};

use liter_llm::tower::types::{LlmRequest, LlmResponse};
use liter_llm::types::moderation::ModerationRequest;

use crate::auth::KeyContext;
use crate::error::ProxyError;
use crate::state::AppState;

/// POST /v1/moderations
///
/// Classify content for policy violations. The model field is optional and
/// defaults to `"text-moderation-stable"` when omitted.
#[utoipa::path(
    post,
    path = "/v1/moderations",
    tag = "moderations",
    request_body(content_type = "application/json", description = "Moderation request"),
    responses(
        (status = 200, description = "Moderation response"),
        (status = 400, description = "Bad request", body = crate::openapi::ProxyErrorBody),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 429, description = "Rate limited", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn create_moderation(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
    Json(req): Json<ModerationRequest>,
) -> Result<Response, ProxyError> {
    let model = req.model.clone().unwrap_or_else(|| "text-moderation-stable".to_owned());
    let resp = super::dispatch(&state, &key_ctx, &model, LlmRequest::Moderate(req)).await?;

    match resp {
        LlmResponse::Moderate(r) => Ok(Json(r).into_response()),
        other => Err(ProxyError::internal(format!("unexpected response variant: {other:?}"))),
    }
}
