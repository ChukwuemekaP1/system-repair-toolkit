use crate::config::Config;
use crate::errors::RepairResult;
use crate::types::AdvancedCommand;

pub mod windows;
pub mod linux;

/// Executes an advanced command
pub async fn execute(cmd: &AdvancedCommand, config: &Config) -> RepairResult<()> {
    if cfg!(target_os = "windows") {
        windows::execute(cmd, config).await
    } else {
        linux::execute(cmd, config).await
    }
}

/// Verifies that the advanced command was executed successfully
pub async fn verify(cmd: &AdvancedCommand, config: &Config) -> RepairResult<()> {
    if cfg!(target_os = "windows") {
        windows::verify(cmd, config).await
    } else {
        linux::verify(cmd, config).await
    }
}
