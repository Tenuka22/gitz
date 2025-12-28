use crate::git::{collect_git_metadata, get_git_files_contents};
use gemini_rust::{Gemini, Model};
use serde::Deserialize;
use std::{fs, io};

#[derive(Debug, Deserialize)]
pub struct ReadmeAnalysis {
    pub questions: Vec<Question>,
    pub extracted: Extracted,
}

#[derive(Debug, Deserialize)]
pub struct Question {
    pub qe: String,
    pub and: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Extracted {
    pub project_name: Option<String>,
    pub project_main_points: Vec<String>,
    pub tech_stack: Vec<String>,
}

const README_SYSTEM_PROMPT: &str = r#"
You are an AI assistant that generates high-quality GitHub README files.

PHASE 1 — ANALYSIS MODE:
- Analyze the repository
- Extract concrete facts
- Determine missing information

OUTPUT FORMAT (STRICT JSON ONLY):
{
  "questions": [
    {
      "qe": "Question text",
      "and": ["1: option", "2: option"]
    }
  ],
  "extracted": {
    "project_name": "",
    "project_main_points": [],
    "tech_stack": []
  }
}

RULES:
1. Do NOT assume intent or audience
2. Use git metadata when available
3. Ask ONLY enum-style questions
4. Do NOT generate README in phase 1
5. Return valid JSON only

PHASE 2 — GENERATION MODE:
- Use provided extracted data verbatim
- Do NOT re-analyze repository
- Do NOT repeat existing information
- Fill ONLY missing sections
- Output Markdown README only
"#;

pub async fn handle_readme() -> Result<(), Box<dyn std::error::Error>> {
    let Some(file_contents) = get_git_files_contents()? else {
        return Err("No git files found".into());
    };

    let git_context = collect_git_metadata();

    let api_key = std::env::var("GEMINI_API_KEY")?;
    let client = Gemini::with_model(api_key, Model::Gemini25Flash)?;

    let analysis_response = client
        .generate_content()
        .with_system_instruction(README_SYSTEM_PROMPT)
        .with_user_message(&file_contents)
        .with_user_message(&git_context)
        .with_user_message("Analyze this repository and return missing README information.")
        .execute()
        .await?;

    let analysis_text = analysis_response.text();

    let mut json_str = analysis_text.trim();
    if json_str.starts_with("```json") {
        json_str = json_str.strip_prefix("```json").unwrap();
    }
    if json_str.ends_with("```") {
        json_str = json_str.strip_suffix("```").unwrap();
    }
    json_str = json_str.trim();

    let analysis: ReadmeAnalysis = serde_json::from_str(json_str)
        .map_err(|e| format!("Invalid analysis JSON: {e}\n{analysis_text}"))?;

    println!("\n=== README QUESTIONS ===\n");
    for (i, q) in analysis.questions.iter().enumerate() {
        println!("{}. {}", i + 1, q.qe);
        for opt in &q.and {
            println!("   {}", opt);
        }
        println!();
    }

    println!("Enter answers (one per space):");
    let mut answers_raw = String::new();
    io::stdin().read_line(&mut answers_raw)?;

    let readme_response = client
        .generate_content()
        .with_system_instruction(README_SYSTEM_PROMPT)
        .with_user_message(format!(
            r#"
KNOWN PROJECT CONTEXT (DO NOT REPEAT OR RE-INFER):

Project name:
{}

Main points:
- {}

Tech stack:
{}

USER ENUM ANSWERS:
{}

INSTRUCTIONS:
- Use main points as-is
- Do NOT restate existing facts
- Fill only missing README sections
- Generate final README.md in Markdown
"#,
            analysis.extracted.project_name.unwrap_or_default(),
            analysis.extracted.project_main_points.join("\n- "),
            analysis.extracted.tech_stack.join(", "),
            answers_raw
        ))
        .execute()
        .await?;

    let file_path = "README.md";

    fs::write(file_path, readme_response.text())?;
    Ok(())
}
