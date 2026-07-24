//! PH-S1398: Galaxy horizon close band 75 — Audit live stand smoke.
//! Suite: `galaxy_horizon_s1389_integration`.

use poolai_ui_core::audit_stand_smoke_depth::{
    audit_stand_smoke_criteria_total, audit_stand_smoke_depth_stub, AuditStandSmokeDepth,
    AUDIT_STAND_SMOKE_BAND75_ROWS, AUDIT_STAND_SMOKE_CASES, AUDIT_STAND_SMOKE_CRITERIA,
    FM_BAND75_ROWS,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1389_band_audit_stand_smoke_close_ph_s1398() {
    assert_eq!(
        audit_stand_smoke_depth_stub(Some(&json!({"audit_stand_smoke_depth": true}))),
        AuditStandSmokeDepth::DepthModule
    );
    assert_eq!(
        audit_stand_smoke_depth_stub(Some(&json!({
            "audit_stand_smoke_depth": true,
            "live_store": true,
            "live_events_query": true,
            "live_event_field_fixtures": true,
            "cli_flag": true,
            "loc_audit_flag": true,
            "verify_dev_stand_hook": true,
            "audit_stand_smoke_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        AuditStandSmokeDepth::FullBand75
    );

    assert_eq!(AUDIT_STAND_SMOKE_CRITERIA.len(), 10);
    assert_eq!(audit_stand_smoke_criteria_total(), 10);
    assert!(AUDIT_STAND_SMOKE_CASES.contains(&"audit_stand_smoke_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND75_ROWS {
        assert!(fm.contains(row), "FM missing band-75 row {row}");
    }
    assert!(fm.contains("PH-S1398"));
    assert!(fm.contains("5.56"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1389") || handoff.contains("band 75"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 76"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--audit-stand-smoke"));
    assert!(run_local.contains("VERIFY_AUDIT_STAND_SMOKE"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("audit_stand_smoke_depth") || strategy.contains("band 75"));

    let audit_doc = include_str!("../docs/development/AUDIT_STAND_SMOKE.md");
    assert!(audit_doc.contains("/api/enterprise/audit/store"));
    assert!(
        audit_doc.contains("smoke_audit_events_query") || audit_doc.contains("--audit-stand-smoke")
    );

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1389") || roadmap.contains("stand smoke"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_AUDIT_STAND_SMOKE"));
    assert!(verify.contains("--audit-stand-smoke"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--audit-stand-smoke"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("audit_stand_smoke_mode"));
    assert!(loc_audit.contains("audit_stand_smoke_criteria_met_count"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("smoke_audit_store_wire"));
    assert!(smoke.contains("smoke_audit_events_query"));
    assert!(smoke.contains("smoke_audit_event_field_fixtures"));
    assert!(smoke.contains("audit_stand_smoke_only"));

    for marker in AUDIT_STAND_SMOKE_BAND75_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker),
            "band-75 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/audit_stand_smoke_depth.rs").exists());
    assert!(Path::new("docs/development/AUDIT_STAND_SMOKE.md").exists());
    assert!(Path::new("tests/audit_stand_smoke_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("audit_stand_smoke_mode").is_some());
}
