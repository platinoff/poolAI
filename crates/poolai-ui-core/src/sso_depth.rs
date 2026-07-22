//! SSO depth scaffold (PH-S1249…S1258, band 61 — enterprise phase B).

use serde_json::Value;

/// SSO depth flags (production verify stub + ops hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SsoDepth {
    None,
    DepthModule,
    StoreWire,
    ApiContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    FullBand61,
}

/// SSO criteria registry (PH-S1249): id · marker · doc path.
pub const SSO_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "sso_depth",
        "SsoDepth",
        "crates/poolai-ui-core/src/sso_depth.rs",
    ),
    (
        "store_wire",
        "POOLAI_SSO_STORE",
        "src/enterprise/security.rs",
    ),
    (
        "api_contracts",
        "sso_depth_audit",
        "tests/sso_depth_audit.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_SSO",
        "bin/verify-dev-stand.sh",
    ),
    ("quick_flag", "--sso", "bin/run-poolai.sh"),
    (
        "stand_smoke_export",
        "sso_band61_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    ("loc_audit_flag", "--sso", "src/bin/poolai_loc_audit.rs"),
    ("sso_docs", "SSO_DEPTH.md", "docs/development/SSO_DEPTH.md"),
];

/// `poolai-loc-audit --sso` case names (PH-S1254).
pub const SSO_CASES: &[&str] = &[
    "sso_depth",
    "store_wire",
    "api_contracts",
    "verify_dev_stand_hook",
    "quick_flag",
    "stand_smoke_export",
    "loc_audit_flag",
    "sso_docs",
];

/// FM §5.42 band-61 marker rows.
pub const FM_BAND61_ROWS: &[&str] = &["5.42", "SSO depth", "PH-S1249…S1258", "sso_depth"];

/// SSO adoption markers for band 61.
pub const SSO_BAND61_ROWS: &[&str] = &[
    "PH-S1249",
    "sso_depth",
    "PH-S1250",
    "POOLAI_SSO_STORE",
    "PH-S1252",
    "VERIFY_SSO",
    "PH-S1254",
    "--sso",
    "PH-S1258",
];

/// Classify SSO band depth from optional feature stub (PH-S1249).
pub fn sso_depth_stub(features: Option<&Value>) -> SsoDepth {
    let Some(f) = features else {
        return SsoDepth::None;
    };
    let depth = f
        .get("sso_depth")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let store = f
        .get("store_wire")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let api = f
        .get("api_contracts")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let verify = f
        .get("verify_dev_stand_hook")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let smoke = f
        .get("stand_smoke_export")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let loc = f
        .get("loc_audit_flag")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let docs = f.get("sso_docs").and_then(|v| v.as_bool()).unwrap_or(false);

    if depth && store && api && verify && smoke && loc && docs {
        return SsoDepth::FullBand61;
    }
    if docs {
        return SsoDepth::DocsCanon;
    }
    if loc {
        return SsoDepth::LocAuditFlag;
    }
    if smoke {
        return SsoDepth::StandSmokeExport;
    }
    if verify {
        return SsoDepth::VerifyDevStandHook;
    }
    if api {
        return SsoDepth::ApiContracts;
    }
    if store {
        return SsoDepth::StoreWire;
    }
    if depth {
        return SsoDepth::DepthModule;
    }
    SsoDepth::None
}

/// Total SSO criteria in registry (PH-S1249).
pub fn sso_criteria_total() -> usize {
    SSO_CRITERIA.len()
}

/// Env key for future durable SSO provider store (PH-S1249 scaffold).
pub const SSO_STORE_ENV: &str = "POOLAI_SSO_STORE";

/// Canonical store modes for band 61+ (memory default; sqlite horizon band 62+).
pub const SSO_STORE_MODES: &[&str] = &["memory", "sqlite"];

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn sso_depth_stub_ph_s1249() {
        assert_eq!(sso_depth_stub(None), SsoDepth::None);
        assert_eq!(
            sso_depth_stub(Some(&json!({"sso_depth": true}))),
            SsoDepth::DepthModule
        );
        assert_eq!(
            sso_depth_stub(Some(&json!({
                "sso_depth": true,
                "store_wire": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "sso_docs": true,
            }))),
            SsoDepth::FullBand61
        );
        assert_eq!(SSO_CRITERIA.len(), 8);
        assert_eq!(sso_criteria_total(), 8);
        assert_eq!(SSO_STORE_ENV, "POOLAI_SSO_STORE");
        assert!(SSO_STORE_MODES.contains(&"sqlite"));
        assert!(FM_BAND61_ROWS.contains(&"PH-S1249…S1258"));
    }
}
