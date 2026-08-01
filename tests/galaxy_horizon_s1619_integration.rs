//! PH-S1628: Galaxy horizon close band 98 — Monitoring vision sync.
//! Suite: `galaxy_horizon_s1619_integration`.

use poolai_ui_core::monitoring_vision_sync_depth::{
    monitoring_vision_sync_criteria_total, monitoring_vision_sync_depth_stub,
    monitoring_vision_sync_slices_met, MonitoringVisionSyncDepth, FM_BAND98_ROWS,
    MONITORING_VISION_SYNC_BAND98_ROWS, MONITORING_VISION_SYNC_CASES,
    MONITORING_VISION_SYNC_CRITERIA, MONITORING_VISION_SYNC_SLICES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1619_band_monitoring_vision_sync_close_ph_s1628() {
    assert_eq!(
        monitoring_vision_sync_depth_stub(Some(&json!({"monitoring_vision_sync_depth": true}))),
        MonitoringVisionSyncDepth::DepthModule
    );
    assert_eq!(
        monitoring_vision_sync_depth_stub(Some(&json!({
            "monitoring_vision_sync_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "monitoring_vision_sync_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        MonitoringVisionSyncDepth::FullBand98
    );

    assert_eq!(MONITORING_VISION_SYNC_CRITERIA.len(), 10);
    assert_eq!(monitoring_vision_sync_criteria_total(), 10);
    assert!(MONITORING_VISION_SYNC_CASES.contains(&"vision_manifest"));
    assert_eq!(MONITORING_VISION_SYNC_SLICES.len(), 6);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("monitoring_vision_sync_mode"));
    assert!(loc_audit.contains("monitoring_vision_sync_criteria_met_count"));
    assert!(loc_audit.contains("--monitoring-vision-sync"));

    let monitoring_doc = include_str!("../docs/development/MONITORING_VISION_SYNC.md");
    assert_eq!(monitoring_vision_sync_slices_met(monitoring_doc), (6, 6));
    assert!(monitoring_doc.contains("--monitoring-vision-sync"));
    assert!(
        monitoring_doc.contains("MONITORING_VISION_SYNC_SLICES")
            || monitoring_doc.contains("MONITORING_DOCS_CANON.md")
    );

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND98_ROWS {
        assert!(fm.contains(row), "FM missing band-98 row {row}");
    }
    assert!(fm.contains("PH-S1628"));
    assert!(fm.contains("5.79"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1619") || handoff.contains("band 98"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 99"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--monitoring-vision-sync"));
    assert!(run_local.contains("VERIFY_MONITORING_VISION_SYNC"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("monitoring_vision_sync_depth") || strategy.contains("band 98"));

    let roadmap = include_str!("../docs/development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md");
    assert!(roadmap.contains("PH-S1619") || roadmap.contains("vision-sync"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_MONITORING_VISION_SYNC"));
    assert!(verify.contains("--monitoring-vision-sync"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--monitoring-vision-sync"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("monitoring_vision_sync_band98_export_shape"));

    for marker in MONITORING_VISION_SYNC_BAND98_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || monitoring_doc.contains(marker),
            "band-98 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/monitoring_vision_sync_depth.rs").exists());
    assert!(Path::new("docs/development/MONITORING_VISION_SYNC.md").exists());
    assert!(Path::new("tests/monitoring_vision_sync_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("monitoring_vision_sync_mode").is_some());
}
