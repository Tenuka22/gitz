use crate::{
    handlers::{self, commit::filter},
    models::{self, error::APIError},
};
use gemini_rust::{Gemini, Model};
use std::env;

pub async fn handle_commit_message(
    commit_scope: Option<models::cli::CommitVarient>,
) -> Result<(), APIError> {
    let diff = handlers::commit::diff::get_git_diff(commit_scope)?
        .ok_or_else(|| APIError::new_msg("Diff extraction", "No diff found"))?;

    let filtered_contents = filter::filter_diff(&diff);

    let api_key =
        env::var("GEMINI_API_KEY").map_err(|e| APIError::new("GEMINI_API_KEY not found", e))?;

    let client =
        Gemini::with_model(&api_key, Model::Gemini25Flash).map_err(|e| APIError::new("Gemini", e))?;

    let response = client
        .generate_content()
        .with_system_prompt(
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
        )
        .with_user_message(format!(
            "Generate a commit message for this git diff, which is preceded by an index of changed files:\n\n```\n{}\n```\n\n\
            Output only the commit message without extra commentary.",
            filtered_contents
        ))
        .execute()
        .await
        .map_err(|e| APIError::new("Gemini", e))?;

    log::info!("Commti message done \n{}", response.text());

    Ok(())
}

