//! Job store persistence backends.
//!
//! - JSON file (`jobs.json`) by default
//! - Optional SQLite (`jobs.db`) with `feature = "job-store-sqlite"` and `POOLAI_JOB_STORE=sqlite`
//! - RAID-backed snapshot when `POOLAI_JOB_STORE=raid` (PH-S48)
//!
//! For JSON/SQLite, set `POOLAI_JOB_DATA_DIR` (e.g. `data/jobs`) to persist across restarts.
//! For RAID, job snapshot is stored as a RAID artifact (logical name constant below).
//!
//! Default backend: JSON (`jobs.json`). With `feature = "job-store-sqlite"` and
//! `POOLAI_JOB_STORE=sqlite`, uses `jobs.db` and migrates legacy `jobs.json` on first open.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, Mutex};

use crate::core::error::AppError;
use crate::raid::{self, RaidManager};
use chrono::Utc;

use crate::job::lease_acquire::{
    acquire_lease_on_record, maybe_acquire_lease_on_schedule, resolve_lease_owner,
    AcquireLeaseError,
};
use crate::job::onchain::emit_job_completed_if_anchor;
use crate::job::{allows_transition, JobLeaseConfig, JobRecord, JobSpec, JobStatus};

pub(crate) const JOBS_FILE: &str = "jobs.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PersistBackend {
    Json,
    Raid,
    #[cfg(feature = "job-store-sqlite")]
    Sqlite,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub(crate) struct JobsFile {
    pub jobs: Vec<JobRecord>,
}

/// In-process job registry with optional JSON or SQLite persistence.
pub struct JobStore {
    jobs: Mutex<Vec<JobRecord>>,
    data_dir: Option<PathBuf>,
    backend: PersistBackend,
}

impl JobStore {
    /// Shared store for HTTP handlers (respects `POOLAI_JOB_DATA_DIR` at first use).
    pub fn global() -> &'static JobStore {
        static STORE: LazyLock<JobStore> = LazyLock::new(|| JobStore::open(data_dir_from_env()));
        &STORE
    }

    #[cfg(any(test, feature = "test-utils"))]
    pub fn open_for_test(data_dir: Option<PathBuf>) -> Self {
        Self::open(data_dir)
    }

    fn open(data_dir: Option<PathBuf>) -> Self {
        let backend = persist_backend_from_env();
        let jobs = match backend {
            PersistBackend::Raid => load_jobs_from_raid(raid_manager_from_env()).ok(),
            _ => data_dir
                .as_ref()
                .map(|d| load_persisted_jobs(d, backend))
                .transpose()
                .ok()
                .flatten(),
        }
        .unwrap_or_default();
        Self {
            jobs: Mutex::new(jobs),
            data_dir,
            backend,
        }
    }

    /// Active persistence backend label for admin UI / OpenAPI (`json`, `sqlite`, `raid`).
    pub fn store_backend_label(&self) -> &'static str {
        match self.backend {
            PersistBackend::Json => "json",
            PersistBackend::Raid => "raid",
            #[cfg(feature = "job-store-sqlite")]
            PersistBackend::Sqlite => "sqlite",
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
        let row = self
            .get(id)?
            .ok_or_else(|| AppError::InternalError("job missing after force_status".into()))?;
        emit_job_completed_if_anchor(&row);
        Ok(row)
    }

    /// PH-S98: explicit lease acquire (`POST /api/v1/jobs/{id}/lease`).
    pub fn acquire_lease(
        &self,
        id: &str,
        lease_owner: Option<String>,
    ) -> Result<JobRecord, AppError> {
        let now = Utc::now();
        let updated = {
            let mut guard = self
                .jobs
                .lock()
                .map_err(|_| AppError::InternalError("job store lock poisoned".into()))?;
            let record = guard
                .iter_mut()
                .find(|r| r.spec.id.0 == id)
                .ok_or_else(|| AppError::ApiNotFound(format!("job '{id}' not found")))?;
            let owner = resolve_lease_owner(
                lease_owner.as_deref(),
                record.worker_id.as_deref(),
                record.vm_id.as_deref(),
            )
            .ok_or_else(|| {
                AppError::ValidationError(
                    "lease_owner required when job has no worker_id or vm_id binding".into(),
                )
            })?;
            let cfg = JobLeaseConfig::from_env();
            acquire_lease_on_record(record, &owner, &cfg, now, true).map_err(|e| match e {
                AcquireLeaseError::NoLeaseOwner => {
                    AppError::ValidationError("lease_owner must be non-empty".into())
                }
                AcquireLeaseError::LeaseAlreadyActive => AppError::RestError {
                    code: "lease_already_active",
                    message: format!("job '{id}' already has an active lease (Galaxy §4.3.1)"),
                },
            })?;
            record.clone()
        };
        self.persist()?;
        Ok(updated)
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
        emit_job_completed_if_anchor(&updated);
        Ok(updated)
    }

    /// FM-020: transition all `Submitted` rows to `Scheduled` (priority desc), then persist once.
    pub fn promote_submitted_to_scheduled(&self) -> Result<usize, AppError> {
        let (scheduled, _, _, _) = self
            .promote_submitted_to_scheduled_with(|_| crate::job::JobScheduleBinding::default())?;
        Ok(scheduled)
    }

    /// FM-034 / PH-S38: promote `Submitted` → `Scheduled` (or `Failed` when past deadline).
    pub fn promote_submitted_to_scheduled_with<F>(
        &self,
        mut assign: F,
    ) -> Result<(usize, usize, usize, usize), AppError>
    where
        F: FnMut(&JobSpec) -> crate::job::JobScheduleBinding,
    {
        let (scheduled, bound_workers, bound_vms, expired) = {
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
            let mut scheduled = 0usize;
            let mut bound_workers = 0usize;
            let mut bound_vms = 0usize;
            let mut expired = 0usize;
            let now = Utc::now();
            for i in indices {
                if job_past_deadline(&guard[i].spec, now) {
                    guard[i].status = JobStatus::Failed;
                    expired += 1;
                    continue;
                }
                let binding = assign(&guard[i].spec);
                guard[i].status = JobStatus::Scheduled;
                scheduled += 1;
                if binding.worker_id.is_some() {
                    bound_workers += 1;
                }
                if binding.vm_id.is_some() {
                    bound_vms += 1;
                }
                guard[i].worker_id = binding.worker_id;
                guard[i].vm_id = binding.vm_id;
                maybe_acquire_lease_on_schedule(&mut guard[i], now);
            }
            (scheduled, bound_workers, bound_vms, expired)
        };
        if scheduled > 0 || expired > 0 {
            self.persist()?;
        }
        Ok((scheduled, bound_workers, bound_vms, expired))
    }

    fn persist(&self) -> Result<(), AppError> {
        let backend = self.backend;
        let guard = self
            .jobs
            .lock()
            .map_err(|_| AppError::InternalError("job store lock poisoned".into()))?;

        match backend {
            PersistBackend::Raid => persist_jobs_to_raid(raid_manager_from_env(), &guard),
            PersistBackend::Json => {
                let Some(dir) = self.data_dir.as_ref() else {
                    return Ok(());
                };
                persist_jobs(dir, backend, &guard)
            }
            #[cfg(feature = "job-store-sqlite")]
            PersistBackend::Sqlite => {
                let Some(dir) = self.data_dir.as_ref() else {
                    return Ok(());
                };
                persist_jobs(dir, backend, &guard)
            }
        }
        .map_err(|e| AppError::InternalError(format!("persist jobs: {e}")))
    }
}

