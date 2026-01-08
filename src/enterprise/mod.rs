//! Enterprise features module
//!
//! This module provides enterprise-grade features including:
//! - Multi-tenancy support
//! - Advanced security (OAuth2, SAML)
//! - Comprehensive audit logging
//! - Advanced monitoring and analytics
//!
//! # Architecture
//!
//! The enterprise module follows Rust best practices:
//! - Zero-cost abstractions with trait-based design
//! - Thread-safe state management with `Arc<RwLock<T>>`
//! - Async/await for non-blocking operations
//! - Type-safe error handling with `Result<T, AppError>`
//!
//! # Example
//!
//! ```rust,no_run
//! use poolai::enterprise::{AuditLogger, AuditEvent};
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let logger = AuditLogger::new();
//! logger.initialize().await?;
//!
//! logger.log_event(AuditEvent {
//!     user_id: "user123".to_string(),
//!     action: "create_instance".to_string(),
//!     resource: "vm:instance-456".to_string(),
//!     result: "success".to_string(),
//! }).await?;
//! # Ok(())
//! # }
//! ```

pub mod audit;
pub mod multi_tenancy;
pub mod security;
pub mod monitoring;

use crate::core::error::AppError;
use std::sync::Arc;
use tracing::info;

/// Enterprise module manager
///
/// Coordinates all enterprise features and provides a unified interface
/// for initialization and management.
pub struct EnterpriseManager {
    audit_logger: Arc<audit::AuditLogger>,
    tenant_manager: Arc<multi_tenancy::TenantManager>,
    security_manager: Arc<security::SecurityManager>,
    monitoring_manager: Arc<monitoring::MonitoringManager>,
}

impl EnterpriseManager {
    /// Creates a new enterprise manager instance
    pub fn new() -> Self {
        Self {
            audit_logger: Arc::new(audit::AuditLogger::new()),
            tenant_manager: Arc::new(multi_tenancy::TenantManager::new()),
            security_manager: Arc::new(security::SecurityManager::new()),
            monitoring_manager: Arc::new(monitoring::MonitoringManager::new()),
        }
    }

    /// Initializes all enterprise features
    ///
    /// # Errors
    ///
    /// Returns `AppError` if initialization fails for any component.
    pub async fn initialize(&self) -> Result<(), AppError> {
        info!("Initializing enterprise features...");

        self.audit_logger.initialize().await?;
        info!("✅ Audit logger initialized");

        self.tenant_manager.initialize().await?;
        info!("✅ Tenant manager initialized");

        self.security_manager.initialize().await?;
        info!("✅ Security manager initialized");

        self.monitoring_manager.initialize().await?;
        info!("✅ Monitoring manager initialized");

        info!("✅ All enterprise features initialized");
        Ok(())
    }

    /// Shuts down all enterprise features gracefully
    pub async fn shutdown(&self) -> Result<(), AppError> {
        info!("Shutting down enterprise features...");

        self.audit_logger.shutdown().await?;
        self.tenant_manager.shutdown().await?;
        self.security_manager.shutdown().await?;
        self.monitoring_manager.shutdown().await?;

        info!("✅ All enterprise features shut down");
        Ok(())
    }

    /// Returns a reference to the audit logger
    pub fn audit_logger(&self) -> &Arc<audit::AuditLogger> {
        &self.audit_logger
    }

    /// Returns a reference to the tenant manager
    pub fn tenant_manager(&self) -> &Arc<multi_tenancy::TenantManager> {
        &self.tenant_manager
    }

    /// Returns a reference to the security manager
    pub fn security_manager(&self) -> &Arc<security::SecurityManager> {
        &self.security_manager
    }

    /// Returns a reference to the monitoring manager
    pub fn monitoring_manager(&self) -> &Arc<monitoring::MonitoringManager> {
        &self.monitoring_manager
    }
}

impl Default for EnterpriseManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Initializes the enterprise module
///
/// This is a convenience function that creates and initializes
/// the enterprise manager.
pub async fn initialize() -> Result<Arc<EnterpriseManager>, AppError> {
    let manager = Arc::new(EnterpriseManager::new());
    manager.initialize().await?;
    Ok(manager)
}
