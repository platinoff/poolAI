//! GPULimits migration band depth 5 (PH-S1929.S1938, band 129 — enterprise phase H).
//!
//! Mirrors the band-104 ratio96 admin/ops slice and band-128 migration depth.
//! Reads the durable GPU-limits store via `GET /api/v1/gpu-limits`,
//! a refresh ops glue button, HTML contracts, verify/loc-audit hooks and docs canon.

use serde_json::Value;

/// GPULimits migration band depth flags 5 (store strip / refresh ops glue / verify hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuLimitsMigrationDepth5 {
    None,
    DepthModule,
    StoreStrip,
    QueryOpsGlue,
    HtmlContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand129,
}

/// GPULimits migration criteria registry 5 (PH-S1929): id · marker · doc path.
pub const GPU_LIMITS_MIGRATION5_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "gpu_limits_migration_depth5",
        "GpuLimitsMigrationDepth5",
        "crates/poolai-ui-core/src/gpu_limits_migration5_depth.rs",
    ),
    (
        "store_strip",
        "gpu-limits-store-badge",
        "src/ui/admin/dashboard.rs",
    ),
    (
        "query_ops_glue",
        "refreshGpuLimits",
        "src/ui/admin/dashboard.rs",
    ),
    (
        "html_contracts",
        "gpu_limits_migration5_integration",
        "tests/gpu_limits_migration5_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_GPU_LIMITS_ADMIN_OPS",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "gpu_limits_migration5_band129_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--gpu-limits-admin-ops",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "docs_canon",
        "GPU_LIMITS_CONTINUATION5.md",
        "docs/development/GPU_LIMITS_CONTINUATION5.md",
    ),
    (
        "ratio_hold",
        "gpu-limits-admin-ops",
        "docs/development/RUN_LOCAL.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1929_integration",
        "tests/galaxy_horizon_s1929_integration.rs",
    ),
];

/// `poolai-loc-audit --gpu-limits-admin-ops` case names (PH-S1904).
pub const GPU_LIMITS_MIGRATION5_CASES: &[&str] = &[
    "gpu_limits_migration_depth5",
    "store_strip",
    "query_ops_glue",
    "html_contracts",
    "verify_dev_stand_hook",
    "stand_smoke_export",
    "loc_audit_flag",
    "docs_canon",
    "ratio_hold",
    "band_close",
];

/// FM §5.129 band-129 marker rows.
pub const FM_BAND129_ROWS: &[&str] = &[
    "5.129",
    "GPULimits migration 5",
    "PH-S1929.S1938",
    "gpu_limits_migration_depth5",
];

/// GPULimits migration 5 adoption markers for band 129.
pub const GPU_LIMITS_MIGRATION5_BAND129_ROWS: &[&str] = &[
    "PH-S1929",
    "gpu_limits_migration_depth5",
    "PH-S1930",
    "gpu-limits-store-badge",
    "PH-S1931",
    "gpu_limits_migration5_contracts",
    "PH-S1932",
    "refreshGpuLimits",
    "PH-S1934",
    "--gpu-limits-admin-ops",
    "PH-S1938",
];

/// Classification GPULimits migration band depth 5 from optional feature stub (PH-S1929).
pub fn gpu_limits_migration_depth5_stub(features: Option<&Value>) -> GpuLimitsMigrationDepth5 {
    let Some(f) = features else {
        return GpuLimitsMigrationDepth5::None;
    };
    let enabled = |key| f.get(key).and_then(Value::as_bool).unwrap_or(false);
    let depth = enabled("gpu_limits_migration_depth5");
    let store = enabled("store_strip");
    let query = enabled("query_ops_glue");
    let html = enabled("html_contracts");
    let verify = enabled("verify_dev_stand_hook");
    let smoke = enabled("stand_smoke_export");
    let loc = enabled("loc_audit_flag");
    let docs = enabled("docs_canon");
    let ratio = enabled("ratio_hold");
    let close = enabled("band_close");

    if depth && store && query && html && verify && smoke && loc && docs && ratio && close {
        return GpuLimitsMigrationDepth5::FullBand129;
    }
    if close || ratio {
        return GpuLimitsMigrationDepth5::RatioHold;
    }
    if docs {
        return GpuLimitsMigrationDepth5::DocsCanon;
    }
    if loc {
        return GpuLimitsMigrationDepth5::LocAuditFlag;
    }
    if smoke {
        return GpuLimitsMigrationDepth5::StandSmokeExport;
    }
    if verify {
        return GpuLimitsMigrationDepth5::VerifyDevStandHook;
    }
    if html {
        return GpuLimitsMigrationDepth5::HtmlContracts;
    }
    if query {
        return GpuLimitsMigrationDepth5::QueryOpsGlue;
    }
    if store {
        return GpuLimitsMigrationDepth5::StoreStrip;
    }
    if depth {
        return GpuLimitsMigrationDepth5::DepthModule;
    }
    GpuLimitsMigrationDepth5::None
}

/// Total GPULimits migration 5 criteria in registry (PH-S1929).
pub fn gpu_limits_migration5_criteria_total() -> usize {
    GPU_LIMITS_MIGRATION5_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn gpu_limits_migration_depth5_stub_ph_s1929() {
        assert_eq!(
            gpu_limits_migration_depth5_stub(None),
            GpuLimitsMigrationDepth5::None
        );
        assert_eq!(
            gpu_limits_migration_depth5_stub(Some(&json!({"gpu_limits_migration_depth5": true}))),
            GpuLimitsMigrationDepth5::DepthModule
        );
        assert_eq!(
            gpu_limits_migration_depth5_stub(Some(&json!({
                "gpu_limits_migration_depth5": true,
                "store_strip": true,
                "query_ops_glue": true,
                "html_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "docs_canon": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            GpuLimitsMigrationDepth5::FullBand129
        );
        assert_eq!(GPU_LIMITS_MIGRATION5_CRITERIA.len(), 10);
        assert_eq!(gpu_limits_migration5_criteria_total(), 10);
        assert!(FM_BAND129_ROWS.contains(&"PH-S1929.S1938"));
    }
}
