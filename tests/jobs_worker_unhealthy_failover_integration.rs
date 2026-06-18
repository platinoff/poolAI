//! PH-S524: worker-unhealthy lease failover integration.

use chrono::{Duration, Utc};
use poolai::grid::galaxy_worker_health::{on_heartbeat_miss, reset_worker_health_for_test};
use poolai::job::{
    JobId, JobKind, JobRecord, JobSpec, JobStatus, JobStore, ENV_JOB_MAX_MIGRATIONS_PER_JOB,
};

fn leased_record(id: &str, owner: &str) -> JobRecord {
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
        worker_id: Some(owner.into()),
        vm_id: None,
        lease_owner: Some(owner.into()),
        lease_epoch: Some(1),
        lease_expires_at: Some(now + Duration::seconds(120)),
        migration_count: None,
        fail_reason: None,
        leased_at: Some(now),
    }
}

#[test]
fn worker_unhealthy_requeues_with_fail_reason_ph_s524() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    reset_worker_health_for_test();
    std::env::set_var(ENV_JOB_MAX_MIGRATIONS_PER_JOB, "3");
    std::env::set_var("POOLAI_GALAXY_HEARTBEAT_UNHEALTHY_THRESHOLD", "1");

    on_heartbeat_miss("w-unhealthy");
    let store = JobStore::open_for_test(None);
    store
        .push(leased_record("job-wh-1", "w-unhealthy"))
        .expect("push");
    store
        .promote_submitted_to_scheduled_with(|_| Default::default())
        .expect("promote");

    let row = store.get("job-wh-1").expect("get").expect("row");
    assert_eq!(row.fail_reason.as_deref(), Some("worker-unhealthy"));
    assert_eq!(row.migration_count, Some(1));
    assert_ne!(row.status, JobStatus::Failed);

    std::env::remove_var(ENV_JOB_MAX_MIGRATIONS_PER_JOB);
    std::env::remove_var("POOLAI_GALAXY_HEARTBEAT_UNHEALTHY_THRESHOLD");
    reset_worker_health_for_test();
}
