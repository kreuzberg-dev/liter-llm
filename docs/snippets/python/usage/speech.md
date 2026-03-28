<!-- snippet:compile-only -->

```python
import asyncio
import os
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(api_key=os.environ["OPENAI_API_KEY"])
    audio_bytes = await client.speech(
        model="openai/tts-1",
        input="Hello, world!",
        voice="alloy",
    )
    with open("output.mp3", "wb") as f:
        f.write(audio_bytes)
    print(f"Wrote {len(audio_bytes)} bytes to output.mp3")

asyncio.run(main())
```
