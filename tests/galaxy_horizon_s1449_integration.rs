//! PH-S1458: Galaxy horizon close band 81 — Policies depth scaffold.
//! Suite: `galaxy_horizon_s1449_integration`.

use poolai_ui_core::policy_depth::{
    policy_criteria_total, policy_depth_stub, PolicyDepth, FM_BAND81_ROWS, POLICY_BAND81_ROWS,
    POLICY_CASES, POLICY_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1449_band_policies_depth_close_ph_s1458() {
    assert_eq!(
        policy_depth_stub(Some(&json!({"policy_depth": true}))),
        PolicyDepth::DepthModule
    );
    assert_eq!(
        policy_depth_stub(Some(&json!({
            "policy_depth": true,
            "store_wire": true,
            "api_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "policy_docs": true,
        }))),
        PolicyDepth::FullBand81
    );

    assert_eq!(POLICY_CRITERIA.len(), 8);
    assert_eq!(policy_criteria_total(), 8);
    assert!(POLICY_CASES.contains(&"policy_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND81_ROWS {
        assert!(fm.contains(row), "FM missing band-81 row {row}");
    }
    assert!(fm.contains("PH-S1458"));
    assert!(fm.contains("5.62"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1449") || handoff.contains("band 81"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 82"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--policy"));
    assert!(run_local.contains("VERIFY_POLICY"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("policy_depth") || strategy.contains("band 81"));

    let policy_doc = include_str!("../docs/development/POLICIES_DEPTH.md");
    assert!(policy_doc.contains("POOLAI_POLICY_STORE"));
    assert!(policy_doc.contains("policy_depth"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1449") || roadmap.contains("Policies"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_POLICY"));
    assert!(verify.contains("--policy"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--policy"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("policy_mode"));
    assert!(loc_audit.contains("policy_criteria_met_count"));

    let security_mod = include_str!("../src/enterprise/security.rs");
    assert!(security_mod.contains("POOLAI_POLICY_STORE"));
    assert!(security_mod.contains("validate_security_policy_fields"));

    for marker in POLICY_BAND81_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || policy_doc.contains(marker)
                || verify.contains(marker),
            "band-81 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/policy_depth.rs").exists());
    assert!(Path::new("docs/development/POLICIES_DEPTH.md").exists());
    assert!(Path::new("docs/development/PH_S_MASTER_BACKLOG_1000.md").exists());
    assert!(Path::new("tests/policy_depth_audit.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("policy_mode").is_some());
    assert!(ratio.get("policy_criteria_total").is_some());
}
