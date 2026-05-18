//! Core module for PoolAI system
//! Provides core functionality for model interface, error handling, configuration, and state management

pub mod config;
pub mod dev_stand;
pub mod discovery_handle;
pub mod discovery_types;
pub mod error;
pub mod model_interface;
pub mod state;
pub mod user_manager;
pub mod ws_manager;

#[cfg(feature = "enterprise")]
pub mod oauth2_pending;

use self::config::{get_config, initialize_config, PoolAIConfig};
use self::error::AppError;
use tracing::info;

/// Initialize core module
pub async fn initialize() -> Result<(), AppError> {
    info!("Initializing core module");

    // Initialize with default configuration
    let config = PoolAIConfig::default();
    initialize_config(config)?;

    info!("Core module initialized successfully");
    Ok(())
}

/// Initialize core module with custom configuration
pub async fn initialize_with_config(config: PoolAIConfig) -> Result<(), AppError> {
    info!("Initializing core module with custom configuration");

    // Validate and initialize configuration
    config.validate()?;
    initialize_config(config)?;

    info!("Core module initialized with custom configuration successfully");
    Ok(())
}

/// Shutdown core module
pub async fn shutdown() -> Result<(), AppError> {
    info!("Shutting down core module");

    // Cleanup core components
    // Note: Global config cleanup is handled automatically

    info!("Core module shutdown completed");
    Ok(())
}

/// Health check for core module
pub async fn health_check() -> Result<(), AppError> {
    info!("Core module health check");

    // Check if configuration is available
    let _config = get_config()?;

    // Check core components health
    info!("Core module health check passed");
    Ok(())
}

/// Get system configuration
pub fn get_system_config() -> Result<PoolAIConfig, AppError> {
    get_config()
}

/// Update system configuration
pub fn update_system_config(config: PoolAIConfig) -> Result<(), AppError> {
    config.validate()?;
    self::config::update_config(config)?;
    info!("System configuration updated successfully");
    Ok(())
}
