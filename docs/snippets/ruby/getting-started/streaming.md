```ruby
# frozen_string_literal: true

require "liter_llm"

client = LiterLlm::Client.new
client.chat_stream(
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Tell me a story" }]
) do |chunk|
  print chunk.delta
end
puts
```
