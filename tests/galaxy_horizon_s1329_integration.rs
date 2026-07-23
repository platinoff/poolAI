//! PH-S1338: Galaxy horizon close band 69 — SSO ratio advisory.
//! Suite: `galaxy_horizon_s1329_integration`.

use poolai_ui_core::sso_ratio_advisory_depth::{
    sso_ratio_advisory_criteria_total, sso_ratio_advisory_depth_stub,
    sso_ratio_advisory_slices_met, SsoRatioAdvisoryDepth, FM_BAND69_ROWS,
    SSO_RATIO_ADVISORY_BAND69_ROWS, SSO_RATIO_ADVISORY_CASES, SSO_RATIO_ADVISORY_CRITERIA,
    SSO_RATIO_ADVISORY_SLICES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1329_band_sso_ratio_advisory_close_ph_s1338() {
    assert_eq!(
        sso_ratio_advisory_depth_stub(Some(&json!({"sso_ratio_advisory_depth": true}))),
        SsoRatioAdvisoryDepth::DepthModule
    );
    assert_eq!(
        sso_ratio_advisory_depth_stub(Some(&json!({
            "sso_ratio_advisory_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "sso_ratio_advisory_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        SsoRatioAdvisoryDepth::FullBand69
    );

    assert_eq!(SSO_RATIO_ADVISORY_CRITERIA.len(), 10);
    assert_eq!(sso_ratio_advisory_criteria_total(), 10);
    assert!(SSO_RATIO_ADVISORY_CASES.contains(&"doc_vision_sync"));
    assert_eq!(SSO_RATIO_ADVISORY_SLICES.len(), 6);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("sso_ratio_advisory_mode"));
    assert!(loc_audit.contains("sso_ratio_advisory_criteria_met_count"));
    assert!(loc_audit.contains("--sso-ratio-advisory"));

    let sso_doc = include_str!("../docs/development/SSO_RATIO_ADVISORY.md");
    assert_eq!(sso_ratio_advisory_slices_met(sso_doc), (6, 6));
    assert!(sso_doc.contains("--sso-ratio-advisory"));
    assert!(
        sso_doc.contains("SSO_RATIO_ADVISORY_SLICES") || sso_doc.contains("SSO_VISION_SYNC.md")
    );

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND69_ROWS {
        assert!(fm.contains(row), "FM missing band-69 row {row}");
    }
    assert!(fm.contains("PH-S1338"));
    assert!(fm.contains("5.50"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1329") || handoff.contains("band 69"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 70"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--sso-ratio-advisory"));
    assert!(run_local.contains("VERIFY_SSO_RATIO_ADVISORY"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("sso_ratio_advisory_depth") || strategy.contains("band 69"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1329") || roadmap.contains("ratio-advisory"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_SSO_RATIO_ADVISORY"));
    assert!(verify.contains("--sso-ratio-advisory"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--sso-ratio-advisory"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("sso_ratio_advisory_band69_export_shape"));

    for marker in SSO_RATIO_ADVISORY_BAND69_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || sso_doc.contains(marker),
            "band-69 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/sso_ratio_advisory_depth.rs").exists());
    assert!(Path::new("docs/development/SSO_RATIO_ADVISORY.md").exists());
    assert!(Path::new("tests/sso_ratio_advisory_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("sso_ratio_advisory_mode").is_some());
}
