//! Budget enforcement middleware.
//!
//! [`BudgetLayer`] wraps any [`Service<LlmRequest>`] and enforces spending
//! limits (global and per-model) in USD.  Cost is calculated after each
//! successful response using [`crate::cost::completion_cost`] and accumulated
//! atomically in [`BudgetState`].
//!
//! Two enforcement modes are supported:
//!
//! - **Hard** — pre-request check rejects with [`LiterLlmError::BudgetExceeded`]
//!   when the accumulated spend is at or above the configured limit.
//! - **Soft** — requests are never rejected; a `tracing::warn!` is emitted when
//!   the limit is exceeded.
//!
//! # Example
//!
//! ```rust,ignore
//! use liter_llm::tower::{BudgetConfig, BudgetLayer, BudgetState, Enforcement, LlmService};
//! use tower::ServiceBuilder;
//! use std::sync::Arc;
//!
//! let state = Arc::new(BudgetState::new());
//! let config = BudgetConfig {
//!     global_limit: Some(10.0),
//!     model_limits: Default::default(),
//!     enforcement: Enforcement::Hard,
//! };
//!
//! let client = liter_llm::DefaultClient::new(cfg, None)?;
//! let service = ServiceBuilder::new()
//!     .layer(BudgetLayer::new(config, Arc::clone(&state)))
//!     .service(LlmService::new(client));
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::task::{Context, Poll};

use dashmap::DashMap;
use tower::{Layer, Service};

use super::types::{LlmRequest, LlmResponse};
use crate::client::BoxFuture;
use crate::cost;
use crate::error::{LiterLlmError, Result};

// ---- Types -----------------------------------------------------------------

/// How budget limits are enforced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Enforcement {
    /// Reject requests that would exceed the budget with
    /// [`LiterLlmError::BudgetExceeded`].
    Hard,
    /// Allow requests through but emit a `tracing::warn!` when the budget is
    /// exceeded.
    Soft,
}

// ---- Config ----------------------------------------------------------------

/// Configuration for budget enforcement.
#[derive(Debug, Clone)]
pub struct BudgetConfig {
    /// Maximum total spend across all models, in USD.  `None` means unlimited.
    pub global_limit: Option<f64>,
    /// Per-model spending limits in USD.  Models not listed here are only
    /// constrained by `global_limit`.
    pub model_limits: HashMap<String, f64>,
    /// Whether to reject requests or merely warn when a limit is exceeded.
    pub enforcement: Enforcement,
}

impl Default for BudgetConfig {
    fn default() -> Self {
        Self {
            global_limit: None,
            model_limits: HashMap::new(),
            enforcement: Enforcement::Hard,
        }
    }
}

// ---- State -----------------------------------------------------------------

/// Shared, thread-safe budget accumulator.
///
/// All values are stored in **microcents** (USD * 1_000_000) as `AtomicU64` to
/// avoid floating-point atomics while retaining sub-cent precision.
#[derive(Debug)]
pub struct BudgetState {
    /// Total spend across all models (microcents).
    global_spend: AtomicU64,
    /// Per-model spend (microcents).
    model_spend: DashMap<String, AtomicU64>,
}

impl BudgetState {
    /// Create a new, zeroed budget state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            global_spend: AtomicU64::new(0),
            model_spend: DashMap::new(),
        }
    }

    /// Return the total global spend in USD.
    #[must_use]
    pub fn global_spend(&self) -> f64 {
        microcents_to_usd(self.global_spend.load(Ordering::Relaxed))
    }

    /// Return the spend for a specific model in USD, or `0.0` if the model has
    /// not been seen.
    #[must_use]
    pub fn model_spend(&self, model: &str) -> f64 {
        self.model_spend
            .get(model)
            .map(|v| microcents_to_usd(v.load(Ordering::Relaxed)))
            .unwrap_or(0.0)
    }

    /// Reset all counters to zero.
    pub fn reset(&self) {
        self.global_spend.store(0, Ordering::Relaxed);
        self.model_spend.clear();
    }

    /// Add `usd` to the global and per-model counters.
    fn record(&self, model: &str, usd: f64) {
        let mc = usd_to_microcents(usd);
        self.global_spend.fetch_add(mc, Ordering::Relaxed);
        self.model_spend
            .entry(model.to_owned())
            .or_insert_with(|| AtomicU64::new(0))
            .fetch_add(mc, Ordering::Relaxed);
    }
}

