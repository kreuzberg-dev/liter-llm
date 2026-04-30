//! Integration tests for File, Batch, and Response CRUD operations.
//!
//! Each test starts a lightweight mock HTTP server, constructs a `DefaultClient`
//! pointing at it, calls the relevant trait method, and verifies:
//! - The correct HTTP method and URL path were used.
//! - Query parameters are passed correctly.
//! - Request bodies serialize as expected.
//! - Responses deserialize into the correct Rust types.

mod common;

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

use liter_llm::client::config::ClientConfigBuilder;
use liter_llm::client::{BatchClient, DefaultClient, FileClient, ResponseClient};
use liter_llm::types::batch::{BatchListQuery, CreateBatchRequest};
use liter_llm::types::files::FileListQuery;
use liter_llm::types::responses::CreateResponseRequest;

// ── Mock HTTP server ───────────────────────────────────────────────────────

/// A captured HTTP request.
#[derive(Debug, Clone)]
struct CapturedRequest {
    method: String,
    path: String,
    headers: Vec<(String, String)>,
    body: String,
}

/// Configurable mock server that returns different responses based on method+path.
struct MockServer {
    addr: String,
    captured: Arc<Mutex<Vec<CapturedRequest>>>,
    _handle: thread::JoinHandle<()>,
}

/// A route definition for the mock server.
#[derive(Clone)]
struct MockRoute {
    method: String,
    path_prefix: String,
    status: u16,
    body: String,
}

