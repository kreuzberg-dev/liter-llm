```python
import asyncio
import os
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(api_key=os.environ["OPENAI_API_KEY"])
    messages = [
        {"role": "system", "content": "You are a helpful assistant."},
        {"role": "user", "content": "What is the capital of France?"},
    ]

    response = await client.chat(model="openai/gpt-4o", messages=messages)
    content = response.choices[0].message.content
    print(f"Assistant: {content}")

    # Continue the conversation
    messages.append({"role": "assistant", "content": content})
    messages.append({"role": "user", "content": "What about Germany?"})

    response = await client.chat(model="openai/gpt-4o", messages=messages)
    print(f"Assistant: {response.choices[0].message.content}")

    # Token usage
    if response.usage:
        print(f"Tokens: {response.usage.prompt_tokens} in, {response.usage.completion_tokens} out")

asyncio.run(main())
```
