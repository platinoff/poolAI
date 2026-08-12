//! GPULimits migration band depth 6 (PH-S1939.S1948, band 130 — enterprise phase H).
//!
//! Mirrors the band-104 ratio96 admin/ops slice and band-129 migration depth.
//! Reads the durable GPU-limits store via `GET /api/v1/gpu-limits`,
//! a refresh ops glue button, HTML contracts, verify/loc-audit hooks and docs canon.

use serde_json::Value;

/// GPULimits migration band depth flags 6 (store strip / refresh ops glue / verify hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuLimitsMigrationDepth6 {
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
    FullBand130,
}

/// GPULimits migration criteria registry 6 (PH-S1939): id · marker · doc path.
pub const GPU_LIMITS_MIGRATION6_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "gpu_limits_migration_depth6",
        "GpuLimitsMigrationDepth6",
        "crates/poolai-ui-core/src/gpu_limits_migration6_depth.rs",
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
        "gpu_limits_migration6_integration",
        "tests/gpu_limits_migration6_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_GPU_LIMITS_ADMIN_OPS",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "gpu_limits_migration6_band130_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--gpu-limits-admin-ops",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "docs_canon",
        "GPU_LIMITS_CONTINUATION6.md",
        "docs/development/GPU_LIMITS_CONTINUATION6.md",
    ),
    (
        "ratio_hold",
        "gpu-limits-admin-ops",
        "docs/development/RUN_LOCAL.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1939_integration",
        "tests/galaxy_horizon_s1939_integration.rs",
    ),
];

/// `poolai-loc-audit --gpu-limits-admin-ops` case names (PH-S1904).
pub const GPU_LIMITS_MIGRATION6_CASES: &[&str] = &[
    "gpu_limits_migration_depth6",
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

/// FM §5.130 band-130 marker rows.
pub const FM_BAND130_ROWS: &[&str] = &[
    "5.130",
    "GPULimits migration 6",
    "PH-S1939.S1948",
    "gpu_limits_migration_depth6",
];

/// GPULimits migration 6 adoption markers for band 130.
pub const GPU_LIMITS_MIGRATION6_BAND130_ROWS: &[&str] = &[
    "PH-S1939",
    "gpu_limits_migration_depth6",
    "PH-S1940",
    "gpu-limits-store-badge",
    "PH-S1941",
    "gpu_limits_migration6_contracts",
    "PH-S1942",
    "refreshGpuLimits",
    "PH-S1944",
    "--gpu-limits-admin-ops",
    "PH-S1948",
];

/// Classification GPULimits migration band depth 6 from optional feature stub (PH-S1939).
pub fn gpu_limits_migration_depth6_stub(features: Option<&Value>) -> GpuLimitsMigrationDepth6 {
    let Some(f) = features else {
        return GpuLimitsMigrationDepth6::None;
    };
    let enabled = |key| f.get(key).and_then(Value::as_bool).unwrap_or(false);
    let depth = enabled("gpu_limits_migration_depth6");
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
        return GpuLimitsMigrationDepth6::FullBand130;
    }
    if close || ratio {
        return GpuLimitsMigrationDepth6::RatioHold;
    }
    if docs {
        return GpuLimitsMigrationDepth6::DocsCanon;
    }
    if loc {
        return GpuLimitsMigrationDepth6::LocAuditFlag;
    }
    if smoke {
        return GpuLimitsMigrationDepth6::StandSmokeExport;
    }
    if verify {
        return GpuLimitsMigrationDepth6::VerifyDevStandHook;
    }
    if html {
        return GpuLimitsMigrationDepth6::HtmlContracts;
    }
    if query {
        return GpuLimitsMigrationDepth6::QueryOpsGlue;
    }
    if store {
        return GpuLimitsMigrationDepth6::StoreStrip;
    }
    if depth {
        return GpuLimitsMigrationDepth6::DepthModule;
    }
    GpuLimitsMigrationDepth6::None
}

/// Total GPULimits migration 6 criteria in registry (PH-S1939).
pub fn gpu_limits_migration6_criteria_total() -> usize {
    GPU_LIMITS_MIGRATION6_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn gpu_limits_migration_depth6_stub_ph_s1939() {
        assert_eq!(
            gpu_limits_migration_depth6_stub(None),
            GpuLimitsMigrationDepth6::None
        );
        assert_eq!(
            gpu_limits_migration_depth6_stub(Some(&json!({"gpu_limits_migration_depth6": true}))),
            GpuLimitsMigrationDepth6::DepthModule
        );
        assert_eq!(
            gpu_limits_migration_depth6_stub(Some(&json!({
                "gpu_limits_migration_depth6": true,
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
            GpuLimitsMigrationDepth6::FullBand130
        );
        assert_eq!(GPU_LIMITS_MIGRATION6_CRITERIA.len(), 10);
        assert_eq!(gpu_limits_migration6_criteria_total(), 10);
        assert!(FM_BAND130_ROWS.contains(&"PH-S1939.S1948"));
    }
}