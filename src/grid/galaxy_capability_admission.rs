//! Task-probe capability admission gate (PH-S540, Galaxy §6.1 / FM-016).
//!
//! Gate `telegram_edge` GPU / high-trust jobs using `raid_artifact_probe` history.

use std::sync::{LazyLock, Mutex};

use crate::core::error::AppError;
use crate::grid::galaxy_trust_score::{infer_worker_origin, WorkerOrigin};

/// Minimum successful probe count before telegram_edge GPU jobs are admitted.
pub const TELEGRAM_EDGE_PROBE_MIN_SUCCESS: u32 = 1;

static PROBE_SUCCESS_BY_PEER: LazyLock<Mutex<std::collections::HashMap<String, u32>>> =
    LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_probe_success_for_test() {
    if let Ok(mut g) = PROBE_SUCCESS_BY_PEER.lock() {
        g.clear();
    }
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
    if probe_success_count(peer) >= TELEGRAM_EDGE_PROBE_MIN_SUCCESS {
        return Ok(());
    }
    Err(AppError::RestError {
        code: "capability_probe_required",
        message: format!(
            "telegram_edge peer '{peer}' requires raid_artifact_probe history before GPU/high-trust jobs (Galaxy §6.1)"
        ),
    })
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
        record_raid_artifact_probe_success("tg-edge");
        check_telegram_edge_capability_admission(Some("tg-edge"), "inference:gpu").expect("admit");
        reset_probe_success_for_test();
    }
}
