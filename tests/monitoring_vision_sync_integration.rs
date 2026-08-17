//! PH-S1621: Monitoring vision-sync contracts (band 98).
//! Marker: monitoring_vision_sync_integration
//!
//! Verifies vision slices + prior MONITORING_DOCS_CANON are present and criteria totals are consistent.

use poolai_ui_core::monitoring_docs_canon_depth::monitoring_docs_canon_criteria_total;
use poolai_ui_core::monitoring_vision_sync_depth::{
    monitoring_vision_sync_criteria_total, monitoring_vision_sync_depth_stub,
    monitoring_vision_sync_slices_met, MonitoringVisionSyncDepth, MONITORING_VISION_SYNC_CASES,
    MONITORING_VISION_SYNC_CRITERIA, MONITORING_VISION_SYNC_SLICES,
};
use serde_json::json;

#[test]
fn monitoring_vision_sync_depth_registry_ph_s1619() {
    assert_eq!(MONITORING_VISION_SYNC_CRITERIA.len(), 10);
    assert_eq!(monitoring_vision_sync_criteria_total(), 10);
    assert!(MONITORING_VISION_SYNC_CASES.contains(&"aggregate_flag"));
    assert!(MONITORING_VISION_SYNC_CASES.contains(&"vision_manifest"));
    assert_eq!(
        monitoring_vision_sync_depth_stub(Some(&json!({"slice_aggregate": true}))),
        MonitoringVisionSyncDepth::SliceAggregate
    );
}

#[test]
fn monitoring_vision_sync_slice_docs_present_ph_s1620() {
    let canon = include_str!("../docs/development/MONITORING_VISION_SYNC.md");
    let (met, total) = monitoring_vision_sync_slices_met(canon);
    assert_eq!(total, 6);
    assert_eq!(met, 6, "all vision-sync slices must be listed");
    for name in MONITORING_VISION_SYNC_SLICES {
        assert!(canon.contains(name), "missing vision-sync slice {name}");
    }
    assert!(std::path::Path::new("docs/vision/manifest.json").exists());
    assert!(std::path::Path::new("docs/vision/extensions.json").exists());
    assert!(std::path::Path::new("docs/vision/README.md").exists());
    assert!(std::path::Path::new("docs/vision/vision.svg").exists());
    assert!(std::path::Path::new("docs/vision/index.html").exists());
    assert!(std::path::Path::new("docs/development/MONITORING_DOCS_CANON.md").exists());
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--monitoring-vision-sync"));
}

#[test]
fn monitoring_vision_sync_criteria_totals_consistent_ph_s1621() {
    assert_eq!(monitoring_docs_canon_criteria_total(), 10);
    assert_eq!(monitoring_vision_sync_criteria_total(), 10);
    assert_eq!(MONITORING_VISION_SYNC_SLICES.len(), 6);

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
}
