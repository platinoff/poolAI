//! SSO HTTP API contracts band depth (PH-S1269…S1278, band 63 — enterprise phase B).

use serde_json::Value;

/// SSO HTTP API contracts depth flags (OAuth2/SAML CRUD + store-wire + ops hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SsoApiContractsDepth {
    None,
    DepthModule,
    Oauth2HttpCrud,
    SamlHttpCrud,
    StoreWireHttp,
    OpenApiSchemas,
    CallbackFixtures,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    FullBand63,
}

/// SSO HTTP API contracts criteria registry (PH-S1269): id · marker · doc path.
pub const SSO_API_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "sso_api_depth",
        "SsoApiContractsDepth",
        "crates/poolai-ui-core/src/sso_api_contracts_depth.rs",
    ),
    (
        "oauth2_http_crud",
        "sso_oauth2_http_crud_lifecycle_ph_s1270",
        "tests/sso_api_contracts_integration.rs",
    ),
    (
        "saml_http_crud",
        "sso_saml_http_crud_lifecycle_ph_s1271",
        "tests/sso_api_contracts_integration.rs",
    ),
    (
        "store_wire_http",
        "GET /security/sso/store",
        "src/network/enterprise_api/security.rs",
    ),
    ("openapi_schemas", "SsoStoreWire", "docs/openapi.yaml"),
    (
        "callback_fixtures",
        "sso_callback_fixtures_http_ph_s1274",
        "tests/sso_api_contracts_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_SSO_API",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "sso_api_band63_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    ("loc_audit_flag", "--sso-api", "src/bin/poolai_loc_audit.rs"),
    ("sso_api_docs", "SSO_API.md", "docs/development/SSO_API.md"),
];

/// `poolai-loc-audit --sso-api` case names (PH-S1276).
pub const SSO_API_CASES: &[&str] = &[
    "sso_api_depth",
    "oauth2_http_crud",
    "saml_http_crud",
    "store_wire_http",
    "openapi_schemas",
    "callback_fixtures",
    "verify_dev_stand_hook",
    "stand_smoke_export",
    "loc_audit_flag",
    "sso_api_docs",
];

/// FM §5.44 band-63 marker rows.
pub const FM_BAND63_ROWS: &[&str] = &[
    "5.44",
    "SSO API contracts",
    "PH-S1269…S1278",
    "sso_api_contracts_depth",
];

/// SSO HTTP API contracts adoption markers for band 63.
pub const SSO_API_BAND63_ROWS: &[&str] = &[
    "PH-S1269",
    "sso_api_contracts_depth",
    "PH-S1270",
    "sso_api_contracts_integration",
    "PH-S1272",
    "GET /security/sso/store",
    "PH-S1275",
    "VERIFY_SSO_API",
    "PH-S1276",
    "--sso-api",
    "PH-S1278",
];

/// Classify SSO HTTP API contracts band depth from optional feature stub (PH-S1269).
pub fn sso_api_contracts_depth_stub(features: Option<&Value>) -> SsoApiContractsDepth {
    let Some(f) = features else {
        return SsoApiContractsDepth::None;
    };
    let depth = f
        .get("sso_api_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let oauth2 = f
        .get("oauth2_http_crud")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let saml = f
        .get("saml_http_crud")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let store_http = f
        .get("store_wire_http")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let openapi = f
        .get("openapi_schemas")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let callbacks = f
        .get("callback_fixtures")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let verify = f
        .get("verify_dev_stand_hook")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let smoke = f
        .get("stand_smoke_export")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let loc = f
        .get("loc_audit_flag")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let docs = f
        .get("sso_api_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if depth
        && oauth2
        && saml
        && store_http
        && openapi
        && callbacks
        && verify
        && smoke
        && loc
        && docs
    {
        return SsoApiContractsDepth::FullBand63;
    }
    if docs {
        return SsoApiContractsDepth::DocsCanon;
    }
    if loc {
        return SsoApiContractsDepth::LocAuditFlag;
    }
    if smoke {
        return SsoApiContractsDepth::StandSmokeExport;
    }
    if verify {
        return SsoApiContractsDepth::VerifyDevStandHook;
    }
    if callbacks {
        return SsoApiContractsDepth::CallbackFixtures;
    }
    if openapi {
        return SsoApiContractsDepth::OpenApiSchemas;
    }
    if store_http {
        return SsoApiContractsDepth::StoreWireHttp;
    }
    if saml {
        return SsoApiContractsDepth::SamlHttpCrud;
    }
    if oauth2 {
        return SsoApiContractsDepth::Oauth2HttpCrud;
    }
    if depth {
        return SsoApiContractsDepth::DepthModule;
    }
    SsoApiContractsDepth::None
}

/// Total SSO HTTP API contracts criteria in registry (PH-S1269).
pub fn sso_api_criteria_total() -> usize {
    SSO_API_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn sso_api_contracts_depth_stub_ph_s1269() {
        assert_eq!(
            sso_api_contracts_depth_stub(None),
            SsoApiContractsDepth::None
        );
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
        assert!(FM_BAND63_ROWS.contains(&"PH-S1269…S1278"));
    }
}
