<!-- snippet:compile-only -->

```ruby
# frozen_string_literal: true

require "liter_llm"
require "json"

client = LiterLlm::LlmClient.new(ENV.fetch("OPENAI_API_KEY"), {})

audio_bytes = client.speech(JSON.generate(
  model: "openai/tts-1",
  input: "Hello, world!",
  voice: "alloy"
))

File.binwrite("output.mp3", audio_bytes)
puts "Wrote #{audio_bytes.bytesize} bytes to output.mp3"
```
