//! PH-S1531: Policies ratio-advisory contracts (band 89).
//! Marker: policy_ratio_advisory_integration
//!
//! Verifies prior Policies + vision-sync slices are present and criteria totals are consistent.

use poolai_ui_core::policy_ratio_advisory_depth::{
    policy_ratio_advisory_criteria_total, policy_ratio_advisory_depth_stub,
    policy_ratio_advisory_slices_met, PolicyRatioAdvisoryDepth, POLICY_RATIO_ADVISORY_CASES,
    POLICY_RATIO_ADVISORY_CRITERIA, POLICY_RATIO_ADVISORY_SLICES,
};
use poolai_ui_core::policy_vision_sync_depth::policy_vision_sync_criteria_total;
use serde_json::json;

#[test]
fn policy_ratio_advisory_depth_registry_ph_s1529() {
    assert_eq!(POLICY_RATIO_ADVISORY_CRITERIA.len(), 10);
    assert_eq!(policy_ratio_advisory_criteria_total(), 10);
    assert!(POLICY_RATIO_ADVISORY_CASES.contains(&"aggregate_flag"));
    assert!(POLICY_RATIO_ADVISORY_CASES.contains(&"doc_vision_sync"));
    assert_eq!(
        policy_ratio_advisory_depth_stub(Some(&json!({"slice_aggregate": true}))),
        PolicyRatioAdvisoryDepth::SliceAggregate
    );
}

#[test]
fn policy_ratio_advisory_slice_docs_present_ph_s1530() {
    let canon = include_str!("../docs/development/POLICIES_RATIO_ADVISORY.md");
    let (met, total) = policy_ratio_advisory_slices_met(canon);
    assert_eq!(total, 6);
    assert_eq!(met, 6, "all ratio-advisory slices must be listed");
    for name in POLICY_RATIO_ADVISORY_SLICES {
        assert!(canon.contains(name), "missing ratio-advisory slice {name}");
    }
    assert!(std::path::Path::new("docs/development/POLICIES_VISION_SYNC.md").exists());
    assert!(std::path::Path::new("docs/development/POLICIES_DOCS_CANON.md").exists());
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--policy-ratio-advisory"));
    assert!(loc_audit.contains("--policy-vision-sync"));
}

#[test]
fn policy_ratio_advisory_criteria_totals_consistent_ph_s1531() {
    assert_eq!(policy_vision_sync_criteria_total(), 10);
    assert_eq!(policy_ratio_advisory_criteria_total(), 10);
    assert_eq!(POLICY_RATIO_ADVISORY_SLICES.len(), 6);

    assert_eq!(
        policy_ratio_advisory_depth_stub(Some(&json!({
            "policy_ratio_advisory_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "policy_ratio_advisory_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        PolicyRatioAdvisoryDepth::FullBand89
    );
}
