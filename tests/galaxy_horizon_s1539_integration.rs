//! PH-S1548: Galaxy horizon close band 90 — Policies horizon close.
//! Suite: `galaxy_horizon_s1539_integration`.

use poolai_ui_core::policy_horizon_depth::{
    policy_horizon_criteria_total, policy_horizon_depth_stub, policy_horizon_slices_met,
    PolicyHorizonDepth, FM_BAND90_ROWS, POLICY_HORIZON_BAND90_ROWS, POLICY_HORIZON_CASES,
    POLICY_HORIZON_CRITERIA, POLICY_HORIZON_SLICES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1539_band_policy_horizon_close_ph_s1548() {
    assert_eq!(
        policy_horizon_depth_stub(Some(&json!({"policy_horizon_depth": true}))),
        PolicyHorizonDepth::DepthModule
    );
    assert_eq!(
        policy_horizon_depth_stub(Some(&json!({
            "policy_horizon_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "policy_horizon_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        PolicyHorizonDepth::FullBand90
    );

    assert_eq!(POLICY_HORIZON_CRITERIA.len(), 10);
    assert_eq!(policy_horizon_criteria_total(), 10);
    assert!(POLICY_HORIZON_CASES.contains(&"doc_ratio_advisory"));
    assert_eq!(POLICY_HORIZON_SLICES.len(), 10);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("policy_horizon_mode"));
    assert!(loc_audit.contains("policy_horizon_criteria_met_count"));
    assert!(loc_audit.contains("--policy-horizon"));

    let policy_doc = include_str!("../docs/development/POLICIES_HORIZON.md");
    assert_eq!(policy_horizon_slices_met(policy_doc), (10, 10));
    assert!(policy_doc.contains("--policy-horizon"));
    assert!(policy_doc.contains("POLICY_HORIZON_SLICES"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND90_ROWS {
        assert!(fm.contains(row), "FM missing band-90 row {row}");
    }
    assert!(fm.contains("PH-S1548"));
    assert!(fm.contains("5.71"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1539") || handoff.contains("band 90"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 91"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--policy-horizon"));
    assert!(run_local.contains("VERIFY_POLICY_HORIZON"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("policy_horizon_depth") || strategy.contains("band 90"));

    let roadmap = include_str!("../docs/development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md");
    assert!(
        roadmap.contains("PH-S1539")
            || roadmap.contains("horizon close")
            || roadmap.contains("Policies")
    );

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_POLICY_HORIZON"));
    assert!(verify.contains("--policy-horizon"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--policy-horizon"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("policy_horizon_band90_export_shape"));

    for marker in POLICY_HORIZON_BAND90_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || policy_doc.contains(marker),
            "band-90 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/policy_horizon_depth.rs").exists());
    assert!(Path::new("docs/development/POLICIES_HORIZON.md").exists());
    assert!(Path::new("tests/policy_horizon_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("policy_horizon_mode").is_some());
}
