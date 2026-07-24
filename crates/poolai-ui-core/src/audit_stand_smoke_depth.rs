//! Audit live stand-smoke band depth (PH-S1389…S1398, band 75 — enterprise phase C).

use serde_json::Value;

/// Audit stand-smoke depth flags (live HTTP / CLI / verify / docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditStandSmokeDepth {
    None,
    DepthModule,
    LiveStore,
    LiveEventsQuery,
    LiveEventFieldFixtures,
    CliFlag,
    LocAuditFlag,
    VerifyDevStandHook,
    DocsCanon,
    RatioHold,
    FullBand75,
}

/// Audit stand-smoke criteria registry (PH-S1389): id · marker · doc path.
pub const AUDIT_STAND_SMOKE_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "audit_stand_smoke_depth",
        "AuditStandSmokeDepth",
        "crates/poolai-ui-core/src/audit_stand_smoke_depth.rs",
    ),
    (
        "live_store",
        "smoke_audit_store_wire",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "live_events_query",
        "smoke_audit_events_query",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "live_event_field_fixtures",
        "smoke_audit_event_field_fixtures",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "cli_flag",
        "--audit-stand-smoke",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--audit-stand-smoke",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_AUDIT_STAND_SMOKE",
        "bin/verify-dev-stand.sh",
    ),
    (
        "audit_stand_smoke_docs",
        "AUDIT_STAND_SMOKE.md",
        "docs/development/AUDIT_STAND_SMOKE.md",
    ),
    ("ratio_hold", "min-ratio", "docs/development/RUN_LOCAL.md"),
    (
        "band_close",
        "galaxy_horizon_s1389_integration",
        "tests/galaxy_horizon_s1389_integration.rs",
    ),
];

/// `poolai-loc-audit --audit-stand-smoke` case names (PH-S1394).
pub const AUDIT_STAND_SMOKE_CASES: &[&str] = &[
    "audit_stand_smoke_depth",
    "live_store",
    "live_events_query",
    "live_event_field_fixtures",
    "cli_flag",
    "loc_audit_flag",
    "verify_dev_stand_hook",
    "audit_stand_smoke_docs",
    "ratio_hold",
    "band_close",
];

/// FM §5.56 band-75 marker rows.
pub const FM_BAND75_ROWS: &[&str] = &[
    "5.56",
    "Audit stand smoke",
    "PH-S1389…S1398",
    "audit_stand_smoke_depth",
];

/// Audit stand-smoke adoption markers for band 75.
pub const AUDIT_STAND_SMOKE_BAND75_ROWS: &[&str] = &[
    "PH-S1389",
    "audit_stand_smoke_depth",
    "PH-S1390",
    "smoke_audit_store_wire",
    "PH-S1391",
    "smoke_audit_events_query",
    "PH-S1392",
    "smoke_audit_event_field_fixtures",
    "PH-S1393",
    "--audit-stand-smoke",
    "PH-S1395",
    "VERIFY_AUDIT_STAND_SMOKE",
    "PH-S1398",
];

/// Classify audit stand-smoke band depth from optional feature stub (PH-S1389).
pub fn audit_stand_smoke_depth_stub(features: Option<&Value>) -> AuditStandSmokeDepth {
    let Some(f) = features else {
        return AuditStandSmokeDepth::None;
    };
    let depth = f
        .get("audit_stand_smoke_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let store = f
        .get("live_store")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let events = f
        .get("live_events_query")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let fixtures = f
        .get("live_event_field_fixtures")
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
        .get("audit_stand_smoke_docs")
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

    if depth && store && events && fixtures && cli && loc && verify && docs && ratio && close {
        return AuditStandSmokeDepth::FullBand75;
    }
    if close || ratio {
        return AuditStandSmokeDepth::RatioHold;
    }
    if docs {
        return AuditStandSmokeDepth::DocsCanon;
    }
    if verify {
        return AuditStandSmokeDepth::VerifyDevStandHook;
    }
    if loc {
        return AuditStandSmokeDepth::LocAuditFlag;
    }
    if cli {
        return AuditStandSmokeDepth::CliFlag;
    }
    if fixtures {
        return AuditStandSmokeDepth::LiveEventFieldFixtures;
    }
    if events {
        return AuditStandSmokeDepth::LiveEventsQuery;
    }
    if store {
        return AuditStandSmokeDepth::LiveStore;
    }
    if depth {
        return AuditStandSmokeDepth::DepthModule;
    }
    AuditStandSmokeDepth::None
}

/// Total audit stand-smoke criteria in registry (PH-S1389).
pub fn audit_stand_smoke_criteria_total() -> usize {
    AUDIT_STAND_SMOKE_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn audit_stand_smoke_depth_stub_ph_s1389() {
        assert_eq!(
            audit_stand_smoke_depth_stub(None),
            AuditStandSmokeDepth::None
        );
        assert_eq!(
            audit_stand_smoke_depth_stub(Some(&json!({"audit_stand_smoke_depth": true}))),
            AuditStandSmokeDepth::DepthModule
        );
        assert_eq!(
            audit_stand_smoke_depth_stub(Some(&json!({
                "audit_stand_smoke_depth": true,
                "live_store": true,
                "live_events_query": true,
                "live_event_field_fixtures": true,
                "cli_flag": true,
                "loc_audit_flag": true,
                "verify_dev_stand_hook": true,
                "audit_stand_smoke_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            AuditStandSmokeDepth::FullBand75
        );
        assert_eq!(AUDIT_STAND_SMOKE_CRITERIA.len(), 10);
        assert_eq!(audit_stand_smoke_criteria_total(), 10);
        assert!(FM_BAND75_ROWS.contains(&"PH-S1389…S1398"));
    }
}
