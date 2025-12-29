//! # Content Filter
//!
//! This module provides functions for filtering and sanitizing content,
//! primarily aimed at reducing the token count and improving the quality of
//! context provided to the Gemini model.

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
    let mut filtered_parts = Vec::new();
    let mut changed_files_summary = Vec::new();

    for part in diff.split("diff --git ").skip(1) {
        if let Some(first_line) = part.lines().next() {
            if let Some(file_path_a) = first_line.split_whitespace().next() {
                let file_name = file_path_a.strip_prefix("a/").unwrap_or(file_path_a);
                if is_ignorable_file(file_name) {
                    continue;
                }

                let status = match part {
                    _ if part.contains("new file mode ") => "added",
                    _ if part.contains("deleted file mode ") => "deleted",
                    _ => "modified",
                };
                changed_files_summary.push(format!("- {} ({})", file_name, status));
            }
        }
        filtered_parts.push(format!("diff --git {}", part));
    }

    if filtered_parts.is_empty() && !diff.is_empty() {
        return "Filtered out diff contents. Likely only lockfiles or ignored files were changed."
            .to_string();
    }

    let mut indexed_diff = String::new();
    if !changed_files_summary.is_empty() {
        indexed_diff.push_str("An index of the changed files:\n");
        indexed_diff.push_str(&changed_files_summary.join("\n"));
        indexed_diff.push_str("\n\nFull diff for each file:\n");
    }

    indexed_diff.push_str(&filtered_parts.join("\n"));

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
