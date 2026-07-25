//! PH-S1448: Galaxy horizon close band 80 — Audit horizon.
//! Suite: `galaxy_horizon_s1439_integration`.

use poolai_ui_core::audit_horizon_depth::{
    audit_horizon_criteria_total, audit_horizon_depth_stub, audit_horizon_slices_met,
    AuditHorizonDepth, AUDIT_HORIZON_BAND80_ROWS, AUDIT_HORIZON_CASES, AUDIT_HORIZON_CRITERIA,
    AUDIT_HORIZON_SLICES, FM_BAND80_ROWS,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1439_band_audit_horizon_close_ph_s1448() {
    assert_eq!(
        audit_horizon_depth_stub(Some(&json!({"audit_horizon_depth": true}))),
        AuditHorizonDepth::DepthModule
    );
    assert_eq!(
        audit_horizon_depth_stub(Some(&json!({
            "audit_horizon_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "audit_horizon_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        AuditHorizonDepth::FullBand80
    );

    assert_eq!(AUDIT_HORIZON_CRITERIA.len(), 10);
    assert_eq!(audit_horizon_criteria_total(), 10);
    assert!(AUDIT_HORIZON_CASES.contains(&"doc_ratio_advisory"));
    assert_eq!(AUDIT_HORIZON_SLICES.len(), 10);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("audit_horizon_mode"));
    assert!(loc_audit.contains("audit_horizon_criteria_met_count"));
    assert!(loc_audit.contains("--audit-horizon"));

    let audit_doc = include_str!("../docs/development/AUDIT_HORIZON.md");
    assert_eq!(audit_horizon_slices_met(audit_doc), (10, 10));
    assert!(audit_doc.contains("--audit-horizon"));
    assert!(
        audit_doc.contains("AUDIT_HORIZON_SLICES") || audit_doc.contains("AUDIT_RATIO_ADVISORY.md")
    );

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND80_ROWS {
        assert!(fm.contains(row), "FM missing band-80 row {row}");
    }
    assert!(fm.contains("PH-S1448"));
    assert!(fm.contains("5.61"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1439") || handoff.contains("band 80"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 81"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--audit-horizon"));
    assert!(run_local.contains("VERIFY_AUDIT_HORIZON"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("audit_horizon_depth") || strategy.contains("band 80"));

    let roadmap = include_str!("../docs/development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md");
    assert!(
        roadmap.contains("PH-S1439")
            || roadmap.contains("horizon close")
            || roadmap.contains("Audit")
    );

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_AUDIT_HORIZON"));
    assert!(verify.contains("--audit-horizon"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--audit-horizon"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("audit_horizon_band80_export_shape"));

    for marker in AUDIT_HORIZON_BAND80_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || audit_doc.contains(marker),
            "band-80 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/audit_horizon_depth.rs").exists());
    assert!(Path::new("docs/development/AUDIT_HORIZON.md").exists());
    assert!(Path::new("tests/audit_horizon_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("audit_horizon_mode").is_some());
}
