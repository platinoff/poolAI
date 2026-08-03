//! PH-S1707: Galaxy horizon close band 106 — Ratio96 loc-audit.
//! Suite: `galaxy_horizon_s1699_integration`.

use poolai_ui_core::ratio96_loc_audit_depth::{
    ratio96_loc_audit_criteria_total, ratio96_loc_audit_depth_stub, Ratio96LocAuditDepth,
    FM_BAND106_ROWS, RATIO96_LOC_AUDIT_BAND106_ROWS, RATIO96_LOC_AUDIT_CASES,
    RATIO96_LOC_AUDIT_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1699_band_ratio96_loc_audit_close_ph_s1707() {
    assert_eq!(
        ratio96_loc_audit_depth_stub(Some(&json!({"ratio96_loc_audit_depth": true}))),
        Ratio96LocAuditDepth::DepthModule
    );
    assert_eq!(
        ratio96_loc_audit_depth_stub(Some(&json!({
            "ratio96_loc_audit_depth": true,
            "loc_audit_smoke": true,
            "migration_advisory": true,
            "export_shape": true,
            "loc_audit_flag": true,
            "docs_canon": true,
            "vision_sync": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        Ratio96LocAuditDepth::FullBand106
    );

    assert_eq!(RATIO96_LOC_AUDIT_CRITERIA.len(), 10);
    assert_eq!(ratio96_loc_audit_criteria_total(), 10);
    assert!(RATIO96_LOC_AUDIT_CASES.contains(&"loc_audit_smoke"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND106_ROWS {
        assert!(fm.contains(row), "FM missing band-106 row {row}");
    }
    assert!(fm.contains("PH-S1707"));
    assert!(fm.contains("5.87"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1699") || handoff.contains("band 106"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 107"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--ratio96-loc-audit"));
    assert!(run_local.contains("VERIFY_RATIO96_LOC_AUDIT"));

    let ratio_doc = include_str!("../docs/development/RATIO96_LOC_AUDIT.md");
    assert!(ratio_doc.contains("smoke_ratio96_loc_audit"));
    assert!(ratio_doc.contains("/api/v1/ops/ratio96"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_RATIO96_LOC_AUDIT"));
    assert!(verify.contains("--ratio96-loc-audit"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--ratio96-loc-audit"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("ratio96_loc_audit_mode"));
    assert!(loc_audit.contains("ratio96_loc_audit_criteria_met_count"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("smoke_ratio96_loc_audit"));
    assert!(smoke.contains("smoke_ratio96_migration_advisory"));
    assert!(smoke.contains("ratio96_loc_audit_band106_export_shape"));

    for marker in RATIO96_LOC_AUDIT_BAND106_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || ratio_doc.contains(marker),
            "band-106 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/ratio96_loc_audit_depth.rs").exists());
    assert!(Path::new("docs/development/RATIO96_LOC_AUDIT.md").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("ratio96_loc_audit_mode").is_some());
    assert!(ratio.get("ratio96_loc_audit_criteria_total").is_some());
}
