//! PH-S1618: Galaxy horizon close band 97 — Monitoring docs canon.
//! Suite: `galaxy_horizon_s1609_integration`.

use poolai_ui_core::monitoring_docs_canon_depth::{
    monitoring_docs_canon_criteria_total, monitoring_docs_canon_depth_stub,
    MonitoringDocsCanonDepth, FM_BAND97_ROWS, MONITORING_DOCS_CANON_BAND97_ROWS,
    MONITORING_DOCS_CANON_CASES, MONITORING_DOCS_CANON_CRITERIA, MONITORING_DOCS_CANON_SLICES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1609_band_monitoring_docs_canon_close_ph_s1618() {
    // Depth stub
    assert_eq!(
        monitoring_docs_canon_depth_stub(Some(&json!({"monitoring_docs_canon_depth": true}))),
        MonitoringDocsCanonDepth::DepthModule
    );
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

    assert_eq!(MONITORING_DOCS_CANON_CRITERIA.len(), 10);
    assert_eq!(monitoring_docs_canon_criteria_total(), 10);
    assert!(MONITORING_DOCS_CANON_CASES.contains(&"aggregate_flag"));
    assert_eq!(MONITORING_DOCS_CANON_SLICES.len(), 6);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("monitoring_docs_canon_mode"));
    assert!(loc_audit.contains("monitoring_docs_canon_criteria_met_count"));
    assert!(loc_audit.contains("--monitoring-docs-canon"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND97_ROWS {
        assert!(fm.contains(row), "FM missing band-97 row {row}");
    }
    assert!(fm.contains("PH-S1618"));
    assert!(fm.contains("5.78"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1609") || handoff.contains("band 97"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 98"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--monitoring-docs-canon"));
    assert!(run_local.contains("VERIFY_MONITORING_DOCS_CANON"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(
        strategy.contains("monitoring_docs_canon_depth") || strategy.contains("band 97"),
        "RUST_RATIO_STRATEGY missing monitoring docs-canon mention"
    );

    let monitoring_doc = include_str!("../docs/development/MONITORING_DOCS_CANON.md");
    assert!(monitoring_doc.contains("--monitoring-docs-canon"));
    assert!(
        monitoring_doc.contains("MONITORING_DOCS_CANON_SLICES")
            || monitoring_doc.contains("MONITORING_DEPTH.md")
    );

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_MONITORING_DOCS_CANON"));
    assert!(verify.contains("--monitoring-docs-canon"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--monitoring-docs-canon"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("monitoring_docs_canon_band97_export_shape"));

    // Band markers coverage (FM / run_local / loc-audit / verify / docs / smoke)
    for marker in MONITORING_DOCS_CANON_BAND97_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || monitoring_doc.contains(marker),
            "band-97 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/monitoring_docs_canon_depth.rs").exists());
    assert!(Path::new("docs/development/MONITORING_DOCS_CANON.md").exists());
    assert!(Path::new("tests/monitoring_docs_canon_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("monitoring_docs_canon_mode").is_some());
}
