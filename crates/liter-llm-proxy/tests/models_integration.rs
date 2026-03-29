mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn list_models_returns_configured_models() {
    let upstream = common::mock_upstream::MockUpstream::start(vec![]).await;
    let proxy = common::test_proxy::TestProxy::new(&upstream.url);

    let resp = proxy
        .router()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/models")
                .header("authorization", "Bearer sk-master")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let bytes = Body::new(resp.into_body()).collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["object"], "list");

    let data = body["data"].as_array().expect("data should be an array");
    let ids: Vec<&str> = data.iter().filter_map(|m| m["id"].as_str()).collect();
    assert!(ids.contains(&"test-model"), "expected test-model in {ids:?}");

    upstream.shutdown();
}

#[tokio::test]
async fn list_models_filtered_by_key_permissions() {
    let upstream = common::mock_upstream::MockUpstream::start(vec![]).await;

    // Config with two models, but the virtual key only has access to one.
    let config = liter_llm_proxy::config::ProxyConfig::from_toml_str(&format!(
        r#"
[general]
master_key = "sk-master"

[[models]]
name = "model-a"
provider_model = "openai/gpt-4o"
api_key = "sk-upstream"
base_url = "{url}"

[[models]]
name = "model-b"
provider_model = "openai/gpt-4o-mini"
api_key = "sk-upstream"
base_url = "{url}"

[[keys]]
key = "sk-limited"
models = ["model-a"]
"#,
        url = upstream.url,
    ))
    .expect("valid TOML");

    let proxy = common::test_proxy::TestProxy::with_config(config);

    let resp = proxy
        .router()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/models")
                .header("authorization", "Bearer sk-limited")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let bytes = Body::new(resp.into_body()).collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let data = body["data"].as_array().expect("data should be an array");
    let ids: Vec<&str> = data.iter().filter_map(|m| m["id"].as_str()).collect();

    assert!(ids.contains(&"model-a"), "should contain model-a");
    assert!(!ids.contains(&"model-b"), "should NOT contain model-b");

    upstream.shutdown();
}

#[tokio::test]
async fn list_models_requires_auth() {
    let upstream = common::mock_upstream::MockUpstream::start(vec![]).await;
    let proxy = common::test_proxy::TestProxy::new(&upstream.url);

    let resp = proxy
        .router()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/models")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    upstream.shutdown();
}
