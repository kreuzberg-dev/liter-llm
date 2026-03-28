<!-- snippet:compile-only -->

```python
import asyncio
import os
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(api_key=os.environ["OPENAI_API_KEY"])
    response = await client.rerank(
        model="cohere/rerank-v3.5",
        query="What is the capital of France?",
        documents=[
            "Paris is the capital of France.",
            "Berlin is the capital of Germany.",
            "London is the capital of England.",
        ],
    )
    for result in response.results:
        print(f"Index: {result.index}, Score: {result.relevance_score:.4f}")

asyncio.run(main())
```
