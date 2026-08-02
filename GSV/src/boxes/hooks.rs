//! Tests/bench hooks box — run status from `target/` artifacts WITHOUT recompiling.
//!
//! - `GET /api/hooks/tests` → status + discovered test binaries under
//!   `{repo_root}/target/debug/deps/` + latest rust diagnostics (warnings/errors).
//! - `GET /api/hooks/bench` → Criterion medians (read `target/criterion/` if
//!   present) + latest `speed_index.json` wall-clock.
//!
//! Read-only: never invokes `cargo build`/`cargo test`.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::vision;

/// `/api/hooks/tests` response wire.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HooksTestsWire {
    pub test_bins: Vec<String>,
    pub diagnostics: Option<DiagnosticsSummary>,
    pub status: String,
}

/// `/api/hooks/bench` response wire.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HooksBenchWire {
    pub criterion_dirs: Vec<String>,
    pub speed_index: Option<SpeedSummary>,
    pub status: String,
}

/// Latest Rust/Clippy diagnostics summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsSummary {
    pub warnings: u64,
    pub errors: u64,
    pub ok: bool,
    pub recorded_at: Option<String>,
}

/// Latest `cargo test-ci` wall-clock summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedSummary {
    pub test_ci_wall_secs: f64,
    pub test_ci_ok: bool,
    pub recorded_at: Option<String>,
}

/// List `target/debug/deps/*.exe` test binaries (read-only).
pub fn test_bins(repo_root: &Path) -> Vec<String> {
    let dir = repo_root.join("target/debug/deps");
    let Ok(read) = fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut bins: Vec<String> = read
        .flatten()
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            let is_test = (name.starts_with("poolai") || name.starts_with("test_"))
                && name.ends_with(".exe")
                && !name.contains('\\');
            is_test.then_some(name)
        })
        .collect();
    bins.sort();
    bins
}

/// Read diagnostics summary from `docs/{development,vision}/rust_diagnostics.json`.
pub fn diagnostics(repo_root: &Path) -> Option<DiagnosticsSummary> {
    let v = vision::read_vision_json(repo_root, "rust_diagnostics.json")?;
    let latest = v.get("latest")?;
    Some(DiagnosticsSummary {
        warnings: latest.get("warnings")?.as_u64()?,
        errors: latest.get("errors")?.as_u64()?,
        ok: latest.get("ok")?.as_bool()?,
        recorded_at: latest
            .get("recorded_at")
            .and_then(|r| r.as_str())
            .map(ToOwned::to_owned),
    })
}

/// Read speed index summary from `docs/{development,vision}/speed_index.json`.
pub fn speed(repo_root: &Path) -> Option<SpeedSummary> {
    let v = vision::read_vision_json(repo_root, "speed_index.json")?;
    let latest = v.get("latest")?;
    Some(SpeedSummary {
        test_ci_wall_secs: latest.get("test_ci_wall_secs")?.as_f64()?,
        test_ci_ok: latest.get("test_ci_ok")?.as_bool()?,
        recorded_at: latest
            .get("test_ci_recorded_at")
            .and_then(|r| r.as_str())
            .map(ToOwned::to_owned),
    })
}

/// List `target/criterion/` benchmark dirs (read-only).
pub fn criterion_dirs(repo_root: &Path) -> Vec<String> {
    let dir = repo_root.join("target/criterion");
    let Ok(read) = fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut dirs: Vec<String> = read
        .flatten()
        .filter_map(|e| {
            e.path()
                .is_dir()
                .then(|| e.file_name().to_string_lossy().to_string())
        })
        .collect();
    dirs.sort();
    dirs
}

/// Serve `/api/hooks/tests`.
pub fn tests_wire(repo_root: &Path) -> HooksTestsWire {
    let diagnostics = diagnostics(repo_root);
    let test_bins = test_bins(repo_root);
    let status = if test_bins.is_empty() && diagnostics.is_none() {
        "no-artifacts"
    } else {
        "ready"
    };
    HooksTestsWire {
        test_bins,
        diagnostics,
        status: status.to_string(),
    }
}

/// Serve `/api/hooks/bench`.
pub fn bench_wire(repo_root: &Path) -> HooksBenchWire {
    let criterion_dirs = criterion_dirs(repo_root);
    let speed_index = speed(repo_root);
    let status = if criterion_dirs.is_empty() && speed_index.is_none() {
        "no-artifacts"
    } else {
        "ready"
    };
    HooksBenchWire {
        criterion_dirs,
        speed_index,
        status: status.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostics_reads_canon_or_none() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("parent");
        // The poolAI repo canon file may or may not exist during GSV-only builds.
        let _ = diagnostics(root);
        let _ = speed(root);
    }

    #[test]
    fn test_bins_no_panic_on_missing_target() {
        let tmp = std::env::temp_dir().join("gsv-no-target");
        assert!(test_bins(&tmp).is_empty());
        assert!(criterion_dirs(&tmp).is_empty());
    }
}
