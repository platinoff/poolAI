//! PH-S1282: SSO admin/ops HTML glue contracts (band 64).
//! Module: `sso_admin_ops_integration`.

use poolai_ui_core::sso_admin_ops_depth::{
    sso_admin_ops_criteria_total, sso_admin_ops_depth_stub, SsoAdminOpsDepth, SSO_ADMIN_OPS_CASES,
    SSO_ADMIN_OPS_CRITERIA,
};
use serde_json::json;

#[test]
fn sso_admin_ops_depth_registry_ph_s1279() {
    assert_eq!(SSO_ADMIN_OPS_CRITERIA.len(), 10);
    assert_eq!(sso_admin_ops_criteria_total(), 10);
    assert!(SSO_ADMIN_OPS_CASES.contains(&"store_strip"));
    assert!(SSO_ADMIN_OPS_CASES.contains(&"providers_glue"));
    assert_eq!(
        sso_admin_ops_depth_stub(Some(&json!({"store_strip": true}))),
        SsoAdminOpsDepth::StoreStrip
    );
}

#[tokio::test]
async fn sso_admin_ops_html_markers_ph_s1282() {
    let src = include_str!("../src/ui/admin/security.rs");
    assert!(src.contains("sso-store-badge"));
    assert!(src.contains("loadSsoStoreWire"));
    assert!(src.contains("/api/enterprise/security/sso/store"));
    assert!(src.contains("refreshOAuth2Providers"));
    assert!(src.contains("refreshSamlProviders"));
    assert!(src.contains("admin.sso.btn.refreshOauth"));
    assert!(src.contains("admin.sso.btn.refreshSaml"));
}

#[test]
fn sso_admin_ops_i18n_keys_ph_s1283() {
    let en = include_str!("../crates/poolai-ui-core/src/i18n.rs");
    assert!(en.contains("ADMIN_SSO_EN"));
    assert!(en.contains("ADMIN_SSO_UK"));
    assert!(en.contains("admin.sso.storeLabel"));
    assert!(en.contains("admin.sso.btn.refreshOauth"));
    assert!(en.contains("admin.sso.btn.refreshSaml"));
}
