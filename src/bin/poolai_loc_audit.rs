//! LOC ratio baseline audit (PH-S143, PH-S150 advisory, PH-S159 stretch, PH-S165 hold gate) per
//! [`docs/development/RUST_RATIO_STRATEGY_2026-06-13.md`].
//!
//! ```text
//! cargo run --bin poolai-loc-audit
//! cargo run --bin poolai-loc-audit -- --output docs/development/rust_ratio.json
//! cargo run --bin poolai-loc-audit -- --warn-below 0.93 --target 0.95 --stretch 0.96 --min-ratio 0.95 --advisory
//! cargo run --bin poolai-loc-audit -- --min-ratio 0.91
//! ```

use poolai_ui_core::ci_canon_depth::{ci_canon_criteria_total, CI_CANON_CASES, CI_CANON_CRITERIA};
use poolai_ui_core::galaxy_edge_verification_depth::{
    edge_verification_criteria_total, EDGE_VERIFICATION_CASES, EDGE_VERIFICATION_CRITERIA,
};
use poolai_ui_core::pre_push_hook_depth::{
    pre_push_hook_criteria_total, PRE_PUSH_HOOK_CASES, PRE_PUSH_HOOK_CRITERIA,
};
use poolai_ui_core::rust_migration_advisory_depth::{
    migration_registry_total, ADMIN_JS_MIGRATION_CANDIDATES, ARCHIVED_E2E_MIGRATION_CANON,
    MIGRATION_ADVISORY_CASES,
};
use poolai_ui_core::sso_admin_ops_depth::{
    sso_admin_ops_criteria_total, SSO_ADMIN_OPS_CASES, SSO_ADMIN_OPS_CRITERIA,
};
use poolai_ui_core::sso_api_contracts_depth::{
    sso_api_criteria_total, SSO_API_CASES, SSO_API_CRITERIA,
};
use poolai_ui_core::sso_depth::{sso_criteria_total, SSO_CASES, SSO_CRITERIA};
use poolai_ui_core::sso_docs_canon_depth::{
    sso_docs_canon_criteria_total, SSO_DOCS_CANON_CASES, SSO_DOCS_CANON_CRITERIA,
};
use poolai_ui_core::sso_loc_audit_depth::{
    sso_loc_audit_criteria_total, SSO_LOC_AUDIT_CASES, SSO_LOC_AUDIT_CRITERIA,
};
use poolai_ui_core::sso_ratio_advisory_depth::{
    sso_ratio_advisory_criteria_total, SSO_RATIO_ADVISORY_CASES, SSO_RATIO_ADVISORY_CRITERIA,
};
use poolai_ui_core::sso_stand_smoke_depth::{
    sso_stand_smoke_criteria_total, SSO_STAND_SMOKE_CASES, SSO_STAND_SMOKE_CRITERIA,
};
use poolai_ui_core::sso_store_depth::{
    sso_store_criteria_total, SSO_STORE_CASES, SSO_STORE_CRITERIA,
};
use poolai_ui_core::sso_vision_sync_depth::{
    sso_vision_sync_criteria_total, SSO_VISION_SYNC_CASES, SSO_VISION_SYNC_CRITERIA,
};
use poolai_ui_core::stable_state_touchup_depth::{
    stable_criteria_total, STABLE_TOUCHUP_CASES, STABLE_TOUCHUP_CRITERIA,
};
use poolai_ui_core::tenant_admin_ops_depth::{
    tenant_admin_ops_criteria_total, TENANT_ADMIN_OPS_CASES, TENANT_ADMIN_OPS_CRITERIA,
};
use poolai_ui_core::tenant_api_contracts_depth::{
    tenant_api_criteria_total, TENANT_API_CASES, TENANT_API_CRITERIA,
};
use poolai_ui_core::tenant_depth::{tenant_criteria_total, TENANT_CASES, TENANT_CRITERIA};
use poolai_ui_core::tenant_docs_canon_depth::{
    tenant_docs_canon_criteria_total, TENANT_DOCS_CANON_CASES, TENANT_DOCS_CANON_CRITERIA,
};
use poolai_ui_core::tenant_horizon_depth::{
    tenant_horizon_criteria_total, TENANT_HORIZON_CASES, TENANT_HORIZON_CRITERIA,
};
use poolai_ui_core::tenant_loc_audit_depth::{
    tenant_loc_audit_criteria_total, TENANT_LOC_AUDIT_CASES, TENANT_LOC_AUDIT_CRITERIA,
};
use poolai_ui_core::tenant_persistence_depth::{
    tenant_persist_criteria_total, TENANT_PERSIST_CASES, TENANT_PERSIST_CRITERIA,
};
use poolai_ui_core::tenant_ratio_advisory_depth::{
    tenant_ratio_advisory_criteria_total, TENANT_RATIO_ADVISORY_CASES,
    TENANT_RATIO_ADVISORY_CRITERIA,
};
use poolai_ui_core::tenant_stand_smoke_depth::{
    tenant_stand_smoke_criteria_total, TENANT_STAND_SMOKE_CASES, TENANT_STAND_SMOKE_CRITERIA,
};
use poolai_ui_core::tenant_vision_sync_depth::{
    tenant_vision_sync_criteria_total, TENANT_VISION_SYNC_CASES, TENANT_VISION_SYNC_CRITERIA,
};
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
const SPRINT: &str = "PH-S1010";
/// ui_js LOC at PH-S925 zriz (band 28 baseline for PH-S934 reduction metric).
const UI_JS_BAND28_BASELINE_LOC: u64 = 2141;
/// e2e_ts LOC at PH-S940 zriz (band 29 baseline for PH-S941 reduction metric).
const E2E_TS_BAND29_BASELINE_LOC: u64 = 1155;
const RATIO_95_FORMAL_GATE: f64 = 0.95;
const STRETCH_SPIRIT_GATE: f64 = 0.96;

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
    /// Emit band-46 migration advisory fields (PH-S1100).
    migration_advisory: bool,
    /// Emit band-47 STABLE touch-up fields (PH-S1110).
    stable_touchup: bool,
    /// Emit band-48 edge verification advisory fields (PH-S1120).
    edge_verification_advisory: bool,
    /// Emit band-49 pre-push canon gate fields (PH-S1130).
    pre_push_canon: bool,
    /// Emit band-50 CI canon gate fields (PH-S1140).
    ci_canon: bool,
    /// Emit band-51 tenant persist fields (PH-S1150).
    tenant_persist: bool,
    /// Emit band-52 tenant store-wire fields (PH-S1164).
    tenant_store: bool,
    /// Emit band-53 tenant HTTP API contracts fields (PH-S1176).
    tenant_api: bool,
    /// Emit band-54 tenant admin/ops fields (PH-S1185).
    tenant_admin_ops: bool,
    /// Emit band-55 tenant stand-smoke fields (PH-S1194).
    tenant_stand_smoke: bool,
    /// Emit band-56 tenant loc-audit aggregate fields (PH-S1204).
    tenant_loc_audit: bool,
    /// Emit band-57 tenant docs-canon fields (PH-S1214).
    tenant_docs_canon: bool,
    /// Emit band-58 tenant vision-sync fields (PH-S1224).
    tenant_vision_sync: bool,
    /// Emit band-59 tenant ratio-advisory fields (PH-S1234).
    tenant_ratio_advisory: bool,
    /// Emit band-60 tenant horizon-close fields (PH-S1244).
    tenant_horizon: bool,
    /// Emit band-61 SSO depth fields (PH-S1254).
    sso: bool,
    /// Emit band-62 SSO store-wire fields (PH-S1264).
    sso_store: bool,
    /// Emit band-63 SSO HTTP API contracts fields (PH-S1276).
    sso_api: bool,
    /// Emit band-64 SSO admin/ops fields (PH-S1285).
    sso_admin_ops: bool,
    /// Emit band-65 SSO stand-smoke fields (PH-S1294).
    sso_stand_smoke: bool,
    /// Emit band-66 SSO loc-audit aggregate fields (PH-S1304).
    sso_loc_audit: bool,
    /// Emit band-67 SSO docs-canon fields (PH-S1314).
    sso_docs_canon: bool,
    /// Emit band-68 SSO vision-sync fields (PH-S1324).
    sso_vision_sync: bool,
    /// Emit band-69 SSO ratio-advisory fields (PH-S1334).
    sso_ratio_advisory: bool,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            warn_below: DEFAULT_WARN_BELOW,
            target: DEFAULT_TARGET,
            stretch: DEFAULT_STRETCH,
            advisory: false,
            min_ratio: None,
            migration_advisory: false,
            stable_touchup: false,
            edge_verification_advisory: false,
            pre_push_canon: false,
            ci_canon: false,
            tenant_persist: false,
            tenant_store: false,
            tenant_api: false,
            tenant_admin_ops: false,
            tenant_stand_smoke: false,
            tenant_loc_audit: false,
            tenant_docs_canon: false,
            tenant_vision_sync: false,
            tenant_ratio_advisory: false,
            tenant_horizon: false,
            sso: false,
            sso_store: false,
            sso_api: false,
            sso_admin_ops: false,
            sso_stand_smoke: false,
            sso_loc_audit: false,
            sso_docs_canon: false,
            sso_vision_sync: false,
            sso_ratio_advisory: false,
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
    /// Current `ui_js` bucket LOC (PH-S934).
    ui_js_loc: u64,
    /// Band-28 baseline `ui_js` LOC before admin_common slim (PH-S925 zriz).
    ui_js_band28_baseline_loc: u64,
    /// Positive when `ui_js` LOC decreased vs band-28 baseline (PH-S934).
    ui_js_loc_reduction: i64,
    /// True when rust_ratio ≥ 0.95 formal gate (PH-S933); advisory hold when false.
    ratio_95_formal_gate_met: bool,
    /// True when rust_ratio ≥ 0.96 stretch spirit gate (PH-S942).
    stretch_spirit_gate_met: bool,
    /// Current `e2e_ts` bucket LOC (PH-S941).
    e2e_ts_loc: u64,
    /// Band-29 baseline `e2e_ts` LOC before API spec archive (PH-S940 zriz).
    e2e_ts_band29_baseline_loc: u64,
    /// Positive when `e2e_ts` LOC decreased vs band-29 baseline (PH-S941).
    e2e_ts_loc_reduction: i64,
    /// True when no `.rs` under `bin/` or `scripts/` (PH-S943 REPOSITORY_LAYOUT canon).
    ops_shell_canon_met: bool,
    /// Band-46 migration advisory mode (PH-S1100).
    migration_advisory_mode: bool,
    /// Total ui_js + archived e2e migration registry entries (PH-S1100).
    migration_candidate_total: usize,
    /// Admin JS glue files pending wasm migration (PH-S1102).
    migration_ui_js_candidate_count: usize,
    /// Archived Playwright API specs with Rust wire canon (PH-S1103).
    migration_e2e_archived_count: usize,
    /// Band-47 STABLE touch-up mode (PH-S1110).
    stable_touchup_mode: bool,
    /// STABLE maintenance criteria registry size (PH-S1112).
    stable_criteria_total: usize,
    /// Criteria with marker present in canonical doc path (PH-S1112).
    stable_criteria_met_count: usize,
    /// Band-48 edge verification advisory mode (PH-S1120).
    edge_verification_advisory_mode: bool,
    /// Edge verification criteria registry size (PH-S1121).
    edge_verification_criteria_total: usize,
    /// Edge verification criteria met count (PH-S1121).
    edge_verification_criteria_met_count: usize,
    /// Band-49 pre-push canon gate mode (PH-S1130).
    pre_push_canon_mode: bool,
    /// Pre-push canon criteria registry size (PH-S1131).
    pre_push_criteria_total: usize,
    /// Pre-push canon criteria met count (PH-S1131).
    pre_push_criteria_met_count: usize,
    /// Band-50 CI canon gate mode (PH-S1140).
    ci_canon_mode: bool,
    /// CI canon criteria registry size (PH-S1141).
    ci_canon_criteria_total: usize,
    /// CI canon criteria met count (PH-S1141).
    ci_canon_criteria_met_count: usize,
    /// Band-51 tenant persist mode (PH-S1150).
    tenant_persist_mode: bool,
    /// Tenant persist criteria registry size (PH-S1151).
    tenant_persist_criteria_total: usize,
    /// Tenant persist criteria met count (PH-S1151).
    tenant_persist_criteria_met_count: usize,
    /// Band-52 tenant store-wire mode (PH-S1164).
    tenant_store_mode: bool,
    /// Tenant store-wire criteria registry size (PH-S1164).
    tenant_store_criteria_total: usize,
    /// Tenant store-wire criteria met count (PH-S1164).
    tenant_store_criteria_met_count: usize,
    /// Band-53 tenant HTTP API contracts mode (PH-S1176).
    tenant_api_mode: bool,
    /// Tenant HTTP API criteria registry size (PH-S1176).
    tenant_api_criteria_total: usize,
    /// Tenant HTTP API criteria met count (PH-S1176).
    tenant_api_criteria_met_count: usize,
    /// Band-54 tenant admin/ops mode (PH-S1185).
    tenant_admin_ops_mode: bool,
    /// Tenant admin/ops criteria registry size (PH-S1185).
    tenant_admin_ops_criteria_total: usize,
    /// Tenant admin/ops criteria met count (PH-S1185).
    tenant_admin_ops_criteria_met_count: usize,
    /// Band-55 tenant stand-smoke mode (PH-S1194).
    tenant_stand_smoke_mode: bool,
    /// Tenant stand-smoke criteria registry size (PH-S1194).
    tenant_stand_smoke_criteria_total: usize,
    /// Tenant stand-smoke criteria met count (PH-S1194).
    tenant_stand_smoke_criteria_met_count: usize,
    /// Band-56 tenant loc-audit aggregate mode (PH-S1204).
    tenant_loc_audit_mode: bool,
    /// Tenant loc-audit criteria registry size (PH-S1204).
    tenant_loc_audit_criteria_total: usize,
    /// Tenant loc-audit criteria met count (PH-S1204).
    tenant_loc_audit_criteria_met_count: usize,
    /// Band-57 tenant docs-canon mode (PH-S1214).
    tenant_docs_canon_mode: bool,
    /// Tenant docs-canon criteria registry size (PH-S1214).
    tenant_docs_canon_criteria_total: usize,
    /// Tenant docs-canon criteria met count (PH-S1214).
    tenant_docs_canon_criteria_met_count: usize,
    /// Band-58 tenant vision-sync mode (PH-S1224).
    tenant_vision_sync_mode: bool,
    /// Tenant vision-sync criteria registry size (PH-S1224).
    tenant_vision_sync_criteria_total: usize,
    /// Tenant vision-sync criteria met count (PH-S1224).
    tenant_vision_sync_criteria_met_count: usize,
    /// Band-59 tenant ratio-advisory mode (PH-S1234).
    tenant_ratio_advisory_mode: bool,
    /// Tenant ratio-advisory criteria registry size (PH-S1234).
    tenant_ratio_advisory_criteria_total: usize,
    /// Tenant ratio-advisory criteria met count (PH-S1234).
    tenant_ratio_advisory_criteria_met_count: usize,
    /// Band-60 tenant horizon-close mode (PH-S1244).
    tenant_horizon_mode: bool,
    /// Tenant horizon criteria registry size (PH-S1244).
    tenant_horizon_criteria_total: usize,
    /// Tenant horizon criteria met count (PH-S1244).
    tenant_horizon_criteria_met_count: usize,
    /// Band-61 SSO depth mode (PH-S1254).
    sso_mode: bool,
    /// SSO criteria registry size (PH-S1254).
    sso_criteria_total: usize,
    /// SSO criteria met count (PH-S1254).
    sso_criteria_met_count: usize,
    /// Band-62 SSO store-wire mode (PH-S1264).
    sso_store_mode: bool,
    /// SSO store-wire criteria registry size (PH-S1264).
    sso_store_criteria_total: usize,
    /// SSO store-wire criteria met count (PH-S1264).
    sso_store_criteria_met_count: usize,
    /// Band-63 SSO HTTP API contracts mode (PH-S1276).
    sso_api_mode: bool,
    /// SSO HTTP API criteria registry size (PH-S1276).
    sso_api_criteria_total: usize,
    /// SSO HTTP API criteria met count (PH-S1276).
    sso_api_criteria_met_count: usize,
    /// Band-64 SSO admin/ops mode (PH-S1285).
    sso_admin_ops_mode: bool,
    /// SSO admin/ops criteria registry size (PH-S1285).
    sso_admin_ops_criteria_total: usize,
    /// SSO admin/ops criteria met count (PH-S1285).
    sso_admin_ops_criteria_met_count: usize,
    /// Band-65 SSO stand-smoke mode (PH-S1294).
    sso_stand_smoke_mode: bool,
    /// SSO stand-smoke criteria registry size (PH-S1294).
    sso_stand_smoke_criteria_total: usize,
    /// SSO stand-smoke criteria met count (PH-S1294).
    sso_stand_smoke_criteria_met_count: usize,
    /// Band-66 SSO loc-audit aggregate mode (PH-S1304).
    sso_loc_audit_mode: bool,
    /// SSO loc-audit criteria registry size (PH-S1304).
    sso_loc_audit_criteria_total: usize,
    /// SSO loc-audit criteria met count (PH-S1304).
    sso_loc_audit_criteria_met_count: usize,
    /// Band-67 SSO docs-canon mode (PH-S1314).
    sso_docs_canon_mode: bool,
    /// SSO docs-canon criteria registry size (PH-S1314).
    sso_docs_canon_criteria_total: usize,
    /// SSO docs-canon criteria met count (PH-S1314).
    sso_docs_canon_criteria_met_count: usize,
    /// Band-68 SSO vision-sync mode (PH-S1324).
    sso_vision_sync_mode: bool,
    /// SSO vision-sync criteria registry size (PH-S1324).
    sso_vision_sync_criteria_total: usize,
    /// SSO vision-sync criteria met count (PH-S1324).
    sso_vision_sync_criteria_met_count: usize,
    /// Band-69 SSO ratio-advisory mode (PH-S1334).
    sso_ratio_advisory_mode: bool,
    /// SSO ratio-advisory criteria registry size (PH-S1334).
    sso_ratio_advisory_criteria_total: usize,
    /// SSO ratio-advisory criteria met count (PH-S1334).
    sso_ratio_advisory_criteria_met_count: usize,
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

