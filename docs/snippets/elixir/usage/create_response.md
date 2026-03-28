<!-- snippet:compile-only -->

```elixir
{:ok, response} =
  LiterLlm.create_response(
    %{
      model: "openai/gpt-4o",
      input: "Explain quantum computing in one sentence."
    },
    api_key: System.fetch_env!("OPENAI_API_KEY")
  )

IO.inspect(response)
```
