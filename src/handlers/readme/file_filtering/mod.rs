use crate::handlers::utils::is_config_file;
use crate::models::error::APIError;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const MAX_FILE_CONTENT_LENGTH: usize = 2000;
const MAX_TOTAL_CONTENT_LENGTH: usize = 10000;

/// Checks if a file is likely a test file based on its path and name.
fn is_test_file(file_path: &str) -> bool {
    let path = Path::new(file_path);

    // Check for test-related path components (e.g., /tests/, /spec/)
    if path.components().any(|c| {
        let comp = c.as_os_str().to_string_lossy().to_lowercase();
        matches!(
            comp.as_str(),
            "tests" | "test" | "spec" | "specs" | "__tests__"
        )
    }) {
        return true;
    }

    // Check for common test file naming conventions
    let file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    file_name.starts_with("test_")
        || file_name.ends_with("_test.go")
        || file_name.ends_with("_test.py")
        || file_name.contains(".test.")
        || file_name.contains(".spec.")
}

/// Checks if a file is "interesting" for codebase analysis, ignoring priority files.
fn is_interesting_file(file_path: &str) -> bool {
    let path = Path::new(file_path);
    let file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    // Rule 1: Check for important root-level configuration files.
    // Checks if the file is in the root directory (parent is "" or ".").
    if path.parent().map_or(true, |p| {
        p.as_os_str().is_empty() || p.to_str() == Some(".")
    }) {
        if matches!(
            file_name.as_str(),
            "dockerfile"
                | "docker-compose.yml"
                | "docker-compose.yaml"
                | "makefile"
                | "build.gradle"
                | "pom.xml"
        ) {
            return true;
        }
    }

    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    // Rule 2: Check for source files within common source directories
    if path.components().any(|c| {
        matches!(
            c.as_os_str().to_str(),
            Some("src")
                | Some("source")
                | Some("app")
                | Some("lib")
                | Some("include")
                | Some("cmd")
                | Some("server")
                | Some("core")
        )
    }) {
        return matches!(
            extension,
            "rs" | "go"
                | "py"
                | "js"
                | "ts"
                | "jsx"
                | "tsx"
                | "java"
                | "kt"
                | "kts"
                | "m"
                | "swift"
                | "cpp"
                | "c"
                | "h"
                | "hpp"
                | "cs"
                | "html"
                | "css"
                | "scss"
                | "sh"
                | "bash"
                | "sql"
                | "rb"
                | "php"
        );
    }

    // Rule 3: Check for general configuration files anywhere
    matches!(
        extension,
        "json" | "xml" | "yaml" | "yml" | "toml" | "ini" | "cfg"
    )
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

pub fn filter_and_process_readme_files(files: Vec<&str>) -> Result<String, APIError> {
    let mut content = String::new();
    let mut total_len = 0;

    let mut priority_files = Vec::new();
    let mut interesting_files = Vec::new();

    // Partition files into priority, interesting, and ignored.
    for &file in &files {
        if is_test_file(file) {
            continue; // Explicitly ignore test files
        }

        if is_config_file(file) {
            priority_files.push(file);
        } else if is_interesting_file(file) {
            interesting_files.push(file);
        }
    }

    // Sort for deterministic output
    priority_files.sort();
    interesting_files.sort();

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

    let process_files = |file_paths: &[&str],
                         content: &mut String,
                         total_len: &mut usize|
     -> Result<(), APIError> {
        for file_path in file_paths {
            if *total_len > MAX_TOTAL_CONTENT_LENGTH {
                break;
            }
            let mut file_content = fs::read_to_string(file_path)
                .map_err(|e| APIError::new("fs::read_to_string", e))?;

            content.push_str(&format!("---\nFile: {} ---\n", file_path));
            if file_content.len() > MAX_FILE_CONTENT_LENGTH {
                let mut cut_off_point = MAX_FILE_CONTENT_LENGTH;
                while cut_off_point > 0 && !file_content.is_char_boundary(cut_off_point) {
                    cut_off_point -= 1;
                }
                file_content.truncate(cut_off_point);
                file_content.push('\n');
                file_content.push_str("... (file truncated)");
                file_content.push('\n');
            }
            *total_len += file_content.len();
            content.push_str(&file_content);
            content.push('\n');
            content.push('\n');
        }
        Ok(())
    };

    process_files(&priority_files, &mut content, &mut total_len)?;
    process_files(&interesting_files, &mut content, &mut total_len)?;

    if total_len > MAX_TOTAL_CONTENT_LENGTH {
        let mut cut_off_point = content
            .char_indices()
            .nth(MAX_TOTAL_CONTENT_LENGTH)
            .map_or(content.len(), |(idx, _)| idx);
        while !content.is_char_boundary(cut_off_point) && cut_off_point > 0 {
            cut_off_point -= 1;
        }
        content.truncate(cut_off_point);
        content.push_str("... (total content truncated)");
        content.push('\n');
    }

    Ok(content)
}
