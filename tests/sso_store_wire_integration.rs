//! PH-S1261: SSO store-wire API / contract coverage (band 62).
//! Marker: sso_store_wire_integration

#[cfg(feature = "enterprise")]
use poolai::enterprise::security::{
    sso_store_wire, sso_store_wire_label, SecurityManager, SsoStoreWire, POOLAI_SSO_DATA_DIR,
    POOLAI_SSO_STORE, SSO_STORE_SQLITE_FILE,
};
#[cfg(feature = "enterprise")]
use poolai_ui_core::sso_store_depth::{
    sso_store_criteria_total, sso_store_depth_stub, SsoStoreDepth,
};
#[cfg(feature = "enterprise")]
use serde_json::json;

#[cfg(feature = "enterprise")]
#[test]
fn sso_store_wire_contract_ph_s1261() {
    std::env::remove_var(POOLAI_SSO_STORE);
    std::env::remove_var(POOLAI_SSO_DATA_DIR);

    let memory = sso_store_wire();
    assert_eq!(
        memory,
        SsoStoreWire {
            mode: "memory".into(),
            durable_path: None,
            configured: false,
        }
    );
    assert_eq!(sso_store_wire_label(&memory), "memory");

    std::env::set_var(POOLAI_SSO_STORE, "sqlite");
    let unconfigured = sso_store_wire();
    assert_eq!(unconfigured.mode, "sqlite");
    assert!(!unconfigured.configured);
    assert_eq!(sso_store_wire_label(&unconfigured), "sqlite_unconfigured");

    std::env::set_var(POOLAI_SSO_DATA_DIR, "data/dev/sso");
    let configured = sso_store_wire();
    assert!(configured.configured);
    assert_eq!(sso_store_wire_label(&configured), "sqlite");
    let path = configured.durable_path.as_deref().expect("path");
    assert!(path.ends_with(SSO_STORE_SQLITE_FILE) || path.contains(SSO_STORE_SQLITE_FILE));

    std::env::remove_var(POOLAI_SSO_STORE);
    std::env::remove_var(POOLAI_SSO_DATA_DIR);

    assert_eq!(
        sso_store_depth_stub(Some(&json!({"store_wire": true}))),
        SsoStoreDepth::StoreWire
    );
    assert_eq!(sso_store_criteria_total(), 7);
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn sso_lifecycle_unaffected_by_store_wire_ph_s1261() {
    std::env::remove_var(POOLAI_SSO_STORE);
    std::env::remove_var(POOLAI_SSO_DATA_DIR);

    let manager = SecurityManager::new();
    manager.initialize().await.expect("init");
    let listed = manager.list_oauth2_providers().await.expect("list");
    assert!(listed.is_empty());
    assert_eq!(sso_store_wire().mode, "memory");
}
