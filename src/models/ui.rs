use colored::*;

pub struct Logger;

impl Logger {
    // Clear the entire screen
    pub fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
    }

    // Clear current line
    pub fn clear_line() {
        print!("\r\x1B[K");
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
    }

    // Clear last N lines
    pub fn clear_last_lines(n: usize) {
        for _ in 0..n {
            print!("\x1B[1A"); // Move up one line
            print!("\x1B[2K"); // Clear line
        }
        print!("\r");
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
    }

    // Move cursor up N lines
    pub fn move_up(n: usize) {
        print!("\x1B[{}A", n);
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
    }

    // Success messages - green with checkmark
    pub fn success(msg: &str) {
        println!("{} {}", "✓".green().bold(), msg.bright_green());
    }

    // Error messages - red with cross
    pub fn error(msg: &str) {
        eprintln!("{} {}", "✗".red().bold(), msg.bright_red());
    }

    // Fatal error with panic details
    pub fn fatal(msg: &str, location: Option<&str>) {
        eprintln!("\n{} {}", "💥".bold(), "FATAL ERROR".bright_red().bold());
        eprintln!("{} {}", "✗".red().bold(), msg.bright_red());
        eprintln!("{} Contact the admins ASAP >", "⚠".red().bold(),);
        if let Some(loc) = location {
            eprintln!("  {} {}", "at".dimmed(), loc.dimmed());
        }
        eprintln!();
    }

    // Warning messages - yellow with warning symbol
    pub fn warning(msg: &str) {
        println!("{} {}", "⚠".yellow().bold(), msg.bright_yellow());
    }

    // Info messages - blue with info symbol
    pub fn info(msg: &str) {
        println!("{} {}", "ℹ".blue().bold(), msg.bright_blue());
    }

    // Step/progress messages - cyan with arrow
    pub fn step(msg: &str) {
        println!("{} {}", "→".cyan().bold(), msg.cyan());
    }

    // Highlight/emphasis - magenta
    pub fn highlight(msg: &str) {
        println!("{} {}", "★".magenta().bold(), msg.bright_magenta());
    }

    // Subtle/dimmed messages
    pub fn dim(msg: &str) {
        println!("  {}", msg.dimmed());
    }

    // Header/section separator
    pub fn header(msg: &str) {
        println!("\n{}", msg.bright_white().bold().underline());
    }

    // Loading/processing animation frame
    pub fn processing(msg: &str) {
        print!("\r{} {}...", "◐".cyan().bold(), msg.cyan());
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
    }

    // Command/code display
    pub fn command(cmd: &str) {
        println!("  {} {}", "$".bright_black(), cmd.white().on_black());
    }

    // Key-value pair display
    pub fn kv(key: &str, value: &str) {
        println!(
            "  {} {}",
            format!("{}:", key).bright_white().bold(),
            value.bright_cyan()
        );
    }

    // List item
    pub fn item(msg: &str) {
        println!("  {} {}", "•".bright_white(), msg.white());
    }

    // Completion message with emoji
    pub fn done(msg: &str) {
        println!("\n{} {}\n", "🎉".bold(), msg.bright_green().bold());
    }

    // Custom colored message
    pub fn custom(symbol: &str, msg: &str, color: Color) {
        println!("{} {}", symbol.color(color).bold(), msg.color(color));
    }
}

// Progress bar helper
pub struct ProgressBar {
    total: usize,
    current: usize,
    label: String,
}

impl ProgressBar {
    pub fn new(total: usize, label: &str) -> Self {
        Self {
            total,
            current: 0,
            label: label.to_string(),
        }
    }

    pub fn update(&mut self, current: usize) {
        self.current = current;
        let percentage = (current as f32 / self.total as f32 * 100.0) as usize;
        let filled = percentage / 5;
        let empty = 20 - filled;

        let bar = format!(
            "{}{}",
            "█".repeat(filled).bright_green(),
            "░".repeat(empty).dimmed()
        );

        print!(
            "\r{} {} {}% {}/{} {}",
            "⟳".cyan().bold(),
            bar,
            percentage.to_string().bright_white().bold(),
            current.to_string().bright_cyan(),
            self.total.to_string().cyan(),
            self.label.white()
        );

        use std::io::{self, Write};
        io::stdout().flush().unwrap();
    }

    pub fn finish(&self) {
        println!(
            "\r{} {} 100% {}/{} {} {}",
            "✓".green().bold(),
            "█".repeat(20).bright_green(),
            self.total.to_string().bright_cyan(),
            self.total.to_string().cyan(),
            self.label.white(),
            "Done!".bright_green().bold()
        );
    }
}

pub struct InfiniteLoader {
    frames: Vec<&'static str>,
    current_frame: usize,
    message: String,
    progress: Option<f32>,
}

impl InfiniteLoader {
    pub fn new(message: &str) -> Self {
        Self {
            frames: vec!["◐", "◓", "◑", "◒"],
            current_frame: 0,
            message: message.to_string(),
            progress: None,
        }
    }

    // Update without progress (infinite spin)
    pub fn tick(&mut self) {
        if let Some(progress) = self.progress {
            // If progress is set, show percentage
            if progress >= 100.0 {
                print!(
                    "\r{} {} {}",
                    "✓".green().bold(),
                    self.message.bright_green(),
                    "100%".bright_green().bold()
                );
            } else {
                print!(
                    "\r{} {} {}",
                    self.frames[self.current_frame].cyan().bold(),
                    self.message.cyan(),
                    format!("{}%", progress as u32).bright_white()
                );
            }
        } else {
            // No progress, just show spinner
            print!(
                "\r{} {}",
                self.frames[self.current_frame].cyan().bold(),
                self.message.cyan()
            );
        }

        use std::io::{self, Write};
        io::stdout().flush().unwrap();
        self.current_frame = (self.current_frame + 1) % self.frames.len();
    }

