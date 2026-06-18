//! PH-S518: lease failover retry budget integration.

use chrono::{Duration, Utc};
use poolai::job::{
    acquire_lease_on_record, JobId, JobKind, JobRecord, JobSpec, JobStatus, JobStore,
    LeaseFailReason, ENV_JOB_MAX_MIGRATIONS_PER_JOB,
};

fn leased_record(id: &str) -> JobRecord {
    let now = Utc::now();
    JobRecord {
        spec: JobSpec {
            id: JobId::new(id),
            kind: JobKind::Inference,
            resources: Default::default(),
            priority: 1,
            max_duration_secs: None,
            input_artifact_ids: vec![],
            verification_policy: None,
            deadline: None,
        },
        status: JobStatus::Leased,
        created_at: now,
        worker_id: Some("w1".into()),
        vm_id: None,
        lease_owner: Some("w1".into()),
        lease_epoch: Some(1),
        lease_expires_at: Some(now - Duration::seconds(1)),
        migration_count: None,
        fail_reason: None,
    }
}

#[test]
fn promote_requeue_sets_fail_reason_ph_s518() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    std::env::set_var(ENV_JOB_MAX_MIGRATIONS_PER_JOB, "3");

    let store = JobStore::open_for_test(None);
    store.push(leased_record("job-failover-1")).expect("push");
    store
        .promote_submitted_to_scheduled_with(|_| Default::default())
        .expect("promote");

    let row = store.get("job-failover-1").expect("get").expect("row");
    assert_eq!(row.fail_reason.as_deref(), Some("lease-timeout"));
    assert_eq!(row.migration_count, Some(1));

    std::env::remove_var(ENV_JOB_MAX_MIGRATIONS_PER_JOB);
}

#[test]
fn acquire_lease_increments_epoch_after_failover_requeue_ph_s518() {
    let mut record = leased_record("job-failover-2");
    record.status = JobStatus::Submitted;
    record.worker_id = None;
    record.lease_owner = None;
    record.lease_expires_at = None;
    record.migration_count = Some(1);
    record.fail_reason = Some(LeaseFailReason::LeaseTimeout.as_str().into());
    acquire_lease_on_record(
        &mut record,
        "w2",
        &poolai::job::JobLeaseConfig::from_env(),
        Utc::now(),
        true,
    )
    .expect("acquire");
    assert_eq!(record.status, JobStatus::Leased);
    assert_eq!(record.lease_epoch, Some(2));
}
