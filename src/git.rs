use crate::cli::CommitVarient;
use std::error::Error;
use std::fs;
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

fn is_ignorable_file(file_path: &str) -> bool {
    file_path.ends_with(".lock") || file_path.ends_with(".sum")
}

pub fn get_git_files_contents() -> Result<Option<String>, Box<dyn Error>> {
    let output = Command::new("git").arg("ls-files").output()?;

    let file_list = String::from_utf8_lossy(&output.stdout);
    let files: Vec<&str> = file_list
        .lines()
        .filter(|file| !is_ignorable_file(file))
        .collect();

    if files.is_empty() {
        eprintln!("No files found or all files were ignorable.");
        return Ok(None);
    }

    let mut combined_contents = String::new();
    for file in files {
        if let Ok(contents) = fs::read_to_string(file) {
            combined_contents.push_str(&contents);
            combined_contents.push('\n');
        } else {
            eprintln!("Warning: could not read file `{}`", file);
        }
    }

    Ok(Some(combined_contents))
}

fn git_config(key: &str) -> Option<String> {
    let out = Command::new("git")
        .args(["config", "--get", key])
        .output()
        .ok()?;

    if out.stdout.is_empty() {
        None
    } else {
        Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
    }
}

pub fn collect_git_metadata() -> String {
    let name = git_config("user.name").unwrap_or("Unknown".into());
    let email = git_config("user.email").unwrap_or("Unknown".into());

    format!("Git metadata:\n- Author: {}\n- Email: {}\n", name, email)
}
