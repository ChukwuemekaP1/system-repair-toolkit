//! User Interface Utilities
//!
//! FILE LOCATION: src/ui.rs
//!
//! This module provides reusable UI components for the command-line interface
//! including progress bars, status messages, menus, and formatted output.

use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use log::debug;
use std::io::{self, Write};
use std::process::Command;
use std::time::Duration;

/// Creates a styled progress bar for visual feedback
///
/// # Arguments
///
/// * `message` - The message to display alongside the progress bar
/// * `length` - The total number of steps in the operation
///
/// # Returns
///
/// A configured ProgressBar ready to use
pub fn create_progress_bar(message: &str, length: u64) -> ProgressBar {
    let pb = ProgressBar::new(length);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("=>-"),
    );
    pb.set_message(message.to_string());
    pb
}

/// Prints a numbered step indicator with formatting
///
/// # Arguments
///
/// * `step` - The step number to display
/// * `message` - The description of this step
pub fn print_step(step: u8, message: &str) {
    println!(
        "{} Step {}: {}",
        "►".bright_blue(),
        step.to_string().bright_yellow(),
        message.bright_white()
    );
    debug!("Step {}: {}", step, message);
}

/// Prints a status message with appropriate icon and color
///
/// # Arguments
///
/// * `status_type` - The type of status: "success", "error", "warning", or "info"
/// * `message` - The status message to display
pub fn print_status(status_type: &str, message: &str) {
    let (icon, color) = match status_type {
        "success" => ("✓", "green"),
        "error" => ("✗", "red"),
        "warning" => ("⚠", "yellow"),
        _ => ("ℹ", "blue"),
    };

    let colored_msg = match color {
        "green" => message.bright_green(),
        "red" => message.bright_red(),
        "yellow" => message.bright_yellow(),
        _ => message.bright_blue(),
    };

    println!("  {} {}", icon, colored_msg);
}

/// Displays the application banner
pub fn show_banner() {
    clear_screen();
    println!(
        "{}",
        "╔══════════════════════════════════════════════╗".bright_cyan()
    );
    println!(
        "{}",
        "║     SYSTEM REPAIR TOOLKIT v1.0               ║".bright_cyan()
    );
    println!(
        "{}",
        "║     Professional Edition for Windows         ║".bright_cyan()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════╝".bright_cyan()
    );
    println!();
}

/// Displays a section header
///
/// # Arguments
///
/// * `title` - The title text for the section
pub fn show_section_header(title: &str) {
    clear_screen();
    println!(
        "{}",
        "╔══════════════════════════════════════════════╗".bright_cyan()
    );
    println!("║ {} {}                    ", "🔧".bright_yellow(), title);
    println!(
        "{}",
        "╚══════════════════════════════════════════════╝".bright_cyan()
    );
    println!();
}

/// Clears the terminal screen using platform-specific commands
pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(&["/C", "cls"]).status();
    } else {
        let _ = Command::new("clear").status();
    }
}

/// Prompts user for yes/no confirmation
///
/// # Arguments
///
/// * `prompt` - The question to ask the user
///
/// # Returns
///
/// true if user confirms (y/Y), false otherwise
pub fn confirm(prompt: &str) -> bool {
    print!("{} (y/N): ", prompt.bright_cyan());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_lowercase() == "y"
}

/// Waits for user to press Enter
pub fn wait_for_enter() {
    println!();
    println!("{}", "Press Enter to continue...".bright_black());
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}

/// Displays a completion message with visual styling
///
/// # Arguments
///
/// * `success` - Whether the operation succeeded
/// * `message` - The message to display
pub fn show_completion(success: bool, message: &str) {
    println!();
    if success {
        println!("{}", format!("✓ {}", message).bright_green().bold());
    } else {
        println!("{}", format!("✗ {}", message).bright_red().bold());
    }
}

/// Displays an error message with formatting
///
/// # Arguments
///
/// * `error` - The error message to display
pub fn show_error(error: &str) {
    println!();
    println!("{}", format!("Error: {}", error).bright_red().bold());
}

/// Displays a warning message with formatting
///
/// # Arguments
///
/// * `warning` - The warning message to display
pub fn show_warning(warning: &str) {
    println!();
    println!("{}", format!("⚠ Warning: {}", warning).bright_yellow().bold());
}

/// Displays an info box with a message
///
/// # Arguments
///
/// * `title` - The title of the info box
/// * `lines` - The lines of text to display
pub fn show_info_box(title: &str, lines: &[&str]) {
    println!();
    println!("{}", format!("╔═ {} ═╗", title).bright_blue());
    for line in lines {
        println!("{}", format!("║ {} ", line).bright_blue());
    }
    println!("{}", "╚═══════╝".bright_blue());
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar_creation() {
        let pb = create_progress_bar("Testing", 100);
        assert_eq!(pb.length().unwrap(), 100);
    }

    #[test]
    fn test_print_step() {
        // This would normally output to stdout
        print_step(1, "Test step");
    }

    #[test]
    fn test_print_status() {
        print_status("success", "Test success");
        print_status("error", "Test error");
        print_status("warning", "Test warning");
        print_status("info", "Test info");
    }

    #[test]
    fn test_clear_screen() {
        clear_screen(); // Should not panic
    }
}