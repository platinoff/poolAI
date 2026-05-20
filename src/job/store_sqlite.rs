//! SQLite persistence for [`super::store::JobStore`] (`feature = "job-store-sqlite"`).
//!
//! Set `POOLAI_JOB_STORE=sqlite` with `POOLAI_JOB_DATA_DIR` (e.g. `data/jobs` → `jobs.db`).
//! On first open, imports `jobs.json` when the DB is empty and renames JSON to `jobs.json.migrated`.

use std::fs;
use std::path::Path;

use rusqlite::{params, Connection};

use super::store::{load_jobs_file, JOBS_FILE};
use crate::job::JobRecord;

const DB_FILE: &str = "jobs.db";
const MIGRATED_SUFFIX: &str = "json.migrated";

pub fn load(dir: &Path) -> Result<Vec<JobRecord>, String> {
    fs::create_dir_all(dir).map_err(|e| format!("mkdir {}: {e}", dir.display()))?;
    let db_path = dir.join(DB_FILE);
    let mut conn =
        Connection::open(&db_path).map_err(|e| format!("open {}: {e}", db_path.display()))?;
    init_schema(&conn)?;
    let mut jobs = read_all(&conn)?;
    let json_path = dir.join(JOBS_FILE);
    if jobs.is_empty() && json_path.exists() {
        let from_json = load_jobs_file(&json_path)?;
        if !from_json.is_empty() {
            write_all(&mut conn, &from_json)?;
            let migrated = json_path.with_extension(MIGRATED_SUFFIX);
            fs::rename(&json_path, &migrated).map_err(|e| {
                format!(
                    "rename {} → {}: {e}",
                    json_path.display(),
                    migrated.display()
                )
            })?;
            jobs = from_json;
        }
    }
    Ok(jobs)
}

pub fn persist(dir: &Path, jobs: &[JobRecord]) -> Result<(), String> {
    fs::create_dir_all(dir).map_err(|e| format!("mkdir {}: {e}", dir.display()))?;
    let db_path = dir.join(DB_FILE);
    let mut conn =
        Connection::open(&db_path).map_err(|e| format!("open {}: {e}", db_path.display()))?;
    init_schema(&conn)?;
    write_all(&mut conn, jobs)
}

fn init_schema(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS jobs (
            id TEXT PRIMARY KEY NOT NULL,
            record_json TEXT NOT NULL
        );",
    )
    .map_err(|e| format!("init schema: {e}"))
}

fn read_all(conn: &Connection) -> Result<Vec<JobRecord>, String> {
    let mut stmt = conn
        .prepare("SELECT record_json FROM jobs ORDER BY rowid")
        .map_err(|e| format!("prepare select: {e}"))?;
    let rows = stmt
        .query_map([], |row| {
            let json: String = row.get(0)?;
            Ok(json)
        })
        .map_err(|e| format!("query jobs: {e}"))?;
    let mut jobs = Vec::new();
    for row in rows {
        let json = row.map_err(|e| format!("read row: {e}"))?;
        let record: JobRecord =
            serde_json::from_str(&json).map_err(|e| format!("parse job row: {e}"))?;
        jobs.push(record);
    }
    Ok(jobs)
}

fn write_all(conn: &mut Connection, jobs: &[JobRecord]) -> Result<(), String> {
    let tx = conn
        .transaction()
        .map_err(|e| format!("begin transaction: {e}"))?;
    tx.execute("DELETE FROM jobs", [])
        .map_err(|e| format!("clear jobs: {e}"))?;
    for record in jobs {
        let json = serde_json::to_string(record).map_err(|e| format!("serialize job: {e}"))?;
        tx.execute(
            "INSERT INTO jobs (id, record_json) VALUES (?1, ?2)",
            params![record.spec.id.0, json],
        )
        .map_err(|e| format!("insert job {}: {e}", record.spec.id.0))?;
    }
    tx.commit().map_err(|e| format!("commit jobs: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::job::{JobId, JobKind, JobSpec, JobStatus};
    use chrono::Utc;
    use tempfile::TempDir;

    fn sample_record(id: &str) -> JobRecord {
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
            status: JobStatus::Submitted,
            created_at: Utc::now(),
        }
    }

    #[test]
    fn sqlite_persist_and_reload() {
        let tmp = TempDir::new().expect("tempdir");
        let dir = tmp.path();
        let record = sample_record("job-sqlite-1");

        persist(dir, std::slice::from_ref(&record)).expect("persist");
        let jobs = load(dir).expect("load");
        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].spec.id.0, "job-sqlite-1");
    }

    #[test]
    fn sqlite_migrates_json_when_db_empty() {
        let tmp = TempDir::new().expect("tempdir");
        let dir = tmp.path();
        let record = sample_record("job-migrate-1");
        let json_path = dir.join(JOBS_FILE);
        let snapshot = super::super::store::JobsFile {
            jobs: vec![record.clone()],
        };
        let data = serde_json::to_vec_pretty(&snapshot).expect("json");
        fs::write(&json_path, data).expect("write json");

        let jobs = load(dir).expect("load");
        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].spec.id.0, "job-migrate-1");
        assert!(!json_path.exists());
        assert!(dir.join("jobs.json.migrated").exists());
        assert!(dir.join(DB_FILE).exists());
    }
}
