//! PH-S1328: Galaxy horizon close band 68 — SSO vision sync.
//! Suite: `galaxy_horizon_s1319_integration`.

use poolai_ui_core::sso_vision_sync_depth::{
    sso_vision_sync_criteria_total, sso_vision_sync_depth_stub, sso_vision_sync_slices_met,
    SsoVisionSyncDepth, FM_BAND68_ROWS, SSO_VISION_SYNC_BAND68_ROWS, SSO_VISION_SYNC_CASES,
    SSO_VISION_SYNC_CRITERIA, SSO_VISION_SYNC_SLICES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1319_band_sso_vision_sync_close_ph_s1328() {
    assert_eq!(
        sso_vision_sync_depth_stub(Some(&json!({"sso_vision_sync_depth": true}))),
        SsoVisionSyncDepth::DepthModule
    );
    assert_eq!(
        sso_vision_sync_depth_stub(Some(&json!({
            "sso_vision_sync_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "sso_vision_sync_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        SsoVisionSyncDepth::FullBand68
    );

    assert_eq!(SSO_VISION_SYNC_CRITERIA.len(), 10);
    assert_eq!(sso_vision_sync_criteria_total(), 10);
    assert!(SSO_VISION_SYNC_CASES.contains(&"vision_manifest"));
    assert_eq!(SSO_VISION_SYNC_SLICES.len(), 6);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("sso_vision_sync_mode"));
    assert!(loc_audit.contains("sso_vision_sync_criteria_met_count"));
    assert!(loc_audit.contains("--sso-vision-sync"));

    let sso_doc = include_str!("../docs/development/SSO_VISION_SYNC.md");
    assert_eq!(sso_vision_sync_slices_met(sso_doc), (6, 6));
    assert!(sso_doc.contains("--sso-vision-sync"));
    assert!(sso_doc.contains("SSO_VISION_SYNC_SLICES") || sso_doc.contains("SSO_DOCS_CANON.md"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND68_ROWS {
        assert!(fm.contains(row), "FM missing band-68 row {row}");
    }
    assert!(fm.contains("PH-S1328"));
    assert!(fm.contains("5.49"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1319") || handoff.contains("band 68"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 69"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--sso-vision-sync"));
    assert!(run_local.contains("VERIFY_SSO_VISION_SYNC"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("sso_vision_sync_depth") || strategy.contains("band 68"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1319") || roadmap.contains("vision-sync"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_SSO_VISION_SYNC"));
    assert!(verify.contains("--sso-vision-sync"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--sso-vision-sync"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("sso_vision_sync_band68_export_shape"));

    for marker in SSO_VISION_SYNC_BAND68_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || sso_doc.contains(marker),
            "band-68 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/sso_vision_sync_depth.rs").exists());
    assert!(Path::new("docs/development/SSO_VISION_SYNC.md").exists());
    assert!(Path::new("tests/sso_vision_sync_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("sso_vision_sync_mode").is_some());
}
