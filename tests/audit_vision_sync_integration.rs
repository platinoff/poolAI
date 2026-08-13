//! PH-S1421: Audit vision-sync contracts (band 78).
//! Marker: audit_vision_sync_integration
//!
//! Verifies vision slices + prior AUDIT_DOCS_CANON are present and criteria totals are consistent.

use poolai_ui_core::audit_docs_canon_depth::audit_docs_canon_criteria_total;
use poolai_ui_core::audit_vision_sync_depth::{
    audit_vision_sync_criteria_total, audit_vision_sync_depth_stub, audit_vision_sync_slices_met,
    AuditVisionSyncDepth, AUDIT_VISION_SYNC_CASES, AUDIT_VISION_SYNC_CRITERIA,
    AUDIT_VISION_SYNC_SLICES,
};
use serde_json::json;

#[test]
fn audit_vision_sync_depth_registry_ph_s1419() {
    assert_eq!(AUDIT_VISION_SYNC_CRITERIA.len(), 10);
    assert_eq!(audit_vision_sync_criteria_total(), 10);
    assert!(AUDIT_VISION_SYNC_CASES.contains(&"aggregate_flag"));
    assert!(AUDIT_VISION_SYNC_CASES.contains(&"vision_manifest"));
    assert_eq!(
        audit_vision_sync_depth_stub(Some(&json!({"slice_aggregate": true}))),
        AuditVisionSyncDepth::SliceAggregate
    );
}

#[test]
fn audit_vision_sync_slice_docs_present_ph_s1420() {
    let canon = include_str!("../docs/development/AUDIT_VISION_SYNC.md");
    let (met, total) = audit_vision_sync_slices_met(canon);
    assert_eq!(total, 6);
    assert_eq!(met, 6, "all vision-sync slices must be listed");
    for name in AUDIT_VISION_SYNC_SLICES {
        assert!(canon.contains(name), "missing vision-sync slice {name}");
    }
    assert!(std::path::Path::new("GSV/docs/vision/manifest.json").exists());
    assert!(std::path::Path::new("GSV/docs/vision/extensions.json").exists());
    assert!(std::path::Path::new("GSV/docs/vision/README.md").exists());
    assert!(std::path::Path::new("GSV/docs/vision/vision.svg").exists());
    assert!(std::path::Path::new("GSV/docs/vision/index.html").exists());
    assert!(std::path::Path::new("docs/development/AUDIT_DOCS_CANON.md").exists());
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--audit-vision-sync"));
}

#[test]
fn audit_vision_sync_criteria_totals_consistent_ph_s1421() {
    assert_eq!(audit_docs_canon_criteria_total(), 10);
    assert_eq!(audit_vision_sync_criteria_total(), 10);
    assert_eq!(AUDIT_VISION_SYNC_SLICES.len(), 6);

    assert_eq!(
        audit_vision_sync_depth_stub(Some(&json!({
            "audit_vision_sync_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "audit_vision_sync_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        AuditVisionSyncDepth::FullBand78
    );
}
