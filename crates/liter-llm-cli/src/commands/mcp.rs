use std::path::PathBuf;

use clap::Args;
use liter_llm_proxy::config::ProxyConfig;

#[derive(Args)]
pub struct McpArgs {
    /// Path to config file.
    #[arg(long, short)]
    pub config: Option<PathBuf>,
    /// Transport mode: stdio or http.
    #[arg(long, default_value = "stdio")]
    pub transport: String,
    /// Host for HTTP transport.
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,
    /// Port for HTTP transport.
    #[arg(long, default_value_t = 3001)]
    pub port: u16,
}

pub async fn run(args: McpArgs) -> Result<(), String> {
    use std::sync::Arc;

    use liter_llm_proxy::auth::KeyStore;
    use liter_llm_proxy::file_store::FileStore;
    use liter_llm_proxy::mcp::LiterLlmMcp;
    use liter_llm_proxy::service_pool::ServicePool;
    use rmcp::ServiceExt;

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let config = if let Some(path) = &args.config {
        ProxyConfig::from_toml_file(path)?
    } else {
        ProxyConfig::discover()?.unwrap_or_default()
    };

    let service_pool = Arc::new(ServicePool::from_config(&config)?);
    let _key_store = Arc::new(KeyStore::from_config(config.general.master_key.clone(), &config.keys));
    let file_store = Arc::new(FileStore::from_config(
        config.files.as_ref().unwrap_or(&Default::default()),
    )?);

    let mcp = LiterLlmMcp::new(service_pool.clone(), file_store.clone());

    match args.transport.as_str() {
        "stdio" => {
            tracing::info!("starting MCP server with stdio transport");
            let service = mcp
                .serve(rmcp::transport::stdio())
                .await
                .map_err(|e| format!("MCP stdio serve failed: {e}"))?;
            service.waiting().await.map_err(|e| format!("MCP server error: {e}"))?;
        }
        "http" => {
            use rmcp::transport::streamable_http_server::StreamableHttpService;
            use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;

            let addr: std::net::SocketAddr = format!("{}:{}", args.host, args.port)
                .parse()
                .map_err(|e| format!("invalid MCP listen address: {e}"))?;

            let http_service = StreamableHttpService::new(
                move || {
                    let sp = service_pool.clone();
                    let fs = file_store.clone();
                    Ok(LiterLlmMcp::new(sp, fs))
                },
                LocalSessionManager::default().into(),
                Default::default(),
            );

            let router = axum::Router::new().nest_service("/mcp", http_service);

            tracing::info!("starting MCP server with HTTP transport on {addr}");
            let listener = tokio::net::TcpListener::bind(addr)
                .await
                .map_err(|e| format!("failed to bind MCP HTTP {addr}: {e}"))?;
            axum::serve(listener, router)
                .await
                .map_err(|e| format!("MCP HTTP server error: {e}"))?;
        }
        other => return Err(format!("unknown transport '{other}', use 'stdio' or 'http'")),
    }

    Ok(())
}
