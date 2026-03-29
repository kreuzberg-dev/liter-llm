pub mod files;
pub mod key;
pub mod model;
pub mod server;

pub use files::FileStorageConfig;
pub use key::VirtualKeyConfig;
pub use model::{AliasEntry, ModelEntry};
pub use server::ServerConfig;

use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;

// ---------------------------------------------------------------------------
// Default helpers
// ---------------------------------------------------------------------------

fn default_timeout() -> u64 {
    120
}

fn default_retries() -> u32 {
    3
}

fn default_cache_backend() -> String {
    "memory".to_string()
}

// ---------------------------------------------------------------------------
// Sub-configs defined in mod.rs (not large enough for their own files)
// ---------------------------------------------------------------------------

/// General proxy behaviour: master key, timeouts, retries, feature flags.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeneralConfig {
    pub master_key: Option<String>,
    #[serde(default = "default_timeout")]
    pub default_timeout_secs: u64,
    #[serde(default = "default_retries")]
    pub max_retries: u32,
    #[serde(default)]
    pub enable_cost_tracking: bool,
    #[serde(default)]
    pub enable_tracing: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            master_key: None,
            default_timeout_secs: default_timeout(),
            max_retries: default_retries(),
            enable_cost_tracking: false,
            enable_tracing: false,
        }
    }
}

/// Global rate-limit settings (requests-per-minute / tokens-per-minute).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RateLimitConfig {
    pub rpm: Option<u32>,
    pub tpm: Option<u64>,
}

/// How budget limits are enforced.
#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EnforcementMode {
    /// Requests exceeding the budget are rejected.
    #[default]
    Hard,
    /// Requests exceeding the budget are logged but allowed through.
    Soft,
}

/// Budget enforcement settings with optional per-model limits.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BudgetConfig {
    pub global_limit: Option<f64>,
    #[serde(default)]
    pub model_limits: HashMap<String, f64>,
    #[serde(default)]
    pub enforcement: EnforcementMode,
}

/// Semantic cache configuration.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CacheConfig {
    pub max_entries: Option<usize>,
    pub ttl_seconds: Option<u64>,
    #[serde(default = "default_cache_backend")]
    pub backend: String,
    #[serde(default)]
    pub backend_config: HashMap<String, String>,
}

/// Periodic health-check probe settings.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HealthConfig {
    pub interval_secs: Option<u64>,
    pub probe_model: Option<String>,
}

/// Provider cooldown duration after consecutive failures.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CooldownConfig {
    pub duration_secs: u64,
}

// ---------------------------------------------------------------------------
// Top-level ProxyConfig
// ---------------------------------------------------------------------------

/// Root configuration for the liter-llm proxy server.
///
/// Loaded from a `liter-llm-proxy.toml` file. After deserialization all
/// `${VAR_NAME}` patterns in string values are replaced with the
/// corresponding environment variable.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct ProxyConfig {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub models: Vec<ModelEntry>,
    #[serde(default)]
    pub aliases: Vec<AliasEntry>,
    pub rate_limit: Option<RateLimitConfig>,
    pub budget: Option<BudgetConfig>,
    pub cache: Option<CacheConfig>,
    pub files: Option<FileStorageConfig>,
    #[serde(default)]
    pub keys: Vec<VirtualKeyConfig>,
    pub health: Option<HealthConfig>,
    pub cooldown: Option<CooldownConfig>,
}

// ---------------------------------------------------------------------------
// Environment variable interpolation
// ---------------------------------------------------------------------------

/// Replace all `${VAR_NAME}` occurrences in `s` with the value of the
/// corresponding environment variable. Unknown variables are replaced with
/// the empty string.
pub fn interpolate_env_vars(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '$' && chars.peek() == Some(&'{') {
            // consume '{'
            chars.next();
            let mut var_name = String::new();
            let mut found_closing = false;
            for c in chars.by_ref() {
                if c == '}' {
                    found_closing = true;
                    break;
                }
                var_name.push(c);
            }
            if found_closing {
                if let Ok(val) = std::env::var(&var_name) {
                    result.push_str(&val);
                }
            } else {
                // No closing '}' found — treat `${...` as literal text.
                result.push('$');
                result.push('{');
                result.push_str(&var_name);
            }
        } else {
            result.push(ch);
        }
    }

    result
}

