defmodule LiterLlm.Native do
  @moduledoc """
  Rustler NIF bindings for liter-llm.

  This module loads the precompiled Rust NIF (`liter_llm_rustler`) and
  exposes raw NIF functions.  Each function accepts JSON strings and returns
  JSON strings; use `LiterLlm.Client` for the idiomatic Elixir interface.

  ## Building from source

  Set the environment variable `LITER_LLM_BUILD=1` to compile the NIF from
  source instead of downloading a precompiled binary:

      LITER_LLM_BUILD=1 mix compile

  This requires a Rust toolchain (Cargo ≥ 1.75) on the build machine.

  ## Precompiled targets

  | Platform | Target triple |
  |----------|--------------|
  | macOS (Apple Silicon) | `aarch64-apple-darwin` |
  | Linux x86_64 | `x86_64-unknown-linux-gnu` |
  | Linux aarch64 | `aarch64-unknown-linux-gnu` |
  """

  use RustlerPrecompiled,
    otp_app: :liter_llm,
    crate: "liter_llm_rustler",
    base_url: "https://github.com/kreuzberg-dev/liter-llm/releases/download",
    version: "1.0.0-rc.1",
    force_build: System.get_env("LITER_LLM_BUILD") in ["1", "true"],
    targets: [
      "aarch64-apple-darwin",
      "aarch64-unknown-linux-gnu",
      "x86_64-unknown-linux-gnu"
    ],
    nif_versions: ["2.16", "2.17"]

  # ── Core inference ────────────────────────────────────────────────────────

  @doc """
  Send a chat completion request.

  - `config_json` — JSON string: `{"api_key":"...", "base_url":"...", "max_retries":3}`
  - `request_json` — JSON string matching the OpenAI chat completion request shape

  Returns `{:ok, response_json}` or `{:error, message}`.
  """
  @spec chat(String.t(), String.t()) :: {:ok, String.t()} | {:error, String.t()}
  def chat(_config_json, _request_json), do: :erlang.nif_error(:nif_not_loaded)

  @doc "Send an embedding request."
  @spec embed(String.t(), String.t()) :: {:ok, String.t()} | {:error, String.t()}
  def embed(_config_json, _request_json), do: :erlang.nif_error(:nif_not_loaded)

  @doc "List available models."
  @spec list_models(String.t()) :: {:ok, String.t()} | {:error, String.t()}
  def list_models(_config_json), do: :erlang.nif_error(:nif_not_loaded)

  @doc "Generate an image from a text prompt."
  @spec image_generate(String.t(), String.t()) :: {:ok, String.t()} | {:error, String.t()}
  def image_generate(_config_json, _request_json), do: :erlang.nif_error(:nif_not_loaded)

  @doc "Generate speech audio from text. Returns raw audio bytes."
  @spec speech(String.t(), String.t()) :: {:ok, binary()} | {:error, String.t()}
  def speech(_config_json, _request_json), do: :erlang.nif_error(:nif_not_loaded)

  @doc "Transcribe audio to text."
  @spec transcribe(String.t(), String.t()) :: {:ok, String.t()} | {:error, String.t()}
  def transcribe(_config_json, _request_json), do: :erlang.nif_error(:nif_not_loaded)

  @doc "Check content against moderation policies."
  @spec moderate(String.t(), String.t()) :: {:ok, String.t()} | {:error, String.t()}
  def moderate(_config_json, _request_json), do: :erlang.nif_error(:nif_not_loaded)

  @doc "Rerank documents by relevance to a query."
  @spec rerank(String.t(), String.t()) :: {:ok, String.t()} | {:error, String.t()}
  def rerank(_config_json, _request_json), do: :erlang.nif_error(:nif_not_loaded)

  # ── File management ───────────────────────────────────────────────────────

  @doc "Upload a file."
  @spec create_file(String.t(), String.t()) :: {:ok, String.t()} | {:error, String.t()}
  def create_file(_config_json, _request_json), do: :erlang.nif_error(:nif_not_loaded)

  @doc "Retrieve metadata for a file."
  @spec retrieve_file(String.t(), String.t()) :: {:ok, String.t()} | {:error, String.t()}
  def retrieve_file(_config_json, _file_id), do: :erlang.nif_error(:nif_not_loaded)

  @doc "Delete a file."
  @spec delete_file(String.t(), String.t()) :: {:ok, String.t()} | {:error, String.t()}
  def delete_file(_config_json, _file_id), do: :erlang.nif_error(:nif_not_loaded)

  @doc "List files. Pass `\"null\"` as query_json to list all files."
  @spec list_files(String.t(), String.t()) :: {:ok, String.t()} | {:error, String.t()}
  def list_files(_config_json, _query_json), do: :erlang.nif_error(:nif_not_loaded)

  @doc "Retrieve the raw content of a file."
  @spec file_content(String.t(), String.t()) :: {:ok, binary()} | {:error, String.t()}
  def file_content(_config_json, _file_id), do: :erlang.nif_error(:nif_not_loaded)

  # ── Batch management ──────────────────────────────────────────────────────

  @doc "Create a new batch job."
  @spec create_batch(String.t(), String.t()) :: {:ok, String.t()} | {:error, String.t()}
  def create_batch(_config_json, _request_json), do: :erlang.nif_error(:nif_not_loaded)

  @doc "Retrieve a batch by ID."
  @spec retrieve_batch(String.t(), String.t()) :: {:ok, String.t()} | {:error, String.t()}
  def retrieve_batch(_config_json, _batch_id), do: :erlang.nif_error(:nif_not_loaded)

  @doc "List batches. Pass `\"null\"` as query_json to list all batches."
  @spec list_batches(String.t(), String.t()) :: {:ok, String.t()} | {:error, String.t()}
  def list_batches(_config_json, _query_json), do: :erlang.nif_error(:nif_not_loaded)

  @doc "Cancel an in-progress batch."
  @spec cancel_batch(String.t(), String.t()) :: {:ok, String.t()} | {:error, String.t()}
  def cancel_batch(_config_json, _batch_id), do: :erlang.nif_error(:nif_not_loaded)

  # ── Response management ───────────────────────────────────────────────────

  @doc "Create a new response."
  @spec create_response(String.t(), String.t()) :: {:ok, String.t()} | {:error, String.t()}
  def create_response(_config_json, _request_json), do: :erlang.nif_error(:nif_not_loaded)

  @doc "Retrieve a response by ID."
  @spec retrieve_response(String.t(), String.t()) :: {:ok, String.t()} | {:error, String.t()}
  def retrieve_response(_config_json, _response_id), do: :erlang.nif_error(:nif_not_loaded)

  @doc "Cancel an in-progress response."
  @spec cancel_response(String.t(), String.t()) :: {:ok, String.t()} | {:error, String.t()}
  def cancel_response(_config_json, _response_id), do: :erlang.nif_error(:nif_not_loaded)
end
