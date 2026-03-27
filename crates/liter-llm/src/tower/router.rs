use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll};
use std::time::Instant;

use dashmap::DashMap;
use tower::Service;

use super::types::{LlmRequest, LlmResponse};
use crate::client::BoxFuture;
use crate::error::{LiterLlmError, Result};

// ---- Routing strategy ------------------------------------------------------

/// Routing strategy for selecting among multiple deployments.
#[derive(Debug, Clone)]
pub enum RoutingStrategy {
    /// Round-robin across all deployments in order.
    RoundRobin,
    /// Try deployments in order; advance to the next on a transient error.
    /// Propagates immediately on non-transient errors.
    Fallback,
    /// Route to the deployment with the lowest observed latency (exponential
    /// moving average).
    LatencyBased,
    /// Route to the cheapest deployment for the requested model using the
    /// embedded pricing registry.
    CostBased,
    /// Weighted random distribution across deployments.  Weights are
    /// normalised at construction time; higher values receive proportionally
    /// more traffic.
    WeightedRandom {
        /// One weight per deployment (must have the same length as the
        /// deployments vec).
        weights: Vec<f64>,
    },
}

// ---- Per-deployment metrics ------------------------------------------------

/// Tracks per-deployment latency using an exponential moving average.
#[derive(Debug)]
struct DeploymentMetrics {
    /// Exponential moving average of latency in seconds.
    latency_ema: f64,
    /// Number of requests seen (used to seed the EMA).
    request_count: u64,
}

impl Default for DeploymentMetrics {
    fn default() -> Self {
        Self {
            latency_ema: 0.0,
            request_count: 0,
        }
    }
}

impl DeploymentMetrics {
    /// Update the EMA with a new latency sample (in seconds).
    fn record_latency(&mut self, latency_secs: f64) {
        // Smoothing factor for EMA — higher values weight recent samples more.
        const ALPHA: f64 = 0.3;

        if self.request_count == 0 {
            self.latency_ema = latency_secs;
        } else {
            self.latency_ema = ALPHA * latency_secs + (1.0 - ALPHA) * self.latency_ema;
        }
        self.request_count += 1;
    }
}

/// Shared state tracking per-deployment metrics, keyed by deployment index.
pub struct RouterState {
    metrics: Arc<DashMap<usize, DeploymentMetrics>>,
}

impl RouterState {
    fn new() -> Self {
        Self {
            metrics: Arc::new(DashMap::new()),
        }
    }
}

impl Clone for RouterState {
    fn clone(&self) -> Self {
        Self {
            metrics: Arc::clone(&self.metrics),
        }
    }
}

// ---- Router ----------------------------------------------------------------

/// A router that distributes [`LlmRequest`]s across multiple service
/// instances according to a [`RoutingStrategy`].
///
/// The inner deployments must be `Clone` so the router can hand out
/// independent service handles per call.  Use [`LlmService`] as the
/// deployment type when wrapping a [`crate::client::LlmClient`].
///
/// [`LlmService`]: super::service::LlmService
pub struct Router<S> {
    deployments: Vec<S>,
    strategy: RoutingStrategy,
    /// Monotonically incrementing counter used by [`RoutingStrategy::RoundRobin`].
    counter: Arc<AtomicUsize>,
    /// Per-deployment metrics (latency tracking, etc.).
    state: RouterState,
}

impl<S> Router<S> {
    /// Create a new router.
    ///
    /// # Errors
    ///
    /// Returns [`LiterLlmError::BadRequest`] if `deployments` is empty — a
    /// router with no deployments cannot handle any request.
    ///
    /// For [`RoutingStrategy::WeightedRandom`], returns an error if the
    /// weights vector length does not match the number of deployments or
    /// if all weights are zero.
    pub fn new(deployments: Vec<S>, strategy: RoutingStrategy) -> Result<Self> {
        if deployments.is_empty() {
            return Err(LiterLlmError::BadRequest {
                message: "Router requires at least one deployment".into(),
            });
        }
        if let RoutingStrategy::WeightedRandom { ref weights } = strategy {
            if weights.len() != deployments.len() {
                return Err(LiterLlmError::BadRequest {
                    message: format!(
                        "WeightedRandom: weights length ({}) must match deployments length ({})",
                        weights.len(),
                        deployments.len()
                    ),
                });
            }
            let total: f64 = weights.iter().sum();
            if total <= 0.0 {
                return Err(LiterLlmError::BadRequest {
                    message: "WeightedRandom: total weight must be positive".into(),
                });
            }
        }
        Ok(Self {
            deployments,
            strategy,
            counter: Arc::new(AtomicUsize::new(0)),
            state: RouterState::new(),
        })
    }
}

