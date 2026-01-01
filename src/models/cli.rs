use std::fmt;

use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
pub enum CliVarient {
    #[value(name = "commit")]
    CommitMessage,
    #[value(name = "readme")]
    Readme,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum CommitVarient {
    #[value(name = "stage")]
    Staged,
    #[value(name = "any")]
    Any,
}

#[derive(Clone, Debug, ValueEnum, Default)]
pub enum Provider {
    #[default]
    #[value(name = "gemini")]
    Gemini,
    #[value(name = "cerebras")]
    Cerebras,
}

impl fmt::Display for CommitVarient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CommitVarient::Staged => "staged",
            CommitVarient::Any => "any",
        };
        write!(f, "{s}")
    }
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[arg(value_enum)]
    pub varient: CliVarient,

    #[arg(value_enum)]
    pub commit_scope: Option<CommitVarient>,

    /// Automatically commit with the generated message
    #[arg(long)]
    pub commit: bool,

    /// Generate commit message without emojis (follows conventional git commit format)
    #[arg(long)]
    pub no_emoji: bool,

    /// AI provider to use (gemini or cerebras)
    #[arg(long, default_value = "gemini")]
    pub provider: Provider,
}
