use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::common::{
    AssistantMessage, ChatCompletionTool, Message, ResponseFormat, StopSequence, ToolChoice, ToolType, Usage,
};
use crate::cost;

/// Output modality requested from the model.
///
/// Passed as `modalities: ["text", "audio"]` (OpenAI) or translated to
/// `generationConfig.responseModalities` (Gemini / Vertex AI).
///
/// # Example
///
/// ```
/// use liter_llm::types::{ChatCompletionRequest, Modality};
///
/// let req = ChatCompletionRequest {
///     model: "gpt-4o-audio-preview".into(),
///     modalities: Some(vec![Modality::Text, Modality::Audio]),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Modality {
    /// Text output (the default for all providers).
    Text,
    /// Audio / speech output.
    Audio,
    /// Image output (Gemini Imagen, gpt-image-1).
    Image,
}

/// Why a choice stopped generating tokens.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    #[default]
    Stop,
    Length,
    ToolCalls,
    ContentFilter,
    /// Deprecated legacy finish reason; retained for API compatibility.
    #[serde(rename = "function_call")]
    FunctionCall,
    /// Catch-all for unknown finish reasons returned by non-OpenAI providers.
    ///
    /// Note: this intentionally does **not** carry the original string (e.g.
    /// `Other(String)`).  Using `#[serde(other)]` requires a unit variant, and
    /// switching to `#[serde(untagged)]` would change deserialization semantics
    /// for all variants.  The original value can be recovered by inspecting the
    /// raw JSON if needed.
    #[serde(other)]
    Other,
}

#[cfg_attr(alef, alef(skip))]
impl std::fmt::Display for FinishReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self)
            .ok()
            .and_then(|v| v.as_str().map(str::to_owned))
            .unwrap_or_default();
        f.write_str(&s)
    }
}

/// Controls how much reasoning effort the model should use.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    Low,
    #[default]
    Medium,
    High,
    Minimal,
    Max,
}

/// Chat completion request (compatible with OpenAI and similar APIs).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatCompletionRequest {
    /// Model ID (e.g., `"gpt-4o-mini"`, `"claude-3-5-sonnet"`).
    pub model: String,
    /// Conversation history from oldest to newest.
    pub messages: Vec<Message>,
    /// Sampling temperature. Higher increases randomness, lower is more deterministic.
    /// Defaults to 1.0.
    ///
    /// The accepted range depends on the provider the request is routed to. OpenAI-compatible
    /// providers accept `[0.0, 2.0]`; Anthropic and Amazon Bedrock both cap it at `1.0`, and
    /// for those two a value above the cap is rejected with a `BadRequest` error before the
    /// request is sent, rather than being silently clamped or left for the provider to reject.
    ///
    /// No range is enforced for providers whose own documentation does not state one — the
    /// value is forwarded and the provider decides. Consult the target provider's reference
    /// rather than assuming `[0.0, 2.0]` is portable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Nucleus sampling parameter. Lower is more focused.
    ///
    /// Accepted ranges vary by provider (most document `[0.0, 1.0]`, but this is not
    /// universal — check the target provider's own documentation for its exact bounds).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// Number of chat completions to generate. Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    /// Whether to stream the response.
    ///
    /// Managed by the client layer — do not set directly.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Stop sequence(s) that halt token generation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopSequence>,
    /// Max output tokens. Different from max_completion_tokens in some providers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    /// Presence penalty in `[-2.0, 2.0]`. Positive discourages repeated topics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    /// Frequency penalty in `[-2.0, 2.0]`. Positive discourages repeated tokens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    /// Token bias map.  Uses `BTreeMap` (sorted keys) for deterministic
    /// serialization order — important when hashing or signing requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<BTreeMap<String, f64>>,
    /// User identifier for request tracking and abuse detection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Tools the model can invoke.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ChatCompletionTool>>,
    /// Tool usage mode (auto, required, none, or specific tool).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// Whether the model can call multiple tools in parallel. Defaults to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    /// Output format constraint (text, JSON, JSON schema).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// Streaming options (e.g., include_usage).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,
    /// Random seed for reproducible outputs. Provider support varies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Reasoning effort level (minimal, low, medium, high, max) for extended-thinking models.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
    /// Output modalities to request from the model.
    ///
    /// For OpenAI audio models, pass `["text", "audio"]`. Vertex AI / Gemini
    /// translates these to `generationConfig.responseModalities` (uppercase).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<Modality>>,
    /// Whether to return log probabilities of the output tokens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    /// Number of most-likely tokens to return log probabilities for, `0..=20`.
    /// Requires `logprobs` to be `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,
    /// Upper bound on generated tokens, including reasoning tokens.
    ///
    /// Supersedes `max_tokens` on OpenAI reasoning models, which reject
    /// `max_tokens` outright.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u64>,
    /// Latency tier to process the request under (e.g. `"auto"`, `"default"`,
    /// `"flex"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
    /// Whether to store the completion for later retrieval by the provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// Developer-defined tags attached to the completion.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
    /// Predicted output, for latency reduction when much of the response is
    /// known ahead of time.
    ///
    /// Untyped: the shape is provider-defined and still evolving, and the
    /// value is forwarded verbatim. ~keep
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prediction: Option<serde_json::Value>,
    /// Audio output parameters, required when `modalities` includes `audio`.
    ///
    /// Untyped for the same reason as `prediction`. ~keep
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<serde_json::Value>,
    /// Web-search tool configuration for search-enabled models.
    ///
    /// Untyped for the same reason as `prediction`. ~keep
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_search_options: Option<serde_json::Value>,
    /// Provider-specific extra parameters merged into the request body.
    /// Use for guardrails, safety settings, grounding config, etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_body: Option<serde_json::Value>,
}

