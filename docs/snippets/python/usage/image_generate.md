<!-- snippet:compile-only -->

```python
import asyncio
import os
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(api_key=os.environ["OPENAI_API_KEY"])
    response = await client.image_generate(
        model="openai/dall-e-3",
        prompt="A sunset over mountains",
        n=1,
        size="1024x1024",
    )
    print(response.data[0].url)

asyncio.run(main())
```
