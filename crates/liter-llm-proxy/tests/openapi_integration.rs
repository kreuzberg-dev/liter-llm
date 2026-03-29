#[allow(dead_code)]
mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

use common::test_proxy::{TestProxy, empty_config};

#[tokio::test]
async fn openapi_returns_200() {
    let proxy = TestProxy::with_config(empty_config());

    let resp = proxy
        .router()
        .oneshot(Request::get("/openapi.json").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn openapi_is_valid_json() {
    let proxy = TestProxy::with_config(empty_config());

    let resp = proxy
        .router()
        .oneshot(Request::get("/openapi.json").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let result: Result<serde_json::Value, _> = serde_json::from_slice(&body);
    assert!(result.is_ok(), "response body should be valid JSON");
}

#[tokio::test]
async fn openapi_has_expected_paths() {
    let proxy = TestProxy::with_config(empty_config());

    let resp = proxy
        .router()
        .oneshot(Request::get("/openapi.json").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    let paths = json["paths"].as_object().expect("paths should be an object");

    let expected_paths = [
        "/v1/chat/completions",
        "/v1/embeddings",
        "/v1/models",
        "/health",
        "/health/liveness",
        "/health/readiness",
    ];

    for path in &expected_paths {
        assert!(
            paths.contains_key(*path),
            "OpenAPI spec should contain path '{path}', found: {:?}",
            paths.keys().collect::<Vec<_>>()
        );
    }
}
