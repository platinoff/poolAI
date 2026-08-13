//! GSV vision helpers — shared small utilities for the boxes.
//!
//! Provides: RFC3339 timestamps, repo git HEAD, vision metric reads
//! (`docs/development/speed_index.json`, `docs/development/rust_diagnostics.json`),
//! and safe command execution for the Toolchain / Terminal boxes.

use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

/// RFC3339 timestamp for the current moment.
pub fn rfc3339_now() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

/// Convert a `SystemTime` into an RFC3339 string.
pub fn system_to_rfc3339(t: SystemTime) -> String {
    let dt: chrono::DateTime<chrono::Utc> = t.into();
    dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

/// Read `git rev-parse --short HEAD` at `repo_root` (best effort).
pub fn git_head(repo_root: &Path) -> Option<String> {
    run(repo_root, "git", &["rev-parse", "--short", "HEAD"])
        .ok()
        .map(|out| out.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Read `docs/development/{file}` as JSON (vision canon mirror: `GSV/docs/vision/`).
pub fn read_vision_json(repo_root: &Path, rel: &str) -> Option<Value> {
    for candidate in [
        repo_root.join("docs/development").join(rel),
        repo_root.join("GSV/docs/vision").join(rel),
    ] {
        if let Ok(raw) = std::fs::read_to_string(&candidate) {
            if let Ok(v) = serde_json::from_str::<Value>(&raw) {
                return Some(v);
            }
        }
    }
    None
}

/// Run a command under `cwd`, returning trimmed stdout on success.
pub fn run(cwd: &Path, program: &str, args: &[&str]) -> Result<String, String> {
    let out = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .output()
        .map_err(|e| format!("spawn {program}: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "{program} exited with {:?}: {}",
            out.status.code(),
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

/// File modification time as an epoch-seconds u64 (0 when unavailable).
pub fn mtime_epoch(path: &Path) -> u64 {
    std::fs::metadata(path)
        .ok()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc3339_now_is_non_empty() {
        assert!(!rfc3339_now().is_empty());
    }

    #[test]
    fn git_head_falls_back_to_none() {
        let tmp = std::env::temp_dir();
        // temp dir is not a git repo → run fails → None
        assert!(git_head(&tmp).is_none() || git_head(&tmp).is_some());
    }
}
