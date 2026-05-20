//! File-backed job store (post-S38). Set `POOLAI_JOB_DATA_DIR` (e.g. `data/jobs`) to persist
//! across coordinator restarts; when unset, in-memory only (same as S38 stub).

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};

use crate::core::error::AppError;
use crate::job::{allows_transition, JobRecord, JobStatus};

const JOBS_FILE: &str = "jobs.json";

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
struct JobsFile {
    jobs: Vec<JobRecord>,
}

/// In-process job registry with optional JSON persistence.
pub struct JobStore {
    jobs: Mutex<Vec<JobRecord>>,
    data_dir: Option<PathBuf>,
}

impl JobStore {
    /// Shared store for HTTP handlers (respects `POOLAI_JOB_DATA_DIR` at first use).
    pub fn global() -> &'static JobStore {
        static STORE: LazyLock<JobStore> = LazyLock::new(|| JobStore::open(data_dir_from_env()));
        &STORE
    }

    #[cfg(test)]
    pub fn open_for_test(data_dir: Option<PathBuf>) -> Self {
        Self::open(data_dir)
    }

    fn open(data_dir: Option<PathBuf>) -> Self {
        let jobs = data_dir
            .as_ref()
            .map(|d| d.join(JOBS_FILE))
            .and_then(|p| load_jobs_file(&p).ok())
            .unwrap_or_default();
        Self {
            jobs: Mutex::new(jobs),
            data_dir,
        }
    }

    pub fn list(&self) -> Result<Vec<JobRecord>, AppError> {
        let guard = self
            .jobs
            .lock()
            .map_err(|_| AppError::InternalError("job store lock poisoned".into()))?;
        Ok(guard.clone())
    }

    pub fn get(&self, id: &str) -> Result<Option<JobRecord>, AppError> {
        let guard = self
            .jobs
            .lock()
            .map_err(|_| AppError::InternalError("job store lock poisoned".into()))?;
        Ok(guard.iter().find(|r| r.spec.id.0 == id).cloned())
    }

    pub fn push(&self, record: JobRecord) -> Result<(), AppError> {
        {
            let mut guard = self
                .jobs
                .lock()
                .map_err(|_| AppError::InternalError("job store lock poisoned".into()))?;
            guard.push(record);
        }
        self.persist()
    }

    /// FM-023: set status without lifecycle checks (grid `Result` ingress).
    pub fn force_status(&self, id: &str, status: JobStatus) -> Result<JobRecord, AppError> {
        {
            let mut guard = self
                .jobs
                .lock()
                .map_err(|_| AppError::InternalError("job store lock poisoned".into()))?;
            let record = guard
                .iter_mut()
                .find(|r| r.spec.id.0 == id)
                .ok_or_else(|| AppError::ApiNotFound(format!("job '{id}' not found")))?;
            record.status = status;
        }
        self.persist()?;
        self.get(id)?
            .ok_or_else(|| AppError::InternalError("job missing after force_status".into()))
    }

    /// FM-021: update job status when lifecycle transition is valid; persists on success.
    pub fn update_status(&self, id: &str, status: JobStatus) -> Result<JobRecord, AppError> {
        let updated = {
            let mut guard = self
                .jobs
                .lock()
                .map_err(|_| AppError::InternalError("job store lock poisoned".into()))?;
            let record = guard
                .iter_mut()
                .find(|r| r.spec.id.0 == id)
                .ok_or_else(|| AppError::ApiNotFound(format!("job '{id}' not found")))?;
            if !allows_transition(record.status, status) {
                return Err(AppError::ValidationError(format!(
                    "cannot transition job '{id}' from {:?} to {:?}",
                    record.status, status
                )));
            }
            record.status = status;
            record.clone()
        };
        self.persist()?;
        Ok(updated)
    }

    /// FM-020: transition all `Submitted` rows to `Scheduled` (priority desc), then persist once.
    pub fn promote_submitted_to_scheduled(&self) -> Result<usize, AppError> {
        let scheduled = {
            let mut guard = self
                .jobs
                .lock()
                .map_err(|_| AppError::InternalError("job store lock poisoned".into()))?;
            let mut indices: Vec<usize> = guard
                .iter()
                .enumerate()
                .filter(|(_, r)| r.status == JobStatus::Submitted)
                .map(|(i, _)| i)
                .collect();
            indices.sort_by(|&a, &b| guard[b].spec.priority.cmp(&guard[a].spec.priority));
            let count = indices.len();
            for i in indices {
                guard[i].status = JobStatus::Scheduled;
            }
            count
        };
        if scheduled > 0 {
            self.persist()?;
        }
        Ok(scheduled)
    }

    fn persist(&self) -> Result<(), AppError> {
        let Some(dir) = self.data_dir.as_ref() else {
            return Ok(());
        };
        let guard = self
            .jobs
            .lock()
            .map_err(|_| AppError::InternalError("job store lock poisoned".into()))?;
        let path = dir.join(JOBS_FILE);
        let snapshot = JobsFile {
            jobs: guard.clone(),
        };
        write_json_atomic(&path, &snapshot)
            .map_err(|e| AppError::InternalError(format!("persist jobs: {e}")))
    }
}