    // Set progress percentage (0-100)
    pub fn set_progress(&mut self, percent: f32) {
        self.progress = Some(percent.min(100.0).max(0.0));
    }

    // Clear progress back to infinite
    pub fn clear_progress(&mut self) {
        self.progress = None;
    }

    pub fn finish(&self, msg: &str) {
        println!("\r{} {}", "✓".green().bold(), msg.bright_green());
    }

    pub fn update_message(&mut self, msg: &str) {
        self.message = msg.to_string();
    }
}
use colored::*;
use std::io::{self, Write};

pub struct Input;

impl Input {
    // Simple text input with prompt
    pub fn text(prompt: &str) -> String {
        print!("{} {} ", "?".cyan().bold(), prompt.bright_white());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    // Text input with default value shown
    pub fn text_with_default(prompt: &str, default: &str) -> String {
        print!(
            "{} {} {}: ",
            "?".cyan().bold(),
            prompt.bright_white(),
            format!("(default: {})", default).dimmed()
        );
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            default.to_string()
        } else {
            input.to_string()
        }
    }

    // Password/secret input (hidden)
    pub fn password(prompt: &str) -> String {
        print!("{} {} ", "🔒".yellow().bold(), prompt.bright_white());
        io::stdout().flush().unwrap();

        // Use rpassword crate for secure password input
        // For basic version without external crate:
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    // Yes/No confirmation
    pub fn confirm(prompt: &str) -> bool {
        print!(
            "{} {} {}: ",
            "?".cyan().bold(),
            prompt.bright_white(),
            "[y/N]".dimmed()
        );
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
    }

    // Yes/No with default true
    pub fn confirm_default_yes(prompt: &str) -> bool {
        print!(
            "{} {} {}: ",
            "?".cyan().bold(),
            prompt.bright_white(),
            "[Y/n]".dimmed()
        );
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim().to_lowercase();
        !matches!(trimmed.as_str(), "n" | "no")
    }

    // Multiple choice selection
    pub fn select(prompt: &str, options: &[&str]) -> usize {
        println!("{} {}", "?".cyan().bold(), prompt.bright_white());

        for (i, option) in options.iter().enumerate() {
            println!(
                "  {} {}",
                format!("{})", i + 1).bright_cyan(),
                option.white()
            );
        }

        loop {
            print!("{} {} ", "→".cyan().bold(), "Select (1-{}):".bright_white());
            print!("{}", options.len().to_string().bright_cyan());
            print!(" ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if let Ok(choice) = input.trim().parse::<usize>() {
                if choice > 0 && choice <= options.len() {
                    return choice - 1;
                }
            }

            Logger::error("Invalid selection. Please try again.");
        }
    }

    // Multiple choice with descriptions
    pub fn select_with_desc(prompt: &str, options: &[(&str, &str)]) -> usize {
        println!("{} {}", "?".cyan().bold(), prompt.bright_white());

        for (i, (option, desc)) in options.iter().enumerate() {
            println!(
                "  {} {}",
                format!("{})", i + 1).bright_cyan(),
                option.white().bold()
            );
            println!("     {}", desc.dimmed());
        }

        loop {
            print!("{} {} ", "→".cyan().bold(), "Select (1-{}):".bright_white());
            print!("{}", options.len().to_string().bright_cyan());
            print!(" ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if let Ok(choice) = input.trim().parse::<usize>() {
                if choice > 0 && choice <= options.len() {
                    return choice - 1;
                }
            }

            Logger::error("Invalid selection. Please try again.");
        }
    }

    // Number input with validation
    pub fn number<T: std::str::FromStr>(prompt: &str) -> T
    where
        T::Err: std::fmt::Display,
    {
        loop {
            print!("{} {} ", "?".cyan().bold(), prompt.bright_white());
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<T>() {
                Ok(num) => return num,
                Err(e) => Logger::error(&format!("Invalid number: {}", e)),
            }
        }
    }

    // Number input with range validation
    pub fn number_in_range<T>(prompt: &str, min: T, max: T) -> T
    where
        T: std::str::FromStr + PartialOrd + std::fmt::Display + Copy,
        T::Err: std::fmt::Display,
    {
        loop {
            print!(
                "{} {} {}: ",
                "?".cyan().bold(),
                prompt.bright_white(),
                format!("[{}-{}]", min, max).dimmed()
            );
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<T>() {
                Ok(num) if num >= min && num <= max => return num,
                Ok(_) => Logger::error(&format!("Number must be between {} and {}", min, max)),
                Err(e) => Logger::error(&format!("Invalid number: {}", e)),
            }
        }
    }

    // Multi-line text input (ends with empty line)
    pub fn multiline(prompt: &str) -> String {
        println!("{} {}", "?".cyan().bold(), prompt.bright_white());
        println!(
            "  {} {}",
            "ℹ".blue(),
            "Press Enter twice to finish".dimmed()
        );

        let mut lines = Vec::new();
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim().is_empty() {
                break;
            }

            lines.push(input.trim_end().to_string());
        }

        lines.join("\n")
    }

    // List input (comma-separated)
    pub fn list(prompt: &str) -> Vec<String> {
        print!(
            "{} {} {}: ",
            "?".cyan().bold(),
            prompt.bright_white(),
            "(comma-separated)".dimmed()
        );
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    // Custom validation
    pub fn validated<F>(prompt: &str, validator: F) -> String
    where
        F: Fn(&str) -> Result<(), String>,
    {
        loop {
            print!("{} {} ", "?".cyan().bold(), prompt.bright_white());
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            match validator(input) {
                Ok(()) => return input.to_string(),
                Err(err) => Logger::error(&err),
            }
        }
    }
}
