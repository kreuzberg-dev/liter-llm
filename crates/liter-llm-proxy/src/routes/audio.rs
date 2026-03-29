use axum::extract::State;
use axum::http::header;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};

use liter_llm::tower::types::{LlmRequest, LlmResponse};
use liter_llm::types::audio::{CreateSpeechRequest, CreateTranscriptionRequest};

use crate::auth::KeyContext;
use crate::error::ProxyError;
use crate::state::AppState;

/// POST /v1/audio/speech
///
/// Generate speech audio from text input. Returns raw audio bytes with
/// `Content-Type: audio/mpeg`.
#[utoipa::path(
    post,
    path = "/v1/audio/speech",
    tag = "audio",
    request_body(content_type = "application/json", description = "Speech generation request"),
    responses(
        (status = 200, description = "Audio bytes", content_type = "audio/mpeg"),
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
pub async fn create_speech(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
    Json(req): Json<CreateSpeechRequest>,
) -> Result<Response, ProxyError> {
    let model = req.model.clone();
    let resp = super::dispatch(&state, &key_ctx, &model, LlmRequest::Speech(req)).await?;

    match resp {
        LlmResponse::Speech(audio_bytes) => Ok(([(header::CONTENT_TYPE, "audio/mpeg")], audio_bytes).into_response()),
        other => Err(ProxyError::internal(format!("unexpected response variant: {other:?}"))),
    }
}

/// POST /v1/audio/transcriptions
///
/// Transcribe audio into text. Returns a JSON transcription response.
#[utoipa::path(
    post,
    path = "/v1/audio/transcriptions",
    tag = "audio",
    request_body(content_type = "application/json", description = "Audio transcription request"),
    responses(
        (status = 200, description = "Transcription response"),
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
pub async fn create_transcription(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
    Json(req): Json<CreateTranscriptionRequest>,
) -> Result<Response, ProxyError> {
    let model = req.model.clone();
    let resp = super::dispatch(&state, &key_ctx, &model, LlmRequest::Transcribe(req)).await?;

    match resp {
        LlmResponse::Transcribe(r) => Ok(Json(r).into_response()),
        other => Err(ProxyError::internal(format!("unexpected response variant: {other:?}"))),
    }
}
