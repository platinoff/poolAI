//! PH-S1151: Tenant persistence gate audit — criteria registry + maintenance markers.

use poolai_ui_core::tenant_persistence_depth::{
    tenant_persist_criteria_total, tenant_persistence_depth_stub, TenantPersistenceDepth,
    FM_BAND51_ROWS, TENANT_PERSIST_BAND51_ROWS, TENANT_PERSIST_CASES, TENANT_PERSIST_CRITERIA,
    TENANT_STORE_ENV,
};
use serde_json::json;

#[test]
fn tenant_persistence_audit_ph_s1151() {
    assert_eq!(
        tenant_persistence_depth_stub(Some(&json!({"loc_audit_flag": true}))),
        TenantPersistenceDepth::LocAuditFlag
    );
    assert_eq!(
        tenant_persistence_depth_stub(Some(&json!({
            "tenant_persistence_depth": true,
            "loc_audit_flag": true,
            "audit_test": true,
            "verify_dev_stand_hook": true,
            "quick_flag": true,
            "stand_smoke_export": true,
            "tenant_persist_docs": true,
        }))),
        TenantPersistenceDepth::FullBand51
    );

    assert_eq!(TENANT_PERSIST_CRITERIA.len(), 7);
    assert_eq!(tenant_persist_criteria_total(), 7);
    assert!(TENANT_PERSIST_CASES.contains(&"multi_tenancy_store_hint"));
    assert_eq!(TENANT_STORE_ENV, "POOLAI_TENANT_STORE");

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND51_ROWS {
        assert!(
            fm.contains(row) || row.starts_with("PH-S"),
            "FM missing {row}"
        );
    }
    for marker in TENANT_PERSIST_BAND51_ROWS {
        assert!(
            fm.contains(marker) || marker.starts_with("PH-S"),
            "band-51 marker missing: {marker}"
        );
    }

    let criteria_ids: Vec<_> = TENANT_PERSIST_CRITERIA
        .iter()
        .map(|(id, _, _)| *id)
        .collect();
    assert!(criteria_ids.contains(&"tenant_persistence_depth"));
    assert!(criteria_ids.contains(&"verify_dev_stand_hook"));
}
