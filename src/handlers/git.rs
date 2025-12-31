use crate::models::{error::APIError, ui};
use std::process::Command;

pub fn git_cmd(args: &[&str], context: &str) -> Result<String, APIError> {
    let out = Command::new("git")
        .args(args)
        .output()
        .map_err(|e| APIError::new_msg(context, &format!("Failed to execute git: {e}")))?;

    if !out.status.success() {
        return Err(APIError::new_msg(
            context,
            String::from_utf8_lossy(&out.stderr).trim(),
        ));
    }

    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

pub fn ensure_git_repo() -> Result<(), APIError> {
    git_cmd(
        &["rev-parse", "--is-inside-work-tree"],
        "Git repository check",
    )?;

    Ok(())
}

pub fn git_config(key: &str) -> Result<String, APIError> {
    git_cmd(&["config", "--get", key], &format!("git config {}", key))
}

pub fn get_git_files() -> Result<Vec<String>, APIError> {
    let output = git_cmd(&["ls-files", "-c", "--exclude-standard"], "git ls-files")?;

    let files: Vec<String> = output.lines().map(String::from).collect();

    if files.is_empty() {
        return Err(APIError::new_msg(
            "Git file list",
            "No files found in git repository.",
        ));
    }

    Ok(files)
}

pub fn collect_git_metadata() -> Result<String, APIError> {
    let mut loader = ui::InfiniteLoader::new("Collecting Git Metadata");

    loader.tick();
    loader.tick();
    loader.tick();

    let name = git_config("user.name")?;
    let email = git_config("user.email")?;
    let repo_root = git_cmd(
        &["rev-parse", "--show-toplevel"],
        "rev-parse --show-toplevel",
    )?;
    let repo_name = repo_root
        .split(std::path::MAIN_SEPARATOR)
        .last()
        .unwrap_or("Unknown");
    let branch = git_cmd(
        &["rev-parse", "--abbrev-ref", "HEAD"],
        "rev-parse --abbrev-ref HEAD",
    )?;
    let is_dirty =
        git_cmd(&["status", "--porcelain"], "status --porcelain").map(|s| !s.is_empty())?;
    let last_commit = git_cmd(&["log", "-1", "--pretty=%h %s"], "log -1")?;
    let origin = git_cmd(&["remote", "get-url", "origin"], "remote get-url origin")?;

    loader.set_progress(100.0);
    loader.finish("Successfully extracted user data!");

    Ok(format!(
        "\
Git metadata:
- Repository: {}
- Branch: {}
- Dirty: {}
- Author: {}
- Email: {}
- Last commit: {}
- Origin: {}
",
        repo_name, branch, is_dirty, name, email, last_commit, origin
    ))
}
