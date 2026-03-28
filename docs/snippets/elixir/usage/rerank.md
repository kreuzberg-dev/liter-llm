<!-- snippet:compile-only -->

```elixir
{:ok, response} =
  LiterLlm.rerank(
    %{
      model: "cohere/rerank-v3.5",
      query: "What is the capital of France?",
      documents: [
        "Paris is the capital of France.",
        "Berlin is the capital of Germany.",
        "London is the capital of England."
      ]
    },
    api_key: System.fetch_env!("OPENAI_API_KEY")
  )

for result <- response["results"] do
  IO.puts("Index: #{result["index"]}, Score: #{Float.round(result["relevance_score"], 4)}")
end
```
