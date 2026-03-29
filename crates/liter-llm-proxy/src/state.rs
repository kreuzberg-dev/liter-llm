use std::sync::Arc;

use crate::auth::KeyStore;
use crate::config::ProxyConfig;
use crate::file_store::FileStore;
use crate::service_pool::ServicePool;

/// Shared application state passed to all axum handlers via `State`.
#[derive(Clone)]
pub struct AppState {
    pub key_store: Arc<KeyStore>,
    pub service_pool: Arc<ServicePool>,
    pub file_store: Arc<FileStore>,
    pub config: Arc<ProxyConfig>,
}
