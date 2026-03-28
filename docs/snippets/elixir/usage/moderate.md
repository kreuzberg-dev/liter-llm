<!-- snippet:compile-only -->

```elixir
{:ok, response} =
  LiterLlm.moderate(
    %{
      model: "openai/omni-moderation-latest",
      input: "This is a test message."
    },
    api_key: System.fetch_env!("OPENAI_API_KEY")
  )

result = hd(response["results"])
IO.puts("Flagged: #{result["flagged"]}")

for {category, true} <- result["categories"] do
  score = result["category_scores"][category]
  IO.puts("  #{category}: #{Float.round(score, 4)}")
end
```
