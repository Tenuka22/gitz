# gitz

<p align="center">
  <img src="https://raw.githubusercontent.com/rust-lang/rust/master/src/doc/rust-logo.svg" alt="gitz Logo" width="120" />
</p>

<h1 align="center">✨ gitz ✨</h1>

<p align="center">
  <i>Supercharge your Git workflow with AI-powered commit messages and instant README generation.</i>
</p>

<p align="center">
  <a href="https://github.com/USER/REPO/actions/workflows/ci.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/USER/REPO/ci.yml?branch=main&style=flat-square&label=build" alt="Build Status">
  </a>
  <a href="https://crates.io/crates/gitz">
    <img src="https://img.shields.io/crates/v/gitz?style=flat-square&color=blue" alt="Crates.io Version">
  </a>
  <a href="https://github.com/USER/REPO/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square" alt="License: MIT">
  </a>
  <img src="https://img.shields.io/badge/rust-1.70%2B-orange?style=flat-square" alt="Rust Version">
</p>

```
    ____ _     _   _ ____  
   / ___| |_ _| |_| |___ \ 
  | |  _| __| '_| | | __) |
  | |_| | |_| | | | |/ ___/ 
   \____|\__|_| |_|_|_____|
                           
```

---

## ✨ Features

`gitz` brings AI capabilities directly into your Git workflow, streamlining two crucial aspects of repository management:

*   **AI-Powered Commit Messages**: Automatically generate conventional Git commit messages from your codebase diffs. It supports analyzing either staged changes or all changes (staged and unstaged) to provide relevant and structured suggestions.
*   **GitHub README Generation**: Instantly create comprehensive `README.md` files for your repositories by analyzing their existing content and structure.
*   **Intelligent Diff Filtering**: Filters and optimizes Git diff content to ensure it fits within AI model token limits and maintains relevance for accurate AI processing.
*   **Seamless Git Integration**: Seamlessly integrates with underlying Git commands to efficiently retrieve necessary diffs and repository file contents for its operations.

---

## 🚀 Quick Start

Get `gitz` up and running in a flash!

1.  **Install `gitz`**:

    ```bash
    cargo install gitz
    ```

2.  **Set your AI API Key**:
    `gitz` uses the Google Gemini API. Obtain an API key from [Google AI Studio](https://makersuite.google.com/app/apikey) and set it as an environment variable:

    ```bash
    export GEMINI_API_KEY="YOUR_GEMINI_API_KEY_HERE"
    ```

3.  **Generate a Commit Message**:
    Make some changes, stage them (`git add .`), then run:

    ```bash
    git add .
    gitz commit -s
    ```

    _Expected Output:_
    ```
    ? Use this commit message? (y/N)
    feat: Add initial AI-powered commit message generation feature

    This commit introduces the core functionality for gitz to analyze staged
    changes and generate a conventional commit message.
    ```

4.  **Generate a README**:
    Navigate to your project root and run:

    ```bash
    gitz readme
    ```

    _Expected Output:_
    ```
    README.md generated successfully!
    ```

---

## 📦 Installation

Before installing `gitz`, ensure you have the following prerequisites:

*   **Rust Toolchain**: `gitz` is a Rust application. You'll need the Rust compiler and Cargo (Rust's package manager) installed. If you don't have it, follow the instructions on [rustup.rs](https://rustup.rs/).
*   **Git**: `gitz` interacts with your Git repositories. Ensure Git is installed and available in your PATH.

### Install via Cargo

