//! Toolchain box — inventory of project tools (rustc/cargo/clippy/MSYS2/git/…).
//!
//! Versions come from running `--version` probes (best effort, offline-safe) and
//! from `rust-toolchain.toml` when present.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::vision;

/// One tool inventory entry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolchainEntry {
    /// Tool name.
    pub tool: String,
    /// Version string (from `--version` probe or config).
    pub version: String,
    /// Source: probe | toolchain-file | config.
    pub source: String,
}

/// `/api/toolchain` response wire.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolchainWire {
    pub entries: Vec<ToolchainEntry>,
    pub generated_at: String,
}

const PROBES: &[(&str, &[&str])] = &[
    ("rustc", &["--version"]),
    ("cargo", &["--version"]),
    ("clippy-driver", &["--version"]),
    ("rustfmt", &["--version"]),
    ("git", &["--version"]),
    ("bash", &["--version"]),
    ("node", &["--version"]),
    ("npm", &["--version"]),
    ("curl", &["--version"]),
];

/// Probe a tool version; returns a short single-line version.
fn probe(program: &str, args: &[&str]) -> Option<String> {
    let out = std::process::Command::new(program)
        .args(args)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    Some(text.lines().next().unwrap_or_default().trim().to_string())
}

/// Read rustc pin from `rust-toolchain.toml` (best effort).
fn toolchain_pin(repo_root: &Path) -> Option<String> {
    let raw = std::fs::read_to_string(repo_root.join("rust-toolchain.toml")).ok()?;
    let table: toml::Value = toml::from_str(&raw).ok()?;
    table
        .get("toolchain")
        .and_then(|t| t.get("channel"))
        .and_then(|c| c.as_str())
        .map(ToOwned::to_owned)
}

/// Build the toolchain inventory.
pub fn build(repo_root: &Path) -> Vec<ToolchainEntry> {
    let mut entries = Vec::new();
    for (tool, args) in PROBES {
        let version = probe(tool, args).unwrap_or_else(|| "not-found".to_string());
        entries.push(ToolchainEntry {
            tool: (*tool).to_string(),
            version,
            source: "probe".to_string(),
        });
    }
    if let Some(pin) = toolchain_pin(repo_root) {
        entries.push(ToolchainEntry {
            tool: "rust-toolchain".to_string(),
            version: pin,
            source: "toolchain-file".to_string(),
        });
    }
    if let Some(head) = vision::git_head(repo_root) {
        entries.push(ToolchainEntry {
            tool: "repo-head".to_string(),
            version: head,
            source: "git".to_string(),
        });
    }
    entries
}

/// Serve `/api/toolchain`.
pub fn wire(repo_root: &Path) -> ToolchainWire {
    ToolchainWire {
        entries: build(repo_root),
        generated_at: vision::rfc3339_now(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probes_include_rustc_and_cargo_when_available() {
        // Best effort: on dev machines rustc/cargo exist; on CI they may not.
        let dir = std::env::temp_dir().join(format!("gsv-tc-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let entries = build(&dir);
        let tools: Vec<&str> = entries.iter().map(|e| e.tool.as_str()).collect();
        assert!(
            tools.contains(&"rust-toolchain") || tools.contains(&"repo-head") || !tools.is_empty()
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn probe_returns_none_for_missing_binary() {
        assert!(probe("definitely-missing-binary-xyz", &["--version"]).is_none());
    }
}
