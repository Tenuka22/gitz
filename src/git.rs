use std::process::Command;

use crate::cli::CommitVarient;

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
