defmodule LiterLlm do
  @moduledoc """
  Universal LLM API client for Elixir.

  Speaks the OpenAI-compatible HTTP wire protocol — no NIFs, no native libraries.
  The model-name prefix selects the provider and endpoint
  (e.g. `"groq/llama3-70b"` routes to Groq).

  ## Quick Start

      # Configure via application environment
      config :liter_llm,
        api_key: System.fetch_env!("OPENAI_API_KEY"),
        base_url: "https://api.openai.com/v1"

      # Or pass options directly per call
      {:ok, response} = LiterLlm.chat(
        %{
          model: "gpt-4o-mini",
          messages: [LiterLlm.Types.user_message("Hello!")],
          max_tokens: 256
        },
        api_key: System.fetch_env!("OPENAI_API_KEY")
      )

      hd(response["choices"])["message"]["content"]

  ## Configuration

  Options may be provided per-call or via application environment:

  | Key | Default | Description |
  |-----|---------|-------------|
  | `:api_key` | `""` | API key for `Authorization: Bearer` header |
  | `:base_url` | `"https://api.openai.com/v1"` | Provider base URL |
  | `:max_retries` | `2` | Retry count for 429 and 5xx errors |
  | `:receive_timeout` | `60_000` | Request timeout in milliseconds |

  ## Error Handling

  All functions return `{:ok, result}` or `{:error, %LiterLlm.Error{}}`.
  Match on `:kind` for programmatic handling:

      case LiterLlm.chat(request) do
        {:ok, response} -> process(response)
        {:error, %LiterLlm.Error{kind: :rate_limit}} -> retry()
        {:error, %LiterLlm.Error{kind: :authentication}} -> raise "Invalid API key"
        {:error, err} -> Logger.error("LLM error", error: inspect(err))
      end

  """

  alias LiterLlm.Client
  alias LiterLlm.Types

  # ─── Public API ───────────────────────────────────────────────────────────

  @doc """
  Sends a chat completion request and returns the full response.

  ## Parameters

  - `request` — a `LiterLlm.Types.chat_request()` map. Required keys: `:model`, `:messages`.
  - `opts` — client options (see module docs) or extra `Req` options.

  ## Returns

  `{:ok, map()}` where the map matches `LiterLlm.Types.chat_response()`, or
  `{:error, LiterLlm.Error.t()}`.

  ## Examples

      {:ok, response} = LiterLlm.chat(%{
        model: "gpt-4o-mini",
        messages: [
          LiterLlm.Types.system_message("You are a helpful assistant."),
          LiterLlm.Types.user_message("What is the capital of Germany?")
        ],
        max_tokens: 128
      })
      hd(response["choices"])["message"]["content"]
      #=> "The capital of Germany is Berlin."

  """
  @spec chat(Types.chat_request(), keyword()) ::
          {:ok, Types.chat_response()} | {:error, LiterLlm.Error.t()}
  def chat(request, opts \\ []) do
    {client_opts, req_opts} = split_opts(opts)
    client = build_client(client_opts)
    Client.chat(client, request, req_opts)
  end

  @doc """
  Sends an embedding request and returns the embedding response.

  ## Parameters

  - `request` — a `LiterLlm.Types.embedding_request()` map.
  - `opts` — client options or extra `Req` options.

  ## Returns

  `{:ok, map()}` where the map matches `LiterLlm.Types.embedding_response()`, or
  `{:error, LiterLlm.Error.t()}`.

  ## Examples

      {:ok, response} = LiterLlm.embed(%{
        model: "text-embedding-3-small",
        input: "The quick brown fox"
      })
      hd(response["data"])["embedding"] |> length()

  """
  @spec embed(Types.embedding_request(), keyword()) ::
          {:ok, Types.embedding_response()} | {:error, LiterLlm.Error.t()}
  def embed(request, opts \\ []) do
    {client_opts, req_opts} = split_opts(opts)
    client = build_client(client_opts)
    Client.embed(client, request, req_opts)
  end

  @doc """
  Lists available models for the configured provider.

  ## Parameters

  - `opts` — client options or extra `Req` options.

  ## Returns

  `{:ok, map()}` where the map matches `LiterLlm.Types.models_list_response()`, or
  `{:error, LiterLlm.Error.t()}`.

  ## Examples

      {:ok, response} = LiterLlm.list_models()
      Enum.map(response["data"], & &1["id"])

  """
  @spec list_models(keyword()) ::
          {:ok, Types.models_list_response()} | {:error, LiterLlm.Error.t()}
  def list_models(opts \\ []) do
    {client_opts, req_opts} = split_opts(opts)
    client = build_client(client_opts)
    Client.list_models(client, req_opts)
  end

  # ── Additional inference methods ─────────────────────────────────────────

  @doc """
  Generates an image from a text prompt.

  ## Parameters

  - `request` — a map with `:prompt`, `:model`, and optional params.
  - `opts` — client options or extra `Req` options.

  ## Returns

  `{:ok, map()}` or `{:error, LiterLlm.Error.t()}`.

  ## Examples

      {:ok, response} = LiterLlm.image_generate(%{
        model: "dall-e-3",
        prompt: "A sunset over mountains"
      })
      hd(response["data"])["url"]

  """
  @spec image_generate(map(), keyword()) ::
          {:ok, map()} | {:error, LiterLlm.Error.t()}
  def image_generate(request, opts \\ []) do
    {client_opts, req_opts} = split_opts(opts)
    client = build_client(client_opts)
    Client.image_generate(client, request, req_opts)
  end

  @doc """
  Generates speech audio from text.

  ## Parameters

  - `request` — a map with `:model`, `:input`, `:voice`, and optional params.
  - `opts` — client options or extra `Req` options.

  ## Returns

  `{:ok, binary()}` containing the raw audio bytes, or `{:error, LiterLlm.Error.t()}`.

  ## Examples

      {:ok, audio_bytes} = LiterLlm.speech(%{
        model: "tts-1",
        input: "Hello, world!",
        voice: "alloy"
      })

  """
  @spec speech(map(), keyword()) ::
          {:ok, binary()} | {:error, LiterLlm.Error.t()}
  def speech(request, opts \\ []) do
    {client_opts, req_opts} = split_opts(opts)
    client = build_client(client_opts)
    Client.speech(client, request, req_opts)
  end

  @doc """
  Transcribes audio to text.

  ## Parameters

  - `request` — a map with `:model`, `:file`, and optional params.
  - `opts` — client options or extra `Req` options.

  ## Returns

  `{:ok, map()}` or `{:error, LiterLlm.Error.t()}`.

  """
  @spec transcribe(map(), keyword()) ::
          {:ok, map()} | {:error, LiterLlm.Error.t()}
  def transcribe(request, opts \\ []) do
    {client_opts, req_opts} = split_opts(opts)
    client = build_client(client_opts)
    Client.transcribe(client, request, req_opts)
  end

  @doc """
  Checks content against moderation policies.

  ## Parameters

  - `request` — a map with `:input` and optional `:model`.
  - `opts` — client options or extra `Req` options.

  ## Returns

  `{:ok, map()}` or `{:error, LiterLlm.Error.t()}`.

  """
  @spec moderate(map(), keyword()) ::
          {:ok, map()} | {:error, LiterLlm.Error.t()}
  def moderate(request, opts \\ []) do
    {client_opts, req_opts} = split_opts(opts)
    client = build_client(client_opts)
    Client.moderate(client, request, req_opts)
  end

  @doc """
  Reranks documents by relevance to a query.

  ## Parameters

  - `request` — a map with `:model`, `:query`, `:documents`.
  - `opts` — client options or extra `Req` options.

  ## Returns

  `{:ok, map()}` or `{:error, LiterLlm.Error.t()}`.

  """
  @spec rerank(map(), keyword()) ::
          {:ok, map()} | {:error, LiterLlm.Error.t()}
  def rerank(request, opts \\ []) do
    {client_opts, req_opts} = split_opts(opts)
    client = build_client(client_opts)
    Client.rerank(client, request, req_opts)
  end

  # ── File management ────────────────────────────────────────────────────

  @doc """
  Uploads a file.

  ## Parameters

  - `request` — a map with `:file`, `:purpose`, and optional `:filename`.
  - `opts` — client options or extra `Req` options.

  ## Returns

  `{:ok, map()}` with file object fields, or `{:error, LiterLlm.Error.t()}`.

  """
  @spec create_file(map(), keyword()) ::
          {:ok, map()} | {:error, LiterLlm.Error.t()}
  def create_file(request, opts \\ []) do
    {client_opts, req_opts} = split_opts(opts)
    client = build_client(client_opts)
    Client.create_file(client, request, req_opts)
  end

  @doc """
  Retrieves metadata for a file by ID.

  ## Parameters

  - `file_id` — the file ID string.
  - `opts` — client options or extra `Req` options.

  ## Returns

  `{:ok, map()}` with file object fields, or `{:error, LiterLlm.Error.t()}`.

  """
  @spec retrieve_file(String.t(), keyword()) ::
          {:ok, map()} | {:error, LiterLlm.Error.t()}
  def retrieve_file(file_id, opts \\ []) do
    {client_opts, req_opts} = split_opts(opts)
    client = build_client(client_opts)
    Client.retrieve_file(client, file_id, req_opts)
  end

  @doc """
  Deletes a file by ID.

  ## Parameters

  - `file_id` — the file ID string.
  - `opts` — client options or extra `Req` options.

  ## Returns

  `{:ok, map()}` with deletion status, or `{:error, LiterLlm.Error.t()}`.

  """
  @spec delete_file(String.t(), keyword()) ::
          {:ok, map()} | {:error, LiterLlm.Error.t()}
  def delete_file(file_id, opts \\ []) do
    {client_opts, req_opts} = split_opts(opts)
    client = build_client(client_opts)
    Client.delete_file(client, file_id, req_opts)
  end

  @doc """
  Lists files, optionally filtered by query parameters.

  ## Parameters

  - `query` — optional map of query parameters (`:purpose`, `:limit`, `:after`).
  - `opts` — client options or extra `Req` options.

  ## Returns

  `{:ok, map()}` with a `"data"` list, or `{:error, LiterLlm.Error.t()}`.

  """
  @spec list_files(map() | nil, keyword()) ::
          {:ok, map()} | {:error, LiterLlm.Error.t()}
  def list_files(query \\ nil, opts \\ []) do
    {client_opts, req_opts} = split_opts(opts)
    client = build_client(client_opts)
    Client.list_files(client, query, req_opts)
  end

  @doc """
  Retrieves the raw content of a file.

  ## Parameters

  - `file_id` — the file ID string.
  - `opts` — client options or extra `Req` options.

  ## Returns

  `{:ok, binary()}` containing the raw file bytes, or `{:error, LiterLlm.Error.t()}`.

  """
  @spec file_content(String.t(), keyword()) ::
          {:ok, binary()} | {:error, LiterLlm.Error.t()}
  def file_content(file_id, opts \\ []) do
    {client_opts, req_opts} = split_opts(opts)
    client = build_client(client_opts)
    Client.file_content(client, file_id, req_opts)
  end

  # ── Batch management ───────────────────────────────────────────────────

  @doc """
  Creates a new batch job.

  ## Parameters

  - `request` — a map with `:input_file_id`, `:endpoint`, `:completion_window`.
  - `opts` — client options or extra `Req` options.

  ## Returns

  `{:ok, map()}` with batch object fields, or `{:error, LiterLlm.Error.t()}`.

  """
  @spec create_batch(map(), keyword()) ::
          {:ok, map()} | {:error, LiterLlm.Error.t()}
  def create_batch(request, opts \\ []) do
    {client_opts, req_opts} = split_opts(opts)
    client = build_client(client_opts)
    Client.create_batch(client, request, req_opts)
  end

  @doc """
  Retrieves a batch by ID.

  ## Parameters

  - `batch_id` — the batch ID string.
  - `opts` — client options or extra `Req` options.

  ## Returns

  `{:ok, map()}` with batch object fields, or `{:error, LiterLlm.Error.t()}`.

  """
  @spec retrieve_batch(String.t(), keyword()) ::
          {:ok, map()} | {:error, LiterLlm.Error.t()}
  def retrieve_batch(batch_id, opts \\ []) do
    {client_opts, req_opts} = split_opts(opts)
    client = build_client(client_opts)
    Client.retrieve_batch(client, batch_id, req_opts)
  end

  @doc """
  Lists batches, optionally filtered by query parameters.

  ## Parameters

  - `query` — optional map of query parameters (`:limit`, `:after`).
  - `opts` — client options or extra `Req` options.

  ## Returns

  `{:ok, map()}` with a `"data"` list, or `{:error, LiterLlm.Error.t()}`.

  """
  @spec list_batches(map() | nil, keyword()) ::
          {:ok, map()} | {:error, LiterLlm.Error.t()}
  def list_batches(query \\ nil, opts \\ []) do
    {client_opts, req_opts} = split_opts(opts)
    client = build_client(client_opts)
    Client.list_batches(client, query, req_opts)
  end

  @doc """
  Cancels an in-progress batch.

  ## Parameters

  - `batch_id` — the batch ID string.
  - `opts` — client options or extra `Req` options.

  ## Returns

  `{:ok, map()}` with batch object fields, or `{:error, LiterLlm.Error.t()}`.

  """
  @spec cancel_batch(String.t(), keyword()) ::
          {:ok, map()} | {:error, LiterLlm.Error.t()}
  def cancel_batch(batch_id, opts \\ []) do
    {client_opts, req_opts} = split_opts(opts)
    client = build_client(client_opts)
    Client.cancel_batch(client, batch_id, req_opts)
  end

  # ── Response management ────────────────────────────────────────────────

  @doc """
  Creates a new response.

  ## Parameters

  - `request` — a map with `:model`, `:input`, and optional params.
  - `opts` — client options or extra `Req` options.

  ## Returns

  `{:ok, map()}` with response object fields, or `{:error, LiterLlm.Error.t()}`.

  """
  @spec create_response(map(), keyword()) ::
          {:ok, map()} | {:error, LiterLlm.Error.t()}
  def create_response(request, opts \\ []) do
    {client_opts, req_opts} = split_opts(opts)
    client = build_client(client_opts)
    Client.create_response(client, request, req_opts)
  end

  @doc """
  Retrieves a response by ID.

  ## Parameters

  - `response_id` — the response ID string.
  - `opts` — client options or extra `Req` options.

  ## Returns

  `{:ok, map()}` with response object fields, or `{:error, LiterLlm.Error.t()}`.

  """
  @spec retrieve_response(String.t(), keyword()) ::
          {:ok, map()} | {:error, LiterLlm.Error.t()}
  def retrieve_response(response_id, opts \\ []) do
    {client_opts, req_opts} = split_opts(opts)
    client = build_client(client_opts)
    Client.retrieve_response(client, response_id, req_opts)
  end

  @doc """
  Cancels an in-progress response.

  ## Parameters

  - `response_id` — the response ID string.
  - `opts` — client options or extra `Req` options.

  ## Returns

  `{:ok, map()}` with response object fields, or `{:error, LiterLlm.Error.t()}`.

  """
  @spec cancel_response(String.t(), keyword()) ::
          {:ok, map()} | {:error, LiterLlm.Error.t()}
  def cancel_response(response_id, opts \\ []) do
    {client_opts, req_opts} = split_opts(opts)
    client = build_client(client_opts)
    Client.cancel_response(client, response_id, req_opts)
  end

  # ─── Hooks & Custom Providers ─────────────────────────────────────────────

  @doc """
  Registers a lifecycle hook module on the default client configuration.

  See `LiterLlm.Client.add_hook/2` for details.
  """
  @spec add_hook(Client.t(), module()) :: Client.t()
  defdelegate add_hook(client, hook_module), to: Client

  @doc """
  Registers a custom provider configuration on a client.

  See `LiterLlm.Client.register_provider/2` for details.
  """
  @spec register_provider(Client.t(), Types.provider_config()) :: Client.t()
  defdelegate register_provider(client, provider), to: Client

  # ─── Helpers ──────────────────────────────────────────────────────────────

  # Known client config keys — all others are forwarded to Req.
  @client_keys [:api_key, :base_url, :max_retries, :receive_timeout]

  defp split_opts(opts) do
    Enum.split_with(opts, fn {key, _value} -> key in @client_keys end)
    |> then(fn {client, req} -> {client, req} end)
  end

  defp build_client(opts) do
    app_config = Application.get_all_env(:liter_llm)
    merged = Keyword.merge(app_config, opts)
    Client.new(merged)
  end
end
