use std::borrow::Cow;

use serde_json::{Value, json};

use crate::error::{LiterLmError, Result};
use crate::provider::Provider;
use crate::types::{ChatCompletionChunk, FinishReason, StreamChoice, StreamDelta, StreamFunctionCall, StreamToolCall};

/// Anthropic's stable API version. This is the only GA version as of 2025;
/// Anthropic gates new features via beta headers (e.g. `anthropic-beta`),
/// not by bumping the version date.
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
    /// - Consecutive same-role messages are merged (Anthropic requires alternating roles).
    /// - `max_tokens` defaults to 4096 if not set (Anthropic requires it).
    /// - `stop` renamed to `stop_sequences` and normalised to an array.
    /// - `tool_choice` mapped from OpenAI semantics to Anthropic semantics.
    /// - Tools converted from OpenAI `function` wrappers to Anthropic `input_schema` format.
    /// - Unsupported parameters removed: `n`, `presence_penalty`, `frequency_penalty`,
    ///   `logit_bias`, `stream` (the client handles stream separately).
    fn transform_request(&self, body: &mut Value) -> Result<()> {
        // ── 0. Validate messages array is non-empty ────────────────────────────
        // Take ownership of the messages array to avoid cloning.
        let messages = body
            .as_object_mut()
            .and_then(|o| o.remove("messages"))
            .and_then(|v| match v {
                Value::Array(arr) => Some(arr),
                _ => None,
            })
            .unwrap_or_default();

        if messages.is_empty() {
            return Err(LiterLmError::BadRequest {
                message: "messages array must not be empty".to_owned(),
            });
        }

        // ── 1. Extract system messages ────────────────────────────────────────
        let mut system_blocks: Vec<Value> = Vec::new();
        let mut non_system_messages: Vec<Value> = Vec::new();

        for msg in messages {
            let role = msg.get("role").and_then(|r| r.as_str()).unwrap_or("");
            match role {
                "system" | "developer" => {
                    // Both system and developer roles map to Anthropic system content.
                    // Content may be a string or a content-block array.
                    match msg.get("content") {
                        Some(Value::String(s)) if !s.is_empty() => {
                            let mut block = json!({"type": "text", "text": s});
                            // Propagate cache_control if present.
                            if let Some(cc) = msg.get("cache_control") {
                                block["cache_control"] = cc.clone();
                            }
                            system_blocks.push(block);
                        }
                        Some(Value::Array(parts)) => {
                            for part in parts {
                                system_blocks.push(part.clone());
                            }
                        }
                        _ => {}
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

        // ── 3. Merge consecutive same-role messages ───────────────────────────
        // Anthropic requires strictly alternating user/assistant roles.
        let merged_messages = merge_consecutive_same_role(converted_messages);

        body["messages"] = json!(merged_messages);

        // ── 4. Ensure max_tokens is present (required by Anthropic) ───────────
        // Also handle the OpenAI alias max_completion_tokens.
        if body.get("max_tokens").is_none() {
            if let Some(mct) = body.get("max_completion_tokens").cloned() {
                body["max_tokens"] = mct;
            } else {
                body["max_tokens"] = json!(DEFAULT_MAX_TOKENS);
            }
        }
        body.as_object_mut().map(|o| o.remove("max_completion_tokens"));

        // ── 5. Convert stop → stop_sequences (must be array) ─────────────────
        if let Some(stop) = body.as_object_mut().and_then(|o| o.remove("stop")) {
            let stop_sequences = match stop {
                Value::String(s) => json!([s]),
                arr @ Value::Array(_) => arr,
                _ => json!([]),
            };
            body["stop_sequences"] = stop_sequences;
        }

        // ── 6. Convert tool_choice ─────────────────────────────────────────────
        if let Some(tool_choice) = body.as_object_mut().and_then(|o| o.remove("tool_choice")) {
            let anthropic_tool_choice = convert_tool_choice(&tool_choice);
            match anthropic_tool_choice {
                Some(tc) => {
                    body["tool_choice"] = tc;
                }
                None => {
                    // tool_choice: "none" → remove tools entirely
                    body.as_object_mut().map(|o| o.remove("tools"));
                }
            }
        }

        // ── 7. Convert tools from OpenAI format to Anthropic format ───────────
        if let Some(tools) = body.as_object_mut().and_then(|o| o.remove("tools"))
            && let Some(tools_array) = tools.as_array()
        {
            let anthropic_tools: Vec<Value> = tools_array.iter().map(convert_tool_to_anthropic).collect();
            body["tools"] = json!(anthropic_tools);
        }

        // ── 8. Remove unsupported parameters ──────────────────────────────────
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

        // Extract text content from `type: "text"` blocks only.
        // Thinking blocks (`type: "thinking"`) are Anthropic's internal chain-of-thought
        // and are intentionally excluded from the user-facing content field.
        let text_content: Option<String> = content_blocks.as_ref().map(|blocks| {
            blocks
                .iter()
                .filter(|b| b.get("type").and_then(|t| t.as_str()) == Some("text"))
                .filter_map(|b| b.get("text").and_then(|t| t.as_str()))
                .collect::<Vec<_>>()
                .join("")
        });

        // Extract tool_use blocks into OpenAI-format tool_calls.
        // Treat both "tool_use" and "server_tool_use" the same way.
        let tool_calls: Option<Vec<Value>> = content_blocks.as_ref().map(|blocks| {
            blocks
                .iter()
                .filter(|b| {
                    matches!(
                        b.get("type").and_then(|t| t.as_str()),
                        Some("tool_use") | Some("server_tool_use")
                    )
                })
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
        // Cache tokens count towards prompt tokens for billing purposes.
        let input_tokens = body
            .pointer("/usage/input_tokens")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let cache_creation_tokens = body
            .pointer("/usage/cache_creation_input_tokens")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let cache_read_tokens = body
            .pointer("/usage/cache_read_input_tokens")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let output_tokens = body
            .pointer("/usage/output_tokens")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let prompt_tokens = input_tokens + cache_creation_tokens + cache_read_tokens;

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
            "created": super::unix_timestamp_secs(),
            "model": model,
            "choices": [{
                "index": 0,
                "message": message,
                "finish_reason": finish_reason
            }],
            "usage": {
                "prompt_tokens": prompt_tokens,
                "completion_tokens": output_tokens,
                "total_tokens": prompt_tokens + output_tokens
            }
        });

        Ok(())
    }

    /// Parse an Anthropic SSE event into an OpenAI-compatible `ChatCompletionChunk`.
    ///
    /// Anthropic event types handled:
    /// - `message_start`: emits a role-only delta chunk.
    /// - `content_block_start`: emits empty delta (tool_use: emits tool_call header chunk).
    /// - `content_block_delta`: emits text, thinking, or tool input JSON delta.
    /// - `message_delta`: emits final chunk with finish_reason and usage.
    /// - `message_stop`: signals end of stream, returns `Ok(None)`.
    /// - `content_block_stop`, `ping`: skipped (returns `Ok(None)` — no content to emit).
    /// - `error`: returns `Err(LiterLmError::Streaming)`.
    fn parse_stream_event(&self, event_data: &str) -> Result<Option<ChatCompletionChunk>> {
        // NOTE: `[DONE]` is handled at the SSE parser level; no check needed here.

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
                // Also account for cache tokens in the prompt count.
                let input_tokens = msg.pointer("/usage/input_tokens").and_then(|v| v.as_u64()).unwrap_or(0);
                let cache_creation = msg
                    .pointer("/usage/cache_creation_input_tokens")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);
                let cache_read = msg
                    .pointer("/usage/cache_read_input_tokens")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);
                let prompt_tokens = input_tokens + cache_creation + cache_read;

                let usage = if prompt_tokens > 0 {
                    Some(crate::types::Usage {
                        prompt_tokens,
                        completion_tokens: 0,
                        total_tokens: prompt_tokens,
                    })
                } else {
                    None
                };

                Ok(Some(ChatCompletionChunk {
                    id,
                    object: "chat.completion.chunk".to_owned(),
                    created: super::unix_timestamp_secs(),
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
                // NOTE: We use Anthropic's block index directly, which counts ALL content blocks
                // (text + thinking + tool_use). This differs from OpenAI's sequential tool-call
                // indices (0, 1, 2...) but is safe because:
                // 1. The same index is used in both content_block_start and content_block_delta,
                //    so they are consistent with each other.
                // 2. OpenAI clients typically correlate tool calls by `id`, not by index.
                // 3. OpenAI's own streaming format allows index gaps in tool_calls[].
                // Known limitation: indices may have gaps when text/thinking blocks precede tool_use.
                let anthropic_index = event.get("index").and_then(|v| v.as_u64()).unwrap_or(0) as u32;

                if block_type == "tool_use" || block_type == "server_tool_use" {
                    let tool_id = block.get("id").and_then(|v| v.as_str()).unwrap_or("").to_owned();
                    let tool_name = block.get("name").and_then(|v| v.as_str()).unwrap_or("").to_owned();

                    return Ok(Some(make_empty_chunk_with_tool_start(
                        anthropic_index,
                        tool_id,
                        tool_name,
                    )));
                }

                // Text / thinking block start — emit Ok(None) since there is no content yet.
                Ok(None)
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
                    "thinking_delta" => {
                        // Extended thinking blocks are Anthropic's internal chain-of-thought.
                        // Non-streaming already filters thinking blocks from content, so
                        // streaming must be consistent: skip them entirely.
                        Ok(None)
                    }
                    "input_json_delta" => {
                        // Partial JSON for tool input — emit as tool_call arguments delta.
                        let partial_json = delta.get("partial_json").and_then(|v| v.as_str()).unwrap_or("");
                        Ok(Some(make_tool_arguments_delta(index, partial_json)))
                    }
                    _ => Ok(None),
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
                    created: super::unix_timestamp_secs(),
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
                // These events carry no delta content; return Ok(None).
                Ok(None)
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
                Ok(None)
            }
        }
    }
}

// ── Helper functions ──────────────────────────────────────────────────────────

/// Sanitize a tool_call_id so it only contains characters allowed by Anthropic: `[a-zA-Z0-9_-]`.
/// Any other character is replaced with `_`.
///
/// Convert an OpenAI `image_url` URL to an Anthropic image source block.
///
/// Handles two cases:
/// - Data URIs (`data:<media_type>;base64,<data>`) → base64 source.
/// - Plain URLs → url source.
fn convert_image_url_to_anthropic_source(url: &str) -> Value {
    if url.starts_with("data:")
        && let Some((header, data)) = url.split_once(',')
    {
        let media_type = header.trim_start_matches("data:").trim_end_matches(";base64");
        return json!({
            "type": "image",
            "source": {
                "type": "base64",
                "media_type": media_type,
                "data": data
            }
        });
    }
    json!({
        "type": "image",
        "source": {"type": "url", "url": url}
    })
}

/// Returns a borrowed `Cow` when the ID is already valid, avoiding allocation
/// on the common path (e.g. IDs starting with `toolu_`).
fn sanitize_tool_call_id(id: &str) -> Cow<'_, str> {
    if id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-') {
        Cow::Borrowed(id)
    } else {
        Cow::Owned(
            id.chars()
                .map(|c| {
                    if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                        c
                    } else {
                        '_'
                    }
                })
                .collect(),
        )
    }
}

