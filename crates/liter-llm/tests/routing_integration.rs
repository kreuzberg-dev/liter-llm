//! Integration tests for provider routing behaviour in `DefaultClient`.
//!
//! These tests verify that:
//! - `base_url` override always wins over per-request routing.
//! - Requests are correctly dispatched to the mock server with proper auth headers.
//! - Custom providers registered at runtime are routed correctly.
//! - Model prefixes are stripped in request bodies where expected.

mod common;

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

use liter_llm::client::config::ClientConfigBuilder;
use liter_llm::client::{DefaultClient, LlmClient};
use liter_llm::types::ChatCompletionRequest;
use liter_llm::{AuthHeaderFormat, CustomProviderConfig, register_custom_provider, unregister_custom_provider};
use serial_test::serial;

// ── Minimal mock HTTP server ───────────────────────────────────────────────

/// A captured HTTP request from the mock server.
#[derive(Debug, Clone)]
struct CapturedRequest {
    method: String,
    path: String,
    headers: Vec<(String, String)>,
    body: String,
}

/// A simple mock HTTP server that captures incoming requests and returns
/// a canned chat completion JSON response.
struct MockServer {
    addr: String,
    captured: Arc<Mutex<Vec<CapturedRequest>>>,
    _handle: thread::JoinHandle<()>,
}

impl MockServer {
    fn start() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind mock server");
        let addr = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
        let captured: Arc<Mutex<Vec<CapturedRequest>>> = Arc::new(Mutex::new(Vec::new()));
        let captured_writer = Arc::clone(&captured);

        let handle = thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(mut stream) = stream else { break };
                let mut reader = BufReader::new(stream.try_clone().unwrap());

                let mut request_line = String::new();
                if reader.read_line(&mut request_line).is_err() {
                    continue;
                }
                let parts: Vec<&str> = request_line.trim().splitn(3, ' ').collect();
                let method = parts.first().unwrap_or(&"").to_string();
                let path = parts.get(1).unwrap_or(&"").to_string();

                let mut headers = Vec::new();
                let mut content_length: usize = 0;
                loop {
                    let mut line = String::new();
                    if reader.read_line(&mut line).is_err() || line.trim().is_empty() {
                        break;
                    }
                    if let Some((name, value)) = line.trim().split_once(':') {
                        let name = name.trim().to_lowercase();
                        let value = value.trim().to_string();
                        if name == "content-length" {
                            content_length = value.parse().unwrap_or(0);
                        }
                        headers.push((name, value));
                    }
                }

                let mut body_buf = vec![0u8; content_length];
                if content_length > 0 {
                    let _ = std::io::Read::read_exact(&mut reader, &mut body_buf);
                }
                let body = String::from_utf8_lossy(&body_buf).to_string();

                captured_writer.lock().unwrap().push(CapturedRequest {
                    method,
                    path,
                    headers,
                    body,
                });

                let response_body = serde_json::json!({
                    "id": "chatcmpl-test",
                    "object": "chat.completion",
                    "created": 1_700_000_000_u64,
                    "model": "test-model",
                    "choices": [{
                        "index": 0,
                        "message": {
                            "role": "assistant",
                            "content": "Hello from mock"
                        },
                        "finish_reason": "stop"
                    }],
                    "usage": {
                        "prompt_tokens": 5,
                        "completion_tokens": 5,
                        "total_tokens": 10
                    }
                });
                let body_str = response_body.to_string();
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    body_str.len(),
                    body_str
                );
                let _ = stream.write_all(response.as_bytes());
                let _ = stream.flush();
            }
        });

        MockServer {
            addr,
            captured,
            _handle: handle,
        }
    }

    fn requests(&self) -> Vec<CapturedRequest> {
        self.captured.lock().unwrap().clone()
    }

    fn url(&self) -> &str {
        &self.addr
    }
}

