<!-- snippet:compile-only -->

```ruby
# frozen_string_literal: true

require "liter_llm"
require "json"

client = LiterLlm::LlmClient.new(ENV.fetch("OPENAI_API_KEY"), {})

response = JSON.parse(client.image_generate(JSON.generate(
  model: "openai/dall-e-3",
  prompt: "A sunset over mountains",
  n: 1,
  size: "1024x1024"
)))

puts response.dig("data", 0, "url")
```
