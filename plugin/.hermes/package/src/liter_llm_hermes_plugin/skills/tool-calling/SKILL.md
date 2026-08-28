---
name: tool-calling
description: Use when defining functions/tools for an LLM to call through liter-llm, or requesting structured JSON outputs. Covers tool schemas, tool_calls handling, and response formats.
---

<!--
AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Content-Hash: blake3:636c5d6448b2e3c003bd5caec347391513d72e415ceaef0740241141bc363b35
Source-Hash: blake3:303551e5dcfae75158588635b2ef5d8742d0251f7c3bba85e65a85abf1b3beb2
Schema-Version: v1
-->

# Tool Calling

Pass a `tools` array of function definitions; the model may respond with
`tool_calls` instead of (or alongside) text. Execute the named function and feed
the result back as a `tool` message.

## Python

```python
import asyncio, json, os
from liter_llm import create_client
from liter_llm._internal_bindings import ChatCompletionRequest

payload = {
    "model": "openai/gpt-4o",
    "messages": [{"role": "user", "content": "What is the weather in Berlin?"}],
    "tools": [
        {
            "type": "function",
            "function": {
                "name": "get_weather",
                "description": "Get the current weather for a location",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "location": {"type": "string", "description": "City name"},
                    },
                    "required": ["location"],
                },
            },
        }
    ],
    "tool_choice": "auto",
}

async def main() -> None:
    client = create_client(api_key=os.environ["OPENAI_API_KEY"])
    request = ChatCompletionRequest.from_json(json.dumps(payload))
    response = await client.chat(request)
    for call in response.choices[0].message.tool_calls or []:
        print(call.function.name, call.function.arguments)  # arguments is a JSON string

asyncio.run(main())
```

## Structured outputs

Request strict JSON with `response_format`:

```python
request = ChatCompletionRequest.from_json(json.dumps({
    "model": "openai/gpt-4o",
    "messages": [{"role": "user", "content": "Extract name and age as JSON."}],
    "response_format": {"type": "json_object"},
}))
response = await client.chat(request)
```

## Notes

- `function.arguments` is a JSON **string** — parse it before use.
- Append each tool result as a message with `role="tool"` and the matching
  `tool_call_id`, then call `chat` again to let the model continue.
- Tool support and JSON-mode availability vary by provider; check the provider
  reference if a model ignores `tools`.
