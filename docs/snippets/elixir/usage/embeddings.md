```elixir
{:ok, response} =
  LiterLlm.embed(
    %{
      model: "openai/text-embedding-3-small",
      input: ["The quick brown fox jumps over the lazy dog"]
    },
    api_key: System.fetch_env!("OPENAI_API_KEY")
  )

embedding = hd(response["data"])["embedding"]
IO.puts("Dimensions: #{length(embedding)}")
IO.puts("First 5 values: #{inspect(Enum.take(embedding, 5))}")
```
