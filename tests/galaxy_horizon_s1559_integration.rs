//! PH-S1568: Galaxy horizon close band 92 — Monitoring store wire.
//! Suite: `galaxy_horizon_s1559_integration`.

use poolai_ui_core::monitoring_store_depth::{
    monitoring_store_criteria_total, monitoring_store_depth_stub, MonitoringStoreDepth,
    FM_BAND92_ROWS, MONITORING_BAND92_ROWS, MONITORING_STORE_CASES, MONITORING_STORE_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1559_band_monitoring_store_close_ph_s1568() {
    assert_eq!(
        monitoring_store_depth_stub(Some(&json!({"monitoring_store_depth": true}))),
        MonitoringStoreDepth::DepthModule
    );
    assert_eq!(
        monitoring_store_depth_stub(Some(&json!({
            "monitoring_store_depth": true,
            "store_wire": true,
            "api_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "monitoring_store_docs": true,
        }))),
        MonitoringStoreDepth::FullBand92
    );

    assert_eq!(MONITORING_STORE_CRITERIA.len(), 7);
    assert_eq!(monitoring_store_criteria_total(), 7);
    assert!(MONITORING_STORE_CASES.contains(&"monitoring_store_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND92_ROWS {
        assert!(fm.contains(row), "FM missing band-92 row {row}");
    }
    assert!(fm.contains("PH-S1568"));
    assert!(fm.contains("5.73"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1559") || handoff.contains("band 92"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 93"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--monitoring-store"));
    assert!(run_local.contains("VERIFY_MONITORING_STORE"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("monitoring_store_depth") || strategy.contains("band 92"));

    let monitoring_doc = include_str!("../docs/development/MONITORING_STORE.md");
    assert!(monitoring_doc.contains("POOLAI_MONITORING_DATA_DIR"));
    assert!(monitoring_doc.contains("monitoring_store_wire"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1559") || roadmap.contains("Monitoring"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_MONITORING_STORE"));
    assert!(verify.contains("--monitoring-store"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--monitoring-store"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("monitoring_store_mode"));
    assert!(loc_audit.contains("monitoring_store_criteria_met_count"));

    let monitoring_mod = include_str!("../src/enterprise/monitoring.rs");
    assert!(monitoring_mod.contains("monitoring_store_wire"));
    assert!(monitoring_mod.contains("POOLAI_MONITORING_STORE"));

    for marker in MONITORING_BAND92_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || monitoring_doc.contains(marker)
                || verify.contains(marker),
            "band-92 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/monitoring_store_depth.rs").exists());
    assert!(Path::new("docs/development/MONITORING_STORE.md").exists());
    assert!(Path::new("docs/development/PH_S_MASTER_BACKLOG_1000.md").exists());
    assert!(Path::new("tests/monitoring_store_wire_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("monitoring_store_mode").is_some());
    assert!(ratio.get("monitoring_store_criteria_total").is_some());
}