/// Merge consecutive messages with the same role by concatenating their content blocks.
/// Anthropic requires strictly alternating user/assistant roles.
fn merge_consecutive_same_role(messages: Vec<Value>) -> Vec<Value> {
    let mut merged: Vec<Value> = Vec::new();

    for msg in messages {
        let role = msg.get("role").and_then(|r| r.as_str()).unwrap_or("");

        if let Some(last) = merged.last_mut() {
            let last_role = last.get("role").and_then(|r| r.as_str()).unwrap_or("");
            if last_role == role {
                // Merge content blocks.
                let incoming_content = match msg.get("content") {
                    Some(Value::Array(arr)) => arr.clone(),
                    Some(Value::String(s)) => vec![json!({"type": "text", "text": s})],
                    Some(other) => vec![json!({"type": "text", "text": other.to_string()})],
                    None => vec![],
                };

                if let Some(Value::Array(existing)) = last.get_mut("content") {
                    existing.extend(incoming_content);
                } else {
                    // Normalize existing content to array first.
                    let existing_content = match last.get("content") {
                        Some(Value::String(s)) => vec![json!({"type": "text", "text": s.clone()})],
                        Some(Value::Array(arr)) => arr.clone(),
                        Some(other) => vec![json!({"type": "text", "text": other.to_string()})],
                        None => vec![],
                    };
                    let mut combined = existing_content;
                    combined.extend(incoming_content);
                    last["content"] = json!(combined);
                }
                continue;
            }
        }

        merged.push(msg);
    }

    merged
}

