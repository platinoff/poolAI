//! PH-S1351: Audit depth gate audit — criteria registry + maintenance markers.
//! Marker: audit_depth_audit

use poolai_ui_core::audit_depth::{
    audit_criteria_total, audit_depth_stub, AuditDepth, AUDIT_BAND71_ROWS, AUDIT_CASES,
    AUDIT_CRITERIA, AUDIT_STORE_ENV, FM_BAND71_ROWS,
};
use serde_json::json;

#[test]
fn audit_depth_audit_ph_s1351() {
    assert_eq!(
        audit_depth_stub(Some(&json!({"loc_audit_flag": true}))),
        AuditDepth::LocAuditFlag
    );
    assert_eq!(
        audit_depth_stub(Some(&json!({
            "audit_depth": true,
            "store_wire": true,
            "api_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "audit_docs": true,
        }))),
        AuditDepth::FullBand71
    );

    assert_eq!(AUDIT_CRITERIA.len(), 8);
    assert_eq!(audit_criteria_total(), 8);
    assert!(AUDIT_CASES.contains(&"store_wire"));
    assert_eq!(AUDIT_STORE_ENV, "POOLAI_AUDIT_STORE");

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND71_ROWS {
        assert!(
            fm.contains(row) || row.starts_with("PH-S"),
            "FM missing {row}"
        );
    }
    for marker in AUDIT_BAND71_ROWS {
        assert!(
            fm.contains(marker) || marker.starts_with("PH-S"),
            "band-71 marker missing: {marker}"
        );
    }

    let criteria_ids: Vec<_> = AUDIT_CRITERIA.iter().map(|(id, _, _)| *id).collect();
    assert!(criteria_ids.contains(&"audit_depth"));
    assert!(criteria_ids.contains(&"verify_dev_stand_hook"));
}
