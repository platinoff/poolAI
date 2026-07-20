//! PH-S1182: Tenant admin/ops HTML glue contracts (band 54).
//! Module: `tenant_admin_ops_integration`.

use poolai_ui_core::tenant_admin_ops_depth::{
    tenant_admin_ops_criteria_total, tenant_admin_ops_depth_stub, TenantAdminOpsDepth,
    TENANT_ADMIN_OPS_CASES, TENANT_ADMIN_OPS_CRITERIA,
};
use serde_json::json;

#[test]
fn tenant_admin_ops_depth_registry_ph_s1179() {
    assert_eq!(TENANT_ADMIN_OPS_CRITERIA.len(), 10);
    assert_eq!(tenant_admin_ops_criteria_total(), 10);
    assert!(TENANT_ADMIN_OPS_CASES.contains(&"store_strip"));
    assert!(TENANT_ADMIN_OPS_CASES.contains(&"usage_quota_glue"));
    assert_eq!(
        tenant_admin_ops_depth_stub(Some(&json!({"store_strip": true}))),
        TenantAdminOpsDepth::StoreStrip
    );
}

#[tokio::test]
async fn tenant_admin_ops_html_markers_ph_s1182() {
    // Render via source include — avoids enterprise feature coupling in test binary.
    let src = include_str!("../src/ui/admin/tenants.rs");
    assert!(src.contains("tenant-store-badge"));
    assert!(src.contains("loadTenantStoreWire"));
    assert!(src.contains("/api/enterprise/tenants/store"));
    assert!(src.contains("refreshTenantUsage"));
    assert!(src.contains("probeTenantQuota"));
    assert!(src.contains("/usage"));
    assert!(src.contains("/quota"));
    assert!(src.contains("admin.tenants.btn.usage"));
    assert!(src.contains("admin.tenants.btn.quota"));
}

#[test]
fn tenant_admin_ops_i18n_keys_ph_s1181() {
    let en = include_str!("../crates/poolai-ui-core/src/i18n.rs");
    assert!(en.contains("admin.tenants.storeLabel"));
    assert!(en.contains("admin.tenants.btn.usage"));
    assert!(en.contains("admin.tenants.btn.quota"));
    assert!(en.contains("admin.tenants.quotaAllow"));
}
