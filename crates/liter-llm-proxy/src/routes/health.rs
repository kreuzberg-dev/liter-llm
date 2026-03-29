use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde::Serialize;
use utoipa::ToSchema;

use crate::state::AppState;

/// Health check response body.
#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub models: Vec<String>,
}

/// GET /health — full health check with model list.
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Health check response", body = HealthResponse),
    ),
)]
pub async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    let models: Vec<String> = state
        .service_pool
        .model_names()
        .iter()
        .map(|s| (*s).to_owned())
        .collect();
    let status = if state.service_pool.has_any_service() {
        "healthy"
    } else {
        "degraded"
    };
    Json(HealthResponse {
        status: status.into(),
        models,
    })
}

/// GET /health/liveness — always returns 200 OK.
#[utoipa::path(
    get,
    path = "/health/liveness",
    tag = "health",
    responses(
        (status = 200, description = "Service is alive"),
    ),
)]
pub async fn liveness() -> StatusCode {
    StatusCode::OK
}

/// GET /health/readiness — returns 200 only when at least one service is configured.
#[utoipa::path(
    get,
    path = "/health/readiness",
    tag = "health",
    responses(
        (status = 200, description = "Service is ready"),
        (status = 503, description = "Service unavailable"),
    ),
)]
pub async fn readiness(State(state): State<AppState>) -> StatusCode {
    if state.service_pool.has_any_service() {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}
