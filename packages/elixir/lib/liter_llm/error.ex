defmodule LiterLlm.Error do
  @moduledoc """
  Error types for the liter-llm client.

  All errors returned by `LiterLlm` follow the `{:error, %LiterLlm.Error{}}` convention.
  Use pattern matching on `:kind` for programmatic error handling.

  ## Error Codes

  Numeric codes (1000+) are stable across releases:

  | Code | Kind | Description |
  |------|------|-------------|
  | 1000 | `:unknown` | Unknown or unclassified error |
  | 1400 | `:invalid_request` | Malformed request body |
  | 1401 | `:authentication` | API key rejected |
  | 1404 | `:not_found` | Model or resource not found |
  | 1429 | `:rate_limit` | Provider rate limit exceeded |
  | 1500 | `:provider_error` | Provider 5xx error |
  | 1600 | `:stream_error` | Streaming response parse failure |
  | 1700 | `:serialization` | JSON encode/decode failure |

  ## Examples

      case LiterLlm.chat(request) do
        {:ok, response} -> process(response)
        {:error, %LiterLlm.Error{kind: :rate_limit}} -> retry_after_backoff()
        {:error, %LiterLlm.Error{kind: :authentication, message: msg}} -> raise "Auth failed: \#{msg}"
        {:error, %LiterLlm.Error{} = err} -> Logger.error("LLM error", error: err)
      end

  """

  @type kind ::
          :unknown
          | :invalid_request
          | :authentication
          | :not_found
          | :rate_limit
          | :provider_error
          | :stream_error
          | :serialization
          | :budget_exceeded
          | :hook_rejected

  @typedoc "A structured liter-llm error."
  @type t :: %__MODULE__{
          kind: kind(),
          message: String.t(),
          code: pos_integer(),
          http_status: pos_integer() | nil
        }

  defstruct [:kind, :message, :code, :http_status]

  # Error code constants
  @code_unknown 1000
  @code_invalid_request 1400
  @code_authentication 1401
  @code_not_found 1404
  @code_rate_limit 1429
  @code_provider_error 1500
  @code_stream_error 1600
  @code_serialization 1700

  @doc """
  Creates an `:unknown` error.
  """
  @spec unknown(String.t()) :: t()
  def unknown(message) do
    %__MODULE__{kind: :unknown, message: message, code: @code_unknown}
  end

  @doc """
  Creates an `:invalid_request` error (HTTP 400/422).
  """
  @spec invalid_request(String.t()) :: t()
  def invalid_request(message) do
    %__MODULE__{
      kind: :invalid_request,
      message: "liter-llm: invalid request: #{message}",
      code: @code_invalid_request,
      http_status: 400
    }
  end

  @doc """
  Creates an `:authentication` error (HTTP 401/403).
  """
  @spec authentication(String.t()) :: t()
  def authentication(message) do
    %__MODULE__{
      kind: :authentication,
      message: "liter-llm: authentication failed: #{message}",
      code: @code_authentication,
      http_status: 401
    }
  end

  @doc """
  Creates a `:not_found` error (HTTP 404).
  """
  @spec not_found(String.t()) :: t()
  def not_found(message) do
    %__MODULE__{
      kind: :not_found,
      message: "liter-llm: not found: #{message}",
      code: @code_not_found,
      http_status: 404
    }
  end

  @doc """
  Creates a `:rate_limit` error (HTTP 429).
  """
  @spec rate_limit(String.t()) :: t()
  def rate_limit(message) do
    %__MODULE__{
      kind: :rate_limit,
      message: "liter-llm: rate limit exceeded: #{message}",
      code: @code_rate_limit,
      http_status: 429
    }
  end

  @doc """
  Creates a `:provider_error` for a given HTTP status code.
  """
  @spec provider_error(pos_integer(), String.t()) :: t()
  def provider_error(http_status, message) do
    %__MODULE__{
      kind: :provider_error,
      message: "liter-llm: provider error #{http_status}: #{message}",
      code: @code_provider_error,
      http_status: http_status
    }
  end

  @doc """
  Creates a `:stream_error` for a streaming parse failure.
  """
  @spec stream_error(String.t()) :: t()
  def stream_error(message) do
    %__MODULE__{
      kind: :stream_error,
      message: "liter-llm: stream error: #{message}",
      code: @code_stream_error
    }
  end

  @doc """
  Creates a `:serialization` error for JSON encode/decode failures.
  """
  @spec serialization(String.t()) :: t()
  def serialization(message) do
    %__MODULE__{
      kind: :serialization,
      message: "liter-llm: serialization error: #{message}",
      code: @code_serialization
    }
  end

  @code_budget_exceeded 1800
  @code_hook_rejected 1801

  @doc """
  Creates a `:budget_exceeded` error.
  """
  @spec budget_exceeded(String.t()) :: t()
  def budget_exceeded(message) do
    %__MODULE__{
      kind: :budget_exceeded,
      message: "liter-llm: budget exceeded: #{message}",
      code: @code_budget_exceeded
    }
  end

  @doc """
  Creates a `:hook_rejected` error.
  """
  @spec hook_rejected(String.t()) :: t()
  def hook_rejected(message) do
    %__MODULE__{
      kind: :hook_rejected,
      message: "liter-llm: hook rejected request: #{message}",
      code: @code_hook_rejected
    }
  end

  @doc """
  Classifies an HTTP status code into the appropriate error struct.

  ## Examples

      iex> LiterLlm.Error.from_http_status(429, "too many requests")
      %LiterLlm.Error{kind: :rate_limit, code: 1429, http_status: 429, ...}

  """
  @spec from_http_status(pos_integer(), String.t()) :: t()
  def from_http_status(status, message) do
    cond do
      status in [400, 422] -> invalid_request(message)
      status in [401, 403] -> authentication(message)
      status == 404 -> not_found(message)
      status == 429 -> rate_limit(message)
      true -> provider_error(status, message)
    end
  end

  @doc """
  Extracts a human-readable message from a provider error response body.

  Returns the value of `error.message` when present, otherwise truncates the raw body.
  """
  @spec extract_message(map() | String.t() | nil) :: String.t()
  def extract_message(nil), do: "empty response body"
  def extract_message(""), do: "empty response body"

  def extract_message(body) when is_map(body) do
    get_in(body, ["error", "message"]) ||
      get_in(body, [:error, :message]) ||
      inspect(body)
  end

  def extract_message(body) when is_binary(body) do
    case Jason.decode(body) do
      {:ok, decoded} ->
        extract_message(decoded)

      {:error, _} ->
        if byte_size(body) > 200 do
          String.slice(body, 0, 200) <> "…"
        else
          body
        end
    end
  end

  defimpl String.Chars do
    def to_string(%LiterLlm.Error{message: message}), do: message
  end
end
