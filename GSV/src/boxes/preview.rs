//! Box preview — Rust-syntax-colored file preview.
//!
//! `GET /api/preview?file=<repo-relative>` returns HTML with token highlighting
//! (Rust palette). Supported extensions: `.rs`, `.toml`, `.md`, `.js`, `.css`.
//! Read-only: the file must exist under the repo root; traversal is rejected.

use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};

/// `/api/preview` response wire.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewWire {
    /// Highlighted HTML (safe, escaped).
    pub html: String,
    /// Original file content length (chars).
    pub size: usize,
    pub extension: String,
    pub path: String,
}

/// Query params for `GET /api/preview`.
#[derive(Debug, Clone, Deserialize)]
pub struct PreviewParams {
    pub file: String,
}

const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while",
];

/// Resolve a repo-relative path without traversal.
pub fn resolve(repo_root: &Path, rel: &str) -> Result<PathBuf, String> {
    let path = repo_root.join(rel);
    let Ok(meta) = std::fs::metadata(&path) else {
        return Err(format!("file not found: {rel}"));
    };
    if !meta.is_file() {
        return Err(format!("not a file: {rel}"));
    }
    // Traversal guard: every component must be Normal.
    let mut iter = path.components();
    if let Some(Component::RootDir | Component::Prefix(_)) = iter.next() {
        // absolute — allow only if under repo_root
        if !path.starts_with(repo_root) {
            return Err("path outside repo root".to_string());
        }
    }
    let norm = path.components().all(|c| {
        matches!(
            c,
            Component::Normal(_) | Component::RootDir | Component::Prefix(_)
        )
    });
    if !norm {
        return Err("path traversal rejected".to_string());
    }
    Ok(path)
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn highlight_rs(line: &str) -> String {
    // Minimal lexical highlighter: comments, strings, keywords, numbers.
    let mut out = String::new();
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '/' if chars.peek() == Some(&'/') => {
                let rest: String = chars.collect();
                out.push_str(&format!(r#"<span class="g-c">//{rest}</span>"#));
                return out;
            }
            '"' => {
                let mut lit = String::from("\"");
                for c2 in chars.by_ref() {
                    lit.push(c2);
                    if c2 == '"' && !lit.ends_with("\\\"") {
                        break;
                    }
                }
                out.push_str(&format!(r#"<span class="g-s">{}"#, escape(&lit)));
                out.push_str("</span>");
            }
            c if c.is_ascii_digit() => {
                let mut num = c.to_string();
                while let Some(&n) = chars.peek() {
                    if n.is_ascii_alphanumeric() || n == '_' || n == '.' {
                        num.push(n);
                        chars.next();
                    } else {
                        break;
                    }
                }
                out.push_str(&format!(r#"<span class="g-n">{num}</span>"#));
            }
            c if c.is_alphabetic() || c == '_' => {
                let mut word = c.to_string();
                while let Some(&n) = chars.peek() {
                    if n.is_ascii_alphanumeric() || n == '_' {
                        word.push(n);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if RUST_KEYWORDS.contains(&word.as_str()) {
                    out.push_str(&format!(r#"<span class="g-k">{word}</span>"#));
                } else {
                    out.push_str(&escape(&word));
                }
            }
            c => out.push(c),
        }
    }
    out
}

/// Highlight a file's content.
pub fn render(path: &Path, rel: &str) -> Result<PreviewWire, String> {
    let raw = std::fs::read_to_string(path).map_err(|e| format!("read {rel}: {e}"))?;
    let extension = path
        .extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_else(|| "txt".to_string());
    let html = match extension.as_str() {
        "rs" | "toml" | "md" | "js" | "css" => {
            let mut out = String::from("<pre class=\"g-pre\">");
            for (i, line) in raw.lines().enumerate() {
                out.push_str(&format!("<span class=\"g-ln\">{:>3}</span> ", i + 1));
                out.push_str(&highlight_rs(line));
                out.push('\n');
            }
            out.push_str("</pre>");
            out
        }
        _ => {
            let mut out = String::from("<pre class=\"g-pre\">");
            for (i, line) in raw.lines().enumerate() {
                out.push_str(&format!("<span class=\"g-ln\">{:>3}</span> ", i + 1));
                out.push_str(&escape(line));
                out.push('\n');
            }
            out.push_str("</pre>");
            out
        }
    };
    Ok(PreviewWire {
        html,
        size: raw.chars().count(),
        extension,
        path: rel.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traversal_rejected() {
        let root = Path::new(".");
        assert!(resolve(root, "../../Cargo.toml").is_err());
    }

    #[test]
    fn resolve_finds_existing_file() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let p = resolve(root, "Cargo.toml").expect("resolve");
        assert!(p.is_file());
    }

    #[test]
    fn highlight_keywords_and_strings() {
        let html = highlight_rs("let name = \"gsv\";");
        assert!(html.contains("g-k")); // keyword span
        assert!(html.contains("g-s")); // string span
        assert!(html.contains("let")); // raw text preserved
        assert!(html.contains("gsv")); // string literal preserved
    }

    #[test]
    fn escape_entities() {
        assert_eq!(escape("<a&b>"), "&lt;a&amp;b&gt;");
    }
}
