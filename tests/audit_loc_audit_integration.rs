//! PH-S1401: Audit loc-audit aggregate contracts (band 76).
//! Marker: audit_loc_audit_integration
//!
//! Verifies band 71–75 slice flags are present and aggregate criteria totals are consistent.

use poolai_ui_core::audit_admin_ops_depth::audit_admin_ops_criteria_total;
use poolai_ui_core::audit_api_contracts_depth::audit_api_criteria_total;
use poolai_ui_core::audit_depth::audit_criteria_total;
use poolai_ui_core::audit_loc_audit_depth::{
    audit_loc_audit_criteria_total, audit_loc_audit_depth_stub, audit_loc_audit_slices_met,
    AuditLocAuditDepth, AUDIT_LOC_AUDIT_CASES, AUDIT_LOC_AUDIT_CRITERIA, AUDIT_LOC_AUDIT_SLICES,
};
use poolai_ui_core::audit_stand_smoke_depth::audit_stand_smoke_criteria_total;
use poolai_ui_core::audit_store_depth::audit_store_criteria_total;
use serde_json::json;

#[test]
fn audit_loc_audit_depth_registry_ph_s1399() {
    assert_eq!(AUDIT_LOC_AUDIT_CRITERIA.len(), 10);
    assert_eq!(audit_loc_audit_criteria_total(), 10);
    assert!(AUDIT_LOC_AUDIT_CASES.contains(&"aggregate_flag"));
    assert!(AUDIT_LOC_AUDIT_CASES.contains(&"slice_audit"));
    assert_eq!(
        audit_loc_audit_depth_stub(Some(&json!({"slice_aggregate": true}))),
        AuditLocAuditDepth::SliceAggregate
    );
}

#[test]
fn audit_loc_audit_slice_flags_present_ph_s1400() {
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    let (met, total) = audit_loc_audit_slices_met(loc_audit);
    assert_eq!(total, 5);
    assert_eq!(
        met, 5,
        "all band 71–75 --audit* loc-audit slices must exist"
    );
    for flag in AUDIT_LOC_AUDIT_SLICES {
        assert!(
            loc_audit.contains(flag),
            "missing loc-audit slice flag {flag}"
        );
    }
    assert!(loc_audit.contains("--audit-loc-audit"));
}

#[test]
fn audit_loc_audit_criteria_totals_consistent_ph_s1401() {
    // Band 71–72 use smaller registries; 73–75 use 9–10; aggregate band 76 uses 10.
    assert_eq!(audit_criteria_total(), 8);
    assert_eq!(audit_store_criteria_total(), 7);
    assert_eq!(audit_api_criteria_total(), 9);
    assert_eq!(audit_admin_ops_criteria_total(), 10);
    assert_eq!(audit_stand_smoke_criteria_total(), 10);
    assert_eq!(audit_loc_audit_criteria_total(), 10);

    let slice_total: usize = AUDIT_LOC_AUDIT_SLICES.len();
    assert_eq!(slice_total, 5);

    assert_eq!(
        audit_loc_audit_depth_stub(Some(&json!({
            "audit_loc_audit_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "audit_loc_audit_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        AuditLocAuditDepth::FullBand76
    );
}
