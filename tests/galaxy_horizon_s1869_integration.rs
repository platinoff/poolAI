//! PH-S1878: Galaxy horizon close band 123 — GPULimits API contracts.
//! Suite: `galaxy_horizon_s1869_integration`.

use poolai_ui_core::gpu_limits_api_depth::{
    gpu_limits_api_criteria_total, gpu_limits_api_depth_stub, GpuLimitsApiDepth, FM_BAND123_ROWS,
    GPU_LIMITS_API_BAND123_ROWS, GPU_LIMITS_API_CASES, GPU_LIMITS_API_CRITERIA,
};
use poolai_ui_core::gpu_limits_store::{gpu_limits_store_wire_json, GPU_LIMITS_STORE_PATH};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1869_band_gpu_limits_api_close_ph_s1878() {
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
    assert!(GPU_LIMITS_API_CASES.contains(&"http_route"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND123_ROWS {
        assert!(fm.contains(row), "FM missing band-123 row {row}");
    }
    assert!(fm.contains("PH-S1878"));
    assert!(fm.contains("5.104"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1869") || handoff.contains("band 123"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 123"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--gpu-limits-api"));
    assert!(run_local.contains("VERIFY_GPU_LIMITS_API"));

    let canon_doc = include_str!("../docs/development/GPU_LIMITS.md");
    assert!(canon_doc.contains("GPU_LIMITS_API_SLICES"));
    assert!(canon_doc.contains("gpu_limits_api_band123_export_shape"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_GPU_LIMITS_API"));
    assert!(verify.contains("--gpu-limits-api"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--gpu-limits-api"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("gpu_limits_api_mode"));
    assert!(loc_audit.contains("gpu_limits_api_criteria_met_count"));

    let system_mod = include_str!("../src/network/api/system.rs");
    assert!(system_mod.contains("gpu-limits"));
    assert!(system_mod.contains("gpu_limits_store_wire_json"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("gpu_limits_api_band123_export_shape"));

    for marker in GPU_LIMITS_API_BAND123_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || canon_doc.contains(marker)
                || system_mod.contains(marker),
            "band-123 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/gpu_limits_api_depth.rs").exists());
    assert!(Path::new("docs/development/GPU_LIMITS.md").exists());
    assert!(Path::new(GPU_LIMITS_STORE_PATH).exists());

    let wire = gpu_limits_store_wire_json();
    assert!(wire.get("mode").is_some());
    assert!(wire.get("admission_active").is_some());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("gpu_limits_api_mode").is_some());
    assert!(ratio.get("gpu_limits_api_criteria_total").is_some());
}
