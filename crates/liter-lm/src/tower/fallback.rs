use std::task::{Context, Poll};

use tower_layer::Layer;
use tower_service::Service;

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
    F: Service<LlmRequest, Response = LlmResponse, Error = LiterLmError> + Send + 'static,
    F::Future: Send + 'static,
{
    type Response = LlmResponse;
    type Error = LiterLmError;
    type Future = BoxFuture<'static, LlmResponse>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        // Both services must be ready; return the first not-ready result.
        match self.primary.poll_ready(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
            Poll::Ready(Ok(())) => {}
        }
        self.fallback.poll_ready(cx)
    }

    fn call(&mut self, req: LlmRequest) -> Self::Future {
        // Clone the request so it can be replayed on the fallback if needed.
        let fallback_req = clone_request(&req);
        let primary_fut = self.primary.call(req);
        let fallback_fut = self.fallback.call(fallback_req);

        Box::pin(async move {
            match primary_fut.await {
                Ok(resp) => Ok(resp),
                Err(e) if is_transient(&e) => {
                    tracing::warn!(
                        error = %e,
                        "primary service failed with transient error; trying fallback"
                    );
                    fallback_fut.await
                }
                Err(e) => Err(e),
            }
        })
    }
}

/// Returns `true` for errors that are worth retrying on a different service.
fn is_transient(e: &LiterLmError) -> bool {
    matches!(
        e,
        LiterLmError::RateLimited { .. }
            | LiterLmError::ServiceUnavailable { .. }
            | LiterLmError::Timeout
            | LiterLmError::ServerError { .. }
    )
}

/// Produce a clone of an [`LlmRequest`] for replay on the fallback service.
fn clone_request(req: &LlmRequest) -> LlmRequest {
    match req {
        LlmRequest::Chat(r) => LlmRequest::Chat(r.clone()),
        LlmRequest::ChatStream(r) => LlmRequest::ChatStream(r.clone()),
        LlmRequest::Embed(r) => LlmRequest::Embed(r.clone()),
        LlmRequest::ListModels => LlmRequest::ListModels,
    }
}
