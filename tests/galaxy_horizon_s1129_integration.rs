//! PH-S1138: Galaxy horizon close band 49 — pre-push vision canon gate.

use poolai_ui_core::pre_push_hook_depth::{
    pre_push_hook_criteria_total, pre_push_hook_depth_stub, PrePushHookDepth, FM_BAND49_ROWS,
    PRE_PUSH_HOOK_BAND49_ROWS, PRE_PUSH_HOOK_CASES, PRE_PUSH_HOOK_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1129_band_pre_push_canon_close_ph_s1138() {
    assert_eq!(
        pre_push_hook_depth_stub(Some(&json!({"vision_sync_check": true}))),
        PrePushHookDepth::VisionSyncCheck
    );
    assert_eq!(
        pre_push_hook_depth_stub(Some(&json!({
            "pre_push_hook_script": true,
            "install_hook": true,
            "vision_sync_canon": true,
            "vision_sync_check": true,
            "cargo_fmt_gate": true,
            "pre_push_hook_docs": true,
            "verify_dev_stand_hook": true,
        }))),
        PrePushHookDepth::FullBand49
    );

    assert_eq!(PRE_PUSH_HOOK_CRITERIA.len(), 7);
    assert_eq!(pre_push_hook_criteria_total(), 7);
    assert!(PRE_PUSH_HOOK_CASES.contains(&"pre_push_hook_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND49_ROWS {
        assert!(fm.contains(row), "FM missing band-49 row {row}");
    }
    assert!(fm.contains("PH-S1138"));
    assert!(fm.contains("5.30"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1129") || handoff.contains("band 49"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 50"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--pre-push-canon"));
    assert!(run_local.contains("VERIFY_PRE_PUSH_CANON"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("pre_push_hook_depth"));

    let pre_push_doc = include_str!("../docs/development/PRE_PUSH_HOOK.md");
    assert!(pre_push_doc.contains("pre-push-hook.sh"));
    assert!(pre_push_doc.contains("poolai-vision-sync"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_PRE_PUSH_CANON"));
    assert!(verify.contains("--pre-push-canon"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--pre-push-canon"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("pre_push_canon_mode"));
    assert!(loc_audit.contains("pre_push_criteria_met_count"));

    let vision_sync = include_str!("../src/bin/poolai_vision_sync.rs");
    assert!(vision_sync.contains("collect_canon_docs_drift"));
    assert!(vision_sync.contains("sync_vision_canon_docs"));

    for marker in PRE_PUSH_HOOK_BAND49_ROWS {
        assert!(
            fm.contains(marker) || run_local.contains(marker) || loc_audit.contains(marker),
            "band-49 marker missing: {marker}"
        );
    }

    assert!(Path::new("bin/pre-push-hook.sh").exists());
    assert!(Path::new("bin/install-pre-push-hook.sh").exists());
    assert!(Path::new("crates/poolai-ui-core/src/pre_push_hook_depth.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("pre_push_canon_mode").is_some());
    assert!(ratio.get("pre_push_criteria_total").is_some());
}
