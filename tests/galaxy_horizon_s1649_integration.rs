//! PH-S1658: Galaxy horizon close band 101 — Ratio96 depth scaffold.
//! Suite: `galaxy_horizon_s1649_integration`.

use poolai_ui_core::ratio96_depth::{
    ratio96_criteria_total, ratio96_depth_stub, ratio96_phase_f_slices_met, Ratio96Depth,
    FM_BAND101_ROWS, RATIO96_BAND101_ROWS, RATIO96_CASES, RATIO96_CRITERIA, RATIO96_PHASE_F_SLICES,
};
use poolai_ui_core::ratio96_store_depth::ratio96_store_wire;
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1649_band_ratio96_depth_scaffold_ph_s1658() {
    assert_eq!(
        ratio96_depth_stub(Some(&json!({"ratio96_depth": true}))),
        Ratio96Depth::DepthModule
    );
    assert_eq!(
        ratio96_depth_stub(Some(&json!({
            "ratio96_depth": true,
            "slice_aggregate": true,
            "store_wire": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "ratio96_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        Ratio96Depth::FullBand101
    );

    assert_eq!(RATIO96_CRITERIA.len(), 10);
    assert_eq!(ratio96_criteria_total(), 10);
    assert!(RATIO96_CASES.contains(&"ratio_hold_advisory"));
    assert_eq!(RATIO96_PHASE_F_SLICES.len(), 10);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("ratio96_mode"));
    assert!(loc_audit.contains("ratio96_criteria_met_count"));
    assert!(loc_audit.contains("--ratio96"));

    let ratio_doc = include_str!("../docs/development/RATIO96_DEPTH.md");
    assert_eq!(ratio96_phase_f_slices_met(ratio_doc), (10, 10));
    assert!(ratio_doc.contains("--ratio96"));
    assert!(ratio_doc.contains("RATIO96_PHASE_F_SLICES"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND101_ROWS {
        assert!(fm.contains(row), "FM missing band-101 row {row}");
    }
    assert!(fm.contains("PH-S1658"));
    assert!(fm.contains("5.82"));
    assert!(fm.contains("5.18"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1649") || handoff.contains("band 101"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 102"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--ratio96"));
    assert!(run_local.contains("VERIFY_RATIO96"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("ratio96_depth") || strategy.contains("band 101"));

    let roadmap = include_str!("../docs/development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md");
    assert!(
        roadmap.contains("PH-S1649")
            || roadmap.contains("Ratio96")
            || roadmap.contains("ratio96 depth")
    );

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_RATIO96"));
    assert!(verify.contains("--ratio96"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--ratio96"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("ratio96_band101_export_shape"));

    for marker in RATIO96_BAND101_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || ratio_doc.contains(marker),
            "band-101 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/ratio96_depth.rs").exists());
    assert!(Path::new("crates/poolai-ui-core/src/ratio96_store_depth.rs").exists());
    assert!(Path::new("docs/development/RATIO96_DEPTH.md").exists());
    assert!(Path::new("docs/development/RATIO96_RATIO_ADVISORY.md").exists());
    assert!(Path::new("tests/ratio96_depth_contracts.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("ratio96_mode").is_some());

    ratio96_store_wire().expect("durable ratio store readable");
}
