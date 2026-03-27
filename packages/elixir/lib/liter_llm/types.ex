defmodule LiterLlm.Types do
  @moduledoc """
  Type definitions for the liter-llm unified LLM API.

  All types mirror the OpenAI-compatible wire format and match the Rust core types in
  `crates/liter-llm/src/types/`. Values are plain Elixir maps; this module provides
  `@type` specifications and constructor helpers.

  ## Message Construction

      iex> LiterLlm.Types.system_message("You are helpful.")
      %{role: "system", content: "You are helpful."}

      iex> LiterLlm.Types.user_message("Hello!")
      %{role: "user", content: "Hello!"}

      iex> LiterLlm.Types.assistant_message(content: "Hi there!")
      %{role: "assistant", content: "Hi there!"}

  """

  # ─── Cache / Budget / Provider Config ────────────────────────────────────

  @typedoc """
  Configuration for response caching.

  - `:max_entries` — maximum number of cached responses to retain
  - `:ttl_seconds` — time-to-live for each cache entry in seconds (default 300)
  """
  @type cache_config :: %{
          required(:max_entries) => pos_integer(),
          optional(:ttl_seconds) => pos_integer()
        }

  @typedoc """
  Configuration for cost budget enforcement.

  - `:global_limit` — maximum total cost allowed across all models (optional)
  - `:model_limits` — per-model cost limits keyed by model name (optional)
  - `:enforcement` — `"strict"` or `"warn"` (default `"strict"`)
  """
  @type budget_config :: %{
          optional(:global_limit) => float() | nil,
          optional(:model_limits) => %{String.t() => float()},
          optional(:enforcement) => String.t()
        }

  @typedoc """
  Configuration for a custom LLM provider not in the built-in registry.

  - `:name` — unique provider name
  - `:base_url` — provider API base URL
  - `:auth_header` — header name for authentication
  - `:model_prefixes` — model name prefixes that route to this provider
  """
  @type provider_config :: %{
          required(:name) => String.t(),
          required(:base_url) => String.t(),
          required(:auth_header) => String.t(),
          required(:model_prefixes) => [String.t()]
        }

  # ─── Primitive Types ──────────────────────────────────────────────────────

  @type role :: String.t()
  @type finish_reason :: :stop | :length | :tool_calls | :content_filter | :function_call | :other
  @type tool_type :: :function
  @type image_detail :: :low | :high | :auto

  # ─── Content Parts ────────────────────────────────────────────────────────

  @typedoc """
  An image URL with an optional detail level controlling image resolution.

  Fields:
  - `:url` — image URL (required)
  - `:detail` — one of `:low`, `:high`, `:auto` (optional)
  """
  @type image_url :: %{
          required(:url) => String.t(),
          optional(:detail) => image_detail()
        }

  @typedoc """
  A single part of a multi-modal message content array.

  Text part: `%{type: "text", text: String.t()}`
  Image part: `%{type: "image_url", image_url: image_url()}`
  """
  @type content_part ::
          %{type: String.t(), text: String.t()}
          | %{type: String.t(), image_url: image_url()}

  # ─── Messages ─────────────────────────────────────────────────────────────

  @typedoc """
  A chat message in a conversation. The `:role` field selects the message type.

  - `"system"` — context or instructions for the model
  - `"user"` — input from the human turn
  - `"assistant"` — model reply, may include tool calls
  - `"tool"` — tool call result
  - `"developer"` — provider-specific system-level guidance
  - `"function"` — deprecated legacy role retained for API compatibility
  """
  @type message ::
          system_message()
          | user_message()
          | assistant_message()
          | tool_message()
          | developer_message()
          | function_message()

  @type user_content :: String.t() | [content_part()]

  @type system_message :: %{
          required(:role) => String.t(),
          required(:content) => String.t(),
          optional(:name) => String.t()
        }

  @type user_message :: %{
          required(:role) => String.t(),
          required(:content) => user_content(),
          optional(:name) => String.t()
        }

  @type assistant_message :: %{
          required(:role) => String.t(),
          optional(:content) => String.t() | nil,
          optional(:name) => String.t(),
          optional(:tool_calls) => [tool_call()],
          optional(:refusal) => String.t() | nil,
          optional(:function_call) => function_call() | nil
        }

  @type tool_message :: %{
          required(:role) => String.t(),
          required(:content) => String.t(),
          required(:tool_call_id) => String.t(),
          optional(:name) => String.t()
        }

  @type developer_message :: %{
          required(:role) => String.t(),
          required(:content) => String.t(),
          optional(:name) => String.t()
        }

  @typedoc "Deprecated legacy function-role message retained for API compatibility."
  @type function_message :: %{
          required(:role) => String.t(),
          required(:content) => String.t(),
          required(:name) => String.t()
        }

  # ─── Tools ────────────────────────────────────────────────────────────────

  @type function_definition :: %{
          required(:name) => String.t(),
          optional(:description) => String.t(),
          optional(:parameters) => map(),
          optional(:strict) => boolean()
        }

  @type chat_completion_tool :: %{
          required(:type) => String.t(),
          required(:function) => function_definition()
        }

  @type function_call :: %{
          required(:name) => String.t(),
          required(:arguments) => String.t()
        }

  @type tool_call :: %{
          required(:id) => String.t(),
          required(:type) => String.t(),
          required(:function) => function_call()
        }

  # ─── Tool Choice ──────────────────────────────────────────────────────────

  @typedoc """
  Controls whether and how the model calls tools.

  - `"auto"` — model decides
  - `"required"` — model must call at least one tool
  - `"none"` — model must not call any tools
  - `%{type: "function", function: %{name: name}}` — force a specific function
  """
  @type tool_choice :: String.t() | %{type: String.t(), function: %{name: String.t()}}

  # ─── Response Format ──────────────────────────────────────────────────────

  @typedoc """
  Instructs the model to produce output in a specific format.

  - `%{type: "text"}` — plain text
  - `%{type: "json_object"}` — unconstrained JSON object
  - `%{type: "json_schema", json_schema: json_schema_format()}` — structured JSON
  """
  @type response_format ::
          %{type: String.t()}
          | %{type: String.t(), json_schema: json_schema_format()}

  @type json_schema_format :: %{
          required(:name) => String.t(),
          optional(:description) => String.t(),
          optional(:schema) => map(),
          optional(:strict) => boolean()
        }

  # ─── Usage ────────────────────────────────────────────────────────────────

  @type usage :: %{
          required(:prompt_tokens) => non_neg_integer(),
          required(:completion_tokens) => non_neg_integer(),
          required(:total_tokens) => non_neg_integer()
        }

  # ─── Stream Options ───────────────────────────────────────────────────────

  @type stream_options :: %{optional(:include_usage) => boolean()}

  # ─── Chat Request ─────────────────────────────────────────────────────────

  @typedoc """
  Request body for a chat completion API call.

  Only `:model` and `:messages` are required. All other fields are optional.
  """
  @type chat_request :: %{
          required(:model) => String.t(),
          required(:messages) => [message()],
          optional(:temperature) => float(),
          optional(:top_p) => float(),
          optional(:n) => pos_integer(),
          optional(:stream) => boolean(),
          optional(:stop) => String.t() | [String.t()],
          optional(:max_tokens) => pos_integer(),
          optional(:presence_penalty) => float(),
          optional(:frequency_penalty) => float(),
          optional(:logit_bias) => %{String.t() => float()},
          optional(:user) => String.t(),
          optional(:tools) => [chat_completion_tool()],
          optional(:tool_choice) => tool_choice(),
          optional(:parallel_tool_calls) => boolean(),
          optional(:response_format) => response_format(),
          optional(:stream_options) => stream_options(),
          optional(:seed) => integer()
        }

  # ─── Chat Response ────────────────────────────────────────────────────────

  @type choice :: %{
          required(:index) => non_neg_integer(),
          required(:message) => assistant_message(),
          required(:finish_reason) => finish_reason() | nil
        }

  @type chat_response :: %{
          required(:id) => String.t(),
          required(:object) => String.t(),
          required(:created) => non_neg_integer(),
          required(:model) => String.t(),
          required(:choices) => [choice()],
          optional(:usage) => usage() | nil,
          optional(:system_fingerprint) => String.t() | nil,
          optional(:service_tier) => String.t() | nil
        }

  # ─── Stream Chunk ─────────────────────────────────────────────────────────

  @type stream_function_call :: %{
          optional(:name) => String.t() | nil,
          optional(:arguments) => String.t() | nil
        }

  @type stream_tool_call :: %{
          required(:index) => non_neg_integer(),
          optional(:id) => String.t() | nil,
          optional(:type) => String.t() | nil,
          optional(:function) => stream_function_call() | nil
        }

  @type stream_delta :: %{
          optional(:role) => String.t() | nil,
          optional(:content) => String.t() | nil,
          optional(:tool_calls) => [stream_tool_call()] | nil,
          optional(:function_call) => stream_function_call() | nil,
          optional(:refusal) => String.t() | nil
        }

  @type stream_choice :: %{
          required(:index) => non_neg_integer(),
          required(:delta) => stream_delta(),
          required(:finish_reason) => finish_reason() | nil
        }

  @type chat_chunk :: %{
          required(:id) => String.t(),
          required(:object) => String.t(),
          required(:created) => non_neg_integer(),
          required(:model) => String.t(),
          required(:choices) => [stream_choice()],
          optional(:usage) => usage() | nil,
          optional(:service_tier) => String.t() | nil
        }

  # ─── Embedding ────────────────────────────────────────────────────────────

  @type embedding_input :: String.t() | [String.t()]

  @type embedding_request :: %{
          required(:model) => String.t(),
          required(:input) => embedding_input(),
          optional(:encoding_format) => String.t(),
          optional(:dimensions) => pos_integer(),
          optional(:user) => String.t()
        }

  @type embedding_object :: %{
          required(:object) => String.t(),
          required(:embedding) => [float()],
          required(:index) => non_neg_integer()
        }

  @type embedding_response :: %{
          required(:object) => String.t(),
          required(:data) => [embedding_object()],
          required(:model) => String.t(),
          required(:usage) => usage()
        }

  # ─── Models ───────────────────────────────────────────────────────────────

  @type model_object :: %{
          required(:id) => String.t(),
          required(:object) => String.t(),
          required(:created) => non_neg_integer(),
          required(:owned_by) => String.t()
        }

  @type models_list_response :: %{
          required(:object) => String.t(),
          required(:data) => [model_object()]
        }

  # ─── Constructor Helpers ──────────────────────────────────────────────────

  @doc """
  Creates a system message.

  ## Examples

      iex> LiterLlm.Types.system_message("You are helpful.")
      %{role: "system", content: "You are helpful."}

      iex> LiterLlm.Types.system_message("You are helpful.", name: "sys")
      %{role: "system", content: "You are helpful.", name: "sys"}

  """
  @spec system_message(String.t(), keyword()) :: system_message()
  def system_message(content, opts \\ []) do
    base = %{role: "system", content: content}
    add_opts(base, opts, [:name])
  end

  @doc """
  Creates a user message with string or multi-part content.

  ## Examples

      iex> LiterLlm.Types.user_message("Hello!")
      %{role: "user", content: "Hello!"}

  """
  @spec user_message(user_content(), keyword()) :: user_message()
  def user_message(content, opts \\ []) do
    base = %{role: "user", content: content}
    add_opts(base, opts, [:name])
  end

  @doc """
  Creates an assistant message.

  ## Examples

      iex> LiterLlm.Types.assistant_message(content: "Hi there!")
      %{role: "assistant", content: "Hi there!"}

  """
  @spec assistant_message(keyword()) :: assistant_message()
  def assistant_message(opts \\ []) do
    base = %{role: "assistant"}
    add_opts(base, opts, [:content, :name, :tool_calls, :refusal, :function_call])
  end

  @doc """
  Creates a tool result message.

  ## Examples

      iex> LiterLlm.Types.tool_message("sunny", "call-123")
      %{role: "tool", content: "sunny", tool_call_id: "call-123"}

  """
  @spec tool_message(String.t(), String.t(), keyword()) :: tool_message()
  def tool_message(content, tool_call_id, opts \\ []) do
    base = %{role: "tool", content: content, tool_call_id: tool_call_id}
    add_opts(base, opts, [:name])
  end

  @doc """
  Creates a developer message.

  ## Examples

      iex> LiterLlm.Types.developer_message("Focus on accuracy.")
      %{role: "developer", content: "Focus on accuracy."}

  """
  @spec developer_message(String.t(), keyword()) :: developer_message()
  def developer_message(content, opts \\ []) do
    base = %{role: "developer", content: content}
    add_opts(base, opts, [:name])
  end

  @doc """
  Creates a chat completion tool definition.

  ## Examples

      iex> LiterLlm.Types.tool("get_weather", "Get current weather", %{"type" => "object"})
      %{
        type: "function",
        function: %{name: "get_weather", description: "Get current weather", parameters: %{"type" => "object"}}
      }

  """
  @spec tool(String.t(), String.t() | nil, map() | nil, keyword()) :: chat_completion_tool()
  def tool(name, description \\ nil, parameters \\ nil, opts \\ []) do
    function_def =
      %{name: name}
      |> maybe_put(:description, description)
      |> maybe_put(:parameters, parameters)
      |> maybe_put(:strict, Keyword.get(opts, :strict))

    %{type: "function", function: function_def}
  end

  @doc """
  Parses a finish reason string into an atom.

  ## Examples

      iex> LiterLlm.Types.parse_finish_reason("stop")
      :stop

      iex> LiterLlm.Types.parse_finish_reason("some_new_value")
      :other

  """
  @spec parse_finish_reason(String.t() | nil) :: finish_reason() | nil
  def parse_finish_reason(nil), do: nil
  def parse_finish_reason("stop"), do: :stop
  def parse_finish_reason("length"), do: :length
  def parse_finish_reason("tool_calls"), do: :tool_calls
  def parse_finish_reason("content_filter"), do: :content_filter
  def parse_finish_reason("function_call"), do: :function_call
  def parse_finish_reason(_other), do: :other

  # ─── Private Helpers ──────────────────────────────────────────────────────

  defp add_opts(map, opts, allowed_keys) do
    Enum.reduce(allowed_keys, map, fn key, acc ->
      case Keyword.fetch(opts, key) do
        {:ok, value} -> Map.put(acc, key, value)
        :error -> acc
      end
    end)
  end

  defp maybe_put(map, _key, nil), do: map
  defp maybe_put(map, key, value), do: Map.put(map, key, value)
end
