//! PH-S1868: Galaxy horizon close band 122 — GPULimits store/wire.
//! Suite: `galaxy_horizon_s1859_integration`.

use poolai_ui_core::gpu_limits_depth::{
    gpu_limits_criteria_total, gpu_limits_depth_stub, GpuLimitsDepth, FM_BAND122_ROWS,
    GPU_LIMITS_BAND122_ROWS, GPU_LIMITS_CASES, GPU_LIMITS_CRITERIA,
};
use poolai_ui_core::gpu_limits_store::{gpu_limits_store_wire_json, GPU_LIMITS_STORE_PATH};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1859_band_gpu_limits_close_ph_s1868() {
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
    assert!(GPU_LIMITS_CASES.contains(&"store_wire"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND122_ROWS {
        assert!(fm.contains(row), "FM missing band-122 row {row}");
    }
    assert!(fm.contains("PH-S1868"));
    assert!(fm.contains("5.103"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1859") || handoff.contains("band 122"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 122"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--gpu-limits"));
    assert!(run_local.contains("VERIFY_GPU_LIMITS"));

    let canon_doc = include_str!("../docs/development/GPU_LIMITS.md");
    assert!(canon_doc.contains("GPU_LIMITS_SLICES"));
    assert!(canon_doc.contains("gpu_limits_band122_export_shape"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_GPU_LIMITS"));
    assert!(verify.contains("--gpu-limits"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--gpu-limits"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("gpu_limits_mode"));
    assert!(loc_audit.contains("gpu_limits_criteria_met_count"));

    let smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(smoke.contains("gpu_limits_band122_export_shape"));

    for marker in GPU_LIMITS_BAND122_ROWS {
        assert!(
            fm.contains(marker)
                || run_local.contains(marker)
                || loc_audit.contains(marker)
                || smoke.contains(marker)
                || verify.contains(marker)
                || canon_doc.contains(marker),
            "band-122 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/gpu_limits_depth.rs").exists());
    assert!(Path::new("docs/development/GPU_LIMITS.md").exists());
    assert!(Path::new(GPU_LIMITS_STORE_PATH).exists());

    let wire = gpu_limits_store_wire_json();
    assert!(wire.get("mode").is_some());
    assert!(wire.get("admission_active").is_some());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("gpu_limits_mode").is_some());
    assert!(ratio.get("gpu_limits_criteria_total").is_some());
}
