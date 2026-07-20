//! PH-S1178: Galaxy horizon close band 53 — tenant HTTP API contracts.

use poolai_ui_core::tenant_api_contracts_depth::{
    tenant_api_contracts_depth_stub, tenant_api_criteria_total, TenantApiContractsDepth,
    FM_BAND53_ROWS, TENANT_API_BAND53_ROWS, TENANT_API_CASES, TENANT_API_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1169_band_tenant_api_close_ph_s1178() {
    assert_eq!(
        tenant_api_contracts_depth_stub(Some(&json!({"tenant_api_depth": true}))),
        TenantApiContractsDepth::DepthModule
    );
    assert_eq!(
        tenant_api_contracts_depth_stub(Some(&json!({
            "tenant_api_depth": true,
            "http_crud": true,
            "quota_usage": true,
            "isolation": true,
            "store_wire_http": true,
            "openapi_schemas": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "tenant_api_docs": true,
        }))),
        TenantApiContractsDepth::FullBand53
    );

    assert_eq!(TENANT_API_CRITERIA.len(), 10);
    assert_eq!(tenant_api_criteria_total(), 10);
    assert!(TENANT_API_CASES.contains(&"tenant_api_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND53_ROWS {
        assert!(fm.contains(row), "FM missing band-53 row {row}");
    }
    assert!(fm.contains("PH-S1178"));
    assert!(fm.contains("5.34"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1169") || handoff.contains("band 53"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 54"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--tenant-api"));
    assert!(run_local.contains("VERIFY_TENANT_API"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("tenant_api_contracts_depth") || strategy.contains("band 53"));

    let tenant_doc = include_str!("../docs/development/TENANT_API.md");
    assert!(tenant_doc.contains("/api/enterprise/tenants"));
    assert!(tenant_doc.contains("TenantStoreWire"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1169") || roadmap.contains("API contracts"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_TENANT_API"));
    assert!(verify.contains("--tenant-api"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--tenant-api"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("tenant_api_mode"));
    assert!(loc_audit.contains("tenant_api_criteria_met_count"));

    let tenants_api = include_str!("../src/network/enterprise_api/tenants.rs");
    assert!(tenants_api.contains("tenant_store_wire_handler"));

    let openapi = include_str!("../docs/openapi.yaml");
    assert!(openapi.contains("TenantStoreWire"));
    assert!(openapi.contains("/tenants/store"));

    for marker in TENANT_API_BAND53_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || tenants_api.contains(marker)
                || openapi.contains(marker),
            "band-53 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/tenant_api_contracts_depth.rs").exists());
    assert!(Path::new("docs/development/TENANT_API.md").exists());
    assert!(Path::new("tests/tenant_api_contracts_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("tenant_api_mode").is_some());
    assert!(ratio.get("tenant_api_criteria_total").is_some());
}
