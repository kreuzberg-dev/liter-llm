---
name: streaming-responses
description: Use when streaming tokens incrementally from an LLM via liter-llm over SSE or async iterators. Covers chat_stream, delta handling, and null-content chunks.
---

<!--
AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Content-Hash: blake3:13410510899be94ef40712ffcb27a284ef4ac4977b6e45b38b014d60e913f995
Source-Hash: blake3:fe747c4e744a42207f650b123a76e989550ed74082700d02a33acb0bc9599b03
Schema-Version: v1
-->

# Streaming Responses

Use `chat_stream(...)` to receive tokens as they are produced instead of waiting
for the full completion. The proxy streams over SSE; bindings expose async
iterators.

## Python

```python
import asyncio, os
from liter_llm import create_client
from liter_llm._internal_bindings import ChatCompletionRequest

async def main() -> None:
    client = create_client(api_key=os.environ["OPENAI_API_KEY"])
    request = ChatCompletionRequest.from_json(
        '{"model":"openai/gpt-4o","messages":[{"role":"user","content":"Tell me a story"}],"stream":true}'
    )
    async for chunk in client.chat_stream(request):
        if chunk.choices and chunk.choices[0].delta.content:
            print(chunk.choices[0].delta.content, end="", flush=True)
    print()

asyncio.run(main())
```

## TypeScript

```typescript
import { createClient } from "@xberg-io/liter-llm";

const client = createClient(process.env.OPENAI_API_KEY!);
const chunks = await client.chatStream({
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Tell me a story" }],
});
for await (const chunk of chunks) {
  process.stdout.write(chunk.choices?.[0]?.delta?.content ?? "");
}
```

## Notes

- The first and last chunks often carry null content. Always null-check
  `chunk.choices[0].delta.content` (Python) or
  `chunk.choices[0]?.delta?.content` (TypeScript) before using it.
- Tool-call deltas arrive in `delta.tool_calls` (Python) /
  `delta.toolCalls` (TypeScript); accumulate `function.arguments` fragments
  across chunks before parsing.
- Through the proxy, request streaming with `"stream": true` on
  `/v1/chat/completions`.
