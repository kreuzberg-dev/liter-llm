```python
import asyncio
import os
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(api_key=os.environ["OPENAI_API_KEY"])
    response = await client.embed(
        model="openai/text-embedding-3-small",
        input=["The quick brown fox jumps over the lazy dog"],
    )
    print(f"Dimensions: {len(response.data[0].embedding)}")
    print(f"First 5 values: {response.data[0].embedding[:5]}")

asyncio.run(main())
```
