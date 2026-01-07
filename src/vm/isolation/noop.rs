//! No-op isolation implementations for unsupported platforms
//!
//! These implementations do nothing but return success, allowing the code
//! to compile and run on platforms where isolation is not yet implemented.

use crate::core::error::AppError;
use crate::vm::isolation::{
    FilesystemIsolationConfig, FilesystemIsolator, NetworkIsolationConfig, NetworkIsolator,
};

/// No-op network isolator for unsupported platforms
pub struct NoopNetworkIsolator;

impl NoopNetworkIsolator {
    pub fn new() -> Self {
        Self
    }
}

impl NetworkIsolator for NoopNetworkIsolator {
    fn apply_network_isolation(
        &self,
        _process_id: u32,
        _config: &NetworkIsolationConfig,
    ) -> Result<(), AppError> {
        // No-op: isolation not supported on this platform
        Ok(())
    }

    fn remove_network_isolation(&self, _process_id: u32) -> Result<(), AppError> {
        // No-op: isolation not supported on this platform
        Ok(())
    }

    fn is_supported(&self) -> bool {
        false
    }
}

/// No-op filesystem isolator for unsupported platforms
pub struct NoopFilesystemIsolator;

impl NoopFilesystemIsolator {
    pub fn new() -> Self {
        Self
    }
}

impl FilesystemIsolator for NoopFilesystemIsolator {
    fn apply_filesystem_isolation(
        &self,
        _process_id: u32,
        _config: &FilesystemIsolationConfig,
    ) -> Result<(), AppError> {
        // No-op: isolation not supported on this platform
        Ok(())
    }

    fn remove_filesystem_isolation(&self, _process_id: u32) -> Result<(), AppError> {
        // No-op: isolation not supported on this platform
        Ok(())
    }

    fn is_supported(&self) -> bool {
        false
    }
}
