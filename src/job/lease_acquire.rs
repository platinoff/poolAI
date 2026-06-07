//! Job lease acquire (PH-S98, Galaxy §4.3.1).
//!
//! Populates `lease_owner` / `lease_epoch` / `lease_expires_at` from `JobLeaseConfig`.
//! Renew (PH-S99) extends `lease_expires_at` with epoch CAS; failover — PH-S101+.

use chrono::{DateTime, Duration, Utc};

use crate::job::{allows_transition, JobLeaseConfig, JobRecord, JobStatus};
use crate::observability::lease_trace::{trace_acquire_success, LeaseSource};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcquireLeaseError {
    NoLeaseOwner,
    LeaseAlreadyActive,
}

/// Renew/heartbeat validation errors (PH-S99).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenewLeaseError {
    NoLeaseOnJob,
    EpochRejected,
    LeaseExpired,
}

/// Set `Leased` when lifecycle allows (Galaxy §4.3.2; PH-S100).
pub fn maybe_transition_to_leased(record: &mut JobRecord) {
    if allows_transition(record.status, JobStatus::Leased) {
        record.status = JobStatus::Leased;
    }
}

/// Resolve lease holder: explicit body → bound `worker_id` → bound `vm_id`.
pub fn resolve_lease_owner(
    explicit: Option<&str>,
    worker_id: Option<&str>,
    vm_id: Option<&str>,
) -> Option<String> {
    explicit
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .or_else(|| worker_id.filter(|s| !s.is_empty()).map(str::to_string))
        .or_else(|| vm_id.filter(|s| !s.is_empty()).map(str::to_string))
}

/// Acquire or bump lease on `record` (mutates in place).
pub fn acquire_lease_on_record(
    record: &mut JobRecord,
    owner: &str,
    cfg: &JobLeaseConfig,
    now: DateTime<Utc>,
    reject_if_active: bool,
) -> Result<(), AcquireLeaseError> {
    if owner.is_empty() {
        return Err(AcquireLeaseError::NoLeaseOwner);
    }
    if record.has_lease_fields() && record.lease_active_at(now) {
        if reject_if_active {
            return Err(AcquireLeaseError::LeaseAlreadyActive);
        }
        return Ok(());
    }
    let next_epoch = record.lease_epoch.unwrap_or(0).saturating_add(1);
    record.lease_owner = Some(owner.to_string());
    record.lease_epoch = Some(next_epoch);
    record.lease_expires_at = Some(now + Duration::seconds(cfg.lease_ttl_secs as i64));
    maybe_transition_to_leased(record);
    Ok(())
}

/// Extend active lease TTL; `lease_epoch` must match (Galaxy §4.3.1 heartbeat).
pub fn renew_lease_on_record(
    record: &mut JobRecord,
    epoch: u64,
    cfg: &JobLeaseConfig,
    now: DateTime<Utc>,
) -> Result<(), RenewLeaseError> {
    if !record.has_lease_fields() {
        return Err(RenewLeaseError::NoLeaseOnJob);
    }
    if record.lease_epoch != Some(epoch) {
        return Err(RenewLeaseError::EpochRejected);
    }
    if !record.lease_active_at(now) {
        return Err(RenewLeaseError::LeaseExpired);
    }
    record.lease_expires_at = Some(now + Duration::seconds(cfg.lease_ttl_secs as i64));
    Ok(())
}

