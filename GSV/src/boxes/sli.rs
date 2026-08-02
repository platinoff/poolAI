//! SLI console box — command catalog from `bin/` + `scripts/` + `src/bin/`.
//!
//! Parses executable scripts and Rust bins into SLI entries (name, path, kind,
//! description, inputs). Marks entries as `used` when their name appears in the
//! recent shell history / tracker commands.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::vision;

/// One SLI catalog entry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SliEntry {
    /// Command name (file stem).
    pub name: String,
    /// Repo-relative path.
    pub path: String,
    /// Kind: sh | rs | bin.
    pub kind: String,
    /// One-line description (doc comment / shebang doc).
    pub description: String,
    /// Whether the command appears in recent history (used).
    pub used: bool,
    /// Invocation example.
    pub example: String,
}

/// Full SLI catalog wire.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SliCatalog {
    /// Catalog entries (sorted by name).
    pub entries: Vec<SliEntry>,
    /// Directory roots scanned.
    pub roots: Vec<String>,
    /// Count of used commands.
    pub used_count: usize,
    /// Count of unused scripts (potential new SLI functions).
    pub unused_count: usize,
}

/// `/api/sli` response wire.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliWire {
    pub catalog: SliCatalog,
    pub generated_at: String,
}

impl SliCatalog {
    /// Scan the repo (best effort): read-only, never mutates.
    pub fn scan(repo_root: &Path) -> Self {
        let mut entries = Vec::new();
        for (rel_dir, kind) in [("bin", "sh"), ("scripts", "sh"), ("src/bin", "rs")] {
            let dir = repo_root.join(rel_dir);
            let Ok(read) = fs::read_dir(&dir) else {
                continue;
            };
            for entry in read.flatten() {
                let path = entry.path();
                let Some(name) = path.file_stem().and_then(|s| s.to_str()) else {
                    continue;
                };
                let is_rs = kind == "rs" && path.extension().is_some_and(|e| e == "rs");
                let is_sh = kind == "sh"
                    && (path.extension().is_some_and(|e| e == "sh") || {
                        // shebang detection for extensionless scripts
                        fs::read_to_string(&path)
                            .ok()
                            .is_some_and(|raw| raw.starts_with("#!") || raw.contains("#!/"))
                    });
                if !is_rs && !is_sh {
                    continue;
                }
                let description = first_doc_line(&path, kind);
                let rel = format!("{rel_dir}/{name}");
                entries.push(SliEntry {
                    example: example_for(kind, name),
                    kind: kind.to_string(),
                    path: rel,
                    description,
                    name: name.to_string(),
                    used: is_used(name),
                });
            }
        }
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        let used_count = entries.iter().filter(|e| e.used).count();
        let unused_count = entries.len() - used_count;
        let roots = vec![
            "bin/".to_string(),
            "scripts/".to_string(),
            "src/bin/".to_string(),
        ];
        Self {
            entries,
            roots,
            used_count,
            unused_count,
        }
    }
}

/// First doc/comment line for a script or Rust bin.
fn first_doc_line(path: &Path, kind: &str) -> String {
    let Ok(raw) = fs::read_to_string(path) else {
        return String::new();
    };
    for line in raw.lines() {
        let line = line.trim_start();
        let desc = if kind == "rs" {
            line.strip_prefix("//!")
                .or_else(|| line.strip_prefix("///"))
        } else {
            line.strip_prefix('#')
        };
        if let Some(desc) = desc {
            let desc = desc.trim();
            if !desc.is_empty() {
                return desc.to_string();
            }
        }
    }
    String::new()
}

fn example_for(kind: &str, name: &str) -> String {
    match kind {
        "sh" => format!("bash {name}.sh …"),
        _ => format!("cargo run --bin {name} -- …"),
    }
}

/// Whether the command name appears in recent shell history.
fn is_used(name: &str) -> bool {
    crate::tracker::recent_commands(500)
        .into_iter()
        .any(|cmd| cmd.contains(name))
}

/// Serve `/api/sli`.
pub fn wire(repo_root: &Path) -> SliWire {
    SliWire {
        catalog: SliCatalog::scan(repo_root),
        generated_at: vision::rfc3339_now(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_doc_line_rs() {
        let dir = std::env::temp_dir().join(format!("gsv-sli-{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        let f = dir.join("tool.rs");
        fs::write(&f, "//! A tool doc\n//! second\nfn main() {}").expect("write");
        assert_eq!(first_doc_line(&f, "rs"), "A tool doc");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn example_for_kinds() {
        assert!(example_for("sh", "run").starts_with("bash"));
        assert!(example_for("rs", "sync").starts_with("cargo"));
    }
}
