//! PH-S1228: Galaxy horizon close band 58 — tenant vision sync.
//! Suite: `galaxy_horizon_s1219_integration`.

use poolai_ui_core::tenant_vision_sync_depth::{
    tenant_vision_sync_criteria_total, tenant_vision_sync_depth_stub,
    tenant_vision_sync_slices_met, TenantVisionSyncDepth, FM_BAND58_ROWS,
    TENANT_VISION_SYNC_BAND58_ROWS, TENANT_VISION_SYNC_CASES, TENANT_VISION_SYNC_CRITERIA,
    TENANT_VISION_SYNC_SLICES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1219_band_tenant_vision_sync_close_ph_s1228() {
    assert_eq!(
        tenant_vision_sync_depth_stub(Some(&json!({"tenant_vision_sync_depth": true}))),
        TenantVisionSyncDepth::DepthModule
    );
    assert_eq!(
        tenant_vision_sync_depth_stub(Some(&json!({
            "tenant_vision_sync_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "tenant_vision_sync_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        TenantVisionSyncDepth::FullBand58
    );

    assert_eq!(TENANT_VISION_SYNC_CRITERIA.len(), 10);
    assert_eq!(tenant_vision_sync_criteria_total(), 10);
    assert!(TENANT_VISION_SYNC_CASES.contains(&"vision_manifest"));
    assert_eq!(TENANT_VISION_SYNC_SLICES.len(), 6);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("tenant_vision_sync_mode"));
    assert!(loc_audit.contains("tenant_vision_sync_criteria_met_count"));
    assert!(loc_audit.contains("--tenant-vision-sync"));

    let tenant_doc = include_str!("../docs/development/TENANT_VISION_SYNC.md");
    assert_eq!(tenant_vision_sync_slices_met(tenant_doc), (6, 6));
    assert!(tenant_doc.contains("--tenant-vision-sync"));
    assert!(
        tenant_doc.contains("TENANT_VISION_SYNC_SLICES") || tenant_doc.contains("manifest.json")
    );

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND58_ROWS {
        assert!(fm.contains(row), "FM missing band-58 row {row}");
    }
    assert!(fm.contains("PH-S1228"));
    assert!(fm.contains("5.39"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1219") || handoff.contains("band 58"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 59"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--tenant-vision-sync"));
    assert!(run_local.contains("VERIFY_TENANT_VISION_SYNC"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("tenant_vision_sync_depth") || strategy.contains("band 58"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1219") || roadmap.contains("vision-sync"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_TENANT_VISION_SYNC"));
    assert!(verify.contains("--tenant-vision-sync"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--tenant-vision-sync"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("tenant_vision_sync_band58_export_shape"));

    for marker in TENANT_VISION_SYNC_BAND58_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || tenant_doc.contains(marker),
            "band-58 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/tenant_vision_sync_depth.rs").exists());
    assert!(Path::new("docs/development/TENANT_VISION_SYNC.md").exists());
    assert!(Path::new("tests/tenant_vision_sync_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("tenant_vision_sync_mode").is_some());
}
