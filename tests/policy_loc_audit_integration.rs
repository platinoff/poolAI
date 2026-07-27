//! PH-S1501: Policies loc-audit aggregate contracts (band 86).
//! Marker: policy_loc_audit_integration
//!
//! Verifies band 81–85 slice flags are present and aggregate criteria totals are consistent.

use poolai_ui_core::policy_admin_ops_depth::policy_admin_ops_criteria_total;
use poolai_ui_core::policy_api_contracts_depth::policy_api_criteria_total;
use poolai_ui_core::policy_depth::policy_criteria_total;
use poolai_ui_core::policy_loc_audit_depth::{
    policy_loc_audit_criteria_total, policy_loc_audit_depth_stub, policy_loc_audit_slices_met,
    PolicyLocAuditDepth, POLICY_LOC_AUDIT_CASES, POLICY_LOC_AUDIT_CRITERIA,
    POLICY_LOC_AUDIT_SLICES,
};
use poolai_ui_core::policy_stand_smoke_depth::policy_stand_smoke_criteria_total;
use poolai_ui_core::policy_store_depth::policy_store_criteria_total;
use serde_json::json;

#[test]
fn policy_loc_audit_depth_registry_ph_s1499() {
    assert_eq!(POLICY_LOC_AUDIT_CRITERIA.len(), 10);
    assert_eq!(policy_loc_audit_criteria_total(), 10);
    assert!(POLICY_LOC_AUDIT_CASES.contains(&"aggregate_flag"));
    assert!(POLICY_LOC_AUDIT_CASES.contains(&"slice_policy"));
    assert_eq!(
        policy_loc_audit_depth_stub(Some(&json!({"slice_aggregate": true}))),
        PolicyLocAuditDepth::SliceAggregate
    );
}

#[test]
fn policy_loc_audit_slice_flags_present_ph_s1500() {
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    let (met, total) = policy_loc_audit_slices_met(loc_audit);
    assert_eq!(total, 5);
    assert_eq!(
        met, 5,
        "all band 81–85 --policy* loc-audit slices must exist"
    );
    for flag in POLICY_LOC_AUDIT_SLICES {
        assert!(
            loc_audit.contains(flag),
            "missing loc-audit slice flag {flag}"
        );
    }
    assert!(loc_audit.contains("--policy-loc-audit"));
}

#[test]
fn policy_loc_audit_criteria_totals_consistent_ph_s1501() {
    // Band 81–82 use smaller registries; 83–85 use 9–10; aggregate band 86 uses 10.
    assert_eq!(policy_criteria_total(), 8);
    assert_eq!(policy_store_criteria_total(), 7);
    assert_eq!(policy_api_criteria_total(), 9);
    assert_eq!(policy_admin_ops_criteria_total(), 10);
    assert_eq!(policy_stand_smoke_criteria_total(), 10);
    assert_eq!(policy_loc_audit_criteria_total(), 10);

    let slice_total: usize = POLICY_LOC_AUDIT_SLICES.len();
    assert_eq!(slice_total, 5);

    assert_eq!(
        policy_loc_audit_depth_stub(Some(&json!({
            "policy_loc_audit_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "policy_loc_audit_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        PolicyLocAuditDepth::FullBand86
    );
}
