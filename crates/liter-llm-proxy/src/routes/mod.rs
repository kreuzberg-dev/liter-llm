pub mod audio;
pub mod batches;
pub mod chat;
pub mod embeddings;
pub mod files;
pub mod health;
pub mod images;
pub mod models;
pub mod moderations;
pub mod ocr;
pub mod rerank;
pub mod responses;
pub mod search;

use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::http::HeaderValue;
use axum::http::header::AUTHORIZATION;
use axum::middleware;
use axum::routing::{get, post};
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::sensitive_headers::SetSensitiveHeadersLayer;
use tower_http::trace::TraceLayer;

use tower::Service;

use liter_llm::tower::types::{LlmRequest, LlmResponse};

use crate::auth;
use crate::auth::KeyContext;
use crate::error::ProxyError;
use crate::state::AppState;

/// Check model access for the authenticated key, then dispatch the request
/// through the Tower service stack.
pub(crate) async fn dispatch(
    state: &AppState,
    key_ctx: &KeyContext,
    model: &str,
    request: LlmRequest,
) -> Result<LlmResponse, ProxyError> {
    if !key_ctx.can_access_model(model) {
        return Err(ProxyError::forbidden(format!(
            "key '{}' is not allowed to access model '{model}'",
            key_ctx.key_id
        )));
    }
    let mut svc = state.service_pool.get_service(model)?;
    Ok(svc.call(request).await?)
}

/// Build the full axum router with all routes, middleware, and shared state.
pub fn build_router(state: AppState) -> Router {
    let v1_routes = Router::new()
        // Chat & Completions
        .route("/v1/chat/completions", post(chat::chat_completions))
        // Embeddings
        .route("/v1/embeddings", post(embeddings::create_embedding))
        // Models
        .route("/v1/models", get(models::list_models))
        // Images
        .route("/v1/images/generations", post(images::create_image))
        // Audio
        .route("/v1/audio/speech", post(audio::create_speech))
        .route(
            "/v1/audio/transcriptions",
            post(audio::create_transcription),
        )
        // Moderations
        .route("/v1/moderations", post(moderations::create_moderation))
        // Extended endpoints
        .route("/v1/rerank", post(rerank::rerank))
        .route("/v1/search", post(search::search))
        .route("/v1/ocr", post(ocr::ocr))
        // Files
        .route("/v1/files", post(files::create_file).get(files::list_files))
        .route(
            "/v1/files/{file_id}",
            get(files::retrieve_file).delete(files::delete_file),
        )
        .route("/v1/files/{file_id}/content", get(files::file_content))
        // Batches
        .route(
            "/v1/batches",
            post(batches::create_batch).get(batches::list_batches),
        )
        .route("/v1/batches/{batch_id}", get(batches::retrieve_batch))
        .route(
            "/v1/batches/{batch_id}/cancel",
            post(batches::cancel_batch),
        )
        // Responses
        .route("/v1/responses", post(responses::create_response))
        .route(
            "/v1/responses/{response_id}",
            get(responses::retrieve_response),
        )
        .route(
            "/v1/responses/{response_id}/cancel",
            post(responses::cancel_response),
        )
        // Auth middleware on all /v1 routes
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth::validate_api_key,
        ));

    let health_routes = Router::new()
        .route("/health", get(health::health))
        .route("/health/liveness", get(health::liveness))
        .route("/health/readiness", get(health::readiness))
        .route("/openapi.json", get(crate::openapi::openapi_schema));

    let cors_layer = if state.config.server.cors_origins.iter().any(|o| o == "*") {
        CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any)
    } else {
        let origins: Vec<HeaderValue> = state
            .config
            .server
            .cors_origins
            .iter()
            .filter_map(|o| o.parse().ok())
            .collect();
        CorsLayer::new()
            .allow_origin(AllowOrigin::list(origins))
            .allow_methods(Any)
            .allow_headers(Any)
    };

    Router::new()
        .merge(v1_routes)
        .merge(health_routes)
        .layer(SetSensitiveHeadersLayer::new([AUTHORIZATION]))
        .layer(DefaultBodyLimit::max(state.config.server.body_limit_bytes))
        .layer(cors_layer)
        .layer(CompressionLayer::new())
        .layer(CatchPanicLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