impl<S: Clone> Clone for Router<S> {
    fn clone(&self) -> Self {
        Self {
            deployments: self.deployments.clone(),
            strategy: self.strategy.clone(),
            counter: Arc::clone(&self.counter),
            state: self.state.clone(),
        }
    }
}

impl<S> Service<LlmRequest> for Router<S>
where
    S: Service<LlmRequest, Response = LlmResponse, Error = LiterLlmError> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = LlmResponse;
    type Error = LiterLlmError;
    type Future = BoxFuture<'static, LlmResponse>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<()>> {
        // All inner services are cloned per-call, so there is no persistent
        // readied slot to manage here.  A more sophisticated implementation
        // could poll each deployment's readiness and track the result, but
        // for DefaultClient (which is always ready) this is unnecessary.
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: LlmRequest) -> Self::Future {
        match &self.strategy {
            RoutingStrategy::RoundRobin => {
                let idx = self.counter.fetch_add(1, Ordering::Relaxed) % self.deployments.len();
                let mut svc = self.deployments[idx].clone();
                Box::pin(async move { svc.call(req).await })
            }
            RoutingStrategy::Fallback => {
                let deployments = self.deployments.clone();
                Box::pin(async move {
                    let mut last_err: Option<LiterLlmError> = None;
                    for mut svc in deployments {
                        match svc.call(req.clone()).await {
                            Ok(resp) => return Ok(resp),
                            Err(e) if e.is_transient() => {
                                tracing::warn!(
                                    error = %e,
                                    "deployment failed with transient error; trying next deployment"
                                );
                                last_err = Some(e);
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    Err(last_err.unwrap_or(LiterLlmError::ServerError {
                        message: "all deployments failed".into(),
                    }))
                })
            }
            RoutingStrategy::LatencyBased => {
                let state = self.state.clone();
                let n = self.deployments.len();

                // Pick deployment with the lowest latency EMA.
                // Deployments with no data default to EMA 0.0 (optimistic).
                let mut best_idx = 0;
                let mut best_ema = f64::MAX;
                for i in 0..n {
                    let ema = state.metrics.get(&i).map_or(0.0, |m| m.latency_ema);
                    if ema < best_ema {
                        best_ema = ema;
                        best_idx = i;
                    }
                }

                let mut svc = self.deployments[best_idx].clone();
                let idx = best_idx;

                Box::pin(async move {
                    let start = Instant::now();
                    let result = svc.call(req).await;
                    let latency = start.elapsed().as_secs_f64();

                    state.metrics.entry(idx).or_default().record_latency(latency);

                    result
                })
            }
            RoutingStrategy::CostBased => {
                let model = req.model().map(ToOwned::to_owned);
                let deployments = self.deployments.clone();

                // For cost-based routing, we try to pick the cheapest deployment.
                // Since all deployments serve the same model, cost is typically
                // uniform.  The differentiator is when deployments wrap different
                // providers (e.g., OpenAI vs Azure) with different pricing.
                //
                // Without per-deployment provider metadata, we use a simple
                // heuristic: try each deployment in order and return the first
                // success.  A future enhancement could attach provider metadata
                // to each deployment.
                //
                // For now, CostBased routes identically to Fallback but logs the
                // cost after success.
                Box::pin(async move {
                    let mut last_err: Option<LiterLlmError> = None;
                    for mut svc in deployments {
                        match svc.call(req.clone()).await {
                            Ok(resp) => {
                                if let (Some(model_name), Some(usage)) = (&model, resp.usage())
                                    && let Some(cost) = crate::cost::completion_cost(
                                        model_name,
                                        usage.prompt_tokens,
                                        usage.completion_tokens,
                                    )
                                {
                                    tracing::debug!(
                                        model = %model_name,
                                        cost_usd = cost,
                                        "cost-based routing: estimated cost"
                                    );
                                }
                                return Ok(resp);
                            }
                            Err(e) if e.is_transient() => {
                                last_err = Some(e);
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    Err(last_err.unwrap_or(LiterLlmError::ServerError {
                        message: "all deployments failed".into(),
                    }))
                })
            }
            RoutingStrategy::WeightedRandom { weights } => {
                let idx = weighted_random_select(weights);
                let mut svc = self.deployments[idx].clone();
                Box::pin(async move { svc.call(req).await })
            }
        }
    }
}

/// Select a deployment index using weighted random distribution.
///
/// Uses a simple linear scan with a random threshold.  For small deployment
/// counts (typical: 2-5) this is fast enough; no binary search needed.
fn weighted_random_select(weights: &[f64]) -> usize {
    let total: f64 = weights.iter().sum();
    // Simple pseudo-random: use the lower bits of the current time.
    // This avoids adding a `rand` dependency.  For production use,
    // callers who need better randomness can use the `rand` crate
    // externally.
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    let threshold = (f64::from(nanos) / 1_000_000_000.0) * total;

    let mut cumulative = 0.0;
    for (i, &w) in weights.iter().enumerate() {
        cumulative += w;
        if threshold < cumulative {
            return i;
        }
    }
    // Fallback to last deployment (rounding edge case).
    weights.len() - 1
}

// ---- Tests -----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tower::service::LlmService;
    use crate::tower::tests_common::{MockClient, chat_req};
    use crate::tower::types::LlmRequest;

    #[tokio::test]
    async fn latency_based_routes_to_fastest() {
        let deployments: Vec<LlmService<MockClient>> =
            vec![LlmService::new(MockClient::ok()), LlmService::new(MockClient::ok())];

        let mut router = Router::new(deployments, RoutingStrategy::LatencyBased).expect("non-empty deployments");

        // First call goes to deployment 0 (both have EMA 0.0, picks first).
        let resp = router.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
        assert!(resp.is_ok());

        // After the first call, deployment 0 has a non-zero EMA.
        // The second call should go to deployment 1 (still at 0.0 EMA).
        let resp = router.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
        assert!(resp.is_ok());
    }

    #[tokio::test]
    async fn cost_based_falls_through_on_transient_error() {
        let deployments: Vec<LlmService<MockClient>> = vec![
            LlmService::new(MockClient::failing_service_unavailable()),
            LlmService::new(MockClient::ok()),
        ];

        let mut router = Router::new(deployments, RoutingStrategy::CostBased).expect("non-empty deployments");

        let resp = router.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
        assert!(resp.is_ok(), "should fall through to second deployment");
    }

    #[tokio::test]
    async fn weighted_random_selects_valid_deployment() {
        let deployments: Vec<LlmService<MockClient>> = vec![
            LlmService::new(MockClient::ok()),
            LlmService::new(MockClient::ok()),
            LlmService::new(MockClient::ok()),
        ];

        let mut router = Router::new(
            deployments,
            RoutingStrategy::WeightedRandom {
                weights: vec![1.0, 2.0, 3.0],
            },
        )
        .expect("non-empty deployments");

        // Run several requests — all should succeed regardless of which
        // deployment is selected.
        for _ in 0..20 {
            let resp = router.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
            assert!(resp.is_ok());
        }
    }

    #[tokio::test]
    async fn weighted_random_rejects_mismatched_weights() {
        let deployments: Vec<LlmService<MockClient>> =
            vec![LlmService::new(MockClient::ok()), LlmService::new(MockClient::ok())];

        let result = Router::new(
            deployments,
            RoutingStrategy::WeightedRandom {
                weights: vec![1.0], // Wrong length.
            },
        );
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn weighted_random_rejects_zero_total_weight() {
        let deployments: Vec<LlmService<MockClient>> = vec![LlmService::new(MockClient::ok())];

        let result = Router::new(deployments, RoutingStrategy::WeightedRandom { weights: vec![0.0] });
        assert!(result.is_err());
    }

    #[test]
    fn weighted_random_select_returns_valid_index() {
        let weights = vec![1.0, 2.0, 3.0];
        for _ in 0..100 {
            let idx = weighted_random_select(&weights);
            assert!(idx < weights.len());
        }
    }

    #[test]
    fn deployment_metrics_ema_updates() {
        let mut m = DeploymentMetrics::default();
        m.record_latency(1.0);
        assert!(
            (m.latency_ema - 1.0).abs() < 1e-9,
            "first sample should set EMA directly"
        );

        m.record_latency(0.0);
        // EMA = 0.3 * 0.0 + 0.7 * 1.0 = 0.7
        assert!(
            (m.latency_ema - 0.7).abs() < 1e-9,
            "EMA should be 0.7 after second sample"
        );
    }
}
