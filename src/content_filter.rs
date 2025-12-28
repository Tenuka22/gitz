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
    let mut filtered_parts = Vec::new();
    let mut changed_files_summary = Vec::new();
    let diff_parts = diff.split("diff --git ");

    for part in diff_parts.skip(1) {
        // The first split part is usually empty
        if let Some(first_line) = part.lines().next() {
            // The first line of a diff part looks like: "a/src/main.rs b/src/main.rs"
            if let Some(file_path_a) = first_line.split_whitespace().next() {
                let file_name = file_path_a.strip_prefix("a/").unwrap_or(file_path_a);
                if is_ignorable_file(file_name) {
                    continue;
                }

                let status = if part.contains(
                    "
new file mode ",
                ) {
                    "added"
                } else if part.contains(
                    "
deleted file mode ",
                ) {
                    "deleted"
                } else {
                    "modified"
                };
                changed_files_summary.push(format!("- {} ({})", file_name, status));

                let diff_chunk = format!("diff --git {}", part);
                filtered_parts.push(diff_chunk);
            } else {
                // If the file path can't be determined, include it to be safe.
                filtered_parts.push(format!("diff --git {}", part));
            }
        }
    }

    if filtered_parts.is_empty() && !diff.is_empty() {
        // This case occurs if all changes were in files that are filtered out.
        return "Filtered out diff contents. Likely only lockfiles or other ignored files were changed."
            .to_string();
    }

    let mut indexed_diff = String::new();
    if !changed_files_summary.is_empty() {
        indexed_diff.push_str(
            "An index of the changed files:
",
        );
        indexed_diff.push_str(&changed_files_summary.join(
            "
",
        ));
        indexed_diff.push_str(
            "

",
        );
    }

    indexed_diff.push_str(
        "Full diff for each file:
",
    );
    indexed_diff.push_str(&filtered_parts.join("")); // Join with no separator, parts already start with "diff --git"

    // Truncate the final diff if it's too long.
    if indexed_diff.len() > MAX_CONTEXT_LENGTH {
        // Try to find the last line break within the limit to avoid cutting a line mid-stream.
        if let Some(last_newline) = indexed_diff[..MAX_CONTEXT_LENGTH].rfind("") {
            indexed_diff.truncate(last_newline);
        } else {
            indexed_diff.truncate(MAX_CONTEXT_LENGTH);
        }
        indexed_diff.push_str(
            "
... (diff truncated)",
        );
    }

    indexed_diff
}