/// Apply env-var interpolation to a raw TOML string, then deserialize.
///
/// This is the simplest correct approach: interpolate the whole TOML source
/// before parsing, so every string value (including nested tables and arrays)
/// gets expanded uniformly.
fn parse_with_env_interpolation(raw: &str) -> Result<ProxyConfig, String> {
    let expanded = interpolate_env_vars(raw);
    toml::from_str(&expanded).map_err(|e| format!("invalid TOML config: {e}"))
}

impl ProxyConfig {
    /// Parse from a TOML string with env-var interpolation.
    pub fn from_toml_str(s: &str) -> Result<Self, String> {
        parse_with_env_interpolation(s)
    }

    /// Load from a TOML file path with env-var interpolation.
    pub fn from_toml_file(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref();
        let content =
            std::fs::read_to_string(path).map_err(|e| format!("failed to read config file {}: {e}", path.display()))?;
        Self::from_toml_str(&content)
    }

    /// Discover `liter-llm-proxy.toml` by walking from the current directory
    /// up to the filesystem root.
    ///
    /// Returns `Ok(None)` if no config file is found.
    pub fn discover() -> Result<Option<Self>, String> {
        let mut current = std::env::current_dir().map_err(|e| format!("failed to get current directory: {e}"))?;
        loop {
            let config_path = current.join("liter-llm-proxy.toml");
            if config_path.exists() {
                return Ok(Some(Self::from_toml_file(config_path)?));
            }
            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => break,
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 1. Parse minimal config (empty string)
    #[test]
    fn parse_minimal_config() {
        let config = ProxyConfig::from_toml_str("").unwrap();
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 4000);
        assert_eq!(config.general.default_timeout_secs, 120);
        assert_eq!(config.general.max_retries, 3);
        assert!(config.models.is_empty());
        assert!(config.keys.is_empty());
        assert!(config.rate_limit.is_none());
        assert!(config.budget.is_none());
        assert!(config.cache.is_none());
        assert!(config.files.is_none());
        assert!(config.health.is_none());
        assert!(config.cooldown.is_none());
    }

    // 2. Parse full config with all sections
    #[test]
    fn parse_full_config() {
        let toml = r#"
[server]
host = "127.0.0.1"
port = 8080
request_timeout_secs = 300
body_limit_bytes = 5242880
cors_origins = ["https://example.com"]

[general]
master_key = "sk-master"
default_timeout_secs = 60
max_retries = 5
enable_cost_tracking = true
enable_tracing = true

[[models]]
name = "gpt-4o"
provider_model = "openai/gpt-4o"
api_key = "sk-openai"
base_url = "https://api.openai.com/v1"
timeout_secs = 30
fallbacks = ["claude-sonnet"]

[[models]]
name = "claude-sonnet"
provider_model = "anthropic/claude-sonnet-4-20250514"

[[aliases]]
pattern = "anthropic/*"
api_key = "sk-anthropic"

[[keys]]
key = "vk-team-a"
description = "Team A key"
models = ["gpt-4o"]
rpm = 60
tpm = 100000
budget_limit = 50.0

[rate_limit]
rpm = 120
tpm = 500000

[budget]
global_limit = 100.0
enforcement = "soft"

[budget.model_limits]
"openai/gpt-4o" = 50.0

[cache]
max_entries = 1024
ttl_seconds = 600
backend = "memory"

[files]
backend = "s3"
prefix = "proxy-files/"

[files.backend_config]
bucket = "my-bucket"

[health]
interval_secs = 30
probe_model = "openai/gpt-4o-mini"

[cooldown]
duration_secs = 60
"#;
        let config = ProxyConfig::from_toml_str(toml).unwrap();

        // Server
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.server.request_timeout_secs, 300);
        assert_eq!(config.server.body_limit_bytes, 5_242_880);
        assert_eq!(config.server.cors_origins, vec!["https://example.com"]);

        // General
        assert_eq!(config.general.master_key.as_deref(), Some("sk-master"));
        assert_eq!(config.general.default_timeout_secs, 60);
        assert_eq!(config.general.max_retries, 5);
        assert!(config.general.enable_cost_tracking);
        assert!(config.general.enable_tracing);

        // Models
        assert_eq!(config.models.len(), 2);
        assert_eq!(config.models[0].name, "gpt-4o");
        assert_eq!(config.models[0].provider_model, "openai/gpt-4o");
        assert_eq!(config.models[0].api_key.as_deref(), Some("sk-openai"));
        assert_eq!(config.models[0].fallbacks, vec!["claude-sonnet"]);
        assert_eq!(config.models[1].name, "claude-sonnet");
        assert!(config.models[1].api_key.is_none());

        // Aliases
        assert_eq!(config.aliases.len(), 1);
        assert_eq!(config.aliases[0].pattern, "anthropic/*");

        // Keys
        assert_eq!(config.keys.len(), 1);
        assert_eq!(config.keys[0].key, "vk-team-a");
        assert_eq!(config.keys[0].models, vec!["gpt-4o"]);
        assert_eq!(config.keys[0].rpm, Some(60));

        // Rate limit
        let rl = config.rate_limit.unwrap();
        assert_eq!(rl.rpm, Some(120));
        assert_eq!(rl.tpm, Some(500_000));

        // Budget
        let budget = config.budget.unwrap();
        assert_eq!(budget.global_limit, Some(100.0));
        assert_eq!(budget.enforcement, EnforcementMode::Soft);
        assert_eq!(budget.model_limits.get("openai/gpt-4o"), Some(&50.0));

        // Cache
        let cache = config.cache.unwrap();
        assert_eq!(cache.max_entries, Some(1024));
        assert_eq!(cache.ttl_seconds, Some(600));
        assert_eq!(cache.backend, "memory");

        // Files
        let files = config.files.unwrap();
        assert_eq!(files.backend, "s3");
        assert_eq!(files.prefix, "proxy-files/");
        assert_eq!(files.backend_config.get("bucket").unwrap(), "my-bucket");

        // Health
        let health = config.health.unwrap();
        assert_eq!(health.interval_secs, Some(30));
        assert_eq!(health.probe_model.as_deref(), Some("openai/gpt-4o-mini"));

        // Cooldown
        assert_eq!(config.cooldown.unwrap().duration_secs, 60);
    }

