```python
import asyncio
from liter_llm import LlmClient, Tool, ToolParameter

async def main() -> None:
    client = LlmClient()

    get_weather = Tool(
        name="get_weather",
        description="Get the current weather for a location",
        parameters=[
            ToolParameter(name="location", type="string", description="City name", required=True),
        ],
    )

    response = await client.chat(
        model="openai/gpt-4o",
        messages=[{"role": "user", "content": "What is the weather in Berlin?"}],
        tools=[get_weather],
    )

    if response.tool_calls:
        for call in response.tool_calls:
            print(f"Tool: {call.name}, Args: {call.arguments}")

asyncio.run(main())
```
