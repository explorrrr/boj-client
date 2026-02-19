use boj_mcp_server::{config::Config, run_stdio};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();
    run_stdio(config).await
}
