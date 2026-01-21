use std::fmt;

use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
pub enum CliVariant {
    #[value(name = "commit")]
    CommitMessage,
    #[value(name = "readme")]
    Readme,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum CliModel {
    #[value(name = "gemini-2.5-flash")]
    Gemini25Flash,
    #[value(name = "gemini-2.5-pro")]
    Gemini25Pro,
    #[value(name = "gemini-2.5-flash-lite")]
    Gemini25FlashLite,

    #[value(name = "llama3.1-70b")]
    Llama31_70B,
    #[value(name = "llama3.1-8b")]
    Llama31_8B,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum CommitVariant {
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

impl fmt::Display for CommitVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CommitVariant::Staged => "staged",
            CommitVariant::Any => "any",
        };
        write!(f, "{s}")
    }
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[arg(value_enum)]
    pub variant: CliVariant,

    #[arg(value_enum)]
    pub commit_scope: Option<CommitVariant>,

    /// Automatically commit with the generated message
    #[arg(long)]
    pub commit: bool,

    /// Generate commit message without emojis (follows conventional git commit format)
    #[arg(long)]
    pub no_emoji: bool,

    /// AI provider to use (gemini or cerebras)
    #[arg(long, default_value = "gemini")]
    pub provider: Provider,

    /// AI model to use
    #[arg(long)]
    pub model: Option<CliModel>,
}
