//! PH-S1648: Galaxy horizon close band 100 — Monitoring horizon close.
//! Suite: `galaxy_horizon_s1639_integration`.

use poolai_ui_core::monitoring_horizon_depth::{
    monitoring_horizon_criteria_total, monitoring_horizon_depth_stub,
    monitoring_horizon_slices_met, MonitoringHorizonDepth, FM_BAND100_ROWS,
    MONITORING_HORIZON_BAND100_ROWS, MONITORING_HORIZON_CASES, MONITORING_HORIZON_CRITERIA,
    MONITORING_HORIZON_SLICES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1639_band_monitoring_horizon_close_ph_s1648() {
    assert_eq!(
        monitoring_horizon_depth_stub(Some(&json!({"monitoring_horizon_depth": true}))),
        MonitoringHorizonDepth::DepthModule
    );
    assert_eq!(
        monitoring_horizon_depth_stub(Some(&json!({
            "monitoring_horizon_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "monitoring_horizon_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        MonitoringHorizonDepth::FullBand100
    );

    assert_eq!(MONITORING_HORIZON_CRITERIA.len(), 10);
    assert_eq!(monitoring_horizon_criteria_total(), 10);
    assert!(MONITORING_HORIZON_CASES.contains(&"doc_ratio_advisory"));
    assert_eq!(MONITORING_HORIZON_SLICES.len(), 10);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("monitoring_horizon_mode"));
    assert!(loc_audit.contains("monitoring_horizon_criteria_met_count"));
    assert!(loc_audit.contains("--monitoring-horizon"));

    let monitoring_doc = include_str!("../docs/development/MONITORING_HORIZON.md");
    assert_eq!(monitoring_horizon_slices_met(monitoring_doc), (10, 10));
    assert!(monitoring_doc.contains("--monitoring-horizon"));
    assert!(monitoring_doc.contains("MONITORING_HORIZON_SLICES"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND100_ROWS {
        assert!(fm.contains(row), "FM missing band-100 row {row}");
    }
    assert!(fm.contains("PH-S1648"));
    assert!(fm.contains("5.81"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1639") || handoff.contains("band 100"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 101"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--monitoring-horizon"));
    assert!(run_local.contains("VERIFY_MONITORING_HORIZON"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("monitoring_horizon_depth") || strategy.contains("band 100"));

    let roadmap = include_str!("../docs/development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md");
    assert!(
        roadmap.contains("PH-S1639")
            || roadmap.contains("horizon close")
            || roadmap.contains("Monitoring")
    );

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_MONITORING_HORIZON"));
    assert!(verify.contains("--monitoring-horizon"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--monitoring-horizon"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("monitoring_horizon_band100_export_shape"));

    for marker in MONITORING_HORIZON_BAND100_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || monitoring_doc.contains(marker),
            "band-100 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/monitoring_horizon_depth.rs").exists());
    assert!(Path::new("docs/development/MONITORING_HORIZON.md").exists());
    assert!(Path::new("tests/monitoring_horizon_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("monitoring_horizon_mode").is_some());
}
