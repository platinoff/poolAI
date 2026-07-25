//! PH-S1482: Policies admin/ops HTML glue contracts (band 84).
//! Module: `policy_admin_ops_integration`.

use poolai_ui_core::policy_admin_ops_depth::{
    policy_admin_ops_criteria_total, policy_admin_ops_depth_stub, PolicyAdminOpsDepth,
    POLICY_ADMIN_OPS_CASES, POLICY_ADMIN_OPS_CRITERIA,
};
use serde_json::json;

#[test]
fn policy_admin_ops_depth_registry_ph_s1479() {
    assert_eq!(POLICY_ADMIN_OPS_CRITERIA.len(), 10);
    assert_eq!(policy_admin_ops_criteria_total(), 10);
    assert!(POLICY_ADMIN_OPS_CASES.contains(&"store_strip"));
    assert!(POLICY_ADMIN_OPS_CASES.contains(&"query_ops_glue"));
    assert_eq!(
        policy_admin_ops_depth_stub(Some(&json!({"store_strip": true}))),
        PolicyAdminOpsDepth::StoreStrip
    );
}

#[tokio::test]
async fn policy_admin_ops_html_markers_ph_s1482() {
    let src = include_str!("../src/ui/admin/security.rs");
    assert!(src.contains("policy-store-badge"));
    assert!(src.contains("loadPolicyStoreWire"));
    assert!(src.contains("/api/enterprise/policy/store"));
    assert!(src.contains("refreshSecurityPolicies"));
    assert!(src.contains("admin.policy.storeLabel"));
    assert!(src.contains("admin.policy.btn.refresh"));
}

#[test]
fn policy_admin_ops_i18n_keys_ph_s1483() {
    let en = include_str!("../crates/poolai-ui-core/src/i18n.rs");
    assert!(en.contains("ADMIN_POLICY_EN"));
    assert!(en.contains("ADMIN_POLICY_UK"));
    assert!(en.contains("admin.policy.storeLabel"));
    assert!(en.contains("admin.policy.btn.refresh"));
    assert!(en.contains("admin.policy.refreshOk"));
}
