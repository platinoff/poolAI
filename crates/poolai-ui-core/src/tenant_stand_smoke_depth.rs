//! Tenant live stand-smoke band depth (PH-S1189…S1198, band 55 — enterprise phase A).

use serde_json::Value;

/// Tenant stand-smoke depth flags (live HTTP / CLI / verify / docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TenantStandSmokeDepth {
    None,
    DepthModule,
    LiveStore,
    LiveCrud,
    LiveUsageQuota,
    CliFlag,
    LocAuditFlag,
    VerifyDevStandHook,
    DocsCanon,
    RatioHold,
    FullBand55,
}

/// Tenant stand-smoke criteria registry (PH-S1189): id · marker · doc path.
pub const TENANT_STAND_SMOKE_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "tenant_stand_smoke_depth",
        "TenantStandSmokeDepth",
        "crates/poolai-ui-core/src/tenant_stand_smoke_depth.rs",
    ),
    (
        "live_store",
        "smoke_tenants_store_wire",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "live_crud",
        "smoke_tenants_crud_lifecycle",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "live_usage_quota",
        "smoke_tenants_usage_quota_isolation",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "cli_flag",
        "--tenant-stand-smoke",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--tenant-stand-smoke",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_TENANT_STAND_SMOKE",
        "bin/verify-dev-stand.sh",
    ),
    (
        "tenant_stand_smoke_docs",
        "TENANT_STAND_SMOKE.md",
        "docs/development/TENANT_STAND_SMOKE.md",
    ),
    ("ratio_hold", "min-ratio", "docs/development/RUN_LOCAL.md"),
    (
        "band_close",
        "galaxy_horizon_s1189_integration",
        "tests/galaxy_horizon_s1189_integration.rs",
    ),
];

/// `poolai-loc-audit --tenant-stand-smoke` case names (PH-S1194).
pub const TENANT_STAND_SMOKE_CASES: &[&str] = &[
    "tenant_stand_smoke_depth",
    "live_store",
    "live_crud",
    "live_usage_quota",
    "cli_flag",
    "loc_audit_flag",
    "verify_dev_stand_hook",
    "tenant_stand_smoke_docs",
    "ratio_hold",
    "band_close",
];

/// FM §5.36 band-55 marker rows.
pub const FM_BAND55_ROWS: &[&str] = &[
    "5.36",
    "Tenant stand smoke",
    "PH-S1189…S1198",
    "tenant_stand_smoke_depth",
];

/// Tenant stand-smoke adoption markers for band 55.
pub const TENANT_STAND_SMOKE_BAND55_ROWS: &[&str] = &[
    "PH-S1189",
    "tenant_stand_smoke_depth",
    "PH-S1190",
    "smoke_tenants_store_wire",
    "PH-S1191",
    "smoke_tenants_crud_lifecycle",
    "PH-S1192",
    "smoke_tenants_usage_quota_isolation",
    "PH-S1193",
    "--tenant-stand-smoke",
    "PH-S1195",
    "VERIFY_TENANT_STAND_SMOKE",
    "PH-S1198",
];

/// Classify tenant stand-smoke band depth from optional feature stub (PH-S1189).
pub fn tenant_stand_smoke_depth_stub(features: Option<&Value>) -> TenantStandSmokeDepth {
    let Some(f) = features else {
        return TenantStandSmokeDepth::None;
    };
    let depth = f
        .get("tenant_stand_smoke_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let store = f
        .get("live_store")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let crud = f
        .get("live_crud")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let usage = f
        .get("live_usage_quota")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let cli = f.get("cli_flag").and_then(|v| v.as_bool()).unwrap_or(false);
    let loc = f
        .get("loc_audit_flag")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let verify = f
        .get("verify_dev_stand_hook")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let docs = f
        .get("tenant_stand_smoke_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let ratio = f
        .get("ratio_hold")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let close = f
        .get("band_close")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if depth && store && crud && usage && cli && loc && verify && docs && ratio && close {
        return TenantStandSmokeDepth::FullBand55;
    }
    if close || ratio {
        return TenantStandSmokeDepth::RatioHold;
    }
    if docs {
        return TenantStandSmokeDepth::DocsCanon;
    }
    if verify {
        return TenantStandSmokeDepth::VerifyDevStandHook;
    }
    if loc {
        return TenantStandSmokeDepth::LocAuditFlag;
    }
    if cli {
        return TenantStandSmokeDepth::CliFlag;
    }
    if usage {
        return TenantStandSmokeDepth::LiveUsageQuota;
    }
    if crud {
        return TenantStandSmokeDepth::LiveCrud;
    }
    if store {
        return TenantStandSmokeDepth::LiveStore;
    }
    if depth {
        return TenantStandSmokeDepth::DepthModule;
    }
    TenantStandSmokeDepth::None
}

/// Total tenant stand-smoke criteria in registry (PH-S1189).
pub fn tenant_stand_smoke_criteria_total() -> usize {
    TENANT_STAND_SMOKE_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn tenant_stand_smoke_depth_stub_ph_s1189() {
        assert_eq!(
            tenant_stand_smoke_depth_stub(None),
            TenantStandSmokeDepth::None
        );
        assert_eq!(
            tenant_stand_smoke_depth_stub(Some(&json!({"tenant_stand_smoke_depth": true}))),
            TenantStandSmokeDepth::DepthModule
        );
        assert_eq!(
            tenant_stand_smoke_depth_stub(Some(&json!({
                "tenant_stand_smoke_depth": true,
                "live_store": true,
                "live_crud": true,
                "live_usage_quota": true,
                "cli_flag": true,
                "loc_audit_flag": true,
                "verify_dev_stand_hook": true,
                "tenant_stand_smoke_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            TenantStandSmokeDepth::FullBand55
        );
        assert_eq!(TENANT_STAND_SMOKE_CRITERIA.len(), 10);
        assert_eq!(tenant_stand_smoke_criteria_total(), 10);
        assert!(FM_BAND55_ROWS.contains(&"PH-S1189…S1198"));
    }
}
