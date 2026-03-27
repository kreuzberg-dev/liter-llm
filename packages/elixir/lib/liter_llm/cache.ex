defmodule LiterLlm.Cache do
  @moduledoc """
  ETS-backed in-memory response cache for the liter-llm client.

  Cached entries are keyed by a hash of the serialized request JSON. Each entry
  has a configurable TTL (time-to-live); expired entries are lazily evicted on
  lookup and proactively pruned when the cache exceeds `:max_entries`.

  This module is used internally by `LiterLlm.Client` when a `:cache` config
  is provided. You generally do not need to call it directly.

  ## Configuration

  Pass a `LiterLlm.Types.cache_config()` map to `LiterLlm.Client.new/1`:

      LiterLlm.Client.new(
        api_key: "sk-...",
        cache: %{max_entries: 256, ttl_seconds: 300}
      )

  """

  @table_name :liter_llm_cache

  @doc """
  Initializes the ETS cache table.

  Safe to call multiple times -- if the table already exists it is a no-op.

  Returns `:ok`.
  """
  @spec init(LiterLlm.Types.cache_config()) :: :ok
  def init(_config) do
    case :ets.whereis(@table_name) do
      :undefined ->
        :ets.new(@table_name, [:set, :public, :named_table, read_concurrency: true])
        :ok

      _ref ->
        :ok
    end
  end

  @doc """
  Looks up a cached response by request key.

  Returns `{:ok, cached_response}` if found and not expired, or `:miss`
  otherwise. Expired entries are deleted on access.
  """
  @spec get(String.t(), pos_integer()) :: {:ok, term()} | :miss
  def get(key, ttl_seconds) do
    case :ets.lookup(@table_name, key) do
      [{^key, value, inserted_at}] ->
        now = System.monotonic_time(:second)

        if now - inserted_at <= ttl_seconds do
          {:ok, value}
        else
          :ets.delete(@table_name, key)
          :miss
        end

      [] ->
        :miss
    end
  rescue
    ArgumentError -> :miss
  end

  @doc """
  Stores a response in the cache.

  If the cache exceeds `max_entries`, the oldest entries are evicted first.
  """
  @spec put(String.t(), term(), pos_integer()) :: :ok
  def put(key, value, max_entries) do
    now = System.monotonic_time(:second)
    :ets.insert(@table_name, {key, value, now})
    maybe_evict(max_entries)
    :ok
  rescue
    ArgumentError -> :ok
  end

  @doc """
  Generates a cache key from a request map by hashing its JSON representation.
  """
  @spec cache_key(map()) :: String.t()
  def cache_key(request) when is_map(request) do
    request
    |> :erlang.term_to_binary()
    |> then(&:crypto.hash(:sha256, &1))
    |> Base.encode16(case: :lower)
  end

  @doc """
  Clears all entries from the cache.
  """
  @spec clear() :: :ok
  def clear do
    :ets.delete_all_objects(@table_name)
    :ok
  rescue
    ArgumentError -> :ok
  end

  @doc """
  Returns the number of entries currently in the cache.
  """
  @spec size() :: non_neg_integer()
  def size do
    :ets.info(@table_name, :size)
  rescue
    ArgumentError -> 0
  end

  # ── Private ──────────────────────────────────────────────────────────────

  defp maybe_evict(max_entries) do
    current_size = :ets.info(@table_name, :size)

    if current_size > max_entries do
      # Collect all entries, sort by insertion time, delete oldest ones
      overflow = current_size - max_entries

      @table_name
      |> :ets.tab2list()
      |> Enum.sort_by(fn {_key, _value, inserted_at} -> inserted_at end)
      |> Enum.take(overflow)
      |> Enum.each(fn {key, _value, _inserted_at} ->
        :ets.delete(@table_name, key)
      end)
    end
  end
end
