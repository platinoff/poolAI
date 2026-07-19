//! CI canon gate band depth (PH-S1139…S1148, band 50).

use serde_json::Value;

/// Local CI canon gate depth flags (test-ci + openapi-gap + rust-ratio advisory).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CiCanonDepth {
    None,
    TestCiScope,
    OpenapiGapAudit,
    RustRatioAudit,
    CiYmlJobs,
    VerifyDevStandHook,
    CiCanonDocs,
    DualGate,
    FullBand50,
}

/// CI canon gate criteria registry (PH-S1141): id · marker · doc path.
pub const CI_CANON_CRITERIA: &[(&str, &str, &str)] = &[
    ("test_ci_scope", "test-ci =", ".cargo/config.toml"),
    (
        "openapi_gap_audit",
        "poolai-openapi-gap-audit",
        "src/bin/poolai_openapi_gap_audit.rs",
    ),
    (
        "rust_ratio_audit",
        "rust-ratio-audit",
        ".github/workflows/ci.yml",
    ),
    (
        "openapi_gap_ci_job",
        "openapi-gap-audit",
        ".github/workflows/ci.yml",
    ),
    (
        "verify_dev_stand_hook",
        "VERIFY_CI_CANON",
        "bin/verify-dev-stand.sh",
    ),
    (
        "ci_canon_docs",
        "CI_CANON.md",
        "docs/development/CI_CANON.md",
    ),
    (
        "dual_gate",
        "PH-S1004",
        ".cursor/rules/poolai-testing-policy.mdc",
    ),
];

/// `poolai-loc-audit --ci-canon` case names (PH-S1140).
pub const CI_CANON_CASES: &[&str] = &[
    "test_ci_scope",
    "openapi_gap_audit",
    "rust_ratio_audit",
    "openapi_gap_ci_job",
    "verify_dev_stand_hook",
    "ci_canon_docs",
    "dual_gate",
];

/// FM §5.31 band-50 marker rows.
pub const FM_BAND50_ROWS: &[&str] = &["5.31", "CI canon gate", "PH-S1139…S1148", "ci_canon_depth"];

/// CI canon gate adoption markers for band 50.
pub const CI_CANON_BAND50_ROWS: &[&str] = &[
    "PH-S1139",
    "ci_canon_depth",
    "PH-S1140",
    "--ci-canon",
    "PH-S1142",
    "VERIFY_CI_CANON",
    "PH-S1143",
    "--ci-canon",
    "PH-S1148",
];

/// Classify CI canon band depth from optional feature stub (PH-S1139).
pub fn ci_canon_depth_stub(features: Option<&Value>) -> CiCanonDepth {
    let Some(f) = features else {
        return CiCanonDepth::None;
    };
    let test_ci = f
        .get("test_ci_scope")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let gap = f
        .get("openapi_gap_audit")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let ratio = f
        .get("rust_ratio_audit")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let ci_yml = f
        .get("openapi_gap_ci_job")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let verify = f
        .get("verify_dev_stand_hook")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let docs = f
        .get("ci_canon_docs")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let dual = f
        .get("dual_gate")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if test_ci && gap && ratio && ci_yml && verify && docs && dual {
        return CiCanonDepth::FullBand50;
    }
    if dual {
        return CiCanonDepth::DualGate;
    }
    if docs {
        return CiCanonDepth::CiCanonDocs;
    }
    if verify {
        return CiCanonDepth::VerifyDevStandHook;
    }
    if ci_yml {
        return CiCanonDepth::CiYmlJobs;
    }
    if ratio {
        return CiCanonDepth::RustRatioAudit;
    }
    if gap {
        return CiCanonDepth::OpenapiGapAudit;
    }
    if test_ci {
        return CiCanonDepth::TestCiScope;
    }
    CiCanonDepth::None
}

/// Total CI canon criteria in registry (PH-S1141).
pub fn ci_canon_criteria_total() -> usize {
    CI_CANON_CRITERIA.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn ci_canon_depth_stub_ph_s1139() {
        assert_eq!(ci_canon_depth_stub(None), CiCanonDepth::None);
        assert_eq!(
            ci_canon_depth_stub(Some(&json!({"test_ci_scope": true}))),
            CiCanonDepth::TestCiScope
        );
        assert_eq!(
            ci_canon_depth_stub(Some(&json!({
                "test_ci_scope": true,
                "openapi_gap_audit": true,
                "rust_ratio_audit": true,
                "openapi_gap_ci_job": true,
                "verify_dev_stand_hook": true,
                "ci_canon_docs": true,
                "dual_gate": true,
            }))),
            CiCanonDepth::FullBand50
        );
        assert_eq!(CI_CANON_CRITERIA.len(), 7);
        assert_eq!(ci_canon_criteria_total(), 7);
        assert!(!CI_CANON_CASES.is_empty());
        assert!(FM_BAND50_ROWS.contains(&"PH-S1139…S1148"));
    }
}
