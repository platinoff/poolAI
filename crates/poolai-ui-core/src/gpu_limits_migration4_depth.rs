//! GPULimits migration band depth 4 (PH-S1919.S1928, band 128 — enterprise phase H).
//!
//! Mirrors the band-104 ratio96 admin/ops slice and band-127 migration depth.
//! Reads the durable GPU-limits store via `GET /api/v1/gpu-limits`,
//! a refresh ops glue button, HTML contracts, verify/loc-audit hooks and docs canon.

use serde_json::Value;

/// GPULimits migration band depth flags 4 (store strip / refresh ops glue / verify hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuLimitsMigrationDepth4 {
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
    FullBand128,
}

/// GPULimits migration criteria registry 4 (PH-S1919): id · marker · doc path.
pub const GPU_LIMITS_MIGRATION4_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "gpu_limits_migration_depth4",
        "GpuLimitsMigrationDepth4",
        "crates/poolai-ui-core/src/gpu_limits_migration4_depth.rs",
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
        "gpu_limits_migration4_integration",
        "tests/gpu_limits_migration4_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_GPU_LIMITS_ADMIN_OPS",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "gpu_limits_migration4_band128_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--gpu-limits-admin-ops",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "docs_canon",
        "GPU_LIMITS_CONTINUATION4.md",
        "docs/development/GPU_LIMITS_CONTINUATION4.md",
    ),
    (
        "ratio_hold",
        "gpu-limits-admin-ops",
        "docs/development/RUN_LOCAL.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1919_integration",
        "tests/galaxy_horizon_s1919_integration.rs",
    ),
];

/// `poolai-loc-audit --gpu-limits-admin-ops` case names (PH-S1904).
pub const GPU_LIMITS_MIGRATION4_CASES: &[&str] = &[
    "gpu_limits_migration_depth4",
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

/// FM §5.128 band-128 marker rows.
pub const FM_BAND128_ROWS: &[&str] = &[
    "5.128",
    "GPULimits migration 4",
    "PH-S1919.S1928",
    "gpu_limits_migration_depth4",
];

/// GPULimits migration 4 adoption markers for band 128.
pub const GPU_LIMITS_MIGRATION4_BAND128_ROWS: &[&str] = &[
    "PH-S1919",
    "gpu_limits_migration_depth4",
    "PH-S1920",
    "gpu-limits-store-badge",
    "PH-S1921",
    "gpu_limits_migration4_contracts",
    "PH-S1922",
    "refreshGpuLimits",
    "PH-S1924",
    "--gpu-limits-admin-ops",
    "PH-S1928",
];

/// Classification GPULimits migration band depth 4 from optional feature stub (PH-S1919).
pub fn gpu_limits_migration_depth4_stub(features: Option<&Value>) -> GpuLimitsMigrationDepth4 {
    let Some(f) = features else {
        return GpuLimitsMigrationDepth4::None;
    };
    let enabled = |key| f.get(key).and_then(Value::as_bool).unwrap_or(false);
    let depth = enabled("gpu_limits_migration_depth4");
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
        return GpuLimitsMigrationDepth4::FullBand128;
    }
    if close || ratio {
        return GpuLimitsMigrationDepth4::RatioHold;
    }
    if docs {
        return GpuLimitsMigrationDepth4::DocsCanon;
    }
    if loc {
        return GpuLimitsMigrationDepth4::LocAuditFlag;
    }
    if smoke {
        return GpuLimitsMigrationDepth4::StandSmokeExport;
    }
    if verify {
        return GpuLimitsMigrationDepth4::VerifyDevStandHook;
    }
    if html {
        return GpuLimitsMigrationDepth4::HtmlContracts;
    }
    if query {
        return GpuLimitsMigrationDepth4::QueryOpsGlue;
    }
    if store {
        return GpuLimitsMigrationDepth4::StoreStrip;
    }
    if depth {
        return GpuLimitsMigrationDepth4::DepthModule;
    }
    GpuLimitsMigrationDepth4::None
}

/// Total GPULimits migration 4 criteria in registry (PH-S1919).
pub fn gpu_limits_migration4_criteria_total() -> usize {
    GPU_LIMITS_MIGRATION4_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn gpu_limits_migration_depth4_stub_ph_s1919() {
        assert_eq!(
            gpu_limits_migration_depth4_stub(None),
            GpuLimitsMigrationDepth4::None
        );
        assert_eq!(
            gpu_limits_migration_depth4_stub(Some(&json!({"gpu_limits_migration_depth4": true}))),
            GpuLimitsMigrationDepth4::DepthModule
        );
        assert_eq!(
            gpu_limits_migration_depth4_stub(Some(&json!({
                "gpu_limits_migration_depth4": true,
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
            GpuLimitsMigrationDepth4::FullBand128
        );
        assert_eq!(GPU_LIMITS_MIGRATION4_CRITERIA.len(), 10);
        assert_eq!(gpu_limits_migration4_criteria_total(), 10);
        assert!(FM_BAND128_ROWS.contains(&"PH-S1919.S1928"));
    }
}
