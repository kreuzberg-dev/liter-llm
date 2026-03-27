```ruby
# frozen_string_literal: true

require "liter_llm"
require "json"

client = LiterLlm::LlmClient.new(ENV.fetch("OPENAI_API_KEY"), {})

response = JSON.parse(client.chat(JSON.generate(
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Hello!" }]
)))

puts response.dig("choices", 0, "message", "content")
```
