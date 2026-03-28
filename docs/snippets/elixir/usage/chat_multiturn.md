```elixir
messages = [
  %{role: "system", content: "You are a helpful assistant."},
  %{role: "user", content: "What is the capital of France?"}
]

{:ok, response} =
  LiterLlm.chat(
    %{model: "openai/gpt-4o", messages: messages},
    api_key: System.fetch_env!("OPENAI_API_KEY")
  )

content = hd(response["choices"])["message"]["content"]
IO.puts("Assistant: #{content}")

# Continue the conversation
messages =
  messages ++
    [
      %{role: "assistant", content: content},
      %{role: "user", content: "What about Germany?"}
    ]

{:ok, response} =
  LiterLlm.chat(
    %{model: "openai/gpt-4o", messages: messages},
    api_key: System.fetch_env!("OPENAI_API_KEY")
  )

IO.puts("Assistant: #{hd(response["choices"])["message"]["content"]}")

# Token usage
usage = response["usage"]
if usage do
  IO.puts("Tokens: #{usage["prompt_tokens"]} in, #{usage["completion_tokens"]} out")
end
```
