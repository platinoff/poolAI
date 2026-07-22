//! PH-S1258: Galaxy horizon close band 61 — SSO depth scaffold.

use poolai_ui_core::sso_depth::{
    sso_criteria_total, sso_depth_stub, SsoDepth, FM_BAND61_ROWS, SSO_BAND61_ROWS, SSO_CASES,
    SSO_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1249_band_sso_depth_close_ph_s1258() {
    assert_eq!(
        sso_depth_stub(Some(&json!({"sso_depth": true}))),
        SsoDepth::DepthModule
    );
    assert_eq!(
        sso_depth_stub(Some(&json!({
            "sso_depth": true,
            "store_wire": true,
            "api_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "sso_docs": true,
        }))),
        SsoDepth::FullBand61
    );

    assert_eq!(SSO_CRITERIA.len(), 8);
    assert_eq!(sso_criteria_total(), 8);
    assert!(SSO_CASES.contains(&"sso_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND61_ROWS {
        assert!(fm.contains(row), "FM missing band-61 row {row}");
    }
    assert!(fm.contains("PH-S1258"));
    assert!(fm.contains("5.42"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1249") || handoff.contains("band 61"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 62"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--sso"));
    assert!(run_local.contains("VERIFY_SSO"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("sso_depth") || strategy.contains("band 61"));

    let sso_doc = include_str!("../docs/development/SSO_DEPTH.md");
    assert!(sso_doc.contains("POOLAI_SSO_STORE"));
    assert!(sso_doc.contains("sso_depth"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1249") || roadmap.contains("SSO"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_SSO"));
    assert!(verify.contains("--sso"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--sso"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("sso_mode"));
    assert!(loc_audit.contains("sso_criteria_met_count"));

    let security = include_str!("../src/enterprise/security.rs");
    assert!(security.contains("POOLAI_SSO_STORE"));
    assert!(security.contains("validate_saml_audience_and_time"));

    for marker in SSO_BAND61_ROWS {
        assert!(
            fm.contains(marker) || run_local.contains(marker) || loc_audit.contains(marker),
            "band-61 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/sso_depth.rs").exists());
    assert!(Path::new("docs/development/SSO_DEPTH.md").exists());
    assert!(Path::new("docs/development/PH_S_MASTER_BACKLOG_1000.md").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("sso_mode").is_some());
    assert!(ratio.get("sso_criteria_total").is_some());
}
