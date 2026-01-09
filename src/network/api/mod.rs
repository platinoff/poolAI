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
pub mod libraries;
pub mod raid;
pub mod rewards;
pub mod system;
pub mod users;
pub mod vm;
pub mod workers;

// Re-export check_permission for backward compatibility with api_legacy.rs
pub use common::check_permission;

// All API modules have been migrated to modular structure ✅
// api_legacy.rs is kept for backward compatibility and distributed RAID handlers

use axum::Router;

/// Create API routes
///
/// Composes all API endpoint modules into a single router.
///
/// Currently, most routes are still in `api_legacy.rs` and will be gradually migrated.
pub fn create_api_routes() -> Router {
    Router::new()
        .merge(system::create_system_routes())
        .merge(workers::create_workers_routes())
        .merge(rewards::create_rewards_routes())
        .merge(vm::create_vm_routes())
        .merge(raid::create_raid_routes())
        .merge(libraries::create_libraries_routes())
        .merge(users::create_users_routes())
        // api_legacy.rs is kept for backward compatibility
        // All handlers have been migrated to modular structure
}

// Legacy routes function removed - all handlers migrated to modules ✅
