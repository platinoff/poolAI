//! PH-S1541: Policies horizon-close contracts (band 90).
//! Marker: policy_horizon_integration
//!
//! Verifies phase-D Policies slices + horizon criteria totals.

use poolai_ui_core::policy_horizon_depth::{
    policy_horizon_criteria_total, policy_horizon_depth_stub, policy_horizon_slices_met,
    PolicyHorizonDepth, POLICY_HORIZON_CASES, POLICY_HORIZON_CRITERIA, POLICY_HORIZON_SLICES,
};
use poolai_ui_core::policy_ratio_advisory_depth::policy_ratio_advisory_criteria_total;
use serde_json::json;

#[test]
fn policy_horizon_depth_registry_ph_s1539() {
    assert_eq!(POLICY_HORIZON_CRITERIA.len(), 10);
    assert_eq!(policy_horizon_criteria_total(), 10);
    assert!(POLICY_HORIZON_CASES.contains(&"aggregate_flag"));
    assert!(POLICY_HORIZON_CASES.contains(&"phase_d_slices"));
    assert_eq!(
        policy_horizon_depth_stub(Some(&json!({"slice_aggregate": true}))),
        PolicyHorizonDepth::SliceAggregate
    );
}

#[test]
fn policy_horizon_slice_docs_present_ph_s1540() {
    let canon = include_str!("../docs/development/POLICIES_HORIZON.md");
    let (met, total) = policy_horizon_slices_met(canon);
    assert_eq!(total, 10);
    assert_eq!(met, 10, "all horizon slices must be listed");
    for name in POLICY_HORIZON_SLICES {
        assert!(canon.contains(name), "missing horizon slice {name}");
    }

    assert!(std::path::Path::new("docs/development/POLICIES_RATIO_ADVISORY.md").exists());
    assert!(std::path::Path::new("docs/development/POLICIES_LOC_AUDIT.md").exists());
    assert!(std::path::Path::new("docs/development/POLICIES_STORE.md").exists());

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--policy-horizon"));
    assert!(loc_audit.contains("--policy-ratio-advisory"));
}

#[test]
fn policy_horizon_criteria_totals_consistent_ph_s1541() {
    assert_eq!(policy_ratio_advisory_criteria_total(), 10);
    assert_eq!(policy_horizon_criteria_total(), 10);
    assert_eq!(POLICY_HORIZON_SLICES.len(), 10);

    assert_eq!(
        policy_horizon_depth_stub(Some(&json!({
            "policy_horizon_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "policy_horizon_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        PolicyHorizonDepth::FullBand90
    );
}
