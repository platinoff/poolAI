//! Cleared settlement → Solana sidecar stub (PH-S539, FM-010 / Galaxy §7).
//!
//! On Cleared path, emit `JobCompleted` with `payout_lamports` via NDJSON events dir.

use crate::grid::galaxy_settlement::PayoutBatchLedgerEntry;
use crate::job::{emit_envelope, DomainEvent, DomainEventEnvelope, JobCompletedEvent};

/// Emit settlement reward event for cleared grid result (PH-S539).
pub fn emit_settlement_job_rewarded(entry: &PayoutBatchLedgerEntry, executor_peer_id: &str) {
    let payout_lamports = entry.gross_lamports.or_else(|| {
        entry
            .primary_dev_lamports
            .map(|p| p.saturating_add(entry.secondary_admin_lamports.unwrap_or(0)))
    });
    let event_id = format!("settlement:{}:{}", entry.job_id, entry.cleared_at);
    emit_envelope(&DomainEventEnvelope::new(
        event_id,
        DomainEvent::JobCompleted(JobCompletedEvent {
            job_id: entry.job_id.clone(),
            executor_peer_id: executor_peer_id.to_string(),
            payout_lamports,
            verification_digest: entry.payout_pubkey.clone(),
        }),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::galaxy_settlement::PayoutBatchLedgerEntry;
    use std::fs;
    use std::sync::{Mutex, OnceLock};
    use tempfile::TempDir;

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    #[test]
    fn emit_settlement_job_rewarded_ph_s539() {
        let _guard = env_lock();
        let tmp = TempDir::new().expect("tempdir");
        std::env::set_var(
            "POOLAI_ONCHAIN_EVENTS_DIR",
            tmp.path().to_string_lossy().as_ref(),
        );
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
}
