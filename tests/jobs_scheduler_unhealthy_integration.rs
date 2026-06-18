//! PH-S525: scheduler skips unhealthy workers on bind.

use poolai::grid::galaxy_worker_health::{on_heartbeat_miss, reset_worker_health_for_test};
use poolai::job::{
    schedule_with_workers, JobId, JobKind, JobRecord, JobSpec, JobStatus, JobStore, WorkerCandidate,
};

#[test]
fn schedule_skips_unhealthy_worker_ph_s525() {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    reset_worker_health_for_test();
    std::env::set_var("POOLAI_GALAXY_HEARTBEAT_UNHEALTHY_THRESHOLD", "1");
    on_heartbeat_miss("w-sick");

    let store = JobStore::open_for_test(None);
    store
        .push(JobRecord {
            spec: JobSpec {
                id: JobId::new("job-bind-1"),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority: 1,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Submitted,
            created_at: chrono::Utc::now(),
            worker_id: None,
            vm_id: None,
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
            migration_count: None,
            fail_reason: None,
            leased_at: None,
        })
        .expect("push");

    let workers = vec![
        WorkerCandidate {
            id: "w-sick".into(),
            active_connections: 0,
            is_healthy: true,
            free_memory_mb: 16_384,
            has_gpu: false,
        },
        WorkerCandidate {
            id: "w-ok".into(),
            active_connections: 1,
            is_healthy: true,
            free_memory_mb: 16_384,
            has_gpu: false,
        },
    ];
    schedule_with_workers(&store, &workers, &[]).expect("schedule");
    let row = store.get("job-bind-1").expect("get").expect("row");
    assert_eq!(row.worker_id.as_deref(), Some("w-ok"));

    std::env::remove_var("POOLAI_GALAXY_HEARTBEAT_UNHEALTHY_THRESHOLD");
    reset_worker_health_for_test();
}
