<!-- snippet:compile-only -->

```ruby
# frozen_string_literal: true

require "liter_llm"
require "json"

client = LiterLlm::LlmClient.new(ENV.fetch("OPENAI_API_KEY"), {})

audio_bytes = File.binread("audio.mp3")
response = JSON.parse(client.transcribe(JSON.generate(
  model: "openai/whisper-1",
  filename: "audio.mp3"
), audio_bytes))

puts response["text"]
```
