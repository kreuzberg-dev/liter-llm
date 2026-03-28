```elixir
{:ok, chunks} =
  LiterLlm.chat_stream(
    %{
      model: "openai/gpt-4o",
      messages: [%{role: "user", content: "Explain quantum computing briefly"}]
    },
    api_key: System.fetch_env!("OPENAI_API_KEY")
  )

full_text =
  Enum.reduce(chunks, "", fn chunk, acc ->
    delta = hd(chunk["choices"])["delta"]["content"]

    if delta do
      IO.write(delta)
      acc <> delta
    else
      acc
    end
  end)

IO.puts("")
IO.puts("\nFull response length: #{String.length(full_text)} characters")
```
