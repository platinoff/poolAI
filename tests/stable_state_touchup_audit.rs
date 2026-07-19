//! PH-S1111: STABLE state touch-up audit — criteria registry + maintenance markers.

use poolai_ui_core::stable_state_touchup_depth::{
    stable_criteria_total, stable_state_touchup_depth_stub, StableStateTouchupDepth,
    FM_BAND47_ROWS, STABLE_TOUCHUP_BAND47_ROWS, STABLE_TOUCHUP_CASES, STABLE_TOUCHUP_CRITERIA,
};
use serde_json::json;

#[test]
fn stable_state_touchup_audit_ph_s1111() {
    assert_eq!(
        stable_state_touchup_depth_stub(Some(&json!({"stable_summary": true}))),
        StableStateTouchupDepth::StableSummary
    );
    assert_eq!(
        stable_state_touchup_depth_stub(Some(&json!({
            "criteria_registry": true,
            "stable_summary": true,
            "index_canon": true,
            "handoff_zriz": true,
            "loc_audit_touchup": true,
            "verify_dev_stand_hook": true,
            "quick_touchup": true,
            "docs_canon": true,
        }))),
        StableStateTouchupDepth::FullBand47
    );

    assert_eq!(STABLE_TOUCHUP_CRITERIA.len(), 7);
    assert_eq!(stable_criteria_total(), 7);
    assert!(STABLE_TOUCHUP_CASES.contains(&"product_complete"));
    assert!(STABLE_TOUCHUP_CASES.contains(&"vision_sync"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND47_ROWS {
        assert!(
            fm.contains(row) || row.starts_with("PH-S"),
            "FM missing {row}"
        );
    }
    for marker in STABLE_TOUCHUP_BAND47_ROWS {
        assert!(
            fm.contains(marker) || marker.starts_with("PH-S"),
            "band-47 marker missing: {marker}"
        );
    }

    let stable = include_str!("../docs/status/STABLE_STATE_SUMMARY.md");
    assert!(stable.contains("product-complete") || stable.contains("§5.15"));
    assert!(stable.contains("maintenance"));

    let criteria_ids: Vec<_> = STABLE_TOUCHUP_CRITERIA
        .iter()
        .map(|(id, _, _)| *id)
        .collect();
    assert!(criteria_ids.contains(&"docs_canon"));
    assert!(criteria_ids.contains(&"ops_test_ci"));
}
