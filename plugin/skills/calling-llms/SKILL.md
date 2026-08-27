---
name: calling-llms
description: Use when sending chat completions through liter-llm and routing to a specific provider via the `provider/model` prefix. Covers the chat call shape, provider routing, model_hint, message roles, and error categories.
---

<!--
AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Content-Hash: blake3:116e9240923b4f529f06471698b0e15ce1a14688a5beb1cae28e0b62e3a2837f
Source-Hash: blake3:297ed1a031a211f05299efd5867c02e050ce50620d284f2bb4727a631cea7f83
Schema-Version: v1
-->

# Calling LLMs

Build a `ChatCompletionRequest` and send it with `client.chat(request)`. Create
the client with `create_client(...)`. The model string is `provider/model`; the
prefix selects the backend.

```python
import asyncio, json, os
from liter_llm import create_client
from liter_llm._internal_bindings import ChatCompletionRequest

async def main() -> None:
    client = create_client(api_key=os.environ["OPENAI_API_KEY"])
    request = ChatCompletionRequest.from_json(json.dumps({
        "model": "openai/gpt-4o",
        "messages": [
            {"role": "system", "content": "You are concise."},
            {"role": "user", "content": "Name three Rust crates for HTTP."},
        ],
    }))
    response = await client.chat(request)
    print(response.choices[0].message.content)

asyncio.run(main())
```

## Provider routing

The model string's prefix selects the provider; build a request per backend:

```python
ChatCompletionRequest.from_json('{"model":"anthropic/claude-sonnet-4-20250514","messages":[...]}')
ChatCompletionRequest.from_json('{"model":"google/gemini-2.0-flash","messages":[...]}')
ChatCompletionRequest.from_json('{"model":"groq/llama3-70b","messages":[...]}')
ChatCompletionRequest.from_json('{"model":"mistral/mistral-large-latest","messages":[...]}')
ChatCompletionRequest.from_json('{"model":"bedrock/anthropic.claude-v2","messages":[...]}')
```

Set `model_hint` at construction to drop the prefix on every call:

```python
client = create_client(api_key="sk-...", model_hint="openai")
# the request model can now omit the provider prefix:
request = ChatCompletionRequest.from_json('{"model":"gpt-4o","messages":[...]}')
await client.chat(request)  # routes to OpenAI
```

## Notes

- Keys come from env vars (`OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, …); never
  hardcode them.
- Without a prefix and without `model_hint`, routing fails.
- Python errors are typed exceptions exported from `liter_llm`:
  `AuthenticationError`, `RateLimitedError`, `BadRequestError`,
  `ContextWindowExceededError`, `ContentPolicyError`, `NotFoundError`,
  `ServerError`, `ServiceUnavailableError`, `LiterLlmTimeoutError`,
  `BudgetExceededError` — all subclasses of `LiterLlmError`.
