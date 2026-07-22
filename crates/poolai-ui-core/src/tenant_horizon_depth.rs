//! Tenant horizon-close band depth (PH-S1239…S1248, band 60 — enterprise phase A close).
//!
//! Aggregates bands 51–59 `--tenant-*` loc-audit slices under one horizon gate,
//! closing multi-tenancy phase A before SSO (band 61).

use serde_json::Value;

/// Tenant horizon depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TenantHorizonDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand60,
}

/// Phase-A tenant loc-audit / canon slices covered by horizon aggregate (PH-S1240).
pub const TENANT_HORIZON_SLICES: &[&str] = &[
    "--tenant-persist",
    "--tenant-store",
    "--tenant-api",
    "--tenant-admin-ops",
    "--tenant-stand-smoke",
    "--tenant-loc-audit",
    "--tenant-docs-canon",
    "--tenant-vision-sync",
    "--tenant-ratio-advisory",
    "tenants.sqlite",
];

/// Tenant horizon criteria registry (PH-S1239): id · marker · doc path.
pub const TENANT_HORIZON_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "tenant_horizon_depth",
        "TenantHorizonDepth",
        "crates/poolai-ui-core/src/tenant_horizon_depth.rs",
    ),
    (
        "phase_a_slices",
        "TENANT_HORIZON_SLICES",
        "crates/poolai-ui-core/src/tenant_horizon_depth.rs",
    ),
    (
        "sqlite_restart_safe",
        "persist_tenant_to_sqlite",
        "src/enterprise/multi_tenancy.rs",
    ),
    (
        "doc_tenant_horizon",
        "TENANT_HORIZON.md",
        "docs/development/TENANT_HORIZON.md",
    ),
    (
        "doc_ratio_advisory",
        "TENANT_RATIO_ADVISORY.md",
        "docs/development/TENANT_RATIO_ADVISORY.md",
    ),
    (
        "aggregate_flag",
        "--tenant-horizon",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_TENANT_HORIZON",
        "bin/verify-dev-stand.sh",
    ),
    (
        "integration_contracts",
        "tenant_horizon_integration",
        "tests/tenant_horizon_integration.rs",
    ),
    (
        "ratio_advisory_prior",
        "tenant_ratio_advisory_depth",
        "crates/poolai-ui-core/src/tenant_ratio_advisory_depth.rs",
    ),
    (
        "band_close",
        "galaxy_horizon_s1239_integration",
        "tests/galaxy_horizon_s1239_integration.rs",
    ),
];

/// `poolai-loc-audit --tenant-horizon` case names (PH-S1244).
pub const TENANT_HORIZON_CASES: &[&str] = &[
    "tenant_horizon_depth",
    "phase_a_slices",
    "sqlite_restart_safe",
    "doc_tenant_horizon",
    "doc_ratio_advisory",
    "aggregate_flag",
    "verify_dev_stand_hook",
    "integration_contracts",
    "ratio_advisory_prior",
    "band_close",
];

/// FM §5.41 band-60 marker rows.
pub const FM_BAND60_ROWS: &[&str] = &[
    "5.41",
    "Tenant horizon close",
    "PH-S1239…S1248",
    "tenant_horizon_depth",
];

/// Tenant horizon adoption markers for band 60.
pub const TENANT_HORIZON_BAND60_ROWS: &[&str] = &[
    "PH-S1239",
    "tenant_horizon_depth",
    "PH-S1240",
    "TENANT_HORIZON_SLICES",
    "PH-S1241",
    "tenant_horizon_integration",
    "PH-S1242",
    "VERIFY_TENANT_HORIZON",
    "PH-S1244",
    "--tenant-horizon",
    "PH-S1248",
];

/// Production-verify stub: how many horizon slices are referenced (PH-S1240).
pub fn tenant_horizon_slices_met(canon_src: &str) -> (usize, usize) {
    let total = TENANT_HORIZON_SLICES.len();
    let met = TENANT_HORIZON_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify tenant horizon band depth from optional feature stub (PH-S1239).
pub fn tenant_horizon_depth_stub(features: Option<&Value>) -> TenantHorizonDepth {
    let Some(f) = features else {
        return TenantHorizonDepth::None;
    };
    let depth = f
        .get("tenant_horizon_depth")
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
        .get("tenant_horizon_docs")
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
        return TenantHorizonDepth::FullBand60;
    }
    if close || ratio {
        return TenantHorizonDepth::RatioHold;
    }
    if docs {
        return TenantHorizonDepth::DocsCanon;
    }
    if loc {
        return TenantHorizonDepth::LocAuditFlag;
    }
    if export {
        return TenantHorizonDepth::StandSmokeExport;
    }
    if verify {
        return TenantHorizonDepth::VerifyDevStandHook;
    }
    if contracts {
        return TenantHorizonDepth::CriteriaContracts;
    }
    if slices {
        return TenantHorizonDepth::SliceAggregate;
    }
    if depth {
        return TenantHorizonDepth::DepthModule;
    }
    TenantHorizonDepth::None
}

/// Total tenant horizon criteria in registry (PH-S1239).
pub fn tenant_horizon_criteria_total() -> usize {
    TENANT_HORIZON_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn tenant_horizon_depth_stub_ph_s1239() {
        assert_eq!(tenant_horizon_depth_stub(None), TenantHorizonDepth::None);
        assert_eq!(
            tenant_horizon_depth_stub(Some(&json!({
                "tenant_horizon_depth": true
            }))),
            TenantHorizonDepth::DepthModule
        );
        assert_eq!(
            tenant_horizon_depth_stub(Some(&json!({
                "tenant_horizon_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "tenant_horizon_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            TenantHorizonDepth::FullBand60
        );
        assert_eq!(TENANT_HORIZON_CRITERIA.len(), 10);
        assert_eq!(tenant_horizon_criteria_total(), 10);
        assert_eq!(TENANT_HORIZON_SLICES.len(), 10);
        assert!(FM_BAND60_ROWS.contains(&"PH-S1239…S1248"));
    }

    #[test]
    fn tenant_horizon_slices_met_ph_s1240() {
        let src = "--tenant-persist --tenant-store --tenant-api --tenant-admin-ops --tenant-stand-smoke --tenant-loc-audit --tenant-docs-canon --tenant-vision-sync --tenant-ratio-advisory tenants.sqlite";
        assert_eq!(tenant_horizon_slices_met(src), (10, 10));
        assert_eq!(tenant_horizon_slices_met("--tenant-persist"), (1, 10));
    }
}
