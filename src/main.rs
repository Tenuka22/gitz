mod cli;
mod commit_message;
mod content_filter;
mod git;
mod readme;
mod readme_data;
use dotenvy::dotenv;

use clap::Parser;
use cli::{Cli, CliVarient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cli = Cli::parse();
    // Must call like gtiz.exe commit stage | gtiz.exe commit any | gitz.exe readme
    match cli.varient {
        CliVarient::CommitMessage => {
            commit_message::handle_commit_message(cli.commit_scope).await?
        }
        CliVarient::Readme => readme::handle_readme().await?,
    }

    Ok(())
}
