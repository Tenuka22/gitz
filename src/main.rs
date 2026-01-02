mod handlers;
mod models;

use crate::models::cli;
use crate::models::error;
use crate::models::ui;
use clap::Parser;
use dotenvy::dotenv_override;

async fn run(cli: cli::Cli) -> Result<(), error::APIError> {
    match cli.variant {
        cli::CliVariant::CommitMessage => {
            let message = crate::handlers::commit::message::handle_commit_message(cli.commit_scope, cli.no_emoji, cli.provider).await?;
            
            if cli.commit {
                ui::Logger::dim("Executing git commit...");
                handlers::git::git_cmd(
                    &["commit", "-m", &message],
                    "git commit",
                )?;
                ui::Logger::success("Commit created successfully!");
            }
        }
        cli::CliVariant::Readme => {
            crate::handlers::readme::handle_readme(cli.provider).await?;
        }
    }
    Ok(())
}
use std::panic;

#[tokio::main]
async fn main() {
    dotenv_override().ok();

    panic::set_hook(Box::new(|panic_info| {
        let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Undiagnized heart attack occured,".to_string()
        };

        let location = panic_info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()));

        ui::Logger::fatal(&msg, location.as_deref());
    }));

    ui::Logger::header("GITZ A renovated ai commits and readmes");

    let git_repo_result =
        handlers::git::ensure_git_repo().map_err(|e| error::APIError::new("Git", e));

    match git_repo_result {
        Ok(_) => ui::Logger::dim("GITZ found a git repo."),
        Err(e) => {
            ui::Logger::error(&e.to_string());
            std::process::exit(1);
        }
    }
    let cli = cli::Cli::parse();

    if let Err(e) = run(cli).await {
        println!();
        ui::Logger::error(&e.to_string());
        std::process::exit(1);
    }
}
