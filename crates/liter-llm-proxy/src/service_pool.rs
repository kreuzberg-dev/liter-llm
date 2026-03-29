use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tower::Layer;

use liter_llm::client::{ClientConfigBuilder, DefaultClient};
use liter_llm::error::LiterLlmError;
use liter_llm::tower::types::{LlmRequest, LlmResponse};
use liter_llm::tower::{
    BudgetConfig, BudgetLayer, BudgetState, CacheConfig, CacheLayer, CooldownLayer, CostTrackingLayer, Enforcement,
    HealthCheckLayer, LlmService, ModelRateLimitLayer, RateLimitConfig, TracingLayer,
};

use crate::config::{ModelEntry, ProxyConfig};
use crate::error::ProxyError;

type Bcs = tower::util::BoxCloneService<LlmRequest, LlmResponse, LiterLlmError>;

/// Thread-safe wrapper around `BoxCloneService`.
///
/// Tower's `BoxCloneService` is `Send` but not `Sync`, because `Service::call`
/// takes `&mut self`. We wrap it in a `Mutex` and clone on each request — the
/// lock is held only for the duration of `Clone::clone` (a handful of `Arc`
/// ref-count bumps).
struct SyncBoxService {
    inner: Mutex<Bcs>,
}

impl SyncBoxService {
    /// Clone the inner service out of the mutex.
    ///
    /// # Errors
    ///
    /// Returns `ProxyError::internal` if the mutex is poisoned.
    fn clone_service(&self) -> Result<Bcs, ProxyError> {
        self.inner
            .lock()
            .map(|guard| guard.clone())
            .map_err(|_| ProxyError::internal("service mutex poisoned"))
    }
}

/// A pool of Tower service stacks, one per configured model name.
///
/// Each model name maps to a type-erased `BoxCloneService` with the full
/// middleware stack applied (cache, health check, cooldown, rate limit, cost
/// tracking, budget, tracing).
pub struct ServicePool {
    /// Model name -> Tower service stack.
    services: HashMap<String, SyncBoxService>,
    /// Model name -> raw `DefaultClient` (for File/Batch/Response operations
    /// that bypass the Tower stack).
    clients: HashMap<String, Arc<DefaultClient>>,
    /// The first client inserted during construction, for deterministic
    /// `first_client()` behaviour regardless of `HashMap` iteration order.
    default_client: Option<Arc<DefaultClient>>,
}

// SAFETY: `SyncBoxService` wraps a `Mutex<BoxCloneService>` which is `Send + Sync`.
// `Arc<DefaultClient>` is `Send + Sync`. The compiler verifies these bounds.

impl ServicePool {
    /// Build a pool from the proxy configuration.
    ///
    /// Groups `config.models` by `name` and creates a Tower service stack for
    /// each unique model name.  When multiple deployments share a name, the
    /// first entry is used (round-robin load balancing is planned for v2).
    ///
    /// # Errors
    ///
    /// Returns an error string if a `DefaultClient` cannot be constructed for
    /// any model entry.
    pub fn from_config(config: &ProxyConfig) -> Result<Self, String> {
        // Group model entries by name, preserving insertion order for the
        // first-entry-wins rule.
        let mut grouped: HashMap<String, Vec<&ModelEntry>> = HashMap::new();
        for entry in &config.models {
            grouped.entry(entry.name.clone()).or_default().push(entry);
        }

        let mut services = HashMap::new();
        let mut clients = HashMap::new();
        let mut default_client: Option<Arc<DefaultClient>> = None;

        for (name, entries) in &grouped {
            // Use the first entry for now (round-robin is v2).
            let entry = entries[0];

            let client = build_client(entry, config)?;
            let client_arc = Arc::new(client);

            // Capture the very first client for deterministic `first_client()`.
            if default_client.is_none() {
                default_client = Some(Arc::clone(&client_arc));
            }

            let svc = build_service_stack(config, Arc::clone(&client_arc));

            services.insert(name.clone(), SyncBoxService { inner: Mutex::new(svc) });
            clients.insert(name.clone(), client_arc);
        }

        Ok(Self {
            services,
            clients,
            default_client,
        })
    }

    /// Clone and return a Tower service stack for the given model name.
    ///
    /// # Errors
    ///
    /// Returns `ProxyError::not_found` if no model with that name exists.
    pub fn get_service(&self, model: &str) -> Result<Bcs, ProxyError> {
        self.services
            .get(model)
            .ok_or_else(|| ProxyError::not_found(format!("model '{model}' not found")))?
            .clone_service()
    }

