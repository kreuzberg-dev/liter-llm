# Smoke tests for the liter_llm published Elixir package.
#
# Validates the published package works against real LLM APIs.
# Requires API keys in environment variables or .env file at repo root.
#
# Run: mix deps.get && mix run main.exs

# ── .env loader ──────────────────────────────────────────────────────────────

defmodule DotenvLoader do
  @doc "Load .env file by walking up from the script directory."
  def load do
    dir = __DIR__ |> Path.expand()
    find_and_load(dir, 5)
  end

  defp find_and_load(_dir, 0), do: :ok

  defp find_and_load(dir, remaining) do
    env_file = Path.join(dir, ".env")

    if File.exists?(env_file) do
      env_file
      |> File.read!()
      |> String.split("\n")
      |> Enum.each(fn line ->
        line = String.trim(line)

        unless line == "" or String.starts_with?(line, "#") do
          case String.split(line, "=", parts: 2) do
            [key, value] ->
              key = String.trim(key)
              value = String.trim(value)

              if System.get_env(key) == nil do
                System.put_env(key, value)
              end

            _ ->
              :ok
          end
        end
      end)
    else
      parent = Path.dirname(dir)

      if parent != dir do
        find_and_load(parent, remaining - 1)
      end
    end
  end
end

DotenvLoader.load()

# ── Test runner ──────────────────────────────────────────────────────────────

defmodule SmokeTest do
  defstruct passed: 0, failed: 0, skipped: 0

  def run(%__MODULE__{} = suite, name, test_fn) do
    IO.write("  #{name}... ")

    try do
      case test_fn.() do
        nil ->
          IO.puts("SKIP")
          %{suite | skipped: suite.skipped + 1}

        _ok ->
          IO.puts("PASS")
          %{suite | passed: suite.passed + 1}
      end
    rescue
      e ->
        IO.puts("FAIL: #{Exception.message(e)}")
        %{suite | failed: suite.failed + 1}
    end
  end

  def summary(%__MODULE__{passed: p, failed: f, skipped: s}) do
    total = p + f + s
    IO.puts("")
    IO.puts(String.duplicate("=", 60))
    IO.puts("Results: #{p} passed, #{f} failed, #{s} skipped (#{total} total)")
    if f > 0, do: 1, else: 0
  end
end

# ── Helper ───────────────────────────────────────────────────────────────────

defmodule EnvHelper do
  def env_key(name) do
    case System.get_env(name) do
      nil -> nil
      "" -> nil
      val -> val
    end
  end
end

# ── Test cases ───────────────────────────────────────────────────────────────

