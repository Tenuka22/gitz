# 🚀 gitz-cli

> AI-powered Git commit message generation for a focused and accurate development workflow.
> 
[![Rust](https://github.com/Tenuka22/gitz/actions/workflows/rust.yml/badge.svg)](https://github.com/Tenuka22/gitz/actions/workflows/rust.yml)
![Version](https://img.shields.io/crates/v/gitz-cli?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)
![Language](https://img.shields.io/badge/rust-1.70+-orange?style=flat-square)

```
$ gitz-cli commit
✨ Analyzing staged changes...
📦 Filtering diff (excluding node_modules, .env)...
🚀 Sending diff to AI (Gemini)...
AI responded with a commit message:

feat: Add AI-powered commit message generation

This commit introduces a new feature to automatically
generate Git commit messages using AI.

Key changes include:
- Integration with Gemini and Cerebras AI providers.
- Intelligent filtering of Git diffs to exclude irrelevant files.
- Command-line interface for easy invocation before 'git commit'.

> Use this message? (Y/n): Y
Commit message copied to clipboard. Paste into your Git editor.
```
*An example of `gitz-cli commit` in action, generating a relevant commit message based on staged changes.*

---

## 📖 Table of Contents

- [✨ Features](#-features)
- [🚀 Quick Start](#-quick-start)
- [📦 Installation](#-installation)
- [💻 Usage](#-usage)
- [⚙️ Configuration](#️-configuration)
- [🤝 Contributing](#-contributing)
- [📝 License](#-license)

---

## ✨ Features

`gitz-cli` is designed for anyone working with Git who wants AI assistance for commit messages. It seamlessly integrates into your development workflow, primarily by running it manually before `git commit`. The most critical aspect of the AI-generated commit messages is their accuracy and relevance to the code changes.

🎯 **AI-Powered Commit Messages**: Automatically generates highly relevant and accurate Git commit messages by analyzing your staged changes, reducing manual effort and improving commit quality.

⚡ **Pluggable AI Providers**: Comes with built-in support for multiple AI services like Google Gemini and Cerebras, allowing you to choose your preferred backend.

📦 **Smart Diff Filtering**: Intelligently extracts and filters Git diffs, ignoring irrelevant files and boilerplate code (e.g., `node_modules`, `.env`) to provide the AI with only the most crucial context.

🔧 **CLI-First Experience**: A fast, interactive command-line interface makes it easy to integrate into your existing Git workflow, enhancing productivity without disrupting your flow.

---

## 🚀 Quick Start

Get `gitz-cli` up and running in under a minute!

First, ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
# Install gitz-cli from crates.io
cargo install gitz-cli

# Set your AI API key (e.g., for Gemini)
# You can get one from Google AI Studio: https://makersuite.google.com/app/apikey
export GEMINI_API_KEY="YOUR_GEMINI_API_KEY"
# For Cerebras, set CEREBRAS_API_KEY="YOUR_CEREBRAS_API_KEY"

# Navigate to your Git repository and stage some changes
git add .

# Generate a commit message
gitz-cli commit

# This will copy the generated message to your clipboard.
# Now, simply run `git commit` and paste the message.
git commit
```

---

## 📦 Installation

### Prerequisites

*   **Rust Toolchain**: `gitz-cli` is built with Rust. You'll need `rustup` to manage your Rust installations.
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
    Ensure you have Rust 1.70.0 or newer.

### From Crates.io (Recommended)

The easiest way to install `gitz-cli` is via `cargo`, Rust's package manager.

```bash
cargo install gitz-cli
```

### From Source

If you prefer to build from source, or want to contribute to the project, you can clone the repository and build it yourself.

```bash
# Clone the repository
git clone https://github.com/Tenuka22/gitz.git
cd gitz

# Build and install locally
cargo install --path .
```

### Pre-built Binaries

Pre-built binaries for Linux, Windows, and macOS are available on the [GitHub Releases page](https://github.com/Tenuka22/gitz/releases). Download the appropriate binary for your system and add it to your system's `PATH`.

---

## 💻 Usage

`gitz-cli` is a command-line tool designed to be run within your Git repository. The primary command is `commit`, which orchestrates the diff extraction, AI interaction, and message generation.

### CLI Commands

| Command             | Description                                                            | Example Usage                   |
| :------------------ | :--------------------------------------------------------------------- | :------------------------------ |
| `gitz-cli commit`   | Generates an AI-powered commit message based on staged Git changes.    | `gitz-cli commit`               |
| `gitz-cli diff`     | Shows the filtered Git diff that would be sent to the AI.              | `gitz-cli diff`                 |
| `gitz-cli providers`| Lists available AI providers and their configuration status.           | `gitz-cli providers`            |
| `gitz-cli --version`| Displays the current version of `gitz-cli`.                            | `gitz-cli --version`            |
| `gitz-cli --help`   | Shows general help message.                                            | `gitz-cli --help`               |
| `gitz-cli [CMD] --help` | Shows help for a specific command.                                 | `gitz-cli commit --help`        |

### Generating a Commit Message

To generate a commit message, simply run `gitz-cli commit` after staging your changes.

```bash
# 1. Make your code changes
# 2. Stage your changes as usual
git add src/main.rs src/lib.rs

# 3. Run gitz-cli to generate a message
$ gitz-cli commit
✨ Analyzing staged changes...
📦 Filtering diff (excluding target/, Cargo.lock)...
🚀 Sending diff to AI (Gemini)...
AI responded with a commit message:

feat: Implement AI commit message generation

This commit introduces the core functionality for generating
AI-powered commit messages based on staged changes.

- Added `clap` for CLI argument parsing.
- Integrated with `gemini-rust` for AI interaction.
- Implemented diff filtering logic to focus on relevant changes.

> Use this message? (Y/n): Y
Commit message copied to clipboard. Paste into your Git editor.

# 4. Now, commit with the generated message
git commit -m "feat: Implement AI commit message generation

This commit introduces the core functionality for generating
AI-powered commit messages based on staged changes.

- Added `clap` for CLI argument parsing.
- Integrated with `gemini-rust` for AI interaction.
- Implemented diff filtering logic to focus on relevant changes."
```

---

## ⚙️ Configuration

`gitz-cli` primarily uses environment variables for configuring AI provider API keys.

| Environment Variable  | Description                                                                                                     | Example Value                   |
| :-------------------- | :-------------------------------------------------------------------------------------------------------------- | :------------------------------ |
| `GEMINI_API_KEY`      | Your API key for the Google Gemini AI service. Required if using Gemini.                                        | `AIzaSyB...`                    |
| `CEREBRAS_API_KEY`    | Your API key for the Cerebras AI service. Required if using Cerebras.                                           | `cbk-...`                       |
| `GITZ_AI_PROVIDER`    | (Optional) Specifies the default AI provider to use. Defaults to `gemini` if `GEMINI_API_KEY` is set.           | `gemini`, `cerebras`            |
| `GITZ_MAX_DIFF_TOKENS`| (Optional) Maximum number of tokens for the diff sent to the AI. Helps prevent large diffs from exceeding limits. | `8000` (default)                |
| `GITZ_DIFF_EXCLUDE`   | (Optional) Comma-separated list of glob patterns to exclude from the diff analysis (e.g., `*.log,target/`).     | `.env,node_modules,target/`     |

> 💡 **Tip**: For persistent environment variables, add them to your shell's configuration file (e.g., `.bashrc`, `.zshrc`, `.profile`) or use a tool like [`direnv`](https://direnv.net/).

---

## 🤝 Contributing

Contributions are welcome! Whether it's reporting bugs, suggesting features, or submitting pull requests, your help makes `gitz-cli` better for everyone.

For substantial changes, please open an issue first to discuss what you would like to change.

### Development Setup

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/Tenuka22/gitz.git
    cd gitz
    ```
2.  **Build the project**:
    ```bash
    cargo build
    ```
3.  **Run tests**:
    ```bash
    cargo test
    ```
4.  **Run locally**:
    ```bash
    cargo run -- commit
    ```

Please ensure your code adheres to Rust's official style guidelines and passes all tests. This project was last updated by Tenuka22 on `959f57f`, which added support for multiple AI providers.

---

## 📝 License

This project is licensed under the MIT License.

Copyright (c) 2024 Tenuka22
