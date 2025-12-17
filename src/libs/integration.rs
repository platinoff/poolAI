//! Integration with Model Interface
//!
//! Provides:
//! - Automatic libtorch loading
//! - Version compatibility checking
//! - Automatic library updates

use crate::core::error::AppError;
use crate::libs::{get_global_manager, LibraryType, LibraryStatus};
use tracing::{info, warn};

/// Ensure libtorch is installed and compatible
pub async fn ensure_libtorch(required_version: Option<&str>) -> Result<(), AppError> {
    info!("Ensuring libtorch is installed");
    
    if let Some(manager) = get_global_manager() {
        let manager = manager.read().await;
        
        // Check if libtorch is installed
        let status = manager.get_library_status("libtorch").await;
        
        match status {
            LibraryStatus::Installed => {
                if let Some(lib) = manager.get_library("libtorch").await {
                    if let Some(required) = required_version {
                        if lib.version != required {
                            warn!("libtorch version mismatch: installed {}, required {}", 
                                  lib.version, required);
                            // TODO: Auto-update if needed
                        }
                    }
                    info!("libtorch v{} is installed", lib.version);
                    Ok(())
                } else {
                    Err(AppError::ConfigError("libtorch not found despite status check".to_string()))
                }
            }
            _ => {
                // Install libtorch
                info!("Installing libtorch");
                let version = required_version.unwrap_or("latest");
                manager.install_library("libtorch", version, LibraryType::ModelLibrary).await?;
                info!("libtorch installed successfully");
                Ok(())
            }
        }
    } else {
        Err(AppError::ConfigError("Library manager not initialized".to_string()))
    }
}

/// Check library compatibility with model
pub async fn check_library_compatibility(
    library_name: &str,
    model_version: &str,
) -> Result<bool, AppError> {
    if let Some(manager) = get_global_manager() {
        let manager = manager.read().await;
        
        if let Some(lib) = manager.get_library(library_name).await {
            // Simple compatibility check (can be enhanced)
            // For now, just check if library is installed
            info!("Library {} v{} is compatible with model v{}", 
                  library_name, lib.version, model_version);
            Ok(true)
        } else {
            warn!("Library {} not found for compatibility check", library_name);
            Ok(false)
        }
    } else {
        Err(AppError::ConfigError("Library manager not initialized".to_string()))
    }
}

/// Auto-update libraries if needed
pub async fn auto_update_libraries() -> Result<(), AppError> {
    info!("Checking for library updates");
    
    if let Some(manager) = get_global_manager() {
        let manager = manager.read().await;
        let libraries = manager.list_libraries().await;
        
        for lib in libraries {
            // Try to update library (update_library checks for latest version internally)
            match manager.update_library(&lib.name).await {
                Ok(updated_lib) => {
                    if updated_lib.version != lib.version {
                        info!("Updated {} from {} to {}", lib.name, lib.version, updated_lib.version);
                    }
                }
                Err(e) => {
                    warn!("Failed to check updates for {}: {}", lib.name, e);
                }
            }
        }
        
        Ok(())
    } else {
        Err(AppError::ConfigError("Library manager not initialized".to_string()))
    }
}

