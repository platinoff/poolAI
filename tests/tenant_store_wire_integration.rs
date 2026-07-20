//! PH-S1161: Tenant store-wire API / contract coverage (band 52).
//! Marker: tenant_store_wire_integration

#[cfg(feature = "enterprise")]
use poolai::enterprise::multi_tenancy::{
    tenant_store_wire, tenant_store_wire_label, TenantConfig, TenantManager, TenantStoreWire,
    POOLAI_TENANT_DATA_DIR, POOLAI_TENANT_STORE, TENANT_STORE_SQLITE_FILE,
};
#[cfg(feature = "enterprise")]
use poolai_ui_core::tenant_depth::{tenant_criteria_total, tenant_depth_stub, TenantDepth};
#[cfg(feature = "enterprise")]
use serde_json::json;

#[cfg(feature = "enterprise")]
#[test]
fn tenant_store_wire_contract_ph_s1161() {
    std::env::remove_var(POOLAI_TENANT_STORE);
    std::env::remove_var(POOLAI_TENANT_DATA_DIR);

    let memory = tenant_store_wire();
    assert_eq!(
        memory,
        TenantStoreWire {
            mode: "memory".into(),
            durable_path: None,
            configured: false,
        }
    );
    assert_eq!(tenant_store_wire_label(&memory), "memory");

    std::env::set_var(POOLAI_TENANT_STORE, "sqlite");
    let unconfigured = tenant_store_wire();
    assert_eq!(unconfigured.mode, "sqlite");
    assert!(!unconfigured.configured);
    assert_eq!(
        tenant_store_wire_label(&unconfigured),
        "sqlite_unconfigured"
    );

    std::env::set_var(POOLAI_TENANT_DATA_DIR, "data/dev/tenants");
    let configured = tenant_store_wire();
    assert!(configured.configured);
    assert_eq!(tenant_store_wire_label(&configured), "sqlite");
    let path = configured.durable_path.as_deref().expect("path");
    assert!(path.ends_with(TENANT_STORE_SQLITE_FILE) || path.contains(TENANT_STORE_SQLITE_FILE));

    std::env::remove_var(POOLAI_TENANT_STORE);
    std::env::remove_var(POOLAI_TENANT_DATA_DIR);

    assert_eq!(
        tenant_depth_stub(Some(&json!({"store_wire": true}))),
        TenantDepth::StoreWire
    );
    assert_eq!(tenant_criteria_total(), 7);
}

#[cfg(feature = "enterprise")]
#[tokio::test]
async fn tenant_lifecycle_unaffected_by_store_wire_ph_s1161() {
    std::env::remove_var(POOLAI_TENANT_STORE);
    std::env::remove_var(POOLAI_TENANT_DATA_DIR);

    let manager = TenantManager::new();
    manager.initialize().await.expect("init");
    let tenant = manager
        .create_tenant("wire-contract-tenant".to_string(), TenantConfig::default())
        .await
        .expect("create");
    let listed = manager.list_tenants().await.expect("list");
    assert!(listed.iter().any(|t| t.id == tenant.id));
    assert_eq!(tenant_store_wire().mode, "memory");
}
