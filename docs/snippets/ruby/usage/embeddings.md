```ruby
# frozen_string_literal: true

require "liter_llm"
require "json"

client = LiterLlm::LlmClient.new(ENV.fetch("OPENAI_API_KEY"), {})

response = JSON.parse(client.embed(JSON.generate(
  model: "openai/text-embedding-3-small",
  input: ["The quick brown fox jumps over the lazy dog"]
)))

embedding = response.dig("data", 0, "embedding")
puts "Dimensions: #{embedding.length}"
puts "First 5 values: #{embedding.first(5)}"
```
