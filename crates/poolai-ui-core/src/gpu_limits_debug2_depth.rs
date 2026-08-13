//! GPULimits UI debugging band depth 2 (PH-S1959.S1968, band 132 — enterprise phase H).
//!
//! Mirrors the band-104 ratio96 admin/ops slice and band-131 migration depth.
//! Provides UI debugging depth tracking via `GET /api/v1/debug/ui`,
//! a refresh ops glue button, HTML contracts, verify/loc-audit hooks and docs canon.

use serde_json::Value;

/// GPULimits UI debugging band depth flags 2 (store strip / refresh ops glue / verify hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuLimitsDebugDepth2 {
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
    FullBand132,
}

/// GPULimits UI debugging criteria registry 2 (PH-S1959): id · marker · doc path.
pub const GPU_LIMITS_DEBUG2_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "gpu_limits_debug_depth2",
        "GpuLimitsDebugDepth2",
        "crates/poolai-ui-core/src/gpu_limits_debug2_depth.rs",
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
        "gpu_limits_debug2_integration",
        "tests/gpu_limits_debug2_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_DEBUG_UI_ADMIN_OPS",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "gpu_limits_debug2_band132_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--debug-ui-loc-audit",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "docs_canon",
        "GPU_DEBUG_CONTINUATION2.md",
        "docs/development/GPU_DEBUG_CONTINUATION2.md",
    ),
    (
        "ratio_hold",
        "debug-limits-admin-ops",
        "docs/development/RUN_LOCAL.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1959_integration",
        "tests/galaxy_horizon_s1959_integration.rs",
    ),
];

/// `poolai-loc-audit --debug-ui-loc-audit` case names (PH-S1904).
pub const GPU_LIMITS_DEBUG2_CASES: &[&str] = &[
    "gpu_limits_debug_depth2",
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

/// FM §5.132 band-132 marker rows.
pub const FM_BAND132_ROWS: &[&str] = &[
    "5.132",
    "GPULimits UI debug 2",
    "PH-S1959.S1968",
    "gpu_limits_debug_depth2",
];

/// GPULimits UI debug 2 adoption markers for band 132.
pub const GPU_LIMITS_DEBUG2_BAND132_ROWS: &[&str] = &[
    "PH-S1959",
    "gpu_limits_debug_depth2",
    "PH-S1960",
    "debug-limits-store-badge",
    "PH-S1961",
    "gpu_limits_debug2_contracts",
    "PH-S1962",
    "refreshDebugLimits",
    "PH-S1964",
    "--debug-ui-loc-audit",
    "PH-S1968",
];

/// Classification GPULimits UI debugging band depth 2 from optional feature stub (PH-S1959).
pub fn gpu_limits_debug_depth2_stub(features: Option<&Value>) -> GpuLimitsDebugDepth2 {
    let Some(f) = features else {
        return GpuLimitsDebugDepth2::None;
    };
    let enabled = |key| f.get(key).and_then(Value::as_bool).unwrap_or(false);
    let depth = enabled("gpu_limits_debug_depth2");
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
        return GpuLimitsDebugDepth2::FullBand132;
    }
    if close || ratio {
        return GpuLimitsDebugDepth2::RatioHold;
    }
    if docs {
        return GpuLimitsDebugDepth2::DocsCanon;
    }
    if loc {
        return GpuLimitsDebugDepth2::LocAuditFlag;
    }
    if smoke {
        return GpuLimitsDebugDepth2::StandSmokeExport;
    }
    if verify {
        return GpuLimitsDebugDepth2::VerifyDevStandHook;
    }
    if html {
        return GpuLimitsDebugDepth2::HtmlContracts;
    }
    if query {
        return GpuLimitsDebugDepth2::QueryOpsGlue;
    }
    if store {
        return GpuLimitsDebugDepth2::StoreStrip;
    }
    if depth {
        return GpuLimitsDebugDepth2::DepthModule;
    }
    GpuLimitsDebugDepth2::None
}

/// Total GPULimits UI debugging 2 criteria in registry (PH-S1959).
pub fn gpu_limits_debug2_criteria_total() -> usize {
    GPU_LIMITS_DEBUG2_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn gpu_limits_debug_depth2_stub_ph_s1959() {
        assert_eq!(
            gpu_limits_debug_depth2_stub(None),
            GpuLimitsDebugDepth2::None
        );
        assert_eq!(
            gpu_limits_debug_depth2_stub(Some(&json!({"gpu_limits_debug_depth2": true}))),
            GpuLimitsDebugDepth2::DepthModule
        );
        assert_eq!(
            gpu_limits_debug_depth2_stub(Some(&json!({
                "gpu_limits_debug_depth2": true,
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
            GpuLimitsDebugDepth2::FullBand132
        );
        assert_eq!(GPU_LIMITS_DEBUG2_CRITERIA.len(), 10);
        assert_eq!(gpu_limits_debug2_criteria_total(), 10);
        assert!(FM_BAND132_ROWS.contains(&"PH-S1959.S1968"));
    }
}