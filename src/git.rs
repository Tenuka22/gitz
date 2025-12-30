use crate::cli::CommitVarient;
use std::error::Error;
use std::process::Command;

pub fn get_git_diff(
    commit_scope: Option<CommitVarient>,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let scope = commit_scope.unwrap_or(CommitVarient::All);

    let mut cmd = Command::new("git");
    cmd.arg("diff")
        .arg("--no-color")
        .arg("--no-ext-diff")
        .arg("--unified=3")
        .arg("--minimal");

    match scope {
        CommitVarient::Staged => {
            cmd.arg("--staged");
        }
        CommitVarient::All => {
            cmd.arg("HEAD");
        }
    }

    let output = cmd.output()?;
    let diff = String::from_utf8_lossy(&output.stdout);

    if diff.trim().is_empty() {
        eprintln!("No diff found.");
        return Ok(None);
    }

    Ok(Some(diff.to_string()))
}

pub fn get_git_files() -> Result<Option<Vec<String>>, Box<dyn Error>> {
    let output = Command::new("git")
        .args(&["ls-files", "-c", "--exclude-standard"])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "git ls-files failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let file_list = String::from_utf8_lossy(&output.stdout);
    let files: Vec<String> = file_list.lines().map(String::from).collect();

    if files.is_empty() {
        eprintln!("No files found in git repository.");
        return Ok(None);
    }

    Ok(Some(files))
}

fn git_cmd(args: &[&str]) -> Option<String> {
    let out = Command::new("git").args(args).output().ok()?;
    if out.stdout.is_empty() {
        None
    } else {
        Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
    }
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
