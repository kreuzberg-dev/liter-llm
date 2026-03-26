use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll};

use tower::Service;

use super::types::{LlmRequest, LlmResponse};
use crate::client::BoxFuture;
use crate::error::{LiterLmError, Result};

/// Routing strategy for selecting among multiple deployments.
#[derive(Debug, Clone, Copy)]
pub enum RoutingStrategy {
    /// Round-robin across all deployments in order.
    RoundRobin,
    /// Try deployments in order; advance to the next on a transient error.
    /// Propagates immediately on non-transient errors.
    Fallback,
}

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
}

impl<S> Router<S> {
    /// Create a new router.
    ///
    /// # Errors
    ///
    /// Returns [`LiterLmError::BadRequest`] if `deployments` is empty — a
    /// router with no deployments cannot handle any request.
    pub fn new(deployments: Vec<S>, strategy: RoutingStrategy) -> Result<Self> {
        if deployments.is_empty() {
            return Err(LiterLmError::BadRequest {
                message: "Router requires at least one deployment".into(),
            });
        }
        Ok(Self {
            deployments,
            strategy,
            counter: Arc::new(AtomicUsize::new(0)),
        })
    }
}

impl<S: Clone> Clone for Router<S> {
    fn clone(&self) -> Self {
        Self {
            deployments: self.deployments.clone(),
            strategy: self.strategy,
            counter: Arc::clone(&self.counter),
        }
    }
}

impl<S> Service<LlmRequest> for Router<S>
where
    S: Service<LlmRequest, Response = LlmResponse, Error = LiterLmError> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = LlmResponse;
    type Error = LiterLmError;
    type Future = BoxFuture<'static, LlmResponse>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<()>> {
        // All inner services are cloned per-call, so there is no persistent
        // readied slot to manage here.  A more sophisticated implementation
        // could poll each deployment's readiness and track the result, but
        // for DefaultClient (which is always ready) this is unnecessary.
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: LlmRequest) -> Self::Future {
        match self.strategy {
            RoutingStrategy::RoundRobin => {
                let idx = self.counter.fetch_add(1, Ordering::Relaxed) % self.deployments.len();
                let mut svc = self.deployments[idx].clone();
                Box::pin(async move { svc.call(req).await })
            }
            RoutingStrategy::Fallback => {
                let deployments = self.deployments.clone();
                Box::pin(async move {
                    let mut last_err: Option<LiterLmError> = None;
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
                    Err(last_err.unwrap_or(LiterLmError::ServerError {
                        message: "all deployments failed".into(),
                    }))
                })
            }
        }
    }
}
