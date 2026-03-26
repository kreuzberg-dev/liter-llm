```python
import asyncio
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient()
    response = await client.chat(
        model="openai/gpt-4o",
        messages=[{"role": "user", "content": "Hello!"}],
    )
    print(response.content)

asyncio.run(main())
```
