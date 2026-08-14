//! PH-S1861: GPULimits API contracts (band 122).
//! Marker: gpu_limits_integration
//!
//! Verifies the durable GPU-limits store round-trips and the wire shape exposes
//! the admission fields.

use poolai_ui_core::gpu_limits_depth::{
    gpu_limits_criteria_total, gpu_limits_depth_stub, gpu_limits_slices_met, GpuLimitsDepth,
    GPU_LIMITS_CASES, GPU_LIMITS_CRITERIA, GPU_LIMITS_SLICES,
};
use poolai_ui_core::gpu_limits_store::{
    gpu_limits_store_state, gpu_limits_store_wire_json, GpuLimits, GpuLimitsStoreState,
    GPU_LIMITS_STORE_PATH,
};
use serde_json::json;
use std::path::Path;

#[test]
fn gpu_limits_depth_registry_ph_s1859() {
    assert_eq!(GPU_LIMITS_CRITERIA.len(), 10);
    assert_eq!(gpu_limits_criteria_total(), 10);
    assert!(GPU_LIMITS_CASES.contains(&"gpu_limits_depth"));
    assert!(GPU_LIMITS_CASES.contains(&"store_wire"));
    assert_eq!(
        gpu_limits_depth_stub(Some(&json!({"store_wire": true}))),
        GpuLimitsDepth::StoreWireSlice
    );
}

#[test]
fn gpu_limits_store_slice_docs_present_ph_s1860() {
    let canon = include_str!("../docs/development/GPU_LIMITS.md");
    let (met, total) = gpu_limits_slices_met(canon);
    assert_eq!(total, 3);
    assert_eq!(
        met, 3,
        "all band 122 GPU-limits slices must be listed in the canon doc"
    );
    for name in GPU_LIMITS_SLICES {
        assert!(
            canon.contains(name),
            "missing GPU limits canon marker {name}"
        );
    }
    let store_path = "docs/development/gpu_limits.json".to_string();
    assert!(
        Path::new(&store_path).exists(),
        "missing durable store docs/development/gpu_limits.json"
    );
    let store = include_str!("../docs/development/gpu_limits.json");
    let state = gpu_limits_store_state(&serde_json::from_str(store).expect("gpu_limits.json"))
        .expect("store state parses");
    assert_eq!(state.max_gpus, 0);
    assert!(
        !state.admission_active(),
        "default store has no GPU admission"
    );
}

#[test]
fn gpu_limits_store_roundtrip_ph_s1861() {
    let limits = GpuLimits {
        max_gpus: 2,
        gpu_memory_mb: Some(8192),
        admission_enabled: true,
        utilization_threshold: Some(95.0),
    };
    let raw = serde_json::to_string(&limits).expect("serialize");
    let back: GpuLimits = serde_json::from_str(&raw).expect("deserialize");
    assert_eq!(limits, back);

    let state = gpu_limits_store_state(&serde_json::from_str(&raw).expect("parse")).expect("state");
    assert_eq!(state.max_gpus, 2);
    assert!(state.admission_active());
}

#[test]
fn gpu_limits_wire_shape_ph_s1861() {
    let wire = gpu_limits_store_wire_json();
    assert!(wire.get("mode").is_some(), "wire exposes mode");
    assert!(wire.get("available").is_some(), "wire exposes available");
    assert!(wire.get("max_gpus").is_some(), "wire exposes max_gpus");
    assert!(
        wire.get("admission_enabled").is_some(),
        "wire exposes admission_enabled"
    );
    assert!(
        wire.get("admission_active").is_some(),
        "wire exposes admission_active"
    );
    assert!(GpuLimitsStoreState {
        max_gpus: 4,
        admission_enabled: true,
        gpu_memory_mb_cap: Some(16384),
    }
    .admission_active());
    assert_eq!(GPU_LIMITS_STORE_PATH, "docs/development/gpu_limits.json");
}
