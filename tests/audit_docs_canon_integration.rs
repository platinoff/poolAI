//! PH-S1411: Audit docs-canon contracts (band 77).
//! Marker: audit_docs_canon_integration
//!
//! Verifies band 71–76 AUDIT_*.md slices are present and criteria totals are consistent.

use poolai_ui_core::audit_docs_canon_depth::{
    audit_docs_canon_criteria_total, audit_docs_canon_depth_stub, audit_docs_canon_slices_met,
    AuditDocsCanonDepth, AUDIT_DOCS_CANON_CASES, AUDIT_DOCS_CANON_CRITERIA,
    AUDIT_DOCS_CANON_SLICES,
};
use poolai_ui_core::audit_loc_audit_depth::audit_loc_audit_criteria_total;
use serde_json::json;

#[test]
fn audit_docs_canon_depth_registry_ph_s1409() {
    assert_eq!(AUDIT_DOCS_CANON_CRITERIA.len(), 10);
    assert_eq!(audit_docs_canon_criteria_total(), 10);
    assert!(AUDIT_DOCS_CANON_CASES.contains(&"aggregate_flag"));
    assert!(AUDIT_DOCS_CANON_CASES.contains(&"doc_depth"));
    assert_eq!(
        audit_docs_canon_depth_stub(Some(&json!({"slice_aggregate": true}))),
        AuditDocsCanonDepth::SliceAggregate
    );
}

#[test]
fn audit_docs_canon_slice_docs_present_ph_s1410() {
    let canon = include_str!("../docs/development/AUDIT_DOCS_CANON.md");
    let (met, total) = audit_docs_canon_slices_met(canon);
    assert_eq!(total, 6);
    assert_eq!(met, 6, "all band 71–76 AUDIT_*.md slices must be listed");
    for name in AUDIT_DOCS_CANON_SLICES {
        assert!(canon.contains(name), "missing Audit canon doc {name}");
        let path = format!("docs/development/{name}");
        assert!(
            std::path::Path::new(&path).exists(),
            "missing file docs/development/{name}"
        );
    }
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--audit-docs-canon"));
}

#[test]
fn audit_docs_canon_criteria_totals_consistent_ph_s1411() {
    assert_eq!(audit_loc_audit_criteria_total(), 10);
    assert_eq!(audit_docs_canon_criteria_total(), 10);
    assert_eq!(AUDIT_DOCS_CANON_SLICES.len(), 6);

    assert_eq!(
        audit_docs_canon_depth_stub(Some(&json!({
            "audit_docs_canon_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "audit_docs_canon_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        AuditDocsCanonDepth::FullBand77
    );
}
