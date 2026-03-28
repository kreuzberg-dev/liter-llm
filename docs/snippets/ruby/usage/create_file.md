<!-- snippet:compile-only -->

```ruby
# frozen_string_literal: true

require "liter_llm"
require "json"

client = LiterLlm::LlmClient.new(ENV.fetch("OPENAI_API_KEY"), {})

file_bytes = File.binread("data.jsonl")
response = JSON.parse(client.create_file(JSON.generate(
  filename: "data.jsonl",
  purpose: "batch"
), file_bytes))

puts "File ID: #{response["id"]}"
puts "Size: #{response["bytes"]} bytes"
```
