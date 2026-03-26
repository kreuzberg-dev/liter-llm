defmodule LiterLlm.Client do
  @moduledoc """
  HTTP client implementation for the liter-llm unified LLM API.

  Speaks the OpenAI-compatible wire protocol using `Req`. The model-name prefix
  selects the provider and endpoint (e.g. `"groq/llama3-70b"` routes to Groq).

  Use this module directly for advanced configuration, or use the top-level
  `LiterLlm` module for the standard interface.

  ## Configuration

  | Option | Default | Description |
  |--------|---------|-------------|
  | `:api_key` | `""` | API key for `Authorization: Bearer` header |
  | `:base_url` | `"https://api.openai.com/v1"` | Provider base URL |
  | `:max_retries` | `2` | Retry count for 429 and 5xx errors |
  | `:receive_timeout` | `60_000` | Request timeout in milliseconds |

  ## Example

      client = LiterLlm.Client.new(api_key: System.fetch_env!("OPENAI_API_KEY"))
      {:ok, response} = LiterLlm.Client.chat(client, %{
        model: "gpt-4o-mini",
        messages: [%{role: "user", content: "Hello!"}],
        max_tokens: 256
      })

  """

  alias LiterLlm.Error
  alias LiterLlm.Types

  @default_base_url "https://api.openai.com/v1"
  @default_max_retries 2
  @default_timeout_ms 60_000

  @typedoc "An opaque client configuration map."
  @type t :: %__MODULE__{
          api_key: String.t(),
          base_url: String.t(),
          max_retries: non_neg_integer(),
          receive_timeout: pos_integer()
        }

  defstruct api_key: "",
            base_url: @default_base_url,
            max_retries: @default_max_retries,
            receive_timeout: @default_timeout_ms

  # ─── Constructor ──────────────────────────────────────────────────────────

  @doc """
  Creates a new client configuration.

  ## Options

  - `:api_key` — API key (required for live requests)
  - `:base_url` — Base URL, defaults to `"https://api.openai.com/v1"`
  - `:max_retries` — Retry count for 429/5xx, defaults to `#{@default_max_retries}`
  - `:receive_timeout` — Timeout in ms, defaults to `#{@default_timeout_ms}`

  ## Examples

      iex> LiterLlm.Client.new(api_key: "sk-...")
      %LiterLlm.Client{api_key: "sk-...", base_url: "https://api.openai.com/v1", ...}

  """
  @spec new(keyword()) :: t()
  def new(opts \\ []) do
    struct!(__MODULE__, opts)
  end

  # ─── Public API ───────────────────────────────────────────────────────────

  @doc """
  Sends a chat completion request and returns the full response.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `request` — a `LiterLlm.Types.chat_request()` map
  - `opts` — additional `Req` options (e.g. `plug:` for testing)

  ## Returns

  - `{:ok, LiterLlm.Types.chat_response()}` on success
  - `{:error, LiterLlm.Error.t()}` on failure

  ## Examples

      {:ok, response} = LiterLlm.Client.chat(client, %{
        model: "gpt-4o-mini",
        messages: [LiterLlm.Types.user_message("Hello!")],
        max_tokens: 256
      })
      hd(response.choices).message.content

  """
  @spec chat(t(), Types.chat_request(), keyword()) ::
          {:ok, Types.chat_response()} | {:error, Error.t()}
  def chat(%__MODULE__{} = client, request, opts \\ []) do
    post(client, "/chat/completions", request, opts)
  end

  @doc """
  Sends an embedding request and returns the embedding response.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `request` — a `LiterLlm.Types.embedding_request()` map
  - `opts` — additional `Req` options

  ## Returns

  - `{:ok, LiterLlm.Types.embedding_response()}` on success
  - `{:error, LiterLlm.Error.t()}` on failure

  """
  @spec embed(t(), Types.embedding_request(), keyword()) ::
          {:ok, Types.embedding_response()} | {:error, Error.t()}
  def embed(%__MODULE__{} = client, request, opts \\ []) do
    post(client, "/embeddings", request, opts)
  end

  @doc """
  Lists available models for the configured provider.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `opts` — additional `Req` options

  ## Returns

  - `{:ok, LiterLlm.Types.models_list_response()}` on success
  - `{:error, LiterLlm.Error.t()}` on failure

  """
  @spec list_models(t(), keyword()) ::
          {:ok, Types.models_list_response()} | {:error, Error.t()}
  def list_models(%__MODULE__{} = client, opts \\ []) do
    get(client, "/models", opts)
  end

  # ── Additional inference methods ─────────────────────────────────────────

  @doc """
  Generates an image from a text prompt.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `request` — a map with `:prompt`, `:model`, and optional params
  - `opts` — additional `Req` options

  ## Returns

  - `{:ok, map()}` on success
  - `{:error, LiterLlm.Error.t()}` on failure

  """
  @spec image_generate(t(), map(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def image_generate(%__MODULE__{} = client, request, opts \\ []) do
    post(client, "/images/generations", request, opts)
  end

  @doc """
  Generates speech audio from text.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `request` — a map with `:model`, `:input`, `:voice`, and optional params
  - `opts` — additional `Req` options

  ## Returns

  - `{:ok, binary()}` containing the raw audio bytes on success
  - `{:error, LiterLlm.Error.t()}` on failure

  """
  @spec speech(t(), map(), keyword()) ::
          {:ok, binary()} | {:error, Error.t()}
  def speech(%__MODULE__{} = client, request, opts \\ []) do
    post_raw(client, "/audio/speech", request, opts)
  end

  @doc """
  Transcribes audio to text.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `request` — a map with `:model`, `:file`, and optional params
  - `opts` — additional `Req` options

  ## Returns

  - `{:ok, map()}` on success
  - `{:error, LiterLlm.Error.t()}` on failure

  """
  @spec transcribe(t(), map(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def transcribe(%__MODULE__{} = client, request, opts \\ []) do
    post(client, "/audio/transcriptions", request, opts)
  end

  @doc """
  Checks content against moderation policies.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `request` — a map with `:input` and optional `:model`
  - `opts` — additional `Req` options

  ## Returns

  - `{:ok, map()}` on success
  - `{:error, LiterLlm.Error.t()}` on failure

  """
  @spec moderate(t(), map(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def moderate(%__MODULE__{} = client, request, opts \\ []) do
    post(client, "/moderations", request, opts)
  end

  @doc """
  Reranks documents by relevance to a query.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `request` — a map with `:model`, `:query`, `:documents`
  - `opts` — additional `Req` options

  ## Returns

  - `{:ok, map()}` on success
  - `{:error, LiterLlm.Error.t()}` on failure

  """
  @spec rerank(t(), map(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def rerank(%__MODULE__{} = client, request, opts \\ []) do
    post(client, "/rerank", request, opts)
  end

  # ── File management methods ──────────────────────────────────────────────

  @doc """
  Uploads a file.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `request` — a map with `:file`, `:purpose`, and optional `:filename`
  - `opts` — additional `Req` options

  ## Returns

  - `{:ok, map()}` with file object fields on success
  - `{:error, LiterLlm.Error.t()}` on failure

  """
  @spec create_file(t(), map(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def create_file(%__MODULE__{} = client, request, opts \\ []) do
    post(client, "/files", request, opts)
  end

  @doc """
  Retrieves metadata for a file by ID.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `file_id` — the file ID string
  - `opts` — additional `Req` options

  ## Returns

  - `{:ok, map()}` with file object fields on success
  - `{:error, LiterLlm.Error.t()}` on failure

  """
  @spec retrieve_file(t(), String.t(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def retrieve_file(%__MODULE__{} = client, file_id, opts \\ []) do
    get(client, "/files/#{file_id}", opts)
  end

  @doc """
  Deletes a file by ID.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `file_id` — the file ID string
  - `opts` — additional `Req` options

  ## Returns

  - `{:ok, map()}` with deletion status on success
  - `{:error, LiterLlm.Error.t()}` on failure

  """
  @spec delete_file(t(), String.t(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def delete_file(%__MODULE__{} = client, file_id, opts \\ []) do
    delete(client, "/files/#{file_id}", opts)
  end

  @doc """
  Lists files, optionally filtered by query parameters.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `query` — optional map of query parameters (`:purpose`, `:limit`, `:after`)
  - `opts` — additional `Req` options

  ## Returns

  - `{:ok, map()}` with a `"data"` list on success
  - `{:error, LiterLlm.Error.t()}` on failure

  """
  @spec list_files(t(), map() | nil, keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def list_files(%__MODULE__{} = client, query \\ nil, opts \\ []) do
    path = build_query_path("/files", query)
    get(client, path, opts)
  end

  @doc """
  Retrieves the raw content of a file.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `file_id` — the file ID string
  - `opts` — additional `Req` options

  ## Returns

  - `{:ok, binary()}` containing the raw file bytes on success
  - `{:error, LiterLlm.Error.t()}` on failure

  """
  @spec file_content(t(), String.t(), keyword()) ::
          {:ok, binary()} | {:error, Error.t()}
  def file_content(%__MODULE__{} = client, file_id, opts \\ []) do
    get_raw(client, "/files/#{file_id}/content", opts)
  end

  # ── Batch management methods ─────────────────────────────────────────────

  @doc """
  Creates a new batch job.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `request` — a map with `:input_file_id`, `:endpoint`, `:completion_window`
  - `opts` — additional `Req` options

  ## Returns

  - `{:ok, map()}` with batch object fields on success
  - `{:error, LiterLlm.Error.t()}` on failure

  """
  @spec create_batch(t(), map(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def create_batch(%__MODULE__{} = client, request, opts \\ []) do
    post(client, "/batches", request, opts)
  end

  @doc """
  Retrieves a batch by ID.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `batch_id` — the batch ID string
  - `opts` — additional `Req` options

  ## Returns

  - `{:ok, map()}` with batch object fields on success
  - `{:error, LiterLlm.Error.t()}` on failure

  """
  @spec retrieve_batch(t(), String.t(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def retrieve_batch(%__MODULE__{} = client, batch_id, opts \\ []) do
    get(client, "/batches/#{batch_id}", opts)
  end

  @doc """
  Lists batches, optionally filtered by query parameters.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `query` — optional map of query parameters (`:limit`, `:after`)
  - `opts` — additional `Req` options

  ## Returns

  - `{:ok, map()}` with a `"data"` list on success
  - `{:error, LiterLlm.Error.t()}` on failure

  """
  @spec list_batches(t(), map() | nil, keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def list_batches(%__MODULE__{} = client, query \\ nil, opts \\ []) do
    path = build_query_path("/batches", query)
    get(client, path, opts)
  end

  @doc """
  Cancels an in-progress batch.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `batch_id` — the batch ID string
  - `opts` — additional `Req` options

  ## Returns

  - `{:ok, map()}` with batch object fields on success
  - `{:error, LiterLlm.Error.t()}` on failure

  """
  @spec cancel_batch(t(), String.t(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def cancel_batch(%__MODULE__{} = client, batch_id, opts \\ []) do
    post(client, "/batches/#{batch_id}/cancel", %{}, opts)
  end

  # ── Response management methods ──────────────────────────────────────────

  @doc """
  Creates a new response.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `request` — a map with `:model`, `:input`, and optional params
  - `opts` — additional `Req` options

  ## Returns

  - `{:ok, map()}` with response object fields on success
  - `{:error, LiterLlm.Error.t()}` on failure

  """
  @spec create_response(t(), map(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def create_response(%__MODULE__{} = client, request, opts \\ []) do
    post(client, "/responses", request, opts)
  end

  @doc """
  Retrieves a response by ID.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `response_id` — the response ID string
  - `opts` — additional `Req` options

  ## Returns

  - `{:ok, map()}` with response object fields on success
  - `{:error, LiterLlm.Error.t()}` on failure

  """
  @spec retrieve_response(t(), String.t(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def retrieve_response(%__MODULE__{} = client, response_id, opts \\ []) do
    get(client, "/responses/#{response_id}", opts)
  end

  @doc """
  Cancels an in-progress response.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `response_id` — the response ID string
  - `opts` — additional `Req` options

  ## Returns

  - `{:ok, map()}` with response object fields on success
  - `{:error, LiterLlm.Error.t()}` on failure

  """
  @spec cancel_response(t(), String.t(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def cancel_response(%__MODULE__{} = client, response_id, opts \\ []) do
    post(client, "/responses/#{response_id}/cancel", %{}, opts)
  end

  # ─── HTTP Internals ───────────────────────────────────────────────────────

  defp post(%__MODULE__{} = client, path, body, extra_opts) do
    json_body =
      case Jason.encode(body) do
        {:ok, encoded} ->
          encoded

        {:error, reason} ->
          return = {:error, Error.serialization("failed to encode request: #{inspect(reason)}")}
          throw(return)
      end

    req = build_req(client, extra_opts)

    do_with_retry(client.max_retries, fn ->
      case Req.post(req, url: path, body: json_body) do
        {:ok, %{status: status, body: resp_body}} when status in 200..299 ->
          decode_response(resp_body)

        {:ok, %{status: status, body: resp_body}} ->
          message = Error.extract_message(resp_body)
          {:error, Error.from_http_status(status, message)}

        {:error, exception} ->
          {:error, Error.unknown("HTTP request failed: #{Exception.message(exception)}")}
      end
    end)
  catch
    {:error, _} = err -> err
  end

  defp get(%__MODULE__{} = client, path, extra_opts) do
    req = build_req(client, extra_opts)

    do_with_retry(client.max_retries, fn ->
      case Req.get(req, url: path) do
        {:ok, %{status: status, body: resp_body}} when status in 200..299 ->
          decode_response(resp_body)

        {:ok, %{status: status, body: resp_body}} ->
          message = Error.extract_message(resp_body)
          {:error, Error.from_http_status(status, message)}

        {:error, exception} ->
          {:error, Error.unknown("HTTP request failed: #{Exception.message(exception)}")}
      end
    end)
  end

  defp build_req(%__MODULE__{} = client, extra_opts) do
    base_url = String.trim_trailing(client.base_url, "/")

    req_opts =
      [
        base_url: base_url,
        headers: [
          {"authorization", "Bearer #{client.api_key}"},
          {"content-type", "application/json"},
          {"accept", "application/json"}
        ],
        receive_timeout: client.receive_timeout,
        decode_body: false
      ] ++ extra_opts

    Req.new(req_opts)
  end

  defp decode_response(body) when is_binary(body) do
    case Jason.decode(body) do
      {:ok, decoded} ->
        {:ok, decoded}

      {:error, reason} ->
        {:error, Error.serialization("failed to decode response: #{inspect(reason)}")}
    end
  end

  defp decode_response(body) when is_map(body) do
    # Req already decoded the JSON (shouldn't happen with decode_body: false, but be safe)
    {:ok, body}
  end

  defp delete(%__MODULE__{} = client, path, extra_opts) do
    req = build_req(client, extra_opts)

    do_with_retry(client.max_retries, fn ->
      case Req.delete(req, url: path) do
        {:ok, %{status: status, body: resp_body}} when status in 200..299 ->
          decode_response(resp_body)

        {:ok, %{status: status, body: resp_body}} ->
          message = Error.extract_message(resp_body)
          {:error, Error.from_http_status(status, message)}

        {:error, exception} ->
          {:error, Error.unknown("HTTP request failed: #{Exception.message(exception)}")}
      end
    end)
  end

  defp post_raw(%__MODULE__{} = client, path, body, extra_opts) do
    json_body =
      case Jason.encode(body) do
        {:ok, encoded} ->
          encoded

        {:error, reason} ->
          return = {:error, Error.serialization("failed to encode request: #{inspect(reason)}")}
          throw(return)
      end

    req = build_req(client, extra_opts)

    do_with_retry(client.max_retries, fn ->
      case Req.post(req, url: path, body: json_body) do
        {:ok, %{status: status, body: resp_body}} when status in 200..299 ->
          {:ok, resp_body}

        {:ok, %{status: status, body: resp_body}} ->
          message = Error.extract_message(resp_body)
          {:error, Error.from_http_status(status, message)}

        {:error, exception} ->
          {:error, Error.unknown("HTTP request failed: #{Exception.message(exception)}")}
      end
    end)
  catch
    {:error, _} = err -> err
  end

  defp get_raw(%__MODULE__{} = client, path, extra_opts) do
    req = build_req(client, extra_opts)

    do_with_retry(client.max_retries, fn ->
      case Req.get(req, url: path) do
        {:ok, %{status: status, body: resp_body}} when status in 200..299 ->
          {:ok, resp_body}

        {:ok, %{status: status, body: resp_body}} ->
          message = Error.extract_message(resp_body)
          {:error, Error.from_http_status(status, message)}

        {:error, exception} ->
          {:error, Error.unknown("HTTP request failed: #{Exception.message(exception)}")}
      end
    end)
  end

  defp build_query_path(base_path, nil), do: base_path

  defp build_query_path(base_path, query) when is_map(query) do
    params =
      query
      |> Enum.reject(fn {_k, v} -> is_nil(v) end)
      |> Enum.map(fn {k, v} -> "#{k}=#{URI.encode_www_form(to_string(v))}" end)
      |> Enum.join("&")

    if params == "" do
      base_path
    else
      "#{base_path}?#{params}"
    end
  end

  # Retries on rate_limit and provider_error kinds.
  defp do_with_retry(0, fun), do: fun.()

  defp do_with_retry(retries_left, fun) do
    case fun.() do
      {:error, %Error{kind: kind}} = err when kind in [:rate_limit, :provider_error] ->
        if retries_left > 0 do
          do_with_retry(retries_left - 1, fun)
        else
          err
        end

      other ->
        other
    end
  end
end
