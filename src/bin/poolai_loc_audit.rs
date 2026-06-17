//! LOC ratio baseline audit (PH-S143, PH-S150 advisory, PH-S159 stretch, PH-S165 hold gate) per
//! [`docs/development/RUST_RATIO_STRATEGY_2026-06-13.md`].
//!
//! ```text
//! cargo run --bin poolai-loc-audit
//! cargo run --bin poolai-loc-audit -- --output docs/development/rust_ratio.json
//! cargo run --bin poolai-loc-audit -- --warn-below 0.93 --target 0.95 --stretch 0.96 --min-ratio 0.95 --advisory
//! cargo run --bin poolai-loc-audit -- --min-ratio 0.91
//! ```

use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const DEFAULT_OUTPUT: &str = "docs/development/rust_ratio.json";
const FORMAL_BAND_MIN: f64 = 0.90;
const FORMAL_BAND_MAX: f64 = 0.95;
const DEFAULT_WARN_BELOW: f64 = 0.93;
const DEFAULT_TARGET: f64 = 0.95;
const DEFAULT_STRETCH: f64 = 0.96;
const SPRINT: &str = "PH-S318";

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

#[derive(Debug, Clone, Copy)]
struct AuditConfig {
    warn_below: f64,
    target: f64,
    stretch: f64,
    /// When true, ratio below `warn_below` prints a warning but exits 0 (PH-S159 CI stretch advisory).
    advisory: bool,
    /// Optional hold/regression floor (PH-S165: 0.95 hold band top with `--advisory` in CI).
    min_ratio: Option<f64>,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            warn_below: DEFAULT_WARN_BELOW,
            target: DEFAULT_TARGET,
            stretch: DEFAULT_STRETCH,
            advisory: false,
            min_ratio: None,
        }
    }
}

#[derive(Debug, Clone)]
struct AuditCli {
    output: PathBuf,
    config: AuditConfig,
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
    formal_band_min: f64,
    formal_band_max: f64,
    warn_below: f64,
    target_ratio: f64,
    stretch_spirit: f64,
    advisory_mode: bool,
    min_ratio: Option<f64>,
    rust_loc: u64,
    non_rust_product_loc: u64,
    product_loc_total: u64,
    rust_ratio: f64,
    rust_ratio_pct: f64,
    in_formal_band: bool,
    below_warn_threshold: bool,
    below_target: bool,
    below_stretch_spirit: bool,
    meets_min_ratio: Option<bool>,
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

fn classify_product_path(path: &str) -> ProductCategory {
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

fn count_non_blank_lines(path: &Path) -> std::io::Result<u64> {
    let text = fs::read_to_string(path)?;
    Ok(text.lines().filter(|line| !line.trim().is_empty()).count() as u64)
}

fn parse_ratio_arg(name: &str, raw: &str) -> Result<f64, String> {
    let value: f64 = raw
        .parse()
        .map_err(|_| format!("{name}: invalid ratio `{raw}` (expected 0.0–1.0)"))?;
    if !(0.0..=1.0).contains(&value) {
        return Err(format!("{name}: ratio must be in 0.0–1.0, got {value}"));
    }
    Ok(value)
}

fn parse_cli() -> Result<AuditCli, String> {
    let mut output = repo_root().join(DEFAULT_OUTPUT);
    let mut config = AuditConfig::default();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" | "-o" => {
                let path = args.next().ok_or("--output requires a path".to_string())?;
                output = repo_root().join(path);
            }
            "--warn-below" => {
                config.warn_below = parse_ratio_arg(
                    "--warn-below",
                    &args
                        .next()
                        .ok_or("--warn-below requires a ratio".to_string())?,
                )?;
            }
            "--target" => {
                config.target = parse_ratio_arg(
                    "--target",
                    &args.next().ok_or("--target requires a ratio".to_string())?,
                )?;
            }
            "--stretch" => {
                config.stretch = parse_ratio_arg(
                    "--stretch",
                    &args
                        .next()
                        .ok_or("--stretch requires a ratio".to_string())?,
                )?;
            }
            "--min-ratio" => {
                config.min_ratio = Some(parse_ratio_arg(
                    "--min-ratio",
                    &args
                        .next()
                        .ok_or("--min-ratio requires a ratio".to_string())?,
                )?);
            }
            "--advisory" => config.advisory = true,
            "--strict" => config.advisory = false,
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument `{other}` (try --help)")),
        }
    }
    Ok(AuditCli { output, config })
}

