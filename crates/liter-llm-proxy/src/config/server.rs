use serde::Deserialize;

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    4000
}

fn default_request_timeout() -> u64 {
    600
}

fn default_body_limit() -> usize {
    10_485_760
}

fn default_cors() -> Vec<String> {
    vec!["*".to_string()]
}

/// HTTP server configuration for the proxy.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_request_timeout")]
    pub request_timeout_secs: u64,
    #[serde(default = "default_body_limit")]
    pub body_limit_bytes: usize,
    #[serde(default = "default_cors")]
    pub cors_origins: Vec<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            request_timeout_secs: default_request_timeout(),
            body_limit_bytes: default_body_limit(),
            cors_origins: default_cors(),
        }
    }
}
