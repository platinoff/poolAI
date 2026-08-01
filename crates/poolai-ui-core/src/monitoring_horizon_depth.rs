//! Monitoring horizon-close band depth (PH-S1639…S1648, band 100 — enterprise phase E close).
//!
//! Aggregates bands 91–99 `--monitoring*` + `--monitoring-ratio-advisory` slices under one horizon gate.

use serde_json::Value;

/// Monitoring horizon depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonitoringHorizonDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand100,
}

/// Phase-E Monitoring loc-audit / canon slices covered by horizon aggregate (PH-S1640).
pub const MONITORING_HORIZON_SLICES: &[&str] = &[
    "--monitoring",
    "--monitoring-store",
    "--monitoring-api",
    "--monitoring-admin-ops",
    "--monitoring-stand-smoke",
    "--monitoring-loc-audit",
    "--monitoring-docs-canon",
    "--monitoring-vision-sync",
    "--monitoring-ratio-advisory",
    "MONITORING_RATIO_ADVISORY.md",
];

/// Monitoring horizon criteria registry (PH-S1639): id · marker · doc path.
pub const MONITORING_HORIZON_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "monitoring_horizon_depth",
        "MonitoringHorizonDepth",
        "crates/poolai-ui-core/src/monitoring_horizon_depth.rs",
    ),
    (
        "phase_e_slices",
        "MONITORING_HORIZON_SLICES",
        "crates/poolai-ui-core/src/monitoring_horizon_depth.rs",
    ),
    (
        "prior_monitoring_store",
        "--monitoring-store",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "doc_monitoring_horizon",
        "MONITORING_HORIZON.md",
        "docs/development/MONITORING_HORIZON.md",
    ),
    (
        "doc_ratio_advisory",
        "MONITORING_RATIO_ADVISORY.md",
        "docs/development/MONITORING_RATIO_ADVISORY.md",
    ),
    (
        "aggregate_flag",
        "--monitoring-horizon",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_MONITORING_HORIZON",
        "bin/verify-dev-stand.sh",
    ),
    (
        "integration_contracts",
        "monitoring_horizon_integration",
        "tests/monitoring_horizon_integration.rs",
    ),
    (
        "ratio_advisory_prior",
        "monitoring_ratio_advisory_depth",
        "crates/poolai-ui-core/src/monitoring_ratio_advisory_depth.rs",
    ),
    (
        "band_close",
        "galaxy_horizon_s1639_integration",
        "tests/galaxy_horizon_s1639_integration.rs",
    ),
];

/// `poolai-loc-audit --monitoring-horizon` case names (PH-S1644).
pub const MONITORING_HORIZON_CASES: &[&str] = &[
    "monitoring_horizon_depth",
    "phase_e_slices",
    "prior_monitoring_store",
    "doc_monitoring_horizon",
    "doc_ratio_advisory",
    "aggregate_flag",
    "verify_dev_stand_hook",
    "integration_contracts",
    "ratio_advisory_prior",
    "band_close",
];

/// FM §5.81 band-100 marker rows.
pub const FM_BAND100_ROWS: &[&str] = &[
    "5.81",
    "Monitoring horizon close",
    "PH-S1639…S1648",
    "monitoring_horizon_depth",
];

/// Monitoring horizon adoption markers for band 100.
pub const MONITORING_HORIZON_BAND100_ROWS: &[&str] = &[
    "PH-S1639",
    "monitoring_horizon_depth",
    "PH-S1640",
    "MONITORING_HORIZON_SLICES",
    "PH-S1641",
    "monitoring_horizon_integration",
    "PH-S1642",
    "VERIFY_MONITORING_HORIZON",
    "PH-S1644",
    "--monitoring-horizon",
    "PH-S1648",
];

/// Production-verify stub: how many horizon slices are referenced (PH-S1640).
pub fn monitoring_horizon_slices_met(canon_src: &str) -> (usize, usize) {
    let total = MONITORING_HORIZON_SLICES.len();
    let met = MONITORING_HORIZON_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify monitoring horizon band depth from optional feature stub (PH-S1639).
pub fn monitoring_horizon_depth_stub(features: Option<&Value>) -> MonitoringHorizonDepth {
    let Some(f) = features else {
        return MonitoringHorizonDepth::None;
    };
    let depth = f
        .get("monitoring_horizon_depth")
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
        .get("monitoring_horizon_docs")
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
        return MonitoringHorizonDepth::FullBand100;
    }
    if close || ratio {
        return MonitoringHorizonDepth::RatioHold;
    }
    if docs {
        return MonitoringHorizonDepth::DocsCanon;
    }
    if loc {
        return MonitoringHorizonDepth::LocAuditFlag;
    }
    if export {
        return MonitoringHorizonDepth::StandSmokeExport;
    }
    if verify {
        return MonitoringHorizonDepth::VerifyDevStandHook;
    }
    if contracts {
        return MonitoringHorizonDepth::CriteriaContracts;
    }
    if slices {
        return MonitoringHorizonDepth::SliceAggregate;
    }
    if depth {
        return MonitoringHorizonDepth::DepthModule;
    }
    MonitoringHorizonDepth::None
}

/// Total monitoring horizon criteria in registry (PH-S1639).
pub fn monitoring_horizon_criteria_total() -> usize {
    MONITORING_HORIZON_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn monitoring_horizon_depth_stub_ph_s1639() {
        assert_eq!(
            monitoring_horizon_depth_stub(None),
            MonitoringHorizonDepth::None
        );
        assert_eq!(
            monitoring_horizon_depth_stub(Some(&json!({"monitoring_horizon_depth": true}))),
            MonitoringHorizonDepth::DepthModule
        );
        assert_eq!(
            monitoring_horizon_depth_stub(Some(&json!({
                "monitoring_horizon_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "monitoring_horizon_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            MonitoringHorizonDepth::FullBand100
        );
        assert_eq!(MONITORING_HORIZON_CRITERIA.len(), 10);
        assert_eq!(monitoring_horizon_criteria_total(), 10);
        assert_eq!(MONITORING_HORIZON_SLICES.len(), 10);
        assert!(FM_BAND100_ROWS.contains(&"PH-S1639…S1648"));
    }

    #[test]
    fn monitoring_horizon_slices_met_ph_s1640() {
        let src = "--monitoring --monitoring-store --monitoring-api --monitoring-admin-ops --monitoring-stand-smoke --monitoring-loc-audit --monitoring-docs-canon --monitoring-vision-sync --monitoring-ratio-advisory MONITORING_RATIO_ADVISORY.md";
        assert_eq!(monitoring_horizon_slices_met(src), (10, 10));
        assert_eq!(monitoring_horizon_slices_met("--monitoring"), (1, 10));
    }
}
