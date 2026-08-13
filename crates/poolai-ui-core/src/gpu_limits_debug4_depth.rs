//! GPULimits UI debugging band depth 4 (PH-S1979.S1988, band 134 — galaxy phase II).
//!
//! Mirrors the band-106 ratio96 admin/ops slice and band-133 migration depth.
//! Provides UI debugging depth tracking via `GET /api/v1/debug/ui`,
//! a refresh ops glue button, HTML contracts, verify/loc-audit hooks and docs canon.

use serde_json::Value;

/// GPULimits UI debugging band depth flags 4 (store strip / refresh ops glue / verify hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuLimitsDebugDepth4 {
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
    FullBand134,
}

/// GPULimits UI debugging criteria registry 4 (PH-S1979): id · marker · doc path.
pub const GPU_LIMITS_DEBUG4_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "gpu_limits_debug_depth4",
        "GpuLimitsDebugDepth4",
        "crates/poolai-ui-core/src/gpu_limits_debug4_depth.rs",
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
        "gpu_limits_debug4_integration",
        "tests/gpu_limits_debug4_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_DEBUG_UI_ADMIN_OPS",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "gpu_limits_debug4_band134_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--debug-ui-loc-audit",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "docs_canon",
        "GPU_DEBUG_CONTINUATION4.md",
        "docs/development/GPU_DEBUG_CONTINUATION4.md",
    ),
    (
        "ratio_hold",
        "debug-limits-admin-ops",
        "docs/development/RUN_LOCAL.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1979_integration",
        "tests/galaxy_horizon_s1979_integration.rs",
    ),
];

/// `poolai-loc-audit --debug-ui-loc-audit` case names (PH-S1904).
pub const GPU_LIMITS_DEBUG4_CASES: &[&str] = &[
    "gpu_limits_debug_depth4",
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

/// FM §5.134 band-134 marker rows.
pub const FM_BAND134_ROWS: &[&str] = &[
    "5.134",
    "GPULimits UI debug 4",
    "PH-S1979.S1988",
    "gpu_limits_debug_depth4",
];

/// GPULimits UI debug 4 adoption markers for band 134.
pub const GPU_LIMITS_DEBUG4_BAND134_ROWS: &[&str] = &[
    "PH-S1979",
    "gpu_limits_debug_depth4",
    "PH-S1980",
    "debug-limits-store-badge",
    "PH-S1981",
    "gpu_limits_debug4_contracts",
    "PH-S1982",
    "refreshDebugLimits",
    "PH-S1984",
    "--debug-ui-loc-audit",
    "PH-S1988",
];

/// Classification GPULimits UI debugging band depth 4 from optional feature stub (PH-S1979).
pub fn gpu_limits_debug_depth4_stub(features: Option<&Value>) -> GpuLimitsDebugDepth4 {
    let Some(f) = features else {
        return GpuLimitsDebugDepth4::None;
    };
    let enabled = |key| f.get(key).and_then(Value::as_bool).unwrap_or(false);
    let depth = enabled("gpu_limits_debug_depth4");
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
        return GpuLimitsDebugDepth4::FullBand134;
    }
    if close || ratio {
        return GpuLimitsDebugDepth4::RatioHold;
    }
    if docs {
        return GpuLimitsDebugDepth4::DocsCanon;
    }
    if loc {
        return GpuLimitsDebugDepth4::LocAuditFlag;
    }
    if smoke {
        return GpuLimitsDebugDepth4::StandSmokeExport;
    }
    if verify {
        return GpuLimitsDebugDepth4::VerifyDevStandHook;
    }
    if html {
        return GpuLimitsDebugDepth4::HtmlContracts;
    }
    if query {
        return GpuLimitsDebugDepth4::QueryOpsGlue;
    }
    if store {
        return GpuLimitsDebugDepth4::StoreStrip;
    }
    if depth {
        return GpuLimitsDebugDepth4::DepthModule;
    }
    GpuLimitsDebugDepth4::None
}

/// Total GPULimits UI debugging 4 criteria in registry (PH-S1979).
pub fn gpu_limits_debug4_criteria_total() -> usize {
    GPU_LIMITS_DEBUG4_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn gpu_limits_debug_depth4_stub_ph_s1979() {
        assert_eq!(
            gpu_limits_debug_depth4_stub(None),
            GpuLimitsDebugDepth4::None
        );
        assert_eq!(
            gpu_limits_debug_depth4_stub(Some(&json!({"gpu_limits_debug_depth4": true}))),
            GpuLimitsDebugDepth4::DepthModule
        );
        assert_eq!(
            gpu_limits_debug_depth4_stub(Some(&json!({
                "gpu_limits_debug_depth4": true,
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
            GpuLimitsDebugDepth4::FullBand134
        );
        assert_eq!(GPU_LIMITS_DEBUG4_CRITERIA.len(), 10);
        assert_eq!(gpu_limits_debug4_criteria_total(), 10);
        assert!(FM_BAND134_ROWS.contains(&"PH-S1979.S1988"));
    }
}
