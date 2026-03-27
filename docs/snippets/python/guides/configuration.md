```python
import asyncio
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(
        api_key="sk-...",          # or set OPENAI_API_KEY env var
        base_url=None,             # override provider base URL
        model_hint="openai",       # pre-resolve provider at construction
        max_retries=3,             # retry on transient failures
        timeout=60,                # request timeout in seconds
    )
    response = await client.chat(
        model="openai/gpt-4o",
        messages=[{"role": "user", "content": "Hello!"}],
    )
    print(response.choices[0].message.content)

asyncio.run(main())
```