    /// Return a reference to the raw `DefaultClient` for the given model.
    ///
    /// Useful for File, Batch, and Response API operations that bypass the
    /// Tower middleware stack.
    ///
    /// # Errors
    ///
    /// Returns `ProxyError::not_found` if no model with that name exists.
    pub fn get_client(&self, model: &str) -> Result<Arc<DefaultClient>, ProxyError> {
        self.clients
            .get(model)
            .cloned()
            .ok_or_else(|| ProxyError::not_found(format!("model '{model}' not found")))
    }

    /// Return the first available raw client.
    ///
    /// Used by File, Batch, and Response API endpoints that do not carry a
    /// model field in the request body.
    pub fn first_client(&self) -> Result<Arc<DefaultClient>, ProxyError> {
        self.default_client
            .clone()
            .ok_or_else(|| ProxyError::internal("no models configured"))
    }

    /// Return the names of all available models.
    pub fn model_names(&self) -> Vec<&str> {
        self.services.keys().map(String::as_str).collect()
    }

    /// Return `true` if the pool contains at least one service.
    pub fn has_any_service(&self) -> bool {
        !self.services.is_empty()
    }
}

/// Build a `DefaultClient` from a `ModelEntry` and global config defaults.
fn build_client(entry: &ModelEntry, config: &ProxyConfig) -> Result<DefaultClient, String> {
    let api_key = entry.api_key.as_deref().unwrap_or("");

    let mut builder = ClientConfigBuilder::new(api_key);

    if let Some(ref url) = entry.base_url {
        builder = builder.base_url(url);
    }

    let timeout_secs = entry.timeout_secs.unwrap_or(config.general.default_timeout_secs);
    builder = builder.timeout(Duration::from_secs(timeout_secs));
    builder = builder.max_retries(config.general.max_retries);

    let client_config = builder.build();

    DefaultClient::new(client_config, Some(&entry.provider_model))
        .map_err(|e| format!("failed to build client for model '{}': {e}", entry.name))
}

