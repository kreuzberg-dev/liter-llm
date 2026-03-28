```ruby
# frozen_string_literal: true

require "liter_llm"
require "json"

client = LiterLlm::LlmClient.new(
  "sk-...",                     # or ENV.fetch("OPENAI_API_KEY")
  {
    "base_url" => nil,          # override provider base URL
    "model_hint" => "openai",   # pre-resolve provider at construction
    "max_retries" => 3,         # retry on transient failures
    "timeout" => 60             # request timeout in seconds
  }
)

response = JSON.parse(client.chat(JSON.generate(
  model: "openai/gpt-4o",
  messages: [{ role: "user", content: "Hello!" }]
)))
puts response.dig("choices", 0, "message", "content")
```
