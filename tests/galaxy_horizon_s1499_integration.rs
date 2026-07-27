//! PH-S1508: Galaxy horizon close band 86 — Policies loc-audit aggregate.
//! Suite: `galaxy_horizon_s1499_integration`.

use poolai_ui_core::policy_loc_audit_depth::{
    policy_loc_audit_criteria_total, policy_loc_audit_depth_stub, policy_loc_audit_slices_met,
    PolicyLocAuditDepth, FM_BAND86_ROWS, POLICY_LOC_AUDIT_BAND86_ROWS, POLICY_LOC_AUDIT_CASES,
    POLICY_LOC_AUDIT_CRITERIA, POLICY_LOC_AUDIT_SLICES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1499_band_policy_loc_audit_close_ph_s1508() {
    assert_eq!(
        policy_loc_audit_depth_stub(Some(&json!({"policy_loc_audit_depth": true}))),
        PolicyLocAuditDepth::DepthModule
    );
    assert_eq!(
        policy_loc_audit_depth_stub(Some(&json!({
            "policy_loc_audit_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "policy_loc_audit_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        PolicyLocAuditDepth::FullBand86
    );

    assert_eq!(POLICY_LOC_AUDIT_CRITERIA.len(), 10);
    assert_eq!(policy_loc_audit_criteria_total(), 10);
    assert!(POLICY_LOC_AUDIT_CASES.contains(&"policy_loc_audit_docs"));
    assert_eq!(POLICY_LOC_AUDIT_SLICES.len(), 5);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert_eq!(policy_loc_audit_slices_met(loc_audit), (5, 5));
    assert!(loc_audit.contains("policy_loc_audit_mode"));
    assert!(loc_audit.contains("policy_loc_audit_criteria_met_count"));
    assert!(loc_audit.contains("--policy-loc-audit"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND86_ROWS {
        assert!(fm.contains(row), "FM missing band-86 row {row}");
    }
    assert!(fm.contains("PH-S1508"));
    assert!(fm.contains("5.67"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1499") || handoff.contains("band 86"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 87"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--policy-loc-audit"));
    assert!(run_local.contains("VERIFY_POLICY_LOC_AUDIT"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("policy_loc_audit_depth") || strategy.contains("band 86"));

    let policy_doc = include_str!("../docs/development/POLICIES_LOC_AUDIT.md");
    assert!(policy_doc.contains("--policy-loc-audit"));
    assert!(
        policy_doc.contains("POLICY_LOC_AUDIT_SLICES")
            || policy_doc.contains("--policy-stand-smoke")
    );

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1499") || roadmap.contains("loc-audit"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_POLICY_LOC_AUDIT"));
    assert!(verify.contains("--policy-loc-audit"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--policy-loc-audit"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("policy_loc_audit_band86_export_shape"));

    for marker in POLICY_LOC_AUDIT_BAND86_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || policy_doc.contains(marker),
            "band-86 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/policy_loc_audit_depth.rs").exists());
    assert!(Path::new("docs/development/POLICIES_LOC_AUDIT.md").exists());
    assert!(Path::new("tests/policy_loc_audit_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("policy_loc_audit_mode").is_some());
}
