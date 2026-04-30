//! Integration tests for the Tower middleware stack composition.
//!
//! These tests compose MULTIPLE middleware layers together (unlike the unit tests
//! in each middleware module which test one layer at a time) and verify their
//! interactions through a mock client that avoids real HTTP calls.

mod common;

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll};
use std::time::Duration;

use futures_core::Stream;
use tower::{Layer, Service};

use liter_llm::client::{BoxFuture, BoxStream, LlmClient};
use liter_llm::error::{LiterLlmError, Result};
use liter_llm::tower::LlmService;
use liter_llm::tower::budget::{BudgetConfig, BudgetLayer, BudgetState, Enforcement};
use liter_llm::tower::cache::{CacheBackend, CacheConfig, CacheLayer};
use liter_llm::tower::cooldown::CooldownLayer;
use liter_llm::tower::hooks::{HooksLayer, LlmHook};
use liter_llm::tower::rate_limit::{ModelRateLimitLayer, RateLimitConfig};
use liter_llm::tower::tracing::TracingLayer;
use liter_llm::tower::types::{LlmRequest, LlmResponse};
use liter_llm::types::audio::{CreateSpeechRequest, CreateTranscriptionRequest, TranscriptionResponse};
use liter_llm::types::image::{CreateImageRequest, ImagesResponse};
use liter_llm::types::moderation::{ModerationRequest, ModerationResponse};
use liter_llm::types::ocr::{OcrRequest, OcrResponse};
use liter_llm::types::rerank::{RerankRequest, RerankResponse};
use liter_llm::types::search::{SearchRequest, SearchResponse};
use liter_llm::types::{
    AssistantMessage, ChatCompletionChunk, ChatCompletionRequest, ChatCompletionResponse, Choice, EmbeddingObject,
    EmbeddingRequest, EmbeddingResponse, FinishReason, ModelsListResponse, Usage,
};

// ─── Mock Client ─────────────────────────────────────────────────────────────

/// A stream that yields no items.
struct EmptyStream;

impl Stream for EmptyStream {
    type Item = Result<ChatCompletionChunk>;
    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(None)
    }
}

/// A mock LLM client for testing middleware stacks without real HTTP calls.
///
/// Tracks the number of times `chat` has been called via a shared atomic counter.
#[derive(Clone)]
struct MockClient {
    /// Number of times `chat` / `chat_stream` has been called.
    call_count: Arc<AtomicUsize>,
}

impl MockClient {
    fn new() -> Self {
        Self {
            call_count: Arc::new(AtomicUsize::new(0)),
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
            prompt_tokens_details: None,
        }),
        system_fingerprint: None,
        service_tier: None,
    }
}

