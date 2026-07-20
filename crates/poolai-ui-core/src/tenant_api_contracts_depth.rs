//! Tenant HTTP API contracts band depth (PH-S1169…S1178, band 53 — enterprise phase A).

use serde_json::Value;

/// Tenant HTTP API contracts depth flags (CRUD / quota / isolation + ops hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TenantApiContractsDepth {
    None,
    DepthModule,
    HttpCrud,
    QuotaUsage,
    Isolation,
    StoreWireHttp,
    OpenApiSchemas,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    FullBand53,
}

/// Tenant HTTP API contracts criteria registry (PH-S1169): id · marker · doc path.
pub const TENANT_API_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "tenant_api_depth",
        "TenantApiContractsDepth",
        "crates/poolai-ui-core/src/tenant_api_contracts_depth.rs",
    ),
    (
        "http_crud",
        "tenant_api_contracts_integration",
        "tests/tenant_api_contracts_integration.rs",
    ),
    (
        "quota_usage",
        "tenant_quota_usage_http_ph_s1171",
        "tests/tenant_api_contracts_integration.rs",
    ),
    (
        "isolation",
        "tenant_cross_tenant_isolation_http_ph_s1172",
        "tests/tenant_api_contracts_integration.rs",
    ),
    (
        "store_wire_http",
        "GET /tenants/store",
        "src/network/enterprise_api/tenants.rs",
    ),
    ("openapi_schemas", "TenantStoreWire", "docs/openapi.yaml"),
    (
        "verify_dev_stand_hook",
        "VERIFY_TENANT_API",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "tenant_api_band53_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--tenant-api",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "tenant_api_docs",
        "TENANT_API.md",
        "docs/development/TENANT_API.md",
    ),
];

/// `poolai-loc-audit --tenant-api` case names (PH-S1176).
pub const TENANT_API_CASES: &[&str] = &[
    "tenant_api_depth",
    "http_crud",
    "quota_usage",
    "isolation",
    "store_wire_http",
    "openapi_schemas",
    "verify_dev_stand_hook",
    "stand_smoke_export",
    "loc_audit_flag",
    "tenant_api_docs",
];

/// FM §5.34 band-53 marker rows.
pub const FM_BAND53_ROWS: &[&str] = &[
    "5.34",
    "Tenant API contracts",
    "PH-S1169…S1178",
    "tenant_api_contracts_depth",
];

/// Tenant HTTP API contracts adoption markers for band 53.
pub const TENANT_API_BAND53_ROWS: &[&str] = &[
    "PH-S1169",
    "tenant_api_contracts_depth",
    "PH-S1170",
    "tenant_api_contracts_integration",
    "PH-S1173",
    "GET /tenants/store",
    "PH-S1175",
    "VERIFY_TENANT_API",
    "PH-S1176",
    "--tenant-api",
    "PH-S1178",
];

/// Classify tenant HTTP API contracts band depth from optional feature stub (PH-S1169).
pub fn tenant_api_contracts_depth_stub(features: Option<&Value>) -> TenantApiContractsDepth {
    let Some(f) = features else {
        return TenantApiContractsDepth::None;
    };
    let depth = f
        .get("tenant_api_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let crud = f
        .get("http_crud")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let quota = f
        .get("quota_usage")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let isolation = f
        .get("isolation")
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
        .get("tenant_api_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if depth
        && crud
        && quota
        && isolation
        && store_http
        && openapi
        && verify
        && smoke
        && loc
        && docs
    {
        return TenantApiContractsDepth::FullBand53;
    }
    if docs {
        return TenantApiContractsDepth::DocsCanon;
    }
    if loc {
        return TenantApiContractsDepth::LocAuditFlag;
    }
    if smoke {
        return TenantApiContractsDepth::StandSmokeExport;
    }
    if verify {
        return TenantApiContractsDepth::VerifyDevStandHook;
    }
    if openapi {
        return TenantApiContractsDepth::OpenApiSchemas;
    }
    if store_http {
        return TenantApiContractsDepth::StoreWireHttp;
    }
    if isolation {
        return TenantApiContractsDepth::Isolation;
    }
    if quota {
        return TenantApiContractsDepth::QuotaUsage;
    }
    if crud {
        return TenantApiContractsDepth::HttpCrud;
    }
    if depth {
        return TenantApiContractsDepth::DepthModule;
    }
    TenantApiContractsDepth::None
}

/// Total tenant HTTP API contracts criteria in registry (PH-S1169).
pub fn tenant_api_criteria_total() -> usize {
    TENANT_API_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn tenant_api_contracts_depth_stub_ph_s1169() {
        assert_eq!(
            tenant_api_contracts_depth_stub(None),
            TenantApiContractsDepth::None
        );
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
        assert!(FM_BAND53_ROWS.contains(&"PH-S1169…S1178"));
    }
}