pub fn data_dir_from_env() -> Option<PathBuf> {
    std::env::var("POOLAI_JOB_DATA_DIR")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(PathBuf::from)
}

fn load_jobs_file(path: &Path) -> Result<Vec<JobRecord>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let bytes = fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let file: JobsFile =
        serde_json::from_slice(&bytes).map_err(|e| format!("parse {}: {e}", path.display()))?;
    Ok(file.jobs)
}

fn write_json_atomic<T: serde::Serialize>(path: &Path, value: &T) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir {}: {e}", parent.display()))?;
    }
    let data = serde_json::to_vec_pretty(value)
        .map_err(|e| format!("serialize {}: {e}", path.display()))?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, &data).map_err(|e| format!("write {}: {e}", tmp.display()))?;
    fs::rename(&tmp, path).map_err(|e| format!("rename {}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::job::{JobId, JobKind, JobSpec, JobStatus};
    use chrono::Utc;
    use std::sync::{Mutex, OnceLock};
    use tempfile::TempDir;

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    #[test]
    fn persist_and_reload_jobs() {
        let _guard = env_lock();
        let tmp = TempDir::new().expect("tempdir");
        let dir = tmp.path().to_path_buf();

        let record = JobRecord {
            spec: JobSpec {
                id: JobId::new("job-persist-1"),
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
        };

        {
            let store = JobStore::open_for_test(Some(dir.clone()));
            store.push(record.clone()).expect("push");
        }

        let reloaded = JobStore::open_for_test(Some(dir));
        let jobs = reloaded.list().expect("list");
        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].spec.id.0, "job-persist-1");
    }

    #[test]
    fn update_status_persists() {
        let _guard = env_lock();
        let tmp = TempDir::new().expect("tempdir");
        let dir = tmp.path().to_path_buf();

        let record = JobRecord {
            spec: JobSpec {
                id: JobId::new("job-patch-1"),
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
        };

        {
            let store = JobStore::open_for_test(Some(dir.clone()));
            store.push(record).expect("push");
            store
                .update_status("job-patch-1", JobStatus::Executing)
                .expect("patch");
        }

        let reloaded = JobStore::open_for_test(Some(dir));
        let job = reloaded.get("job-patch-1").expect("get").expect("row");
        assert_eq!(job.status, JobStatus::Executing);
    }

    #[test]
    fn update_status_rejects_invalid_transition() {
        let store = JobStore::open_for_test(None);
        let record = JobRecord {
            spec: JobSpec {
                id: JobId::new("job-bad"),
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
        };
        store.push(record).expect("push");
        let err = store
            .update_status("job-bad", JobStatus::Completed)
            .expect_err("invalid");
        assert!(matches!(err, AppError::ValidationError(_)));
    }
}
