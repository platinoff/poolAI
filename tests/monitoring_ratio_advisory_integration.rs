//! PH-S1631: Monitoring ratio-advisory contracts (band 99).
//! Marker: monitoring_ratio_advisory_integration
//!
//! Verifies ratio slices + prior MONITORING_VISION_SYNC are present and criteria totals are consistent.

use poolai_ui_core::monitoring_ratio_advisory_depth::{
    monitoring_ratio_advisory_criteria_total, monitoring_ratio_advisory_depth_stub,
    monitoring_ratio_advisory_slices_met, MonitoringRatioAdvisoryDepth,
    MONITORING_RATIO_ADVISORY_CASES, MONITORING_RATIO_ADVISORY_CRITERIA,
    MONITORING_RATIO_ADVISORY_SLICES,
};
use poolai_ui_core::monitoring_vision_sync_depth::monitoring_vision_sync_criteria_total;
use serde_json::json;

#[test]
fn monitoring_ratio_advisory_depth_registry_ph_s1629() {
    assert_eq!(MONITORING_RATIO_ADVISORY_CRITERIA.len(), 10);
    assert_eq!(monitoring_ratio_advisory_criteria_total(), 10);
    assert!(MONITORING_RATIO_ADVISORY_CASES.contains(&"aggregate_flag"));
    assert!(MONITORING_RATIO_ADVISORY_CASES.contains(&"ratio_json"));
    assert_eq!(
        monitoring_ratio_advisory_depth_stub(Some(&json!({"slice_aggregate": true}))),
        MonitoringRatioAdvisoryDepth::SliceAggregate
    );
}

#[test]
fn monitoring_ratio_advisory_slice_docs_present_ph_s1630() {
    let canon = include_str!("../docs/development/MONITORING_RATIO_ADVISORY.md");
    let (met, total) = monitoring_ratio_advisory_slices_met(canon);
    assert_eq!(total, 6);
    assert_eq!(met, 6, "all ratio-advisory slices must be listed");
    for name in MONITORING_RATIO_ADVISORY_SLICES {
        assert!(canon.contains(name), "missing ratio-advisory slice {name}");
    }
    assert!(std::path::Path::new("docs/development/rust_ratio.json").exists());
    assert!(std::path::Path::new("docs/development/RUST_RATIO_STRATEGY_2026-06-13.md").exists());
    assert!(std::path::Path::new("docs/development/MONITORING_VISION_SYNC.md").exists());
    assert!(std::path::Path::new("src/bin/poolai_loc_audit.rs").exists());
    assert!(std::path::Path::new("bin/run-poolai.sh").exists());
    assert!(std::path::Path::new("bin/verify-dev-stand.sh").exists());
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--monitoring-ratio-advisory"));
    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_MONITORING_RATIO_ADVISORY"));
}

#[test]
fn monitoring_ratio_advisory_criteria_totals_consistent_ph_s1631() {
    assert_eq!(monitoring_vision_sync_criteria_total(), 10);
    assert_eq!(monitoring_ratio_advisory_criteria_total(), 10);
    assert_eq!(MONITORING_RATIO_ADVISORY_SLICES.len(), 6);

    assert_eq!(
        monitoring_ratio_advisory_depth_stub(Some(&json!({
            "monitoring_ratio_advisory_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "monitoring_ratio_advisory_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        MonitoringRatioAdvisoryDepth::FullBand99
    );
}
