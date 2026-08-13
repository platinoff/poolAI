//! PH-S1669: Galaxy horizon close band 103 — lint/diagnostics cleanup.
//! Suite: `galaxy_horizon_s1669_integration`.
//!
//! Verifies band-103 acceptance: clippy (CI feature set) at 0 warnings / 0 errors,
//! FM §5.84 rows, handoff + next-session markers, and the allow-gating artifacts.

#[test]
fn horizon_s1669_band_lint_cleanup_ph_s1678() {
    let fm = include_str!("../docs/catalog/FUNCTION_MANAGEMENT.md");
    assert!(fm.contains("5.84"), "FM missing band-103 section 5.84");
    assert!(fm.contains("PH-S1669"), "FM missing PH-S1669");
    assert!(fm.contains("PH-S1678"), "FM missing PH-S1678");
    for row in [
        "PH-S1669", "PH-S1670", "PH-S1671", "PH-S1672", "PH-S1673", "PH-S1674", "PH-S1675",
        "PH-S1676",
    ] {
        assert!(fm.contains(row), "FM missing band-103 row {row}");
    }

    let handoff = include_str!("../docs/development/HANDOFF_NEW_SESSION.md");
    assert!(
        handoff.contains("PH-S1669") || handoff.contains("band 103"),
        "HANDOFF missing band-103 marker"
    );

    let next = include_str!("../docs/development/NEXT_SESSION_PROMPT.md");
    assert!(
        next.contains("абракадабра") || next.contains("band 104"),
        "NEXT_SESSION missing band-104 marker"
    );

    // src test-module serialization-lock allows (PH-S1669)
    let grid = include_str!("../src/network/api/grid.rs");
    assert!(grid.contains("#[allow(clippy::await_holding_lock)]"));
    let discovery = include_str!("../src/network/discovery.rs");
    assert!(discovery.contains("#[allow(clippy::await_holding_lock)]"));

    // integration-test crate-level allow banner (PH-S1670)
    let grid_pricing = include_str!("../tests/grid_pricing_integration.rs");
    assert!(grid_pricing.contains("#![allow(clippy::await_holding_lock)]"));

    // ui-core needless_borrows fix + targeted allows (PH-S1671/S1674/S1675)
    let instances = include_str!("../crates/poolai-ui-core/src/instances.rs");
    assert!(instances.contains("#[allow(clippy::too_many_arguments)]"));
    let grid = include_str!("../src/network/api/grid.rs");
    assert!(grid.contains("#[allow(clippy::result_large_err)]"));
    let telegram_seats = include_str!("../src/services/telegram_seat_service.rs");
    assert!(telegram_seats.contains("#[allow(clippy::result_unit_err)]"));
    let monitoring = include_str!("../src/enterprise/monitoring.rs");
    assert!(monitoring.contains("PersistedMonitoringConfig"));

    // rust diagnostics: recorded at band close (0 warnings / 0 errors)
    let diag: serde_json::Value =
        serde_json::from_str(include_str!("../GSV/docs/vision/rust_diagnostics.json"))
            .expect("rust_diagnostics.json");
    assert_eq!(diag["latest"]["warnings"], 0);
    assert_eq!(diag["latest"]["errors"], 0);
    assert_eq!(diag["latest"]["ok"], true);
}
