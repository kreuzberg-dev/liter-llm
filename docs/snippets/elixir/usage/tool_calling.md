```elixir
tools = [
  %{
    type: "function",
    function: %{
      name: "get_weather",
      description: "Get the current weather for a location",
      parameters: %{
        type: "object",
        properties: %{
          location: %{type: "string", description: "City name"}
        },
        required: ["location"]
      }
    }
  }
]

{:ok, response} =
  LiterLlm.chat(
    %{
      model: "openai/gpt-4o",
      messages: [%{role: "user", content: "What is the weather in Berlin?"}],
      tools: tools
    },
    api_key: System.fetch_env!("OPENAI_API_KEY")
  )

for call <- hd(response["choices"])["message"]["tool_calls"] || [] do
  IO.puts("Tool: #{call["function"]["name"]}, Args: #{call["function"]["arguments"]}")
end
```
