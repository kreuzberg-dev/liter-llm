mod fixture;
mod generators;

use anyhow::Result;
use camino::Utf8PathBuf;
use clap::{Parser, Subcommand, ValueEnum};
use fixture::load_fixtures;

#[derive(Parser)]
#[command(author, version, about = "Generate language-specific E2E test suites from fixtures")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate e2e tests for a language (or all languages).
    Generate {
        /// Target language to generate for. Use "all" for every language.
        #[arg(long, value_enum)]
        lang: Language,
        /// Fixture directory.
        #[arg(long, default_value = "fixtures")]
        fixtures: Utf8PathBuf,
        /// Output directory.
        #[arg(long, default_value = "e2e")]
        output: Utf8PathBuf,
    },
    /// List all loaded fixtures (for quick inspection).
    List {
        /// Fixture directory.
        #[arg(long, default_value = "fixtures")]
        fixtures: Utf8PathBuf,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Language {
    All,
    Rust,
    Python,
    Typescript,
    Ruby,
    Go,
    Java,
    Csharp,
    Php,
    Elixir,
    Wasm,
    C,
}

impl Language {
    fn all_concrete() -> &'static [Language] {
        &[
            Language::Rust,
            Language::Python,
            Language::Typescript,
            Language::Ruby,
            Language::Go,
            Language::Java,
            Language::Csharp,
            Language::Php,
            Language::Elixir,
            Language::Wasm,
            Language::C,
        ]
    }

    fn as_str(self) -> &'static str {
        match self {
            Language::All => "all",
            Language::Rust => "rust",
            Language::Python => "python",
            Language::Typescript => "typescript",
            Language::Ruby => "ruby",
            Language::Go => "go",
            Language::Java => "java",
            Language::Csharp => "csharp",
            Language::Php => "php",
            Language::Elixir => "elixir",
            Language::Wasm => "wasm",
            Language::C => "c",
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { lang, fixtures, output } => {
            let fixtures = load_fixtures(fixtures.as_path())?;
            let langs = if matches!(lang, Language::All) {
                Language::all_concrete().to_vec()
            } else {
                vec![lang]
            };
            for lang in langs {
                generators::run_generator(lang.as_str(), &fixtures, output.as_path())?;
            }
        }
        Commands::List { fixtures } => {
            let fixtures = load_fixtures(fixtures.as_path())?;
            if fixtures.is_empty() {
                println!("No fixtures found.");
            } else {
                for fixture in &fixtures {
                    println!("[{}] {} — {}", fixture.category, fixture.id, fixture.description);
                }
                println!("\n{} fixture(s) total.", fixtures.len());
            }
        }
    }

    Ok(())
}
