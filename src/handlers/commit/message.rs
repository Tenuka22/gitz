use crate::{
    handlers::{self, commit::filter},
    models::{
        self,
        error::APIError,
        ui::{self, InfiniteLoader},
    },
};
use gemini_rust::{Gemini, GenerationResponse, Model};
use std::env;
use tokio_retry::{Retry, strategy::FixedInterval};

pub async fn handle_commit_message(
    commit_scope: Option<models::cli::CommitVarient>,
    no_emoji: bool,
) -> Result<String, APIError> {
    ui::Logger::dim(&format!(
        "Starting execution of creating a {} commit",
        commit_scope
            .as_ref()
            .map_or(&models::cli::CommitVarient::Any, |v| v)
    ));

    let diff = handlers::commit::diff::get_git_diff(commit_scope)?;

    let filtered_contents = filter::filter_diff(&diff);

    let api_key =
        env::var("GEMINI_API_KEY").map_err(|e| APIError::new("GEMINI_API_KEY not found", e))?;

    let mut loader = InfiniteLoader::new("Ai Agent initialization.");

    loader.tick();
    loader.tick();
    loader.tick();

    let client = Gemini::with_model(&api_key, Model::Gemini25Flash)
        .map_err(|e| APIError::new("Gemini", e))?;

    loader.set_progress(45.0);
    loader.tick();

    let system_prompt = if no_emoji {
        "You are an AI assistant that generates concise, clear, and conventional Git commit messages following the Conventional Commits specification. \
        Follow these rules:\n\
        1. Be imperative (e.g., 'Add', 'Fix', 'Update', 'Implement', 'Enable').\n\
        2. Keep subject line under 72 characters.\n\
        3. Use conventional commit format: <type>(<scope>): <subject>\n\
           Types: feat, fix, docs, style, refactor, perf, test, chore, build, ci\n\
        4. Include detailed explanation in the body.\n\
        5. Prioritize changes by importance:\n\
           P1: User-facing features\n\
           P2: Bug fixes\n\
           P3: Business logic\n\
           P4: Security & Auth\n\
           P5: Performance\n\
           P6: Refactoring\n\
           P7: Configuration\n\
           P8: Dependencies\n\
           P9: Documentation\n\
           P10: Formatting\n\n\
        Use the provided index of changed files for a quick overview, but focus on the highest priority changes in the full diff. \
        If auth code enables sign-in, highlight that functionality, not just dependency additions. \
        Be specific about WHAT changed, not just HOW."
    } else {
        "You are an AI assistant that generates concise, clear, and conventional Git commit messages. \
        Follow these rules:\n\
        1. Be imperative (e.g., 'Add', 'Fix', 'Update', 'Implement', 'Enable').\n\
        2. Keep subject line under 72 characters.\n\
        3. Start with an appropriate emoji prefix.\n\
        4. Include detailed explanation in the body.\n\
        5. Prioritize changes by importance:\n\
           P1: User-facing features\n\
           P2: Bug fixes\n\
           P3: Business logic\n\
           P4: Security & Auth\n\
           P5: Performance\n\
           P6: Refactoring\n\
           P7: Configuration\n\
           P8: Dependencies\n\
           P9: Documentation\n\
           P10: Formatting\n\n\
        EMOJI GUIDE:\n\
        ✨ New feature | 🐛 Bug fix | 🔒 Security/auth | ⚡ Performance\n\
        🎨 UI/UX | ♻️ Refactoring | 🔧 Config | 📦 Dependencies\n\
        📝 Docs | 💄 Formatting | 🚀 Deployment | 🔥 Remove code\n\
        🚧 WIP | ⬆️ Upgrade deps | ⬇️ Downgrade deps | 🎉 Initial commit\n\n\
        Use the provided index of changed files for a quick overview, but focus on the highest priority changes in the full diff. \
        If auth code enables sign-in, highlight that functionality, not just dependency additions. \
        Be specific about WHAT changed, not just HOW."
    };

    let attempts = 3; // TODO: Add custom attempts

    let result = Retry::spawn(
        FixedInterval::from_millis(100).take(attempts),
        || async {
            let response = client
                .generate_content()
                .with_system_prompt(system_prompt)
                .with_user_message(format!(
                    "Generate a commit message for this git diff, which is preceded by an index of changed files:\n\n```\n{}\n```\n\n\
                    Output only the commit message without extra commentary.",
                    filtered_contents
                ))
                .execute()
                .await
                .map_err(|e| APIError::new("Gemini commit message generation", e))?;

            Ok::<GenerationResponse, APIError>(response)
        },
    )
    .await
    .map_err(|e| APIError::new("Gemini commit message generation", e))?;

    loader.set_progress(100.0);
    loader.tick();

    loader.finish("Commti message done");
    println!();
    let message = result.text();
    ui::Logger::command(&message);

    Ok(message)
}
