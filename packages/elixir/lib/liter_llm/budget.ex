defmodule LiterLlm.Budget do
  @moduledoc """
  ETS-backed cost budget tracker for the liter-llm client.

  Tracks cumulative spend per model and globally, enforcing limits configured
  via `LiterLlm.Types.budget_config()`. When enforcement is `"strict"`,
  requests that would exceed the budget return an error. When enforcement is
  `"warn"`, overages are logged but requests proceed.

  This module is used internally by `LiterLlm.Client` when a `:budget` config
  is provided. You generally do not need to call it directly.

  ## Configuration

      LiterLlm.Client.new(
        api_key: "sk-...",
        budget: %{
          global_limit: 10.0,
          model_limits: %{"gpt-4o" => 5.0},
          enforcement: "strict"
        }
      )

  """

  require Logger

  @table_name :liter_llm_budget

  @doc """
  Initializes the ETS budget table.

  Safe to call multiple times -- if the table already exists it is a no-op.
  """
  @spec init(LiterLlm.Types.budget_config()) :: :ok
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
  Checks whether the next request for `model` is within budget.

  Returns `:ok` if the request may proceed, or `{:error, LiterLlm.Error.t()}`
  if the budget is exceeded (strict mode only). In warn mode, logs a warning
  and returns `:ok`.
  """
  @spec check(LiterLlm.Types.budget_config(), String.t() | nil) ::
          :ok | {:error, LiterLlm.Error.t()}
  def check(config, model) do
    enforcement = Map.get(config, :enforcement, "strict")
    global_limit = Map.get(config, :global_limit)
    model_limits = Map.get(config, :model_limits, %{})

    global_spend = get_spend(:global)
    model_spend = if model, do: get_spend({:model, model}), else: 0.0

    cond do
      not is_nil(global_limit) and global_spend >= global_limit ->
        handle_exceeded(
          enforcement,
          "global budget exceeded: $#{Float.round(global_spend, 4)} >= $#{Float.round(global_limit, 4)}"
        )

      not is_nil(model) and Map.has_key?(model_limits, model) and
          model_spend >= model_limits[model] ->
        limit = model_limits[model]

        handle_exceeded(
          enforcement,
          "model '#{model}' budget exceeded: $#{Float.round(model_spend, 4)} >= $#{Float.round(limit, 4)}"
        )

      true ->
        :ok
    end
  end

  @doc """
  Records a cost against the global and per-model budget trackers.
  """
  @spec record(String.t() | nil, float()) :: :ok
  def record(model, cost) when is_float(cost) or is_integer(cost) do
    cost = cost / 1
    update_spend(:global, cost)
    if model, do: update_spend({:model, model}, cost)
    :ok
  rescue
    ArgumentError -> :ok
  end

  @doc """
  Returns the current global spend.
  """
  @spec global_spend() :: float()
  def global_spend, do: get_spend(:global)

  @doc """
  Returns the current spend for a specific model.
  """
  @spec model_spend(String.t()) :: float()
  def model_spend(model), do: get_spend({:model, model})

  @doc """
  Resets all spend counters to zero.
  """
  @spec reset() :: :ok
  def reset do
    :ets.delete_all_objects(@table_name)
    :ok
  rescue
    ArgumentError -> :ok
  end

  # ── Private ──────────────────────────────────────────────────────────────

  defp get_spend(key) do
    case :ets.lookup(@table_name, key) do
      [{^key, amount}] -> amount
      [] -> 0.0
    end
  rescue
    ArgumentError -> 0.0
  end

  defp update_spend(key, cost) do
    case :ets.lookup(@table_name, key) do
      [{^key, current}] ->
        :ets.insert(@table_name, {key, current + cost})

      [] ->
        :ets.insert(@table_name, {key, cost})
    end
  end

  defp handle_exceeded("strict", message) do
    {:error, LiterLlm.Error.budget_exceeded(message)}
  end

  defp handle_exceeded(_warn, message) do
    Logger.warning("liter-llm: #{message}")
    :ok
  end
end
