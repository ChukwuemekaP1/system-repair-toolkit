
use crate::config::Config;
use crate::errors::{RepairError, RepairResult};
use crate::ui::{create_progress_bar, print_status, print_step};
use log::info;
use std::process::Command;
use tokio::time::sleep;
use std::time::Duration;

/// Initiates the audio repair process.
/// 
/// This function detects audio devices and restarts audio services if devices are found.
/// 
/// # Arguments
/// 
/// * `config` - A reference to the application configuration.
/// 
/// # Returns
/// 
/// * `RepairResult<()>` - Ok if repair succeeds, or an error if issues are encountered.
pub async fn repair(config: &Config) -> RepairResult<()> {
    info!("Starting audio repair");

    print_step(1, "Detecting audio devices...");
    let audio_detected = detect_audio_devices().await?;

    if !audio_detected {
        return Err(RepairError::HardwareNotFound("Audio devices".to_string()));
    }
    print_status("success", "Audio devices detected");

    print_step(2, "Restarting audio services...");
    let pb = create_progress_bar("Restarting audio...", 100);

    restart_audio_service().await?;

    for i in 0..=100 {
        pb.set_position(i);
        sleep(Duration::from_millis(15)).await;
    }
    pb.finish_with_message("Audio restarted");
    print_status("success", "Audio services restarted");

    info!("Audio repair completed");
    Ok(())
}

/// Detects available audio devices on the system.
/// 
/// Uses platform-specific commands to check for audio hardware.
/// 
/// # Returns
/// 
/// * `RepairResult<bool>` - True if audio devices are detected, false otherwise.
async fn detect_audio_devices() -> RepairResult<bool> {
    if cfg!(target_os = "windows") {
        let output = tokio::task::spawn_blocking(|| {
            Command::new("powershell")
                .args(&["-Command", "Get-CimInstance Win32_SoundDevice | Measure-Object | Select-Object -ExpandProperty Count"])
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
    } else {
        let output = tokio::task::spawn_blocking(|| Command::new("aplay").args(&["-l"]).output())
            .await
            .map_err(|e| RepairError::CommandFailed(e.to_string()))??;

        let result = String::from_utf8_lossy(&output.stdout);
        Ok(result.contains("card"))
    }
}

/// Restarts the audio service on the system.
/// 
/// Uses platform-specific commands to stop and start audio services.
/// 
/// # Returns
/// 
/// * `RepairResult<()>` - Ok if restart succeeds, or an error if commands fail.
async fn restart_audio_service() -> RepairResult<()> {
    if cfg!(target_os = "windows") {
        tokio::task::spawn_blocking(|| Command::new("net").args(&["stop", "audiosrv"]).output())
            .await
            .map_err(|e| RepairError::CommandFailed(e.to_string()))??;

        sleep(Duration::from_secs(1)).await;

        tokio::task::spawn_blocking(|| Command::new("net").args(&["start", "audiosrv"]).output())
            .await
            .map_err(|e| RepairError::CommandFailed(e.to_string()))??;
    } else {
        tokio::task::spawn_blocking(|| Command::new("pulseaudio").args(&["-k"]).output())
            .await
            .map_err(|e| RepairError::CommandFailed(e.to_string()))??;

        sleep(Duration::from_secs(1)).await;

        tokio::task::spawn_blocking(|| Command::new("pulseaudio").args(&["--start"]).output())
            .await
            .map_err(|e| RepairError::CommandFailed(e.to_string()))??;
    }
    Ok(())
}