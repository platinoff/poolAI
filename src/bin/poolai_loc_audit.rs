//! LOC ratio baseline audit (PH-S143, PH-S150 advisory, PH-S159 stretch, PH-S165 hold gate) per
//! [`docs/development/RUST_RATIO_STRATEGY_2026-06-13.md`].
//!
//! ```text
//! cargo run --bin poolai-loc-audit
//! cargo run --bin poolai-loc-audit -- --output docs/development/rust_ratio.json
//! cargo run --bin poolai-loc-audit -- --warn-below 0.93 --target 0.95 --stretch 0.96 --min-ratio 0.95 --advisory
//! cargo run --bin poolai-loc-audit -- --min-ratio 0.91
//! ```

use poolai_ui_core::audit_admin_ops_depth::{
    audit_admin_ops_criteria_total, AUDIT_ADMIN_OPS_CASES, AUDIT_ADMIN_OPS_CRITERIA,
};
use poolai_ui_core::audit_api_contracts_depth::{
    audit_api_criteria_total, AUDIT_API_CASES, AUDIT_API_CRITERIA,
};
use poolai_ui_core::audit_depth::{audit_criteria_total, AUDIT_CASES, AUDIT_CRITERIA};
use poolai_ui_core::audit_docs_canon_depth::{
    audit_docs_canon_criteria_total, AUDIT_DOCS_CANON_CASES, AUDIT_DOCS_CANON_CRITERIA,
};
use poolai_ui_core::audit_horizon_depth::{
    audit_horizon_criteria_total, AUDIT_HORIZON_CASES, AUDIT_HORIZON_CRITERIA,
};
use poolai_ui_core::audit_loc_audit_depth::{
    audit_loc_audit_criteria_total, AUDIT_LOC_AUDIT_CASES, AUDIT_LOC_AUDIT_CRITERIA,
};
use poolai_ui_core::audit_ratio_advisory_depth::{
    audit_ratio_advisory_criteria_total, AUDIT_RATIO_ADVISORY_CASES, AUDIT_RATIO_ADVISORY_CRITERIA,
};
use poolai_ui_core::audit_stand_smoke_depth::{
    audit_stand_smoke_criteria_total, AUDIT_STAND_SMOKE_CASES, AUDIT_STAND_SMOKE_CRITERIA,
};
use poolai_ui_core::audit_store_depth::{
    audit_store_criteria_total, AUDIT_STORE_CASES, AUDIT_STORE_CRITERIA,
};
use poolai_ui_core::audit_vision_sync_depth::{
    audit_vision_sync_criteria_total, AUDIT_VISION_SYNC_CASES, AUDIT_VISION_SYNC_CRITERIA,
};
use poolai_ui_core::ci_canon_depth::{ci_canon_criteria_total, CI_CANON_CASES, CI_CANON_CRITERIA};
use poolai_ui_core::galaxy_edge_verification_depth::{
    edge_verification_criteria_total, EDGE_VERIFICATION_CASES, EDGE_VERIFICATION_CRITERIA,
};
use poolai_ui_core::gpu_limits_admin_ops_depth::{
    gpu_limits_admin_ops_criteria_total, GPU_LIMITS_ADMIN_OPS_CASES, GPU_LIMITS_ADMIN_OPS_CRITERIA,
};
use poolai_ui_core::gpu_limits_api_depth::{
    gpu_limits_api_criteria_total, GPU_LIMITS_API_CASES, GPU_LIMITS_API_CRITERIA,
};
use poolai_ui_core::gpu_limits_depth::{
    gpu_limits_criteria_total, GPU_LIMITS_CASES, GPU_LIMITS_CRITERIA,
};
use poolai_ui_core::monitoring_admin_ops_depth::{
    monitoring_admin_ops_criteria_total, MONITORING_ADMIN_OPS_CASES, MONITORING_ADMIN_OPS_CRITERIA,
};
use poolai_ui_core::monitoring_api_contracts_depth::{
    monitoring_api_criteria_total, MONITORING_API_CASES, MONITORING_API_CRITERIA,
};
use poolai_ui_core::monitoring_depth::{
    monitoring_criteria_total, MONITORING_CASES, MONITORING_CRITERIA,
};
use poolai_ui_core::monitoring_docs_canon_depth::{
    monitoring_docs_canon_criteria_total, MONITORING_DOCS_CANON_CASES,
    MONITORING_DOCS_CANON_CRITERIA,
};
use poolai_ui_core::monitoring_horizon_depth::{
    monitoring_horizon_criteria_total, MONITORING_HORIZON_CASES, MONITORING_HORIZON_CRITERIA,
};
use poolai_ui_core::monitoring_loc_audit_depth::{
    monitoring_loc_audit_criteria_total, MONITORING_LOC_AUDIT_CASES, MONITORING_LOC_AUDIT_CRITERIA,
};
use poolai_ui_core::monitoring_ratio_advisory_depth::{
    monitoring_ratio_advisory_criteria_total, MONITORING_RATIO_ADVISORY_CASES,
    MONITORING_RATIO_ADVISORY_CRITERIA,
};
use poolai_ui_core::monitoring_stand_smoke_depth::{
    monitoring_stand_smoke_criteria_total, MONITORING_STAND_SMOKE_CASES,
    MONITORING_STAND_SMOKE_CRITERIA,
};
use poolai_ui_core::monitoring_store_depth::{
    monitoring_store_criteria_total, MONITORING_STORE_CASES, MONITORING_STORE_CRITERIA,
};
use poolai_ui_core::monitoring_vision_sync_depth::{
    monitoring_vision_sync_criteria_total, MONITORING_VISION_SYNC_CASES,
    MONITORING_VISION_SYNC_CRITERIA,
};
use poolai_ui_core::policy_admin_ops_depth::{
    policy_admin_ops_criteria_total, POLICY_ADMIN_OPS_CASES, POLICY_ADMIN_OPS_CRITERIA,
};
use poolai_ui_core::policy_api_contracts_depth::{
    policy_api_criteria_total, POLICY_API_CASES, POLICY_API_CRITERIA,
};
use poolai_ui_core::policy_depth::{policy_criteria_total, POLICY_CASES, POLICY_CRITERIA};
use poolai_ui_core::policy_docs_canon_depth::{
    policy_docs_canon_criteria_total, POLICY_DOCS_CANON_CASES, POLICY_DOCS_CANON_CRITERIA,
};
use poolai_ui_core::policy_horizon_depth::{
    policy_horizon_criteria_total, POLICY_HORIZON_CASES, POLICY_HORIZON_CRITERIA,
};
use poolai_ui_core::policy_loc_audit_depth::{
    policy_loc_audit_criteria_total, POLICY_LOC_AUDIT_CASES, POLICY_LOC_AUDIT_CRITERIA,
};
use poolai_ui_core::policy_ratio_advisory_depth::{
    policy_ratio_advisory_criteria_total, POLICY_RATIO_ADVISORY_CASES,
    POLICY_RATIO_ADVISORY_CRITERIA,
};
use poolai_ui_core::policy_stand_smoke_depth::{
    policy_stand_smoke_criteria_total, POLICY_STAND_SMOKE_CASES, POLICY_STAND_SMOKE_CRITERIA,
};
use poolai_ui_core::policy_store_depth::{
    policy_store_criteria_total, POLICY_STORE_CASES, POLICY_STORE_CRITERIA,
};
use poolai_ui_core::policy_vision_sync_depth::{
    policy_vision_sync_criteria_total, POLICY_VISION_SYNC_CASES, POLICY_VISION_SYNC_CRITERIA,
};
use poolai_ui_core::pre_push_hook_depth::{
    pre_push_hook_criteria_total, PRE_PUSH_HOOK_CASES, PRE_PUSH_HOOK_CRITERIA,
};
use poolai_ui_core::ratio96_admin_ops_depth::{
    ratio96_admin_ops_criteria_total, RATIO96_ADMIN_OPS_CASES, RATIO96_ADMIN_OPS_CRITERIA,
};
use poolai_ui_core::ratio96_depth::{ratio96_criteria_total, RATIO96_CASES, RATIO96_CRITERIA};
use poolai_ui_core::ratio96_docs_canon_depth::{
    ratio96_docs_canon_criteria_total, RATIO96_DOCS_CANON_CASES, RATIO96_DOCS_CANON_CRITERIA,
};
use poolai_ui_core::ratio96_loc_audit_depth::{
    ratio96_loc_audit_criteria_total, RATIO96_LOC_AUDIT_CRITERIA,
};
use poolai_ui_core::ratio96_stand_smoke_depth::{
    ratio96_stand_smoke_criteria_total, RATIO96_STAND_SMOKE_CASES, RATIO96_STAND_SMOKE_CRITERIA,
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
use poolai_ui_core::sso_horizon_depth::{
    sso_horizon_criteria_total, SSO_HORIZON_CASES, SSO_HORIZON_CRITERIA,
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
    /// Emit band-70 SSO horizon-close fields (PH-S1344).
    sso_horizon: bool,
    /// Emit band-71 audit depth fields (PH-S1354).
    audit: bool,
    /// Emit band-72 audit store-wire fields (PH-S1364).
    audit_store: bool,
    /// Emit band-73 audit HTTP API contracts fields (PH-S1375).
    audit_api: bool,
    /// Emit band-74 audit admin/ops fields (PH-S1385).
    audit_admin_ops: bool,
    /// Emit band-75 audit stand-smoke fields (PH-S1394).
    audit_stand_smoke: bool,
    /// Emit band-76 audit loc-audit aggregate fields (PH-S1404).
    audit_loc_audit: bool,
    /// Emit band-77 audit docs-canon fields (PH-S1414).
    audit_docs_canon: bool,
    /// Emit band-78 audit vision-sync fields (PH-S1424).
    audit_vision_sync: bool,
    /// Emit band-79 audit ratio-advisory fields (PH-S1434).
    audit_ratio_advisory: bool,
    /// Emit band-80 audit horizon-close fields (PH-S1444).
    audit_horizon: bool,
    /// Emit band-81 policies depth fields (PH-S1454).
    policy: bool,
    /// Emit band-82 policies store-wire fields (PH-S1464).
    policy_store: bool,
    /// Emit band-83 policies HTTP API contracts fields (PH-S1475).
    policy_api: bool,
    /// Emit band-84 policies admin/ops fields (PH-S1485).
    policy_admin_ops: bool,
    /// Emit band-85 policies stand-smoke fields (PH-S1494).
    policy_stand_smoke: bool,
    /// Emit band-86 policies loc-audit aggregate fields (PH-S1504).
    policy_loc_audit: bool,
    /// Emit band-87 policies docs-canon fields (PH-S1514).
    policy_docs_canon: bool,
    /// Emit band-88 policies vision-sync fields (PH-S1524).
    policy_vision_sync: bool,
    /// Emit band-89 policies ratio-advisory fields (PH-S1534).
    policy_ratio_advisory: bool,
    /// Emit band-90 policies horizon-close fields (PH-S1544).
    policy_horizon: bool,
    /// Emit band-91 monitoring depth fields (PH-S1554).
    monitoring: bool,
    /// Emit band-92 monitoring store-wire fields (PH-S1564).
    monitoring_store: bool,
    /// Emit band-93 monitoring HTTP API contracts fields (PH-S1575).
    monitoring_api: bool,
    /// Emit band-94 monitoring admin/ops fields (PH-S1585).
    monitoring_admin_ops: bool,
    /// Emit band-95 monitoring stand-smoke fields (PH-S1594).
    monitoring_stand_smoke: bool,
    /// Emit band-96 monitoring loc-audit aggregate fields (PH-S1604).
    monitoring_loc_audit: bool,
    /// Emit band-97 monitoring docs-canon fields (PH-S1614).
    monitoring_docs_canon: bool,
    /// Emit band-98 monitoring vision-sync fields (PH-S1624).
    monitoring_vision_sync: bool,
    /// Emit band-99 monitoring ratio-advisory fields (PH-S1634).
    monitoring_ratio_advisory: bool,
    /// Emit band-100 monitoring horizon-close fields (PH-S1644).
    monitoring_horizon: bool,
    /// Emit band-101 ratio96 depth scaffold fields (PH-S1654).
    ratio96: bool,
    /// Emit band-104 ratio96 admin/ops glue fields (PH-S1684).
    ratio96_admin_ops: bool,
    /// Emit band-105 ratio96 stand smoke fields (PH-S1694).
    ratio96_stand_smoke: bool,
    /// Emit band-106 ratio96 loc-audit fields (PH-S1703).
    ratio96_loc_audit: bool,
    /// Emit band-107 ratio96 docs-canon fields (PH-S1714).
    ratio96_docs_canon: bool,
    /// Emit band-122 GPU limits store/wire fields (PH-S1862).
    gpu_limits: bool,
    /// Emit band-123 GPU limits API fields (PH-S1872).
    gpu_limits_api: bool,
    /// Emit band-124 GPU limits admin/ops glue fields (PH-S1884).
    gpu_limits_admin_ops: bool,
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
            sso_horizon: false,
            audit: false,
            audit_store: false,
            audit_api: false,
            audit_admin_ops: false,
            audit_stand_smoke: false,
            audit_loc_audit: false,
            audit_docs_canon: false,
            audit_vision_sync: false,
            audit_ratio_advisory: false,
            audit_horizon: false,
            policy: false,
            policy_store: false,
            policy_api: false,
            policy_admin_ops: false,
            policy_stand_smoke: false,
            policy_loc_audit: false,
            policy_docs_canon: false,
            policy_vision_sync: false,
            policy_ratio_advisory: false,
            policy_horizon: false,
            monitoring: false,
            monitoring_store: false,
            monitoring_api: false,
            monitoring_admin_ops: false,
            monitoring_stand_smoke: false,
            monitoring_loc_audit: false,
            monitoring_docs_canon: false,
            monitoring_vision_sync: false,
            monitoring_ratio_advisory: false,
            monitoring_horizon: false,
            ratio96: false,
            ratio96_admin_ops: false,
            ratio96_stand_smoke: false,
            ratio96_loc_audit: false,
            ratio96_docs_canon: false,
            gpu_limits: false,
            gpu_limits_api: false,
            gpu_limits_admin_ops: false,
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
    /// Band-70 SSO horizon-close mode (PH-S1344).
    sso_horizon_mode: bool,
    /// SSO horizon criteria registry size (PH-S1344).
    sso_horizon_criteria_total: usize,
    /// SSO horizon criteria met count (PH-S1344).
    sso_horizon_criteria_met_count: usize,
    /// Band-71 audit depth mode (PH-S1354).
    audit_mode: bool,
    /// Audit depth criteria registry size (PH-S1354).
    audit_criteria_total: usize,
    /// Audit depth criteria met count (PH-S1354).
    audit_criteria_met_count: usize,
    /// Band-72 audit store-wire mode (PH-S1364).
    audit_store_mode: bool,
    /// Audit store-wire criteria registry size (PH-S1364).
    audit_store_criteria_total: usize,
    /// Audit store-wire criteria met count (PH-S1364).
    audit_store_criteria_met_count: usize,
    /// Band-73 audit HTTP API contracts mode (PH-S1375).
    audit_api_mode: bool,
    /// Audit HTTP API criteria registry size (PH-S1375).
    audit_api_criteria_total: usize,
    /// Audit HTTP API criteria met count (PH-S1375).
    audit_api_criteria_met_count: usize,
    /// Band-74 audit admin/ops mode (PH-S1385).
    audit_admin_ops_mode: bool,
    /// Audit admin/ops criteria registry size (PH-S1385).
    audit_admin_ops_criteria_total: usize,
    /// Audit admin/ops criteria met count (PH-S1385).
    audit_admin_ops_criteria_met_count: usize,
    /// Band-75 audit stand-smoke mode (PH-S1394).
    audit_stand_smoke_mode: bool,
    /// Audit stand-smoke criteria registry size (PH-S1394).
    audit_stand_smoke_criteria_total: usize,
    /// Audit stand-smoke criteria met count (PH-S1394).
    audit_stand_smoke_criteria_met_count: usize,
    /// Band-76 audit loc-audit aggregate mode (PH-S1404).
    audit_loc_audit_mode: bool,
    /// Audit loc-audit criteria registry size (PH-S1404).
    audit_loc_audit_criteria_total: usize,
    /// Audit loc-audit criteria met count (PH-S1404).
    audit_loc_audit_criteria_met_count: usize,
    /// Band-77 audit docs-canon mode (PH-S1414).
    audit_docs_canon_mode: bool,
    /// Audit docs-canon criteria registry size (PH-S1414).
    audit_docs_canon_criteria_total: usize,
    /// Audit docs-canon criteria met count (PH-S1414).
    audit_docs_canon_criteria_met_count: usize,
    /// Band-78 audit vision-sync mode (PH-S1424).
    audit_vision_sync_mode: bool,
    /// Audit vision-sync criteria registry size (PH-S1424).
    audit_vision_sync_criteria_total: usize,
    /// Audit vision-sync criteria met count (PH-S1424).
    audit_vision_sync_criteria_met_count: usize,
    /// Band-79 audit ratio-advisory mode (PH-S1434).
    audit_ratio_advisory_mode: bool,
    /// Audit ratio-advisory criteria registry size (PH-S1434).
    audit_ratio_advisory_criteria_total: usize,
    /// Audit ratio-advisory criteria met count (PH-S1434).
    audit_ratio_advisory_criteria_met_count: usize,
    /// Band-80 audit horizon-close mode (PH-S1444).
    audit_horizon_mode: bool,
    /// Audit horizon criteria registry size (PH-S1444).
    audit_horizon_criteria_total: usize,
    /// Audit horizon criteria met count (PH-S1444).
    audit_horizon_criteria_met_count: usize,
    /// Band-81 policies depth mode (PH-S1454).
    policy_mode: bool,
    /// Policies depth criteria registry size (PH-S1454).
    policy_criteria_total: usize,
    /// Policies depth criteria met count (PH-S1454).
    policy_criteria_met_count: usize,
    /// Band-82 policies store-wire mode (PH-S1464).
    policy_store_mode: bool,
    /// Policies store-wire criteria registry size (PH-S1464).
    policy_store_criteria_total: usize,
    /// Policies store-wire criteria met count (PH-S1464).
    policy_store_criteria_met_count: usize,
    /// Band-83 policies HTTP API contracts mode (PH-S1475).
    policy_api_mode: bool,
    /// Policies HTTP API criteria registry size (PH-S1475).
    policy_api_criteria_total: usize,
    /// Policies HTTP API criteria met count (PH-S1475).
    policy_api_criteria_met_count: usize,
    /// Band-84 policies admin/ops mode (PH-S1485).
    policy_admin_ops_mode: bool,
    /// Policies admin/ops criteria registry size (PH-S1485).
    policy_admin_ops_criteria_total: usize,
    /// Policies admin/ops criteria met count (PH-S1485).
    policy_admin_ops_criteria_met_count: usize,
    /// Band-85 policies stand-smoke mode (PH-S1494).
    policy_stand_smoke_mode: bool,
    /// Policies stand-smoke criteria registry size (PH-S1494).
    policy_stand_smoke_criteria_total: usize,
    /// Policies stand-smoke criteria met count (PH-S1494).
    policy_stand_smoke_criteria_met_count: usize,
    /// Band-86 policies loc-audit aggregate mode (PH-S1504).
    policy_loc_audit_mode: bool,
    /// Policies loc-audit criteria registry size (PH-S1504).
    policy_loc_audit_criteria_total: usize,
    /// Policies loc-audit criteria met count (PH-S1504).
    policy_loc_audit_criteria_met_count: usize,
    /// Band-87 policies docs-canon mode (PH-S1514).
    policy_docs_canon_mode: bool,
    /// Policies docs-canon criteria registry size (PH-S1514).
    policy_docs_canon_criteria_total: usize,
    /// Policies docs-canon criteria met count (PH-S1514).
    policy_docs_canon_criteria_met_count: usize,
    /// Band-88 policies vision-sync mode (PH-S1524).
    policy_vision_sync_mode: bool,
    /// Policies vision-sync criteria registry size (PH-S1524).
    policy_vision_sync_criteria_total: usize,
    /// Policies vision-sync criteria met count (PH-S1524).
    policy_vision_sync_criteria_met_count: usize,
    /// Band-89 policies ratio-advisory mode (PH-S1534).
    policy_ratio_advisory_mode: bool,
    /// Policies ratio-advisory criteria registry size (PH-S1534).
    policy_ratio_advisory_criteria_total: usize,
    /// Policies ratio-advisory criteria met count (PH-S1534).
    policy_ratio_advisory_criteria_met_count: usize,
    /// Band-90 policies horizon-close mode (PH-S1544).
    policy_horizon_mode: bool,
    /// Policies horizon-close criteria registry size (PH-S1544).
    policy_horizon_criteria_total: usize,
    /// Policies horizon-close criteria met count (PH-S1544).
    policy_horizon_criteria_met_count: usize,
    /// Band-91 monitoring depth mode (PH-S1554).
    monitoring_mode: bool,
    /// Monitoring depth criteria registry size (PH-S1554).
    monitoring_criteria_total: usize,
    /// Monitoring depth criteria met count (PH-S1554).
    monitoring_criteria_met_count: usize,
    /// Band-92 monitoring store-wire mode (PH-S1564).
    monitoring_store_mode: bool,
    /// Monitoring store-wire criteria registry size (PH-S1564).
    monitoring_store_criteria_total: usize,
    /// Monitoring store-wire criteria met count (PH-S1564).
    monitoring_store_criteria_met_count: usize,
    /// Band-93 monitoring HTTP API contracts mode (PH-S1575).
    monitoring_api_mode: bool,
    /// Monitoring HTTP API criteria registry size (PH-S1575).
    monitoring_api_criteria_total: usize,
    /// Monitoring HTTP API criteria met count (PH-S1575).
    monitoring_api_criteria_met_count: usize,
    /// Band-94 monitoring admin/ops mode (PH-S1585).
    monitoring_admin_ops_mode: bool,
    /// Monitoring admin/ops criteria registry size (PH-S1585).
    monitoring_admin_ops_criteria_total: usize,
    /// Monitoring admin/ops criteria met count (PH-S1585).
    monitoring_admin_ops_criteria_met_count: usize,
    /// Band-95 monitoring stand-smoke mode (PH-S1594).
    monitoring_stand_smoke_mode: bool,
    /// Monitoring stand-smoke criteria registry size (PH-S1594).
    monitoring_stand_smoke_criteria_total: usize,
    /// Monitoring stand-smoke criteria met count (PH-S1594).
    monitoring_stand_smoke_criteria_met_count: usize,
    /// Band-96 monitoring loc-audit aggregate mode (PH-S1604).
    monitoring_loc_audit_mode: bool,
    /// Monitoring loc-audit criteria registry size (PH-S1604).
    monitoring_loc_audit_criteria_total: usize,
    /// Monitoring loc-audit criteria met count (PH-S1604).
    monitoring_loc_audit_criteria_met_count: usize,
    /// Band-97 monitoring docs-canon mode (PH-S1614).
    monitoring_docs_canon_mode: bool,
    /// Monitoring docs-canon criteria registry size (PH-S1614).
    monitoring_docs_canon_criteria_total: usize,
    /// Monitoring docs-canon criteria met count (PH-S1614).
    monitoring_docs_canon_criteria_met_count: usize,
    /// Band-98 monitoring vision-sync mode (PH-S1624).
    monitoring_vision_sync_mode: bool,
    /// Monitoring vision-sync criteria registry size (PH-S1624).
    monitoring_vision_sync_criteria_total: usize,
    /// Monitoring vision-sync criteria met count (PH-S1624).
    monitoring_vision_sync_criteria_met_count: usize,
    /// Band-99 monitoring ratio-advisory mode (PH-S1634).
    monitoring_ratio_advisory_mode: bool,
    /// Monitoring ratio-advisory criteria registry size (PH-S1634).
    monitoring_ratio_advisory_criteria_total: usize,
    /// Monitoring ratio-advisory criteria met count (PH-S1634).
    monitoring_ratio_advisory_criteria_met_count: usize,
    /// Band-100 monitoring horizon-close mode (PH-S1644).
    monitoring_horizon_mode: bool,
    /// Monitoring horizon-close criteria registry size (PH-S1644).
    monitoring_horizon_criteria_total: usize,
    /// Monitoring horizon-close criteria met count (PH-S1644).
    monitoring_horizon_criteria_met_count: usize,
    /// Band-101 ratio96 depth scaffold mode (PH-S1654).
    ratio96_mode: bool,
    /// Ratio96 criteria registry size (PH-S1654).
    ratio96_criteria_total: usize,
    /// Ratio96 criteria met count (PH-S1654).
    ratio96_criteria_met_count: usize,
    /// Band-104 ratio96 admin/ops glue mode (PH-S1684).
    ratio96_admin_ops_mode: bool,
    /// Ratio96 admin/ops criteria registry size (PH-S1684).
    ratio96_admin_ops_criteria_total: usize,
    /// Ratio96 admin/ops criteria met count (PH-S1684).
    ratio96_admin_ops_criteria_met_count: usize,
    /// Band-105 ratio96 stand smoke mode (PH-S1694).
    ratio96_stand_smoke_mode: bool,
    /// Ratio96 stand smoke criteria registry size (PH-S1694).
    ratio96_stand_smoke_criteria_total: usize,
    /// Ratio96 stand smoke criteria met count (PH-S1694).
    ratio96_stand_smoke_criteria_met_count: usize,
    /// Band-106 ratio96 loc-audit mode (PH-S1703).
    ratio96_loc_audit_mode: bool,
    /// Ratio96 loc-audit criteria registry size (PH-S1703).
    ratio96_loc_audit_criteria_total: usize,
    /// Ratio96 loc-audit criteria met count (PH-S1703).
    ratio96_loc_audit_criteria_met_count: usize,
    /// Band-107 ratio96 docs-canon mode (PH-S1714).
    ratio96_docs_canon_mode: bool,
    /// Ratio96 docs-canon criteria registry size (PH-S1714).
    ratio96_docs_canon_criteria_total: usize,
    /// Ratio96 docs-canon criteria met count (PH-S1714).
    ratio96_docs_canon_criteria_met_count: usize,
    /// Band-122 GPU limits mode (PH-S1862).
    gpu_limits_mode: bool,
    /// GPU limits criteria registry size (PH-S1862).
    gpu_limits_criteria_total: usize,
    /// GPU limits criteria met count (PH-S1862).
    gpu_limits_criteria_met_count: usize,
    /// Band-123 GPU limits API mode (PH-S1872).
    gpu_limits_api_mode: bool,
    /// GPU limits API criteria registry size (PH-S1872).
    gpu_limits_api_criteria_total: usize,
    /// GPU limits API criteria met count (PH-S1872).
    gpu_limits_api_criteria_met_count: usize,
    /// Band-124 GPU limits admin/ops glue mode (PH-S1884).
    gpu_limits_admin_ops_mode: bool,
    /// GPU limits admin/ops criteria registry size (PH-S1884).
    gpu_limits_admin_ops_criteria_total: usize,
    /// GPU limits admin/ops criteria met count (PH-S1884).
    gpu_limits_admin_ops_criteria_met_count: usize,
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

fn audit_depth_criteria_met(root: &Path) -> (usize, usize) {
    let total = audit_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in AUDIT_CRITERIA {
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

fn policy_depth_criteria_met(root: &Path) -> (usize, usize) {
    let total = policy_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in POLICY_CRITERIA {
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

fn monitoring_depth_criteria_met(root: &Path) -> (usize, usize) {
    let total = monitoring_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in MONITORING_CRITERIA {
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

fn monitoring_store_wire_criteria_met(root: &Path) -> (usize, usize) {
    let total = monitoring_store_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in MONITORING_STORE_CRITERIA {
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

fn monitoring_api_criteria_met(root: &Path) -> (usize, usize) {
    let total = monitoring_api_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in MONITORING_API_CRITERIA {
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

fn monitoring_admin_ops_criteria_met(root: &Path) -> (usize, usize) {
    let total = monitoring_admin_ops_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in MONITORING_ADMIN_OPS_CRITERIA {
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

fn monitoring_stand_smoke_criteria_met(root: &Path) -> (usize, usize) {
    let total = monitoring_stand_smoke_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in MONITORING_STAND_SMOKE_CRITERIA {
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

fn monitoring_loc_audit_criteria_met(root: &Path) -> (usize, usize) {
    let total = monitoring_loc_audit_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in MONITORING_LOC_AUDIT_CRITERIA {
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

fn monitoring_docs_canon_criteria_met(root: &Path) -> (usize, usize) {
    let total = monitoring_docs_canon_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in MONITORING_DOCS_CANON_CRITERIA {
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

fn monitoring_vision_sync_criteria_met(root: &Path) -> (usize, usize) {
    let total = monitoring_vision_sync_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in MONITORING_VISION_SYNC_CRITERIA {
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

fn monitoring_ratio_advisory_criteria_met(root: &Path) -> (usize, usize) {
    let total = monitoring_ratio_advisory_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in MONITORING_RATIO_ADVISORY_CRITERIA {
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

fn monitoring_horizon_criteria_met(root: &Path) -> (usize, usize) {
    let total = monitoring_horizon_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in MONITORING_HORIZON_CRITERIA {
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

fn ratio96_criteria_met(root: &Path) -> (usize, usize) {
    let total = ratio96_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in RATIO96_CRITERIA {
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

fn ratio96_admin_ops_criteria_met(root: &Path) -> (usize, usize) {
    let total = ratio96_admin_ops_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in RATIO96_ADMIN_OPS_CRITERIA {
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

fn ratio96_stand_smoke_criteria_met(root: &Path) -> (usize, usize) {
    let total = ratio96_stand_smoke_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in RATIO96_STAND_SMOKE_CRITERIA {
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

fn ratio96_loc_audit_criteria_met(root: &Path) -> (usize, usize) {
    let total = ratio96_loc_audit_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in RATIO96_LOC_AUDIT_CRITERIA {
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

fn ratio96_docs_canon_criteria_met(root: &Path) -> (usize, usize) {
    let total = ratio96_docs_canon_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in RATIO96_DOCS_CANON_CRITERIA {
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

fn gpu_limits_criteria_met(root: &Path) -> (usize, usize) {
    let total = gpu_limits_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in GPU_LIMITS_CRITERIA {
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

fn gpu_limits_api_criteria_met(root: &Path) -> (usize, usize) {
    let total = gpu_limits_api_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in GPU_LIMITS_API_CRITERIA {
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

fn gpu_limits_admin_ops_criteria_met(root: &Path) -> (usize, usize) {
    let total = gpu_limits_admin_ops_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in GPU_LIMITS_ADMIN_OPS_CRITERIA {
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

fn policy_store_wire_criteria_met(root: &Path) -> (usize, usize) {
    let total = policy_store_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in POLICY_STORE_CRITERIA {
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

fn policy_policy_api_criteria_met(root: &Path) -> (usize, usize) {
    let total = policy_api_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in POLICY_API_CRITERIA {
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

fn policy_policy_admin_ops_criteria_met(root: &Path) -> (usize, usize) {
    let total = policy_admin_ops_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in POLICY_ADMIN_OPS_CRITERIA {
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

fn policy_policy_stand_smoke_criteria_met(root: &Path) -> (usize, usize) {
    let total = policy_stand_smoke_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in POLICY_STAND_SMOKE_CRITERIA {
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

fn policy_policy_loc_audit_criteria_met(root: &Path) -> (usize, usize) {
    let total = policy_loc_audit_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in POLICY_LOC_AUDIT_CRITERIA {
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

fn policy_policy_docs_canon_criteria_met(root: &Path) -> (usize, usize) {
    let total = policy_docs_canon_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in POLICY_DOCS_CANON_CRITERIA {
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

fn policy_policy_vision_sync_criteria_met(root: &Path) -> (usize, usize) {
    let total = policy_vision_sync_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in POLICY_VISION_SYNC_CRITERIA {
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

fn policy_policy_ratio_advisory_criteria_met(root: &Path) -> (usize, usize) {
    let total = policy_ratio_advisory_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in POLICY_RATIO_ADVISORY_CRITERIA {
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

fn audit_store_wire_criteria_met(root: &Path) -> (usize, usize) {
    let total = audit_store_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in AUDIT_STORE_CRITERIA {
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

fn audit_audit_api_criteria_met(root: &Path) -> (usize, usize) {
    let total = audit_api_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in AUDIT_API_CRITERIA {
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

fn audit_audit_admin_ops_criteria_met(root: &Path) -> (usize, usize) {
    let total = audit_admin_ops_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in AUDIT_ADMIN_OPS_CRITERIA {
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

fn audit_audit_stand_smoke_criteria_met(root: &Path) -> (usize, usize) {
    let total = audit_stand_smoke_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in AUDIT_STAND_SMOKE_CRITERIA {
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

fn audit_audit_loc_audit_criteria_met(root: &Path) -> (usize, usize) {
    let total = audit_loc_audit_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in AUDIT_LOC_AUDIT_CRITERIA {
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

fn audit_audit_docs_canon_criteria_met(root: &Path) -> (usize, usize) {
    let total = audit_docs_canon_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in AUDIT_DOCS_CANON_CRITERIA {
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

fn audit_audit_vision_sync_criteria_met(root: &Path) -> (usize, usize) {
    let total = audit_vision_sync_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in AUDIT_VISION_SYNC_CRITERIA {
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

fn audit_audit_ratio_advisory_criteria_met(root: &Path) -> (usize, usize) {
    let total = audit_ratio_advisory_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in AUDIT_RATIO_ADVISORY_CRITERIA {
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

fn audit_audit_horizon_criteria_met(root: &Path) -> (usize, usize) {
    let total = audit_horizon_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in AUDIT_HORIZON_CRITERIA {
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

fn audit_sso_horizon_criteria_met(root: &Path) -> (usize, usize) {
    let total = sso_horizon_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in SSO_HORIZON_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(text) = fs::read_to_string(&path) {
                if text.contains(marker) {
                    met += 1;
                }
            }
        }
    }
    (met, total)
}

fn audit_policy_horizon_criteria_met(root: &Path) -> (usize, usize) {
    let total = policy_horizon_criteria_total();
    let mut met = 0usize;
    for (_, marker, rel) in POLICY_HORIZON_CRITERIA {
        let path = root.join(rel);
        if path.is_file() {
            if let Ok(text) = fs::read_to_string(&path) {
                if text.contains(marker) {
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
            "--sso-horizon" => config.sso_horizon = true,
            "--audit-store" => config.audit_store = true,
            "--audit-api" => config.audit_api = true,
            "--audit-admin-ops" => config.audit_admin_ops = true,
            "--audit-stand-smoke" => config.audit_stand_smoke = true,
            "--audit-loc-audit" => config.audit_loc_audit = true,
            "--audit-docs-canon" => config.audit_docs_canon = true,
            "--audit-vision-sync" => config.audit_vision_sync = true,
            "--audit-ratio-advisory" => config.audit_ratio_advisory = true,
            "--audit-horizon" => config.audit_horizon = true,
            "--audit" => config.audit = true,
            "--policy-store" => config.policy_store = true,
            "--policy-api" => config.policy_api = true,
            "--policy-admin-ops" => config.policy_admin_ops = true,
            "--policy-stand-smoke" => config.policy_stand_smoke = true,
            "--policy-loc-audit" => config.policy_loc_audit = true,
            "--policy-docs-canon" => config.policy_docs_canon = true,
            "--policy-vision-sync" => config.policy_vision_sync = true,
            "--policy-ratio-advisory" => config.policy_ratio_advisory = true,
            "--policy-horizon" => config.policy_horizon = true,
            "--policy" => config.policy = true,
            "--monitoring" => config.monitoring = true,
            "--monitoring-store" => config.monitoring_store = true,
            "--monitoring-api" => config.monitoring_api = true,
            "--monitoring-admin-ops" => config.monitoring_admin_ops = true,
            "--monitoring-stand-smoke" => config.monitoring_stand_smoke = true,
            "--monitoring-loc-audit" => config.monitoring_loc_audit = true,
            "--monitoring-docs-canon" => config.monitoring_docs_canon = true,
            "--monitoring-vision-sync" => config.monitoring_vision_sync = true,
            "--monitoring-ratio-advisory" => config.monitoring_ratio_advisory = true,
            "--monitoring-horizon" => config.monitoring_horizon = true,
            "--ratio96" => config.ratio96 = true,
            "--ratio96-admin-ops" => config.ratio96_admin_ops = true,
            "--ratio96-stand-smoke" => config.ratio96_stand_smoke = true,
            "--ratio96-loc-audit" => config.ratio96_loc_audit = true,
            "--ratio96-docs-canon" => config.ratio96_docs_canon = true,
            "--gpu-limits" => config.gpu_limits = true,
            "--gpu-limits-api" => config.gpu_limits_api = true,
            "--gpu-limits-admin-ops" => config.gpu_limits_admin_ops = true,
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
           --sso-horizon               band-70 SSO horizon-close fields (PH-S1344)\n\
           --audit                     band-71 audit depth scaffold fields (PH-S1354)\n\
           --audit-store               band-72 audit store-wire fields (PH-S1364)\n\
           --audit-api                 band-73 audit HTTP API contracts fields (PH-S1375)\n\
           --audit-admin-ops           band-74 audit admin/ops fields (PH-S1385)\n\
           --audit-stand-smoke         band-75 audit stand-smoke fields (PH-S1394)\n\
           --audit-loc-audit           band-76 audit loc-audit aggregate fields (PH-S1404)\n\
           --audit-docs-canon          band-77 audit docs-canon fields (PH-S1414)\n\
           --audit-vision-sync         band-78 audit vision-sync fields (PH-S1424)\n\
           --audit-ratio-advisory      band-79 audit ratio-advisory fields (PH-S1434)\n\
           --audit-horizon             band-80 audit horizon-close fields (PH-S1444)\n\
           --policy                    band-81 policies depth scaffold fields (PH-S1454)\n\
           --policy-store              band-82 policies store-wire fields (PH-S1464)\n\
           --policy-api                band-83 policies HTTP API contracts fields (PH-S1475)\n\
           --policy-admin-ops          band-84 policies admin/ops fields (PH-S1485)\n\
           --policy-stand-smoke        band-85 policies stand-smoke fields (PH-S1494)\n\
           --policy-loc-audit          band-86 policies loc-audit aggregate fields (PH-S1504)\n\
           --policy-docs-canon         band-87 policies docs-canon fields (PH-S1514)\n\
           --policy-vision-sync        band-88 policies vision-sync fields (PH-S1524)\n\
           --policy-ratio-advisory     band-89 policies ratio-advisory fields (PH-S1534)\n\
           --policy-horizon            band-90 policies horizon-close fields (PH-S1544)\n\
           --monitoring                band-91 monitoring depth scaffold fields (PH-S1554)\n\
           --monitoring-store          band-92 monitoring store-wire fields (PH-S1564)\n\
           --monitoring-api            band-93 monitoring HTTP API contracts fields (PH-S1575)\n\
           --monitoring-admin-ops      band-94 monitoring admin/ops fields (PH-S1585)\n\
           --monitoring-stand-smoke    band-95 monitoring stand-smoke fields (PH-S1594)\n\
           --monitoring-docs-canon      band-97 monitoring docs-canon fields (PH-S1614)\n\
           --monitoring-vision-sync    band-98 monitoring vision-sync fields (PH-S1624)\n\
           --monitoring-ratio-advisory  band-99 monitoring ratio-advisory fields (PH-S1634)\n\
            --monitoring-horizon        band-100 monitoring horizon-close fields (PH-S1644)\n\
            --ratio96                   band-101 ratio96 depth scaffold fields (PH-S1654)\n\
            --ratio96-admin-ops         band-104 ratio96 admin/ops glue fields (PH-S1684)\n\
            --ratio96-stand-smoke        band-105 ratio96 stand smoke fields (PH-S1694)\n\
            --ratio96-loc-audit          band-106 ratio96 loc-audit fields (PH-S1703)\n\
            --ratio96-docs-canon         band-107 ratio96 docs-canon fields (PH-S1714)\n            --gpu-limits                band-122 GPU limits store/wire fields (PH-S1862)\n            --gpu-limits-api            band-123 GPU limits API fields (PH-S1872)\n            --gpu-limits-admin-ops       band-124 GPU limits admin/ops glue fields (PH-S1884)\n\n            --monitoring-loc-audit      band-96 monitoring loc-audit aggregate fields (PH-S1604)\n\
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
    let (sso_horizon_criteria_met_count, sso_horizon_criteria_total_count) = if config.sso_horizon {
        audit_sso_horizon_criteria_met(root)
    } else {
        (0, sso_horizon_criteria_total())
    };
    if config.sso_horizon {
        notes.push("PH-S1344: sso_horizon_mode — aggregate band 61–69 SSO slices (phase B close)");
        notes.push("PH-S1348: band 70 SSO horizon close — criteria met vs registry");
    }
    let (audit_criteria_met_count, audit_criteria_total_count) = if config.audit {
        audit_depth_criteria_met(root)
    } else {
        (0, audit_criteria_total())
    };
    if config.audit {
        notes.push(
            "PH-S1354: audit_mode — audit depth scaffold (POOLAI_AUDIT_STORE + event metadata stub)",
        );
        notes.push("PH-S1358: band 71 audit depth — criteria met vs registry");
    }
    let (audit_store_criteria_met_count, audit_store_criteria_total_count) = if config.audit_store {
        audit_store_wire_criteria_met(root)
    } else {
        (0, audit_store_criteria_total())
    };
    if config.audit_store {
        notes.push("PH-S1364: audit_store_mode — durable path wire stub (POOLAI_AUDIT_DATA_DIR)");
        notes.push("PH-S1368: band 72 audit store wire — criteria met vs registry");
    }

    let (audit_api_criteria_met_count, audit_api_criteria_total_count) = if config.audit_api {
        audit_audit_api_criteria_met(root)
    } else {
        (0, audit_api_criteria_total())
    };
    if config.audit_api {
        notes.push("PH-S1375: audit_api_mode — query/store HTTP contracts + field fixtures");
        notes.push("PH-S1378: band 73 audit API contracts — criteria met vs registry");
    }

    let (audit_admin_ops_criteria_met_count, audit_admin_ops_criteria_total_count) =
        if config.audit_admin_ops {
            audit_audit_admin_ops_criteria_met(root)
        } else {
            (0, audit_admin_ops_criteria_total())
        };
    if config.audit_admin_ops {
        notes.push("PH-S1385: audit_admin_ops_mode — store strip / query refresh / verify hooks");
        notes.push("PH-S1388: band 74 audit admin/ops glue — criteria met vs registry");
    }

    let (audit_stand_smoke_criteria_met_count, audit_stand_smoke_criteria_total_count) =
        if config.audit_stand_smoke {
            audit_audit_stand_smoke_criteria_met(root)
        } else {
            (0, audit_stand_smoke_criteria_total())
        };
    if config.audit_stand_smoke {
        notes.push(
            "PH-S1394: audit_stand_smoke_mode — live store/events/validate fixtures + verify hooks",
        );
        notes.push("PH-S1398: band 75 audit stand-smoke — criteria met vs registry");
    }

    let (audit_loc_audit_criteria_met_count, audit_loc_audit_criteria_total_count) =
        if config.audit_loc_audit {
            audit_audit_loc_audit_criteria_met(root)
        } else {
            (0, audit_loc_audit_criteria_total())
        };
    if config.audit_loc_audit {
        notes.push(
            "PH-S1404: audit_loc_audit_mode — aggregate band 71–75 --audit* loc-audit slices",
        );
        notes.push("PH-S1408: band 76 audit loc-audit — criteria met vs registry");
    }

    let (audit_docs_canon_criteria_met_count, audit_docs_canon_criteria_total_count) =
        if config.audit_docs_canon {
            audit_audit_docs_canon_criteria_met(root)
        } else {
            (0, audit_docs_canon_criteria_total())
        };
    if config.audit_docs_canon {
        notes.push("PH-S1414: audit_docs_canon_mode — aggregate band 71–76 AUDIT_*.md canon docs");
        notes.push("PH-S1418: band 77 audit docs-canon — criteria met vs registry");
    }

    let (audit_vision_sync_criteria_met_count, audit_vision_sync_criteria_total_count) =
        if config.audit_vision_sync {
            audit_audit_vision_sync_criteria_met(root)
        } else {
            (0, audit_vision_sync_criteria_total())
        };
    if config.audit_vision_sync {
        notes.push("PH-S1424: audit_vision_sync_mode — aggregate docs/vision/* + AUDIT_DOCS_CANON");
        notes.push("PH-S1428: band 78 audit vision-sync — criteria met vs registry");
    }

    let (audit_ratio_advisory_criteria_met_count, audit_ratio_advisory_criteria_total_count) =
        if config.audit_ratio_advisory {
            audit_audit_ratio_advisory_criteria_met(root)
        } else {
            (0, audit_ratio_advisory_criteria_total())
        };
    if config.audit_ratio_advisory {
        notes.push(
            "PH-S1434: audit_ratio_advisory_mode — aggregate prior --audit* + vision-sync slices",
        );
        notes.push("PH-S1438: band 79 audit ratio-advisory — criteria met vs registry");
    }

    let (audit_horizon_criteria_met_count, audit_horizon_criteria_total_count) =
        if config.audit_horizon {
            audit_audit_horizon_criteria_met(root)
        } else {
            (0, audit_horizon_criteria_total())
        };
    if config.audit_horizon {
        notes.push(
            "PH-S1444: audit_horizon_mode — aggregate band 71–79 Audit slices (phase C close)",
        );
        notes.push("PH-S1448: band 80 audit horizon — criteria met vs registry");
    }
    let (policy_criteria_met_count, policy_criteria_total_count) = if config.policy {
        policy_depth_criteria_met(root)
    } else {
        (0, policy_criteria_total())
    };
    if config.policy {
        notes.push(
            "PH-S1454: policy_mode — policies depth scaffold (POOLAI_POLICY_STORE + field stub)",
        );
        notes.push("PH-S1458: band 81 policies depth — criteria met vs registry");
    }
    let (policy_store_criteria_met_count, policy_store_criteria_total_count) =
        if config.policy_store {
            policy_store_wire_criteria_met(root)
        } else {
            (0, policy_store_criteria_total())
        };
    if config.policy_store {
        notes.push("PH-S1464: policy_store_mode — durable path wire stub (POOLAI_POLICY_DATA_DIR)");
        notes.push("PH-S1468: band 82 policies store wire — criteria met vs registry");
    }
    let (policy_api_criteria_met_count, policy_api_criteria_total_count) = if config.policy_api {
        policy_policy_api_criteria_met(root)
    } else {
        (0, policy_api_criteria_total())
    };
    if config.policy_api {
        notes.push("PH-S1475: policy_api_mode — query/store HTTP contracts + field fixtures");
        notes.push("PH-S1478: band 83 policies API contracts — criteria met vs registry");
    }
    let (policy_admin_ops_criteria_met_count, policy_admin_ops_criteria_total_count) =
        if config.policy_admin_ops {
            policy_policy_admin_ops_criteria_met(root)
        } else {
            (0, policy_admin_ops_criteria_total())
        };
    if config.policy_admin_ops {
        notes.push("PH-S1485: policy_admin_ops_mode — store strip / policy refresh / verify hooks");
        notes.push("PH-S1488: band 84 policies admin/ops glue — criteria met vs registry");
    }
    let (policy_stand_smoke_criteria_met_count, policy_stand_smoke_criteria_total_count) =
        if config.policy_stand_smoke {
            policy_policy_stand_smoke_criteria_met(root)
        } else {
            (0, policy_stand_smoke_criteria_total())
        };
    if config.policy_stand_smoke {
        notes.push(
            "PH-S1494: policy_stand_smoke_mode — live store / policies query / validate fixtures",
        );
        notes.push("PH-S1498: band 85 policies stand smoke — criteria met vs registry");
    }

    let (policy_loc_audit_criteria_met_count, policy_loc_audit_criteria_total_count) =
        if config.policy_loc_audit {
            policy_policy_loc_audit_criteria_met(root)
        } else {
            (0, policy_loc_audit_criteria_total())
        };
    if config.policy_loc_audit {
        notes.push(
            "PH-S1504: policy_loc_audit_mode — aggregate band 81–85 --policy* loc-audit slices",
        );
        notes.push("PH-S1508: band 86 policies loc-audit — criteria met vs registry");
    }

    let (policy_docs_canon_criteria_met_count, policy_docs_canon_criteria_total_count) =
        if config.policy_docs_canon {
            policy_policy_docs_canon_criteria_met(root)
        } else {
            (0, policy_docs_canon_criteria_total())
        };
    if config.policy_docs_canon {
        notes.push(
            "PH-S1514: policy_docs_canon_mode — aggregate band 81–86 POLICIES_*.md canon docs",
        );
        notes.push("PH-S1518: band 87 policies docs-canon — criteria met vs registry");
    }

    let (policy_vision_sync_criteria_met_count, policy_vision_sync_criteria_total_count) =
        if config.policy_vision_sync {
            policy_policy_vision_sync_criteria_met(root)
        } else {
            (0, policy_vision_sync_criteria_total())
        };
    if config.policy_vision_sync {
        notes.push(
            "PH-S1524: policy_vision_sync_mode — aggregate docs/vision/* + POLICIES_DOCS_CANON",
        );
        notes.push("PH-S1528: band 88 policies vision-sync — criteria met vs registry");
    }

    let (policy_ratio_advisory_criteria_met_count, policy_ratio_advisory_criteria_total_count) =
        if config.policy_ratio_advisory {
            policy_policy_ratio_advisory_criteria_met(root)
        } else {
            (0, policy_ratio_advisory_criteria_total())
        };
    if config.policy_ratio_advisory {
        notes.push(
            "PH-S1534: policy_ratio_advisory_mode — aggregate prior --policy* + vision-sync slices",
        );
        notes.push("PH-S1538: band 89 policies ratio-advisory — criteria met vs registry");
    }

    let (policy_horizon_criteria_met_count, policy_horizon_criteria_total_count) =
        if config.policy_horizon {
            audit_policy_horizon_criteria_met(root)
        } else {
            (0, policy_horizon_criteria_total())
        };
    if config.policy_horizon {
        notes.push(
            "PH-S1544: policy_horizon_mode — aggregate band 81–89 policy slices (phase D close)",
        );
        notes.push("PH-S1548: band 90 policies horizon close — criteria met vs registry");
    }
    let (monitoring_criteria_met_count, monitoring_criteria_total_count) = if config.monitoring {
        monitoring_depth_criteria_met(root)
    } else {
        (0, monitoring_criteria_total())
    };
    if config.monitoring {
        notes.push(
            "PH-S1554: monitoring_mode — monitoring depth scaffold (POOLAI_MONITORING_DATA_DIR + field stub)",
        );
        notes.push("PH-S1558: band 91 monitoring depth — criteria met vs registry");
    }
    let (monitoring_store_criteria_met_count, monitoring_store_criteria_total_count) =
        if config.monitoring_store {
            monitoring_store_wire_criteria_met(root)
        } else {
            (0, monitoring_store_criteria_total())
        };
    if config.monitoring_store {
        notes.push(
            "PH-S1564: monitoring_store_mode — durable path wire stub (POOLAI_MONITORING_STORE + DATA_DIR)",
        );
        notes.push("PH-S1568: band 92 monitoring store wire — criteria met vs registry");
    }
    let (monitoring_api_criteria_met_count, monitoring_api_criteria_total_count) =
        if config.monitoring_api {
            monitoring_api_criteria_met(root)
        } else {
            (0, monitoring_api_criteria_total())
        };
    if config.monitoring_api {
        notes.push("PH-S1575: monitoring_api_mode — query/store HTTP contracts + field fixtures");
        notes.push("PH-S1578: band 93 monitoring API contracts — criteria met vs registry");
    }
    let (monitoring_admin_ops_criteria_met_count, monitoring_admin_ops_criteria_total_count) =
        if config.monitoring_admin_ops {
            monitoring_admin_ops_criteria_met(root)
        } else {
            (0, monitoring_admin_ops_criteria_total())
        };
    if config.monitoring_admin_ops {
        notes.push(
            "PH-S1585: monitoring_admin_ops_mode — store strip / monitoring refresh / verify hooks",
        );
        notes.push("PH-S1588: band 94 monitoring admin/ops — criteria met vs registry");
    }
    let (monitoring_stand_smoke_criteria_met_count, monitoring_stand_smoke_criteria_total_count) =
        if config.monitoring_stand_smoke {
            monitoring_stand_smoke_criteria_met(root)
        } else {
            (0, monitoring_stand_smoke_criteria_total())
        };
    if config.monitoring_stand_smoke {
        notes.push(
            "PH-S1594: monitoring_stand_smoke_mode — live store / alerts query / validate fixtures",
        );
        notes.push("PH-S1598: band 95 monitoring stand smoke — criteria met vs registry");
    }
    let (monitoring_loc_audit_criteria_met_count, monitoring_loc_audit_criteria_total_count) =
        if config.monitoring_loc_audit {
            monitoring_loc_audit_criteria_met(root)
        } else {
            (0, monitoring_loc_audit_criteria_total())
        };
    if config.monitoring_loc_audit {
        notes.push(
            "PH-S1604: monitoring_loc_audit_mode — aggregate band 91–95 --monitoring* loc-audit slices",
        );
        notes.push("PH-S1608: band 96 monitoring loc-audit — criteria met vs registry");
    }

    let (monitoring_docs_canon_criteria_met_count, monitoring_docs_canon_criteria_total_count) =
        if config.monitoring_docs_canon {
            monitoring_docs_canon_criteria_met(root)
        } else {
            (0, monitoring_docs_canon_criteria_total())
        };
    if config.monitoring_docs_canon {
        notes.push(
            "PH-S1614: monitoring_docs_canon_mode — aggregate band 91–95 MONITORING_*.md canon docs",
        );
        notes.push("PH-S1618: band 97 monitoring docs-canon — criteria met vs registry");
    }

    let (monitoring_vision_sync_criteria_met_count, monitoring_vision_sync_criteria_total_count) =
        if config.monitoring_vision_sync {
            monitoring_vision_sync_criteria_met(root)
        } else {
            (0, monitoring_vision_sync_criteria_total())
        };
    if config.monitoring_vision_sync {
        notes.push(
            "PH-S1624: monitoring_vision_sync_mode — aggregate docs/vision/* + MONITORING_DOCS_CANON",
        );
        notes.push("PH-S1628: band 98 monitoring vision-sync — criteria met vs registry");
    }

    let (
        monitoring_ratio_advisory_criteria_met_count,
        monitoring_ratio_advisory_criteria_total_count,
    ) = if config.monitoring_ratio_advisory {
        monitoring_ratio_advisory_criteria_met(root)
    } else {
        (0, monitoring_ratio_advisory_criteria_total())
    };
    if config.monitoring_ratio_advisory {
        notes.push(
            "PH-S1634: monitoring_ratio_advisory_mode — aggregate rust_ratio.json + ratio strategy",
        );
        notes.push("PH-S1638: band 99 monitoring ratio-advisory — criteria met vs registry");
    }

    let (monitoring_horizon_criteria_met_count, monitoring_horizon_criteria_total_count) =
        if config.monitoring_horizon {
            monitoring_horizon_criteria_met(root)
        } else {
            (0, monitoring_horizon_criteria_total())
        };
    if config.monitoring_horizon {
        notes.push(
            "PH-S1644: monitoring_horizon_mode — aggregate band 91–99 monitoring slices (phase E close)",
        );
        notes.push("PH-S1648: band 100 monitoring horizon close — criteria met vs registry");
    }

    let (ratio96_criteria_met_count, ratio96_criteria_total_count) = if config.ratio96 {
        ratio96_criteria_met(root)
    } else {
        (0, ratio96_criteria_total())
    };
    if config.ratio96 {
        notes.push(
            "PH-S1654: ratio96_mode — aggregate phase-F Ratio96 slices (stretch 96% depth scaffold)",
        );
        notes.push("PH-S1658: band 101 ratio96 depth scaffold — criteria met vs registry");
    }

    let (ratio96_admin_ops_criteria_met_count, ratio96_admin_ops_criteria_total_count) =
        if config.ratio96_admin_ops {
            ratio96_admin_ops_criteria_met(root)
        } else {
            (0, ratio96_admin_ops_criteria_total())
        };
    if config.ratio96_admin_ops {
        notes.push(
            "PH-S1684: ratio96_admin_ops_mode — dashboard store strip / refresh ops glue / verify hooks",
        );
        notes.push("PH-S1688: band 104 ratio96 admin/ops glue — criteria met vs registry");
    }

    let (ratio96_stand_smoke_criteria_met_count, ratio96_stand_smoke_criteria_total_count) =
        if config.ratio96_stand_smoke {
            ratio96_stand_smoke_criteria_met(root)
        } else {
            (0, ratio96_stand_smoke_criteria_total())
        };
    if config.ratio96_stand_smoke {
        notes.push(
            "PH-S1694: ratio96_stand_smoke_mode — live store/query/fixtures smoke / verify hooks",
        );
        notes.push("PH-S1698: band 105 ratio96 stand smoke — criteria met vs registry");
    }

    let (ratio96_loc_audit_criteria_met_count, ratio96_loc_audit_criteria_total_count) =
        if config.ratio96_loc_audit {
            ratio96_loc_audit_criteria_met(root)
        } else {
            (0, ratio96_loc_audit_criteria_total())
        };
    if config.ratio96_loc_audit {
        notes.push(
            "PH-S1703: ratio96_loc_audit_mode — loc-audit + migration advisory / verify hooks",
        );
        notes.push("PH-S1708: band 106 ratio96 loc-audit — criteria met vs registry");
    }

    let (ratio96_docs_canon_criteria_met_count, ratio96_docs_canon_criteria_total_count) =
        if config.ratio96_docs_canon {
            ratio96_docs_canon_criteria_met(root)
        } else {
            (0, ratio96_docs_canon_criteria_total())
        };
    if config.ratio96_docs_canon {
        notes.push(
            "PH-S1714: ratio96_docs_canon_mode — aggregate band 101–106 RATIO96_*.md canon docs",
        );
        notes.push("PH-S1718: band 107 ratio96 docs canon — criteria met vs registry");
    }

    let (gpu_limits_criteria_met_count, gpu_limits_criteria_total_count) = if config.gpu_limits {
        gpu_limits_criteria_met(root)
    } else {
        (0, gpu_limits_criteria_total())
    };
    if config.gpu_limits {
        notes.push(
            "PH-S1862: gpu_limits_mode — GPULimits store/wire + durable gpu_limits.json (band 122)",
        );
        notes.push("PH-S1866: band 122 GPU limits — criteria met vs registry");
    }

    let (gpu_limits_api_criteria_met_count, gpu_limits_api_criteria_total_count) =
        if config.gpu_limits_api {
            gpu_limits_api_criteria_met(root)
        } else {
            (0, gpu_limits_api_criteria_total())
        };
    if config.gpu_limits_api {
        notes.push(
            "PH-S1872: gpu_limits_api_mode — GPULimits API contracts + GET /api/v1/gpu-limits (band 123)",
        );
        notes.push("PH-S1876: band 123 GPU limits API — criteria met vs registry");
    }

    let (gpu_limits_admin_ops_criteria_met_count, gpu_limits_admin_ops_criteria_total_count) =
        if config.gpu_limits_admin_ops {
            gpu_limits_admin_ops_criteria_met(root)
        } else {
            (0, gpu_limits_admin_ops_criteria_total())
        };
    if config.gpu_limits_admin_ops {
        notes.push(
            "PH-S1884: gpu_limits_admin_ops_mode — dashboard store strip / refresh ops glue / verify hooks (band 124)",
        );
        notes.push("PH-S1888: band 124 GPU limits admin/ops — criteria met vs registry");
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
        sso_horizon_mode: config.sso_horizon,
        sso_horizon_criteria_total: sso_horizon_criteria_total_count,
        sso_horizon_criteria_met_count,
        audit_mode: config.audit,
        audit_criteria_total: audit_criteria_total_count,
        audit_criteria_met_count,
        audit_store_mode: config.audit_store,
        audit_store_criteria_total: audit_store_criteria_total_count,
        audit_store_criteria_met_count,
        audit_api_mode: config.audit_api,
        audit_api_criteria_total: audit_api_criteria_total_count,
        audit_api_criteria_met_count,
        audit_admin_ops_mode: config.audit_admin_ops,
        audit_admin_ops_criteria_total: audit_admin_ops_criteria_total_count,
        audit_admin_ops_criteria_met_count,
        audit_stand_smoke_mode: config.audit_stand_smoke,
        audit_stand_smoke_criteria_total: audit_stand_smoke_criteria_total_count,
        audit_stand_smoke_criteria_met_count,
        audit_loc_audit_mode: config.audit_loc_audit,
        audit_loc_audit_criteria_total: audit_loc_audit_criteria_total_count,
        audit_loc_audit_criteria_met_count,
        audit_docs_canon_mode: config.audit_docs_canon,
        audit_docs_canon_criteria_total: audit_docs_canon_criteria_total_count,
        audit_docs_canon_criteria_met_count,
        audit_vision_sync_mode: config.audit_vision_sync,
        audit_vision_sync_criteria_total: audit_vision_sync_criteria_total_count,
        audit_vision_sync_criteria_met_count,
        audit_ratio_advisory_mode: config.audit_ratio_advisory,
        audit_ratio_advisory_criteria_total: audit_ratio_advisory_criteria_total_count,
        audit_ratio_advisory_criteria_met_count,
        audit_horizon_mode: config.audit_horizon,
        audit_horizon_criteria_total: audit_horizon_criteria_total_count,
        audit_horizon_criteria_met_count,
        policy_mode: config.policy,
        policy_criteria_total: policy_criteria_total_count,
        policy_criteria_met_count,
        policy_store_mode: config.policy_store,
        policy_store_criteria_total: policy_store_criteria_total_count,
        policy_store_criteria_met_count,
        policy_api_mode: config.policy_api,
        policy_api_criteria_total: policy_api_criteria_total_count,
        policy_api_criteria_met_count,
        policy_admin_ops_mode: config.policy_admin_ops,
        policy_admin_ops_criteria_total: policy_admin_ops_criteria_total_count,
        policy_admin_ops_criteria_met_count,
        policy_stand_smoke_mode: config.policy_stand_smoke,
        policy_stand_smoke_criteria_total: policy_stand_smoke_criteria_total_count,
        policy_stand_smoke_criteria_met_count,
        policy_loc_audit_mode: config.policy_loc_audit,
        policy_loc_audit_criteria_total: policy_loc_audit_criteria_total_count,
        policy_loc_audit_criteria_met_count,
        policy_docs_canon_mode: config.policy_docs_canon,
        policy_docs_canon_criteria_total: policy_docs_canon_criteria_total_count,
        policy_docs_canon_criteria_met_count,
        policy_vision_sync_mode: config.policy_vision_sync,
        policy_vision_sync_criteria_total: policy_vision_sync_criteria_total_count,
        policy_vision_sync_criteria_met_count,
        policy_ratio_advisory_mode: config.policy_ratio_advisory,
        policy_ratio_advisory_criteria_total: policy_ratio_advisory_criteria_total_count,
        policy_ratio_advisory_criteria_met_count,
        policy_horizon_mode: config.policy_horizon,
        policy_horizon_criteria_total: policy_horizon_criteria_total_count,
        policy_horizon_criteria_met_count,
        monitoring_mode: config.monitoring,
        monitoring_criteria_total: monitoring_criteria_total_count,
        monitoring_criteria_met_count,
        monitoring_store_mode: config.monitoring_store,
        monitoring_store_criteria_total: monitoring_store_criteria_total_count,
        monitoring_store_criteria_met_count,
        monitoring_api_mode: config.monitoring_api,
        monitoring_api_criteria_total: monitoring_api_criteria_total_count,
        monitoring_api_criteria_met_count,
        monitoring_admin_ops_mode: config.monitoring_admin_ops,
        monitoring_admin_ops_criteria_total: monitoring_admin_ops_criteria_total_count,
        monitoring_admin_ops_criteria_met_count,
        monitoring_stand_smoke_mode: config.monitoring_stand_smoke,
        monitoring_stand_smoke_criteria_total: monitoring_stand_smoke_criteria_total_count,
        monitoring_stand_smoke_criteria_met_count,
        monitoring_loc_audit_mode: config.monitoring_loc_audit,
        monitoring_loc_audit_criteria_total: monitoring_loc_audit_criteria_total_count,
        monitoring_loc_audit_criteria_met_count,
        monitoring_docs_canon_mode: config.monitoring_docs_canon,
        monitoring_docs_canon_criteria_total: monitoring_docs_canon_criteria_total_count,
        monitoring_docs_canon_criteria_met_count,
        monitoring_vision_sync_mode: config.monitoring_vision_sync,
        monitoring_vision_sync_criteria_total: monitoring_vision_sync_criteria_total_count,
        monitoring_vision_sync_criteria_met_count,
        monitoring_ratio_advisory_mode: config.monitoring_ratio_advisory,
        monitoring_ratio_advisory_criteria_total: monitoring_ratio_advisory_criteria_total_count,
        monitoring_ratio_advisory_criteria_met_count,
        monitoring_horizon_mode: config.monitoring_horizon,
        monitoring_horizon_criteria_total: monitoring_horizon_criteria_total_count,
        monitoring_horizon_criteria_met_count,
        ratio96_mode: config.ratio96,
        ratio96_criteria_total: ratio96_criteria_total_count,
        ratio96_criteria_met_count,
        ratio96_admin_ops_mode: config.ratio96_admin_ops,
        ratio96_admin_ops_criteria_total: ratio96_admin_ops_criteria_total_count,
        ratio96_admin_ops_criteria_met_count,
        ratio96_stand_smoke_mode: config.ratio96_stand_smoke,
        ratio96_stand_smoke_criteria_total: ratio96_stand_smoke_criteria_total_count,
        ratio96_stand_smoke_criteria_met_count,
        ratio96_loc_audit_mode: config.ratio96_loc_audit,
        ratio96_loc_audit_criteria_total: ratio96_loc_audit_criteria_total_count,
        ratio96_loc_audit_criteria_met_count,
        ratio96_docs_canon_mode: config.ratio96_docs_canon,
        ratio96_docs_canon_criteria_total: ratio96_docs_canon_criteria_total_count,
        ratio96_docs_canon_criteria_met_count,
        gpu_limits_mode: config.gpu_limits,
        gpu_limits_criteria_total: gpu_limits_criteria_total_count,
        gpu_limits_criteria_met_count,
        gpu_limits_api_mode: config.gpu_limits_api,
        gpu_limits_api_criteria_total: gpu_limits_api_criteria_total_count,
        gpu_limits_api_criteria_met_count,
        gpu_limits_admin_ops_mode: config.gpu_limits_admin_ops,
        gpu_limits_admin_ops_criteria_total: gpu_limits_admin_ops_criteria_total_count,
        gpu_limits_admin_ops_criteria_met_count,
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
    if report.sso_horizon_mode {
        println!("  sso_horizon:           true (PH-S1344 band 70)");
        println!(
            "  sso_horizon_criteria: {}/{} met",
            report.sso_horizon_criteria_met_count, report.sso_horizon_criteria_total
        );
        println!("  sso_horizon_cases: {}", SSO_HORIZON_CASES.join(", "));
    }
    if report.audit_mode {
        println!("  audit:                 true (PH-S1354 band 71)");
        println!(
            "  audit_criteria:        {}/{} met",
            report.audit_criteria_met_count, report.audit_criteria_total
        );
        println!("  audit_cases:           {}", AUDIT_CASES.join(", "));
    }
    if report.audit_store_mode {
        println!("  audit_store:           true (PH-S1364 band 72)");
        println!(
            "  audit_store_criteria:  {}/{} met",
            report.audit_store_criteria_met_count, report.audit_store_criteria_total
        );
        println!("  audit_store_cases:     {}", AUDIT_STORE_CASES.join(", "));
    }
    if report.audit_api_mode {
        println!("  audit_api:             true (PH-S1375 band 73)");
        println!(
            "  audit_api_criteria:    {}/{} met",
            report.audit_api_criteria_met_count, report.audit_api_criteria_total
        );
        println!("  audit_api_cases:       {}", AUDIT_API_CASES.join(", "));
    }
    if report.audit_admin_ops_mode {
        println!("  audit_admin_ops:       true (PH-S1385 band 74)");
        println!(
            "  audit_admin_ops_criteria: {}/{} met",
            report.audit_admin_ops_criteria_met_count, report.audit_admin_ops_criteria_total
        );
        println!(
            "  audit_admin_ops_cases: {}",
            AUDIT_ADMIN_OPS_CASES.join(", ")
        );
    }
    if report.audit_stand_smoke_mode {
        println!("  audit_stand_smoke:     true (PH-S1394 band 75)");
        println!(
            "  audit_stand_smoke_criteria: {}/{} met",
            report.audit_stand_smoke_criteria_met_count, report.audit_stand_smoke_criteria_total
        );
        println!(
            "  audit_stand_smoke_cases: {}",
            AUDIT_STAND_SMOKE_CASES.join(", ")
        );
    }
    if report.audit_loc_audit_mode {
        println!("  audit_loc_audit:       true (PH-S1404 band 76)");
        println!(
            "  audit_loc_audit_criteria: {}/{} met",
            report.audit_loc_audit_criteria_met_count, report.audit_loc_audit_criteria_total
        );
        println!(
            "  audit_loc_audit_cases: {}",
            AUDIT_LOC_AUDIT_CASES.join(", ")
        );
    }
    if report.audit_docs_canon_mode {
        println!("  audit_docs_canon:      true (PH-S1414 band 77)");
        println!(
            "  audit_docs_canon_criteria: {}/{} met",
            report.audit_docs_canon_criteria_met_count, report.audit_docs_canon_criteria_total
        );
        println!(
            "  audit_docs_canon_cases: {}",
            AUDIT_DOCS_CANON_CASES.join(", ")
        );
    }
    if report.audit_vision_sync_mode {
        println!("  audit_vision_sync:     true (PH-S1424 band 78)");
        println!(
            "  audit_vision_sync_criteria: {}/{} met",
            report.audit_vision_sync_criteria_met_count, report.audit_vision_sync_criteria_total
        );
        println!(
            "  audit_vision_sync_cases: {}",
            AUDIT_VISION_SYNC_CASES.join(", ")
        );
    }
    if report.audit_ratio_advisory_mode {
        println!("  audit_ratio_advisory:  true (PH-S1434 band 79)");
        println!(
            "  audit_ratio_advisory_criteria: {}/{} met",
            report.audit_ratio_advisory_criteria_met_count,
            report.audit_ratio_advisory_criteria_total
        );
        println!(
            "  audit_ratio_advisory_cases: {}",
            AUDIT_RATIO_ADVISORY_CASES.join(", ")
        );
    }
    if report.audit_horizon_mode {
        println!("  audit_horizon:         true (PH-S1444 band 80)");
        println!(
            "  audit_horizon_criteria: {}/{} met",
            report.audit_horizon_criteria_met_count, report.audit_horizon_criteria_total
        );
        println!("  audit_horizon_cases: {}", AUDIT_HORIZON_CASES.join(", "));
    }
    if report.policy_mode {
        println!("  policy:                true (PH-S1454 band 81)");
        println!(
            "  policy_criteria:       {}/{} met",
            report.policy_criteria_met_count, report.policy_criteria_total
        );
        println!("  policy_cases:          {}", POLICY_CASES.join(", "));
    }
    if report.policy_store_mode {
        println!("  policy_store:          true (PH-S1464 band 82)");
        println!(
            "  policy_store_criteria: {}/{} met",
            report.policy_store_criteria_met_count, report.policy_store_criteria_total
        );
        println!("  policy_store_cases:    {}", POLICY_STORE_CASES.join(", "));
    }
    if report.policy_api_mode {
        println!("  policy_api:            true (PH-S1475 band 83)");
        println!(
            "  policy_api_criteria:   {}/{} met",
            report.policy_api_criteria_met_count, report.policy_api_criteria_total
        );
        println!("  policy_api_cases:      {}", POLICY_API_CASES.join(", "));
    }
    if report.policy_admin_ops_mode {
        println!("  policy_admin_ops:      true (PH-S1485 band 84)");
        println!(
            "  policy_admin_ops_criteria: {}/{} met",
            report.policy_admin_ops_criteria_met_count, report.policy_admin_ops_criteria_total
        );
        println!(
            "  policy_admin_ops_cases: {}",
            POLICY_ADMIN_OPS_CASES.join(", ")
        );
    }
    if report.policy_stand_smoke_mode {
        println!("  policy_stand_smoke:    true (PH-S1494 band 85)");
        println!(
            "  policy_stand_smoke_criteria: {}/{} met",
            report.policy_stand_smoke_criteria_met_count, report.policy_stand_smoke_criteria_total
        );
        println!(
            "  policy_stand_smoke_cases: {}",
            POLICY_STAND_SMOKE_CASES.join(", ")
        );
    }
    if report.policy_loc_audit_mode {
        println!("  policy_loc_audit:      true (PH-S1504 band 86)");
        println!(
            "  policy_loc_audit_criteria: {}/{} met",
            report.policy_loc_audit_criteria_met_count, report.policy_loc_audit_criteria_total
        );
        println!(
            "  policy_loc_audit_cases: {}",
            POLICY_LOC_AUDIT_CASES.join(", ")
        );
    }
    if report.policy_docs_canon_mode {
        println!("  policy_docs_canon:     true (PH-S1514 band 87)");
        println!(
            "  policy_docs_canon_criteria: {}/{} met",
            report.policy_docs_canon_criteria_met_count, report.policy_docs_canon_criteria_total
        );
        println!(
            "  policy_docs_canon_cases: {}",
            POLICY_DOCS_CANON_CASES.join(", ")
        );
    }
    if report.policy_vision_sync_mode {
        println!("  policy_vision_sync:    true (PH-S1524 band 88)");
        println!(
            "  policy_vision_sync_criteria: {}/{} met",
            report.policy_vision_sync_criteria_met_count, report.policy_vision_sync_criteria_total
        );
        println!(
            "  policy_vision_sync_cases: {}",
            POLICY_VISION_SYNC_CASES.join(", ")
        );
    }
    if report.policy_ratio_advisory_mode {
        println!("  policy_ratio_advisory: true (PH-S1534 band 89)");
        println!(
            "  policy_ratio_advisory_criteria: {}/{} met",
            report.policy_ratio_advisory_criteria_met_count,
            report.policy_ratio_advisory_criteria_total
        );
        println!(
            "  policy_ratio_advisory_cases: {}",
            POLICY_RATIO_ADVISORY_CASES.join(", ")
        );
    }
    if report.policy_horizon_mode {
        println!("  policy_horizon:        true (PH-S1544 band 90)");
        println!(
            "  policy_horizon_criteria: {}/{} met",
            report.policy_horizon_criteria_met_count, report.policy_horizon_criteria_total
        );
        println!(
            "  policy_horizon_cases: {}",
            POLICY_HORIZON_CASES.join(", ")
        );
    }
    if report.monitoring_mode {
        println!("  monitoring:            true (PH-S1554 band 91)");
        println!(
            "  monitoring_criteria:   {}/{} met",
            report.monitoring_criteria_met_count, report.monitoring_criteria_total
        );
        println!("  monitoring_cases:      {}", MONITORING_CASES.join(", "));
    }
    if report.monitoring_store_mode {
        println!("  monitoring_store:      true (PH-S1564 band 92)");
        println!(
            "  monitoring_store_criteria: {}/{} met",
            report.monitoring_store_criteria_met_count, report.monitoring_store_criteria_total
        );
        println!(
            "  monitoring_store_cases: {}",
            MONITORING_STORE_CASES.join(", ")
        );
    }
    if report.monitoring_api_mode {
        println!("  monitoring_api:        true (PH-S1575 band 93)");
        println!(
            "  monitoring_api_criteria: {}/{} met",
            report.monitoring_api_criteria_met_count, report.monitoring_api_criteria_total
        );
        println!(
            "  monitoring_api_cases: {}",
            MONITORING_API_CASES.join(", ")
        );
    }
    if report.monitoring_admin_ops_mode {
        println!("  monitoring_admin_ops:  true (PH-S1585 band 94)");
        println!(
            "  monitoring_admin_ops_criteria: {}/{} met",
            report.monitoring_admin_ops_criteria_met_count,
            report.monitoring_admin_ops_criteria_total
        );
        println!(
            "  monitoring_admin_ops_cases: {}",
            MONITORING_ADMIN_OPS_CASES.join(", ")
        );
    }
    if report.monitoring_stand_smoke_mode {
        println!("  monitoring_stand_smoke: true (PH-S1594 band 95)");
        println!(
            "  monitoring_stand_smoke_criteria: {}/{} met",
            report.monitoring_stand_smoke_criteria_met_count,
            report.monitoring_stand_smoke_criteria_total
        );
        println!(
            "  monitoring_stand_smoke_cases: {}",
            MONITORING_STAND_SMOKE_CASES.join(", ")
        );
    }
    if report.monitoring_loc_audit_mode {
        println!("  monitoring_loc_audit:  true (PH-S1604 band 96)");
        println!(
            "  monitoring_loc_audit_criteria: {}/{} met",
            report.monitoring_loc_audit_criteria_met_count,
            report.monitoring_loc_audit_criteria_total
        );
        println!(
            "  monitoring_loc_audit_cases: {}",
            MONITORING_LOC_AUDIT_CASES.join(", ")
        );
    }
    if report.monitoring_docs_canon_mode {
        println!("  monitoring_docs_canon: true (PH-S1614 band 97)");
        println!(
            "  monitoring_docs_canon_criteria: {}/{} met",
            report.monitoring_docs_canon_criteria_met_count,
            report.monitoring_docs_canon_criteria_total
        );
        println!(
            "  monitoring_docs_canon_cases: {}",
            MONITORING_DOCS_CANON_CASES.join(", ")
        );
    }
    if report.monitoring_vision_sync_mode {
        println!("  monitoring_vision_sync: true (PH-S1624 band 98)");
        println!(
            "  monitoring_vision_sync_criteria: {}/{} met",
            report.monitoring_vision_sync_criteria_met_count,
            report.monitoring_vision_sync_criteria_total
        );
        println!(
            "  monitoring_vision_sync_cases: {}",
            MONITORING_VISION_SYNC_CASES.join(", ")
        );
    }
    if report.monitoring_ratio_advisory_mode {
        println!("  monitoring_ratio_advisory: true (PH-S1634 band 99)");
        println!(
            "  monitoring_ratio_advisory_criteria: {}/{} met",
            report.monitoring_ratio_advisory_criteria_met_count,
            report.monitoring_ratio_advisory_criteria_total
        );
        println!(
            "  monitoring_ratio_advisory_cases: {}",
            MONITORING_RATIO_ADVISORY_CASES.join(", ")
        );
    }
    if report.monitoring_horizon_mode {
        println!("  monitoring_horizon:        true (PH-S1644 band 100)");
        println!(
            "  monitoring_horizon_criteria: {}/{} met",
            report.monitoring_horizon_criteria_met_count, report.monitoring_horizon_criteria_total
        );
        println!(
            "  monitoring_horizon_cases: {}",
            MONITORING_HORIZON_CASES.join(", ")
        );
    }
    if report.ratio96_mode {
        println!("  ratio96:                   true (PH-S1654 band 101)");
        println!(
            "  ratio96_criteria:          {}/{} met",
            report.ratio96_criteria_met_count, report.ratio96_criteria_total
        );
        println!("  ratio96_cases:             {}", RATIO96_CASES.join(", "));
    }
    if report.ratio96_admin_ops_mode {
        println!("  ratio96_admin_ops:         true (PH-S1684 band 104)");
        println!(
            "  ratio96_admin_ops_criteria: {}/{} met",
            report.ratio96_admin_ops_criteria_met_count, report.ratio96_admin_ops_criteria_total
        );
        println!(
            "  ratio96_admin_ops_cases:   {}",
            RATIO96_ADMIN_OPS_CASES.join(", ")
        );
    }
    if report.ratio96_stand_smoke_mode {
        println!("  ratio96_stand_smoke:      true (PH-S1694 band 105)");
        println!(
            "  ratio96_stand_smoke_criteria: {}/{} met",
            report.ratio96_stand_smoke_criteria_met_count,
            report.ratio96_stand_smoke_criteria_total
        );
        println!(
            "  ratio96_stand_smoke_cases: {}",
            RATIO96_STAND_SMOKE_CASES.join(", ")
        );
    }
    if report.ratio96_docs_canon_mode {
        println!("  ratio96_docs_canon:         true (PH-S1714 band 107)");
        println!(
            "  ratio96_docs_canon_criteria: {}/{} met",
            report.ratio96_docs_canon_criteria_met_count, report.ratio96_docs_canon_criteria_total
        );
        println!(
            "  ratio96_docs_canon_cases:   {}",
            RATIO96_DOCS_CANON_CASES.join(", ")
        );
    }
    if report.gpu_limits_mode {
        println!("  gpu_limits:                 true (PH-S1862 band 122)");
        println!(
            "  gpu_limits_criteria:        {}/{} met",
            report.gpu_limits_criteria_met_count, report.gpu_limits_criteria_total
        );
        println!(
            "  gpu_limits_cases:           {}",
            GPU_LIMITS_CASES.join(", ")
        );
    }
    if report.gpu_limits_api_mode {
        println!("  gpu_limits_api:             true (PH-S1872 band 123)");
        println!(
            "  gpu_limits_api_criteria:    {}/{} met",
            report.gpu_limits_api_criteria_met_count, report.gpu_limits_api_criteria_total
        );
        println!(
            "  gpu_limits_api_cases:       {}",
            GPU_LIMITS_API_CASES.join(", ")
        );
    }
    if report.gpu_limits_admin_ops_mode {
        println!("  gpu_limits_admin_ops:      true (PH-S1884 band 124)");
        println!(
            "  gpu_limits_admin_ops_criteria: {}/{} met",
            report.gpu_limits_admin_ops_criteria_met_count,
            report.gpu_limits_admin_ops_criteria_total
        );
        println!(
            "  gpu_limits_admin_ops_cases: {}",
            GPU_LIMITS_ADMIN_OPS_CASES.join(", ")
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