fn print_help() {
    println!(
        "Usage: poolai-loc-audit [OPTIONS]\n\
         \n\
         Options:\n\
           -o, --output PATH     JSON report path (default: {DEFAULT_OUTPUT})\n\
           --warn-below RATIO    advisory floor (default {DEFAULT_WARN_BELOW})\n\
           --target RATIO        stretch-band target (default {DEFAULT_TARGET})\n\
           --stretch RATIO       spirit goal (default {DEFAULT_STRETCH})\n\
           --min-ratio RATIO     hold/regression floor (fail unless --advisory)\n\
           --advisory            warn below floors but exit 0 (PH-S165 CI hold gate)\n\
           --strict              fail when ratio < --warn-below (default without --advisory)\n\
           -h, --help            show help\n\
         \n\
         Formal product-code band: {FORMAL_BAND_MIN:.0}–{FORMAL_BAND_MAX:.0} (strategy §1)."
    );
}

fn build_report(
    root: &Path,
    files: &[String],
    config: AuditConfig,
) -> Result<RustRatioReport, String> {
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

    let meets_min_ratio = config
        .min_ratio
        .map(|floor| rust_ratio + f64::EPSILON >= floor);

    Ok(RustRatioReport {
        generated_at: chrono::Utc::now().format("%Y-%m-%d").to_string(),
        sprint: SPRINT,
        formal_band_min: FORMAL_BAND_MIN,
        formal_band_max: FORMAL_BAND_MAX,
        warn_below: config.warn_below,
        target_ratio: config.target,
        stretch_spirit: config.stretch,
        advisory_mode: config.advisory,
        min_ratio: config.min_ratio,
        rust_loc,
        non_rust_product_loc,
        product_loc_total,
        rust_ratio,
        rust_ratio_pct: rust_ratio * 100.0,
        in_formal_band: (FORMAL_BAND_MIN..=FORMAL_BAND_MAX).contains(&rust_ratio),
        below_warn_threshold: rust_ratio + f64::EPSILON < config.warn_below,
        below_target: rust_ratio + f64::EPSILON < config.target,
        below_stretch_spirit: rust_ratio + f64::EPSILON < config.stretch,
        meets_min_ratio,
        by_category,
        notes: vec![
            "Denominator: product code only (strategy §1); docs/yaml/png excluded",
            "GitHub Languages bar is heuristic; this report uses git-tracked LOC buckets",
            "PH-S165: CI --min-ratio 0.95 hold band (advisory); stretch spirit 96% via --stretch",
        ],
    })
}

fn gh_annotation(level: &str, title: &str, message: &str) {
    println!("::{level} title={title}::{message}");
}

fn print_summary(report: &RustRatioReport) {
    println!("PoolAI LOC ratio audit ({})", report.sprint);
    println!("  rust_loc:              {}", report.rust_loc);
    println!("  non_rust_product_loc:  {}", report.non_rust_product_loc);
    println!(
        "  rust_ratio:            {:.2}% (formal {:.0}–{:.0}%)",
        report.rust_ratio_pct,
        report.formal_band_min * 100.0,
        report.formal_band_max * 100.0
    );
    println!(
        "  thresholds:            warn {:.0}% · target {:.0}% · stretch {:.0}%",
        report.warn_below * 100.0,
        report.target_ratio * 100.0,
        report.stretch_spirit * 100.0
    );
    println!("  in_formal_band:        {}", report.in_formal_band);
    if report.advisory_mode {
        println!("  advisory_mode:         true (warn below floors, exit 0)");
    }
    if let Some(floor) = report.min_ratio {
        println!("  hold_floor (--min-ratio): {:.0}%", floor * 100.0);
    }
    if let Some(ok) = report.meets_min_ratio {
        println!("  meets_min_ratio:       {ok}");
    }
    for (name, loc) in &report.by_category {
        println!("  {name}: {} files, {} loc", loc.files, loc.loc);
    }
}

fn emit_threshold_messages(report: &RustRatioReport) {
    if report.below_warn_threshold {
        let msg = format!(
            "Rust product-code ratio {:.2}% is below advisory floor {:.0}%",
            report.rust_ratio_pct,
            report.warn_below * 100.0
        );
        eprintln!("warning: {msg}");
        gh_annotation("warning", "Rust ratio advisory floor", &msg);
    }
    if let Some(false) = report.meets_min_ratio {
        let msg = format!(
            "Rust product-code ratio {:.2}% is below hold floor {:.0}% (--min-ratio)",
            report.rust_ratio_pct,
            report.min_ratio.unwrap_or(0.0) * 100.0
        );
        if report.advisory_mode {
            eprintln!("warning: {msg}");
            gh_annotation("warning", "Rust ratio hold band", &msg);
        }
    }
    if report.below_target {
        eprintln!(
            "note: ratio {:.2}% is below PH-S165 hold target {:.0}% (formal band top; stretch spirit 96%)",
            report.rust_ratio_pct,
            report.target_ratio * 100.0
        );
    }
    if report.below_stretch_spirit {
        eprintln!(
            "note: ratio {:.2}% is below stretch spirit {:.0}% — migrate JS/TS/shell → Rust/wasm",
            report.rust_ratio_pct,
            report.stretch_spirit * 100.0
        );
    }
    if report.in_formal_band && !report.below_target {
        println!(
            "ok: ratio {:.2}% meets target {:.0}%",
            report.rust_ratio_pct,
            report.target_ratio * 100.0
        );
    }
}

