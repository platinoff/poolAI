//! Cleared settlement → Solana sidecar stub (PH-S539 / PH-S568, FM-010 / Galaxy §7).
//!
//! On Cleared path, emit `JobCompleted` with `payout_lamports` via NDJSON events dir.
//! When `POOLAI_SETTLEMENT_ON_CHAIN=1`, also run sidecar-compatible mock RPC submit.

use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{LazyLock, Mutex};

use crate::grid::galaxy_settlement::PayoutBatchLedgerEntry;
use crate::grid::galaxy_settlement_mode::settlement_on_chain_enabled;
use crate::job::{
    emit_envelope, events_dir_from_env, DomainEvent, DomainEventEnvelope, JobCompletedEvent,
};

/// Prometheus gauge name for mock/devnet on-chain submit ack count (PH-S870 / PH-S873).
pub const METRIC_SETTLEMENT_ONCHAIN_SUBMIT_TOTAL: &str = "galaxy_settlement_onchain_submit_total";

static LAST_RPC_SIGNATURE_LEN: AtomicU64 = AtomicU64::new(0);
static ONCHAIN_SUBMIT_TOTAL: AtomicU64 = AtomicU64::new(0);
static SUBMITTED_EVENT_IDS: LazyLock<Mutex<HashSet<String>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

/// Whether `POOLAI_ONCHAIN_EVENTS_DIR` is configured.
pub fn onchain_events_dir_configured() -> bool {
    events_dir_from_env().is_some()
}

/// Mock/devnet on-chain submit ack count (tests / metrics).
pub fn onchain_submit_total() -> u64 {
    ONCHAIN_SUBMIT_TOTAL.load(Ordering::Relaxed)
}

