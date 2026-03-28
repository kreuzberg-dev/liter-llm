```ruby
# frozen_string_literal: true

require "liter_llm"
require "json"

client = LiterLlm::LlmClient.new(ENV.fetch("OPENAI_API_KEY"), {})

tools = [
  {
    type: "function",
    function: {
      name: "get_weather",
      description: "Get the current weather for a location",
      parameters: {
        type: "object",
        properties: {
          location: { type: "string", description: "City name" }
        },
        required: ["location"]
      }
    }
  }
]

response = JSON.parse(client.chat(JSON.generate(
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "What is the weather in Berlin?" }],
  tools: tools
)))

response.dig("choices", 0, "message", "tool_calls")&.each do |call|
  puts "Tool: #{call.dig("function", "name")}, Args: #{call.dig("function", "arguments")}"
end
```
