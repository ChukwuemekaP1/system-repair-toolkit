//! RAM Health Check Module
//!
//! This module performs RAM health monitoring and reports on memory usage and potential issues.

use crate::config::Config;
use crate::errors::{RepairError, RepairResult};
use crate::ui::{print_status, print_step};
use log::info;
use sysinfo::{RefreshKind, System};

/// Analyzes RAM health and usage
///
/// This function checks total memory, used memory, and provides health assessment.
///
/// # Arguments
///
/// * `sys` - Mutable reference to system information
/// * `config` - User configuration settings
///
/// # Returns
///
/// Result indicating success or failure with detailed error information
pub async fn analyze_ram(sys: &mut System, config: &Config) -> RepairResult<()> {
    info!("Starting RAM health check");

    // Refresh memory information
    sys.refresh_memory();

    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let available_memory = sys.available_memory();
    let usage_percent = (used_memory as f64 / total_memory as f64) * 100.0;

    print_step(1, "RAM Statistics:");
    println!("Total Memory: {} MB", total_memory / 1024);
    println!("Used Memory: {} MB", used_memory / 1024);
    println!("Available Memory: {} MB", available_memory / 1024);
    println!("Usage: {:.2}%", usage_percent);

    if usage_percent > 90.0 {
        print_status("warning", "High memory usage detected - consider closing applications or adding RAM");
    } else if usage_percent > 75.0 {
        print_status("info", "Moderate memory usage - monitor for performance issues");
    } else {
        print_status("success", "RAM usage is healthy");
    }

    info!("RAM health check completed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::System;

    #[tokio::test]
    async fn test_analyze_ram() {
        let mut sys = System::new_all();
        let config = Config::default();
        let result = analyze_ram(&mut sys, &config).await;
        assert!(result.is_ok());
    }
}

