defmodule LiterLm.MixProject do
  use Mix.Project

  @version "1.0.0-rc.1"
  @source_url "https://github.com/kreuzberg-dev/liter-lm"

  def project do
    [
      app: :liter_lm,
      version: @version,
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      elixirc_paths: elixirc_paths(Mix.env()),
      deps: deps(),
      description:
        "High-performance LLM client with streaming, tool calling, and provider routing",
      package: package(),
      docs: docs(),
      source_url: @source_url,
      rustler_crates: [liter_lm: [mode: :release]]
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:jason, "~> 1.4"},
      {:req, "~> 0.5"},
      {:rustler, "~> 0.34.0", optional: true, runtime: false},
      {:rustler_precompiled, "~> 0.8"},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false},
      {:ex_doc, "~> 0.34", only: :dev, runtime: false}
    ]
  end

  defp package do
    [
      licenses: ["MIT"],
      links: %{GitHub: @source_url},
      files: ~w(
        lib
        native/liter_lm_rustler/src
        native/liter_lm_rustler/Cargo.toml
        native/liter_lm_rustler/Cargo.lock
        mix.exs
        README.md
        .formatter.exs
      )
    ]
  end

  defp docs do
    [
      main: "LiterLm",
      source_url: @source_url,
      extras: ["README.md"],
      deps: [elixir: "https://hexdocs.pm/elixir/"]
    ]
  end

  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]
end
