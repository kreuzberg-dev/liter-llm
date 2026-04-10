```ruby
require 'liter_llm'
require 'json'

client = LiterLlm::LlmClient.new(
  api_key: ENV.fetch('OPENAI_API_KEY')
)

request = {
  model: 'openai/gpt-4o',
  messages: [{ role: 'user', content: 'Hello' }]
}.to_json

begin
  response = JSON.parse(client.chat(request))
  puts response.dig('choices', 0, 'message', 'content')
rescue RuntimeError => e
  # The Ruby binding raises plain RuntimeError. The message is the Rust
  # error's Display string — branch on its prefix to identify the category.
  case e.message
  when /\Arate limited:/            then warn "rate limited: #{e.message}"
  when /\Aauthentication failed:/   then warn "auth failed: #{e.message}"
  when /\Abudget exceeded:/         then warn "budget exceeded: #{e.message}"
  when /\Acontext window exceeded:/ then warn "prompt too long: #{e.message}"
  when /\Aservice unavailable:/     then warn "provider unavailable: #{e.message}"
  else warn "llm error: #{e.message}"
  end
end
```
