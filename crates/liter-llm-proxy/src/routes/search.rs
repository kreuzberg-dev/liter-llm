use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};

use liter_llm::tower::types::{LlmRequest, LlmResponse};
use liter_llm::types::search::SearchRequest;

use crate::auth::KeyContext;
use crate::error::ProxyError;
use crate::state::AppState;

/// POST /v1/search
///
/// Search the web or document collections via a provider.
#[utoipa::path(
    post,
    path = "/v1/search",
    tag = "search",
    request_body(content_type = "application/json", description = "Search request"),
    responses(
        (status = 200, description = "Search response"),
        (status = 400, description = "Bad request", body = crate::openapi::ProxyErrorBody),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 429, description = "Rate limited", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn search(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
    Json(req): Json<SearchRequest>,
) -> Result<Response, ProxyError> {
    let model = req.model.clone();
    let resp = super::dispatch(&state, &key_ctx, &model, LlmRequest::Search(req)).await?;

    match resp {
        LlmResponse::Search(r) => Ok(Json(r).into_response()),
        other => Err(ProxyError::internal(format!("unexpected response variant: {other:?}"))),
    }
}
