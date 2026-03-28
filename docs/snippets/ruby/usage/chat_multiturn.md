```ruby
# frozen_string_literal: true

require "liter_llm"
require "json"

client = LiterLlm::LlmClient.new(ENV.fetch("OPENAI_API_KEY"), {})

messages = [
  { role: "system", content: "You are a helpful assistant." },
  { role: "user", content: "What is the capital of France?" }
]

response = JSON.parse(client.chat(JSON.generate(
  model: "openai/gpt-4o",
  messages: messages
)))
content = response.dig("choices", 0, "message", "content")
puts "Assistant: #{content}"

# Continue the conversation
messages << { role: "assistant", content: content }
messages << { role: "user", content: "What about Germany?" }

response = JSON.parse(client.chat(JSON.generate(
  model: "openai/gpt-4o",
  messages: messages
)))
puts "Assistant: #{response.dig("choices", 0, "message", "content")}"

# Token usage
usage = response["usage"]
if usage
  puts "Tokens: #{usage["prompt_tokens"]} in, #{usage["completion_tokens"]} out"
end
```
