//! PH-S1561: Monitoring store-wire API / contract coverage (band 92).
//! Marker: monitoring_store_wire_integration

#[cfg(feature = "enterprise")]
use poolai::enterprise::monitoring::{
    monitoring_store_wire, monitoring_store_wire_label, MonitoringStoreWire, MONITORING_DB_FILE,
    POOLAI_MONITORING_DATA_DIR, POOLAI_MONITORING_STORE,
};
#[cfg(feature = "enterprise")]
use poolai_ui_core::monitoring_store_depth::{
    monitoring_store_criteria_total, monitoring_store_depth_stub, MonitoringStoreDepth,
};
#[cfg(feature = "enterprise")]
use serde_json::json;

#[cfg(feature = "enterprise")]
#[test]
fn monitoring_store_wire_contract_ph_s1561() {
    std::env::remove_var(POOLAI_MONITORING_STORE);
    std::env::remove_var(POOLAI_MONITORING_DATA_DIR);

    let memory = monitoring_store_wire();
    assert_eq!(
        memory,
        MonitoringStoreWire {
            mode: "memory".into(),
            durable_path: None,
            configured: false,
        }
    );
    assert_eq!(monitoring_store_wire_label(&memory), "memory");

    std::env::set_var(POOLAI_MONITORING_STORE, "sqlite");
    let unconfigured = monitoring_store_wire();
    assert_eq!(unconfigured.mode, "sqlite");
    assert!(!unconfigured.configured);
    assert_eq!(
        monitoring_store_wire_label(&unconfigured),
        "sqlite_unconfigured"
    );

    std::env::set_var(POOLAI_MONITORING_DATA_DIR, "data/dev/monitoring");
    let configured = monitoring_store_wire();
    assert!(configured.configured);
    assert_eq!(monitoring_store_wire_label(&configured), "sqlite");
    let path = configured.durable_path.as_deref().expect("path");
    assert!(path.ends_with(MONITORING_DB_FILE) || path.contains(MONITORING_DB_FILE));

    std::env::remove_var(POOLAI_MONITORING_STORE);
    std::env::remove_var(POOLAI_MONITORING_DATA_DIR);

    assert_eq!(
        monitoring_store_depth_stub(Some(&json!({"store_wire": true}))),
        MonitoringStoreDepth::StoreWire
    );
    assert_eq!(monitoring_store_criteria_total(), 7);
}
