//! PH-S1511: Policies docs-canon contracts (band 87).
//! Marker: policy_docs_canon_integration
//!
//! Verifies band 81–86 POLICIES_*.md slices are present and criteria totals are consistent.

use poolai_ui_core::policy_docs_canon_depth::{
    policy_docs_canon_criteria_total, policy_docs_canon_depth_stub, policy_docs_canon_slices_met,
    PolicyDocsCanonDepth, POLICY_DOCS_CANON_CASES, POLICY_DOCS_CANON_CRITERIA,
    POLICY_DOCS_CANON_SLICES,
};
use poolai_ui_core::policy_loc_audit_depth::policy_loc_audit_criteria_total;
use serde_json::json;

#[test]
fn policy_docs_canon_depth_registry_ph_s1509() {
    assert_eq!(POLICY_DOCS_CANON_CRITERIA.len(), 10);
    assert_eq!(policy_docs_canon_criteria_total(), 10);
    assert!(POLICY_DOCS_CANON_CASES.contains(&"aggregate_flag"));
    assert!(POLICY_DOCS_CANON_CASES.contains(&"doc_depth"));
    assert_eq!(
        policy_docs_canon_depth_stub(Some(&json!({"slice_aggregate": true}))),
        PolicyDocsCanonDepth::SliceAggregate
    );
}

#[test]
fn policy_docs_canon_slice_docs_present_ph_s1510() {
    let canon = include_str!("../docs/development/POLICIES_DOCS_CANON.md");
    let (met, total) = policy_docs_canon_slices_met(canon);
    assert_eq!(total, 6);
    assert_eq!(met, 6, "all band 81–86 POLICIES_*.md slices must be listed");
    for name in POLICY_DOCS_CANON_SLICES {
        assert!(canon.contains(name), "missing Policies canon doc {name}");
        let path = format!("docs/development/{name}");
        assert!(
            std::path::Path::new(&path).exists(),
            "missing file docs/development/{name}"
        );
    }
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--policy-docs-canon"));
}

#[test]
fn policy_docs_canon_criteria_totals_consistent_ph_s1511() {
    assert_eq!(policy_loc_audit_criteria_total(), 10);
    assert_eq!(policy_docs_canon_criteria_total(), 10);
    assert_eq!(POLICY_DOCS_CANON_SLICES.len(), 6);

    assert_eq!(
        policy_docs_canon_depth_stub(Some(&json!({
            "policy_docs_canon_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "policy_docs_canon_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        PolicyDocsCanonDepth::FullBand87
    );
}
