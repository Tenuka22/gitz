# `gitz-cli` Usage Guide

This guide provides detailed instructions on how to install, configure, and use `gitz-cli`.

## 🚀 Installation

There are two main ways to install `gitz-cli`: from source using Cargo, or by downloading a pre-compiled binary.

### From Source (with Rust)

If you have the Rust toolchain installed, you can easily install and run `gitz-cli` from its source code.

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/Tenuka22/gitz.git
    cd gitz
    ```

2.  **Build the project**:
    This will compile the project and place the executable in the `target/release/` directory.
    ```bash
    cargo build --release
    ```

3.  **Run the built executable directly**:
    After building, you can run the executable from the `target/release/` directory.
    *   **Linux/macOS:**
        ```bash
        ./target/release/gitz-cli commit stage
        ```
    *   **Windows:**
        ```powershell
        .\target\release\gitz-cli.exe commit stage
        ```

4.  **Install to Cargo's bin directory (recommended for system-wide use):**
    This will compile (if not already built) and copy the executable to your Cargo bin directory (e.g., `~/.cargo/bin` on Linux/macOS, or `%USERPROFILE%\.cargo\bin` on Windows). Ensure this directory is in your system's PATH.
    ```bash
    cargo install --path .
    ```
    After this, you can run `gitz-cli commit stage` from any directory.

### From Pre-compiled Binaries (GitHub Releases)

For users who don't have Rust installed, pre-compiled binaries are available for Windows, Linux, and macOS from the [GitHub Releases page](https://github.com/Tenuka22/gitz/releases).

1.  **Download the binary for your platform:**
    *   Go to the [latest release](https://github.com/Tenuka22/gitz/releases/latest).
    *   In the "Assets" section, find the binary for your operating system (e.g., `gitz-cli-windows-latest.exe`, `gitz-cli-ubuntu-latest`, `gitz-cli-macos-latest`).
    *   Download the file.

2.  **Make it executable (for Linux/macOS):**
    After downloading, you'll need to make the binary executable.
    ```bash
    chmod +x gitz-cli-ubuntu-latest
    ```

3.  **Rename and add to PATH (optional but recommended):**
    For easier use, you can rename the binary and move it to a directory that is in your system's PATH.

    **For Linux/macOS:**
    ```bash
    # Rename the binary
    mv gitz-cli-ubuntu-latest gitz-cli

    # Move it to a common location for user binaries
    sudo mv gitz-cli /usr/local/bin/
    ```

    **For Windows:**
    1.  Rename the downloaded file from `gitz-cli-windows-latest.exe` to `gitz-cli.exe`.
    2.  Create a folder for your command-line tools if you don't have one (e.g., `C:\Users\YourUser\bin`).
    3.  Move `gitz-cli.exe` into that folder.
    4.  Add the folder to your system's PATH environment variable:
        *   Search for "Edit the system environment variables" in the Start Menu.
        *   Click the "Environment Variables..." button.
        *   In the "System variables" section, find and select the `Path` variable, then click "Edit...".
        *   Click "New" and add the path to the folder where you placed `gitz-cli.exe` (e.g., `C:\Users\YourUser\bin`).
        *   Click "OK" on all windows to save the changes.
    5.  You may need to restart your terminal for the changes to take effect.

## ⚙️ Configuration

`gitz-cli` requires a Google Gemini API key to function.

### Environment Variable

You need to set the `GEMINI_API_KEY` environment variable.

*   **For Linux/macOS:**
    ```bash
    # For a temporary session
    export GEMINI_API_KEY="YOUR_API_KEY"

    # For persistent use, add it to your shell's config file (e.g., ~/.bashrc, ~/.zshrc)
    echo 'export GEMINI_API_KEY="YOUR_API_KEY"' >> ~/.bashrc
    source ~/.bashrc
    ```

*   **For Windows:**
    You can set this in the same "Environment Variables" window where you edited the PATH.
    *   Under "User variables", click "New...".
    *   Variable name: `GEMINI_API_KEY`
    *   Variable value: `YOUR_API_KEY`
    *   Click "OK".

## 💻 Usage

Once installed and configured, you can use `gitz-cli` to generate commit messages.

### Commands

The primary command is `gitz-cli commit`.

*   `stage`: Generate a message for staged changes only.
*   `any`: Generate a message for all local changes (staged and unstaged).

### Examples

1.  **Generate a commit for staged changes:**
    ```bash
    # Stage your changes
    git add .

    # Run the command
    gitz-cli commit stage
    ```

2.  **Generate a commit for all changes:**
    ```bash
    # Make changes but don't stage them
    gitz-cli commit any
    ```
