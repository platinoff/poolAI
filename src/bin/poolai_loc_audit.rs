//! LOC ratio baseline audit (PH-S143) per [`docs/development/RUST_RATIO_STRATEGY_2026-06-13.md`].
//!
//! ```text
//! cargo run --bin poolai-loc-audit
//! cargo run --bin poolai-loc-audit -- --output docs/development/rust_ratio.json
//! ```

use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const DEFAULT_OUTPUT: &str = "docs/development/rust_ratio.json";
const TARGET_MIN: f64 = 0.90;
const TARGET_MAX: f64 = 0.95;
const WARN_MIN: f64 = 0.88;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
enum ProductCategory {
    Ignored,
    RustSrc,
    RustTests,
    RustCrates,
    RustBenches,
    UiJs,
    UiCss,
    E2eTs,
    OpsShell,
    OpsPs1,
}

impl ProductCategory {
    fn is_rust(self) -> bool {
        matches!(
            self,
            Self::RustSrc | Self::RustTests | Self::RustCrates | Self::RustBenches
        )
    }

    fn is_non_rust_product(self) -> bool {
        matches!(
            self,
            Self::UiJs | Self::UiCss | Self::E2eTs | Self::OpsShell | Self::OpsPs1
        )
    }

    fn label(self) -> &'static str {
        match self {
            Self::Ignored => "ignored",
            Self::RustSrc => "rust_src",
            Self::RustTests => "rust_tests",
            Self::RustCrates => "rust_crates",
            Self::RustBenches => "rust_benches",
            Self::UiJs => "ui_js",
            Self::UiCss => "ui_css",
            Self::E2eTs => "e2e_ts",
            Self::OpsShell => "ops_shell",
            Self::OpsPs1 => "ops_ps1",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct CategoryLoc {
    files: u64,
    loc: u64,
}

#[derive(Debug, Clone, Serialize)]
struct RustRatioReport {
    generated_at: String,
    sprint: &'static str,
    target_band_min: f64,
    target_band_max: f64,
    warn_min: f64,
    rust_loc: u64,
    non_rust_product_loc: u64,
    product_loc_total: u64,
    rust_ratio: f64,
    rust_ratio_pct: f64,
    in_target_band: bool,
    below_warn_threshold: bool,
    by_category: BTreeMap<String, CategoryLoc>,
    notes: Vec<&'static str>,
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn git_tracked_files(root: &Path) -> Result<Vec<String>, String> {
    let output = Command::new("git")
        .args(["ls-files", "-z"])
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
        .collect())
}

/// Classify git-tracked path into product-code bucket (strategy §1).
pub fn classify_product_path(path: &str) -> ProductCategory {
    let p = path.replace('\\', "/");
    if p.starts_with("src/") && p.ends_with(".rs") {
        return ProductCategory::RustSrc;
    }
    if p.starts_with("tests/") && p.ends_with(".rs") {
        return ProductCategory::RustTests;
    }
    if p.starts_with("crates/") && p.ends_with(".rs") {
        return ProductCategory::RustCrates;
    }
    if p.starts_with("benches/") && p.ends_with(".rs") {
        return ProductCategory::RustBenches;
    }
    if p.starts_with("src/ui/") && p.ends_with(".js") {
        return ProductCategory::UiJs;
    }
    if p.starts_with("src/ui/") && p.ends_with(".css") {
        return ProductCategory::UiCss;
    }
    if p.starts_with("e2e/archive/") {
        return ProductCategory::Ignored;
    }
    if p.starts_with("e2e/") && (p.ends_with(".ts") || p.ends_with(".tsx")) {
        return ProductCategory::E2eTs;
    }
    if (p.starts_with("bin/") || p.starts_with("scripts/")) && p.ends_with(".sh") {
        return ProductCategory::OpsShell;
    }
    if (p.starts_with("bin/") || p.starts_with("scripts/")) && p.ends_with(".ps1") {
        return ProductCategory::OpsPs1;
    }
    ProductCategory::Ignored
}

pub fn count_non_blank_lines(path: &Path) -> std::io::Result<u64> {
    let text = fs::read_to_string(path)?;
    Ok(text.lines().filter(|line| !line.trim().is_empty()).count() as u64)
}

pub fn build_report(root: &Path, files: &[String]) -> Result<RustRatioReport, String> {
    let mut by_cat: BTreeMap<ProductCategory, CategoryLoc> = BTreeMap::new();

    for rel in files {
        let category = classify_product_path(rel);
        if category == ProductCategory::Ignored {
            continue;
        }
        let abs = root.join(rel);
        if !abs.is_file() {
            continue;
        }
        let loc = count_non_blank_lines(&abs).map_err(|e| format!("{rel}: {e}"))?;
        let entry = by_cat
            .entry(category)
            .or_insert(CategoryLoc { files: 0, loc: 0 });
        entry.files += 1;
        entry.loc += loc;
    }

    let rust_loc: u64 = by_cat
        .iter()
        .filter(|(c, _)| c.is_rust())
        .map(|(_, v)| v.loc)
        .sum();
    let non_rust_product_loc: u64 = by_cat
        .iter()
        .filter(|(c, _)| c.is_non_rust_product())
        .map(|(_, v)| v.loc)
        .sum();
    let product_loc_total = rust_loc + non_rust_product_loc;
    let rust_ratio = if product_loc_total == 0 {
        0.0
    } else {
        rust_loc as f64 / product_loc_total as f64
    };

    let by_category = by_cat
        .into_iter()
        .map(|(cat, loc)| (cat.label().to_string(), loc))
        .collect();

    Ok(RustRatioReport {
        generated_at: chrono::Utc::now().format("%Y-%m-%d").to_string(),
        sprint: "PH-S143",
        target_band_min: TARGET_MIN,
        target_band_max: TARGET_MAX,
        warn_min: WARN_MIN,
        rust_loc,
        non_rust_product_loc,
        product_loc_total,
        rust_ratio,
        rust_ratio_pct: rust_ratio * 100.0,
        in_target_band: (TARGET_MIN..=TARGET_MAX).contains(&rust_ratio),
        below_warn_threshold: rust_ratio < WARN_MIN,
        by_category,
        notes: vec![
            "Denominator: product code only (strategy §1); docs/yaml/png excluded",
            "GitHub Languages bar is heuristic; this report uses git-tracked LOC buckets",
        ],
    })
}

fn parse_args() -> PathBuf {
    let mut args = std::env::args().skip(1);
    let mut output = repo_root().join(DEFAULT_OUTPUT);
    while let Some(arg) = args.next() {
        if arg == "--output" || arg == "-o" {
            if let Some(path) = args.next() {
                output = repo_root().join(path);
            }
        }
    }
    output
}

fn print_summary(report: &RustRatioReport) {
    println!("PoolAI LOC ratio baseline (PH-S143)");
    println!("  rust_loc:              {}", report.rust_loc);
    println!("  non_rust_product_loc:  {}", report.non_rust_product_loc);
    println!(
        "  rust_ratio:            {:.2}% (target {:.0}–{:.0}%)",
        report.rust_ratio_pct,
        report.target_band_min * 100.0,
        report.target_band_max * 100.0
    );
    println!("  in_target_band:        {}", report.in_target_band);
    for (name, loc) in &report.by_category {
        println!("  {name}: {} files, {} loc", loc.files, loc.loc);
    }
}

fn main() -> ExitCode {
    let root = repo_root();
    let output = parse_args();
    let files = match git_tracked_files(&root) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };
    let report = match build_report(&root, &files) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };
    print_summary(&report);
    if let Some(parent) = output.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("error: create output dir: {e}");
            return ExitCode::from(2);
        }
    }
    let json = match serde_json::to_string_pretty(&report) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("error: serialize: {e}");
            return ExitCode::from(2);
        }
    };
    match fs::File::create(&output).and_then(|mut f| f.write_all(json.as_bytes())) {
        Ok(()) => {
            println!("wrote {}", output.display());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: write {}: {e}", output.display());
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn classify_product_paths() {
        assert_eq!(
            classify_product_path("src/grid/dispatch.rs"),
            ProductCategory::RustSrc
        );
        assert_eq!(
            classify_product_path("tests/jobs_api_contracts.rs"),
            ProductCategory::RustTests
        );
        assert_eq!(
            classify_product_path("crates/foo/src/lib.rs"),
            ProductCategory::RustCrates
        );
        assert_eq!(
            classify_product_path("src/ui/i18n_core.js"),
            ProductCategory::UiJs
        );
        assert_eq!(
            classify_product_path("e2e/tests/admin.spec.ts"),
            ProductCategory::E2eTs
        );
        assert_eq!(
            classify_product_path("bin/e2e-playwright.sh"),
            ProductCategory::OpsShell
        );
        assert_eq!(
            classify_product_path("docs/README.md"),
            ProductCategory::Ignored
        );
    }

    #[test]
    fn build_report_computes_ratio() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        let files = vec![
            "src/a.rs".to_string(),
            "src/ui/x.js".to_string(),
            "e2e/t.spec.ts".to_string(),
        ];
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/a.rs"), "fn main() {\n}\n\n").unwrap();
        fs::create_dir_all(root.join("src/ui")).unwrap();
        fs::write(root.join("src/ui/x.js"), "console.log(1);\n").unwrap();
        fs::create_dir_all(root.join("e2e")).unwrap();
        fs::write(root.join("e2e/t.spec.ts"), "test();\n\n").unwrap();
        let report = build_report(root, &files).expect("report");
        assert_eq!(report.rust_loc, 2);
        assert_eq!(report.non_rust_product_loc, 2);
        assert!((report.rust_ratio - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn count_non_blank_lines_skips_empty() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("f.rs");
        let mut f = fs::File::create(&path).unwrap();
        writeln!(f, "line1").unwrap();
        writeln!(f).unwrap();
        writeln!(f, "  ").unwrap();
        writeln!(f, "line2").unwrap();
        assert_eq!(count_non_blank_lines(&path).unwrap(), 2);
    }
}
