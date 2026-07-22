//! PH-S1278: Galaxy horizon close band 63 — SSO HTTP API contracts.

use poolai_ui_core::sso_api_contracts_depth::{
    sso_api_contracts_depth_stub, sso_api_criteria_total, SsoApiContractsDepth, FM_BAND63_ROWS,
    SSO_API_BAND63_ROWS, SSO_API_CASES, SSO_API_CRITERIA,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1269_band_sso_api_close_ph_s1278() {
    assert_eq!(
        sso_api_contracts_depth_stub(Some(&json!({"sso_api_depth": true}))),
        SsoApiContractsDepth::DepthModule
    );
    assert_eq!(
        sso_api_contracts_depth_stub(Some(&json!({
            "sso_api_depth": true,
            "oauth2_http_crud": true,
            "saml_http_crud": true,
            "store_wire_http": true,
            "openapi_schemas": true,
            "callback_fixtures": true,
            "verify_dev_stand_hook": true,
            "stand_smoke_export": true,
            "loc_audit_flag": true,
            "sso_api_docs": true,
        }))),
        SsoApiContractsDepth::FullBand63
    );

    assert_eq!(SSO_API_CRITERIA.len(), 10);
    assert_eq!(sso_api_criteria_total(), 10);
    assert!(SSO_API_CASES.contains(&"sso_api_docs"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND63_ROWS {
        assert!(fm.contains(row), "FM missing band-63 row {row}");
    }
    assert!(fm.contains("PH-S1278"));
    assert!(fm.contains("5.44"));
    assert!(fm.contains("5.17"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1269") || handoff.contains("band 63"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 64"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--sso-api"));
    assert!(run_local.contains("VERIFY_SSO_API"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("sso_api_contracts_depth") || strategy.contains("band 63"));

    let sso_doc = include_str!("../docs/development/SSO_API.md");
    assert!(sso_doc.contains("/api/enterprise/security"));
    assert!(sso_doc.contains("SsoStoreWire"));

    let roadmap = include_str!("../docs/development/PH_S_ENTERPRISE_ROADMAP_2026-07-19.md");
    assert!(roadmap.contains("PH-S1269") || roadmap.contains("API contracts"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_SSO_API"));
    assert!(verify.contains("--sso-api"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--sso-api"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("sso_api_mode"));
    assert!(loc_audit.contains("sso_api_criteria_met_count"));

    let security_api = include_str!("../src/network/enterprise_api/security.rs");
    assert!(security_api.contains("sso_store_wire_handler"));
    assert!(security_api.contains("GET /security/sso/store"));

    let openapi = include_str!("../docs/openapi.yaml");
    assert!(openapi.contains("SsoStoreWire"));
    assert!(openapi.contains("/security/sso/store"));

    for marker in SSO_API_BAND63_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || security_api.contains(marker)
                || openapi.contains(marker),
            "band-63 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/sso_api_contracts_depth.rs").exists());
    assert!(Path::new("docs/development/SSO_API.md").exists());
    assert!(Path::new("tests/sso_api_contracts_integration.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("sso_api_mode").is_some());
    assert!(ratio.get("sso_api_criteria_total").is_some());
}
