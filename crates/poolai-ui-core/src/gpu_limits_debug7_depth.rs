//! GPULimits UI debugging band depth 7 (PH-S2009.S2018, band 137 — galaxy phase V).
//!
//! Mirrors the band-109 ratio96 admin/ops slice and band-136 migration depth.
//! Provides UI debugging depth tracking via `GET /api/v1/debug/ui`,
//! a refresh ops glue button, HTML contracts, verify/loc-audit hooks and docs canon.

use serde_json::Value;

/// GPULimits UI debugging band depth flags 7 (store strip / refresh ops glue / verify hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuLimitsDebugDepth7 {
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
    FullBand137,
}

/// GPULimits UI debugging criteria registry 7 (PH-S2009): id · marker · doc path.
pub const GPU_LIMITS_DEBUG7_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "gpu_limits_debug_depth7",
        "GpuLimitsDebugDepth7",
        "crates/poolai-ui-core/src/gpu_limits_debug7_depth.rs",
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
        "gpu_limits_debug7_integration",
        "tests/gpu_limits_debug7_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_DEBUG_UI_ADMIN_OPS",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "gpu_limits_debug7_band137_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--debug-ui-loc-audit",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "docs_canon",
        "GPU_DEBUG_CONTINUATION7.md",
        "docs/development/GPU_DEBUG_CONTINUATION7.md",
    ),
    (
        "ratio_hold",
        "debug-limits-admin-ops",
        "docs/development/RUN_LOCAL.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s2009_integration",
        "tests/galaxy_horizon_s2009_integration.rs",
    ),
];

/// `poolai-loc-audit --debug-ui-loc-audit` case names (PH-S1904).
pub const GPU_LIMITS_DEBUG7_CASES: &[&str] = &[
    "gpu_limits_debug_depth7",
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

/// FM §5.137 band-137 marker rows.
pub const FM_BAND137_ROWS: &[&str] = &[
    "5.137",
    "GPULimits UI debug 7",
    "PH-S2009.S2018",
    "gpu_limits_debug_depth7",
];

/// GPULimits UI debug 7 adoption markers for band 137.
pub const GPU_LIMITS_DEBUG7_BAND137_ROWS: &[&str] = &[
    "PH-S2009",
    "gpu_limits_debug_depth7",
    "PH-S2010",
    "debug-limits-store-badge",
    "PH-S2011",
    "gpu_limits_debug7_contracts",
    "PH-S2012",
    "refreshDebugLimits",
    "PH-S2014",
    "--debug-ui-loc-audit",
    "PH-S2018",
];

/// Classification GPULimits UI debugging band depth 7 from optional feature stub (PH-S2009).
pub fn gpu_limits_debug_depth7_stub(features: Option<&Value>) -> GpuLimitsDebugDepth7 {
    let Some(f) = features else {
        return GpuLimitsDebugDepth7::None;
    };
    let enabled = |key| f.get(key).and_then(Value::as_bool).unwrap_or(false);
    let depth = enabled("gpu_limits_debug_depth7");
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
        return GpuLimitsDebugDepth7::FullBand137;
    }
    if close || ratio {
        return GpuLimitsDebugDepth7::RatioHold;
    }
    if docs {
        return GpuLimitsDebugDepth7::DocsCanon;
    }
    if loc {
        return GpuLimitsDebugDepth7::LocAuditFlag;
    }
    if smoke {
        return GpuLimitsDebugDepth7::StandSmokeExport;
    }
    if verify {
        return GpuLimitsDebugDepth7::VerifyDevStandHook;
    }
    if html {
        return GpuLimitsDebugDepth7::HtmlContracts;
    }
    if query {
        return GpuLimitsDebugDepth7::QueryOpsGlue;
    }
    if store {
        return GpuLimitsDebugDepth7::StoreStrip;
    }
    if depth {
        return GpuLimitsDebugDepth7::DepthModule;
    }
    GpuLimitsDebugDepth7::None
}

/// Total GPULimits UI debugging 7 criteria in registry (PH-S2009).
pub fn gpu_limits_debug7_criteria_total() -> usize {
    GPU_LIMITS_DEBUG7_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn gpu_limits_debug_depth7_stub_ph_s2009() {
        assert_eq!(
            gpu_limits_debug_depth7_stub(None),
            GpuLimitsDebugDepth7::None
        );
        assert_eq!(
            gpu_limits_debug_depth7_stub(Some(&json!({"gpu_limits_debug_depth7": true}))),
            GpuLimitsDebugDepth7::DepthModule
        );
        assert_eq!(
            gpu_limits_debug_depth7_stub(Some(&json!({
                "gpu_limits_debug_depth7": true,
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
            GpuLimitsDebugDepth7::FullBand137
        );
        assert_eq!(GPU_LIMITS_DEBUG7_CRITERIA.len(), 10);
        assert_eq!(gpu_limits_debug7_criteria_total(), 10);
        assert!(FM_BAND137_ROWS.contains(&"PH-S2009.S2018"));
    }
}