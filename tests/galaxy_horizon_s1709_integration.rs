//! PH-S1718: Galaxy horizon close band 107 — Ratio96 docs canon.
//! Suite: `galaxy_horizon_s1709_integration`.

use poolai_ui_core::ratio96_docs_canon_depth::{
    ratio96_docs_canon_criteria_total, ratio96_docs_canon_depth_stub, Ratio96DocsCanonDepth,
    FM_BAND107_ROWS, RATIO96_DOCS_CANON_BAND107_ROWS, RATIO96_DOCS_CANON_CASES,
    RATIO96_DOCS_CANON_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1709_band_ratio96_docs_canon_close_ph_s1718() {
    assert_eq!(
        ratio96_docs_canon_depth_stub(Some(&json!({"ratio96_docs_canon_depth": true}))),
        Ratio96DocsCanonDepth::DepthModule
    );
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

    assert_eq!(RATIO96_DOCS_CANON_CRITERIA.len(), 10);
    assert_eq!(ratio96_docs_canon_criteria_total(), 10);
    assert!(RATIO96_DOCS_CANON_CASES.contains(&"aggregate_flag"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND107_ROWS {
        assert!(fm.contains(row), "FM missing band-107 row {row}");
    }
    assert!(fm.contains("PH-S1718"));
    assert!(fm.contains("5.88"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1709") || handoff.contains("band 107"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 107"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--ratio96-docs-canon"));
    assert!(run_local.contains("VERIFY_RATIO96_DOCS_CANON"));

    let canon_doc = include_str!("../docs/development/RATIO96_DOCS_CANON.md");
    assert!(canon_doc.contains("RATIO96_DOCS_CANON_SLICES"));
    assert!(canon_doc.contains("ratio96_docs_canon_band107_export_shape"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_RATIO96_DOCS_CANON"));
    assert!(verify.contains("--ratio96-docs-canon"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--ratio96-docs-canon"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("ratio96_docs_canon_mode"));
    assert!(loc_audit.contains("ratio96_docs_canon_criteria_met_count"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("ratio96_docs_canon_band107_export_shape"));

    for marker in RATIO96_DOCS_CANON_BAND107_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || canon_doc.contains(marker),
            "band-107 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/ratio96_docs_canon_depth.rs").exists());
    assert!(Path::new("docs/development/RATIO96_DOCS_CANON.md").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("ratio96_docs_canon_mode").is_some());
    assert!(ratio.get("ratio96_docs_canon_criteria_total").is_some());
}
