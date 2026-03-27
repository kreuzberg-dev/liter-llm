---
description: "When and how to use streaming responses with liter-llm."
---

# Streaming Guide

Streaming delivers tokens incrementally as they are generated, rather than waiting for the full response. Use it for real-time UIs, long responses, or when time-to-first-token matters.

## When to Use Streaming

| Scenario | Recommendation |
| --- | --- |
| Chat UI showing tokens as they arrive | Stream |
| Background batch processing | Non-streaming |
| Long-form content generation | Stream |
| Short answers (classification, yes/no) | Non-streaming |
| Need `usage` data immediately | Non-streaming (some providers omit usage in streams) |

## Basic Streaming

=== "Python"

    --8<-- "snippets/python/getting-started/streaming.md"

=== "TypeScript"

    --8<-- "snippets/typescript/getting-started/streaming.md"

=== "Go"

    --8<-- "snippets/go/getting-started/streaming.md"

=== "Ruby"

    --8<-- "snippets/ruby/getting-started/streaming.md"

=== "Java"

    --8<-- "snippets/java/getting-started/streaming.md"

=== "C#"

    --8<-- "snippets/csharp/getting-started/streaming.md"

=== "Elixir"

    --8<-- "snippets/elixir/getting-started/streaming.md"

=== "WASM"

    --8<-- "snippets/wasm/getting-started/streaming.md"

## Processing Chunks

Each chunk contains a `choices[].delta.content` field with the incremental text. The first and last chunks may have a `null` content value. The final chunk includes a `finish_reason` of `"stop"`.

## Collecting the Full Response

If you need both real-time output and the complete text, accumulate deltas as you iterate:

=== "Python"

    --8<-- "snippets/python/guides/stream_collect.md"

=== "TypeScript"

    --8<-- "snippets/typescript/guides/stream_collect.md"

=== "Go"

    --8<-- "snippets/go/guides/stream_collect.md"

## Streaming with Parameters

All `chat` parameters work with `chat_stream` -- temperature, max_tokens, tools, and response_format are all supported:

```python
async for chunk in await client.chat_stream(
    model="anthropic/claude-3-5-sonnet-20241022",
    messages=[
        {"role": "system", "content": "You are a creative writer."},
        {"role": "user", "content": "Write a short story"},
    ],
    temperature=0.9,
    max_tokens=500,
):
    if chunk.choices:
        delta = chunk.choices[0].delta.content
        if delta:
            print(delta, end="", flush=True)
```

## Error Handling

Errors can occur at two points during streaming:

1. **Before any chunks** -- connection failures, auth errors, invalid requests. Raised when calling `chat_stream()`.
2. **During iteration** -- network drops, provider errors mid-response. Raised from the stream iterator.

```python
try:
    stream = await client.chat_stream(
        model="openai/gpt-4o",
        messages=[{"role": "user", "content": "Hello"}],
    )
    async for chunk in stream:
        if chunk.choices:
            delta = chunk.choices[0].delta.content
            if delta:
                print(delta, end="")
except Exception as e:
    print(f"Error: {e}")
```

!!! warning
    A successful `chat_stream()` call does not guarantee a complete response. Always handle errors from the iteration loop as well.