defmodule Tests do
  import EnvHelper

  def test_chat_openai do
    key = env_key("OPENAI_API_KEY")
    unless key, do: throw(:skip)

    {:ok, resp} =
      LiterLlm.chat(
        %{
          model: "openai/gpt-4o-mini",
          messages: [LiterLlm.Types.user_message("Say hello in one word.")],
          max_tokens: 10
        },
        api_key: key
      )

    if resp["choices"] == nil or resp["choices"] == [] do
      raise "no choices in response"
    end

    if resp["choices"] |> hd() |> get_in(["message", "content"]) |> is_nil() do
      raise "empty content"
    end

    if resp["usage"] == nil or (resp["usage"]["total_tokens"] || 0) <= 0 do
      raise "no usage data"
    end

    "ok"
  catch
    :skip -> nil
  end

  def test_chat_anthropic do
    key = env_key("ANTHROPIC_API_KEY")
    unless key, do: throw(:skip)

    {:ok, resp} =
      LiterLlm.chat(
        %{
          model: "anthropic/claude-sonnet-4-20250514",
          messages: [LiterLlm.Types.user_message("Say hello in one word.")],
          max_tokens: 10
        },
        api_key: key
      )

    if resp["choices"] == nil or resp["choices"] == [] do
      raise "no choices"
    end

    if resp["choices"] |> hd() |> get_in(["message", "content"]) |> is_nil() do
      raise "empty content"
    end

    "ok"
  catch
    :skip -> nil
  end

  def test_chat_gemini do
    key = env_key("GEMINI_API_KEY")
    unless key, do: throw(:skip)

    {:ok, resp} =
      LiterLlm.chat(
        %{
          model: "gemini/gemini-2.5-flash-lite",
          messages: [LiterLlm.Types.user_message("Say hello in one word.")],
          max_tokens: 10
        },
        api_key: key
      )

    if resp["choices"] == nil or resp["choices"] == [] do
      raise "no choices"
    end

    if resp["choices"] |> hd() |> get_in(["message", "content"]) |> is_nil() do
      raise "empty content"
    end

    "ok"
  catch
    :skip -> nil
  end

  def test_streaming_openai do
    key = env_key("OPENAI_API_KEY")
    unless key, do: throw(:skip)

    {:ok, chunks} =
      LiterLlm.Client.new(api_key: key)
      |> LiterLlm.Client.chat_stream(%{
        model: "openai/gpt-4o-mini",
        messages: [LiterLlm.Types.user_message("Count from 1 to 5.")],
        max_tokens: 50
      })

    if chunks == nil or chunks == [] do
      raise "no chunks received"
    end

    "ok"
  catch
    :skip -> nil
  end

  def test_embed_openai do
    key = env_key("OPENAI_API_KEY")
    unless key, do: throw(:skip)

    {:ok, resp} =
      LiterLlm.embed(
        %{
          model: "openai/text-embedding-3-small",
          input: "Hello, world!"
        },
        api_key: key
      )

    if resp["data"] == nil or resp["data"] == [] do
      raise "no embeddings"
    end

    if resp["data"] |> hd() |> Map.get("embedding") |> is_nil() do
      raise "empty embedding vector"
    end

    "ok"
  catch
    :skip -> nil
  end

  def test_list_models_openai do
    key = env_key("OPENAI_API_KEY")
    unless key, do: throw(:skip)

    {:ok, resp} = LiterLlm.list_models(api_key: key)

    if resp["data"] == nil or resp["data"] == [] do
      raise "no models returned"
    end

    "ok"
  catch
    :skip -> nil
  end

  def test_provider_routing do
    openai_key = env_key("OPENAI_API_KEY")
    anthropic_key = env_key("ANTHROPIC_API_KEY")
    unless openai_key && anthropic_key, do: throw(:skip)

    messages = [LiterLlm.Types.user_message("Say hi.")]

    {:ok, r1} =
      LiterLlm.chat(
        %{model: "openai/gpt-4o-mini", messages: messages, max_tokens: 5},
        api_key: openai_key
      )

    if r1["choices"] == nil or r1["choices"] == [] do
      raise "OpenAI failed"
    end

    {:ok, r2} =
      LiterLlm.chat(
        %{model: "anthropic/claude-sonnet-4-20250514", messages: messages, max_tokens: 5},
        api_key: anthropic_key
      )

    if r2["choices"] == nil or r2["choices"] == [] do
      raise "Anthropic failed"
    end

    "ok"
  catch
    :skip -> nil
  end

  def test_cache_memory do
    key = env_key("OPENAI_API_KEY")
    unless key, do: throw(:skip)

    messages = [LiterLlm.Types.user_message("What is 2+2? Answer with just the number.")]
    request = %{model: "openai/gpt-4o-mini", messages: messages, max_tokens: 5}
    opts = [api_key: key]

    {:ok, r1} = LiterLlm.chat(request, opts)
    {:ok, r2} = LiterLlm.chat(request, opts)

    if r1["choices"] == nil or r1["choices"] == [] do
      raise "first request failed"
    end

    if r2["choices"] == nil or r2["choices"] == [] do
      raise "second request failed"
    end

    c1 = r1["choices"] |> hd() |> get_in(["message", "content"])
    c2 = r2["choices"] |> hd() |> get_in(["message", "content"])

    if c1 != c2 do
      raise "cache miss - responses differ"
    end

    "ok"
  catch
    :skip -> nil
  end
end

# ── Main ─────────────────────────────────────────────────────────────────────

IO.puts("liter-llm Smoke Tests (Elixir)")
IO.puts(String.duplicate("=", 60))
IO.puts("")

IO.puts("Chat Completions:")

suite = %SmokeTest{}
suite = SmokeTest.run(suite, "OpenAI gpt-4o-mini", &Tests.test_chat_openai/0)
suite = SmokeTest.run(suite, "Anthropic claude-3-5-haiku", &Tests.test_chat_anthropic/0)
suite = SmokeTest.run(suite, "Google gemini-2.5-flash-lite", &Tests.test_chat_gemini/0)

suite = SmokeTest.run(suite, "OpenAI streaming", &Tests.test_streaming_openai/0)
suite = SmokeTest.run(suite, "OpenAI text-embedding-3-small", &Tests.test_embed_openai/0)
suite = SmokeTest.run(suite, "OpenAI list models", &Tests.test_list_models_openai/0)
suite = SmokeTest.run(suite, "Multi-provider routing", &Tests.test_provider_routing/0)
suite = SmokeTest.run(suite, "In-memory cache hit", &Tests.test_cache_memory/0)

exit_code = SmokeTest.summary(suite)
System.halt(exit_code)
