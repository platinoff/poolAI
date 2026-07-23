//! PH-S1361: Audit store-wire API / contract coverage (band 72).
//! Marker: audit_store_wire_integration

#[cfg(feature = "enterprise")]
use poolai::enterprise::audit::{
    audit_store_wire, audit_store_wire_label, AuditStoreWire, AUDIT_STORE_SQLITE_FILE,
    POOLAI_AUDIT_DATA_DIR, POOLAI_AUDIT_STORE,
};
#[cfg(feature = "enterprise")]
use poolai_ui_core::audit_store_depth::{
    audit_store_criteria_total, audit_store_depth_stub, AuditStoreDepth,
};
#[cfg(feature = "enterprise")]
use serde_json::json;

#[cfg(feature = "enterprise")]
#[test]
fn audit_store_wire_contract_ph_s1361() {
    std::env::remove_var(POOLAI_AUDIT_STORE);
    std::env::remove_var(POOLAI_AUDIT_DATA_DIR);

    let file = audit_store_wire();
    assert_eq!(
        file,
        AuditStoreWire {
            mode: "file".into(),
            durable_path: None,
            configured: false,
        }
    );
    assert_eq!(audit_store_wire_label(&file), "file");

    std::env::set_var(POOLAI_AUDIT_STORE, "sqlite");
    let unconfigured = audit_store_wire();
    assert_eq!(unconfigured.mode, "sqlite");
    assert!(!unconfigured.configured);
    assert_eq!(audit_store_wire_label(&unconfigured), "sqlite_unconfigured");

    std::env::set_var(POOLAI_AUDIT_DATA_DIR, "data/dev/audit");
    let configured = audit_store_wire();
    assert!(configured.configured);
    assert_eq!(audit_store_wire_label(&configured), "sqlite");
    let path = configured.durable_path.as_deref().expect("path");
    assert!(path.ends_with(AUDIT_STORE_SQLITE_FILE) || path.contains(AUDIT_STORE_SQLITE_FILE));

    std::env::remove_var(POOLAI_AUDIT_STORE);
    std::env::remove_var(POOLAI_AUDIT_DATA_DIR);

    assert_eq!(
        audit_store_depth_stub(Some(&json!({"store_wire": true}))),
        AuditStoreDepth::StoreWire
    );
    assert_eq!(audit_store_criteria_total(), 7);
}
