//! IDE box — opencode + cursor chat sessions, read-only scan, selectable.
//!
//! Discovers chat/session artifacts under `~/.local/share/opencode/` (opencode
//! storage) and `.cursor/` (cursor config/sessions). The active selection is kept
//! in-memory on `AppState.ide_selection`.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::vision;

/// One discovered session/chat artifact.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdeSession {
    /// Tool: opencode | cursor.
    pub tool: String,
    /// Stable session id (relative path).
    pub id: String,
    /// Human label (file/dir name).
    pub label: String,
    /// Absolute path (read-only).
    pub path: String,
    /// Modified timestamp (RFC3339 best effort).
    pub modified: String,
}

/// User selection of an IDE session.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdeSelection {
    pub tool: String,
    pub session: String,
    pub selected_at: String,
}

/// `/api/ide/sessions` response wire.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeWire {
    pub sessions: Vec<IdeSession>,
    pub selection: Option<IdeSelection>,
    pub generated_at: String,
}

/// Home directory (USERPROFILE on Windows, HOME elsewhere).
pub fn home_dir() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(PathBuf::from))
}

fn mtime_rfc3339(path: &Path) -> String {
    let m = fs::metadata(path)
        .ok()
        .and_then(|m| m.modified().ok())
        .unwrap_or(SystemTime::UNIX_EPOCH);
    crate::vision::system_to_rfc3339(m)
}

/// Scan opencode sessions under `{home}/.local/share/opencode/`.
fn scan_opencode() -> Vec<IdeSession> {
    let Some(home) = home_dir() else {
        return Vec::new();
    };
    let base = home.join(".local/share/opencode");
    let mut out = Vec::new();
    collect_jsonl(&base, "opencode", &mut out);
    out
}

/// Scan cursor artifacts under `{home}/.cursor/`.
fn scan_cursor() -> Vec<IdeSession> {
    let Some(home) = home_dir() else {
        return Vec::new();
    };
    let base = home.join(".cursor");
    let mut out = Vec::new();
    collect_jsonl(&base, "cursor", &mut out);
    if out.is_empty() {
        // Fall back to the repo-local `.cursor/` config (rules, README).
        if let Ok(read) = fs::read_dir(base.join("rules")) {
            for e in read.flatten() {
                if e.path().extension().is_some_and(|x| x == "mdc") {
                    out.push(IdeSession {
                        tool: "cursor".to_string(),
                        id: format!("cursor/rules/{}", e.file_name().to_string_lossy()),
                        label: e.file_name().to_string_lossy().to_string(),
                        path: e.path().to_string_lossy().to_string(),
                        modified: mtime_rfc3339(&e.path()),
                    });
                }
            }
        }
    }
    out
}

/// Collect `*.jsonl` chat/session files under `base` (recursive, capped).
fn collect_jsonl(base: &Path, tool: &str, out: &mut Vec<IdeSession>) {
    let Ok(read) = fs::read_dir(base) else {
        return;
    };
    let mut stack: Vec<PathBuf> = read.flatten().map(|e| e.path()).collect();
    let mut guard = 0usize;
    while let Some(p) = stack.pop() {
        guard += 1;
        if guard > 2000 {
            break;
        }
        let Ok(meta) = fs::metadata(&p) else {
            continue;
        };
        if meta.is_dir() {
            if let Ok(rd) = fs::read_dir(&p) {
                stack.extend(rd.flatten().map(|e| e.path()));
            }
            continue;
        }
        let is_chat = p.extension().is_some_and(|x| x == "jsonl")
            || p.file_name()
                .is_some_and(|n| n.to_string_lossy().contains("chat"));
        if !is_chat {
            continue;
        }
        out.push(IdeSession {
            tool: tool.to_string(),
            id: format!(
                "{tool}/{}",
                p.strip_prefix(base).unwrap_or(&p).to_string_lossy()
            ),
            label: p
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default(),
            path: p.to_string_lossy().to_string(),
            modified: mtime_rfc3339(&p),
        });
    }
    out.sort_by(|a, b| b.modified.cmp(&a.modified));
}

/// Discover all IDE sessions (opencode + cursor).
pub fn discover() -> Vec<IdeSession> {
    let mut sessions = scan_opencode();
    sessions.extend(scan_cursor());
    sessions
}

/// Serve `/api/ide/sessions`.
pub fn wire(selection: Option<&IdeSelection>) -> IdeWire {
    IdeWire {
        sessions: discover(),
        selection: selection.cloned(),
        generated_at: vision::rfc3339_now(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discover_is_read_only() {
        // Must never panic; returns a list (possibly empty on CI).
        let _ = discover();
    }

    #[test]
    fn selection_serializes() {
        let sel = IdeSelection {
            tool: "opencode".to_string(),
            session: "opencode/abc.jsonl".to_string(),
            selected_at: "now".to_string(),
        };
        let raw = serde_json::to_string(&sel).expect("json");
        assert!(raw.contains("opencode"));
    }
}
