mod filter;

use crate::{
    handlers::git::{collect_git_metadata, get_git_files},
    models::{error::APIError, readme::ReadmeAnalysis},
};
use filter::filter_and_process_readme_files;
use gemini_rust::{Gemini, Model};
use std::{env, fs, io};

const README_SYSTEM_PROMPT: &str = r#"
You are an expert GitHub README architect specializing in creating visually stunning, developer-friendly documentation.

PHASE 1 — ANALYSIS MODE:
- Analyze the repository structure and code
- Extract concrete technical facts
- Identify missing critical information
- Determine project type (library, CLI tool, web app, etc.)

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
    "tech_stack": [],
    "project_type": ""
  }
}

ANALYSIS RULES:
1. Do NOT assume intent, audience, or use cases
2. Use git metadata (commits, branches, contributors) when available
3. Ask ONLY enum-style questions (multiple choice)
4. Do NOT generate README content in phase 1
5. Return valid JSON only
6. Identify project type: "library", "cli", "web-app", "api", "mobile", "game", or "other"

PHASE 2 — GENERATION MODE:
Create a modern, visually rich README with these characteristics:

VISUAL DESIGN PRINCIPLES:
- Use emoji strategically for section headers (✨ 🚀 📦 ⚡ 🎯 🔧 📚 🤝 📝 ⚠️)
- Include colorful badges from shields.io for: build status, version, license, language
- Add syntax-highlighted code blocks with language tags
- Use tables for structured data (features, commands, configurations)
- Include horizontal rules (---) to separate major sections
- Add blockquotes (>) for important callouts and tips
- Use proper heading hierarchy (# ## ### ####)

REQUIRED SECTIONS (in order):
1. **Header Section**:
   - Project name (large, centered with logo emoji)
   - Catchy tagline (one sentence, italicized)
   - Badges row (build, version, license, downloads, language)
   - Screenshot/demo GIF placeholder or ASCII art for CLI tools

2. **Features Section** (✨):
   - Bullet points with emoji prefixes
   - Highlight 3-5 key capabilities
   - Use bold for feature names

3. **Quick Start** (🚀):
   - Installation commands in code blocks
   - Basic usage example
   - Expected output

4. **Installation** (📦):
   - Multiple installation methods if applicable
   - Prerequisites clearly listed
   - Platform-specific instructions in collapsible sections

5. **Usage** (💻):
   - Common use cases with code examples
   - CLI: command reference table
   - Library: API examples with types
   - Include expected outputs

6. **Configuration** (⚙️) - if applicable:
   - Config file examples
   - Environment variables table
   - Options reference

7. **Examples** (📖):
   - Real-world scenarios
   - Progressive complexity (basic → advanced)
   - Link to examples/ directory if exists

8. **API Reference** (📚) - for libraries:
   - Core functions/methods
   - Parameters and return types
   - Brief descriptions

9. **Contributing** (🤝):
   - Link to CONTRIBUTING.md or brief guidelines
   - Development setup
   - Testing instructions

10. **License** (📝):
    - License type
    - Copyright year and holder

MARKDOWN ENHANCEMENTS:
- Use diff syntax highlighting for before/after comparisons
- Add warning/info callouts using blockquotes with emoji
- Create ASCII diagrams for architecture
- Use details/summary tags for long content
- Add "back to top" links in long READMEs

BADGE EXAMPLES TO INCLUDE:
![Build Status](https://img.shields.io/github/actions/workflow/status/USER/REPO/ci.yml?style=flat-square)
![Version](https://img.shields.io/crates/v/CRATE_NAME?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange?style=flat-square)

CODE BLOCK BEST PRACTICES:
- Always specify language: ```rust, ```bash, ```json, etc.
- Add comments explaining non-obvious code
- Show both command and expected output
- Use // ... for truncated output

TONE AND STYLE:
- Professional yet approachable
- Action-oriented (use imperatives: "Install", "Run", "Configure")
- Assume intelligent audience (avoid over-explanation)
- Be concise but complete
- Use active voice

GENERATION RULES:
1. Use provided extracted data verbatim - DO NOT re-analyze
2. DO NOT repeat information from extracted context
3. Fill ONLY missing sections based on user answers
4. Output complete, copy-paste ready Markdown
5. Replace placeholder values (USER, REPO, etc.) with actual project info
6. Ensure all code blocks are properly formatted and tested
7. Make it visually scannable with consistent spacing
8. Add table of contents for READMEs longer than 200 lines

OUTPUT: Pure Markdown only, no explanations or meta-commentary.
"#;

pub async fn handle_readme() -> Result<(), APIError> {
    let files = get_git_files()?
        .ok_or_else(|| APIError::new_msg("README", "Failed to get git files"))?;
    let file_contents =
        filter_and_process_readme_files(files.iter().map(AsRef::as_ref).collect())?;

    let git_context = collect_git_metadata();

    let api_key =
        env::var("GEMINI_API_KEY").map_err(|e| APIError::new("GEMINI_API_KEY not found", e))?;
    let client =
        Gemini::with_model(api_key, Model::Gemini25Flash).map_err(|e| APIError::new("Gemini", e))?;

    let analysis_response = client
        .generate_content()
        .with_system_instruction(README_SYSTEM_PROMPT)
        .with_user_message(&file_contents)
        .with_user_message(&git_context)
        .with_user_message("Analyze this repository and return missing README information.")
        .execute()
        .await
        .map_err(|e| APIError::new("Gemini", e))?;

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
        .map_err(|e| APIError::new("Invalid analysis JSON", e))?;

    log::info!("\n=== README QUESTIONS ===\n");

    for (i, q) in analysis.questions.iter().enumerate() {
        log::info!("{}. {}", i + 1, q.qe);

        for opt in &q.and {
            log::info!("   {}", opt);
        }

        log::info!("");
    }

    log::info!("Enter answers (one per space):");
    let mut answers_raw = String::new();
    io::stdin()
        .read_line(&mut answers_raw)
        .map_err(|e| APIError::new("STDIN", e))?;

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
        .await
        .map_err(|e| APIError::new("Gemini", e))?;

    let file_path = "README.md";

    fs::write(file_path, readme_response.text()).map_err(|e| APIError::new("fs::write", e))?;
    log::info!("Readme successfully added.");

    Ok(())
}