/// Compose the Tower middleware stack, following the same layering order as
/// `managed.rs:build_service_stack`:
///
/// 1. Cache (innermost)
/// 2. HealthCheck
/// 3. Cooldown
/// 4. RateLimit
/// 5. CostTracking
/// 6. Budget
/// 7. Tracing (outermost)
fn build_service_stack(config: &ProxyConfig, client: Arc<DefaultClient>) -> Bcs {
    let base = LlmService::new_from_arc(client);
    let mut svc: Bcs = tower::util::BoxCloneService::new(base);

    // 1. Cache (innermost).
    if let Some(ref cache_cfg) = config.cache {
        let max_entries = cache_cfg.max_entries.unwrap_or(256);
        let ttl = Duration::from_secs(cache_cfg.ttl_seconds.unwrap_or(300));
        let tower_cache_cfg = CacheConfig {
            max_entries,
            ttl,
            backend: liter_llm::tower::CacheBackend::Memory,
        };
        let layer = CacheLayer::new(tower_cache_cfg);
        svc = tower::util::BoxCloneService::new(layer.layer(svc));
    }

    // 2. HealthCheck.
    if let Some(ref health_cfg) = config.health
        && let Some(interval_secs) = health_cfg.interval_secs
    {
        let layer = HealthCheckLayer::new(Duration::from_secs(interval_secs));
        svc = tower::util::BoxCloneService::new(layer.layer(svc));
    }

    // 3. Cooldown.
    if let Some(ref cooldown_cfg) = config.cooldown {
        let layer = CooldownLayer::new(Duration::from_secs(cooldown_cfg.duration_secs));
        svc = tower::util::BoxCloneService::new(layer.layer(svc));
    }

    // 4. RateLimit.
    if let Some(ref rl_cfg) = config.rate_limit {
        let tower_rl_cfg = RateLimitConfig {
            rpm: rl_cfg.rpm,
            tpm: rl_cfg.tpm,
            window: Duration::from_secs(60),
        };
        let layer = ModelRateLimitLayer::new(tower_rl_cfg);
        svc = tower::util::BoxCloneService::new(layer.layer(svc));
    }

    // 5. CostTracking.
    if config.general.enable_cost_tracking {
        svc = tower::util::BoxCloneService::new(CostTrackingLayer.layer(svc));
    }

    // 6. Budget.
    if let Some(ref budget_cfg) = config.budget {
        let enforcement = match budget_cfg.enforcement {
            crate::config::EnforcementMode::Soft => Enforcement::Soft,
            crate::config::EnforcementMode::Hard => Enforcement::Hard,
        };
        let tower_budget_cfg = BudgetConfig {
            global_limit: budget_cfg.global_limit,
            model_limits: budget_cfg.model_limits.clone(),
            enforcement,
        };
        let state = Arc::new(BudgetState::new());
        let layer = BudgetLayer::new(tower_budget_cfg, state);
        svc = tower::util::BoxCloneService::new(layer.layer(svc));
    }

    // 7. Tracing (outermost).
    if config.general.enable_tracing {
        svc = tower::util::BoxCloneService::new(TracingLayer.layer(svc));
    }

    svc
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ProxyConfig;

    fn config_with_one_model() -> ProxyConfig {
        ProxyConfig::from_toml_str(
            r#"
[[models]]
name = "test-model"
provider_model = "openai/gpt-4o"
api_key = "sk-test"
"#,
        )
        .expect("valid TOML")
    }

    fn config_with_two_models() -> ProxyConfig {
        ProxyConfig::from_toml_str(
            r#"
[[models]]
name = "model-a"
provider_model = "openai/gpt-4o"
api_key = "sk-a"

[[models]]
name = "model-b"
provider_model = "anthropic/claude-sonnet-4-20250514"
api_key = "sk-b"
"#,
        )
        .expect("valid TOML")
    }

    #[test]
    fn build_from_empty_config() {
        let config = ProxyConfig::default();
        let pool = ServicePool::from_config(&config).expect("empty config should build");
        assert!(pool.services.is_empty());
        assert!(pool.clients.is_empty());
        assert!(!pool.has_any_service());
    }

    #[test]
    fn build_from_config_with_one_model() {
        let config = config_with_one_model();
        let pool = ServicePool::from_config(&config).expect("should build");
        assert_eq!(pool.services.len(), 1);
        assert_eq!(pool.clients.len(), 1);
        assert!(pool.has_any_service());
    }

    #[test]
    fn get_service_for_unknown_model_returns_not_found() {
        let config = config_with_one_model();
        let pool = ServicePool::from_config(&config).expect("should build");
        let result = pool.get_service("nonexistent");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn get_service_for_known_model_succeeds() {
        let config = config_with_one_model();
        let pool = ServicePool::from_config(&config).expect("should build");
        let result = pool.get_service("test-model");
        assert!(result.is_ok());
    }

    #[test]
    fn get_client_for_known_model_succeeds() {
        let config = config_with_one_model();
        let pool = ServicePool::from_config(&config).expect("should build");
        let result = pool.get_client("test-model");
        assert!(result.is_ok());
    }

    #[test]
    fn get_client_for_unknown_model_returns_not_found() {
        let config = config_with_one_model();
        let pool = ServicePool::from_config(&config).expect("should build");
        let result = pool.get_client("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn model_names_returns_correct_list() {
        let config = config_with_two_models();
        let pool = ServicePool::from_config(&config).expect("should build");
        let mut names = pool.model_names();
        names.sort();
        assert_eq!(names, vec!["model-a", "model-b"]);
    }

    #[test]
    fn has_any_service_returns_false_for_empty_pool() {
        let config = ProxyConfig::default();
        let pool = ServicePool::from_config(&config).expect("should build");
        assert!(!pool.has_any_service());
    }

    #[test]
    fn has_any_service_returns_true_for_nonempty_pool() {
        let config = config_with_one_model();
        let pool = ServicePool::from_config(&config).expect("should build");
        assert!(pool.has_any_service());
    }

    #[tokio::test]
    async fn build_with_middleware_config() {
        let config = ProxyConfig::from_toml_str(
            r#"
[general]
enable_cost_tracking = true
enable_tracing = true

[[models]]
name = "gpt"
provider_model = "openai/gpt-4o"
api_key = "sk-test"

[cache]
max_entries = 128
ttl_seconds = 60

[rate_limit]
rpm = 100

[budget]
global_limit = 50.0
enforcement = "soft"

[cooldown]
duration_secs = 30

[health]
interval_secs = 10
"#,
        )
        .expect("valid TOML");

        let pool = ServicePool::from_config(&config).expect("should build with middleware");
        assert!(pool.has_any_service());
        assert!(pool.get_service("gpt").is_ok());
    }

    #[test]
    fn duplicate_model_names_use_first_entry() {
        let config = ProxyConfig::from_toml_str(
            r#"
[[models]]
name = "gpt"
provider_model = "openai/gpt-4o"
api_key = "sk-1"

[[models]]
name = "gpt"
provider_model = "azure/gpt-4o"
api_key = "sk-2"
"#,
        )
        .expect("valid TOML");

        let pool = ServicePool::from_config(&config).expect("should build");
        // Only one entry in the pool despite two config entries with same name.
        assert_eq!(pool.services.len(), 1);
        assert!(pool.get_service("gpt").is_ok());
    }
}
