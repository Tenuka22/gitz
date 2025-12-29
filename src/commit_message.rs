use crate::cli::CommitVarient;
use crate::content_filter;
use crate::git;
use gemini_rust::{Gemini, Model};
use std::env;
use std::io::Write;
use tempfile::NamedTempFile;

pub async fn handle_commit_message(
    commit_scope: Option<CommitVarient>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut temp = NamedTempFile::new()?;

    let diff = git::get_git_diff(commit_scope)?.ok_or("No diff found")?;

    write!(temp, "{}", diff)?;

    let contents = std::fs::read_to_string(temp.path())?;

    let filtered_contents = content_filter::filter_diff(&contents);

    let api_key = env::var("GEMINI_API_KEY")?;

    let client = Gemini::with_model(&api_key, Model::Gemini25Flash)?;

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
        .await?;

    log::info!("Commti message done \n{}", response.text());

    Ok(())
}
