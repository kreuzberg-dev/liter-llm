use std::borrow::Cow;

use serde_json::{Value, json};

use crate::error::{LiterLmError, Result};
use crate::provider::Provider;
use crate::types::{ChatCompletionChunk, FinishReason, StreamChoice, StreamDelta, StreamFunctionCall, StreamToolCall};

static ANTHROPIC_EXTRA_HEADERS: &[(&str, &str)] = &[("anthropic-version", "2023-06-01")];

/// Default max_tokens for Anthropic requests when none is specified.
/// Anthropic requires this field; OpenAI makes it optional.
const DEFAULT_MAX_TOKENS: u64 = 4096;

/// Anthropic provider (Claude model family).
///
/// Differences from the OpenAI-compatible baseline:
/// - Auth uses `x-api-key` instead of `Authorization: Bearer`.
/// - Requires a mandatory `anthropic-version` header on every request.
/// - Model names start with `claude-` or are routed via the `anthropic/` prefix.
/// - Chat endpoint is `/messages`, not `/chat/completions`.
/// - Request and response JSON formats differ from OpenAI.
pub struct AnthropicProvider;

impl Provider for AnthropicProvider {
    fn name(&self) -> &str {
        "anthropic"
    }

    fn base_url(&self) -> &str {
        "https://api.anthropic.com/v1"
    }