fn job_past_deadline(spec: &JobSpec, now: chrono::DateTime<Utc>) -> bool {
    spec.deadline.is_some_and(|d| d < now)
}

const RAID_JOBS_SNAPSHOT_NAME: &str = "poolai-jobs-snapshot";

fn raid_manager_from_env() -> Arc<RaidManager> {
    // Uses RAID module singleton; requires that `POOLAI_RAID_BASE_PATH` was set before first use.
    raid::get_global_manager()
}

fn block_on_result<T, F>(fut: F) -> Result<T, String>
where
    F: std::future::Future<Output = Result<T, String>> + Send + 'static,
    T: Send + 'static,
{
    let run_on_runtime = |fut: F| -> Result<T, String> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| e.to_string())?;
        rt.block_on(fut)
    };

    // `Handle::block_on` panics when called from an async worker thread (HTTP handlers).
    if tokio::runtime::Handle::try_current().is_ok() {
        std::thread::spawn(move || run_on_runtime(fut))
            .join()
            .map_err(|_| "raid io thread panicked".to_string())?
    } else {
        run_on_runtime(fut)
    }
}

fn load_jobs_from_raid(manager: Arc<RaidManager>) -> Result<Vec<JobRecord>, String> {
    block_on_result(async move {
        let artifacts = manager.list_artifacts().await;
        let mut snapshots = artifacts
            .into_iter()
            .filter(|a| a.name == RAID_JOBS_SNAPSHOT_NAME)
            .collect::<Vec<_>>();

        if snapshots.is_empty() {
            return Ok(Vec::new());
        }

        // Pick the newest snapshot by timestamp.
        snapshots.sort_by_key(|a| a.stored_at);
        let latest = snapshots
            .last()
            .ok_or_else(|| "raid: missing latest snapshot".to_string())?;

        let bytes = manager
            .get_artifact(&latest.path)
            .await
            .map_err(|e| e.to_string())?;

        let file: JobsFile =
            serde_json::from_slice(&bytes).map_err(|e| format!("parse jobs snapshot: {e}"))?;
        Ok(file.jobs)
    })
}

