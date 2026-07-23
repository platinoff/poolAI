//! PH-S1368: Galaxy horizon close band 72 — Audit store wire.
//! Suite: `galaxy_horizon_s1359_integration`.

use poolai_ui_core::audit_store_depth::{
    audit_store_criteria_total, audit_store_depth_stub, AuditStoreDepth, AUDIT_BAND72_ROWS,
    AUDIT_STORE_CASES, AUDIT_STORE_CRITERIA, FM_BAND72_ROWS,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1359_band_audit_store_close_ph_s1368() {
    assert_eq!(
        audit_store_depth_stub(Some(&json!({"audit_store_depth": true}))),
        AuditStoreDepth::DepthModule
    );
    assert_eq!(
        audit_store_depth_stub(Some(&json!({
            "audit_store_depth": true,
            "store_wire": true,
            "api_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "audit_store_docs": true,
        }))),
        AuditStoreDepth::FullBand72
    );

    assert_eq!(AUDIT_STORE_CRITERIA.len(), 7);
    assert_eq!(audit_store_criteria_total(), 7);
    assert!(AUDIT_STORE_CASES.contains(&"audit_store_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND72_ROWS {
        assert!(fm.contains(row), "FM missing band-72 row {row}");
    }
    assert!(fm.contains("PH-S1368"));
    assert!(fm.contains("5.53"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1359") || handoff.contains("band 72"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 73"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--audit-store"));
    assert!(run_local.contains("VERIFY_AUDIT_STORE"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("audit_store") || strategy.contains("band 72"));

    let audit_doc = include_str!("../docs/development/AUDIT_STORE.md");
    assert!(audit_doc.contains("POOLAI_AUDIT_STORE"));
    assert!(audit_doc.contains("audit_store_wire"));
    assert!(audit_doc.contains("POOLAI_AUDIT_DATA_DIR"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1359") || roadmap.contains("Audit"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_AUDIT_STORE"));
    assert!(verify.contains("--audit-store"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--audit-store"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("audit_store_mode"));
    assert!(loc_audit.contains("audit_store_criteria_met_count"));

    let audit_mod = include_str!("../src/enterprise/audit.rs");
    assert!(audit_mod.contains("POOLAI_AUDIT_STORE"));
    assert!(audit_mod.contains("audit_store_wire"));
    assert!(audit_mod.contains("POOLAI_AUDIT_DATA_DIR"));

    for marker in AUDIT_BAND72_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || audit_doc.contains(marker)
                || verify.contains(marker),
            "band-72 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/audit_store_depth.rs").exists());
    assert!(Path::new("docs/development/AUDIT_STORE.md").exists());
    assert!(Path::new("docs/development/PH_S_MASTER_BACKLOG_1000.md").exists());
    assert!(Path::new("tests/audit_store_wire_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("audit_store_mode").is_some());
    assert!(ratio.get("audit_store_criteria_total").is_some());
}
