//! PH-S1488: Galaxy horizon close band 84 — Policies admin/ops glue.
//! Suite: `galaxy_horizon_s1479_integration`.

use poolai_ui_core::policy_admin_ops_depth::{
    policy_admin_ops_criteria_total, policy_admin_ops_depth_stub, PolicyAdminOpsDepth,
    FM_BAND84_ROWS, POLICY_ADMIN_OPS_BAND84_ROWS, POLICY_ADMIN_OPS_CASES,
    POLICY_ADMIN_OPS_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1479_band_policy_admin_ops_close_ph_s1488() {
    assert_eq!(
        policy_admin_ops_depth_stub(Some(&json!({"policy_admin_ops_depth": true}))),
        PolicyAdminOpsDepth::DepthModule
    );
    assert_eq!(
        policy_admin_ops_depth_stub(Some(&json!({
            "policy_admin_ops_depth": true,
            "store_strip": true,
            "query_ops_glue": true,
            "html_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "policy_admin_ops_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        PolicyAdminOpsDepth::FullBand84
    );

    assert_eq!(POLICY_ADMIN_OPS_CRITERIA.len(), 10);
    assert_eq!(policy_admin_ops_criteria_total(), 10);
    assert!(POLICY_ADMIN_OPS_CASES.contains(&"policy_admin_ops_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND84_ROWS {
        assert!(fm.contains(row), "FM missing band-84 row {row}");
    }
    assert!(fm.contains("PH-S1488"));
    assert!(fm.contains("5.65"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1479") || handoff.contains("band 84"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 85"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--policy-admin-ops"));
    assert!(run_local.contains("VERIFY_POLICY_ADMIN_OPS"));

    let policy_doc = include_str!("../docs/development/POLICIES_ADMIN_OPS.md");
    assert!(policy_doc.contains("policy-store-badge"));
    assert!(policy_doc.contains("/api/enterprise/policy/store"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_POLICY_ADMIN_OPS"));
    assert!(verify.contains("--policy-admin-ops"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--policy-admin-ops"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("policy_admin_ops_mode"));
    assert!(loc_audit.contains("policy_admin_ops_criteria_met_count"));

    let policy_ui = include_str!("../src/ui/admin/security.rs");
    assert!(policy_ui.contains("policy-store-badge"));
    assert!(policy_ui.contains("refreshSecurityPolicies"));

    for marker in POLICY_ADMIN_OPS_BAND84_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || policy_ui.contains(marker)
                || verify.contains(marker)
                || policy_doc.contains(marker),
            "band-84 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/policy_admin_ops_depth.rs").exists());
    assert!(Path::new("docs/development/POLICIES_ADMIN_OPS.md").exists());
    assert!(Path::new("tests/policy_admin_ops_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("policy_admin_ops_mode").is_some());
    assert!(ratio.get("policy_admin_ops_criteria_total").is_some());
}
