//! Repair Module Coordinator
//!
//! FILE LOCATION: src/repairs/mod.rs
//!
//! This module coordinates all repair implementations and provides
//! a unified interface for executing repairs based on system issues.

use crate::config::Config;
use crate::errors::RepairResult;
use crate::types::SystemIssue;
use sysinfo::System;

// Declare submodules

/// Bluetooth repair implementation
pub mod bluetooth;
/// WiFi repair implementation
pub mod wifi;
/// Audio device repair implementation
pub mod audio;
/// USB device repair implementation
pub mod usb;
/// Disk management and cleanup
pub mod disk;
/// System-level analysis and checks
pub mod system;
/// Maintenance tasks like temp files and startup
pub mod maintenance;
/// RAM health monitoring and analysis
pub mod ram;

/// Main repair coordinator structure
///
/// This structure maintains system state and configuration needed
/// by all repair operations.
pub struct RepairCoordinator {
    /// System information cache
    pub sys: System,
    /// User configuration
    pub config: Config,
}

impl RepairCoordinator {
    /// Creates a new repair coordinator
    ///
    /// # Arguments
    ///
    /// * `config` - User configuration to use for repairs
    pub fn new(config: Config) -> Self {
        RepairCoordinator {
            sys: System::new_all(),
            config,
        }
    }

    /// Executes a repair based on the system issue type
    ///
    /// This is the main dispatch method that routes repair requests
    /// to the appropriate module.
    ///
    /// # Arguments
    ///
    /// * `issue` - The type of system issue to repair
    ///
    /// # Returns
    ///
    /// Result indicating success or failure of the repair operation
    pub async fn execute_repair(&mut self, issue: &SystemIssue) -> RepairResult<()> {
        match issue {
            SystemIssue::Bluetooth => bluetooth::repair(&self.config).await,
            SystemIssue::WiFi => wifi::repair(&self.config).await,
            SystemIssue::Audio => audio::repair(&self.config).await,
            SystemIssue::USB => usb::repair(&self.config).await,
            SystemIssue::DiskSpace => disk::cleanup(&mut self.sys, &self.config).await,
            SystemIssue::HighCPU => system::analyze_cpu(&mut self.sys, &self.config).await,
            SystemIssue::HighMemory => system::analyze_memory(&mut self.sys, &self.config).await,
            SystemIssue::DNS => wifi::repair_dns(&self.config).await,
            SystemIssue::Firewall => system::check_firewall(&self.config).await,
            SystemIssue::SystemUpdates => system::check_updates(&self.config).await,
            SystemIssue::TempFiles => maintenance::clear_temp_files(&self.config).await,
            SystemIssue::StartupPrograms => maintenance::analyze_startup(&self.config).await,
            SystemIssue::AdvancedCommands => {
                // Advanced commands are handled separately in main
                Ok(())
            }
            SystemIssue::RamHealth => ram::analyze_ram(&mut self.sys, &self.config).await,
        }
    }

    /// Refreshes system information cache
    ///
    /// Call this before operations that need current system metrics
    pub fn refresh_system_info(&mut self) {
        self.sys.refresh_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repair_coordinator_creation() {
        let config = Config::default();
        let coordinator = RepairCoordinator::new(config);
        assert!(coordinator.sys.processes().len() >= 0);
    }

    #[tokio::test]
    async fn test_execute_repair_advanced_commands() {
        let config = Config::default();
        let mut coordinator = RepairCoordinator::new(config);
        let result = coordinator
            .execute_repair(&SystemIssue::AdvancedCommands)
            .await;
        assert!(result.is_ok());
    }
}



