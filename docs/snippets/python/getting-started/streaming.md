```python
import asyncio
import os
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(api_key=os.environ["OPENAI_API_KEY"])
    async for chunk in await client.chat_stream(
        model="openai/gpt-4o",
        messages=[{"role": "user", "content": "Tell me a story"}],
    ):
        if chunk.choices and chunk.choices[0].delta.content:
            print(chunk.choices[0].delta.content, end="", flush=True)
    print()

asyncio.run(main())
```
