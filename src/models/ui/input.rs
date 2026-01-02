use colored::*;
use std::io::{self, Write};

use super::logger::Logger;

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
            print!(
                "{} {}{}{}: ",
                "→".cyan().bold(),
                "Select (1-".bright_white(),
                options.len().to_string().bright_white(),
                ")".bright_white()
            );
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
            print!(
                "{} {}{}{}: ",
                "→".cyan().bold(),
                "Select (1-".bright_white(),
                options.len().to_string().bright_white(),
                ")".bright_white()
            );
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
