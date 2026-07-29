//! PH-S1598: Galaxy horizon close band 95 — Monitoring stand smoke.
//! Suite: `galaxy_horizon_s1589_integration`.

use poolai_ui_core::monitoring_stand_smoke_depth::{
    monitoring_stand_smoke_criteria_total, monitoring_stand_smoke_depth_stub,
    MonitoringStandSmokeDepth, FM_BAND95_ROWS, MONITORING_STAND_SMOKE_BAND95_ROWS,
    MONITORING_STAND_SMOKE_CASES, MONITORING_STAND_SMOKE_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1589_band_monitoring_stand_smoke_close_ph_s1598() {
    assert_eq!(
        monitoring_stand_smoke_depth_stub(Some(&json!({"monitoring_stand_smoke_depth": true}))),
        MonitoringStandSmokeDepth::DepthModule
    );
    assert_eq!(
        monitoring_stand_smoke_depth_stub(Some(&json!({
            "monitoring_stand_smoke_depth": true,
            "live_store": true,
            "live_alerts_query": true,
            "live_monitoring_field_fixtures": true,
            "cli_flag": true,
            "loc_audit_flag": true,
            "verify_dev_stand_hook": true,
            "monitoring_stand_smoke_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        MonitoringStandSmokeDepth::FullBand95
    );

    assert_eq!(MONITORING_STAND_SMOKE_CRITERIA.len(), 10);
    assert_eq!(monitoring_stand_smoke_criteria_total(), 10);
    assert!(MONITORING_STAND_SMOKE_CASES.contains(&"monitoring_stand_smoke_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND95_ROWS {
        assert!(fm.contains(row), "FM missing band-95 row {row}");
    }
    assert!(fm.contains("PH-S1598"));
    assert!(fm.contains("5.76"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1589") || handoff.contains("band 95"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 96"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--monitoring-stand-smoke"));
    assert!(run_local.contains("VERIFY_MONITORING_STAND_SMOKE"));

    let mon_doc = include_str!("../docs/development/MONITORING_STAND_SMOKE.md");
    assert!(mon_doc.contains("/api/enterprise/monitoring/store"));
    assert!(mon_doc.contains("--monitoring-stand-smoke"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_MONITORING_STAND_SMOKE"));
    assert!(verify.contains("--monitoring-stand-smoke"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--monitoring-stand-smoke"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("monitoring_stand_smoke_mode"));
    assert!(loc_audit.contains("monitoring_stand_smoke_criteria_met_count"));

    let stand_smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(stand_smoke.contains("smoke_monitoring_store_wire"));
    assert!(stand_smoke.contains("smoke_monitoring_alerts_query"));
    assert!(stand_smoke.contains("smoke_monitoring_field_fixtures"));

    for marker in MONITORING_STAND_SMOKE_BAND95_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || stand_smoke.contains(marker)
                || verify.contains(marker)
                || mon_doc.contains(marker),
            "band-95 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/monitoring_stand_smoke_depth.rs").exists());
    assert!(Path::new("docs/development/MONITORING_STAND_SMOKE.md").exists());
    assert!(Path::new("tests/monitoring_stand_smoke_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("monitoring_stand_smoke_mode").is_some());
    assert!(ratio.get("monitoring_stand_smoke_criteria_total").is_some());
}
