```elixir
{:ok, response} =
  LiterLlm.chat(
    %{
      model: "openai/gpt-4o",
      messages: [%{role: "user", content: "Hello!"}]
    },
    api_key: System.fetch_env!("OPENAI_API_KEY")
  )

IO.puts(hd(response["choices"])["message"]["content"])
```
