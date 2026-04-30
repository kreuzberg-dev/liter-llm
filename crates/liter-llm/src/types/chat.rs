use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::common::{
    AssistantMessage, ChatCompletionTool, Message, ResponseFormat, StopSequence, ToolChoice, ToolType, Usage,
};
use crate::cost;

// ─── Finish Reason ────────────────────────────────────────────────────────────

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

// ─── Reasoning Effort ────────────────────────────────────────────────────────

/// Controls how much reasoning effort the model should use.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    Low,
    #[default]
    Medium,
    High,
}

// ─── Request ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    /// Whether to stream the response.
    ///
    /// Managed by the client layer — do not set directly.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopSequence>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Token bias map.  Uses `BTreeMap` (sorted keys) for deterministic
    /// serialization order — important when hashing or signing requests.
    pub logit_bias: Option<BTreeMap<String, f64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ChatCompletionTool>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
    /// Provider-specific extra parameters merged into the request body.
    /// Use for guardrails, safety settings, grounding config, etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_body: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StreamOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_usage: Option<bool>,
}

// ─── Response ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    /// Always `"chat.completion"` from OpenAI-compatible APIs.  Stored as a
    /// plain `String` so non-standard provider values do not break deserialization.
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
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
    #[must_use]
    pub fn estimated_cost(&self) -> Option<f64> {
        let usage = self.usage.as_ref()?;
        let cached = usage.prompt_tokens_details.as_ref().map_or(0, |d| d.cached_tokens);
        cost::completion_cost_with_cache(&self.model, usage.prompt_tokens, cached, usage.completion_tokens)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: AssistantMessage,
    pub finish_reason: Option<FinishReason>,
}

// ─── Stream Chunk ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    /// Always `"chat.completion.chunk"` from OpenAI-compatible APIs.  Stored
    /// as a plain `String` so non-standard provider values do not fail parsing.
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<StreamChoice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StreamChoice {
    pub index: u32,
    pub delta: StreamDelta,
    pub finish_reason: Option<FinishReason>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StreamDelta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<StreamToolCall>>,
    /// Deprecated legacy function_call delta; retained for API compatibility.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function_call: Option<StreamFunctionCall>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StreamToolCall {
    pub index: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub call_type: Option<ToolType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function: Option<StreamFunctionCall>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StreamFunctionCall {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::common::PromptTokensDetails;

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
        // claude-sonnet-4-5 has cache_read pricing in the registry.
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
        // gpt-4 has no cache pricing — cached tokens should not change cost.
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
        let a = make_response("gpt-4", usage_with_cached).estimated_cost().unwrap();
        let b = make_response("gpt-4", usage_no_details).estimated_cost().unwrap();
        assert!((a - b).abs() < 1e-12);
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
        let reser = serde_json::to_string(&usage).unwrap();
        assert!(reser.contains("\"cached_tokens\":30"));
    }
}
