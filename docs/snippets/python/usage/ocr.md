<!-- snippet:compile-only -->

```python
import asyncio
import os
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(api_key=os.environ["MISTRAL_API_KEY"])
    response = await client.ocr(
        model="mistral/mistral-ocr-latest",
        document={"type": "document_url", "url": "https://example.com/invoice.pdf"},
    )
    for page in response.pages:
        print(f"Page {page.index}: {page.markdown[:100]}...")

asyncio.run(main())
```
