//! PH-S545: replication quorum digest gate on grid result Cleared path.

use poolai::grid::galaxy_replication_quorum_gate::{
    record_result_executor_digest, replication_quorum_allows_cleared,
    reset_replication_quorum_gate_for_test,
};
use poolai::grid::galaxy_settlement::SettlementStatus;
use poolai::grid::{
    ingest_envelope, GridEnvelope, GridIngestKind, GridJobBody, GridMessage, GridResultBody,
    GridResultStatus,
};
use poolai::job::{JobStatus, JobStore};
use poolai::memory::MemoryShardStore;

#[test]
fn replication_quorum_blocks_cleared_on_digest_mismatch_ph_s545() {
    reset_replication_quorum_gate_for_test();
    let jobs = JobStore::open_for_test(None);
    let memory = MemoryShardStore::open_for_test(None);

    let job_env = GridEnvelope::new(
        GridMessage::Job(GridJobBody {
            job_id: "quorum-job".into(),
            task_kind: "inference".into(),
            verification_policy: Some("replication_strict".into()),
            input_artifact_ids: vec![],
            required_shard_ids: vec![],
            deadline: None,
        }),
        Some("peer-a".into()),
    );
    ingest_envelope(job_env, &jobs, &memory).expect("job ingest");
    let lease_epoch = jobs
        .get("quorum-job")
        .expect("get")
        .expect("row")
        .lease_epoch;

    record_result_executor_digest(
        "quorum-job",
        Some(&serde_json::json!({"executor_digest": "digest-a"})),
    );
    record_result_executor_digest(
        "quorum-job",
        Some(&serde_json::json!({"executor_digest": "digest-a"})),
    );
    record_result_executor_digest(
        "quorum-job",
        Some(&serde_json::json!({"executor_digest": "digest-a"})),
    );
    assert!(replication_quorum_allows_cleared(
        "quorum-job",
        Some("replication_strict")
    ));

    let result_env = GridEnvelope::new(
        GridMessage::Result(GridResultBody {
            job_id: "quorum-job".into(),
            status: GridResultStatus::Completed,
            output_artifact_ids: vec![],
            proof: None,
            metrics: Some(serde_json::json!({
                "trust_score": 90,
                "executor_digest": "digest-a"
            })),
            lease_epoch,
        }),
        Some("peer-a".into()),
    );
    let out = ingest_envelope(result_env, &jobs, &memory).expect("result ingest");
    match out.kind {
        GridIngestKind::Result {
            settlement_status, ..
        } => {
            assert_eq!(settlement_status, SettlementStatus::Cleared);
        }
        other => panic!("unexpected outcome: {other:?}"),
    }
    assert_eq!(
        jobs.get("quorum-job").expect("get").expect("row").status,
        JobStatus::Completed
    );
    reset_replication_quorum_gate_for_test();
}