/// After scheduler binding: acquire when no active lease and an owner id is known.
pub fn maybe_acquire_lease_on_schedule(record: &mut JobRecord, now: DateTime<Utc>) {
    let Some(owner) =
        resolve_lease_owner(None, record.worker_id.as_deref(), record.vm_id.as_deref())
    else {
        return;
    };
    let cfg = JobLeaseConfig::from_env();
    let had_active = record.has_lease_fields() && record.lease_active_at(now);
    if acquire_lease_on_record(record, &owner, &cfg, now, false).is_ok() && !had_active {
        trace_acquire_success(record, LeaseSource::Scheduler, cfg.lease_ttl_secs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::job::{JobId, JobKind, JobRecord, JobSpec, JobStatus};
    use chrono::TimeZone;

    fn sample_record(worker_id: Option<&str>) -> JobRecord {
        JobRecord {
            spec: JobSpec {
                id: JobId::new("lease-acquire-test"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Scheduled,
            created_at: Utc::now(),
            worker_id: worker_id.map(str::to_string),
            vm_id: None,
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
        }
    }

    #[test]
    fn acquire_sets_epoch_and_expiry() {
        let now = Utc.with_ymd_and_hms(2026, 5, 27, 12, 0, 0).unwrap();
        let cfg = JobLeaseConfig {
            lease_ttl_secs: 90,
            lease_renew_interval_secs: 30,
        };
        let mut record = sample_record(Some("worker-a"));
        acquire_lease_on_record(&mut record, "worker-a", &cfg, now, true).expect("acquire");
        assert_eq!(record.status, JobStatus::Leased);
        assert_eq!(record.lease_owner.as_deref(), Some("worker-a"));
        assert_eq!(record.lease_epoch, Some(1));
        assert_eq!(record.lease_expires_at, Some(now + Duration::seconds(90)));
        assert!(record.lease_active_at(now));
    }

    #[test]
    fn acquire_rejects_active_lease_when_explicit() {
        let now = Utc.with_ymd_and_hms(2026, 5, 27, 12, 0, 0).unwrap();
        let cfg = JobLeaseConfig::default();
        let mut record = sample_record(Some("worker-a"));
        acquire_lease_on_record(&mut record, "worker-a", &cfg, now, true).expect("first");
        assert_eq!(
            acquire_lease_on_record(&mut record, "worker-a", &cfg, now, true),
            Err(AcquireLeaseError::LeaseAlreadyActive)
        );
    }

    #[test]
    fn schedule_acquire_skips_when_active() {
        let now = Utc.with_ymd_and_hms(2026, 5, 27, 12, 0, 0).unwrap();
        let cfg = JobLeaseConfig::default();
        let mut record = sample_record(Some("worker-a"));
        acquire_lease_on_record(&mut record, "worker-a", &cfg, now, true).expect("acquire");
        let epoch = record.lease_epoch;
        maybe_acquire_lease_on_schedule(&mut record, now);
        assert_eq!(record.lease_epoch, epoch);
    }

    #[test]
    fn schedule_acquire_after_binding() {
        let now = Utc.with_ymd_and_hms(2026, 5, 27, 12, 0, 0).unwrap();
        let mut record = sample_record(Some("worker-b"));
        maybe_acquire_lease_on_schedule(&mut record, now);
        assert_eq!(record.status, JobStatus::Leased);
        assert!(record.has_lease_fields());
        assert_eq!(record.lease_owner.as_deref(), Some("worker-b"));
        assert_eq!(record.lease_epoch, Some(1));
    }

    #[test]
    fn acquire_on_submitted_transitions_to_leased() {
        let now = Utc.with_ymd_and_hms(2026, 5, 27, 12, 0, 0).unwrap();
        let cfg = JobLeaseConfig::default();
        let mut record = sample_record(None);
        record.status = JobStatus::Submitted;
        acquire_lease_on_record(&mut record, "worker-x", &cfg, now, true).expect("acquire");
        assert_eq!(record.status, JobStatus::Leased);
    }

    #[test]
    fn renew_extends_expiry_when_epoch_matches() {
        let now = Utc.with_ymd_and_hms(2026, 5, 27, 12, 0, 0).unwrap();
        let cfg = JobLeaseConfig {
            lease_ttl_secs: 60,
            lease_renew_interval_secs: 20,
        };
        let mut record = sample_record(Some("worker-a"));
        acquire_lease_on_record(&mut record, "worker-a", &cfg, now, true).expect("acquire");
        let before_expires = record.lease_expires_at;
        let renew_at = now + Duration::seconds(30);
        renew_lease_on_record(&mut record, 1, &cfg, renew_at).expect("renew");
        assert_eq!(
            record.lease_expires_at,
            Some(renew_at + Duration::seconds(60))
        );
        assert_ne!(record.lease_expires_at, before_expires);
        assert!(record.lease_active_at(renew_at));
    }

    #[test]
    fn renew_rejects_epoch_mismatch_or_expired() {
        let now = Utc.with_ymd_and_hms(2026, 5, 27, 12, 0, 0).unwrap();
        let cfg = JobLeaseConfig::default();
        let mut record = sample_record(Some("worker-a"));
        acquire_lease_on_record(&mut record, "worker-a", &cfg, now, true).expect("acquire");
        assert_eq!(
            renew_lease_on_record(&mut record, 0, &cfg, now),
            Err(RenewLeaseError::EpochRejected)
        );
        let expired_at = now + Duration::seconds(120);
        assert_eq!(
            renew_lease_on_record(&mut record, 1, &cfg, expired_at),
            Err(RenewLeaseError::LeaseExpired)
        );
    }
}
