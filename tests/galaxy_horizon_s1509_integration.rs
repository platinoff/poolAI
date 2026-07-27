//! PH-S1518: Galaxy horizon close band 87 — Policies docs canon.
//! Suite: `galaxy_horizon_s1509_integration`.

use poolai_ui_core::policy_docs_canon_depth::{
    policy_docs_canon_criteria_total, policy_docs_canon_depth_stub, policy_docs_canon_slices_met,
    PolicyDocsCanonDepth, FM_BAND87_ROWS, POLICY_DOCS_CANON_BAND87_ROWS, POLICY_DOCS_CANON_CASES,
    POLICY_DOCS_CANON_CRITERIA, POLICY_DOCS_CANON_SLICES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1509_band_policy_docs_canon_close_ph_s1518() {
    assert_eq!(
        policy_docs_canon_depth_stub(Some(&json!({"policy_docs_canon_depth": true}))),
        PolicyDocsCanonDepth::DepthModule
    );
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

    assert_eq!(POLICY_DOCS_CANON_CRITERIA.len(), 10);
    assert_eq!(policy_docs_canon_criteria_total(), 10);
    assert!(POLICY_DOCS_CANON_CASES.contains(&"doc_loc_audit"));
    assert_eq!(POLICY_DOCS_CANON_SLICES.len(), 6);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("policy_docs_canon_mode"));
    assert!(loc_audit.contains("policy_docs_canon_criteria_met_count"));
    assert!(loc_audit.contains("--policy-docs-canon"));

    let policy_doc = include_str!("../docs/development/POLICIES_DOCS_CANON.md");
    assert_eq!(policy_docs_canon_slices_met(policy_doc), (6, 6));
    assert!(policy_doc.contains("--policy-docs-canon"));
    assert!(
        policy_doc.contains("POLICY_DOCS_CANON_SLICES") || policy_doc.contains("POLICIES_DEPTH.md")
    );

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND87_ROWS {
        assert!(fm.contains(row), "FM missing band-87 row {row}");
    }
    assert!(fm.contains("PH-S1518"));
    assert!(fm.contains("5.68"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1509") || handoff.contains("band 87"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 88"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--policy-docs-canon"));
    assert!(run_local.contains("VERIFY_POLICY_DOCS_CANON"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("policy_docs_canon_depth") || strategy.contains("band 87"));

    let roadmap = include_str!("../docs/development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md");
    assert!(roadmap.contains("PH-S1509") || roadmap.contains("docs canon"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_POLICY_DOCS_CANON"));
    assert!(verify.contains("--policy-docs-canon"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--policy-docs-canon"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("policy_docs_canon_band87_export_shape"));

    for marker in POLICY_DOCS_CANON_BAND87_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || policy_doc.contains(marker),
            "band-87 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/policy_docs_canon_depth.rs").exists());
    assert!(Path::new("docs/development/POLICIES_DOCS_CANON.md").exists());
    assert!(Path::new("tests/policy_docs_canon_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("policy_docs_canon_mode").is_some());
}
