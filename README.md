# 🚀 gitz-cli

> ✨ AI-powered Git commit message generation for streamlined workflows and improved commit hygiene.

![Build](https://img.shields.io/github/actions/workflow/status/Tenuka22/gitz/ci.yml?style=flat-square)
[![Version](https://img.shields.io/crates/v/gitz-cli?style=flat-square)](https://crates.io/crates/gitz-cli)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Language](https://img.shields.io/badge/rust-1.70+-orange?style=flat-square)](https://www.rust-lang.org/)

```bash
# Example: Generate a commit message for your staged changes
gitz-cli --staged

# Expected output (example)
#
# Generating commit message with Cerebras...
#
# feat: Add new user authentication module
#
# This commit introduces a complete user authentication module,
# including signup, login, and session management functionalities.
# It uses secure password hashing and JWT for session tokens.
#
# Implements: #123
```

---

## 🎯 Table of Contents

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

`gitz-cli` is a command-line interface tool designed to assist individual developers in generating intelligent, context-aware Git commit messages with the help of Artificial Intelligence. It focuses solely on this core functionality to enhance your commit hygiene without additional Git-related complexities.

-   🎯 **AI-Powered Commit Generation**: Generate smart, relevant commit messages using advanced AI models from providers like Gemini and Cerebras.
-   ⚡ **Intelligent Diff Extraction & Filtering**: Automatically extracts Git differences (staged or HEAD), filters content, and limits size to ensure optimal AI processing and stay within token limits.
-   🛠️ **Flexible AI Provider Selection**: Easily switch between different AI providers based on your preference for factors like response speed.
-   ⚙️ **Environment Variable Configuration**: Securely manage your API keys and settings through environment variables for seamless integration into your workflow.
-   📦 **Cross-Platform Compatibility**: Available as pre-built binaries for Linux, Windows, and macOS, thanks to GitHub Actions.

---

## 🚀 Quick Start

Get `gitz-cli` up and running in under a minute!

```bash
# 1. Install gitz-cli via Cargo (Rust's package manager)
cargo install gitz-cli

# 2. Set your AI provider API key (example for Gemini)
#    Replace 'YOUR_GEMINI_API_KEY_HERE' with your actual key.
#    You might add this to your shell profile (.bashrc, .zshrc) for persistence.
export GEMINI_API_KEY="YOUR_GEMINI_API_KEY_HERE"

# 3. Navigate to a Git repository, stage some changes, and generate a commit message
cd my-rust-project
git add .
gitz-cli --staged

# Alternatively, generate a commit message for changes relative to HEAD
# gitz-cli --head
```

---

## 📦 Installation

### Prerequisites

You'll need a Rust toolchain installed to build `gitz-cli` from source or use `cargo install`. Rust version 1.70.0 or newer is recommended.

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### From `crates.io`

The easiest way to install `gitz-cli` is through `cargo`, Rust's package manager:

```bash
cargo install gitz-cli
```

### From Source

If you prefer to build from source or contribute to the project, you can clone the repository and build it yourself.

```bash
# Clone the repository
git clone https://github.com/Tenuka22/gitz.git
cd gitz

# Build the project
cargo build --release

# The executable will be available at ./target/release/gitz-cli
# You might want to add this directory to your PATH:
# export PATH="$(pwd)/target/release:$PATH"
```

### Pre-built Binaries

Pre-built binaries for various platforms (Linux, Windows, macOS) are available in the [GitHub Releases](https://github.com/Tenuka22/gitz/releases) section. Download the appropriate archive for your system, extract the `gitz-cli` executable, and place it in a directory included in your system's `PATH`.

---

## 💻 Usage

`gitz-cli` offers a straightforward command-line interface. The primary options allow you to specify which Git diff to analyze and which AI provider to use.

### Basic Commands

```bash
gitz-cli --help
```

```
gitz-cli 0.1.0
Tenuka22 <tenukaomaljith2009@gmail.com>
AI-powered Git commit message generation

Usage: gitz-cli [OPTIONS]

Options:
  -s, --staged                Generate a commit message for staged changes
  -H, --head                  Generate a commit message for changes relative to HEAD
  -p, --provider <PROVIDER>   AI provider to use (e.g., "gemini", "cerebras") [default: gemini]
  -l, --limit <LIMIT>         Maximum length of the diff content to send to the AI model [default: 4000]
  -v, --verbose               Enable verbose logging
  -h, --help                  Print help
  -V, --version               Print version
```

### Examples

#### Generate for Staged Changes

This is the most common use case, generating a commit message for the changes you've already added to your Git staging area.

```bash
git add src/main.rs
git add Cargo.toml
gitz-cli --staged
```

#### Generate for Changes Relative to HEAD

To get a commit message for all unstaged and uncommitted changes in your working directory (compared to the last commit):

```bash
# Make some changes in your files but don't stage them
gitz-cli --head
```

#### Specifying an AI Provider

You can explicitly choose which AI model to use. Currently, `gemini` and `cerebras` are supported. The default is `gemini`.

```bash
# Use the Cerebras provider
gitz-cli --staged --provider cerebras
```

#### Limiting Diff Size

To manage AI token limits or focus on smaller changes, you can limit the characters sent from the diff.

```bash
# Generate for staged changes, sending a maximum of 2000 characters of the diff
gitz-cli --staged --limit 2000
```

#### Verbose Output

For debugging or more detailed information about the process, enable verbose logging.

```bash
gitz-cli --staged --verbose
```

---

## ⚙️ Configuration

`gitz-cli` uses environment variables to configure AI API keys. These should be set in your shell's profile file (e.g., `.bashrc`, `.zshrc`, `config.fish`) or directly in your CI/CD environment.

| Environment Variable | Description                                    | Required |
| :------------------- | :--------------------------------------------- | :------- |
| `GEMINI_API_KEY`     | Your API key for the Google Gemini AI service. | Yes      |
| `CEREBRAS_API_KEY`   | Your API key for the Cerebras AI service.      | Yes      |

> ⚠️ **Important**: Never hardcode API keys directly into your scripts or commit them to version control. Always use environment variables for sensitive information.

Example for `~/.bashrc` or `~/.zshrc`:

```bash
# .bashrc or .zshrc
export GEMINI_API_KEY="your_actual_gemini_api_key_here"
export CEREBRAS_API_KEY="your_actual_cerebras_api_key_here"

# Reload your shell profile after adding the variables
# source ~/.bashrc
```

---

## 📖 Examples

Here are a few more comprehensive scenarios to demonstrate `gitz-cli`'s capabilities.

### Scenario 1: Developing a new feature

You're working on a new feature, making changes across several files.

```bash
# 1. Make changes to multiple files, e.g., add a new function
#    src/lib.rs (new function)
#    tests/test_feature.rs (new test)

# 2. Stage your changes
git add src/lib.rs tests/test_feature.rs

# 3. Ask gitz-cli to generate a commit message for these staged changes
gitz-cli --staged
# Expected output might look something like:
#
# Generating commit message with Gemini...
#
# feat: Implement secure data encryption utility
#
# This commit introduces a new data encryption utility using AES-256 GCM.
# It includes functions for encrypting and decrypting data, along with
# unit tests to ensure correctness and security.
#
# Resolves: #42
```

### Scenario 2: Fixing a bug

You've identified and fixed a bug in a specific module.

```bash
# 1. Modify src/buggy_module.rs to fix the bug
# 2. Stage only the fix
git add src/buggy_module.rs

# 3. Generate a commit message focused on the fix
gitz-cli --staged --provider cerebras
# Expected output:
#
# Generating commit message with Cerebras...
#
# fix: Correct off-by-one error in pagination logic
#
# Addressed an issue where pagination for search results
# was incorrectly calculating the last page, leading to missing
# items on the final page. The index calculation has been adjusted.
#
# Closes: #88
```

### Scenario 3: Reviewing unstaged changes

You have several changes but haven't decided what to commit yet. You want a quick summary of everything currently modified.

```bash
# 1. Make various modifications, but don't git add anything
# 2. Use --head to analyze all changes relative to the last commit
gitz-cli --head
# Expected output:
#
# Generating commit message with Gemini...
#
# chore: Update dependencies and refactor logging
#
# Updated several project dependencies to their latest versions,
# including `serde` and `tokio`. Also refactored the internal
# logging mechanism to use `env_logger` more consistently.
```

---

## 🤝 Contributing

`gitz-cli` is developed by Tenuka22. Contributions are welcome! If you have suggestions, bug reports, or want to contribute code, please feel free to open an issue or pull request on the [GitHub repository](https://github.com/Tenuka22/gitz).

### Development Setup

To set up your development environment:

1.  Fork the `gitz` repository.
2.  Clone your forked repository:
    ```bash
    git clone https://github.com/YOUR_USERNAME/gitz.git
    cd gitz
    ```
3.  Ensure you have Rust and Cargo installed.
4.  Build the project:
    ```bash
    cargo build
    ```

### Running Tests

To run the test suite:

```bash
cargo test
```

---

## 📝 License

This project is licensed under the MIT License.

Copyright (c) 2023 Tenuka22.

See the [LICENSE](https://github.com/Tenuka22/gitz/blob/master/LICENSE) file for full details.
