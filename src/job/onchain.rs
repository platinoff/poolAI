//! On-chain submit epics: append NDJSON domain events for the Solana sidecar (PH-S38).

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use crate::job::domain_events::{
    DomainEvent, DomainEventEnvelope, JobCompletedEvent, MemoryUpdatedEvent, SeedProvidedEvent,
};
use crate::job::{JobRecord, JobStatus};

const EVENTS_FILE: &str = "events.ndjson";

static EMIT_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn emit_lock() -> std::sync::MutexGuard<'static, ()> {
    EMIT_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

/// Directory for NDJSON lines (`events.ndjson`). Unset → emit disabled (default).
pub fn events_dir_from_env() -> Option<PathBuf> {
    std::env::var("POOLAI_ONCHAIN_EVENTS_DIR")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(PathBuf::from)
}

/// Append one validated envelope line when [`events_dir_from_env`] is set.
pub fn emit_envelope(envelope: &DomainEventEnvelope) {
    let Some(dir) = events_dir_from_env() else {
        return;
    };
    let Ok(line) = envelope.to_json_line() else {
        tracing::warn!("onchain event: serialize failed");
        return;
    };
    let _guard = emit_lock();
    if let Err(e) = append_line(&dir, &line) {
        tracing::warn!("onchain event: append failed: {e}");
    }
}

fn append_line(dir: &Path, line: &str) -> Result<(), String> {
    fs::create_dir_all(dir).map_err(|e| format!("mkdir {}: {e}", dir.display()))?;
    let path = dir.join(EVENTS_FILE);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| format!("open {}: {e}", path.display()))?;
    writeln!(file, "{line}").map_err(|e| format!("write {}: {e}", path.display()))?;
    file.flush()
        .map_err(|e| format!("flush {}: {e}", path.display()))
}

/// Emit `JobCompleted` when a job reaches a terminal reward state (Rewarded / Completed).
pub fn emit_job_completed_if_anchor(record: &JobRecord) {
    if !matches!(record.status, JobStatus::Rewarded | JobStatus::Completed) {
        return;
    }
    let executor = record
        .worker_id
        .clone()
        .or_else(|| record.vm_id.clone())
        .unwrap_or_else(|| "coordinator".into());
    let event_id = format!(
        "job:{}:{}",
        record.spec.id.0,
        serde_json::to_string(&record.status)
            .unwrap_or_default()
            .trim_matches('"')
    );
    let digest = record
        .spec
        .verification_policy
        .as_ref()
        .map(|p| format!("policy:{p}"));
    emit_envelope(&DomainEventEnvelope::new(
        event_id,
        DomainEvent::JobCompleted(JobCompletedEvent {
            job_id: record.spec.id.0.clone(),
            executor_peer_id: executor,
            payout_lamports: None,
            verification_digest: digest,
        }),
    ));
}

pub fn emit_seed_provided(
    shard_id: &str,
    provider_peer_id: &str,
    artifact_id: &str,
    event_id: impl Into<String>,
) {
    emit_envelope(&DomainEventEnvelope::new(
        event_id,
        DomainEvent::SeedProvided(SeedProvidedEvent {
            shard_id: shard_id.into(),
            provider_peer_id: provider_peer_id.into(),
            artifact_id: artifact_id.into(),
        }),
    ));
}

pub fn emit_memory_updated(
    artifact_id: &str,
    version: &str,
    content_digest: &str,
    raid_logical_name: Option<&str>,
    event_id: impl Into<String>,
) {
    emit_envelope(&DomainEventEnvelope::new(
        event_id,
        DomainEvent::MemoryUpdated(MemoryUpdatedEvent {
            artifact_id: artifact_id.into(),
            version: version.into(),
            content_digest: content_digest.into(),
            raid_logical_name: raid_logical_name.map(str::to_string),
        }),
    ));
}

/// Simple digest for memory anchor metadata (no blob on-chain).
pub fn memory_content_digest(artifact_id: &str, version: &str) -> String {
    format!("poolai:v1:{artifact_id}:{version}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::job::{JobId, JobKind, JobSpec};
    use chrono::Utc;
    use std::sync::{Mutex, OnceLock};
    use tempfile::TempDir;

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    #[test]
    fn append_ndjson_when_dir_set() {
        let _guard = env_lock();
        let tmp = TempDir::new().expect("tempdir");
        let dir = tmp.path().to_path_buf();
        std::env::set_var("POOLAI_ONCHAIN_EVENTS_DIR", dir.to_string_lossy().as_ref());

        let record = JobRecord {
            spec: JobSpec {
                id: JobId::new("job-onchain-1"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: Some("strict".into()),
                deadline: None,
            },
            status: JobStatus::Rewarded,
            created_at: Utc::now(),
            worker_id: Some("worker-a".into()),
            vm_id: None,
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
            migration_count: None,
            fail_reason: None,
        };
        emit_job_completed_if_anchor(&record);

        let path = dir.join(EVENTS_FILE);
        let text = fs::read_to_string(&path).expect("read events");
        let line = text.lines().next().expect("one line");
        let v: serde_json::Value = serde_json::from_str(line).expect("json");
        assert_eq!(
            v.get("type").and_then(|t| t.as_str()),
            Some("job_completed")
        );
        assert_eq!(
            v.get("job_id").and_then(|j| j.as_str()),
            Some("job-onchain-1")
        );
        assert_eq!(
            v.get("executor_peer_id").and_then(|e| e.as_str()),
            Some("worker-a")
        );

        std::env::remove_var("POOLAI_ONCHAIN_EVENTS_DIR");
    }

    #[test]
    fn no_file_without_env_dir() {
        let _guard = env_lock();
        std::env::remove_var("POOLAI_ONCHAIN_EVENTS_DIR");
        let record = JobRecord {
            spec: JobSpec {
                id: JobId::new("job-silent"),
                kind: JobKind::System,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Completed,
            created_at: Utc::now(),
            worker_id: None,
            vm_id: None,
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
            migration_count: None,
            fail_reason: None,
        };
        emit_job_completed_if_anchor(&record);
    }
}
