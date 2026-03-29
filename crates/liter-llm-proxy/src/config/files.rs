use std::collections::HashMap;

use serde::Deserialize;

fn default_backend() -> String {
    "memory".to_string()
}

fn default_prefix() -> String {
    "liter-llm-files/".to_string()
}

/// File storage backend configuration for the proxy.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileStorageConfig {
    #[serde(default = "default_backend")]
    pub backend: String,
    #[serde(default = "default_prefix")]
    pub prefix: String,
    #[serde(default)]
    pub backend_config: HashMap<String, String>,
}

impl Default for FileStorageConfig {
    fn default() -> Self {
        Self {
            backend: default_backend(),
            prefix: default_prefix(),
            backend_config: HashMap::new(),
        }
    }
}
