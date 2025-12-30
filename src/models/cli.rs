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

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[arg(value_enum)]
    pub varient: CliVarient,

    #[arg(value_enum)]
    pub commit_scope: Option<CommitVarient>,
}
