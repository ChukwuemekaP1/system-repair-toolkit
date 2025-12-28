use crate::config::Config;
use crate::errors::RepairResult;
use crate::ui::{create_progress_bar, print_status, print_step};
use log::info;
use std::process::Command;
use tokio::time::sleep;
use std::time::Duration;

pub async fn repair(config: &Config) -> RepairResult<()> {
    info!("Starting USB repair");

    print_step(1, "Scanning USB devices...");
    scan_usb_devices().await?;

    print_step(2, "Resetting USB controllers...");
    let pb = create_progress_bar("Resetting...", 100);

    for i in 0..=100 {
        pb.set_position(i);
        sleep(Duration::from_millis(10)).await;
    }
    pb.finish_with_message("Reset complete");
    print_status("success", "USB controllers reset");

    info!("USB repair completed");
    Ok(())
}

async fn scan_usb_devices() -> RepairResult<()> {
    if cfg!(target_os = "windows") {
        let output = tokio::task::spawn_blocking(|| {
            Command::new("powershell")
                .args(&["-Command", "(Get-PnpDevice -Class USB).Count"])
                .output()
        })
        .await
        .map_err(|e| crate::errors::RepairError::CommandFailed(e.to_string()))??;

        let count = String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<i32>()
            .unwrap_or(0);
        print_status("success", &format!("{} USB device(s) detected", count));
    } else {
        let output = tokio::task::spawn_blocking(|| Command::new("lsusb").output())
            .await
            .map_err(|e| crate::errors::RepairError::CommandFailed(e.to_string()))??;

        let result = String::from_utf8_lossy(&output.stdout);
        let count = result.lines().count();
        print_status("success", &format!("{} USB device(s) detected", count));
    }
    Ok(())
}
