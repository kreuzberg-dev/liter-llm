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

| Role        | Purpose                                                |
| ----------- | ------------------------------------------------------ |
| `system`    | Sets the assistant's behavior. Sent once at the start. |
| `user`      | User input -- questions, instructions, data.           |
| `assistant` | Previous assistant responses for multi-turn context.   |
| `tool`      | Results from tool calls.                               |
| `developer` | Developer-level instructions (some providers).         |

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

| Parameter           | Type          | Description                                                 |
| ------------------- | ------------- | ----------------------------------------------------------- |
| `model`             | string        | Provider/model identifier (e.g. `"openai/gpt-4o"`)          |
| `messages`          | array         | Conversation messages                                       |
| `temperature`       | float         | Sampling temperature (0.0-2.0)                              |
| `max_tokens`        | int           | Maximum tokens to generate                                  |
| `top_p`             | float         | Nucleus sampling threshold                                  |
| `n`                 | int           | Number of completions to generate                           |
| `stop`              | string/array  | Stop sequences                                              |
| `tools`             | array         | Tool/function definitions                                   |
| `tool_choice`       | string/object | Tool selection strategy                                     |
| `response_format`   | object        | Force JSON output (`{"type": "json_object"}`)               |
| `seed`              | int           | Deterministic sampling seed                                 |
| `presence_penalty`  | float         | Penalize new topics (-2.0 to 2.0)                           |
| `frequency_penalty` | float         | Penalize repetition (-2.0 to 2.0)                           |
| `reasoning_effort`  | string        | Reasoning budget for o-series and extended-thinking models. |
| `extra_body`        | object        | Provider-specific fields passed through verbatim.           |

## Reasoning Effort

OpenAI o-series models and Anthropic extended-thinking models accept a `reasoning_effort` parameter that controls how much compute the model spends on internal reasoning before producing the final response.

=== "Python"

    ```python
    response = client.chat({
        "model": "openai/o3-mini",
        "messages": [{"role": "user", "content": "Prove the Pythagorean theorem."}],
        "reasoning_effort": "high",
    })
    ```

=== "TypeScript"

    ```typescript
    const response = await client.chat({
      model: "openai/o3-mini",
      messages: [{ role: "user", content: "Prove the Pythagorean theorem." }],
      reasoningEffort: "high",
    });
    ```

=== "Rust"

    ```rust
    let req = ChatCompletionRequest {
        model: "openai/o3-mini".into(),
        messages: vec![/* ... */],
        reasoning_effort: Some("high".into()),
        ..Default::default()
    };
    ```

=== "Go"

    ```go
    resp, err := client.Chat(ctx, &llm.ChatRequest{
        Model:           "openai/o3-mini",
        Messages:        messages,
        ReasoningEffort: "high",
    })
    ```

Accepted values for OpenAI o-series: `"low"`, `"medium"`, `"high"`. Anthropic extended thinking uses a `budget_tokens` integer instead, which maps to `reasoning_effort` when the binding converts the field.

## Structured Outputs (JSON Schema)

Pass a JSON Schema to `response_format` to constrain the model output to a specific structure. Use `"type": "json_schema"` instead of `"type": "json_object"` for schema-validated output.

=== "Python"

    ```python
    schema = {
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "age":  {"type": "integer"},
        },
        "required": ["name", "age"],
        "additionalProperties": False,
    }

    response = client.chat({
        "model": "openai/gpt-4o",
        "messages": [{"role": "user", "content": "Extract: Alice is 30 years old."}],
        "response_format": {
            "type": "json_schema",
            "json_schema": {
                "name": "person",
                "strict": True,
                "schema": schema,
            },
        },
    })
    ```

=== "TypeScript"

    ```typescript
    const response = await client.chat({
      model: "openai/gpt-4o",
      messages: [{ role: "user", content: "Extract: Alice is 30 years old." }],
      responseFormat: {
        type: "json_schema",
        jsonSchema: {
          name: "person",
          strict: true,
          schema: {
            type: "object",
            properties: {
              name: { type: "string" },
              age:  { type: "integer" },
            },
            required: ["name", "age"],
            additionalProperties: false,
          },
        },
      },
    });
    ```

=== "Rust"

    ```rust
    use serde_json::json;

    let req = ChatCompletionRequest {
        model: "openai/gpt-4o".into(),
        messages: vec![/* ... */],
        response_format: Some(json!({
            "type": "json_schema",
            "json_schema": {
                "name": "person",
                "strict": true,
                "schema": {
                    "type": "object",
                    "properties": {
                        "name": { "type": "string" },
                        "age":  { "type": "integer" }
                    },
                    "required": ["name", "age"],
                    "additionalProperties": false
                }
            }
        })),
        ..Default::default()
    };
    ```