    // 3. Env var interpolation
    #[test]
    fn env_var_interpolation() {
        // SAFETY: test is not running concurrently with other tests that
        // depend on these specific env vars.
        unsafe {
            std::env::set_var("LITER_TEST_KEY", "sk-from-env");
            std::env::set_var("LITER_TEST_HOST", "10.0.0.1");
        }

        let toml = r#"
[server]
host = "${LITER_TEST_HOST}"

[general]
master_key = "${LITER_TEST_KEY}"
"#;
        let config = ProxyConfig::from_toml_str(toml).unwrap();
        assert_eq!(config.server.host, "10.0.0.1");
        assert_eq!(config.general.master_key.as_deref(), Some("sk-from-env"));

        // SAFETY: cleaning up test-only env vars.
        unsafe {
            std::env::remove_var("LITER_TEST_KEY");
            std::env::remove_var("LITER_TEST_HOST");
        }
    }

    #[test]
    fn env_var_interpolation_preserves_literals() {
        let toml = r#"
[server]
host = "literal-value"
"#;
        let config = ProxyConfig::from_toml_str(toml).unwrap();
        assert_eq!(config.server.host, "literal-value");
    }

    #[test]
    fn env_var_interpolation_unknown_var_becomes_empty() {
        let result = interpolate_env_vars("prefix-${SURELY_NONEXISTENT_VAR_12345}-suffix");
        assert_eq!(result, "prefix--suffix");
    }

    // 4. Unknown field rejection
    #[test]
    fn rejects_unknown_top_level_field() {
        let toml = r#"
unknown_field = true
"#;
        assert!(ProxyConfig::from_toml_str(toml).is_err());
    }

    #[test]
    fn rejects_unknown_server_field() {
        let toml = r#"
[server]
host = "0.0.0.0"
bogus = 42
"#;
        assert!(ProxyConfig::from_toml_str(toml).is_err());
    }

    #[test]
    fn rejects_unknown_general_field() {
        let toml = r#"
[general]
unknown_option = true
"#;
        assert!(ProxyConfig::from_toml_str(toml).is_err());
    }

