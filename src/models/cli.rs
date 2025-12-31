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
}
