//! Tenant admin/ops glue band depth (PH-S1179…S1188, band 54 — enterprise phase A).

use serde_json::Value;

/// Tenant admin/ops glue depth flags (store strip / usage+quota / verify hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TenantAdminOpsDepth {
    None,
    DepthModule,
    StoreStrip,
    UsageQuotaGlue,
    HtmlContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand54,
}

/// Tenant admin/ops criteria registry (PH-S1179): id · marker · doc path.
pub const TENANT_ADMIN_OPS_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "tenant_admin_ops_depth",
        "TenantAdminOpsDepth",
        "crates/poolai-ui-core/src/tenant_admin_ops_depth.rs",
    ),
    (
        "store_strip",
        "tenant-store-badge",
        "src/ui/admin/tenants.rs",
    ),
    (
        "usage_quota_glue",
        "probeTenantQuota",
        "src/ui/admin/tenants.rs",
    ),
    (
        "html_contracts",
        "tenant_admin_ops_integration",
        "tests/tenant_admin_ops_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_TENANT_ADMIN_OPS",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "tenant_admin_ops_band54_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--tenant-admin-ops",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "tenant_admin_ops_docs",
        "TENANT_ADMIN_OPS.md",
        "docs/development/TENANT_ADMIN_OPS.md",
    ),
    ("ratio_hold", "min-ratio", "docs/development/RUN_LOCAL.md"),
    (
        "band_close",
        "galaxy_horizon_s1179_integration",
        "tests/galaxy_horizon_s1179_integration.rs",
    ),
];

/// `poolai-loc-audit --tenant-admin-ops` case names (PH-S1185).
pub const TENANT_ADMIN_OPS_CASES: &[&str] = &[
    "tenant_admin_ops_depth",
    "store_strip",
    "usage_quota_glue",
    "html_contracts",
    "verify_dev_stand_hook",
    "stand_smoke_export",
    "loc_audit_flag",
    "tenant_admin_ops_docs",
    "ratio_hold",
    "band_close",
];

/// FM §5.35 band-54 marker rows.
pub const FM_BAND54_ROWS: &[&str] = &[
    "5.35",
    "Tenant admin/ops glue",
    "PH-S1179…S1188",
    "tenant_admin_ops_depth",
];

/// Tenant admin/ops adoption markers for band 54.
pub const TENANT_ADMIN_OPS_BAND54_ROWS: &[&str] = &[
    "PH-S1179",
    "tenant_admin_ops_depth",
    "PH-S1180",
    "tenant-store-badge",
    "PH-S1181",
    "probeTenantQuota",
    "PH-S1184",
    "VERIFY_TENANT_ADMIN_OPS",
    "PH-S1185",
    "--tenant-admin-ops",
    "PH-S1188",
];

/// Classify tenant admin/ops band depth from optional feature stub (PH-S1179).
pub fn tenant_admin_ops_depth_stub(features: Option<&Value>) -> TenantAdminOpsDepth {
    let Some(f) = features else {
        return TenantAdminOpsDepth::None;
    };
    let depth = f
        .get("tenant_admin_ops_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let store = f
        .get("store_strip")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let usage = f
        .get("usage_quota_glue")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let html = f
        .get("html_contracts")
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
        .get("tenant_admin_ops_docs")
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

    if depth && store && usage && html && verify && smoke && loc && docs && ratio && close {
        return TenantAdminOpsDepth::FullBand54;
    }
    if close || ratio {
        return TenantAdminOpsDepth::RatioHold;
    }
    if docs {
        return TenantAdminOpsDepth::DocsCanon;
    }
    if loc {
        return TenantAdminOpsDepth::LocAuditFlag;
    }
    if smoke {
        return TenantAdminOpsDepth::StandSmokeExport;
    }
    if verify {
        return TenantAdminOpsDepth::VerifyDevStandHook;
    }
    if html {
        return TenantAdminOpsDepth::HtmlContracts;
    }
    if usage {
        return TenantAdminOpsDepth::UsageQuotaGlue;
    }
    if store {
        return TenantAdminOpsDepth::StoreStrip;
    }
    if depth {
        return TenantAdminOpsDepth::DepthModule;
    }
    TenantAdminOpsDepth::None
}

/// Total tenant admin/ops criteria in registry (PH-S1179).
pub fn tenant_admin_ops_criteria_total() -> usize {
    TENANT_ADMIN_OPS_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn tenant_admin_ops_depth_stub_ph_s1179() {
        assert_eq!(tenant_admin_ops_depth_stub(None), TenantAdminOpsDepth::None);
        assert_eq!(
            tenant_admin_ops_depth_stub(Some(&json!({"tenant_admin_ops_depth": true}))),
            TenantAdminOpsDepth::DepthModule
        );
        assert_eq!(
            tenant_admin_ops_depth_stub(Some(&json!({
                "tenant_admin_ops_depth": true,
                "store_strip": true,
                "usage_quota_glue": true,
                "html_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "tenant_admin_ops_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            TenantAdminOpsDepth::FullBand54
        );
        assert_eq!(TENANT_ADMIN_OPS_CRITERIA.len(), 10);
        assert_eq!(tenant_admin_ops_criteria_total(), 10);
        assert!(FM_BAND54_ROWS.contains(&"PH-S1179…S1188"));
    }
}
