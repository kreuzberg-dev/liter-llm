use axum::Extension;
use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};
use liter_llm::client::FileClient;
use liter_llm::types::files::{CreateFileRequest, DeleteResponse, FileListQuery, FileListResponse, FileObject};
use serde::Deserialize;

use crate::auth::KeyContext;
use crate::error::ProxyError;
use crate::state::AppState;

/// Local query struct for list files that does not use `deny_unknown_fields`,
/// allowing callers to pass arbitrary query parameters without rejection.
#[derive(Debug, Default, Deserialize)]
pub struct ListFilesQuery {
    #[serde(default)]
    pub purpose: Option<String>,
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub after: Option<String>,
}

/// POST /v1/files
#[utoipa::path(
    post,
    path = "/v1/files",
    tag = "files",
    request_body(content_type = "application/json", description = "File upload request"),
    responses(
        (status = 200, description = "File object"),
        (status = 400, description = "Bad request", body = crate::openapi::ProxyErrorBody),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn create_file(
    State(state): State<AppState>,
    Extension(_key_ctx): Extension<KeyContext>,
    Json(req): Json<CreateFileRequest>,
) -> Result<Json<FileObject>, ProxyError> {
    let client = state.service_pool.first_client()?;
    let result = client.create_file(req).await?;
    Ok(Json(result))
}

/// GET /v1/files
#[utoipa::path(
    get,
    path = "/v1/files",
    tag = "files",
    params(
        ("purpose" = Option<String>, Query, description = "Filter by purpose"),
        ("limit" = Option<u32>, Query, description = "Maximum number of results"),
        ("after" = Option<String>, Query, description = "Cursor for pagination"),
    ),
    responses(
        (status = 200, description = "List of file objects"),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn list_files(
    State(state): State<AppState>,
    Extension(_key_ctx): Extension<KeyContext>,
    Query(params): Query<ListFilesQuery>,
) -> Result<Json<FileListResponse>, ProxyError> {
    let client = state.service_pool.first_client()?;
    let query = if params.purpose.is_some() || params.limit.is_some() || params.after.is_some() {
        Some(FileListQuery {
            purpose: params.purpose,
            limit: params.limit,
            after: params.after,
        })
    } else {
        None
    };
    let result = client.list_files(query).await?;
    Ok(Json(result))
}

/// GET /v1/files/{file_id}
#[utoipa::path(
    get,
    path = "/v1/files/{file_id}",
    tag = "files",
    params(("file_id" = String, Path, description = "File identifier")),
    responses(
        (status = 200, description = "File object"),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 404, description = "Not found", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn retrieve_file(
    State(state): State<AppState>,
    Extension(_key_ctx): Extension<KeyContext>,
    Path(file_id): Path<String>,
) -> Result<Json<FileObject>, ProxyError> {
    let client = state.service_pool.first_client()?;
    let result = client.retrieve_file(&file_id).await?;
    Ok(Json(result))
}

/// DELETE /v1/files/{file_id}
#[utoipa::path(
    delete,
    path = "/v1/files/{file_id}",
    tag = "files",
    params(("file_id" = String, Path, description = "File identifier")),
    responses(
        (status = 200, description = "Deletion confirmation"),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 404, description = "Not found", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn delete_file(
    State(state): State<AppState>,
    Extension(_key_ctx): Extension<KeyContext>,
    Path(file_id): Path<String>,
) -> Result<Json<DeleteResponse>, ProxyError> {
    let client = state.service_pool.first_client()?;
    let result = client.delete_file(&file_id).await?;
    Ok(Json(result))
}

/// GET /v1/files/{file_id}/content
#[utoipa::path(
    get,
    path = "/v1/files/{file_id}/content",
    tag = "files",
    params(("file_id" = String, Path, description = "File identifier")),
    responses(
        (status = 200, description = "File content bytes", content_type = "application/octet-stream"),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 404, description = "Not found", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn file_content(
    State(state): State<AppState>,
    Extension(_key_ctx): Extension<KeyContext>,
    Path(file_id): Path<String>,
) -> Result<Response, ProxyError> {
    let client = state.service_pool.first_client()?;
    let bytes = client.file_content(&file_id).await?;
    Ok(([(CONTENT_TYPE, "application/octet-stream")], bytes).into_response())
}
