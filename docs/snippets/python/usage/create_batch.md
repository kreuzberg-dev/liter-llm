<!-- snippet:compile-only -->

```python
import asyncio
import os
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(api_key=os.environ["OPENAI_API_KEY"])
    response = await client.create_batch(
        input_file_id="file-abc123",
        endpoint="/v1/chat/completions",
        completion_window="24h",
    )
    print(f"Batch ID: {response.id}")
    print(f"Status: {response.status}")

asyncio.run(main())
```
