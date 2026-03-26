```python
import asyncio
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient()
    async for chunk in client.chat_stream(
        model="openai/gpt-4o",
        messages=[{"role": "user", "content": "Tell me a story"}],
    ):
        print(chunk.delta, end="", flush=True)
    print()

asyncio.run(main())
```
