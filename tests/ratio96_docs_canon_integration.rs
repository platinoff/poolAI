//! PH-S1711: Ratio96 docs-canon contracts (band 107).
//! Marker: ratio96_docs_canon_integration
//!
//! Verifies band 101–106 RATIO96_*.md slices are present and criteria totals are consistent.

use poolai_ui_core::ratio96_docs_canon_depth::{
    ratio96_docs_canon_criteria_total, ratio96_docs_canon_depth_stub,
    ratio96_docs_canon_slices_met, Ratio96DocsCanonDepth, RATIO96_DOCS_CANON_CASES,
    RATIO96_DOCS_CANON_CRITERIA, RATIO96_DOCS_CANON_SLICES,
};
use poolai_ui_core::ratio96_loc_audit_depth::ratio96_loc_audit_criteria_total;
use serde_json::json;

#[test]
fn ratio96_docs_canon_depth_registry_ph_s1709() {
    assert_eq!(RATIO96_DOCS_CANON_CRITERIA.len(), 10);
    assert_eq!(ratio96_docs_canon_criteria_total(), 10);
    assert!(RATIO96_DOCS_CANON_CASES.contains(&"aggregate_flag"));
    assert!(RATIO96_DOCS_CANON_CASES.contains(&"doc_depth"));
    assert_eq!(
        ratio96_docs_canon_depth_stub(Some(&json!({"slice_aggregate": true}))),
        Ratio96DocsCanonDepth::SliceAggregate
    );
}

#[test]
fn ratio96_docs_canon_slice_docs_present_ph_s1710() {
    let canon = include_str!("../docs/development/RATIO96_DOCS_CANON.md");
    let (met, total) = ratio96_docs_canon_slices_met(canon);
    assert_eq!(total, 4);
    assert_eq!(
        met, 4,
        "all band 101–106 RATIO96_*.md slices must be listed"
    );
    for name in RATIO96_DOCS_CANON_SLICES {
        assert!(canon.contains(name), "missing ratio96 canon doc {name}");
        let path = format!("docs/development/{name}");
        assert!(
            std::path::Path::new(&path).exists(),
            "missing file docs/development/{name}"
        );
    }
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--ratio96-docs-canon"));
}

#[test]
fn ratio96_docs_canon_criteria_totals_consistent_ph_s1711() {
    assert_eq!(ratio96_loc_audit_criteria_total(), 10);
    assert_eq!(ratio96_docs_canon_criteria_total(), 10);
    assert_eq!(RATIO96_DOCS_CANON_SLICES.len(), 4);

    assert_eq!(
        ratio96_docs_canon_depth_stub(Some(&json!({
            "ratio96_docs_canon_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "ratio96_docs_canon_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        Ratio96DocsCanonDepth::FullBand107
    );
}
