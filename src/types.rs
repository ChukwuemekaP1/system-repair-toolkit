//! Shared Type Definitions
//!
//! FILE LOCATION: src/types.rs
//!
//! This module contains type definitions, enums, and structures that are
//! used across multiple modules in the application. Centralizing these
//! types prevents duplication and ensures consistency.

/// System issues that can be diagnosed and repaired
///
/// Each variant represents a category of system problems that the toolkit
/// can address. The AdvancedCommands variant leads to a submenu of
/// platform-specific repair utilities.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SystemIssue {
    /// Bluetooth connectivity problems
    Bluetooth,
    /// WiFi connection issues
    WiFi,
    /// Audio output/input failures
    Audio,
    /// USB device recognition problems
    USB,
    /// Low disk space conditions
    DiskSpace,
    /// High CPU utilization
    HighCPU,
    /// High memory usage
    HighMemory,
    /// DNS resolution failures
    DNS,
    /// Firewall configuration issues
    Firewall,
    /// Pending system updates
    SystemUpdates,
    /// Accumulated temporary files
    TempFiles,
    /// Excessive startup programs
    StartupPrograms,
    /// Access to advanced system commands
    AdvancedCommands,
    RamHealth,
}

impl SystemIssue {
    /// Returns the user-facing name for this issue type
    ///
    /// This name appears in the main menu and progress messages.
    pub fn name(&self) -> &str {
        match self {
            &Self::Bluetooth => "Bluetooth Repair",
            &Self::WiFi => "WiFi Repair",
            &Self::Audio => "Audio Repair",
            &Self::USB => "USB Repair",
            &Self::DiskSpace => "Disk Cleanup",
            &Self::HighCPU => "High CPU Analysis",
            &Self::HighMemory => "High Memory Analysis",
            &Self::DNS => "DNS Repair",
            &Self::Firewall => "Firewall Check",
            &Self::SystemUpdates => "System Updates",
            &Self::TempFiles => "Clear Temporary Files",
            &Self::StartupPrograms => "Analyze Startup Programs",
            &Self::AdvancedCommands => "Advanced Commands",
            &Self::RamHealth => "RAM Health Check",
        }
    }

    /// Gets the icon for the system issue
    pub fn icon(&self) -> &str {
        match self {
            &Self::Bluetooth => "📡",
            &Self::WiFi => "🌐",
            &Self::Audio => "🔊",
            &Self::USB => "🔌",
            &Self::DiskSpace => "💽",
            &Self::HighCPU => "⚡",
            &Self::HighMemory => "🧠",
            &Self::DNS => "🔗",
            &Self::Firewall => "🛡️",
            &Self::SystemUpdates => "📦",
            &Self::TempFiles => "🗑️",
            &Self::StartupPrograms => "🚀",
            &Self::AdvancedCommands => "💻",
            &Self::RamHealth => "🧠",
        }
    }

    /// Gets the description for the system issue
    pub fn description(&self) -> &str {
        match self {
            &Self::Bluetooth => "Repair Bluetooth connectivity issues",
            &Self::WiFi => "Repair WiFi connectivity issues",
            &Self::Audio => "Repair audio device issues",
            &Self::USB => "Repair USB device issues",
            &Self::DiskSpace => "Clean up disk space",
            &Self::HighCPU => "Analyze high CPU usage",
            &Self::HighMemory => "Analyze high memory usage",
            &Self::DNS => "Repair DNS configuration",
            &Self::Firewall => "Check firewall settings",
            &Self::SystemUpdates => "Check for system updates",
            &Self::TempFiles => "Clear temporary files",
            &Self::StartupPrograms => "Analyze and optimize startup programs",
            &Self::AdvancedCommands => "Run advanced diagnostic commands",
            &Self::RamHealth => "Monitor and analyze system RAM quality and health",
        }
    }



