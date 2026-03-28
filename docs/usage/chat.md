---
description: "Chat completions, streaming, multi-turn conversations, and tool calling with liter-llm."
---

# Chat & Streaming

## Basic Chat

Send a message and get a response:

=== "Python"

    --8<-- "snippets/python/getting-started/basic_chat.md"

=== "TypeScript"

    --8<-- "snippets/typescript/getting-started/basic_chat.md"

=== "Rust"

    --8<-- "snippets/rust/getting-started/basic_chat.md"

=== "Go"

    --8<-- "snippets/go/getting-started/basic_chat.md"

=== "Java"

    --8<-- "snippets/java/getting-started/basic_chat.md"

=== "C#"

    --8<-- "snippets/csharp/getting-started/basic_chat.md"

=== "Ruby"

    --8<-- "snippets/ruby/getting-started/basic_chat.md"

=== "PHP"

    --8<-- "snippets/php/getting-started/basic_chat.md"

=== "Elixir"

    --8<-- "snippets/elixir/getting-started/basic_chat.md"

=== "WASM"

    --8<-- "snippets/wasm/getting-started/basic_chat.md"

## Provider Routing

liter-llm uses a `provider/model` prefix convention. The prefix determines which API endpoint, auth header, and parameter mappings to use:

```text
openai/gpt-4o            -> OpenAI
anthropic/claude-sonnet-4-20250514  -> Anthropic
groq/llama3-70b          -> Groq
google/gemini-2.0-flash  -> Google AI
mistral/mistral-large    -> Mistral
bedrock/anthropic.claude-v2 -> AWS Bedrock
```

Switch providers by changing the model string -- no other code changes needed.

## Message Roles

| Role | Purpose |
| --- | --- |
| `system` | Sets the assistant's behavior. Sent once at the start. |
| `user` | User input -- questions, instructions, data. |
| `assistant` | Previous assistant responses for multi-turn context. |
| `tool` | Results from tool calls. |
| `developer` | Developer-level instructions (some providers). |

## Multi-Turn Conversations

Append the assistant's response and the next user message, then call `chat` again:

=== "Python"

    --8<-- "snippets/python/guides/chat_multiturn.md"

=== "TypeScript"

    --8<-- "snippets/typescript/guides/chat_multiturn.md"

=== "Rust"

    --8<-- "snippets/rust/usage/chat_multiturn.md"

=== "Go"

    --8<-- "snippets/go/guides/chat_multiturn.md"

=== "Java"

    --8<-- "snippets/java/usage/chat_multiturn.md"

=== "C#"

    --8<-- "snippets/csharp/usage/chat_multiturn.md"

=== "Ruby"

    --8<-- "snippets/ruby/usage/chat_multiturn.md"

=== "PHP"

    --8<-- "snippets/php/usage/chat_multiturn.md"

=== "Elixir"

    --8<-- "snippets/elixir/usage/chat_multiturn.md"

=== "WASM"

    --8<-- "snippets/wasm/usage/chat_multiturn.md"

## Streaming

Stream tokens as they arrive instead of waiting for the full response:

=== "Python"

    --8<-- "snippets/python/getting-started/streaming.md"

=== "TypeScript"

    --8<-- "snippets/typescript/getting-started/streaming.md"

=== "Rust"

    --8<-- "snippets/rust/getting-started/streaming.md"

=== "Go"

    --8<-- "snippets/go/getting-started/streaming.md"

=== "Java"

    --8<-- "snippets/java/getting-started/streaming.md"

=== "C#"

    --8<-- "snippets/csharp/getting-started/streaming.md"

=== "Ruby"

    --8<-- "snippets/ruby/getting-started/streaming.md"

=== "PHP"

    --8<-- "snippets/php/getting-started/streaming.md"

=== "Elixir"

    --8<-- "snippets/elixir/getting-started/streaming.md"

=== "WASM"

    --8<-- "snippets/wasm/getting-started/streaming.md"

Each chunk contains `choices[].delta.content` with incremental text. The final chunk includes `finish_reason: "stop"`.

### Collecting the Full Response

Accumulate deltas to get both real-time output and the complete text:

=== "Python"

    --8<-- "snippets/python/guides/stream_collect.md"

=== "TypeScript"

    --8<-- "snippets/typescript/guides/stream_collect.md"

=== "Rust"

    --8<-- "snippets/rust/usage/stream_collect.md"

=== "Go"

    --8<-- "snippets/go/guides/stream_collect.md"

=== "Java"

    --8<-- "snippets/java/usage/stream_collect.md"

=== "C#"

    --8<-- "snippets/csharp/usage/stream_collect.md"

=== "Ruby"

    --8<-- "snippets/ruby/usage/stream_collect.md"

=== "PHP"

    --8<-- "snippets/php/usage/stream_collect.md"

=== "Elixir"

    --8<-- "snippets/elixir/usage/stream_collect.md"

=== "WASM"

    --8<-- "snippets/wasm/usage/stream_collect.md"

## Tool Calling

Define tools as JSON schema functions. The model can request tool calls, which you execute and return results for:

=== "Python"

    --8<-- "snippets/python/getting-started/tool_calling.md"

=== "TypeScript"

    --8<-- "snippets/typescript/getting-started/tool_calling.md"

=== "Rust"

    --8<-- "snippets/rust/usage/tool_calling.md"

=== "Go"

    --8<-- "snippets/go/usage/tool_calling.md"

=== "Java"

    --8<-- "snippets/java/usage/tool_calling.md"

=== "C#"

    --8<-- "snippets/csharp/usage/tool_calling.md"

=== "Ruby"

    --8<-- "snippets/ruby/usage/tool_calling.md"

=== "PHP"

    --8<-- "snippets/php/usage/tool_calling.md"

=== "Elixir"

    --8<-- "snippets/elixir/usage/tool_calling.md"

=== "WASM"

    --8<-- "snippets/wasm/usage/tool_calling.md"

## Chat Parameters

All chat parameters work with both `chat` and `chat_stream`:

| Parameter | Type | Description |
| --- | --- | --- |
| `model` | string | Provider/model identifier (e.g. `"openai/gpt-4o"`) |
| `messages` | array | Conversation messages |
| `temperature` | float | Sampling temperature (0.0-2.0) |
| `max_tokens` | int | Maximum tokens to generate |
| `top_p` | float | Nucleus sampling threshold |
| `n` | int | Number of completions to generate |
| `stop` | string/array | Stop sequences |
| `tools` | array | Tool/function definitions |
| `tool_choice` | string/object | Tool selection strategy |
| `response_format` | object | Force JSON output (`{"type": "json_object"}`) |
| `seed` | int | Deterministic sampling seed |
| `presence_penalty` | float | Penalize new topics (-2.0 to 2.0) |
| `frequency_penalty` | float | Penalize repetition (-2.0 to 2.0) |
