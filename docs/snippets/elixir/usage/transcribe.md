<!-- snippet:compile-only -->

```elixir
audio_bytes = File.read!("audio.mp3")

{:ok, response} =
  LiterLlm.transcribe(
    %{
      model: "openai/whisper-1",
      file: audio_bytes,
      filename: "audio.mp3"
    },
    api_key: System.fetch_env!("OPENAI_API_KEY")
  )

IO.puts(response["text"])
```
