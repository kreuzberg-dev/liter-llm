use std::sync::Arc;

use axum::Router;

use liter_llm_proxy::auth::KeyStore;
use liter_llm_proxy::config::ProxyConfig;
use liter_llm_proxy::file_store::FileStore;
use liter_llm_proxy::routes::build_router;
use liter_llm_proxy::service_pool::ServicePool;
use liter_llm_proxy::state::AppState;

/// A lightweight test proxy that builds an axum `Router` from config without
/// binding to a network port.  Tests call `router()` and use
/// `tower::ServiceExt::oneshot` for request dispatch.
pub struct TestProxy {
    state: AppState,
}

impl TestProxy {
    /// Create a proxy whose single model (`test-model`) points at
    /// `mock_url` and accepts `Bearer sk-master` or `Bearer sk-test`.
    pub fn new(mock_url: &str) -> Self {
        Self::with_config(default_config(mock_url))
    }

    /// Create a proxy with a fully customised configuration.
    pub fn with_config(config: ProxyConfig) -> Self {
        let service_pool = ServicePool::from_config(&config).expect("ServicePool::from_config");
        let key_store = KeyStore::from_config(config.general.master_key.clone(), &config.keys);
        let file_store = FileStore::from_config(config.files.as_ref().unwrap_or(&Default::default()))
            .expect("FileStore::from_config");

        let state = AppState {
            key_store: Arc::new(key_store),
            service_pool: Arc::new(service_pool),
            file_store: Arc::new(file_store),
            config: Arc::new(config),
        };

        Self { state }
    }

    /// Return the assembled axum router, ready for `oneshot()`.
    pub fn router(&self) -> Router {
        build_router(self.state.clone())
    }
}

/// Build a default `ProxyConfig` suitable for most integration tests.
///
/// - master key: `sk-master`
/// - virtual key: `sk-test` (access to `test-model`)
/// - one model: `test-model` backed by `openai/gpt-4o` at `mock_url`
pub fn default_config(mock_url: &str) -> ProxyConfig {
    ProxyConfig::from_toml_str(&format!(
        r#"
[general]
master_key = "sk-master"

[[models]]
name = "test-model"
provider_model = "openai/gpt-4o"
api_key = "sk-upstream"
base_url = "{mock_url}"

[[keys]]
key = "sk-test"
description = "integration test virtual key"
models = ["test-model"]
"#
    ))
    .expect("default_config TOML")
}

/// Build an empty `ProxyConfig` with no models and no keys.
#[allow(dead_code)]
pub fn empty_config() -> ProxyConfig {
    ProxyConfig::default()
}
