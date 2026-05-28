//! Map Job types ↔ Grid envelope.

use crate::grid::{GridEnvelope, GridJobBody, GridMessage, GridResultBody, GridResultStatus};
use crate::job::types::{JobId, JobKind, JobSpec, JobStatus};

pub fn job_spec_to_grid_job(spec: &JobSpec) -> GridJobBody {
    GridJobBody {
        job_id: spec.id.0.clone(),
        task_kind: spec.kind.as_str().to_string(),
        verification_policy: spec.verification_policy.clone(),
        input_artifact_ids: spec.input_artifact_ids.clone(),
        deadline: spec.deadline,
    }
}

pub fn envelope_from_job_spec(spec: &JobSpec) -> GridEnvelope {
    GridEnvelope::new(GridMessage::Job(job_spec_to_grid_job(spec)), None)
}

pub fn job_spec_from_grid_job(body: &GridJobBody) -> JobSpec {
    JobSpec {
        id: JobId::new(body.job_id.clone()),
        kind: parse_job_kind(&body.task_kind),
        resources: Default::default(),
        priority: 0,
        max_duration_secs: None,
        input_artifact_ids: body.input_artifact_ids.clone(),
        verification_policy: body.verification_policy.clone(),
        deadline: body.deadline,
    }
}

pub fn job_spec_from_envelope(env: &GridEnvelope) -> Option<JobSpec> {
    match &env.msg {
        GridMessage::Job(body) => Some(job_spec_from_grid_job(body)),
        _ => None,
    }
}

pub fn job_status_from_grid_result(status: GridResultStatus) -> JobStatus {
    match status {
        GridResultStatus::Completed => JobStatus::Completed,
        GridResultStatus::Failed => JobStatus::Failed,
        GridResultStatus::Verified => JobStatus::Verifying,
    }
}

pub fn grid_result_from_status(
    job_id: &JobId,
    status: JobStatus,
    output_artifact_ids: Vec<String>,
    lease_epoch: Option<u64>,
) -> GridResultBody {
    let grid_status = match status {
        JobStatus::Completed | JobStatus::Rewarded => GridResultStatus::Completed,
        JobStatus::Verifying => GridResultStatus::Verified,
        JobStatus::Failed => GridResultStatus::Failed,
        _ => GridResultStatus::Completed,
    };
    GridResultBody {
        job_id: job_id.0.clone(),
        status: grid_status,
        output_artifact_ids,
        proof: None,
        metrics: None,
        lease_epoch,
    }
}

fn parse_job_kind(s: &str) -> JobKind {
    match s {
        "training" => JobKind::Training,
        "fine_tune" => JobKind::FineTune,
        "indexing" => JobKind::Indexing,
        "embeddings" => JobKind::Embeddings,
        "memory" => JobKind::Memory,
        "system" => JobKind::System,
        _ => JobKind::Inference,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::job::types::JobResources;
    use chrono::Utc;

    #[test]
    fn job_spec_grid_round_trip() {
        let spec = JobSpec {
            id: JobId::new("job-1"),
            kind: JobKind::Inference,
            resources: JobResources {
                cpu_threads: Some(4),
                ..Default::default()
            },
            priority: 1,
            max_duration_secs: Some(3600),
            input_artifact_ids: vec!["art-1".into()],
            verification_policy: Some("quorum".into()),
            deadline: Some(Utc::now()),
        };
        let env = envelope_from_job_spec(&spec);
        let json = env.to_json().expect("serialize");
        let parsed = GridEnvelope::from_json(&json).expect("parse");
        let back = job_spec_from_envelope(&parsed).expect("job message");
        assert_eq!(back.id, spec.id);
        assert_eq!(back.kind, spec.kind);
        assert_eq!(back.input_artifact_ids, spec.input_artifact_ids);
    }
}
