//! Monitoring HTTP API contracts band depth (PH-S1569…S1578, band 93 — enterprise phase E).

use serde_json::Value;

/// Monitoring HTTP API contracts depth flags (query + store-wire + ops hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonitoringApiContractsDepth {
    None,
    DepthModule,
    QueryHttpLifecycle,
    StoreWireHttp,
    OpenApiSchemas,
    MonitoringFieldFixtures,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    FullBand93,
}

/// Monitoring HTTP API contracts criteria registry (PH-S1569): id · marker · doc path.
pub const MONITORING_API_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "monitoring_api_depth",
        "MonitoringApiContractsDepth",
        "crates/poolai-ui-core/src/monitoring_api_contracts_depth.rs",
    ),
    (
        "query_http_lifecycle",
        "monitoring_query_http_lifecycle_ph_s1570",
        "tests/monitoring_api_contracts_integration.rs",
    ),
    (
        "store_wire_http",
        "GET /monitoring/store",
        "src/network/enterprise_api/monitoring.rs",
    ),
    (
        "openapi_schemas",
        "MonitoringStoreWire",
        "docs/openapi.yaml",
    ),
    (
        "monitoring_field_fixtures",
        "monitoring_field_fixtures_http_ph_s1573",
        "tests/monitoring_api_contracts_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_MONITORING_API",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "monitoring_api_band93_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--monitoring-api",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "monitoring_api_docs",
        "MONITORING_API.md",
        "docs/development/MONITORING_API.md",
    ),
];

/// `poolai-loc-audit --monitoring-api` case names (PH-S1575).
pub const MONITORING_API_CASES: &[&str] = &[
    "monitoring_api_depth",
    "query_http_lifecycle",
    "store_wire_http",
    "openapi_schemas",
    "monitoring_field_fixtures",
    "verify_dev_stand_hook",
    "stand_smoke_export",
    "loc_audit_flag",
    "monitoring_api_docs",
];

/// FM §5.74 band-93 marker rows.
pub const FM_BAND93_ROWS: &[&str] = &[
    "5.74",
    "Monitoring API contracts",
    "PH-S1569…S1578",
    "monitoring_api_contracts_depth",
];

/// Monitoring HTTP API contracts adoption markers for band 93.
pub const MONITORING_API_BAND93_ROWS: &[&str] = &[
    "PH-S1569",
    "monitoring_api_contracts_depth",
    "PH-S1570",
    "monitoring_api_contracts_integration",
    "PH-S1571",
    "GET /monitoring/store",
    "PH-S1574",
    "VERIFY_MONITORING_API",
    "PH-S1575",
    "--monitoring-api",
    "PH-S1578",
];

/// Classify Monitoring HTTP API contracts band depth from optional feature stub (PH-S1569).
pub fn monitoring_api_contracts_depth_stub(
    features: Option<&Value>,
) -> MonitoringApiContractsDepth {
    let Some(f) = features else {
        return MonitoringApiContractsDepth::None;
    };
    let depth = f
        .get("monitoring_api_depth")
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
        .get("monitoring_field_fixtures")
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
        .get("monitoring_api_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if depth && query && store_http && openapi && fixtures && verify && smoke && loc && docs {
        return MonitoringApiContractsDepth::FullBand93;
    }
    if docs {
        return MonitoringApiContractsDepth::DocsCanon;
    }
    if loc {
        return MonitoringApiContractsDepth::LocAuditFlag;
    }
    if smoke {
        return MonitoringApiContractsDepth::StandSmokeExport;
    }
    if verify {
        return MonitoringApiContractsDepth::VerifyDevStandHook;
    }
    if fixtures {
        return MonitoringApiContractsDepth::MonitoringFieldFixtures;
    }
    if openapi {
        return MonitoringApiContractsDepth::OpenApiSchemas;
    }
    if store_http {
        return MonitoringApiContractsDepth::StoreWireHttp;
    }
    if query {
        return MonitoringApiContractsDepth::QueryHttpLifecycle;
    }
    if depth {
        return MonitoringApiContractsDepth::DepthModule;
    }
    MonitoringApiContractsDepth::None
}

/// Total Monitoring HTTP API contracts criteria in registry (PH-S1569).
pub fn monitoring_api_criteria_total() -> usize {
    MONITORING_API_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn monitoring_api_contracts_depth_stub_ph_s1569() {
        assert_eq!(
            monitoring_api_contracts_depth_stub(None),
            MonitoringApiContractsDepth::None
        );
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
        assert!(FM_BAND93_ROWS.contains(&"PH-S1569…S1578"));
    }
}