    fn auth_header<'a>(&'a self, api_key: &'a str) -> Option<(Cow<'static, str>, Cow<'a, str>)> {
        // Anthropic uses x-api-key, not Authorization: Bearer.
        Some((Cow::Borrowed("x-api-key"), Cow::Borrowed(api_key)))
    }

    fn extra_headers(&self) -> &'static [(&'static str, &'static str)] {
        ANTHROPIC_EXTRA_HEADERS
    }

    fn matches_model(&self, model: &str) -> bool {
        model.starts_with("claude-") || model.starts_with("anthropic/")
    }

    fn strip_model_prefix<'m>(&self, model: &'m str) -> &'m str {
        model.strip_prefix("anthropic/").unwrap_or(model)
    }

    /// Anthropic uses `/messages` instead of `/chat/completions`.
    fn chat_completions_path(&self) -> &str {
        "/messages"
    }

    /// Transform an OpenAI-format request body into Anthropic Messages API format.
    ///
    /// Key differences handled here:
    /// - System messages extracted to top-level `system` field as content blocks.
    /// - User/assistant messages converted to Anthropic content block arrays.
    /// - Tool messages (role=tool) become user messages with `tool_result` blocks.
    /// - `max_tokens` defaults to 4096 if not set (Anthropic requires it).
    /// - `stop` renamed to `stop_sequences` and normalised to an array.
    /// - `tool_choice` mapped from OpenAI semantics to Anthropic semantics.
    /// - Tools converted from OpenAI `function` wrappers to Anthropic `input_schema` format.
    /// - Unsupported parameters removed: `n`, `presence_penalty`, `frequency_penalty`,
    ///   `logit_bias`, `stream` (the client handles stream separately).
    fn transform_request(&self, body: &mut Value) -> Result<()> {
        // ── 1. Extract system messages ────────────────────────────────────────
        let messages = body
            .get("messages")
            .and_then(|m| m.as_array())
            .cloned()
            .unwrap_or_default();

        let mut system_blocks: Vec<Value> = Vec::new();
        let mut non_system_messages: Vec<Value> = Vec::new();

        for msg in messages {
            let role = msg.get("role").and_then(|r| r.as_str()).unwrap_or("");
            match role {
                "system" | "developer" => {
                    // Both system and developer roles map to Anthropic system content.
                    if let Some(content) = msg.get("content").and_then(|c| c.as_str()) {
                        system_blocks.push(json!({"type": "text", "text": content}));
                    }
                }
                _ => non_system_messages.push(msg),
            }
        }

        if !system_blocks.is_empty() {
            body["system"] = json!(system_blocks);
        }

        // ── 2. Convert non-system messages to Anthropic format ────────────────
        let converted_messages: Vec<Value> = non_system_messages
            .into_iter()
            .map(convert_message_to_anthropic)
            .collect();

        body["messages"] = json!(converted_messages);

        // ── 3. Ensure max_tokens is present (required by Anthropic) ───────────
        if body.get("max_tokens").is_none() {
            body["max_tokens"] = json!(DEFAULT_MAX_TOKENS);
        }

        // ── 4. Convert stop → stop_sequences (must be array) ─────────────────
        if let Some(stop) = body.get("stop").cloned() {
            let stop_sequences = match stop {
                Value::String(s) => json!([s]),
                Value::Array(_) => stop,
                _ => json!([]),
            };
            body["stop_sequences"] = stop_sequences;
            body.as_object_mut().map(|o| o.remove("stop"));
        }

        // ── 5. Convert tool_choice ─────────────────────────────────────────────
        if let Some(tool_choice) = body.get("tool_choice").cloned() {
            let anthropic_tool_choice = convert_tool_choice(&tool_choice);
            match anthropic_tool_choice {
                Some(tc) => {
                    body["tool_choice"] = tc;
                }
                None => {
                    // tool_choice: "none" → remove tools entirely
                    body.as_object_mut().map(|o| o.remove("tool_choice"));
                    body.as_object_mut().map(|o| o.remove("tools"));
                }
            }
        }

        // ── 6. Convert tools from OpenAI format to Anthropic format ───────────
        if let Some(tools) = body.get("tools").cloned()
            && let Some(tools_array) = tools.as_array()
        {
            let anthropic_tools: Vec<Value> = tools_array.iter().map(convert_tool_to_anthropic).collect();
            body["tools"] = json!(anthropic_tools);
        }

        // ── 7. Remove unsupported parameters ──────────────────────────────────
        if let Some(obj) = body.as_object_mut() {
            for key in &[
                "n",
                "presence_penalty",
                "frequency_penalty",
                "logit_bias",
                "stream",
                "stream_options",
                "parallel_tool_calls",
                "response_format",
                "service_tier",
                "user",
            ] {
                obj.remove(*key);
            }
        }

        Ok(())
    }

    /// Normalize an Anthropic Messages API response into OpenAI chat completion format.
    ///
    /// Anthropic response shape:
    /// ```json
    /// { "id": "msg_...", "type": "message", "role": "assistant",
    ///   "content": [{"type": "text", "text": "..."}],
    ///   "stop_reason": "end_turn",
    ///   "usage": {"input_tokens": N, "output_tokens": M} }
    /// ```
    fn transform_response(&self, body: &mut Value) -> Result<()> {
        // Only transform if this looks like an Anthropic response (has "stop_reason").
        if body.get("stop_reason").is_none() {
            return Ok(());
        }

        let id = body.get("id").cloned().unwrap_or(json!(""));
        let model = body.get("model").cloned().unwrap_or(json!(""));

        let content_blocks = body.get("content").and_then(|v| v.as_array()).cloned();

        // Extract text content by joining all text blocks.
        let text_content: Option<String> = content_blocks.as_ref().map(|blocks| {
            blocks
                .iter()
                .filter(|b| b.get("type").and_then(|t| t.as_str()) == Some("text"))
                .filter_map(|b| b.get("text").and_then(|t| t.as_str()))
                .collect::<Vec<_>>()
                .join("")
        });

        // Extract tool_use blocks into OpenAI-format tool_calls.
        let tool_calls: Option<Vec<Value>> = content_blocks.as_ref().map(|blocks| {
            blocks
                .iter()
                .filter(|b| b.get("type").and_then(|t| t.as_str()) == Some("tool_use"))
                .map(|b| {
                    let arguments = serde_json::to_string(b.get("input").unwrap_or(&json!({}))).unwrap_or_default();
                    json!({
                        "id": b.get("id").cloned().unwrap_or(json!("")),
                        "type": "function",
                        "function": {
                            "name": b.get("name").cloned().unwrap_or(json!("")),
                            "arguments": arguments
                        }
                    })
                })
                .collect()
        });

        // Map Anthropic stop_reason → OpenAI finish_reason.
        let stop_reason = body.get("stop_reason").and_then(|v| v.as_str()).unwrap_or("end_turn");
        let finish_reason = map_stop_reason(stop_reason);

        // Map Anthropic usage → OpenAI usage.
        let input_tokens = body
            .pointer("/usage/input_tokens")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let output_tokens = body
            .pointer("/usage/output_tokens")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        // Build the message object — content is null when only tool_calls are present.
        let has_tool_calls = tool_calls.as_ref().is_some_and(|tc| !tc.is_empty());
        let message_content = if has_tool_calls && text_content.as_deref().unwrap_or("").is_empty() {
            Value::Null
        } else {
            json!(text_content)
        };

        let mut message = json!({
            "role": "assistant",
            "content": message_content
        });

        if let (Some(tc), true) = (tool_calls, has_tool_calls) {
            message["tool_calls"] = json!(tc);
        }

        *body = json!({
            "id": id,
            "object": "chat.completion",
            "created": 0u64,
            "model": model,
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

    /// Parse an Anthropic SSE event into an OpenAI-compatible `ChatCompletionChunk`.
    ///
    /// Anthropic event types handled:
    /// - `message_start`: emits a role-only delta chunk.
    /// - `content_block_start`: emits empty delta (tool_use: emits tool_call header chunk).
    /// - `content_block_delta`: emits text or tool input JSON delta.
    /// - `message_delta`: emits final chunk with finish_reason and usage.
    /// - `message_stop`: signals end of stream, returns `Ok(None)`.
    /// - `content_block_stop`, `ping`: skipped (returns empty delta chunk).
    /// - `error`: returns `Err(LiterLmError::Streaming)`.
    fn parse_stream_event(&self, event_data: &str) -> Result<Option<ChatCompletionChunk>> {
        if event_data == "[DONE]" {
            return Ok(None);
        }

        let event: Value = serde_json::from_str(event_data).map_err(|e| LiterLmError::Streaming {
            message: format!("failed to parse Anthropic SSE event: {e}"),
        })?;

        let event_type = event.get("type").and_then(|t| t.as_str()).unwrap_or("");

        match event_type {
            "message_start" => {
                let msg = &event["message"];
                let id = msg.get("id").and_then(|v| v.as_str()).unwrap_or("").to_owned();
                let model = msg.get("model").and_then(|v| v.as_str()).unwrap_or("").to_owned();

                // Anthropic sends initial usage in message_start (input tokens only).
                let input_tokens = msg.pointer("/usage/input_tokens").and_then(|v| v.as_u64());

                let usage = input_tokens.map(|pt| crate::types::Usage {
                    prompt_tokens: pt,
                    completion_tokens: 0,
                    total_tokens: pt,
                });

                Ok(Some(ChatCompletionChunk {
                    id,
                    object: "chat.completion.chunk".to_owned(),
                    created: 0,
                    model,
                    choices: vec![StreamChoice {
                        index: 0,
                        delta: StreamDelta {
                            role: Some("assistant".to_owned()),
                            content: None,
                            tool_calls: None,
                            function_call: None,
                            refusal: None,
                        },
                        finish_reason: None,
                    }],
                    usage,
                    system_fingerprint: None,
                    service_tier: None,
                }))
            }

            "content_block_start" => {
                // For tool_use blocks, emit the tool_call header (id + name, empty arguments).
                let block = &event["content_block"];
                let block_type = block.get("type").and_then(|t| t.as_str()).unwrap_or("");
                let index = event.get("index").and_then(|v| v.as_u64()).unwrap_or(0) as u32;

                if block_type == "tool_use" {
                    let tool_id = block.get("id").and_then(|v| v.as_str()).unwrap_or("").to_owned();
                    let tool_name = block.get("name").and_then(|v| v.as_str()).unwrap_or("").to_owned();

                    return Ok(Some(make_empty_chunk_with_tool_start(index, tool_id, tool_name)));
                }

                // Text block start — emit an empty delta so callers can track state.
                Ok(Some(make_empty_chunk("", "")))
            }

            "content_block_delta" => {
                let delta = &event["delta"];
                let delta_type = delta.get("type").and_then(|t| t.as_str()).unwrap_or("");
                let index = event.get("index").and_then(|v| v.as_u64()).unwrap_or(0) as u32;

                match delta_type {
                    "text_delta" => {
                        let text = delta.get("text").and_then(|t| t.as_str()).unwrap_or("");
                        Ok(Some(make_text_chunk("", "", text)))
                    }
                    "input_json_delta" => {
                        // Partial JSON for tool input — emit as tool_call arguments delta.
                        let partial_json = delta.get("partial_json").and_then(|v| v.as_str()).unwrap_or("");
                        Ok(Some(make_tool_arguments_delta(index, partial_json)))
                    }
                    _ => Ok(Some(make_empty_chunk("", ""))),
                }
            }

            "message_delta" => {
                // Final chunk: carries stop_reason and output token count.
                let stop_reason = event.pointer("/delta/stop_reason").and_then(|v| v.as_str());
                let finish_reason = stop_reason.map(map_stop_reason);
                let output_tokens = event.pointer("/usage/output_tokens").and_then(|v| v.as_u64());

                let finish = finish_reason.map(|fr| match fr {
                    "stop" => FinishReason::Stop,
                    "length" => FinishReason::Length,
                    "tool_calls" => FinishReason::ToolCalls,
                    _ => FinishReason::Other,
                });

                let usage = output_tokens.map(|ct| crate::types::Usage {
                    prompt_tokens: 0,
                    completion_tokens: ct,
                    total_tokens: ct,
                });

                Ok(Some(ChatCompletionChunk {
                    id: String::new(),
                    object: "chat.completion.chunk".to_owned(),
                    created: 0,
                    model: String::new(),
                    choices: vec![StreamChoice {
                        index: 0,
                        delta: StreamDelta {
                            role: None,
                            content: None,
                            tool_calls: None,
                            function_call: None,
                            refusal: None,
                        },
                        finish_reason: finish,
                    }],
                    usage,
                    system_fingerprint: None,
                    service_tier: None,
                }))
            }

            "message_stop" => Ok(None),

            "content_block_stop" | "ping" => {
                // These events carry no delta content; return an empty chunk.
                Ok(Some(make_empty_chunk("", "")))
            }

            "error" => {
                let message = event
                    .pointer("/error/message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown Anthropic streaming error");
                Err(LiterLmError::Streaming {
                    message: message.to_owned(),
                })
            }

            _ => {
                // Unknown event types are silently skipped.
                Ok(Some(make_empty_chunk("", "")))
            }
        }
    }
}

// ── Helper functions ──────────────────────────────────────────────────────────

/// Convert an OpenAI-format message JSON value to Anthropic Messages API format.
fn convert_message_to_anthropic(msg: Value) -> Value {
    let role = msg.get("role").and_then(|r| r.as_str()).unwrap_or("").to_owned();

    match role.as_str() {
        "user" => {
            let content = convert_user_content_to_anthropic(msg.get("content"));
            json!({"role": "user", "content": content})
        }
        "assistant" => {
            let mut blocks: Vec<Value> = Vec::new();

            // Text content block.
            if let Some(text) = msg.get("content").and_then(|c| c.as_str())
                && !text.is_empty()
            {
                blocks.push(json!({"type": "text", "text": text}));
            }

            // Tool call blocks.
            if let Some(tool_calls) = msg.get("tool_calls").and_then(|tc| tc.as_array()) {
                for tc in tool_calls {
                    let id = tc.get("id").and_then(|v| v.as_str()).unwrap_or("");
                    let name = tc.pointer("/function/name").and_then(|v| v.as_str()).unwrap_or("");
                    let arguments_str = tc
                        .pointer("/function/arguments")
                        .and_then(|v| v.as_str())
                        .unwrap_or("{}");
                    let input: Value = serde_json::from_str(arguments_str).unwrap_or_else(|_| json!({}));
                    blocks.push(json!({
                        "type": "tool_use",
                        "id": id,
                        "name": name,
                        "input": input
                    }));
                }
            }

            // If no blocks were produced, emit a single empty text block to satisfy Anthropic.
            if blocks.is_empty() {
                blocks.push(json!({"type": "text", "text": ""}));
            }

            json!({"role": "assistant", "content": blocks})
        }
        "tool" => {
            // OpenAI tool message → Anthropic user message with tool_result block.
            let tool_call_id = msg.get("tool_call_id").and_then(|v| v.as_str()).unwrap_or("");
            let content_text = msg.get("content").and_then(|c| c.as_str()).unwrap_or("");
            json!({
                "role": "user",
                "content": [{
                    "type": "tool_result",
                    "tool_use_id": tool_call_id,
                    "content": [{"type": "text", "text": content_text}]
                }]
            })
        }
        "function" => {
            // Deprecated function-role message — treat as a tool result.
            let name = msg.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let content_text = msg.get("content").and_then(|c| c.as_str()).unwrap_or("");
            json!({
                "role": "user",
                "content": [{
                    "type": "tool_result",
                    "tool_use_id": name,
                    "content": [{"type": "text", "text": content_text}]
                }]
            })
        }
        _ => {
            // Unknown role — pass through as-is.
            msg
        }
    }
}

/// Convert OpenAI user content (string or content-part array) to Anthropic content blocks.
fn convert_user_content_to_anthropic(content: Option<&Value>) -> Value {
    match content {
        None => json!([]),
        Some(Value::String(s)) => json!([{"type": "text", "text": s}]),
        Some(Value::Array(parts)) => {
            let blocks: Vec<Value> = parts
                .iter()
                .filter_map(|part| {
                    let part_type = part.get("type").and_then(|t| t.as_str())?;
                    match part_type {
                        "text" => {
                            let text = part.get("text").and_then(|t| t.as_str()).unwrap_or("");
                            Some(json!({"type": "text", "text": text}))
                        }
                        "image_url" => {
                            // Convert data-URI or plain URL to Anthropic image source.
                            let url = part.pointer("/image_url/url").and_then(|u| u.as_str())?;
                            if url.starts_with("data:") {
                                // data:<media_type>;base64,<data>
                                if let Some((header, data)) = url.split_once(',') {
                                    let media_type = header.trim_start_matches("data:").trim_end_matches(";base64");
                                    return Some(json!({
                                        "type": "image",
                                        "source": {
                                            "type": "base64",
                                            "media_type": media_type,
                                            "data": data
                                        }
                                    }));
                                }
                            }
                            // Plain URL — use Anthropic's url source type.
                            Some(json!({
                                "type": "image",
                                "source": {
                                    "type": "url",
                                    "url": url
                                }
                            }))
                        }
                        _ => None,
                    }
                })
                .collect();
            json!(blocks)
        }
        Some(other) => json!([{"type": "text", "text": other.to_string()}]),
    }
}

/// Map an OpenAI `tool_choice` value to Anthropic format.
///
/// Returns `None` when the tool_choice means "none" (tools should be removed entirely).
fn convert_tool_choice(tool_choice: &Value) -> Option<Value> {
    match tool_choice {
        Value::String(s) => match s.as_str() {
            "none" => None,
            "required" => Some(json!({"type": "any"})),
            _ => Some(json!({"type": "auto"})),
        },
        Value::Object(_) => {
            // {"type": "function", "function": {"name": "X"}} → {"type": "tool", "name": "X"}
            let name = tool_choice.pointer("/function/name").and_then(|v| v.as_str());
            if let Some(name) = name {
                Some(json!({"type": "tool", "name": name}))
            } else {
                Some(json!({"type": "auto"}))
            }
        }
        _ => Some(json!({"type": "auto"})),
    }
}

/// Convert an OpenAI tool definition to Anthropic format.
///
/// OpenAI: `{"type": "function", "function": {"name": "X", "description": "Y", "parameters": Z}}`
/// Anthropic: `{"name": "X", "description": "Y", "input_schema": Z}`
fn convert_tool_to_anthropic(tool: &Value) -> Value {
    let function = tool.get("function");
    let name = function.and_then(|f| f.get("name")).cloned().unwrap_or(json!(""));
    let description = function.and_then(|f| f.get("description")).cloned();
    let parameters = function
        .and_then(|f| f.get("parameters"))
        .cloned()
        .unwrap_or(json!({"type": "object", "properties": {}}));

    let mut tool_def = json!({
        "name": name,
        "input_schema": parameters
    });

    if let Some(desc) = description {
        tool_def["description"] = desc;
    }

    tool_def
}

/// Map an Anthropic `stop_reason` string to an OpenAI `finish_reason` string.
fn map_stop_reason(stop_reason: &str) -> &'static str {
    match stop_reason {
        "end_turn" | "stop_sequence" => "stop",
        "tool_use" => "tool_calls",
        "max_tokens" => "length",
        _ => "stop",
    }
}

/// Build a `ChatCompletionChunk` with an empty content delta.
fn make_empty_chunk(id: &str, model: &str) -> ChatCompletionChunk {
    ChatCompletionChunk {
        id: id.to_owned(),
        object: "chat.completion.chunk".to_owned(),
        created: 0,
        model: model.to_owned(),
        choices: vec![StreamChoice {
            index: 0,
            delta: StreamDelta {
                role: None,
                content: None,
                tool_calls: None,
                function_call: None,
                refusal: None,
            },
            finish_reason: None,
        }],
        usage: None,
        system_fingerprint: None,
        service_tier: None,
    }
}

/// Build a `ChatCompletionChunk` with a text content delta.
fn make_text_chunk(id: &str, model: &str, text: &str) -> ChatCompletionChunk {
    ChatCompletionChunk {
        id: id.to_owned(),
        object: "chat.completion.chunk".to_owned(),
        created: 0,
        model: model.to_owned(),
        choices: vec![StreamChoice {
            index: 0,
            delta: StreamDelta {
                role: None,
                content: Some(text.to_owned()),
                tool_calls: None,
                function_call: None,
                refusal: None,
            },
            finish_reason: None,
        }],
        usage: None,
        system_fingerprint: None,
        service_tier: None,
    }
}

/// Build a `ChatCompletionChunk` that starts a tool call (id + name, no arguments yet).
fn make_empty_chunk_with_tool_start(tool_index: u32, tool_id: String, tool_name: String) -> ChatCompletionChunk {
    ChatCompletionChunk {
        id: String::new(),
        object: "chat.completion.chunk".to_owned(),
        created: 0,
        model: String::new(),
        choices: vec![StreamChoice {
            index: 0,
            delta: StreamDelta {
                role: None,
                content: None,
                tool_calls: Some(vec![StreamToolCall {
                    index: tool_index,
                    id: Some(tool_id),
                    call_type: Some(crate::types::ToolType::Function),
                    function: Some(StreamFunctionCall {
                        name: Some(tool_name),
                        arguments: None,
                    }),
                }]),
                function_call: None,
                refusal: None,
            },
            finish_reason: None,
        }],
        usage: None,
        system_fingerprint: None,
        service_tier: None,
    }
}

/// Build a `ChatCompletionChunk` that carries a partial tool arguments JSON delta.
fn make_tool_arguments_delta(tool_index: u32, partial_json: &str) -> ChatCompletionChunk {
    ChatCompletionChunk {
        id: String::new(),
        object: "chat.completion.chunk".to_owned(),
        created: 0,
        model: String::new(),
        choices: vec![StreamChoice {
            index: 0,
            delta: StreamDelta {
                role: None,
                content: None,
                tool_calls: Some(vec![StreamToolCall {
                    index: tool_index,
                    id: None,
                    call_type: None,
                    function: Some(StreamFunctionCall {
                        name: None,
                        arguments: Some(partial_json.to_owned()),
                    }),
                }]),
                function_call: None,
                refusal: None,
            },
            finish_reason: None,
        }],
        usage: None,
        system_fingerprint: None,
        service_tier: None,
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn provider() -> AnthropicProvider {
        AnthropicProvider
    }

    // ── transform_request tests ───────────────────────────────────────────────

    #[test]
    fn transform_request_extracts_system_message() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [
                {"role": "system", "content": "You are a helpful assistant."},
                {"role": "user", "content": "Hello!"}
            ]
        });

        provider().transform_request(&mut body).unwrap();

        // System messages lifted to top-level `system` field.
        assert_eq!(
            body["system"],
            json!([{"type": "text", "text": "You are a helpful assistant."}])
        );

        // Only the user message remains in `messages`.
        let messages = body["messages"].as_array().unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0]["role"], "user");
    }

    #[test]
    fn transform_request_multiple_system_messages_merged() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [
                {"role": "system", "content": "First instruction."},
                {"role": "system", "content": "Second instruction."},
                {"role": "user", "content": "Question"}
            ]
        });

        provider().transform_request(&mut body).unwrap();

        let system = body["system"].as_array().unwrap();
        assert_eq!(system.len(), 2);
        assert_eq!(system[0]["text"], "First instruction.");
        assert_eq!(system[1]["text"], "Second instruction.");
    }

    #[test]
    fn transform_request_defaults_max_tokens() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [{"role": "user", "content": "Hi"}]
        });

        provider().transform_request(&mut body).unwrap();

        assert_eq!(body["max_tokens"], json!(DEFAULT_MAX_TOKENS));
    }

    #[test]
    fn transform_request_preserves_explicit_max_tokens() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [{"role": "user", "content": "Hi"}],
            "max_tokens": 1024
        });

        provider().transform_request(&mut body).unwrap();

        assert_eq!(body["max_tokens"], json!(1024u64));
    }

    #[test]
    fn transform_request_converts_stop_string_to_array() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [{"role": "user", "content": "Hi"}],
            "stop": "\n"
        });

        provider().transform_request(&mut body).unwrap();

        assert_eq!(body["stop_sequences"], json!(["\n"]));
        assert!(body.get("stop").is_none(), "old `stop` key should be removed");
    }

    #[test]
    fn transform_request_stop_array_passes_through() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [{"role": "user", "content": "Hi"}],
            "stop": ["STOP", "END"]
        });

        provider().transform_request(&mut body).unwrap();

        assert_eq!(body["stop_sequences"], json!(["STOP", "END"]));
        assert!(body.get("stop").is_none());
    }

    #[test]
    fn transform_request_tool_choice_required_maps_to_any() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [{"role": "user", "content": "Hi"}],
            "tool_choice": "required",
            "tools": [{"type": "function", "function": {"name": "f", "parameters": {}}}]
        });

        provider().transform_request(&mut body).unwrap();

        assert_eq!(body["tool_choice"], json!({"type": "any"}));
    }

    #[test]
    fn transform_request_tool_choice_none_removes_tools() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [{"role": "user", "content": "Hi"}],
            "tool_choice": "none",
            "tools": [{"type": "function", "function": {"name": "f", "parameters": {}}}]
        });

        provider().transform_request(&mut body).unwrap();

        assert!(body.get("tool_choice").is_none(), "tool_choice should be removed");
        assert!(
            body.get("tools").is_none(),
            "tools should be removed for tool_choice=none"
        );
    }

    #[test]
    fn transform_request_tool_choice_specific_function() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [{"role": "user", "content": "Hi"}],
            "tool_choice": {"type": "function", "function": {"name": "my_tool"}},
            "tools": [{"type": "function", "function": {"name": "my_tool", "parameters": {}}}]
        });

        provider().transform_request(&mut body).unwrap();

        assert_eq!(body["tool_choice"], json!({"type": "tool", "name": "my_tool"}));
    }

    #[test]
    fn transform_request_converts_tools_to_anthropic_format() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [{"role": "user", "content": "Hi"}],
            "tools": [{
                "type": "function",
                "function": {
                    "name": "get_weather",
                    "description": "Get current weather",
                    "parameters": {"type": "object", "properties": {}}
                }
            }]
        });

        provider().transform_request(&mut body).unwrap();

        let tools = body["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0]["name"], "get_weather");
        assert_eq!(tools[0]["description"], "Get current weather");
        assert!(tools[0].get("input_schema").is_some());
        // OpenAI-style "function" wrapper should be gone.
        assert!(tools[0].get("function").is_none());
    }

    #[test]
    fn transform_request_removes_unsupported_fields() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [{"role": "user", "content": "Hi"}],
            "n": 2,
            "presence_penalty": 0.5,
            "frequency_penalty": 0.3,
            "logit_bias": {"1234": 5},
            "stream": true
        });

        provider().transform_request(&mut body).unwrap();

        for key in &["n", "presence_penalty", "frequency_penalty", "logit_bias", "stream"] {
            assert!(body.get(key).is_none(), "`{key}` should be removed");
        }
    }

    #[test]
    fn transform_request_converts_tool_message_to_tool_result() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [
                {"role": "user", "content": "What is the weather?"},
                {"role": "assistant", "content": null, "tool_calls": [{
                    "id": "call_abc",
                    "type": "function",
                    "function": {"name": "get_weather", "arguments": "{\"location\": \"London\"}"}
                }]},
                {"role": "tool", "tool_call_id": "call_abc", "content": "15°C, sunny"}
            ]
        });

        provider().transform_request(&mut body).unwrap();

        let messages = body["messages"].as_array().unwrap();
        // tool message → user message with tool_result block
        let tool_result_msg = &messages[2];
        assert_eq!(tool_result_msg["role"], "user");
        let content = tool_result_msg["content"].as_array().unwrap();
        assert_eq!(content[0]["type"], "tool_result");
        assert_eq!(content[0]["tool_use_id"], "call_abc");
    }

    #[test]
    fn transform_request_converts_user_content_parts() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [{
                "role": "user",
                "content": [
                    {"type": "text", "text": "What is in this image?"},
                    {"type": "image_url", "image_url": {"url": "data:image/jpeg;base64,/9j/abc=="}}
                ]
            }]
        });

        provider().transform_request(&mut body).unwrap();

        let messages = body["messages"].as_array().unwrap();
        let content = messages[0]["content"].as_array().unwrap();
        assert_eq!(content[0]["type"], "text");
        assert_eq!(content[1]["type"], "image");
        assert_eq!(content[1]["source"]["type"], "base64");
        assert_eq!(content[1]["source"]["media_type"], "image/jpeg");
    }

    // ── transform_response tests ──────────────────────────────────────────────

    #[test]
    fn transform_response_basic_text() {
        let mut body = json!({
            "id": "msg_01Xfn7",
            "type": "message",
            "role": "assistant",
            "content": [{"type": "text", "text": "Hello, world!"}],
            "model": "claude-3-5-sonnet-20241022",
            "stop_reason": "end_turn",
            "usage": {"input_tokens": 10, "output_tokens": 5}
        });

        provider().transform_response(&mut body).unwrap();

        assert_eq!(body["object"], "chat.completion");
        assert_eq!(body["id"], "msg_01Xfn7");
        let choice = &body["choices"][0];
        assert_eq!(choice["message"]["content"], "Hello, world!");
        assert_eq!(choice["finish_reason"], "stop");
        assert_eq!(body["usage"]["prompt_tokens"], 10);
        assert_eq!(body["usage"]["completion_tokens"], 5);
        assert_eq!(body["usage"]["total_tokens"], 15);
    }

    #[test]
    fn transform_response_stop_reason_max_tokens_maps_to_length() {
        let mut body = json!({
            "id": "msg_abc",
            "type": "message",
            "role": "assistant",
            "content": [{"type": "text", "text": "truncated"}],
            "model": "claude-3-haiku-20240307",
            "stop_reason": "max_tokens",
            "usage": {"input_tokens": 5, "output_tokens": 50}
        });

        provider().transform_response(&mut body).unwrap();

        assert_eq!(body["choices"][0]["finish_reason"], "length");
    }

    #[test]
    fn transform_response_tool_use_block() {
        let mut body = json!({
            "id": "msg_tool",
            "type": "message",
            "role": "assistant",
            "content": [{
                "type": "tool_use",
                "id": "toolu_01abc",
                "name": "get_weather",
                "input": {"location": "London"}
            }],
            "model": "claude-3-5-sonnet-20241022",
            "stop_reason": "tool_use",
            "usage": {"input_tokens": 20, "output_tokens": 10}
        });

        provider().transform_response(&mut body).unwrap();

        let choice = &body["choices"][0];
        assert_eq!(choice["finish_reason"], "tool_calls");
        assert_eq!(choice["message"]["content"], Value::Null);

        let tool_calls = choice["message"]["tool_calls"].as_array().unwrap();
        assert_eq!(tool_calls.len(), 1);
        assert_eq!(tool_calls[0]["id"], "toolu_01abc");
        assert_eq!(tool_calls[0]["function"]["name"], "get_weather");

        // arguments must be a JSON string
        let args_str = tool_calls[0]["function"]["arguments"].as_str().unwrap();
        let args: Value = serde_json::from_str(args_str).unwrap();
        assert_eq!(args["location"], "London");
    }

    #[test]
    fn transform_response_is_noop_for_openai_format() {
        // A body without "stop_reason" should be left unchanged (already OpenAI format).
        let original = json!({
            "id": "chatcmpl-xxx",
            "object": "chat.completion",
            "choices": [{"index": 0, "message": {"role": "assistant", "content": "hi"}, "finish_reason": "stop"}]
        });
        let mut body = original.clone();

        provider().transform_response(&mut body).unwrap();

        assert_eq!(body, original);
    }

    // ── parse_stream_event tests ──────────────────────────────────────────────

    #[test]
    fn parse_stream_event_done_returns_none() {
        let result = provider().parse_stream_event("[DONE]").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn parse_stream_event_message_stop_returns_none() {
        let event = r#"{"type":"message_stop"}"#;
        let result = provider().parse_stream_event(event).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn parse_stream_event_text_delta() {
        let event = r#"{"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":"Hello"}}"#;
        let chunk = provider().parse_stream_event(event).unwrap().expect("expected chunk");
        assert_eq!(chunk.choices[0].delta.content.as_deref(), Some("Hello"));
    }

    #[test]
    fn parse_stream_event_message_delta_with_finish_reason() {
        let event = r#"{"type":"message_delta","delta":{"stop_reason":"end_turn","stop_sequence":null},"usage":{"output_tokens":12}}"#;
        let chunk = provider().parse_stream_event(event).unwrap().expect("expected chunk");
        assert_eq!(chunk.choices[0].finish_reason, Some(FinishReason::Stop));
        let usage = chunk.usage.unwrap();
        assert_eq!(usage.completion_tokens, 12);
    }

    #[test]
    fn parse_stream_event_message_delta_tool_use_stop_reason() {
        let event = r#"{"type":"message_delta","delta":{"stop_reason":"tool_use"},"usage":{"output_tokens":5}}"#;
        let chunk = provider().parse_stream_event(event).unwrap().expect("expected chunk");
        assert_eq!(chunk.choices[0].finish_reason, Some(FinishReason::ToolCalls));
    }

    #[test]
    fn parse_stream_event_message_start() {
        let event = r#"{"type":"message_start","message":{"id":"msg_abc","type":"message","role":"assistant","content":[],"model":"claude-3-5-sonnet-20241022","stop_reason":null,"usage":{"input_tokens":25,"output_tokens":1}}}"#;
        let chunk = provider().parse_stream_event(event).unwrap().expect("expected chunk");
        assert_eq!(chunk.id, "msg_abc");
        assert_eq!(chunk.model, "claude-3-5-sonnet-20241022");
        assert_eq!(chunk.choices[0].delta.role.as_deref(), Some("assistant"));
        let usage = chunk.usage.unwrap();
        assert_eq!(usage.prompt_tokens, 25);
    }

    #[test]
    fn parse_stream_event_input_json_delta() {
        let event =
            r#"{"type":"content_block_delta","index":0,"delta":{"type":"input_json_delta","partial_json":"{\"loc"}}"#;
        let chunk = provider().parse_stream_event(event).unwrap().expect("expected chunk");
        let tc = &chunk.choices[0].delta.tool_calls.as_ref().unwrap()[0];
        assert_eq!(tc.function.as_ref().unwrap().arguments.as_deref(), Some("{\"loc"));
    }

    #[test]
    fn parse_stream_event_error_returns_err() {
        let event = r#"{"type":"error","error":{"type":"overloaded_error","message":"Overloaded"}}"#;
        let result = provider().parse_stream_event(event);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Overloaded"));
    }

    #[test]
    fn parse_stream_event_ping_returns_empty_chunk() {
        let event = r#"{"type":"ping"}"#;
        let chunk = provider().parse_stream_event(event).unwrap();
        assert!(chunk.is_some());
        let chunk = chunk.unwrap();
        assert!(chunk.choices[0].delta.content.is_none());
    }

    // ── chat_completions_path test ────────────────────────────────────────────

    #[test]
    fn chat_completions_path_is_messages() {
        assert_eq!(provider().chat_completions_path(), "/messages");
    }
}
