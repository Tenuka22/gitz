# gitz

A CLI tool that enhances your Git workflows with powerful AI capabilities, primarily leveraging the Gemini API.

## Features

-   **AI-Powered Commit Messages**: Automatically generate conventional Git commit messages from your codebase diffs. It supports analyzing either staged changes or all changes (staged and unstaged) to provide relevant and structured suggestions.
-   **GitHub README Generation**: Instantly create comprehensive `README.md` files for your repositories by analyzing their existing content and structure.
-   **Intelligent Diff Filtering**: Filters and optimizes Git diff content to ensure it fits within AI model token limits and maintains relevance for accurate AI processing.
-   Seamlessly integrates with underlying Git commands to efficiently retrieve necessary diffs and repository file contents for its operations.

## Installation

As `gitz` is built with Rust, you can install it using `cargo` if you have Rust and Cargo installed on your system:

```bash
cargo install gitz
```

Alternatively, you can build the project from source:

```bash
git clone https://github.com/Tenuka22/gitz.git
cd gitz
cargo build --release
# The executable will be located in target/release/gitz.
# Consider adding target/release to your system's PATH, or copy the gitz executable to a directory already in your PATH.
```

## Configuration

`gitz` relies on the Gemini API for its AI functionalities. You need to provide your Gemini API key as an environment variable. The recommended approach is to create a `.env` file in the root directory of your project or in a globally accessible location with the following content:

```ini
GEMINI_API_KEY=YOUR_GEMINI_API_KEY
```

Replace `YOUR_GEMINI_API_KEY` with the actual API key you obtain from the Google AI Studio or your Gemini API provider.

## Usage

`gitz` offers straightforward commands for generating commit messages and README files.

### Generate Git Commit Messages

To generate a conventional Git commit message based on your code changes:

-   **For staged changes only:**
    ```bash
    gitz commit --staged
    ```
-   **For all changes (staged and unstaged):**
    ```bash
    gitz commit --all
    ```
    The tool will analyze your diffs and propose a commit message, which you can then review and use.

### Generate GitHub README

To automatically generate a `README.md` file for your current repository:

```bash
gitz readme
```
This command will analyze the contents of your repository and generate a `README.md` file in your current working directory.

## Tech Stack

`gitz` is built using the following technologies:

-   [Rust](https://www.rust-lang.org/): A language empowering everyone to build reliable and efficient software.
-   [clap](https://crates.io/crates/clap): A full-featured, fast, and easy-to-use argument parser for Rust.
-   [dotenvy](https://crates.io/crates/dotenvy): A lightweight library for loading environment variables from `.env` files.
-   [gemini-rust](https://crates.io/crates/gemini-rust): A Rust client for interacting with the Gemini API.
-   [serde](https://crates.io/crates/serde) & [serde_json](https://crates.io/crates/serde_json): A powerful and generic serialization/deserialization framework for Rust data structures, with a focus on JSON.
-   [tempfile](https://crates.io/crates/tempfile): Provides utilities for creating temporary files and directories.
-   [tokio](https://crates.io/crates/tokio): An asynchronous runtime for writing network applications in Rust.

## Contributing

Contributions are highly encouraged! Whether it's reporting bugs, suggesting new features, or submitting pull requests, your input is valuable. Please feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License.
