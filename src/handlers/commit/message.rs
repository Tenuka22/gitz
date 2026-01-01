use crate::{
    handlers::{self, commit::filter, ai},
    models::{
        self,
        error::APIError,
        ui::{self, InfiniteLoader},
    },
};
use tokio_retry::{Retry, strategy::FixedInterval};

pub async fn handle_commit_message(
    commit_scope: Option<models::cli::CommitVarient>,
    no_emoji: bool,
    provider: models::cli::Provider,
) -> Result<String, APIError> {
    ui::Logger::dim(&format!(
        "Starting execution of creating a {} commit",
        commit_scope
            .as_ref()
            .map_or(&models::cli::CommitVarient::Any, |v| v)
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
        "You are an AI assistant that generates concise, clear, and conventional Git commit messages following the Conventional Commits specification. \n\n1. Be imperative (e.g., 'Add', 'Fix', 'Update', 'Implement', 'Enable').\n2. Keep subject line under 72 characters.\n3. Use conventional commit format: <type>(<scope>): <subject>\n   Types: feat, fix, docs, style, refactor, perf, test, chore, build, ci\n4. Include detailed explanation in the body.\n5. Prioritize changes by importance:\n   P1: User-facing features\n   P2: Bug fixes\n   P3: Business logic\n   P4: Security & Auth\n   P5: Performance\n   P6: Refactoring\n   P7: Configuration\n   P8: Dependencies\n   P9: Documentation\n   P10: Formatting\n\nUse the provided index of changed files for a quick overview, but focus on the highest priority changes in the full diff. \nIf auth code enables sign-in, highlight that functionality, not just dependency additions. \nBe specific about WHAT changed, not just HOW.\n\nCRITICAL: Output ONLY the commit message itself. Do NOT include any explanations, introductions, meta-commentary, or text like \n'Here's a commit message' or 'This commit message follows'. Start directly with the commit message format."
    } else {
        "You are an AI assistant that generates concise, clear, and conventional Git commit messages. \n\n1. Be imperative (e.g., 'Add', 'Fix', 'Update', 'Implement', 'Enable').\n2. Keep subject line under 72 characters.\n3. Start with an appropriate emoji prefix.\n4. Include detailed explanation in the body.\n5. Prioritize changes by importance:\n   P1: User-facing features\n   P2: Bug fixes\n   P3: Business logic\n   P4: Security & Auth\n   P5: Performance\n   P6: Refactoring\n   P7: Configuration\n   P8: Dependencies\n   P9: Documentation\n   P10: Formatting\n\nEMOJI GUIDE:\n✨ New feature | 🐛 Bug fix | 🔒 Security/auth | ⚡ Performance\n🎨 UI/UX | ♻️ Refactoring | 🔧 Config | 📦 Dependencies\n📝 Docs | 💄 Formatting | 🚀 Deployment | 🔥 Remove code\n🚧 WIP | ⬆️ Upgrade deps | ⬇️ Downgrade deps | 🎉 Initial commit\n\nUse the provided index of changed files for a quick overview, but focus on the highest priority changes in the full diff. \nIf auth code enables sign-in, highlight that functionality, not just dependency additions. \nBe specific about WHAT changed, not just HOW.\n\nCRITICAL: Output ONLY the commit message itself. Do NOT include any explanations, introductions, meta-commentary, or text like \n'Here's a commit message' or 'This commit message follows'. Start directly with the commit message format."
    };

    let attempts = 3; // TODO: Add custom attempts

    let mut message = Retry::spawn(
        FixedInterval::from_millis(100).take(attempts),
        || async {
            let response = ai_provider
                .generate_content(
                    Some(system_prompt),
                    vec![&format!(
                        "Generate a commit message for this git diff, which is preceded by an index of changed files:\n\n```\n{}\n```\n\nIMPORTANT: Output ONLY the commit message itself. Do NOT include:\n- Any introductory text like 'Here's a commit message' or 'This commit message follows'\n- Explanations about the commit format\n- Meta-commentary or descriptions\n- Code blocks or markdown formatting around the message\nStart directly with the commit message (e.g., 'fix(scope): description' or '✨ fix(scope): description').",
                        filtered_contents
                    )],
                )
                .await?;
            Ok::<String, APIError>(response)
        },
    )
    .await
    .map_err(|e| APIError::new("AI provider commit message generation", e))?;

    // Post-process to remove any unwanted prefixes or explanations
    message = message.trim().to_string();
    
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
        if message.starts_with(prefix) {
            // Find the first line that looks like a commit message (starts with type or emoji)
            let lines: Vec<&str> = message.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                let trimmed = line.trim();
                // Check if this line looks like a commit message
                if trimmed.starts_with("fix(") || trimmed.starts_with("feat(") || 
                   trimmed.starts_with("✨") || trimmed.starts_with("🐛") ||
                   trimmed.starts_with("refactor(") || trimmed.starts_with("docs(") ||
                   trimmed.starts_with("chore(") || trimmed.starts_with("test(") ||
                   trimmed.starts_with("perf(") || trimmed.starts_with("style(") ||
                   trimmed.starts_with("build(") || trimmed.starts_with("ci(") {
                    message = lines[i..].join("\n").trim().to_string();
                    break;
                }
            }
        }
    }
    
    // Remove markdown code blocks if present
    if message.starts_with("```") {
        message = message
            .trim_start_matches("```")
            .trim_start_matches("git")
            .trim_start_matches("commit")
            .trim_end_matches("```")
            .trim()
            .to_string();
    }

    loader.set_progress(100.0);
    loader.tick();

    loader.finish("Commit message done");
    println!();
    ui::Logger::command(&message);

    Ok(message)
}