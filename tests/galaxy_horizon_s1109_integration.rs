//! PH-S1118: Galaxy horizon close band 47 — STABLE touch-up.

use poolai_ui_core::stable_state_touchup_depth::{
    stable_criteria_total, stable_state_touchup_depth_stub, StableStateTouchupDepth,
    FM_BAND47_ROWS, STABLE_TOUCHUP_BAND47_ROWS, STABLE_TOUCHUP_CASES, STABLE_TOUCHUP_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1109_band_stable_touchup_close_ph_s1118() {
    assert_eq!(
        stable_state_touchup_depth_stub(Some(&json!({"loc_audit_touchup": true}))),
        StableStateTouchupDepth::LocAuditTouchup
    );
    assert_eq!(
        stable_state_touchup_depth_stub(Some(&json!({
            "criteria_registry": true,
            "stable_summary": true,
            "index_canon": true,
            "handoff_zriz": true,
            "loc_audit_touchup": true,
            "verify_dev_stand_hook": true,
            "quick_touchup": true,
            "docs_canon": true,
        }))),
        StableStateTouchupDepth::FullBand47
    );

    assert_eq!(STABLE_TOUCHUP_CRITERIA.len(), 7);
    assert_eq!(stable_criteria_total(), 7);
    assert!(STABLE_TOUCHUP_CASES.contains(&"rust_ratio_formal"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND47_ROWS {
        assert!(fm.contains(row), "FM missing band-47 row {row}");
    }
    assert!(fm.contains("PH-S1118"));
    assert!(fm.contains("5.28"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1109") || handoff.contains("band 47"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 48"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--stable-touchup"));
    assert!(run_local.contains("VERIFY_STABLE_TOUCHUP"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("stable_state_touchup_depth"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_STABLE_TOUCHUP"));
    assert!(verify.contains("--stable-touchup"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--stable-touchup"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("stable_touchup_mode"));
    assert!(loc_audit.contains("stable_criteria_met_count"));

    let stand_smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(stand_smoke.contains("stable_state_touchup_band47_export_shape_ph_s1114"));

    for marker in STABLE_TOUCHUP_BAND47_ROWS {
        assert!(
            fm.contains(marker) || run_local.contains(marker) || loc_audit.contains(marker),
            "band-47 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/stable_state_touchup_depth.rs").exists());

    let stable = include_str!("../docs/status/STABLE_STATE_SUMMARY.md");
    assert!(stable.contains("band 47") || stable.contains("PH-S1118"));

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("stable_touchup_mode").is_some());
    assert!(ratio.get("stable_criteria_total").is_some());
}
