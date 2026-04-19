```elixir
client =
  LiterLlm.Client.new(
    api_key: System.fetch_env!("OPENAI_API_KEY")
  )

request = %{
  model: "openai/gpt-4o",
  messages: [%{role: "user", content: "Hello"}]
}

case LiterLlm.Client.chat(client, request) do
  {:ok, response} ->
    IO.puts(response["choices"] |> hd() |> get_in(["message", "content"]))

  # 401/403 — rotate the key.
  {:error, %LiterLlm.Error{kind: :authentication, message: message}} ->
    IO.warn("auth failed: #{message}")

  # 429 — transient, back off and retry or fall back.
  {:error, %LiterLlm.Error{kind: :rate_limit, message: message}} ->
    IO.warn("rate limited: #{message}")

  {:error, %LiterLlm.Error{kind: :budget_exceeded, message: message}} ->
    IO.warn("budget exceeded: #{message}")

  # 5xx — inspect http_status when present.
  {:error, %LiterLlm.Error{kind: :provider_error, http_status: status, message: message}} ->
    IO.warn("provider #{status}: #{message}")

  {:error, %LiterLlm.Error{kind: kind, message: message}} ->
    IO.warn("llm error (#{kind}): #{message}")
end
```
