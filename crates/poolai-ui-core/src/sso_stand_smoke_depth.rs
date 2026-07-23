//! SSO live stand-smoke band depth (PH-S1289…S1298, band 65 — enterprise phase B).

use serde_json::Value;

/// SSO stand-smoke depth flags (live HTTP / CLI / verify / docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SsoStandSmokeDepth {
    None,
    DepthModule,
    LiveStore,
    LiveCrud,
    LiveCallbackFixtures,
    CliFlag,
    LocAuditFlag,
    VerifyDevStandHook,
    DocsCanon,
    RatioHold,
    FullBand65,
}

/// SSO stand-smoke criteria registry (PH-S1289): id · marker · doc path.
pub const SSO_STAND_SMOKE_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "sso_stand_smoke_depth",
        "SsoStandSmokeDepth",
        "crates/poolai-ui-core/src/sso_stand_smoke_depth.rs",
    ),
    (
        "live_store",
        "smoke_sso_store_wire",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "live_crud",
        "smoke_sso_oauth2_saml_crud",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "live_callback_fixtures",
        "smoke_sso_callback_fixtures",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "cli_flag",
        "--sso-stand-smoke",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--sso-stand-smoke",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_SSO_STAND_SMOKE",
        "bin/verify-dev-stand.sh",
    ),
    (
        "sso_stand_smoke_docs",
        "SSO_STAND_SMOKE.md",
        "docs/development/SSO_STAND_SMOKE.md",
    ),
    ("ratio_hold", "min-ratio", "docs/development/RUN_LOCAL.md"),
    (
        "band_close",
        "galaxy_horizon_s1289_integration",
        "tests/galaxy_horizon_s1289_integration.rs",
    ),
];

/// `poolai-loc-audit --sso-stand-smoke` case names (PH-S1294).
pub const SSO_STAND_SMOKE_CASES: &[&str] = &[
    "sso_stand_smoke_depth",
    "live_store",
    "live_crud",
    "live_callback_fixtures",
    "cli_flag",
    "loc_audit_flag",
    "verify_dev_stand_hook",
    "sso_stand_smoke_docs",
    "ratio_hold",
    "band_close",
];

/// FM §5.46 band-65 marker rows.
pub const FM_BAND65_ROWS: &[&str] = &[
    "5.46",
    "SSO stand smoke",
    "PH-S1289…S1298",
    "sso_stand_smoke_depth",
];

/// SSO stand-smoke adoption markers for band 65.
pub const SSO_STAND_SMOKE_BAND65_ROWS: &[&str] = &[
    "PH-S1289",
    "sso_stand_smoke_depth",
    "PH-S1290",
    "smoke_sso_store_wire",
    "PH-S1291",
    "smoke_sso_oauth2_saml_crud",
    "PH-S1292",
    "smoke_sso_callback_fixtures",
    "PH-S1293",
    "--sso-stand-smoke",
    "PH-S1295",
    "VERIFY_SSO_STAND_SMOKE",
    "PH-S1298",
];

/// Classify SSO stand-smoke band depth from optional feature stub (PH-S1289).
pub fn sso_stand_smoke_depth_stub(features: Option<&Value>) -> SsoStandSmokeDepth {
    let Some(f) = features else {
        return SsoStandSmokeDepth::None;
    };
    let depth = f
        .get("sso_stand_smoke_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let store = f
        .get("live_store")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let crud = f
        .get("live_crud")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let callbacks = f
        .get("live_callback_fixtures")
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
        .get("sso_stand_smoke_docs")
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

    if depth && store && crud && callbacks && cli && loc && verify && docs && ratio && close {
        return SsoStandSmokeDepth::FullBand65;
    }
    if close || ratio {
        return SsoStandSmokeDepth::RatioHold;
    }
    if docs {
        return SsoStandSmokeDepth::DocsCanon;
    }
    if verify {
        return SsoStandSmokeDepth::VerifyDevStandHook;
    }
    if loc {
        return SsoStandSmokeDepth::LocAuditFlag;
    }
    if cli {
        return SsoStandSmokeDepth::CliFlag;
    }
    if callbacks {
        return SsoStandSmokeDepth::LiveCallbackFixtures;
    }
    if crud {
        return SsoStandSmokeDepth::LiveCrud;
    }
    if store {
        return SsoStandSmokeDepth::LiveStore;
    }
    if depth {
        return SsoStandSmokeDepth::DepthModule;
    }
    SsoStandSmokeDepth::None
}

/// Total SSO stand-smoke criteria in registry (PH-S1289).
pub fn sso_stand_smoke_criteria_total() -> usize {
    SSO_STAND_SMOKE_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn sso_stand_smoke_depth_stub_ph_s1289() {
        assert_eq!(sso_stand_smoke_depth_stub(None), SsoStandSmokeDepth::None);
        assert_eq!(
            sso_stand_smoke_depth_stub(Some(&json!({"sso_stand_smoke_depth": true}))),
            SsoStandSmokeDepth::DepthModule
        );
        assert_eq!(
            sso_stand_smoke_depth_stub(Some(&json!({
                "sso_stand_smoke_depth": true,
                "live_store": true,
                "live_crud": true,
                "live_callback_fixtures": true,
                "cli_flag": true,
                "loc_audit_flag": true,
                "verify_dev_stand_hook": true,
                "sso_stand_smoke_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            SsoStandSmokeDepth::FullBand65
        );
        assert_eq!(SSO_STAND_SMOKE_CRITERIA.len(), 10);
        assert_eq!(sso_stand_smoke_criteria_total(), 10);
        assert!(FM_BAND65_ROWS.contains(&"PH-S1289…S1298"));
    }
}
