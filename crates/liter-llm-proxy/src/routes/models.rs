use axum::Extension;
use axum::Json;
use axum::extract::State;

use liter_llm::types::{ModelObject, ModelsListResponse};

use crate::auth::KeyContext;
use crate::error::ProxyError;
use crate::state::AppState;

/// GET /v1/models
///
/// Returns the list of models configured in the proxy.  The response is
/// filtered to only include models the authenticated key is allowed to
/// access.  The model entries are synthetic — they reflect what the proxy
/// has configured, not what the upstream provider reports.
#[utoipa::path(
    get,
    path = "/v1/models",
    tag = "models",
    responses(
        (status = 200, description = "List of available models"),
        (status = 401, description = "Unauthorized", body = crate::openapi::ProxyErrorBody),
        (status = 500, description = "Internal server error", body = crate::openapi::ProxyErrorBody),
        (status = 503, description = "Service unavailable", body = crate::openapi::ProxyErrorBody),
    ),
    security(("bearer_auth" = [])),
)]
pub async fn list_models(
    State(state): State<AppState>,
    Extension(key_ctx): Extension<KeyContext>,
) -> Result<Json<ModelsListResponse>, ProxyError> {
    let models: Vec<ModelObject> = state
        .service_pool
        .model_names()
        .into_iter()
        .filter(|name| key_ctx.can_access_model(name))
        .map(|name| ModelObject {
            id: name.to_string(),
            object: "model".to_string(),
            created: 0,
            owned_by: "liter-llm-proxy".to_string(),
        })
        .collect();

    Ok(Json(ModelsListResponse {
        object: "list".to_string(),
        data: models,
    }))
}
