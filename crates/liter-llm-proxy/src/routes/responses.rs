use axum::Extension;
use axum::Json;
use axum::extract::{Path, State};
use liter_llm::client::ResponseClient;
use liter_llm::types::responses::{CreateResponseRequest, ResponseObject};

use crate::auth::KeyContext;
use crate::error::ProxyError;
use crate::state::AppState;

/// POST /v1/responses
#[utoipa::path(
    post,
    path = "/v1/responses",
    tag = "responses",
    request_body(content_type = "application/json", description = "Create response request"),
    responses(
        (status = 200, description = "Response object"),
        (status = 400, description = "Bad request", body = crate::openapi::ProxyErrorBody),
        (status = 422, description = "Unprocessable entity", body = crate::openapi::ProxyErrorBody),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 415, description = "Unsupported media type", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn create_response(
    State(state): State<AppState>,
    Extension(_key_ctx): Extension<KeyContext>,
    Json(req): Json<CreateResponseRequest>,
) -> Result<Json<ResponseObject>, ProxyError> {
    let client = state.service_pool.first_client()?;
    let result = client.create_response(req).await?;
    Ok(Json(result))
}

/// GET /v1/responses/{response_id}
#[utoipa::path(
    get,
    path = "/v1/responses/{response_id}",
    tag = "responses",
    params(("response_id" = String, Path, description = "Response identifier")),
    responses(
        (status = 200, description = "Response object"),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 404, description = "Not found", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 415, description = "Unsupported media type", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn retrieve_response(
    State(state): State<AppState>,
    Extension(_key_ctx): Extension<KeyContext>,
    Path(response_id): Path<String>,
) -> Result<Json<ResponseObject>, ProxyError> {
    let client = state.service_pool.first_client()?;
    let result = client.retrieve_response(&response_id).await?;
    Ok(Json(result))
}

/// POST /v1/responses/{response_id}/cancel
#[utoipa::path(
    post,
    path = "/v1/responses/{response_id}/cancel",
    tag = "responses",
    params(("response_id" = String, Path, description = "Response identifier")),
    responses(
        (status = 200, description = "Cancelled response object"),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 404, description = "Not found", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 415, description = "Unsupported media type", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn cancel_response(
    State(state): State<AppState>,
    Extension(_key_ctx): Extension<KeyContext>,
    Path(response_id): Path<String>,
) -> Result<Json<ResponseObject>, ProxyError> {
    let client = state.service_pool.first_client()?;
    let result = client.cancel_response(&response_id).await?;
    Ok(Json(result))
}
