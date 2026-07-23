//! PH-S1348: Galaxy horizon close band 70 — SSO horizon.
//! Suite: `galaxy_horizon_s1339_integration`.

use poolai_ui_core::sso_horizon_depth::{
    sso_horizon_criteria_total, sso_horizon_depth_stub, sso_horizon_slices_met, SsoHorizonDepth,
    FM_BAND70_ROWS, SSO_HORIZON_BAND70_ROWS, SSO_HORIZON_CASES, SSO_HORIZON_CRITERIA,
    SSO_HORIZON_SLICES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1339_band_sso_horizon_close_ph_s1348() {
    assert_eq!(
        sso_horizon_depth_stub(Some(&json!({"sso_horizon_depth": true}))),
        SsoHorizonDepth::DepthModule
    );
    assert_eq!(
        sso_horizon_depth_stub(Some(&json!({
            "sso_horizon_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "sso_horizon_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        SsoHorizonDepth::FullBand70
    );

    assert_eq!(SSO_HORIZON_CRITERIA.len(), 10);
    assert_eq!(sso_horizon_criteria_total(), 10);
    assert!(SSO_HORIZON_CASES.contains(&"doc_ratio_advisory"));
    assert_eq!(SSO_HORIZON_SLICES.len(), 10);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("sso_horizon_mode"));
    assert!(loc_audit.contains("sso_horizon_criteria_met_count"));
    assert!(loc_audit.contains("--sso-horizon"));

    let sso_doc = include_str!("../docs/development/SSO_HORIZON.md");
    assert_eq!(sso_horizon_slices_met(sso_doc), (10, 10));
    assert!(sso_doc.contains("--sso-horizon"));
    assert!(sso_doc.contains("SSO_HORIZON_SLICES") || sso_doc.contains("SSO_RATIO_ADVISORY.md"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND70_ROWS {
        assert!(fm.contains(row), "FM missing band-70 row {row}");
    }
    assert!(fm.contains("PH-S1348"));
    assert!(fm.contains("5.51"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1339") || handoff.contains("band 70"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 71"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--sso-horizon"));
    assert!(run_local.contains("VERIFY_SSO_HORIZON"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("sso_horizon_depth") || strategy.contains("band 70"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1339") || roadmap.contains("horizon close"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_SSO_HORIZON"));
    assert!(verify.contains("--sso-horizon"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--sso-horizon"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("sso_horizon_band70_export_shape"));

    for marker in SSO_HORIZON_BAND70_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || sso_doc.contains(marker),
            "band-70 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/sso_horizon_depth.rs").exists());
    assert!(Path::new("docs/development/SSO_HORIZON.md").exists());
    assert!(Path::new("tests/sso_horizon_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("sso_horizon_mode").is_some());
}
