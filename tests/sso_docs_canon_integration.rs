//! PH-S1311: SSO docs-canon contracts (band 67).
//! Marker: sso_docs_canon_integration
//!
//! Verifies band 61–66 SSO_*.md slices are present and criteria totals are consistent.

use poolai_ui_core::sso_docs_canon_depth::{
    sso_docs_canon_criteria_total, sso_docs_canon_depth_stub, sso_docs_canon_slices_met,
    SsoDocsCanonDepth, SSO_DOCS_CANON_CASES, SSO_DOCS_CANON_CRITERIA, SSO_DOCS_CANON_SLICES,
};
use poolai_ui_core::sso_loc_audit_depth::sso_loc_audit_criteria_total;
use serde_json::json;

#[test]
fn sso_docs_canon_depth_registry_ph_s1309() {
    assert_eq!(SSO_DOCS_CANON_CRITERIA.len(), 10);
    assert_eq!(sso_docs_canon_criteria_total(), 10);
    assert!(SSO_DOCS_CANON_CASES.contains(&"aggregate_flag"));
    assert!(SSO_DOCS_CANON_CASES.contains(&"doc_depth"));
    assert_eq!(
        sso_docs_canon_depth_stub(Some(&json!({"slice_aggregate": true}))),
        SsoDocsCanonDepth::SliceAggregate
    );
}

#[test]
fn sso_docs_canon_slice_docs_present_ph_s1310() {
    let canon = include_str!("../docs/development/SSO_DOCS_CANON.md");
    let (met, total) = sso_docs_canon_slices_met(canon);
    assert_eq!(total, 6);
    assert_eq!(met, 6, "all band 61–66 SSO_*.md slices must be listed");
    for name in SSO_DOCS_CANON_SLICES {
        assert!(canon.contains(name), "missing SSO canon doc {name}");
        let path = format!("docs/development/{name}");
        assert!(
            std::path::Path::new(&path).exists(),
            "missing file docs/development/{name}"
        );
    }
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--sso-docs-canon"));
}

#[test]
fn sso_docs_canon_criteria_totals_consistent_ph_s1311() {
    assert_eq!(sso_loc_audit_criteria_total(), 10);
    assert_eq!(sso_docs_canon_criteria_total(), 10);
    assert_eq!(SSO_DOCS_CANON_SLICES.len(), 6);

    assert_eq!(
        sso_docs_canon_depth_stub(Some(&json!({
            "sso_docs_canon_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "sso_docs_canon_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        SsoDocsCanonDepth::FullBand67
    );
}
