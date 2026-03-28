<!-- snippet:compile-only -->

```ruby
# frozen_string_literal: true

require "liter_llm"
require "json"

client = LiterLlm::LlmClient.new(ENV.fetch("OPENAI_API_KEY"), {})

response = JSON.parse(client.rerank(JSON.generate(
  model: "cohere/rerank-v3.5",
  query: "What is the capital of France?",
  documents: [
    "Paris is the capital of France.",
    "Berlin is the capital of Germany.",
    "London is the capital of England."
  ]
)))

response["results"].each do |result|
  puts "Index: #{result["index"]}, Score: #{format("%.4f", result["relevance_score"])}"
end
```