    /// Returns all available system issues in menu order
    pub fn all() -> Vec<SystemIssue> {
        vec![
            SystemIssue::Bluetooth,
            SystemIssue::WiFi,
            SystemIssue::Audio,
            SystemIssue::USB,
            SystemIssue::DiskSpace,
            SystemIssue::HighCPU,
            SystemIssue::HighMemory,
            SystemIssue::DNS,
            SystemIssue::Firewall,
            SystemIssue::SystemUpdates,
            SystemIssue::TempFiles,
            SystemIssue::StartupPrograms,
            SystemIssue::AdvancedCommands,
            SystemIssue::RamHealth,
        ]
    }
}

/// Advanced system repair commands
///
/// These commands provide direct access to OS-level repair utilities.
/// Each command is platform-specific and may require elevated privileges.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdvancedCommand {
    // Windows-specific commands
    /// System File Checker - repairs corrupted system files
    SfcScannow,
    /// DISM image repair - fixes Windows component store
    DismRestoreHealth,
    /// Disk check utility - scans for errors and bad sectors
    ChkdskRepair,
    /// Master Boot Record repair
    BootrecFixMbr,
    /// Boot sector repair
    BootrecFixBoot,
    /// Restart cryptographic services
    CryptoServiceRestart,
    /// Automatic startup repair
    StartupRepair,
    /// Registry editor access
    RegistryEditor,

    // Linux/Unix-specific commands
    /// File system check utility
    FsckCheck,
    /// Ext2/3/4 file system repair
    E2fsckRepair,
    /// XFS file system repair
    XfsRepair,
    /// File permission correction
    ChmodFix,
    /// File ownership correction
    ChownFix,
    /// Service restart utility
    SystemctlRestart,
    /// File system mount utility
    MountCheck,
}

impl AdvancedCommand {
    /// Returns the user-facing name for this command
    pub fn name(&self) -> &str {
        match self {
            AdvancedCommand::SfcScannow => "SFC Scan (System File Checker)",
            AdvancedCommand::DismRestoreHealth => "DISM Repair Windows Image",
            AdvancedCommand::ChkdskRepair => "Check Disk for Errors",
            AdvancedCommand::BootrecFixMbr => "Fix Master Boot Record",
            AdvancedCommand::BootrecFixBoot => "Fix Boot Sector",
            AdvancedCommand::CryptoServiceRestart => "Restart Cryptographic Services",
            AdvancedCommand::StartupRepair => "Startup Repair",
            AdvancedCommand::RegistryEditor => "Open Registry Editor",
            AdvancedCommand::FsckCheck => "File System Check (fsck)",
            AdvancedCommand::E2fsckRepair => "Ext2/3/4 File System Repair",
            AdvancedCommand::XfsRepair => "XFS File System Repair",
            AdvancedCommand::ChmodFix => "Fix File Permissions",
            AdvancedCommand::ChownFix => "Fix File Ownership",
            AdvancedCommand::SystemctlRestart => "Restart System Service",
            AdvancedCommand::MountCheck => "Mount/Unmount File System",
        }
    }

    /// Returns a description of what this command does
    pub fn description(&self) -> &str {
        match self {
            AdvancedCommand::SfcScannow => {
                "Scans and replaces corrupted system files using Windows component store"
            }
            AdvancedCommand::DismRestoreHealth => {
                "Repairs Windows image by downloading healthy files from Microsoft"
            }
            AdvancedCommand::ChkdskRepair => {
                "Fixes disk errors, bad sectors, and file system inconsistencies"
            }
            AdvancedCommand::BootrecFixMbr => "Rewrites Master Boot Record to fix boot issues",
            AdvancedCommand::BootrecFixBoot => "Repairs boot sector on system partition",
            AdvancedCommand::CryptoServiceRestart => {
                "Fixes update and installation hangs by restarting services"
            }
            AdvancedCommand::StartupRepair => "Automatic boot repair for startup problems",
            AdvancedCommand::RegistryEditor => {
                "Manual registry fixes (advanced users only - use with caution)"
            }
            AdvancedCommand::FsckCheck => "Checks and repairs file system on unmounted partitions",
            AdvancedCommand::E2fsckRepair => "Repairs ext2/ext3/ext4 file systems after journal replay",
            AdvancedCommand::XfsRepair => "Repairs XFS file system metadata and inconsistencies",
            AdvancedCommand::ChmodFix => "Changes file permissions to resolve access errors",
            AdvancedCommand::ChownFix => "Changes file ownership to fix permission issues",
            AdvancedCommand::SystemctlRestart => "Restarts failed or hung system services",
            AdvancedCommand::MountCheck => "Mounts file system to check accessibility",
        }
    }

