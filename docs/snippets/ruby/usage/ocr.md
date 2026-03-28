<!-- snippet:compile-only -->

```ruby
# frozen_string_literal: true

require "liter_llm"
require "json"

client = LiterLlm::LlmClient.new(ENV.fetch("MISTRAL_API_KEY"), {})

response = JSON.parse(client.ocr(JSON.generate(
  model: "mistral/mistral-ocr-latest",
  document: { type: "document_url", url: "https://example.com/invoice.pdf" }
)))

response["pages"].each do |page|
  puts "Page #{page["index"]}: #{page["markdown"][0, 100]}..."
end
```