/// Options for streaming responses.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StreamOptions {
    /// If true, include token usage in the final stream chunk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_usage: Option<bool>,
}

/// Chat completion response from the API.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    /// Unique identifier for this response.
    pub id: String,
    /// Always `"chat.completion"` from OpenAI-compatible APIs.  Stored as a
    /// plain `String` so non-standard provider values do not break deserialization.
    pub object: String,
    /// Unix timestamp of response creation.
    pub created: u64,
    /// Model used to generate the response.
    pub model: String,
    /// List of completion choices.
    pub choices: Vec<Choice>,
    /// Token usage statistics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
    /// Fingerprint of the system configuration (OpenAI-specific).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
    /// Service tier used (OpenAI-specific).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
}

impl ChatCompletionResponse {
    /// Estimate the cost of this response based on embedded pricing data.
    ///
    /// Returns `None` if:
    /// - the `model` field is not present in the embedded pricing registry, or
    /// - the `usage` field is absent from the response.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let cost = response.estimated_cost();
    /// if let Some(usd) = cost {
    ///     println!("Request cost: ${usd:.6}");
    /// }
    /// ```
    #[cfg_attr(alef, alef(skip))]
    #[must_use]
    pub fn estimated_cost(&self) -> Option<f64> {
        let usage = self.usage.as_ref()?;
        let cached = usage.prompt_tokens_details.as_ref().map_or(0, |d| d.cached_tokens);
        cost::completion_cost_with_cache(&self.model, usage.prompt_tokens, cached, usage.completion_tokens)
    }
}

/// A single completion choice.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    /// Index of this choice in the choices array.
    pub index: u32,
    /// The assistant's message response.
    ///
    /// Serialized with an explicit `role: "assistant"`. The field is not stored
    /// on [`AssistantMessage`] because [`Message`] is an internally-tagged enum
    /// keyed on `role`, so a stored field would emit the key twice inside a
    /// request. OpenAI's response schema requires it here. ~keep
    #[serde(serialize_with = "serialize_assistant_message_with_role")]
    pub message: AssistantMessage,
    /// Why the model stopped generating (stop, length, tool_calls, content_filter, etc.).
    pub finish_reason: Option<FinishReason>,
    /// Per-token log probabilities, when the request asked for them.
    ///
    /// Required by OpenAI's response schema as an always-present, nullable key,
    /// so this is deliberately not `skip_serializing_if`. ~keep
    #[serde(default)]
    pub logprobs: Option<serde_json::Value>,
}

/// Serialize an [`AssistantMessage`] with the `role: "assistant"` discriminator
/// that OpenAI's response schema requires on `choices[].message`.
fn serialize_assistant_message_with_role<S>(message: &AssistantMessage, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    #[derive(Serialize)]
    struct WithRole<'a> {
        role: &'static str,
        #[serde(flatten)]
        message: &'a AssistantMessage,
    }

    WithRole {
        role: "assistant",
        message,
    }
    .serialize(serializer)
}

