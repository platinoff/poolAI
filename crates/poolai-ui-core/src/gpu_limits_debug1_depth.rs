//! GPULimits UI debugging band depth 1 (PH-S1949.S1958, band 131 — enterprise phase H).
//!
//! Mirrors the band-104 ratio96 admin/ops slice and band-130 migration depth.
//! Provides UI debugging depth tracking via `GET /api/v1/debug/ui`,
//! a refresh ops glue button, HTML contracts, verify/loc-audit hooks and docs canon.

use serde_json::Value;

/// GPULimits UI debugging band depth flags 1 (store strip / refresh ops glue / verify hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuLimitsDebugDepth1 {
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
    FullBand131,
}

/// GPULimits UI debugging criteria registry 1 (PH-S1949): id · marker · doc path.
pub const GPU_LIMITS_DEBUG1_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "gpu_limits_debug_depth1",
        "GpuLimitsDebugDepth1",
        "crates/poolai-ui-core/src/gpu_limits_debug1_depth.rs",
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
        "gpu_limits_debug1_integration",
        "tests/gpu_limits_debug1_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_DEBUG_UI_ADMIN_OPS",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "gpu_limits_debug1_band131_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--debug-ui-loc-audit",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "docs_canon",
        "GPU_DEBUG_CONTINUATION1.md",
        "docs/development/GPU_DEBUG_CONTINUATION1.md",
    ),
    (
        "ratio_hold",
        "debug-limits-admin-ops",
        "docs/development/RUN_LOCAL.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1949_integration",
        "tests/galaxy_horizon_s1949_integration.rs",
    ),
];

/// `poolai-loc-audit --debug-ui-loc-audit` case names (PH-S1904).
pub const GPU_LIMITS_DEBUG1_CASES: &[&str] = &[
    "gpu_limits_debug_depth1",
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

/// FM §5.131 band-131 marker rows.
pub const FM_BAND131_ROWS: &[&str] = &[
    "5.131",
    "GPULimits UI debug 1",
    "PH-S1949.S1958",
    "gpu_limits_debug_depth1",
];

/// GPULimits UI debug 1 adoption markers for band 131.
pub const GPU_LIMITS_DEBUG1_BAND131_ROWS: &[&str] = &[
    "PH-S1949",
    "gpu_limits_debug_depth1",
    "PH-S1950",
    "debug-limits-store-badge",
    "PH-S1951",
    "gpu_limits_debug1_contracts",
    "PH-S1952",
    "refreshDebugLimits",
    "PH-S1954",
    "--debug-ui-loc-audit",
    "PH-S1958",
];

/// Classification GPULimits UI debugging band depth 1 from optional feature stub (PH-S1949).
pub fn gpu_limits_debug_depth1_stub(features: Option<&Value>) -> GpuLimitsDebugDepth1 {
    let Some(f) = features else {
        return GpuLimitsDebugDepth1::None;
    };
    let enabled = |key| f.get(key).and_then(Value::as_bool).unwrap_or(false);
    let depth = enabled("gpu_limits_debug_depth1");
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
        return GpuLimitsDebugDepth1::FullBand131;
    }
    if close || ratio {
        return GpuLimitsDebugDepth1::RatioHold;
    }
    if docs {
        return GpuLimitsDebugDepth1::DocsCanon;
    }
    if loc {
        return GpuLimitsDebugDepth1::LocAuditFlag;
    }
    if smoke {
        return GpuLimitsDebugDepth1::StandSmokeExport;
    }
    if verify {
        return GpuLimitsDebugDepth1::VerifyDevStandHook;
    }
    if html {
        return GpuLimitsDebugDepth1::HtmlContracts;
    }
    if query {
        return GpuLimitsDebugDepth1::QueryOpsGlue;
    }
    if store {
        return GpuLimitsDebugDepth1::StoreStrip;
    }
    if depth {
        return GpuLimitsDebugDepth1::DepthModule;
    }
    GpuLimitsDebugDepth1::None
}

/// Total GPULimits UI debugging 1 criteria in registry (PH-S1949).
pub fn gpu_limits_debug1_criteria_total() -> usize {
    GPU_LIMITS_DEBUG1_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn gpu_limits_debug_depth1_stub_ph_s1949() {
        assert_eq!(
            gpu_limits_debug_depth1_stub(None),
            GpuLimitsDebugDepth1::None
        );
        assert_eq!(
            gpu_limits_debug_depth1_stub(Some(&json!({"gpu_limits_debug_depth1": true}))),
            GpuLimitsDebugDepth1::DepthModule
        );
        assert_eq!(
            gpu_limits_debug_depth1_stub(Some(&json!({
                "gpu_limits_debug_depth1": true,
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
            GpuLimitsDebugDepth1::FullBand131
        );
        assert_eq!(GPU_LIMITS_DEBUG1_CRITERIA.len(), 10);
        assert_eq!(gpu_limits_debug1_criteria_total(), 10);
        assert!(FM_BAND131_ROWS.contains(&"PH-S1949.S1958"));
    }
}
