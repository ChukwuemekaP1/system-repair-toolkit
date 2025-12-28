use anyhow::Result;
use colored::*;
use log::{error, info};
use std::io::{self, Write};
use tokio::time::sleep;
use std::time::Duration;

use system_repair_toolkit::{
    config::Config,
    types::{SystemIssue, AdvancedCommand},
    ui::*,
    repairs::RepairCoordinator,
    advanced,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = Config::load().unwrap_or_default();

    // Initialize logging
    init_logger(&config);

    info!("System Repair Toolkit v3.0 starting");
    info!(
        "Configuration: verbose={}, confirm_actions={}",
        config.verbose, config.confirm_actions
    );

    // Create repair coordinator
    let mut coordinator = RepairCoordinator::new(config.clone());

    // Main application loop
    match run_application(&mut coordinator, &config).await {
        Ok(_) => {
            info!("Application exited successfully");
            Ok(())
        }
        Err(e) => {
            error!("Application error: {}", e);
            show_error(&format!("Fatal error: {}", e));
            Err(e)
        }
    }
}

async fn run_application(
    coordinator: &mut RepairCoordinator,
    config: &Config,
) -> Result<()> {
    loop {
        show_banner();
        show_main_menu();

        match get_user_choice(13)? {
            Some(0) => {
                clear_screen();
                println!("{}", "╔══════════════════════════════════════════════╗".bright_cyan());
                println!("{}", "║  Thanks for using System Repair Toolkit!    ║".bright_cyan());
                println!("{}", "║  Your system is now optimized.              ║".bright_cyan());
                println!("{}", "╚══════════════════════════════════════════════╝".bright_cyan());
                info!("Application exit requested by user");
                break;
            }
            Some(idx) => {
                let issues = SystemIssue::all();
                let issue = &issues[idx];

                if issue == &SystemIssue::AdvancedCommands {
                    show_advanced_menu(config).await?;
                } else {
                    show_section_header(issue.name());

                    match coordinator.execute_repair(issue).await {
                        Ok(_) => {
                            show_completion(true, "Operation completed successfully!");
                        }
                        Err(e) => {
                            show_completion(false, &format!("Operation failed: {}", e));
                        }
                    }

                    wait_for_enter();
                }
            }
            None => continue,
        }
    }

    Ok(())
}

fn show_main_menu() {
    println!(
        "{}",
        "Available Issues to Diagnose & Repair:".bright_yellow().bold()
    );
    println!();

    let issues = SystemIssue::all();
    for (idx, issue) in issues.iter().enumerate() {
        println!(
            "  {} [{}] {} {}",
            "→".bright_green(),
            (idx + 1).to_string().bright_white().bold(),
            issue.icon(),
            issue.name().bright_white()
        );
    }

    println!();
    println!(
        "  {} [{}] {}",
        "→".bright_red(),
        "0".bright_white().bold(),
        "Exit".bright_red()
    );
    println!();
}

async fn show_advanced_menu(config: &Config) -> Result<()> {
    loop {
        show_section_header("Advanced System Commands");

        let commands = AdvancedCommand::for_current_platform();

        println!(
            "{}",
            format!("Platform: WINDOWS | {} Commands Available", commands.len())
                .bright_yellow()
        );
        println!();

        for (idx, cmd) in commands.iter().enumerate() {
            println!(
                "  {} [{}] {}",
                "→".bright_green(),
                (idx + 1).to_string().bright_white().bold(),
                cmd.name().bright_white()
            );
            println!("      {}", cmd.description().bright_black());
        }

        println!();
        println!(
            "  {} [{}] {}",
            "→".bright_red(),
            "0".bright_white().bold(),
            "Back to Main Menu".bright_red()
        );
        println!();

        match get_user_choice(commands.len())? {
            Some(0) => break,
            Some(idx) => {
                let cmd = &commands[idx];
                show_section_header(&format!("Executing: {}", cmd.name()));

                match advanced::execute(cmd, config).await {
                    Ok(_) => {
                        show_completion(true, "Command executed successfully!");
                    }
                    Err(e) => {
                        show_completion(false, &format!("Command failed: {}", e));
                    }
                }

                wait_for_enter();
            }
            None => continue,
        }
    }

    Ok(())
}

fn get_user_choice(max_option: usize) -> Result<Option<usize>> {
    print!("{}", "Select option: ".bright_cyan());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim().parse::<usize>() {
        Ok(0) => Ok(Some(0)),
        Ok(n) if n > 0 && n <= max_option => Ok(Some(n - 1)),
        _ => {
            println!("{}", "Invalid selection! Try again.".bright_red());
            std::thread::sleep(Duration::from_secs(1));
            Ok(None)
        }
    }
}

fn init_logger(config: &Config) {
    env_logger::Builder::from_default_env()
        .filter_level(config.get_log_filter())
        .format_timestamp_secs()
        .init();
}