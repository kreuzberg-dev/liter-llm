use std::task::{Context, Poll};

use tower::Layer;
use tower::Service;

use super::types::{LlmRequest, LlmResponse};
use crate::client::BoxFuture;
use crate::error::{LiterLmError, Result};

/// Tower [`Layer`] that routes to a fallback service when the primary service
/// returns an error.
///
/// Only transient errors trigger the fallback — specifically:
/// [`LiterLmError::RateLimited`], [`LiterLmError::ServiceUnavailable`],
/// [`LiterLmError::Timeout`], and [`LiterLmError::ServerError`].
/// Authentication or bad-request errors are propagated directly without
/// consulting the fallback because retrying on a different service would
/// produce the same result.
pub struct FallbackLayer<F> {
    fallback: F,
}

impl<F> FallbackLayer<F> {
    /// Create a new fallback layer with the given fallback service.
    #[must_use]
    pub fn new(fallback: F) -> Self {
        Self { fallback }
    }
}

impl<S, F> Layer<S> for FallbackLayer<F>
where
    F: Clone,
{
    type Service = FallbackService<S, F>;

    fn layer(&self, primary: S) -> Self::Service {
        FallbackService {
            primary,
            // Clone the fallback so the produced service owns it independently.
            fallback: self.fallback.clone(),
        }
    }
}

/// Tower service produced by [`FallbackLayer`].
pub struct FallbackService<S, F> {
    primary: S,
    fallback: F,
}

impl<S, F> Clone for FallbackService<S, F>
where
    S: Clone,
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            primary: self.primary.clone(),
            fallback: self.fallback.clone(),
        }
    }
}

impl<S, F> Service<LlmRequest> for FallbackService<S, F>
where
    S: Service<LlmRequest, Response = LlmResponse, Error = LiterLmError> + Send + 'static,
    S::Future: Send + 'static,
    F: Service<LlmRequest, Response = LlmResponse, Error = LiterLmError> + Clone + Send + 'static,
    F::Future: Send + 'static,
{
    type Response = LlmResponse;
    type Error = LiterLmError;
    type Future = BoxFuture<'static, LlmResponse>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        // Tower contract: poll_ready should prepare exactly one service for a
        // subsequent call.  Ideally we would only poll the primary here and
        // poll the fallback lazily in `call`.  However, because `call` takes
        // `&mut self` and must return a `'static` future (no reference to
        // `self`), we cannot hold a mutable borrow across the await point.
        // For our concrete use case (DefaultClient is always ready), polling
        // both here is not harmful — neither service blocks and both remain
        // ready until the next call.  Callers that compose non-trivially-ready
        // services should use a dedicated load-balancing layer instead.
        match self.primary.poll_ready(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
            Poll::Ready(Ok(())) => {}
        }
        self.fallback.poll_ready(cx)
    }

    fn call(&mut self, req: LlmRequest) -> Self::Future {
        // Clone the request so it can be replayed on the fallback if needed.
        let fallback_req = req.clone();
        let primary_fut = self.primary.call(req);

        // `poll_ready` readied `self.fallback` for exactly one call.
        // We move the readied service into the async block (so the future is
        // 'static) and replace it with a fresh clone for the *next* call cycle.
        // Tower's contract guarantees at most one `call` per `poll_ready`, so
        // the fresh clone is not used until `poll_ready` runs again.
        let fresh = self.fallback.clone();
        let mut readied_fallback = std::mem::replace(&mut self.fallback, fresh);

        Box::pin(async move {
            match primary_fut.await {
                Ok(resp) => Ok(resp),
                Err(e) if e.is_transient() => {
                    tracing::warn!(
                        error = %e,
                        "primary service failed with transient error; trying fallback"
                    );
                    readied_fallback.call(fallback_req).await
                }
                Err(e) => Err(e),
            }
        })
    }
}
