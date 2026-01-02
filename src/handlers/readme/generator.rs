use crate::{
    handlers::{ai, readme::prompts},
    models::{cli::Provider, error::APIError, readme::ReadmeAnalysis, ui},
};
use prompts::generation::README_GENERATION_PROMPT;
use std::fs;
use tokio_retry::{strategy::FixedInterval, Retry};

fn build_generation_prompt(
    analysis: &ReadmeAnalysis,
    git_context: &str,
    answers: &[String],
) -> String {
    format!(
        r##"# EXTRACTED PROJECT DATA (use as-is, do not repeat):

**Project**: {}
**Type**: {}
**Tech Stack**: {}
**Core Functionality**:
{}

**Inferred Features**:
{}

---

# GIT METADATA (for natural reference):
{}

---

# USER RESPONSES:
{}

---

Generate a complete, production-ready README.md using the above context. Use extracted data verbatim, incorporate git metadata naturally, and fill gaps based on user responses."##,
        analysis
            .extracted
            .project_name
            .as_deref()
            .unwrap_or("Unknown"),
        analysis
            .extracted
            .project_type
            .as_deref()
            .unwrap_or("other"),
        analysis.extracted.tech_stack.join(", "),
        analysis
            .extracted
            .main_functionality
            .iter()
            .map(|s| format!("- {}", s))
            .collect::<Vec<_>>()
            .join("\n"),
        analysis
            .extracted
            .inferred_features
            .as_ref()
            .map(|features| features
                .iter()
                .map(|s| format!("- {}", s))
                .collect::<Vec<_>>()
                .join("\n"))
            .unwrap_or_else(|| "None".to_string()),
        git_context,
        answers.join("\n\n")
    )
}

pub async fn generate_final_readme(
    provider: Provider,
    analysis: ReadmeAnalysis,
    git_context: String,
    answers: Vec<String>,
) -> Result<(), APIError> {
    ui::Logger::step("Generating README with your selections...");

    let context_message = build_generation_prompt(&analysis, &git_context, &answers);

    let ai_provider = ai::create_provider(provider)?;
    let attempts = 3; // TODO: Add custom attempts

    let readme_content = Retry::spawn(FixedInterval::from_millis(100).take(attempts), || async {
        ai_provider
            .generate_content(
                Some(README_GENERATION_PROMPT),
                vec![&context_message],
            )
            .await
    })
    .await
    .map_err(|e| APIError::new("AI provider Readme Generation", e))?;

    let file_path = "README.md";

    ui::Logger::step("Writing README.md...");
    fs::write(file_path, readme_content).map_err(|e| APIError::new("fs::write", e))?;

    ui::Logger::done("README.md successfully generated!");
    ui::Logger::kv("Location", file_path);

    Ok(())
}