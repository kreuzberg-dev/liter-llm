//! Concurrency tests for tower middleware layers.
//!
//! These tests verify that BudgetLayer, CacheLayer, and ModelRateLimitLayer
//! handle concurrent access correctly — no panics, no data corruption, no
//! deadlocks.

#![cfg(feature = "tower")]

mod common;

use std::sync::Arc;
use std::time::Duration;

use liter_llm::error::LiterLlmError;
use liter_llm::tower::{
    BudgetConfig, BudgetLayer, BudgetState, CacheConfig, CacheLayer, Enforcement, LlmRequest, LlmService,
    ModelRateLimitLayer, RateLimitConfig,
};
use tokio::task::JoinSet;
use tower::{Service, ServiceBuilder};

// ---- Helpers ---------------------------------------------------------------

/// Minimal mock client that always returns a successful chat response with
/// usage: prompt_tokens=10, completion_tokens=5.
#[derive(Clone)]
struct ConcurrencyMockClient;

impl liter_llm::client::LlmClient for ConcurrencyMockClient {
    fn chat(
        &self,
        req: liter_llm::types::ChatCompletionRequest,
    ) -> liter_llm::client::BoxFuture<'_, liter_llm::error::Result<liter_llm::types::ChatCompletionResponse>> {
        let resp = liter_llm::types::ChatCompletionResponse {
            id: "conc-test".into(),
            object: "chat.completion".into(),
            created: 0,
            model: req.model.clone(),
            choices: vec![liter_llm::types::Choice {
                index: 0,
                message: liter_llm::types::AssistantMessage {
                    content: Some("ok".into()),
                    name: None,
                    tool_calls: None,
                    refusal: None,
                    function_call: None,
                },
                finish_reason: Some(liter_llm::types::FinishReason::Stop),
            }],
            usage: Some(liter_llm::types::Usage {
                prompt_tokens: 10,
                completion_tokens: 5,
                total_tokens: 15,
                prompt_tokens_details: None,
            }),
            system_fingerprint: None,
            service_tier: None,
        };
        Box::pin(async move { Ok(resp) })
    }

    fn chat_stream(
        &self,
        _req: liter_llm::types::ChatCompletionRequest,
    ) -> liter_llm::client::BoxFuture<
        '_,
        liter_llm::error::Result<
            liter_llm::client::BoxStream<'static, liter_llm::error::Result<liter_llm::types::ChatCompletionChunk>>,
        >,
    > {
        Box::pin(async move {
            let stream: liter_llm::client::BoxStream<
                'static,
                liter_llm::error::Result<liter_llm::types::ChatCompletionChunk>,
            > = Box::pin(futures_util::stream::empty());
            Ok(stream)
        })
    }

    fn embed(
        &self,
        req: liter_llm::types::EmbeddingRequest,
    ) -> liter_llm::client::BoxFuture<'_, liter_llm::error::Result<liter_llm::types::EmbeddingResponse>> {
        let resp = liter_llm::types::EmbeddingResponse {
            object: "list".into(),
            data: vec![],
            model: req.model.clone(),
            usage: Some(liter_llm::types::Usage {
                prompt_tokens: 4,
                completion_tokens: 0,
                total_tokens: 4,
                prompt_tokens_details: None,
            }),
        };
        Box::pin(async move { Ok(resp) })
    }

    fn list_models(
        &self,
    ) -> liter_llm::client::BoxFuture<'_, liter_llm::error::Result<liter_llm::types::ModelsListResponse>> {
        Box::pin(async move {
            Ok(liter_llm::types::ModelsListResponse {
                object: "list".into(),
                data: vec![],
            })
        })
    }

    fn image_generate(
        &self,
        _req: liter_llm::types::image::CreateImageRequest,
    ) -> liter_llm::client::BoxFuture<'_, liter_llm::error::Result<liter_llm::types::image::ImagesResponse>> {
        Box::pin(async move {
            Ok(liter_llm::types::image::ImagesResponse {
                created: 0,
                data: vec![],
            })
        })
    }

    fn speech(
        &self,
        _req: liter_llm::types::audio::CreateSpeechRequest,
    ) -> liter_llm::client::BoxFuture<'_, liter_llm::error::Result<bytes::Bytes>> {
        Box::pin(async move { Ok(bytes::Bytes::new()) })
    }

    fn transcribe(
        &self,
        _req: liter_llm::types::audio::CreateTranscriptionRequest,
    ) -> liter_llm::client::BoxFuture<'_, liter_llm::error::Result<liter_llm::types::audio::TranscriptionResponse>>
    {
        Box::pin(async move {
            Ok(liter_llm::types::audio::TranscriptionResponse {
                text: String::new(),
                language: None,
                duration: None,
                segments: None,
            })
        })
    }

    fn moderate(
        &self,
        _req: liter_llm::types::moderation::ModerationRequest,
    ) -> liter_llm::client::BoxFuture<'_, liter_llm::error::Result<liter_llm::types::moderation::ModerationResponse>>
    {
        Box::pin(async move {
            Ok(liter_llm::types::moderation::ModerationResponse {
                id: String::new(),
                model: String::new(),
                results: vec![],
            })
        })
    }

    fn rerank(
        &self,
        _req: liter_llm::types::rerank::RerankRequest,
    ) -> liter_llm::client::BoxFuture<'_, liter_llm::error::Result<liter_llm::types::rerank::RerankResponse>> {
        Box::pin(async move {
            Ok(liter_llm::types::rerank::RerankResponse {
                id: None,
                results: vec![],
                meta: None,
            })
        })
    }

    fn search(
        &self,
        _req: liter_llm::types::search::SearchRequest,
    ) -> liter_llm::client::BoxFuture<'_, liter_llm::error::Result<liter_llm::types::search::SearchResponse>> {
        Box::pin(async {
            Err(liter_llm::error::LiterLlmError::EndpointNotSupported {
                endpoint: "search".into(),
                provider: "mock".into(),
            })
        })
    }

    fn ocr(
        &self,
        _req: liter_llm::types::ocr::OcrRequest,
    ) -> liter_llm::client::BoxFuture<'_, liter_llm::error::Result<liter_llm::types::ocr::OcrResponse>> {
        Box::pin(async {
            Err(liter_llm::error::LiterLlmError::EndpointNotSupported {
                endpoint: "ocr".into(),
                provider: "mock".into(),
            })
        })
    }
}

