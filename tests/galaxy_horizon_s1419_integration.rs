//! PH-S1428: Galaxy horizon close band 78 — Audit vision sync.
//! Suite: `galaxy_horizon_s1419_integration`.

use poolai_ui_core::audit_vision_sync_depth::{
    audit_vision_sync_criteria_total, audit_vision_sync_depth_stub, audit_vision_sync_slices_met,
    AuditVisionSyncDepth, AUDIT_VISION_SYNC_BAND78_ROWS, AUDIT_VISION_SYNC_CASES,
    AUDIT_VISION_SYNC_CRITERIA, AUDIT_VISION_SYNC_SLICES, FM_BAND78_ROWS,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1419_band_audit_vision_sync_close_ph_s1428() {
    assert_eq!(
        audit_vision_sync_depth_stub(Some(&json!({"audit_vision_sync_depth": true}))),
        AuditVisionSyncDepth::DepthModule
    );
    assert_eq!(
        audit_vision_sync_depth_stub(Some(&json!({
            "audit_vision_sync_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "audit_vision_sync_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        AuditVisionSyncDepth::FullBand78
    );

    assert_eq!(AUDIT_VISION_SYNC_CRITERIA.len(), 10);
    assert_eq!(audit_vision_sync_criteria_total(), 10);
    assert!(AUDIT_VISION_SYNC_CASES.contains(&"vision_manifest"));
    assert_eq!(AUDIT_VISION_SYNC_SLICES.len(), 6);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("audit_vision_sync_mode"));
    assert!(loc_audit.contains("audit_vision_sync_criteria_met_count"));
    assert!(loc_audit.contains("--audit-vision-sync"));

    let audit_doc = include_str!("../docs/development/AUDIT_VISION_SYNC.md");
    assert_eq!(audit_vision_sync_slices_met(audit_doc), (6, 6));
    assert!(audit_doc.contains("--audit-vision-sync"));
    assert!(
        audit_doc.contains("AUDIT_VISION_SYNC_SLICES") || audit_doc.contains("AUDIT_DOCS_CANON.md")
    );

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND78_ROWS {
        assert!(fm.contains(row), "FM missing band-78 row {row}");
    }
    assert!(fm.contains("PH-S1428"));
    assert!(fm.contains("5.59"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1419") || handoff.contains("band 78"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 79"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--audit-vision-sync"));
    assert!(run_local.contains("VERIFY_AUDIT_VISION_SYNC"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("audit_vision_sync_depth") || strategy.contains("band 78"));

    let roadmap = include_str!("../docs/development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md");
    assert!(roadmap.contains("PH-S1419") || roadmap.contains("vision-sync"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_AUDIT_VISION_SYNC"));
    assert!(verify.contains("--audit-vision-sync"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--audit-vision-sync"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("audit_vision_sync_band78_export_shape"));

    for marker in AUDIT_VISION_SYNC_BAND78_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || audit_doc.contains(marker),
            "band-78 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/audit_vision_sync_depth.rs").exists());
    assert!(Path::new("docs/development/AUDIT_VISION_SYNC.md").exists());
    assert!(Path::new("tests/audit_vision_sync_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("audit_vision_sync_mode").is_some());
}
