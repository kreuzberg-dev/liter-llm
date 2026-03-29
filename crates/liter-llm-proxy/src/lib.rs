pub mod auth;
pub mod config;
pub mod error;
pub mod file_store;
pub mod mcp;
pub mod openapi;
pub mod routes;
pub mod service_pool;
pub mod state;
pub mod streaming;

use std::net::SocketAddr;
use std::sync::Arc;

use config::ProxyConfig;
use state::AppState;

/// Builder for the liter-llm proxy server.
///
/// Constructs the shared [`AppState`] from a [`ProxyConfig`], builds the axum
/// router, and serves on the configured address.
pub struct ProxyServer {
    config: ProxyConfig,
}

impl ProxyServer {
    /// Create a new proxy server with the given configuration.
    pub fn new(config: ProxyConfig) -> Self {
        Self { config }
    }

    /// Build the application state, assemble the router, and start serving.
    pub async fn serve(self) -> Result<(), String> {
        let service_pool = service_pool::ServicePool::from_config(&self.config)?;
        let key_store = auth::KeyStore::from_config(self.config.general.master_key.clone(), &self.config.keys);
        let file_store = file_store::FileStore::from_config(self.config.files.as_ref().unwrap_or(&Default::default()))?;

        let state = AppState {
            key_store: Arc::new(key_store),
            service_pool: Arc::new(service_pool),
            file_store: Arc::new(file_store),
            config: Arc::new(self.config),
        };

        let addr: SocketAddr = format!("{}:{}", state.config.server.host, state.config.server.port)
            .parse()
            .map_err(|e| format!("invalid listen address: {e}"))?;

        let router = routes::build_router(state);

        tracing::info!("liter-llm proxy listening on {addr}");

        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| format!("failed to bind {addr}: {e}"))?;

        let shutdown = async {
            let _ = tokio::signal::ctrl_c().await;
            tracing::info!("shutdown signal received");
        };

        axum::serve(listener, router)
            .with_graceful_shutdown(shutdown)
            .await
            .map_err(|e| format!("server error: {e}"))?;

        Ok(())
    }
}
