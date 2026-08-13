//! GPULimits durable store + wire (PH-S1860, band 122 — enterprise phase H).
//!
//! Durable JSON store for GPU admission + worker-limit config. Pattern mirror:
//! band 101 `ratio96_store_depth` (repo-file store + wire).

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Relative path of the durable GPU-limits store inside the repo.
pub const GPU_LIMITS_STORE_PATH: &str = "docs/development/gpu_limits.json";

/// GPU admission + worker-limit configuration (single-host).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GpuLimits {
    /// Max concurrently admitted GPU devices (0 = no GPU admission).
    pub max_gpus: u32,
    /// Per-worker GPU memory cap in MB (None = unlimited).
    pub gpu_memory_mb: Option<u64>,
    /// GPU admission enabled for new workers.
    pub admission_enabled: bool,
    /// Utilization alert threshold percent (0.0–100.0).
    pub utilization_threshold: Option<f32>,
}

/// Snapshot of the durable GPU-limits store state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GpuLimitsStoreState {
    pub max_gpus: u32,
    pub admission_enabled: bool,
    pub gpu_memory_mb_cap: Option<u64>,
}

impl GpuLimitsStoreState {
    /// GPU admission is active when admission is enabled and at least one GPU is allowed.
    pub fn admission_active(&self) -> bool {
        self.admission_enabled && self.max_gpus > 0
    }
}

/// Parse GPU-limits fields from a JSON document (PH-S1860).
pub fn gpu_limits_store_state(doc: &Value) -> Option<GpuLimitsStoreState> {
    Some(GpuLimitsStoreState {
        max_gpus: doc.get("max_gpus")?.as_u64()?.try_into().ok()?,
        admission_enabled: doc
            .get("admission_enabled")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        gpu_memory_mb_cap: doc.get("gpu_memory_mb").and_then(Value::as_u64),
    })
}

fn repo_relative_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(GPU_LIMITS_STORE_PATH)
}

/// Load the durable GPU-limits store from the repo path (PH-S1860).
pub fn gpu_limits_store_load() -> Result<GpuLimitsStoreState, String> {
    let path = repo_relative_path();
    let raw = std::fs::read_to_string(&path).map_err(|e| format!("gpu_limits store read: {e}"))?;
    let doc: Value =
        serde_json::from_str(&raw).map_err(|e| format!("gpu_limits store parse: {e}"))?;
    gpu_limits_store_state(&doc).ok_or_else(|| "gpu_limits store missing fields".to_string())
}

/// Save the durable GPU-limits store to the repo path (PH-S1860).
pub fn gpu_limits_store_save(limits: &GpuLimits) -> Result<(), String> {
    let path = repo_relative_path();
    let raw = serde_json::to_string_pretty(limits)
        .map_err(|e| format!("gpu_limits store serialize: {e}"))?;
    std::fs::write(&path, raw).map_err(|e| format!("gpu_limits store write: {e}"))
}

/// Admin/ops store wire: a lossless JSON snapshot of the durable store.
/// `available: false` when the store file is missing or unparseable.
pub fn gpu_limits_store_wire_json() -> Value {
    match gpu_limits_store_load() {
        Ok(s) => serde_json::json!({
            "mode": "repo_file",
            "available": true,
            "max_gpus": s.max_gpus,
            "admission_enabled": s.admission_enabled,
            "gpu_memory_mb_cap": s.gpu_memory_mb_cap,
            "admission_active": s.admission_active(),
        }),
        Err(_) => serde_json::json!({
            "mode": "missing",
            "available": false,
            "max_gpus": 0,
            "admission_enabled": false,
            "gpu_memory_mb_cap": null,
            "admission_active": false,
        }),
    }
}

/// Debug UI wire: same store surface served under `/api/v1/debug/ui`
/// (band 136 UI debugging mirror of the GPU-limits store).
pub fn gpu_debug_store_wire_json() -> Value {
    gpu_limits_store_wire_json()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn gpu_limits_store_state_ph_s1860() {
        let doc = json!({
            "max_gpus": 2,
            "gpu_memory_mb": 8192,
            "admission_enabled": true,
            "utilization_threshold": 95.0,
        });
        let state = gpu_limits_store_state(&doc).expect("store state");
        assert_eq!(state.max_gpus, 2);
        assert!(state.admission_enabled);
        assert_eq!(state.gpu_memory_mb_cap, Some(8192));
        assert!(state.admission_active());
    }

    #[test]
    fn gpu_limits_store_state_admission_inactive_ph_s1860() {
        let doc = json!({
            "max_gpus": 0,
            "gpu_memory_mb": null,
            "admission_enabled": true,
            "utilization_threshold": null,
        });
        let state = gpu_limits_store_state(&doc).expect("store state");
        assert_eq!(state.max_gpus, 0);
        assert!(!state.admission_active(), "zero GPUs disables admission");
    }

    #[test]
    fn gpu_limits_store_state_missing_ph_s1860() {
        assert!(
            gpu_limits_store_state(&json!({"max_gpus": "not_a_number"})).is_none(),
            "non-numeric max_gpus should return None"
        );
        assert!(
            gpu_limits_store_state(&json!({})).is_none(),
            "empty document should return None"
        );
    }

    #[test]
    fn gpu_limits_store_wire_json_shape_ph_s1860() {
        let wire = gpu_limits_store_wire_json();
        assert!(wire.get("mode").is_some(), "wire exposes mode");
        assert!(wire.get("available").is_some(), "wire exposes available");
        assert!(wire.get("max_gpus").is_some(), "wire exposes max_gpus");
        assert!(
            wire.get("admission_active").is_some(),
            "wire exposes admission_active"
        );
    }

    #[test]
    fn gpu_limits_serde_roundtrip_ph_s1860() {
        let limits = GpuLimits {
            max_gpus: 4,
            gpu_memory_mb: Some(16384),
            admission_enabled: true,
            utilization_threshold: Some(90.0),
        };
        let raw = serde_json::to_string(&limits).expect("serialize");
        let back: GpuLimits = serde_json::from_str(&raw).expect("deserialize");
        assert_eq!(limits, back);
    }
}
