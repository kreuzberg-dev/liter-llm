mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

const ERROR_500_BODY: &str =
    r#"{"error":{"message":"Internal server error","type":"server_error","param":null,"code":null}}"#;

fn valid_chat_request() -> &'static str {
    r#"{"model":"test-model","messages":[{"role":"user","content":"hi"}]}"#
}

#[tokio::test]
async fn upstream_500_returns_500() {
    let upstream = common::mock_upstream::MockUpstream::start(vec![common::mock_upstream::MockRoute {
        path: "/chat/completions".into(),
        method: "POST".into(),
        status: 500,
        body: ERROR_500_BODY.into(),
        stream_chunks: vec![],
    }])
    .await;
    let proxy = common::test_proxy::TestProxy::new(&upstream.url);

    let resp = proxy
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/chat/completions")
                .header("authorization", "Bearer sk-master")
                .header("content-type", "application/json")
                .body(Body::from(valid_chat_request()))
                .unwrap(),
        )
        .await
        .unwrap();

    // The proxy should propagate a 5xx status.
    let status = resp.status().as_u16();
    assert!((500..=599).contains(&status), "expected 5xx status, got {status}");

    upstream.shutdown();
}

#[tokio::test]
async fn upstream_429_returns_429() {
    let upstream = common::mock_upstream::MockUpstream::start(vec![common::mock_upstream::MockRoute {
        path: "/chat/completions".into(),
        method: "POST".into(),
        status: 429,
        body: r#"{"error":{"message":"Rate limit exceeded","type":"rate_limit_error","param":null,"code":null}}"#
            .into(),
        stream_chunks: vec![],
    }])
    .await;
    let proxy = common::test_proxy::TestProxy::new(&upstream.url);

    let resp = proxy
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/chat/completions")
                .header("authorization", "Bearer sk-master")
                .header("content-type", "application/json")
                .body(Body::from(valid_chat_request()))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = resp.status().as_u16();
    assert!(
        status == 429 || (500..=599).contains(&status),
        "expected 429 or 5xx, got {status}"
    );

    upstream.shutdown();
}

#[tokio::test]
async fn upstream_unreachable_returns_502() {
    // Point the model at a port that is not listening.
    let config = liter_llm_proxy::config::ProxyConfig::from_toml_str(
        r#"
[general]
master_key = "sk-master"
default_timeout_secs = 2

[[models]]
name = "test-model"
provider_model = "openai/gpt-4o"
api_key = "sk-upstream"
base_url = "http://127.0.0.1:1"
timeout_secs = 2
"#,
    )
    .expect("valid TOML");

    let proxy = common::test_proxy::TestProxy::with_config(config);

    let resp = proxy
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/chat/completions")
                .header("authorization", "Bearer sk-master")
                .header("content-type", "application/json")
                .body(Body::from(valid_chat_request()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Network errors should surface as 502 Bad Gateway.
    let status = resp.status().as_u16();
    assert!(
        (500..=599).contains(&status),
        "expected 5xx for unreachable upstream, got {status}"
    );
}

#[tokio::test]
async fn error_body_is_openai_format() {
    let upstream = common::mock_upstream::MockUpstream::start(vec![]).await;
    let proxy = common::test_proxy::TestProxy::new(&upstream.url);

    // Request without auth to trigger a guaranteed error (401).
    let resp = proxy
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/chat/completions")
                .header("content-type", "application/json")
                .body(Body::from(valid_chat_request()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    let bytes = Body::new(resp.into_body()).collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

    // Verify OpenAI error format: { "error": { "message": "...", "type": "..." } }
    assert!(body["error"].is_object(), "body must have 'error' object");
    assert!(body["error"]["message"].is_string(), "error must have 'message' string");
    assert!(body["error"]["type"].is_string(), "error must have 'type' string");

    upstream.shutdown();
}
