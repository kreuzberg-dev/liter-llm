defmodule LiterLlm.Hook do
  @moduledoc """
  Behaviour for intercepting LLM client request/response lifecycle events.

  Implement this behaviour and register hooks via `LiterLlm.Client.add_hook/2`.
  Hooks are invoked in registration order, synchronously.

  ## Example

      defmodule MyApp.LoggingHook do
        @behaviour LiterLlm.Hook

        @impl true
        def on_request(request) do
          Logger.debug("LLM request", model: request[:model])
          :ok
        end

        @impl true
        def on_response(request, response) do
          Logger.info("LLM response", model: request[:model])
          :ok
        end

        @impl true
        def on_error(request, error) do
          Logger.error("LLM error", model: request[:model], error: inspect(error))
          :ok
        end
      end

      client = LiterLlm.Client.new(api_key: "sk-...")
      client = LiterLlm.Client.add_hook(client, MyApp.LoggingHook)

  """

  @doc """
  Called before the HTTP request is sent.

  Return `:ok` to proceed, or `{:error, reason}` to reject the request.
  A rejected request returns `{:error, %LiterLlm.Error{kind: :hook_rejected}}`.
  """
  @callback on_request(request :: map()) :: :ok | {:error, term()}

  @doc """
  Called after a successful response is received.
  """
  @callback on_response(request :: map(), response :: map()) :: :ok

  @doc """
  Called after a request fails.
  """
  @callback on_error(request :: map(), error :: LiterLlm.Error.t()) :: :ok
end
