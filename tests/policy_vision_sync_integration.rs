//! PH-S1521: Policies vision-sync contracts (band 88).
//! Marker: policy_vision_sync_integration
//!
//! Verifies vision slices + prior POLICIES_DOCS_CANON are present and criteria totals are consistent.

use poolai_ui_core::policy_docs_canon_depth::policy_docs_canon_criteria_total;
use poolai_ui_core::policy_vision_sync_depth::{
    policy_vision_sync_criteria_total, policy_vision_sync_depth_stub,
    policy_vision_sync_slices_met, PolicyVisionSyncDepth, POLICY_VISION_SYNC_CASES,
    POLICY_VISION_SYNC_CRITERIA, POLICY_VISION_SYNC_SLICES,
};
use serde_json::json;

#[test]
fn policy_vision_sync_depth_registry_ph_s1519() {
    assert_eq!(POLICY_VISION_SYNC_CRITERIA.len(), 10);
    assert_eq!(policy_vision_sync_criteria_total(), 10);
    assert!(POLICY_VISION_SYNC_CASES.contains(&"aggregate_flag"));
    assert!(POLICY_VISION_SYNC_CASES.contains(&"vision_manifest"));
    assert_eq!(
        policy_vision_sync_depth_stub(Some(&json!({"slice_aggregate": true}))),
        PolicyVisionSyncDepth::SliceAggregate
    );
}

#[test]
fn policy_vision_sync_slice_docs_present_ph_s1520() {
    let canon = include_str!("../docs/development/POLICIES_VISION_SYNC.md");
    let (met, total) = policy_vision_sync_slices_met(canon);
    assert_eq!(total, 6);
    assert_eq!(met, 6, "all vision-sync slices must be listed");
    for name in POLICY_VISION_SYNC_SLICES {
        assert!(canon.contains(name), "missing vision-sync slice {name}");
    }
    assert!(std::path::Path::new("GSV/docs/vision/manifest.json").exists());
    assert!(std::path::Path::new("GSV/docs/vision/extensions.json").exists());
    assert!(std::path::Path::new("GSV/docs/vision/README.md").exists());
    assert!(std::path::Path::new("GSV/docs/vision/vision.svg").exists());
    assert!(std::path::Path::new("GSV/docs/vision/index.html").exists());
    assert!(std::path::Path::new("docs/development/POLICIES_DOCS_CANON.md").exists());
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--policy-vision-sync"));
}

#[test]
fn policy_vision_sync_criteria_totals_consistent_ph_s1521() {
    assert_eq!(policy_docs_canon_criteria_total(), 10);
    assert_eq!(policy_vision_sync_criteria_total(), 10);
    assert_eq!(POLICY_VISION_SYNC_SLICES.len(), 6);

    assert_eq!(
        policy_vision_sync_depth_stub(Some(&json!({
            "policy_vision_sync_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "policy_vision_sync_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        PolicyVisionSyncDepth::FullBand88
    );
}
