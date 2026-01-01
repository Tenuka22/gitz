use crate::{
    handlers::{
        ai,
        git::{collect_git_metadata, get_git_files},
        json,
        readme::file_filtering,
        readme::prompts,
    },
    models::{cli::Provider, error::APIError, readme::ReadmeAnalysis, ui},
};
use file_filtering::filter_and_process_readme_files;
use prompts::analysis::README_ANALYSIS_PROMPT;
use tokio_retry::{strategy::FixedInterval, Retry};

pub async fn analyze_readme_content(provider: Provider) -> Result<(ReadmeAnalysis, String, Vec<String>), APIError> {
    ui::Logger::step("Collecting repository files...");
    let files =
        get_git_files().map_err(|_| APIError::new_msg("README", "Failed to get git files"))?;

    let file_contents = filter_and_process_readme_files(files.iter().map(AsRef::as_ref).collect())?;

    ui::Logger::step("Gathering git metadata...");
    let git_context = collect_git_metadata()?;

    let provider_name = match provider {
        Provider::Gemini => "Gemini",
        Provider::Cerebras => "Cerebras",
    };
    ui::Logger::step(&format!("Initializing {} AI...", provider_name));

    let ai_provider = ai::create_provider(provider)?;

    let attempts = 3; // TODO: Add custom attempts

    ui::Logger::step("Analyzing repository structure...");

    let analysis_text = Retry::spawn(
        FixedInterval::from_millis(100).take(attempts),
        || async {
            let response = ai_provider
                .generate_content(
                    Some(README_ANALYSIS_PROMPT),
                    vec![
                        &file_contents,
                        "Analyze this codebase. Extract as much info as possible to make the most comprehensive analysis, then ask ONLY essential questions about information you cannot infer from the code.",
                    ],
                )
                .await?;
            Ok::<String, APIError>(response)
        },
    )
    .await
    .map_err(|e| APIError::new("AI provider Readme Analysis", e))?;
    let json_str = json::handle_json_strip(&analysis_text);

    let analysis: ReadmeAnalysis =
        serde_json::from_str(&json_str).map_err(|e| APIError::new("Invalid analysis JSON", e))?;

    ui::Logger::success("Analysis complete!");

    ui::Logger::header("README CONFIGURATION");

    let mut answers = Vec::new();

    for (i, q) in analysis.questions.iter().enumerate() {
        let options: Vec<&str> = q.options.iter().map(|s| s.as_str()).collect();

        ui::Logger::dim(&format!("Question {}/{}", i + 1, analysis.questions.len()));

        let selected_idx = ui::Input::select(&q.question, &options);

        answers.push(format!(
            "Q{}: {}
A: {}",
            i + 1,
            q.question,
            options[selected_idx]
        ));

        println!();
    }
    Ok((analysis, git_context, answers))
}
