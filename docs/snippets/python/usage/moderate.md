<!-- snippet:compile-only -->

```python
import asyncio
import os
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(api_key=os.environ["OPENAI_API_KEY"])
    response = await client.moderate(
        model="openai/omni-moderation-latest",
        input="This is a test message.",
    )
    result = response.results[0]
    print(f"Flagged: {result.flagged}")
    for category, flagged in result.categories.items():
        if flagged:
            print(f"  {category}: {result.category_scores[category]:.4f}")

asyncio.run(main())
```
