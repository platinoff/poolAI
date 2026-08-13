//! GPULimits UI debugging band depth 3 (PH-S1969.S1978, band 133 — galaxy phase I).
//!
//! Mirrors the band-105 ratio96 admin/ops slice and band-132 migration depth.
//! Provides UI debugging depth tracking via `GET /api/v1/debug/ui`,
//! a refresh ops glue button, HTML contracts, verify/loc-audit hooks and docs canon.

use serde_json::Value;

/// GPULimits UI debugging band depth flags 3 (store strip / refresh ops glue / verify hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuLimitsDebugDepth3 {
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
    FullBand133,
}

/// GPULimits UI debugging criteria registry 3 (PH-S1969): id · marker · doc path.
pub const GPU_LIMITS_DEBUG3_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "gpu_limits_debug_depth3",
        "GpuLimitsDebugDepth3",
        "crates/poolai-ui-core/src/gpu_limits_debug3_depth.rs",
    ),
    (
        "store_strip",
        "debug-limits-store-badge",
        "src/ui/admin/dashboard.rs",
    ),
    (
        "query_ops_glue",
        "refreshDebugLimits",
        "src/ui/admin/dashboard.rs",
    ),
    (
        "html_contracts",
        "gpu_limits_debug3_integration",
        "tests/gpu_limits_debug3_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_DEBUG_UI_ADMIN_OPS",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "gpu_limits_debug3_band133_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--debug-ui-loc-audit",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "docs_canon",
        "GPU_DEBUG_CONTINUATION3.md",
        "docs/development/GPU_DEBUG_CONTINUATION3.md",
    ),
    (
        "ratio_hold",
        "debug-limits-admin-ops",
        "docs/development/RUN_LOCAL.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1969_integration",
        "tests/galaxy_horizon_s1969_integration.rs",
    ),
];

/// `poolai-loc-audit --debug-ui-loc-audit` case names (PH-S1904).
pub const GPU_LIMITS_DEBUG3_CASES: &[&str] = &[
    "gpu_limits_debug_depth3",
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

/// FM §5.133 band-133 marker rows.
pub const FM_BAND133_ROWS: &[&str] = &[
    "5.133",
    "GPULimits UI debug 3",
    "PH-S1969.S1978",
    "gpu_limits_debug_depth3",
];

/// GPULimits UI debug 3 adoption markers for band 133.
pub const GPU_LIMITS_DEBUG3_BAND133_ROWS: &[&str] = &[
    "PH-S1969",
    "gpu_limits_debug_depth3",
    "PH-S1970",
    "debug-limits-store-badge",
    "PH-S1971",
    "gpu_limits_debug3_contracts",
    "PH-S1972",
    "refreshDebugLimits",
    "PH-S1974",
    "--debug-ui-loc-audit",
    "PH-S1978",
];

/// Classification GPULimits UI debugging band depth 3 from optional feature stub (PH-S1969).
pub fn gpu_limits_debug_depth3_stub(features: Option<&Value>) -> GpuLimitsDebugDepth3 {
    let Some(f) = features else {
        return GpuLimitsDebugDepth3::None;
    };
    let enabled = |key| f.get(key).and_then(Value::as_bool).unwrap_or(false);
    let depth = enabled("gpu_limits_debug_depth3");
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
        return GpuLimitsDebugDepth3::FullBand133;
    }
    if close || ratio {
        return GpuLimitsDebugDepth3::RatioHold;
    }
    if docs {
        return GpuLimitsDebugDepth3::DocsCanon;
    }
    if loc {
        return GpuLimitsDebugDepth3::LocAuditFlag;
    }
    if smoke {
        return GpuLimitsDebugDepth3::StandSmokeExport;
    }
    if verify {
        return GpuLimitsDebugDepth3::VerifyDevStandHook;
    }
    if html {
        return GpuLimitsDebugDepth3::HtmlContracts;
    }
    if query {
        return GpuLimitsDebugDepth3::QueryOpsGlue;
    }
    if store {
        return GpuLimitsDebugDepth3::StoreStrip;
    }
    if depth {
        return GpuLimitsDebugDepth3::DepthModule;
    }
    GpuLimitsDebugDepth3::None
}

/// Total GPULimits UI debugging 3 criteria in registry (PH-S1969).
pub fn gpu_limits_debug3_criteria_total() -> usize {
    GPU_LIMITS_DEBUG3_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn gpu_limits_debug_depth3_stub_ph_s1969() {
        assert_eq!(
            gpu_limits_debug_depth3_stub(None),
            GpuLimitsDebugDepth3::None
        );
        assert_eq!(
            gpu_limits_debug_depth3_stub(Some(&json!({"gpu_limits_debug_depth3": true}))),
            GpuLimitsDebugDepth3::DepthModule
        );
        assert_eq!(
            gpu_limits_debug_depth3_stub(Some(&json!({
                "gpu_limits_debug_depth3": true,
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
            GpuLimitsDebugDepth3::FullBand133
        );
        assert_eq!(GPU_LIMITS_DEBUG3_CRITERIA.len(), 10);
        assert_eq!(gpu_limits_debug3_criteria_total(), 10);
        assert!(FM_BAND133_ROWS.contains(&"PH-S1969.S1978"));
    }
}