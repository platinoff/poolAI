//! Audit HTTP API contracts band depth (PH-S1369…S1378, band 73 — enterprise phase C).

use serde_json::Value;

/// Audit HTTP API contracts depth flags (query + store-wire + ops hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditApiContractsDepth {
    None,
    DepthModule,
    QueryHttpLifecycle,
    StoreWireHttp,
    OpenApiSchemas,
    EventFieldFixtures,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    FullBand73,
}

/// Audit HTTP API contracts criteria registry (PH-S1369): id · marker · doc path.
pub const AUDIT_API_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "audit_api_depth",
        "AuditApiContractsDepth",
        "crates/poolai-ui-core/src/audit_api_contracts_depth.rs",
    ),
    (
        "query_http_lifecycle",
        "audit_query_http_lifecycle_ph_s1370",
        "tests/audit_api_contracts_integration.rs",
    ),
    (
        "store_wire_http",
        "GET /audit/store",
        "src/network/enterprise_api/audit.rs",
    ),
    ("openapi_schemas", "AuditStoreWire", "docs/openapi.yaml"),
    (
        "event_field_fixtures",
        "audit_event_field_fixtures_http_ph_s1373",
        "tests/audit_api_contracts_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_AUDIT_API",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "audit_api_band73_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--audit-api",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "audit_api_docs",
        "AUDIT_API.md",
        "docs/development/AUDIT_API.md",
    ),
];

/// `poolai-loc-audit --audit-api` case names (PH-S1375).
pub const AUDIT_API_CASES: &[&str] = &[
    "audit_api_depth",
    "query_http_lifecycle",
    "store_wire_http",
    "openapi_schemas",
    "event_field_fixtures",
    "verify_dev_stand_hook",
    "stand_smoke_export",
    "loc_audit_flag",
    "audit_api_docs",
];

/// FM §5.54 band-73 marker rows.
pub const FM_BAND73_ROWS: &[&str] = &[
    "5.54",
    "Audit API contracts",
    "PH-S1369…S1378",
    "audit_api_contracts_depth",
];

/// Audit HTTP API contracts adoption markers for band 73.
pub const AUDIT_API_BAND73_ROWS: &[&str] = &[
    "PH-S1369",
    "audit_api_contracts_depth",
    "PH-S1370",
    "audit_api_contracts_integration",
    "PH-S1371",
    "GET /audit/store",
    "PH-S1374",
    "VERIFY_AUDIT_API",
    "PH-S1375",
    "--audit-api",
    "PH-S1378",
];

/// Classify Audit HTTP API contracts band depth from optional feature stub (PH-S1369).
pub fn audit_api_contracts_depth_stub(features: Option<&Value>) -> AuditApiContractsDepth {
    let Some(f) = features else {
        return AuditApiContractsDepth::None;
    };
    let depth = f
        .get("audit_api_depth")
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
        .get("event_field_fixtures")
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
        .get("audit_api_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if depth && query && store_http && openapi && fixtures && verify && smoke && loc && docs {
        return AuditApiContractsDepth::FullBand73;
    }
    if docs {
        return AuditApiContractsDepth::DocsCanon;
    }
    if loc {
        return AuditApiContractsDepth::LocAuditFlag;
    }
    if smoke {
        return AuditApiContractsDepth::StandSmokeExport;
    }
    if verify {
        return AuditApiContractsDepth::VerifyDevStandHook;
    }
    if fixtures {
        return AuditApiContractsDepth::EventFieldFixtures;
    }
    if openapi {
        return AuditApiContractsDepth::OpenApiSchemas;
    }
    if store_http {
        return AuditApiContractsDepth::StoreWireHttp;
    }
    if query {
        return AuditApiContractsDepth::QueryHttpLifecycle;
    }
    if depth {
        return AuditApiContractsDepth::DepthModule;
    }
    AuditApiContractsDepth::None
}

/// Total Audit HTTP API contracts criteria in registry (PH-S1369).
pub fn audit_api_criteria_total() -> usize {
    AUDIT_API_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn audit_api_contracts_depth_stub_ph_s1369() {
        assert_eq!(
            audit_api_contracts_depth_stub(None),
            AuditApiContractsDepth::None
        );
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
        assert!(FM_BAND73_ROWS.contains(&"PH-S1369…S1378"));
    }
}
