mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

use common::test_proxy::{TestProxy, empty_config};

#[tokio::test]
async fn health_returns_200_with_models() {
    let upstream = common::mock_upstream::MockUpstream::start(vec![]).await;
    let proxy = TestProxy::new(&upstream.url);

    let resp = proxy
        .router()
        .oneshot(Request::get("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "healthy");
    assert!(
        !json["models"].as_array().unwrap().is_empty(),
        "expected at least one model in health response"
    );

    upstream.shutdown();
}

#[tokio::test]
async fn health_returns_degraded_without_models() {
    let proxy = TestProxy::with_config(empty_config());

    let resp = proxy
        .router()
        .oneshot(Request::get("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "degraded");
    assert!(json["models"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn liveness_returns_200() {
    let proxy = TestProxy::with_config(empty_config());

    let resp = proxy
        .router()
        .oneshot(Request::get("/health/liveness").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn readiness_returns_503_without_models() {
    let proxy = TestProxy::with_config(empty_config());

    let resp = proxy
        .router()
        .oneshot(Request::get("/health/readiness").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn readiness_returns_200_with_models() {
    let upstream = common::mock_upstream::MockUpstream::start(vec![]).await;
    let proxy = TestProxy::new(&upstream.url);

    let resp = proxy
        .router()
        .oneshot(Request::get("/health/readiness").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    upstream.shutdown();
}
