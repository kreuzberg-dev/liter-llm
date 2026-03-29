mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

use liter_llm_proxy::config::VirtualKeyConfig;

const CHAT_COMPLETION_BODY: &str = r#"{"id":"chatcmpl-1","object":"chat.completion","created":1700000000,"model":"gpt-4o","choices":[{"index":0,"message":{"role":"assistant","content":"Hello!"},"finish_reason":"stop"}],"usage":{"prompt_tokens":5,"completion_tokens":3,"total_tokens":8}}"#;

fn chat_route() -> common::mock_upstream::MockRoute {
    common::mock_upstream::MockRoute {
        path: "/chat/completions".into(),
        method: "POST".into(),
        status: 200,
        body: CHAT_COMPLETION_BODY.into(),
        stream_chunks: vec![],
    }
}

fn chat_request_body() -> &'static str {
    r#"{"model":"test-model","messages":[{"role":"user","content":"hi"}]}"#
}

#[tokio::test]
async fn missing_auth_header_returns_401() {
    let upstream = common::mock_upstream::MockUpstream::start(vec![chat_route()]).await;
    let proxy = common::test_proxy::TestProxy::new(&upstream.url);

    let resp = proxy
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/chat/completions")
                .header("content-type", "application/json")
                .body(Body::from(chat_request_body()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    let bytes = Body::new(resp.into_body()).collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert!(body["error"]["message"].is_string());
    assert_eq!(body["error"]["type"], "Authentication");

    upstream.shutdown();
}

#[tokio::test]
async fn invalid_bearer_token_returns_401() {
    let upstream = common::mock_upstream::MockUpstream::start(vec![chat_route()]).await;
    let proxy = common::test_proxy::TestProxy::new(&upstream.url);

    let resp = proxy
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/chat/completions")
                .header("authorization", "Bearer sk-wrong")
                .header("content-type", "application/json")
                .body(Body::from(chat_request_body()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    upstream.shutdown();
}

#[tokio::test]
async fn master_key_passes_auth() {
    let upstream = common::mock_upstream::MockUpstream::start(vec![chat_route()]).await;
    let proxy = common::test_proxy::TestProxy::new(&upstream.url);

    let resp = proxy
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/chat/completions")
                .header("authorization", "Bearer sk-master")
                .header("content-type", "application/json")
                .body(Body::from(chat_request_body()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    upstream.shutdown();
}

#[tokio::test]
async fn virtual_key_passes_auth() {
    let upstream = common::mock_upstream::MockUpstream::start(vec![chat_route()]).await;
    let proxy = common::test_proxy::TestProxy::new(&upstream.url);

    let resp = proxy
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/chat/completions")
                .header("authorization", "Bearer sk-test")
                .header("content-type", "application/json")
                .body(Body::from(chat_request_body()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    upstream.shutdown();
}

#[tokio::test]
async fn virtual_key_denied_model_returns_403() {
    let upstream = common::mock_upstream::MockUpstream::start(vec![chat_route()]).await;

    let mut config = common::test_proxy::default_config(&upstream.url);
    config.keys = vec![VirtualKeyConfig {
        key: "sk-restricted".into(),
        description: None,
        models: vec!["other-model".into()],
        rpm: None,
        tpm: None,
        budget_limit: None,
    }];
    let proxy = common::test_proxy::TestProxy::with_config(config);

    let resp = proxy
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/chat/completions")
                .header("authorization", "Bearer sk-restricted")
                .header("content-type", "application/json")
                .body(Body::from(chat_request_body()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    let bytes = Body::new(resp.into_body()).collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["error"]["type"], "Forbidden");

    upstream.shutdown();
}

#[tokio::test]
async fn health_endpoints_require_no_auth() {
    let upstream = common::mock_upstream::MockUpstream::start(vec![]).await;
    let proxy = common::test_proxy::TestProxy::new(&upstream.url);

    // GET /health without auth
    let resp = proxy
        .router()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // GET /health/liveness without auth
    let resp = proxy
        .router()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/health/liveness")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    upstream.shutdown();
}
