//! GPULimits migration band depth (PH-S1889…S1898, band 125 — enterprise phase H).
//!
//! Mirrors the band-104 ratio96 admin/ops slice: a dashboard store strip that reads the
//! durable GPU-limits store (`docs/development/gpu_limits.json`) via `GET /api/v1/gpu-limits`,
//! a refresh ops glue button, HTML contracts, verify/loc-audit hooks and docs canon.

use serde_json::Value;

/// GPULimits migration band depth flags (store strip / refresh ops glue / verify hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuLimitsMigrationDepth {
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
    FullBand125,
}

/// GPULimits migration criteria registry (PH-S1889): id · marker · doc path.
pub const GPU_LIMITS_MIGRATION_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "gpu_limits_migration_depth",
        "GpuLimitsMigrationDepth",
        "crates/poolai-ui-core/src/gpu_limits_migration_depth.rs",
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
        "gpu_limits_migration_integration",
        "tests/gpu_limits_migration_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_GPU_LIMITS_ADMIN_OPS",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "gpu_limits_migration_band125_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--gpu-limits-admin-ops",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "docs_canon",
        "GPU_LIMITS_CONTINUATION.md",
        "docs/development/GPU_LIMITS_CONTINUATION.md",
    ),
    (
        "ratio_hold",
        "gpu-limits-admin-ops",
        "docs/development/RUN_LOCAL.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1889_integration",
        "tests/galaxy_horizon_s1889_integration.rs",
    ),
];

/// `poolai-loc-audit --gpu-limits-admin-ops` case names (PH-S1894).
pub const GPU_LIMITS_MIGRATION_CASES: &[&str] = &[
    "gpu_limits_migration_depth",
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

/// FM §5.125 band-125 marker rows.
pub const FM_BAND125_ROWS: &[&str] = &[
    "5.125",
    "GPULimits migration",
    "PH-S1889…S1898",
    "gpu_limits_migration_depth",
];

/// GPULimits migration adoption markers for band 125.
pub const GPU_LIMITS_MIGRATION_BAND125_ROWS: &[&str] = &[
    "PH-S1889",
    "gpu_limits_migration_depth",
    "PH-S1890",
    "gpu_limits_store_wire_json",
    "PH-S1891",
    "gpu_limits_migration_contracts",
    "PH-S1892",
    "refreshGpuLimits",
    "PH-S1894",
    "--gpu-limits-admin-ops",
    "PH-S1898",
];

/// Classify GPULimits migration band depth from optional feature stub (PH-S1889).
pub fn gpu_limits_migration_depth_stub(features: Option<&Value>) -> GpuLimitsMigrationDepth {
    let Some(f) = features else {
        return GpuLimitsMigrationDepth::None;
    };
    let enabled = |key| f.get(key).and_then(Value::as_bool).unwrap_or(false);
    let depth = enabled("gpu_limits_migration_depth");
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
        return GpuLimitsMigrationDepth::FullBand125;
    }
    if close || ratio {
        return GpuLimitsMigrationDepth::RatioHold;
    }
    if docs {
        return GpuLimitsMigrationDepth::DocsCanon;
    }
    if loc {
        return GpuLimitsMigrationDepth::LocAuditFlag;
    }
    if smoke {
        return GpuLimitsMigrationDepth::StandSmokeExport;
    }
    if verify {
        return GpuLimitsMigrationDepth::VerifyDevStandHook;
    }
    if html {
        return GpuLimitsMigrationDepth::HtmlContracts;
    }
    if query {
        return GpuLimitsMigrationDepth::QueryOpsGlue;
    }
    if store {
        return GpuLimitsMigrationDepth::StoreStrip;
    }
    if depth {
        return GpuLimitsMigrationDepth::DepthModule;
    }
    GpuLimitsMigrationDepth::None
}

/// Total GPULimits migration criteria in registry (PH-S1889).
pub fn gpu_limits_migration_criteria_total() -> usize {
    GPU_LIMITS_MIGRATION_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn gpu_limits_migration_depth_stub_ph_s1889() {
        assert_eq!(
            gpu_limits_migration_depth_stub(None),
            GpuLimitsMigrationDepth::None
        );
        assert_eq!(
            gpu_limits_migration_depth_stub(Some(&json!({"gpu_limits_migration_depth": true}))),
            GpuLimitsMigrationDepth::DepthModule
        );
        assert_eq!(
            gpu_limits_migration_depth_stub(Some(&json!({
                "gpu_limits_migration_depth": true,
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
            GpuLimitsMigrationDepth::FullBand125
        );
        assert_eq!(GPU_LIMITS_MIGRATION_CRITERIA.len(), 10);
        assert_eq!(gpu_limits_migration_criteria_total(), 10);
        assert!(FM_BAND125_ROWS.contains(&"PH-S1889…S1898"));
    }
}