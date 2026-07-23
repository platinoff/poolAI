//! PH-S1358: Galaxy horizon close band 71 — Audit depth scaffold.
//! Suite: `galaxy_horizon_s1349_integration`.

use poolai_ui_core::audit_depth::{
    audit_criteria_total, audit_depth_stub, AuditDepth, AUDIT_BAND71_ROWS, AUDIT_CASES,
    AUDIT_CRITERIA, FM_BAND71_ROWS,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1349_band_audit_depth_close_ph_s1358() {
    assert_eq!(
        audit_depth_stub(Some(&json!({"audit_depth": true}))),
        AuditDepth::DepthModule
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
    assert!(AUDIT_CASES.contains(&"audit_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND71_ROWS {
        assert!(fm.contains(row), "FM missing band-71 row {row}");
    }
    assert!(fm.contains("PH-S1358"));
    assert!(fm.contains("5.52"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1349") || handoff.contains("band 71"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 72"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--audit"));
    assert!(run_local.contains("VERIFY_AUDIT"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("audit_depth") || strategy.contains("band 71"));

    let audit_doc = include_str!("../docs/development/AUDIT_DEPTH.md");
    assert!(audit_doc.contains("POOLAI_AUDIT_STORE"));
    assert!(audit_doc.contains("audit_depth"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1349") || roadmap.contains("Audit"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_AUDIT"));
    assert!(verify.contains("--audit"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--audit"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("audit_mode"));
    assert!(loc_audit.contains("audit_criteria_met_count"));

    let audit_mod = include_str!("../src/enterprise/audit.rs");
    assert!(audit_mod.contains("POOLAI_AUDIT_STORE"));
    assert!(audit_mod.contains("validate_audit_event_fields"));

    for marker in AUDIT_BAND71_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || audit_doc.contains(marker)
                || verify.contains(marker),
            "band-71 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/audit_depth.rs").exists());
    assert!(Path::new("docs/development/AUDIT_DEPTH.md").exists());
    assert!(Path::new("docs/development/PH_S_MASTER_BACKLOG_1000.md").exists());
    assert!(Path::new("tests/audit_depth_audit.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("audit_mode").is_some());
    assert!(ratio.get("audit_criteria_total").is_some());
}
