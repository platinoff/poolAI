//! SQLite persistence for per-peer trust scores (PH-S910, Galaxy §6.5).

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use rusqlite::{params, Connection};

use crate::grid::galaxy_trust_score::TrustScore;

const DB_FILE: &str = "trust_scores.db";
const JSON_LEGACY: &str = "trust_scores.json";
const MIGRATED_SUFFIX: &str = "json.migrated";

/// Env: data directory for SQLite trust store (`trust_scores.db`).
pub const ENV_TRUST_SCORE_DATA_DIR: &str = "POOLAI_TRUST_SCORE_DATA_DIR";

/// Env: `sqlite` enables SQLite backend when [`ENV_TRUST_SCORE_DATA_DIR`] is set.
pub const ENV_TRUST_SCORE_STORE: &str = "POOLAI_TRUST_SCORE_STORE";

pub fn data_dir_from_env() -> Option<PathBuf> {
    std::env::var(ENV_TRUST_SCORE_DATA_DIR)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
}

pub fn sqlite_enabled() -> bool {
    std::env::var(ENV_TRUST_SCORE_STORE)
        .map(|v| v.trim().eq_ignore_ascii_case("sqlite"))
        .unwrap_or(false)
}

pub fn load(dir: &Path) -> Result<HashMap<String, TrustScore>, String> {
    fs::create_dir_all(dir).map_err(|e| format!("mkdir {}: {e}", dir.display()))?;
    let db_path = dir.join(DB_FILE);
    let mut conn =
        Connection::open(&db_path).map_err(|e| format!("open {}: {e}", db_path.display()))?;
    init_schema(&conn)?;
    let mut map = read_all(&conn)?;
    let json_path = dir.join(JSON_LEGACY);
    if map.is_empty() && json_path.exists() {
        let text = fs::read_to_string(&json_path)
            .map_err(|e| format!("read {}: {e}", json_path.display()))?;
        let from_json: HashMap<String, TrustScore> = serde_json::from_str(&text)
            .map_err(|e| format!("parse {}: {e}", json_path.display()))?;
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
            map = from_json;
        }
    }
    Ok(map)
}

pub fn persist(dir: &Path, map: &HashMap<String, TrustScore>) -> Result<(), String> {
    fs::create_dir_all(dir).map_err(|e| format!("mkdir {}: {e}", dir.display()))?;
    let db_path = dir.join(DB_FILE);
    let mut conn =
        Connection::open(&db_path).map_err(|e| format!("open {}: {e}", db_path.display()))?;
    init_schema(&conn)?;
    write_all(&mut conn, map)
}

/// Import legacy single-file JSON path into SQLite dir on first open (PH-S910).
pub fn migrate_legacy_json_file(dir: &Path, legacy_path: &Path) -> Result<(), String> {
    if !legacy_path.exists() {
        return Ok(());
    }
    let db_path = dir.join(DB_FILE);
    let mut conn =
        Connection::open(&db_path).map_err(|e| format!("open {}: {e}", db_path.display()))?;
    init_schema(&conn)?;
    if !read_all(&conn)?.is_empty() {
        return Ok(());
    }
    let text = fs::read_to_string(legacy_path)
        .map_err(|e| format!("read {}: {e}", legacy_path.display()))?;
    let from_json: HashMap<String, TrustScore> =
        serde_json::from_str(&text).map_err(|e| format!("parse {}: {e}", legacy_path.display()))?;
    if from_json.is_empty() {
        return Ok(());
    }
    write_all(&mut conn, &from_json)?;
    let migrated = legacy_path.with_extension(MIGRATED_SUFFIX);
    fs::rename(legacy_path, &migrated).map_err(|e| {
        format!(
            "rename {} → {}: {e}",
            legacy_path.display(),
            migrated.display()
        )
    })
}

fn init_schema(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS peer_trust (
            peer_id TEXT PRIMARY KEY NOT NULL,
            score INTEGER NOT NULL
        );",
    )
    .map_err(|e| format!("init schema: {e}"))
}

fn read_all(conn: &Connection) -> Result<HashMap<String, TrustScore>, String> {
    let mut stmt = conn
        .prepare("SELECT peer_id, score FROM peer_trust")
        .map_err(|e| format!("prepare select: {e}"))?;
    let rows = stmt
        .query_map([], |row| {
            let peer_id: String = row.get(0)?;
            let score: i64 = row.get(1)?;
            Ok((peer_id, score.clamp(0, 100) as TrustScore))
        })
        .map_err(|e| format!("query peer_trust: {e}"))?;
    let mut map = HashMap::new();
    for row in rows {
        let (peer_id, score) = row.map_err(|e| format!("read row: {e}"))?;
        map.insert(peer_id, score);
    }
    Ok(map)
}

fn write_all(conn: &mut Connection, map: &HashMap<String, TrustScore>) -> Result<(), String> {
    let tx = conn
        .transaction()
        .map_err(|e| format!("begin transaction: {e}"))?;
    tx.execute("DELETE FROM peer_trust", [])
        .map_err(|e| format!("clear peer_trust: {e}"))?;
    for (peer_id, score) in map {
        tx.execute(
            "INSERT INTO peer_trust (peer_id, score) VALUES (?1, ?2)",
            params![peer_id, i64::from(*score)],
        )
        .map_err(|e| format!("insert peer_trust {peer_id}: {e}"))?;
    }
    tx.commit().map_err(|e| format!("commit peer_trust: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn trust_score_sqlite_roundtrip_ph_s910() {
        let tmp = TempDir::new().expect("tempdir");
        let dir = tmp.path();
        let mut map = HashMap::new();
        map.insert("tg-edge-1".to_string(), 72u8);
        map.insert("peer-local".to_string(), 50u8);
        persist(dir, &map).expect("persist");
        let loaded = load(dir).expect("load");
        assert_eq!(loaded.get("tg-edge-1"), Some(&72));
        assert_eq!(loaded.get("peer-local"), Some(&50));
    }

    #[test]
    fn trust_score_sqlite_migrates_json_ph_s910() {
        let tmp = TempDir::new().expect("tempdir");
        let dir = tmp.path();
        let json_path = dir.join(JSON_LEGACY);
        fs::write(&json_path, r#"{"tg-migrate":88}"#).expect("write json");
        let loaded = load(dir).expect("load");
        assert_eq!(loaded.get("tg-migrate"), Some(&88));
        assert!(!json_path.exists());
        assert!(dir.join(format!("trust_scores.{MIGRATED_SUFFIX}")).exists());
    }
}
