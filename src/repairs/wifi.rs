//! WiFi Repair Module
//!
//! Handles WiFi connection issues including adapter detection,
//! network manager restart, DNS flushing, and IP renewal.

use crate::config::Config;
use crate::errors::{RepairError, RepairResult};
use crate::ui::{create_progress_bar, print_status, print_step};
use log::{error, info};
use std::process::{Command, Stdio};
use tokio::time::sleep;
use std::time::Duration;

/// Executes a system command asynchronously
///
/// # Arguments
///
/// * `program` - The command to run
/// * `args` - Arguments for the command
///
/// Returns the output if successful, or an error if the command fails.
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

pub async fn repair(config: &Config) -> RepairResult<()> {
    info!("Starting WiFi repair");

    print_step(1, "Scanning WiFi interfaces...");
    let pb = create_progress_bar("Detecting WiFi adapter...", 50);

    let wifi_found = detect_wifi_adapter().await?;
    pb.finish_with_message("Scan complete");

    if !wifi_found {
        return Err(RepairError::HardwareNotFound("WiFi adapter".to_string()));
    }
    print_status("success", "WiFi adapter detected");

    print_step(2, "Restarting network services...");
    restart_network_manager().await?;
    print_status("success", "Network services restarted");

    print_step(3, "Flushing DNS cache...");
    flush_dns().await?;
    print_status("success", "DNS cache cleared");

    print_step(4, "Renewing IP configuration...");
    renew_ip().await?;
    print_status("success", "IP configuration renewed");

    info!("WiFi repair completed");
    Ok(())
}

pub async fn repair_dns(config: &Config) -> RepairResult<()> {
    info!("Starting DNS repair");

    print_step(1, "Testing DNS resolution...");
    test_dns().await?;

    print_step(2, "Flushing DNS cache...");
    flush_dns().await?;
    print_status("success", "DNS cache flushed");

    print_step(3, "Updating DNS servers...");
    set_google_dns().await?;
    print_status("success", "DNS servers updated");

    info!("DNS repair completed");
    Ok(())
}

async fn detect_wifi_adapter() -> RepairResult<bool> {
    let output = if cfg!(target_os = "windows") {
        execute_command("netsh", &["wlan", "show", "interfaces"]).await?
    } else {
        execute_command("ip", &["link", "show"]).await?
    };

    let result = String::from_utf8_lossy(&output.stdout);

    if cfg!(target_os = "windows") {
        Ok(result.contains("SSID") || result.contains("State"))
    } else {
        Ok(result.contains("wlan") || result.contains("wlp") || result.contains("wifi"))
    }
}

async fn restart_network_manager() -> RepairResult<()> {
    if cfg!(target_os = "windows") {
        execute_command("netsh", &["wlan", "disconnect"]).await?;
        info!("Disconnected from WiFi. Windows will attempt to reconnect automatically.");
    } else {
        print_status("info", "Attempting to restart NetworkManager. This may require sudo privileges.");
        execute_command("systemctl", &["restart", "NetworkManager"]).await?;
    }
    Ok(())
}

async fn flush_dns() -> RepairResult<()> {
    if cfg!(target_os = "windows") {
        execute_command("ipconfig", &["/flushdns"]).await?;
    } else {
        execute_command("systemd-resolve", &["--flush-caches"]).await?;
        info!("Also attempting to restart nscd if present.");
        let _ = execute_command("systemctl", &["restart", "nscd"]).await;
    }
    Ok(())
}

async fn renew_ip() -> RepairResult<()> {
    if cfg!(target_os = "windows") {
        execute_command("ipconfig", &["/release"]).await?;
        sleep(Duration::from_millis(500)).await;
        execute_command("ipconfig", &["/renew"]).await?;
    } else {
        print_status("info", "Attempting to renew IP via dhclient. This may require sudo privileges.");
        execute_command("dhclient", &["-r"]).await?;
        sleep(Duration::from_millis(500)).await;
        execute_command("dhclient", &[]).await?;
    }
    Ok(())
}

async fn test_dns() -> RepairResult<()> {
    let output = execute_command("nslookup", &["google.com"]).await?;
    let result = String::from_utf8_lossy(&output.stdout);
    if result.contains("Address") || result.contains("answer") {
        print_status("success", "DNS resolution working");
    } else {
        print_status("warning", "DNS may have issues");
    }
    Ok(())
}

async fn set_google_dns() -> RepairResult<()> {
    if cfg!(target_os = "windows") {
        print_status("info", "Attempting to set DNS to Google's DNS for 'Wi-Fi' adapter.");
        execute_command(
            "netsh",
            &["interface", "ip", "set", "dns", "name=\"Wi-Fi\"", "static", "8.8.8.8"]
        ).await?;
        execute_command(
            "netsh",
            &["interface", "ip", "add", "dns", "name=\"Wi-Fi\"", "8.8.4.4", "index=2"]
        ).await?;
    } else {
        print_status("warning", "Automatic DNS server change on Linux is not yet implemented.");
    }
    Ok(())
}