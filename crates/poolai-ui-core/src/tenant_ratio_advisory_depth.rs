//! Tenant ratio-advisory band depth (PH-S1229…S1238, band 59 — enterprise phase A).
//!
//! Aggregates band 51–58 `--tenant-*` loc-audit slices under one ratio-advisory gate,
//! with restart-safe SQLite CRUD as the code core (`multi_tenancy` sqlite persist).

use serde_json::Value;

/// Tenant ratio-advisory depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TenantRatioAdvisoryDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand59,
}

/// Prior tenant loc-audit / canon slices covered by aggregate (PH-S1230).
pub const TENANT_RATIO_ADVISORY_SLICES: &[&str] = &[
    "--tenant-persist",
    "--tenant-store",
    "--tenant-api",
    "--tenant-docs-canon",
    "--tenant-vision-sync",
    "tenants.sqlite",
];

/// Tenant ratio-advisory criteria registry (PH-S1229): id · marker · doc path.
pub const TENANT_RATIO_ADVISORY_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "tenant_ratio_advisory_depth",
        "TenantRatioAdvisoryDepth",
        "crates/poolai-ui-core/src/tenant_ratio_advisory_depth.rs",
    ),
    (
        "sqlite_restart_safe",
        "persist_tenant_to_sqlite",
        "src/enterprise/multi_tenancy.rs",
    ),
    (
        "doc_tenant_store",
        "Restart-safe SQLite CRUD",
        "docs/development/TENANT_STORE.md",
    ),
    (
        "doc_ratio_advisory",
        "TENANT_RATIO_ADVISORY.md",
        "docs/development/TENANT_RATIO_ADVISORY.md",
    ),
    (
        "doc_vision_sync",
        "TENANT_VISION_SYNC.md",
        "docs/development/TENANT_VISION_SYNC.md",
    ),
    (
        "aggregate_flag",
        "--tenant-ratio-advisory",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_TENANT_RATIO_ADVISORY",
        "bin/verify-dev-stand.sh",
    ),
    (
        "integration_contracts",
        "tenant_ratio_advisory_integration",
        "tests/tenant_ratio_advisory_integration.rs",
    ),
    (
        "sqlite_durable_integration",
        "tenant_sqlite_durable_integration",
        "tests/tenant_sqlite_durable_integration.rs",
    ),
    (
        "band_close",
        "galaxy_horizon_s1229_integration",
        "tests/galaxy_horizon_s1229_integration.rs",
    ),
];

/// `poolai-loc-audit --tenant-ratio-advisory` case names (PH-S1234).
pub const TENANT_RATIO_ADVISORY_CASES: &[&str] = &[
    "tenant_ratio_advisory_depth",
    "sqlite_restart_safe",
    "doc_tenant_store",
    "doc_ratio_advisory",
    "doc_vision_sync",
    "aggregate_flag",
    "verify_dev_stand_hook",
    "integration_contracts",
    "sqlite_durable_integration",
    "band_close",
];

/// FM §5.40 band-59 marker rows.
pub const FM_BAND59_ROWS: &[&str] = &[
    "5.40",
    "Tenant ratio advisory",
    "PH-S1229…S1238",
    "tenant_ratio_advisory_depth",
];

/// Tenant ratio-advisory adoption markers for band 59.
pub const TENANT_RATIO_ADVISORY_BAND59_ROWS: &[&str] = &[
    "PH-S1229",
    "tenant_ratio_advisory_depth",
    "PH-S1230",
    "TENANT_RATIO_ADVISORY_SLICES",
    "PH-S1231",
    "tenant_ratio_advisory_integration",
    "PH-S1232",
    "VERIFY_TENANT_RATIO_ADVISORY",
    "PH-S1234",
    "--tenant-ratio-advisory",
    "PH-S1238",
];

/// Production-verify stub: how many ratio-advisory slices are referenced (PH-S1230).
pub fn tenant_ratio_advisory_slices_met(canon_src: &str) -> (usize, usize) {
    let total = TENANT_RATIO_ADVISORY_SLICES.len();
    let met = TENANT_RATIO_ADVISORY_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify tenant ratio-advisory band depth from optional feature stub (PH-S1229).
pub fn tenant_ratio_advisory_depth_stub(features: Option<&Value>) -> TenantRatioAdvisoryDepth {
    let Some(f) = features else {
        return TenantRatioAdvisoryDepth::None;
    };
    let depth = f
        .get("tenant_ratio_advisory_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let slices = f
        .get("slice_aggregate")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let contracts = f
        .get("criteria_contracts")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let verify = f
        .get("verify_dev_stand_hook")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let export = f
        .get("stand_smoke_export")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let loc = f
        .get("loc_audit_flag")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let docs = f
        .get("tenant_ratio_advisory_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let ratio = f
        .get("ratio_hold")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let close = f
        .get("band_close")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if depth && slices && contracts && verify && export && loc && docs && ratio && close {
        return TenantRatioAdvisoryDepth::FullBand59;
    }
    if close || ratio {
        return TenantRatioAdvisoryDepth::RatioHold;
    }
    if docs {
        return TenantRatioAdvisoryDepth::DocsCanon;
    }
    if loc {
        return TenantRatioAdvisoryDepth::LocAuditFlag;
    }
    if export {
        return TenantRatioAdvisoryDepth::StandSmokeExport;
    }
    if verify {
        return TenantRatioAdvisoryDepth::VerifyDevStandHook;
    }
    if contracts {
        return TenantRatioAdvisoryDepth::CriteriaContracts;
    }
    if slices {
        return TenantRatioAdvisoryDepth::SliceAggregate;
    }
    if depth {
        return TenantRatioAdvisoryDepth::DepthModule;
    }
    TenantRatioAdvisoryDepth::None
}

/// Total tenant ratio-advisory criteria in registry (PH-S1229).
pub fn tenant_ratio_advisory_criteria_total() -> usize {
    TENANT_RATIO_ADVISORY_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn tenant_ratio_advisory_depth_stub_ph_s1229() {
        assert_eq!(
            tenant_ratio_advisory_depth_stub(None),
            TenantRatioAdvisoryDepth::None
        );
        assert_eq!(
            tenant_ratio_advisory_depth_stub(Some(&json!({
                "tenant_ratio_advisory_depth": true
            }))),
            TenantRatioAdvisoryDepth::DepthModule
        );
        assert_eq!(
            tenant_ratio_advisory_depth_stub(Some(&json!({
                "tenant_ratio_advisory_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "tenant_ratio_advisory_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            TenantRatioAdvisoryDepth::FullBand59
        );
        assert_eq!(TENANT_RATIO_ADVISORY_CRITERIA.len(), 10);
        assert_eq!(tenant_ratio_advisory_criteria_total(), 10);
        assert_eq!(TENANT_RATIO_ADVISORY_SLICES.len(), 6);
        assert!(FM_BAND59_ROWS.contains(&"PH-S1229…S1238"));
    }

    #[test]
    fn tenant_ratio_advisory_slices_met_ph_s1230() {
        let src = "--tenant-persist --tenant-store --tenant-api --tenant-docs-canon --tenant-vision-sync tenants.sqlite";
        assert_eq!(tenant_ratio_advisory_slices_met(src), (6, 6));
        assert_eq!(tenant_ratio_advisory_slices_met("--tenant-persist"), (1, 6));
    }
}
