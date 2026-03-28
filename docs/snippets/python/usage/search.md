<!-- snippet:compile-only -->

```python
import asyncio
import os
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(api_key=os.environ["BRAVE_API_KEY"])
    response = await client.search(
        model="brave/web-search",
        query="What is Rust programming language?",
        max_results=5,
    )
    for result in response.results:
        print(f"{result.title}: {result.url}")

asyncio.run(main())
```
