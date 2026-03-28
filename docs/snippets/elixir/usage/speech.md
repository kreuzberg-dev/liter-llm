<!-- snippet:compile-only -->

```elixir
{:ok, audio_bytes} =
  LiterLlm.speech(
    %{
      model: "openai/tts-1",
      input: "Hello, world!",
      voice: "alloy"
    },
    api_key: System.fetch_env!("OPENAI_API_KEY")
  )

File.write!("output.mp3", audio_bytes)
IO.puts("Wrote #{byte_size(audio_bytes)} bytes to output.mp3")
```
