use std::path::PathBuf;

use clap::Args;
use liter_llm_proxy::ProxyServer;
use liter_llm_proxy::config::ProxyConfig;

#[derive(Args)]
pub struct ApiArgs {
    /// Path to config file (default: auto-discover liter-llm-proxy.toml).
    #[arg(long, short)]
    pub config: Option<PathBuf>,
    /// Override bind host.
    #[arg(long, default_value = "0.0.0.0")]
    pub host: String,
    /// Override bind port.
    #[arg(long, short, default_value_t = 4000)]
    pub port: u16,
    /// Master API key (overrides config/env).
    #[arg(long, env = "LITER_LLM_MASTER_KEY")]
    pub master_key: Option<String>,
    /// Enable debug logging.
    #[arg(long)]
    pub debug: bool,
}

pub async fn run(args: ApiArgs) -> Result<(), String> {
    let filter = if args.debug { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(filter)),
        )
        .init();

    let mut config = if let Some(path) = &args.config {
        ProxyConfig::from_toml_file(path)?
    } else {
        ProxyConfig::discover()?.unwrap_or_default()
    };

    // Apply CLI overrides (highest precedence).
    config.server.host.clone_from(&args.host);
    config.server.port = args.port;
    if let Some(key) = args.master_key {
        config.general.master_key = Some(key);
    }

    ProxyServer::new(config).serve().await
}
