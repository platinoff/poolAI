//! PH-S1608: Galaxy horizon close band 96 — Monitoring loc-audit aggregate.
//! Suite: `galaxy_horizon_s1599_integration`.

use poolai_ui_core::monitoring_loc_audit_depth::{
    monitoring_loc_audit_criteria_total, monitoring_loc_audit_depth_stub,
    monitoring_loc_audit_slices_met, MonitoringLocAuditDepth, FM_BAND96_ROWS,
    MONITORING_LOC_AUDIT_BAND96_ROWS, MONITORING_LOC_AUDIT_CASES, MONITORING_LOC_AUDIT_CRITERIA,
    MONITORING_LOC_AUDIT_SLICES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1599_band_monitoring_loc_audit_close_ph_s1608() {
    assert_eq!(
        monitoring_loc_audit_depth_stub(Some(&json!({"monitoring_loc_audit_depth": true}))),
        MonitoringLocAuditDepth::DepthModule
    );
    assert_eq!(
        monitoring_loc_audit_depth_stub(Some(&json!({
            "monitoring_loc_audit_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "monitoring_loc_audit_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        MonitoringLocAuditDepth::FullBand96
    );

    assert_eq!(MONITORING_LOC_AUDIT_CRITERIA.len(), 10);
    assert_eq!(monitoring_loc_audit_criteria_total(), 10);
    assert!(MONITORING_LOC_AUDIT_CASES.contains(&"monitoring_loc_audit_docs"));
    assert_eq!(MONITORING_LOC_AUDIT_SLICES.len(), 5);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert_eq!(monitoring_loc_audit_slices_met(loc_audit), (5, 5));
    assert!(loc_audit.contains("monitoring_loc_audit_mode"));
    assert!(loc_audit.contains("monitoring_loc_audit_criteria_met_count"));
    assert!(loc_audit.contains("--monitoring-loc-audit"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND96_ROWS {
        assert!(fm.contains(row), "FM missing band-96 row {row}");
    }
    assert!(fm.contains("PH-S1608"));
    assert!(fm.contains("5.77"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1599") || handoff.contains("band 96"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 97"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--monitoring-loc-audit"));
    assert!(run_local.contains("VERIFY_MONITORING_LOC_AUDIT"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("monitoring_loc_audit_depth") || strategy.contains("band 96"));

    let monitoring_doc = include_str!("../docs/development/MONITORING_LOC_AUDIT.md");
    assert!(monitoring_doc.contains("--monitoring-loc-audit"));
    assert!(
        monitoring_doc.contains("MONITORING_LOC_AUDIT_SLICES")
            || monitoring_doc.contains("--monitoring-stand-smoke")
    );

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1599") || roadmap.contains("loc-audit"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_MONITORING_LOC_AUDIT"));
    assert!(verify.contains("--monitoring-loc-audit"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--monitoring-loc-audit"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("monitoring_loc_audit_band96_export_shape"));

    for marker in MONITORING_LOC_AUDIT_BAND96_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || monitoring_doc.contains(marker),
            "band-96 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/monitoring_loc_audit_depth.rs").exists());
    assert!(Path::new("docs/development/MONITORING_LOC_AUDIT.md").exists());
    assert!(Path::new("tests/monitoring_loc_audit_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("monitoring_loc_audit_mode").is_some());
}
