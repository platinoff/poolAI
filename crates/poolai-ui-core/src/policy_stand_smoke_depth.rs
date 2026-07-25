//! Policies live stand-smoke band depth (PH-S1489…S1498, band 85 — enterprise phase D).

use serde_json::Value;

/// Policies stand-smoke depth flags (live HTTP / CLI / verify / docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyStandSmokeDepth {
    None,
    DepthModule,
    LiveStore,
    LivePoliciesQuery,
    LivePolicyFieldFixtures,
    CliFlag,
    LocAuditFlag,
    VerifyDevStandHook,
    DocsCanon,
    RatioHold,
    FullBand85,
}

/// Policies stand-smoke criteria registry (PH-S1489): id · marker · doc path.
pub const POLICY_STAND_SMOKE_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "policy_stand_smoke_depth",
        "PolicyStandSmokeDepth",
        "crates/poolai-ui-core/src/policy_stand_smoke_depth.rs",
    ),
    (
        "live_store",
        "smoke_policy_store_wire",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "live_policies_query",
        "smoke_policy_policies_query",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "live_policy_field_fixtures",
        "smoke_policy_field_fixtures",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "cli_flag",
        "--policy-stand-smoke",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--policy-stand-smoke",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_POLICY_STAND_SMOKE",
        "bin/verify-dev-stand.sh",
    ),
    (
        "policy_stand_smoke_docs",
        "POLICIES_STAND_SMOKE.md",
        "docs/development/POLICIES_STAND_SMOKE.md",
    ),
    ("ratio_hold", "min-ratio", "docs/development/RUN_LOCAL.md"),
    (
        "band_close",
        "galaxy_horizon_s1489_integration",
        "tests/galaxy_horizon_s1489_integration.rs",
    ),
];

/// `poolai-loc-audit --policy-stand-smoke` case names (PH-S1494).
pub const POLICY_STAND_SMOKE_CASES: &[&str] = &[
    "policy_stand_smoke_depth",
    "live_store",
    "live_policies_query",
    "live_policy_field_fixtures",
    "cli_flag",
    "loc_audit_flag",
    "verify_dev_stand_hook",
    "policy_stand_smoke_docs",
    "ratio_hold",
    "band_close",
];

/// FM §5.66 band-85 marker rows.
pub const FM_BAND85_ROWS: &[&str] = &[
    "5.66",
    "Policies stand smoke",
    "PH-S1489…S1498",
    "policy_stand_smoke_depth",
];

/// Policies stand-smoke adoption markers for band 85.
pub const POLICY_STAND_SMOKE_BAND85_ROWS: &[&str] = &[
    "PH-S1489",
    "policy_stand_smoke_depth",
    "PH-S1490",
    "smoke_policy_store_wire",
    "PH-S1491",
    "smoke_policy_policies_query",
    "PH-S1492",
    "smoke_policy_field_fixtures",
    "PH-S1493",
    "--policy-stand-smoke",
    "PH-S1495",
    "VERIFY_POLICY_STAND_SMOKE",
    "PH-S1498",
];

/// Classify policies stand-smoke band depth from optional feature stub (PH-S1489).
pub fn policy_stand_smoke_depth_stub(features: Option<&Value>) -> PolicyStandSmokeDepth {
    let Some(f) = features else {
        return PolicyStandSmokeDepth::None;
    };
    let depth = f
        .get("policy_stand_smoke_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let store = f
        .get("live_store")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let policies = f
        .get("live_policies_query")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let fixtures = f
        .get("live_policy_field_fixtures")
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
        .get("policy_stand_smoke_docs")
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

    if depth && store && policies && fixtures && cli && loc && verify && docs && ratio && close {
        return PolicyStandSmokeDepth::FullBand85;
    }
    if close || ratio {
        return PolicyStandSmokeDepth::RatioHold;
    }
    if docs {
        return PolicyStandSmokeDepth::DocsCanon;
    }
    if verify {
        return PolicyStandSmokeDepth::VerifyDevStandHook;
    }
    if loc {
        return PolicyStandSmokeDepth::LocAuditFlag;
    }
    if cli {
        return PolicyStandSmokeDepth::CliFlag;
    }
    if fixtures {
        return PolicyStandSmokeDepth::LivePolicyFieldFixtures;
    }
    if policies {
        return PolicyStandSmokeDepth::LivePoliciesQuery;
    }
    if store {
        return PolicyStandSmokeDepth::LiveStore;
    }
    if depth {
        return PolicyStandSmokeDepth::DepthModule;
    }
    PolicyStandSmokeDepth::None
}

/// Total policies stand-smoke criteria in registry (PH-S1489).
pub fn policy_stand_smoke_criteria_total() -> usize {
    POLICY_STAND_SMOKE_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn policy_stand_smoke_depth_stub_ph_s1489() {
        assert_eq!(
            policy_stand_smoke_depth_stub(None),
            PolicyStandSmokeDepth::None
        );
        assert_eq!(
            policy_stand_smoke_depth_stub(Some(&json!({"policy_stand_smoke_depth": true}))),
            PolicyStandSmokeDepth::DepthModule
        );
        assert_eq!(
            policy_stand_smoke_depth_stub(Some(&json!({
                "policy_stand_smoke_depth": true,
                "live_store": true,
                "live_policies_query": true,
                "live_policy_field_fixtures": true,
                "cli_flag": true,
                "loc_audit_flag": true,
                "verify_dev_stand_hook": true,
                "policy_stand_smoke_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            PolicyStandSmokeDepth::FullBand85
        );
        assert_eq!(POLICY_STAND_SMOKE_CRITERIA.len(), 10);
        assert_eq!(policy_stand_smoke_criteria_total(), 10);
        assert!(FM_BAND85_ROWS.contains(&"PH-S1489…S1498"));
    }
}
