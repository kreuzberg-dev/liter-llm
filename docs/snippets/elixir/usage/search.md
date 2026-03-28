<!-- snippet:compile-only -->

```elixir
{:ok, response} =
  LiterLlm.search(
    %{
      model: "brave/web-search",
      query: "What is Rust programming language?",
      max_results: 5
    },
    api_key: System.fetch_env!("BRAVE_API_KEY")
  )

for result <- response["results"] do
  IO.puts("#{result["title"]}: #{result["url"]}")
end
```
