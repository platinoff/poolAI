//! PH-S1408: Galaxy horizon close band 76 — Audit loc-audit aggregate.
//! Suite: `galaxy_horizon_s1399_integration`.

use poolai_ui_core::audit_loc_audit_depth::{
    audit_loc_audit_criteria_total, audit_loc_audit_depth_stub, audit_loc_audit_slices_met,
    AuditLocAuditDepth, AUDIT_LOC_AUDIT_BAND76_ROWS, AUDIT_LOC_AUDIT_CASES,
    AUDIT_LOC_AUDIT_CRITERIA, AUDIT_LOC_AUDIT_SLICES, FM_BAND76_ROWS,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1399_band_audit_loc_audit_close_ph_s1408() {
    assert_eq!(
        audit_loc_audit_depth_stub(Some(&json!({"audit_loc_audit_depth": true}))),
        AuditLocAuditDepth::DepthModule
    );
    assert_eq!(
        audit_loc_audit_depth_stub(Some(&json!({
            "audit_loc_audit_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "audit_loc_audit_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        AuditLocAuditDepth::FullBand76
    );

    assert_eq!(AUDIT_LOC_AUDIT_CRITERIA.len(), 10);
    assert_eq!(audit_loc_audit_criteria_total(), 10);
    assert!(AUDIT_LOC_AUDIT_CASES.contains(&"audit_loc_audit_docs"));
    assert_eq!(AUDIT_LOC_AUDIT_SLICES.len(), 5);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert_eq!(audit_loc_audit_slices_met(loc_audit), (5, 5));
    assert!(loc_audit.contains("audit_loc_audit_mode"));
    assert!(loc_audit.contains("audit_loc_audit_criteria_met_count"));
    assert!(loc_audit.contains("--audit-loc-audit"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND76_ROWS {
        assert!(fm.contains(row), "FM missing band-76 row {row}");
    }
    assert!(fm.contains("PH-S1408"));
    assert!(fm.contains("5.57"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1399") || handoff.contains("band 76"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 77"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--audit-loc-audit"));
    assert!(run_local.contains("VERIFY_AUDIT_LOC_AUDIT"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("audit_loc_audit_depth") || strategy.contains("band 76"));

    let audit_doc = include_str!("../docs/development/AUDIT_LOC_AUDIT.md");
    assert!(audit_doc.contains("--audit-loc-audit"));
    assert!(
        audit_doc.contains("AUDIT_LOC_AUDIT_SLICES") || audit_doc.contains("--audit-stand-smoke")
    );

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1399") || roadmap.contains("loc-audit"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_AUDIT_LOC_AUDIT"));
    assert!(verify.contains("--audit-loc-audit"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--audit-loc-audit"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("audit_loc_audit_band76_export_shape"));

    for marker in AUDIT_LOC_AUDIT_BAND76_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || audit_doc.contains(marker),
            "band-76 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/audit_loc_audit_depth.rs").exists());
    assert!(Path::new("docs/development/AUDIT_LOC_AUDIT.md").exists());
    assert!(Path::new("tests/audit_loc_audit_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("audit_loc_audit_mode").is_some());
}