/// Find a header value by name (case-insensitive) in captured headers.
fn find_header<'a>(headers: &'a [(String, String)], name: &str) -> Option<&'a str> {
    let lower = name.to_lowercase();
    headers
        .iter()
        .find(|(n, _)| n.to_lowercase() == lower)
        .map(|(_, v)| v.as_str())
}

/// Build a minimal chat completion request with the given model.
///
/// Uses serde deserialization because `ChatCompletionRequest` has a
/// `pub(crate)` field (`stream`) that prevents struct literal construction
/// from integration tests.
fn chat_request(model: &str) -> ChatCompletionRequest {
    serde_json::from_value(serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": "Hi"}]
    }))
    .expect("minimal chat request should deserialize")
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[tokio::test]
async fn should_route_to_mock_with_bearer_auth_when_base_url_set() {
    let mock = MockServer::start();
    let config = ClientConfigBuilder::new("test-key-openai")
        .base_url(mock.url())
        .max_retries(0)
        .build();
    let client = DefaultClient::new(config, None).expect("client creation should succeed");

    let req = chat_request("openai/gpt-4");
    let resp = client.chat(req).await;
    assert!(resp.is_ok(), "chat request should succeed: {resp:?}");

    let requests = mock.requests();
    assert_eq!(requests.len(), 1, "should have exactly one captured request");

    let captured = &requests[0];
    assert_eq!(captured.method, "POST");
    assert_eq!(captured.path, "/chat/completions");

    // With base_url override the provider is OpenAI-compatible (Bearer auth).
    let auth = find_header(&captured.headers, "authorization");
    assert!(auth.is_some(), "should have Authorization header");
    assert!(
        auth.unwrap().starts_with("Bearer "),
        "should use Bearer auth, got: {:?}",
        auth
    );
}

#[tokio::test]
async fn should_use_bearer_auth_even_for_anthropic_model_when_base_url_overrides() {
    // When base_url is set, per-request routing to Anthropic is bypassed.
    // The request should still use Bearer auth (from the custom provider),
    // NOT x-api-key (which Anthropic would use).
    let mock = MockServer::start();
    let config = ClientConfigBuilder::new("test-key-override")
        .base_url(mock.url())
        .max_retries(0)
        .build();
    let client = DefaultClient::new(config, None).expect("client creation should succeed");

    let req = chat_request("anthropic/claude-3-sonnet-20240229");
    let resp = client.chat(req).await;
    assert!(resp.is_ok(), "chat request should succeed: {resp:?}");

    let requests = mock.requests();
    assert_eq!(requests.len(), 1);

    let captured = &requests[0];
    assert_eq!(captured.path, "/chat/completions");

    // With base_url override, auth is Bearer, NOT x-api-key.
    let auth = find_header(&captured.headers, "authorization");
    assert!(auth.is_some(), "should use Bearer auth with base_url override");
    assert!(auth.unwrap().starts_with("Bearer "), "should be Bearer, not x-api-key");

    let api_key = find_header(&captured.headers, "x-api-key");
    assert!(
        api_key.is_none(),
        "x-api-key should not be present when base_url overrides routing"
    );
}

#[tokio::test]
async fn should_strip_stream_false_into_body_for_non_streaming() {
    let mock = MockServer::start();
    let config = ClientConfigBuilder::new("test-key")
        .base_url(mock.url())
        .max_retries(0)
        .build();
    let client = DefaultClient::new(config, None).expect("client creation should succeed");

    let req = chat_request("gpt-4");
    let _ = client.chat(req).await;

    let requests = mock.requests();
    let body: serde_json::Value = serde_json::from_str(&requests[0].body).unwrap();

    // The client should set stream=false for non-streaming requests.
    assert_eq!(body["stream"], false, "non-streaming request should have stream=false");
}

