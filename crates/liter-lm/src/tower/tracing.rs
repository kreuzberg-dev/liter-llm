use std::task::{Context, Poll};

use tower_layer::Layer;
use tower_service::Service;
use tracing::Instrument as _;

use super::types::{LlmRequest, LlmResponse};
use crate::client::BoxFuture;
use crate::error::{LiterLmError, Result};
use crate::types::FinishReason;

/// Tower [`Layer`] that wraps a service with OpenTelemetry GenAI semantic
/// convention tracing spans.
///
/// Each call creates a [`tracing::info_span`] named `"gen_ai"` with the
/// following attributes:
///
/// - `gen_ai.operation.name` — `"chat"`, `"embeddings"`, or `"list_models"`.
/// - `gen_ai.request.model` — the model name from the request, or `""` for
///   [`LlmRequest::ListModels`].
/// - `gen_ai.system` — the provider prefix extracted from the model name (e.g.
///   `"openai"` for `"openai/gpt-4"`), or `""` when absent.
/// - `gen_ai.usage.input_tokens` — populated on successful chat / embed
///   responses where usage data is present.
/// - `gen_ai.usage.output_tokens` — populated on successful chat responses.
/// - `gen_ai.response.id` — the completion ID from the response.
/// - `gen_ai.response.model` — the actual model used (may differ from requested).
/// - `gen_ai.response.finish_reasons` — space-separated finish reasons from
///   all choices (e.g. `"stop"`).
/// - `error.type` — set to the error variant name if the inner service returns
///   an error.
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
        let operation_name = req.operation_name();
        // Borrow the model string from the request; split_once gives a &str
        // slice so we avoid an extra allocation for the provider prefix.
        let model_str = req.model().unwrap_or("");
        let system = model_str.split_once('/').map_or("", |(prefix, _)| prefix);
        // Clone once so the span owns the string values (required by tracing
        // macros, which store field values inside the span).
        let model = model_str.to_owned();

        let span = tracing::info_span!(
            "gen_ai",
            gen_ai.operation.name = operation_name,
            gen_ai.request.model = %model,
            gen_ai.system = system,
            gen_ai.usage.input_tokens = tracing::field::Empty,
            gen_ai.usage.output_tokens = tracing::field::Empty,
            gen_ai.response.id = tracing::field::Empty,
            gen_ai.response.model = tracing::field::Empty,
            gen_ai.usage.cost = tracing::field::Empty,
            gen_ai.response.finish_reasons = tracing::field::Empty,
            error.type = tracing::field::Empty,
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
                        // Record usage statistics and response metadata from the response when available.
                        record_response(&tracing::Span::current(), &resp);
                        Ok(resp)
                    }
                    Err(e) => {
                        tracing::Span::current().record("error.type", e.error_type());
                        Err(e)
                    }
                }
            }
            .instrument(span),
        )
    }
}

/// Re-export `tracing_opentelemetry` when the `otel` feature is active.
///
/// This lets callers compose a subscriber that exports spans to an
/// OpenTelemetry collector without taking a direct dependency on the crate:
///
/// ```rust,ignore
/// use liter_lm::tower::tracing::otel::tracing_opentelemetry::OpenTelemetryLayer;
/// ```
#[cfg(feature = "otel")]
pub use tracing_opentelemetry;

/// Re-export `opentelemetry` when the `otel` feature is active.
///
/// Provides access to tracer/provider types needed to build a full
/// OpenTelemetry pipeline (e.g. `opentelemetry::global::tracer`).
#[cfg(feature = "otel")]
pub use opentelemetry;

/// Record span attributes from the response according to GenAI semantic conventions.
fn record_response(span: &tracing::Span, resp: &LlmResponse) {
    match resp {
        LlmResponse::Chat(r) => {
            span.record("gen_ai.response.id", r.id.as_str());
            span.record("gen_ai.response.model", r.model.as_str());

            let finish_reasons =
                finish_reasons_str(&r.choices.iter().map(|c| c.finish_reason.as_ref()).collect::<Vec<_>>());
            if !finish_reasons.is_empty() {
                span.record("gen_ai.response.finish_reasons", finish_reasons.as_str());
            }
        }
        LlmResponse::Embed(r) => {
            span.record("gen_ai.response.model", r.model.as_str());
        }
        // Streaming and model-list responses do not carry aggregated usage or response metadata.
        LlmResponse::ChatStream(_) | LlmResponse::ListModels(_) => {}
    }

    // Record usage tokens from the shared accessor — avoids duplicating the
    // match arms that extract `Option<&Usage>` from each response variant.
    if let Some(usage) = resp.usage() {
        span.record("gen_ai.usage.input_tokens", usage.prompt_tokens);
        span.record("gen_ai.usage.output_tokens", usage.completion_tokens);
    }
}

/// Build a space-separated string of finish reason names from an iterator of
/// optional [`FinishReason`] values.  `None` entries are skipped.
fn finish_reasons_str(reasons: &[Option<&FinishReason>]) -> String {
    reasons
        .iter()
        .filter_map(|r| r.map(finish_reason_name))
        .fold(String::new(), |mut acc, name| {
            if !acc.is_empty() {
                acc.push(' ');
            }
            acc.push_str(name);
            acc
        })
}

/// Map a [`FinishReason`] variant to its GenAI semantic convention string.
const fn finish_reason_name(reason: &FinishReason) -> &'static str {
    match reason {
        FinishReason::Stop => "stop",
        FinishReason::Length => "length",
        FinishReason::ToolCalls => "tool_calls",
        FinishReason::ContentFilter => "content_filter",
        FinishReason::FunctionCall => "function_call",
        FinishReason::Other => "other",
    }
}
