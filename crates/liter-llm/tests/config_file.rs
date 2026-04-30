//! Integration tests for TOML configuration file loading.
//!
//! Tests load actual `.toml` fixture files from `tests/fixtures/config/` and
//! verify that parsed values flow correctly through `FileConfig` into
//! `ClientConfig` via `into_builder().build()`.

mod common;

use std::path::Path;
use std::time::Duration;

use secrecy::ExposeSecret;

#[cfg(feature = "tower")]
use liter_llm::tower::{CacheBackend, Enforcement};
use liter_llm::{ClientConfig, FileConfig};

// ── Helpers ─────────────────────────────────────────────────────────────────

const FIXTURES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/config");

fn fixture(name: &str) -> String {
    let path = Path::new(FIXTURES).join(name);
    assert!(path.exists(), "fixture not found: {}", path.display());
    path.to_string_lossy().into_owned()
}

fn load(name: &str) -> FileConfig {
    FileConfig::from_toml_file(fixture(name)).expect("failed to load fixture")
}

fn load_and_build(name: &str) -> ClientConfig {
    load(name).into_builder().build()
}

// ── Parsing: minimal ────────────────────────────────────────────────────────

#[test]
fn minimal_config_parses_api_key() {
    let fc = load("minimal.toml");
    assert_eq!(fc.api_key.as_deref(), Some("sk-test-minimal"));
    assert!(fc.base_url.is_none());
    assert!(fc.cache.is_none());
    assert!(fc.budget.is_none());
    assert!(fc.rate_limit.is_none());
    assert!(fc.providers.is_none());
}

#[test]
fn minimal_config_builds_with_defaults() {
    let config = load_and_build("minimal.toml");
    assert_eq!(config.api_key.expose_secret(), "sk-test-minimal");
    assert!(config.base_url.is_none());
    assert_eq!(config.timeout, Duration::from_secs(60));
    assert_eq!(config.max_retries, 3);
}

// ── Parsing: empty ──────────────────────────────────────────────────────────

#[test]
fn empty_config_is_valid() {
    let fc = FileConfig::from_toml_str("").expect("empty config should parse");
    assert!(fc.api_key.is_none());
    let config = fc.into_builder().build();
    // Empty api_key becomes empty string in the builder
    assert_eq!(config.api_key.expose_secret(), "");
}

// ── Parsing: full config ────────────────────────────────────────────────────

#[test]
fn full_config_parses_all_top_level_fields() {
    let fc = load("full.toml");
    assert_eq!(fc.api_key.as_deref(), Some("sk-test-full"));
    assert_eq!(fc.base_url.as_deref(), Some("https://api.example.com/v1"));
    assert_eq!(fc.model_hint.as_deref(), Some("openai"));
    assert_eq!(fc.timeout_secs, Some(120));
    assert_eq!(fc.max_retries, Some(5));
    assert_eq!(fc.cooldown_secs, Some(30));
    assert_eq!(fc.health_check_secs, Some(60));
    assert_eq!(fc.cost_tracking, Some(true));
    assert_eq!(fc.tracing, Some(true));
}

#[test]
fn full_config_parses_cache_section() {
    let fc = load("full.toml");
    let cache = fc.cache.as_ref().expect("cache section missing");
    assert_eq!(cache.max_entries, Some(512));
    assert_eq!(cache.ttl_seconds, Some(600));
    assert_eq!(cache.backend.as_deref(), Some("memory"));
}

#[test]
fn full_config_parses_budget_section() {
    let fc = load("full.toml");
    let budget = fc.budget.as_ref().expect("budget section missing");
    assert_eq!(budget.global_limit, Some(50.0));
    assert_eq!(budget.enforcement.as_deref(), Some("hard"));
    let limits = budget.model_limits.as_ref().expect("model_limits missing");
    assert_eq!(limits.get("openai/gpt-4o"), Some(&25.0));
    assert_eq!(limits.get("anthropic/claude-3-5-sonnet"), Some(&15.0));
}

#[test]
fn full_config_parses_rate_limit_section() {
    let fc = load("full.toml");
    let rl = fc.rate_limit.as_ref().expect("rate_limit section missing");
    assert_eq!(rl.rpm, Some(60));
    assert_eq!(rl.tpm, Some(100_000));
    assert_eq!(rl.window_seconds, Some(120));
}

#[test]
fn full_config_parses_extra_headers() {
    let fc = load("full.toml");
    let headers = fc.extra_headers.as_ref().expect("extra_headers missing");
    assert_eq!(headers.get("X-Custom-Header").map(String::as_str), Some("custom-value"));
    assert_eq!(headers.get("X-Request-Id").map(String::as_str), Some("test-123"));
}

#[test]
fn full_config_parses_providers() {
    let fc = load("full.toml");
    let providers = fc.providers();
    assert_eq!(providers.len(), 2);

    assert_eq!(providers[0].name, "my-provider");
    assert_eq!(providers[0].base_url, "https://my-llm.example.com/v1");
    assert_eq!(providers[0].auth_header.as_deref(), Some("Authorization"));
    assert_eq!(providers[0].model_prefixes, vec!["my-provider/"]);

    assert_eq!(providers[1].name, "another-provider");
    assert_eq!(providers[1].base_url, "https://another.example.com/api");
    assert!(providers[1].auth_header.is_none());
    assert_eq!(providers[1].model_prefixes, vec!["another/", "alt/"]);
}