#[tokio::test]
async fn should_pass_model_in_body() {
    let mock = MockServer::start();
    let config = ClientConfigBuilder::new("test-key")
        .base_url(mock.url())
        .max_retries(0)
        .build();
    let client = DefaultClient::new(config, None).expect("client creation should succeed");

    let req = chat_request("gpt-4");
    let _ = client.chat(req).await;

    let requests = mock.requests();
    let body: serde_json::Value = serde_json::from_str(&requests[0].body).unwrap();
    assert_eq!(body["model"], "gpt-4", "model should be present in request body");
}

#[tokio::test]
async fn should_construct_client_with_model_hint_for_openai() {
    // Constructing with model_hint="openai/gpt-4" and base_url should work.
    let mock = MockServer::start();
    let config = ClientConfigBuilder::new("test-key")
        .base_url(mock.url())
        .max_retries(0)
        .build();
    let client = DefaultClient::new(config, Some("openai/gpt-4"));
    assert!(client.is_ok(), "client should construct with openai hint");
}

#[tokio::test]
async fn should_construct_client_with_model_hint_for_anthropic() {
    // Constructing with model_hint="anthropic/claude-3" and base_url should work.
    let mock = MockServer::start();
    let config = ClientConfigBuilder::new("test-key")
        .base_url(mock.url())
        .max_retries(0)
        .build();
    let client = DefaultClient::new(config, Some("anthropic/claude-3-sonnet-20240229"));
    assert!(client.is_ok(), "client should construct with anthropic hint");
}

#[tokio::test]
async fn should_default_to_openai_when_no_hint() {
    // Without model_hint, the client defaults to OpenAI.
    // Verify by sending a request and checking Bearer auth is used.
    let mock = MockServer::start();
    let config = ClientConfigBuilder::new("test-key-default")
        .base_url(mock.url())
        .max_retries(0)
        .build();
    let client = DefaultClient::new(config, None).expect("client creation should succeed");

    let req = chat_request("gpt-4");
    let _ = client.chat(req).await;

    let requests = mock.requests();
    let auth = find_header(&requests[0].headers, "authorization");
    assert!(auth.is_some(), "default client should use Bearer auth");
    assert!(
        auth.unwrap().contains("test-key-default"),
        "should use the provided api key"
    );
}

#[tokio::test]
#[serial]
async fn should_route_custom_provider_with_custom_auth_header() {
    let mock = MockServer::start();

    let custom_config = CustomProviderConfig {
        name: "routing-test-provider".into(),
        base_url: mock.url().to_owned(),
        auth_header: AuthHeaderFormat::ApiKey("X-Routing-Test".into()),
        model_prefixes: vec!["routing-test/".into()],
    };
    register_custom_provider(custom_config).expect("registration should succeed");

    // Build client WITHOUT base_url so per-request routing is active.
    // The custom provider's base_url points to our mock.
    let config = ClientConfigBuilder::new("custom-key").max_retries(0).build();
    let client = DefaultClient::new(config, Some("routing-test/my-model")).expect("client creation should succeed");

    let req = chat_request("routing-test/my-model");
    let resp = client.chat(req).await;
    assert!(resp.is_ok(), "chat request should succeed: {resp:?}");

    let requests = mock.requests();
    assert_eq!(requests.len(), 1);

    let captured = &requests[0];
    // Custom provider should use the custom auth header.
    let custom_auth = find_header(&captured.headers, "x-routing-test");
    assert!(custom_auth.is_some(), "should use custom auth header X-Routing-Test");
    assert_eq!(custom_auth.unwrap(), "custom-key");

    // Should NOT have standard Authorization header.
    let bearer = find_header(&captured.headers, "authorization");
    assert!(bearer.is_none(), "custom provider should not use Authorization header");

    // Clean up global state.
    unregister_custom_provider("routing-test-provider").expect("unregister should succeed");
}

