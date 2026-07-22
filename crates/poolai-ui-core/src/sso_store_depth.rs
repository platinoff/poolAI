//! SSO store-wire band depth (PH-S1259…S1268, band 62 — enterprise phase B).

use serde_json::Value;

/// SSO store-wire depth flags (durable path wire + ops hooks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SsoStoreDepth {
    None,
    DepthModule,
    StoreWire,
    ApiContracts,
    VerifyDevStandHook,
    StandSmokeExport,
    LocAuditFlag,
    DocsCanon,
    FullBand62,
}

/// SSO store-wire criteria registry (PH-S1259): id · marker · doc path.
pub const SSO_STORE_CRITERIA: &[(&str, &str, &str)] = &[
    (
        "sso_store_depth",
        "SsoStoreDepth",
        "crates/poolai-ui-core/src/sso_store_depth.rs",
    ),
    ("store_wire", "sso_store_wire", "src/enterprise/security.rs"),
    (
        "api_contracts",
        "sso_store_wire_integration",
        "tests/sso_store_wire_integration.rs",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_SSO_STORE",
        "bin/verify-dev-stand.sh",
    ),
    (
        "stand_smoke_export",
        "sso_store_band62_export_shape",
        "src/bin/poolai_http_stand_smoke.rs",
    ),
    (
        "loc_audit_flag",
        "--sso-store",
        "src/bin/poolai_loc_audit.rs",
    ),
    (
        "sso_store_docs",
        "SSO_STORE.md",
        "docs/development/SSO_STORE.md",
    ),
];

/// `poolai-loc-audit --sso-store` case names (PH-S1264).
pub const SSO_STORE_CASES: &[&str] = &[
    "sso_store_depth",
    "store_wire",
    "api_contracts",
    "verify_dev_stand_hook",
    "stand_smoke_export",
    "loc_audit_flag",
    "sso_store_docs",
];

/// FM §5.43 band-62 marker rows.
pub const FM_BAND62_ROWS: &[&str] = &[
    "5.43",
    "SSO store wire",
    "PH-S1259…S1268",
    "sso_store_depth",
];

/// SSO store-wire adoption markers for band 62.
pub const SSO_BAND62_ROWS: &[&str] = &[
    "PH-S1259",
    "sso_store_depth",
    "PH-S1260",
    "sso_store_wire",
    "PH-S1262",
    "VERIFY_SSO_STORE",
    "PH-S1264",
    "--sso-store",
    "PH-S1268",
];

/// Classify SSO store-wire band depth from optional feature stub (PH-S1259).
pub fn sso_store_depth_stub(features: Option<&Value>) -> SsoStoreDepth {
    let Some(f) = features else {
        return SsoStoreDepth::None;
    };
    let depth = f
        .get("sso_store_depth")
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
    let docs = f
        .get("sso_store_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if depth && store && api && verify && smoke && loc && docs {
        return SsoStoreDepth::FullBand62;
    }
    if docs {
        return SsoStoreDepth::DocsCanon;
    }
    if loc {
        return SsoStoreDepth::LocAuditFlag;
    }
    if smoke {
        return SsoStoreDepth::StandSmokeExport;
    }
    if verify {
        return SsoStoreDepth::VerifyDevStandHook;
    }
    if api {
        return SsoStoreDepth::ApiContracts;
    }
    if store {
        return SsoStoreDepth::StoreWire;
    }
    if depth {
        return SsoStoreDepth::DepthModule;
    }
    SsoStoreDepth::None
}

/// Total SSO store-wire criteria in registry (PH-S1259).
pub fn sso_store_criteria_total() -> usize {
    SSO_STORE_CRITERIA.len()
}

/// Env key for SSO store backend (shared with band 61 scaffold).
pub const SSO_STORE_ENV: &str = "POOLAI_SSO_STORE";

/// Env key for durable SSO data directory (band 62 wire).
pub const SSO_DATA_DIR_ENV: &str = "POOLAI_SSO_DATA_DIR";

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn sso_store_depth_stub_ph_s1259() {
        assert_eq!(sso_store_depth_stub(None), SsoStoreDepth::None);
        assert_eq!(
            sso_store_depth_stub(Some(&json!({"sso_store_depth": true}))),
            SsoStoreDepth::DepthModule
        );
        assert_eq!(
            sso_store_depth_stub(Some(&json!({
                "sso_store_depth": true,
                "store_wire": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "sso_store_docs": true,
            }))),
            SsoStoreDepth::FullBand62
        );
        assert_eq!(SSO_STORE_CRITERIA.len(), 7);
        assert_eq!(sso_store_criteria_total(), 7);
        assert_eq!(SSO_STORE_ENV, "POOLAI_SSO_STORE");
        assert_eq!(SSO_DATA_DIR_ENV, "POOLAI_SSO_DATA_DIR");
        assert!(FM_BAND62_ROWS.contains(&"PH-S1259…S1268"));
    }
}
