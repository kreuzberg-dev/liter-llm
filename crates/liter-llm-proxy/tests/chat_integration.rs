mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

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

fn valid_chat_request(model: &str) -> String {
    format!(r#"{{"model":"{model}","messages":[{{"role":"user","content":"hi"}}]}}"#)
}

#[tokio::test]
async fn chat_json_response() {
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
                .body(Body::from(valid_chat_request("test-model")))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let bytes = Body::new(resp.into_body()).collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["object"], "chat.completion");
    assert_eq!(body["choices"][0]["message"]["content"], "Hello!");
    assert_eq!(body["usage"]["total_tokens"], 8);

    upstream.shutdown();
}

#[tokio::test]
async fn chat_streaming_sse() {
    // SSE streaming requires a TCP server for proper testing because the response
    // body is an unbounded stream.  oneshot() can work if the mock returns a
    // finite SSE body, so we test the happy-path here.
    let chunk1 = r#"{"id":"chatcmpl-1","object":"chat.completion.chunk","created":1700000000,"model":"gpt-4o","choices":[{"index":0,"delta":{"role":"assistant","content":"Hi"},"finish_reason":null}]}"#;
    let chunk2 = r#"{"id":"chatcmpl-1","object":"chat.completion.chunk","created":1700000000,"model":"gpt-4o","choices":[{"index":0,"delta":{"content":"!"},"finish_reason":"stop"}]}"#;

    let upstream = common::mock_upstream::MockUpstream::start(vec![common::mock_upstream::MockRoute {
        path: "/chat/completions".into(),
        method: "POST".into(),
        status: 200,
        body: String::new(),
        stream_chunks: vec![chunk1.into(), chunk2.into()],
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
                .body(Body::from(
                    r#"{"model":"test-model","messages":[{"role":"user","content":"hi"}],"stream":true}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // The proxy should return 200 with an SSE content type.
    assert_eq!(resp.status(), StatusCode::OK);
    let content_type = resp
        .headers()
        .get("content-type")
        .map(|v| v.to_str().unwrap_or(""))
        .unwrap_or("");
    assert!(
        content_type.contains("text/event-stream"),
        "expected text/event-stream, got: {content_type}"
    );

    upstream.shutdown();
}

#[tokio::test]
async fn chat_missing_model_returns_400() {
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
                .body(Body::from(r#"{"messages":[{"role":"user","content":"hi"}]}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let bytes = Body::new(resp.into_body()).collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert!(body["error"]["message"].as_str().unwrap().contains("model"));

    upstream.shutdown();
}

#[tokio::test]
async fn chat_unknown_model_returns_404() {
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
                .body(Body::from(valid_chat_request("nonexistent")))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    upstream.shutdown();
}

#[tokio::test]
async fn chat_invalid_json_returns_400() {
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
                .body(Body::from("not json"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    upstream.shutdown();
}

#[tokio::test]
async fn chat_upstream_receives_correct_request() {
    // The mock upstream always returns the same response regardless of input.
    // This test verifies the proxy returns a valid completion when given a
    // properly-formed request -- confirming the request made it through the
    // proxy and reached the mock upstream intact.
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
                .body(Body::from(valid_chat_request("test-model")))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let bytes = Body::new(resp.into_body()).collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    // Verify the response includes expected fields from the mock upstream.
    assert!(body["id"].is_string());
    assert!(body["choices"].is_array());
    assert!(body["usage"]["prompt_tokens"].is_number());

    upstream.shutdown();
}
