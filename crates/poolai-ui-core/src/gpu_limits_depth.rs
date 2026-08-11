//! GPULimits band depth (PH-S1859…S1868, band 122 — enterprise phase H).
//!
//! Consolidates the GPU admission + worker-limit store/wire slice under one
//! depth gate. Pattern mirror: band 107 `ratio96_docs_canon_depth`.

use serde_json::Value;

/// GPULimits depth flags (registry / store / contracts / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuLimitsDepth {
    None,
    DepthModule,
    StoreWireSlice,
    ApiContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand122,
}

/// Band 122 GPU-limits canon doc filenames covered by the slice (PH-S1860).
pub const GPU_LIMITS_SLICES: &[&str] = &[
    "GPU_LIMITS.md",
    "docs/development/GPU_LIMITS.md",
    "docs/development/gpu_limits.json",
];

/// GPULimits criteria registry (PH-S1859): id · marker · doc path.
pub const GPU_LIMITS_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "gpu_limits_depth",
        "GpuLimitsDepth",
        "crates/poolai-ui-core/src/gpu_limits_depth.rs",
    ),
    (
        "store_wire",
        "gpu_limits_store",
        "crates/poolai-ui-core/src/gpu_limits_store.rs",
    ),
    (
        "durable_store",
        "docs/development/gpu_limits.json",
        "docs/development/GPU_LIMITS.md",
    ),
    (
        "api_contracts",
        "gpu_limits_integration",
        "tests/gpu_limits_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_GPU_LIMITS",
        "bin/verify-dev-stand.sh",
    ),
    ("run_poolai_flag", "--gpu-limits", "bin/run-poolai.sh"),
    (
        "stand_smoke_export",
        "gpu_limits_band122_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--gpu-limits",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "docs_canon",
        "docs/development/GPU_LIMITS.md",
        "docs/development/GPU_LIMITS.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1859_integration",
        "tests/galaxy_horizon_s1859_integration.rs",
    ),
];

/// `poolai-loc-audit --gpu-limits` case names (PH-S1864).
pub const GPU_LIMITS_CASES: &[&str] = &[
    "gpu_limits_depth",
    "store_wire",
    "durable_store",
    "api_contracts",
    "verify_dev_stand_hook",
    "run_poolai_flag",
    "stand_smoke_export",
    "loc_audit_flag",
    "docs_canon",
    "band_close",
];

/// FM §5.103 band-122 marker rows.
pub const FM_BAND122_ROWS: &[&str] = &[
    "5.103",
    "GPULimits store/wire",
    "PH-S1859…S1868",
    "gpu_limits_depth",
];

/// GPULimits adoption markers for band 122.
pub const GPU_LIMITS_BAND122_ROWS: &[&str] = &[
    "PH-S1859",
    "gpu_limits_depth",
    "PH-S1860",
    "gpu_limits_store",
    "PH-S1861",
    "gpu_limits_integration",
    "PH-S1862",
    "VERIFY_GPU_LIMITS",
    "PH-S1864",
    "--gpu-limits",
    "PH-S1868",
];

/// Production-verify stub: how many GPU-limits slice markers are referenced (PH-S1860).
pub fn gpu_limits_slices_met(canon_src: &str) -> (usize, usize) {
    let total = GPU_LIMITS_SLICES.len();
    let met = GPU_LIMITS_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify GPULimits band depth from optional feature stub (PH-S1859).
pub fn gpu_limits_depth_stub(features: Option<&Value>) -> GpuLimitsDepth {
    let Some(f) = features else {
        return GpuLimitsDepth::None;
    };
    let depth = f
        .get("gpu_limits_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let store = f
        .get("store_wire")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let contracts = f
        .get("api_contracts")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let verify = f
        .get("verify_dev_stand_hook")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let export = f
        .get("stand_smoke_export")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let loc = f
        .get("loc_audit_flag")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let docs = f
        .get("gpu_limits_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let ratio = f
        .get("ratio_hold")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let close = f
        .get("band_close")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if depth && store && contracts && verify && export && loc && docs && ratio && close {
        return GpuLimitsDepth::FullBand122;
    }
    if close || ratio {
        return GpuLimitsDepth::RatioHold;
    }
    if docs {
        return GpuLimitsDepth::DocsCanon;
    }
    if loc {
        return GpuLimitsDepth::LocAuditFlag;
    }
    if export {
        return GpuLimitsDepth::StandSmokeExport;
    }
    if verify {
        return GpuLimitsDepth::VerifyDevStandHook;
    }
    if contracts {
        return GpuLimitsDepth::ApiContracts;
    }
    if store {
        return GpuLimitsDepth::StoreWireSlice;
    }
    if depth {
        return GpuLimitsDepth::DepthModule;
    }
    GpuLimitsDepth::None
}

/// Total GPULimits criteria in registry (PH-S1859).
pub fn gpu_limits_criteria_total() -> usize {
    GPU_LIMITS_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn gpu_limits_depth_stub_ph_s1859() {
        assert_eq!(gpu_limits_depth_stub(None), GpuLimitsDepth::None);
        assert_eq!(
            gpu_limits_depth_stub(Some(&json!({"gpu_limits_depth": true}))),
            GpuLimitsDepth::DepthModule
        );
        assert_eq!(
            gpu_limits_depth_stub(Some(&json!({
                "gpu_limits_depth": true,
                "store_wire": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "gpu_limits_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            GpuLimitsDepth::FullBand122
        );
        assert_eq!(GPU_LIMITS_CRITERIA.len(), 10);
        assert_eq!(gpu_limits_criteria_total(), 10);
        assert_eq!(GPU_LIMITS_SLICES.len(), 3);
        assert!(FM_BAND122_ROWS.contains(&"PH-S1859…S1868"));
    }

    #[test]
    fn gpu_limits_slices_met_ph_s1860() {
        let src = "GPU_LIMITS.md docs/development/GPU_LIMITS.md docs/development/gpu_limits.json";
        assert_eq!(gpu_limits_slices_met(src), (3, 3));
        assert_eq!(gpu_limits_slices_met("GPU_LIMITS.md"), (1, 3));
    }
}
