//! PH-S1588: Galaxy horizon close band 94 — Monitoring admin/ops glue.
//! Suite: `galaxy_horizon_s1579_integration`.

use poolai_ui_core::monitoring_admin_ops_depth::{
    monitoring_admin_ops_criteria_total, monitoring_admin_ops_depth_stub, MonitoringAdminOpsDepth,
    FM_BAND94_ROWS, MONITORING_ADMIN_OPS_BAND94_ROWS, MONITORING_ADMIN_OPS_CASES,
    MONITORING_ADMIN_OPS_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1579_band_monitoring_admin_ops_close_ph_s1588() {
    assert_eq!(
        monitoring_admin_ops_depth_stub(Some(&json!({"monitoring_admin_ops_depth": true}))),
        MonitoringAdminOpsDepth::DepthModule
    );
    assert_eq!(
        monitoring_admin_ops_depth_stub(Some(&json!({
            "monitoring_admin_ops_depth": true,
            "store_strip": true,
            "query_ops_glue": true,
            "html_contracts": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "monitoring_admin_ops_docs": true,
            "ratio_hold": true,
            "band_close": true,
        }))),
        MonitoringAdminOpsDepth::FullBand94
    );

    assert_eq!(MONITORING_ADMIN_OPS_CRITERIA.len(), 10);
    assert_eq!(monitoring_admin_ops_criteria_total(), 10);
    assert!(MONITORING_ADMIN_OPS_CASES.contains(&"monitoring_admin_ops_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND94_ROWS {
        assert!(fm.contains(row), "FM missing band-94 row {row}");
    }
    assert!(fm.contains("PH-S1588"));
    assert!(fm.contains("5.75"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1579") || handoff.contains("band 94"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 95"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--monitoring-admin-ops"));
    assert!(run_local.contains("VERIFY_MONITORING_ADMIN_OPS"));

    let mon_doc = include_str!("../docs/development/MONITORING_ADMIN_OPS.md");
    assert!(mon_doc.contains("monitoring-store-badge"));
    assert!(mon_doc.contains("/api/enterprise/monitoring/store"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_MONITORING_ADMIN_OPS"));
    assert!(verify.contains("--monitoring-admin-ops"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--monitoring-admin-ops"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("monitoring_admin_ops_mode"));
    assert!(loc_audit.contains("monitoring_admin_ops_criteria_met_count"));

    let mon_ui = include_str!("../src/ui/admin/monitoring.rs");
    assert!(mon_ui.contains("monitoring-store-badge"));
    assert!(mon_ui.contains("refreshMonitoring"));

    for marker in MONITORING_ADMIN_OPS_BAND94_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || mon_ui.contains(marker)
                || verify.contains(marker)
                || mon_doc.contains(marker),
            "band-94 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/monitoring_admin_ops_depth.rs").exists());
    assert!(Path::new("docs/development/MONITORING_ADMIN_OPS.md").exists());
    assert!(Path::new("tests/monitoring_admin_ops_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("monitoring_admin_ops_mode").is_some());
    assert!(ratio.get("monitoring_admin_ops_criteria_total").is_some());
}
