//! Integration with Model Interface
//!
//! Provides:
//! - Automatic libtorch loading
//! - Version compatibility checking
//! - Automatic library updates

use crate::core::error::AppError;
use crate::libs::{get_global_manager, LibraryStatus, LibraryType};
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
                    // Use improved compatibility check
                    if let Some(required) = required_version {
                        let compatible = check_libtorch_compatibility(Some(required)).await?;
                        if !compatible {
                            warn!(
                                "libtorch version mismatch: installed {}, required {}",
                                lib.version, required
                            );
                            // Future improvement: Auto-update if needed (based on auto-update policy)
                            // 1. Check auto-update policy from config
                            //    - Get auto_update_enabled flag from LibraryConfig
                            //    - Get auto_update_policy (always, major_only, never)
                            // 2. Determine if update is needed
                            //    - Compare installed version with required version
                            //    - Check if update is allowed by policy (major/minor/patch)
                            //    - Check if update is available from remote registry
                            // 3. Perform auto-update
                            //    - Call manager.update_library() to update to required version
                            //    - Handle update errors gracefully (log warnings, continue with incompatible version)
                            //    - Verify updated library compatibility after update
                            // 4. Handle update failures
                            //    - If update fails, warn but don't block execution (if possible)
                            //    - Fall back to incompatible version if update not possible
                            //    - Log update attempts for monitoring
                            // Example:
                            //    if let Some(config) = &self.auto_update_config {
                            //        if config.enabled && should_auto_update(&lib.version, &required, &config.policy) {
                            //            if let Err(e) = manager.update_library(&lib.name).await {
                            //                warn!("Auto-update failed for {}: {}", lib.name, e);
                            //            }
                            //        }
                            //    }
                        }
                    }
                    info!("libtorch v{} is installed", lib.version);
                    Ok(())
                } else {
                    Err(AppError::ConfigError(
                        "libtorch not found despite status check".to_string(),
                    ))
                }
            }
            _ => {
                // Install libtorch
                info!("Installing libtorch");
                let version = required_version.unwrap_or("latest");
                manager
                    .install_library("libtorch", version, LibraryType::ModelLibrary)
                    .await?;
                info!("libtorch installed successfully");
                Ok(())
            }
        }
    } else {
        Err(AppError::ConfigError(
            "Library manager not initialized".to_string(),
        ))
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
            info!(
                "Library {} v{} is compatible with model v{}",
                library_name, lib.version, model_version
            );
            Ok(true)
        } else {
            warn!("Library {} not found for compatibility check", library_name);
            Ok(false)
        }
    } else {
        Err(AppError::ConfigError(
            "Library manager not initialized".to_string(),
        ))
    }
}

/// Check libtorch compatibility with model requirements
///
/// This function checks if the installed libtorch version meets the model's requirements.
/// For libtorch, we typically check semantic version compatibility (major.minor match).
pub async fn check_libtorch_compatibility(
    required_version: Option<&str>,
) -> Result<bool, AppError> {
    if let Some(manager) = get_global_manager() {
        let manager = manager.read().await;

        if let Some(libtorch) = manager.get_library("libtorch").await {
            if let Some(required) = required_version {
                // Parse versions and check compatibility
                // For libtorch, we check if major.minor versions match
                // (patch versions are usually compatible)
                let installed_parts: Vec<&str> = libtorch.version.split('.').collect();
                let required_parts: Vec<&str> = required.split('.').collect();

                if installed_parts.len() >= 2 && required_parts.len() >= 2 {
                    let installed_major = installed_parts[0].parse::<u32>().unwrap_or(0);
                    let installed_minor = installed_parts[1].parse::<u32>().unwrap_or(0);
                    let required_major = required_parts[0].parse::<u32>().unwrap_or(0);
                    let required_minor = required_parts[1].parse::<u32>().unwrap_or(0);

                    // Check if major.minor versions match
                    if installed_major == required_major && installed_minor == required_minor {
                        info!(
                            "libtorch v{} matches required v{} (major.minor)",
                            libtorch.version, required
                        );
                        Ok(true)
                    } else {
                        warn!("libtorch version mismatch: installed v{} (major.minor: {}.{}), required v{} (major.minor: {}.{})", 
                              libtorch.version, installed_major, installed_minor,
                              required, required_major, required_minor);
                        Ok(false)
                    }
                } else {
                    // Fallback: exact version match
                    if libtorch.version == required {
                        info!(
                            "libtorch v{} matches required v{} (exact)",
                            libtorch.version, required
                        );
                        Ok(true)
                    } else {
                        warn!(
                            "libtorch version mismatch: installed {}, required {}",
                            libtorch.version, required
                        );
                        Ok(false)
                    }
                }
            } else {
                // No specific version required, just check if installed
                info!(
                    "libtorch v{} is installed (no version requirement)",
                    libtorch.version
                );
                Ok(true)
            }
        } else {
            warn!("libtorch not installed for compatibility check");
            Ok(false)
        }
    } else {
        Err(AppError::ConfigError(
            "Library manager not initialized".to_string(),
        ))
    }
}

/// Auto-update policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoUpdatePolicy {
    /// Never auto-update (manual updates only)
    Never,
    /// Auto-update on startup
    OnStartup,
    /// Auto-update on compatibility mismatch
    OnMismatch,
    /// Auto-update on startup and on mismatch
    OnStartupAndMismatch,
}

impl Default for AutoUpdatePolicy {
    fn default() -> Self {
        Self::OnMismatch
    }
}

/// Auto-update libraries if needed based on policy
pub async fn auto_update_libraries(policy: AutoUpdatePolicy) -> Result<(), AppError> {
    if policy == AutoUpdatePolicy::Never {
        info!("Auto-update disabled (policy: Never)");
        return Ok(());
    }

    info!("Checking for library updates (policy: {:?})", policy);

    if let Some(manager) = get_global_manager() {
        let manager = manager.read().await;
        let libraries = manager.list_libraries().await;

        for lib in libraries {
            // Try to update library (update_library checks for latest version internally)
            match manager.update_library(&lib.name).await {
                Ok(updated_lib) => {
                    if updated_lib.version != lib.version {
                        info!(
                            "Updated {} from {} to {}",
                            lib.name, lib.version, updated_lib.version
                        );
                    } else {
                        info!(
                            "{} is already at latest version ({})",
                            lib.name, lib.version
                        );
                    }
                }
                Err(e) => {
                    warn!("Failed to check updates for {}: {}", lib.name, e);
                }
            }
        }

        Ok(())
    } else {
        Err(AppError::ConfigError(
            "Library manager not initialized".to_string(),
        ))
    }
}

/// Auto-update libtorch if version mismatch detected (based on policy)
pub async fn auto_update_libtorch_if_needed(
    required_version: Option<&str>,
    policy: AutoUpdatePolicy,
) -> Result<(), AppError> {
    if policy == AutoUpdatePolicy::Never {
        return Ok(());
    }

    if let Some(required) = required_version {
        let compatible = check_libtorch_compatibility(Some(required)).await?;

        if !compatible
            && (policy == AutoUpdatePolicy::OnMismatch
                || policy == AutoUpdatePolicy::OnStartupAndMismatch)
        {
            info!(
                "Auto-updating libtorch due to version mismatch (policy: {:?})",
                policy
            );
            if let Some(manager) = get_global_manager() {
                let manager = manager.read().await;
                manager.update_library("libtorch").await?;
                info!("libtorch auto-update completed");
            }
        }
    }

    Ok(())
}
