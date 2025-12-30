use crate::{
    handlers,
    models::{self, error::APIError},
};

pub fn get_git_diff(
    commit_scope: Option<models::cli::CommitVarient>,
) -> Result<Option<String>, APIError> {
    let scope = commit_scope.unwrap_or(models::cli::CommitVarient::Any);

    let mut command = vec![
        "diff",
        "--no-color",
        "--no-ext-diff",
        "--minimal",
        "--unified=3",
    ];

    match scope {
        models::cli::CommitVarient::Staged => {
            command.push("--staged");
        }
        models::cli::CommitVarient::Any => {
            command.push("HEAD");
        }
    }

    let diff = handlers::git::git_cmd(&command);

    Ok(diff)
}

