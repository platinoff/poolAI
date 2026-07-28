//! PH-S1558: Galaxy horizon close band 91 — Monitoring depth scaffold.
//! Suite: `galaxy_horizon_s1549_integration`.

use poolai_ui_core::monitoring_depth::{
    monitoring_criteria_total, monitoring_depth_stub, MonitoringDepth, FM_BAND91_ROWS,
    MONITORING_BAND91_ROWS, MONITORING_CASES, MONITORING_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1549_band_monitoring_depth_close_ph_s1558() {
    assert_eq!(
        monitoring_depth_stub(Some(&json!({"monitoring_depth": true}))),
        MonitoringDepth::DepthModule
    );
    assert_eq!(
        monitoring_depth_stub(Some(&json!({
            "monitoring_depth": true,
            "store_wire": true,
            "api_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "monitoring_docs": true,
        }))),
        MonitoringDepth::FullBand91
    );

    assert_eq!(MONITORING_CRITERIA.len(), 8);
    assert_eq!(monitoring_criteria_total(), 8);
    assert!(MONITORING_CASES.contains(&"monitoring_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND91_ROWS {
        assert!(fm.contains(row), "FM missing band-91 row {row}");
    }
    assert!(fm.contains("PH-S1558"));
    assert!(fm.contains("5.72"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1549") || handoff.contains("band 91"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 92"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--monitoring"));
    assert!(run_local.contains("VERIFY_MONITORING"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("monitoring_depth") || strategy.contains("band 91"));

    let monitoring_doc = include_str!("../docs/development/MONITORING_DEPTH.md");
    assert!(monitoring_doc.contains("POOLAI_MONITORING_DATA_DIR"));
    assert!(monitoring_doc.contains("monitoring_depth"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1549") || roadmap.contains("Monitoring"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_MONITORING"));
    assert!(verify.contains("--monitoring"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--monitoring"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("monitoring_mode"));
    assert!(loc_audit.contains("monitoring_criteria_met_count"));

    let monitoring_mod = include_str!("../src/enterprise/monitoring.rs");
    assert!(monitoring_mod.contains("POOLAI_MONITORING_DATA_DIR"));
    assert!(monitoring_mod.contains("validate_monitoring_alert_fields"));

    for marker in MONITORING_BAND91_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || monitoring_doc.contains(marker)
                || verify.contains(marker),
            "band-91 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/monitoring_depth.rs").exists());
    assert!(Path::new("docs/development/MONITORING_DEPTH.md").exists());
    assert!(Path::new("docs/development/PH_S_MASTER_BACKLOG_1000.md").exists());
    assert!(Path::new("tests/monitoring_depth_audit.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("monitoring_mode").is_some());
    assert!(ratio.get("monitoring_criteria_total").is_some());
}
