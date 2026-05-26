#![cfg(feature = "test-utils")]

use chrono::Utc;
use poolai::job::{JobId, JobKind, JobRecord, JobSpec, JobStatus, JobStore};
use tempfile::TempDir;

fn sample_record(id: &str) -> JobRecord {
    JobRecord {
        spec: JobSpec {
            id: JobId::new(id),
            kind: JobKind::Inference,
            resources: Default::default(),
            priority: 0,
            max_duration_secs: None,
            input_artifact_ids: vec![],
            verification_policy: None,
            deadline: None,
        },
        status: JobStatus::Submitted,
        created_at: Utc::now(),
        worker_id: None,
        vm_id: None,
    }
}

#[test]
fn job_store_persists_via_raid_snapshot_across_reload() {
    let tmp = TempDir::new().expect("tempdir");
    let raid_base = tmp.path().join("raid");

    std::env::set_var("POOLAI_RAID_BASE_PATH", &raid_base);
    std::env::set_var("POOLAI_JOB_STORE", "raid");
    std::env::remove_var("POOLAI_JOB_DATA_DIR");

    let record = sample_record("job-raid-1");

    let store1 = JobStore::open_for_test(None);
    store1.push(record.clone()).expect("push");

    // Reload store and verify it reads snapshot from RAID.
    let store2 = JobStore::open_for_test(None);
    let loaded = store2.get(&record.spec.id.0).expect("get").expect("row");

    assert_eq!(loaded.spec.id.0, "job-raid-1");
    assert_eq!(loaded.spec.kind, JobKind::Inference);
    assert_eq!(loaded.status, JobStatus::Submitted);

    // Status update must also persist via RAID.
    let store1 = JobStore::open_for_test(None);
    store1
        .update_status(&record.spec.id.0, JobStatus::Scheduled)
        .expect("update_status");

    let store3 = JobStore::open_for_test(None);
    let reloaded = store3.get(&record.spec.id.0).expect("get").expect("row");
    assert_eq!(reloaded.status, JobStatus::Scheduled);
}