/// REPOSITORY_LAYOUT canon: no Rust sources under `bin/` or `scripts/` (PH-S943).
fn audit_ops_shell_canon(files: &[String]) -> bool {
    !files.iter().any(|path| {
        let p = path.replace('\\', "/");
        (p.starts_with("bin/") || p.starts_with("scripts/")) && p.ends_with(".rs")
    })
}

fn audit_stable_touchup_criteria_met(root: &Path) -> (usize, usize) {
    let total = stable_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in STABLE_TOUCHUP_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_edge_verification_criteria_met(root: &Path) -> (usize, usize) {
    let total = edge_verification_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in EDGE_VERIFICATION_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_pre_push_criteria_met(root: &Path) -> (usize, usize) {
    let total = pre_push_hook_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in PRE_PUSH_HOOK_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_ci_canon_criteria_met(root: &Path) -> (usize, usize) {
    let total = ci_canon_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in CI_CANON_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_tenant_persist_criteria_met(root: &Path) -> (usize, usize) {
    let total = tenant_persist_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in TENANT_PERSIST_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_tenant_store_criteria_met(root: &Path) -> (usize, usize) {
    let total = tenant_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in TENANT_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_tenant_api_criteria_met(root: &Path) -> (usize, usize) {
    let total = tenant_api_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in TENANT_API_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_tenant_admin_ops_criteria_met(root: &Path) -> (usize, usize) {
    let total = tenant_admin_ops_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in TENANT_ADMIN_OPS_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_tenant_stand_smoke_criteria_met(root: &Path) -> (usize, usize) {
    let total = tenant_stand_smoke_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in TENANT_STAND_SMOKE_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_tenant_loc_audit_criteria_met(root: &Path) -> (usize, usize) {
    let total = tenant_loc_audit_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in TENANT_LOC_AUDIT_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_tenant_docs_canon_criteria_met(root: &Path) -> (usize, usize) {
    let total = tenant_docs_canon_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in TENANT_DOCS_CANON_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_tenant_vision_sync_criteria_met(root: &Path) -> (usize, usize) {
    let total = tenant_vision_sync_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in TENANT_VISION_SYNC_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_tenant_ratio_advisory_criteria_met(root: &Path) -> (usize, usize) {
    let total = tenant_ratio_advisory_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in TENANT_RATIO_ADVISORY_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_tenant_horizon_criteria_met(root: &Path) -> (usize, usize) {
    let total = tenant_horizon_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in TENANT_HORIZON_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_sso_criteria_met(root: &Path) -> (usize, usize) {
    let total = sso_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in SSO_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_sso_store_criteria_met(root: &Path) -> (usize, usize) {
    let total = sso_store_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in SSO_STORE_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_sso_api_criteria_met(root: &Path) -> (usize, usize) {
    let total = sso_api_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in SSO_API_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_sso_admin_ops_criteria_met(root: &Path) -> (usize, usize) {
    let total = sso_admin_ops_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in SSO_ADMIN_OPS_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_sso_stand_smoke_criteria_met(root: &Path) -> (usize, usize) {
    let total = sso_stand_smoke_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in SSO_STAND_SMOKE_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_sso_loc_audit_criteria_met(root: &Path) -> (usize, usize) {
    let total = sso_loc_audit_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in SSO_LOC_AUDIT_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_sso_docs_canon_criteria_met(root: &Path) -> (usize, usize) {
    let total = sso_docs_canon_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in SSO_DOCS_CANON_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_sso_vision_sync_criteria_met(root: &Path) -> (usize, usize) {
    let total = sso_vision_sync_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in SSO_VISION_SYNC_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_sso_ratio_advisory_criteria_met(root: &Path) -> (usize, usize) {
    let total = sso_ratio_advisory_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in SSO_RATIO_ADVISORY_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
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
            "--migration-advisory" => config.migration_advisory = true,
            "--stable-touchup" => config.stable_touchup = true,
            "--edge-verification-advisory" => config.edge_verification_advisory = true,
            "--pre-push-canon" => config.pre_push_canon = true,
            "--ci-canon" => config.ci_canon = true,
            "--tenant-persist" => config.tenant_persist = true,
            "--tenant-store" => config.tenant_store = true,
            "--tenant-api" => config.tenant_api = true,
            "--tenant-admin-ops" => config.tenant_admin_ops = true,
            "--tenant-stand-smoke" => config.tenant_stand_smoke = true,
            "--tenant-loc-audit" => config.tenant_loc_audit = true,
            "--tenant-docs-canon" => config.tenant_docs_canon = true,
            "--tenant-vision-sync" => config.tenant_vision_sync = true,
            "--tenant-ratio-advisory" => config.tenant_ratio_advisory = true,
            "--tenant-horizon" => config.tenant_horizon = true,
            "--sso" => config.sso = true,
            "--sso-store" => config.sso_store = true,
            "--sso-api" => config.sso_api = true,
            "--sso-admin-ops" => config.sso_admin_ops = true,
            "--sso-stand-smoke" => config.sso_stand_smoke = true,
            "--sso-loc-audit" => config.sso_loc_audit = true,
            "--sso-docs-canon" => config.sso_docs_canon = true,
            "--sso-vision-sync" => config.sso_vision_sync = true,
            "--sso-ratio-advisory" => config.sso_ratio_advisory = true,
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
           --migration-advisory  band-46 rust migration advisory fields (PH-S1100)\n\
           --stable-touchup      band-47 STABLE touch-up criteria fields (PH-S1110)\n\
           --edge-verification-advisory  band-48 edge verification horizon fields (PH-S1120)\n\
           --pre-push-canon            band-49 pre-push vision canon gate fields (PH-S1130)\n\
           --ci-canon                  band-50 CI canon gate fields (PH-S1140)\n\
           --tenant-persist            band-51 tenant persistence fields (PH-S1150)\n\
           --tenant-store              band-52 tenant store-wire fields (PH-S1164)\n\
           --tenant-api                band-53 tenant HTTP API contracts fields (PH-S1176)\n\
           --tenant-admin-ops          band-54 tenant admin/ops fields (PH-S1185)\n\
           --tenant-stand-smoke        band-55 tenant stand-smoke fields (PH-S1194)\n\
           --tenant-loc-audit          band-56 tenant loc-audit aggregate fields (PH-S1204)\n\
           --tenant-docs-canon         band-57 tenant docs-canon fields (PH-S1214)\n\
           --tenant-vision-sync        band-58 tenant vision-sync fields (PH-S1224)\n\
           --tenant-ratio-advisory     band-59 tenant ratio-advisory fields (PH-S1234)\n\
           --tenant-horizon            band-60 tenant horizon-close fields (PH-S1244)\n\
           --sso                       band-61 SSO depth scaffold fields (PH-S1254)\n\
           --sso-store                 band-62 SSO store-wire fields (PH-S1264)\n\
           --sso-api                   band-63 SSO HTTP API contracts fields (PH-S1276)\n\
           --sso-admin-ops             band-64 SSO admin/ops fields (PH-S1285)\n\
           --sso-stand-smoke           band-65 SSO stand-smoke fields (PH-S1294)\n\
           --sso-loc-audit             band-66 SSO loc-audit aggregate fields (PH-S1304)\n\
           --sso-docs-canon            band-67 SSO docs-canon fields (PH-S1314)\n\
           --sso-vision-sync           band-68 SSO vision-sync fields (PH-S1324)\n\
           --sso-ratio-advisory        band-69 SSO ratio-advisory fields (PH-S1334)\n\
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

    let by_category: BTreeMap<String, CategoryLoc> = by_cat
        .into_iter()
        .map(|(cat, loc)| (cat.label().to_string(), loc))
        .collect();

    let meets_min_ratio = config
        .min_ratio
        .map(|floor| rust_ratio + f64::EPSILON >= floor);

    let ui_js_loc = by_category.get("ui_js").map(|c| c.loc).unwrap_or(0);
    let ui_js_loc_reduction = UI_JS_BAND28_BASELINE_LOC as i64 - ui_js_loc as i64;
    let ratio_95_formal_gate_met = rust_ratio + f64::EPSILON >= RATIO_95_FORMAL_GATE;
    let stretch_spirit_gate_met = rust_ratio + f64::EPSILON >= STRETCH_SPIRIT_GATE;
    let e2e_ts_loc = by_category.get("e2e_ts").map(|c| c.loc).unwrap_or(0);
    let e2e_ts_loc_reduction = E2E_TS_BAND29_BASELINE_LOC as i64 - e2e_ts_loc as i64;
    let ops_shell_canon_met = audit_ops_shell_canon(files);
    let migration_ui_js_candidate_count = ADMIN_JS_MIGRATION_CANDIDATES.len();
    let migration_e2e_archived_count = ARCHIVED_E2E_MIGRATION_CANON.len();
    let migration_candidate_total = migration_registry_total();

    let mut notes = vec![
        "Denominator: product code only (strategy §1); docs/yaml/png excluded",
        "GitHub Languages bar is heuristic; this report uses git-tracked LOC buckets",
        "PH-S165: CI --min-ratio 0.95 hold band (advisory); stretch spirit 96% via --stretch",
        "PH-S933: ratio_95_formal_gate_met when rust_ratio ≥ 0.95 (advisory hold when false)",
        "PH-S934: ui_js_loc_reduction vs band-28 baseline (PH-S925 zriz)",
        "PH-S941: e2e_ts_loc_reduction vs band-29 baseline (PH-S940 zriz)",
        "PH-S942: stretch_spirit_gate_met when rust_ratio ≥ 0.96",
        "PH-S943: ops_shell_canon_met when no .rs under bin/ or scripts/",
        "PH-S948: stretch advisory — below_stretch_spirit is expected until band 29+ migration",
        "PH-S958: digest band 30 hold advisory — in_formal_band true; target 95% hold until band 31+",
        "PH-S968: docs legacy band 31 hold advisory — in_formal_band true; target 95% hold until band 32+",
        "PH-S978: concept band 32 hold advisory — in_formal_band true; target 95% hold until band 33+",
        "PH-S988: STABLE band 33 hold advisory — in_formal_band true; target 95% hold until band 34+",
        "PH-S998: integration gap band 34 hold advisory — in_formal_band true; target 95% hold until band 35+",
        "PH-S1008: multi-module band 35 hold advisory — in_formal_band true; target 95% hold until PH-S1010",
        "PH-S1010: product-complete band 36 — ratio_95_formal_gate_met; stretch 96% advisory if below_stretch_spirit",
        "PH-S1018: owner ops band 37 — run-poolai quick/light + ops power; formal gate held from PH-S1010 zriz",
    ];
    if config.migration_advisory {
        notes.push(
            "PH-S1100: migration_advisory_mode — ui_js + archived e2e registry for stretch 96%",
        );
        notes.push(
            "PH-S1108: band 46 ratio/rust migration advisory — formal gate held; stretch pending",
        );
    }
    let (stable_criteria_met_count, stable_criteria_total_count) = if config.stable_touchup {
        audit_stable_touchup_criteria_met(root)
    } else {
        (0, stable_criteria_total())
    };
    if config.stable_touchup {
        notes.push("PH-S1110: stable_touchup_mode — maintenance STABLE criteria registry touch-up");
        notes.push("PH-S1118: band 47 STABLE touch-up — criteria met count vs registry total");
    }
    let (edge_verification_criteria_met_count, edge_verification_criteria_total_count) =
        if config.edge_verification_advisory {
            audit_edge_verification_criteria_met(root)
        } else {
            (0, edge_verification_criteria_total())
        };
    if config.edge_verification_advisory {
        notes.push(
            "PH-S1120: edge_verification_advisory_mode — Galaxy §6.6 fraud-proof/capability wire",
        );
        notes.push("PH-S1128: band 48 edge verification horizon — criteria met vs registry");
    }
    let (pre_push_criteria_met_count, pre_push_criteria_total_count) = if config.pre_push_canon {
        audit_pre_push_criteria_met(root)
    } else {
        (0, pre_push_hook_criteria_total())
    };
    if config.pre_push_canon {
        notes.push(
            "PH-S1130: pre_push_canon_mode — git pre-push hook + poolai-vision-sync canon docs",
        );
        notes.push("PH-S1138: band 49 pre-push vision canon gate — criteria met vs registry");
    }
    let (ci_canon_criteria_met_count, ci_canon_criteria_total_count) = if config.ci_canon {
        audit_ci_canon_criteria_met(root)
    } else {
        (0, ci_canon_criteria_total())
    };
    if config.ci_canon {
        notes
            .push("PH-S1140: ci_canon_mode — local dual-gate (test-ci + openapi-gap + rust-ratio)");
        notes.push("PH-S1148: band 50 CI canon gate — criteria met vs registry");
    }
    let (tenant_persist_criteria_met_count, tenant_persist_criteria_total_count) =
        if config.tenant_persist {
            audit_tenant_persist_criteria_met(root)
        } else {
            (0, tenant_persist_criteria_total())
        };
    if config.tenant_persist {
        notes.push(
            "PH-S1150: tenant_persist_mode — durable tenant store scaffold (POOLAI_TENANT_STORE)",
        );
        notes.push("PH-S1158: band 51 tenant persistence — criteria met vs registry");
    }
    let (tenant_store_criteria_met_count, tenant_store_criteria_total_count) =
        if config.tenant_store {
            audit_tenant_store_criteria_met(root)
        } else {
            (0, tenant_criteria_total())
        };
    if config.tenant_store {
        notes.push("PH-S1164: tenant_store_mode — durable path wire stub (POOLAI_TENANT_DATA_DIR)");
        notes.push("PH-S1168: band 52 tenant store wire — criteria met vs registry");
    }
    let (tenant_api_criteria_met_count, tenant_api_criteria_total_count) = if config.tenant_api {
        audit_tenant_api_criteria_met(root)
    } else {
        (0, tenant_api_criteria_total())
    };
    if config.tenant_api {
        notes.push(
            "PH-S1176: tenant_api_mode — HTTP CRUD/quota/isolation + store-wire read contracts",
        );
        notes.push("PH-S1178: band 53 tenant API contracts — criteria met vs registry");
    }
    let (tenant_admin_ops_criteria_met_count, tenant_admin_ops_criteria_total_count) =
        if config.tenant_admin_ops {
            audit_tenant_admin_ops_criteria_met(root)
        } else {
            (0, tenant_admin_ops_criteria_total())
        };
    if config.tenant_admin_ops {
        notes.push("PH-S1185: tenant_admin_ops_mode — store strip / usage+quota / verify hooks");
        notes.push("PH-S1188: band 54 tenant admin/ops — criteria met vs registry");
    }
    let (tenant_stand_smoke_criteria_met_count, tenant_stand_smoke_criteria_total_count) =
        if config.tenant_stand_smoke {
            audit_tenant_stand_smoke_criteria_met(root)
        } else {
            (0, tenant_stand_smoke_criteria_total())
        };
    if config.tenant_stand_smoke {
        notes
            .push("PH-S1194: tenant_stand_smoke_mode — live store/CRUD/usage+quota + verify hooks");
        notes.push("PH-S1198: band 55 tenant stand-smoke — criteria met vs registry");
    }
    let (tenant_loc_audit_criteria_met_count, tenant_loc_audit_criteria_total_count) =
        if config.tenant_loc_audit {
            audit_tenant_loc_audit_criteria_met(root)
        } else {
            (0, tenant_loc_audit_criteria_total())
        };
    if config.tenant_loc_audit {
        notes.push(
            "PH-S1204: tenant_loc_audit_mode — aggregate band 51–55 --tenant-* loc-audit slices",
        );
        notes.push("PH-S1208: band 56 tenant loc-audit — criteria met vs registry");
    }
    let (tenant_docs_canon_criteria_met_count, tenant_docs_canon_criteria_total_count) =
        if config.tenant_docs_canon {
            audit_tenant_docs_canon_criteria_met(root)
        } else {
            (0, tenant_docs_canon_criteria_total())
        };
    if config.tenant_docs_canon {
        notes
            .push("PH-S1214: tenant_docs_canon_mode — aggregate band 51–56 TENANT_*.md canon docs");
        notes.push("PH-S1218: band 57 tenant docs-canon — criteria met vs registry");
    }
    let (tenant_vision_sync_criteria_met_count, tenant_vision_sync_criteria_total_count) =
        if config.tenant_vision_sync {
            audit_tenant_vision_sync_criteria_met(root)
        } else {
            (0, tenant_vision_sync_criteria_total())
        };
    if config.tenant_vision_sync {
        notes.push(
            "PH-S1224: tenant_vision_sync_mode — aggregate docs/vision/* + TENANT_DOCS_CANON",
        );
        notes.push("PH-S1228: band 58 tenant vision-sync — criteria met vs registry");
    }
    let (tenant_ratio_advisory_criteria_met_count, tenant_ratio_advisory_criteria_total_count) =
        if config.tenant_ratio_advisory {
            audit_tenant_ratio_advisory_criteria_met(root)
        } else {
            (0, tenant_ratio_advisory_criteria_total())
        };
    if config.tenant_ratio_advisory {
        notes.push(
            "PH-S1234: tenant_ratio_advisory_mode — aggregate band 51–58 tenant slices + sqlite CRUD",
        );
        notes.push("PH-S1238: band 59 tenant ratio-advisory — criteria met vs registry");
    }
    let (tenant_horizon_criteria_met_count, tenant_horizon_criteria_total_count) =
        if config.tenant_horizon {
            audit_tenant_horizon_criteria_met(root)
        } else {
            (0, tenant_horizon_criteria_total())
        };
    if config.tenant_horizon {
        notes.push(
            "PH-S1244: tenant_horizon_mode — aggregate band 51–59 tenant slices (phase A close)",
        );
        notes.push("PH-S1248: band 60 tenant horizon close — criteria met vs registry");
    }
    let (sso_criteria_met_count, sso_criteria_total_count) = if config.sso {
        audit_sso_criteria_met(root)
    } else {
        (0, sso_criteria_total())
    };
    if config.sso {
        notes.push(
            "PH-S1254: sso_mode — SSO depth scaffold (POOLAI_SSO_STORE + audience/time stub)",
        );
        notes.push("PH-S1258: band 61 SSO depth — criteria met vs registry");
    }
    let (sso_store_criteria_met_count, sso_store_criteria_total_count) = if config.sso_store {
        audit_sso_store_criteria_met(root)
    } else {
        (0, sso_store_criteria_total())
    };
    if config.sso_store {
        notes.push("PH-S1264: sso_store_mode — durable path wire stub (POOLAI_SSO_DATA_DIR)");
        notes.push("PH-S1268: band 62 SSO store wire — criteria met vs registry");
    }
    let (sso_api_criteria_met_count, sso_api_criteria_total_count) = if config.sso_api {
        audit_sso_api_criteria_met(root)
    } else {
        (0, sso_api_criteria_total())
    };
    if config.sso_api {
        notes.push("PH-S1276: sso_api_mode — OAuth2/SAML HTTP CRUD + store-wire read contracts");
        notes.push("PH-S1278: band 63 SSO API contracts — criteria met vs registry");
    }
    let (sso_admin_ops_criteria_met_count, sso_admin_ops_criteria_total_count) =
        if config.sso_admin_ops {
            audit_sso_admin_ops_criteria_met(root)
        } else {
            (0, sso_admin_ops_criteria_total())
        };
    if config.sso_admin_ops {
        notes.push("PH-S1285: sso_admin_ops_mode — store strip / provider refresh / verify hooks");
        notes.push("PH-S1288: band 64 SSO admin/ops glue — criteria met vs registry");
    }
    let (sso_stand_smoke_criteria_met_count, sso_stand_smoke_criteria_total_count) =
        if config.sso_stand_smoke {
            audit_sso_stand_smoke_criteria_met(root)
        } else {
            (0, sso_stand_smoke_criteria_total())
        };
    if config.sso_stand_smoke {
        notes.push(
            "PH-S1294: sso_stand_smoke_mode — live store/CRUD/callback fixtures + verify hooks",
        );
        notes.push("PH-S1298: band 65 SSO stand-smoke — criteria met vs registry");
    }
    let (sso_loc_audit_criteria_met_count, sso_loc_audit_criteria_total_count) =
        if config.sso_loc_audit {
            audit_sso_loc_audit_criteria_met(root)
        } else {
            (0, sso_loc_audit_criteria_total())
        };
    if config.sso_loc_audit {
        notes.push("PH-S1304: sso_loc_audit_mode — aggregate band 61–65 --sso* loc-audit slices");
        notes.push("PH-S1308: band 66 SSO loc-audit — criteria met vs registry");
    }
    let (sso_docs_canon_criteria_met_count, sso_docs_canon_criteria_total_count) =
        if config.sso_docs_canon {
            audit_sso_docs_canon_criteria_met(root)
        } else {
            (0, sso_docs_canon_criteria_total())
        };
    if config.sso_docs_canon {
        notes.push("PH-S1314: sso_docs_canon_mode — aggregate band 61–66 SSO_*.md canon docs");
        notes.push("PH-S1318: band 67 SSO docs-canon — criteria met vs registry");
    }
    let (sso_vision_sync_criteria_met_count, sso_vision_sync_criteria_total_count) =
        if config.sso_vision_sync {
            audit_sso_vision_sync_criteria_met(root)
        } else {
            (0, sso_vision_sync_criteria_total())
        };
    if config.sso_vision_sync {
        notes.push("PH-S1324: sso_vision_sync_mode — aggregate docs/vision/* + SSO_DOCS_CANON");
        notes.push("PH-S1328: band 68 SSO vision-sync — criteria met vs registry");
    }
    let (sso_ratio_advisory_criteria_met_count, sso_ratio_advisory_criteria_total_count) =
        if config.sso_ratio_advisory {
            audit_sso_ratio_advisory_criteria_met(root)
        } else {
            (0, sso_ratio_advisory_criteria_total())
        };
    if config.sso_ratio_advisory {
        notes.push(
            "PH-S1334: sso_ratio_advisory_mode — aggregate prior --sso* + vision-sync slices",
        );
        notes.push("PH-S1338: band 69 SSO ratio-advisory — criteria met vs registry");
    }

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
        ui_js_loc,
        ui_js_band28_baseline_loc: UI_JS_BAND28_BASELINE_LOC,
        ui_js_loc_reduction,
        ratio_95_formal_gate_met,
        stretch_spirit_gate_met,
        e2e_ts_loc,
        e2e_ts_band29_baseline_loc: E2E_TS_BAND29_BASELINE_LOC,
        e2e_ts_loc_reduction,
        ops_shell_canon_met,
        migration_advisory_mode: config.migration_advisory,
        migration_candidate_total,
        migration_ui_js_candidate_count,
        migration_e2e_archived_count,
        stable_touchup_mode: config.stable_touchup,
        stable_criteria_total: stable_criteria_total_count,
        stable_criteria_met_count,
        edge_verification_advisory_mode: config.edge_verification_advisory,
        edge_verification_criteria_total: edge_verification_criteria_total_count,
        edge_verification_criteria_met_count,
        pre_push_canon_mode: config.pre_push_canon,
        pre_push_criteria_total: pre_push_criteria_total_count,
        pre_push_criteria_met_count,
        ci_canon_mode: config.ci_canon,
        ci_canon_criteria_total: ci_canon_criteria_total_count,
        ci_canon_criteria_met_count,
        tenant_persist_mode: config.tenant_persist,
        tenant_persist_criteria_total: tenant_persist_criteria_total_count,
        tenant_persist_criteria_met_count,
        tenant_store_mode: config.tenant_store,
        tenant_store_criteria_total: tenant_store_criteria_total_count,
        tenant_store_criteria_met_count,
        tenant_api_mode: config.tenant_api,
        tenant_api_criteria_total: tenant_api_criteria_total_count,
        tenant_api_criteria_met_count,
        tenant_admin_ops_mode: config.tenant_admin_ops,
        tenant_admin_ops_criteria_total: tenant_admin_ops_criteria_total_count,
        tenant_admin_ops_criteria_met_count,
        tenant_stand_smoke_mode: config.tenant_stand_smoke,
        tenant_stand_smoke_criteria_total: tenant_stand_smoke_criteria_total_count,
        tenant_stand_smoke_criteria_met_count,
        tenant_loc_audit_mode: config.tenant_loc_audit,
        tenant_loc_audit_criteria_total: tenant_loc_audit_criteria_total_count,
        tenant_loc_audit_criteria_met_count,
        tenant_docs_canon_mode: config.tenant_docs_canon,
        tenant_docs_canon_criteria_total: tenant_docs_canon_criteria_total_count,
        tenant_docs_canon_criteria_met_count,
        tenant_vision_sync_mode: config.tenant_vision_sync,
        tenant_vision_sync_criteria_total: tenant_vision_sync_criteria_total_count,
        tenant_vision_sync_criteria_met_count,
        tenant_ratio_advisory_mode: config.tenant_ratio_advisory,
        tenant_ratio_advisory_criteria_total: tenant_ratio_advisory_criteria_total_count,
        tenant_ratio_advisory_criteria_met_count,
        tenant_horizon_mode: config.tenant_horizon,
        tenant_horizon_criteria_total: tenant_horizon_criteria_total_count,
        tenant_horizon_criteria_met_count,
        sso_mode: config.sso,
        sso_criteria_total: sso_criteria_total_count,
        sso_criteria_met_count,
        sso_store_mode: config.sso_store,
        sso_store_criteria_total: sso_store_criteria_total_count,
        sso_store_criteria_met_count,
        sso_api_mode: config.sso_api,
        sso_api_criteria_total: sso_api_criteria_total_count,
        sso_api_criteria_met_count,
        sso_admin_ops_mode: config.sso_admin_ops,
        sso_admin_ops_criteria_total: sso_admin_ops_criteria_total_count,
        sso_admin_ops_criteria_met_count,
        sso_stand_smoke_mode: config.sso_stand_smoke,
        sso_stand_smoke_criteria_total: sso_stand_smoke_criteria_total_count,
        sso_stand_smoke_criteria_met_count,
        sso_loc_audit_mode: config.sso_loc_audit,
        sso_loc_audit_criteria_total: sso_loc_audit_criteria_total_count,
        sso_loc_audit_criteria_met_count,
        sso_docs_canon_mode: config.sso_docs_canon,
        sso_docs_canon_criteria_total: sso_docs_canon_criteria_total_count,
        sso_docs_canon_criteria_met_count,
        sso_vision_sync_mode: config.sso_vision_sync,
        sso_vision_sync_criteria_total: sso_vision_sync_criteria_total_count,
        sso_vision_sync_criteria_met_count,
        sso_ratio_advisory_mode: config.sso_ratio_advisory,
        sso_ratio_advisory_criteria_total: sso_ratio_advisory_criteria_total_count,
        sso_ratio_advisory_criteria_met_count,
        by_category,
        notes,
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
    println!(
        "  ui_js_loc:             {} (band28 baseline {} · reduction {})",
        report.ui_js_loc, report.ui_js_band28_baseline_loc, report.ui_js_loc_reduction
    );
    println!(
        "  ratio_95_formal_gate:  {}",
        report.ratio_95_formal_gate_met
    );
    println!(
        "  stretch_spirit_gate:   {}",
        report.stretch_spirit_gate_met
    );
    println!(
        "  e2e_ts_loc:            {} (band29 baseline {} · reduction {})",
        report.e2e_ts_loc, report.e2e_ts_band29_baseline_loc, report.e2e_ts_loc_reduction
    );
    println!("  ops_shell_canon_met:   {}", report.ops_shell_canon_met);
    if report.migration_advisory_mode {
        println!("  migration_advisory:    true (PH-S1100 band 46)");
        println!(
            "  migration_candidates:  {} (ui_js {} · e2e archived {})",
            report.migration_candidate_total,
            report.migration_ui_js_candidate_count,
            report.migration_e2e_archived_count
        );
        println!(
            "  migration_cases:       {}",
            MIGRATION_ADVISORY_CASES.join(", ")
        );
    }
    if report.stable_touchup_mode {
        println!("  stable_touchup:        true (PH-S1110 band 47)");
        println!(
            "  stable_criteria:       {}/{} met",
            report.stable_criteria_met_count, report.stable_criteria_total
        );
        println!(
            "  stable_touchup_cases:  {}",
            STABLE_TOUCHUP_CASES.join(", ")
        );
    }
    if report.edge_verification_advisory_mode {
        println!("  edge_verification:     true (PH-S1120 band 48)");
        println!(
            "  edge_criteria:         {}/{} met",
            report.edge_verification_criteria_met_count, report.edge_verification_criteria_total
        );
        println!(
            "  edge_verification_cases: {}",
            EDGE_VERIFICATION_CASES.join(", ")
        );
    }
    if report.pre_push_canon_mode {
        println!("  pre_push_canon:        true (PH-S1130 band 49)");
        println!(
            "  pre_push_criteria:     {}/{} met",
            report.pre_push_criteria_met_count, report.pre_push_criteria_total
        );
        println!(
            "  pre_push_canon_cases:  {}",
            PRE_PUSH_HOOK_CASES.join(", ")
        );
    }
    if report.ci_canon_mode {
        println!("  ci_canon:              true (PH-S1140 band 50)");
        println!(
            "  ci_canon_criteria:     {}/{} met",
            report.ci_canon_criteria_met_count, report.ci_canon_criteria_total
        );
        println!("  ci_canon_cases:        {}", CI_CANON_CASES.join(", "));
    }
    if report.tenant_persist_mode {
        println!("  tenant_persist:        true (PH-S1150 band 51)");
        println!(
            "  tenant_persist_criteria: {}/{} met",
            report.tenant_persist_criteria_met_count, report.tenant_persist_criteria_total
        );
        println!(
            "  tenant_persist_cases:  {}",
            TENANT_PERSIST_CASES.join(", ")
        );
    }
    if report.tenant_store_mode {
        println!("  tenant_store:          true (PH-S1164 band 52)");
        println!(
            "  tenant_store_criteria: {}/{} met",
            report.tenant_store_criteria_met_count, report.tenant_store_criteria_total
        );
        println!("  tenant_store_cases:    {}", TENANT_CASES.join(", "));
    }
    if report.tenant_api_mode {
        println!("  tenant_api:            true (PH-S1176 band 53)");
        println!(
            "  tenant_api_criteria:   {}/{} met",
            report.tenant_api_criteria_met_count, report.tenant_api_criteria_total
        );
        println!("  tenant_api_cases:      {}", TENANT_API_CASES.join(", "));
    }
    if report.tenant_admin_ops_mode {
        println!("  tenant_admin_ops:      true (PH-S1185 band 54)");
        println!(
            "  tenant_admin_ops_criteria: {}/{} met",
            report.tenant_admin_ops_criteria_met_count, report.tenant_admin_ops_criteria_total
        );
        println!(
            "  tenant_admin_ops_cases: {}",
            TENANT_ADMIN_OPS_CASES.join(", ")
        );
    }
    if report.tenant_stand_smoke_mode {
        println!("  tenant_stand_smoke:    true (PH-S1194 band 55)");
        println!(
            "  tenant_stand_smoke_criteria: {}/{} met",
            report.tenant_stand_smoke_criteria_met_count, report.tenant_stand_smoke_criteria_total
        );
        println!(
            "  tenant_stand_smoke_cases: {}",
            TENANT_STAND_SMOKE_CASES.join(", ")
        );
    }
    if report.tenant_loc_audit_mode {
        println!("  tenant_loc_audit:      true (PH-S1204 band 56)");
        println!(
            "  tenant_loc_audit_criteria: {}/{} met",
            report.tenant_loc_audit_criteria_met_count, report.tenant_loc_audit_criteria_total
        );
        println!(
            "  tenant_loc_audit_cases: {}",
            TENANT_LOC_AUDIT_CASES.join(", ")
        );
    }
    if report.tenant_docs_canon_mode {
        println!("  tenant_docs_canon:     true (PH-S1214 band 57)");
        println!(
            "  tenant_docs_canon_criteria: {}/{} met",
            report.tenant_docs_canon_criteria_met_count, report.tenant_docs_canon_criteria_total
        );
        println!(
            "  tenant_docs_canon_cases: {}",
            TENANT_DOCS_CANON_CASES.join(", ")
        );
    }
    if report.tenant_vision_sync_mode {
        println!("  tenant_vision_sync:    true (PH-S1224 band 58)");
        println!(
            "  tenant_vision_sync_criteria: {}/{} met",
            report.tenant_vision_sync_criteria_met_count, report.tenant_vision_sync_criteria_total
        );
        println!(
            "  tenant_vision_sync_cases: {}",
            TENANT_VISION_SYNC_CASES.join(", ")
        );
    }
    if report.tenant_ratio_advisory_mode {
        println!("  tenant_ratio_advisory: true (PH-S1234 band 59)");
        println!(
            "  tenant_ratio_advisory_criteria: {}/{} met",
            report.tenant_ratio_advisory_criteria_met_count,
            report.tenant_ratio_advisory_criteria_total
        );
        println!(
            "  tenant_ratio_advisory_cases: {}",
            TENANT_RATIO_ADVISORY_CASES.join(", ")
        );
    }
    if report.tenant_horizon_mode {
        println!("  tenant_horizon:        true (PH-S1244 band 60)");
        println!(
            "  tenant_horizon_criteria: {}/{} met",
            report.tenant_horizon_criteria_met_count, report.tenant_horizon_criteria_total
        );
        println!(
            "  tenant_horizon_cases: {}",
            TENANT_HORIZON_CASES.join(", ")
        );
    }
    if report.sso_mode {
        println!("  sso:                   true (PH-S1254 band 61)");
        println!(
            "  sso_criteria:          {}/{} met",
            report.sso_criteria_met_count, report.sso_criteria_total
        );
        println!("  sso_cases:             {}", SSO_CASES.join(", "));
    }
    if report.sso_store_mode {
        println!("  sso_store:             true (PH-S1264 band 62)");
        println!(
            "  sso_store_criteria:    {}/{} met",
            report.sso_store_criteria_met_count, report.sso_store_criteria_total
        );
        println!("  sso_store_cases:       {}", SSO_STORE_CASES.join(", "));
    }
    if report.sso_api_mode {
        println!("  sso_api:               true (PH-S1276 band 63)");
        println!(
            "  sso_api_criteria:      {}/{} met",
            report.sso_api_criteria_met_count, report.sso_api_criteria_total
        );
        println!("  sso_api_cases:         {}", SSO_API_CASES.join(", "));
    }
    if report.sso_admin_ops_mode {
        println!("  sso_admin_ops:         true (PH-S1285 band 64)");
        println!(
            "  sso_admin_ops_criteria: {}/{} met",
            report.sso_admin_ops_criteria_met_count, report.sso_admin_ops_criteria_total
        );
        println!(
            "  sso_admin_ops_cases:   {}",
            SSO_ADMIN_OPS_CASES.join(", ")
        );
    }
    if report.sso_stand_smoke_mode {
        println!("  sso_stand_smoke:       true (PH-S1294 band 65)");
        println!(
            "  sso_stand_smoke_criteria: {}/{} met",
            report.sso_stand_smoke_criteria_met_count, report.sso_stand_smoke_criteria_total
        );
        println!(
            "  sso_stand_smoke_cases: {}",
            SSO_STAND_SMOKE_CASES.join(", ")
        );
    }
    if report.sso_loc_audit_mode {
        println!("  sso_loc_audit:         true (PH-S1304 band 66)");
        println!(
            "  sso_loc_audit_criteria: {}/{} met",
            report.sso_loc_audit_criteria_met_count, report.sso_loc_audit_criteria_total
        );
        println!("  sso_loc_audit_cases: {}", SSO_LOC_AUDIT_CASES.join(", "));
    }
    if report.sso_docs_canon_mode {
        println!("  sso_docs_canon:        true (PH-S1314 band 67)");
        println!(
            "  sso_docs_canon_criteria: {}/{} met",
            report.sso_docs_canon_criteria_met_count, report.sso_docs_canon_criteria_total
        );
        println!(
            "  sso_docs_canon_cases: {}",
            SSO_DOCS_CANON_CASES.join(", ")
        );
    }
    if report.sso_vision_sync_mode {
        println!("  sso_vision_sync:       true (PH-S1324 band 68)");
        println!(
            "  sso_vision_sync_criteria: {}/{} met",
            report.sso_vision_sync_criteria_met_count, report.sso_vision_sync_criteria_total
        );
        println!(
            "  sso_vision_sync_cases: {}",
            SSO_VISION_SYNC_CASES.join(", ")
        );
    }
    if report.sso_ratio_advisory_mode {
        println!("  sso_ratio_advisory:    true (PH-S1334 band 69)");
        println!(
            "  sso_ratio_advisory_criteria: {}/{} met",
            report.sso_ratio_advisory_criteria_met_count, report.sso_ratio_advisory_criteria_total
        );
        println!(
            "  sso_ratio_advisory_cases: {}",
            SSO_RATIO_ADVISORY_CASES.join(", ")
        );
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
    fn ratio_95_formal_gate_ph_s933() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        let files = vec!["src/a.rs".to_string()];
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/a.rs"), "line\n".repeat(95)).unwrap();
        let report = build_report(root, &files, AuditConfig::default()).expect("report");
        assert!(report.ratio_95_formal_gate_met);
        assert!(report.rust_ratio + f64::EPSILON >= RATIO_95_FORMAL_GATE);

        let files_low = vec!["src/a.rs".to_string(), "src/ui/x.js".to_string()];
        fs::create_dir_all(root.join("src/ui")).unwrap();
        fs::write(root.join("src/ui/x.js"), "a\n".repeat(20)).unwrap();
        let report_low = build_report(root, &files_low, AuditConfig::default()).expect("report");
        assert!(!report_low.ratio_95_formal_gate_met);
        assert!(report_low.below_target);
    }

    #[test]
    fn stretch_spirit_gate_ph_s942() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        let files = vec!["src/a.rs".to_string()];
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/a.rs"), "line\n".repeat(96)).unwrap();
        let report = build_report(root, &files, AuditConfig::default()).expect("report");
        assert!(report.stretch_spirit_gate_met);
        assert!(report.rust_ratio + f64::EPSILON >= STRETCH_SPIRIT_GATE);

        let files_low = vec!["src/a.rs".to_string(), "e2e/t.spec.ts".to_string()];
        fs::create_dir_all(root.join("e2e")).unwrap();
        fs::write(root.join("e2e/t.spec.ts"), "a\n".repeat(10)).unwrap();
        let report_low = build_report(root, &files_low, AuditConfig::default()).expect("report");
        assert!(!report_low.stretch_spirit_gate_met);
        assert!(report_low.below_stretch_spirit);
    }

    #[test]
    fn ops_shell_canon_ph_s943() {
        assert!(audit_ops_shell_canon(&["bin/run.sh".to_string()]));
        assert!(audit_ops_shell_canon(&["scripts/setup.sh".to_string()]));
        assert!(!audit_ops_shell_canon(&["bin/bad.rs".to_string()]));
        assert!(!audit_ops_shell_canon(&["scripts/tool.rs".to_string()]));
    }

    #[test]
    fn e2e_ts_loc_reduction_metric_ph_s941() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        let files = vec!["e2e/tests/smoke.spec.ts".to_string()];
        fs::create_dir_all(root.join("e2e/tests")).unwrap();
        fs::write(root.join("e2e/tests/smoke.spec.ts"), "a\nb\n").unwrap();
        let report = build_report(root, &files, AuditConfig::default()).expect("report");
        assert_eq!(report.e2e_ts_loc, 2);
        assert_eq!(
            report.e2e_ts_band29_baseline_loc,
            E2E_TS_BAND29_BASELINE_LOC
        );
        assert_eq!(
            report.e2e_ts_loc_reduction,
            E2E_TS_BAND29_BASELINE_LOC as i64 - 2
        );
    }

    #[test]
    fn ui_js_loc_reduction_metric_ph_s934() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        let files = vec!["src/ui/admin_common.js".to_string()];
        fs::create_dir_all(root.join("src/ui")).unwrap();
        fs::write(root.join("src/ui/admin_common.js"), "a\nb\n").unwrap();
        let report = build_report(root, &files, AuditConfig::default()).expect("report");
        assert_eq!(report.ui_js_loc, 2);
        assert_eq!(report.ui_js_band28_baseline_loc, UI_JS_BAND28_BASELINE_LOC);
        assert_eq!(
            report.ui_js_loc_reduction,
            UI_JS_BAND28_BASELINE_LOC as i64 - 2
        );
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
