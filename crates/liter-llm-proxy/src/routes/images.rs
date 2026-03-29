use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};

use liter_llm::tower::types::{LlmRequest, LlmResponse};
use liter_llm::types::image::CreateImageRequest;

use crate::auth::KeyContext;
use crate::error::ProxyError;
use crate::state::AppState;

/// POST /v1/images/generations
///
/// Generate images from a text prompt. Delegates to the provider resolved from
/// the request's model field (defaults to `"dall-e-3"` when omitted).
#[utoipa::path(
    post,
    path = "/v1/images/generations",
    tag = "images",
    request_body(content_type = "application/json", description = "Image generation request"),
    responses(
        (status = 200, description = "Image generation response"),
        (status = 400, description = "Bad request", body = crate::openapi::ProxyErrorBody),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 429, description = "Rate limited", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn create_image(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
    Json(req): Json<CreateImageRequest>,
) -> Result<Response, ProxyError> {
    let model = req.model.clone().unwrap_or_else(|| "dall-e-3".to_owned());
    let resp = super::dispatch(&state, &key_ctx, &model, LlmRequest::ImageGenerate(req)).await?;

    match resp {
        LlmResponse::ImageGenerate(r) => Ok(Json(r).into_response()),
        other => Err(ProxyError::internal(format!("unexpected response variant: {other:?}"))),
    }
}
