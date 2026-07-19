//! PH-S1128: Galaxy horizon close band 48 — edge verification horizon.

use poolai_ui_core::galaxy_edge_verification_depth::{
    edge_verification_criteria_total, galaxy_edge_verification_depth_stub,
    GalaxyEdgeVerificationDepth, EDGE_VERIFICATION_BAND48_ROWS, EDGE_VERIFICATION_CASES,
    EDGE_VERIFICATION_CRITERIA, FM_BAND48_ROWS,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1119_band_edge_verification_close_ph_s1128() {
    assert_eq!(
        galaxy_edge_verification_depth_stub(Some(&json!({"metrics_http": true}))),
        GalaxyEdgeVerificationDepth::MetricsHttp
    );
    assert_eq!(
        galaxy_edge_verification_depth_stub(Some(&json!({
            "fraud_proof_stub": true,
            "capability_admission": true,
            "network_profile_stale": true,
            "tee_attestation": true,
            "metrics_http": true,
            "stand_smoke_parity": true,
        }))),
        GalaxyEdgeVerificationDepth::FullBand48
    );

    assert_eq!(EDGE_VERIFICATION_CRITERIA.len(), 7);
    assert_eq!(edge_verification_criteria_total(), 7);
    assert!(EDGE_VERIFICATION_CASES.contains(&"stand_smoke_parity"));

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND48_ROWS {
        assert!(fm.contains(row), "FM missing band-48 row {row}");
    }
    assert!(fm.contains("PH-S1128"));
    assert!(fm.contains("5.29"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1119") || handoff.contains("band 48"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра") || next.contains("band 49"));

    let run_local = include_str!("../docs/development/RUN_LOCAL.md");
    assert!(run_local.contains("--edge-verification"));
    assert!(run_local.contains("VERIFY_EDGE_VERIFICATION"));

    let strategy = include_str!("../docs/development/RUST_RATIO_STRATEGY_2026-06-13.md");
    assert!(strategy.contains("galaxy_edge_verification_depth"));

    let verify = include_str!("../bin/verify-dev-stand.sh");
    assert!(verify.contains("VERIFY_EDGE_VERIFICATION"));
    assert!(verify.contains("--edge-verification-advisory"));

    let run_poolai = include_str!("../bin/run-poolai.sh");
    assert!(run_poolai.contains("--edge-verification"));

    let loc_audit = include_str!("../src/bin/poolai_loc_audit.rs");
    assert!(loc_audit.contains("edge_verification_advisory_mode"));
    assert!(loc_audit.contains("edge_verification_criteria_met_count"));

    let stand_smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(stand_smoke.contains("galaxy_edge_verification_band48_export_shape_ph_s1125"));

    for marker in EDGE_VERIFICATION_BAND48_ROWS {
        assert!(
            fm.contains(marker) || run_local.contains(marker) || loc_audit.contains(marker),
            "band-48 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/galaxy_edge_verification_depth.rs").exists());

    let ratio: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert!(ratio.get("edge_verification_advisory_mode").is_some());
    assert!(ratio.get("edge_verification_criteria_total").is_some());
}
