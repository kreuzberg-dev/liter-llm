use std::borrow::Cow;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::error::{LiterLmError, Result};
use crate::provider::Provider;
use crate::types::ChatCompletionChunk;

/// Default Vertex AI location when none is specified.
const DEFAULT_LOCATION: &str = "us-central1";

/// Global counter for generating unique tool call IDs.
static TOOL_CALL_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Google Vertex AI / Gemini provider.
///
/// Differences from the OpenAI-compatible baseline:
/// - Auth uses `Authorization: Bearer <token>` where the token is a Google
///   Cloud OAuth2 access token (obtained via ADC, service account, or
///   `gcloud auth print-access-token`).
/// - The base URL is constructed from `VERTEXAI_PROJECT` and `VERTEXAI_LOCATION`
///   environment variables, or can be overridden via `base_url` in [`ClientConfig`].
///   The resulting URL follows the pattern:
///   `https://{location}-aiplatform.googleapis.com/v1/projects/{project}/locations/{location}`
/// - Model names are routed via the `vertex_ai/` prefix which is stripped
///   before being sent in the request body.
/// - The native Gemini `generateContent` format is used, not the OpenAI
///   `/chat/completions` path. Request and response are translated accordingly.
/// - Streaming uses SSE with `?alt=sse`; each chunk is a full `generateContent`
///   response JSON wrapped in a standard SSE `data:` line.
///
/// # Token management
///
/// Supply a pre-obtained access token as the `api_key` parameter.
/// Token refresh is the caller's responsibility.  A future release will add
/// ADC / service-account-based automatic refresh.
///
/// # Environment variables
///
/// - `VERTEXAI_PROJECT` (required): Google Cloud project ID.
/// - `VERTEXAI_LOCATION` (optional): GCP region, defaults to `us-central1`.
///
/// # Configuration
///
/// ```rust,ignore
/// // Option 1: Use environment variables (recommended).
/// // export VERTEXAI_PROJECT=my-project
/// // export VERTEXAI_LOCATION=us-central1
/// let config = ClientConfigBuilder::new("ya29.your-access-token").build();
/// let client = DefaultClient::new(config, Some("vertex_ai/gemini-2.0-flash"))?;
///
/// // Option 2: Explicit base_url override (bypasses env var resolution).
/// let config = ClientConfigBuilder::new("ya29.your-access-token")
///     .base_url(
///         "https://us-central1-aiplatform.googleapis.com/v1/\
///          projects/my-project/locations/us-central1",
///     )
///     .build();
/// let client = DefaultClient::new(config, Some("vertex_ai/gemini-2.0-flash"))?;
/// ```
pub struct VertexAiProvider {
    /// Cached base URL: `https://{location}-aiplatform.googleapis.com/v1/projects/{project}/locations/{location}`.
    base_url: String,
}

impl VertexAiProvider {
    /// Construct with an explicit project and location.
    #[must_use]
    pub fn new(project: impl Into<String>, location: impl Into<String>) -> Self {
        let project = project.into();
        let location = location.into();
        let base_url =
            format!("https://{location}-aiplatform.googleapis.com/v1/projects/{project}/locations/{location}");
        Self { base_url }
    }

    /// Construct from environment variables.
    ///
    /// Reads `VERTEXAI_PROJECT` and `VERTEXAI_LOCATION` (defaults to `us-central1`).
    /// If `VERTEXAI_PROJECT` is not set, the base URL will be empty and
    /// [`validate`] will return an error.
    #[must_use]
    pub fn from_env() -> Self {
        let project = std::env::var("VERTEXAI_PROJECT").unwrap_or_default();
        let location = std::env::var("VERTEXAI_LOCATION").unwrap_or_else(|_| DEFAULT_LOCATION.to_owned());
        if project.is_empty() {
            return Self {
                base_url: String::new(),
            };
        }
        Self::new(project, location)
    }
}

impl Provider for VertexAiProvider {
    fn name(&self) -> &str {
        "vertex_ai"
    }

    /// Vertex AI base URL constructed from project and location.
    ///
    /// Returns an empty string when the provider was constructed without a
    /// valid project (e.g. `VERTEXAI_PROJECT` not set). The [`validate`]
    /// method catches this at client construction time.
    fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Validate that required configuration is present.
    ///
    /// Checks that the base URL was successfully constructed from environment
    /// variables (`VERTEXAI_PROJECT` is required, `VERTEXAI_LOCATION` defaults
    /// to `us-central1`).
    fn validate(&self) -> Result<()> {
        if self.base_url.is_empty() {
            return Err(LiterLmError::BadRequest {
                message: "Vertex AI requires a project ID. \
                          Set VERTEXAI_PROJECT (and optionally VERTEXAI_LOCATION) \
                          in the environment, or provide an explicit base_url in \
                          ClientConfig."
                    .into(),
            });
        }
        Ok(())
    }

    fn auth_header<'a>(&'a self, api_key: &'a str) -> Option<(Cow<'static, str>, Cow<'a, str>)> {
        // Vertex AI requires an OAuth2 Bearer token.
        Some((Cow::Borrowed("Authorization"), Cow::Owned(format!("Bearer {api_key}"))))
    }

    fn matches_model(&self, model: &str) -> bool {
        model.starts_with("vertex_ai/")
    }

    fn strip_model_prefix<'m>(&self, model: &'m str) -> &'m str {
        model.strip_prefix("vertex_ai/").unwrap_or(model)
    }

    /// Build the full URL for a Gemini API request.
    ///
    /// Chat completions → `{base}/publishers/google/models/{model}:generateContent`
    /// Embeddings       → `{base}/publishers/google/models/{model}:predict`
    /// Other paths      → `{base}{endpoint_path}`
    fn build_url(&self, endpoint_path: &str, model: &str) -> String {
        let base = self.base_url();
        if base.is_empty() {
            // Caller must supply a base_url; will fail at validate() / HTTP layer.
            return String::new();
        }
        let base = base.trim_end_matches('/');
        if endpoint_path.contains("chat/completions") {
            format!("{base}/publishers/google/models/{model}:generateContent")
        } else if endpoint_path.contains("embeddings") {
            format!("{base}/publishers/google/models/{model}:predict")
        } else {
            format!("{base}{endpoint_path}")
        }
    }

    fn transform_request(&self, body: &mut serde_json::Value) -> Result<()> {
        transform_gemini_request(body)
    }

    fn transform_response(&self, body: &mut serde_json::Value) -> Result<()> {
        transform_gemini_response(body)
    }

    /// Build the streaming URL: appends `?alt=sse` to enable SSE streaming.
    ///
    /// Gemini's streaming endpoint uses the same path as the non-streaming
    /// `generateContent` endpoint but requires `?alt=sse` to switch to
    /// Server-Sent Events mode.
    fn build_stream_url(&self, endpoint_path: &str, model: &str) -> String {
        let url = self.build_url(endpoint_path, model);
        if url.is_empty() {
            return url;
        }
        format!("{url}?alt=sse")
    }

    fn parse_stream_event(&self, event_data: &str) -> Result<Option<ChatCompletionChunk>> {
        parse_gemini_stream_event(event_data)
    }
}

