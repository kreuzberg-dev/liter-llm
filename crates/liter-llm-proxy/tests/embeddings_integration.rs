mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

use liter_llm_proxy::config::VirtualKeyConfig;

const EMBEDDING_RESPONSE: &str = r#"{"object":"list","data":[{"object":"embedding","embedding":[0.1,0.2,0.3],"index":0}],"model":"text-embedding-3-small","usage":{"prompt_tokens":5,"total_tokens":5}}"#;

fn embedding_route() -> common::mock_upstream::MockRoute {
    common::mock_upstream::MockRoute {
        path: "/embeddings".into(),
        method: "POST".into(),
        status: 200,
        body: EMBEDDING_RESPONSE.into(),
        stream_chunks: vec![],
    }
}

fn valid_embedding_body(model: &str) -> String {
    format!(r#"{{"model":"{model}","input":"hello world"}}"#)
}

#[tokio::test]
async fn embedding_returns_200() {
    let upstream = common::mock_upstream::MockUpstream::start(vec![embedding_route()]).await;
    let proxy = common::test_proxy::TestProxy::new(&upstream.url);

    let resp = proxy
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/embeddings")
                .header("authorization", "Bearer sk-master")
                .header("content-type", "application/json")
                .body(Body::from(valid_embedding_body("test-model")))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let bytes = Body::new(resp.into_body()).collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["object"], "list");
    assert!(body["data"][0]["embedding"].is_array());

    upstream.shutdown();
}

#[tokio::test]
async fn embedding_unknown_model_returns_404() {
    let upstream = common::mock_upstream::MockUpstream::start(vec![embedding_route()]).await;
    let proxy = common::test_proxy::TestProxy::new(&upstream.url);

    let resp = proxy
        .router()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/embeddings")
                .header("authorization", "Bearer sk-master")
                .header("content-type", "application/json")
                .body(Body::from(valid_embedding_body("nonexistent")))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    upstream.shutdown();
}

#[tokio::test]
async fn embedding_denied_model_returns_403() {
    let upstream = common::mock_upstream::MockUpstream::start(vec![embedding_route()]).await;

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
                .uri("/v1/embeddings")
                .header("authorization", "Bearer sk-restricted")
                .header("content-type", "application/json")
                .body(Body::from(valid_embedding_body("test-model")))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    upstream.shutdown();
}
