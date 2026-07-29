//! PH-S1601: Monitoring loc-audit aggregate contracts (band 96).
//! Marker: monitoring_loc_audit_integration
//!
//! Verifies band 91–95 slice flags are present and aggregate criteria totals are consistent.

use poolai_ui_core::monitoring_admin_ops_depth::monitoring_admin_ops_criteria_total;
use poolai_ui_core::monitoring_api_contracts_depth::monitoring_api_criteria_total;
use poolai_ui_core::monitoring_depth::monitoring_criteria_total;
use poolai_ui_core::monitoring_loc_audit_depth::{
    monitoring_loc_audit_criteria_total, monitoring_loc_audit_depth_stub,
    monitoring_loc_audit_slices_met, MonitoringLocAuditDepth, MONITORING_LOC_AUDIT_CASES,
    MONITORING_LOC_AUDIT_CRITERIA, MONITORING_LOC_AUDIT_SLICES,
};
use poolai_ui_core::monitoring_stand_smoke_depth::monitoring_stand_smoke_criteria_total;
use poolai_ui_core::monitoring_store_depth::monitoring_store_criteria_total;
use serde_json::json;

#[test]
fn monitoring_loc_audit_depth_registry_ph_s1599() {
    assert_eq!(MONITORING_LOC_AUDIT_CRITERIA.len(), 10);
    assert_eq!(monitoring_loc_audit_criteria_total(), 10);
    assert!(MONITORING_LOC_AUDIT_CASES.contains(&"aggregate_flag"));
    assert!(MONITORING_LOC_AUDIT_CASES.contains(&"slice_monitoring"));
    assert_eq!(
        monitoring_loc_audit_depth_stub(Some(&json!({"slice_aggregate": true}))),
        MonitoringLocAuditDepth::SliceAggregate
    );
}

#[test]
fn monitoring_loc_audit_slice_flags_present_ph_s1600() {
    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    let (met, total) = monitoring_loc_audit_slices_met(loc_audit);
    assert_eq!(total, 5);
    assert_eq!(
        met, 5,
        "all band 91–95 --monitoring* loc-audit slices must exist"
    );
    for flag in MONITORING_LOC_AUDIT_SLICES {
        assert!(
            loc_audit.contains(flag),
            "missing loc-audit slice flag {flag}"
        );
    }
    assert!(loc_audit.contains("--monitoring-loc-audit"));
}

#[test]
fn monitoring_loc_audit_criteria_totals_consistent_ph_s1601() {
    assert_eq!(monitoring_criteria_total(), 8);
    assert_eq!(monitoring_store_criteria_total(), 7);
    assert_eq!(monitoring_api_criteria_total(), 9);
    assert_eq!(monitoring_admin_ops_criteria_total(), 10);
    assert_eq!(monitoring_stand_smoke_criteria_total(), 10);
    assert_eq!(monitoring_loc_audit_criteria_total(), 10);

    let slice_total: usize = MONITORING_LOC_AUDIT_SLICES.len();
    assert_eq!(slice_total, 5);

    assert_eq!(
        monitoring_loc_audit_depth_stub(Some(&json!({
            "monitoring_loc_audit_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "monitoring_loc_audit_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        MonitoringLocAuditDepth::FullBand96
    );
}
