<!-- snippet:compile-only -->

```ruby
# frozen_string_literal: true

require "liter_llm"
require "json"

client = LiterLlm::LlmClient.new(ENV.fetch("OPENAI_API_KEY"), {})

response = JSON.parse(client.create_response(JSON.generate(
  model: "openai/gpt-4o",
  input: "Explain quantum computing in one sentence."
)))

puts response
```
