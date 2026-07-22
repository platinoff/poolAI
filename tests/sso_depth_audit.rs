//! PH-S1251: SSO depth gate audit — criteria registry + maintenance markers.

use poolai_ui_core::sso_depth::{
    sso_criteria_total, sso_depth_stub, SsoDepth, FM_BAND61_ROWS, SSO_BAND61_ROWS, SSO_CASES,
    SSO_CRITERIA, SSO_STORE_ENV,
};
use serde_json::json;

#[test]
fn sso_depth_audit_ph_s1251() {
    assert_eq!(
        sso_depth_stub(Some(&json!({"loc_audit_flag": true}))),
        SsoDepth::LocAuditFlag
    );
    assert_eq!(
        sso_depth_stub(Some(&json!({
            "sso_depth": true,
            "store_wire": true,
            "api_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "sso_docs": true,
        }))),
        SsoDepth::FullBand61
    );

    assert_eq!(SSO_CRITERIA.len(), 8);
    assert_eq!(sso_criteria_total(), 8);
    assert!(SSO_CASES.contains(&"store_wire"));
    assert_eq!(SSO_STORE_ENV, "POOLAI_SSO_STORE");

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND61_ROWS {
        assert!(
            fm.contains(row) || row.starts_with("PH-S"),
            "FM missing {row}"
        );
    }
    for marker in SSO_BAND61_ROWS {
        assert!(
            fm.contains(marker) || marker.starts_with("PH-S"),
            "band-61 marker missing: {marker}"
        );
    }

    let criteria_ids: Vec<_> = SSO_CRITERIA.iter().map(|(id, _, _)| *id).collect();
    assert!(criteria_ids.contains(&"sso_depth"));
    assert!(criteria_ids.contains(&"verify_dev_stand_hook"));
}
