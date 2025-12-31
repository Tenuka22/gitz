use crate::{
    handlers,
    models::{self, error::APIError, ui},
};

pub fn get_git_diff(commit_scope: Option<models::cli::CommitVarient>) -> Result<String, APIError> {
    let mut loader = ui::InfiniteLoader::new("Extracting the git diff");

    loader.tick();

    let scope = &commit_scope
        .clone()
        .unwrap_or(models::cli::CommitVarient::Any);
    loader.tick();

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
    loader.tick();

    let diff = handlers::git::git_cmd(&command, "Git diff extraction").map_err(|_| {
        ui::Logger::clear_line();

        return APIError::new_msg(
            "Git diff extraction",
            &format!(
                "Failed to extract {} diff. Check if there are any differences.",
                &commit_scope
                    .clone()
                    .as_ref()
                    .map_or(&models::cli::CommitVarient::Any, |v| v)
            ),
        );
    })?;

    if diff.trim() == "" {
        return Err(APIError::new_msg(
            "Git diff extraction",
            &format!(
                "Failed to extract {} diff. The diff was empty.",
                &commit_scope
                    .clone()
                    .as_ref()
                    .map_or(&models::cli::CommitVarient::Any, |v| v)
            ),
        ));
    }
    loader.tick();

    loader.finish("Extracted the git diff from the system!");

    Ok(diff)
}
