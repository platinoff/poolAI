//! PH-S1211: Tenant docs-canon contracts (band 57).
//! Marker: tenant_docs_canon_integration
//!
//! Verifies band 51–56 TENANT_*.md slices are present and criteria totals are consistent.

use poolai_ui_core::tenant_docs_canon_depth::{
    tenant_docs_canon_criteria_total, tenant_docs_canon_depth_stub, tenant_docs_canon_slices_met,
    TenantDocsCanonDepth, TENANT_DOCS_CANON_CASES, TENANT_DOCS_CANON_CRITERIA,
    TENANT_DOCS_CANON_SLICES,
};
use poolai_ui_core::tenant_loc_audit_depth::tenant_loc_audit_criteria_total;
use serde_json::json;

#[test]
fn tenant_docs_canon_depth_registry_ph_s1209() {
    assert_eq!(TENANT_DOCS_CANON_CRITERIA.len(), 10);
    assert_eq!(tenant_docs_canon_criteria_total(), 10);
    assert!(TENANT_DOCS_CANON_CASES.contains(&"aggregate_flag"));
    assert!(TENANT_DOCS_CANON_CASES.contains(&"doc_persist"));
    assert_eq!(
        tenant_docs_canon_depth_stub(Some(&json!({"slice_aggregate": true}))),
        TenantDocsCanonDepth::SliceAggregate
    );
}

#[test]
fn tenant_docs_canon_slice_docs_present_ph_s1210() {
    let canon = include_str!("../docs/development/TENANT_DOCS_CANON.md");
    let (met, total) = tenant_docs_canon_slices_met(canon);
    assert_eq!(total, 6);
    assert_eq!(met, 6, "all band 51–56 TENANT_*.md slices must be listed");
    for name in TENANT_DOCS_CANON_SLICES {
        assert!(canon.contains(name), "missing tenant canon doc {name}");
        let path = format!("docs/development/{name}");
        assert!(
            std::path::Path::new(&path).exists(),
            "missing file docs/development/{name}"
        );
    }
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--tenant-docs-canon"));
}

#[test]
fn tenant_docs_canon_criteria_totals_consistent_ph_s1211() {
    assert_eq!(tenant_loc_audit_criteria_total(), 10);
    assert_eq!(tenant_docs_canon_criteria_total(), 10);
    assert_eq!(TENANT_DOCS_CANON_SLICES.len(), 6);

    assert_eq!(
        tenant_docs_canon_depth_stub(Some(&json!({
            "tenant_docs_canon_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "tenant_docs_canon_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        TenantDocsCanonDepth::FullBand57
    );
}
