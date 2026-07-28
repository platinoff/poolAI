//! PH-S1578: Galaxy horizon close band 93 — Monitoring HTTP API contracts.
//! Suite: `galaxy_horizon_s1569_integration`.

use poolai_ui_core::monitoring_api_contracts_depth::{
    monitoring_api_contracts_depth_stub, monitoring_api_criteria_total,
    MonitoringApiContractsDepth, FM_BAND93_ROWS, MONITORING_API_BAND93_ROWS, MONITORING_API_CASES,
    MONITORING_API_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1569_band_monitoring_api_close_ph_s1578() {
    assert_eq!(
        monitoring_api_contracts_depth_stub(Some(&json!({"monitoring_api_depth": true}))),
        MonitoringApiContractsDepth::DepthModule
    );
    assert_eq!(
        monitoring_api_contracts_depth_stub(Some(&json!({
            "monitoring_api_depth": true,
            "query_http_lifecycle": true,
            "store_wire_http": true,
            "openapi_schemas": true,
            "monitoring_field_fixtures": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "monitoring_api_docs": true,
        }))),
        MonitoringApiContractsDepth::FullBand93
    );

    assert_eq!(MONITORING_API_CRITERIA.len(), 9);
    assert_eq!(monitoring_api_criteria_total(), 9);
    assert!(MONITORING_API_CASES.contains(&"monitoring_api_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND93_ROWS {
        assert!(fm.contains(row), "FM missing band-93 row {row}");
    }
    assert!(fm.contains("PH-S1578"));
    assert!(fm.contains("5.74"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1569") || handoff.contains("band 93"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 94"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--monitoring-api"));
    assert!(run_local.contains("VERIFY_MONITORING_API"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("monitoring_api") || strategy.contains("band 93"));

    let monitoring_doc = include_str!("../docs/development/MONITORING_API.md");
    assert!(monitoring_doc.contains("/api/enterprise/monitoring"));
    assert!(monitoring_doc.contains("MonitoringStoreWire"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_MONITORING_API"));
    assert!(verify.contains("--monitoring-api"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--monitoring-api"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("monitoring_api_mode"));
    assert!(loc_audit.contains("monitoring_api_criteria_met_count"));

    let monitoring_api = include_str!("../src/network/enterprise_api/monitoring.rs");
    assert!(monitoring_api.contains("monitoring_store_wire_handler"));
    assert!(monitoring_api.contains("GET /monitoring/store"));

    let openapi = include_str!("../docs/openapi.yaml");
    assert!(openapi.contains("MonitoringStoreWire"));
    assert!(openapi.contains("/monitoring/store"));

    for marker in MONITORING_API_BAND93_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || monitoring_api.contains(marker)
                || openapi.contains(marker)
                || verify.contains(marker),
            "band-93 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/monitoring_api_contracts_depth.rs").exists());
    assert!(Path::new("docs/development/MONITORING_API.md").exists());
    assert!(Path::new("tests/monitoring_api_contracts_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("monitoring_api_mode").is_some());
    assert!(ratio.get("monitoring_api_criteria_total").is_some());
}