/// A streamed chunk of a chat completion response.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionChunk {
    /// Unique identifier for this stream.
    ///
    // ~keep `#[serde(default)]` on the header fields: OpenAI-compatible providers
    // (e.g. OpenCode Zen/Go) emit a trailing metadata-only event — empty `choices`
    // plus cost/usage — with no `id`/`object`/`created`/`model` right before
    // `[DONE]`. Without defaults serde fails with `missing field 'id'` and aborts
    // an otherwise-complete stream (#155). Defaults let it decode to an empty chunk.
    #[serde(default)]
    pub id: String,
    /// Always `"chat.completion.chunk"` from OpenAI-compatible APIs.  Stored
    /// as a plain `String` so non-standard provider values do not fail parsing.
    #[serde(default)]
    pub object: String,
    /// Unix timestamp of chunk creation.
    #[serde(default)]
    pub created: u64,
    /// Model used to generate the chunk.
    #[serde(default)]
    pub model: String,
    /// Streaming choices (delta updates).
    ///
    // ~keep `#[serde(default)]` for the same reason as the header fields above,
    // plus one of its own: a provider that aborts mid-stream sends an
    // error-object event with no `choices` at all (see `parse_stream_event`).
    // That guard runs before deserialization and returns the provider's real
    // error, so this default never silently absorbs one.
    #[serde(default)]
    pub choices: Vec<StreamChoice>,
    /// Token usage (typically only in the final chunk).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
    /// Fingerprint of the system configuration (OpenAI-specific).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
    /// Service tier used (OpenAI-specific).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
}

/// A streaming choice with incremental delta.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StreamChoice {
    /// Index of this choice in the choices array.
    pub index: u32,
    /// Incremental update to the message (content, tool calls, etc.).
    pub delta: StreamDelta,
    /// Why the stream ended (present only in final chunk).
    pub finish_reason: Option<FinishReason>,
}

/// Incremental delta in a stream chunk.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StreamDelta {
    /// Role (typically present only in the first chunk).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Partial content chunk (e.g., a few words of the response).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Partial tool calls being streamed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<StreamToolCall>>,
    /// Deprecated legacy function_call delta; retained for API compatibility.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function_call: Option<StreamFunctionCall>,
    /// Partial refusal message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    /// Partial reasoning/thinking tokens (OpenAI-compatible extension used by DeepSeek R1, Qwen, etc.).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
}

/// A streaming tool call being built incrementally.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StreamToolCall {
    /// Index of this tool call in the tool_calls array.
    pub index: u32,
    /// Tool call ID (typically in the first chunk for this call).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Tool type (typically "function").
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub call_type: Option<ToolType>,
    /// Partial function name and arguments.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function: Option<StreamFunctionCall>,
}