impl MockServer {
    /// Start a mock server with configurable routes.
    /// Falls back to a 200 OK with a generic JSON response for unmatched routes.
    fn start_with_routes(routes: Vec<MockRoute>) -> Self {
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
                let full_path = parts.get(1).unwrap_or(&"").to_string();
                // Separate path from query string.
                let path = full_path.split('?').next().unwrap_or("").to_string();

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
                    method: method.clone(),
                    path: full_path.clone(),
                    headers,
                    body,
                });

                // Find matching route.
                let response_body = routes
                    .iter()
                    .find(|r| r.method == method && path.starts_with(&r.path_prefix))
                    .map(|r| (r.status, r.body.clone()))
                    .unwrap_or((200, r#"{"ok": true}"#.to_string()));

                let (status, resp_body) = response_body;
                let status_text = match status {
                    200 => "OK",
                    201 => "Created",
                    204 => "No Content",
                    _ => "OK",
                };
                let response = format!(
                    "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                    status,
                    status_text,
                    resp_body.len(),
                    resp_body
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

fn build_client(mock: &MockServer) -> DefaultClient {
    let config = ClientConfigBuilder::new("test-api-key")
        .base_url(mock.url())
        .max_retries(0)
        .build();
    DefaultClient::new(config, None).expect("client creation should succeed")
}

// ── File operation JSON responses ──────────────────────────────────────────

fn file_object_json() -> String {
    serde_json::json!({
        "id": "file-abc123",
        "object": "file",
        "bytes": 1024,
        "created_at": 1_700_000_000_u64,
        "filename": "data.jsonl",
        "purpose": "batch"
    })
    .to_string()
}

fn file_list_json() -> String {
    serde_json::json!({
        "object": "list",
        "data": [{
            "id": "file-abc123",
            "object": "file",
            "bytes": 1024,
            "created_at": 1_700_000_000_u64,
            "filename": "data.jsonl",
            "purpose": "batch"
        }],
        "has_more": false
    })
    .to_string()
}

fn delete_response_json() -> String {
    serde_json::json!({
        "id": "file-abc123",
        "object": "file",
        "deleted": true
    })
    .to_string()
}

fn batch_object_json() -> String {
    serde_json::json!({
        "id": "batch-xyz789",
        "object": "batch",
        "endpoint": "/v1/chat/completions",
        "input_file_id": "file-abc123",
        "completion_window": "24h",
        "status": "validating",
        "created_at": 1_700_000_000_u64
    })
    .to_string()
}

fn batch_list_json() -> String {
    serde_json::json!({
        "object": "list",
        "data": [{
            "id": "batch-xyz789",
            "object": "batch",
            "endpoint": "/v1/chat/completions",
            "input_file_id": "file-abc123",
            "completion_window": "24h",
            "status": "completed",
            "created_at": 1_700_000_000_u64
        }],
        "has_more": false
    })
    .to_string()
}

fn response_object_json() -> String {
    serde_json::json!({
        "id": "resp-001",
        "object": "response",
        "created_at": 1_700_000_000_u64,
        "model": "gpt-4",
        "status": "completed",
        "output": [{
            "type": "message",
            "content": "Hello"
        }],
        "usage": {
            "input_tokens": 10,
            "output_tokens": 20,
            "total_tokens": 30
        }
    })
    .to_string()
}

// ── File Client Tests ──────────────────────────────────────────────────────

#[tokio::test]
async fn retrieve_file_should_send_get_to_correct_path() {
    let mock = MockServer::start_with_routes(vec![MockRoute {
        method: "GET".into(),
        path_prefix: "/files/".into(),
        status: 200,
        body: file_object_json(),
    }]);
    let client = build_client(&mock);

    let result = client.retrieve_file("file-abc123").await;
    assert!(result.is_ok(), "retrieve_file should succeed: {result:?}");

    let file = result.unwrap();
    assert_eq!(file.id, "file-abc123");
    assert_eq!(file.filename, "data.jsonl");
    assert_eq!(file.bytes, 1024);

    let requests = mock.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, "GET");
    assert_eq!(requests[0].path, "/files/file-abc123");
}

#[tokio::test]
async fn delete_file_should_send_delete_method() {
    let mock = MockServer::start_with_routes(vec![MockRoute {
        method: "DELETE".into(),
        path_prefix: "/files/".into(),
        status: 200,
        body: delete_response_json(),
    }]);
    let client = build_client(&mock);

    let result = client.delete_file("file-abc123").await;
    assert!(result.is_ok(), "delete_file should succeed: {result:?}");

    let resp = result.unwrap();
    assert_eq!(resp.id, "file-abc123");
    assert!(resp.deleted, "deleted should be true");

    let requests = mock.requests();
    assert_eq!(requests[0].method, "DELETE");
    assert_eq!(requests[0].path, "/files/file-abc123");
}

#[tokio::test]
async fn list_files_should_send_get_without_query_when_none() {
    let mock = MockServer::start_with_routes(vec![MockRoute {
        method: "GET".into(),
        path_prefix: "/files".into(),
        status: 200,
        body: file_list_json(),
    }]);
    let client = build_client(&mock);

    let result = client.list_files(None).await;
    assert!(result.is_ok(), "list_files should succeed: {result:?}");

    let list = result.unwrap();
    assert_eq!(list.data.len(), 1);

    let requests = mock.requests();
    assert_eq!(requests[0].method, "GET");
    assert_eq!(requests[0].path, "/files", "no query string when query is None");
}

#[tokio::test]
async fn list_files_should_include_query_parameters() {
    let mock = MockServer::start_with_routes(vec![MockRoute {
        method: "GET".into(),
        path_prefix: "/files".into(),
        status: 200,
        body: file_list_json(),
    }]);
    let client = build_client(&mock);

    let query = FileListQuery {
        purpose: Some("batch".into()),
        limit: Some(10),
        after: Some("file-cursor".into()),
    };
    let result = client.list_files(Some(query)).await;
    assert!(result.is_ok(), "list_files with query should succeed");

    let requests = mock.requests();
    let path = &requests[0].path;
    assert!(path.contains("purpose=batch"), "should include purpose param: {path}");
    assert!(path.contains("limit=10"), "should include limit param: {path}");
    assert!(path.contains("after=file-cursor"), "should include after param: {path}");
}

#[tokio::test]
async fn file_content_should_return_bytes() {
    let content_bytes = b"file content here";
    let mock = MockServer::start_with_routes(vec![MockRoute {
        method: "GET".into(),
        path_prefix: "/files/".into(),
        status: 200,
        body: String::from_utf8_lossy(content_bytes).to_string(),
    }]);
    let client = build_client(&mock);

    let result = client.file_content("file-abc123").await;
    assert!(result.is_ok(), "file_content should succeed: {result:?}");

    let bytes = result.unwrap();
    assert_eq!(bytes.as_ref(), content_bytes);

    let requests = mock.requests();
    assert_eq!(requests[0].method, "GET");
    assert_eq!(requests[0].path, "/files/file-abc123/content");
}

// ── Batch Client Tests ─────────────────────────────────────────────────────

#[tokio::test]
async fn create_batch_should_send_post_with_json_body() {
    let mock = MockServer::start_with_routes(vec![MockRoute {
        method: "POST".into(),
        path_prefix: "/batches".into(),
        status: 200,
        body: batch_object_json(),
    }]);
    let client = build_client(&mock);

    let req = CreateBatchRequest {
        input_file_id: "file-abc123".into(),
        endpoint: "/v1/chat/completions".into(),
        completion_window: "24h".into(),
        metadata: None,
    };
    let result = client.create_batch(req).await;
    assert!(result.is_ok(), "create_batch should succeed: {result:?}");

    let batch = result.unwrap();
    assert_eq!(batch.id, "batch-xyz789");
    assert_eq!(batch.endpoint, "/v1/chat/completions");
    assert_eq!(batch.input_file_id, "file-abc123");

    let requests = mock.requests();
    assert_eq!(requests[0].method, "POST");
    assert_eq!(requests[0].path, "/batches");

    // Verify the request body contains expected fields.
    let body: serde_json::Value = serde_json::from_str(&requests[0].body).unwrap();
    assert_eq!(body["input_file_id"], "file-abc123");
    assert_eq!(body["endpoint"], "/v1/chat/completions");
    assert_eq!(body["completion_window"], "24h");
}

#[tokio::test]
async fn retrieve_batch_should_send_get_to_correct_path() {
    let mock = MockServer::start_with_routes(vec![MockRoute {
        method: "GET".into(),
        path_prefix: "/batches/".into(),
        status: 200,
        body: batch_object_json(),
    }]);
    let client = build_client(&mock);

    let result = client.retrieve_batch("batch-xyz789").await;
    assert!(result.is_ok(), "retrieve_batch should succeed: {result:?}");

    let batch = result.unwrap();
    assert_eq!(batch.id, "batch-xyz789");

    let requests = mock.requests();
    assert_eq!(requests[0].method, "GET");
    assert_eq!(requests[0].path, "/batches/batch-xyz789");
}

#[tokio::test]
async fn list_batches_should_include_query_parameters() {
    let mock = MockServer::start_with_routes(vec![MockRoute {
        method: "GET".into(),
        path_prefix: "/batches".into(),
        status: 200,
        body: batch_list_json(),
    }]);
    let client = build_client(&mock);

    let query = BatchListQuery {
        limit: Some(5),
        after: Some("batch-cursor".into()),
    };
    let result = client.list_batches(Some(query)).await;
    assert!(result.is_ok(), "list_batches should succeed: {result:?}");

    let list = result.unwrap();
    assert_eq!(list.data.len(), 1);

    let requests = mock.requests();
    let path = &requests[0].path;
    assert!(path.contains("limit=5"), "should include limit param: {path}");
    assert!(
        path.contains("after=batch-cursor"),
        "should include after param: {path}"
    );
}

#[tokio::test]
async fn list_batches_should_send_get_without_query_when_none() {
    let mock = MockServer::start_with_routes(vec![MockRoute {
        method: "GET".into(),
        path_prefix: "/batches".into(),
        status: 200,
        body: batch_list_json(),
    }]);
    let client = build_client(&mock);

    let result = client.list_batches(None).await;
    assert!(result.is_ok());

    let requests = mock.requests();
    assert_eq!(requests[0].method, "GET");
    assert_eq!(requests[0].path, "/batches", "no query string when query is None");
}

#[tokio::test]
async fn cancel_batch_should_post_to_cancel_endpoint() {
    let mock = MockServer::start_with_routes(vec![MockRoute {
        method: "POST".into(),
        path_prefix: "/batches/".into(),
        status: 200,
        body: batch_object_json(),
    }]);
    let client = build_client(&mock);

    let result = client.cancel_batch("batch-xyz789").await;
    assert!(result.is_ok(), "cancel_batch should succeed: {result:?}");

    let requests = mock.requests();
    assert_eq!(requests[0].method, "POST");
    assert_eq!(requests[0].path, "/batches/batch-xyz789/cancel");
}

// ── Response Client Tests ──────────────────────────────────────────────────

#[tokio::test]
async fn create_response_should_send_post_with_json_body() {
    let mock = MockServer::start_with_routes(vec![MockRoute {
        method: "POST".into(),
        path_prefix: "/responses".into(),
        status: 200,
        body: response_object_json(),
    }]);
    let client = build_client(&mock);

    let req = CreateResponseRequest {
        model: "gpt-4".into(),
        input: serde_json::json!("What is the capital of France?"),
        instructions: Some("Be concise.".into()),
        tools: None,
        temperature: None,
        max_output_tokens: None,
        metadata: None,
    };
    let result = client.create_response(req).await;
    assert!(result.is_ok(), "create_response should succeed: {result:?}");

    let resp = result.unwrap();
    assert_eq!(resp.id, "resp-001");
    assert_eq!(resp.model, "gpt-4");
    assert_eq!(resp.status, "completed");
    assert_eq!(resp.output.len(), 1);

    let usage = resp.usage.expect("usage should be present");
    assert_eq!(usage.input_tokens, 10);
    assert_eq!(usage.output_tokens, 20);
    assert_eq!(usage.total_tokens, 30);

    let requests = mock.requests();
    assert_eq!(requests[0].method, "POST");
    assert_eq!(requests[0].path, "/responses");

    let body: serde_json::Value = serde_json::from_str(&requests[0].body).unwrap();
    assert_eq!(body["model"], "gpt-4");
    assert!(body["input"].is_string(), "input should be serialized");
}

#[tokio::test]
async fn retrieve_response_should_send_get_to_correct_path() {
    let mock = MockServer::start_with_routes(vec![MockRoute {
        method: "GET".into(),
        path_prefix: "/responses/".into(),
        status: 200,
        body: response_object_json(),
    }]);
    let client = build_client(&mock);

    let result = client.retrieve_response("resp-001").await;
    assert!(result.is_ok(), "retrieve_response should succeed: {result:?}");

    let resp = result.unwrap();
    assert_eq!(resp.id, "resp-001");

    let requests = mock.requests();
    assert_eq!(requests[0].method, "GET");
    assert_eq!(requests[0].path, "/responses/resp-001");
}

#[tokio::test]
async fn cancel_response_should_post_to_cancel_endpoint() {
    let mock = MockServer::start_with_routes(vec![MockRoute {
        method: "POST".into(),
        path_prefix: "/responses/".into(),
        status: 200,
        body: response_object_json(),
    }]);
    let client = build_client(&mock);

    let result = client.cancel_response("resp-001").await;
    assert!(result.is_ok(), "cancel_response should succeed: {result:?}");

    let requests = mock.requests();
    assert_eq!(requests[0].method, "POST");
    assert_eq!(requests[0].path, "/responses/resp-001/cancel");
}

// ── Cross-cutting concerns ─────────────────────────────────────────────────

#[tokio::test]
async fn all_operations_should_include_auth_header() {
    let mock = MockServer::start_with_routes(vec![
        MockRoute {
            method: "GET".into(),
            path_prefix: "/files".into(),
            status: 200,
            body: file_list_json(),
        },
        MockRoute {
            method: "GET".into(),
            path_prefix: "/batches".into(),
            status: 200,
            body: batch_list_json(),
        },
    ]);
    let client = build_client(&mock);

    // Issue two requests.
    let _ = client.list_files(None).await;
    let _ = client.list_batches(None).await;

    let requests = mock.requests();
    assert_eq!(requests.len(), 2);

    for (i, req) in requests.iter().enumerate() {
        let auth = req
            .headers
            .iter()
            .find(|(n, _)| n == "authorization")
            .map(|(_, v)| v.as_str());
        assert!(auth.is_some(), "request {i} should include Authorization header");
        assert!(
            auth.unwrap().contains("test-api-key"),
            "request {i} should contain the api key"
        );
    }
}

#[tokio::test]
async fn batch_create_with_metadata_should_serialize_metadata() {
    let mock = MockServer::start_with_routes(vec![MockRoute {
        method: "POST".into(),
        path_prefix: "/batches".into(),
        status: 200,
        body: batch_object_json(),
    }]);
    let client = build_client(&mock);

    let req = CreateBatchRequest {
        input_file_id: "file-meta".into(),
        endpoint: "/v1/chat/completions".into(),
        completion_window: "24h".into(),
        metadata: Some(serde_json::json!({"run_id": "test-run-42"})),
    };
    let _ = client.create_batch(req).await;

    let requests = mock.requests();
    let body: serde_json::Value = serde_json::from_str(&requests[0].body).unwrap();
    assert_eq!(body["metadata"]["run_id"], "test-run-42");
}

#[tokio::test]
async fn response_object_deserializes_output_items() {
    let mock = MockServer::start_with_routes(vec![MockRoute {
        method: "GET".into(),
        path_prefix: "/responses/".into(),
        status: 200,
        body: response_object_json(),
    }]);
    let client = build_client(&mock);

    let resp = client.retrieve_response("resp-001").await.expect("should succeed");

    assert_eq!(resp.output.len(), 1);
    assert_eq!(resp.output[0].item_type, "message");
}
