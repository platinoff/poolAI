//! PH-S1611: Monitoring docs-canon contracts (band 97).
//!
//! Marker: monitoring_docs_canon_integration
//!
//! Verifies band 91–95 MONITORING_*.md slices are present and criteria totals are consistent.

use poolai_ui_core::monitoring_docs_canon_depth::{
    monitoring_docs_canon_criteria_total, monitoring_docs_canon_depth_stub,
    monitoring_docs_canon_slices_met, MonitoringDocsCanonDepth, MONITORING_DOCS_CANON_CASES,
    MONITORING_DOCS_CANON_CRITERIA, MONITORING_DOCS_CANON_SLICES,
};
use poolai_ui_core::monitoring_loc_audit_depth::monitoring_loc_audit_criteria_total;
use serde_json::json;

#[test]
fn monitoring_docs_canon_depth_registry_ph_s1609() {
    assert_eq!(MONITORING_DOCS_CANON_CRITERIA.len(), 10);
    assert_eq!(monitoring_docs_canon_criteria_total(), 10);
    assert!(MONITORING_DOCS_CANON_CASES.contains(&"aggregate_flag"));
    assert!(MONITORING_DOCS_CANON_CASES.contains(&"doc_depth"));
    assert_eq!(
        monitoring_docs_canon_depth_stub(Some(&json!({"slice_aggregate": true}))),
        MonitoringDocsCanonDepth::SliceAggregate
    );
}

#[test]
fn monitoring_docs_canon_slice_docs_present_ph_s1610() {
    let canon = include_str!("../docs/development/MONITORING_DOCS_CANON.md");
    let (met, total) = monitoring_docs_canon_slices_met(canon);
    assert_eq!(total, 6);
    assert_eq!(
        met, 6,
        "all band 91–95 MONITORING_*.md slices must be listed"
    );

    for name in MONITORING_DOCS_CANON_SLICES {
        assert!(canon.contains(name), "missing Monitoring canon doc {name}");
        let path = format!("docs/development/{name}");
        assert!(
            std::path::Path::new(&path).exists(),
            "missing file docs/development/{name}"
        );
    }

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--monitoring-docs-canon"));
}

#[test]
fn monitoring_docs_canon_criteria_totals_consistent_ph_s1611() {
    assert_eq!(monitoring_loc_audit_criteria_total(), 10);
    assert_eq!(monitoring_docs_canon_criteria_total(), 10);
    assert_eq!(MONITORING_DOCS_CANON_SLICES.len(), 6);

    assert_eq!(
        monitoring_docs_canon_depth_stub(Some(&json!({
            "monitoring_docs_canon_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "monitoring_docs_canon_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        MonitoringDocsCanonDepth::FullBand97
    );
}
