//! GPULimits API band depth (PH-S1869…S1878, band 123 — enterprise phase H).
//!
//! Exposes the durable `gpu_limits.json` store over the HTTP surface
//! (`GET /api/v1/gpu-limits`) under one depth gate. Pattern mirror:
//! band 122 `gpu_limits_depth`.

use serde_json::Value;

/// GPULimits API depth flags (module / route / contracts / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuLimitsApiDepth {
    None,
    DepthModule,
    HttpRouteSlice,
    ApiContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand123,
}

/// Band 123 GPU-limits API canon doc filenames covered by the slice (PH-S1870).
pub const GPU_LIMITS_API_SLICES: &[&str] = &[
    "GPU_LIMITS.md",
    "docs/development/GPU_LIMITS.md",
    "docs/development/gpu_limits.json",
];

/// GPULimits API criteria registry (PH-S1869): id · marker · doc path.
pub const GPU_LIMITS_API_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "gpu_limits_api_depth",
        "GpuLimitsApiDepth",
        "crates/poolai-ui-core/src/gpu_limits_api_depth.rs",
    ),
    ("http_route", "gpu-limits", "src/network/api/system.rs"),
    (
        "wire_shape",
        "gpu_limits_store_wire_json",
        "src/network/api/system.rs",
    ),
    (
        "api_contracts",
        "gpu_limits_api_contracts_integration",
        "tests/gpu_limits_api_contracts_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_GPU_LIMITS_API",
        "bin/verify-dev-stand.sh",
    ),
    ("run_poolai_flag", "--gpu-limits-api", "bin/run-poolai.sh"),
    (
        "stand_smoke_export",
        "gpu_limits_api_band123_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--gpu-limits-api",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "docs_canon",
        "docs/development/GPU_LIMITS.md",
        "docs/development/GPU_LIMITS.md",
    ),
    (
        "band_close",
        "galaxy_horizon_s1869_integration",
        "tests/galaxy_horizon_s1869_integration.rs",
    ),
];

/// `poolai-loc-audit --gpu-limits-api` case names (PH-S1874).
pub const GPU_LIMITS_API_CASES: &[&str] = &[
    "gpu_limits_api_depth",
    "http_route",
    "wire_shape",
    "api_contracts",
    "verify_dev_stand_hook",
    "run_poolai_flag",
    "stand_smoke_export",
    "loc_audit_flag",
    "docs_canon",
    "band_close",
];

/// FM §5.104 band-123 marker rows.
pub const FM_BAND123_ROWS: &[&str] = &[
    "5.104",
    "GPULimits API contracts",
    "PH-S1869…S1878",
    "gpu_limits_api_depth",
];

/// GPULimits API adoption markers for band 123.
pub const GPU_LIMITS_API_BAND123_ROWS: &[&str] = &[
    "PH-S1869",
    "gpu_limits_api_depth",
    "PH-S1870",
    "gpu-limits",
    "PH-S1871",
    "gpu_limits_api_contracts_integration",
    "PH-S1872",
    "VERIFY_GPU_LIMITS_API",
    "PH-S1874",
    "--gpu-limits-api",
    "PH-S1878",
];

/// Production-verify stub: how many GPU-limits API slice markers are referenced (PH-S1870).
pub fn gpu_limits_api_slices_met(canon_src: &str) -> (usize, usize) {
    let total = GPU_LIMITS_API_SLICES.len();
    let met = GPU_LIMITS_API_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify GPULimits API band depth from optional feature stub (PH-S1869).
pub fn gpu_limits_api_depth_stub(features: Option<&Value>) -> GpuLimitsApiDepth {
    let Some(f) = features else {
        return GpuLimitsApiDepth::None;
    };
    let depth = f
        .get("gpu_limits_api_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let route = f
        .get("http_route")
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
        .get("gpu_limits_api_docs")
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

    if depth && route && contracts && verify && export && loc && docs && ratio && close {
        return GpuLimitsApiDepth::FullBand123;
    }
    if close || ratio {
        return GpuLimitsApiDepth::RatioHold;
    }
    if docs {
        return GpuLimitsApiDepth::DocsCanon;
    }
    if loc {
        return GpuLimitsApiDepth::LocAuditFlag;
    }
    if export {
        return GpuLimitsApiDepth::StandSmokeExport;
    }
    if verify {
        return GpuLimitsApiDepth::VerifyDevStandHook;
    }
    if contracts {
        return GpuLimitsApiDepth::ApiContracts;
    }
    if route {
        return GpuLimitsApiDepth::HttpRouteSlice;
    }
    if depth {
        return GpuLimitsApiDepth::DepthModule;
    }
    GpuLimitsApiDepth::None
}

/// Total GPULimits API criteria in registry (PH-S1869).
pub fn gpu_limits_api_criteria_total() -> usize {
    GPU_LIMITS_API_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn gpu_limits_api_depth_stub_ph_s1869() {
        assert_eq!(gpu_limits_api_depth_stub(None), GpuLimitsApiDepth::None);
        assert_eq!(
            gpu_limits_api_depth_stub(Some(&json!({"gpu_limits_api_depth": true}))),
            GpuLimitsApiDepth::DepthModule
        );
        assert_eq!(
            gpu_limits_api_depth_stub(Some(&json!({
                "gpu_limits_api_depth": true,
                "http_route": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "gpu_limits_api_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            GpuLimitsApiDepth::FullBand123
        );
        assert_eq!(GPU_LIMITS_API_CRITERIA.len(), 10);
        assert_eq!(gpu_limits_api_criteria_total(), 10);
        assert_eq!(GPU_LIMITS_API_SLICES.len(), 3);
        assert!(FM_BAND123_ROWS.contains(&"PH-S1869…S1878"));
    }

    #[test]
    fn gpu_limits_api_slices_met_ph_s1870() {
        let src = "GPU_LIMITS.md docs/development/GPU_LIMITS.md docs/development/gpu_limits.json";
        assert_eq!(gpu_limits_api_slices_met(src), (3, 3));
        assert_eq!(gpu_limits_api_slices_met("GPU_LIMITS.md"), (1, 3));
    }
}
