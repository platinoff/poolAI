//! PH-S2002: GPULimits UI debugging 6 HTML glue contracts (band 136).
//! Marker: `debug-limits-store-badge` · Module: `gpu_limits_debug6_integration`.

use poolai_ui_core::gpu_limits_debug6_depth::{
    gpu_limits_debug6_criteria_total, gpu_limits_debug_depth6_stub, GpuLimitsDebugDepth6,
    GPU_LIMITS_DEBUG6_CASES, GPU_LIMITS_DEBUG6_CRITERIA,
};
use serde_json::json;

#[test]
fn gpu_limits_debug6_depth_registry_ph_s1999() {
    assert_eq!(GPU_LIMITS_DEBUG6_CRITERIA.len(), 10);
    assert_eq!(gpu_limits_debug6_criteria_total(), 10);
    assert!(GPU_LIMITS_DEBUG6_CASES.contains(&"store_strip"));
    assert!(GPU_LIMITS_DEBUG6_CASES.contains(&"query_ops_glue"));
    assert_eq!(
        gpu_limits_debug_depth6_stub(Some(&json!({"store_strip": true}))),
        GpuLimitsDebugDepth6::StoreStrip
    );
}

#[tokio::test]
async fn gpu_limits_debug6_html_markers_ph_s2002() {
    let src = include_str!("../src/ui/admin/dashboard.rs");
    assert!(src.contains("debug-limits-store-badge"));
    assert!(src.contains("loadDebugLimitsStoreWire"));
    assert!(src.contains("/api/v1/debug/ui"));
    assert!(src.contains("refreshDebugLimits"));
    assert!(src.contains("admin.debug.migrationLabel"));
    assert!(src.contains("admin.debug.btn.refresh"));
}

#[test]
fn gpu_limits_debug6_i18n_keys_ph_s2002() {
    let i18n = include_str!("../crates/poolai-ui-core/src/i18n.rs");
    assert!(i18n.contains("ADMIN_DASHBOARD_EN"));
    assert!(i18n.contains("ADMIN_DASHBOARD_UK"));
    assert!(i18n.contains("admin.debug.migrationLabel"));
    assert!(i18n.contains("admin.debug.migrationRefreshOk"));
    assert!(i18n.contains("admin.debug.migrationRefreshErr"));
}

#[test]
fn gpu_limits_debug6_wire_surface_ph_s2002() {
    let system = include_str!("../src/network/api/system.rs");
    assert!(system.contains("/debug/ui"));
    assert!(system.contains("gpu_debug_store_wire_json"));
}