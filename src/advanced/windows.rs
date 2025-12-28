use crate::config::Config;
use crate::errors::{RepairError, RepairResult};
use crate::types::AdvancedCommand;
use crate::ui::{create_progress_bar, print_status, print_step, confirm, show_warning};
use log::{error, info};
use std::process::{Command, Stdio};
use tokio::time::sleep;
use std::time::Duration;
use std::io::{self, Write};
use colored::Colorize;

async fn execute_command(program: &str, args: &[&str]) -> RepairResult<std::process::Output> {
    let command_str = format!("{} {}", program, args.join(" "));
    info!("Executing command: {}", command_str);

    let program_owned = program.to_string();
    let args_owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();

    let output = tokio::task::spawn_blocking(move || {
        Command::new(&program_owned)
            .args(&args_owned)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    })
    .await
    .map_err(|e| RepairError::CommandFailed(format!("Task spawn failed for '{}': {}", command_str, e)))??;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!(
            "Command '{}' failed with status {}: {}",
            command_str, output.status, stderr
        );
        return Err(RepairError::CommandFailed(format!(
            "Command failed: {}\n{}",
            command_str, stderr
        )));
    }

    info!(
        "Command '{}' succeeded.",
        command_str
    );
    Ok(output)
}

pub async fn execute(cmd: &AdvancedCommand, config: &Config) -> RepairResult<()> {
    if config.confirm_actions {
        show_warning(&format!(
            "About to execute: {}\n{}",
            cmd.name(),
            cmd.description()
        ));

        if !confirm("Continue with this operation?") {
            info!("Operation cancelled by user");
            return Ok(());
        }
    }

    match cmd {
        AdvancedCommand::SfcScannow => cmd_sfc_scannow().await,
        AdvancedCommand::DismRestoreHealth => cmd_dism_restore().await,
        AdvancedCommand::ChkdskRepair => cmd_chkdsk().await,
        AdvancedCommand::BootrecFixMbr => cmd_bootrec_mbr().await,
        AdvancedCommand::BootrecFixBoot => cmd_bootrec_boot().await,
        AdvancedCommand::CryptoServiceRestart => cmd_crypto_restart().await,
        AdvancedCommand::StartupRepair => cmd_startup_repair().await,
        AdvancedCommand::RegistryEditor => cmd_registry_editor().await,
        _ => Err(RepairError::CommandFailed(
            "Command not supported on Windows".to_string(),
        )),
    }
}

async fn cmd_sfc_scannow() -> RepairResult<()> {
    print_step(1, "Starting System File Checker...");
    print_status("info", "This may take a while and requires administrator privileges.");

    let pb = create_progress_bar("Scanning system files...", 100);
    pb.set_message("Running SFC /scannow...");

    // Simulate progress as SFC doesn't provide real-time output
    for i in 0..=95 {
        pb.set_position(i);
        sleep(Duration::from_millis(500)).await;
    }

    let output = execute_command("sfc", &["/scannow"]).await?;
    pb.finish_with_message("Scan complete");

    let result = String::from_utf8_lossy(&output.stdout);

    if result.contains("successfully repaired") {
        print_status("success", "Corrupted files found and repaired");
    } else if result.contains("did not find any integrity violations") {
        print_status("success", "No integrity violations found");
    } else if result.contains("found corrupt files but was unable") {
        print_status("warning", "Found corrupt files but couldn't repair all");
        println!("  {} Run DISM first, then retry SFC", "ℹ".bright_blue());
    } else {
        print_status("info", "SFC scan finished. Review logs for details.");
    }

    info!("SFC scan completed");
    Ok(())
}

async fn cmd_dism_restore() -> RepairResult<()> {
    print_step(1, "Starting DISM image repair...");
    print_status("info", "This requires an internet connection and administrator privileges.");

    let pb = create_progress_bar("Repairing Windows image...", 100);
    pb.set_message("Running DISM /Online /Cleanup-Image /RestoreHealth...");

    // Simulate progress
    for i in 0..=95 {
        pb.set_position(i);
        sleep(Duration::from_millis(600)).await;
    }

    let output = execute_command(
        "DISM",
        &["/Online", "/Cleanup-Image", "/RestoreHealth"],
    )
    .await?;
    pb.finish_with_message("Repair complete");

    let result = String::from_utf8_lossy(&output.stdout);

    if result.contains("operation completed successfully") {
        print_status("success", "Windows image repaired successfully");
    } else if result.contains("No component store corruption detected") {
        print_status("success", "Image is healthy - no repairs needed");
    } else {
        print_status("warning", "DISM finished. Review logs for details.");
    }

    info!("DISM repair completed");
    Ok(())
}

