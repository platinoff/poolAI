//! PH-S1912: GPULimits migration 3 HTML glue contracts (band 127).
//! Marker: `gpu_limits_migration3_contracts` · Module: `gpu_limits_migration3_integration`.

use poolai_ui_core::gpu_limits_migration3_depth::{
    gpu_limits_migration3_criteria_total, gpu_limits_migration_depth3_stub, GpuLimitsMigrationDepth3,
    GPU_LIMITS_MIGRATION3_CASES, GPU_LIMITS_MIGRATION3_CRITERIA,
};
use serde_json::json;

#[test]
fn gpu_limits_migration3_depth_registry_ph_s1909() {
    assert_eq!(GPU_LIMITS_MIGRATION3_CRITERIA.len(), 10);
    assert_eq!(gpu_limits_migration3_criteria_total(), 10);
    assert!(GPU_LIMITS_MIGRATION3_CASES.contains(&"store_strip"));
    assert!(GPU_LIMITS_MIGRATION3_CASES.contains(&"query_ops_glue"));
    assert_eq!(
        gpu_limits_migration_depth3_stub(Some(&json!({"store_strip": true}))),
        GpuLimitsMigrationDepth3::StoreStrip
    );
}

#[tokio::test]
async fn gpu_limits_migration3_html_markers_ph_s1912() {
    let src = include_str!("../src/ui/admin/dashboard.rs");
    assert!(src.contains("gpu-limits-store-badge"));
    assert!(src.contains("loadGpuLimitsStoreWire"));
    assert!(src.contains("/api/v1/gpu-limits"));
    assert!(src.contains("refreshGpuLimits"));
    assert!(src.contains("admin.gpuLimits.migrationLabel"));
    assert!(src.contains("admin.gpuLimits.btn.refresh"));
}

#[test]
fn gpu_limits_migration3_i18n_keys_ph_s1912() {
    let i18n = include_str!("../crates/poolai-ui-core/src/i18n.rs");
    assert!(i18n.contains("ADMIN_DASHBOARD_EN"));
    assert!(i18n.contains("ADMIN_DASHBOARD_UK"));
    assert!(i18n.contains("admin.gpuLimits.migrationLabel"));
    assert!(i18n.contains("admin.gpuLimits.migrationRefreshOk"));
    assert!(i18n.contains("admin.gpuLimits.migrationRefreshErr"));
}

#[test]
fn gpu_limits_migration3_wire_surface_ph_s1912() {
    let system = include_str!("../src/network/api/system.rs");
    assert!(system.contains("/gpu-limits"));
    assert!(system.contains("gpu_limits_store_wire_json"));
}