// ── Shared Gemini transform functions ────────────────────────────────────────
//
// These are `pub(crate)` so that both `VertexAiProvider` and `GoogleAiProvider`
// can reuse the same Gemini request/response translation logic.

/// Convert an OpenAI-style chat request to Gemini `generateContent` format.
///
/// Key translations:
/// - System messages → `systemInstruction.parts[]`.
/// - Assistant role → `model` role.
/// - Tool calls → `functionCall` parts; tool results → `functionResponse` parts.
/// - Generation parameters → `generationConfig`.
/// - Multimodal content arrays → Gemini's `inlineData` / `fileData` format.
/// - `response_format` → `generationConfig.responseMimeType`.
/// - `tool_choice` → `toolConfig.functionCallingConfig.mode`.
/// - `extra_body.safety_settings` → top-level `safetySettings` array.
/// - `extra_body.grounding_config` / `google_search_retrieval` → `tools` entry.
/// - `extra_body.cached_content` → top-level `cachedContent` field.
/// - `ContentPart::Document` → `inlineData` with the document's MIME type.
pub(crate) fn transform_gemini_request(body: &mut serde_json::Value) -> Result<()> {
    use serde_json::json;

    // Extract extra_body before taking ownership of fields, since it may contain
    // Gemini-specific extensions (safety_settings, grounding_config, cached_content).
    let extra_body = body
        .as_object_mut()
        .and_then(|o| o.remove("extra_body"))
        .and_then(|v| match v {
            serde_json::Value::Object(map) => Some(map),
            _ => None,
        });

    // Take ownership of the messages array to avoid cloning.
    let messages = body
        .as_object_mut()
        .and_then(|o| o.remove("messages"))
        .and_then(|v| match v {
            serde_json::Value::Array(arr) => Some(arr),
            _ => None,
        })
        .unwrap_or_default();

    let mut system_parts: Vec<serde_json::Value> = vec![];
    let mut contents: Vec<serde_json::Value> = vec![];

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
                let parts = convert_user_content_to_gemini(content);
                contents.push(json!({"role": "user", "parts": parts}));
            }
            "assistant" => {
                let mut parts: Vec<serde_json::Value> = vec![];
                if let Some(text) = content.and_then(|c| c.as_str())
                    && !text.is_empty()
                {
                    parts.push(json!({"text": text}));
                }
                // Convert OpenAI tool_calls to Gemini functionCall parts.
                if let Some(tool_calls) = msg.get("tool_calls").and_then(|t| t.as_array()) {
                    for tc in tool_calls {
                        let args: serde_json::Value = tc
                            .pointer("/function/arguments")
                            .and_then(|a| a.as_str())
                            .and_then(|s| serde_json::from_str(s).ok())
                            .unwrap_or_else(|| json!({}));
                        parts.push(json!({
                            "functionCall": {
                                "name": tc.pointer("/function/name"),
                                "args": args
                            }
                        }));
                    }
                }
                if parts.is_empty() {
                    parts.push(json!({"text": ""}));
                }
                // Gemini uses "model" role for assistant turns.
                contents.push(json!({"role": "model", "parts": parts}));
            }
            "tool" => {
                // Map tool result back to a user turn with a functionResponse part.
                // Gemini requires the function name — use the `name` field only.
                // The `tool_call_id` is an OpenAI correlation ID, not a function name,
                // so we must not fall back to it.
                let name = msg.get("name").and_then(|n| n.as_str()).unwrap_or("tool");
                let result_content = content.cloned().unwrap_or(json!(null));
                contents.push(json!({
                    "role": "user",
                    "parts": [{
                        "functionResponse": {
                            "name": name,
                            "response": {"result": result_content}
                        }
                    }]
                }));
            }
            _ => {}
        }
    }

    // Build generationConfig from OpenAI parameters.
    let mut gen_config = json!({});
    // Support both max_tokens (legacy) and max_completion_tokens (newer OpenAI spec).
    if let Some(max_tokens) = body.get("max_completion_tokens").or_else(|| body.get("max_tokens")) {
        gen_config["maxOutputTokens"] = max_tokens.clone();
    }
    if let Some(temp) = body.get("temperature") {
        gen_config["temperature"] = temp.clone();
    }
    if let Some(top_p) = body.get("top_p") {
        gen_config["topP"] = top_p.clone();
    }
    if let Some(stop) = body.get("stop") {
        let sequences = if let Some(s) = stop.as_str() {
            vec![json!(s)]
        } else {
            stop.as_array().cloned().unwrap_or_default()
        };
        gen_config["stopSequences"] = json!(sequences);
    }

    // Translate response_format to Gemini's responseMimeType.
    if let Some(rf) = body.get("response_format") {
        let rf_type = rf.get("type").and_then(|t| t.as_str()).unwrap_or("");
        match rf_type {
            "json_object" => {
                gen_config["responseMimeType"] = json!("application/json");
            }
            "json_schema" => {
                gen_config["responseMimeType"] = json!("application/json");
                // If a JSON schema is provided, pass it through.
                if let Some(schema) = rf.get("json_schema").and_then(|s| s.get("schema")) {
                    gen_config["responseSchema"] = schema.clone();
                }
            }
            // "text" or unknown types: no special handling needed.
            _ => {}
        }
    }

    // Translate OpenAI tools array to Gemini functionDeclarations.
    let mut tools_value = body.get("tools").and_then(|t| t.as_array()).map(|arr| {
        let declarations: Vec<serde_json::Value> = arr
            .iter()
            .map(|t| {
                let name = t.pointer("/function/name").cloned().unwrap_or(json!("unknown"));
                let description = t.pointer("/function/description").cloned().unwrap_or(json!(""));
                let parameters = t
                    .pointer("/function/parameters")
                    .cloned()
                    .unwrap_or_else(|| json!({"type": "object"}));
                json!({
                    "name": name,
                    "description": description,
                    "parameters": parameters
                })
            })
            .collect();
        json!([{"functionDeclarations": declarations}])
    });

    // Translate tool_choice to Gemini toolConfig.functionCallingConfig.mode.
    let tool_config = translate_tool_choice(body.get("tool_choice"));

    // ── extra_body extensions ────────────────────────────────────────────────
    let mut safety_settings: Option<serde_json::Value> = None;
    let mut cached_content: Option<serde_json::Value> = None;

    if let Some(ref eb) = extra_body {
        // Safety settings: inject as top-level safetySettings array.
        if let Some(ss) = eb.get("safety_settings") {
            safety_settings = Some(ss.clone());
        }

        // Grounding / Google Search: add google_search_retrieval to tools array.
        if eb.contains_key("grounding_config") || eb.contains_key("google_search_retrieval") {
            let grounding_tool = json!({"google_search_retrieval": {}});
            match &mut tools_value {
                Some(existing) => {
                    if let Some(arr) = existing.as_array_mut() {
                        arr.push(grounding_tool);
                    }
                }
                None => {
                    tools_value = Some(json!([grounding_tool]));
                }
            }
        }

        // Context caching: inject as top-level cachedContent field.
        if let Some(cc) = eb.get("cached_content") {
            cached_content = Some(cc.clone());
        }
    }

    let mut new_body = json!({"contents": contents});
    if !system_parts.is_empty() {
        // Gemini API requires camelCase: systemInstruction.
        new_body["systemInstruction"] = json!({"parts": system_parts});
    }
    if let Some(obj) = gen_config.as_object()
        && !obj.is_empty()
    {
        new_body["generationConfig"] = gen_config;
    }
    if let Some(tools) = tools_value {
        new_body["tools"] = tools;
    }
    if let Some(tc) = tool_config {
        new_body["toolConfig"] = tc;
    }
    if let Some(ss) = safety_settings {
        new_body["safetySettings"] = ss;
    }
    if let Some(cc) = cached_content {
        new_body["cachedContent"] = cc;
    }

    *body = new_body;
    Ok(())
}

