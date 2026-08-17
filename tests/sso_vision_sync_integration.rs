//! PH-S1321: SSO vision-sync contracts (band 68).
//! Marker: sso_vision_sync_integration
//!
//! Verifies vision slices + prior SSO_DOCS_CANON are present and criteria totals are consistent.

use poolai_ui_core::sso_docs_canon_depth::sso_docs_canon_criteria_total;
use poolai_ui_core::sso_vision_sync_depth::{
    sso_vision_sync_criteria_total, sso_vision_sync_depth_stub, sso_vision_sync_slices_met,
    SsoVisionSyncDepth, SSO_VISION_SYNC_CASES, SSO_VISION_SYNC_CRITERIA, SSO_VISION_SYNC_SLICES,
};
use serde_json::json;

#[test]
fn sso_vision_sync_depth_registry_ph_s1319() {
    assert_eq!(SSO_VISION_SYNC_CRITERIA.len(), 10);
    assert_eq!(sso_vision_sync_criteria_total(), 10);
    assert!(SSO_VISION_SYNC_CASES.contains(&"aggregate_flag"));
    assert!(SSO_VISION_SYNC_CASES.contains(&"vision_manifest"));
    assert_eq!(
        sso_vision_sync_depth_stub(Some(&json!({"slice_aggregate": true}))),
        SsoVisionSyncDepth::SliceAggregate
    );
}

#[test]
fn sso_vision_sync_slice_docs_present_ph_s1320() {
    let canon = include_str!("../docs/development/SSO_VISION_SYNC.md");
    let (met, total) = sso_vision_sync_slices_met(canon);
    assert_eq!(total, 6);
    assert_eq!(met, 6, "all vision-sync slices must be listed");
    for name in SSO_VISION_SYNC_SLICES {
        assert!(canon.contains(name), "missing vision-sync slice {name}");
    }
    assert!(std::path::Path::new("docs/vision/manifest.json").exists());
    assert!(std::path::Path::new("docs/vision/extensions.json").exists());
    assert!(std::path::Path::new("docs/vision/README.md").exists());
    assert!(std::path::Path::new("docs/vision/vision.svg").exists());
    assert!(std::path::Path::new("docs/vision/index.html").exists());
    assert!(std::path::Path::new("docs/development/SSO_DOCS_CANON.md").exists());
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--sso-vision-sync"));
}

#[test]
fn sso_vision_sync_criteria_totals_consistent_ph_s1321() {
    assert_eq!(sso_docs_canon_criteria_total(), 10);
    assert_eq!(sso_vision_sync_criteria_total(), 10);
    assert_eq!(SSO_VISION_SYNC_SLICES.len(), 6);

    assert_eq!(
        sso_vision_sync_depth_stub(Some(&json!({
            "sso_vision_sync_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "sso_vision_sync_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        SsoVisionSyncDepth::FullBand68
    );
}
