//! Unit tests for the tower middleware integration.
//!
//! These tests use a mock [`LlmClient`] to avoid real HTTP calls.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use tower_service::Service;

use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::Stream;

use crate::client::{BoxFuture, BoxStream, LlmClient};
use crate::error::{LiterLmError, Result};
use crate::tower::fallback::FallbackLayer;
use crate::tower::service::LlmService;
use crate::tower::tracing::TracingLayer;
use crate::tower::types::{LlmRequest, LlmResponse};
use crate::types::{
    AssistantMessage, ChatCompletionRequest, ChatCompletionResponse, Choice, EmbeddingInput, EmbeddingObject,
    EmbeddingRequest, EmbeddingResponse, FinishReason, Message, ModelsListResponse, SystemMessage, Usage,
};
use tower_layer::Layer;

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// A stream that yields no items.
struct EmptyStream;

impl Stream for EmptyStream {
    type Item = Result<crate::types::ChatCompletionChunk>;
    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(None)
    }
}

// ─── Mock client ─────────────────────────────────────────────────────────────

/// A synchronous mock client.  All methods return configurable canned
/// responses or errors.
///
/// The inner state is wrapped in `Arc` so the struct can be cheaply cloned
/// (required by [`FallbackLayer`] which requires `F: Clone`).
#[derive(Clone)]
struct MockClient {
    /// Shared inner state.
    inner: Arc<MockClientInner>,
}

struct MockClientInner {
    /// When set, `chat` returns this error instead of the canned response.
    chat_error: Option<LiterLmErrorKind>,
    /// Number of times `chat` has been called.
    call_count: AtomicUsize,
}

/// A serializable subset of [`LiterLmError`] variants used in tests.
/// `LiterLmError` is not `Clone`, so we store an enum of the variants we care about.
enum LiterLmErrorKind {
    RateLimited { message: String },
    ServiceUnavailable { message: String },
    Timeout,
    Authentication { message: String },
}

impl LiterLmErrorKind {
    fn into_error(&self) -> LiterLmError {
        match self {
            Self::RateLimited { message } => LiterLmError::RateLimited {
                message: message.clone(),
                retry_after: None,
            },
            Self::ServiceUnavailable { message } => LiterLmError::ServiceUnavailable {
                message: message.clone(),
            },
            Self::Timeout => LiterLmError::Timeout,
            // MockClient maps auth error to BadRequest (not Authentication) because
            // the mock's chat() doesn't distinguish — see MockClient::ok().
            Self::Authentication { message } => LiterLmError::BadRequest {
                message: message.clone(),
            },
        }
    }
}

impl MockClient {
    fn ok() -> Self {
        Self {
            inner: Arc::new(MockClientInner {
                chat_error: None,
                call_count: AtomicUsize::new(0),
            }),
        }
    }

    fn failing_rate_limited() -> Self {
        Self {
            inner: Arc::new(MockClientInner {
                chat_error: Some(LiterLmErrorKind::RateLimited {
                    message: "too many requests".into(),
                }),
                call_count: AtomicUsize::new(0),
            }),
        }
    }

    fn failing_service_unavailable() -> Self {
        Self {
            inner: Arc::new(MockClientInner {
                chat_error: Some(LiterLmErrorKind::ServiceUnavailable { message: "503".into() }),
                call_count: AtomicUsize::new(0),
            }),
        }
    }

    fn failing_auth() -> Self {
        Self {
            inner: Arc::new(MockClientInner {
                chat_error: Some(LiterLmErrorKind::Authentication {
                    message: "invalid key".into(),
                }),
                call_count: AtomicUsize::new(0),
            }),
        }
    }

    fn failing_timeout() -> Self {
        Self {
            inner: Arc::new(MockClientInner {
                chat_error: Some(LiterLmErrorKind::Timeout),
                call_count: AtomicUsize::new(0),
            }),
        }
    }
}

fn make_chat_response(model: &str) -> ChatCompletionResponse {
    ChatCompletionResponse {
        id: "test-id".into(),
        object: "chat.completion".into(),
        created: 0,
        model: model.into(),
        choices: vec![Choice {
            index: 0,
            message: AssistantMessage {
                content: Some("Hello!".into()),
                name: None,
                tool_calls: None,
                refusal: None,
                function_call: None,
            },
            finish_reason: Some(FinishReason::Stop),
        }],
        usage: Some(Usage {
            prompt_tokens: 10,
            completion_tokens: 5,
            total_tokens: 15,
        }),
        system_fingerprint: None,
        service_tier: None,
    }
}

