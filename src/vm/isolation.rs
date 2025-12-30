//! VM Isolation module
//!
//! Provides network and filesystem isolation for VM instances.
//! This module defines traits and implementations for isolating VM instances
//! from the host system and from each other.

use crate::core::error::AppError;
use std::path::PathBuf;

/// Network isolation configuration
#[derive(Debug, Clone)]
pub struct NetworkIsolationConfig {
    /// Whether to enable network isolation
    pub enabled: bool,
    /// Allowed network interfaces (empty = all blocked)
    pub allowed_interfaces: Vec<String>,
    /// Allowed ports (empty = all blocked)
    pub allowed_ports: Vec<u16>,
    /// Whether to allow loopback
    pub allow_loopback: bool,
}

impl Default for NetworkIsolationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            allowed_interfaces: vec![],
            allowed_ports: vec![],
            allow_loopback: true,
        }
    }
}

/// Filesystem isolation configuration
#[derive(Debug, Clone)]
pub struct FilesystemIsolationConfig {
    /// Whether to enable filesystem isolation
    pub enabled: bool,
    /// Root directory for the VM instance
    pub root_dir: Option<PathBuf>,
    /// Allowed paths (empty = all blocked)
    pub allowed_paths: Vec<PathBuf>,
    /// Read-only paths
    pub read_only_paths: Vec<PathBuf>,
    /// Whether to use chroot
    pub use_chroot: bool,
}

impl Default for FilesystemIsolationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            root_dir: None,
            allowed_paths: vec![],
            read_only_paths: vec![],
            use_chroot: false,
        }
    }
}

/// Trait for network isolation implementation
pub trait NetworkIsolator: Send + Sync {
    /// Apply network isolation to a process
    ///
    /// # Arguments
    /// * `process_id` - Native process ID
    /// * `config` - Network isolation configuration
    ///
    /// # Returns
    /// `Ok(())` if isolation was applied successfully
    fn apply_network_isolation(
        &self,
        process_id: u32,
        config: &NetworkIsolationConfig,
    ) -> Result<(), AppError>;

    /// Remove network isolation from a process
    ///
    /// # Arguments
    /// * `process_id` - Native process ID
    ///
    /// # Returns
    /// `Ok(())` if isolation was removed successfully
    fn remove_network_isolation(&self, process_id: u32) -> Result<(), AppError>;

    /// Check if network isolation is supported on this platform
    fn is_supported(&self) -> bool;
}

/// Trait for filesystem isolation implementation
pub trait FilesystemIsolator: Send + Sync {
    /// Apply filesystem isolation to a process
    ///
    /// # Arguments
    /// * `process_id` - Native process ID
    /// * `config` - Filesystem isolation configuration
    ///
    /// # Returns
    /// `Ok(())` if isolation was applied successfully
    fn apply_filesystem_isolation(
        &self,
        process_id: u32,
        config: &FilesystemIsolationConfig,
    ) -> Result<(), AppError>;

    /// Remove filesystem isolation from a process
    ///
    /// # Arguments
    /// * `process_id` - Native process ID
    ///
    /// # Returns
    /// `Ok(())` if isolation was removed successfully
    fn remove_filesystem_isolation(&self, process_id: u32) -> Result<(), AppError>;

    /// Check if filesystem isolation is supported on this platform
    fn is_supported(&self) -> bool;
}

/// Platform-agnostic network isolator
///
/// Automatically selects the appropriate implementation based on the platform.
pub struct PlatformNetworkIsolator {
    #[cfg(target_os = "linux")]
    inner: linux::LinuxNetworkIsolator,
    #[cfg(target_os = "windows")]
    inner: windows::WindowsNetworkIsolator,
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    inner: noop::NoopNetworkIsolator,
}

impl PlatformNetworkIsolator {
    pub fn new() -> Self {
        Self {
            #[cfg(target_os = "linux")]
            inner: linux::LinuxNetworkIsolator::new(),
            #[cfg(target_os = "windows")]
            inner: windows::WindowsNetworkIsolator::new(),
            #[cfg(not(any(target_os = "linux", target_os = "windows")))]
            inner: noop::NoopNetworkIsolator::new(),
        }
    }
}

impl NetworkIsolator for PlatformNetworkIsolator {
    fn apply_network_isolation(
        &self,
        process_id: u32,
        config: &NetworkIsolationConfig,
    ) -> Result<(), AppError> {
        self.inner.apply_network_isolation(process_id, config)
    }

    fn remove_network_isolation(&self, process_id: u32) -> Result<(), AppError> {
        self.inner.remove_network_isolation(process_id)
    }

    fn is_supported(&self) -> bool {
        self.inner.is_supported()
    }
}

/// Platform-agnostic filesystem isolator
///
/// Automatically selects the appropriate implementation based on the platform.
pub struct PlatformFilesystemIsolator {
    #[cfg(target_os = "linux")]
    inner: linux::LinuxFilesystemIsolator,
    #[cfg(target_os = "windows")]
    inner: windows::WindowsFilesystemIsolator,
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    inner: noop::NoopFilesystemIsolator,
}

impl PlatformFilesystemIsolator {
    pub fn new() -> Self {
        Self {
            #[cfg(target_os = "linux")]
            inner: linux::LinuxFilesystemIsolator::new(),
            #[cfg(target_os = "windows")]
            inner: windows::WindowsFilesystemIsolator::new(),
            #[cfg(not(any(target_os = "linux", target_os = "windows")))]
            inner: noop::NoopFilesystemIsolator::new(),
        }
    }
}

impl FilesystemIsolator for PlatformFilesystemIsolator {
    fn apply_filesystem_isolation(
        &self,
        process_id: u32,
        config: &FilesystemIsolationConfig,
    ) -> Result<(), AppError> {
        self.inner.apply_filesystem_isolation(process_id, config)
    }

    fn remove_filesystem_isolation(&self, process_id: u32) -> Result<(), AppError> {
        self.inner.remove_filesystem_isolation(process_id)
    }

    fn is_supported(&self) -> bool {
        self.inner.is_supported()
    }
}

// Platform-specific implementations
#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
pub mod noop;

