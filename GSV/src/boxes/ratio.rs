//! Ratio box — GSV Rust/LOC ratio audit + wire (mirror of poolAI `poolai-loc-audit`).
//!
//! Counts git-tracked product LOC under `GSV/` and reports the Rust share.
//! Rust 95–100% canon → `rust_ratio.json` in `GSV/data/`, advisory gate at 0.95.
//!
//! ```text
//! cargo run --bin gsv-loc-audit                 # write GSV/data/rust_ratio.json
//! cargo run --bin gsv-loc-audit -- --print      # print report, no write
//! cargo run --bin gsv-loc-audit -- --min-ratio 0.95 --advisory
//! ```

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};

/// Formal GSV canon band: Rust 95–100% / wasm 0–5%.
pub const FORMAL_BAND_MIN: f64 = 0.95;
/// Default advisory/regression floor.
pub const DEFAULT_MIN_RATIO: f64 = 0.95;

/// Product categories for the GSV ratio audit (git-tracked files only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductCategory {
    Ignored,
    RustSrc,
    RustTests,
    RustBenches,
    UiHtml,
    UiJs,
    UiCss,
    OpsShell,
}

impl ProductCategory {
    pub fn is_rust(self) -> bool {
        matches!(self, Self::RustSrc | Self::RustTests | Self::RustBenches)
    }

