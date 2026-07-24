//! PH-S1438: Galaxy horizon close band 79 — Audit ratio advisory.
//! Suite: `galaxy_horizon_s1429_integration`.

use poolai_ui_core::audit_ratio_advisory_depth::{
    audit_ratio_advisory_criteria_total, audit_ratio_advisory_depth_stub,
    audit_ratio_advisory_slices_met, AuditRatioAdvisoryDepth, AUDIT_RATIO_ADVISORY_BAND79_ROWS,
    AUDIT_RATIO_ADVISORY_CASES, AUDIT_RATIO_ADVISORY_CRITERIA, AUDIT_RATIO_ADVISORY_SLICES,
    FM_BAND79_ROWS,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1429_band_audit_ratio_advisory_close_ph_s1438() {
    assert_eq!(
        audit_ratio_advisory_depth_stub(Some(&json!({"audit_ratio_advisory_depth": true}))),
        AuditRatioAdvisoryDepth::DepthModule
    );
    assert_eq!(
        audit_ratio_advisory_depth_stub(Some(&json!({
            "audit_ratio_advisory_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "audit_ratio_advisory_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        AuditRatioAdvisoryDepth::FullBand79
    );

    assert_eq!(AUDIT_RATIO_ADVISORY_CRITERIA.len(), 10);
    assert_eq!(audit_ratio_advisory_criteria_total(), 10);
    assert!(AUDIT_RATIO_ADVISORY_CASES.contains(&"doc_vision_sync"));
    assert_eq!(AUDIT_RATIO_ADVISORY_SLICES.len(), 6);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("audit_ratio_advisory_mode"));
    assert!(loc_audit.contains("audit_ratio_advisory_criteria_met_count"));
    assert!(loc_audit.contains("--audit-ratio-advisory"));

    let audit_doc = include_str!("../docs/development/AUDIT_RATIO_ADVISORY.md");
    assert_eq!(audit_ratio_advisory_slices_met(audit_doc), (6, 6));
    assert!(audit_doc.contains("--audit-ratio-advisory"));
    assert!(
        audit_doc.contains("AUDIT_RATIO_ADVISORY_SLICES")
            || audit_doc.contains("AUDIT_VISION_SYNC.md")
    );

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND79_ROWS {
        assert!(fm.contains(row), "FM missing band-79 row {row}");
    }
    assert!(fm.contains("PH-S1438"));
    assert!(fm.contains("5.60"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1429") || handoff.contains("band 79"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 80"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--audit-ratio-advisory"));
    assert!(run_local.contains("VERIFY_AUDIT_RATIO_ADVISORY"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("audit_ratio_advisory_depth") || strategy.contains("band 79"));

    let roadmap = include_str!("../docs/development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md");
    assert!(
        roadmap.contains("PH-S1429")
            || roadmap.contains("ratio-advisory")
            || roadmap.contains("ratio hold advisory")
    );

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_AUDIT_RATIO_ADVISORY"));
    assert!(verify.contains("--audit-ratio-advisory"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--audit-ratio-advisory"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("audit_ratio_advisory_band79_export_shape"));

    for marker in AUDIT_RATIO_ADVISORY_BAND79_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || audit_doc.contains(marker),
            "band-79 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/audit_ratio_advisory_depth.rs").exists());
    assert!(Path::new("docs/development/AUDIT_RATIO_ADVISORY.md").exists());
    assert!(Path::new("tests/audit_ratio_advisory_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("audit_ratio_advisory_mode").is_some());
}
