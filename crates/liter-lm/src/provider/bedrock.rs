use std::borrow::Cow;

#[cfg(feature = "bedrock")]
use crate::error::LiterLmError;
use crate::error::Result;
use crate::provider::Provider;

/// Default AWS region for Bedrock when none is specified.
const DEFAULT_REGION: &str = "us-east-1";

/// Percent-encode a model ID for use in a URL path segment.
///
/// Bedrock model IDs can contain colons and slashes that must be encoded.
fn percent_encode_model(model: &str) -> String {
    let mut encoded = String::with_capacity(model.len());
    for byte in model.bytes() {
        match byte {
            // Unreserved characters per RFC 3986 §2.3 — safe to pass through.
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            other => {
                encoded.push('%');
                encoded.push(char::from_digit(u32::from(other >> 4), 16).unwrap_or('0'));
                encoded.push(char::from_digit(u32::from(other & 0xf), 16).unwrap_or('0'));
            }
        }
    }
    encoded
}

/// AWS Bedrock provider.
///
/// Differences from the OpenAI-compatible baseline:
/// - Routes `bedrock/` prefixed model names to the Bedrock runtime endpoint.
/// - The model prefix is stripped before the model ID is sent in the request.
/// - When the `bedrock` feature is enabled, every request is signed with
///   AWS Signature Version 4 using credentials from the environment
///   (`AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, `AWS_SESSION_TOKEN`).
/// - When the `bedrock` feature is disabled, the provider is usable with a
///   `base_url` override (e.g. in tests against a mock server) without any
///   signing.
///
/// # Region resolution
///
/// The region is resolved in priority order:
/// 1. Explicit value passed to [`BedrockProvider::with_region`].
/// 2. `AWS_DEFAULT_REGION` environment variable.
/// 3. `AWS_REGION` environment variable.
/// 4. Hard-coded default: `us-east-1`.
///
/// # Configuration
///
/// ```rust,ignore
/// let config = ClientConfigBuilder::new("unused-for-sigv4")
///     .build();
/// let client = DefaultClient::new(config, Some("bedrock/anthropic.claude-3-sonnet-20240229-v1:0"))?;
/// ```
pub struct BedrockProvider {
    #[allow(dead_code)] // used by region() accessor and in sigv4_sign
    region: String,
    /// Cached base URL: `https://bedrock-runtime.{region}.amazonaws.com`.
    base_url: String,
}

impl BedrockProvider {
    /// Construct with the given AWS region.
    #[must_use]
    pub fn new(region: impl Into<String>) -> Self {
        let region = region.into();
        let base_url = format!("https://bedrock-runtime.{region}.amazonaws.com");
        Self { region, base_url }
    }

    /// Construct using region from the environment, falling back to `us-east-1`.
    ///
    /// Reads `AWS_DEFAULT_REGION` then `AWS_REGION`.
    #[must_use]
    pub fn from_env() -> Self {
        let region = std::env::var("AWS_DEFAULT_REGION")
            .or_else(|_| std::env::var("AWS_REGION"))
            .unwrap_or_else(|_| DEFAULT_REGION.to_owned());
        Self::new(region)
    }

    /// Return the AWS region this provider is configured for.
    #[must_use]
    #[allow(dead_code)] // useful for consumers of the library
    pub fn region(&self) -> &str {
        &self.region
    }
}

impl Provider for BedrockProvider {
    fn name(&self) -> &str {
        "bedrock"
    }

