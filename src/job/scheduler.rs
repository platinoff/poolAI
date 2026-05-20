//! In-process job scheduler MVP (FM-020): `Submitted` → `Scheduled` without VM binding.

use crate::core::error::AppError;
use crate::job::store::JobStore;
/// Promote all `Submitted` jobs to `Scheduled`, highest `priority` first.
pub fn schedule_pending(store: &JobStore) -> Result<usize, AppError> {
    store.promote_submitted_to_scheduled()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::job::{JobId, JobKind, JobRecord, JobSpec, JobStatus};
    use chrono::Utc;
    use tempfile::TempDir;

    fn sample_record(id: &str, priority: u8) -> JobRecord {
        JobRecord {
            spec: JobSpec {
                id: JobId::new(id),
                kind: JobKind::Inference,
                resources: Default::default(),
                priority,
                max_duration_secs: None,
                input_artifact_ids: vec![],
                verification_policy: None,
                deadline: None,
            },
            status: JobStatus::Submitted,
            created_at: Utc::now(),
        }
    }

    #[test]
    fn promotes_submitted_by_priority() {
        let tmp = TempDir::new().expect("tempdir");
        let store = JobStore::open_for_test(Some(tmp.path().to_path_buf()));

        store.push(sample_record("low", 1)).expect("push");
        store.push(sample_record("high", 10)).expect("push");

        let n = schedule_pending(&store).expect("schedule");
        assert_eq!(n, 2);

        let high = store.get("high").expect("get").expect("row");
        let low = store.get("low").expect("get").expect("row");
        assert_eq!(high.status, JobStatus::Scheduled);
        assert_eq!(low.status, JobStatus::Scheduled);
    }

    #[test]
    fn schedule_persists_across_reload() {
        let tmp = TempDir::new().expect("tempdir");
        let dir = tmp.path().to_path_buf();

        {
            let store = JobStore::open_for_test(Some(dir.clone()));
            store.push(sample_record("job-1", 0)).expect("push");
            schedule_pending(&store).expect("schedule");
        }

        let reloaded = JobStore::open_for_test(Some(dir));
        let job = reloaded.get("job-1").expect("get").expect("row");
        assert_eq!(job.status, JobStatus::Scheduled);
    }

    #[test]
    fn skips_non_submitted() {
        let store = JobStore::open_for_test(None);
        let mut record = sample_record("done", 0);
        record.status = JobStatus::Executing;
        store.push(record).expect("push");

        let n = schedule_pending(&store).expect("schedule");
        assert_eq!(n, 0);
        assert_eq!(
            store.get("done").expect("get").expect("row").status,
            JobStatus::Executing
        );
    }
}
