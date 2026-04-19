```python
import asyncio
import os

from liter_llm import (
    LlmClient,
    LlmError,
    AuthenticationError,
    RateLimitedError,
    ContextWindowExceededError,
    BudgetExceededError,
)

async def main() -> None:
    client = LlmClient(api_key=os.environ["OPENAI_API_KEY"])
    try:
        response = await client.chat(
            model="openai/gpt-4o",
            messages=[{"role": "user", "content": "Hello"}],
        )
        print(response.choices[0].message.content)
    except AuthenticationError as e:
        # 401/403 – rotate the key, do not retry.
        print(f"auth failed: {e}")
    except RateLimitedError as e:
        # 429 – transient, retry with backoff or fall back to another model.
        print(f"rate limited: {e}")
    except ContextWindowExceededError as e:
        # Trim the prompt or use a larger context window.
        print(f"prompt too long: {e}")
    except BudgetExceededError as e:
        # Virtual-key or global budget cap hit.
        print(f"budget exceeded: {e}")
    except LlmError as e:
        # Catch-all for the remaining liter-llm errors.
        print(f"llm error: {e}")

asyncio.run(main())
```
