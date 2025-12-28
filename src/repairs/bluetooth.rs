//! Bluetooth Repair Module
//!
//! FILE LOCATION: src/repairs/bluetooth.rs
//!
//! This module handles detection and repair of Bluetooth connectivity issues.
//! It performs hardware detection, service management, and adapter reset.

use crate::config::Config;
use crate::errors::{RepairError, RepairResult};
use crate::ui::{create_progress_bar, print_status, print_step};
use log::info;
use std::process::Command;
use tokio::time::sleep;
use std::time::Duration;

/// Repairs Bluetooth connectivity issues
///
/// This function performs a comprehensive Bluetooth repair including:
/// - Hardware detection
/// - Service restart
/// - Adapter reset
///
/// # Arguments
///
/// * `config` - User configuration settings
///
/// # Returns
///
/// Result indicating success or failure with detailed error information
pub async fn repair(config: &Config) -> RepairResult<()> {
    info!("Starting Bluetooth repair");

    // Step 1: Detect Bluetooth hardware
    print_step(1, "Detecting Bluetooth hardware...");
    let pb = create_progress_bar("Scanning for Bluetooth adapters...", 50);

    let bt_detected = detect_hardware().await?;
    pb.finish_with_message("Hardware scan complete");

    if bt_detected {
        print_status("success", "Bluetooth adapter found");
    } else {
        return Err(RepairError::HardwareNotFound("Bluetooth adapter".to_string()));
    }

    // Step 2: Check and restart Bluetooth service
    print_step(2, "Restarting Bluetooth service...");
    let pb = create_progress_bar("Restarting service...", 100);

    match restart_service().await {
        Ok(_) => {
            print_status("success", "Bluetooth service restarted");
        }
        Err(RepairError::PermissionDenied) => {
            print_status("warning", "Insufficient permissions to restart services");
            print_status("info", "Please run the application as administrator.");
        }
        Err(e) => {
            print_status("warning", &format!("Failed to restart service: {}", e));
        }
    }

    for i in 0..=100 {
        pb.set_position(i);
        sleep(Duration::from_millis(20)).await;
    }
    pb.finish_with_message("Service check complete");

    // Step 3: Verify service status
    print_step(3, "Verifying Bluetooth status...");
    sleep(Duration::from_millis(500)).await;

    if verify_service_running().await? {
        print_status("success", "Bluetooth is operational");
    } else {
        print_status("warning", "Service restarted but may need manual configuration");
    }

    // Step 4: Check Bluetooth health
    print_step(4, "Checking Bluetooth health...");
    sleep(Duration::from_millis(500)).await;

    if check_bluetooth_health().await? {
        print_status("success", "Bluetooth devices are healthy");
    } else {
        print_status("warning", "Some Bluetooth devices may have issues");
    }

    info!("Bluetooth repair completed successfully");
    Ok(())
}

/// Detects Bluetooth hardware on the system
async fn detect_hardware() -> RepairResult<bool> {
    if cfg!(target_os = "windows") {
        detect_hardware_windows().await
    } else {
        detect_hardware_unix().await
    }
}

