//! Aggregated snapshots for the admin UI and future admin APIs.

use crate::core::state::{ApiContext, SystemStatus};
use crate::services::library_service::LibraryService;
use crate::services::raid_service::RaidService;
use crate::services::vm_service::VmService;
use crate::version::APP_VERSION;
use serde::Serialize;

/// Which core singletons are attached to [`crate::core::state::AppState`].
#[derive(Debug, Clone, Serialize)]
pub struct AdminSubsystemFlags {
    pub pool: bool,
    pub raid: bool,
    pub vm: bool,
    pub library: bool,
    pub instance_manager: bool,
    pub topology: bool,
    #[cfg(feature = "cloud")]
    pub cloud: bool,
}

/// Single JSON payload for the admin dashboard (system + quick counts + RAID/libs).
#[derive(Debug, Clone, Serialize)]
pub struct AdminOverview {
    /// High-level health label for UI (`healthy`, `degraded`, …).
    pub status: String,
    pub uptime_seconds: u64,
    pub version: String,
    /// Active / total workers from [`crate::core::state::SystemState`].
    pub workers: usize,
    pub workers_total: usize,
    /// Workers registered in [`AppState::workers`].
    pub workers_registered: usize,
    pub vm_instances: usize,
    pub active_models: usize,
    pub cpu_usage_percent: f32,
    pub memory_usage_mb: f32,
    pub gpu_usage_percent: f32,
    pub total_requests: u64,
    pub libraries: usize,
    pub raid_nodes: usize,
    pub raid_artifacts: usize,
    pub subsystems: AdminSubsystemFlags,
}

fn status_label(ready: bool, status: &SystemStatus) -> String {
    if !ready {
        return "initializing".to_string();
    }
    match status {
        SystemStatus::Running => "healthy".to_string(),
        SystemStatus::Degraded => "degraded".to_string(),
        SystemStatus::Initializing => "initializing".to_string(),
        SystemStatus::Error => "error".to_string(),
        SystemStatus::Shutdown => "shutdown".to_string(),
        SystemStatus::Maintenance => "maintenance".to_string(),
    }
}

pub struct AdminService;

impl AdminService {
    /// Build a dashboard-oriented overview (best-effort when managers are not attached).
    pub async fn overview(ctx: &ApiContext) -> AdminOverview {
        let system = ctx.get_system_state();
        let metrics = system.system_metrics.clone();
        let ready = ctx.is_ready();

        let vm_instances = VmService::list_instances(ctx)
            .await
            .map(|v| v.len())
            .unwrap_or(0);
        let libraries = LibraryService::list_libraries(ctx)
            .await
            .map(|v| v.len())
            .unwrap_or(0);
        let raid_nodes = RaidService::list_nodes(ctx)
            .await
            .map(|v| v.len())
            .unwrap_or(0);
        let raid_artifacts = RaidService::list_artifacts(ctx)
            .await
            .map(|v| v.len())
            .unwrap_or(0);

        AdminOverview {
            status: status_label(ready, &system.status),
            uptime_seconds: crate::version::get_uptime_seconds(),
            version: APP_VERSION.to_string(),
            workers: system.active_workers,
            workers_total: system.total_workers,
            workers_registered: ctx.get_all_workers().len(),
            vm_instances,
            active_models: system.active_models,
            cpu_usage_percent: metrics.total_cpu_utilization,
            memory_usage_mb: metrics.total_memory_usage_mb,
            gpu_usage_percent: metrics.total_gpu_utilization,
            total_requests: metrics.total_requests,
            libraries,
            raid_nodes,
            raid_artifacts,
            subsystems: AdminSubsystemFlags {
                pool: ctx.pool.get().is_some(),
                raid: ctx.raid_manager.get().is_some(),
                vm: ctx.vm_manager.get().is_some(),
                library: ctx.library_manager.get().is_some(),
                instance_manager: ctx.instance_manager.get().is_some(),
                topology: ctx.topology_manager.get().is_some(),
                #[cfg(feature = "cloud")]
                cloud: ctx.cloud_manager.get().is_some(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn overview_after_initialize_reports_healthy_and_zero_counts() {
        let ctx = ApiContext::default();
        ctx.initialize().await.expect("init");
        let o = AdminService::overview(&ctx).await;
        assert_eq!(o.status, "healthy");
        assert!(o.uptime_seconds <= 1_000_000);
        assert!(!o.version.is_empty());
        assert_eq!(o.workers, 0);
        assert_eq!(o.vm_instances, 0);
        assert_eq!(o.libraries, 0);
        assert_eq!(o.raid_nodes, 0);
        assert!(!o.subsystems.pool);
    }
}
