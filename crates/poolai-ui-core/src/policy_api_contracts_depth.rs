//! Policies HTTP API contracts band depth (PH-S1469…S1478, band 83 — enterprise phase D).

use serde_json::Value;

/// Policies HTTP API contracts depth flags (query + store-wire + ops hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyApiContractsDepth {
    None,
    DepthModule,
    QueryHttpLifecycle,
    StoreWireHttp,
    OpenApiSchemas,
    PolicyFieldFixtures,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    FullBand83,
}

/// Policies HTTP API contracts criteria registry (PH-S1469): id · marker · doc path.
pub const POLICY_API_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "policy_api_depth",
        "PolicyApiContractsDepth",
        "crates/poolai-ui-core/src/policy_api_contracts_depth.rs",
    ),
    (
        "query_http_lifecycle",
        "policy_query_http_lifecycle_ph_s1470",
        "tests/policy_api_contracts_integration.rs",
    ),
    (
        "store_wire_http",
        "GET /policy/store",
        "src/network/enterprise_api/security.rs",
    ),
    ("openapi_schemas", "PolicyStoreWire", "docs/openapi.yaml"),
    (
        "policy_field_fixtures",
        "policy_field_fixtures_http_ph_s1473",
        "tests/policy_api_contracts_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_POLICY_API",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "policy_api_band83_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--policy-api",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "policy_api_docs",
        "POLICIES_API.md",
        "docs/development/POLICIES_API.md",
    ),
];

/// `poolai-loc-audit --policy-api` case names (PH-S1475).
pub const POLICY_API_CASES: &[&str] = &[
    "policy_api_depth",
    "query_http_lifecycle",
    "store_wire_http",
    "openapi_schemas",
    "policy_field_fixtures",
    "verify_dev_stand_hook",
    "stand_smoke_export",
    "loc_audit_flag",
    "policy_api_docs",
];

/// FM §5.64 band-83 marker rows.
pub const FM_BAND83_ROWS: &[&str] = &[
    "5.64",
    "Policies API contracts",
    "PH-S1469…S1478",
    "policy_api_contracts_depth",
];

/// Policies HTTP API contracts adoption markers for band 83.
pub const POLICY_API_BAND83_ROWS: &[&str] = &[
    "PH-S1469",
    "policy_api_contracts_depth",
    "PH-S1470",
    "policy_api_contracts_integration",
    "PH-S1471",
    "GET /policy/store",
    "PH-S1474",
    "VERIFY_POLICY_API",
    "PH-S1475",
    "--policy-api",
    "PH-S1478",
];

/// Classify Policies HTTP API contracts band depth from optional feature stub (PH-S1469).
pub fn policy_api_contracts_depth_stub(features: Option<&Value>) -> PolicyApiContractsDepth {
    let Some(f) = features else {
        return PolicyApiContractsDepth::None;
    };
    let depth = f
        .get("policy_api_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let query = f
        .get("query_http_lifecycle")
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
    let fixtures = f
        .get("policy_field_fixtures")
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
        .get("policy_api_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if depth && query && store_http && openapi && fixtures && verify && smoke && loc && docs {
        return PolicyApiContractsDepth::FullBand83;
    }
    if docs {
        return PolicyApiContractsDepth::DocsCanon;
    }
    if loc {
        return PolicyApiContractsDepth::LocAuditFlag;
    }
    if smoke {
        return PolicyApiContractsDepth::StandSmokeExport;
    }
    if verify {
        return PolicyApiContractsDepth::VerifyDevStandHook;
    }
    if fixtures {
        return PolicyApiContractsDepth::PolicyFieldFixtures;
    }
    if openapi {
        return PolicyApiContractsDepth::OpenApiSchemas;
    }
    if store_http {
        return PolicyApiContractsDepth::StoreWireHttp;
    }
    if query {
        return PolicyApiContractsDepth::QueryHttpLifecycle;
    }
    if depth {
        return PolicyApiContractsDepth::DepthModule;
    }
    PolicyApiContractsDepth::None
}

/// Total Policies HTTP API contracts criteria in registry (PH-S1469).
pub fn policy_api_criteria_total() -> usize {
    POLICY_API_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn policy_api_contracts_depth_stub_ph_s1469() {
        assert_eq!(
            policy_api_contracts_depth_stub(None),
            PolicyApiContractsDepth::None
        );
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
        assert!(FM_BAND83_ROWS.contains(&"PH-S1469…S1478"));
    }
}
