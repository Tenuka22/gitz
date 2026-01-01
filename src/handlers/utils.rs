use std::path::Path;

pub fn is_config_file(file_path: &str) -> bool {
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
