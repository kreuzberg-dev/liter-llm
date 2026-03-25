use anyhow::Result;
use camino::Utf8Path;

use crate::fixture::Fixture;

pub mod rust;

/// Common interface implemented by each language generator.
#[allow(dead_code)]
pub trait Generator {
    /// Short identifier for the target language (used in log messages).
    fn name(&self) -> &'static str;

    /// Generate a complete, runnable test project under `output_root/{lang}/`.
    fn generate(&self, fixtures: &[Fixture], output_root: &Utf8Path) -> Result<()>;
}

/// All concrete generator targets (one per supported language).
#[allow(dead_code)]
pub const ALL_TARGETS: &[&str] = &[
    "rust",
    "python",
    "typescript",
    "ruby",
    "go",
    "java",
    "csharp",
    "php",
    "elixir",
    "wasm",
    "c",
];

/// Dispatch to the generator for the given language name.
pub fn run_generator(lang: &str, fixtures: &[Fixture], output_root: &Utf8Path) -> Result<()> {
    match lang {
        "rust" => rust::RustGenerator.generate(fixtures, output_root),
        other => {
            println!("TODO: {other} generator (not yet implemented)");
            Ok(())
        }
    }
}