fn persist_jobs_to_raid(manager: Arc<RaidManager>, jobs: &[JobRecord]) -> Result<(), String> {
    let jobs = jobs.to_vec();
    block_on_result(async move {
        let snapshot = JobsFile { jobs };
        let bytes =
            serde_json::to_vec(&snapshot).map_err(|e| format!("serialize jobs snapshot: {e}"))?;

        // Store new snapshot as a RAID artifact; then delete older snapshots with the same logical name.
        let new_artifact = manager
            .put_artifact(RAID_JOBS_SNAPSHOT_NAME, &bytes)
            .await
            .map_err(|e| e.to_string())?;

        let artifacts = manager.list_artifacts().await;
        for a in artifacts {
            if a.name == RAID_JOBS_SNAPSHOT_NAME && a.id != new_artifact.id {
                let _ = manager.delete_artifact(a.id).await;
            }
        }

        Ok(())
    })
}

pub fn data_dir_from_env() -> Option<PathBuf> {
    std::env::var("POOLAI_JOB_DATA_DIR")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(PathBuf::from)
}

fn persist_backend_from_env() -> PersistBackend {
    let want = std::env::var("POOLAI_JOB_STORE").unwrap_or_else(|_| "".to_string());
    let want = want.trim();
    if want.eq_ignore_ascii_case("raid") {
        return PersistBackend::Raid;
    }

    let want_sqlite = want.eq_ignore_ascii_case("sqlite");
    #[cfg(feature = "job-store-sqlite")]
    {
        if want_sqlite {
            return PersistBackend::Sqlite;
        }
    }
    #[cfg(not(feature = "job-store-sqlite"))]
    let _ = want_sqlite;
    if want_sqlite {
        tracing::warn!("POOLAI_JOB_STORE=sqlite ignored: rebuild with --features job-store-sqlite");
    }
    PersistBackend::Json
}

fn load_persisted_jobs(dir: &Path, backend: PersistBackend) -> Result<Vec<JobRecord>, String> {
    match backend {
        PersistBackend::Json => {
            let path = dir.join(JOBS_FILE);
            load_jobs_file(&path)
        }
        PersistBackend::Raid => load_jobs_from_raid(raid_manager_from_env()),
        #[cfg(feature = "job-store-sqlite")]
        PersistBackend::Sqlite => super::store_sqlite::load(dir),
    }
}

fn persist_jobs(dir: &Path, backend: PersistBackend, jobs: &[JobRecord]) -> Result<(), String> {
    match backend {
        PersistBackend::Json => {
            let path = dir.join(JOBS_FILE);
            let snapshot = JobsFile {
                jobs: jobs.to_vec(),
            };
            write_json_atomic(&path, &snapshot)
        }
        PersistBackend::Raid => persist_jobs_to_raid(raid_manager_from_env(), jobs),
        #[cfg(feature = "job-store-sqlite")]
        PersistBackend::Sqlite => super::store_sqlite::persist(dir, jobs),
    }
}

pub(crate) fn load_jobs_file(path: &Path) -> Result<Vec<JobRecord>, String> {
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
            worker_id: None,
            vm_id: None,
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
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
            worker_id: None,
            vm_id: None,
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
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
            worker_id: None,
            vm_id: None,
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
        };
        store.push(record).expect("push");
        let err = store
            .update_status("job-bad", JobStatus::Completed)
            .expect_err("invalid");
        assert!(matches!(err, AppError::ValidationError(_)));
    }

    #[cfg(feature = "job-store-sqlite")]
    #[test]
    fn sqlite_backend_persists_via_job_store() {
        let _guard = env_lock();
        let tmp = TempDir::new().expect("tempdir");
        let dir = tmp.path().to_path_buf();
        std::env::set_var("POOLAI_JOB_STORE", "sqlite");

        let record = JobRecord {
            spec: JobSpec {
                id: JobId::new("job-sqlite-store"),
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
            lease_owner: None,
            lease_epoch: None,
            lease_expires_at: None,
        };

        {
            let store = JobStore::open_for_test(Some(dir.clone()));
            store.push(record).expect("push");
        }

        std::env::remove_var("POOLAI_JOB_STORE");
        std::env::set_var("POOLAI_JOB_STORE", "sqlite");
        let reloaded = JobStore::open_for_test(Some(dir));
        let jobs = reloaded.list().expect("list");
        std::env::remove_var("POOLAI_JOB_STORE");
        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].spec.id.0, "job-sqlite-store");
    }
}
