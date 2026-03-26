```elixir
{:ok, response} =
  LiterLlm.chat(
    model: "openai/gpt-4o",
    messages: [
      %{"role" => "user", "content" => "Hello!"}
    ]
  )

IO.puts(response["content"])
```
