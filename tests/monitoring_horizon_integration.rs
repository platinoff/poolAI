//! PH-S1641: Monitoring horizon-close contracts (band 100).
//! Marker: monitoring_horizon_integration
//!
//! Verifies phase-E Monitoring slices + horizon criteria totals.

use poolai_ui_core::monitoring_horizon_depth::{
    monitoring_horizon_criteria_total, monitoring_horizon_depth_stub,
    monitoring_horizon_slices_met, MonitoringHorizonDepth, MONITORING_HORIZON_CASES,
    MONITORING_HORIZON_CRITERIA, MONITORING_HORIZON_SLICES,
};
use poolai_ui_core::monitoring_ratio_advisory_depth::monitoring_ratio_advisory_criteria_total;
use serde_json::json;

#[test]
fn monitoring_horizon_depth_registry_ph_s1639() {
    assert_eq!(MONITORING_HORIZON_CRITERIA.len(), 10);
    assert_eq!(monitoring_horizon_criteria_total(), 10);
    assert!(MONITORING_HORIZON_CASES.contains(&"aggregate_flag"));
    assert!(MONITORING_HORIZON_CASES.contains(&"phase_e_slices"));
    assert_eq!(
        monitoring_horizon_depth_stub(Some(&json!({"slice_aggregate": true}))),
        MonitoringHorizonDepth::SliceAggregate
    );
}

#[test]
fn monitoring_horizon_slice_docs_present_ph_s1640() {
    let canon = include_str!("../docs/development/MONITORING_HORIZON.md");
    let (met, total) = monitoring_horizon_slices_met(canon);
    assert_eq!(total, 10);
    assert_eq!(met, 10, "all horizon slices must be listed");
    for name in MONITORING_HORIZON_SLICES {
        assert!(canon.contains(name), "missing horizon slice {name}");
    }

    assert!(std::path::Path::new("docs/development/MONITORING_RATIO_ADVISORY.md").exists());
    assert!(std::path::Path::new("docs/development/MONITORING_LOC_AUDIT.md").exists());
    assert!(std::path::Path::new("docs/development/MONITORING_STORE.md").exists());

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("--monitoring-horizon"));
    assert!(loc_audit.contains("--monitoring-ratio-advisory"));
}

#[test]
fn monitoring_horizon_criteria_totals_consistent_ph_s1641() {
    assert_eq!(monitoring_ratio_advisory_criteria_total(), 10);
    assert_eq!(monitoring_horizon_criteria_total(), 10);
    assert_eq!(MONITORING_HORIZON_SLICES.len(), 10);

    assert_eq!(
        monitoring_horizon_depth_stub(Some(&json!({
            "monitoring_horizon_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "monitoring_horizon_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        MonitoringHorizonDepth::FullBand100
    );
}