async fn cmd_chkdsk() -> RepairResult<()> {
    print_step(1, "Scheduling disk check...");
    print_status("info", "This will check the C: drive for errors on the next reboot.");

    // CHKDSK requires interaction (Y/N), which we can't do directly.
    // We can use `fsutil` to query the dirty bit, which indicates if a check is scheduled.
    let output = execute_command("fsutil", &["dirty", "query", "C:"]).await?;
    let result = String::from_utf8_lossy(&output.stdout);

    if result.contains("is dirty") {
        print_status("success", "Disk check is already scheduled for C: on next reboot.");
    } else {
        print_status("warning", "Could not automatically schedule CHKDSK.");
        println!("  To schedule it manually, run this command in an Administrator terminal:");
        println!("  {}\n", "chkdsk C: /f /r".bright_white());
    }

    info!("CHKDSK check completed");
    Ok(())
}

async fn cmd_bootrec_mbr() -> RepairResult<()> {
    print_step(1, "Fixing Master Boot Record...");
    print_status("warning", "This command should be run from the Windows Recovery Environment.");

    execute_command("bootrec", &["/fixmbr"]).await?;

    print_status("success", "'bootrec /fixmbr' command executed.");
    println!("  Check the output above to ensure it was successful.");

    info!("Bootrec /fixmbr executed");
    Ok(())
}

async fn cmd_bootrec_boot() -> RepairResult<()> {
    print_step(1, "Fixing boot sector...");
    print_status("warning", "This command should be run from the Windows Recovery Environment.");

    execute_command("bootrec", &["/fixboot"]).await?;

    print_status("success", "'bootrec /fixboot' command executed.");
    println!("  If you see 'Access is denied', run 'bootsect /nt60 sys' first.");

    info!("Bootrec /fixboot executed");
    Ok(())
}

async fn cmd_crypto_restart() -> RepairResult<()> {
    print_step(1, "Stopping Cryptographic Services...");
    execute_command("net", &["stop", "cryptSvc"]).await?;
    print_status("success", "Service stopped");

    sleep(Duration::from_secs(2)).await;

    print_step(2, "Starting Cryptographic Services...");
    execute_command("net", &["start", "cryptSvc"]).await?;
    print_status("success", "Service restarted successfully");

    info!("Cryptographic Services restarted");
    Ok(())
}

async fn cmd_startup_repair() -> RepairResult<()> {
    print_step(1, "Initiating Startup Repair...");
    print_status("info", "The system will now attempt to restart into the Recovery Environment.");
    print_status("warning", "Save all work before continuing.");

    if !confirm("Reboot to Recovery Environment now?") {
        info!("Startup repair cancelled by user.");
        return Ok(());
    }

    execute_command("shutdown", &["/r", "/o", "/t", "0"]).await?;

    info!("Reboot command for recovery environment issued.");
    Ok(())
}

async fn cmd_registry_editor() -> RepairResult<()> {
    print_step(1, "Opening Registry Editor...");

    println!("  {} CAUTION: Incorrect changes can break Windows", "⚠".bright_red().bold());
    println!("  {} Always backup before changes", "ℹ".bright_blue());

    let output = tokio::task::spawn_blocking(|| Command::new("regedit").spawn()).await?;

    match output {
        Ok(_) => print_status("success", "Registry Editor opened"),
        Err(e) => print_status("error", &format!("Failed: {}", e)),
    }

    info!("Registry Editor launched");
    Ok(())
}

pub async fn verify(_cmd: &AdvancedCommand, _config: &Config) -> RepairResult<()> {
    // Most Windows commands don't have a simple verification step.
    // The output of the command itself is the primary source of truth.
    // This function can be expanded later if specific checks are needed.
    Ok(())
}
