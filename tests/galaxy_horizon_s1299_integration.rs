//! PH-S1308: Galaxy horizon close band 66 — SSO loc-audit aggregate.
//! Suite: `galaxy_horizon_s1299_integration`.

use poolai_ui_core::sso_loc_audit_depth::{
    sso_loc_audit_criteria_total, sso_loc_audit_depth_stub, sso_loc_audit_slices_met,
    SsoLocAuditDepth, FM_BAND66_ROWS, SSO_LOC_AUDIT_BAND66_ROWS, SSO_LOC_AUDIT_CASES,
    SSO_LOC_AUDIT_CRITERIA, SSO_LOC_AUDIT_SLICES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1299_band_sso_loc_audit_close_ph_s1308() {
    assert_eq!(
        sso_loc_audit_depth_stub(Some(&json!({"sso_loc_audit_depth": true}))),
        SsoLocAuditDepth::DepthModule
    );
    assert_eq!(
        sso_loc_audit_depth_stub(Some(&json!({
            "sso_loc_audit_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "sso_loc_audit_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        SsoLocAuditDepth::FullBand66
    );

    assert_eq!(SSO_LOC_AUDIT_CRITERIA.len(), 10);
    assert_eq!(sso_loc_audit_criteria_total(), 10);
    assert!(SSO_LOC_AUDIT_CASES.contains(&"sso_loc_audit_docs"));
    assert_eq!(SSO_LOC_AUDIT_SLICES.len(), 5);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert_eq!(sso_loc_audit_slices_met(loc_audit), (5, 5));
    assert!(loc_audit.contains("sso_loc_audit_mode"));
    assert!(loc_audit.contains("sso_loc_audit_criteria_met_count"));
    assert!(loc_audit.contains("--sso-loc-audit"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND66_ROWS {
        assert!(fm.contains(row), "FM missing band-66 row {row}");
    }
    assert!(fm.contains("PH-S1308"));
    assert!(fm.contains("5.47"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1299") || handoff.contains("band 66"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 67"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--sso-loc-audit"));
    assert!(run_local.contains("VERIFY_SSO_LOC_AUDIT"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("sso_loc_audit_depth") || strategy.contains("band 66"));

    let sso_doc = include_str!("../docs/development/SSO_LOC_AUDIT.md");
    assert!(sso_doc.contains("--sso-loc-audit"));
    assert!(sso_doc.contains("SSO_LOC_AUDIT_SLICES") || sso_doc.contains("--sso-stand-smoke"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1299") || roadmap.contains("loc-audit"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_SSO_LOC_AUDIT"));
    assert!(verify.contains("--sso-loc-audit"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--sso-loc-audit"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("sso_loc_audit_band66_export_shape"));

    for marker in SSO_LOC_AUDIT_BAND66_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || sso_doc.contains(marker),
            "band-66 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/sso_loc_audit_depth.rs").exists());
    assert!(Path::new("docs/development/SSO_LOC_AUDIT.md").exists());
    assert!(Path::new("tests/sso_loc_audit_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("sso_loc_audit_mode").is_some());
}
