//! Tenant store-wire band depth (PH-S1159…S1168, band 52 — enterprise phase A).

use serde_json::Value;

/// Tenant store-wire depth flags (durable path / production verify stub + ops hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TenantDepth {
    None,
    DepthModule,
    StoreWire,
    ApiContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    FullBand52,
}

/// Tenant store-wire criteria registry (PH-S1159): id · marker · doc path.
pub const TENANT_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "tenant_depth",
        "TenantDepth",
        "crates/poolai-ui-core/src/tenant_depth.rs",
    ),
    (
        "store_wire",
        "tenant_store_wire",
        "src/enterprise/multi_tenancy.rs",
    ),
    (
        "api_contracts",
        "tenant_store_wire_integration",
        "tests/tenant_store_wire_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_TENANT_STORE",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "tenant_store_band52_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--tenant-store",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "tenant_store_docs",
        "TENANT_STORE.md",
        "docs/development/TENANT_STORE.md",
    ),
];

/// `poolai-loc-audit --tenant-store` case names (PH-S1164).
pub const TENANT_CASES: &[&str] = &[
    "tenant_depth",
    "store_wire",
    "api_contracts",
    "verify_dev_stand_hook",
    "stand_smoke_export",
    "loc_audit_flag",
    "tenant_store_docs",
];

/// FM §5.33 band-52 marker rows.
pub const FM_BAND52_ROWS: &[&str] = &[
    "5.33",
    "Tenant store wire",
    "PH-S1159…S1168",
    "tenant_depth",
];

/// Tenant store-wire adoption markers for band 52.
pub const TENANT_BAND52_ROWS: &[&str] = &[
    "PH-S1159",
    "tenant_depth",
    "PH-S1160",
    "tenant_store_wire",
    "PH-S1162",
    "VERIFY_TENANT_STORE",
    "PH-S1164",
    "--tenant-store",
    "PH-S1168",
];

/// Classify tenant store-wire band depth from optional feature stub (PH-S1159).
pub fn tenant_depth_stub(features: Option<&Value>) -> TenantDepth {
    let Some(f) = features else {
        return TenantDepth::None;
    };
    let depth = f
        .get("tenant_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let store = f
        .get("store_wire")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let api = f
        .get("api_contracts")
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
        .get("tenant_store_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if depth && store && api && verify && smoke && loc && docs {
        return TenantDepth::FullBand52;
    }
    if docs {
        return TenantDepth::DocsCanon;
    }
    if loc {
        return TenantDepth::LocAuditFlag;
    }
    if smoke {
        return TenantDepth::StandSmokeExport;
    }
    if verify {
        return TenantDepth::VerifyDevStandHook;
    }
    if api {
        return TenantDepth::ApiContracts;
    }
    if store {
        return TenantDepth::StoreWire;
    }
    if depth {
        return TenantDepth::DepthModule;
    }
    TenantDepth::None
}

/// Total tenant store-wire criteria in registry (PH-S1159).
pub fn tenant_criteria_total() -> usize {
    TENANT_CRITERIA.len()
}

/// Env key for tenant store backend (shared with band 51 scaffold).
pub const TENANT_STORE_ENV: &str = "POOLAI_TENANT_STORE";

/// Env key for durable tenant data directory (band 52 wire).
pub const TENANT_DATA_DIR_ENV: &str = "POOLAI_TENANT_DATA_DIR";

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn tenant_depth_stub_ph_s1159() {
        assert_eq!(tenant_depth_stub(None), TenantDepth::None);
        assert_eq!(
            tenant_depth_stub(Some(&json!({"tenant_depth": true}))),
            TenantDepth::DepthModule
        );
        assert_eq!(
            tenant_depth_stub(Some(&json!({
                "tenant_depth": true,
                "store_wire": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "tenant_store_docs": true,
            }))),
            TenantDepth::FullBand52
        );
        assert_eq!(TENANT_CRITERIA.len(), 7);
        assert_eq!(tenant_criteria_total(), 7);
        assert_eq!(TENANT_STORE_ENV, "POOLAI_TENANT_STORE");
        assert_eq!(TENANT_DATA_DIR_ENV, "POOLAI_TENANT_DATA_DIR");
        assert!(FM_BAND52_ROWS.contains(&"PH-S1159…S1168"));
    }
}
