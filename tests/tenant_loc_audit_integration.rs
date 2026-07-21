//! PH-S1201: Tenant loc-audit aggregate contracts (band 56).
//! Marker: tenant_loc_audit_integration
//!
//! Verifies band 51–55 slice flags are present and aggregate criteria totals are consistent.

use poolai_ui_core::tenant_admin_ops_depth::tenant_admin_ops_criteria_total;
use poolai_ui_core::tenant_api_contracts_depth::tenant_api_criteria_total;
use poolai_ui_core::tenant_depth::tenant_criteria_total;
use poolai_ui_core::tenant_loc_audit_depth::{
    tenant_loc_audit_criteria_total, tenant_loc_audit_depth_stub, tenant_loc_audit_slices_met,
    TenantLocAuditDepth, TENANT_LOC_AUDIT_CASES, TENANT_LOC_AUDIT_CRITERIA,
    TENANT_LOC_AUDIT_SLICES,
};
use poolai_ui_core::tenant_persistence_depth::tenant_persist_criteria_total;
use poolai_ui_core::tenant_stand_smoke_depth::tenant_stand_smoke_criteria_total;
use serde_json::json;

#[test]
fn tenant_loc_audit_depth_registry_ph_s1199() {
    assert_eq!(TENANT_LOC_AUDIT_CRITERIA.len(), 10);
    assert_eq!(tenant_loc_audit_criteria_total(), 10);
    assert!(TENANT_LOC_AUDIT_CASES.contains(&"aggregate_flag"));
    assert!(TENANT_LOC_AUDIT_CASES.contains(&"slice_persist"));
    assert_eq!(
        tenant_loc_audit_depth_stub(Some(&json!({"slice_aggregate": true}))),
        TenantLocAuditDepth::SliceAggregate
    );
}

#[test]
fn tenant_loc_audit_slice_flags_present_ph_s1200() {
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    let (met, total) = tenant_loc_audit_slices_met(loc_audit);
    assert_eq!(total, 5);
    assert_eq!(
        met, 5,
        "all band 51–55 --tenant-* loc-audit slices must exist"
    );
    for flag in TENANT_LOC_AUDIT_SLICES {
        assert!(
            loc_audit.contains(flag),
            "missing loc-audit slice flag {flag}"
        );
    }
    assert!(loc_audit.contains("--tenant-loc-audit"));
}

#[test]
fn tenant_loc_audit_criteria_totals_consistent_ph_s1201() {
    // Band 51–52 use 7-criteria registries; 53–55 use 10; aggregate band 56 uses 10.
    assert_eq!(tenant_persist_criteria_total(), 7);
    assert_eq!(tenant_criteria_total(), 7);
    assert_eq!(tenant_api_criteria_total(), 10);
    assert_eq!(tenant_admin_ops_criteria_total(), 10);
    assert_eq!(tenant_stand_smoke_criteria_total(), 10);
    assert_eq!(tenant_loc_audit_criteria_total(), 10);

    let slice_total: usize = TENANT_LOC_AUDIT_SLICES.len();
    assert_eq!(slice_total, 5);

    assert_eq!(
        tenant_loc_audit_depth_stub(Some(&json!({
            "tenant_loc_audit_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "tenant_loc_audit_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        TenantLocAuditDepth::FullBand56
    );
}