impl LlmClient for MockClient {
    fn chat(&self, req: ChatCompletionRequest) -> BoxFuture<'_, Result<ChatCompletionResponse>> {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        let resp = make_chat_response(&req.model);
        Box::pin(async move { Ok(resp) })
    }

    fn chat_stream(
        &self,
        _req: ChatCompletionRequest,
    ) -> BoxFuture<'_, Result<BoxStream<'static, Result<ChatCompletionChunk>>>> {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        Box::pin(async move {
            let stream: BoxStream<'static, Result<ChatCompletionChunk>> = Box::pin(EmptyStream);
            Ok(stream)
        })
    }

    fn embed(&self, req: EmbeddingRequest) -> BoxFuture<'_, Result<EmbeddingResponse>> {
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
                prompt_tokens_details: None,
            }),
        };
        Box::pin(async move { Ok(resp) })
    }

    fn list_models(&self) -> BoxFuture<'_, Result<ModelsListResponse>> {
        Box::pin(async move {
            Ok(ModelsListResponse {
                object: "list".into(),
                data: vec![],
            })
        })
    }

    fn image_generate(&self, _req: CreateImageRequest) -> BoxFuture<'_, Result<ImagesResponse>> {
        Box::pin(async move {
            Ok(ImagesResponse {
                created: 0,
                data: vec![],
            })
        })
    }

    fn speech(&self, _req: CreateSpeechRequest) -> BoxFuture<'_, Result<bytes::Bytes>> {
        Box::pin(async move { Ok(bytes::Bytes::new()) })
    }

    fn transcribe(&self, _req: CreateTranscriptionRequest) -> BoxFuture<'_, Result<TranscriptionResponse>> {
        Box::pin(async move {
            Ok(TranscriptionResponse {
                text: String::new(),
                language: None,
                duration: None,
                segments: None,
            })
        })
    }

    fn moderate(&self, _req: ModerationRequest) -> BoxFuture<'_, Result<ModerationResponse>> {
        Box::pin(async move {
            Ok(ModerationResponse {
                id: String::new(),
                model: String::new(),
                results: vec![],
            })
        })
    }

    fn rerank(&self, _req: RerankRequest) -> BoxFuture<'_, Result<RerankResponse>> {
        Box::pin(async move {
            Ok(RerankResponse {
                id: None,
                results: vec![],
                meta: None,
            })
        })
    }

    fn search(&self, _req: SearchRequest) -> BoxFuture<'_, Result<SearchResponse>> {
        Box::pin(async {
            Err(LiterLlmError::EndpointNotSupported {
                endpoint: "search".into(),
                provider: "mock".into(),
            })
        })
    }

    fn ocr(&self, _req: OcrRequest) -> BoxFuture<'_, Result<OcrResponse>> {
        Box::pin(async {
            Err(LiterLlmError::EndpointNotSupported {
                endpoint: "ocr".into(),
                provider: "mock".into(),
            })
        })
    }
}

// ─── Mock Client that always fails with a rate-limit error ──────────────────

/// A mock client that always returns a rate-limited error on `chat`.
#[derive(Clone)]
struct RateLimitedMockClient {
    call_count: Arc<AtomicUsize>,
}

impl RateLimitedMockClient {
    fn new() -> Self {
        Self {
            call_count: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl LlmClient for RateLimitedMockClient {
    fn chat(&self, _req: ChatCompletionRequest) -> BoxFuture<'_, Result<ChatCompletionResponse>> {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        Box::pin(async {
            Err(LiterLlmError::RateLimited {
                message: "too many requests".into(),
                retry_after: None,
            })
        })
    }

    fn chat_stream(
        &self,
        _req: ChatCompletionRequest,
    ) -> BoxFuture<'_, Result<BoxStream<'static, Result<ChatCompletionChunk>>>> {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        Box::pin(async {
            Err(LiterLlmError::RateLimited {
                message: "too many requests".into(),
                retry_after: None,
            })
        })
    }

    fn embed(&self, _req: EmbeddingRequest) -> BoxFuture<'_, Result<EmbeddingResponse>> {
        Box::pin(async {
            Err(LiterLlmError::RateLimited {
                message: "too many requests".into(),
                retry_after: None,
            })
        })
    }

    fn list_models(&self) -> BoxFuture<'_, Result<ModelsListResponse>> {
        Box::pin(async {
            Err(LiterLlmError::RateLimited {
                message: "too many requests".into(),
                retry_after: None,
            })
        })
    }

    fn image_generate(&self, _req: CreateImageRequest) -> BoxFuture<'_, Result<ImagesResponse>> {
        Box::pin(async {
            Err(LiterLlmError::RateLimited {
                message: "too many requests".into(),
                retry_after: None,
            })
        })
    }

    fn speech(&self, _req: CreateSpeechRequest) -> BoxFuture<'_, Result<bytes::Bytes>> {
        Box::pin(async {
            Err(LiterLlmError::RateLimited {
                message: "too many requests".into(),
                retry_after: None,
            })
        })
    }

    fn transcribe(&self, _req: CreateTranscriptionRequest) -> BoxFuture<'_, Result<TranscriptionResponse>> {
        Box::pin(async {
            Err(LiterLlmError::RateLimited {
                message: "too many requests".into(),
                retry_after: None,
            })
        })
    }

    fn moderate(&self, _req: ModerationRequest) -> BoxFuture<'_, Result<ModerationResponse>> {
        Box::pin(async {
            Err(LiterLlmError::RateLimited {
                message: "too many requests".into(),
                retry_after: None,
            })
        })
    }

    fn rerank(&self, _req: RerankRequest) -> BoxFuture<'_, Result<RerankResponse>> {
        Box::pin(async {
            Err(LiterLlmError::RateLimited {
                message: "too many requests".into(),
                retry_after: None,
            })
        })
    }

    fn search(&self, _req: SearchRequest) -> BoxFuture<'_, Result<SearchResponse>> {
        Box::pin(async {
            Err(LiterLlmError::RateLimited {
                message: "too many requests".into(),
                retry_after: None,
            })
        })
    }

    fn ocr(&self, _req: OcrRequest) -> BoxFuture<'_, Result<OcrResponse>> {
        Box::pin(async {
            Err(LiterLlmError::RateLimited {
                message: "too many requests".into(),
                retry_after: None,
            })
        })
    }
}