    /// Base URL for the Bedrock runtime service.
    ///
    /// When a `base_url` override is set in [`ClientConfig`] (as in tests),
    /// the override takes precedence and this value is never used.
    fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Bedrock uses SigV4 signing rather than a static authorization header.
    ///
    /// Returns `None` so the HTTP layer skips adding an `Authorization` header.
    /// Actual signing headers are injected by [`BedrockProvider::signing_headers`]
    /// when the `bedrock` feature is enabled.
    fn auth_header<'a>(&'a self, _api_key: &'a str) -> Option<(Cow<'static, str>, Cow<'a, str>)> {
        None
    }

    fn matches_model(&self, model: &str) -> bool {
        model.starts_with("bedrock/")
    }

    fn strip_model_prefix<'m>(&self, model: &'m str) -> &'m str {
        model.strip_prefix("bedrock/").unwrap_or(model)
    }

    /// Validate that the provider is usable in the current environment.
    ///
    /// When the `bedrock` feature is enabled, checks that AWS credentials are
    /// available in the environment (`AWS_ACCESS_KEY_ID` at minimum).  Without
    /// credentials, every real Bedrock request will be rejected with a 403.
    ///
    /// When the `bedrock` feature is disabled (e.g. in tests with `base_url`
    /// override), validation is skipped so callers can connect to a mock server
    /// without real AWS credentials.
    fn validate(&self) -> Result<()> {
        #[cfg(feature = "bedrock")]
        {
            if std::env::var("AWS_ACCESS_KEY_ID").is_err() {
                return Err(LiterLmError::BadRequest {
                    message: "AWS Bedrock requires AWS credentials. \
                              Set AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY (and optionally \
                              AWS_SESSION_TOKEN) in the environment."
                        .into(),
                });
            }
        }
        Ok(())
    }

    /// Build the full URL for a Bedrock Converse API request.
    ///
    /// Chat completions map to `/model/{encoded_model}/converse`.
    /// Embeddings map to `/model/{encoded_model}/invoke`.
    /// All other paths are passed through unchanged.
    fn build_url(&self, endpoint_path: &str, model: &str) -> String {
        let base = self.base_url();
        let encoded_model = percent_encode_model(model);
        if endpoint_path.contains("chat/completions") {
            // TODO: streaming will need `/model/{model}/converse-stream`
            // (binary EventStream, not SSE) — deferred to a future release.
            format!("{base}/model/{encoded_model}/converse")
        } else if endpoint_path.contains("embeddings") {
            format!("{base}/model/{encoded_model}/invoke")
        } else {
            format!("{base}{endpoint_path}")
        }
    }

    /// Convert an OpenAI-style chat request to Bedrock Converse API format.
    ///
    /// Key differences from the OpenAI format:
    /// - System messages are extracted to a top-level `system` array.
    /// - Messages use `content` arrays with typed blocks (`text`, `toolUse`, `toolResult`).
    /// - Generation parameters live in `inferenceConfig`.
    /// - Tools are described in `toolConfig.tools[].toolSpec`.
    fn transform_request(&self, body: &mut serde_json::Value) -> Result<()> {
        use serde_json::json;

        let messages = body
            .get("messages")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let mut system_parts = vec![];
        let mut converse_messages = vec![];

        for msg in &messages {
            let role = msg.get("role").and_then(|r| r.as_str()).unwrap_or("");
            let content = msg.get("content");

            match role {
                "system" | "developer" => {
                    if let Some(text) = content.and_then(|c| c.as_str()) {
                        system_parts.push(json!({"text": text}));
                    }
                }
                "user" => {
                    let parts = if let Some(text) = content.and_then(|c| c.as_str()) {
                        vec![json!({"text": text})]
                    } else {
                        // Handle content parts array or fall back to string representation.
                        let text_fallback = content.map(|c| c.to_string()).unwrap_or_default();
                        vec![json!({"text": text_fallback})]
                    };
                    converse_messages.push(json!({"role": "user", "content": parts}));
                }
                "assistant" => {
                    let mut parts = vec![];
                    if let Some(text) = content.and_then(|c| c.as_str())
                        && !text.is_empty()
                    {
                        parts.push(json!({"text": text}));
                    }
                    // Convert OpenAI tool_calls to Bedrock toolUse blocks.
                    if let Some(tool_calls) = msg.get("tool_calls").and_then(|t| t.as_array()) {
                        for tc in tool_calls {
                            let input: serde_json::Value = tc
                                .pointer("/function/arguments")
                                .and_then(|a| a.as_str())
                                .and_then(|s| serde_json::from_str(s).ok())
                                .unwrap_or_else(|| json!({}));
                            parts.push(json!({
                                "toolUse": {
                                    "toolUseId": tc.get("id"),
                                    "name": tc.pointer("/function/name"),
                                    "input": input
                                }
                            }));
                        }
                    }
                    if parts.is_empty() {
                        parts.push(json!({"text": ""}));
                    }
                    converse_messages.push(json!({"role": "assistant", "content": parts}));
                }
                "tool" => {
                    let tool_call_id = msg.get("tool_call_id").and_then(|t| t.as_str()).unwrap_or("");
                    let result_text = content.and_then(|c| c.as_str()).unwrap_or("");
                    converse_messages.push(json!({
                        "role": "user",
                        "content": [{
                            "toolResult": {
                                "toolUseId": tool_call_id,
                                "content": [{"text": result_text}],
                                "status": "success"
                            }
                        }]
                    }));
                }
                _ => {}
            }
        }

        // Build inferenceConfig from OpenAI generation parameters.
        let mut inference_config = json!({});
        if let Some(max_tokens) = body.get("max_tokens").or_else(|| body.get("max_completion_tokens")) {
            inference_config["maxTokens"] = max_tokens.clone();
        }
        if let Some(temp) = body.get("temperature") {
            inference_config["temperature"] = temp.clone();
        }
        if let Some(top_p) = body.get("top_p") {
            inference_config["topP"] = top_p.clone();
        }
        if let Some(stop) = body.get("stop") {
            let sequences = if let Some(s) = stop.as_str() {
                vec![json!(s)]
            } else {
                stop.as_array().cloned().unwrap_or_default()
            };
            inference_config["stopSequences"] = json!(sequences);
        }

        // Build toolConfig if tools are present.
        let tool_config = body.get("tools").and_then(|tools| {
            tools.as_array().map(|arr| {
                let bedrock_tools: Vec<serde_json::Value> = arr
                    .iter()
                    .map(|t| {
                        let parameters = t
                            .pointer("/function/parameters")
                            .cloned()
                            .unwrap_or_else(|| json!({"type": "object"}));
                        json!({
                            "toolSpec": {
                                "name": t.pointer("/function/name"),
                                "description": t.pointer("/function/description"),
                                "inputSchema": {"json": parameters}
                            }
                        })
                    })
                    .collect();
                json!({"tools": bedrock_tools})
            })
        });

        // Assemble the Bedrock Converse request body.
        let mut new_body = json!({
            "messages": converse_messages,
        });
        if !system_parts.is_empty() {
            new_body["system"] = json!(system_parts);
        }
        if let Some(obj) = inference_config.as_object()
            && !obj.is_empty()
        {
            new_body["inferenceConfig"] = inference_config;
        }
        if let Some(tc) = tool_config {
            new_body["toolConfig"] = tc;
        }

        *body = new_body;
        Ok(())
    }

    /// Normalize a Bedrock Converse API response to OpenAI chat completion format.
    ///
    /// Bedrock wraps the assistant's message in `output.message.content[]` blocks.
    /// Stop reasons use Bedrock terminology (`end_turn`, `tool_use`, etc.) and are
    /// mapped to the OpenAI `finish_reason` set.
    fn transform_response(&self, body: &mut serde_json::Value) -> Result<()> {
        use serde_json::json;

        let stop_reason = body.get("stopReason").and_then(|s| s.as_str()).unwrap_or("end_turn");
        let usage = body.get("usage").cloned();

        // Content blocks live under output.message.content[].
        let content_blocks = body
            .pointer("/output/message/content")
            .and_then(|c| c.as_array())
            .cloned()
            .unwrap_or_default();

        // Collect text and toolUse blocks separately.
        let text: String = content_blocks
            .iter()
            .filter_map(|b| b.get("text").and_then(|t| t.as_str()))
            .collect::<Vec<_>>()
            .join("");

        let tool_calls: Vec<serde_json::Value> = content_blocks
            .iter()
            .filter_map(|b| {
                b.get("toolUse").map(|tu| {
                    let arguments = serde_json::to_string(tu.get("input").unwrap_or(&json!({}))).unwrap_or_default();
                    json!({
                        "id": tu.get("toolUseId"),
                        "type": "function",
                        "function": {
                            "name": tu.get("name"),
                            "arguments": arguments
                        }
                    })
                })
            })
            .collect();

        let finish_reason = match stop_reason {
            "end_turn" => "stop",
            "tool_use" => "tool_calls",
            "max_tokens" => "length",
            "stop_sequence" => "stop",
            "content_filtered" => "content_filter",
            _ => "stop",
        };

        let input_tokens = usage
            .as_ref()
            .and_then(|u| u.get("inputTokens"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let output_tokens = usage
            .as_ref()
            .and_then(|u| u.get("outputTokens"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let response_id = body
            .get("requestId")
            .or_else(|| body.get("conversationId"))
            .cloned()
            .unwrap_or_else(|| json!("bedrock-resp"));

        let content_value: serde_json::Value = if text.is_empty() { json!(null) } else { json!(text) };

        let mut message = json!({"role": "assistant", "content": content_value});
        if !tool_calls.is_empty() {
            message["tool_calls"] = json!(tool_calls);
        }

        *body = json!({
            "id": response_id,
            "object": "chat.completion",
            "created": 0u64,
            "model": "",
            "choices": [{
                "index": 0,
                "message": message,
                "finish_reason": finish_reason
            }],
            "usage": {
                "prompt_tokens": input_tokens,
                "completion_tokens": output_tokens,
                "total_tokens": input_tokens + output_tokens
            }
        });

        Ok(())
    }

    /// Compute AWS SigV4 signing headers for the request.
    ///
    /// When the `bedrock` feature is enabled, derives the `Authorization`,
    /// `x-amz-date`, and (when a session token is present) `x-amz-security-token`
    /// headers from the current request parameters and AWS credentials.
    ///
    /// When the `bedrock` feature is disabled, returns an empty vector so
    /// requests work against override base-URLs (e.g. mock servers in tests).
    fn signing_headers(&self, method: &str, url: &str, body: &[u8]) -> Vec<(String, String)> {
        #[cfg(feature = "bedrock")]
        {
            sigv4_sign(method, url, body, &self.region).unwrap_or_default()
        }

        #[cfg(not(feature = "bedrock"))]
        {
            let _ = (method, url, body);
            vec![]
        }
    }
}

/// Compute AWS SigV4 signing headers using the `aws-sigv4` crate.
///
/// Reads credentials from the standard AWS environment variables:
/// - `AWS_ACCESS_KEY_ID` (required)
/// - `AWS_SECRET_ACCESS_KEY` (required)
/// - `AWS_SESSION_TOKEN` (optional, for temporary credentials)
///
/// Returns a vector of `(header-name, header-value)` pairs to inject into the
/// outgoing HTTP request.
#[cfg(feature = "bedrock")]
fn sigv4_sign(method: &str, url: &str, body: &[u8], region: &str) -> Result<Vec<(String, String)>> {
    use aws_credential_types::Credentials;
    use aws_sigv4::http_request::{SignableBody, SignableRequest, SigningSettings, sign};
    use aws_sigv4::sign::v4::SigningParams;

    let access_key = std::env::var("AWS_ACCESS_KEY_ID").map_err(|_| LiterLmError::BadRequest {
        message: "AWS_ACCESS_KEY_ID environment variable is required for Bedrock requests".into(),
    })?;
    let secret_key = std::env::var("AWS_SECRET_ACCESS_KEY").map_err(|_| LiterLmError::BadRequest {
        message: "AWS_SECRET_ACCESS_KEY environment variable is required for Bedrock requests".into(),
    })?;
    let session_token = std::env::var("AWS_SESSION_TOKEN").ok();

    let credentials = Credentials::new(
        access_key,
        secret_key,
        session_token,
        None, // expiry
        "env",
    );

    let identity = credentials.into();

    let signing_settings = SigningSettings::default();
    let now = std::time::SystemTime::now();

    let params = SigningParams::builder()
        .identity(&identity)
        .region(region)
        .name("bedrock")
        .time(now)
        .settings(signing_settings)
        .build()
        .map_err(|e| LiterLmError::BadRequest {
            message: format!("failed to build SigV4 signing params: {e}"),
        })?;

    // Build a signable request from the method, URL, and body.
    let signable = SignableRequest::new(
        method,
        url,
        std::iter::empty::<(&str, &str)>(),
        SignableBody::Bytes(body),
    )
    .map_err(|e| LiterLmError::BadRequest {
        message: format!("failed to create signable request: {e}"),
    })?;

    let signing_output = sign(signable, &params.into()).map_err(|e| LiterLmError::BadRequest {
        message: format!("SigV4 signing failed: {e}"),
    })?;

    let instructions = signing_output.output();
    let signed_headers: Vec<(String, String)> = instructions
        .headers()
        .map(|(name, value)| (name.to_owned(), value.to_owned()))
        .collect();

    Ok(signed_headers)
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::provider::Provider;

    fn provider() -> BedrockProvider {
        BedrockProvider::new("us-east-1")
    }

    // ── build_url ─────────────────────────────────────────────────────────────

    #[test]
    fn build_url_chat_completions() {
        let p = provider();
        let url = p.build_url("/chat/completions", "anthropic.claude-3-sonnet-20240229-v1:0");
        assert_eq!(
            url,
            "https://bedrock-runtime.us-east-1.amazonaws.com/model/anthropic.claude-3-sonnet-20240229-v1%3a0/converse"
        );
    }

    #[test]
    fn build_url_embeddings() {
        let p = provider();
        let url = p.build_url("/embeddings", "amazon.titan-embed-text-v1");
        assert_eq!(
            url,
            "https://bedrock-runtime.us-east-1.amazonaws.com/model/amazon.titan-embed-text-v1/invoke"
        );
    }

    #[test]
    fn build_url_other_path() {
        let p = provider();
        let url = p.build_url("/models", "any-model");
        assert_eq!(url, "https://bedrock-runtime.us-east-1.amazonaws.com/models");
    }

    // ── percent_encode_model ──────────────────────────────────────────────────

    #[test]
    fn percent_encode_model_colon() {
        let encoded = percent_encode_model("anthropic.claude-3-sonnet-20240229-v1:0");
        assert!(encoded.contains("%3a"), "colon should be percent-encoded: {encoded}");
        assert!(!encoded.contains(':'), "raw colon should not remain: {encoded}");
    }

    #[test]
    fn percent_encode_model_safe_chars() {
        let encoded = percent_encode_model("amazon.titan-embed-text-v1");
        assert_eq!(encoded, "amazon.titan-embed-text-v1");
    }

    // ── transform_request ─────────────────────────────────────────────────────

    #[test]
    fn transform_request_basic_chat() {
        let p = provider();
        let mut body = json!({
            "model": "anthropic.claude-3-sonnet",
            "messages": [
                {"role": "system", "content": "You are helpful."},
                {"role": "user", "content": "Hello!"}
            ],
            "max_tokens": 100,
            "temperature": 0.7
        });

        p.transform_request(&mut body).unwrap();

        // System messages extracted to top-level array.
        assert_eq!(body["system"][0]["text"], "You are helpful.");

        // User message converted to content blocks.
        assert_eq!(body["messages"][0]["role"], "user");
        assert_eq!(body["messages"][0]["content"][0]["text"], "Hello!");

        // Generation params in inferenceConfig.
        assert_eq!(body["inferenceConfig"]["maxTokens"], 100);
        assert_eq!(body["inferenceConfig"]["temperature"], 0.7);
    }

    #[test]
    fn transform_request_with_tool_calls() {
        let p = provider();
        let mut body = json!({
            "messages": [
                {"role": "user", "content": "What is the weather?"},
                {
                    "role": "assistant",
                    "content": null,
                    "tool_calls": [{
                        "id": "call_abc",
                        "type": "function",
                        "function": {"name": "get_weather", "arguments": "{\"city\":\"Berlin\"}"}
                    }]
                },
                {
                    "role": "tool",
                    "tool_call_id": "call_abc",
                    "content": "Sunny, 22°C"
                }
            ]
        });

        p.transform_request(&mut body).unwrap();

        let messages = body["messages"].as_array().unwrap();
        assert_eq!(messages.len(), 3);

        // Assistant message has toolUse block.
        let assistant = &messages[1];
        assert_eq!(assistant["role"], "assistant");
        let tool_use = &assistant["content"][0]["toolUse"];
        assert_eq!(tool_use["toolUseId"], "call_abc");
        assert_eq!(tool_use["name"], "get_weather");
        assert_eq!(tool_use["input"]["city"], "Berlin");

        // Tool result converted to user message with toolResult block.
        let tool_result_msg = &messages[2];
        assert_eq!(tool_result_msg["role"], "user");
        let tool_result = &tool_result_msg["content"][0]["toolResult"];
        assert_eq!(tool_result["toolUseId"], "call_abc");
        assert_eq!(tool_result["status"], "success");
    }

    #[test]
    fn transform_request_tools_schema() {
        let p = provider();
        let mut body = json!({
            "messages": [{"role": "user", "content": "hi"}],
            "tools": [{
                "type": "function",
                "function": {
                    "name": "search",
                    "description": "Search the web",
                    "parameters": {"type": "object", "properties": {"query": {"type": "string"}}}
                }
            }]
        });

        p.transform_request(&mut body).unwrap();

        let tools = body["toolConfig"]["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 1);
        let spec = &tools[0]["toolSpec"];
        assert_eq!(spec["name"], "search");
        assert_eq!(spec["description"], "Search the web");
        assert_eq!(spec["inputSchema"]["json"]["type"], "object");
    }

    // ── transform_response ────────────────────────────────────────────────────

    #[test]
    fn transform_response_basic() {
        let p = provider();
        let mut body = json!({
            "requestId": "req-123",
            "stopReason": "end_turn",
            "output": {
                "message": {
                    "role": "assistant",
                    "content": [{"text": "Hello, world!"}]
                }
            },
            "usage": {
                "inputTokens": 10,
                "outputTokens": 5
            }
        });

        p.transform_response(&mut body).unwrap();

        assert_eq!(body["object"], "chat.completion");
        assert_eq!(body["id"], "req-123");
        assert_eq!(body["choices"][0]["message"]["content"], "Hello, world!");
        assert_eq!(body["choices"][0]["finish_reason"], "stop");
        assert_eq!(body["usage"]["prompt_tokens"], 10);
        assert_eq!(body["usage"]["completion_tokens"], 5);
        assert_eq!(body["usage"]["total_tokens"], 15);
    }

    #[test]
    fn transform_response_tool_calls() {
        let p = provider();
        let mut body = json!({
            "stopReason": "tool_use",
            "output": {
                "message": {
                    "role": "assistant",
                    "content": [
                        {"toolUse": {
                            "toolUseId": "call_xyz",
                            "name": "get_weather",
                            "input": {"city": "Berlin"}
                        }}
                    ]
                }
            },
            "usage": {"inputTokens": 20, "outputTokens": 10}
        });

        p.transform_response(&mut body).unwrap();

        assert_eq!(body["choices"][0]["finish_reason"], "tool_calls");
        let tool_calls = body["choices"][0]["message"]["tool_calls"].as_array().unwrap();
        assert_eq!(tool_calls.len(), 1);
        assert_eq!(tool_calls[0]["id"], "call_xyz");
        assert_eq!(tool_calls[0]["function"]["name"], "get_weather");
        let args: serde_json::Value =
            serde_json::from_str(tool_calls[0]["function"]["arguments"].as_str().unwrap()).unwrap();
        assert_eq!(args["city"], "Berlin");
    }

    #[test]
    fn transform_response_finish_reason_mapping() {
        let p = provider();

        for (bedrock_reason, expected_oai_reason) in [
            ("end_turn", "stop"),
            ("tool_use", "tool_calls"),
            ("max_tokens", "length"),
            ("stop_sequence", "stop"),
            ("content_filtered", "content_filter"),
            ("unknown_future_reason", "stop"),
        ] {
            let mut body = json!({
                "stopReason": bedrock_reason,
                "output": {"message": {"role": "assistant", "content": [{"text": ""}]}},
                "usage": {"inputTokens": 0, "outputTokens": 0}
            });
            p.transform_response(&mut body).unwrap();
            assert_eq!(
                body["choices"][0]["finish_reason"], expected_oai_reason,
                "bedrock stopReason '{bedrock_reason}' should map to '{expected_oai_reason}'"
            );
        }
    }

    // ── model prefix / matching ───────────────────────────────────────────────

    #[test]
    fn strip_model_prefix() {
        let p = provider();
        assert_eq!(p.strip_model_prefix("bedrock/anthropic.claude-3"), "anthropic.claude-3");
        assert_eq!(p.strip_model_prefix("anthropic.claude-3"), "anthropic.claude-3");
    }

    #[test]
    fn matches_model() {
        let p = provider();
        assert!(p.matches_model("bedrock/anthropic.claude-3"));
        assert!(!p.matches_model("anthropic.claude-3"));
        assert!(!p.matches_model("gpt-4"));
    }
}
