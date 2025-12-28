//! Configuration Management System
//!
//! FILE LOCATION: src/config.rs
//!
//! This module handles loading, saving, and managing user configuration.
//! Configuration is stored in TOML format in the user's config directory
//! following platform conventions.
//!
//! # Platform-Specific Paths
//!
//! - Windows: `%USERPROFILE%\.config\system-repair-toolkit\config.toml`
//! - Linux/macOS: `~/.config/system-repair-toolkit/config.toml`
//!
//! # Example Configuration File
//!
//! ```toml
//! auto_restart = false
//! verbose = true
//! log_level = "info"
//! confirm_actions = true
//! timeout_seconds = 30
//! ```

use anyhow::{Context, Result};
use log::info;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application configuration structure
///
/// This structure holds all user-configurable preferences that persist
/// between application sessions. Values are loaded from a TOML file on
/// startup and can be modified by editing the configuration file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// Whether to automatically restart services without prompting
    ///
    /// When true, services are restarted immediately during repairs.
    /// When false, the user is prompted before each restart.
    /// Default: false (safer, requires confirmation)
    pub auto_restart: bool,

    /// Enable verbose output with detailed status messages
    ///
    /// When true, displays detailed progress information during repairs.
    /// When false, shows only essential information.
    /// Default: true (helpful for understanding what's happening)
    pub verbose: bool,

    /// Logging verbosity level
    ///
    /// Valid values: "trace", "debug", "info", "warn", "error"
    /// Can be overridden with RUST_LOG environment variable.
    /// Default: "info"
    pub log_level: String,

    /// Require user confirmation before dangerous operations
    ///
    /// When true, prompts user before registry edits, disk operations, etc.
    /// When false, executes operations immediately (use with caution).
    /// Default: true (prevents accidental damage)
    pub confirm_actions: bool,

    /// Maximum duration for operations before timeout (seconds)
    ///
    /// Operations exceeding this duration will be cancelled to prevent hangs.
    /// Applies to network operations, command execution, etc.
    /// Default: 30 seconds
    pub timeout_seconds: u64,

    /// Show progress bars for long-running operations
    ///
    /// When true, displays visual progress indicators.
    /// When false, uses simple text output.
    /// Default: true (better user experience)
    pub show_progress: bool,

    /// Maximum number of repair retries on failure
    ///
    /// If a repair fails, it will be retried up to this many times.
    /// Set to 0 to disable retries.
    /// Default: 2
    pub max_retries: u32,
}

impl Default for Config {
    /// Creates a configuration with sensible default values
    ///
    /// These defaults prioritize safety and user awareness over convenience.
    /// They provide a good starting point for most users while allowing
    /// customization for advanced use cases.
    fn default() -> Self {
        Config {
            auto_restart: false,
            verbose: true,
            log_level: "info".to_string(),
            confirm_actions: true,
            timeout_seconds: 30,
            show_progress: true,
            max_retries: 2,
        }
    }
}

impl Config {
    /// Loads configuration from the standard configuration file
    ///
    /// If the configuration file doesn't exist, creates it with default values.
    /// If the file exists but is corrupted, returns an error with details.
    ///
    /// # Returns
    ///
    /// Returns the loaded configuration or an error if loading fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use system_repair_toolkit::config::Config;
    ///
    /// let config = Config::load().expect("Failed to load config");
    /// println!("Verbose mode: {}", config.verbose);
    /// ```
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        if config_path.exists() {
            // Configuration exists, load and parse it
            let contents = fs::read_to_string(&config_path)
                .context("Failed to read configuration file")?;

            let config: Config = toml::from_str(&contents)
                .context("Failed to parse configuration - check TOML syntax")?;

            // Validate configuration values
            config.validate()?;

            info!("Configuration loaded from {:?}", config_path);
            Ok(config)
        } else {
            // No configuration exists, create default
            let config = Config::default();
            config.save()?;
            info!("Created default configuration at {:?}", config_path);
            Ok(config)
        }
    }

    /// Saves the current configuration to disk
    ///
    /// Serializes the configuration to TOML format and writes it to the
    /// standard configuration file. Creates parent directories if needed.
    ///
    /// # Returns
    ///
    /// Returns Ok(()) on success, or an error if saving fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use system_repair_toolkit::config::Config;
    ///
    /// let mut config = Config::default();
    /// config.verbose = false;
    /// config.save().expect("Failed to save config");
    /// ```
    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;

        // Ensure parent directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create configuration directory")?;
        }

        // Serialize to pretty-printed TOML
        let contents = toml::to_string_pretty(self)
            .context("Failed to serialize configuration")?;

        // Write to file
        fs::write(&config_path, contents)
            .context("Failed to write configuration file")?;

        info!("Configuration saved to {:?}", config_path);
        Ok(())
    }

    /// Validates configuration values
    ///
    /// Ensures all configuration values are within acceptable ranges and
    /// formats. Returns an error if any value is invalid.
    fn validate(&self) -> Result<()> {
        // Validate log level
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&self.log_level.as_str()) {
            anyhow::bail!(
                "Invalid log_level '{}'. Must be one of: {}",
                self.log_level,
                valid_levels.join(", ")
            );
        }

        // Validate timeout
        if self.timeout_seconds == 0 {
            anyhow::bail!("timeout_seconds must be greater than 0");
        }

        if self.timeout_seconds > 3600 {
            anyhow::bail!("timeout_seconds must not exceed 3600 (1 hour)");
        }

        Ok(())
    }

    /// Determines the standard configuration file path
    ///
    /// Returns the platform-appropriate path for storing configuration.
    /// On Windows, uses USERPROFILE environment variable.
    /// On Unix systems, uses HOME environment variable.
    fn get_config_path() -> Result<PathBuf> {
        // Try Windows path first
        let home = std::env::var("USERPROFILE")
            .or_else(|_| std::env::var("HOME"))
            .context("Cannot determine home directory - set USERPROFILE or HOME")?;

        Ok(PathBuf::from(home)
            .join(".config")
            .join("system-repair-toolkit")
            .join("config.toml"))
    }

    /// Returns the log level as a log::LevelFilter
    ///
    /// Converts the string log level to the appropriate enum value
    /// for use with the logging system.
    pub fn get_log_filter(&self) -> log::LevelFilter {
        match self.log_level.as_str() {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            _ => log::LevelFilter::Info,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.auto_restart, false);
        assert_eq!(config.verbose, true);
        assert_eq!(config.log_level, "info");
        assert_eq!(config.confirm_actions, true);
        assert_eq!(config.timeout_seconds, 30);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        assert!(config.validate().is_ok());

        // Test invalid log level
        config.log_level = "invalid".to_string();
        assert!(config.validate().is_err());

        // Test invalid timeout
        config.log_level = "info".to_string();
        config.timeout_seconds = 0;
        assert!(config.validate().is_err());

        config.timeout_seconds = 5000;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_log_filter_conversion() {
        let mut config = Config::default();

        config.log_level = "debug".to_string();
        assert_eq!(config.get_log_filter(), log::LevelFilter::Debug);

        config.log_level = "error".to_string();
        assert_eq!(config.get_log_filter(), log::LevelFilter::Error);
    }

    #[test]
    fn test_config_path() {
        let path = Config::get_config_path();
        assert!(path.is_ok());
        let path = path.unwrap();
        assert!(path.to_string_lossy().contains("system-repair-toolkit"));
        assert!(path.to_string_lossy().ends_with("config.toml"));
    }
}