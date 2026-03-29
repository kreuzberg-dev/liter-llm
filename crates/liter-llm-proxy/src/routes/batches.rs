use axum::Extension;
use axum::Json;
use axum::extract::{Path, Query, State};
use liter_llm::client::BatchClient;
use liter_llm::types::batch::{BatchListQuery, BatchListResponse, BatchObject, CreateBatchRequest};
use serde::Deserialize;

use crate::auth::KeyContext;
use crate::error::ProxyError;
use crate::state::AppState;

/// Local query struct for list batches that does not use `deny_unknown_fields`,
/// allowing callers to pass arbitrary query parameters without rejection.
#[derive(Debug, Default, Deserialize)]
pub struct ListBatchesQuery {
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub after: Option<String>,
}

/// POST /v1/batches
#[utoipa::path(
    post,
    path = "/v1/batches",
    tag = "batches",
    request_body(content_type = "application/json", description = "Create batch request"),
    responses(
        (status = 200, description = "Batch object"),
        (status = 400, description = "Bad request", body = crate::openapi::ProxyErrorBody),
        (status = 422, description = "Unprocessable entity", body = crate::openapi::ProxyErrorBody),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 415, description = "Unsupported media type", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn create_batch(
    State(state): State<AppState>,
    Extension(_key_ctx): Extension<KeyContext>,
    Json(req): Json<CreateBatchRequest>,
) -> Result<Json<BatchObject>, ProxyError> {
    let client = state.service_pool.first_client()?;
    let result = client.create_batch(req).await?;
    Ok(Json(result))
}

/// GET /v1/batches
#[utoipa::path(
    get,
    path = "/v1/batches",
    tag = "batches",
    params(
        ("limit" = Option<u32>, Query, description = "Maximum number of results"),
        ("after" = Option<String>, Query, description = "Cursor for pagination"),
    ),
    responses(
        (status = 200, description = "List of batch objects"),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 415, description = "Unsupported media type", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn list_batches(
    State(state): State<AppState>,
    Extension(_key_ctx): Extension<KeyContext>,
    Query(params): Query<ListBatchesQuery>,
) -> Result<Json<BatchListResponse>, ProxyError> {
    let client = state.service_pool.first_client()?;
    let query = if params.limit.is_some() || params.after.is_some() {
        Some(BatchListQuery {
            limit: params.limit,
            after: params.after,
        })
    } else {
        None
    };
    let result = client.list_batches(query).await?;
    Ok(Json(result))
}

/// GET /v1/batches/{batch_id}
#[utoipa::path(
    get,
    path = "/v1/batches/{batch_id}",
    tag = "batches",
    params(("batch_id" = String, Path, description = "Batch identifier")),
    responses(
        (status = 200, description = "Batch object"),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 404, description = "Not found", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 415, description = "Unsupported media type", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn retrieve_batch(
    State(state): State<AppState>,
    Extension(_key_ctx): Extension<KeyContext>,
    Path(batch_id): Path<String>,
) -> Result<Json<BatchObject>, ProxyError> {
    let client = state.service_pool.first_client()?;
    let result = client.retrieve_batch(&batch_id).await?;
    Ok(Json(result))
}

/// POST /v1/batches/{batch_id}/cancel
#[utoipa::path(
    post,
    path = "/v1/batches/{batch_id}/cancel",
    tag = "batches",
    params(("batch_id" = String, Path, description = "Batch identifier")),
    responses(
        (status = 200, description = "Cancelled batch object"),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 404, description = "Not found", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 415, description = "Unsupported media type", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn cancel_batch(
    State(state): State<AppState>,
    Extension(_key_ctx): Extension<KeyContext>,
    Path(batch_id): Path<String>,
) -> Result<Json<BatchObject>, ProxyError> {
    let client = state.service_pool.first_client()?;
    let result = client.cancel_batch(&batch_id).await?;
    Ok(Json(result))
}
