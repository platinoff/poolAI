//! GPULimits migration band depth 3 (PH-S1909.S1918, band 127 — enterprise phase H).
//!
//! Mirrors the band-104 ratio96 admin/ops slice and band-126 migration depth.
//! Reads the durable GPU-limits store via `GET /api/v1/gpu-limits`,
//! a refresh ops glue button, HTML contracts, verify/loc-audit hooks and docs canon.

use serde_json::Value;

/// GPULimits migration band depth flags 3 (store strip / refresh ops glue / verify hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuLimitsMigrationDepth3 {
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
    FullBand127,
}

/// GPULimits migration criteria registry 3 (PH-S1909): id · marker · doc path.
pub const GPU_LIMITS_MIGRATION3_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "gpu_limits_migration_depth3",
        "GpuLimitsMigrationDepth3",
        "crates/poolai-ui-core/src/gpu_limits_migration3_depth.rs",
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
        "gpu_limits_migration3_integration",
        "tests/gpu_limits_migration3_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_GPU_LIMITS_ADMIN_OPS",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "gpu_limits_migration3_band127_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--gpu-limits-admin-ops",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "docs_canon",
        "GPU_LIMITS_CONTINUATION3.md",
        "docs/development/GPU_LIMITS_CONTINUATION3.md",
    ),
    (
        "ratio_hold",
        "gpu-limits-admin-ops",
        "docs/development/RUN_LOCAL.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1909_integration",
        "tests/galaxy_horizon_s1909_integration.rs",
    ),
];

/// `poolai-loc-audit --gpu-limits-admin-ops` case names (PH-S1904).
pub const GPU_LIMITS_MIGRATION3_CASES: &[&str] = &[
    "gpu_limits_migration_depth3",
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

/// FM §5.127 band-127 marker rows.
pub const FM_BAND127_ROWS: &[&str] = &[
    "5.127",
    "GPULimits migration 3",
    "PH-S1909.S1918",
    "gpu_limits_migration_depth3",
];

/// GPULimits migration 3 adoption markers for band 127.
pub const GPU_LIMITS_MIGRATION3_BAND127_ROWS: &[&str] = &[
    "PH-S1909",
    "gpu_limits_migration_depth3",
    "PH-S1910",
    "gpu-limits-store-badge",
    "PH-S1911",
    "gpu_limits_migration3_contracts",
    "PH-S1912",
    "refreshGpuLimits",
    "PH-S1914",
    "--gpu-limits-admin-ops",
    "PH-S1918",
];

/// Classification GPULimits migration band depth 3 from optional feature stub (PH-S1909).
pub fn gpu_limits_migration_depth3_stub(features: Option<&Value>) -> GpuLimitsMigrationDepth3 {
    let Some(f) = features else {
        return GpuLimitsMigrationDepth3::None;
    };
    let enabled = |key| f.get(key).and_then(Value::as_bool).unwrap_or(false);
    let depth = enabled("gpu_limits_migration_depth3");
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
        return GpuLimitsMigrationDepth3::FullBand127;
    }
    if close || ratio {
        return GpuLimitsMigrationDepth3::RatioHold;
    }
    if docs {
        return GpuLimitsMigrationDepth3::DocsCanon;
    }
    if loc {
        return GpuLimitsMigrationDepth3::LocAuditFlag;
    }
    if smoke {
        return GpuLimitsMigrationDepth3::StandSmokeExport;
    }
    if verify {
        return GpuLimitsMigrationDepth3::VerifyDevStandHook;
    }
    if html {
        return GpuLimitsMigrationDepth3::HtmlContracts;
    }
    if query {
        return GpuLimitsMigrationDepth3::QueryOpsGlue;
    }
    if store {
        return GpuLimitsMigrationDepth3::StoreStrip;
    }
    if depth {
        return GpuLimitsMigrationDepth3::DepthModule;
    }
    GpuLimitsMigrationDepth3::None
}

/// Total GPULimits migration 3 criteria in registry (PH-S1909).
pub fn gpu_limits_migration3_criteria_total() -> usize {
    GPU_LIMITS_MIGRATION3_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn gpu_limits_migration_depth3_stub_ph_s1909() {
        assert_eq!(
            gpu_limits_migration_depth3_stub(None),
            GpuLimitsMigrationDepth3::None
        );
        assert_eq!(
            gpu_limits_migration_depth3_stub(Some(&json!({"gpu_limits_migration_depth3": true}))),
            GpuLimitsMigrationDepth3::DepthModule
        );
        assert_eq!(
            gpu_limits_migration_depth3_stub(Some(&json!({
                "gpu_limits_migration_depth3": true,
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
            GpuLimitsMigrationDepth3::FullBand127
        );
        assert_eq!(GPU_LIMITS_MIGRATION3_CRITERIA.len(), 10);
        assert_eq!(gpu_limits_migration3_criteria_total(), 10);
        assert!(FM_BAND127_ROWS.contains(&"PH-S1909.S1918"));
    }
}
