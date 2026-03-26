//! Deployment cooldown middleware.
//!
//! [`CooldownLayer`] wraps a service and implements a cooldown period after
//! transient errors.  When the inner service returns a transient error (as
//! determined by [`LiterLmError::is_transient`]), the service is marked as
//! cooling down for a configurable duration.  During the cooldown period,
//! incoming requests are immediately rejected with
//! [`LiterLmError::ServiceUnavailable`] without calling the inner service.

use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use tokio::sync::RwLock;
use tower::{Layer, Service};

use super::types::{LlmRequest, LlmResponse};
use crate::client::BoxFuture;
use crate::error::{LiterLmError, Result};

// ---- State -----------------------------------------------------------------

struct CooldownState {
    /// `None` when not cooling down, `Some(start)` when a cooldown is active.
    cooldown_start: Option<Instant>,
}

// ---- Layer -----------------------------------------------------------------

/// Tower [`Layer`] that applies a cooldown period after transient errors.
pub struct CooldownLayer {
    duration: Duration,
}

impl CooldownLayer {
    /// Create a new cooldown layer.
    ///
    /// After a transient error, the wrapped service will reject all requests
    /// for `duration` before allowing traffic through again.
    #[must_use]
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }
}

impl<S> Layer<S> for CooldownLayer {
    type Service = CooldownService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CooldownService {
            inner,
            duration: self.duration,
            state: Arc::new(RwLock::new(CooldownState { cooldown_start: None })),
        }
    }
}

// ---- Service ---------------------------------------------------------------

/// Tower service produced by [`CooldownLayer`].
pub struct CooldownService<S> {
    inner: S,
    duration: Duration,
    state: Arc<RwLock<CooldownState>>,
}

impl<S: Clone> Clone for CooldownService<S> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            duration: self.duration,
            state: Arc::clone(&self.state),
        }
    }
}

impl<S> Service<LlmRequest> for CooldownService<S>
where
    S: Service<LlmRequest, Response = LlmResponse, Error = LiterLmError> + Send + Clone + 'static,
    S::Future: Send + 'static,
{
    type Response = LlmResponse;
    type Error = LiterLmError;
    type Future = BoxFuture<'static, LlmResponse>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: LlmRequest) -> Self::Future {
        let state = Arc::clone(&self.state);
        let duration = self.duration;
        // IMPORTANT: do NOT call self.inner.call(req) here — the inner service
        // must only be invoked *after* the cooldown check passes inside the
        // async block.  Calling it eagerly would send the request even when the
        // service is in a cooldown period.
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Check whether we are in a cooldown period.
            {
                let read = state.read().await;
                if let Some(start) = read.cooldown_start {
                    if start.elapsed() < duration {
                        return Err(LiterLmError::ServiceUnavailable {
                            message: format!(
                                "service is cooling down for {:.0}s after a transient error",
                                duration.as_secs_f64()
                            ),
                        });
                    }
                    // Cooldown has expired — we need to reset it.
                    // Drop the read lock first, then take the write lock.
                    drop(read);
                    let mut write = state.write().await;
                    // Double-check under write lock (another task may have reset it).
                    if let Some(s) = write.cooldown_start
                        && s.elapsed() >= duration
                    {
                        write.cooldown_start = None;
                    }
                }
            }

            // Only call the inner service after cooldown check passes.
            match inner.call(req).await {
                Ok(resp) => Ok(resp),
                Err(e) if e.is_transient() => {
                    // Enter cooldown.
                    let mut write = state.write().await;
                    write.cooldown_start = Some(Instant::now());
                    Err(e)
                }
                Err(e) => Err(e),
            }
        })
    }
}

// ---- Tests -----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use tower::{Layer as _, Service as _};

    use super::*;
    use crate::tower::service::LlmService;
    use crate::tower::tests_common::{MockClient, chat_req};
    use crate::tower::types::LlmRequest;

    #[tokio::test]
    async fn passes_through_on_success() {
        let layer = CooldownLayer::new(Duration::from_secs(10));
        let inner = LlmService::new(MockClient::ok());
        let mut svc = layer.layer(inner);

        let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
        assert!(resp.is_ok());
    }

    #[tokio::test]
    async fn enters_cooldown_after_transient_error() {
        let layer = CooldownLayer::new(Duration::from_secs(60));
        let inner = LlmService::new(MockClient::failing_timeout());
        let mut svc = layer.layer(inner);

        // First call — transient error.
        let err = svc
            .call(LlmRequest::Chat(chat_req("gpt-4")))
            .await
            .expect_err("should fail");
        assert!(matches!(err, LiterLmError::Timeout));

        // Second call — should be rejected with ServiceUnavailable (cooldown).
        let err = svc
            .call(LlmRequest::Chat(chat_req("gpt-4")))
            .await
            .expect_err("should be in cooldown");
        assert!(
            matches!(err, LiterLmError::ServiceUnavailable { .. }),
            "expected ServiceUnavailable during cooldown, got {err:?}"
        );
    }

    #[tokio::test]
    async fn cooldown_expires_after_duration() {
        // Use a zero-second cooldown so it expires immediately.
        let layer = CooldownLayer::new(Duration::from_millis(0));
        let inner = LlmService::new(MockClient::failing_timeout());
        let mut svc = layer.layer(inner);

        // First call — transient error triggers cooldown.
        svc.call(LlmRequest::Chat(chat_req("gpt-4")))
            .await
            .expect_err("should fail");

        // With zero duration, cooldown is already expired.  The next call should
        // reach the inner service (which will fail again with Timeout, not
        // ServiceUnavailable).
        let err = svc
            .call(LlmRequest::Chat(chat_req("gpt-4")))
            .await
            .expect_err("should fail");
        assert!(
            matches!(err, LiterLmError::Timeout),
            "expected Timeout (cooldown expired), got {err:?}"
        );
    }

    #[tokio::test]
    async fn non_transient_error_does_not_trigger_cooldown() {
        let layer = CooldownLayer::new(Duration::from_secs(60));
        let inner = LlmService::new(MockClient::failing_auth());
        let mut svc = layer.layer(inner);

        // First call — non-transient error.
        svc.call(LlmRequest::Chat(chat_req("gpt-4")))
            .await
            .expect_err("should fail");

        // Second call — should reach inner service (not cooldown).
        let err = svc
            .call(LlmRequest::Chat(chat_req("gpt-4")))
            .await
            .expect_err("should fail with auth, not cooldown");
        assert!(
            matches!(err, LiterLmError::BadRequest { .. }),
            "expected BadRequest (auth), not ServiceUnavailable, got {err:?}"
        );
    }
}