Structured output availability depends on provider support. OpenAI `gpt-4o` and later support `json_schema`. Providers that do not support it fall back to `json_object` or return `EndpointNotSupported`.

## extra_body

Pass provider-specific parameters that liter-llm does not model natively via `extra_body`. Fields in `extra_body` are merged into the top-level request JSON before it is sent to the provider.

=== "Python"

    ```python
    response = client.chat({
        "model": "openai/gpt-4o",
        "messages": [{"role": "user", "content": "Hello"}],
        "extra_body": {
            "store": True,           # OpenAI conversation store
            "metadata": {"user": "alice"},
        },
    })
    ```

=== "TypeScript"

    ```typescript
    const response = await client.chat({
      model: "openai/gpt-4o",
      messages: [{ role: "user", content: "Hello" }],
      extraBody: {
        store: true,
        metadata: { user: "alice" },
      },
    });
    ```

=== "Rust"

    ```rust
    use serde_json::json;

    let req = ChatCompletionRequest {
        model: "openai/gpt-4o".into(),
        messages: vec![/* ... */],
        extra_body: Some(json!({ "store": true, "metadata": { "user": "alice" } })),
        ..Default::default()
    };
    ```

`extra_body` fields take lower precedence than named fields. If a named field and an `extra_body` key conflict, the named field wins.

## Audio Content Parts

Send audio inline in a user message using the `input_audio` content part type. The audio must be base64-encoded.

=== "Python"

    ```python
    import base64

    with open("audio.wav", "rb") as f:
        audio_b64 = base64.b64encode(f.read()).decode()

    response = client.chat({
        "model": "openai/gpt-4o-audio-preview",
        "messages": [{
            "role": "user",
            "content": [
                {
                    "type": "input_audio",
                    "input_audio": {
                        "data": audio_b64,
                        "format": "wav",
                    },
                },
                {"type": "text", "text": "Transcribe and summarize this audio."},
            ],
        }],
    })
    ```

=== "TypeScript"

    ```typescript
    import { readFileSync } from "fs";

    const audioB64 = readFileSync("audio.wav").toString("base64");

    const response = await client.chat({
      model: "openai/gpt-4o-audio-preview",
      messages: [{
        role: "user",
        content: [
          {
            type: "input_audio",
            inputAudio: { data: audioB64, format: "wav" },
          },
          { type: "text", text: "Transcribe and summarize this audio." },
        ],
      }],
    });
    ```

=== "Rust"

    ```rust
    use base64::{Engine, engine::general_purpose::STANDARD};
    use liter_llm::types::{ContentPart, InputAudio};

    let audio_bytes = std::fs::read("audio.wav")?;
    let audio_b64 = STANDARD.encode(&audio_bytes);

    let content = vec![
        ContentPart::InputAudio {
            input_audio: InputAudio {
                data: audio_b64,
                format: "wav".into(),
            },
        },
        ContentPart::Text { text: "Transcribe and summarize this audio.".into() },
    ];
    ```

Supported formats depend on the provider. OpenAI `gpt-4o-audio-preview` accepts `wav`, `mp3`, `ogg`, `flac`, `m4a`.

## AWS EventStream Streaming

When routing to Bedrock providers, responses arrive in AWS EventStream framing rather than SSE. liter-llm handles the framing transparently. `chat_stream` works the same way regardless of provider.

=== "Rust"

    ```rust
    // EventStream framing is transparent to the caller.
    let stream = client.chat_stream(ChatCompletionRequest {
        model: "bedrock/anthropic.claude-3-5-sonnet-20241022-v2:0".into(),
        messages: vec![/* ... */],
        ..Default::default()
    }).await?;

    // Consume exactly like any other stream.
    pin_mut!(stream);
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        if let Some(content) = chunk.choices[0].delta.content.as_deref() {
            print!("{content}");
        }
    }
    ```

=== "Python"

    ```python
    # EventStream framing is transparent to the caller.
    for chunk in client.chat_stream({
        "model": "bedrock/anthropic.claude-3-5-sonnet-20241022-v2:0",
        "messages": [{"role": "user", "content": "Hello"}],
    }):
        print(chunk["choices"][0]["delta"].get("content", ""), end="", flush=True)
    ```

!!! warning "Tower streaming buffer"
When Bedrock streaming is routed through the Tower middleware stack (`LlmService`), the entire stream is buffered in memory before chunks are yielded. This is a Tower `Service` trait constraint. For unbuffered Bedrock streaming, call `LlmClient::chat_stream()` directly, bypassing the Tower stack. See [Architecture](../concepts/architecture.md) for details.
