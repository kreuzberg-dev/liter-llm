```elixir
{:ok, stream} =
  LiterLlm.chat_stream(
    model: "openai/gpt-4o",
    messages: [
      %{"role" => "user", "content" => "Tell me a story"}
    ]
  )

stream
|> Stream.each(fn chunk -> IO.write(chunk["delta"]) end)
|> Stream.run()

IO.puts("")
```