fn exit_for_thresholds(report: &RustRatioReport) -> ExitCode {
    if let Some(false) = report.meets_min_ratio {
        let msg = format!(
            "rust ratio {:.2}% is below --min-ratio hold floor {:.0}%",
            report.rust_ratio_pct,
            report.min_ratio.unwrap_or(0.0) * 100.0
        );
        if report.advisory_mode {
            // Hold-band advisory: surfaced in emit_threshold_messages; exit 0 below.
        } else {
            eprintln!("error: {msg}");
            return ExitCode::from(1);
        }
    } else if report.meets_min_ratio == Some(true) {
        // Explicit --min-ratio gate met: pass even below warn_below.
        return ExitCode::SUCCESS;
    }
    if report.below_warn_threshold && !report.advisory_mode {
        eprintln!(
            "error: rust ratio {:.2}% is below --warn-below {:.0}% (--strict)",
            report.rust_ratio_pct,
            report.warn_below * 100.0
        );
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

fn main() -> ExitCode {
    let cli = match parse_cli() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };
    let root = repo_root();
    let files = match git_tracked_files(&root) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };
    let report = match build_report(&root, &files, cli.config) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };
    print_summary(&report);
    emit_threshold_messages(&report);
    if let Some(parent) = cli.output.parent() {
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
    if let Err(e) = fs::File::create(&cli.output).and_then(|mut f| f.write_all(json.as_bytes())) {
        eprintln!("error: write {}: {e}", cli.output.display());
        return ExitCode::from(2);
    }
    println!("wrote {}", cli.output.display());
    exit_for_thresholds(&report)
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
        let report = build_report(root, &files, AuditConfig::default()).expect("report");
        assert_eq!(report.rust_loc, 2);
        assert_eq!(report.non_rust_product_loc, 2);
        assert!((report.rust_ratio - 0.5).abs() < f64::EPSILON);
        assert!(report.below_warn_threshold);
        assert!(report.below_target);
        assert!(report.below_stretch_spirit);
    }

    #[test]
    fn min_ratio_gate() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        let files = vec!["src/a.rs".to_string(), "src/ui/x.js".to_string()];
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/a.rs"), "line\n").unwrap();
        fs::create_dir_all(root.join("src/ui")).unwrap();
        fs::write(root.join("src/ui/x.js"), "a\nb\nc\n").unwrap();
        let report = build_report(
            root,
            &files,
            AuditConfig {
                min_ratio: Some(0.91),
                ..AuditConfig::default()
            },
        )
        .expect("report");
        assert_eq!(report.meets_min_ratio, Some(false));
        assert_eq!(exit_for_thresholds(&report), ExitCode::from(1));
    }

    #[test]
    fn min_ratio_hold_advisory_exits_zero() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        let files = vec!["src/a.rs".to_string(), "src/ui/x.js".to_string()];
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/a.rs"), "line\n").unwrap();
        fs::create_dir_all(root.join("src/ui")).unwrap();
        fs::write(root.join("src/ui/x.js"), "a\nb\nc\n").unwrap();
        let report = build_report(
            root,
            &files,
            AuditConfig {
                min_ratio: Some(0.95),
                advisory: true,
                ..AuditConfig::default()
            },
        )
        .expect("report");
        assert_eq!(report.meets_min_ratio, Some(false));
        assert_eq!(exit_for_thresholds(&report), ExitCode::SUCCESS);
    }

    #[test]
    fn min_ratio_gate_passes_despite_warn_below() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        let files = vec!["src/a.rs".to_string(), "src/ui/x.js".to_string()];
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/a.rs"), "line\n".repeat(9)).unwrap();
        fs::create_dir_all(root.join("src/ui")).unwrap();
        fs::write(root.join("src/ui/x.js"), "a\n").unwrap();
        let report = build_report(
            root,
            &files,
            AuditConfig {
                min_ratio: Some(0.5),
                ..AuditConfig::default()
            },
        )
        .expect("report");
        assert_eq!(report.meets_min_ratio, Some(true));
        assert!(report.below_warn_threshold);
        assert_eq!(exit_for_thresholds(&report), ExitCode::SUCCESS);
    }

    #[test]
    fn parse_ratio_rejects_out_of_range() {
        assert!(parse_ratio_arg("--target", "1.5").is_err());
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
