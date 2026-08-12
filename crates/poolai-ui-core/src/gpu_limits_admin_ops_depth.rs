//! GPULimits admin/ops glue band depth (PH-S1879…S1888, band 124 — enterprise phase H).
//!
//! Mirrors the band-104 ratio96 admin/ops slice: a dashboard store strip that reads the
//! durable GPU-limits store (`docs/development/gpu_limits.json`) via `GET /api/v1/gpu-limits`,
//! a refresh ops glue button, HTML contracts, verify/loc-audit hooks and docs canon.

use serde_json::Value;

/// GPULimits admin/ops glue depth flags (store strip / refresh ops glue / verify hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuLimitsAdminOpsDepth {
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
    FullBand124,
}

/// GPULimits admin/ops criteria registry (PH-S1879): id · marker · doc path.
pub const GPU_LIMITS_ADMIN_OPS_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "gpu_limits_admin_ops_depth",
        "GpuLimitsAdminOpsDepth",
        "crates/poolai-ui-core/src/gpu_limits_admin_ops_depth.rs",
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
        "gpu_limits_admin_ops_integration",
        "tests/gpu_limits_admin_ops_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_GPU_LIMITS_ADMIN_OPS",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "gpu_limits_admin_ops_band124_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--gpu-limits-admin-ops",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "docs_canon",
        "GPU_LIMITS_ADMIN_OPS.md",
        "docs/development/GPU_LIMITS_ADMIN_OPS.md",
    ),
    (
        "ratio_hold",
        "gpu-limits-admin-ops",
        "docs/development/RUN_LOCAL.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1879_integration",
        "tests/galaxy_horizon_s1879_integration.rs",
    ),
];

/// `poolai-loc-audit --gpu-limits-admin-ops` case names (PH-S1884).
pub const GPU_LIMITS_ADMIN_OPS_CASES: &[&str] = &[
    "gpu_limits_admin_ops_depth",
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

/// FM §5.105 band-124 marker rows.
pub const FM_BAND124_ROWS: &[&str] = &[
    "5.105",
    "GPULimits admin/ops glue",
    "PH-S1879…S1888",
    "gpu_limits_admin_ops_depth",
];

/// GPULimits admin/ops adoption markers for band 124.
pub const GPU_LIMITS_ADMIN_OPS_BAND124_ROWS: &[&str] = &[
    "PH-S1879",
    "gpu_limits_admin_ops_depth",
    "PH-S1880",
    "gpu_limits_store_wire_json",
    "PH-S1881",
    "gpu_limits_admin_ops_contracts",
    "PH-S1882",
    "gpu-limits-store-badge",
    "PH-S1884",
    "--gpu-limits-admin-ops",
    "PH-S1888",
];

/// Classify GPULimits admin/ops band depth from optional feature stub (PH-S1879).
pub fn gpu_limits_admin_ops_depth_stub(features: Option<&Value>) -> GpuLimitsAdminOpsDepth {
    let Some(f) = features else {
        return GpuLimitsAdminOpsDepth::None;
    };
    let enabled = |key| f.get(key).and_then(Value::as_bool).unwrap_or(false);
    let depth = enabled("gpu_limits_admin_ops_depth");
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
        return GpuLimitsAdminOpsDepth::FullBand124;
    }
    if close || ratio {
        return GpuLimitsAdminOpsDepth::RatioHold;
    }
    if docs {
        return GpuLimitsAdminOpsDepth::DocsCanon;
    }
    if loc {
        return GpuLimitsAdminOpsDepth::LocAuditFlag;
    }
    if smoke {
        return GpuLimitsAdminOpsDepth::StandSmokeExport;
    }
    if verify {
        return GpuLimitsAdminOpsDepth::VerifyDevStandHook;
    }
    if html {
        return GpuLimitsAdminOpsDepth::HtmlContracts;
    }
    if query {
        return GpuLimitsAdminOpsDepth::QueryOpsGlue;
    }
    if store {
        return GpuLimitsAdminOpsDepth::StoreStrip;
    }
    if depth {
        return GpuLimitsAdminOpsDepth::DepthModule;
    }
    GpuLimitsAdminOpsDepth::None
}

/// Total GPULimits admin/ops criteria in registry (PH-S1879).
pub fn gpu_limits_admin_ops_criteria_total() -> usize {
    GPU_LIMITS_ADMIN_OPS_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn gpu_limits_admin_ops_depth_stub_ph_s1879() {
        assert_eq!(
            gpu_limits_admin_ops_depth_stub(None),
            GpuLimitsAdminOpsDepth::None
        );
        assert_eq!(
            gpu_limits_admin_ops_depth_stub(Some(&json!({"gpu_limits_admin_ops_depth": true}))),
            GpuLimitsAdminOpsDepth::DepthModule
        );
        assert_eq!(
            gpu_limits_admin_ops_depth_stub(Some(&json!({
                "gpu_limits_admin_ops_depth": true,
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
            GpuLimitsAdminOpsDepth::FullBand124
        );
        assert_eq!(GPU_LIMITS_ADMIN_OPS_CRITERIA.len(), 10);
        assert_eq!(gpu_limits_admin_ops_criteria_total(), 10);
        assert!(FM_BAND124_ROWS.contains(&"PH-S1879…S1888"));
    }
}
