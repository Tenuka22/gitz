# ✨ gitz ✨

*Effortlessly generate AI-powered Git commit messages and READMEs from your diffs.*

[![Build Status](https://img.shields.io/github/actions/workflow/status/YOUR_GITHUB_USER/gitz/ci.yml?style=flat-square)](https://github.com/YOUR_GITHUB_USER/gitz/actions)
[![Crates.io Version](https://img.shields.io/crates/v/gitz?style=flat-square)](https://crates.io/crates/gitz)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange?style=flat-square)](https://www.rust-lang.org)

```bash
$ gitz commit
✨ Generating commit message for staged changes...

[main d1e7f2b] feat: Implement AI-powered commit message generation
 1 file changed, 10 insertions(+)
```
---

## 🎯 Table of Contents

- [✨ Features](#-features)
- [🚀 Quick Start](#-quick-start)
- [📦 Installation](#-installation)
- [💻 Usage](#-usage)
  - [`gitz commit`](#-gitz-commit)
  - [`gitz readme`](#-gitz-readme)
  - [CLI Commands](#-cli-commands)
- [⚙️ Configuration](#️-configuration)
- [📖 Examples](#-examples)
- [🤝 Contributing](#-contributing)
- [📝 License](#-license)

---

## ✨ Features

- **AI-powered Commit Messages**: Automatically craft descriptive Git commit messages based on your staged or unstaged changes.
- **Intelligent Diff Filtering**: Prioritizes and filters Git diff content to provide the most relevant context for AI processing.
- **Gemini AI Integration**: Seamlessly leverages the Gemini AI API for robust content generation.
- **README Generation**: Generate initial `README.md` files for your projects based on the current repository context.
- **Flexible Scope**: Supports processing both **staged and unstaged changes** using the `--all` flag.
- **Direct Git Integration**: By default, `gitz commit` writes the generated message directly into your Git commit.

---

## 🚀 Quick Start

Get up and running with `gitz` in no time!

1.  **Install `gitz`**:
    ```bash
    cargo install gitz
    ```

2.  **Set your Gemini API Key**:
    ```bash
    export GEMINI_API_KEY="YOUR_GEMINI_API_KEY"
    ```
    > ⚠️ You can obtain your API key from the [Google AI Studio](https://makersuite.google.com/app/apikey).

3.  **Generate a commit message**:
    Stage your changes as usual, then run:
    ```bash
    git add .
    gitz commit
    ```
    `gitz` will analyze your staged changes, generate a commit message, and automatically apply it.

---

## 📦 Installation

### Prerequisites

Ensure you have a [Rust toolchain](https://www.rust-lang.org/tools/install) (version 1.70 or higher) installed on your system.

### Install via Cargo

The easiest way to install `gitz` is using Rust's package manager, `cargo`:

```bash
cargo install gitz
```

This command compiles `gitz` and places it in your Cargo bin directory (usually `~/.cargo/bin`), which should be in your system's `PATH`.

### From Source

If you prefer to build from source, follow these steps:

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/YOUR_GITHUB_USER/gitz.git
    cd gitz
    ```
2.  **Build and Install**:
    ```bash
    cargo install --path .
    ```

---

## 💻 Usage

`gitz` provides several commands to help you with your development workflow.

### `gitz commit`

Generates a Git commit message based on your changes and applies it.

```bash
# Generate a commit message for staged changes
# The message will be directly applied to your git commit.
git add .
gitz commit
```

```bash
# Generate a commit message for ALL changes (staged and unstaged)
# This is useful for quickly committing everything without staging explicitly.
gitz commit --all
```

#### Expected Output (Commit message generation)

```bash
$ gitz commit
✨ Generating commit message for staged changes...

[main d1e7f2b] feat: Implement AI-powered commit message generation
 1 file changed, 10 insertions(+)
```

### `gitz readme`

Generates an initial `README.md` file for your project based on the repository's content.

```bash
# Generate a README.md file in the current directory
gitz readme
```

#### Expected Output (README generation)

```bash
$ gitz readme
✨ Generating README.md...
📄 README.md generated successfully!
```

### CLI Commands

| Command    | Description                                                 | Options                       |
| :--------- | :---------------------------------------------------------- | :---------------------------- |
| `commit`   | Generates and applies an AI-powered Git commit message.     | `--all`: Include unstaged changes. |
| `readme`   | Generates an initial `README.md` file for the repository.   | None                          |
| `--version`| Prints the current version of `gitz`.                       |                               |
| `--help`   | Displays help information for `gitz` or a subcommand.       |                               |

---

## ⚙️ Configuration

`gitz` requires a **Gemini API Key** to interact with the Gemini AI service. This key must be provided via an environment variable.

### `GEMINI_API_KEY`

This environment variable holds your API key for authentication with the Gemini AI API.

-   **Required**: Yes
-   **Example Value**: `AIzaSyB-YOUR_ACTUAL_API_KEY_HERE`

#### Setting the Environment Variable

1.  **Temporarily (for current session)**:
    ```bash
    export GEMINI_API_KEY="YOUR_GEMINI_API_KEY"
    gitz commit
    ```

2.  **Persistently (recommended)**:
    Add the `export` command to your shell's configuration file (e.g., `~/.bashrc`, `~/.zshrc`, `~/.profile`).

    ```bash
    # In ~/.bashrc or ~/.zshrc
    export GEMINI_API_KEY="YOUR_GEMINI_API_KEY"
    ```
    Remember to source your config file or open a new terminal session for changes to take effect: `source ~/.bashrc`.

3.  **Using a `.env` file**:
    You can also create a `.env` file in the root of your project or in the directory where you run `gitz`.
    ```
    # .env file content
    GEMINI_API_KEY="YOUR_GEMINI_API_KEY"
    ```
    `gitz` uses `dotenvy` to automatically load variables from a `.env` file if found.

---

## 📖 Examples

Here are some real-world scenarios demonstrating `gitz` in action.

### Scenario 1: Generating a commit message for a new feature

Imagine you've just added a new user authentication module.

```bash
# Make changes...
# For example, create src/auth.rs and modify src/main.rs
git add src/auth.rs src/main.rs

# Use gitz to generate and apply the commit message
gitz commit
```

**Potential Generated Commit:**

```
[main a2b3c4d] feat: Implement user authentication module with JWT
 2 files changed, 120 insertions(+)
 create mode 100644 src/auth.rs
```

### Scenario 2: Generating a commit message for a bug fix including unstaged changes

You've fixed a typo and a small logical bug, but haven't staged both changes yet.

```bash
# Make changes to fix a typo in README.md and a bug in src/parser.rs
# Only README.md is staged, src/parser.rs is unstaged.
git add README.md
git status
```
```
On branch main
Your branch is up to date with 'origin/main'.

Changes to be committed:
  (use "git restore --staged <file>..." to unstage)
        modified:   README.md

Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
        modified:   src/parser.rs
```

```bash
# Use gitz with --all to include both staged and unstaged changes
gitz commit --all
```

**Potential Generated Commit:**

```
[main e5f6g7h] fix: Correct typo in README and resolve parsing error
 2 files changed, 5 insertions(+), 2 deletions(-)
```

### Scenario 3: Generating a README for a new project

You've just initialized a new Rust project and want a basic `README.md`.

```bash
# Assume you have a new project structure:
# my_project/
# ├── Cargo.toml
# └── src/
#     └── main.rs

cd my_project
gitz readme
```

This will create a `README.md` file in the `my_project` directory, potentially outlining the project's purpose, installation, and basic usage based on `Cargo.toml` and `src/main.rs` content.

---

## 🤝 Contributing

We welcome contributions to `gitz`! If you're interested in improving this project, please consider:

-   Reporting bugs
-   Suggesting new features
-   Submitting pull requests

For more detailed information, please refer to our [CONTRIBUTING.md](CONTRIBUTING.md) guide.

### Development Setup

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/YOUR_GITHUB_USER/gitz.git
    cd gitz
    ```
2.  **Build the project**:
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

`gitz` is licensed under the **MIT License**.

See the [LICENSE](LICENSE) file for more details.

Copyright © 2023, Your Name or Organization.