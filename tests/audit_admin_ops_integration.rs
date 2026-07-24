//! PH-S1382: Audit admin/ops HTML glue contracts (band 74).
//! Module: `audit_admin_ops_integration`.

use poolai_ui_core::audit_admin_ops_depth::{
    audit_admin_ops_criteria_total, audit_admin_ops_depth_stub, AuditAdminOpsDepth,
    AUDIT_ADMIN_OPS_CASES, AUDIT_ADMIN_OPS_CRITERIA,
};
use serde_json::json;

#[test]
fn audit_admin_ops_depth_registry_ph_s1379() {
    assert_eq!(AUDIT_ADMIN_OPS_CRITERIA.len(), 10);
    assert_eq!(audit_admin_ops_criteria_total(), 10);
    assert!(AUDIT_ADMIN_OPS_CASES.contains(&"store_strip"));
    assert!(AUDIT_ADMIN_OPS_CASES.contains(&"query_ops_glue"));
    assert_eq!(
        audit_admin_ops_depth_stub(Some(&json!({"store_strip": true}))),
        AuditAdminOpsDepth::StoreStrip
    );
}

#[tokio::test]
async fn audit_admin_ops_html_markers_ph_s1382() {
    let src = include_str!("../src/ui/admin/audit.rs");
    assert!(src.contains("audit-store-badge"));
    assert!(src.contains("loadAuditStoreWire"));
    assert!(src.contains("/api/enterprise/audit/store"));
    assert!(src.contains("refreshAuditEvents"));
    assert!(src.contains("admin.audit.storeLabel"));
    assert!(src.contains("admin.audit.btn.refresh"));
}

#[test]
fn audit_admin_ops_i18n_keys_ph_s1383() {
    let en = include_str!("../crates/poolai-ui-core/src/i18n.rs");
    assert!(en.contains("ADMIN_AUDIT_EN"));
    assert!(en.contains("ADMIN_AUDIT_UK"));
    assert!(en.contains("admin.audit.storeLabel"));
    assert!(en.contains("admin.audit.btn.refresh"));
    assert!(en.contains("admin.audit.refreshOk"));
}
