//! Task-probe capability admission gate (PH-S540, Galaxy §6.1 / FM-016).
//!
//! Gate `telegram_edge` GPU / high-trust jobs using `raid_artifact_probe` history.
//! PH-S562: GPU passthrough capability document field.

use std::sync::{LazyLock, Mutex};

use crate::core::error::AppError;
use crate::grid::galaxy_trust_score::{infer_worker_origin, WorkerOrigin};

/// Minimum successful probe count before telegram_edge GPU jobs are admitted.
pub const TELEGRAM_EDGE_PROBE_MIN_SUCCESS: u32 = 1;

static PROBE_SUCCESS_BY_PEER: LazyLock<Mutex<std::collections::HashMap<String, u32>>> =
    LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));

static PEER_CAPABILITIES: LazyLock<Mutex<std::collections::HashMap<String, Vec<String>>>> =
    LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_probe_success_for_test() {
    if let Ok(mut g) = PROBE_SUCCESS_BY_PEER.lock() {
        g.clear();
    }
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_peer_capabilities_for_test() {
    if let Ok(mut g) = PEER_CAPABILITIES.lock() {
        g.clear();
    }
}

/// Record capabilities from register-remote capability_document (PH-S562).
pub fn record_peer_capabilities(peer_id: &str, capabilities: &[String]) {
    if let Ok(mut g) = PEER_CAPABILITIES.lock() {
        g.insert(peer_id.to_string(), capabilities.to_vec());
    }
}

fn peer_capabilities(peer_id: &str) -> Vec<String> {
    PEER_CAPABILITIES
        .lock()
        .ok()
        .and_then(|g| g.get(peer_id).cloned())
        .unwrap_or_default()
}

/// Record a successful `raid_artifact_probe` for a peer (worker callback stub).
pub fn record_raid_artifact_probe_success(peer_id: &str) {
    if let Ok(mut g) = PROBE_SUCCESS_BY_PEER.lock() {
        *g.entry(peer_id.to_string()).or_insert(0) += 1;
    }
}

fn probe_success_count(peer_id: &str) -> u32 {
    PROBE_SUCCESS_BY_PEER
        .lock()
        .ok()
        .and_then(|g| g.get(peer_id).copied())
        .unwrap_or(0)
}

fn task_requires_probe_gate(task_kind: &str) -> bool {
    let lower = task_kind.trim().to_ascii_lowercase();
    lower.contains("gpu")
        || lower.contains("high_trust")
        || lower == "inference:gpu"
        || lower == "training"
}

fn has_gpu_passthrough_capability(caps: &[String]) -> bool {
    caps.iter()
        .any(|c| c.eq_ignore_ascii_case("gpu_passthrough"))
}

/// Gate telegram_edge GPU/high-trust grid jobs; reject with structured error when probe history insufficient.
pub fn check_telegram_edge_capability_admission(
    source_peer_id: Option<&str>,
    task_kind: &str,
) -> Result<(), AppError> {
    let Some(peer) = source_peer_id else {
        return Ok(());
    };
    if infer_worker_origin(Some(peer)) != WorkerOrigin::TelegramEdge {
        return Ok(());
    }
    if !task_requires_probe_gate(task_kind) {
        return Ok(());
    }
    if probe_success_count(peer) < TELEGRAM_EDGE_PROBE_MIN_SUCCESS {
        return Err(AppError::RestError {
            code: "capability_probe_required",
            message: format!(
                "telegram_edge peer '{peer}' requires raid_artifact_probe history before GPU/high-trust jobs (Galaxy §6.1)"
            ),
        });
    }
    let caps = peer_capabilities(peer);
    if !has_gpu_passthrough_capability(&caps) {
        return Err(AppError::RestError {
            code: "gpu_passthrough_required",
            message: format!(
                "telegram_edge peer '{peer}' requires gpu_passthrough in capability_document for GPU jobs (Galaxy §6.6 PH-S562)"
            ),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_telegram_edge_gpu_without_probe_ph_s540() {
        reset_probe_success_for_test();
        let err = check_telegram_edge_capability_admission(Some("tg-edge"), "inference:gpu")
            .expect_err("reject");
        assert!(
            matches!(err, AppError::RestError { code, .. } if code == "capability_probe_required")
        );
    }

    #[test]
    fn admits_after_probe_success_ph_s540() {
        reset_probe_success_for_test();
        reset_peer_capabilities_for_test();
        record_raid_artifact_probe_success("tg-edge");
        record_peer_capabilities("tg-edge", &["gpu_passthrough".into()]);
        check_telegram_edge_capability_admission(Some("tg-edge"), "inference:gpu").expect("admit");
        reset_probe_success_for_test();
        reset_peer_capabilities_for_test();
    }

    #[test]
    fn rejects_gpu_without_passthrough_capability_ph_s562() {
        reset_probe_success_for_test();
        reset_peer_capabilities_for_test();
        record_raid_artifact_probe_success("tg-edge");
        let err = check_telegram_edge_capability_admission(Some("tg-edge"), "inference:gpu")
            .expect_err("reject");
        assert!(
            matches!(err, AppError::RestError { code, .. } if code == "gpu_passthrough_required")
        );
        reset_probe_success_for_test();
        reset_peer_capabilities_for_test();
    }
}
