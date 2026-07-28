//! PH-S1582: Monitoring admin/ops HTML glue contracts (band 94).
//! Module: `monitoring_admin_ops_integration`.

use poolai_ui_core::monitoring_admin_ops_depth::{
    monitoring_admin_ops_criteria_total, monitoring_admin_ops_depth_stub, MonitoringAdminOpsDepth,
    MONITORING_ADMIN_OPS_CASES, MONITORING_ADMIN_OPS_CRITERIA,
};
use serde_json::json;

#[test]
fn monitoring_admin_ops_depth_registry_ph_s1579() {
    assert_eq!(MONITORING_ADMIN_OPS_CRITERIA.len(), 10);
    assert_eq!(monitoring_admin_ops_criteria_total(), 10);
    assert!(MONITORING_ADMIN_OPS_CASES.contains(&"store_strip"));
    assert!(MONITORING_ADMIN_OPS_CASES.contains(&"query_ops_glue"));
    assert_eq!(
        monitoring_admin_ops_depth_stub(Some(&json!({"store_strip": true}))),
        MonitoringAdminOpsDepth::StoreStrip
    );
}

#[tokio::test]
async fn monitoring_admin_ops_html_markers_ph_s1582() {
    let src = include_str!("../src/ui/admin/monitoring.rs");
    assert!(src.contains("monitoring-store-badge"));
    assert!(src.contains("loadMonitoringStoreWire"));
    assert!(src.contains("/api/enterprise/monitoring/store"));
    assert!(src.contains("refreshMonitoring"));
    assert!(src.contains("admin.mon.storeLabel"));
    assert!(src.contains("admin.mon.btn.refresh"));
}

#[test]
fn monitoring_admin_ops_i18n_keys_ph_s1583() {
    let en = include_str!("../crates/poolai-ui-core/src/i18n.rs");
    assert!(en.contains("ADMIN_MONITORING_EN"));
    assert!(en.contains("ADMIN_MONITORING_UK"));
    assert!(en.contains("admin.mon.storeLabel"));
    assert!(en.contains("admin.mon.btn.refresh"));
    assert!(en.contains("admin.mon.refreshOk"));
}
