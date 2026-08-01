//! Monitoring ratio-advisory band depth (PH-S1629…S1638, band 99 — enterprise phase E).
//!
//! Consolidates the Monitoring loc-audit ratio gate (`rust_ratio.json` + `--min-ratio
//! --advisory`) under one ratio-advisory aggregate, mirroring band 88
//! [`POLICIES_RATIO_ADVISORY.md`](../../docs/development/POLICIES_RATIO_ADVISORY.md).

use serde_json::Value;

/// Monitoring ratio-advisory depth flags (registry / slices / verify / close).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonitoringRatioAdvisoryDepth {
    None,
    DepthModule,
    SliceAggregate,
    CriteriaContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    RatioHold,
    FullBand99,
}

/// Ratio-advisory slice files covered by aggregate (PH-S1630).
pub const MONITORING_RATIO_ADVISORY_SLICES: &[&str] = &[
    "rust_ratio.json",
    "RUST_RATIO_STRATEGY_2026-06-13.md",
    "MONITORING_VISION_SYNC.md",
    "poolai_loc_audit.rs",
    "run-poolai.sh",
    "verify-dev-stand.sh",
];

/// Monitoring ratio-advisory criteria registry (PH-S1629): id · marker · doc path.
pub const MONITORING_RATIO_ADVISORY_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "monitoring_ratio_advisory_depth",
        "MonitoringRatioAdvisoryDepth",
        "crates/poolai-ui-core/src/monitoring_ratio_advisory_depth.rs",
    ),
    (
        "ratio_json",
        "rust_ratio.json",
        "docs/development/rust_ratio.json",
    ),
    (
        "ratio_strategy",
        "RUST_RATIO_STRATEGY_2026-06-13.md",
        "docs/development/RUST_RATIO_STRATEGY_2026-06-13.md",
    ),
    (
        "prior_canon",
        "MONITORING_VISION_SYNC.md",
        "docs/development/MONITORING_VISION_SYNC.md",
    ),
    (
        "ratio_doc",
        "MONITORING_RATIO_ADVISORY.md",
        "docs/development/MONITORING_RATIO_ADVISORY.md",
    ),
    (
        "aggregate_flag",
        "--monitoring-ratio-advisory",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "min_ratio_flag",
        "--min-ratio",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_MONITORING_RATIO_ADVISORY",
        "bin/verify-dev-stand.sh",
    ),
    (
        "quick_flag",
        "--monitoring-ratio-advisory",
        "bin/run-poolai.sh",
    ),
    (
        "band_close",
        "galaxy_horizon_s1629_integration",
        "tests/galaxy_horizon_s1629_integration.rs",
    ),
];

/// `poolai-loc-audit --monitoring-ratio-advisory` case names (PH-S1634).
pub const MONITORING_RATIO_ADVISORY_CASES: &[&str] = &[
    "monitoring_ratio_advisory_depth",
    "ratio_json",
    "ratio_strategy",
    "prior_canon",
    "ratio_doc",
    "aggregate_flag",
    "min_ratio_flag",
    "verify_dev_stand_hook",
    "quick_flag",
    "band_close",
];

/// FM §5.80 band-99 marker rows.
pub const FM_BAND99_ROWS: &[&str] = &[
    "5.80",
    "Monitoring ratio advisory",
    "PH-S1629…S1638",
    "monitoring_ratio_advisory_depth",
];

/// Monitoring ratio-advisory adoption markers for band 99.
pub const MONITORING_RATIO_ADVISORY_BAND99_ROWS: &[&str] = &[
    "PH-S1629",
    "monitoring_ratio_advisory_depth",
    "PH-S1630",
    "MONITORING_RATIO_ADVISORY_SLICES",
    "PH-S1631",
    "monitoring_ratio_advisory_integration",
    "PH-S1632",
    "VERIFY_MONITORING_RATIO_ADVISORY",
    "PH-S1634",
    "--monitoring-ratio-advisory",
    "PH-S1638",
];

/// Production-verify stub: how many ratio-advisory slices are referenced (PH-S1630).
pub fn monitoring_ratio_advisory_slices_met(canon_src: &str) -> (usize, usize) {
    let total = MONITORING_RATIO_ADVISORY_SLICES.len();
    let met = MONITORING_RATIO_ADVISORY_SLICES
        .iter()
        .filter(|name| canon_src.contains(*name))
        .count();
    (met, total)
}

/// Classify Monitoring ratio-advisory band depth from optional feature stub (PH-S1629).
pub fn monitoring_ratio_advisory_depth_stub(
    features: Option<&Value>,
) -> MonitoringRatioAdvisoryDepth {
    let Some(f) = features else {
        return MonitoringRatioAdvisoryDepth::None;
    };
    let depth = f
        .get("monitoring_ratio_advisory_depth")
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
        .get("monitoring_ratio_advisory_docs")
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
        return MonitoringRatioAdvisoryDepth::FullBand99;
    }
    if close || ratio {
        return MonitoringRatioAdvisoryDepth::RatioHold;
    }
    if docs {
        return MonitoringRatioAdvisoryDepth::DocsCanon;
    }
    if loc {
        return MonitoringRatioAdvisoryDepth::LocAuditFlag;
    }
    if export {
        return MonitoringRatioAdvisoryDepth::StandSmokeExport;
    }
    if verify {
        return MonitoringRatioAdvisoryDepth::VerifyDevStandHook;
    }
    if contracts {
        return MonitoringRatioAdvisoryDepth::CriteriaContracts;
    }
    if slices {
        return MonitoringRatioAdvisoryDepth::SliceAggregate;
    }
    if depth {
        return MonitoringRatioAdvisoryDepth::DepthModule;
    }
    MonitoringRatioAdvisoryDepth::None
}

/// Total Monitoring ratio-advisory criteria in registry (PH-S1629).
pub fn monitoring_ratio_advisory_criteria_total() -> usize {
    MONITORING_RATIO_ADVISORY_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn monitoring_ratio_advisory_depth_stub_ph_s1629() {
        assert_eq!(
            monitoring_ratio_advisory_depth_stub(None),
            MonitoringRatioAdvisoryDepth::None
        );
        assert_eq!(
            monitoring_ratio_advisory_depth_stub(Some(&json!({
                "monitoring_ratio_advisory_depth": true
            }))),
            MonitoringRatioAdvisoryDepth::DepthModule
        );
        assert_eq!(
            monitoring_ratio_advisory_depth_stub(Some(&json!({
                "monitoring_ratio_advisory_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "monitoring_ratio_advisory_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            MonitoringRatioAdvisoryDepth::FullBand99
        );
        assert_eq!(MONITORING_RATIO_ADVISORY_CRITERIA.len(), 10);
        assert_eq!(monitoring_ratio_advisory_criteria_total(), 10);
        assert_eq!(MONITORING_RATIO_ADVISORY_SLICES.len(), 6);
        assert!(FM_BAND99_ROWS.contains(&"PH-S1629…S1638"));
    }

    #[test]
    fn monitoring_ratio_advisory_slices_met_ph_s1630() {
        let src = "rust_ratio.json RUST_RATIO_STRATEGY_2026-06-13.md MONITORING_VISION_SYNC.md \
            poolai_loc_audit.rs run-poolai.sh verify-dev-stand.sh";
        assert_eq!(monitoring_ratio_advisory_slices_met(src), (6, 6));
        assert_eq!(
            monitoring_ratio_advisory_slices_met("rust_ratio.json"),
            (1, 6)
        );
    }
}