    // 5. Default values applied correctly
    #[test]
    fn default_values_applied() {
        let config = ProxyConfig::default();
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 4000);
        assert_eq!(config.server.request_timeout_secs, 600);
        assert_eq!(config.server.body_limit_bytes, 10_485_760);
        assert_eq!(config.server.cors_origins, vec!["*"]);
        assert_eq!(config.general.default_timeout_secs, 120);
        assert_eq!(config.general.max_retries, 3);
        assert!(!config.general.enable_cost_tracking);
        assert!(!config.general.enable_tracing);
    }

    #[test]
    fn budget_default_enforcement() {
        let toml = r#"
[budget]
global_limit = 100.0
"#;
        let config = ProxyConfig::from_toml_str(toml).unwrap();
        assert_eq!(config.budget.unwrap().enforcement, EnforcementMode::Hard);
    }

    #[test]
    fn cache_default_backend() {
        let toml = r#"
[cache]
max_entries = 256
"#;
        let config = ProxyConfig::from_toml_str(toml).unwrap();
        assert_eq!(config.cache.unwrap().backend, "memory");
    }

    #[test]
    fn files_default_values() {
        let toml = r#"
[files]
"#;
        let config = ProxyConfig::from_toml_str(toml).unwrap();
        let files = config.files.unwrap();
        assert_eq!(files.backend, "memory");
        assert_eq!(files.prefix, "liter-llm-files/");
        assert!(files.backend_config.is_empty());
    }

    // 6. Multiple models with same name (load balancing)
    #[test]
    fn multiple_models_same_name() {
        let toml = r#"
[[models]]
name = "gpt-4o"
provider_model = "openai/gpt-4o"
api_key = "sk-key-1"

[[models]]
name = "gpt-4o"
provider_model = "azure/gpt-4o"
api_key = "sk-key-2"
"#;
        let config = ProxyConfig::from_toml_str(toml).unwrap();
        assert_eq!(config.models.len(), 2);
        assert_eq!(config.models[0].name, "gpt-4o");
        assert_eq!(config.models[1].name, "gpt-4o");
        assert_ne!(config.models[0].provider_model, config.models[1].provider_model);
    }

    // 7. Model with fallbacks
    #[test]
    fn model_with_fallbacks() {
        let toml = r#"
[[models]]
name = "primary"
provider_model = "openai/gpt-4o"
fallbacks = ["fallback-1", "fallback-2"]

[[models]]
name = "fallback-1"
provider_model = "anthropic/claude-sonnet-4-20250514"

[[models]]
name = "fallback-2"
provider_model = "groq/llama3-70b"
"#;
        let config = ProxyConfig::from_toml_str(toml).unwrap();
        assert_eq!(config.models[0].fallbacks, vec!["fallback-1", "fallback-2"]);
        assert!(config.models[1].fallbacks.is_empty());
        assert!(config.models[2].fallbacks.is_empty());
    }

    #[test]
    fn interpolate_env_vars_basic() {
        assert_eq!(interpolate_env_vars("no vars here"), "no vars here");
        assert_eq!(interpolate_env_vars(""), "");
        assert_eq!(interpolate_env_vars("$not_a_var"), "$not_a_var");
    }

    #[test]
    fn interpolate_env_vars_multiple() {
        // SAFETY: test is not running concurrently with other tests that
        // depend on these specific env vars.
        unsafe {
            std::env::set_var("LITER_A", "hello");
            std::env::set_var("LITER_B", "world");
        }
        let result = interpolate_env_vars("${LITER_A} ${LITER_B}!");
        assert_eq!(result, "hello world!");
        // SAFETY: cleaning up test-only env vars.
        unsafe {
            std::env::remove_var("LITER_A");
            std::env::remove_var("LITER_B");
        }
    }

    #[test]
    fn interpolate_env_vars_unclosed_brace_treated_as_literal() {
        // Unclosed `${` should be preserved as literal text, not silently dropped.
        assert_eq!(interpolate_env_vars("prefix-${UNCLOSED"), "prefix-${UNCLOSED");
        assert_eq!(interpolate_env_vars("${"), "${");
        assert_eq!(interpolate_env_vars("a${b"), "a${b");
    }
}
