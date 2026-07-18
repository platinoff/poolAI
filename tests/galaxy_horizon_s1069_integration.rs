//! PH-S1078: Galaxy horizon close band 43 — Grid metrics parity hardening.

use poolai_ui_core::grid_metrics_parity_depth::{
    grid_metrics_parity_depth_stub, GridMetricsParityDepth, FM_BAND43_ROWS,
    GRID_METRICS_PARITY_BAND43_ROWS,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s1069_band_grid_metrics_parity_close_ph_s1078() {
    assert_eq!(
        grid_metrics_parity_depth_stub(Some(&json!({"verification_extended": true}))),
        GridMetricsParityDepth::VerificationExtended
    );
    assert_eq!(
        grid_metrics_parity_depth_stub(Some(&json!({
            "verification_extended": true,
            "replication_extended": true,
            "pricing_extended": true,
            "prefetch_extended": true,
            "settlement_trust_extended": true,
            "stand_smoke_v3": true,
            "parity_contracts": true,
            "docs_canon": true
        }))),
        GridMetricsParityDepth::FullGridMetricsParity
    );

    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    for row in FM_BAND43_ROWS {
        assert!(fm.contains(row), "FM missing band-43 row {row}");
    }
    assert!(fm.contains("PH-S1078"));
    assert!(fm.contains("5.24"));

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(handoff.contains("PH-S1069") || handoff.contains("band 43"));

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(next.contains("абракадабра"));

    let parity = include_str!("../src/grid/stand_smoke_metrics_parity.rs");
    assert!(parity.contains("validate_band6_metrics_parity_v3"));
    assert!(parity.contains("VERIFICATION_EXTENDED_PARITY"));

    let stand_smoke = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(stand_smoke.contains("grid_metrics_json_prometheus_parity_band6_v3"));

    let contracts = include_str!("../tests/grid_metrics_parity_contracts.rs");
    assert!(contracts.contains("ph_s1074"));

    for marker in GRID_METRICS_PARITY_BAND43_ROWS {
        assert!(
            fm.contains(marker)
                || parity.contains(marker)
                || stand_smoke.contains(marker)
                || contracts.contains(marker),
            "band-43 marker missing: {marker}"
        );
    }

    assert!(Path::new("crates/poolai-ui-core/src/grid_metrics_parity_depth.rs").exists());
    assert!(Path::new("tests/grid_metrics_parity_contracts.rs").exists());
}
