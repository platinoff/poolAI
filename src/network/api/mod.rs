//! API routes module
//!
//! Provides REST API endpoints organized by functionality.
//!
//! # Modules
//!
//! - `system` - System endpoints (status, health, metrics, login, models, gpu)
//! - `workers` - Worker management endpoints
//! - `vm` - VM instance management endpoints
//! - `raid` - RAID management endpoints
//! - `libraries` - Library management endpoints
//! - `users` - User management endpoints
//! - `rewards` - Rewards system endpoints
//! - `common` - Shared types and utilities

pub mod common;
pub mod system;

// Re-export check_permission for backward compatibility with api_legacy.rs
pub use common::check_permission;

// Note: Other modules (workers, vm, raid, libraries, users, rewards) are still in api_legacy.rs
// They will be gradually migrated to modular structure
// TODO: Create modules: workers, vm, raid, libraries, users, rewards

use axum::Router;

/// Create API routes
///
/// Composes all API endpoint modules into a single router.
///
/// Currently, most routes are still in `api_legacy.rs` and will be gradually migrated.
pub fn create_api_routes() -> Router {
    Router::new()
        .merge(system::create_system_routes())
        // Temporarily delegate to legacy implementation for other routes
        // TODO: Migrate to modular structure
        .merge(create_legacy_routes())
}

/// Temporary function to create routes from legacy api.rs file
/// TODO: Remove once all handlers are migrated to modules
fn create_legacy_routes() -> Router {
    use crate::network::api_legacy::create_api_routes;
    create_api_routes()
}