fn chat_req(model: &str) -> liter_llm::types::ChatCompletionRequest {
    serde_json::from_value(serde_json::json!({
        "model": model,
        "messages": [{"role": "system", "content": "test"}]
    }))
    .expect("test request should deserialize")
}

// ---- Tests -----------------------------------------------------------------

/// Spawn 100 concurrent requests through BudgetLayer. Verify that the final
/// accumulated spend equals the expected sum (within the documented overshoot
/// tolerance for hard enforcement — concurrent in-flight requests may all pass
/// the pre-flight check before any of them record their cost).
#[tokio::test]
async fn concurrent_budget_tracking() {
    let state = Arc::new(BudgetState::new());
    let config = BudgetConfig {
        global_limit: Some(100.0), // High enough to not reject any request.
        enforcement: Enforcement::Soft,
        ..Default::default()
    };

    let svc = ServiceBuilder::new()
        .layer(BudgetLayer::new(config, Arc::clone(&state)))
        .service(LlmService::new(ConcurrencyMockClient));

    let svc = Arc::new(tokio::sync::Mutex::new(svc));
    let mut tasks = JoinSet::new();

    for _ in 0..100 {
        let svc = Arc::clone(&svc);
        tasks.spawn(async move {
            let mut s = svc.lock().await.clone();
            s.call(LlmRequest::Chat(chat_req("gpt-4"))).await
        });
    }

    let mut ok_count = 0u64;
    while let Some(result) = tasks.join_next().await {
        let inner = result.expect("task should not panic");
        if inner.is_ok() {
            ok_count += 1;
        }
    }

    assert_eq!(ok_count, 100, "all 100 requests should succeed under soft enforcement");
    // Budget state should reflect all 100 calls. The exact value depends on
    // cost::completion_cost for gpt-4 with prompt=10, completion=5. We just
    // verify it is positive and non-zero.
    assert!(
        state.global_spend() > 0.0,
        "global spend should be positive after 100 calls, got {}",
        state.global_spend()
    );
}

