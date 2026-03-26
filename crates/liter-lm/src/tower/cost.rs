//! Tower middleware that records estimated cost as a tracing span attribute.
//!
//! [`CostTrackingLayer`] wraps any [`Service<LlmRequest>`] and, after each
//! successful response, calculates the USD cost from the embedded pricing
//! registry and records it as `gen_ai.usage.cost` on the current tracing span.
//!
//! The layer is a no-op (zero overhead) for models not present in the pricing
//! registry — the span attribute is simply not recorded.
//!
//! # Example
//!
//! ```rust,ignore
//! use liter_lm::tower::{CostTrackingLayer, LlmService, TracingLayer};
//! use tower::ServiceBuilder;
//!
//! let client = liter_lm::DefaultClient::new(config, None)?;
//! let service = ServiceBuilder::new()
//!     .layer(TracingLayer)
//!     .layer(CostTrackingLayer)
//!     .service(LlmService::new(client));
//! ```

use std::task::{Context, Poll};

use tower_layer::Layer;
use tower_service::Service;

use super::types::{LlmRequest, LlmResponse};
use crate::client::BoxFuture;
use crate::cost;
use crate::error::{LiterLmError, Result};

// ─── Layer ────────────────────────────────────────────────────────────────────

/// Tower [`Layer`] that records estimated USD cost on the current tracing span.
///
/// After each successful response the layer calls [`cost::completion_cost`] and
/// records the result as `gen_ai.usage.cost` using
/// [`tracing::Span::record`].  If the model is not in the pricing registry the
/// attribute is simply omitted.
pub struct CostTrackingLayer;

impl<S> Layer<S> for CostTrackingLayer {
    type Service = CostTrackingService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CostTrackingService { inner }
    }
}

// ─── Service ──────────────────────────────────────────────────────────────────

/// Tower service produced by [`CostTrackingLayer`].
pub struct CostTrackingService<S> {
    inner: S,
}

