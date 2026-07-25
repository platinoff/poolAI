//! PH-S1451: Policy depth gate audit — criteria registry + maintenance markers.
//! Marker: policy_depth_audit

use poolai_ui_core::policy_depth::{
    policy_criteria_total, policy_depth_stub, PolicyDepth, FM_BAND81_ROWS, POLICY_BAND81_ROWS,
    POLICY_CASES, POLICY_CRITERIA, POLICY_STORE_ENV,
};
use serde_json::json;

#[test]
fn policy_depth_audit_ph_s1451() {
    assert_eq!(
        policy_depth_stub(Some(&json!({"loc_audit_flag": true}))),
        PolicyDepth::LocAuditFlag
    );
    assert_eq!(
        policy_depth_stub(Some(&json!({
            "policy_depth": true,
            "store_wire": true,
            "api_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "policy_docs": true,
        }))),
        PolicyDepth::FullBand81
    );

    assert_eq!(POLICY_CRITERIA.len(), 8);
    assert_eq!(policy_criteria_total(), 8);
    assert!(POLICY_CASES.contains(&"store_wire"));
    assert_eq!(POLICY_STORE_ENV, "POOLAI_POLICY_STORE");

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND81_ROWS {
        assert!(
            fm.contains(row) || row.starts_with("PH-S"),
            "FM missing {row}"
        );
    }
    for marker in POLICY_BAND81_ROWS {
        assert!(
            fm.contains(marker) || marker.starts_with("PH-S"),
            "band-81 marker missing: {marker}"
        );
    }

    let criteria_ids: Vec<_> = POLICY_CRITERIA.iter().map(|(id, _, _)| *id).collect();
    assert!(criteria_ids.contains(&"policy_depth"));
    assert!(criteria_ids.contains(&"verify_dev_stand_hook"));
}
