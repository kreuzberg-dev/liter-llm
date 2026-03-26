use std::task::{Context, Poll};

use tower_layer::Layer;
use tower_service::Service;
use tracing::Instrument as _;

use super::types::{LlmRequest, LlmResponse};
use crate::client::BoxFuture;
use crate::error::{LiterLmError, Result};

/// Tower [`Layer`] that wraps a service with OTEL-compatible tracing spans.
///
/// Each call creates an [`tracing::info_span`] named `"llm.request"` with the
/// following fields:
///
/// - `llm.request.type` — `"chat"`, `"chat_stream"`, `"embed"`, or
///   `"list_models"`.
/// - `llm.model` — the model name from the request, or `""` for
///   [`LlmRequest::ListModels`].
/// - `llm.usage.input_tokens` — populated on successful chat / embed
///   responses where usage data is present.
/// - `llm.usage.output_tokens` — populated on successful chat responses.
/// - `error` — set to `"true"` if the inner service returns an error.
pub struct TracingLayer;

impl<S> Layer<S> for TracingLayer {
    type Service = TracingService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TracingService { inner }
    }
}

/// Tower service produced by [`TracingLayer`].
pub struct TracingService<S> {
    inner: S,
}

impl<S> Clone for TracingService<S>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<S> Service<LlmRequest> for TracingService<S>
where
    S: Service<LlmRequest, Response = LlmResponse, Error = LiterLmError> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = LlmResponse;
    type Error = LiterLmError;
    type Future = BoxFuture<'static, LlmResponse>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: LlmRequest) -> Self::Future {
        let request_type = req.request_type();
        let model = req.model().unwrap_or("").to_owned();

        let span = tracing::info_span!(
            "llm.request",
            llm.request.type = request_type,
            llm.model = %model,
            llm.usage.input_tokens = tracing::field::Empty,
            llm.usage.output_tokens = tracing::field::Empty,
            error = tracing::field::Empty,
        );

        let fut = self.inner.call(req);

        // Use `.instrument(span)` rather than `span.enter()` in the async
        // block.  `span.enter()` in an async context is incorrect because the
        // guard is dropped when the future suspends at an await point, causing
        // the span to close prematurely.  `Instrument` attaches the span to
        // the future so it is entered and exited correctly around each poll.
        Box::pin(
            async move {
                match fut.await {
                    Ok(resp) => {
                        // Record usage statistics from the response when available.
                        record_usage(&tracing::Span::current(), &resp);
                        Ok(resp)
                    }
                    Err(e) => {
                        tracing::Span::current().record("error", true);
                        Err(e)
                    }
                }
            }
            .instrument(span),
        )
    }
}

/// Record token-usage fields on the span from the response payload.
fn record_usage(span: &tracing::Span, resp: &LlmResponse) {
    match resp {
        LlmResponse::Chat(r) => {
            if let Some(ref usage) = r.usage {
                span.record("llm.usage.input_tokens", usage.prompt_tokens);
                span.record("llm.usage.output_tokens", usage.completion_tokens);
            }
        }
        LlmResponse::Embed(r) => {
            if let Some(ref usage) = r.usage {
                span.record("llm.usage.input_tokens", usage.prompt_tokens);
            }
        }
        // Streaming and model-list responses do not carry aggregated usage.
        LlmResponse::ChatStream(_) | LlmResponse::ListModels(_) => {}
    }
}
