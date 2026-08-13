//! GPULimits UI debugging band depth 5 (PH-S1989.S1998, band 135 — galaxy phase III).
//!
//! Mirrors the band-107 ratio96 admin/ops slice and band-134 migration depth.
//! Provides UI debugging depth tracking via `GET /api/v1/debug/ui`,
//! a refresh ops glue button, HTML contracts, verify/loc-audit hooks and docs canon.

use serde_json::Value;

/// GPULimits UI debugging band depth flags 5 (store strip / refresh ops glue / verify hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuLimitsDebugDepth5 {
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
    FullBand135,
}

/// GPULimits UI debugging criteria registry 5 (PH-S1989): id · marker · doc path.
pub const GPU_LIMITS_DEBUG5_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "gpu_limits_debug_depth5",
        "GpuLimitsDebugDepth5",
        "crates/poolai-ui-core/src/gpu_limits_debug5_depth.rs",
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
        "gpu_limits_debug5_integration",
        "tests/gpu_limits_debug5_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_DEBUG_UI_ADMIN_OPS",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "gpu_limits_debug5_band135_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--debug-ui-loc-audit",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "docs_canon",
        "GPU_DEBUG_CONTINUATION5.md",
        "docs/development/GPU_DEBUG_CONTINUATION5.md",
    ),
    (
        "ratio_hold",
        "debug-limits-admin-ops",
        "docs/development/RUN_LOCAL.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1989_integration",
        "tests/galaxy_horizon_s1989_integration.rs",
    ),
];

/// `poolai-loc-audit --debug-ui-loc-audit` case names (PH-S1904).
pub const GPU_LIMITS_DEBUG5_CASES: &[&str] = &[
    "gpu_limits_debug_depth5",
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

/// FM §5.135 band-135 marker rows.
pub const FM_BAND135_ROWS: &[&str] = &[
    "5.135",
    "GPULimits UI debug 5",
    "PH-S1989.S1998",
    "gpu_limits_debug_depth5",
];

/// GPULimits UI debug 5 adoption markers for band 135.
pub const GPU_LIMITS_DEBUG5_BAND135_ROWS: &[&str] = &[
    "PH-S1989",
    "gpu_limits_debug_depth5",
    "PH-S1990",
    "debug-limits-store-badge",
    "PH-S1991",
    "gpu_limits_debug5_contracts",
    "PH-S1992",
    "refreshDebugLimits",
    "PH-S1994",
    "--debug-ui-loc-audit",
    "PH-S1998",
];

/// Classification GPULimits UI debugging band depth 5 from optional feature stub (PH-S1989).
pub fn gpu_limits_debug_depth5_stub(features: Option<&Value>) -> GpuLimitsDebugDepth5 {
    let Some(f) = features else {
        return GpuLimitsDebugDepth5::None;
    };
    let enabled = |key| f.get(key).and_then(Value::as_bool).unwrap_or(false);
    let depth = enabled("gpu_limits_debug_depth5");
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
        return GpuLimitsDebugDepth5::FullBand135;
    }
    if close || ratio {
        return GpuLimitsDebugDepth5::RatioHold;
    }
    if docs {
        return GpuLimitsDebugDepth5::DocsCanon;
    }
    if loc {
        return GpuLimitsDebugDepth5::LocAuditFlag;
    }
    if smoke {
        return GpuLimitsDebugDepth5::StandSmokeExport;
    }
    if verify {
        return GpuLimitsDebugDepth5::VerifyDevStandHook;
    }
    if html {
        return GpuLimitsDebugDepth5::HtmlContracts;
    }
    if query {
        return GpuLimitsDebugDepth5::QueryOpsGlue;
    }
    if store {
        return GpuLimitsDebugDepth5::StoreStrip;
    }
    if depth {
        return GpuLimitsDebugDepth5::DepthModule;
    }
    GpuLimitsDebugDepth5::None
}

/// Total GPULimits UI debugging 5 criteria in registry (PH-S1989).
pub fn gpu_limits_debug5_criteria_total() -> usize {
    GPU_LIMITS_DEBUG5_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn gpu_limits_debug_depth5_stub_ph_s1989() {
        assert_eq!(
            gpu_limits_debug_depth5_stub(None),
            GpuLimitsDebugDepth5::None
        );
        assert_eq!(
            gpu_limits_debug_depth5_stub(Some(&json!({"gpu_limits_debug_depth5": true}))),
            GpuLimitsDebugDepth5::DepthModule
        );
        assert_eq!(
            gpu_limits_debug_depth5_stub(Some(&json!({
                "gpu_limits_debug_depth5": true,
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
            GpuLimitsDebugDepth5::FullBand135
        );
        assert_eq!(GPU_LIMITS_DEBUG5_CRITERIA.len(), 10);
        assert_eq!(gpu_limits_debug5_criteria_total(), 10);
        assert!(FM_BAND135_ROWS.contains(&"PH-S1989.S1998"));
    }
}