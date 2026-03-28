//! Cache backend integration tests.
//!
//! Tests for InMemoryStore (in-process), filesystem cache via OpenDAL,
//! and Redis cache via OpenDAL (requires Docker, gated with `#[ignore]`).

#![cfg(feature = "tower")]

use std::time::Duration;

use liter_llm::tower::{CacheConfig, CacheStore, CachedResponse, InMemoryStore};
use liter_llm::types::{AssistantMessage, ChatCompletionResponse, Choice, FinishReason};

// ---- Helpers ---------------------------------------------------------------

fn dummy_response(id: &str) -> CachedResponse {
    CachedResponse::Chat(ChatCompletionResponse {
        id: id.into(),
        object: "chat.completion".into(),
        created: 1_700_000_000,
        model: "gpt-4".into(),
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
        usage: None,
        system_fingerprint: None,
        service_tier: None,
    })
}

// ---- InMemoryStore: LRU eviction under load --------------------------------

#[tokio::test]
async fn in_memory_lru_eviction_under_load() {
    let config = CacheConfig {
        max_entries: 10,
        ttl: Duration::from_secs(300),
        ..Default::default()
    };
    let store = InMemoryStore::new(&config);

    // Fill the cache to max_entries.
    for i in 0..10u64 {
        let body = format!("request-{i}");
        store.put(i, body, dummy_response(&format!("resp-{i}"))).await;
    }

    // Verify all 10 entries are present.
    for i in 0..10u64 {
        let body = format!("request-{i}");
        let result = store.get(i, &body).await;
        assert!(result.is_some(), "entry {i} should still be in cache before eviction");
    }

    // Add one more entry — should evict key=0 (the oldest).
    store.put(10, "request-10".into(), dummy_response("resp-10")).await;

    // Key=0 should be evicted.
    let evicted = store.get(0, "request-0").await;
    assert!(evicted.is_none(), "oldest entry (key=0) should have been evicted");

    // Key=1 through key=10 should still be present.
    for i in 1..=10u64 {
        let body = format!("request-{i}");
        let result = store.get(i, &body).await;
        assert!(
            result.is_some(),
            "entry {i} should still be in cache after eviction of key=0"
        );
    }
}

/// Cache key collision guard: put with key=1 body="A", get with key=1 body="B"
/// should return None because the request bodies do not match.
#[tokio::test]
async fn cache_key_collision_guard() {
    let config = CacheConfig {
        max_entries: 100,
        ttl: Duration::from_secs(300),
        ..Default::default()
    };
    let store = InMemoryStore::new(&config);

    store.put(1, "request-body-A".into(), dummy_response("resp-A")).await;

    // Same key, different body — should be a miss (collision detected).
    let result = store.get(1, "request-body-B").await;
    assert!(
        result.is_none(),
        "get with different request body should return None (collision guard)"
    );

    // Same key, same body — should be a hit.
    let result = store.get(1, "request-body-A").await;
    assert!(
        result.is_some(),
        "get with matching request body should return the cached response"
    );
}

// ---- OpenDAL cache backend tests -------------------------------------------

#[cfg(feature = "opendal-cache")]
mod opendal_tests {
    use super::*;
    use liter_llm::tower::OpenDalCacheStore;
    use std::collections::HashMap;

    /// OpenDAL memory backend: put/get round-trip, collision guard, and remove.
    ///
    /// Uses the in-process `memory` scheme (always available — no external
    /// dependencies) to exercise the `OpenDalCacheStore` code paths that are
    /// shared across all OpenDAL backends.
    #[tokio::test]
    async fn opendal_memory_put_get_remove() {
        let store = OpenDalCacheStore::from_config("memory", HashMap::new(), "cache/", Duration::from_secs(300))
            .expect("memory backend should build");

        // Put
        store
            .put(42, "opendal-request-body".into(), dummy_response("opendal-resp"))
            .await;

        // Get — should hit.
        let result = store.get(42, "opendal-request-body").await;
        assert!(result.is_some(), "OpenDAL memory cache should return stored entry");
        match result.unwrap() {
            CachedResponse::Chat(r) => assert_eq!(r.id, "opendal-resp"),
            _ => panic!("expected CachedResponse::Chat"),
        }

        // Get with wrong body — collision guard.
        let miss = store.get(42, "different-body").await;
        assert!(
            miss.is_none(),
            "OpenDAL memory cache should return None for mismatched request body"
        );

        // Remove
        store.remove(42).await;
        let after_remove = store.get(42, "opendal-request-body").await;
        assert!(after_remove.is_none(), "entry should be gone after remove");
    }

    /// OpenDAL memory backend: TTL expiry. Uses 0-second TTL so entries expire
    /// on the next second boundary.
    #[tokio::test]
    async fn opendal_memory_ttl_expiry() {
        // 0-second TTL: entries expire immediately (on next second boundary).
        let store = OpenDalCacheStore::from_config("memory", HashMap::new(), "cache/", Duration::from_secs(0))
            .expect("memory backend should build");

        store.put(99, "ttl-body".into(), dummy_response("ttl-resp")).await;

        // Wait for the wall clock to advance past the expires_at timestamp.
        tokio::time::sleep(Duration::from_millis(1100)).await;

        let result = store.get(99, "ttl-body").await;
        assert!(result.is_none(), "expired entry should return None");
    }

    /// Redis cache via OpenDAL. Requires a running Redis instance at
    /// localhost:6379 (e.g. via `docker compose up -d redis`).
    ///
    /// Requires Redis on localhost:6379 (see docker-compose.yml).
    #[tokio::test]
    #[ignore = "requires Redis on localhost:6379"]
    async fn redis_cache_put_get_ttl_remove() {
        let mut config = HashMap::new();
        config.insert("connection_string".into(), "redis://localhost:6379".into());

        let store = OpenDalCacheStore::from_config("redis", config, "liter-test/", Duration::from_secs(300))
            .expect("redis backend should build");

        // Put
        store.put(1, "redis-body".into(), dummy_response("redis-resp")).await;

        // Get — should hit.
        let result = store.get(1, "redis-body").await;
        assert!(result.is_some(), "redis cache should return stored entry");
        match result.unwrap() {
            CachedResponse::Chat(r) => assert_eq!(r.id, "redis-resp"),
            _ => panic!("expected CachedResponse::Chat"),
        }

        // Collision guard.
        let miss = store.get(1, "wrong-body").await;
        assert!(miss.is_none(), "redis cache should miss on body mismatch");

        // Remove.
        store.remove(1).await;
        let after_remove = store.get(1, "redis-body").await;
        assert!(after_remove.is_none(), "entry should be gone after remove");
    }
}