impl Default for BudgetState {
    fn default() -> Self {
        Self::new()
    }
}

// ---- Conversions -----------------------------------------------------------

fn usd_to_microcents(usd: f64) -> u64 {
    // Clamp negative values to zero to avoid wrapping in unsigned arithmetic.
    if usd <= 0.0 {
        return 0;
    }
    (usd * 1_000_000.0).round() as u64
}

fn microcents_to_usd(mc: u64) -> f64 {
    mc as f64 / 1_000_000.0
}

// ---- Layer -----------------------------------------------------------------

/// Tower [`Layer`] that enforces spending budgets.
pub struct BudgetLayer {
    config: BudgetConfig,
    state: Arc<BudgetState>,
}

impl BudgetLayer {
    /// Create a new budget layer with the given configuration and shared state.
    ///
    /// The caller retains an `Arc<BudgetState>` for runtime introspection
    /// (e.g. dashboard queries, manual resets).
    #[must_use]
    pub fn new(config: BudgetConfig, state: Arc<BudgetState>) -> Self {
        Self { config, state }
    }
}

impl<S> Layer<S> for BudgetLayer {
    type Service = BudgetService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        BudgetService {
            inner,
            config: self.config.clone(),
            state: Arc::clone(&self.state),
        }
    }
}

// ---- Service ---------------------------------------------------------------

/// Tower service produced by [`BudgetLayer`].
pub struct BudgetService<S> {
    inner: S,
    config: BudgetConfig,
    state: Arc<BudgetState>,
}

impl<S: Clone> Clone for BudgetService<S> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            config: self.config.clone(),
            state: Arc::clone(&self.state),
        }
    }
}

impl<S> Service<LlmRequest> for BudgetService<S>
where
    S: Service<LlmRequest, Response = LlmResponse, Error = LiterLlmError> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = LlmResponse;
    type Error = LiterLlmError;
    type Future = BoxFuture<'static, LlmResponse>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: LlmRequest) -> Self::Future {
        let model = req.model().unwrap_or("unknown").to_owned();
        let config = self.config.clone();
        let state = Arc::clone(&self.state);

        // --- Pre-flight: hard enforcement check ---
        if config.enforcement == Enforcement::Hard
            && let Some(err) = check_budget(&config, &state, &model)
        {
            return Box::pin(async move { Err(err) });
        }

        let fut = self.inner.call(req);

        Box::pin(async move {
            let resp = fut.await?;

            // --- Post-flight: record cost ---
            if let Some(usage) = resp.usage()
                && let Some(usd) = cost::completion_cost(&model, usage.prompt_tokens, usage.completion_tokens)
            {
                state.record(&model, usd);

                // Soft enforcement: warn after recording.
                if config.enforcement == Enforcement::Soft {
                    emit_soft_warnings(&config, &state, &model);
                }
            }

            Ok(resp)
        })
    }
}

// ---- Helpers ---------------------------------------------------------------

/// Check whether the current spend exceeds any configured limit.  Returns
/// `Some(LiterLlmError)` if the budget is exceeded under hard enforcement.
fn check_budget(config: &BudgetConfig, state: &BudgetState, model: &str) -> Option<LiterLlmError> {
    // Global limit check.
    if let Some(limit) = config.global_limit
        && state.global_spend() >= limit
    {
        return Some(LiterLlmError::BudgetExceeded {
            message: format!(
                "global budget exceeded: spent ${:.6}, limit ${:.6}",
                state.global_spend(),
                limit,
            ),
            model: None,
        });
    }

    // Per-model limit check.
    if let Some(&limit) = config.model_limits.get(model)
        && state.model_spend(model) >= limit
    {
        return Some(LiterLlmError::BudgetExceeded {
            message: format!(
                "model {model} budget exceeded: spent ${:.6}, limit ${:.6}",
                state.model_spend(model),
                limit,
            ),
            model: Some(model.to_owned()),
        });
    }

    None
}

/// Emit `tracing::warn!` messages for any exceeded limits (soft mode).
fn emit_soft_warnings(config: &BudgetConfig, state: &BudgetState, model: &str) {
    if let Some(limit) = config.global_limit
        && state.global_spend() >= limit
    {
        tracing::warn!(
            spend = state.global_spend(),
            limit,
            "global budget exceeded (soft enforcement)"
        );
    }

    if let Some(&limit) = config.model_limits.get(model)
        && state.model_spend(model) >= limit
    {
        tracing::warn!(
            model,
            spend = state.model_spend(model),
            limit,
            "model budget exceeded (soft enforcement)"
        );
    }
}

