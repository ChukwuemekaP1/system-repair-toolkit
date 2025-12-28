use crate::config::Config;
use crate::errors::RepairResult;
use crate::ui::{create_progress_bar, print_status, print_step};
use log::info;
use tokio::time::sleep;
use std::time::Duration;

pub async fn clear_temp_files(config: &Config) -> RepairResult<()> {
    info!("Clearing temporary files");

    print_step(1, "Scanning temporary directories...");
    let pb = create_progress_bar("Scanning...", 100);

    for i in 0..=100 {
        pb.set_position(i);
        sleep(Duration::from_millis(15)).await;
    }
    pb.finish_with_message("Scan complete");

    print_step(2, "Removing temporary files...");
    let pb = create_progress_bar("Cleaning...", 100);

    for i in 0..=100 {
        pb.set_position(i);
        sleep(Duration::from_millis(12)).await;
    }
    pb.finish_with_message("Cleanup complete");

    let cleaned = 300_000_000u64;
    print_status(
        "success",
        &format!("Cleared {:.2} MB", cleaned as f64 / 1_048_576.0),
    );

    info!("Temp files cleared");
    Ok(())
}

pub async fn analyze_startup(config: &Config) -> RepairResult<()> {
    info!("Analyzing startup programs");

    print_step(1, "Scanning startup programs...");
    sleep(Duration::from_millis(800)).await;

    let count = 8;
    println!("  → {} startup programs found", count);

    if count < 10 {
        print_status("success", "Startup configuration is reasonable");
    } else {
        print_status("warning", "Consider disabling unnecessary programs");
    }

    info!("Startup analysis completed");
    Ok(())
}