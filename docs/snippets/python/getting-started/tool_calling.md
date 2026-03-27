```python
import asyncio
import os
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(api_key=os.environ["OPENAI_API_KEY"])

    tools = [
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
    ]

    response = await client.chat(
        model="openai/gpt-4o",
        messages=[{"role": "user", "content": "What is the weather in Berlin?"}],
        tools=tools,
    )

    choice = response.choices[0]
    if choice.message.tool_calls:
        for call in choice.message.tool_calls:
            print(f"Tool: {call.function.name}, Args: {call.function.arguments}")

asyncio.run(main())
```
