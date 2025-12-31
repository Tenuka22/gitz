pub fn handle_json_strip(text: &str) -> String {
    let mut json_str = text.trim();
    if json_str.starts_with("```json") {
        json_str = json_str.strip_prefix("```json").unwrap();
    }
    if json_str.ends_with("```") {
        json_str = json_str.strip_suffix("```").unwrap();
    }

    json_str = json_str.trim();

    return json_str.to_string();
}