/// Last mock/devnet RPC signature length from on-chain submit (tests).
pub fn last_onchain_rpc_signature_len() -> u64 {
    LAST_RPC_SIGNATURE_LEN.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_onchain_submit_metrics_for_test() {
    LAST_RPC_SIGNATURE_LEN.store(0, Ordering::Relaxed);
    ONCHAIN_SUBMIT_TOTAL.store(0, Ordering::Relaxed);
    if let Ok(mut g) = SUBMITTED_EVENT_IDS.lock() {
        g.clear();
    }
}

fn mock_rpc_submit(line: &str) -> bool {
    let Ok(env) = serde_json::from_str::<DomainEventEnvelope>(line.trim()) else {
        return false;
    };
    if env.validate().is_err() {
        return false;
    }
    let Ok(mut seen) = SUBMITTED_EVENT_IDS.lock() else {
        return false;
    };
    if !seen.insert(env.event_id.clone()) {
        return true;
    }
    let sig = format!("mocksig:{}", env.event_id);
    LAST_RPC_SIGNATURE_LEN.store(sig.len() as u64, Ordering::Relaxed);
    ONCHAIN_SUBMIT_TOTAL.fetch_add(1, Ordering::Relaxed);
    true
}

/// Emit settlement reward event for cleared grid result (PH-S539 / PH-S568).
pub fn emit_settlement_job_rewarded(entry: &PayoutBatchLedgerEntry, executor_peer_id: &str) {
    let payout_lamports = entry.gross_lamports.or_else(|| {
        entry
            .primary_dev_lamports
            .map(|p| p.saturating_add(entry.secondary_admin_lamports.unwrap_or(0)))
    });
    let event_id = format!("settlement:{}:{}", entry.job_id, entry.cleared_at);
    let envelope = DomainEventEnvelope::new(
        event_id,
        DomainEvent::JobCompleted(JobCompletedEvent {
            job_id: entry.job_id.clone(),
            executor_peer_id: executor_peer_id.to_string(),
            payout_lamports,
            verification_digest: entry.payout_pubkey.clone(),
        }),
    );
    emit_envelope(&envelope);
    if settlement_on_chain_enabled() {
        if let Ok(line) = envelope.to_json_line() {
            let _ = mock_rpc_submit(&line);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::galaxy_settlement::PayoutBatchLedgerEntry;
    use std::fs;
    use std::sync::{Mutex, OnceLock};

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    #[test]
    fn emit_settlement_job_rewarded_ph_s539() {
        let _guard = env_lock();
        let tmp = tempfile::TempDir::new().expect("tempdir");
        std::env::set_var(
            "POOLAI_ONCHAIN_EVENTS_DIR",
            tmp.path().to_string_lossy().as_ref(),
        );
        std::env::remove_var("POOLAI_SETTLEMENT_ON_CHAIN");
        let entry = PayoutBatchLedgerEntry {
            job_id: "job-reward-1".into(),
            cleared_at: "2026-06-18T12:00:00Z".into(),
            gross_lamports: Some(10_000),
            payout_pubkey: Some("pubkey123".into()),
            ..PayoutBatchLedgerEntry::minimal("", "")
        };
        emit_settlement_job_rewarded(&entry, "tg-edge");
        let path = tmp.path().join("events.ndjson");
        let text = fs::read_to_string(&path).expect("read");
        assert!(text.contains("job-reward-1"));
        assert!(text.contains("10000"));
        std::env::remove_var("POOLAI_ONCHAIN_EVENTS_DIR");
    }

    #[test]
    fn onchain_submit_mock_rpc_ph_s568() {
        let _guard = env_lock();
        reset_onchain_submit_metrics_for_test();
        let tmp = tempfile::TempDir::new().expect("tempdir");
        std::env::set_var(
            "POOLAI_ONCHAIN_EVENTS_DIR",
            tmp.path().to_string_lossy().as_ref(),
        );
        std::env::set_var("POOLAI_SETTLEMENT_ON_CHAIN", "1");
        let entry = PayoutBatchLedgerEntry {
            job_id: "job-onchain-1".into(),
            cleared_at: "2026-06-19T12:00:00Z".into(),
            gross_lamports: Some(5_000),
            payout_pubkey: None,
            ..PayoutBatchLedgerEntry::minimal("", "")
        };
        emit_settlement_job_rewarded(&entry, "peer-onchain");
        assert!(last_onchain_rpc_signature_len() > 0);
        std::env::remove_var("POOLAI_ONCHAIN_EVENTS_DIR");
        std::env::remove_var("POOLAI_SETTLEMENT_ON_CHAIN");
        reset_onchain_submit_metrics_for_test();
    }

    #[test]
    fn onchain_cleared_mock_rpc_depth_ph_s870() {
        let _guard = env_lock();
        reset_onchain_submit_metrics_for_test();
        let tmp = tempfile::TempDir::new().expect("tempdir");
        std::env::set_var(
            "POOLAI_ONCHAIN_EVENTS_DIR",
            tmp.path().to_string_lossy().as_ref(),
        );
        std::env::set_var("POOLAI_SETTLEMENT_ON_CHAIN", "1");
        let entry = PayoutBatchLedgerEntry {
            job_id: "job-depth-1".into(),
            cleared_at: "2026-06-21T12:00:00Z".into(),
            gross_lamports: Some(8_000),
            payout_pubkey: Some("pk-depth".into()),
            ..PayoutBatchLedgerEntry::minimal("", "")
        };
        emit_settlement_job_rewarded(&entry, "peer-depth");
        assert_eq!(onchain_submit_total(), 1);
        assert!(last_onchain_rpc_signature_len() > 0);
        let depth = crate::grid::galaxy_settlement_onchain_depth::settlement_onchain_depth_stub(
            true,
            true,
            onchain_submit_total(),
            last_onchain_rpc_signature_len(),
        );
        assert_eq!(
            depth,
            crate::grid::galaxy_settlement_onchain_depth::SettlementOnchainDepth::FullDepth
        );
        std::env::remove_var("POOLAI_ONCHAIN_EVENTS_DIR");
        std::env::remove_var("POOLAI_SETTLEMENT_ON_CHAIN");
        reset_onchain_submit_metrics_for_test();
    }
}