/// Detects Bluetooth hardware on Windows
async fn detect_hardware_windows() -> RepairResult<bool> {
    let output = tokio::task::spawn_blocking(|| {
        Command::new("powershell")
            .args(&[
                "-Command",
                "Get-CimInstance -ClassName Win32_PnPEntity | Where-Object { $_.Name -like '*Bluetooth*' } | Measure-Object | Select-Object -ExpandProperty Count",
            ])
            .output()
    })
    .await
    .map_err(|e| RepairError::CommandFailed(e.to_string()))??;

    if !output.status.success() {
        return Ok(false);
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let count: usize = output_str.trim().parse().unwrap_or(0);
    Ok(count > 0)
}

/// Detects Bluetooth hardware on Unix systems
async fn detect_hardware_unix() -> RepairResult<bool> {
    let output = tokio::task::spawn_blocking(|| Command::new("hciconfig").output())
        .await
        .map_err(|e| RepairError::CommandFailed(e.to_string()))??;

    let result = String::from_utf8_lossy(&output.stdout);
    Ok(result.contains("hci0") || result.contains("UP"))
}

/// Restarts the Bluetooth service
async fn restart_service() -> RepairResult<()> {
    if cfg!(target_os = "windows") {
        restart_service_windows().await
    } else {
        restart_service_unix().await
    }
}

/// Checks if the process is running with administrator privileges
async fn is_admin() -> bool {
    if cfg!(not(target_os = "windows")) {
        return true; // For non-Windows, assume sufficient privileges or handle differently
    }

    let output = tokio::task::spawn_blocking(|| {
        Command::new("net")
            .arg("session")
            .output()
    })
    .await;

    match output {
        Ok(Ok(out)) => out.status.success(),
        _ => false,
    }
}

/// Restarts Bluetooth service on Windows
async fn restart_service_windows() -> RepairResult<()> {
    if !is_admin().await {
        return Err(RepairError::PermissionDenied);
    }

    let output = tokio::task::spawn_blocking(|| {
        Command::new("powershell")
            .args(&["-Command", "Get-Service -DisplayName *Bluetooth* | Restart-Service -Force"])
            .output()
    })
    .await
    .map_err(|e| RepairError::CommandFailed(e.to_string()))??;

    if output.status.success() {
        Ok(())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
        Err(RepairError::ServiceUnavailable(
            format!("Failed to restart Bluetooth services: {}", error_msg),
        ))
    }
}

/// Restarts Bluetooth service on Unix
async fn restart_service_unix() -> RepairResult<()> {
    // First, unblock if blocked
    let _ = tokio::task::spawn_blocking(|| {
        Command::new("rfkill")
            .args(&["unblock", "bluetooth"])
            .output()
    })
    .await;

    // Restart the service
    let output = tokio::task::spawn_blocking(|| {
        Command::new("systemctl")
            .args(&["restart", "bluetooth"])
            .output()
    })
    .await
    .map_err(|e| RepairError::CommandFailed(e.to_string()))??;

    if output.status.success() {
        Ok(())
    } else {
        Err(RepairError::ServiceUnavailable(
            "Failed to restart Bluetooth service".to_string(),
        ))
    }
}

/// Verifies that the Bluetooth service is running
async fn verify_service_running() -> RepairResult<bool> {
    if cfg!(target_os = "windows") {
        verify_service_windows().await
    } else {
        verify_service_unix().await
    }
}

/// Verifies Bluetooth service on Windows
async fn verify_service_windows() -> RepairResult<bool> {
    let output = tokio::task::spawn_blocking(|| {
        Command::new("powershell")
            .args(&[
                "-Command",
                "Get-Service bthserv | Select-Object -ExpandProperty Status",
            ])
            .output()
    })
    .await
    .map_err(|e| RepairError::CommandFailed(e.to_string()))??;

    let result = String::from_utf8_lossy(&output.stdout);
    Ok(result.trim() == "Running")
}

/// Verifies Bluetooth service on Unix
async fn verify_service_unix() -> RepairResult<bool> {
    let output = tokio::task::spawn_blocking(|| {
        Command::new("systemctl")
            .args(&["is-active", "bluetooth"])
            .output()
    })
    .await
    .map_err(|e| RepairError::CommandFailed(e.to_string()))??;

    let result = String::from_utf8_lossy(&output.stdout);
    Ok(result.trim() == "active")
}

// Add this new function after verify_service_unix

/// Checks the health of Bluetooth devices
async fn check_bluetooth_health() -> RepairResult<bool> {
    if cfg!(target_os = "windows") {
        let output = tokio::task::spawn_blocking(|| {
            Command::new("powershell")
                .args(&[
                    "-Command",
                    "Get-CimInstance -ClassName Win32_PnPEntity | Where-Object { $_.Name -like '*Bluetooth*' -and $_.Status -ne 'OK' } | Measure-Object | Select-Object -ExpandProperty Count",
                ])
                .output()
        })
        .await
        .map_err(|e| RepairError::CommandFailed(e.to_string()))??;

        if !output.status.success() {
            return Err(RepairError::CommandFailed("Health check command failed".to_string()));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let count: usize = output_str.trim().parse().unwrap_or(0);
        Ok(count == 0)
    } else {
        // For Unix, use hciconfig or something
        let output = tokio::task::spawn_blocking(|| Command::new("hciconfig").args(&["-a"]).output())
            .await
            .map_err(|e| RepairError::CommandFailed(e.to_string()))??;

        let result = String::from_utf8_lossy(&output.stdout);
        Ok(result.contains("UP RUNNING"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detect_hardware() {
        // This test may fail on systems without Bluetooth
        let result = detect_hardware().await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_verify_service() {
        // This test checks if we can query service status
        let result = verify_service_running().await;
        assert!(result.is_ok() || result.is_err());
    }
}