```python
import asyncio
import os
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(api_key=os.environ["OPENAI_API_KEY"])
    full_text = ""
    async for chunk in await client.chat_stream(
        model="openai/gpt-4o",
        messages=[{"role": "user", "content": "Explain quantum computing briefly"}],
    ):
        delta = chunk.choices[0].delta.content if chunk.choices else None
        if delta:
            full_text += delta
            print(delta, end="", flush=True)
    print()
    print(f"\nFull response length: {len(full_text)} characters")

asyncio.run(main())
```
