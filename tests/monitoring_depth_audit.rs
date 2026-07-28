//! PH-S1551: Monitoring depth gate audit — criteria registry + maintenance markers.
//! Marker: monitoring_depth_audit

use poolai_ui_core::monitoring_depth::{
    monitoring_criteria_total, monitoring_depth_stub, MonitoringDepth, FM_BAND91_ROWS,
    MONITORING_BAND91_ROWS, MONITORING_CASES, MONITORING_CRITERIA, MONITORING_STORE_ENV,
};
use serde_json::json;

#[test]
fn monitoring_depth_audit_ph_s1551() {
    assert_eq!(
        monitoring_depth_stub(Some(&json!({"loc_audit_flag": true}))),
        MonitoringDepth::LocAuditFlag
    );
    assert_eq!(
        monitoring_depth_stub(Some(&json!({
            "monitoring_depth": true,
            "store_wire": true,
            "api_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "monitoring_docs": true,
        }))),
        MonitoringDepth::FullBand91
    );

    assert_eq!(MONITORING_CRITERIA.len(), 8);
    assert_eq!(monitoring_criteria_total(), 8);
    assert!(MONITORING_CASES.contains(&"store_wire"));
    assert_eq!(MONITORING_STORE_ENV, "POOLAI_MONITORING_DATA_DIR");

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND91_ROWS {
        assert!(
            fm.contains(row) || row.starts_with("PH-S"),
            "FM missing {row}"
        );
    }
    for marker in MONITORING_BAND91_ROWS {
        assert!(
            fm.contains(marker) || marker.starts_with("PH-S"),
            "band-91 marker missing: {marker}"
        );
    }

    let criteria_ids: Vec<_> = MONITORING_CRITERIA.iter().map(|(id, _, _)| *id).collect();
    assert!(criteria_ids.contains(&"monitoring_depth"));
    assert!(criteria_ids.contains(&"verify_dev_stand_hook"));
}
