//! PH-S1441: Audit horizon-close contracts (band 80).
//! Marker: audit_horizon_integration
//!
//! Verifies phase-C Audit slices + horizon criteria totals.

use poolai_ui_core::audit_horizon_depth::{
    audit_horizon_criteria_total, audit_horizon_depth_stub, audit_horizon_slices_met,
    AuditHorizonDepth, AUDIT_HORIZON_CASES, AUDIT_HORIZON_CRITERIA, AUDIT_HORIZON_SLICES,
};
use poolai_ui_core::audit_ratio_advisory_depth::audit_ratio_advisory_criteria_total;
use serde_json::json;

#[test]
fn audit_horizon_depth_registry_ph_s1439() {
    assert_eq!(AUDIT_HORIZON_CRITERIA.len(), 10);
    assert_eq!(audit_horizon_criteria_total(), 10);
    assert!(AUDIT_HORIZON_CASES.contains(&"aggregate_flag"));
    assert!(AUDIT_HORIZON_CASES.contains(&"phase_c_slices"));
    assert_eq!(
        audit_horizon_depth_stub(Some(&json!({"slice_aggregate": true}))),
        AuditHorizonDepth::SliceAggregate
    );
}

#[test]
fn audit_horizon_slice_docs_present_ph_s1440() {
    let canon = include_str!("../docs/development/AUDIT_HORIZON.md");
    let (met, total) = audit_horizon_slices_met(canon);
    assert_eq!(total, 10);
    assert_eq!(met, 10, "all horizon slices must be listed");
    for name in AUDIT_HORIZON_SLICES {
        assert!(canon.contains(name), "missing horizon slice {name}");
    }
    assert!(std::path::Path::new("docs/development/AUDIT_RATIO_ADVISORY.md").exists());
    assert!(std::path::Path::new("docs/development/AUDIT_STORE.md").exists());
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--audit-horizon"));
    assert!(loc_audit.contains("--audit-ratio-advisory"));
}

#[test]
fn audit_horizon_criteria_totals_consistent_ph_s1441() {
    assert_eq!(audit_ratio_advisory_criteria_total(), 10);
    assert_eq!(audit_horizon_criteria_total(), 10);
    assert_eq!(AUDIT_HORIZON_SLICES.len(), 10);

    assert_eq!(
        audit_horizon_depth_stub(Some(&json!({
            "audit_horizon_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "audit_horizon_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        AuditHorizonDepth::FullBand80
    );
}
