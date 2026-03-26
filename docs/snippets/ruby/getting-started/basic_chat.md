```ruby
# frozen_string_literal: true

require "liter_llm"

client = LiterLlm::Client.new
response = client.chat(
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Hello!" }]
)
puts response.content
```
