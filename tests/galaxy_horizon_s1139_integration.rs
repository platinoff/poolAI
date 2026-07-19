//! PH-S1148: Galaxy horizon close band 50 — CI canon gate.

use poolai_ui_core::ci_canon_depth::{
    ci_canon_criteria_total, ci_canon_depth_stub, CiCanonDepth, CI_CANON_BAND50_ROWS,
    CI_CANON_CASES, CI_CANON_CRITERIA, FM_BAND50_ROWS,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1139_band_ci_canon_close_ph_s1148() {
    assert_eq!(
        ci_canon_depth_stub(Some(&json!({"test_ci_scope": true}))),
        CiCanonDepth::TestCiScope
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
    assert!(CI_CANON_CASES.contains(&"ci_canon_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND50_ROWS {
        assert!(fm.contains(row), "FM missing band-50 row {row}");
    }
    assert!(fm.contains("PH-S1148"));
    assert!(fm.contains("5.31"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1139") || handoff.contains("band 50"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 51"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--ci-canon"));
    assert!(run_local.contains("VERIFY_CI_CANON"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("ci_canon_depth"));

    let ci_canon_doc = include_str!("../docs/development/CI_CANON.md");
    assert!(ci_canon_doc.contains("poolai-openapi-gap-audit"));
    assert!(ci_canon_doc.contains("cargo test-ci"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_CI_CANON"));
    assert!(verify.contains("--ci-canon"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--ci-canon"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("ci_canon_mode"));
    assert!(loc_audit.contains("ci_canon_criteria_met_count"));

    for marker in CI_CANON_BAND50_ROWS {
        assert!(
            fm.contains(marker) || run_local.contains(marker) || loc_audit.contains(marker),
            "band-50 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/ci_canon_depth.rs").exists());
    assert!(Path::new("docs/development/CI_CANON.md").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("ci_canon_mode").is_some());
    assert!(ratio.get("ci_canon_criteria_total").is_some());
}