// ---- Tests -----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use tower::{Layer as _, Service as _};

    use super::*;
    use crate::tower::service::LlmService;
    use crate::tower::tests_common::{MockClient, chat_req};
    use crate::tower::types::LlmRequest;

    /// Helper: build a budget layer + service with the given config.
    fn build_service(config: BudgetConfig, state: Arc<BudgetState>) -> BudgetService<LlmService<MockClient>> {
        let layer = BudgetLayer::new(config, state);
        let inner = LlmService::new(MockClient::ok());
        layer.layer(inner)
    }

    // ── Hard enforcement ────────────────────────────────────────────────────

    #[tokio::test]
    async fn hard_enforcement_rejects_when_global_limit_exceeded() {
        let state = Arc::new(BudgetState::new());
        // Pre-seed spend above the limit.
        state.global_spend.store(usd_to_microcents(10.0), Ordering::Relaxed);

        let config = BudgetConfig {
            global_limit: Some(5.0),
            enforcement: Enforcement::Hard,
            ..Default::default()
        };

        let mut svc = build_service(config, state);
        let err = svc
            .call(LlmRequest::Chat(chat_req("gpt-4")))
            .await
            .expect_err("should reject over-budget request");
        assert!(matches!(err, LiterLlmError::BudgetExceeded { .. }));
    }

    #[tokio::test]
    async fn hard_enforcement_rejects_when_model_limit_exceeded() {
        let state = Arc::new(BudgetState::new());
        // Pre-seed per-model spend above the model limit.
        state
            .model_spend
            .entry("gpt-4".to_owned())
            .or_insert_with(|| AtomicU64::new(0))
            .store(usd_to_microcents(2.0), Ordering::Relaxed);

        let mut limits = HashMap::new();
        limits.insert("gpt-4".into(), 1.0);

        let config = BudgetConfig {
            global_limit: None,
            model_limits: limits,
            enforcement: Enforcement::Hard,
        };

        let mut svc = build_service(config, state);
        let err = svc
            .call(LlmRequest::Chat(chat_req("gpt-4")))
            .await
            .expect_err("should reject over-budget model request");

        match &err {
            LiterLlmError::BudgetExceeded { model, .. } => {
                assert_eq!(model.as_deref(), Some("gpt-4"));
            }
            other => panic!("expected BudgetExceeded, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn hard_enforcement_allows_requests_under_limit() {
        let state = Arc::new(BudgetState::new());
        let config = BudgetConfig {
            global_limit: Some(100.0),
            enforcement: Enforcement::Hard,
            ..Default::default()
        };

        let mut svc = build_service(config, state);
        let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
        assert!(resp.is_ok(), "request under budget should succeed");
    }

    // ── Soft enforcement ────────────────────────────────────────────────────

    #[tokio::test]
    async fn soft_enforcement_allows_requests_over_global_limit() {
        let state = Arc::new(BudgetState::new());
        state.global_spend.store(usd_to_microcents(100.0), Ordering::Relaxed);

        let config = BudgetConfig {
            global_limit: Some(5.0),
            enforcement: Enforcement::Soft,
            ..Default::default()
        };

        let mut svc = build_service(config, state);
        let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
        assert!(resp.is_ok(), "soft mode should never reject");
    }

    #[tokio::test]
    async fn soft_enforcement_allows_requests_over_model_limit() {
        let state = Arc::new(BudgetState::new());
        state
            .model_spend
            .entry("gpt-4".to_owned())
            .or_insert_with(|| AtomicU64::new(0))
            .store(usd_to_microcents(10.0), Ordering::Relaxed);

        let mut limits = HashMap::new();
        limits.insert("gpt-4".into(), 1.0);

        let config = BudgetConfig {
            global_limit: None,
            model_limits: limits,
            enforcement: Enforcement::Soft,
        };

        let mut svc = build_service(config, state);
        let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
        assert!(resp.is_ok(), "soft mode should never reject");
    }

    // ── Cost accumulation ───────────────────────────────────────────────────

    #[tokio::test]
    async fn accumulates_cost_after_response() {
        let state = Arc::new(BudgetState::new());
        let config = BudgetConfig {
            global_limit: Some(100.0),
            enforcement: Enforcement::Hard,
            ..Default::default()
        };

        let mut svc = build_service(config, Arc::clone(&state));
        // MockClient returns usage: prompt=10, completion=5 for the model.
        svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.unwrap();

        // gpt-4 pricing: input=0.00003/token, output=0.00006/token
        // 10 * 0.00003 + 5 * 0.00006 = 0.0003 + 0.0003 = 0.0006
        assert!(state.global_spend() > 0.0, "global spend should be recorded");
        assert!(state.model_spend("gpt-4") > 0.0, "model spend should be recorded");
    }

    // ── Per-model limits (independent) ──────────────────────────────────────

    #[tokio::test]
    async fn per_model_limits_are_independent() {
        let state = Arc::new(BudgetState::new());
        // Set gpt-4 over its limit, but gpt-3.5-turbo has no model limit.
        state
            .model_spend
            .entry("gpt-4".to_owned())
            .or_insert_with(|| AtomicU64::new(0))
            .store(usd_to_microcents(5.0), Ordering::Relaxed);

        let mut limits = HashMap::new();
        limits.insert("gpt-4".into(), 1.0);

        let config = BudgetConfig {
            global_limit: None,
            model_limits: limits,
            enforcement: Enforcement::Hard,
        };

        let mut svc = build_service(config, state);

        // gpt-4 should be rejected.
        let err = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
        assert!(err.is_err(), "gpt-4 should be rejected");

        // gpt-3.5-turbo has no per-model limit, should succeed.
        let ok = svc.call(LlmRequest::Chat(chat_req("gpt-3.5-turbo"))).await;
        assert!(ok.is_ok(), "gpt-3.5-turbo should not be limited");
    }

    // ── Reset ───────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn reset_clears_all_counters() {
        let state = Arc::new(BudgetState::new());
        state.global_spend.store(usd_to_microcents(50.0), Ordering::Relaxed);
        state
            .model_spend
            .entry("gpt-4".to_owned())
            .or_insert_with(|| AtomicU64::new(0))
            .store(usd_to_microcents(25.0), Ordering::Relaxed);

        assert!(state.global_spend() > 0.0);
        assert!(state.model_spend("gpt-4") > 0.0);

        state.reset();

        assert_eq!(state.global_spend(), 0.0, "global spend should be zero after reset");
        assert_eq!(
            state.model_spend("gpt-4"),
            0.0,
            "model spend should be zero after reset"
        );
    }

    // ── Reset then allow ────────────────────────────────────────────────────

    #[tokio::test]
    async fn reset_allows_previously_blocked_requests() {
        let state = Arc::new(BudgetState::new());
        state.global_spend.store(usd_to_microcents(10.0), Ordering::Relaxed);

        let config = BudgetConfig {
            global_limit: Some(5.0),
            enforcement: Enforcement::Hard,
            ..Default::default()
        };

        let mut svc = build_service(config, Arc::clone(&state));

        // Should be rejected.
        let err = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
        assert!(err.is_err());

        // Reset and retry.
        state.reset();
        let ok = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
        assert!(ok.is_ok(), "should succeed after reset");
    }

    // ── Unlimited config ────────────────────────────────────────────────────

    #[tokio::test]
    async fn unlimited_config_allows_all_requests() {
        let state = Arc::new(BudgetState::new());
        let config = BudgetConfig::default();

        let mut svc = build_service(config, state);
        for _ in 0..20 {
            assert!(svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.is_ok());
        }
    }

    // ── Propagates inner errors ─────────────────────────────────────────────

    #[tokio::test]
    async fn propagates_inner_service_errors() {
        let state = Arc::new(BudgetState::new());
        let config = BudgetConfig {
            global_limit: Some(100.0),
            enforcement: Enforcement::Hard,
            ..Default::default()
        };

        let layer = BudgetLayer::new(config, state);
        let inner = LlmService::new(MockClient::failing_timeout());
        let mut svc = layer.layer(inner);

        let err = svc
            .call(LlmRequest::Chat(chat_req("gpt-4")))
            .await
            .expect_err("should propagate inner error");
        assert!(matches!(err, LiterLlmError::Timeout));
    }
}
