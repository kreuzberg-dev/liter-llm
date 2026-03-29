use axum::Router;
use axum::body::Body;
use axum::http::StatusCode;
use axum::response::Response;
use std::sync::Arc;
use tokio::task::JoinHandle;

/// A single mock route definition for the upstream server.
#[derive(Clone, Debug)]
pub struct MockRoute {
    /// URL path to match, e.g. "/v1/chat/completions".
    pub path: String,
    /// HTTP method to match: "POST", "GET", "DELETE", etc.
    pub method: String,
    /// HTTP status code to return.
    pub status: u16,
    /// JSON response body (used when `stream_chunks` is empty).
    pub body: String,
    /// SSE chunks to return. When non-empty the response is sent as
    /// `text/event-stream` instead of `application/json`.
    pub stream_chunks: Vec<String>,
}

/// An axum-based mock LLM provider server for integration tests.
pub struct MockUpstream {
    /// The base URL of the running server, e.g. `http://127.0.0.1:12345`.
    pub url: String,
    handle: JoinHandle<()>,
}

impl MockUpstream {
    /// Start the mock upstream server on a random port.
    ///
    /// The provided `routes` are matched against incoming requests by method
    /// and path prefix. The first matching route wins.
    pub async fn start(routes: Vec<MockRoute>) -> Self {
        let routes = Arc::new(routes);

        let app = Router::new().fallback(move |req: axum::extract::Request| {
            let routes = Arc::clone(&routes);
            async move {
                let method = req.method().to_string();
                let path = req.uri().path().to_string();

                for route in routes.iter() {
                    if route.method.eq_ignore_ascii_case(&method) && path.starts_with(&route.path) {
                        if !route.stream_chunks.is_empty() {
                            let body = build_sse_body(&route.stream_chunks);
                            return Response::builder()
                                .status(route.status)
                                .header("content-type", "text/event-stream")
                                .body(Body::from(body))
                                .expect("valid SSE response");
                        }

                        return Response::builder()
                            .status(route.status)
                            .header("content-type", "application/json")
                            .body(Body::from(route.body.clone()))
                            .expect("valid JSON response");
                    }
                }

                // No matching route found.
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from(r#"{"error":"mock: no matching route"}"#))
                    .expect("valid 404 response")
            }
        });

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("failed to bind mock upstream listener");
        let addr = listener.local_addr().expect("failed to get local addr");
        let url = format!("http://127.0.0.1:{}", addr.port());

        let handle = tokio::spawn(async move {
            axum::serve(listener, app).await.expect("mock upstream serve failed");
        });

        Self { url, handle }
    }

    /// Abort the background server task.
    pub fn shutdown(self) {
        self.handle.abort();
    }
}

/// Build an SSE body from a list of data chunks, terminated by `[DONE]`.
fn build_sse_body(chunks: &[String]) -> String {
    let mut body = String::new();
    for chunk in chunks {
        body.push_str(&format!("data: {chunk}\n\n"));
    }
    body.push_str("data: [DONE]\n\n");
    body
}
