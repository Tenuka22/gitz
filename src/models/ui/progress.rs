use colored::*;

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
            "\r{} {} {}% {}/{}
            {}",
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
            "\r{} {} 100% {}/{}
            {}
            {}",
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
