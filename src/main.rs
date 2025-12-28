mod cli;
mod commit_message;
mod content_filter;
mod git;
mod readme;
use dotenvy::dotenv;

use clap::Parser;
use cli::{Cli, CliVarient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cli = Cli::parse();

    match cli.varient {
        CliVarient::CommitMessage => {
            commit_message::handle_commit_message(cli.commit_scope).await?
        }
        CliVarient::Readme => readme::handle_readme().await?,
    }

    Ok(())
}
