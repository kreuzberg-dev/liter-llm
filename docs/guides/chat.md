---
description: "How to use chat completions, multi-turn conversations, and sampling parameters with liter-llm."
---

# Chat Completions

The `chat` method sends a list of messages to an LLM and returns a single response. This is the primary API for most use cases.

## Basic Chat

=== "Python"

    --8<-- "snippets/python/getting-started/basic_chat.md"

=== "TypeScript"

    --8<-- "snippets/typescript/getting-started/basic_chat.md"

=== "Go"

    --8<-- "snippets/go/getting-started/basic_chat.md"

=== "Ruby"

    --8<-- "snippets/ruby/getting-started/basic_chat.md"

=== "Java"

    --8<-- "snippets/java/getting-started/basic_chat.md"

=== "C#"

    --8<-- "snippets/csharp/getting-started/basic_chat.md"

=== "Elixir"

    --8<-- "snippets/elixir/getting-started/basic_chat.md"

=== "PHP"

    --8<-- "snippets/php/getting-started/basic_chat.md"

## Message Roles

Messages use the OpenAI-compatible role system:

| Role | Purpose |
| --- | --- |
| `system` | Sets the assistant's behavior and persona. Sent once at the start. |
| `user` | User input -- questions, instructions, data to process. |
| `assistant` | Previous assistant responses. Include these for multi-turn context. |
| `tool` | Results from tool calls. Sent after the assistant requests a tool invocation. |
| `developer` | Developer-level instructions (supported by some providers). |

## Multi-Turn Conversations

To continue a conversation, append the assistant's response and the next user message to the messages list, then call `chat` again.

=== "Python"

    --8<-- "snippets/python/guides/chat_multiturn.md"

=== "TypeScript"

    --8<-- "snippets/typescript/guides/chat_multiturn.md"

=== "Go"

    --8<-- "snippets/go/guides/chat_multiturn.md"

## Sampling Parameters

Control response generation with these parameters:

| Parameter | Type | Default | Description |
| --- | --- | --- | --- |
| `temperature` | float | 1.0 | Randomness. 0 = deterministic, 2 = very random. |
| `top_p` | float | 1.0 | Nucleus sampling. 0.1 = only top 10% probability mass. |
| `max_tokens` | int | model default | Maximum tokens in the response. |
| `n` | int | 1 | Number of completions to generate. |
| `stop` | string/list | none | Stop sequences. Generation stops when any is encountered. |
| `presence_penalty` | float | 0 | Penalize tokens that have appeared. Range: -2.0 to 2.0. |
| `frequency_penalty` | float | 0 | Penalize tokens by frequency. Range: -2.0 to 2.0. |
| `seed` | int | none | For deterministic outputs (provider support varies). |
| `reasoning_effort` | string | none | Hint for reasoning models (e.g. `"low"`, `"medium"`, `"high"`). |

```python
response = await client.chat(
    model="openai/gpt-4o",
    messages=[{"role": "user", "content": "Write a haiku about Rust"}],
    temperature=0.7,
    max_tokens=100,
    top_p=0.9,
)
print(response.choices[0].message.content)
```

!!! note "Parameter support varies by provider"
    Not all providers support all parameters. Unsupported parameters are silently ignored by most providers. Check your provider's documentation for specifics.

## Token Usage

Every `ChatCompletionResponse` includes a `usage` field with token counts:

```python
response = await client.chat(
    model="openai/gpt-4o",
    messages=[{"role": "user", "content": "Hello!"}],
)
if response.usage:
    print(f"Prompt tokens:     {response.usage.prompt_tokens}")
    print(f"Completion tokens: {response.usage.completion_tokens}")
    print(f"Total tokens:      {response.usage.total_tokens}")
```

## Cost Estimation

In Rust, the response includes an `estimated_cost()` method that calculates the approximate USD cost based on embedded pricing data for the provider and model:

```rust
if let Some(cost) = response.estimated_cost() {
    println!("Estimated cost: ${cost:.6}");
}
```

!!! tip "Cost tracking at scale"
    For production cost tracking, use the [CostTrackingLayer](../concepts/architecture.md#tower-middleware-stack) Tower middleware, which emits cost data as OpenTelemetry span attributes.

## Response Format

Use `response_format` to request structured output:

```python
response = await client.chat(
    model="openai/gpt-4o",
    messages=[{"role": "user", "content": "List 3 colors as JSON"}],
    response_format={"type": "json_object"},
)
```

## Tool Calling

Pass tools to let the model invoke functions. See the tool calling example:

=== "Python"

    --8<-- "snippets/python/getting-started/tool_calling.md"

=== "TypeScript"

    --8<-- "snippets/typescript/getting-started/tool_calling.md"
