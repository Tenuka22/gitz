use crate::handlers::utils::is_priority_file;
use crate::models::error::APIError;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const MAX_FILE_CONTENT_LENGTH: usize = 2000;
const MAX_TOTAL_CONTENT_LENGTH: usize = 10000;

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

pub fn filter_and_process_readme_files(files: Vec<&str>) -> Result<String, APIError> {
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
     -> Result<(), APIError> {
        let mut sorted_paths = file_paths;
        sorted_paths.sort();
        for file_path in sorted_paths {
            if *total_len > MAX_TOTAL_CONTENT_LENGTH {
                break;
            }
            let mut file_content = fs::read_to_string(file_path)
                .map_err(|e| APIError::new("fs::read_to_string", e))?;

            content.push_str(&format!("--- File: {} ---\n", file_path));
            if file_content.len() > MAX_FILE_CONTENT_LENGTH {
                file_content.truncate(MAX_FILE_CONTENT_LENGTH);
                file_content.push_str("\n... (file truncated)\n");
            }
            *total_len += file_content.len();
            content.push_str(&file_content);
            content.push_str("\n\n");
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
