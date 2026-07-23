//! PH-S1301: SSO loc-audit aggregate contracts (band 66).
//! Marker: sso_loc_audit_integration
//!
//! Verifies band 61–65 slice flags are present and aggregate criteria totals are consistent.

use poolai_ui_core::sso_admin_ops_depth::sso_admin_ops_criteria_total;
use poolai_ui_core::sso_api_contracts_depth::sso_api_criteria_total;
use poolai_ui_core::sso_depth::sso_criteria_total;
use poolai_ui_core::sso_loc_audit_depth::{
    sso_loc_audit_criteria_total, sso_loc_audit_depth_stub, sso_loc_audit_slices_met,
    SsoLocAuditDepth, SSO_LOC_AUDIT_CASES, SSO_LOC_AUDIT_CRITERIA, SSO_LOC_AUDIT_SLICES,
};
use poolai_ui_core::sso_stand_smoke_depth::sso_stand_smoke_criteria_total;
use poolai_ui_core::sso_store_depth::sso_store_criteria_total;
use serde_json::json;

#[test]
fn sso_loc_audit_depth_registry_ph_s1299() {
    assert_eq!(SSO_LOC_AUDIT_CRITERIA.len(), 10);
    assert_eq!(sso_loc_audit_criteria_total(), 10);
    assert!(SSO_LOC_AUDIT_CASES.contains(&"aggregate_flag"));
    assert!(SSO_LOC_AUDIT_CASES.contains(&"slice_sso"));
    assert_eq!(
        sso_loc_audit_depth_stub(Some(&json!({"slice_aggregate": true}))),
        SsoLocAuditDepth::SliceAggregate
    );
}

#[test]
fn sso_loc_audit_slice_flags_present_ph_s1300() {
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    let (met, total) = sso_loc_audit_slices_met(loc_audit);
    assert_eq!(total, 5);
    assert_eq!(met, 5, "all band 61–65 --sso* loc-audit slices must exist");
    for flag in SSO_LOC_AUDIT_SLICES {
        assert!(
            loc_audit.contains(flag),
            "missing loc-audit slice flag {flag}"
        );
    }
    assert!(loc_audit.contains("--sso-loc-audit"));
}

#[test]
fn sso_loc_audit_criteria_totals_consistent_ph_s1301() {
    // Band 61–62 use smaller registries; 63–65 use 10; aggregate band 66 uses 10.
    assert_eq!(sso_criteria_total(), 8);
    assert_eq!(sso_store_criteria_total(), 7);
    assert_eq!(sso_api_criteria_total(), 10);
    assert_eq!(sso_admin_ops_criteria_total(), 10);
    assert_eq!(sso_stand_smoke_criteria_total(), 10);
    assert_eq!(sso_loc_audit_criteria_total(), 10);

    let slice_total: usize = SSO_LOC_AUDIT_SLICES.len();
    assert_eq!(slice_total, 5);

    assert_eq!(
        sso_loc_audit_depth_stub(Some(&json!({
            "sso_loc_audit_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "sso_loc_audit_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        SsoLocAuditDepth::FullBand66
    );
}
