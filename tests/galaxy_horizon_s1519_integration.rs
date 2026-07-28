//! PH-S1528: Galaxy horizon close band 88 — Policies vision sync.
//! Suite: `galaxy_horizon_s1519_integration`.

use poolai_ui_core::policy_vision_sync_depth::{
    policy_vision_sync_criteria_total, policy_vision_sync_depth_stub,
    policy_vision_sync_slices_met, PolicyVisionSyncDepth, FM_BAND88_ROWS,
    POLICY_VISION_SYNC_BAND88_ROWS, POLICY_VISION_SYNC_CASES, POLICY_VISION_SYNC_CRITERIA,
    POLICY_VISION_SYNC_SLICES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1519_band_policy_vision_sync_close_ph_s1528() {
    assert_eq!(
        policy_vision_sync_depth_stub(Some(&json!({"policy_vision_sync_depth": true}))),
        PolicyVisionSyncDepth::DepthModule
    );
    assert_eq!(
        policy_vision_sync_depth_stub(Some(&json!({
            "policy_vision_sync_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "policy_vision_sync_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        PolicyVisionSyncDepth::FullBand88
    );

    assert_eq!(POLICY_VISION_SYNC_CRITERIA.len(), 10);
    assert_eq!(policy_vision_sync_criteria_total(), 10);
    assert!(POLICY_VISION_SYNC_CASES.contains(&"vision_manifest"));
    assert_eq!(POLICY_VISION_SYNC_SLICES.len(), 6);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("policy_vision_sync_mode"));
    assert!(loc_audit.contains("policy_vision_sync_criteria_met_count"));
    assert!(loc_audit.contains("--policy-vision-sync"));

    let policy_doc = include_str!("../docs/development/POLICIES_VISION_SYNC.md");
    assert_eq!(policy_vision_sync_slices_met(policy_doc), (6, 6));
    assert!(policy_doc.contains("--policy-vision-sync"));
    assert!(
        policy_doc.contains("POLICY_VISION_SYNC_SLICES")
            || policy_doc.contains("POLICIES_DOCS_CANON.md")
    );

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND88_ROWS {
        assert!(fm.contains(row), "FM missing band-88 row {row}");
    }
    assert!(fm.contains("PH-S1528"));
    assert!(fm.contains("5.69"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1519") || handoff.contains("band 88"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 89"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--policy-vision-sync"));
    assert!(run_local.contains("VERIFY_POLICY_VISION_SYNC"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("policy_vision_sync_depth") || strategy.contains("band 88"));

    let roadmap = include_str!("../docs/development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md");
    assert!(roadmap.contains("PH-S1519") || roadmap.contains("vision-sync"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_POLICY_VISION_SYNC"));
    assert!(verify.contains("--policy-vision-sync"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--policy-vision-sync"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("policy_vision_sync_band88_export_shape"));

    for marker in POLICY_VISION_SYNC_BAND88_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || policy_doc.contains(marker),
            "band-88 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/policy_vision_sync_depth.rs").exists());
    assert!(Path::new("docs/development/POLICIES_VISION_SYNC.md").exists());
    assert!(Path::new("tests/policy_vision_sync_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("policy_vision_sync_mode").is_some());
}
