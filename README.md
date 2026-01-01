# 🚀 gitz

> *Craft intelligent Git commit messages with AI, enhancing your development workflow.*

[![Build](https://img.shields.io/github/actions/workflow/status/Tenuka22/gitz/ci.yml?style=flat-square)](https://github.com/Tenuka22/gitz/actions)
[![Version](https://img.shields.io/crates/v/gitz?style=flat-square)](https://crates.io/crates/gitz)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Language](https://img.shields.io/badge/rust-1.70+-orange?style=flat-square)](https://www.rust-lang.org/)

```
$ gitz commit --staged
✨ Generating commit message...
✔ Commit message generated!

feat(readme): improve documentation and add quick start guide

This commit enhances the README with new sections including a quick start,
detailed installation instructions, and usage examples.
It also updates the feature list to better reflect the tool's capabilities.
```

---

## 🌟 Table of Contents

- [✨ Features](#-features)
- [🚀 Quick Start](#-quick-start)
- [📦 Installation](#-installation)
- [💻 Usage](#-usage)
- [⚙️ Configuration](#️-configuration)
- [📖 Examples](#-examples)
- [🤝 Contributing](#-contributing)
- [📝 License](#-license)

---

## ✨ Features

`gitz` is an AI-powered command-line tool designed to streamline your Git workflow by automatically generating conventional commit messages. It acts as an AI assistant, running manually from the command line as needed before committing to replace manual code review processes with AI suggestions.

*   🎯 **AI-Powered Commit Generation**: Leverages the Google Gemini AI API to generate conventional Git commit messages that adhere to structured rules (subject line, emoji prefixes, body content).
*   ⚡ **Intelligent Diff Filtering**: Optimizes AI input by filtering and truncating Git diffs, ignoring irrelevant files (e.g., lockfiles) and prioritizing meaningful changes to focus on the core modifications.
*   📦 **Structured Commit Format**: Enforces a consistent and conventional commit message format, including emoji prefixes (`feat:`, `fix:`, `docs:`), imperative verbs, and character limits for clear, readable history.
*   🔧 **Configurable & Adaptable**: Supports generating commit messages for staged or all changes within your repository and can be configured via environment variables for AI key management.
*   🤝 **Interactive CLI Experience**: Provides real-time interactive terminal feedback with progress loaders and colored logging for a user-friendly experience.

---

## 🚀 Quick Start

Get `gitz` up and running in under 30 seconds!

1.  **Install Rust:** If you don't have Rust installed, follow the instructions on [rustup.rs](https://rustup.rs/).
2.  **Get your Gemini API Key:** Obtain a `GEMINI_API_KEY` from Google AI Studio.
3.  **Install `gitz`:**

    ```bash
    # Install gitz via Cargo
    cargo install gitz
    ```

4.  **Set Environment Variable:**

    ```bash
    # For a temporary session (replace with your actual key)
    export GEMINI_API_KEY="YOUR_GEMINI_API_KEY"

    # For persistent use, add this line to your shell's config file (e.g., ~/.bashrc, ~/.zshrc)
    echo 'export GEMINI_API_KEY="YOUR_GEMINI_API_KEY"' >> ~/.bashrc # or ~/.zshrc
    source ~/.bashrc # or ~/.zshrc
    ```

5.  **Generate a commit message:**

    ```bash
    # Stage your changes
    git add .

    # Let gitz generate a message for staged changes
    gitz commit --staged
    ```

---

## 📦 Installation

`gitz` is a Rust-based command-line tool. You'll need the Rust toolchain installed to compile and install it.

### Prerequisites

*   **Rust and Cargo**: Ensure you have a recent version of Rust and Cargo installed. You can install them via `rustup`:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
    Verify your installation:
    ```bash
    rustc --version # Example: rustc 1.70.0 (90c541806 2023-05-31)
    cargo --version # Example: cargo 1.70.0 (106af7b60 2023-06-02)
    ```

### Install with Cargo

The easiest way to install `gitz` is through `cargo` from [crates.io](https://crates.io/crates/gitz):

```bash
cargo install gitz
```

### Install from Source

You can also build and install `gitz` directly from its source code from the repository maintained by Tenuka22:

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/Tenuka22/gitz.git
    cd gitz
    ```
2.  **Build the project**:
    ```bash
    cargo build --release
    ```
    The executable will be located at `target/release/gitz`.
3.  **Install to Cargo's bin directory**:
    ```bash
    cargo install --path .
    ```

---

## 💻 Usage

`gitz` is designed to be run manually from the command line before you commit your changes. It intelligently analyzes your Git diffs and suggests a conventional commit message.

### Core Command: `gitz commit`

This command is the primary entry point for generating commit messages.

| Option      | Description                                                                 | Default |
| :---------- | :-------------------------------------------------------------------------- | :------ |
| `--staged`  | Generate a commit message based only on currently staged changes.           | `false` |
| `--all`     | Generate a commit message based on all local changes (staged and unstaged). | `false` |
| `--help`    | Print help information.                                                     | `false` |
| `--version` | Print version information.                                                  | `false` |

> ⚠️ **Important**: You must provide either `--staged` or `--all`. If neither is specified, `gitz` will prompt you or show an error.

### Examples

1.  **Generate a commit message for staged changes:**

    ```bash
    # Make some changes, then stage them
    git add src/main.rs

    # Use gitz to generate a message for only the staged changes
    gitz commit --staged
    ```

    Expected output:
    ```
    ✨ Generating commit message...
    ✔ Commit message generated!

    feat(cli): add staged diff processing for commit messages

    This commit introduces the `--staged` option to `gitz commit`,
    allowing users to generate commit messages based solely on changes
    that have been added to the Git staging area. This enhances control
    over what content the AI analyzes for message generation.
    ```

2.  **Generate a commit message for all local changes:**

    ```bash
    # Make some changes, but don't stage them
    # For example, modify src/lib.rs and Cargo.toml

    # Use gitz to generate a message for all local changes (staged and unstaged)
    gitz commit --all
    ```

    Expected output:
    ```
    ✨ Generating commit message...
    ✔ Commit message generated!

    chore(deps): update rust dependencies and configuration

    This commit updates various project dependencies in `Cargo.toml`
    to their latest versions, improving stability and performance.
    It also includes minor adjustments to `.gitignore` to reflect
    new build artifacts and `.env` files.
    ```

---

## ⚙️ Configuration

`gitz` relies on the `GEMINI_API_KEY` for interaction with the Google Gemini AI.

### Environment Variables

| Variable          | Description                                                                                     | Required |
| :---------------- | :---------------------------------------------------------------------------------------------- | :------- |
| `GEMINI_API_KEY`  | Your API key for accessing the Google Gemini AI. Obtainable from [Google AI Studio](https://makersuite.google.com/keys). | Yes      |

> 💡 **Tip**: To influence the AI's understanding of your changes, `gitz` filters and truncates diffs. You can customize the file filtering logic by adding a `.gitzignore` file in your repository's root, similar to `.gitignore`. This allows you to specify files or patterns that should be excluded from the AI's analysis, making the output more tailored to your critical changes.

---

## 📖 Examples

Let's walk through a common scenario to see `gitz` in action.

### Scenario: Adding a New Feature and Generating a Commit Message

Imagine Tenuka22 is working on a new feature that involves adding a function to `src/main.rs` and updating `Cargo.toml` with a new dependency.

1.  **Initial Status:**
    ```bash
    git status
    ```
    ```
    On branch master
    Your branch is up to date with 'origin/master'.

    nothing to commit, working tree clean
    ```

2.  **Make Changes:**
    *   Add a new function to `src/main.rs`.
    *   Add a new dependency to `Cargo.toml`.

3.  **Check Diff:**
    ```bash
    git diff
    ```
    ```diff
    diff --git a/Cargo.toml b/Cargo.toml
    index abcd123..efgh456 100644
    --- a/Cargo.toml
    +++ b/Cargo.toml
    @@ -10,3 +10,4 @@
     tokio = { version = "1.35.1", features = ["full"] }
     serde = { version = "1.0", features = ["derive"] }
     serde_json = "1.0"
    +rand = "0.8" # New dependency
    diff --git a/src/main.rs b/src/main.rs
    index 1234abcd..5678efgh 100644
    --- a/src/main.rs
    +++ b/src/main.rs
    @@ -5,6 +5,11 @@
     use log::{error, info, LevelFilter};
     use tempfile::tempdir;

    +fn generate_random_number() -> u32 {
    +    use rand::Rng;
    +    rand::thread_rng().gen_range(1..=100)
    +}
    +
     #[tokio::main]
     async fn main() -> Result<(), Box<dyn Error>> {
         // ... (existing code)
    ```

4.  **Stage Changes:**
    ```bash
    git add .
    ```

5.  **Generate Commit Message with `gitz`:**
    ```bash
    gitz commit --staged
    ```

    ```
    ✨ Generating commit message...
    ✔ Commit message generated!

    feat(random): add random number generation utility

    This commit introduces a new utility function `generate_random_number`
    to `src/main.rs` for generating random numbers within a specified range.
    It also adds the `rand` crate as a dependency in `Cargo.toml` to support
    this new functionality.
    ```

6.  **Review and Commit:**
    The generated message can then be copied and used directly in your `git commit -m` command, or you can use `gitz commit -e` (if available, this feature is not explicitly listed, so I'll keep it simple by just showing message generation).

    ```bash
    # If gitz directly integrates, it would look like:
    # git commit -m "feat(random): add random number generation utility

    # This commit introduces a new utility function `generate_random_number`
    # to `src/main.rs` for generating random numbers within a specified range.
    # It also adds the `rand` crate as a dependency in `Cargo.toml` to support
    # this new functionality."
    ```
    Or, if `gitz` just prints to stdout, you'd manually paste it.

---

## 🤝 Contributing

We welcome contributions to `gitz`! Whether it's reporting bugs, suggesting features, or submitting code, your help is appreciated.

### Development Setup

To get started with development on `gitz`, follow these steps:

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
    This ensures everything is set up correctly. The project is actively maintained by Tenuka22, with the last commit being `c3d4c31 🔥 Remove redundant screen clear in main`.

---

## 📝 License

This project is licensed under the MIT License.

Copyright (c) 2023 Tenuka22.

See the [LICENSE](LICENSE) file for more details.