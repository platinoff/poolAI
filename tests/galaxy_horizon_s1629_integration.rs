//! PH-S1638: Galaxy horizon close band 99 — Monitoring ratio advisory.
//! Suite: `galaxy_horizon_s1629_integration`.

use poolai_ui_core::monitoring_ratio_advisory_depth::{
    monitoring_ratio_advisory_criteria_total, monitoring_ratio_advisory_depth_stub,
    monitoring_ratio_advisory_slices_met, MonitoringRatioAdvisoryDepth, FM_BAND99_ROWS,
    MONITORING_RATIO_ADVISORY_BAND99_ROWS, MONITORING_RATIO_ADVISORY_CASES,
    MONITORING_RATIO_ADVISORY_CRITERIA, MONITORING_RATIO_ADVISORY_SLICES,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1629_band_monitoring_ratio_advisory_close_ph_s1638() {
    assert_eq!(
        monitoring_ratio_advisory_depth_stub(Some(&json!({
            "monitoring_ratio_advisory_depth": true
        }))),
        MonitoringRatioAdvisoryDepth::DepthModule
    );
    assert_eq!(
        monitoring_ratio_advisory_depth_stub(Some(&json!({
            "monitoring_ratio_advisory_depth": true,
            "slice_aggregate": true,
            "criteria_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "monitoring_ratio_advisory_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        MonitoringRatioAdvisoryDepth::FullBand99
    );

    assert_eq!(MONITORING_RATIO_ADVISORY_CRITERIA.len(), 10);
    assert_eq!(monitoring_ratio_advisory_criteria_total(), 10);
    assert!(MONITORING_RATIO_ADVISORY_CASES.contains(&"ratio_json"));
    assert_eq!(MONITORING_RATIO_ADVISORY_SLICES.len(), 6);

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("monitoring_ratio_advisory_mode"));
    assert!(loc_audit.contains("monitoring_ratio_advisory_criteria_met_count"));
    assert!(loc_audit.contains("--monitoring-ratio-advisory"));

    let ratio_doc = include_str!("../docs/development/MONITORING_RATIO_ADVISORY.md");
    assert_eq!(monitoring_ratio_advisory_slices_met(ratio_doc), (6, 6));
    assert!(ratio_doc.contains("--monitoring-ratio-advisory"));
    assert!(
        ratio_doc.contains("MONITORING_RATIO_ADVISORY_SLICES")
            || ratio_doc.contains("rust_ratio.json")
    );

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND99_ROWS {
        assert!(fm.contains(row), "FM missing band-99 row {row}");
    }
    assert!(fm.contains("PH-S1638"));
    assert!(fm.contains("5.80"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1629") || handoff.contains("band 99"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 100"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--monitoring-ratio-advisory"));
    assert!(run_local.contains("VERIFY_MONITORING_RATIO_ADVISORY"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("monitoring_ratio_advisory_depth") || strategy.contains("band 99"));

    let roadmap = include_str!("../docs/development/PH_S_PROJECT_COMPLETION_ROADMAP_2026-07-22.md");
    assert!(roadmap.contains("PH-S1629") || roadmap.contains("ratio-advisory"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_MONITORING_RATIO_ADVISORY"));
    assert!(verify.contains("--monitoring-ratio-advisory"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--monitoring-ratio-advisory"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("monitoring_ratio_advisory_band99_export_shape"));

    for marker in MONITORING_RATIO_ADVISORY_BAND99_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || ratio_doc.contains(marker),
            "band-99 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/monitoring_ratio_advisory_depth.rs").exists());
    assert!(Path::new("docs/development/MONITORING_RATIO_ADVISORY.md").exists());
    assert!(Path::new("tests/monitoring_ratio_advisory_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("monitoring_ratio_advisory_mode").is_some());
}
