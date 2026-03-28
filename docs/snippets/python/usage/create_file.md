<!-- snippet:compile-only -->

```python
import asyncio
import os
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(api_key=os.environ["OPENAI_API_KEY"])
    with open("data.jsonl", "rb") as f:
        file_bytes = f.read()
    response = await client.create_file(
        file=file_bytes,
        filename="data.jsonl",
        purpose="batch",
    )
    print(f"File ID: {response.id}")
    print(f"Size: {response.bytes} bytes")

asyncio.run(main())
```
