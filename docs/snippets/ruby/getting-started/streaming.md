```ruby
# frozen_string_literal: true

require "liter_llm"
require "json"

# Note: The Ruby client does not yet support streaming.
# Use the non-streaming chat method instead.
client = LiterLlm::LlmClient.new(ENV.fetch("OPENAI_API_KEY"), {})

response = JSON.parse(client.chat(JSON.generate(
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Tell me a story" }]
)))

puts response.dig("choices", 0, "message", "content")
```
