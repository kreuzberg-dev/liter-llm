```elixir
{:ok, response} =
  LiterLlm.chat(
    %{
      model: "openai/gpt-4o",
      messages: [%{role: "user", content: "Hello!"}]
    },
    api_key: "sk-...",                          # or System.fetch_env!("OPENAI_API_KEY")
    base_url: "https://api.openai.com/v1",      # override provider base URL
    model_hint: "openai",                       # pre-resolve provider at construction
    max_retries: 3,                             # retry on transient failures
    timeout: 60                                 # request timeout in seconds
  )

IO.puts(hd(response["choices"])["message"]["content"])
```
