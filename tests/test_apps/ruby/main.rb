#!/usr/bin/env ruby
# frozen_string_literal: true

# Smoke tests for the liter_llm published Ruby gem.
#
# Validates the published gem works against real LLM APIs.
# Requires API keys in environment variables or .env file at repo root.

require "json"
require "liter_llm"

# ── .env loader ──────────────────────────────────────────────────────────────

def load_dotenv
  dir = File.expand_path("..", __dir__)
  4.times do
    env_file = File.join(dir, ".env")
    if File.exist?(env_file)
      File.readlines(env_file).each do |line|
        line = line.strip
        next if line.empty? || line.start_with?("#")

        idx = line.index("=")
        next unless idx

        key = line[0...idx].strip
        value = line[(idx + 1)..].strip
        ENV[key] ||= value
      end
      break
    end
    dir = File.expand_path("..", dir)
  end
end

load_dotenv

def env_key(name)
  val = ENV[name]
  val && !val.empty? ? val : nil
end

# ── Test runner ──────────────────────────────────────────────────────────────

class SmokeTest
  attr_reader :passed, :failed, :skipped

  def initialize
    @passed = 0
    @failed = 0
    @skipped = 0
  end

  def run(name)
    $stdout.write("  #{name}... ")
    $stdout.flush
    result = yield
    if result.nil?
      puts "SKIP"
      @skipped += 1
    else
      puts "PASS"
      @passed += 1
    end
  rescue StandardError => e
    puts "FAIL: #{e.message}"
    @failed += 1
  end

  def summary
    total = @passed + @failed + @skipped
    puts
    puts "=" * 60
    puts "Results: #{@passed} passed, #{@failed} failed, #{@skipped} skipped (#{total} total)"
    @failed.positive? ? 1 : 0
  end
end

# ── Test cases ───────────────────────────────────────────────────────────────

def test_chat_openai
  key = env_key("OPENAI_API_KEY")
  return nil unless key

  client = LiterLlm::LlmClient.new(key)
  resp = JSON.parse(client.chat(JSON.generate({
    model: "openai/gpt-4o-mini",
    messages: [{ role: "user", content: "Say hello in one word." }],
    max_tokens: 10
  })))
  raise "no choices in response" if resp["choices"].nil? || resp["choices"].empty?
  raise "empty content" if resp["choices"][0]["message"]["content"].nil?
  raise "no usage data" if resp["usage"].nil? || resp["usage"]["total_tokens"].to_i <= 0

  "ok"
end

def test_chat_anthropic
  key = env_key("ANTHROPIC_API_KEY")
  return nil unless key

  client = LiterLlm::LlmClient.new(key)
  resp = JSON.parse(client.chat(JSON.generate({
    model: "anthropic/claude-sonnet-4-20250514",
    messages: [{ role: "user", content: "Say hello in one word." }],
    max_tokens: 10
  })))
  raise "no choices" if resp["choices"].nil? || resp["choices"].empty?
  raise "empty content" if resp["choices"][0]["message"]["content"].nil?

  "ok"
end

def test_chat_gemini
  key = env_key("GEMINI_API_KEY")
  return nil unless key

  client = LiterLlm::LlmClient.new(key)
  resp = JSON.parse(client.chat(JSON.generate({
    model: "gemini/gemini-2.5-flash-lite",
    messages: [{ role: "user", content: "Say hello in one word." }],
    max_tokens: 10
  })))
  raise "no choices" if resp["choices"].nil? || resp["choices"].empty?
  raise "empty content" if resp["choices"][0]["message"]["content"].nil?

  "ok"
end

def test_streaming_openai
  key = env_key("OPENAI_API_KEY")
  return nil unless key

  client = LiterLlm::LlmClient.new(key)
  chunks_json = client.chat_stream(JSON.generate({
    model: "openai/gpt-4o-mini",
    messages: [{ role: "user", content: "Count from 1 to 5." }],
    max_tokens: 50
  }))
  chunks = JSON.parse(chunks_json)
  raise "no chunks received" if chunks.empty?

  "ok"
end

def test_embed_openai
  key = env_key("OPENAI_API_KEY")
  return nil unless key

  client = LiterLlm::LlmClient.new(key)
  resp = JSON.parse(client.embed(JSON.generate({
    model: "openai/text-embedding-3-small",
    input: ["Hello, world!"]
  })))
  raise "no embeddings" if resp["data"].nil? || resp["data"].empty?
  raise "empty embedding vector" if resp["data"][0]["embedding"].nil? || resp["data"][0]["embedding"].empty?

  "ok"
end

def test_list_models_openai
  key = env_key("OPENAI_API_KEY")
  return nil unless key

  client = LiterLlm::LlmClient.new(key)
  resp = JSON.parse(client.list_models)
  raise "no models returned" if resp["data"].nil? || resp["data"].empty?

  "ok"
end

def test_provider_routing
  openai_key = env_key("OPENAI_API_KEY")
  anthropic_key = env_key("ANTHROPIC_API_KEY")
  return nil unless openai_key && anthropic_key

  messages = [{ role: "user", content: "Say hi." }]

  client_openai = LiterLlm::LlmClient.new(openai_key)
  r1 = JSON.parse(client_openai.chat(JSON.generate({
    model: "openai/gpt-4o-mini", messages: messages, max_tokens: 5
  })))
  raise "OpenAI failed" if r1["choices"].nil? || r1["choices"].empty?

  client_anthropic = LiterLlm::LlmClient.new(anthropic_key)
  r2 = JSON.parse(client_anthropic.chat(JSON.generate({
    model: "anthropic/claude-sonnet-4-20250514", messages: messages, max_tokens: 5
  })))
  raise "Anthropic failed" if r2["choices"].nil? || r2["choices"].empty?

  "ok"
end

def test_cache_memory
  key = env_key("OPENAI_API_KEY")
  return nil unless key

  # Ruby binding takes JSON for requests; cache config is via constructor params
  client = LiterLlm::LlmClient.new(key)
  messages = [{ role: "user", content: "What is 2+2? Answer with just the number." }]
  req = JSON.generate({ model: "openai/gpt-4o-mini", messages: messages, max_tokens: 5 })

  r1 = JSON.parse(client.chat(req))
  r2 = JSON.parse(client.chat(req))

  raise "first request failed" if r1["choices"].nil? || r1["choices"].empty?
  raise "second request failed" if r2["choices"].nil? || r2["choices"].empty?
  if r1["choices"][0]["message"]["content"] != r2["choices"][0]["message"]["content"]
    raise "cache miss - responses differ"
  end

  "ok"
end

# ── Main ─────────────────────────────────────────────────────────────────────

puts "liter-llm Smoke Tests (Ruby)"
puts "=" * 60
puts

suite = SmokeTest.new

puts "Chat Completions:"
suite.run("OpenAI gpt-4o-mini") { test_chat_openai }
suite.run("Anthropic claude-3-5-haiku") { test_chat_anthropic }
suite.run("Google gemini-2.5-flash-lite") { test_chat_gemini }

suite.run("OpenAI streaming") { test_streaming_openai }
suite.run("OpenAI text-embedding-3-small") { test_embed_openai }
suite.run("OpenAI list models") { test_list_models_openai }
suite.run("Multi-provider routing") { test_provider_routing }
suite.run("In-memory cache hit") { test_cache_memory }

exit suite.summary
