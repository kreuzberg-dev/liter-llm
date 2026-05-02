defmodule LiterLlm.MixProject do
  use Mix.Project

  def project do
    [
      app: :liter_llm,
      version: "1.4.0-rc.27",
      elixir: "~> 1.14",
      elixirc_paths: ["lib", "../..//packages/elixir/native/liter_llm_nif/src"],
      rustler_crates: [liter_llm_nif: [mode: :release]],
      description: "Universal LLM API client with Rust-powered polyglot bindings.",
      package: package(),
      deps: deps()
    ]
  end

  defp package do
    [
      licenses: ["MIT"],
      links: %{"GitHub" => "https://github.com/kreuzberg-dev/liter-llm"},
      files: ~w(lib native .formatter.exs mix.exs README* checksum-*.exs)
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.37.0", optional: true, runtime: false},
      {:rustler_precompiled, "~> 0.9"},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false},
      {:ex_doc, "~> 0.40", only: :dev, runtime: false}
    ]
  end
end
