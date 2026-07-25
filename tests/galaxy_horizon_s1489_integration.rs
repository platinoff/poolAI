//! PH-S1498: Galaxy horizon close band 85 — Policies stand smoke.
//! Suite: `galaxy_horizon_s1489_integration`.

use poolai_ui_core::policy_stand_smoke_depth::{
    policy_stand_smoke_criteria_total, policy_stand_smoke_depth_stub, PolicyStandSmokeDepth,
    FM_BAND85_ROWS, POLICY_STAND_SMOKE_BAND85_ROWS, POLICY_STAND_SMOKE_CASES,
    POLICY_STAND_SMOKE_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1489_band_policy_stand_smoke_close_ph_s1498() {
    assert_eq!(
        policy_stand_smoke_depth_stub(Some(&json!({"policy_stand_smoke_depth": true}))),
        PolicyStandSmokeDepth::DepthModule
    );
    assert_eq!(
        policy_stand_smoke_depth_stub(Some(&json!({
            "policy_stand_smoke_depth": true,
            "live_store": true,
            "live_policies_query": true,
            "live_policy_field_fixtures": true,
            "cli_flag": true,
            "loc_audit_flag": true,
            "verify_dev_stand_hook": true,
            "policy_stand_smoke_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        PolicyStandSmokeDepth::FullBand85
    );

    assert_eq!(POLICY_STAND_SMOKE_CRITERIA.len(), 10);
    assert_eq!(policy_stand_smoke_criteria_total(), 10);
    assert!(POLICY_STAND_SMOKE_CASES.contains(&"policy_stand_smoke_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND85_ROWS {
        assert!(fm.contains(row), "FM missing band-85 row {row}");
    }
    assert!(fm.contains("PH-S1498"));
    assert!(fm.contains("5.66"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1489") || handoff.contains("band 85"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 86"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--policy-stand-smoke"));
    assert!(run_local.contains("VERIFY_POLICY_STAND_SMOKE"));

    let policy_doc = include_str!("../docs/development/POLICIES_STAND_SMOKE.md");
    assert!(policy_doc.contains("/api/enterprise/policy/store"));
    assert!(policy_doc.contains("--policy-stand-smoke"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_POLICY_STAND_SMOKE"));
    assert!(verify.contains("--policy-stand-smoke"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--policy-stand-smoke"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("policy_stand_smoke_mode"));
    assert!(loc_audit.contains("policy_stand_smoke_criteria_met_count"));

    let stand_smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(stand_smoke.contains("smoke_policy_store_wire"));
    assert!(stand_smoke.contains("smoke_policy_policies_query"));
    assert!(stand_smoke.contains("smoke_policy_field_fixtures"));

    for marker in POLICY_STAND_SMOKE_BAND85_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || stand_smoke.contains(marker)
                || verify.contains(marker)
                || policy_doc.contains(marker),
            "band-85 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/policy_stand_smoke_depth.rs").exists());
    assert!(Path::new("docs/development/POLICIES_STAND_SMOKE.md").exists());
    assert!(Path::new("tests/policy_stand_smoke_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("policy_stand_smoke_mode").is_some());
    assert!(ratio.get("policy_stand_smoke_criteria_total").is_some());
}
