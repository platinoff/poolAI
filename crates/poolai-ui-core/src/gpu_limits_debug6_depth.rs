//! GPULimits UI debugging band depth 6 (PH-S1999.S2008, band 136 — galaxy phase IV).
//!
//! Mirrors the band-108 ratio96 admin/ops slice and band-135 migration depth.
//! Provides UI debugging depth tracking via `GET /api/v1/debug/ui`,
//! a refresh ops glue button, HTML contracts, verify/loc-audit hooks and docs canon.

use serde_json::Value;

/// GPULimits UI debugging band depth flags 6 (store strip / refresh ops glue / verify hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuLimitsDebugDepth6 {
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
    FullBand136,
}

/// GPULimits UI debugging criteria registry 6 (PH-S1999): id · marker · doc path.
pub const GPU_LIMITS_DEBUG6_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "gpu_limits_debug_depth6",
        "GpuLimitsDebugDepth6",
        "crates/poolai-ui-core/src/gpu_limits_debug6_depth.rs",
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
        "gpu_limits_debug6_integration",
        "tests/gpu_limits_debug6_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_DEBUG_UI_ADMIN_OPS",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "gpu_limits_debug6_band136_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--debug-ui-loc-audit",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "docs_canon",
        "GPU_DEBUG_CONTINUATION6.md",
        "docs/development/GPU_DEBUG_CONTINUATION6.md",
    ),
    (
        "ratio_hold",
        "debug-limits-admin-ops",
        "docs/development/RUN_LOCAL.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1999_integration",
        "tests/galaxy_horizon_s1999_integration.rs",
    ),
];

/// `poolai-loc-audit --debug-ui-loc-audit` case names (PH-S1904).
pub const GPU_LIMITS_DEBUG6_CASES: &[&str] = &[
    "gpu_limits_debug_depth6",
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

/// FM §5.136 band-136 marker rows.
pub const FM_BAND136_ROWS: &[&str] = &[
    "5.136",
    "GPULimits UI debug 6",
    "PH-S1999.S2008",
    "gpu_limits_debug_depth6",
];

/// GPULimits UI debug 6 adoption markers for band 136.
pub const GPU_LIMITS_DEBUG6_BAND136_ROWS: &[&str] = &[
    "PH-S1999",
    "gpu_limits_debug_depth6",
    "PH-S2000",
    "debug-limits-store-badge",
    "PH-S2001",
    "gpu_limits_debug6_contracts",
    "PH-S2002",
    "refreshDebugLimits",
    "PH-S2004",
    "--debug-ui-loc-audit",
    "PH-S2008",
];

/// Classification GPULimits UI debugging band depth 6 from optional feature stub (PH-S1999).
pub fn gpu_limits_debug_depth6_stub(features: Option<&Value>) -> GpuLimitsDebugDepth6 {
    let Some(f) = features else {
        return GpuLimitsDebugDepth6::None;
    };
    let enabled = |key| f.get(key).and_then(Value::as_bool).unwrap_or(false);
    let depth = enabled("gpu_limits_debug_depth6");
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
        return GpuLimitsDebugDepth6::FullBand136;
    }
    if close || ratio {
        return GpuLimitsDebugDepth6::RatioHold;
    }
    if docs {
        return GpuLimitsDebugDepth6::DocsCanon;
    }
    if loc {
        return GpuLimitsDebugDepth6::LocAuditFlag;
    }
    if smoke {
        return GpuLimitsDebugDepth6::StandSmokeExport;
    }
    if verify {
        return GpuLimitsDebugDepth6::VerifyDevStandHook;
    }
    if html {
        return GpuLimitsDebugDepth6::HtmlContracts;
    }
    if query {
        return GpuLimitsDebugDepth6::QueryOpsGlue;
    }
    if store {
        return GpuLimitsDebugDepth6::StoreStrip;
    }
    if depth {
        return GpuLimitsDebugDepth6::DepthModule;
    }
    GpuLimitsDebugDepth6::None
}

/// Total GPULimits UI debugging 6 criteria in registry (PH-S1999).
pub fn gpu_limits_debug6_criteria_total() -> usize {
    GPU_LIMITS_DEBUG6_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn gpu_limits_debug_depth6_stub_ph_s1999() {
        assert_eq!(
            gpu_limits_debug_depth6_stub(None),
            GpuLimitsDebugDepth6::None
        );
        assert_eq!(
            gpu_limits_debug_depth6_stub(Some(&json!({"gpu_limits_debug_depth6": true}))),
            GpuLimitsDebugDepth6::DepthModule
        );
        assert_eq!(
            gpu_limits_debug_depth6_stub(Some(&json!({
                "gpu_limits_debug_depth6": true,
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
            GpuLimitsDebugDepth6::FullBand136
        );
        assert_eq!(GPU_LIMITS_DEBUG6_CRITERIA.len(), 10);
        assert_eq!(gpu_limits_debug6_criteria_total(), 10);
        assert!(FM_BAND136_ROWS.contains(&"PH-S1999.S2008"));
    }
}
