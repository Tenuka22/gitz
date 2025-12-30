use crate::models::error::APIError;
use std::process::Command;

pub fn git_cmd(args: &[&str]) -> Option<String> {
    let out = Command::new("git").args(args).output().ok()?;
    if out.stdout.is_empty() {
        None
    } else {
        Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
    }
}

pub fn get_git_files() -> Result<Option<Vec<String>>, APIError> {
    let output = Command::new("git")
        .args(&["ls-files", "-c", "--exclude-standard"])
        .output()
        .map_err(|e| APIError::new("git ls-files command execution", e))?;

    if !output.status.success() {
        return Err(APIError::new_msg(
            "git ls-files failed",
            &String::from_utf8_lossy(&output.stderr),
        ));
    }

    let file_list = String::from_utf8_lossy(&output.stdout);
    let files: Vec<String> = file_list.lines().map(String::from).collect();

    if files.is_empty() {
        return Err(APIError::new_msg(
            "git ls-files",
            "No files found in git repository.",
        ));
    }

    Ok(Some(files))
}


fn git_config(key: &str) -> Option<String> {
    git_cmd(&["config", "--get", key])
}

pub fn collect_git_metadata() -> String {
    let name = git_config("user.name").unwrap_or("Unknown".into());
    let email = git_config("user.email").unwrap_or("Unknown".into());

    let repo_root = git_cmd(&["rev-parse", "--show-toplevel"]).unwrap_or("Unknown".into());

    let repo_name = repo_root
        .split(std::path::MAIN_SEPARATOR)
        .last()
        .unwrap_or("Unknown");

    let branch = git_cmd(&["rev-parse", "--abbrev-ref", "HEAD"]).unwrap_or("Unknown".into());

    let is_dirty = git_cmd(&["status", "--porcelain"])
        .map(|s| !s.is_empty())
        .unwrap_or(false);

    let last_commit = git_cmd(&["log", "-1", "--pretty=%h %s"]).unwrap_or("None".into());

    let origin = git_cmd(&["remote", "get-url", "origin"]).unwrap_or("None".into());

    format!(
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
    )
}