// ── Builder conversion: full config ─────────────────────────────────────────

#[test]
fn full_config_builds_core_fields() {
    let config = load_and_build("full.toml");
    assert_eq!(config.api_key.expose_secret(), "sk-test-full");
    assert_eq!(config.base_url.as_deref(), Some("https://api.example.com/v1"));
    assert_eq!(config.timeout, Duration::from_secs(120));
    assert_eq!(config.max_retries, 5);
}

#[cfg(feature = "tower")]
#[test]
fn full_config_builds_cache_config() {
    let config = load_and_build("full.toml");
    let cache = config.cache_config.as_ref().expect("cache_config not set");
    assert_eq!(cache.max_entries, 512);
    assert_eq!(cache.ttl, Duration::from_secs(600));
    assert!(matches!(cache.backend, CacheBackend::Memory));
}

#[cfg(feature = "tower")]
#[test]
fn full_config_builds_budget_config() {
    let config = load_and_build("full.toml");
    let budget = config.budget_config.as_ref().expect("budget_config not set");
    assert_eq!(budget.global_limit, Some(50.0));
    assert!(matches!(budget.enforcement, Enforcement::Hard));
    assert_eq!(budget.model_limits.get("openai/gpt-4o"), Some(&25.0));
    assert_eq!(budget.model_limits.get("anthropic/claude-3-5-sonnet"), Some(&15.0));
}

#[cfg(feature = "tower")]
#[test]
fn full_config_builds_rate_limit_config() {
    let config = load_and_build("full.toml");
    let rl = config.rate_limit_config.as_ref().expect("rate_limit_config not set");
    assert_eq!(rl.rpm, Some(60));
    assert_eq!(rl.tpm, Some(100_000));
    assert_eq!(rl.window, Duration::from_secs(120));
}

#[cfg(feature = "tower")]
#[test]
fn full_config_builds_cooldown() {
    let config = load_and_build("full.toml");
    assert_eq!(config.cooldown_duration, Some(Duration::from_secs(30)));
}

#[cfg(feature = "tower")]
#[test]
fn full_config_builds_health_check() {
    let config = load_and_build("full.toml");
    assert_eq!(config.health_check_interval, Some(Duration::from_secs(60)));
}

#[cfg(feature = "tower")]
#[test]
fn full_config_builds_cost_tracking_and_tracing() {
    let config = load_and_build("full.toml");
    assert!(config.enable_cost_tracking);
    assert!(config.enable_tracing);
}

// ── Standalone sections ─────────────────────────────────────────────────────

#[cfg(feature = "tower")]
#[test]
fn cache_only_config_sets_cache_leaves_rest_default() {
    let config = load_and_build("cache_only.toml");
    let cache = config.cache_config.as_ref().expect("cache_config not set");
    assert_eq!(cache.max_entries, 1024);
    assert_eq!(cache.ttl, Duration::from_secs(900));
    assert!(matches!(cache.backend, CacheBackend::Memory));

    // Everything else should be default/None
    assert!(config.budget_config.is_none());
    assert!(config.rate_limit_config.is_none());
    assert!(config.cooldown_duration.is_none());
    assert!(config.health_check_interval.is_none());
    assert!(!config.enable_cost_tracking);
    assert!(!config.enable_tracing);
}

#[cfg(feature = "tower")]
#[test]
fn budget_soft_enforcement() {
    let config = load_and_build("budget_soft.toml");
    let budget = config.budget_config.as_ref().expect("budget_config not set");
    assert_eq!(budget.global_limit, Some(100.0));
    assert!(matches!(budget.enforcement, Enforcement::Soft));
    assert_eq!(budget.model_limits.get("openai/gpt-4o"), Some(&50.0));
}

#[cfg(feature = "tower")]
#[test]
fn rate_limit_only_uses_default_window() {
    let config = load_and_build("rate_limit_only.toml");
    let rl = config.rate_limit_config.as_ref().expect("rate_limit_config not set");
    assert_eq!(rl.rpm, Some(100));
    assert_eq!(rl.tpm, Some(200_000));
    // window_seconds not set in fixture -> default 60s
    assert_eq!(rl.window, Duration::from_secs(60));
}

#[test]
fn providers_only_config_parses_multiple_providers() {
    let fc = load("providers_only.toml");
    let providers = fc.providers();
    assert_eq!(providers.len(), 2);
    assert_eq!(providers[0].name, "custom-openai");
    assert_eq!(providers[0].auth_header.as_deref(), Some("X-Api-Key"));
    assert_eq!(providers[1].name, "local-llm");
    assert_eq!(providers[1].model_prefixes, vec!["local/", "local-llm/"]);
}

// ── OpenDAL cache backend ───────────────────────────────────────────────────

