//! Health check middleware.
//!
//! [`HealthCheckLayer`] wraps a service and spawns a background task that
//! periodically probes the service by sending a [`LlmRequest::ListModels`]
//! request.  If the probe fails, the service is marked unhealthy and incoming
//! requests are immediately rejected with [`LiterLlmError::ServiceUnavailable`].
//!
//! The health flag is an [`AtomicBool`] shared between the background probe
//! task and the request path, so checking health adds minimal overhead (a
//! single atomic load).

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll};
use std::time::Duration;

use tower::{Layer, Service};

use super::types::{LlmRequest, LlmResponse};
use crate::client::BoxFuture;
use crate::error::{LiterLlmError, Result};

// ---- Layer -----------------------------------------------------------------

/// Tower [`Layer`] that monitors service health via periodic probes.
///
/// The background health-check task is spawned when the layer wraps a service
/// (i.e. when [`Layer::layer`] is called).  The task runs until the
/// [`HealthCheckService`] (and all its clones) are dropped.
pub struct HealthCheckLayer {
    interval: Duration,
}

impl HealthCheckLayer {
    /// Create a new health-check layer that probes every `interval`.
    #[must_use]
    pub fn new(interval: Duration) -> Self {
        Self { interval }
    }
}

impl<S> Layer<S> for HealthCheckLayer
where
    S: Service<LlmRequest, Response = LlmResponse, Error = LiterLlmError> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Service = HealthCheckService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        let healthy = Arc::new(AtomicBool::new(true));

        // Spawn the background probe task.
        let probe_svc = inner.clone();
        let probe_healthy = Arc::clone(&healthy);
        let interval = self.interval;

        tokio::spawn(async move {
            run_health_probe(probe_svc, probe_healthy, interval).await;
        });

        HealthCheckService { inner, healthy }
    }
}

// ---- Background probe ------------------------------------------------------

async fn run_health_probe<S>(mut svc: S, healthy: Arc<AtomicBool>, interval: Duration)
where
    S: Service<LlmRequest, Response = LlmResponse, Error = LiterLlmError> + Send + 'static,
    S::Future: Send + 'static,
{
    loop {
        tokio::time::sleep(interval).await;

        // If the Arc is held only by us, all service clones have been dropped
        // and we should stop probing.
        if Arc::strong_count(&healthy) <= 1 {
            break;
        }

        let result = svc.call(LlmRequest::ListModels).await;
        let is_healthy = result.is_ok();
        healthy.store(is_healthy, Ordering::Release);

        if !is_healthy {
            tracing::warn!("health check failed; marking service as unhealthy");
        }
    }
}

// ---- Service ---------------------------------------------------------------

/// Tower service produced by [`HealthCheckLayer`].
pub struct HealthCheckService<S> {
    inner: S,
    healthy: Arc<AtomicBool>,
}

impl<S: Clone> Clone for HealthCheckService<S> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            healthy: Arc::clone(&self.healthy),
        }
    }
}

impl<S> HealthCheckService<S> {
    /// Returns `true` if the last health probe succeeded.
    #[must_use]
    pub fn is_healthy(&self) -> bool {
        self.healthy.load(Ordering::Acquire)
    }
}

impl<S> Service<LlmRequest> for HealthCheckService<S>
where
    S: Service<LlmRequest, Response = LlmResponse, Error = LiterLlmError> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = LlmResponse;
    type Error = LiterLlmError;
    type Future = BoxFuture<'static, LlmResponse>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        if !self.healthy.load(Ordering::Acquire) {
            return Poll::Ready(Err(LiterLlmError::ServiceUnavailable {
                message: "service is unhealthy (health check failed)".into(),
            }));
        }
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: LlmRequest) -> Self::Future {
        if !self.healthy.load(Ordering::Acquire) {
            return Box::pin(async {
                Err(LiterLlmError::ServiceUnavailable {
                    message: "service is unhealthy (health check failed)".into(),
                })
            });
        }
        let fut = self.inner.call(req);
        Box::pin(fut)
    }
}

// ---- Tests -----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::sync::atomic::Ordering;

    use tower::Service as _;

    use super::*;
    use crate::tower::service::LlmService;
    use crate::tower::tests_common::{MockClient, chat_req};
    use crate::tower::types::LlmRequest;

    #[tokio::test]
    async fn healthy_service_passes_through() {
        let inner = LlmService::new(MockClient::ok());
        let healthy = Arc::new(AtomicBool::new(true));
        let mut svc = HealthCheckService {
            inner,
            healthy: Arc::clone(&healthy),
        };

        let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
        assert!(resp.is_ok());
    }

    #[tokio::test]
    async fn unhealthy_service_rejects_requests() {
        let inner = LlmService::new(MockClient::ok());
        let healthy = Arc::new(AtomicBool::new(false));
        let mut svc = HealthCheckService {
            inner,
            healthy: Arc::clone(&healthy),
        };

        let err = svc
            .call(LlmRequest::Chat(chat_req("gpt-4")))
            .await
            .expect_err("unhealthy service should reject");
        assert!(matches!(err, LiterLlmError::ServiceUnavailable { .. }));
    }

    #[tokio::test]
    async fn is_healthy_reflects_flag() {
        let inner = LlmService::new(MockClient::ok());
        let healthy = Arc::new(AtomicBool::new(true));
        let svc = HealthCheckService {
            inner,
            healthy: Arc::clone(&healthy),
        };

        assert!(svc.is_healthy());
        healthy.store(false, Ordering::Release);
        assert!(!svc.is_healthy());
    }

    #[tokio::test]
    async fn recovery_after_becoming_healthy_again() {
        let inner = LlmService::new(MockClient::ok());
        let healthy = Arc::new(AtomicBool::new(false));
        let mut svc = HealthCheckService {
            inner,
            healthy: Arc::clone(&healthy),
        };

        // Unhealthy — should reject.
        assert!(svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.is_err());

        // Mark as healthy again.
        healthy.store(true, Ordering::Release);
        assert!(svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.is_ok());
    }
}