/// Convert an OpenAI-format message JSON value to Anthropic Messages API format.
fn convert_message_to_anthropic(msg: Value) -> Value {
    let role = msg.get("role").and_then(|r| r.as_str()).unwrap_or("");

    match role {
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

            // If no blocks were produced AND there are no tool_calls, emit an empty text block.
            // Do NOT inject empty text when tool_use blocks are present — Anthropic rejects it.
            let has_tool_use = blocks
                .iter()
                .any(|b| b.get("type").and_then(|t| t.as_str()) == Some("tool_use"));
            if blocks.is_empty() {
                blocks.push(json!({"type": "text", "text": ""}));
            } else if !has_tool_use {
                // only text blocks: fine as-is
            }
            // When there are tool_use blocks, do not add an empty text block.

            json!({"role": "assistant", "content": blocks})
        }
        "tool" => {
            // OpenAI tool message → Anthropic user message with tool_result block.
            let raw_id = msg.get("tool_call_id").and_then(|v| v.as_str()).unwrap_or("");
            let tool_use_id = sanitize_tool_call_id(raw_id);

            // Content may be a string or an array (e.g. images in tool results).
            let result_content = match msg.get("content") {
                Some(Value::Array(arr)) => {
                    // Pass through content array (may contain image blocks, etc.)
                    arr.iter()
                        .map(|part| {
                            let part_type = part.get("type").and_then(|t| t.as_str()).unwrap_or("text");
                            match part_type {
                                "image_url" => {
                                    let url = part.pointer("/image_url/url").and_then(|u| u.as_str()).unwrap_or("");
                                    convert_image_url_to_anthropic_source(url)
                                }
                                _ => {
                                    let text = part.get("text").and_then(|t| t.as_str()).unwrap_or("");
                                    json!({"type": "text", "text": text})
                                }
                            }
                        })
                        .collect::<Vec<_>>()
                }
                Some(Value::String(s)) => vec![json!({"type": "text", "text": s})],
                _ => vec![json!({"type": "text", "text": ""})],
            };

            let mut tool_result_block = json!({
                "type": "tool_result",
                "tool_use_id": tool_use_id,
                "content": result_content
            });

            // Propagate cache_control if present.
            if let Some(cc) = msg.get("cache_control") {
                tool_result_block["cache_control"] = cc.clone();
            }

            json!({
                "role": "user",
                "content": [tool_result_block]
            })
        }
        "function" => {
            // Deprecated function-role message — treat as a tool result.
            let name = msg.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let sanitized_name = sanitize_tool_call_id(name);
            let content_text = msg.get("content").and_then(|c| c.as_str()).unwrap_or("");
            json!({
                "role": "user",
                "content": [{
                    "type": "tool_result",
                    "tool_use_id": sanitized_name,
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
                            let mut block = json!({"type": "text", "text": text});
                            // Propagate cache_control if present.
                            if let Some(cc) = part.get("cache_control") {
                                block["cache_control"] = cc.clone();
                            }
                            Some(block)
                        }
                        "image_url" => {
                            let url = part.pointer("/image_url/url").and_then(|u| u.as_str())?;
                            Some(convert_image_url_to_anthropic_source(url))
                        }
                        _ => {
                            // Unknown content part types: fall back to text representation
                            // and log a warning so callers can investigate.
                            #[cfg(feature = "tracing")]
                            tracing::warn!(
                                part_type = part_type,
                                "unrecognized user content part type; falling back to text"
                            );
                            let text = part.get("text").and_then(|t| t.as_str()).unwrap_or("");
                            if text.is_empty() {
                                None
                            } else {
                                Some(json!({"type": "text", "text": text}))
                            }
                        }
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
///
/// Also normalises `input_schema.type` to `"object"` if absent or mis-typed.
fn convert_tool_to_anthropic(tool: &Value) -> Value {
    let function = tool.get("function");
    let name = function.and_then(|f| f.get("name")).cloned().unwrap_or(json!(""));
    let description = function.and_then(|f| f.get("description")).cloned();
    let mut parameters = function
        .and_then(|f| f.get("parameters"))
        .cloned()
        .unwrap_or(json!({"type": "object", "properties": {}}));

    // Normalize input_schema.type to "object" — Anthropic rejects other values.
    if parameters.get("type").and_then(|t| t.as_str()) != Some("object") {
        parameters["type"] = json!("object");
    }

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
        "content_filtered" | "refusal" => "content_filter",
        _ => "stop",
    }
}

/// Build a `ChatCompletionChunk` with a text content delta.
fn make_text_chunk(id: &str, model: &str, text: &str) -> ChatCompletionChunk {
    ChatCompletionChunk {
        id: id.to_owned(),
        object: "chat.completion.chunk".to_owned(),
        created: super::unix_timestamp_secs(),
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
        created: super::unix_timestamp_secs(),
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
        created: super::unix_timestamp_secs(),
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
    fn parse_stream_event_done_is_handled_at_sse_level() {
        // `[DONE]` is now caught by the SSE parser before reaching the provider.
        // If it were to reach the provider, it would be invalid JSON.
        let result = provider().parse_stream_event("[DONE]");
        assert!(
            result.is_err(),
            "[DONE] is not valid JSON and should error if it reaches the provider"
        );
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
    fn parse_stream_event_ping_returns_none() {
        let event = r#"{"type":"ping"}"#;
        let result = provider().parse_stream_event(event).unwrap();
        assert!(result.is_none(), "ping should return Ok(None), not a chunk");
    }

    #[test]
    fn parse_stream_event_content_block_stop_returns_none() {
        let event = r#"{"type":"content_block_stop","index":0}"#;
        let result = provider().parse_stream_event(event).unwrap();
        assert!(result.is_none(), "content_block_stop should return Ok(None)");
    }

    // ── chat_completions_path test ────────────────────────────────────────────

    #[test]
    fn chat_completions_path_is_messages() {
        assert_eq!(provider().chat_completions_path(), "/messages");
    }

    // ── new tests for fixed issues ────────────────────────────────────────────

    #[test]
    fn transform_request_empty_messages_returns_error() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": []
        });
        let result = provider().transform_request(&mut body);
        assert!(result.is_err(), "empty messages should return an error");
    }

    #[test]
    fn transform_request_sanitizes_tool_call_id() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [
                {"role": "user", "content": "What is the weather?"},
                {"role": "assistant", "content": null, "tool_calls": [{
                    "id": "call_abc.123",
                    "type": "function",
                    "function": {"name": "get_weather", "arguments": "{}"}
                }]},
                {"role": "tool", "tool_call_id": "call_abc.123", "content": "Sunny"}
            ]
        });
        provider().transform_request(&mut body).unwrap();
        let messages = body["messages"].as_array().unwrap();
        // The tool_result message is the last one (index 2 after merging nothing)
        let tool_result_msg = messages
            .iter()
            .find(|m| m["role"] == "user" && m["content"][0]["type"] == "tool_result")
            .unwrap();
        // Dots should be replaced with underscores.
        assert_eq!(tool_result_msg["content"][0]["tool_use_id"], "call_abc_123");
    }

    #[test]
    fn transform_request_merges_consecutive_user_messages() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [
                {"role": "user", "content": "First"},
                {"role": "user", "content": "Second"}
            ]
        });
        provider().transform_request(&mut body).unwrap();
        let messages = body["messages"].as_array().unwrap();
        // Two consecutive user messages should be merged into one.
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0]["role"], "user");
        let content = messages[0]["content"].as_array().unwrap();
        assert_eq!(content.len(), 2);
    }

    #[test]
    fn transform_request_system_content_array_passed_through() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [
                {"role": "system", "content": [
                    {"type": "text", "text": "Block one"},
                    {"type": "text", "text": "Block two"}
                ]},
                {"role": "user", "content": "Hello"}
            ]
        });
        provider().transform_request(&mut body).unwrap();
        let system = body["system"].as_array().unwrap();
        assert_eq!(system.len(), 2);
        assert_eq!(system[0]["text"], "Block one");
    }

    #[test]
    fn transform_request_system_cache_control_propagated() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [
                {"role": "system", "content": "Cached instructions", "cache_control": {"type": "ephemeral"}},
                {"role": "user", "content": "Hi"}
            ]
        });
        provider().transform_request(&mut body).unwrap();
        let system = body["system"].as_array().unwrap();
        assert_eq!(system[0]["cache_control"]["type"], "ephemeral");
    }

    #[test]
    fn transform_request_user_content_cache_control_propagated() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [{
                "role": "user",
                "content": [
                    {"type": "text", "text": "Cached text", "cache_control": {"type": "ephemeral"}}
                ]
            }]
        });
        provider().transform_request(&mut body).unwrap();
        let messages = body["messages"].as_array().unwrap();
        let content = messages[0]["content"].as_array().unwrap();
        assert_eq!(content[0]["cache_control"]["type"], "ephemeral");
    }

    #[test]
    fn transform_request_tool_input_schema_type_normalized() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [{"role": "user", "content": "Hi"}],
            "tools": [{
                "type": "function",
                "function": {
                    "name": "my_tool",
                    "parameters": {"properties": {}}
                    // no "type" field
                }
            }]
        });
        provider().transform_request(&mut body).unwrap();
        let tools = body["tools"].as_array().unwrap();
        assert_eq!(tools[0]["input_schema"]["type"], "object");
    }

    #[test]
    fn transform_request_max_completion_tokens_mapped() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [{"role": "user", "content": "Hi"}],
            "max_completion_tokens": 512
        });
        provider().transform_request(&mut body).unwrap();
        assert_eq!(body["max_tokens"], json!(512u64));
        assert!(body.get("max_completion_tokens").is_none());
    }

    #[test]
    fn transform_request_tool_result_content_array_preserved() {
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [
                {"role": "user", "content": "Look"},
                {"role": "assistant", "content": null, "tool_calls": [{
                    "id": "call_img",
                    "type": "function",
                    "function": {"name": "get_image", "arguments": "{}"}
                }]},
                {"role": "tool", "tool_call_id": "call_img", "content": [
                    {"type": "text", "text": "Here is the image"},
                    {"type": "image_url", "image_url": {"url": "data:image/png;base64,abc123"}}
                ]}
            ]
        });
        provider().transform_request(&mut body).unwrap();
        let messages = body["messages"].as_array().unwrap();
        let tool_result_msg = messages
            .iter()
            .find(|m| {
                m["role"] == "user"
                    && m["content"]
                        .as_array()
                        .is_some_and(|c| c.first().is_some_and(|b| b["type"] == "tool_result"))
            })
            .unwrap();
        let result_content = tool_result_msg["content"][0]["content"].as_array().unwrap();
        assert_eq!(result_content.len(), 2);
        assert_eq!(result_content[0]["type"], "text");
        assert_eq!(result_content[1]["type"], "image");
    }

    #[test]
    fn transform_response_thinking_block_excluded_from_content() {
        let mut body = json!({
            "id": "msg_think",
            "type": "message",
            "role": "assistant",
            "content": [
                {"type": "thinking", "thinking": "Let me reason..."},
                {"type": "text", "text": "The answer is 42."}
            ],
            "model": "claude-3-5-sonnet-20241022",
            "stop_reason": "end_turn",
            "usage": {"input_tokens": 10, "output_tokens": 20}
        });
        provider().transform_response(&mut body).unwrap();
        let content = body["choices"][0]["message"]["content"].as_str().unwrap();
        // Thinking blocks are internal chain-of-thought and must NOT appear in user-facing content.
        assert!(
            !content.contains("Let me reason..."),
            "thinking blocks should be filtered out"
        );
        assert_eq!(content, "The answer is 42.");
    }

    #[test]
    fn transform_response_server_tool_use_treated_as_tool_call() {
        let mut body = json!({
            "id": "msg_srv",
            "type": "message",
            "role": "assistant",
            "content": [{
                "type": "server_tool_use",
                "id": "srvtool_01",
                "name": "web_search",
                "input": {"query": "Rust programming"}
            }],
            "model": "claude-3-5-sonnet-20241022",
            "stop_reason": "tool_use",
            "usage": {"input_tokens": 5, "output_tokens": 5}
        });
        provider().transform_response(&mut body).unwrap();
        let tool_calls = body["choices"][0]["message"]["tool_calls"].as_array().unwrap();
        assert_eq!(tool_calls.len(), 1);
        assert_eq!(tool_calls[0]["id"], "srvtool_01");
        assert_eq!(tool_calls[0]["function"]["name"], "web_search");
    }

    #[test]
    fn transform_response_cache_tokens_counted_in_prompt() {
        let mut body = json!({
            "id": "msg_cache",
            "type": "message",
            "role": "assistant",
            "content": [{"type": "text", "text": "ok"}],
            "model": "claude-3-5-sonnet-20241022",
            "stop_reason": "end_turn",
            "usage": {
                "input_tokens": 100,
                "cache_creation_input_tokens": 50,
                "cache_read_input_tokens": 25,
                "output_tokens": 10
            }
        });
        provider().transform_response(&mut body).unwrap();
        // prompt_tokens = 100 + 50 + 25 = 175
        assert_eq!(body["usage"]["prompt_tokens"], 175u64);
        assert_eq!(body["usage"]["completion_tokens"], 10u64);
        assert_eq!(body["usage"]["total_tokens"], 185u64);
    }

    #[test]
    fn transform_response_tool_only_no_empty_text_block_in_request() {
        // When assistant has only tool_calls, the converted message should NOT inject empty text.
        let mut body = json!({
            "model": "claude-3-5-sonnet-20241022",
            "messages": [
                {"role": "user", "content": "Call a tool"},
                {"role": "assistant", "tool_calls": [{
                    "id": "call_xyz",
                    "type": "function",
                    "function": {"name": "my_fn", "arguments": "{}"}
                }]},
                {"role": "tool", "tool_call_id": "call_xyz", "content": "result"}
            ]
        });
        provider().transform_request(&mut body).unwrap();
        let messages = body["messages"].as_array().unwrap();
        let assistant_msg = messages.iter().find(|m| m["role"] == "assistant").unwrap();
        let blocks = assistant_msg["content"].as_array().unwrap();
        // Only the tool_use block should be present; no empty text block.
        assert!(blocks.iter().all(|b| b["type"] != "text" || b["text"] != ""));
        assert!(blocks.iter().any(|b| b["type"] == "tool_use"));
    }

    #[test]
    fn parse_stream_event_thinking_delta_returns_none() {
        // Thinking blocks are filtered in both streaming and non-streaming for consistency.
        let event = r#"{"type":"content_block_delta","index":0,"delta":{"type":"thinking_delta","thinking":"I am thinking..."}}"#;
        let result = provider().parse_stream_event(event).unwrap();
        assert!(result.is_none(), "thinking_delta should be filtered (return None)");
    }

    #[test]
    fn parse_stream_event_message_start_cache_tokens_in_usage() {
        let event = r#"{"type":"message_start","message":{"id":"msg_x","model":"claude-opus","content":[],"usage":{"input_tokens":100,"cache_creation_input_tokens":50,"cache_read_input_tokens":25,"output_tokens":0}}}"#;
        let chunk = provider().parse_stream_event(event).unwrap().expect("expected chunk");
        let usage = chunk.usage.unwrap();
        // prompt_tokens = 100 + 50 + 25 = 175
        assert_eq!(usage.prompt_tokens, 175);
    }

    #[test]
    fn sanitize_tool_call_id_replaces_invalid_chars() {
        assert_eq!(sanitize_tool_call_id("call.abc!123").as_ref(), "call_abc_123");
        assert_eq!(sanitize_tool_call_id("call-abc_123").as_ref(), "call-abc_123");
        assert_eq!(sanitize_tool_call_id("call abc").as_ref(), "call_abc");
        // Already-valid IDs should borrow (no allocation).
        assert!(matches!(sanitize_tool_call_id("toolu_01abc"), Cow::Borrowed(_)));
        // IDs with invalid chars should allocate.
        assert!(matches!(sanitize_tool_call_id("call.123"), Cow::Owned(_)));
    }

    #[test]
    fn map_stop_reason_content_filter() {
        assert_eq!(map_stop_reason("content_filtered"), "content_filter");
        assert_eq!(map_stop_reason("refusal"), "content_filter");
    }
}
