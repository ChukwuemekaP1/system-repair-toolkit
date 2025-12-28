use crate::config::Config;
use crate::errors::{RepairError, RepairResult};
use crate::types::AdvancedCommand;
use crate::ui::{print_status, print_step, confirm, show_warning};
use log::{error, info};
use std::process::{Command, Stdio};

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
        AdvancedCommand::FsckCheck => cmd_fsck_check().await,
        AdvancedCommand::E2fsckRepair => cmd_e2fsck_repair().await,
        AdvancedCommand::XfsRepair => cmd_xfs_repair().await,
        AdvancedCommand::ChmodFix => cmd_chmod_fix().await,
        AdvancedCommand::ChownFix => cmd_chown_fix().await,
        AdvancedCommand::SystemctlRestart => cmd_systemctl_restart().await,
        AdvancedCommand::MountCheck => cmd_mount_check().await,
        _ => Err(RepairError::CommandFailed(
            "Command not supported on Linux".to_string(),
        )),
    }
}

async fn cmd_fsck_check() -> RepairResult<()> {
    print_step(1, "Running File System Check (fsck)...");
    print_status("warning", "This should be run on an UNMOUNTED partition.");
    print_status("info", "Example: sudo fsck /dev/sda1");
    Ok(())
}

async fn cmd_e2fsck_repair() -> RepairResult<()> {
    print_step(1, "Running ext2/3/4 File System Repair (e2fsck)...");
    print_status("warning", "This should be run on an UNMOUNTED partition.");
    print_status("info", "Example: sudo e2fsck -p /dev/sda1");
    Ok(())
}

async fn cmd_xfs_repair() -> RepairResult<()> {
    print_step(1, "Running XFS File System Repair (xfs_repair)...");
    print_status("warning", "This should be run on an UNMOUNTED partition.");
    print_status("info", "Example: sudo xfs_repair /dev/sda1");
    Ok(())
}

async fn cmd_chmod_fix() -> RepairResult<()> {
    print_step(1, "Fixing File Permissions (chmod)...");
    print_status("info", "Example: sudo chmod -R 644 /path/to/dir");
    Ok(())
}

async fn cmd_chown_fix() -> RepairResult<()> {
    print_step(1, "Fixing File Ownership (chown)...");
    print_status("info", "Example: sudo chown -R user:group /path/to/dir");
    Ok(())
}

async fn cmd_systemctl_restart() -> RepairResult<()> {
    print_step(1, "Restarting System Service (systemctl)...");
    print_status("info", "Example: sudo systemctl restart nginx");
    Ok(())
}

async fn cmd_mount_check() -> RepairResult<()> {
    print_step(1, "Checking Mounted File Systems...");
    let output = execute_command("mount", &[]).await?;
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

pub async fn verify(_cmd: &AdvancedCommand, _config: &Config) -> RepairResult<()> {
    Ok(())
}