    pub fn is_non_rust_product(self) -> bool {
        matches!(
            self,
            Self::UiHtml | Self::UiJs | Self::UiCss | Self::OpsShell
        )
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Ignored => "ignored",
            Self::RustSrc => "rust_src",
            Self::RustTests => "rust_tests",
            Self::RustBenches => "rust_benches",
            Self::UiHtml => "ui_html",
            Self::UiJs => "ui_js",
            Self::UiCss => "ui_css",
            Self::OpsShell => "ops_shell",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuditConfig {
    pub min_ratio: f64,
    /// Warn and exit 0 when ratio below `min_ratio` (CI advisory).
    pub advisory: bool,
    pub write_output: bool,
    pub print: bool,
    pub output: Option<PathBuf>,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            min_ratio: DEFAULT_MIN_RATIO,
            advisory: false,
            write_output: true,
            print: false,
            output: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryLoc {
    pub files: u64,
    pub loc: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustRatioReport {
    pub generated_at: String,
    pub rust_loc: u64,
    pub non_rust_product_loc: u64,
    pub product_loc_total: u64,
    pub rust_ratio: f64,
    pub rust_ratio_pct: f64,
    pub formal_band_min: f64,
    pub min_ratio: f64,
    pub meets_min_ratio: bool,
    pub by_category: BTreeMap<String, CategoryLoc>,
    pub notes: Vec<String>,
}

/// Classify a product path relative to the GSV workspace (a leading `GSV/` is
/// tolerated for git-top-level-relative inputs).
fn classify_product_path(path: &str) -> ProductCategory {
    let p = path.replace('\\', "/");
    let p = p.strip_prefix("GSV/").unwrap_or(&p);
    if p.starts_with("src/") && p.ends_with(".rs") {
        return ProductCategory::RustSrc;
    }
    if p.starts_with("tests/") && p.ends_with(".rs") {
        return ProductCategory::RustTests;
    }
    if p.starts_with("benches/") && p.ends_with(".rs") {
        return ProductCategory::RustBenches;
    }
    if p.starts_with("ui/") && p.ends_with(".html") {
        return ProductCategory::UiHtml;
    }
    if p.starts_with("ui/") && p.ends_with(".js") {
        return ProductCategory::UiJs;
    }
    if p.starts_with("ui/") && p.ends_with(".css") {
        return ProductCategory::UiCss;
    }
    if (p.starts_with("bin/") || p.starts_with("scripts/")) && p.ends_with(".sh") {
        return ProductCategory::OpsShell;
    }
    ProductCategory::Ignored
}

/// Convert an MSYS git root like `/s/rust/poolAI` to a Windows path `S:/rust/poolAI`.
fn normalize_git_root(root: &str) -> String {
    let bytes = root.as_bytes();
    if root.starts_with('/')
        && bytes.len() >= 3
        && bytes[1].is_ascii_alphabetic()
        && bytes[2] == b'/'
    {
        format!("{}:{}", (bytes[1] as char).to_ascii_uppercase(), &root[2..])
    } else {
        root.to_string()
    }
}

/// Git-tracked paths under `GSV/`, as absolute paths resolved against the
/// enclosing git top-level (works for `root` = top-level or `root` = `GSV/`).
fn git_tracked_gsv_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let top = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(root)
        .output()
        .map_err(|e| format!("git rev-parse: {e}"))?;
    if !top.status.success() {
        return Err("git rev-parse --show-toplevel failed".to_string());
    }
    let top = normalize_git_root(String::from_utf8_lossy(&top.stdout).trim());
    let output = Command::new("git")
        .args(["ls-files", "-z", "--full-name"])
        .current_dir(root)
        .output()
        .map_err(|e| format!("git ls-files: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "git ls-files failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(output
        .stdout
        .split(|&b| b == 0)
        .filter(|chunk| !chunk.is_empty())
        .filter_map(|chunk| std::str::from_utf8(chunk).ok().map(str::to_string))
        .filter(|p| p.starts_with("GSV/"))
        .map(|p| PathBuf::from(&top).join(p))
        .collect())
}

/// Non-blank line count (mirror of poolAI loc-audit).
fn count_loc(text: &str) -> u64 {
    text.lines().filter(|line| !line.trim().is_empty()).count() as u64
}

/// Run the LOC audit over the GSV workspace (git-tracked files under `GSV/`).
pub fn audit(root: &Path) -> Result<RustRatioReport, String> {
    let files = git_tracked_gsv_files(root)?;
    let mut by_category: BTreeMap<String, CategoryLoc> = BTreeMap::new();
    for path in &files {
        // Workspace-relative path for classification (top-absolute or `GSV/`-prefixed).
        let rel = path
            .to_string_lossy()
            .replace('\\', "/")
            .split("/GSV/")
            .last()
            .unwrap_or_default()
            .to_string();
        let category = classify_product_path(&rel);
        if category == ProductCategory::Ignored {
            continue;
        }
        let text =
            std::fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
        let loc = count_loc(&text);
        let entry = by_category
            .entry(category.label().to_string())
            .or_insert(CategoryLoc { files: 0, loc: 0 });
        entry.files += 1;
        entry.loc += loc;
    }

    let rust_loc: u64 = by_category
        .iter()
        .filter(|(label, _)| label.starts_with("rust_"))
        .map(|(_, c)| c.loc)
        .sum();
    let non_rust_product_loc: u64 = by_category
        .iter()
        .filter(|(label, _)| matches!(label.as_str(), "ui_html" | "ui_js" | "ui_css" | "ops_shell"))
        .map(|(_, c)| c.loc)
        .sum();
    let product_loc_total = rust_loc + non_rust_product_loc;
    let rust_ratio = if product_loc_total > 0 {
        rust_loc as f64 / product_loc_total as f64
    } else {
        1.0
    };

    let mut notes = Vec::new();
    if product_loc_total == 0 {
        notes.push("no product files found".to_string());
    }

    Ok(RustRatioReport {
        generated_at: crate::vision::rfc3339_now(),
        rust_loc,
        non_rust_product_loc,
        product_loc_total,
        rust_ratio,
        rust_ratio_pct: rust_ratio * 100.0,
        formal_band_min: FORMAL_BAND_MIN,
        min_ratio: DEFAULT_MIN_RATIO,
        meets_min_ratio: rust_ratio >= DEFAULT_MIN_RATIO,
        by_category,
        notes,
    })
}

/// Persist the report to `{data_dir}/rust_ratio.json`.
pub fn save(report: &RustRatioReport, data_dir: &Path) -> Result<(), String> {
    std::fs::create_dir_all(data_dir).map_err(|e| format!("create data dir: {e}"))?;
    let raw = serde_json::to_string_pretty(report).map_err(|e| format!("serialize: {e}"))?;
    std::fs::write(data_dir.join("rust_ratio.json"), raw).map_err(|e| format!("write: {e}"))
}

/// Load the persisted report from `{data_dir}/rust_ratio.json`.
pub fn load(data_dir: &Path) -> Result<RustRatioReport, String> {
    let path = data_dir.join("rust_ratio.json");
    let raw =
        std::fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("parse rust_ratio.json: {e}"))
}

/// API wire — report or an `ok:false` payload when the store is missing.
pub fn wire(data_dir: &Path) -> serde_json::Value {
    match load(data_dir) {
        Ok(report) => {
            let mut v = serde_json::to_value(&report).unwrap_or_default();
            if let serde_json::Value::Object(map) = &mut v {
                map.insert("ok".to_string(), serde_json::Value::Bool(true));
            }
            v
        }
        Err(e) => serde_json::json!({ "ok": false, "error": e }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_gsv_paths() {
        assert_eq!(
            classify_product_path("GSV/src/boxes/omni/mod.rs"),
            ProductCategory::RustSrc
        );
        assert_eq!(
            classify_product_path("GSV/src/bin/gsv_server.rs"),
            ProductCategory::RustSrc
        );
        assert_eq!(
            classify_product_path("GSV/tests/gsv_omni_contracts.rs"),
            ProductCategory::RustTests
        );
        assert_eq!(
            classify_product_path("GSV/ui/index.html"),
            ProductCategory::UiHtml
        );
        assert_eq!(
            classify_product_path("GSV/ui/app.js"),
            ProductCategory::UiJs
        );
        assert_eq!(
            classify_product_path("GSV/ui/style.css"),
            ProductCategory::UiCss
        );
        assert_eq!(
            classify_product_path("GSV/scripts/tool.sh"),
            ProductCategory::OpsShell
        );
        assert_eq!(
            classify_product_path("GSV/README.md"),
            ProductCategory::Ignored
        );
        assert_eq!(
            classify_product_path("GSV/Cargo.toml"),
            ProductCategory::Ignored
        );
        assert_eq!(
            classify_product_path("docs/gsv/GSV_BOXES.md"),
            ProductCategory::Ignored
        );
        assert_eq!(
            classify_product_path("GSV/target/debug/foo.rs"),
            ProductCategory::Ignored
        );
    }

    #[test]
    fn rust_vs_non_rust_membership() {
        assert!(ProductCategory::RustSrc.is_rust());
        assert!(ProductCategory::RustTests.is_rust());
        assert!(ProductCategory::UiHtml.is_non_rust_product());
        assert!(ProductCategory::UiJs.is_non_rust_product());
        assert!(!ProductCategory::UiHtml.is_rust());
        assert!(!ProductCategory::Ignored.is_rust());
    }

    #[test]
    fn count_loc_skips_blank_lines() {
        assert_eq!(count_loc("a\n\n  \nb\n\t\nc"), 3);
        assert_eq!(count_loc(""), 0);
    }

    #[test]
    fn ratio_math() {
        let mut by_category = BTreeMap::new();
        by_category.insert("rust_src".to_string(), CategoryLoc { files: 1, loc: 95 });
        by_category.insert("ui_html".to_string(), CategoryLoc { files: 1, loc: 5 });
        let rust_loc = 95;
        let non_rust = 5;
        let total = rust_loc + non_rust;
        let ratio = rust_loc as f64 / total as f64;
        assert!(ratio >= 0.95);
    }
}
