//! PH-S1538: Galaxy horizon close band 89 — Policies ratio advisory.
//! Suite: `galaxy_horizon_s1529_integration`.

use poolai_ui_core::policy_ratio_advisory_depth::{
    policy_ratio_advisory_criteria_total, policy_ratio_advisory_depth_stub,
    policy_ratio_advisory_slices_met, PolicyRatioAdvisoryDepth, FM_BAND89_ROWS,
    POLICY_RATIO_ADVISORY_BAND89_ROWS, POLICY_RATIO_ADVISORY_CASES, POLICY_RATIO_ADVISORY_CRITERIA,
    POLICY_RATIO_ADVISORY_SLICES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1529_band_policy_ratio_advisory_close_ph_s1538() {
    assert_eq!(
        policy_ratio_advisory_depth_stub(Some(&json!({"policy_ratio_advisory_depth": true}))),
        PolicyRatioAdvisoryDepth::DepthModule
    );
    assert_eq!(
        policy_ratio_advisory_depth_stub(Some(&json!({
            "policy_ratio_advisory_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "policy_ratio_advisory_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        PolicyRatioAdvisoryDepth::FullBand89
    );

    assert_eq!(POLICY_RATIO_ADVISORY_CRITERIA.len(), 10);
    assert_eq!(policy_ratio_advisory_criteria_total(), 10);
    assert!(POLICY_RATIO_ADVISORY_CASES.contains(&"doc_vision_sync"));
    assert_eq!(POLICY_RATIO_ADVISORY_SLICES.len(), 6);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("policy_ratio_advisory_mode"));
    assert!(loc_audit.contains("policy_ratio_advisory_criteria_met_count"));
    assert!(loc_audit.contains("--policy-ratio-advisory"));

    let policy_doc = include_str!("../docs/development/POLICIES_RATIO_ADVISORY.md");
    assert_eq!(policy_ratio_advisory_slices_met(policy_doc), (6, 6));
    assert!(policy_doc.contains("--policy-ratio-advisory"));
    assert!(
        policy_doc.contains("POLICY_RATIO_ADVISORY_SLICES")
            || policy_doc.contains("POLICIES_VISION_SYNC.md")
    );

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND89_ROWS {
        assert!(fm.contains(row), "FM missing band-89 row {row}");
    }
    assert!(fm.contains("PH-S1538"));
    assert!(fm.contains("5.70"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1529") || handoff.contains("band 89"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 90"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--policy-ratio-advisory"));
    assert!(run_local.contains("VERIFY_POLICY_RATIO_ADVISORY"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("policy_ratio_advisory_depth") || strategy.contains("band 89"));

    let roadmap = include_str!("../docs/development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md");
    assert!(roadmap.contains("PH-S1529") || roadmap.contains("ratio advisory"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_POLICY_RATIO_ADVISORY"));
    assert!(verify.contains("--policy-ratio-advisory"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--policy-ratio-advisory"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("policy_ratio_advisory_band89_export_shape"));

    for marker in POLICY_RATIO_ADVISORY_BAND89_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || policy_doc.contains(marker),
            "band-89 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/policy_ratio_advisory_depth.rs").exists());
    assert!(Path::new("docs/development/POLICIES_RATIO_ADVISORY.md").exists());
    assert!(Path::new("tests/policy_ratio_advisory_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("policy_ratio_advisory_mode").is_some());
}
