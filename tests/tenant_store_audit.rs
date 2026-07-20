//! Tenant store-wire audit (PH-S1161 companion): criteria + FM markers.

use poolai_ui_core::tenant_depth::{
    tenant_criteria_total, tenant_depth_stub, TenantDepth, FM_BAND52_ROWS, TENANT_BAND52_ROWS,
    TENANT_CASES, TENANT_CRITERIA, TENANT_DATA_DIR_ENV, TENANT_STORE_ENV,
};
use serde_json::json;

#[test]
fn tenant_store_wire_audit_ph_s1161() {
    assert_eq!(
        tenant_depth_stub(Some(&json!({"loc_audit_flag": true}))),
        TenantDepth::LocAuditFlag
    );
    assert_eq!(
        tenant_depth_stub(Some(&json!({
            "tenant_depth": true,
            "store_wire": true,
            "api_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "tenant_store_docs": true,
        }))),
        TenantDepth::FullBand52
    );
    assert_eq!(TENANT_CRITERIA.len(), 7);
    assert_eq!(tenant_criteria_total(), 7);
    assert_eq!(TENANT_STORE_ENV, "POOLAI_TENANT_STORE");
    assert_eq!(TENANT_DATA_DIR_ENV, "POOLAI_TENANT_DATA_DIR");
    assert!(TENANT_CASES.contains(&"store_wire"));

    let criteria_ids: Vec<&str> = TENANT_CRITERIA.iter().map(|(id, _, _)| *id).collect();
    assert!(criteria_ids.contains(&"tenant_depth"));
    assert!(criteria_ids.contains(&"store_wire"));
    assert!(criteria_ids.contains(&"api_contracts"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND52_ROWS {
        assert!(fm.contains(row), "FM missing band-52 row {row}");
    }
    for marker in TENANT_BAND52_ROWS {
        assert!(
            fm.contains(marker)
                || marker.starts_with("--")
                || marker.starts_with("VERIFY_")
                || *marker == "tenant_store_wire"
                || *marker == "tenant_depth",
            "band-52 marker missing from FM (ok if ops-only): {marker}"
        );
    }
}
