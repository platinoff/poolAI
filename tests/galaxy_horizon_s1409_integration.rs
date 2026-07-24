//! PH-S1418: Galaxy horizon close band 77 — Audit docs canon.
//! Suite: `galaxy_horizon_s1409_integration`.

use poolai_ui_core::audit_docs_canon_depth::{
    audit_docs_canon_criteria_total, audit_docs_canon_depth_stub, audit_docs_canon_slices_met,
    AuditDocsCanonDepth, AUDIT_DOCS_CANON_BAND77_ROWS, AUDIT_DOCS_CANON_CASES,
    AUDIT_DOCS_CANON_CRITERIA, AUDIT_DOCS_CANON_SLICES, FM_BAND77_ROWS,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1409_band_audit_docs_canon_close_ph_s1418() {
    assert_eq!(
        audit_docs_canon_depth_stub(Some(&json!({"audit_docs_canon_depth": true}))),
        AuditDocsCanonDepth::DepthModule
    );
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

    assert_eq!(AUDIT_DOCS_CANON_CRITERIA.len(), 10);
    assert_eq!(audit_docs_canon_criteria_total(), 10);
    assert!(AUDIT_DOCS_CANON_CASES.contains(&"doc_loc_audit"));
    assert_eq!(AUDIT_DOCS_CANON_SLICES.len(), 6);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("audit_docs_canon_mode"));
    assert!(loc_audit.contains("audit_docs_canon_criteria_met_count"));
    assert!(loc_audit.contains("--audit-docs-canon"));

    let audit_doc = include_str!("../docs/development/AUDIT_DOCS_CANON.md");
    assert_eq!(audit_docs_canon_slices_met(audit_doc), (6, 6));
    assert!(audit_doc.contains("--audit-docs-canon"));
    assert!(audit_doc.contains("AUDIT_DOCS_CANON_SLICES") || audit_doc.contains("AUDIT_DEPTH.md"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND77_ROWS {
        assert!(fm.contains(row), "FM missing band-77 row {row}");
    }
    assert!(fm.contains("PH-S1418"));
    assert!(fm.contains("5.58"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1409") || handoff.contains("band 77"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 78"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--audit-docs-canon"));
    assert!(run_local.contains("VERIFY_AUDIT_DOCS_CANON"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("audit_docs_canon_depth") || strategy.contains("band 77"));

    let roadmap = include_str!("../docs/development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md");
    assert!(roadmap.contains("PH-S1409") || roadmap.contains("docs canon"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_AUDIT_DOCS_CANON"));
    assert!(verify.contains("--audit-docs-canon"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--audit-docs-canon"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("audit_docs_canon_band77_export_shape"));

    for marker in AUDIT_DOCS_CANON_BAND77_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || audit_doc.contains(marker),
            "band-77 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/audit_docs_canon_depth.rs").exists());
    assert!(Path::new("docs/development/AUDIT_DOCS_CANON.md").exists());
    assert!(Path::new("tests/audit_docs_canon_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("audit_docs_canon_mode").is_some());
}
