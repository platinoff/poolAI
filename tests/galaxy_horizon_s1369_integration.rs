//! PH-S1378: Galaxy horizon close band 73 — Audit HTTP API contracts.
//! Suite: `galaxy_horizon_s1369_integration`.

use poolai_ui_core::audit_api_contracts_depth::{
    audit_api_contracts_depth_stub, audit_api_criteria_total, AuditApiContractsDepth,
    AUDIT_API_BAND73_ROWS, AUDIT_API_CASES, AUDIT_API_CRITERIA, FM_BAND73_ROWS,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1369_band_audit_api_close_ph_s1378() {
    assert_eq!(
        audit_api_contracts_depth_stub(Some(&json!({"audit_api_depth": true}))),
        AuditApiContractsDepth::DepthModule
    );
    assert_eq!(
        audit_api_contracts_depth_stub(Some(&json!({
            "audit_api_depth": true,
            "query_http_lifecycle": true,
            "store_wire_http": true,
            "openapi_schemas": true,
            "event_field_fixtures": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "audit_api_docs": true,
        }))),
        AuditApiContractsDepth::FullBand73
    );

    assert_eq!(AUDIT_API_CRITERIA.len(), 9);
    assert_eq!(audit_api_criteria_total(), 9);
    assert!(AUDIT_API_CASES.contains(&"audit_api_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND73_ROWS {
        assert!(fm.contains(row), "FM missing band-73 row {row}");
    }
    assert!(fm.contains("PH-S1378"));
    assert!(fm.contains("5.54"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1369") || handoff.contains("band 73"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 74"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--audit-api"));
    assert!(run_local.contains("VERIFY_AUDIT_API"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("audit_api") || strategy.contains("band 73"));

    let audit_doc = include_str!("../docs/development/AUDIT_API.md");
    assert!(audit_doc.contains("/api/enterprise/audit"));
    assert!(audit_doc.contains("AuditStoreWire"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1369") || roadmap.contains("Audit"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_AUDIT_API"));
    assert!(verify.contains("--audit-api"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--audit-api"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("audit_api_mode"));
    assert!(loc_audit.contains("audit_api_criteria_met_count"));

    let audit_api = include_str!("../src/network/enterprise_api/audit.rs");
    assert!(audit_api.contains("audit_store_wire_handler"));
    assert!(audit_api.contains("GET /audit/store"));

    let openapi = include_str!("../docs/openapi.yaml");
    assert!(openapi.contains("AuditStoreWire"));
    assert!(openapi.contains("/audit/store"));

    for marker in AUDIT_API_BAND73_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || audit_api.contains(marker)
                || openapi.contains(marker)
                || verify.contains(marker),
            "band-73 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/audit_api_contracts_depth.rs").exists());
    assert!(Path::new("docs/development/AUDIT_API.md").exists());
    assert!(Path::new("tests/audit_api_contracts_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("audit_api_mode").is_some());
    assert!(ratio.get("audit_api_criteria_total").is_some());
}
