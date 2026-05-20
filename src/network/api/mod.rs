//! API routes module
//!
//! Provides REST API endpoints organized by functionality.
//!
//! # Modules
//!
//! - `admin` - Admin overview JSON (`/admin/overview`)
//! - `system` - System endpoints (status, health, metrics, login, models, gpu)
//! - `workers` - Worker management endpoints
//! - `vm` - VM instance management endpoints
//! - `raid` - RAID management endpoints
//! - `libraries` - Library management endpoints
//! - `users` - User management endpoints
//! - `rewards` - Rewards system endpoints
//! - `common` - Shared types and utilities

pub mod admin;
#[cfg(feature = "ml")]
pub mod ai_ml;
pub mod common;
pub mod completions;
pub mod discovery;
pub mod grid;
pub mod instances;
pub mod jobs;
pub mod libraries;
pub mod memory;
pub mod raid;
pub mod raid_admin;
pub(crate) mod raid_http;
pub mod rewards;
pub mod system;
mod system_status_html;
pub mod topology;
pub mod ui;
pub mod users;
pub mod virtual_nodes;
pub mod vm;
pub mod workers;

pub use common::check_permission;

use crate::core::state::ApiContext;
use axum::Router;

/// Create API routes
///
/// Composes all API endpoint modules into a single router.
///
/// Composes modular `api/*` routers (canonical REST surface under `/api/v1`).
pub fn create_api_routes() -> Router<ApiContext> {
    Router::new()
        .merge(admin::create_admin_routes())
        .merge(system::create_system_routes())
        .merge(workers::create_workers_routes())
        .merge(rewards::create_rewards_routes())
        .merge(vm::create_vm_routes())
        .merge(raid::create_raid_routes())
        .merge(raid_admin::create_raid_admin_routes())
        .merge(libraries::create_libraries_routes())
        .merge(users::create_users_routes())
        .merge(discovery::create_discovery_routes())
        .merge(grid::create_grid_routes())
        .merge(jobs::create_jobs_routes())
        .merge(memory::create_memory_routes())
        .merge(virtual_nodes::create_virtual_node_routes())
        .merge(instances::create_instance_routes())
        .merge(completions::create_completions_routes())
        .merge(topology::create_topology_routes())
        .merge(ui::create_ui_routes())
}

// Legacy routes function removed - all handlers migrated to modules ✅
