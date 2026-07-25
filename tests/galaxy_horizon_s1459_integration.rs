//! PH-S1468: Galaxy horizon close band 82 — Policies store wire.
//! Suite: `galaxy_horizon_s1459_integration`.

use poolai_ui_core::policy_store_depth::{
    policy_store_criteria_total, policy_store_depth_stub, PolicyStoreDepth, FM_BAND82_ROWS,
    POLICY_BAND82_ROWS, POLICY_STORE_CASES, POLICY_STORE_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1459_band_policies_store_close_ph_s1468() {
    assert_eq!(
        policy_store_depth_stub(Some(&json!({"policy_store_depth": true}))),
        PolicyStoreDepth::DepthModule
    );
    assert_eq!(
        policy_store_depth_stub(Some(&json!({
            "policy_store_depth": true,
            "store_wire": true,
            "api_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "policy_store_docs": true,
        }))),
        PolicyStoreDepth::FullBand82
    );

    assert_eq!(POLICY_STORE_CRITERIA.len(), 7);
    assert_eq!(policy_store_criteria_total(), 7);
    assert!(POLICY_STORE_CASES.contains(&"policy_store_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND82_ROWS {
        assert!(fm.contains(row), "FM missing band-82 row {row}");
    }
    assert!(fm.contains("PH-S1468"));
    assert!(fm.contains("5.63"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1459") || handoff.contains("band 82"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 83"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--policy-store"));
    assert!(run_local.contains("VERIFY_POLICY_STORE"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("policy_store") || strategy.contains("band 82"));

    let policy_doc = include_str!("../docs/development/POLICIES_STORE.md");
    assert!(policy_doc.contains("POOLAI_POLICY_STORE"));
    assert!(policy_doc.contains("policy_store_wire"));
    assert!(policy_doc.contains("POOLAI_POLICY_DATA_DIR"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1459") || roadmap.contains("Policies"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_POLICY_STORE"));
    assert!(verify.contains("--policy-store"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--policy-store"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("policy_store_mode"));
    assert!(loc_audit.contains("policy_store_criteria_met_count"));

    let security_mod = include_str!("../src/enterprise/security.rs");
    assert!(security_mod.contains("POOLAI_POLICY_STORE"));
    assert!(security_mod.contains("policy_store_wire"));
    assert!(security_mod.contains("POOLAI_POLICY_DATA_DIR"));

    for marker in POLICY_BAND82_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || policy_doc.contains(marker)
                || verify.contains(marker),
            "band-82 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/policy_store_depth.rs").exists());
    assert!(Path::new("docs/development/POLICIES_STORE.md").exists());
    assert!(Path::new("docs/development/PH_S_MASTER_BACKLOG_1000.md").exists());
    assert!(Path::new("tests/policy_store_wire_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("policy_store_mode").is_some());
    assert!(ratio.get("policy_store_criteria_total").is_some());
}
