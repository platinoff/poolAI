//! PH-S1902: GPULimits migration 2 HTML glue contracts (band 126).
//! Marker: `gpu_limits_migration2_contracts` · Module: `gpu_limits_migration2_integration`.

use poolai_ui_core::gpu_limits_migration2_depth::{
    gpu_limits_migration2_criteria_total, gpu_limits_migration_depth2_stub, GpuLimitsMigrationDepth2,
    GPU_LIMITS_MIGRATION2_CASES, GPU_LIMITS_MIGRATION2_CRITERIA,
};
use serde_json::json;

#[test]
fn gpu_limits_migration2_depth_registry_ph_s1899() {
    assert_eq!(GPU_LIMITS_MIGRATION2_CRITERIA.len(), 10);
    assert_eq!(gpu_limits_migration2_criteria_total(), 10);
    assert!(GPU_LIMITS_MIGRATION2_CASES.contains(&"store_strip"));
    assert!(GPU_LIMITS_MIGRATION2_CASES.contains(&"query_ops_glue"));
    assert_eq!(
        gpu_limits_migration_depth2_stub(Some(&json!({"store_strip": true}))),
        GpuLimitsMigrationDepth2::StoreStrip
    );
}

#[tokio::test]
async fn gpu_limits_migration2_html_markers_ph_s1902() {
    let src = include_str!("../src/ui/admin/dashboard.rs");
    assert!(src.contains("gpu-limits-store-badge"));
    assert!(src.contains("loadGpuLimitsStoreWire"));
    assert!(src.contains("/api/v1/gpu-limits"));
    assert!(src.contains("refreshGpuLimits"));
    assert!(src.contains("admin.gpuLimits.migrationLabel"));
    assert!(src.contains("admin.gpuLimits.btn.refresh"));
}

#[test]
fn gpu_limits_migration2_i18n_keys_ph_s1902() {
    let i18n = include_str!("../crates/poolai-ui-core/src/i18n.rs");
    assert!(i18n.contains("ADMIN_DASHBOARD_EN"));
    assert!(i18n.contains("ADMIN_DASHBOARD_UK"));
    assert!(i18n.contains("admin.gpuLimits.migrationLabel"));
    assert!(i18n.contains("admin.gpuLimits.migrationRefreshOk"));
    assert!(i18n.contains("admin.gpuLimits.migrationRefreshErr"));
}

#[test]
fn gpu_limits_migration2_wire_surface_ph_s1902() {
    let system = include_str!("../src/network/api/system.rs");
    assert!(system.contains("/gpu-limits"));
    assert!(system.contains("gpu_limits_store_wire_json"));
}