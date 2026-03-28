<!-- snippet:compile-only -->

```ruby
# frozen_string_literal: true

require "liter_llm"
require "json"

client = LiterLlm::LlmClient.new(ENV.fetch("OPENAI_API_KEY"), {})

response = JSON.parse(client.moderate(JSON.generate(
  model: "openai/omni-moderation-latest",
  input: "This is a test message."
)))

result = response.dig("results", 0)
puts "Flagged: #{result["flagged"]}"
result["categories"].each do |category, flagged|
  if flagged
    puts "  #{category}: #{format("%.4f", result["category_scores"][category])}"
  end
end
```
