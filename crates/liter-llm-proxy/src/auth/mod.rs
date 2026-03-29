pub mod key_store;

pub use key_store::{KeyContext, KeyStore};

use axum::extract::{Request, State};
use axum::http::header;
use axum::middleware::Next;
use axum::response::Response;

use crate::error::ProxyError;
use crate::state::AppState;

/// Axum middleware that validates the `Authorization: Bearer <token>` header
/// against the configured master key and virtual key store.
///
/// On success the resolved [`KeyContext`] is inserted into request extensions
/// so downstream handlers can inspect model-access permissions.
pub async fn validate_api_key(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, ProxyError> {
    // 1. Extract Bearer token from Authorization header.
    let token = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| ProxyError::authentication("Missing or invalid Authorization header"))?;

    // 2. Check master key first.
    if state.key_store.is_master_key(token) {
        request.extensions_mut().insert(KeyContext::master());
        return Ok(next.run(request).await);
    }

    // 3. Look up virtual key.
    let key_config = state
        .key_store
        .get(token)
        .ok_or_else(|| ProxyError::authentication("Invalid API key"))?;

    // 4. Insert KeyContext for downstream handlers.
    let ctx = KeyContext::from_config(&key_config);
    request.extensions_mut().insert(ctx);
    Ok(next.run(request).await)
}
