//! PH-S1298: Galaxy horizon close band 65 — SSO live stand smoke.
//! Suite: `galaxy_horizon_s1289_integration`.

use poolai_ui_core::sso_stand_smoke_depth::{
    sso_stand_smoke_criteria_total, sso_stand_smoke_depth_stub, SsoStandSmokeDepth, FM_BAND65_ROWS,
    SSO_STAND_SMOKE_BAND65_ROWS, SSO_STAND_SMOKE_CASES, SSO_STAND_SMOKE_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1289_band_sso_stand_smoke_close_ph_s1298() {
    assert_eq!(
        sso_stand_smoke_depth_stub(Some(&json!({"sso_stand_smoke_depth": true}))),
        SsoStandSmokeDepth::DepthModule
    );
    assert_eq!(
        sso_stand_smoke_depth_stub(Some(&json!({
            "sso_stand_smoke_depth": true,
            "live_store": true,
            "live_crud": true,
            "live_callback_fixtures": true,
            "cli_flag": true,
            "loc_audit_flag": true,
            "verify_dev_stand_hook": true,
            "sso_stand_smoke_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        SsoStandSmokeDepth::FullBand65
    );

    assert_eq!(SSO_STAND_SMOKE_CRITERIA.len(), 10);
    assert_eq!(sso_stand_smoke_criteria_total(), 10);
    assert!(SSO_STAND_SMOKE_CASES.contains(&"sso_stand_smoke_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND65_ROWS {
        assert!(fm.contains(row), "FM missing band-65 row {row}");
    }
    assert!(fm.contains("PH-S1298"));
    assert!(fm.contains("5.46"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1289") || handoff.contains("band 65"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 66"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--sso-stand-smoke"));
    assert!(run_local.contains("VERIFY_SSO_STAND_SMOKE"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("sso_stand_smoke_depth") || strategy.contains("band 65"));

    let sso_doc = include_str!("../docs/development/SSO_STAND_SMOKE.md");
    assert!(sso_doc.contains("/api/enterprise/security/sso/store"));
    assert!(
        sso_doc.contains("smoke_sso_oauth2_saml_crud") || sso_doc.contains("--sso-stand-smoke")
    );

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1289") || roadmap.contains("stand smoke"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_SSO_STAND_SMOKE"));
    assert!(verify.contains("--sso-stand-smoke"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--sso-stand-smoke"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("sso_stand_smoke_mode"));
    assert!(loc_audit.contains("sso_stand_smoke_criteria_met_count"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("smoke_sso_store_wire"));
    assert!(smoke.contains("smoke_sso_oauth2_saml_crud"));
    assert!(smoke.contains("smoke_sso_callback_fixtures"));
    assert!(smoke.contains("sso_stand_smoke_only"));

    for marker in SSO_STAND_SMOKE_BAND65_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker),
            "band-65 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/sso_stand_smoke_depth.rs").exists());
    assert!(Path::new("docs/development/SSO_STAND_SMOKE.md").exists());
    assert!(Path::new("tests/sso_stand_smoke_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("sso_stand_smoke_mode").is_some());
}
