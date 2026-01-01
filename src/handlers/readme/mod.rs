mod filter;

use crate::{
    handlers::{
        ai,
        git::{collect_git_metadata, get_git_files},
        json,
    },
    models::{error::APIError, readme::ReadmeAnalysis, ui, cli::Provider},
};
use filter::filter_and_process_readme_files;
use std::fs;

const README_ANALYSIS_PROMPT: &str = r#"
You are a GitHub README analyzer. Extract concrete technical facts and ask ONLY essential questions where information cannot be inferred.

EXTRACTION PRIORITY:
1. Project name: From Cargo.toml, package.json, setup.py, or repo name
2. Tech stack: Dependencies, imports, file extensions
3. Project type: Infer from structure (main.rs=CLI, lib.rs=library, server files=API, package manager=library)
4. Core functionality: Analyze main modules, exported functions, CLI commands
5. Architecture patterns: Observe file structure and code organization

INTELLIGENT QUESTION RULES:
- Ask ONLY when critical information cannot be determined from code
- Skip obvious questions if context is clear (don't ask "Is this a CLI?" for a project with clap/argparse)
- Focus on: target audience, primary use cases, deployment preferences, specific example scenarios
- Maximum 5 questions, minimum 2 questions
- All questions MUST be multiple choice with 3-5 options
- Questions should reveal user intent, not confirm obvious technical facts

QUESTION QUALITY GUIDELINES:
❌ BAD: "What type of project is this?" (can be inferred from code)
✅ GOOD: "Who is the primary audience?" (requires user knowledge)

❌ BAD: "Does this use async?" (visible in code)
✅ GOOD: "What's the main deployment target?" (requires user intent)

OUTPUT FORMAT (STRICT JSON):
{
  "extracted": {
    "project_name": "name from manifest",
    "project_type": "library|cli|web-app|api|mobile|game|other",
    "tech_stack": ["rust", "tokio", "serde"],
    "main_functionality": ["brief description of what code does"],
    "inferred_features": ["features visible in code"]
  },
  "questions": [
    {
      "question": "Clear, specific question about missing info",
      "options": [
        "1: First option",
        "2: Second option",
        "3: Third option"
      ]
    }
  ]
}

CRITICAL RULES:
1. Return ONLY valid JSON, no markdown fences or explanations
2. Extract everything possible from code before asking
3. Questions must require human judgment, not code analysis
4. Be intelligent - don't waste user time on obvious things
5. "extracted.main_functionality" should be concise bullet points of what the code actually does
"#;
const README_GENERATION_PROMPT: &str = r#"
You are a GitHub README generator. Create a visually stunning, production-ready README.md.

CONTEXT USAGE:
- Use provided extracted data verbatim - DO NOT re-analyze or repeat
- Git metadata (author, dates, commits) is for reference only - use naturally
- User answers fill gaps in what couldn't be inferred from code
- DO NOT restate information already in extracted context

VISUAL DESIGN (Modern GitHub Style):
- Strategic emoji for section headers: ✨🚀📦⚡🎯🔧📚🤝📝⚠️
- Shields.io badges: ![Build](https://img.shields.io/badge/build-passing-brightgreen)
- Syntax-highlighted code blocks with language tags
- Tables for structured data (commands, config options, API reference)
- Horizontal rules (---) between major sections
- Blockquotes (>) for callouts and tips
- Proper heading hierarchy (# ## ###)

REQUIRED STRUCTURE (adapt to project type):

1. **Hero Section**:
   ```
   # 🚀 Project Name
   > *Catchy one-liner that explains value proposition*

   ![Build](badge) ![Version](badge) ![License](badge) ![Language](badge)

   [Screenshot/Demo/ASCII art based on project type]
   ```

2. **✨ Features** (3-5 key capabilities):
   - 🎯 **Feature Name**: Brief description
   - ⚡ **Another Feature**: Brief description

3. **🚀 Quick Start** (get user running in 30 seconds):
   ```bash
   # Installation
   cargo install project-name

   # Basic usage
   project-name --help
   ```

4. **📦 Installation**:
   - Multiple methods (cargo, npm, pip, from source)
   - Prerequisites
   - Platform-specific notes

5. **💻 Usage**:
   - CLI: Command reference table with examples
   - Library: API examples with types
   - Web: API endpoints or usage patterns
   - Show expected outputs

6. **⚙️ Configuration** (if applicable):
   - Config file examples
   - Environment variables table
   - Options reference

7. **📖 Examples**:
   - Real-world scenarios
   - Progressive complexity (basic → advanced)
   - Full working examples

8. **📚 API Reference** (for libraries):
   - Core functions/methods with signatures
   - Parameters and return types
   - Brief descriptions

9. **🤝 Contributing**:
   - Brief guidelines or link to CONTRIBUTING.md
   - Development setup
   - Testing commands

10. **📝 License**:
    - License type
    - Copyright holder

PROJECT TYPE ADAPTATIONS:
- **CLI**: Emphasize commands, options, usage examples
- **Library**: Focus on API reference, integration examples, types
- **Web App**: Screenshots, features, deployment guide
- **API**: Endpoints, request/response examples, authentication

BADGE TEMPLATES (replace placeholders):
```markdown
![Build](https://img.shields.io/github/actions/workflow/status/USER/REPO/ci.yml?style=flat-square)
![Version](https://img.shields.io/crates/v/CRATE?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)
![Language](https://img.shields.io/badge/rust-1.70+-orange?style=flat-square)
```

CODE BLOCK RULES:
- Always specify language: ```rust, ```bash, ```json
- Add inline comments for clarity
- Show command AND expected output
- Use // ... for truncated output

MARKDOWN ENHANCEMENTS:
- Diff blocks for before/after: ```diff
- Collapsible sections: <details><summary>Title</summary>content</details>
- Tables for structured data
- Callout blocks: > ⚠️ **Warning**: Important note

TONE:
- Professional yet approachable
- Action-oriented (Install, Run, Configure)
- Concise but complete
- Active voice
- Assume intelligent technical audience

CRITICAL OUTPUT RULES:
1. Output ONLY Markdown - no explanations, no meta-commentary
2. Complete, copy-paste ready content
3. Replace ALL placeholders (USER, REPO, CRATE) with actual values
4. Consistent formatting and spacing
5. Add table of contents for READMEs > 200 lines
6. All code blocks must be valid and properly formatted
7. Use git context naturally (author name, dates) without explicitly stating "from git metadata"

WHAT NOT TO DO:
❌ Don't repeat extracted context verbatim
❌ Don't add "based on analysis" or similar meta-statements
❌ Don't include JSON or code analysis in output
❌ Don't use placeholder values if real ones are available
❌ Don't add your own assumptions beyond provided data
"#;

pub async fn handle_readme(provider: Provider) -> Result<(), APIError> {
    ui::Logger::header("README GENERATOR");
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

    ui::Logger::step("Analyzing repository structure...");
    let analysis_text = ai_provider
        .generate_content(
            Some(README_ANALYSIS_PROMPT),
            vec![
                &file_contents,
                "Analyze this codebase. Extract as much info as possible to make the most comprehensive analysis, then ask ONLY essential questions about information you cannot infer from the code.",
            ],
        )
        .await?;
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
            "Q{}: {}\nA: {}",
            i + 1,
            q.question,
            options[selected_idx]
        ));

        println!();
    }

    ui::Logger::step("Generating README with your selections...");

    let context_message = format!(
        r#"# EXTRACTED PROJECT DATA (use as-is, do not repeat):

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

Generate a complete, production-ready README.md using the above context. Use extracted data verbatim, incorporate git metadata naturally, and fill gaps based on user responses."#,
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
    );

    let readme_content = ai_provider
        .generate_content(
            Some(README_GENERATION_PROMPT),
            vec![&context_message],
        )
        .await?;

    let file_path = "README.md";

    ui::Logger::step("Writing README.md...");
    fs::write(file_path, readme_content).map_err(|e| APIError::new("fs::write", e))?;

    ui::Logger::done("README.md successfully generated!");
    ui::Logger::kv("Location", file_path);

    Ok(())
}
