//! # Content Filter
//!
//! This module provides functions for filtering and sanitizing content,
//! primarily aimed at reducing the token count and improving the quality of
//! context provided to the Gemini model.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

// Maximum number of characters to include in the diff context.
// This helps prevent exceeding token limits and keeps the context focused.
const MAX_CONTEXT_LENGTH: usize = 8000;

/// Checks if a file should be ignored based on its path.
///
/// This is used to filter out files that are typically not useful for
/// generating commit messages, such as lockfiles.
fn is_ignorable_file(file_path: &str) -> bool {
    file_path.ends_with(".lock") || file_path.ends_with(".sum")
}

/// Filters and indexes a git diff string to reduce its size, remove noise, and provide a summary.
///
/// The function performs the following operations:
/// 1.  **Indexes Changes**: Creates a summary list of changed files and their status (added, modified, deleted).
/// 2.  **Filters by file type**: Removes diffs for files like `*.lock` and `*.sum`.
/// 3.  **Truncates long diffs**: If the total size exceeds `MAX_CONTEXT_LENGTH`,
///     it truncates the diff and appends a notice.
pub fn filter_diff(diff: &str) -> String {
    log::info!("Filtering a {} length of a diff.", diff.len());
    let mut priority_parts = Vec::new();
    let mut other_parts = Vec::new();
    let mut changed_files_summary = Vec::new();

    for part in diff.split("diff --git ").skip(1) {
        let mut file_processed = false;
        if let Some(first_line) = part.lines().next() {
            if let Some(file_path_a) = first_line.split_whitespace().next() {
                let file_name = file_path_a.strip_prefix("a/").unwrap_or(file_path_a);
                if is_ignorable_file(file_name) {
                    continue; // Skip ignorable files completely
                }
                file_processed = true;

                let status = match part {
                    _ if part.contains("new file mode ") => "added",
                    _ if part.contains("deleted file mode ") => "deleted",
                    _ => "modified",
                };
                changed_files_summary.push(format!("- {} ({})", file_name, status));

                if is_priority_file(file_name) {
                    priority_parts.push(format!("diff --git {}", part));
                } else {
                    other_parts.push(format!("diff --git {}", part));
                }
            }
        }
        if !file_processed {
            // This case handles parts of the diff that might not be associated with a specific file change.
            other_parts.push(format!("diff --git {}", part));
        }
    }

    if priority_parts.is_empty() && other_parts.is_empty() && !diff.is_empty() {
        return "Filtered out diff contents. Likely only lockfiles or ignored files were changed."
            .to_string();
    }

    let mut indexed_diff = String::new();
    if !changed_files_summary.is_empty() {
        indexed_diff.push_str("An index of the changed files:\n");
        changed_files_summary.sort();
        indexed_diff.push_str(&changed_files_summary.join("\n"));
        indexed_diff.push_str("\n\nFull diff for each file (priority files first):\n");
    }

    priority_parts.append(&mut other_parts);
    indexed_diff.push_str(&priority_parts.join("\n"));

    if indexed_diff.len() > MAX_CONTEXT_LENGTH {
        if let Some(last_newline) = indexed_diff[..MAX_CONTEXT_LENGTH].rfind('\n') {
            indexed_diff.truncate(last_newline);
        } else {
            indexed_diff.truncate(MAX_CONTEXT_LENGTH);
        }
        indexed_diff.push_str("\n... (diff truncated)");
    }

    log::info!(
        "Filtering the full {} diff is done and the new diff contain only {} length.",
        diff.len(),
        indexed_diff.len()
    );

    indexed_diff
}

const MAX_FILE_CONTENT_LENGTH: usize = 2000;
const MAX_TOTAL_CONTENT_LENGTH: usize = 10000;

fn is_priority_file(file_path: &str) -> bool {
    matches!(
        Path::new(file_path).file_name().and_then(|s| s.to_str()),
        Some("Cargo.toml")
            | Some("package.json")
            | Some("pyproject.toml")
            | Some("go.mod")
            | Some("pom.xml")
            | Some("build.gradle")
    )
}

