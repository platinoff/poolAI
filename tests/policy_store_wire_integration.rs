//! PH-S1461: Policies store-wire API / contract coverage (band 82).
//! Marker: policy_store_wire_integration

#[cfg(feature = "enterprise")]
use poolai::enterprise::security::{
    policy_store_wire, policy_store_wire_label, PolicyStoreWire, POLICY_STORE_SQLITE_FILE,
    POOLAI_POLICY_DATA_DIR, POOLAI_POLICY_STORE,
};
#[cfg(feature = "enterprise")]
use poolai_ui_core::policy_store_depth::{
    policy_store_criteria_total, policy_store_depth_stub, PolicyStoreDepth,
};
#[cfg(feature = "enterprise")]
use serde_json::json;

#[cfg(feature = "enterprise")]
#[test]
fn policy_store_wire_contract_ph_s1461() {
    std::env::remove_var(POOLAI_POLICY_STORE);
    std::env::remove_var(POOLAI_POLICY_DATA_DIR);

    let memory = policy_store_wire();
    assert_eq!(
        memory,
        PolicyStoreWire {
            mode: "memory".into(),
            durable_path: None,
            configured: false,
        }
    );
    assert_eq!(policy_store_wire_label(&memory), "memory");

    std::env::set_var(POOLAI_POLICY_STORE, "sqlite");
    let unconfigured = policy_store_wire();
    assert_eq!(unconfigured.mode, "sqlite");
    assert!(!unconfigured.configured);
    assert_eq!(
        policy_store_wire_label(&unconfigured),
        "sqlite_unconfigured"
    );

    std::env::set_var(POOLAI_POLICY_DATA_DIR, "data/dev/policy");
    let configured = policy_store_wire();
    assert!(configured.configured);
    assert_eq!(policy_store_wire_label(&configured), "sqlite");
    let path = configured.durable_path.as_deref().expect("path");
    assert!(path.ends_with(POLICY_STORE_SQLITE_FILE) || path.contains(POLICY_STORE_SQLITE_FILE));

    std::env::remove_var(POOLAI_POLICY_STORE);
    std::env::remove_var(POOLAI_POLICY_DATA_DIR);

    assert_eq!(
        policy_store_depth_stub(Some(&json!({"store_wire": true}))),
        PolicyStoreDepth::StoreWire
    );
    assert_eq!(policy_store_criteria_total(), 7);
}
