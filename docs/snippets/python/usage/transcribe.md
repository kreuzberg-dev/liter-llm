<!-- snippet:compile-only -->

```python
import asyncio
import os
from liter_llm import LlmClient

async def main() -> None:
    client = LlmClient(api_key=os.environ["OPENAI_API_KEY"])
    with open("audio.mp3", "rb") as f:
        audio_bytes = f.read()
    response = await client.transcribe(
        model="openai/whisper-1",
        file=audio_bytes,
        filename="audio.mp3",
    )
    print(response.text)

asyncio.run(main())
```
