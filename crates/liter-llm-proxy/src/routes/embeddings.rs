use axum::Extension;
use axum::Json;
use axum::extract::State;

use liter_llm::tower::types::{LlmRequest, LlmResponse};
use liter_llm::types::{EmbeddingRequest, EmbeddingResponse};

use crate::auth::KeyContext;
use crate::error::ProxyError;
use crate::state::AppState;

/// POST /v1/embeddings
///
/// Accepts an OpenAI-compatible embedding request, checks model access for
/// the authenticated key, and dispatches to the Tower service stack.
#[utoipa::path(
    post,
    path = "/v1/embeddings",
    tag = "embeddings",
    request_body(content_type = "application/json", description = "Embedding request"),
    responses(
        (status = 200, description = "Embedding response"),
        (status = 400, description = "Bad request", body = crate::openapi::ProxyErrorBody),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 429, description = "Rate limited", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn create_embedding(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
    Json(req): Json<EmbeddingRequest>,
) -> Result<Json<EmbeddingResponse>, ProxyError> {
    let model = req.model.clone();
    let resp = super::dispatch(&state, &key_ctx, &model, LlmRequest::Embed(req)).await?;

    match resp {
        LlmResponse::Embed(embedding) => Ok(Json(embedding)),
        other => Err(ProxyError::internal(format!("unexpected response variant: {other:?}"))),
    }
}