/// Partial function call details in a stream.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StreamFunctionCall {
    /// Function name (typically in the first chunk).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Partial JSON arguments chunk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::common::PromptTokensDetails;

    #[test]
    fn chat_completion_chunk_deserializes_without_id_field() {
        // ~keep Trailing metadata-only event OpenCode Zen/Go emits before `[DONE]`
        // (#155): no `id`/`object`/`created`/`model`, empty `choices`, extra
        // `cost`/`x-opencode-type`/`normalizedUsage` fields serde must ignore.
        let payload = r#"{"choices":[],"x-opencode-type":"inference-cost","cost":"0.00001400","normalizedUsage":{"inputTokens":84,"outputTokens":8,"reasoningTokens":8,"cacheReadTokens":0,"cacheWrite5mTokens":0,"cacheWrite1hTokens":0}}"#;
        let chunk: ChatCompletionChunk =
            serde_json::from_str(payload).expect("metadata event lacking `id` must not fail to parse");
        assert_eq!(chunk.id, "");
        assert_eq!(chunk.object, "");
        assert_eq!(chunk.created, 0);
        assert_eq!(chunk.model, "");
        assert!(chunk.choices.is_empty());
        assert!(chunk.usage.is_none());
    }

    #[test]
    fn reasoning_effort_minimal_and_max_round_trip_through_serde() {
        assert_eq!(
            serde_json::to_value(ReasoningEffort::Minimal).expect("serialize should not fail"),
            serde_json::json!("minimal")
        );
        assert_eq!(
            serde_json::from_value::<ReasoningEffort>(serde_json::json!("minimal"))
                .expect("deserialize should not fail"),
            ReasoningEffort::Minimal
        );
        assert_eq!(
            serde_json::to_value(ReasoningEffort::Max).expect("serialize should not fail"),
            serde_json::json!("max")
        );
        assert_eq!(
            serde_json::from_value::<ReasoningEffort>(serde_json::json!("max")).expect("deserialize should not fail"),
            ReasoningEffort::Max
        );
    }

    fn make_response(model: &str, usage: Usage) -> ChatCompletionResponse {
        ChatCompletionResponse {
            id: "test".into(),
            object: "chat.completion".into(),
            created: 0,
            model: model.into(),
            choices: vec![],
            usage: Some(usage),
            system_fingerprint: None,
            service_tier: None,
        }
    }

    #[test]
    fn estimated_cost_applies_cache_discount_when_prompt_tokens_details_present() {
        let resp = make_response(
            "claude-sonnet-4-5",
            Usage {
                prompt_tokens: 1_000,
                completion_tokens: 50,
                total_tokens: 1_050,
                prompt_tokens_details: Some(PromptTokensDetails {
                    cached_tokens: 200,
                    audio_tokens: 0,
                }),
            },
        );
        let with_cache = resp.estimated_cost().expect("should price");
        let no_cache = make_response(
            "claude-sonnet-4-5",
            Usage {
                prompt_tokens: 1_000,
                completion_tokens: 50,
                total_tokens: 1_050,
                prompt_tokens_details: None,
            },
        )
        .estimated_cost()
        .expect("should price");
        assert!(
            with_cache < no_cache,
            "cached cost ({with_cache}) must be cheaper than uncached ({no_cache})"
        );
    }

    #[test]
    fn estimated_cost_ignores_cached_tokens_when_no_pricing_difference() {
        let usage_with_cached = Usage {
            prompt_tokens: 1_000,
            completion_tokens: 50,
            total_tokens: 1_050,
            prompt_tokens_details: Some(PromptTokensDetails {
                cached_tokens: 500,
                audio_tokens: 0,
            }),
        };
        let usage_no_details = Usage {
            prompt_tokens: 1_000,
            completion_tokens: 50,
            total_tokens: 1_050,
            prompt_tokens_details: None,
        };
        let a = make_response("gpt-4", usage_with_cached)
            .estimated_cost()
            .expect("cost estimation should succeed for known model");
        let b = make_response("gpt-4", usage_no_details)
            .estimated_cost()
            .expect("cost estimation should succeed for known model");
        assert!((a - b).abs() < 1e-12);
    }

    #[test]
    fn modalities_serializes_when_present() {
        let req = ChatCompletionRequest {
            model: "gpt-4o-audio-preview".into(),
            modalities: Some(vec![Modality::Text, Modality::Audio]),
            ..Default::default()
        };
        let value = serde_json::to_value(&req).expect("must serialise");
        assert_eq!(value["modalities"], serde_json::json!(["text", "audio"]));
    }

    #[test]
    fn modalities_omitted_when_none() {
        let req = ChatCompletionRequest {
            model: "gpt-4o".into(),
            ..Default::default()
        };
        let value = serde_json::to_value(&req).expect("must serialise");
        assert!(value.get("modalities").is_none(), "modalities must be absent when None");
    }

    #[test]
    fn usage_round_trips_prompt_tokens_details_via_serde() {
        let json = r#"{
            "prompt_tokens": 100,
            "completion_tokens": 20,
            "total_tokens": 120,
            "prompt_tokens_details": {"cached_tokens": 30, "audio_tokens": 0}
        }"#;
        let usage: Usage = serde_json::from_str(json).expect("valid OpenAI usage shape");
        assert_eq!(usage.prompt_tokens_details.as_ref().map(|d| d.cached_tokens), Some(30));
        let reser = serde_json::to_string(&usage).expect("serialization should not fail");
        assert!(reser.contains("\"cached_tokens\":30"));
    }

    #[test]
    fn stream_delta_reasoning_content_omitted_when_none() {
        let delta = StreamDelta {
            content: Some("hi".into()),
            ..Default::default()
        };
        let json = serde_json::to_string(&delta).expect("serialization should not fail");
        assert!(
            !json.contains("reasoning_content"),
            "reasoning_content key must be absent when None, got: {json}"
        );
    }

    #[test]
    fn stream_delta_reasoning_content_populated_from_deserialized_chunk() {
        let json = r#"{
            "role": "assistant",
            "content": null,
            "reasoning_content": "thinking..."
        }"#;
        let delta: StreamDelta = serde_json::from_str(json).expect("valid delta shape");
        assert_eq!(delta.reasoning_content.as_deref(), Some("thinking..."));
    }

    /// `ChatCompletionRequest` is `deny_unknown_fields` and the proxy parses
    /// the client's body straight into it, so any documented OpenAI request
    /// field the struct is missing becomes a hard 400 for a stock SDK client
    /// rather than an ignored parameter.  Each of these is a real field a
    /// current OpenAI client can send.
    #[test]
    fn chat_request_accepts_documented_openai_fields() {
        let payload = r#"{
            "model": "gpt-4o",
            "messages": [{"role": "user", "content": "hi"}],
            "logprobs": true,
            "top_logprobs": 5,
            "max_completion_tokens": 1024,
            "service_tier": "flex",
            "store": true,
            "metadata": {"run": "nightly"},
            "prediction": {"type": "content", "content": "draft"},
            "audio": {"voice": "alloy", "format": "wav"},
            "web_search_options": {"search_context_size": "medium"}
        }"#;

        let req: ChatCompletionRequest =
            serde_json::from_str(payload).expect("documented OpenAI fields must not be rejected");

        assert_eq!(req.logprobs, Some(true));
        assert_eq!(req.top_logprobs, Some(5));
        assert_eq!(req.max_completion_tokens, Some(1024));
        assert_eq!(req.service_tier.as_deref(), Some("flex"));
        assert_eq!(req.store, Some(true));
        assert_eq!(
            req.metadata.as_ref().and_then(|m| m.get("run")).map(String::as_str),
            Some("nightly")
        );
        assert!(req.prediction.is_some());
        assert!(req.audio.is_some());
        assert!(req.web_search_options.is_some());
    }

    /// The provider request body is `serde_json::to_value` of this struct, so
    /// a field that parses but does not round-trip would be silently dropped
    /// on the way to the provider — a worse failure than the 400 it replaced.
    #[test]
    fn chat_request_forwards_documented_openai_fields() {
        let req = ChatCompletionRequest {
            model: "gpt-4o".into(),
            logprobs: Some(true),
            top_logprobs: Some(5),
            max_completion_tokens: Some(1024),
            service_tier: Some("flex".into()),
            store: Some(true),
            metadata: Some(BTreeMap::from([("run".to_owned(), "nightly".to_owned())])),
            ..Default::default()
        };

        let body = serde_json::to_value(&req).expect("request must serialize");

        assert_eq!(body["logprobs"], serde_json::json!(true));
        assert_eq!(body["top_logprobs"], serde_json::json!(5));
        assert_eq!(body["max_completion_tokens"], serde_json::json!(1024));
        assert_eq!(body["service_tier"], serde_json::json!("flex"));
        assert_eq!(body["store"], serde_json::json!(true));
        assert_eq!(body["metadata"], serde_json::json!({"run": "nightly"}));
    }

    /// Absent optional fields must stay absent from the wire body — emitting
    /// them as explicit nulls makes providers that validate strictly reject
    /// an otherwise ordinary request.
    #[test]
    fn chat_request_omits_absent_optional_fields() {
        let req = ChatCompletionRequest {
            model: "gpt-4o".into(),
            ..Default::default()
        };

        let body = serde_json::to_value(&req).expect("request must serialize");
        let obj = body.as_object().expect("request serializes as an object");

        for field in [
            "logprobs",
            "top_logprobs",
            "max_completion_tokens",
            "service_tier",
            "store",
            "metadata",
            "prediction",
            "audio",
            "web_search_options",
        ] {
            assert!(!obj.contains_key(field), "absent `{field}` must not be serialized");
        }
    }
}