#[tokio::test]
#[serial]
async fn should_route_custom_provider_per_request_when_construction_hint_differs() {
    let mock = MockServer::start();

    let custom_config = CustomProviderConfig {
        name: "override-test".into(),
        base_url: mock.url().to_owned(),
        auth_header: AuthHeaderFormat::ApiKey("X-Override".into()),
        model_prefixes: vec!["override-test/".into()],
    };
    register_custom_provider(custom_config).expect("registration should succeed");

    // Build client with model_hint for OpenAI (default).
    // Then send a request with override-test/ prefix — should route to custom provider.
    let config = ClientConfigBuilder::new("override-key").max_retries(0).build();
    let client = DefaultClient::new(config, None).expect("client creation should succeed");

    let req = chat_request("override-test/my-model");
    let resp = client.chat(req).await;
    assert!(resp.is_ok(), "chat request should succeed: {resp:?}");

    let requests = mock.requests();
    assert_eq!(requests.len(), 1);

    // Per-request routing should have detected the custom provider.
    let custom_auth = find_header(&requests[0].headers, "x-override");
    assert!(
        custom_auth.is_some(),
        "per-request routing should use custom provider's auth header"
    );

    unregister_custom_provider("override-test").expect("unregister should succeed");
}

#[tokio::test]
async fn should_reject_empty_model() {
    let mock = MockServer::start();
    let config = ClientConfigBuilder::new("test-key")
        .base_url(mock.url())
        .max_retries(0)
        .build();
    let client = DefaultClient::new(config, None).expect("client creation should succeed");

    let req = chat_request("");
    let resp = client.chat(req).await;
    assert!(resp.is_err(), "should reject empty model string");
}

#[tokio::test]
async fn should_send_to_correct_endpoint_path() {
    let mock = MockServer::start();
    let config = ClientConfigBuilder::new("test-key")
        .base_url(mock.url())
        .max_retries(0)
        .build();
    let client = DefaultClient::new(config, None).expect("client creation should succeed");

    let req = chat_request("gpt-4");
    let _ = client.chat(req).await;

    let requests = mock.requests();
    assert_eq!(requests[0].path, "/chat/completions");
    assert_eq!(requests[0].method, "POST");
}

#[tokio::test]
async fn should_include_messages_in_request_body() {
    let mock = MockServer::start();
    let config = ClientConfigBuilder::new("test-key")
        .base_url(mock.url())
        .max_retries(0)
        .build();
    let client = DefaultClient::new(config, None).expect("client creation should succeed");

    let req = chat_request("gpt-4");
    let _ = client.chat(req).await;

    let requests = mock.requests();
    let body: serde_json::Value = serde_json::from_str(&requests[0].body).unwrap();
    assert!(body["messages"].is_array(), "body should contain messages array");
    assert_eq!(body["messages"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn should_deserialize_mock_response_correctly() {
    let mock = MockServer::start();
    let config = ClientConfigBuilder::new("test-key")
        .base_url(mock.url())
        .max_retries(0)
        .build();
    let client = DefaultClient::new(config, None).expect("client creation should succeed");

    let req = chat_request("gpt-4");
    let resp = client.chat(req).await.expect("should succeed");

    assert_eq!(resp.id, "chatcmpl-test");
    assert_eq!(resp.object, "chat.completion");
    assert_eq!(resp.model, "test-model");
    assert_eq!(resp.choices.len(), 1);

    let usage = resp.usage.expect("usage should be present");
    assert_eq!(usage.prompt_tokens, 5);
    assert_eq!(usage.completion_tokens, 5);
    assert_eq!(usage.total_tokens, 10);
}

#[tokio::test]
async fn all_providers_registry_is_populated() {
    // Verify the provider registry is loaded and contains providers.
    let providers = liter_llm::all_providers().expect("registry should load");
    assert!(!providers.is_empty(), "registry should contain providers");

    // Verify some well-known providers exist.
    let names: Vec<&str> = providers.iter().map(|p| p.name.as_str()).collect();
    assert!(names.contains(&"openai"), "registry should contain openai");
    assert!(names.contains(&"anthropic"), "registry should contain anthropic");
    assert!(names.contains(&"groq"), "registry should contain groq");
}
