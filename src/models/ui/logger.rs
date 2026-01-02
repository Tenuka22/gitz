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
        print!("\r{} {}", "◐".cyan().bold(), msg.cyan());
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
