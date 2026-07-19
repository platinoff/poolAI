//! PH-S1141: CI canon gate audit — criteria registry + maintenance markers.

use poolai_ui_core::ci_canon_depth::{
    ci_canon_criteria_total, ci_canon_depth_stub, CiCanonDepth, CI_CANON_BAND50_ROWS,
    CI_CANON_CASES, CI_CANON_CRITERIA, FM_BAND50_ROWS,
};
use serde_json::json;

#[test]
fn ci_canon_audit_ph_s1141() {
    assert_eq!(
        ci_canon_depth_stub(Some(&json!({"openapi_gap_audit": true}))),
        CiCanonDepth::OpenapiGapAudit
    );
    assert_eq!(
        ci_canon_depth_stub(Some(&json!({
            "test_ci_scope": true,
            "openapi_gap_audit": true,
            "rust_ratio_audit": true,
            "openapi_gap_ci_job": true,
            "verify_dev_stand_hook": true,
            "ci_canon_docs": true,
            "dual_gate": true,
        }))),
        CiCanonDepth::FullBand50
    );

    assert_eq!(CI_CANON_CRITERIA.len(), 7);
    assert_eq!(ci_canon_criteria_total(), 7);
    assert!(CI_CANON_CASES.contains(&"dual_gate"));
    assert!(CI_CANON_CASES.contains(&"verify_dev_stand_hook"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND50_ROWS {
        assert!(
            fm.contains(row) || row.starts_with("PH-S"),
            "FM missing {row}"
        );
    }
    for marker in CI_CANON_BAND50_ROWS {
        assert!(
            fm.contains(marker) || marker.starts_with("PH-S"),
            "band-50 marker missing: {marker}"
        );
    }

    let criteria_ids: Vec<_> = CI_CANON_CRITERIA.iter().map(|(id, _, _)| *id).collect();
    assert!(criteria_ids.contains(&"test_ci_scope"));
    assert!(criteria_ids.contains(&"openapi_gap_audit"));
}
