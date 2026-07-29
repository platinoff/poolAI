//! Monitoring live stand-smoke band depth (PH-S1589…S1598, band 95 — enterprise phase E).

use serde_json::Value;

/// Monitoring stand-smoke depth flags (live HTTP / CLI / verify / docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonitoringStandSmokeDepth {
    None,
    DepthModule,
    LiveStore,
    LiveAlertsQuery,
    LiveMonitoringFieldFixtures,
    CliFlag,
    LocAuditFlag,
    VerifyDevStandHook,
    DocsCanon,
    RatioHold,
    FullBand95,
}

/// Monitoring stand-smoke criteria registry (PH-S1589): id · marker · doc path.
pub const MONITORING_STAND_SMOKE_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "monitoring_stand_smoke_depth",
        "MonitoringStandSmokeDepth",
        "crates/poolai-ui-core/src/monitoring_stand_smoke_depth.rs",
    ),
    (
        "live_store",
        "smoke_monitoring_store_wire",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "live_alerts_query",
        "smoke_monitoring_alerts_query",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "live_monitoring_field_fixtures",
        "smoke_monitoring_field_fixtures",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "cli_flag",
        "--monitoring-stand-smoke",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--monitoring-stand-smoke",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_MONITORING_STAND_SMOKE",
        "bin/verify-dev-stand.sh",
    ),
    (
        "monitoring_stand_smoke_docs",
        "MONITORING_STAND_SMOKE.md",
        "docs/development/MONITORING_STAND_SMOKE.md",
    ),
    ("ratio_hold", "min-ratio", "docs/development/RUN_LOCAL.md"),
    (
        "band_close",
        "galaxy_horizon_s1589_integration",
        "tests/galaxy_horizon_s1589_integration.rs",
    ),
];

/// `poolai-loc-audit --monitoring-stand-smoke` case names (PH-S1594).
pub const MONITORING_STAND_SMOKE_CASES: &[&str] = &[
    "monitoring_stand_smoke_depth",
    "live_store",
    "live_alerts_query",
    "live_monitoring_field_fixtures",
    "cli_flag",
    "loc_audit_flag",
    "verify_dev_stand_hook",
    "monitoring_stand_smoke_docs",
    "ratio_hold",
    "band_close",
];

/// FM §5.76 band-95 marker rows.
pub const FM_BAND95_ROWS: &[&str] = &[
    "5.76",
    "Monitoring stand smoke",
    "PH-S1589…S1598",
    "monitoring_stand_smoke_depth",
];

/// Monitoring stand-smoke adoption markers for band 95.
pub const MONITORING_STAND_SMOKE_BAND95_ROWS: &[&str] = &[
    "PH-S1589",
    "monitoring_stand_smoke_depth",
    "PH-S1590",
    "smoke_monitoring_store_wire",
    "PH-S1591",
    "smoke_monitoring_alerts_query",
    "PH-S1592",
    "smoke_monitoring_field_fixtures",
    "PH-S1593",
    "--monitoring-stand-smoke",
    "PH-S1595",
    "VERIFY_MONITORING_STAND_SMOKE",
    "PH-S1598",
];

/// Classify monitoring stand-smoke band depth from optional feature stub (PH-S1589).
pub fn monitoring_stand_smoke_depth_stub(features: Option<&Value>) -> MonitoringStandSmokeDepth {
    let Some(f) = features else {
        return MonitoringStandSmokeDepth::None;
    };
    let depth = f
        .get("monitoring_stand_smoke_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let store = f
        .get("live_store")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let alerts = f
        .get("live_alerts_query")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let fixtures = f
        .get("live_monitoring_field_fixtures")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let cli = f.get("cli_flag").and_then(|v| v.as_bool()).unwrap_or(false);
    let loc = f
        .get("loc_audit_flag")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let verify = f
        .get("verify_dev_stand_hook")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let docs = f
        .get("monitoring_stand_smoke_docs")
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

    if depth && store && alerts && fixtures && cli && loc && verify && docs && ratio && close {
        return MonitoringStandSmokeDepth::FullBand95;
    }
    if close || ratio {
        return MonitoringStandSmokeDepth::RatioHold;
    }
    if docs {
        return MonitoringStandSmokeDepth::DocsCanon;
    }
    if verify {
        return MonitoringStandSmokeDepth::VerifyDevStandHook;
    }
    if loc {
        return MonitoringStandSmokeDepth::LocAuditFlag;
    }
    if cli {
        return MonitoringStandSmokeDepth::CliFlag;
    }
    if fixtures {
        return MonitoringStandSmokeDepth::LiveMonitoringFieldFixtures;
    }
    if alerts {
        return MonitoringStandSmokeDepth::LiveAlertsQuery;
    }
    if store {
        return MonitoringStandSmokeDepth::LiveStore;
    }
    if depth {
        return MonitoringStandSmokeDepth::DepthModule;
    }
    MonitoringStandSmokeDepth::None
}

/// Total monitoring stand-smoke criteria in registry (PH-S1589).
pub fn monitoring_stand_smoke_criteria_total() -> usize {
    MONITORING_STAND_SMOKE_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn monitoring_stand_smoke_depth_stub_ph_s1589() {
        assert_eq!(
            monitoring_stand_smoke_depth_stub(None),
            MonitoringStandSmokeDepth::None
        );
        assert_eq!(
            monitoring_stand_smoke_depth_stub(Some(&json!({"monitoring_stand_smoke_depth": true}))),
            MonitoringStandSmokeDepth::DepthModule
        );
        assert_eq!(
            monitoring_stand_smoke_depth_stub(Some(&json!({
                "monitoring_stand_smoke_depth": true,
                "live_store": true,
                "live_alerts_query": true,
                "live_monitoring_field_fixtures": true,
                "cli_flag": true,
                "loc_audit_flag": true,
                "verify_dev_stand_hook": true,
                "monitoring_stand_smoke_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            MonitoringStandSmokeDepth::FullBand95
        );
        assert_eq!(MONITORING_STAND_SMOKE_CRITERIA.len(), 10);
        assert_eq!(monitoring_stand_smoke_criteria_total(), 10);
        assert!(FM_BAND95_ROWS.contains(&"PH-S1589…S1598"));
    }
}