#[test]
fn opendal_cache_config_parses_backend_and_config() {
    let fc = load("cache_opendal.toml");
    let cache = fc.cache.as_ref().expect("cache section missing");
    assert_eq!(cache.max_entries, Some(2048));
    assert_eq!(cache.ttl_seconds, Some(3600));
    assert_eq!(cache.backend.as_deref(), Some("s3"));
    let backend_config = cache.backend_config.as_ref().expect("backend_config missing");
    assert_eq!(backend_config.get("bucket").map(String::as_str), Some("my-llm-cache"));
    assert_eq!(backend_config.get("region").map(String::as_str), Some("us-west-2"));
}

#[cfg(all(feature = "tower", not(feature = "opendal-cache")))]
#[test]
fn opendal_cache_falls_back_to_memory_without_feature() {
    let config = load_and_build("cache_opendal.toml");
    let cache = config.cache_config.as_ref().expect("cache_config not set");
    // Without opendal-cache feature, non-"memory" backend falls back to Memory
    assert!(matches!(cache.backend, CacheBackend::Memory));
    assert_eq!(cache.max_entries, 2048);
    assert_eq!(cache.ttl, Duration::from_secs(3600));
}

#[cfg(feature = "opendal-cache")]
#[test]
fn opendal_cache_builds_opendal_backend_with_feature() {
    let config = load_and_build("cache_opendal.toml");
    let cache = config.cache_config.as_ref().expect("cache_config not set");
    match &cache.backend {
        CacheBackend::OpenDal { scheme, config } => {
            assert_eq!(scheme, "s3");
            assert_eq!(config.get("bucket").map(String::as_str), Some("my-llm-cache"));
            assert_eq!(config.get("region").map(String::as_str), Some("us-west-2"));
        }
        CacheBackend::Memory => panic!("expected OpenDal backend, got Memory"),
    }
}

// ── Error cases: invalid TOML ───────────────────────────────────────────────

#[test]
fn rejects_unknown_top_level_field() {
    let result = FileConfig::from_toml_file(fixture("invalid_unknown_field.toml"));
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("unknown field"),
        "error should mention unknown field: {err}"
    );
}

#[test]
fn rejects_wrong_type_for_api_key() {
    let result = FileConfig::from_toml_file(fixture("invalid_wrong_type.toml"));
    assert!(result.is_err());
}

#[test]
fn rejects_unknown_nested_field() {
    let result = FileConfig::from_toml_file(fixture("invalid_nested_unknown.toml"));
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("unknown field"),
        "error should mention unknown field: {err}"
    );
}

#[test]
fn nonexistent_file_returns_error() {
    let result = FileConfig::from_toml_file("/tmp/does-not-exist-liter-llm.toml");
    assert!(result.is_err());
}

// ── Edge cases ──────────────────────────────────────────────────────────────

#[test]
fn budget_defaults_to_hard_enforcement() {
    let fc = FileConfig::from_toml_str(
        r#"
[budget]
global_limit = 10.0
"#,
    )
    .unwrap();
    assert!(fc.budget.as_ref().unwrap().enforcement.is_none());
    // into_builder maps None -> Hard
    #[cfg(feature = "tower")]
    {
        let config = fc.into_builder().build();
        let budget = config.budget_config.as_ref().unwrap();
        assert!(matches!(budget.enforcement, Enforcement::Hard));
    }
}

#[cfg(feature = "tower")]
#[test]
fn cache_defaults_applied_when_section_present_but_empty() {
    let fc = FileConfig::from_toml_str("[cache]\n").unwrap();
    let config = fc.into_builder().build();
    let cache = config.cache_config.as_ref().expect("cache_config not set");
    assert_eq!(cache.max_entries, 256);
    assert_eq!(cache.ttl, Duration::from_secs(300));
    assert!(matches!(cache.backend, CacheBackend::Memory));
}

#[cfg(feature = "tower")]
#[test]
fn rate_limit_with_rpm_only() {
    let fc = FileConfig::from_toml_str(
        r#"
[rate_limit]
rpm = 10
"#,
    )
    .unwrap();
    let config = fc.into_builder().build();
    let rl = config.rate_limit_config.as_ref().unwrap();
    assert_eq!(rl.rpm, Some(10));
    assert!(rl.tpm.is_none());
    assert_eq!(rl.window, Duration::from_secs(60));
}

#[cfg(feature = "tower")]
#[test]
fn rate_limit_with_tpm_only() {
    let fc = FileConfig::from_toml_str(
        r#"
[rate_limit]
tpm = 50000
"#,
    )
    .unwrap();
    let config = fc.into_builder().build();
    let rl = config.rate_limit_config.as_ref().unwrap();
    assert!(rl.rpm.is_none());
    assert_eq!(rl.tpm, Some(50_000));
}

#[test]
fn provider_without_auth_header() {
    let fc = FileConfig::from_toml_str(
        r#"
[[providers]]
name = "local"
base_url = "http://localhost:11434/v1"
model_prefixes = ["local/"]
"#,
    )
    .unwrap();
    let providers = fc.providers();
    assert_eq!(providers.len(), 1);
    assert!(providers[0].auth_header.is_none());
}
