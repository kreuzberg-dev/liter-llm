mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn cors_allows_any_origin_by_default() {
    let upstream = common::mock_upstream::MockUpstream::start(vec![]).await;
    let proxy = common::test_proxy::TestProxy::new(&upstream.url);

    // Send a simple GET request with an Origin header to trigger CORS handling.
    let resp = proxy
        .router()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/health")
                .header("origin", "https://example.com")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    // The default config uses cors_origins = ["*"], so the server should echo
    // the wildcard or the specific origin.
    let allow_origin = resp
        .headers()
        .get("access-control-allow-origin")
        .map(|v| v.to_str().unwrap_or(""));
    assert!(allow_origin.is_some(), "expected access-control-allow-origin header");

    upstream.shutdown();
}

#[tokio::test]
async fn cors_preflight_returns_headers() {
    let upstream = common::mock_upstream::MockUpstream::start(vec![]).await;
    let proxy = common::test_proxy::TestProxy::new(&upstream.url);

    // Send an OPTIONS preflight request.
    let resp = proxy
        .router()
        .oneshot(
            Request::builder()
                .method("OPTIONS")
                .uri("/v1/chat/completions")
                .header("origin", "https://example.com")
                .header("access-control-request-method", "POST")
                .header("access-control-request-headers", "authorization,content-type")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Preflight should return 200 (or 204) with CORS headers.
    let status = resp.status().as_u16();
    assert!(
        status == 200 || status == 204,
        "expected 200 or 204 for preflight, got {status}"
    );

    assert!(
        resp.headers().get("access-control-allow-origin").is_some(),
        "missing access-control-allow-origin"
    );
    assert!(
        resp.headers().get("access-control-allow-methods").is_some(),
        "missing access-control-allow-methods"
    );

    upstream.shutdown();
}