fn is_interesting_file(file_path: &str) -> bool {
    if is_priority_file(file_path) {
        return true;
    }
    let path = Path::new(file_path);
    // Check for source directories
    if path
        .components()
        .any(|c| c.as_os_str() == "src" || c.as_os_str() == "app" || c.as_os_str() == "lib")
    {
        // Check for common source file extensions
        return matches!(
            path.extension().and_then(|s| s.to_str()),
            Some("rs")
                | Some("go")
                | Some("py")
                | Some("js")
                | Some("ts")
                | Some("java")
                | Some("kt")
                | Some("m")
                | Some("swift")
                | Some("cpp")
                | Some("h")
                | Some("cs")
        );
    }
    false
}

fn generate_tree_view(files: &[&str]) -> String {
    let mut files_by_dir: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for file in files {
        let path = Path::new(file);
        let parent = path
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or(".")
            .to_string();
        let file_name = path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or(file)
            .to_string();
        files_by_dir.entry(parent).or_default().push(file_name);
    }

    let mut tree = String::new();
    for (dir, file_list) in files_by_dir {
        tree.push_str(&format!("{}\n", dir));
        let mut sorted_files = file_list;
        sorted_files.sort();
        for (i, file) in sorted_files.iter().enumerate() {
            let connector = if i == sorted_files.len() - 1 {
                "└──"
            } else {
                "├──"
            };
            tree.push_str(&format!("  {} {}\n", connector, file));
        }
    }
    tree
}

pub fn filter_and_process_readme_files(
    files: Vec<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut content = String::new();
    let mut total_len = 0;

    let priority_files: Vec<_> = files
        .iter()
        .filter(|f| is_priority_file(f))
        .cloned()
        .collect();
    let interesting_files: Vec<_> = files
        .iter()
        .filter(|f| is_interesting_file(f) && !is_priority_file(f))
        .cloned()
        .collect();

    let all_relevant_files: Vec<&str> = priority_files
        .iter()
        .chain(interesting_files.iter())
        .cloned()
        .collect();

    let tree_view = generate_tree_view(&all_relevant_files);
    content.push_str("Repository file structure:\n");
    content.push_str(&tree_view);
    content.push_str("\n\n");

    content.push_str("Key file contents:\n");

    let process_files = |file_paths: Vec<&str>,
                         content: &mut String,
                         total_len: &mut usize|
     -> Result<(), Box<dyn std::error::Error>> {
        let mut sorted_paths = file_paths;
        sorted_paths.sort();
        for file_path in sorted_paths {
            if *total_len > MAX_TOTAL_CONTENT_LENGTH {
                break;
            }
            if let Ok(mut file_content) = fs::read_to_string(file_path) {
                content.push_str(&format!("--- File: {} ---\n", file_path));
                if file_content.len() > MAX_FILE_CONTENT_LENGTH {
                    file_content.truncate(MAX_FILE_CONTENT_LENGTH);
                    file_content.push_str("\n... (file truncated)\n");
                }
                *total_len += file_content.len();
                content.push_str(&file_content);
                content.push_str("\n\n");
            }
        }
        Ok(())
    };

    process_files(priority_files.clone(), &mut content, &mut total_len)?;
    process_files(interesting_files.clone(), &mut content, &mut total_len)?;

    if total_len > MAX_TOTAL_CONTENT_LENGTH {
        let mut cut_off_point = content
            .char_indices()
            .nth(MAX_TOTAL_CONTENT_LENGTH)
            .map_or(content.len(), |(idx, _)| idx);
        while !content.is_char_boundary(cut_off_point) && cut_off_point > 0 {
            cut_off_point -= 1;
        }

        content.truncate(cut_off_point);
        content.push_str("... (total content truncated)\n");
    }

    Ok(content)
}