impl<S> Clone for CostTrackingService<S>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<S> Service<LlmRequest> for CostTrackingService<S>
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
        // Capture the model name before moving `req` into the inner call, so we
        // can look up pricing after the response arrives.
        let model = req.model().map(ToOwned::to_owned);
        let fut = self.inner.call(req);

        Box::pin(async move {
            let resp = fut.await?;
            record_cost(&model, &resp);
            Ok(resp)
        })
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Extract usage from the response and record an estimated cost on the current
/// tracing span as `gen_ai.usage.cost`.
fn record_cost(model: &Option<String>, resp: &LlmResponse) {
    let Some(model_name) = model else { return };
    let Some(usage) = resp.usage() else { return };

    if let Some(usd) = cost::completion_cost(model_name, usage.prompt_tokens, usage.completion_tokens) {
        tracing::Span::current().record("gen_ai.usage.cost", usd);
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use tower_layer::Layer as _;
    use tower_service::Service as _;

    use crate::tower::service::LlmService;
    use crate::tower::types::{LlmRequest, LlmResponse};
    use crate::types::{
        AssistantMessage, ChatCompletionRequest, ChatCompletionResponse, Choice, EmbeddingObject, EmbeddingRequest,
        EmbeddingResponse, FinishReason, Message, ModelsListResponse, SystemMessage, Usage,
    };
    use crate::{
        client::{BoxFuture, BoxStream, LlmClient},
        error::{LiterLmError, Result},
        types::ChatCompletionChunk,
    };

    use std::pin::Pin;
    use std::task::{Context, Poll};

    use futures_core::Stream;

    use super::CostTrackingLayer;

    // ── Minimal mock ─────────────────────────────────────────────────────────

    struct EmptyStream;

    impl Stream for EmptyStream {
        type Item = Result<ChatCompletionChunk>;
        fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Poll::Ready(None)
        }
    }

    #[derive(Clone)]
    struct PricedMockClient {
        #[allow(dead_code)]
        model: String,
    }

    impl LlmClient for PricedMockClient {
        fn chat(&self, req: ChatCompletionRequest) -> BoxFuture<'_, ChatCompletionResponse> {
            let model = req.model.clone();
            let resp = ChatCompletionResponse {
                id: "test".into(),
                object: "chat.completion".into(),
                created: 0,
                model,
                choices: vec![Choice {
                    index: 0,
                    message: AssistantMessage {
                        content: Some("hi".into()),
                        name: None,
                        tool_calls: None,
                        refusal: None,
                        function_call: None,
                    },
                    finish_reason: Some(FinishReason::Stop),
                }],
                usage: Some(Usage {
                    prompt_tokens: 100,
                    completion_tokens: 50,
                    total_tokens: 150,
                }),
                system_fingerprint: None,
                service_tier: None,
            };
            Box::pin(async move { Ok(resp) })
        }

        fn chat_stream(&self, _req: ChatCompletionRequest) -> BoxFuture<'_, BoxStream<'_, ChatCompletionChunk>> {
            Box::pin(async move {
                let stream: BoxStream<'_, ChatCompletionChunk> = Box::pin(EmptyStream);
                Ok(stream)
            })
        }

        fn embed(&self, req: EmbeddingRequest) -> BoxFuture<'_, EmbeddingResponse> {
            let model = req.model.clone();
            let resp = EmbeddingResponse {
                object: "list".into(),
                data: vec![EmbeddingObject {
                    object: "embedding".into(),
                    embedding: vec![0.1],
                    index: 0,
                }],
                model,
                usage: Some(Usage {
                    prompt_tokens: 10,
                    completion_tokens: 0,
                    total_tokens: 10,
                }),
            };
            Box::pin(async move { Ok(resp) })
        }

        fn list_models(&self) -> BoxFuture<'_, ModelsListResponse> {
            Box::pin(async move {
                Ok(ModelsListResponse {
                    object: "list".into(),
                    data: vec![],
                })
            })
        }
    }

    fn chat_req(model: &str) -> ChatCompletionRequest {
        ChatCompletionRequest {
            model: model.into(),
            messages: vec![Message::System(SystemMessage {
                content: "test".into(),
                name: None,
            })],
            ..Default::default()
        }
    }

    // ── Tests ─────────────────────────────────────────────────────────────────

    /// CostTrackingLayer passes through the response unchanged for a known model.
    #[tokio::test]
    async fn cost_tracking_passes_through_chat_response_for_known_model() {
        let inner = LlmService::new(PricedMockClient { model: "gpt-4".into() });
        let mut svc = CostTrackingLayer.layer(inner);
        let resp = svc
            .call(LlmRequest::Chat(chat_req("gpt-4")))
            .await
            .expect("should succeed");
        // The response must still be a Chat variant with the correct model.
        match resp {
            LlmResponse::Chat(r) => {
                assert_eq!(r.model, "gpt-4");
                // estimated_cost should return Some for gpt-4.
                let cost = r.estimated_cost().expect("gpt-4 must have pricing");
                // 100 * 0.00003 + 50 * 0.00006 = 0.006
                assert!((cost - 0.006).abs() < 1e-9, "unexpected cost: {cost}");
            }
            other => panic!("expected Chat response, got {:?}", std::mem::discriminant(&other)),
        }
    }

    /// CostTrackingLayer is a no-op (does not panic) for unknown models.
    #[tokio::test]
    async fn cost_tracking_no_op_for_unknown_model() {
        let inner = LlmService::new(PricedMockClient {
            model: "unknown-model".into(),
        });
        let mut svc = CostTrackingLayer.layer(inner);
        let resp = svc
            .call(LlmRequest::Chat(chat_req("unknown-model")))
            .await
            .expect("should succeed without error");
        // Response passes through; no panic even though model has no pricing.
        assert!(matches!(resp, LlmResponse::Chat(_)));
    }

    /// CostTrackingLayer propagates errors from the inner service.
    #[tokio::test]
    async fn cost_tracking_propagates_inner_errors() {
        use crate::client::{BoxFuture, BoxStream, LlmClient};
        use crate::tower::service::LlmService;

        #[derive(Clone)]
        struct AlwaysErrorClient;

        impl LlmClient for AlwaysErrorClient {
            fn chat(&self, _req: ChatCompletionRequest) -> BoxFuture<'_, ChatCompletionResponse> {
                Box::pin(async { Err(LiterLmError::Timeout) })
            }
            fn chat_stream(&self, _req: ChatCompletionRequest) -> BoxFuture<'_, BoxStream<'_, ChatCompletionChunk>> {
                Box::pin(async move {
                    let stream: BoxStream<'_, ChatCompletionChunk> = Box::pin(EmptyStream);
                    Ok(stream)
                })
            }
            fn embed(&self, _req: EmbeddingRequest) -> BoxFuture<'_, EmbeddingResponse> {
                Box::pin(async { Err(LiterLmError::Timeout) })
            }
            fn list_models(&self) -> BoxFuture<'_, ModelsListResponse> {
                Box::pin(async { Err(LiterLmError::Timeout) })
            }
        }

        let inner = LlmService::new(AlwaysErrorClient);
        let mut svc = CostTrackingLayer.layer(inner);
        let err = svc
            .call(LlmRequest::Chat(chat_req("gpt-4")))
            .await
            .expect_err("should propagate inner error");
        assert!(matches!(err, LiterLmError::Timeout));
    }
}