    /// Returns the operating systems that support this command
    pub fn supported_os(&self) -> Vec<&'static str> {
        match self {
            AdvancedCommand::SfcScannow
            | AdvancedCommand::DismRestoreHealth
            | AdvancedCommand::ChkdskRepair
            | AdvancedCommand::BootrecFixMbr
            | AdvancedCommand::BootrecFixBoot
            | AdvancedCommand::CryptoServiceRestart
            | AdvancedCommand::StartupRepair
            | AdvancedCommand::RegistryEditor => vec!["windows"],

            AdvancedCommand::FsckCheck
            | AdvancedCommand::E2fsckRepair
            | AdvancedCommand::XfsRepair
            | AdvancedCommand::ChmodFix
            | AdvancedCommand::ChownFix
            | AdvancedCommand::SystemctlRestart
            | AdvancedCommand::MountCheck => vec!["linux", "macos"],
        }
    }

    /// Returns all commands supported on the current platform
    pub fn for_current_platform() -> Vec<AdvancedCommand> {
        let current_os = if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "linux") {
            "linux"
        } else {
            "macos"
        };

        Self::all()
            .into_iter()
            .filter(|cmd| cmd.supported_os().contains(&current_os))
            .collect()
    }

    /// Returns all advanced commands
    fn all() -> Vec<AdvancedCommand> {
        vec![
            AdvancedCommand::SfcScannow,
            AdvancedCommand::DismRestoreHealth,
            AdvancedCommand::ChkdskRepair,
            AdvancedCommand::BootrecFixMbr,
            AdvancedCommand::BootrecFixBoot,
            AdvancedCommand::CryptoServiceRestart,
            AdvancedCommand::StartupRepair,
            AdvancedCommand::RegistryEditor,
            AdvancedCommand::FsckCheck,
            AdvancedCommand::E2fsckRepair,
            AdvancedCommand::XfsRepair,
            AdvancedCommand::ChmodFix,
            AdvancedCommand::ChownFix,
            AdvancedCommand::SystemctlRestart,
            AdvancedCommand::MountCheck,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_issue_names() {
        assert_eq!(SystemIssue::Bluetooth.name(), "Bluetooth Issues");
        assert_eq!(SystemIssue::WiFi.name(), "WiFi Connection Problems");
        assert!(!SystemIssue::Audio.name().is_empty());
    }

    #[test]
    fn test_system_issue_icons() {
        assert_eq!(SystemIssue::Bluetooth.icon(), "📡");
        assert_eq!(SystemIssue::WiFi.icon(), "📶");
        assert!(!SystemIssue::Audio.icon().is_empty());
    }

    #[test]
    fn test_all_system_issues() {
        let issues = SystemIssue::all();
        assert_eq!(issues.len(), 13);
        assert!(issues.contains(&SystemIssue::Bluetooth));
        assert!(issues.contains(&SystemIssue::AdvancedCommands));
    }

    #[test]
    fn test_advanced_command_os_support() {
        let sfc = AdvancedCommand::SfcScannow;
        assert!(sfc.supported_os().contains(&"windows"));
        assert!(!sfc.supported_os().contains(&"linux"));

        let fsck = AdvancedCommand::FsckCheck;
        assert!(fsck.supported_os().contains(&"linux"));
        assert!(!fsck.supported_os().contains(&"windows"));
    }

    #[test]
    fn test_platform_filtering() {
        let commands = AdvancedCommand::for_current_platform();
        assert!(!commands.is_empty());

        // All returned commands should support current platform
        let current_os = if cfg!(target_os = "windows") {
            "windows"
        } else {
            "linux"
        };

        for cmd in commands {
            assert!(cmd.supported_os().contains(&current_os));
        }
    }
}