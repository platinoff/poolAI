//! Job lease tracing spans (PH-S126, Galaxy §4.3.1).
//!
//! Contract: [`OPENTELEMETRY_TRACING.md`](../../docs/development/OPENTELEMETRY_TRACING.md) § Job lease spans.

use chrono::{DateTime, Utc};

use crate::job::JobRecord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaseOperation {
    Acquire,
    Renew,
    PatchCas,
    GridResultCas,
}

impl LeaseOperation {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Acquire => "acquire",
            Self::Renew => "renew",
            Self::PatchCas => "patch_cas",
            Self::GridResultCas => "grid_result_cas",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaseSource {
    Api,
    Scheduler,
    GridIngest,
    WorkerClient,
}

impl LeaseSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Api => "api",
            Self::Scheduler => "scheduler",
            Self::GridIngest => "grid_ingest",
            Self::WorkerClient => "worker_client",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaseOutcome {
    Success,
    Rejected,
    Expired,
    AlreadyActive,
    NoLease,
}

impl LeaseOutcome {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Rejected => "rejected",
            Self::Expired => "expired",
            Self::AlreadyActive => "already_active",
            Self::NoLease => "no_lease",
        }
    }
}

fn expires_at_iso(expires: Option<DateTime<Utc>>) -> Option<String> {
    expires.map(|t| t.to_rfc3339())
}

/// Emit `job.lease.acquire` on successful acquire.
pub fn trace_acquire_success(record: &JobRecord, source: LeaseSource, ttl_secs: u64) {
    let span = tracing::info_span!(
        "job.lease.acquire",
        job.id = %record.spec.id.0,
        job.lease.operation = LeaseOperation::Acquire.as_str(),
        job.lease.source = source.as_str(),
        job.lease.outcome = LeaseOutcome::Success.as_str(),
        job.lease.owner = record.lease_owner.as_deref(),
        job.lease.epoch = record.lease_epoch,
        job.lease.expires_at = expires_at_iso(record.lease_expires_at).as_deref(),
        job.lease.ttl_secs = ttl_secs,
    );
    let _guard = span.enter();
}

/// Emit `job.lease.renew` on successful renew.
pub fn trace_renew_success(
    record: &JobRecord,
    source: LeaseSource,
    epoch_requested: u64,
    ttl_secs: u64,
) {
    let span = tracing::info_span!(
        "job.lease.renew",
        job.id = %record.spec.id.0,
        job.lease.operation = LeaseOperation::Renew.as_str(),
        job.lease.source = source.as_str(),
        job.lease.outcome = LeaseOutcome::Success.as_str(),
        job.lease.owner = record.lease_owner.as_deref(),
        job.lease.epoch = record.lease_epoch,
        job.lease.epoch.requested = epoch_requested,
        job.lease.expires_at = expires_at_iso(record.lease_expires_at).as_deref(),
        job.lease.ttl_secs = ttl_secs,
    );
    let _guard = span.enter();
}

/// Emit `job.lease.reject` on acquire/renew/CAS validation failure.
pub fn trace_lease_reject(
    job_id: &str,
    operation: LeaseOperation,
    source: LeaseSource,
    outcome: LeaseOutcome,
    reject_code: &str,
    epoch: Option<u64>,
    epoch_requested: Option<u64>,
    http_status_code: Option<u16>,
) {
    let span = tracing::info_span!(
        "job.lease.reject",
        job.id = %job_id,
        job.lease.operation = operation.as_str(),
        job.lease.source = source.as_str(),
        job.lease.outcome = outcome.as_str(),
        job.lease.epoch = epoch,
        job.lease.epoch.requested = epoch_requested,
        job.lease.reject.code = reject_code,
        http.status_code = http_status_code,
    );
    let _guard = span.enter();
}

/// Build acquire span for unit tests (metadata name assertion).
#[cfg(test)]
pub(crate) fn acquire_span_for_test(
    record: &JobRecord,
    source: LeaseSource,
    ttl_secs: u64,
) -> tracing::Span {
    tracing::info_span!(
        "job.lease.acquire",
        job.id = %record.spec.id.0,
        job.lease.operation = LeaseOperation::Acquire.as_str(),
        job.lease.source = source.as_str(),
        job.lease.outcome = LeaseOutcome::Success.as_str(),
        job.lease.owner = record.lease_owner.as_deref(),
        job.lease.epoch = record.lease_epoch,
        job.lease.expires_at = expires_at_iso(record.lease_expires_at).as_deref(),
        job.lease.ttl_secs = ttl_secs,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::job::{JobId, JobKind, JobRecord, JobSpec, JobStatus};

    fn sample_record() -> JobRecord {
        JobRecord {
            spec: JobSpec {
                id: JobId::new("otel-lease-test"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 0,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Leased,
            created_at: Utc::now(),
            worker_id: Some("worker-a".into()),
            vm_id: None,
            lease_owner: Some("worker-a".into()),
            lease_epoch: Some(1),
            lease_expires_at: Some(Utc::now()),
            migration_count: None,
            fail_reason: None,
        }
    }

    #[test]
    fn acquire_span_has_contract_name() {
        let record = sample_record();
        let span = acquire_span_for_test(&record, LeaseSource::Api, 90);
        assert_eq!(span.metadata().unwrap().name(), "job.lease.acquire");
    }

    #[test]
    fn reject_emits_job_lease_reject_span_name() {
        let span = tracing::info_span!(
            "job.lease.reject",
            job.id = "job-1",
            job.lease.operation = LeaseOperation::Renew.as_str(),
            job.lease.reject.code = "lease_epoch_rejected",
        );
        assert_eq!(span.metadata().unwrap().name(), "job.lease.reject");
    }
}
