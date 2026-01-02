use crate::{
    handlers::{self, commit::filter, ai, commit::prompts},
    models::{
        self,
        error::APIError,
        ui::{self, InfiniteLoader},
    },
};
use tokio_retry::{Retry, strategy::FixedInterval};

fn clean_commit_message(message: &str) -> String {
    let mut cleaned_message = message.trim().to_string();

    // Remove common unwanted prefixes
    let unwanted_prefixes = [
        "Here's a commit message",
        "Here is a commit message",
        "This commit message",
        "The commit message",
        "Commit message:",
        "```",
    ];

    for prefix in &unwanted_prefixes {
        if cleaned_message.starts_with(prefix) {
            // Find the first line that looks like a commit message (starts with type or emoji)
            if let Some(first_commit_line) = cleaned_message.lines().find(|line| {
                let trimmed = line.trim();
                trimmed.starts_with("fix(") || trimmed.starts_with("feat(") ||
                trimmed.starts_with("✨") || trimmed.starts_with("🐛") ||
                trimmed.starts_with("refactor(") || trimmed.starts_with("docs(") ||
                trimmed.starts_with("chore(") || trimmed.starts_with("test(") ||
                trimmed.starts_with("perf(") || trimmed.starts_with("style(") ||
                trimmed.starts_with("build(") || trimmed.starts_with("ci(")
            }) {
                cleaned_message = first_commit_line.to_string() + "\n" + &cleaned_message.lines().skip(1).collect::<Vec<_>>().join("\n");
                cleaned_message = cleaned_message.trim().to_string();
            }
        }
    }

    // Remove markdown code blocks if present
    if cleaned_message.starts_with("```") {
        cleaned_message = cleaned_message
            .trim_start_matches("```")
            .trim_start_matches("git")
            .trim_start_matches("commit")
            .trim_end_matches("```")
            .trim()
            .to_string();
    }

    cleaned_message
}

pub async fn handle_commit_message(
    commit_scope: Option<models::cli::CommitVariant>,
    no_emoji: bool,
    provider: models::cli::Provider,
) -> Result<String, APIError> {
    ui::Logger::dim(&format!(
        "Starting execution of creating a {} commit",
        commit_scope
            .as_ref()
            .map_or(&models::cli::CommitVariant::Any, |v| v)
    ));

    let diff = handlers::commit::diff::get_git_diff(commit_scope)?;

    let filtered_contents = filter::filter_diff(&diff);

    let mut loader = InfiniteLoader::new("Ai Agent initialization.");

    loader.tick();
    loader.tick();
    loader.tick();

    let ai_provider = ai::create_provider(provider)?;

    loader.set_progress(45.0);
    loader.tick();

    let system_prompt = if no_emoji {
        prompts::COMMIT_PROMPT_NO_EMOJI
    } else {
        prompts::COMMIT_PROMPT_WITH_EMOJI
    };

    let attempts = 3; // TODO: Add custom attempts

    let message = Retry::spawn(
        FixedInterval::from_millis(100).take(attempts),
        || async {
            ai_provider
                .generate_content(
                    Some(system_prompt),
                    vec![&prompts::COMMIT_USER_MESSAGE_PROMPT.replace("{}", &filtered_contents)],
                )
                .await
        },
    )
    .await
    .map_err(|e| APIError::new("AI provider commit message generation", e))?;

    let cleaned_message = clean_commit_message(&message);

    loader.set_progress(100.0);
    loader.tick();

    loader.finish("Commit message done");
    println!();
    ui::Logger::command(&cleaned_message);

    Ok(cleaned_message)
}