// ─── Test Hooks ──────────────────────────────────────────────────────────────

/// A hook that counts invocations of each lifecycle callback.
struct CountingHook {
    on_request_count: AtomicUsize,
    on_response_count: AtomicUsize,
    on_error_count: AtomicUsize,
}

impl CountingHook {
    fn new() -> Self {
        Self {
            on_request_count: AtomicUsize::new(0),
            on_response_count: AtomicUsize::new(0),
            on_error_count: AtomicUsize::new(0),
        }
    }
}

impl LlmHook for CountingHook {
    fn on_request(&self, _req: &LlmRequest) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        self.on_request_count.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn on_response(&self, _req: &LlmRequest, _resp: &LlmResponse) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        self.on_response_count.fetch_add(1, Ordering::SeqCst);
        Box::pin(async {})
    }

    fn on_error(&self, _req: &LlmRequest, _err: &LiterLlmError) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        self.on_error_count.fetch_add(1, Ordering::SeqCst);
        Box::pin(async {})
    }
}

/// A hook that rejects requests whose model name contains a given substring.
struct ContentFilterHook {
    blocked_substring: String,
}

impl ContentFilterHook {
    fn new(blocked: &str) -> Self {
        Self {
            blocked_substring: blocked.into(),
        }
    }
}

impl LlmHook for ContentFilterHook {
    fn on_request(&self, req: &LlmRequest) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        let rejected = req.model().is_some_and(|m| m.contains(&self.blocked_substring));
        Box::pin(async move {
            if rejected {
                Err(LiterLlmError::HookRejected {
                    message: "blocked by content filter".into(),
                })
            } else {
                Ok(())
            }
        })
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn chat_req(model: &str) -> ChatCompletionRequest {
    serde_json::from_value(serde_json::json!({
        "model": model,
        "messages": [{"role": "system", "content": "test"}]
    }))
    .expect("valid chat completion request")
}

// ─── Tests: Cache + Budget ───────────────────────────────────────────────────

/// Cache + Budget composed together.
///
/// Verifies that a cache hit does NOT double-count the budget. The budget layer
/// sits outside (above) the cache layer, so:
///   Request -> Budget (pre-check) -> Cache -> [inner service if miss]
///   Response <- Budget (record cost) <- Cache <- [inner response or cached]
///
/// On a cache hit the response still flows through the budget layer's post-flight
/// cost recording. Whether the cost is re-counted depends on whether the cached
/// response contains `usage` data. Since cached responses preserve the original
/// `Usage`, the budget layer WILL record cost again on cache hits. This test
/// documents that behavior.
#[tokio::test]
async fn cache_plus_budget_cache_hit_does_record_cost_again() {
    let budget_state = Arc::new(BudgetState::new());
    let budget_config = BudgetConfig {
        global_limit: Some(1.0),
        enforcement: Enforcement::Hard,
        ..Default::default()
    };

    let cache_config = CacheConfig {
        max_entries: 10,
        ttl: Duration::from_secs(60),
        backend: CacheBackend::Memory,
    };

    let client = MockClient::new();
    let call_count = Arc::clone(&client.call_count);

    // Stack: Budget -> Cache -> LlmService(MockClient)
    // (Budget is outermost; cache is innermost, closest to the service.)
    let base = LlmService::new(client);
    let with_cache = CacheLayer::new(cache_config).layer(base);
    let mut svc = BudgetLayer::new(budget_config, Arc::clone(&budget_state)).layer(with_cache);

    // First request: cache miss, inner service called, cost recorded.
    let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
    assert!(resp.is_ok(), "first request should succeed");
    assert_eq!(call_count.load(Ordering::SeqCst), 1, "inner service called once");

    let spend_after_first = budget_state.global_spend();
    assert!(spend_after_first > 0.0, "cost should be recorded after first call");

    // Second request (same): cache hit, inner service NOT called.
    let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
    assert!(resp.is_ok(), "cached request should succeed");
    assert_eq!(
        call_count.load(Ordering::SeqCst),
        1,
        "inner service should NOT be called again (cache hit)"
    );

    let spend_after_second = budget_state.global_spend();
    // Document the behavior: cached responses still pass through budget post-flight,
    // and because the cached response includes Usage, cost IS recorded again.
    assert!(
        spend_after_second > spend_after_first,
        "budget records cost even on cache hits because cached response includes Usage (spend: {} -> {})",
        spend_after_first,
        spend_after_second,
    );
}

/// Cache + Budget: verify that the inner service is only called once for
/// identical requests even when budget accumulates.
#[tokio::test]
async fn cache_plus_budget_inner_service_called_once() {
    let budget_state = Arc::new(BudgetState::new());
    let budget_config = BudgetConfig {
        global_limit: Some(100.0),
        enforcement: Enforcement::Hard,
        ..Default::default()
    };

    let cache_config = CacheConfig {
        max_entries: 10,
        ttl: Duration::from_secs(60),
        backend: CacheBackend::Memory,
    };

    let client = MockClient::new();
    let call_count = Arc::clone(&client.call_count);

    let base = LlmService::new(client);
    let with_cache = CacheLayer::new(cache_config).layer(base);
    let mut svc = BudgetLayer::new(budget_config, Arc::clone(&budget_state)).layer(with_cache);

    for i in 0..5 {
        let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
        assert!(resp.is_ok(), "request {i} should succeed");
    }

    assert_eq!(
        call_count.load(Ordering::SeqCst),
        1,
        "inner service should be called exactly once; remaining 4 should be cache hits"
    );
}

// ─── Tests: Hooks + Cache ────────────────────────────────────────────────────

/// Hooks + Cache composed together.
///
/// Hooks sit outside cache, so hooks fire on EVERY request — including cache hits.
/// Stack: Hooks -> Cache -> LlmService
#[tokio::test]
async fn hooks_plus_cache_hooks_fire_on_cache_hits() {
    let hook = Arc::new(CountingHook::new());

    let cache_config = CacheConfig {
        max_entries: 10,
        ttl: Duration::from_secs(60),
        backend: CacheBackend::Memory,
    };

    let client = MockClient::new();
    let call_count = Arc::clone(&client.call_count);

    // Stack: Hooks -> Cache -> LlmService
    let base = LlmService::new(client);
    let with_cache = CacheLayer::new(cache_config).layer(base);
    let mut svc = HooksLayer::single(Arc::clone(&hook) as Arc<dyn LlmHook>).layer(with_cache);

    // First request: cache miss.
    svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.unwrap();
    assert_eq!(hook.on_request_count.load(Ordering::SeqCst), 1);
    assert_eq!(hook.on_response_count.load(Ordering::SeqCst), 1);
    assert_eq!(call_count.load(Ordering::SeqCst), 1);

    // Second request: cache hit. Hooks should still fire.
    svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.unwrap();
    assert_eq!(
        hook.on_request_count.load(Ordering::SeqCst),
        2,
        "on_request should fire even on cache hits"
    );
    assert_eq!(
        hook.on_response_count.load(Ordering::SeqCst),
        2,
        "on_response should fire even on cache hits"
    );
    assert_eq!(
        call_count.load(Ordering::SeqCst),
        1,
        "inner service should NOT be called again (cache hit)"
    );
}

// ─── Tests: Budget Hard Enforcement ──────────────────────────────────────────

/// Budget with hard enforcement and a very low limit.
///
/// First request succeeds (cost is recorded post-response). Second request
/// is rejected because the pre-flight check finds the budget exceeded.
#[tokio::test]
async fn budget_hard_enforcement_blocks_second_request() {
    let budget_state = Arc::new(BudgetState::new());
    let budget_config = BudgetConfig {
        global_limit: Some(0.000_001), // extremely low limit
        enforcement: Enforcement::Hard,
        ..Default::default()
    };

    let client = MockClient::new();
    let base = LlmService::new(client);
    let mut svc = BudgetLayer::new(budget_config, Arc::clone(&budget_state)).layer(base);

    // First request succeeds — cost is recorded after the response.
    let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
    assert!(
        resp.is_ok(),
        "first request should succeed (cost recorded post-response)"
    );
    assert!(budget_state.global_spend() > 0.0, "cost should have been recorded");

    // Second request should be rejected because spend >= limit.
    let err = svc
        .call(LlmRequest::Chat(chat_req("gpt-4")))
        .await
        .expect_err("second request should be rejected by budget");
    assert!(
        matches!(err, LiterLlmError::BudgetExceeded { .. }),
        "expected BudgetExceeded, got {err:?}"
    );
}

// ─── Tests: Rate Limit + Cooldown ────────────────────────────────────────────

/// Rate limit + Cooldown composed together.
///
/// Stack: Cooldown -> RateLimit -> LlmService(RateLimitedMockClient)
///
/// The inner mock always returns RateLimited errors. After the first request
/// fails with RateLimited (transient), cooldown engages. During cooldown,
/// subsequent requests get ServiceUnavailable without reaching the inner service.
#[tokio::test]
async fn rate_limit_triggers_cooldown_on_transient_error() {
    // Use a mock that always fails with RateLimited.
    let client = RateLimitedMockClient::new();
    let inner_call_count = Arc::clone(&client.call_count);

    // Stack: Cooldown(60s) -> LlmService(failing mock)
    let base = LlmService::new(client);
    let mut svc = CooldownLayer::new(Duration::from_secs(60)).layer(base);

    // First request: reaches inner service, gets RateLimited (transient), triggers cooldown.
    let err = svc
        .call(LlmRequest::Chat(chat_req("gpt-4")))
        .await
        .expect_err("should fail with RateLimited");
    assert!(
        matches!(err, LiterLlmError::RateLimited { .. }),
        "expected RateLimited, got {err:?}"
    );
    assert_eq!(inner_call_count.load(Ordering::SeqCst), 1);

    // Second request: cooldown is active, should get ServiceUnavailable
    // without reaching the inner service.
    let err = svc
        .call(LlmRequest::Chat(chat_req("gpt-4")))
        .await
        .expect_err("should fail with ServiceUnavailable during cooldown");
    assert!(
        matches!(err, LiterLlmError::ServiceUnavailable { .. }),
        "expected ServiceUnavailable during cooldown, got {err:?}"
    );
    assert_eq!(
        inner_call_count.load(Ordering::SeqCst),
        1,
        "inner service should NOT be called during cooldown"
    );
}

/// Rate limit + Cooldown: cooldown expires and allows requests through again.
#[tokio::test]
async fn cooldown_expires_and_allows_requests() {
    let client = RateLimitedMockClient::new();
    let inner_call_count = Arc::clone(&client.call_count);

    // Zero-duration cooldown so it expires immediately.
    let base = LlmService::new(client);
    let mut svc = CooldownLayer::new(Duration::from_millis(0)).layer(base);

    // First request: triggers cooldown (which expires immediately).
    let err = svc
        .call(LlmRequest::Chat(chat_req("gpt-4")))
        .await
        .expect_err("should fail");
    assert!(matches!(err, LiterLlmError::RateLimited { .. }));
    assert_eq!(inner_call_count.load(Ordering::SeqCst), 1);

    // Second request: cooldown has expired, request reaches inner service again.
    let err = svc
        .call(LlmRequest::Chat(chat_req("gpt-4")))
        .await
        .expect_err("should fail with RateLimited, not ServiceUnavailable");
    assert!(
        matches!(err, LiterLlmError::RateLimited { .. }),
        "after cooldown expires, inner service should be called (got {err:?})"
    );
    assert_eq!(
        inner_call_count.load(Ordering::SeqCst),
        2,
        "inner service should be called again after cooldown expires"
    );
}

/// Per-model rate limit (RPM=1) combined with cooldown.
///
/// Stack: Cooldown -> RateLimit(rpm=1) -> LlmService(MockClient)
///
/// First request succeeds. Second request is rate-limited (by the rate limit
/// layer). Since RateLimited is transient, cooldown engages. Third request
/// during cooldown gets ServiceUnavailable.
#[tokio::test]
async fn rate_limit_rpm_triggers_cooldown() {
    let client = MockClient::new();
    let call_count = Arc::clone(&client.call_count);

    let rate_config = RateLimitConfig {
        rpm: Some(1),
        tpm: None,
        window: Duration::from_secs(60),
    };

    // Stack: Cooldown(60s) -> RateLimit(rpm=1) -> LlmService
    let base = LlmService::new(client);
    let with_rate_limit = ModelRateLimitLayer::new(rate_config).layer(base);
    let mut svc = CooldownLayer::new(Duration::from_secs(60)).layer(with_rate_limit);

    // First request: succeeds (under RPM limit).
    let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
    assert!(resp.is_ok(), "first request should succeed");
    assert_eq!(call_count.load(Ordering::SeqCst), 1);

    // Second request: rate-limited (RPM exceeded). RateLimited is transient,
    // so cooldown engages.
    let err = svc
        .call(LlmRequest::Chat(chat_req("gpt-4")))
        .await
        .expect_err("should be rate limited");
    assert!(
        matches!(err, LiterLlmError::RateLimited { .. }),
        "expected RateLimited, got {err:?}"
    );

    // Third request: cooldown is active -> ServiceUnavailable.
    let err = svc
        .call(LlmRequest::Chat(chat_req("gpt-4")))
        .await
        .expect_err("should be in cooldown");
    assert!(
        matches!(err, LiterLlmError::ServiceUnavailable { .. }),
        "expected ServiceUnavailable during cooldown, got {err:?}"
    );
}

// ─── Tests: Hooks Guardrail Rejection ────────────────────────────────────────

/// Hooks guardrail rejection: a hook that rejects requests prevents the inner
/// service from being called.
#[tokio::test]
async fn hooks_guardrail_rejection_prevents_inner_call() {
    let client = MockClient::new();
    let call_count = Arc::clone(&client.call_count);

    let filter_hook = Arc::new(ContentFilterHook::new("blocked")) as Arc<dyn LlmHook>;

    // Stack: Hooks(filter) -> LlmService
    let base = LlmService::new(client);
    let mut svc = HooksLayer::single(filter_hook).layer(base);

    // Request with "blocked" in model name -> rejected.
    let err = svc
        .call(LlmRequest::Chat(chat_req("blocked-model")))
        .await
        .expect_err("should be rejected by guardrail");
    assert!(
        matches!(err, LiterLlmError::HookRejected { .. }),
        "expected HookRejected, got {err:?}"
    );
    assert_eq!(
        call_count.load(Ordering::SeqCst),
        0,
        "inner service must NOT be called when hook rejects"
    );

    // Request with a clean model name -> succeeds.
    let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
    assert!(resp.is_ok(), "clean request should succeed");
    assert_eq!(call_count.load(Ordering::SeqCst), 1);
}

// ─── Tests: Full Stack (All Middleware) ──────────────────────────────────────

/// Full middleware stack: Tracing -> Hooks -> Budget -> Cache -> LlmService
///
/// This mirrors the composition order in `ManagedClient::build_service_stack`
/// (innermost to outermost): Cache -> Budget -> Hooks -> Tracing.
///
/// Verifies that all layers fire correctly when composed together.
#[tokio::test]
async fn full_stack_all_middleware_composed() {
    let hook = Arc::new(CountingHook::new());
    let budget_state = Arc::new(BudgetState::new());

    let budget_config = BudgetConfig {
        global_limit: Some(100.0),
        enforcement: Enforcement::Hard,
        ..Default::default()
    };

    let cache_config = CacheConfig {
        max_entries: 10,
        ttl: Duration::from_secs(60),
        backend: CacheBackend::Memory,
    };

    let client = MockClient::new();
    let call_count = Arc::clone(&client.call_count);

    // Build from innermost to outermost, matching ManagedClient order:
    // 1. Cache (innermost)
    // 2. Budget
    // 3. Hooks
    // 4. Tracing (outermost)
    let base = LlmService::new(client);
    let with_cache = CacheLayer::new(cache_config).layer(base);
    let with_budget = BudgetLayer::new(budget_config, Arc::clone(&budget_state)).layer(with_cache);
    let with_hooks = HooksLayer::single(Arc::clone(&hook) as Arc<dyn LlmHook>).layer(with_budget);
    let mut svc = TracingLayer.layer(with_hooks);

    // First request: cache miss, all layers fire.
    let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
    assert!(resp.is_ok(), "first request should succeed");
    assert_eq!(call_count.load(Ordering::SeqCst), 1, "inner service called");
    assert_eq!(hook.on_request_count.load(Ordering::SeqCst), 1, "hook.on_request fired");
    assert_eq!(
        hook.on_response_count.load(Ordering::SeqCst),
        1,
        "hook.on_response fired"
    );
    assert!(budget_state.global_spend() > 0.0, "budget recorded cost");

    // Second request (same): cache hit, hooks and budget still fire.
    let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
    assert!(resp.is_ok(), "cached request should succeed");
    assert_eq!(
        call_count.load(Ordering::SeqCst),
        1,
        "inner service NOT called (cache hit)"
    );
    assert_eq!(
        hook.on_request_count.load(Ordering::SeqCst),
        2,
        "hook.on_request fires on cache hit too"
    );
    assert_eq!(
        hook.on_response_count.load(Ordering::SeqCst),
        2,
        "hook.on_response fires on cache hit too"
    );
    assert_eq!(hook.on_error_count.load(Ordering::SeqCst), 0, "no errors occurred");
}

/// Full stack with different models: verify cache isolation works within
/// the full middleware stack.
#[tokio::test]
async fn full_stack_different_models_are_independent() {
    let budget_state = Arc::new(BudgetState::new());

    let budget_config = BudgetConfig {
        global_limit: Some(100.0),
        enforcement: Enforcement::Hard,
        ..Default::default()
    };

    let cache_config = CacheConfig {
        max_entries: 10,
        ttl: Duration::from_secs(60),
        backend: CacheBackend::Memory,
    };

    let client = MockClient::new();
    let call_count = Arc::clone(&client.call_count);

    let base = LlmService::new(client);
    let with_cache = CacheLayer::new(cache_config).layer(base);
    let mut svc = BudgetLayer::new(budget_config, Arc::clone(&budget_state)).layer(with_cache);

    // Two different models: each should be a cache miss.
    svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.unwrap();
    svc.call(LlmRequest::Chat(chat_req("gpt-3.5-turbo"))).await.unwrap();
    assert_eq!(
        call_count.load(Ordering::SeqCst),
        2,
        "different models should be separate cache entries"
    );

    // Repeat both: should be cache hits.
    svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await.unwrap();
    svc.call(LlmRequest::Chat(chat_req("gpt-3.5-turbo"))).await.unwrap();
    assert_eq!(
        call_count.load(Ordering::SeqCst),
        2,
        "repeated requests should hit cache"
    );
}

/// Full stack: hooks guardrail rejection prevents budget accumulation.
///
/// When a hook rejects the request, neither the cache nor the inner service
/// are consulted, and no cost is recorded.
#[tokio::test]
async fn full_stack_hook_rejection_prevents_budget_and_cache() {
    let hook = Arc::new(ContentFilterHook::new("blocked")) as Arc<dyn LlmHook>;
    let budget_state = Arc::new(BudgetState::new());

    let budget_config = BudgetConfig {
        global_limit: Some(100.0),
        enforcement: Enforcement::Hard,
        ..Default::default()
    };

    let cache_config = CacheConfig {
        max_entries: 10,
        ttl: Duration::from_secs(60),
        backend: CacheBackend::Memory,
    };

    let client = MockClient::new();
    let call_count = Arc::clone(&client.call_count);

    // Stack: Hooks -> Budget -> Cache -> LlmService
    let base = LlmService::new(client);
    let with_cache = CacheLayer::new(cache_config).layer(base);
    let with_budget = BudgetLayer::new(budget_config, Arc::clone(&budget_state)).layer(with_cache);
    let mut svc = HooksLayer::single(hook).layer(with_budget);

    // Request with "blocked" in model -> rejected at hooks layer.
    let err = svc
        .call(LlmRequest::Chat(chat_req("blocked-model")))
        .await
        .expect_err("should be rejected");
    assert!(matches!(err, LiterLlmError::HookRejected { .. }));
    assert_eq!(call_count.load(Ordering::SeqCst), 0, "inner service not called");
    assert_eq!(budget_state.global_spend(), 0.0, "no cost recorded when hook rejects");

    // Clean request should still work through the full stack.
    let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
    assert!(resp.is_ok(), "clean request should succeed through full stack");
    assert_eq!(call_count.load(Ordering::SeqCst), 1);
    assert!(budget_state.global_spend() > 0.0, "cost recorded for clean request");
}

/// Full stack with rate limit + cooldown + budget.
///
/// Stack: Budget -> Cooldown -> RateLimit(rpm=1) -> LlmService
///
/// Verifies that rate limiting triggers cooldown, and budget is unaffected
/// by rejected requests.
#[tokio::test]
async fn rate_limit_cooldown_budget_interaction() {
    let budget_state = Arc::new(BudgetState::new());
    let budget_config = BudgetConfig {
        global_limit: Some(100.0),
        enforcement: Enforcement::Hard,
        ..Default::default()
    };

    let rate_config = RateLimitConfig {
        rpm: Some(1),
        tpm: None,
        window: Duration::from_secs(60),
    };

    let client = MockClient::new();
    let call_count = Arc::clone(&client.call_count);

    // Stack: Budget -> Cooldown(60s) -> RateLimit(rpm=1) -> LlmService
    let base = LlmService::new(client);
    let with_rate = ModelRateLimitLayer::new(rate_config).layer(base);
    let with_cooldown = CooldownLayer::new(Duration::from_secs(60)).layer(with_rate);
    let mut svc = BudgetLayer::new(budget_config, Arc::clone(&budget_state)).layer(with_cooldown);

    // First request: succeeds through all layers.
    let resp = svc.call(LlmRequest::Chat(chat_req("gpt-4"))).await;
    assert!(resp.is_ok(), "first request should succeed");
    assert_eq!(call_count.load(Ordering::SeqCst), 1);

    let spend_after_success = budget_state.global_spend();
    assert!(spend_after_success > 0.0, "cost recorded after successful request");

    // Second request: rate-limited, which triggers cooldown.
    let err = svc
        .call(LlmRequest::Chat(chat_req("gpt-4")))
        .await
        .expect_err("should be rate limited");
    assert!(
        matches!(err, LiterLlmError::RateLimited { .. }),
        "expected RateLimited, got {err:?}"
    );

    // Budget should NOT increase from a failed request (no response, no usage).
    assert_eq!(
        budget_state.global_spend(),
        spend_after_success,
        "budget should not change on rate-limited request"
    );

    // Third request: cooldown active -> ServiceUnavailable.
    let err = svc
        .call(LlmRequest::Chat(chat_req("gpt-4")))
        .await
        .expect_err("should be in cooldown");
    assert!(
        matches!(err, LiterLlmError::ServiceUnavailable { .. }),
        "expected ServiceUnavailable, got {err:?}"
    );

    // Budget unchanged from rejection.
    assert_eq!(
        budget_state.global_spend(),
        spend_after_success,
        "budget should not change during cooldown rejection"
    );
}
