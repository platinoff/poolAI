//! PH-S1288: Galaxy horizon close band 64 — SSO admin/ops glue.
//! Suite: `galaxy_horizon_s1279_integration`.

use poolai_ui_core::sso_admin_ops_depth::{
    sso_admin_ops_criteria_total, sso_admin_ops_depth_stub, SsoAdminOpsDepth, FM_BAND64_ROWS,
    SSO_ADMIN_OPS_BAND64_ROWS, SSO_ADMIN_OPS_CASES, SSO_ADMIN_OPS_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1279_band_sso_admin_ops_close_ph_s1288() {
    assert_eq!(
        sso_admin_ops_depth_stub(Some(&json!({"sso_admin_ops_depth": true}))),
        SsoAdminOpsDepth::DepthModule
    );
    assert_eq!(
        sso_admin_ops_depth_stub(Some(&json!({
            "sso_admin_ops_depth": true,
            "store_strip": true,
            "providers_glue": true,
            "html_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "sso_admin_ops_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        SsoAdminOpsDepth::FullBand64
    );

    assert_eq!(SSO_ADMIN_OPS_CRITERIA.len(), 10);
    assert_eq!(sso_admin_ops_criteria_total(), 10);
    assert!(SSO_ADMIN_OPS_CASES.contains(&"sso_admin_ops_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND64_ROWS {
        assert!(fm.contains(row), "FM missing band-64 row {row}");
    }
    assert!(fm.contains("PH-S1288"));
    assert!(fm.contains("5.45"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1279") || handoff.contains("band 64"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 65"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--sso-admin-ops"));
    assert!(run_local.contains("VERIFY_SSO_ADMIN_OPS"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("sso_admin_ops_depth") || strategy.contains("band 64"));

    let sso_doc = include_str!("../docs/development/SSO_ADMIN_OPS.md");
    assert!(sso_doc.contains("/api/enterprise/security/sso/store"));
    assert!(sso_doc.contains("refreshOAuth2Providers"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1279") || roadmap.contains("admin/ops glue"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_SSO_ADMIN_OPS"));
    assert!(verify.contains("--sso-admin-ops"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--sso-admin-ops"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("sso_admin_ops_mode"));
    assert!(loc_audit.contains("sso_admin_ops_criteria_met_count"));

    let security_ui = include_str!("../src/ui/admin/security.rs");
    assert!(security_ui.contains("sso-store-badge"));
    assert!(security_ui.contains("refreshOAuth2Providers"));

    for marker in SSO_ADMIN_OPS_BAND64_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || security_ui.contains(marker)
                || verify.contains(marker),
            "band-64 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/sso_admin_ops_depth.rs").exists());
    assert!(Path::new("docs/development/SSO_ADMIN_OPS.md").exists());
    assert!(Path::new("tests/sso_admin_ops_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("sso_admin_ops_mode").is_some());
}
