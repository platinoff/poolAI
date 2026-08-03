//! PH-S1681: Ratio96 admin/ops HTML glue contracts (band 104).
//! Marker: `ratio96_admin_ops_contracts` · Module: `ratio96_admin_ops_integration`.

use poolai_ui_core::ratio96_admin_ops_depth::{
    ratio96_admin_ops_criteria_total, ratio96_admin_ops_depth_stub, Ratio96AdminOpsDepth,
    RATIO96_ADMIN_OPS_CASES, RATIO96_ADMIN_OPS_CRITERIA,
};
use serde_json::json;

#[test]
fn ratio96_admin_ops_depth_registry_ph_s1679() {
    assert_eq!(RATIO96_ADMIN_OPS_CRITERIA.len(), 10);
    assert_eq!(ratio96_admin_ops_criteria_total(), 10);
    assert!(RATIO96_ADMIN_OPS_CASES.contains(&"store_strip"));
    assert!(RATIO96_ADMIN_OPS_CASES.contains(&"query_ops_glue"));
    assert_eq!(
        ratio96_admin_ops_depth_stub(Some(&json!({"store_strip": true}))),
        Ratio96AdminOpsDepth::StoreStrip
    );
}

#[tokio::test]
async fn ratio96_admin_ops_html_markers_ph_s1681() {
    let src = include_str!("../src/ui/admin/dashboard.rs");
    assert!(src.contains("ratio96-store-badge"));
    assert!(src.contains("loadRatio96StoreWire"));
    assert!(src.contains("/api/v1/ops/ratio96"));
    assert!(src.contains("refreshRatio96"));
    assert!(src.contains("admin.ratio96.storeLabel"));
    assert!(src.contains("admin.ratio96.btn.refresh"));
}

#[test]
fn ratio96_admin_ops_i18n_keys_ph_s1681() {
    let i18n = include_str!("../crates/poolai-ui-core/src/i18n.rs");
    assert!(i18n.contains("ADMIN_DASHBOARD_EN"));
    assert!(i18n.contains("ADMIN_DASHBOARD_UK"));
    assert!(i18n.contains("admin.ratio96.storeLabel"));
    assert!(i18n.contains("admin.ratio96.btn.refresh"));
    assert!(i18n.contains("admin.ratio96.refreshOk"));
}
