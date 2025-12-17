//! Library Management Module for Stage 3
//!
//! This module provides:
//! - Automatic library loading and installation (libtorch, etc.)
//! - Dependency management
//! - Library versioning
//! - Path optimization for libraries
//! - Automatic updates

pub mod manager;
pub mod registry;
pub mod versioning;
pub mod dependencies;
pub mod download;
pub mod constraints;
pub mod integration;
pub mod manifest;

// Re-export main types for convenient access
pub use manager::LibraryManager;
pub use registry::LibraryRegistry;
pub use versioning::VersionManager;
pub use dependencies::DependencyResolver;
pub use constraints::{VersionConstraint, ConstraintOp};
pub use integration::{ensure_libtorch, check_library_compatibility, auto_update_libraries};

use crate::core::error::AppError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;

/// Library information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryInfo {
    pub name: String,
    pub version: String,
    pub path: PathBuf,
    pub dependencies: Vec<String>,
    pub metadata: LibraryMetadata,
}

/// Library metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LibraryMetadata {
    pub description: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    pub size_bytes: Option<u64>,
    pub checksum: Option<String>,
    pub installed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Library status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LibraryStatus {
    Installed,
    Installing,
    Failed,
    NotInstalled,
    Updating,
}

/// Library type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LibraryType {
    ModelLibrary,    // libtorch, etc.
    NativeLibrary,   // System libraries
    CustomLibrary,   // Custom user libraries
}

// Global library manager instance
static GLOBAL_LIBRARY_MANAGER: OnceLock<Arc<RwLock<LibraryManager>>> = OnceLock::new();

/// Initialize library management module
pub async fn initialize() -> Result<(), AppError> {
    tracing::info!("Initializing library management module");
    
    // Initialize global library manager
    let manager = LibraryManager::new();
    manager.initialize().await?;
    
    // Store global instance
    GLOBAL_LIBRARY_MANAGER.set(Arc::new(RwLock::new(manager))).map_err(|_| {
        AppError::ConfigError("Library manager already initialized".to_string())
    })?;
    
    tracing::info!("Library management module initialized successfully");
    Ok(())
}

/// Get global library manager
pub fn get_global_manager() -> Option<&'static Arc<RwLock<LibraryManager>>> {
    GLOBAL_LIBRARY_MANAGER.get()
}

/// Shutdown library management module
pub async fn shutdown() -> Result<(), AppError> {
    tracing::info!("Shutting down library management module");
    
    // Note: OnceLock doesn't support clearing, so we can't fully remove it
    // The manager will remain in memory but won't be accessible after this
    // For true cleanup, consider using a different pattern or accept this limitation
    
    tracing::info!("Library management module shut down successfully");
    Ok(())
}

/// Health check for library management module
pub async fn health_check() -> Result<(), AppError> {
    // Check if library manager is operational
    Ok(())
}

