//! GPULimits migration band depth 2 (PH-S1899.S1908, band 126 — enterprise phase H).
//!
//! Mirrors the band-104 ratio96 admin/ops slice and band-125 migration depth.
//! Reads the durable GPU-limits store via `GET /api/v1/gpu-limits`,
//! a refresh ops glue button, HTML contracts, verify/loc-audit hooks and docs canon.

use serde_json::Value;

/// GPULimits migration band depth flags 2 (store strip / refresh ops glue / verify hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuLimitsMigrationDepth2 {
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
    FullBand126,
}

/// GPULimits migration criteria registry 2 (PH-S1899): id · marker · doc path.
pub const GPU_LIMITS_MIGRATION2_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "gpu_limits_migration_depth2",
        "GpuLimitsMigrationDepth2",
        "crates/poolai-ui-core/src/gpu_limits_migration2_depth.rs",
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
        "gpu_limits_migration2_integration",
        "tests/gpu_limits_migration2_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_GPU_LIMITS_ADMIN_OPS",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "gpu_limits_migration2_band126_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--gpu-limits-admin-ops",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "docs_canon",
        "GPU_LIMITS_CONTINUATION2.md",
        "docs/development/GPU_LIMITS_CONTINUATION2.md",
    ),
    (
        "ratio_hold",
        "gpu-limits-admin-ops",
        "docs/development/RUN_LOCAL.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1899_integration",
        "tests/galaxy_horizon_s1899_integration.rs",
    ),
];

/// `poolai-loc-audit --gpu-limits-admin-ops` case names (PH-S1904).
pub const GPU_LIMITS_MIGRATION2_CASES: &[&str] = &[
    "gpu_limits_migration_depth2",
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

/// FM §5.126 band-126 marker rows.
pub const FM_BAND126_ROWS: &[&str] = &[
    "5.126",
    "GPULimits migration 2",
    "PH-S1899.S1908",
    "gpu_limits_migration_depth2",
];

/// GPULimits migration 2 adoption markers for band 126.
pub const GPU_LIMITS_MIGRATION2_BAND126_ROWS: &[&str] = &[
    "PH-S1899",
    "gpu_limits_migration_depth2",
    "PH-S1900",
    "gpu-limits-store-badge",
    "PH-S1901",
    "gpu_limits_migration2_contracts",
    "PH-S1902",
    "refreshGpuLimits",
    "PH-S1904",
    "--gpu-limits-admin-ops",
    "PH-S1908",
];

/// Classification GPULimits migration band depth 2 from optional feature stub (PH-S1899).
pub fn gpu_limits_migration_depth2_stub(features: Option<&Value>) -> GpuLimitsMigrationDepth2 {
    let Some(f) = features else {
        return GpuLimitsMigrationDepth2::None;
    };
    let enabled = |key| f.get(key).and_then(Value::as_bool).unwrap_or(false);
    let depth = enabled("gpu_limits_migration_depth2");
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
        return GpuLimitsMigrationDepth2::FullBand126;
    }
    if close || ratio {
        return GpuLimitsMigrationDepth2::RatioHold;
    }
    if docs {
        return GpuLimitsMigrationDepth2::DocsCanon;
    }
    if loc {
        return GpuLimitsMigrationDepth2::LocAuditFlag;
    }
    if smoke {
        return GpuLimitsMigrationDepth2::StandSmokeExport;
    }
    if verify {
        return GpuLimitsMigrationDepth2::VerifyDevStandHook;
    }
    if html {
        return GpuLimitsMigrationDepth2::HtmlContracts;
    }
    if query {
        return GpuLimitsMigrationDepth2::QueryOpsGlue;
    }
    if store {
        return GpuLimitsMigrationDepth2::StoreStrip;
    }
    if depth {
        return GpuLimitsMigrationDepth2::DepthModule;
    }
    GpuLimitsMigrationDepth2::None
}

/// Total GPULimits migration 2 criteria in registry (PH-S1899).
pub fn gpu_limits_migration2_criteria_total() -> usize {
    GPU_LIMITS_MIGRATION2_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn gpu_limits_migration_depth2_stub_ph_s1899() {
        assert_eq!(
            gpu_limits_migration_depth2_stub(None),
            GpuLimitsMigrationDepth2::None
        );
        assert_eq!(
            gpu_limits_migration_depth2_stub(Some(&json!({"gpu_limits_migration_depth2": true}))),
            GpuLimitsMigrationDepth2::DepthModule
        );
        assert_eq!(
            gpu_limits_migration_depth2_stub(Some(&json!({
                "gpu_limits_migration_depth2": true,
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
            GpuLimitsMigrationDepth2::FullBand126
        );
        assert_eq!(GPU_LIMITS_MIGRATION2_CRITERIA.len(), 10);
        assert_eq!(gpu_limits_migration2_criteria_total(), 10);
        assert!(FM_BAND126_ROWS.contains(&"PH-S1899.S1908"));
    }
}