impl LlmClient for MockClient {
    fn chat(&self, req: ChatCompletionRequest) -> BoxFuture<'_, ChatCompletionResponse> {
        self.inner.call_count.fetch_add(1, Ordering::SeqCst);
        let result = match &self.inner.chat_error {
            Some(kind) => Err(kind.into_error()),
            None => Ok(make_chat_response(&req.model)),
        };
        Box::pin(async move { result })
    }

    fn chat_stream(
        &self,
        _req: ChatCompletionRequest,
    ) -> BoxFuture<'_, BoxStream<'_, crate::types::ChatCompletionChunk>> {
        Box::pin(async move {
            // Return an immediately-finished stream.
            let stream: BoxStream<'_, crate::types::ChatCompletionChunk> = Box::pin(EmptyStream);
            Ok(stream)
        })
    }

    fn embed(&self, req: EmbeddingRequest) -> BoxFuture<'_, EmbeddingResponse> {
        let resp = EmbeddingResponse {
            object: "list".into(),
            data: vec![EmbeddingObject {
                object: "embedding".into(),
                embedding: vec![0.1, 0.2, 0.3],
                index: 0,
            }],
            model: req.model.clone(),
            usage: Some(Usage {
                prompt_tokens: 4,
                completion_tokens: 0,
                total_tokens: 4,
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

// ─── Helpers ─────────────────────────────────────────────────────────────────

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

// ─── LlmService tests ────────────────────────────────────────────────────────

#[tokio::test]
async fn service_chat_returns_correct_response() {
    let mut svc = LlmService::new(MockClient::ok());
    let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.unwrap();
    match resp {
        LlmResponse::Chat(r) => assert_eq!(r.model, "gpt-4"),
        other => panic!("expected Chat response, got {:?}", std::mem::discriminant(&other)),
    }
}

#[tokio::test]
async fn service_embed_returns_embedding_response() {
    let mut svc = LlmService::new(MockClient::ok());
    let req = EmbeddingRequest {
        model: "text-embedding-3-small".into(),
        input: EmbeddingInput::Single("hello world".into()),
        encoding_format: None,
        dimensions: None,
        user: None,
    };
    let resp = svc.call(LlmRequest::Embed(req)).await.unwrap();
    match resp {
        LlmResponse::Embed(r) => assert_eq!(r.model, "text-embedding-3-small"),
        other => panic!("expected Embed response, got {:?}", std::mem::discriminant(&other)),
    }
}

#[tokio::test]
async fn service_list_models_returns_model_list() {
    let mut svc = LlmService::new(MockClient::ok());
    let resp = svc.call(LlmRequest::ListModels).await.unwrap();
    assert!(matches!(resp, LlmResponse::ListModels(_)));
}

#[tokio::test]
async fn service_propagates_client_error() {
    let mut svc = LlmService::new(MockClient::failing_auth());
    let err = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.unwrap_err();
    assert!(matches!(
        err,
        LiterLmError::BadRequest { .. } | LiterLmError::Authentication { .. }
    ));
}

// ─── TracingLayer tests ───────────────────────────────────────────────────────

#[tokio::test]
async fn tracing_layer_passes_through_success() {
    let inner = LlmService::new(MockClient::ok());
    let mut svc = TracingLayer.layer(inner);
    let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4o"))).await.unwrap();
    assert!(matches!(resp, LlmResponse::Chat(_)));
}

#[tokio::test]
async fn tracing_layer_propagates_error() {
    let inner = LlmService::new(MockClient::failing_timeout());
    let mut svc = TracingLayer.layer(inner);
    let err = svc.call(LlmRequest::Chat(chat_req("gpt-4o"))).await.unwrap_err();
    assert!(matches!(err, LiterLmError::Timeout));
}

// ─── FallbackLayer tests ──────────────────────────────────────────────────────

#[tokio::test]
async fn fallback_not_triggered_on_success() {
    let primary = LlmService::new(MockClient::ok());
    let fallback = LlmService::new(MockClient::ok());

    let mut svc = FallbackLayer::new(fallback).layer(primary);
    let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.unwrap();
    // The response is Chat — confirming primary was called and succeeded.
    assert!(matches!(resp, LlmResponse::Chat(_)));
}

#[tokio::test]
async fn fallback_triggered_on_rate_limit() {
    let primary = LlmService::new(MockClient::failing_rate_limited());
    let fallback = LlmService::new(MockClient::ok());

    let mut svc = FallbackLayer::new(fallback).layer(primary);
    let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.unwrap();
    assert!(matches!(resp, LlmResponse::Chat(_)));
}

#[tokio::test]
async fn fallback_triggered_on_service_unavailable() {
    let primary = LlmService::new(MockClient::failing_service_unavailable());
    let fallback = LlmService::new(MockClient::ok());

    let mut svc = FallbackLayer::new(fallback).layer(primary);
    let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.unwrap();
    assert!(matches!(resp, LlmResponse::Chat(_)));
}

#[tokio::test]
async fn fallback_not_triggered_on_auth_error() {
    let primary = LlmService::new(MockClient::failing_auth());
    let fallback = LlmService::new(MockClient::ok());

    let mut svc = FallbackLayer::new(fallback).layer(primary);
    // Authentication errors are not transient; fallback should NOT be tried.
    // MockClient::failing_auth maps the error to BadRequest (non-transient),
    // so an error should propagate rather than the fallback succeeding.
    let result = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
    assert!(result.is_err(), "expected auth error to propagate, not fall back");
}

// ─── LlmRequest helpers ───────────────────────────────────────────────────────

#[test]
fn request_type_labels() {
    assert_eq!(LlmRequest::Chat(chat_req("m")).request_type(), "chat");
    assert_eq!(LlmRequest::ChatStream(chat_req("m")).request_type(), "chat_stream");
    assert_eq!(
        LlmRequest::Embed(EmbeddingRequest {
            model: "e".into(),
            input: EmbeddingInput::Single("x".into()),
            encoding_format: None,
            dimensions: None,
            user: None,
        })
        .request_type(),
        "embed"
    );
    assert_eq!(LlmRequest::ListModels.request_type(), "list_models");
}

#[test]
fn request_model_returns_none_for_list_models() {
    assert!(LlmRequest::ListModels.model().is_none());
}

// ─── Router tests ─────────────────────────────────────────────────────────────

use crate::tower::router::{Router, RoutingStrategy};

/// Build a `Router` over three mock `LlmService` instances and return
/// a handle to each service's shared call counter so tests can inspect
/// how many times each was called.
fn make_round_robin_router() -> (Router<LlmService<MockClient>>, [Arc<MockClientInner>; 3]) {
    let clients = [MockClient::ok(), MockClient::ok(), MockClient::ok()];
    let counters = [
        Arc::clone(&clients[0].inner),
        Arc::clone(&clients[1].inner),
        Arc::clone(&clients[2].inner),
    ];
    let deployments: Vec<_> = clients.into_iter().map(LlmService::new).collect();
    (Router::new(deployments, RoutingStrategy::RoundRobin), counters)
}

#[tokio::test]
async fn router_round_robin_distributes_across_deployments() {
    let (mut router, counters) = make_round_robin_router();

    // Fire 6 requests — each deployment should receive exactly 2.
    for _ in 0..6 {
        router.call(LlmRequest::Chat(chat_req("gpt-4"))).await.unwrap();
    }

    assert_eq!(
        counters[0].call_count.load(Ordering::SeqCst),
        2,
        "deployment 0 should receive 2 calls"
    );
    assert_eq!(
        counters[1].call_count.load(Ordering::SeqCst),
        2,
        "deployment 1 should receive 2 calls"
    );
    assert_eq!(
        counters[2].call_count.load(Ordering::SeqCst),
        2,
        "deployment 2 should receive 2 calls"
    );
}

#[tokio::test]
async fn router_fallback_tries_next_on_transient_error_then_succeeds() {
    // First two deployments fail with a transient error; the third succeeds.
    let deployments: Vec<LlmService<MockClient>> = vec![
        LlmService::new(MockClient::failing_rate_limited()),
        LlmService::new(MockClient::failing_service_unavailable()),
        LlmService::new(MockClient::ok()),
    ];
    let third_counter = Arc::clone(&deployments[2].inner().inner);

    let mut router = Router::new(deployments, RoutingStrategy::Fallback);
    let resp = router.call(LlmRequest::Chat(chat_req("gpt-4"))).await.unwrap();

    assert!(
        matches!(resp, LlmResponse::Chat(_)),
        "expected successful Chat response from third deployment"
    );
    assert_eq!(
        third_counter.call_count.load(Ordering::SeqCst),
        1,
        "third deployment should have been called exactly once"
    );
}

#[tokio::test]
async fn router_fallback_stops_on_non_transient_error() {
    // First deployment fails with a non-transient (auth) error; the second
    // would succeed but must NOT be reached.
    let ok_client = MockClient::ok();
    let ok_counter = Arc::clone(&ok_client.inner);

    let deployments: Vec<LlmService<MockClient>> =
        vec![LlmService::new(MockClient::failing_auth()), LlmService::new(ok_client)];

    let mut router = Router::new(deployments, RoutingStrategy::Fallback);
    let result = router.call(LlmRequest::Chat(chat_req("gpt-4"))).await;

    assert!(result.is_err(), "expected non-transient error to propagate immediately");
    assert_eq!(
        ok_counter.call_count.load(Ordering::SeqCst),
        0,
        "second deployment must not be called after a non-transient error"
    );
}