The easiest way to install `gitz` is directly from [crates.io](https://crates.io/crates/gitz) using Cargo:

```bash
cargo install gitz
```

### Build from Source

If you prefer to build `gitz` from source:

1.  **Clone the repository**:

    ```bash
    git clone https://github.com/USER/REPO.git
    cd REPO
    ```

2.  **Build and install**:

    ```bash
    cargo install --path .
    ```

---

## 💻 Usage

`gitz` provides two primary commands: `commit` for generating commit messages and `readme` for generating README files.

### 🤖 AI API Key

Before using any AI-powered features, ensure your `GEMINI_API_KEY` environment variable is set.

```bash
export GEMINI_API_KEY="YOUR_GEMINI_API_KEY_HERE"
```

### `gitz commit` - AI-Powered Commit Message Generation

Generates a conventional commit message based on your Git changes.

#### Options

*   `-s`, `--staged`: Analyze only staged changes (default).
*   `-a`, `--all`: Analyze all changes (staged and unstaged).
*   `-m`, `--model <MODEL_NAME>`: Specify the AI model to use (e.g., `gemini-pro`). Overrides `GITZ_DEFAULT_MODEL`.
*   `--max-tokens <COUNT>`: Set the maximum number of tokens for the AI response. Overrides `GITZ_DEFAULT_MAX_TOKENS`.
*   `--temperature <VALUE>`: Set the AI generation temperature (0.0-1.0). Overrides `GITZ_DEFAULT_TEMPERATURE`.
*   `--conventional`: Generate a strictly conventional commit message (type, scope, subject).

#### Examples

1.  **Generate a commit message for staged changes (default)**:

    ```bash
    git add .
    gitz commit
    ```

2.  **Generate a commit message for all changes (staged and unstaged)**:

    ```bash
    gitz commit -a
    ```

3.  **Specify a different AI model**:

    ```bash
    gitz commit -s --model gemini-1.5-flash
    ```

4.  **Force a strictly conventional commit format**:

    ```bash
    gitz commit -s --conventional
    ```

### `gitz readme` - AI-Powered README Generation

Generates a comprehensive `README.md` file for your current repository.

#### Options

*   `-o`, `--output <FILE_PATH>`: Specify the output file path (default: `README.md` in the current directory).
*   `-m`, `--model <MODEL_NAME>`: Specify the AI model to use. Overrides `GITZ_DEFAULT_MODEL`.
*   `--max-tokens <COUNT>`: Set the maximum number of tokens for the AI response. Overrides `GITZ_DEFAULT_MAX_TOKENS`.
*   `--temperature <VALUE>`: Set the AI generation temperature. Overrides `GITZ_DEFAULT_TEMPERATURE`.
*   `-f`, `--force`, `--overwrite`: Overwrite an existing `README.md` without prompting.

#### Examples

1.  **Generate `README.md` in the current directory**:

    ```bash
    gitz readme
    ```

2.  **Generate and overwrite an existing `README.md`**:

    ```bash
    gitz readme --force
    ```

3.  **Generate a README to a specific file**:

    ```bash
    gitz readme --output docs/PROJECT.md
    ```

---

## ⚙️ Configuration

`gitz` primarily uses environment variables for configuration. These can be set directly in your shell or via a `.env` file loaded by `dotenvy`.

| Environment Variable          | Description                                                    | Default           |
| :---------------------------- | :------------------------------------------------------------- | :---------------- |
| `GEMINI_API_KEY`              | **Required**. Your Google Gemini API key.                      | `None`            |
| `GITZ_DEFAULT_MODEL`          | The default AI model to use for generation.                    | `gemini-pro`      |
| `GITZ_DEFAULT_MAX_TOKENS`     | The default maximum tokens for AI responses.                   | `1000`            |
| `GITZ_DEFAULT_TEMPERATURE`    | The default AI generation temperature (0.0-1.0).               | `0.7`             |
| `GITZ_DISABLE_COLOR`          | Set to `1` or `true` to disable colored output in the terminal.| `false`           |

### Example `.env` file

Create a file named `.env` in your project root or home directory:

```ini
GEMINI_API_KEY="YOUR_GEMINI_API_KEY_HERE"
GITZ_DEFAULT_MODEL="gemini-1.5-flash"
GITZ_DEFAULT_MAX_TOKENS="2000"
GITZ_DEFAULT_TEMPERATURE="0.5"
```

---

## 📖 Examples

### 1. Generating a Commit Message for New Feature Development

Let's say you've implemented a new user authentication module.

```bash
# Make changes and add them to the staging area
git add src/auth.rs src/main.rs Cargo.toml

# Let gitz suggest a commit message
gitz commit -s
```

_`gitz` might suggest something like:_
```
? Use this commit message? (y/N) y
feat(auth): Implement user authentication module

This commit introduces a new user authentication module, allowing users to
log in and manage sessions. It includes JWT token generation and validation.
```

### 2. Creating a README for a Fresh Repository

Start a new project, commit some initial files, and then generate a README.

```bash
# Initialize git and add some initial files (e.g., src/main.rs, Cargo.toml)
mkdir my-new-project && cd my-new-project
git init
echo "fn main() { println!(\"Hello, gitz!\"); }" > src/main.rs
cargo init --bin
git add .
git commit -m "feat: Initial project setup"

# Now, generate the README
gitz readme
```

_`gitz` will analyze your project and create a comprehensive `README.md`._

```markdown
# My New Project

A simple Rust application demonstrating basic functionality.

## Features
- Prints "Hello, gitz!" to the console.
- Basic Cargo project structure.

## Quick Start
...
```

### 3. Overriding AI Parameters for Specific Tasks

You might want a more creative README or a very concise commit message.

```bash
# For a more creative README, increase temperature:
gitz readme --temperature 0.9 --output README_creative.md

# For a very focused, short commit message, decrease max tokens:
git add .
gitz commit -s --max-tokens 50
```

---

## 🤝 Contributing

We welcome contributions from the community! If you're interested in improving `gitz`, please check out our [CONTRIBUTING.md](CONTRIBUTING.md) guide.

### Development Setup

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/USER/REPO.git
    cd REPO
    ```
2.  **Install dependencies**:
    Rust's Cargo will automatically handle dependencies when you build or run.
3.  **Run tests**:
    ```bash
    cargo test
    ```

---

## 📝 License

This project is licensed under the **MIT License**.

See the [LICENSE](LICENSE) file for details.

Copyright © 2023-present The `gitz` Contributors.

---