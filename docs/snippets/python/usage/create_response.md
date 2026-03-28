<!-- snippet:compile-only -->

```python
import asyncio
import os
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(api_key=os.environ["OPENAI_API_KEY"])
    response = await client.create_response(
        model="openai/gpt-4o",
        input="Explain quantum computing in one sentence.",
    )
    print(response)

asyncio.run(main())
```
