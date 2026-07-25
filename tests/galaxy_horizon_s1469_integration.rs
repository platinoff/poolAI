//! PH-S1478: Galaxy horizon close band 83 — Policies HTTP API contracts.
//! Suite: `galaxy_horizon_s1469_integration`.

use poolai_ui_core::policy_api_contracts_depth::{
    policy_api_contracts_depth_stub, policy_api_criteria_total, PolicyApiContractsDepth,
    FM_BAND83_ROWS, POLICY_API_BAND83_ROWS, POLICY_API_CASES, POLICY_API_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1469_band_policies_api_close_ph_s1478() {
    assert_eq!(
        policy_api_contracts_depth_stub(Some(&json!({"policy_api_depth": true}))),
        PolicyApiContractsDepth::DepthModule
    );
    assert_eq!(
        policy_api_contracts_depth_stub(Some(&json!({
            "policy_api_depth": true,
            "query_http_lifecycle": true,
            "store_wire_http": true,
            "openapi_schemas": true,
            "policy_field_fixtures": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "policy_api_docs": true,
        }))),
        PolicyApiContractsDepth::FullBand83
    );

    assert_eq!(POLICY_API_CRITERIA.len(), 9);
    assert_eq!(policy_api_criteria_total(), 9);
    assert!(POLICY_API_CASES.contains(&"policy_api_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND83_ROWS {
        assert!(fm.contains(row), "FM missing band-83 row {row}");
    }
    assert!(fm.contains("PH-S1478"));
    assert!(fm.contains("5.64"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1469") || handoff.contains("band 83"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 84"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--policy-api"));
    assert!(run_local.contains("VERIFY_POLICY_API"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("policy_api") || strategy.contains("band 83"));

    let policy_doc = include_str!("../docs/development/POLICIES_API.md");
    assert!(policy_doc.contains("/api/enterprise/policy"));
    assert!(policy_doc.contains("PolicyStoreWire"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1469") || roadmap.contains("Policies"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_POLICY_API"));
    assert!(verify.contains("--policy-api"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--policy-api"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("policy_api_mode"));
    assert!(loc_audit.contains("policy_api_criteria_met_count"));

    let security_api = include_str!("../src/network/enterprise_api/security.rs");
    assert!(security_api.contains("policy_store_wire_handler"));
    assert!(security_api.contains("GET /policy/store"));

    let openapi = include_str!("../docs/openapi.yaml");
    assert!(openapi.contains("PolicyStoreWire"));
    assert!(openapi.contains("/policy/store"));

    for marker in POLICY_API_BAND83_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || security_api.contains(marker)
                || openapi.contains(marker)
                || verify.contains(marker),
            "band-83 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/policy_api_contracts_depth.rs").exists());
    assert!(Path::new("docs/development/POLICIES_API.md").exists());
    assert!(Path::new("tests/policy_api_contracts_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("policy_api_mode").is_some());
    assert!(ratio.get("policy_api_criteria_total").is_some());
}
