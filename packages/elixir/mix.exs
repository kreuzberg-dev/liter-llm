defmodule LiterLlm.MixProject do
  use Mix.Project

  @version "1.0.0"
  @source_url "https://github.com/kreuzberg-dev/liter-llm"

  def project do
    [
      app: :liter_llm,
      version: @version,
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      elixirc_paths: elixirc_paths(Mix.env()),
      deps: deps(),
      description:
        "Universal LLM API client — 142+ providers, streaming, tool calling, and provider routing. Rust-powered.",
      package: package(),
      docs: docs(),
      source_url: @source_url
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
      {:rustler, "~> 0.37", optional: true, runtime: false},
      {:rustler_precompiled, "~> 0.8"},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false},
      {:ex_doc, "~> 0.34", only: :dev, runtime: false}
    ]
  end

  defp package do
    [
      licenses: ["MIT"],
      maintainers: ["Na'aman Hirschfeld"],
      links: %{GitHub: @source_url, Homepage: "https://kreuzberg.dev"},
      files: ~w(
        lib
        native/liter_llm_rustler/src
        native/liter_llm_rustler/Cargo.toml
        checksum-Elixir.LiterLlm.Native.exs
        mix.exs
        README.md
        .formatter.exs
      )
    ]
  end

  defp docs do
    [
      main: "LiterLlm",
      source_url: @source_url,
      extras: ["README.md"],
      deps: [elixir: "https://hexdocs.pm/elixir/"]
    ]
  end

  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]
end
