//! PH-S1431: Audit ratio-advisory contracts (band 79).
//! Marker: audit_ratio_advisory_integration
//!
//! Verifies prior Audit + vision-sync slices are present and criteria totals are consistent.

use poolai_ui_core::audit_ratio_advisory_depth::{
    audit_ratio_advisory_criteria_total, audit_ratio_advisory_depth_stub,
    audit_ratio_advisory_slices_met, AuditRatioAdvisoryDepth, AUDIT_RATIO_ADVISORY_CASES,
    AUDIT_RATIO_ADVISORY_CRITERIA, AUDIT_RATIO_ADVISORY_SLICES,
};
use poolai_ui_core::audit_vision_sync_depth::audit_vision_sync_criteria_total;
use serde_json::json;

#[test]
fn audit_ratio_advisory_depth_registry_ph_s1429() {
    assert_eq!(AUDIT_RATIO_ADVISORY_CRITERIA.len(), 10);
    assert_eq!(audit_ratio_advisory_criteria_total(), 10);
    assert!(AUDIT_RATIO_ADVISORY_CASES.contains(&"aggregate_flag"));
    assert!(AUDIT_RATIO_ADVISORY_CASES.contains(&"doc_vision_sync"));
    assert_eq!(
        audit_ratio_advisory_depth_stub(Some(&json!({"slice_aggregate": true}))),
        AuditRatioAdvisoryDepth::SliceAggregate
    );
}

#[test]
fn audit_ratio_advisory_slice_docs_present_ph_s1430() {
    let canon = include_str!("../docs/development/AUDIT_RATIO_ADVISORY.md");
    let (met, total) = audit_ratio_advisory_slices_met(canon);
    assert_eq!(total, 6);
    assert_eq!(met, 6, "all ratio-advisory slices must be listed");
    for name in AUDIT_RATIO_ADVISORY_SLICES {
        assert!(canon.contains(name), "missing ratio-advisory slice {name}");
    }
    assert!(std::path::Path::new("docs/development/AUDIT_VISION_SYNC.md").exists());
    assert!(std::path::Path::new("docs/development/AUDIT_DOCS_CANON.md").exists());
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--audit-ratio-advisory"));
    assert!(loc_audit.contains("--audit-vision-sync"));
}

#[test]
fn audit_ratio_advisory_criteria_totals_consistent_ph_s1431() {
    assert_eq!(audit_vision_sync_criteria_total(), 10);
    assert_eq!(audit_ratio_advisory_criteria_total(), 10);
    assert_eq!(AUDIT_RATIO_ADVISORY_SLICES.len(), 6);

    assert_eq!(
        audit_ratio_advisory_depth_stub(Some(&json!({
            "audit_ratio_advisory_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "audit_ratio_advisory_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        AuditRatioAdvisoryDepth::FullBand79
    );
}
