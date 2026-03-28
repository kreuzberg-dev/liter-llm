defmodule LiterLlmSmokeTest.MixProject do
  use Mix.Project

  def project do
    [
      app: :liter_llm_smoke_test,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: false,
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:liter_llm, "~> 1.0.0-rc.9"},
      {:jason, "~> 1.4"}
    ]
  end
end
