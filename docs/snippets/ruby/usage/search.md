<!-- snippet:compile-only -->

```ruby
# frozen_string_literal: true

require "liter_llm"
require "json"

client = LiterLlm::LlmClient.new(ENV.fetch("BRAVE_API_KEY"), {})

response = JSON.parse(client.search(JSON.generate(
  model: "brave/web-search",
  query: "What is Rust programming language?",
  max_results: 5
)))

response["results"].each do |result|
  puts "#{result["title"]}: #{result["url"]}"
end
```
