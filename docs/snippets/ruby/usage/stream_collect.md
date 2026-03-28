```ruby
# frozen_string_literal: true

require "liter_llm"
require "json"

client = LiterLlm::LlmClient.new(ENV.fetch("OPENAI_API_KEY"), {})

chunks = JSON.parse(client.chat_stream(JSON.generate(
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Explain quantum computing briefly" }]
)))

full_text = ""
chunks.each do |chunk|
  delta = chunk.dig("choices", 0, "delta", "content")
  if delta
    full_text += delta
    print delta
  end
end
puts
puts "\nFull response length: #{full_text.length} characters"
```
