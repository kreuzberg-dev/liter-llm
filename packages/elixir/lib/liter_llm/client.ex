defmodule LiterLlm.Client do
  @moduledoc """
  LLM client implementation backed by the Rust NIF (`LiterLlm.Native`).

  The client serializes Elixir maps to JSON, delegates to the Rust core via
  `LiterLlm.Native`, and deserializes the JSON response back to Elixir maps.

  Use this module directly for advanced configuration, or use the top-level
  `LiterLlm` module for the standard interface.

  ## Configuration

  | Option | Default | Description |
  |--------|---------|-------------|
  | `:api_key` | `""` | API key for `Authorization: Bearer` header |
  | `:base_url` | `"https://api.openai.com/v1"` | Provider base URL |
  | `:max_retries` | `3` | Retry count for 429/5xx errors |
  | `:receive_timeout` | `60_000` | Request timeout in milliseconds |

  ## Example

      client = LiterLlm.Client.new(api_key: System.fetch_env!("OPENAI_API_KEY"))
      {:ok, response} = LiterLlm.Client.chat(client, %{
        model: "gpt-4o-mini",
        messages: [%{role: "user", content: "Hello!"}],
        max_tokens: 256
      })

  """

  alias LiterLlm.Budget
  alias LiterLlm.Cache
  alias LiterLlm.Error
  alias LiterLlm.Native
  alias LiterLlm.Types

  @default_base_url "https://api.openai.com/v1"
  @default_max_retries 3
  @default_timeout_ms 60_000

  @typedoc "An opaque client configuration map."
  @type t :: %__MODULE__{
          api_key: String.t(),
          base_url: String.t(),
          max_retries: non_neg_integer(),
          receive_timeout: pos_integer(),
          cache: Types.cache_config() | nil,
          budget: Types.budget_config() | nil,
          hooks: [module()],
          custom_providers: [Types.provider_config()]
        }

  defstruct api_key: "",
            base_url: @default_base_url,
            max_retries: @default_max_retries,
            receive_timeout: @default_timeout_ms,
            cache: nil,
            budget: nil,
            hooks: [],
            custom_providers: []

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
    client = struct!(__MODULE__, opts)

    if client.cache, do: Cache.init(client.cache)
    if client.budget, do: Budget.init(client.budget)

    client
  end

  # ─── Public API ───────────────────────────────────────────────────────────

  @doc """
  Sends a chat completion request and returns the full response.

  ## Parameters

  - `client` — client configuration from `new/1`
  - `request` — a `LiterLlm.Types.chat_request()` map
  - `opts` — reserved for future use

  ## Returns

  - `{:ok, map()}` on success
  - `{:error, LiterLlm.Error.t()}` on failure

  """
  @spec chat(t(), Types.chat_request(), keyword()) ::
          {:ok, Types.chat_response()} | {:error, Error.t()}
  def chat(%__MODULE__{} = client, request, _opts \\ []) do
    with :ok <- run_hooks(client, :on_request, request),
         :ok <- check_budget(client, request[:model]) do
      case check_cache(client, request) do
        {:ok, _cached} = hit ->
          run_hooks(client, :on_response, {request, hit})
          hit

        :miss ->
          case call_nif(:chat, client, request) do
            {:ok, response} = ok ->
              store_cache(client, request, response)
              record_usage(client, request[:model], response)
              run_hooks(client, :on_response, {request, response})
              ok

            {:error, _} = err ->
              run_hooks(client, :on_error, {request, err})
              err
          end
      end
    end
  end

  @doc "Sends an embedding request."
  @spec embed(t(), Types.embedding_request(), keyword()) ::
          {:ok, Types.embedding_response()} | {:error, Error.t()}
  def embed(%__MODULE__{} = client, request, _opts \\ []) do
    with :ok <- run_hooks(client, :on_request, request),
         :ok <- check_budget(client, request[:model]) do
      case check_cache(client, request) do
        {:ok, _cached} = hit ->
          run_hooks(client, :on_response, {request, hit})
          hit

        :miss ->
          case call_nif(:embed, client, request) do
            {:ok, response} = ok ->
              store_cache(client, request, response)
              record_usage(client, request[:model], response)
              run_hooks(client, :on_response, {request, response})
              ok

            {:error, _} = err ->
              run_hooks(client, :on_error, {request, err})
              err
          end
      end
    end
  end

  @doc "Lists available models for the configured provider."
  @spec list_models(t(), keyword()) ::
          {:ok, Types.models_list_response()} | {:error, Error.t()}
  def list_models(%__MODULE__{} = client, _opts \\ []) do
    with :ok <- run_hooks(client, :on_request, %{action: :list_models}) do
      result =
        with {:ok, config_json} <- encode(client_to_config_map(client)),
             {:ok, resp_json} <- Native.list_models(config_json) do
          decode(resp_json)
        else
          {:error, reason} -> wrap_error(reason)
        end

      case result do
        {:ok, response} ->
          run_hooks(client, :on_response, {%{action: :list_models}, response})
          result

        {:error, _} = err ->
          run_hooks(client, :on_error, {%{action: :list_models}, err})
          err
      end
    end
  end

  @doc "Generates an image from a text prompt."
  @spec image_generate(t(), map(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def image_generate(%__MODULE__{} = client, request, _opts \\ []) do
    with :ok <- run_hooks(client, :on_request, request),
         :ok <- check_budget(client, request[:model]) do
      case call_nif(:image_generate, client, request) do
        {:ok, response} = ok ->
          record_usage(client, request[:model], response)
          run_hooks(client, :on_response, {request, response})
          ok

        {:error, _} = err ->
          run_hooks(client, :on_error, {request, err})
          err
      end
    end
  end

  @doc "Generates speech audio from text. Returns raw audio bytes."
  @spec speech(t(), map(), keyword()) ::
          {:ok, binary()} | {:error, Error.t()}
  def speech(%__MODULE__{} = client, request, _opts \\ []) do
    with :ok <- run_hooks(client, :on_request, request),
         :ok <- check_budget(client, request[:model]) do
      case call_nif_raw(:speech, client, request) do
        {:ok, _} = ok ->
          run_hooks(client, :on_response, {request, ok})
          ok

        {:error, _} = err ->
          run_hooks(client, :on_error, {request, err})
          err
      end
    end
  end

  @doc "Transcribes audio to text."
  @spec transcribe(t(), map(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def transcribe(%__MODULE__{} = client, request, _opts \\ []) do
    with :ok <- run_hooks(client, :on_request, request),
         :ok <- check_budget(client, request[:model]) do
      case call_nif(:transcribe, client, request) do
        {:ok, response} = ok ->
          record_usage(client, request[:model], response)
          run_hooks(client, :on_response, {request, response})
          ok

        {:error, _} = err ->
          run_hooks(client, :on_error, {request, err})
          err
      end
    end
  end

  @doc "Checks content against moderation policies."
  @spec moderate(t(), map(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def moderate(%__MODULE__{} = client, request, _opts \\ []) do
    with :ok <- run_hooks(client, :on_request, request),
         :ok <- check_budget(client, request[:model]) do
      case call_nif(:moderate, client, request) do
        {:ok, response} = ok ->
          run_hooks(client, :on_response, {request, response})
          ok

        {:error, _} = err ->
          run_hooks(client, :on_error, {request, err})
          err
      end
    end
  end

  @doc "Reranks documents by relevance to a query."
  @spec rerank(t(), map(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def rerank(%__MODULE__{} = client, request, _opts \\ []) do
    with :ok <- run_hooks(client, :on_request, request),
         :ok <- check_budget(client, request[:model]) do
      case call_nif(:rerank, client, request) do
        {:ok, response} = ok ->
          record_usage(client, request[:model], response)
          run_hooks(client, :on_response, {request, response})
          ok

        {:error, _} = err ->
          run_hooks(client, :on_error, {request, err})
          err
      end
    end
  end

  # ── File management methods ──────────────────────────────────────────────

  @doc "Uploads a file."
  @spec create_file(t(), map(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def create_file(%__MODULE__{} = client, request, _opts \\ []) do
    with :ok <- run_hooks(client, :on_request, request) do
      case call_nif(:create_file, client, request) do
        {:ok, response} = ok ->
          run_hooks(client, :on_response, {request, response})
          ok

        {:error, _} = err ->
          run_hooks(client, :on_error, {request, err})
          err
      end
    end
  end

  @doc "Retrieves metadata for a file."
  @spec retrieve_file(t(), String.t(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def retrieve_file(%__MODULE__{} = client, file_id, _opts \\ []) do
    req = %{action: :retrieve_file, file_id: file_id}

    with :ok <- run_hooks(client, :on_request, req) do
      case call_nif_id(:retrieve_file, client, file_id) do
        {:ok, response} = ok ->
          run_hooks(client, :on_response, {req, response})
          ok

        {:error, _} = err ->
          run_hooks(client, :on_error, {req, err})
          err
      end
    end
  end

  @doc "Deletes a file."
  @spec delete_file(t(), String.t(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def delete_file(%__MODULE__{} = client, file_id, _opts \\ []) do
    req = %{action: :delete_file, file_id: file_id}

    with :ok <- run_hooks(client, :on_request, req) do
      case call_nif_id(:delete_file, client, file_id) do
        {:ok, response} = ok ->
          run_hooks(client, :on_response, {req, response})
          ok

        {:error, _} = err ->
          run_hooks(client, :on_error, {req, err})
          err
      end
    end
  end

  @doc "Lists files, optionally filtered by query parameters."
  @spec list_files(t(), map() | nil, keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def list_files(%__MODULE__{} = client, query \\ nil, _opts \\ []) do
    req = %{action: :list_files, query: query}

    with :ok <- run_hooks(client, :on_request, req) do
      case call_nif_query(:list_files, client, query) do
        {:ok, response} = ok ->
          run_hooks(client, :on_response, {req, response})
          ok

        {:error, _} = err ->
          run_hooks(client, :on_error, {req, err})
          err
      end
    end
  end

  @doc "Retrieves the raw content of a file."
  @spec file_content(t(), String.t(), keyword()) ::
          {:ok, binary()} | {:error, Error.t()}
  def file_content(%__MODULE__{} = client, file_id, _opts \\ []) do
    req = %{action: :file_content, file_id: file_id}

    with :ok <- run_hooks(client, :on_request, req) do
      result =
        with {:ok, config_json} <- encode(client_to_config_map(client)),
             {:ok, bytes} <- Native.file_content(config_json, file_id) do
          {:ok, bytes}
        else
          {:error, reason} -> wrap_error(reason)
        end

      case result do
        {:ok, _} ->
          run_hooks(client, :on_response, {req, result})
          result

        {:error, _} = err ->
          run_hooks(client, :on_error, {req, err})
          err
      end
    end
  end

  # ── Batch management methods ─────────────────────────────────────────────

  @doc "Creates a new batch job."
  @spec create_batch(t(), map(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def create_batch(%__MODULE__{} = client, request, _opts \\ []) do
    with :ok <- run_hooks(client, :on_request, request) do
      case call_nif(:create_batch, client, request) do
        {:ok, response} = ok ->
          run_hooks(client, :on_response, {request, response})
          ok

        {:error, _} = err ->
          run_hooks(client, :on_error, {request, err})
          err
      end
    end
  end

  @doc "Retrieves a batch by ID."
  @spec retrieve_batch(t(), String.t(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def retrieve_batch(%__MODULE__{} = client, batch_id, _opts \\ []) do
    req = %{action: :retrieve_batch, batch_id: batch_id}

    with :ok <- run_hooks(client, :on_request, req) do
      case call_nif_id(:retrieve_batch, client, batch_id) do
        {:ok, response} = ok ->
          run_hooks(client, :on_response, {req, response})
          ok

        {:error, _} = err ->
          run_hooks(client, :on_error, {req, err})
          err
      end
    end
  end

  @doc "Lists batches, optionally filtered by query parameters."
  @spec list_batches(t(), map() | nil, keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def list_batches(%__MODULE__{} = client, query \\ nil, _opts \\ []) do
    req = %{action: :list_batches, query: query}

    with :ok <- run_hooks(client, :on_request, req) do
      case call_nif_query(:list_batches, client, query) do
        {:ok, response} = ok ->
          run_hooks(client, :on_response, {req, response})
          ok

        {:error, _} = err ->
          run_hooks(client, :on_error, {req, err})
          err
      end
    end
  end

  @doc "Cancels an in-progress batch."
  @spec cancel_batch(t(), String.t(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def cancel_batch(%__MODULE__{} = client, batch_id, _opts \\ []) do
    req = %{action: :cancel_batch, batch_id: batch_id}

    with :ok <- run_hooks(client, :on_request, req) do
      case call_nif_id(:cancel_batch, client, batch_id) do
        {:ok, response} = ok ->
          run_hooks(client, :on_response, {req, response})
          ok

        {:error, _} = err ->
          run_hooks(client, :on_error, {req, err})
          err
      end
    end
  end

  # ── Response management methods ──────────────────────────────────────────

  @doc "Creates a new response."
  @spec create_response(t(), map(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def create_response(%__MODULE__{} = client, request, _opts \\ []) do
    with :ok <- run_hooks(client, :on_request, request) do
      case call_nif(:create_response, client, request) do
        {:ok, response} = ok ->
          run_hooks(client, :on_response, {request, response})
          ok

        {:error, _} = err ->
          run_hooks(client, :on_error, {request, err})
          err
      end
    end
  end

  @doc "Retrieves a response by ID."
  @spec retrieve_response(t(), String.t(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def retrieve_response(%__MODULE__{} = client, response_id, _opts \\ []) do
    req = %{action: :retrieve_response, response_id: response_id}

    with :ok <- run_hooks(client, :on_request, req) do
      case call_nif_id(:retrieve_response, client, response_id) do
        {:ok, response} = ok ->
          run_hooks(client, :on_response, {req, response})
          ok

        {:error, _} = err ->
          run_hooks(client, :on_error, {req, err})
          err
      end
    end
  end

  @doc "Cancels an in-progress response."
  @spec cancel_response(t(), String.t(), keyword()) ::
          {:ok, map()} | {:error, Error.t()}
  def cancel_response(%__MODULE__{} = client, response_id, _opts \\ []) do
    req = %{action: :cancel_response, response_id: response_id}

    with :ok <- run_hooks(client, :on_request, req) do
      case call_nif_id(:cancel_response, client, response_id) do
        {:ok, response} = ok ->
          run_hooks(client, :on_response, {req, response})
          ok

        {:error, _} = err ->
          run_hooks(client, :on_error, {req, err})
          err
      end
    end
  end

  # ─── Hooks & Custom Providers ─────────────────────────────────────────────

  @doc """
  Registers a lifecycle hook module.

  The module must implement the `LiterLlm.Hook` behaviour.
  Hooks are invoked in registration order.

  ## Examples

      client = LiterLlm.Client.new(api_key: "sk-...")
      client = LiterLlm.Client.add_hook(client, MyApp.LoggingHook)

  """
  @spec add_hook(t(), module()) :: t()
  def add_hook(%__MODULE__{} = client, hook_module) when is_atom(hook_module) do
    %{client | hooks: client.hooks ++ [hook_module]}
  end

  @doc """
  Registers a custom provider configuration.

  Requests whose model name starts with one of the provider's prefixes
  are routed to its base URL.

  ## Examples

      provider = %{
        name: "my-provider",
        base_url: "https://api.myprovider.com/v1",
        auth_header: "Authorization",
        model_prefixes: ["myprovider/"]
      }
      client = LiterLlm.Client.register_provider(client, provider)

  """
  @spec register_provider(t(), Types.provider_config()) :: t()
  def register_provider(%__MODULE__{} = client, provider) when is_map(provider) do
    %{client | custom_providers: client.custom_providers ++ [provider]}
  end

  # ─── Private helpers ──────────────────────────────────────────────────────

  # Invoke all registered hook modules for the given event.
  #
  # Each hook module is expected to implement the `LiterLlm.Hook` behaviour.
  # If a hook module does not export the requested callback, it is silently
  # skipped.
  #
  # For `:on_request`, a hook may return `{:error, reason}` to reject the
  # request -- this causes the entire pipeline to short-circuit with a
  # `:hook_rejected` error.  For `:on_response` and `:on_error`, return
  # values are ignored (advisory only).
  defp run_hooks(%__MODULE__{hooks: hooks}, :on_request, payload) do
    Enum.reduce_while(hooks, :ok, fn hook_module, :ok ->
      arity = if function_exported?(hook_module, :on_request, 1), do: 1, else: nil

      case arity do
        nil ->
          {:cont, :ok}

        1 ->
          try do
            case apply(hook_module, :on_request, [payload]) do
              :ok -> {:cont, :ok}
              {:error, reason} -> {:halt, {:error, Error.hook_rejected(inspect(reason))}}
              _other -> {:cont, :ok}
            end
          rescue
            e -> {:halt, {:error, Error.hook_rejected(Exception.message(e))}}
          end
      end
    end)
  end

  defp run_hooks(%__MODULE__{hooks: hooks}, event, payload) when event in [:on_response, :on_error] do
    arity =
      case event do
        :on_response -> 2
        :on_error -> 2
      end

    Enum.each(hooks, fn hook_module ->
      if function_exported?(hook_module, event, arity) do
        try do
          {req, resp_or_err} = payload
          apply(hook_module, event, [req, resp_or_err])
        rescue
          _ -> :ok
        end
      end
    end)

    :ok
  end

  defp client_to_config_map(%__MODULE__{} = client) do
    %{
      api_key: client.api_key,
      base_url: client.base_url,
      max_retries: client.max_retries,
      timeout_secs: div(client.receive_timeout, 1_000)
    }
  end

  # Encode a value to a JSON string; returns {:error, Error.t()} on failure.
  defp encode(value) do
    case Jason.encode(value) do
      {:ok, json} -> {:ok, json}
      {:error, reason} -> {:error, Error.serialization("failed to encode: #{inspect(reason)}")}
    end
  end

  # Decode a JSON string; returns {:error, Error.t()} on failure.
  defp decode(json) when is_binary(json) do
    case Jason.decode(json) do
      {:ok, value} ->
        {:ok, value}

      {:error, reason} ->
        {:error, Error.serialization("failed to decode response: #{inspect(reason)}")}
    end
  end

  # Wrap a NIF error string into a structured LiterLlm.Error.
  defp wrap_error(reason) when is_binary(reason), do: {:error, Error.unknown(reason)}
  defp wrap_error(reason), do: {:error, Error.unknown(inspect(reason))}

  # Call a NIF function that takes (config_json, request_json) and returns a JSON response.
  defp call_nif(fun, client, request) do
    with {:ok, config_json} <- encode(client_to_config_map(client)),
         {:ok, request_json} <- encode(request),
         {:ok, resp_json} <- apply(Native, fun, [config_json, request_json]) do
      decode(resp_json)
    else
      {:error, reason} -> wrap_error(reason)
    end
  end

  # Call a NIF that returns raw bytes (no JSON decode on the response).
  defp call_nif_raw(fun, client, request) do
    with {:ok, config_json} <- encode(client_to_config_map(client)),
         {:ok, request_json} <- encode(request),
         {:ok, bytes} <- apply(Native, fun, [config_json, request_json]) do
      {:ok, bytes}
    else
      {:error, reason} -> wrap_error(reason)
    end
  end

  # Call a NIF that takes (config_json, id_string) and returns a JSON response.
  defp call_nif_id(fun, client, id) do
    with {:ok, config_json} <- encode(client_to_config_map(client)),
         {:ok, resp_json} <- apply(Native, fun, [config_json, id]) do
      decode(resp_json)
    else
      {:error, reason} -> wrap_error(reason)
    end
  end

  # Call a NIF that takes (config_json, query_json) where query may be nil.
  defp call_nif_query(fun, client, query) do
    with {:ok, config_json} <- encode(client_to_config_map(client)),
         query_json = if(is_nil(query), do: "null", else: Jason.encode!(query)),
         {:ok, resp_json} <- apply(Native, fun, [config_json, query_json]) do
      decode(resp_json)
    else
      {:error, reason} -> wrap_error(reason)
    end
  end

  # ─── Cache helpers ───────────────────────────────────────────────────────

  defp check_cache(%__MODULE__{cache: nil}, _request), do: :miss

  defp check_cache(%__MODULE__{cache: config}, request) do
    ttl = Map.get(config, :ttl_seconds, 300)
    key = Cache.cache_key(request)
    Cache.get(key, ttl)
  end

  defp store_cache(%__MODULE__{cache: nil}, _request, _response), do: :ok

  defp store_cache(%__MODULE__{cache: config}, request, response) do
    max_entries = Map.get(config, :max_entries, 256)
    key = Cache.cache_key(request)
    Cache.put(key, response, max_entries)
  end

  # ─── Budget helpers ──────────────────────────────────────────────────────

  defp check_budget(%__MODULE__{budget: nil}, _model), do: :ok
  defp check_budget(%__MODULE__{budget: config}, model), do: Budget.check(config, model)

  # Extract usage-based cost from a response and record it.
  # Uses total_tokens as a rough cost proxy ($0.001 per 1K tokens).
  defp record_usage(%__MODULE__{budget: nil}, _model, _response), do: :ok

  defp record_usage(%__MODULE__{budget: _config}, model, response) when is_map(response) do
    usage = response["usage"] || response[:usage]

    if usage do
      total_tokens = usage["total_tokens"] || usage[:total_tokens] || 0
      # Approximate cost: $0.001 per 1K tokens (configurable in a future iteration)
      cost = total_tokens / 1_000.0 * 0.001
      Budget.record(model, cost)
    else
      :ok
    end
  end

  defp record_usage(_client, _model, _response), do: :ok
end