/// Normalize a Gemini `generateContent` response to OpenAI chat completion format.
///
/// Gemini wraps the response in `candidates[0].content.parts[]`.
/// Finish reasons use Gemini terminology (`STOP`, `MAX_TOKENS`, `SAFETY`, ...)
/// and are mapped to the OpenAI `finish_reason` set.
///
/// If `groundingMetadata` is present on the candidate, it is included in the
/// response as `_grounding_metadata` for supplementary use by callers.
///
/// **Known limitation:** The `model` field in the normalized response is
/// always `""`.  Gemini/Vertex AI does not include the model name in its
/// response body -- the model is only present in the request URL path.
pub(crate) fn transform_gemini_response(body: &mut serde_json::Value) -> Result<()> {
    use serde_json::json;

    // Check for a blocked prompt (no candidates, but promptFeedback.blockReason set).
    let candidates = body.get("candidates").and_then(|c| c.as_array());
    if candidates.is_none_or(|c| c.is_empty()) {
        let block_reason = body
            .pointer("/promptFeedback/blockReason")
            .and_then(|r| r.as_str())
            .unwrap_or("UNKNOWN");
        let prompt_tokens = body
            .pointer("/usageMetadata/promptTokenCount")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        *body = json!({
            "id": "gemini-resp",
            "object": "chat.completion",
            "created": super::unix_timestamp_secs(),
            "model": "",
            "choices": [{
                "index": 0,
                "message": {"role": "assistant", "content": null},
                "finish_reason": "content_filter"
            }],
            "usage": {
                "prompt_tokens": prompt_tokens,
                "completion_tokens": 0,
                "total_tokens": prompt_tokens
            },
            "system_fingerprint": null,
            "_block_reason": block_reason
        });
        return Ok(());
    }

    let candidate = body.pointer("/candidates/0").cloned();
    let finish_reason_raw = candidate
        .as_ref()
        .and_then(|c| c.get("finishReason"))
        .and_then(|f| f.as_str())
        .unwrap_or("STOP");
    let parts = candidate
        .as_ref()
        .and_then(|c| c.pointer("/content/parts"))
        .and_then(|p| p.as_array())
        .cloned()
        .unwrap_or_default();

    // Collect text content from parts.
    let text: String = parts
        .iter()
        .filter_map(|p| p.get("text").and_then(|t| t.as_str()))
        .collect::<Vec<_>>()
        .join("");

    // Collect functionCall parts and convert to OpenAI tool_calls.
    // Each call gets a unique ID via an atomic counter to avoid collisions
    // when the same function is called multiple times.
    let tool_calls: Vec<serde_json::Value> = parts
        .iter()
        .filter_map(|p| {
            p.get("functionCall").map(|fc| {
                let name = fc.get("name").and_then(|n| n.as_str()).unwrap_or("unknown");
                let call_id = TOOL_CALL_COUNTER.fetch_add(1, Ordering::Relaxed);
                let arguments = serde_json::to_string(fc.get("args").unwrap_or(&json!({}))).unwrap_or_default();
                json!({
                    "id": format!("call_{name}_{call_id}"),
                    "type": "function",
                    "function": {
                        "name": fc.get("name"),
                        "arguments": arguments
                    }
                })
            })
        })
        .collect();

    let finish_reason = match finish_reason_raw {
        "STOP" => "stop",
        "MAX_TOKENS" => "length",
        "SAFETY" | "RECITATION" | "BLOCKLIST" | "PROHIBITED_CONTENT" | "SPII" | "IMAGE_SAFETY" => "content_filter",
        "LANGUAGE" | "OTHER" => "stop",
        "TOOL_CODE" | "FUNCTION_CALL" => "tool_calls",
        _ => "stop",
    };

    let prompt_tokens = body
        .pointer("/usageMetadata/promptTokenCount")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let completion_tokens = body
        .pointer("/usageMetadata/candidatesTokenCount")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let response_id = body.get("responseId").cloned().unwrap_or_else(|| json!("gemini-resp"));

    let content_value: serde_json::Value = if text.is_empty() { json!(null) } else { json!(text) };

    let mut message = json!({"role": "assistant", "content": content_value});
    if !tool_calls.is_empty() {
        message["tool_calls"] = json!(tool_calls);
    }

    // Extract grounding metadata if present (supplementary data from Google Search grounding).
    let grounding_metadata = candidate.as_ref().and_then(|c| c.get("groundingMetadata")).cloned();

    let mut result = json!({
        "id": response_id,
        "object": "chat.completion",
        "created": super::unix_timestamp_secs(),
        "model": "",
        "choices": [{
            "index": 0,
            "message": message,
            "finish_reason": finish_reason
        }],
        "usage": {
            "prompt_tokens": prompt_tokens,
            "completion_tokens": completion_tokens,
            "total_tokens": prompt_tokens + completion_tokens
        }
    });

    if let Some(gm) = grounding_metadata {
        result["_grounding_metadata"] = gm;
    }

    *body = result;

    Ok(())
}

