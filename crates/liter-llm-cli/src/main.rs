mod commands;

use clap::Parser;
use commands::Command;

#[derive(Parser)]
#[command(name = "liter-llm", version, about = "LiterLLM proxy server and MCP tool server")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli).await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

async fn run(cli: Cli) -> Result<(), String> {
    match cli.command {
        Command::Api(args) => commands::api::run(args).await,
        Command::Mcp(args) => commands::mcp::run(args).await,
    }
}
