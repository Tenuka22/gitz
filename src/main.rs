mod handlers;
mod models;

use colored::*;

use crate::models::cli::{Cli, CliVarient};
use crate::models::error::APIError;
use clap::Parser;
use dotenvy::dotenv_override;
use log::{error, info};
use std::io::Write;

#[tokio::main]
async fn main() {
    init_logger();
    dotenv_override().ok();

    let cli = Cli::parse();

    if let Err(e) = run(cli).await {
        error!("Application error: {}", e);
        std::process::exit(1);
    }
}

fn init_logger() {
    env_logger::Builder::new()
        .format(|buf, record| {
            let level = match record.level() {
                log::Level::Error => "ERROR".red().bold(),
                log::Level::Warn => "WARN ".yellow().bold(),
                log::Level::Info => "INFO ".green().bold(),
                log::Level::Debug => "DEBUG".blue().bold(),
                log::Level::Trace => "TRACE".purple().bold(),
            };

            writeln!(
                buf,
                "{} {} {}",
                "====".bright_black(),
                level,
                record.args().to_string().bright_blue()
            )
        })
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();
}

async fn run(cli: Cli) -> Result<(), APIError> {
    match cli.varient {
        CliVarient::CommitMessage => {
            info!("Generating commit message...");
            crate::handlers::commit::message::handle_commit_message(cli.commit_scope).await?;
        }
        CliVarient::Readme => {
            info!("Generating README...");
            crate::handlers::readme::handle_readme().await?;
        }
    }
    Ok(())
}
