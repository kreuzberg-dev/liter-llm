use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};

use liter_llm::tower::types::{LlmRequest, LlmResponse};
use liter_llm::types::ocr::OcrRequest;

use crate::auth::KeyContext;
use crate::error::ProxyError;
use crate::state::AppState;

/// POST /v1/ocr
///
/// Extract text from documents using optical character recognition.
#[utoipa::path(
    post,
    path = "/v1/ocr",
    tag = "ocr",
    request_body(content_type = "application/json", description = "OCR request"),
    responses(
        (status = 200, description = "OCR response"),
        (status = 400, description = "Bad request", body = crate::openapi::ProxyErrorBody),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 429, description = "Rate limited", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn ocr(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
    Json(req): Json<OcrRequest>,
) -> Result<Response, ProxyError> {
    let model = req.model.clone();
    let resp = super::dispatch(&state, &key_ctx, &model, LlmRequest::Ocr(req)).await?;

    match resp {
        LlmResponse::Ocr(r) => Ok(Json(r).into_response()),
        other => Err(ProxyError::internal(format!("unexpected response variant: {other:?}"))),
    }
}