/// Parse a single SSE event from Gemini's streaming endpoint.
///
/// Gemini streaming uses SSE with `?alt=sse`.  Each event data is a complete
/// `generateContent` JSON response.  We reuse `transform_gemini_response` to
/// normalize it into OpenAI format, then build a `ChatCompletionChunk` from
/// the first choice's message content.
///
/// **Note:** The `id` and `model` fields are empty strings on every chunk
/// because Gemini's streaming payloads do not include them, and this parser
/// is stateless.
pub(crate) fn parse_gemini_stream_event(event_data: &str) -> Result<Option<ChatCompletionChunk>> {
    // NOTE: `[DONE]` is handled at the SSE parser level; no check needed here.
    if event_data.trim().is_empty() {
        return Ok(None);
    }

    let mut body: serde_json::Value = serde_json::from_str(event_data).map_err(|e| LiterLmError::Streaming {
        message: format!("failed to parse Gemini SSE data: {e}"),
    })?;

    // Normalize to OpenAI chat completion format.
    transform_gemini_response(&mut body)?;

    // Extract fields from the normalized response.
    let id = body
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("gemini-resp")
        .to_owned();
    let model = body.get("model").and_then(|v| v.as_str()).unwrap_or("").to_owned();

    let choice = body.pointer("/choices/0");
    let content = choice
        .and_then(|c| c.pointer("/message/content"))
        .and_then(|v| v.as_str())
        .map(ToOwned::to_owned);
    let finish_reason_str = choice
        .and_then(|c| c.get("finish_reason"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // Extract tool_calls from the normalized message if present.
    let stream_tool_calls = choice
        .and_then(|c| c.pointer("/message/tool_calls"))
        .and_then(|v| v.as_array())
        .filter(|arr| !arr.is_empty())
        .map(|arr| {
            use crate::types::{StreamFunctionCall, StreamToolCall, ToolType};
            arr.iter()
                .enumerate()
                .map(|(idx, tc)| StreamToolCall {
                    index: idx as u32,
                    id: tc.get("id").and_then(|v| v.as_str()).map(ToOwned::to_owned),
                    call_type: Some(ToolType::Function),
                    function: tc.get("function").map(|f| StreamFunctionCall {
                        name: f.get("name").and_then(|v| v.as_str()).map(ToOwned::to_owned),
                        arguments: f.get("arguments").and_then(|v| v.as_str()).map(ToOwned::to_owned),
                    }),
                })
                .collect::<Vec<_>>()
        });

    use crate::types::{FinishReason, StreamChoice, StreamDelta};

    let finish_reason = match finish_reason_str {
        "stop" => Some(FinishReason::Stop),
        "length" => Some(FinishReason::Length),
        "tool_calls" => Some(FinishReason::ToolCalls),
        "content_filter" => Some(FinishReason::ContentFilter),
        _ => None,
    };

    let chunk = ChatCompletionChunk {
        id,
        object: "chat.completion.chunk".to_owned(),
        created: super::unix_timestamp_secs(),
        model,
        choices: vec![StreamChoice {
            index: 0,
            delta: StreamDelta {
                role: Some("assistant".to_owned()),
                content,
                tool_calls: stream_tool_calls,
                function_call: None,
                refusal: None,
            },
            finish_reason,
        }],
        usage: None,
        system_fingerprint: None,
        service_tier: None,
    };

    Ok(Some(chunk))
}

// ── Helper functions ──────────────────────────────────────────────────────────

/// Convert OpenAI user content (string or content-part array) to Gemini parts.
///
/// Handles four cases:
/// 1. Plain string -> single text part.
/// 2. Array of content parts -> each part converted to Gemini format.
/// 3. `ContentPart::Document` -> Gemini `inlineData` with the document's MIME type.
/// 4. None/null -> single empty text part.
pub(crate) fn convert_user_content_to_gemini(content: Option<&serde_json::Value>) -> Vec<serde_json::Value> {
    use serde_json::json;

    match content {
        Some(serde_json::Value::String(s)) => vec![json!({"text": s})],
        Some(serde_json::Value::Array(parts)) => {
            parts
                .iter()
                .filter_map(|part| {
                    let part_type = part.get("type").and_then(|t| t.as_str())?;
                    match part_type {
                        "text" => {
                            let text = part.get("text").and_then(|t| t.as_str()).unwrap_or("");
                            Some(json!({"text": text}))
                        }
                        "image_url" => {
                            let url = part.pointer("/image_url/url").and_then(|u| u.as_str())?;
                            if url.starts_with("data:") {
                                // data:<media_type>;base64,<data>
                                if let Some((header, data)) = url.split_once(',') {
                                    let mime_type = header.trim_start_matches("data:").trim_end_matches(";base64");
                                    return Some(json!({
                                        "inlineData": {
                                            "mimeType": mime_type,
                                            "data": data
                                        }
                                    }));
                                }
                            }
                            // Plain URL -- use Gemini's fileData format.
                            Some(json!({
                                "fileData": {
                                    "mimeType": "image/jpeg",
                                    "fileUri": url
                                }
                            }))
                        }
                        "document" => {
                            // ContentPart::Document -> Gemini inlineData.
                            let doc = part.get("document")?;
                            let data = doc.get("data").and_then(|d| d.as_str())?;
                            let media_type = doc
                                .get("media_type")
                                .and_then(|m| m.as_str())
                                .unwrap_or("application/pdf");
                            Some(json!({
                                "inlineData": {
                                    "mimeType": media_type,
                                    "data": data
                                }
                            }))
                        }
                        _ => {
                            // Unknown content part types: fall back to text representation.
                            let text = part.get("text").and_then(|t| t.as_str()).unwrap_or("");
                            if text.is_empty() {
                                None
                            } else {
                                Some(json!({"text": text}))
                            }
                        }
                    }
                })
                .collect()
        }
        _ => vec![json!({"text": ""})],
    }
}

/// Translate OpenAI `tool_choice` to Gemini `toolConfig.functionCallingConfig`.
///
/// OpenAI `tool_choice` values:
/// - `"none"` -> `NONE`
/// - `"auto"` -> `AUTO`
/// - `"required"` -> `ANY`
/// - `{"type": "function", "function": {"name": "..."}}` -> `ANY` with `allowedFunctionNames`
fn translate_tool_choice(tool_choice: Option<&serde_json::Value>) -> Option<serde_json::Value> {
    use serde_json::json;

    let tc = tool_choice?;

    if let Some(s) = tc.as_str() {
        let mode = match s {
            "none" => "NONE",
            "auto" => "AUTO",
            "required" => "ANY",
            _ => return None,
        };
        return Some(json!({
            "functionCallingConfig": {
                "mode": mode
            }
        }));
    }

    // Object form: {"type": "function", "function": {"name": "specific_fn"}}
    if let Some(name) = tc.pointer("/function/name").and_then(|n| n.as_str()) {
        return Some(json!({
            "functionCallingConfig": {
                "mode": "ANY",
                "allowedFunctionNames": [name]
            }
        }));
    }

    None
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::provider::Provider;

    fn provider() -> VertexAiProvider {
        VertexAiProvider::new("test-project", "us-central1")
    }

    fn provider_without_project() -> VertexAiProvider {
        VertexAiProvider {
            base_url: String::new(),
        }
    }

    // ── validate ──────────────────────────────────────────────────────────────

    #[test]
    fn validate_succeeds_with_project() {
        let p = provider();
        assert!(p.validate().is_ok());
    }

    #[test]
    fn validate_fails_without_project() {
        let p = provider_without_project();
        let err = p.validate().unwrap_err();
        assert!(
            err.to_string().contains("VERTEXAI_PROJECT"),
            "error should mention VERTEXAI_PROJECT"
        );
    }

    // ── base_url ──────────────────────────────────────────────────────────────

    #[test]
    fn base_url_constructed_from_project_and_location() {
        let p = provider();
        assert_eq!(
            p.base_url(),
            "https://us-central1-aiplatform.googleapis.com/v1/projects/test-project/locations/us-central1"
        );
    }

    #[test]
    fn base_url_custom_location() {
        let p = VertexAiProvider::new("my-proj", "europe-west1");
        assert_eq!(
            p.base_url(),
            "https://europe-west1-aiplatform.googleapis.com/v1/projects/my-proj/locations/europe-west1"
        );
    }

    // ── build_url ─────────────────────────────────────────────────────────────

    #[test]
    fn build_url_returns_empty_without_base() {
        let p = provider_without_project();
        let url = p.build_url("/chat/completions", "gemini-2.0-flash");
        assert!(url.is_empty(), "should return empty string without a base URL");
    }

    #[test]
    fn build_url_chat_completions() {
        let p = provider();
        let url = p.build_url("/chat/completions", "gemini-2.0-flash");
        assert!(url.ends_with("/publishers/google/models/gemini-2.0-flash:generateContent"));
    }

    #[test]
    fn build_url_embeddings() {
        let p = provider();
        let url = p.build_url("/embeddings", "text-embedding-004");
        assert!(url.ends_with("/publishers/google/models/text-embedding-004:predict"));
    }

    // ── transform_request ─────────────────────────────────────────────────────

    #[test]
    fn transform_request_basic_chat() {
        let p = provider();
        let mut body = json!({
            "messages": [
                {"role": "system", "content": "You are a helpful assistant."},
                {"role": "user", "content": "Hello!"}
            ],
            "max_tokens": 200,
            "temperature": 0.5
        });

        p.transform_request(&mut body).unwrap();

        // System instruction extracted with camelCase key required by Gemini API.
        assert_eq!(
            body["systemInstruction"]["parts"][0]["text"],
            "You are a helpful assistant."
        );

        // User message converted to Gemini format.
        assert_eq!(body["contents"][0]["role"], "user");
        assert_eq!(body["contents"][0]["parts"][0]["text"], "Hello!");

        // Generation config set.
        assert_eq!(body["generationConfig"]["maxOutputTokens"], 200);
        assert_eq!(body["generationConfig"]["temperature"], 0.5);
    }

    #[test]
    fn transform_request_assistant_becomes_model_role() {
        let p = provider();
        let mut body = json!({
            "messages": [
                {"role": "user", "content": "Hi"},
                {"role": "assistant", "content": "Hello there!"}
            ]
        });

        p.transform_request(&mut body).unwrap();

        assert_eq!(body["contents"][1]["role"], "model");
        assert_eq!(body["contents"][1]["parts"][0]["text"], "Hello there!");
    }

    #[test]
    fn transform_request_with_tool_calls() {
        let p = provider();
        let mut body = json!({
            "messages": [
                {"role": "user", "content": "What is the weather in Berlin?"},
                {
                    "role": "assistant",
                    "content": null,
                    "tool_calls": [{
                        "id": "call_1",
                        "type": "function",
                        "function": {"name": "get_weather", "arguments": "{\"city\":\"Berlin\"}"}
                    }]
                },
                {
                    "role": "tool",
                    "name": "get_weather",
                    "tool_call_id": "call_1",
                    "content": "Sunny, 22°C"
                }
            ]
        });

        p.transform_request(&mut body).unwrap();

        let contents = body["contents"].as_array().unwrap();
        assert_eq!(contents.len(), 3);

        // Assistant turn with functionCall part.
        let model_turn = &contents[1];
        assert_eq!(model_turn["role"], "model");
        let fn_call = &model_turn["parts"][0]["functionCall"];
        assert_eq!(fn_call["name"], "get_weather");
        assert_eq!(fn_call["args"]["city"], "Berlin");

        // Tool result as user turn with functionResponse.
        let tool_turn = &contents[2];
        assert_eq!(tool_turn["role"], "user");
        let fn_resp = &tool_turn["parts"][0]["functionResponse"];
        assert_eq!(fn_resp["name"], "get_weather");
    }

    #[test]
    fn transform_request_stop_sequences() {
        let p = provider();
        let mut body = json!({
            "messages": [{"role": "user", "content": "hi"}],
            "stop": ["END", "STOP"]
        });

        p.transform_request(&mut body).unwrap();

        let stop_seqs = body["generationConfig"]["stopSequences"].as_array().unwrap();
        assert_eq!(stop_seqs.len(), 2);
        assert_eq!(stop_seqs[0], "END");
        assert_eq!(stop_seqs[1], "STOP");
    }

    // ── transform_request: safety settings ───────────────────────────────────

    #[test]
    fn transform_request_safety_settings_from_extra_body() {
        let p = provider();
        let mut body = json!({
            "messages": [{"role": "user", "content": "hi"}],
            "extra_body": {
                "safety_settings": [
                    {"category": "HARM_CATEGORY_HATE_SPEECH", "threshold": "BLOCK_MEDIUM_AND_ABOVE"},
                    {"category": "HARM_CATEGORY_DANGEROUS_CONTENT", "threshold": "BLOCK_ONLY_HIGH"}
                ]
            }
        });

        p.transform_request(&mut body).unwrap();

        let settings = body["safetySettings"].as_array().unwrap();
        assert_eq!(settings.len(), 2);
        assert_eq!(settings[0]["category"], "HARM_CATEGORY_HATE_SPEECH");
        assert_eq!(settings[0]["threshold"], "BLOCK_MEDIUM_AND_ABOVE");
        assert_eq!(settings[1]["category"], "HARM_CATEGORY_DANGEROUS_CONTENT");
    }

    // ── transform_request: grounding / Google Search ─────────────────────────

    #[test]
    fn transform_request_grounding_config_adds_google_search() {
        let p = provider();
        let mut body = json!({
            "messages": [{"role": "user", "content": "What happened today?"}],
            "extra_body": {
                "grounding_config": {}
            }
        });

        p.transform_request(&mut body).unwrap();

        let tools = body["tools"].as_array().unwrap();
        assert!(
            tools.iter().any(|t| t.get("google_search_retrieval").is_some()),
            "tools should contain google_search_retrieval"
        );
    }

    #[test]
    fn transform_request_google_search_retrieval_with_existing_tools() {
        let p = provider();
        let mut body = json!({
            "messages": [{"role": "user", "content": "hi"}],
            "tools": [{"type": "function", "function": {"name": "f", "parameters": {}}}],
            "extra_body": {
                "google_search_retrieval": {}
            }
        });

        p.transform_request(&mut body).unwrap();

        let tools = body["tools"].as_array().unwrap();
        // Should have functionDeclarations + google_search_retrieval.
        assert_eq!(tools.len(), 2);
        assert!(tools[0].get("functionDeclarations").is_some());
        assert!(tools[1].get("google_search_retrieval").is_some());
    }

    // ── transform_request: context caching ───────────────────────────────────

    #[test]
    fn transform_request_cached_content_from_extra_body() {
        let p = provider();
        let cached = "projects/xxx/locations/xxx/cachedContents/abc123";
        let mut body = json!({
            "messages": [{"role": "user", "content": "hi"}],
            "extra_body": {
                "cached_content": cached
            }
        });

        p.transform_request(&mut body).unwrap();

        assert_eq!(body["cachedContent"], cached);
    }

    // ── transform_request: document handling ─────────────────────────────────

    #[test]
    fn transform_request_document_content_part() {
        let p = provider();
        let mut body = json!({
            "messages": [{
                "role": "user",
                "content": [
                    {"type": "text", "text": "Summarize this document."},
                    {
                        "type": "document",
                        "document": {
                            "data": "JVBERi0xLjQ=",
                            "media_type": "application/pdf"
                        }
                    }
                ]
            }]
        });

        p.transform_request(&mut body).unwrap();

        let parts = body["contents"][0]["parts"].as_array().unwrap();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0]["text"], "Summarize this document.");
        assert_eq!(parts[1]["inlineData"]["mimeType"], "application/pdf");
        assert_eq!(parts[1]["inlineData"]["data"], "JVBERi0xLjQ=");
    }

    // ── transform_response ────────────────────────────────────────────────────

    #[test]
    fn transform_response_basic() {
        let p = provider();
        let mut body = json!({
            "responseId": "resp-gemini-123",
            "candidates": [{
                "content": {
                    "role": "model",
                    "parts": [{"text": "Hello from Gemini!"}]
                },
                "finishReason": "STOP"
            }],
            "usageMetadata": {
                "promptTokenCount": 8,
                "candidatesTokenCount": 6
            }
        });

        p.transform_response(&mut body).unwrap();

        assert_eq!(body["object"], "chat.completion");
        assert_eq!(body["id"], "resp-gemini-123");
        assert_eq!(body["choices"][0]["message"]["content"], "Hello from Gemini!");
        assert_eq!(body["choices"][0]["finish_reason"], "stop");
        assert_eq!(body["usage"]["prompt_tokens"], 8);
        assert_eq!(body["usage"]["completion_tokens"], 6);
        assert_eq!(body["usage"]["total_tokens"], 14);
    }

    #[test]
    fn transform_response_tool_calls_have_unique_ids() {
        let p = provider();
        let mut body = json!({
            "candidates": [{
                "content": {
                    "role": "model",
                    "parts": [
                        {
                            "functionCall": {
                                "name": "get_weather",
                                "args": {"city": "Berlin"}
                            }
                        },
                        {
                            "functionCall": {
                                "name": "get_weather",
                                "args": {"city": "Paris"}
                            }
                        }
                    ]
                },
                "finishReason": "STOP"
            }],
            "usageMetadata": {"promptTokenCount": 10, "candidatesTokenCount": 5}
        });

        p.transform_response(&mut body).unwrap();

        let tool_calls = body["choices"][0]["message"]["tool_calls"].as_array().unwrap();
        assert_eq!(tool_calls.len(), 2);

        // Both calls should have the function name "get_weather" but different IDs.
        let id0 = tool_calls[0]["id"].as_str().unwrap();
        let id1 = tool_calls[1]["id"].as_str().unwrap();
        assert_ne!(id0, id1, "tool call IDs must be unique even for the same function");
        assert!(id0.starts_with("call_get_weather_"));
        assert!(id1.starts_with("call_get_weather_"));

        // Verify arguments are correct.
        let args0: serde_json::Value =
            serde_json::from_str(tool_calls[0]["function"]["arguments"].as_str().unwrap()).unwrap();
        let args1: serde_json::Value =
            serde_json::from_str(tool_calls[1]["function"]["arguments"].as_str().unwrap()).unwrap();
        assert_eq!(args0["city"], "Berlin");
        assert_eq!(args1["city"], "Paris");
    }

    #[test]
    fn transform_response_single_tool_call() {
        let p = provider();
        let mut body = json!({
            "candidates": [{
                "content": {
                    "role": "model",
                    "parts": [{
                        "functionCall": {
                            "name": "get_weather",
                            "args": {"city": "Berlin"}
                        }
                    }]
                },
                "finishReason": "STOP"
            }],
            "usageMetadata": {"promptTokenCount": 10, "candidatesTokenCount": 5}
        });

        p.transform_response(&mut body).unwrap();

        let tool_calls = body["choices"][0]["message"]["tool_calls"].as_array().unwrap();
        assert_eq!(tool_calls.len(), 1);
        assert_eq!(tool_calls[0]["function"]["name"], "get_weather");
        // ID should contain the function name and a unique counter.
        let id = tool_calls[0]["id"].as_str().unwrap();
        assert!(
            id.starts_with("call_get_weather_"),
            "id should start with call_get_weather_, got: {id}"
        );
    }

    #[test]
    fn transform_response_finish_reason_mapping() {
        let p = provider();

        for (gemini_reason, expected_oai_reason) in [
            ("STOP", "stop"),
            ("MAX_TOKENS", "length"),
            ("SAFETY", "content_filter"),
            ("RECITATION", "content_filter"),
            ("BLOCKLIST", "content_filter"),
            ("PROHIBITED_CONTENT", "content_filter"),
            ("UNKNOWN_FUTURE_REASON", "stop"),
        ] {
            let mut body = json!({
                "candidates": [{
                    "content": {"role": "model", "parts": [{"text": ""}]},
                    "finishReason": gemini_reason
                }],
                "usageMetadata": {"promptTokenCount": 0, "candidatesTokenCount": 0}
            });
            p.transform_response(&mut body).unwrap();
            assert_eq!(
                body["choices"][0]["finish_reason"], expected_oai_reason,
                "Gemini finishReason '{gemini_reason}' should map to '{expected_oai_reason}'"
            );
        }
    }

    #[test]
    fn transform_response_grounding_metadata_preserved() {
        let p = provider();
        let mut body = json!({
            "candidates": [{
                "content": {
                    "role": "model",
                    "parts": [{"text": "grounded answer"}]
                },
                "finishReason": "STOP",
                "groundingMetadata": {
                    "searchEntryPoint": {"renderedContent": "<html>...</html>"},
                    "groundingChunks": [{"web": {"uri": "https://example.com", "title": "Example"}}]
                }
            }],
            "usageMetadata": {"promptTokenCount": 5, "candidatesTokenCount": 3}
        });

        p.transform_response(&mut body).unwrap();

        assert_eq!(body["choices"][0]["message"]["content"], "grounded answer");
        assert!(
            body.get("_grounding_metadata").is_some(),
            "grounding metadata should be preserved"
        );
        assert!(body["_grounding_metadata"]["groundingChunks"].as_array().unwrap().len() == 1);
    }

    // ── parse_stream_event ────────────────────────────────────────────────────

    #[test]
    fn parse_stream_event_empty_returns_none() {
        let p = provider();
        let result = p.parse_stream_event("").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn parse_stream_event_done_is_handled_at_sse_level() {
        // `[DONE]` is now caught by the SSE parser before reaching the provider.
        // If it were to reach the provider, it would be invalid JSON.
        let p = provider();
        let result = p.parse_stream_event("[DONE]");
        assert!(
            result.is_err(),
            "[DONE] is not valid JSON and should error if it reaches the provider"
        );
    }

    #[test]
    fn parse_stream_event_basic_chunk() {
        let p = provider();
        let event_data = r#"{
            "candidates": [{
                "content": {"role": "model", "parts": [{"text": "Hello"}]},
                "finishReason": "STOP"
            }],
            "usageMetadata": {"promptTokenCount": 5, "candidatesTokenCount": 2}
        }"#;

        let chunk = p.parse_stream_event(event_data).unwrap().unwrap();

        assert_eq!(chunk.object, "chat.completion.chunk");
        assert_eq!(chunk.choices[0].delta.content.as_deref(), Some("Hello"));
    }

    // ── model prefix / matching ───────────────────────────────────────────────

    #[test]
    fn strip_model_prefix() {
        let p = provider();
        assert_eq!(p.strip_model_prefix("vertex_ai/gemini-2.0-flash"), "gemini-2.0-flash");
        assert_eq!(p.strip_model_prefix("gemini-2.0-flash"), "gemini-2.0-flash");
    }

    #[test]
    fn matches_model() {
        let p = provider();
        assert!(p.matches_model("vertex_ai/gemini-2.0-flash"));
        assert!(!p.matches_model("gemini-2.0-flash"));
        assert!(!p.matches_model("gpt-4"));
    }

    // ── multimodal content ────────────────────────────────────────────────────

    #[test]
    fn transform_request_multimodal_user_content() {
        let p = provider();
        let mut body = json!({
            "messages": [{
                "role": "user",
                "content": [
                    {"type": "text", "text": "What is in this image?"},
                    {"type": "image_url", "image_url": {"url": "data:image/jpeg;base64,/9j/abc=="}}
                ]
            }]
        });

        p.transform_request(&mut body).unwrap();

        let parts = body["contents"][0]["parts"].as_array().unwrap();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0]["text"], "What is in this image?");
        assert_eq!(parts[1]["inlineData"]["mimeType"], "image/jpeg");
        assert_eq!(parts[1]["inlineData"]["data"], "/9j/abc==");
    }

    #[test]
    fn transform_request_multimodal_url_image() {
        let p = provider();
        let mut body = json!({
            "messages": [{
                "role": "user",
                "content": [
                    {"type": "text", "text": "Describe this."},
                    {"type": "image_url", "image_url": {"url": "https://example.com/image.jpg"}}
                ]
            }]
        });

        p.transform_request(&mut body).unwrap();

        let parts = body["contents"][0]["parts"].as_array().unwrap();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[1]["fileData"]["fileUri"], "https://example.com/image.jpg");
    }

    // ── response_format translation ───────────────────────────────────────────

    #[test]
    fn transform_request_response_format_json_object() {
        let p = provider();
        let mut body = json!({
            "messages": [{"role": "user", "content": "hi"}],
            "response_format": {"type": "json_object"}
        });

        p.transform_request(&mut body).unwrap();

        assert_eq!(body["generationConfig"]["responseMimeType"], "application/json");
    }

    #[test]
    fn transform_request_response_format_json_schema() {
        let p = provider();
        let mut body = json!({
            "messages": [{"role": "user", "content": "hi"}],
            "response_format": {
                "type": "json_schema",
                "json_schema": {
                    "name": "test",
                    "schema": {"type": "object", "properties": {"name": {"type": "string"}}}
                }
            }
        });

        p.transform_request(&mut body).unwrap();

        assert_eq!(body["generationConfig"]["responseMimeType"], "application/json");
        assert_eq!(body["generationConfig"]["responseSchema"]["type"], "object");
    }

    // ── tool_choice translation ───────────────────────────────────────────────

    #[test]
    fn transform_request_tool_choice_auto() {
        let p = provider();
        let mut body = json!({
            "messages": [{"role": "user", "content": "hi"}],
            "tools": [{"type": "function", "function": {"name": "f", "parameters": {}}}],
            "tool_choice": "auto"
        });

        p.transform_request(&mut body).unwrap();

        assert_eq!(body["toolConfig"]["functionCallingConfig"]["mode"], "AUTO");
    }

    #[test]
    fn transform_request_tool_choice_none() {
        let p = provider();
        let mut body = json!({
            "messages": [{"role": "user", "content": "hi"}],
            "tools": [{"type": "function", "function": {"name": "f", "parameters": {}}}],
            "tool_choice": "none"
        });

        p.transform_request(&mut body).unwrap();

        assert_eq!(body["toolConfig"]["functionCallingConfig"]["mode"], "NONE");
    }

    #[test]
    fn transform_request_tool_choice_required() {
        let p = provider();
        let mut body = json!({
            "messages": [{"role": "user", "content": "hi"}],
            "tools": [{"type": "function", "function": {"name": "f", "parameters": {}}}],
            "tool_choice": "required"
        });

        p.transform_request(&mut body).unwrap();

        assert_eq!(body["toolConfig"]["functionCallingConfig"]["mode"], "ANY");
    }

    #[test]
    fn transform_request_tool_choice_specific_function() {
        let p = provider();
        let mut body = json!({
            "messages": [{"role": "user", "content": "hi"}],
            "tools": [{"type": "function", "function": {"name": "get_weather", "parameters": {}}}],
            "tool_choice": {"type": "function", "function": {"name": "get_weather"}}
        });

        p.transform_request(&mut body).unwrap();

        assert_eq!(body["toolConfig"]["functionCallingConfig"]["mode"], "ANY");
        assert_eq!(
            body["toolConfig"]["functionCallingConfig"]["allowedFunctionNames"][0],
            "get_weather"
        );
    }

    // ── helper function tests ─────────────────────────────────────────────────

    #[test]
    fn convert_user_content_string() {
        let content = json!("Hello!");
        let parts = convert_user_content_to_gemini(Some(&content));
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0]["text"], "Hello!");
    }

    #[test]
    fn convert_user_content_array_with_image() {
        let content = json!([
            {"type": "text", "text": "What is this?"},
            {"type": "image_url", "image_url": {"url": "data:image/png;base64,iVBOR"}}
        ]);
        let parts = convert_user_content_to_gemini(Some(&content));
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0]["text"], "What is this?");
        assert_eq!(parts[1]["inlineData"]["mimeType"], "image/png");
        assert_eq!(parts[1]["inlineData"]["data"], "iVBOR");
    }

    #[test]
    fn convert_user_content_none() {
        let parts = convert_user_content_to_gemini(None);
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0]["text"], "");
    }

    #[test]
    fn convert_user_content_document_part() {
        let content = json!([
            {"type": "text", "text": "Read this PDF."},
            {"type": "document", "document": {"data": "base64data==", "media_type": "application/pdf"}}
        ]);
        let parts = convert_user_content_to_gemini(Some(&content));
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0]["text"], "Read this PDF.");
        assert_eq!(parts[1]["inlineData"]["mimeType"], "application/pdf");
        assert_eq!(parts[1]["inlineData"]["data"], "base64data==");
    }

    #[test]
    fn translate_tool_choice_string_values() {
        let auto = translate_tool_choice(Some(&json!("auto"))).unwrap();
        assert_eq!(auto["functionCallingConfig"]["mode"], "AUTO");

        let none = translate_tool_choice(Some(&json!("none"))).unwrap();
        assert_eq!(none["functionCallingConfig"]["mode"], "NONE");

        let required = translate_tool_choice(Some(&json!("required"))).unwrap();
        assert_eq!(required["functionCallingConfig"]["mode"], "ANY");
    }

    #[test]
    fn translate_tool_choice_specific_function() {
        let tc = json!({"type": "function", "function": {"name": "my_fn"}});
        let result = translate_tool_choice(Some(&tc)).unwrap();
        assert_eq!(result["functionCallingConfig"]["mode"], "ANY");
        assert_eq!(result["functionCallingConfig"]["allowedFunctionNames"][0], "my_fn");
    }

    #[test]
    fn translate_tool_choice_none_input() {
        assert!(translate_tool_choice(None).is_none());
    }
}
