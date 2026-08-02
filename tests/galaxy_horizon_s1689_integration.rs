//! PH-S1698: Galaxy horizon close band 105 — Ratio96 stand smoke.
//! Suite: `galaxy_horizon_s1689_integration`.

use poolai_ui_core::ratio96_stand_smoke_depth::{
    ratio96_stand_smoke_criteria_total, ratio96_stand_smoke_depth_stub, Ratio96StandSmokeDepth,
    FM_BAND105_ROWS, RATIO96_STAND_SMOKE_BAND105_ROWS, RATIO96_STAND_SMOKE_CASES,
    RATIO96_STAND_SMOKE_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1689_band_ratio96_stand_smoke_close_ph_s1698() {
    assert_eq!(
        ratio96_stand_smoke_depth_stub(Some(&json!({"ratio96_stand_smoke_depth": true}))),
        Ratio96StandSmokeDepth::DepthModule
    );
    assert_eq!(
        ratio96_stand_smoke_depth_stub(Some(&json!({
            "ratio96_stand_smoke_depth": true,
            "store_wire_smoke": true,
            "query_smoke": true,
            "field_fixture_smoke": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "docs_canon": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        Ratio96StandSmokeDepth::FullBand105
    );

    assert_eq!(RATIO96_STAND_SMOKE_CRITERIA.len(), 10);
    assert_eq!(ratio96_stand_smoke_criteria_total(), 10);
    assert!(RATIO96_STAND_SMOKE_CASES.contains(&"store_wire_smoke"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND105_ROWS {
        assert!(fm.contains(row), "FM missing band-105 row {row}");
    }
    assert!(fm.contains("PH-S1698"));
    assert!(fm.contains("5.86"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1689") || handoff.contains("band 105"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 106"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--ratio96-stand-smoke"));
    assert!(run_local.contains("VERIFY_RATIO96_STAND_SMOKE"));

    let ratio_doc = include_str!("../docs/development/RATIO96_STAND_SMOKE.md");
    assert!(ratio_doc.contains("smoke_ratio96_store_wire"));
    assert!(ratio_doc.contains("/api/v1/ops/ratio96"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_RATIO96_STAND_SMOKE"));
    assert!(verify.contains("--ratio96-stand-smoke"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--ratio96-stand-smoke"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("ratio96_stand_smoke_mode"));
    assert!(loc_audit.contains("ratio96_stand_smoke_criteria_met_count"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("smoke_ratio96_store_wire"));
    assert!(smoke.contains("smoke_ratio96_query"));
    assert!(smoke.contains("smoke_ratio96_field_fixtures"));
    assert!(smoke.contains("ratio96_stand_smoke_band105_export_shape"));

    for marker in RATIO96_STAND_SMOKE_BAND105_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || ratio_doc.contains(marker),
            "band-105 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/ratio96_stand_smoke_depth.rs").exists());
    assert!(Path::new("docs/development/RATIO96_STAND_SMOKE.md").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("ratio96_stand_smoke_mode").is_some());
    assert!(ratio.get("ratio96_stand_smoke_criteria_total").is_some());
}
