//! PH-S1131: Pre-push vision canon gate audit — criteria registry + maintenance markers.

use poolai_ui_core::pre_push_hook_depth::{
    pre_push_hook_criteria_total, pre_push_hook_depth_stub, PrePushHookDepth, FM_BAND49_ROWS,
    PRE_PUSH_HOOK_BAND49_ROWS, PRE_PUSH_HOOK_CASES, PRE_PUSH_HOOK_CRITERIA,
};
use serde_json::json;

#[test]
fn pre_push_hook_audit_ph_s1131() {
    assert_eq!(
        pre_push_hook_depth_stub(Some(&json!({"vision_sync_canon": true}))),
        PrePushHookDepth::VisionSyncCanon
    );
    assert_eq!(
        pre_push_hook_depth_stub(Some(&json!({
            "pre_push_hook_script": true,
            "install_hook": true,
            "vision_sync_canon": true,
            "vision_sync_check": true,
            "cargo_fmt_gate": true,
            "pre_push_hook_docs": true,
            "verify_dev_stand_hook": true,
        }))),
        PrePushHookDepth::FullBand49
    );

    assert_eq!(PRE_PUSH_HOOK_CRITERIA.len(), 7);
    assert_eq!(pre_push_hook_criteria_total(), 7);
    assert!(PRE_PUSH_HOOK_CASES.contains(&"vision_sync_check"));
    assert!(PRE_PUSH_HOOK_CASES.contains(&"cargo_fmt_gate"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND49_ROWS {
        assert!(
            fm.contains(row) || row.starts_with("PH-S"),
            "FM missing {row}"
        );
    }
    for marker in PRE_PUSH_HOOK_BAND49_ROWS {
        assert!(
            fm.contains(marker) || marker.starts_with("PH-S"),
            "band-49 marker missing: {marker}"
        );
    }

    let criteria_ids: Vec<_> = PRE_PUSH_HOOK_CRITERIA
        .iter()
        .map(|(id, _, _)| *id)
        .collect();
    assert!(criteria_ids.contains(&"pre_push_hook_script"));
    assert!(criteria_ids.contains(&"verify_dev_stand_hook"));
}
