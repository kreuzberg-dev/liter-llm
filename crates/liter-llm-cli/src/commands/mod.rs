pub mod api;
pub mod mcp;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    /// Start the OpenAI-compatible proxy server.
    Api(api::ApiArgs),
    /// Start the MCP server exposing LLM operations as tools.
    Mcp(mcp::McpArgs),
}