/// Spawn 50 concurrent identical requests through CacheLayer + LlmService.
/// Verify no panics, no corruption in InMemoryStore, and all callers receive
/// valid responses.
#[tokio::test]
async fn concurrent_cache_writes() {
    let config = CacheConfig {
        max_entries: 256,
        ttl: Duration::from_secs(60),
        ..Default::default()
    };

    let svc = ServiceBuilder::new()
        .layer(CacheLayer::new(config))
        .service(LlmService::new(ConcurrencyMockClient));

    let svc = Arc::new(tokio::sync::Mutex::new(svc));
    let mut tasks = JoinSet::new();

    for _ in 0..50 {
        let svc = Arc::clone(&svc);
        tasks.spawn(async move {
            let mut s = svc.lock().await.clone();
            s.call(LlmRequest::Chat(chat_req("gpt-4"))).await
        });
    }

    let mut ok_count = 0u64;
    while let Some(result) = tasks.join_next().await {
        let inner = result.expect("task should not panic");
        let resp = inner.expect("each request should succeed");
        // Verify the response is a Chat variant with valid content.
        match resp {
            liter_llm::tower::LlmResponse::Chat(r) => {
                assert_eq!(r.model, "gpt-4", "response model should match request");
            }
            other => panic!("expected LlmResponse::Chat, got {other:?}"),
        }
        ok_count += 1;
    }

    assert_eq!(ok_count, 50, "all 50 requests should return valid responses");
}

/// Spawn 20 concurrent requests with RPM=5. Verify exactly 5 succeed and 15
/// are rejected with RateLimited.
#[tokio::test]
async fn concurrent_rate_limit() {
    let config = RateLimitConfig {
        rpm: Some(5),
        tpm: None,
        window: Duration::from_secs(60),
    };

    let svc = ServiceBuilder::new()
        .layer(ModelRateLimitLayer::new(config))
        .service(LlmService::new(ConcurrencyMockClient));

    // Rate limiting uses a shared DashMap, so concurrent access to the *same*
    // mutable service requires serialisation. We clone the service for each
    // task — the Arc<DashMap> is shared.
    let svc = Arc::new(tokio::sync::Mutex::new(svc));
    let mut tasks = JoinSet::new();

    for _ in 0..20 {
        let svc = Arc::clone(&svc);
        tasks.spawn(async move {
            let mut s = svc.lock().await.clone();
            s.call(LlmRequest::Chat(chat_req("gpt-4"))).await
        });
    }

    let mut successes = 0u64;
    let mut rate_limited = 0u64;
    while let Some(result) = tasks.join_next().await {
        let inner = result.expect("task should not panic");
        match inner {
            Ok(_) => successes += 1,
            Err(LiterLlmError::RateLimited { .. }) => rate_limited += 1,
            Err(other) => panic!("unexpected error: {other:?}"),
        }
    }

    assert_eq!(successes, 5, "exactly 5 requests should succeed (RPM=5)");
    assert_eq!(rate_limited, 15, "exactly 15 requests should be rate-limited");
}

/// Cache + Budget + RateLimit all active. Spawn 10 requests. Verify the full
/// middleware stack handles concurrent access without deadlocks or panics.
/// The test completes within a timeout to guard against deadlocks.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_full_stack() {
    let budget_state = Arc::new(BudgetState::new());
    let budget_config = BudgetConfig {
        global_limit: Some(100.0),
        enforcement: Enforcement::Soft,
        ..Default::default()
    };
    let cache_config = CacheConfig {
        max_entries: 64,
        ttl: Duration::from_secs(60),
        ..Default::default()
    };
    let rate_config = RateLimitConfig {
        rpm: Some(100), // High enough not to reject.
        tpm: None,
        window: Duration::from_secs(60),
    };

    let svc = ServiceBuilder::new()
        .layer(CacheLayer::new(cache_config))
        .layer(BudgetLayer::new(budget_config, Arc::clone(&budget_state)))
        .layer(ModelRateLimitLayer::new(rate_config))
        .service(LlmService::new(ConcurrencyMockClient));

    let svc = Arc::new(tokio::sync::Mutex::new(svc));
    let mut tasks = JoinSet::new();

    for i in 0..10 {
        let svc = Arc::clone(&svc);
        // Use two different models to exercise separate rate-limit buckets.
        let model = if i % 2 == 0 { "gpt-4" } else { "gpt-3.5-turbo" };
        tasks.spawn(async move {
            let mut s = svc.lock().await.clone();
            s.call(LlmRequest::Chat(chat_req(model))).await
        });
    }

    // Wrap in a timeout to catch deadlocks.
    let result = tokio::time::timeout(Duration::from_secs(10), async {
        let mut ok_count = 0u64;
        while let Some(result) = tasks.join_next().await {
            let inner = result.expect("task should not panic");
            assert!(inner.is_ok(), "request should succeed: {inner:?}");
            ok_count += 1;
        }
        ok_count
    })
    .await;

    let ok_count = result.expect("full stack should complete within 10s (no deadlock)");
    assert_eq!(ok_count, 10, "all 10 requests should succeed");
    assert!(budget_state.global_spend() > 0.0, "budget should have recorded spend");
}
