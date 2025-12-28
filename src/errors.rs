//! Error Type Definitions
//!
//! FILE LOCATION: src/errors.rs
//!
//! This module provides custom error types for the repair toolkit.
//! Using custom errors instead of generic strings provides better
//! error handling, clearer diagnostics, and type-safe error propagation.
//!
//! # Examples
//!
//! ```rust
//! use system_repair_toolkit::errors::RepairError;
//!
//! fn check_hardware() -> Result<(), RepairError> {
//!     Err(RepairError::HardwareNotFound("Bluetooth adapter".to_string()))
//! }
//! ```

use thiserror::Error;

/// Custom error types for repair operations
///
/// These error variants represent specific failure modes that can occur
/// during system repair operations. Each variant includes contextual
/// information to help diagnose and resolve the issue.
#[derive(Debug, Error)]
pub enum RepairError {
    /// Hardware device not detected on the system
    ///
    /// This error occurs when attempting to repair a device that doesn't
    /// exist or isn't recognized by the operating system.
    ///
    /// # Example
    /// ```
    /// RepairError::HardwareNotFound("Bluetooth adapter".to_string())
    /// ```
    #[error("Hardware not detected: {0}")]
    HardwareNotFound(String),

    /// External command execution failed
    ///
    /// This error occurs when a system command exits with a non-zero status
    /// or cannot be executed at all.
    #[error("Command execution failed: {0}")]
    CommandFailed(String),

    /// Operation requires elevated privileges
    ///
    /// This error indicates the operation needs administrator or root access.
    /// On Windows, run as Administrator. On Unix systems, use sudo.
    #[error("Insufficient permissions - run as Administrator on Windows or with sudo on Unix")]
    PermissionDenied,

    /// Operation exceeded configured timeout duration
    ///
    /// This error occurs when an operation takes longer than the configured
    /// timeout period, preventing indefinite hangs.
    #[error("Operation timeout - exceeded {0} seconds")]
    Timeout(u64),

    /// Service is not available or not installed
    ///
    /// This error occurs when trying to interact with a service that doesn't
    /// exist on the system.
    #[error("Service not available: {0}")]
    ServiceUnavailable(String),

    /// Configuration is invalid or corrupted
    ///
    /// This error occurs when the configuration file cannot be parsed or
    /// contains invalid values.
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// I/O error
    ///
    /// This error occurs during input/output operations.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Task join error
    ///
    /// This error occurs when a spawned task fails to join.
    #[error("Task join error: {0}")]
    Join(#[from] tokio::task::JoinError),
}

/// Result type alias for repair operations
///
/// This type alias simplifies function signatures throughout the codebase
/// by providing a standard Result type with our custom error.
///
/// # Example
/// ```rust
/// use system_repair_toolkit::errors::RepairResult;
///
/// fn repair_bluetooth() -> RepairResult<()> {
///     Ok(())
/// }
/// ```
pub type RepairResult<T> = Result<T, RepairError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_not_found_error() {
        let err = RepairError::HardwareNotFound("Test Device".to_string());
        assert_eq!(err.to_string(), "Hardware not detected: Test Device");
    }

    #[test]
    fn test_permission_denied_error() {
        let err = RepairError::PermissionDenied;
        assert!(err.to_string().contains("Administrator"));
    }

    #[test]
    fn test_timeout_error() {
        let err = RepairError::Timeout(30);
        assert!(err.to_string().contains("30 seconds"));
    }

    #[test]
    fn test_error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<RepairError>();
    }
}