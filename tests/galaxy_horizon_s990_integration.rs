//! PH-S999: Galaxy horizon close band (PH-S990…S998) — integration test gap fill.

use poolai_ui_core::integration_gap_depth::{
    integration_gap_depth_stub, IntegrationGapDepth, INTEGRATION_GAP_BAND34_CANON,
    INTEGRATION_GAP_BAND34_ROWS, JOBS_RAID_RESTART_STAND_SMOKE,
};
use serde_json::json;
use std::path::Path;

#[test]
fn horizon_s990_band_integration_gap_close_ph_s999() {
    assert_eq!(
        integration_gap_depth_stub(Some(&json!({"telegram_wallet": true}))),
        IntegrationGapDepth::TelegramWallet
    );
    assert_eq!(
        integration_gap_depth_stub(Some(&json!({
            "telegram_wallet": true,
            "grid_job_lease": true,
            "protocol_middleware": true,
            "jobs_raid_restart": true,
            "vm_write_lifecycle": true
        }))),
        IntegrationGapDepth::FullGap
    );

    for (sprint, _, rust_canon) in INTEGRATION_GAP_BAND34_CANON {
        assert!(
            Path::new(rust_canon).is_file(),
            "{sprint}: missing Rust canon {rust_canon}"
        );
    }

    let roadmap = include_str!("../docs/development/GALAXY_GRID_ROADMAP_2026-05-27.md");
    for row in INTEGRATION_GAP_BAND34_ROWS {
        assert!(
            roadmap.contains(row),
            "GALAXY_GRID_ROADMAP missing band-34 row {row}"
        );
    }
    assert!(roadmap.contains("PH-S990"));
    assert!(roadmap.contains("band 34"));

    let policy = include_str!("../.cursor/rules/poolai-testing-policy.mdc");
    assert!(policy.contains("band 34"));
    assert!(policy.contains("integration_gap_audit.rs"));

    assert!(Path::new("tests/integration_gap_audit.rs").exists());
    assert!(Path::new("tests/telegram_wallet_integration.rs").exists());
    assert!(JOBS_RAID_RESTART_STAND_SMOKE.contains("raid-restart"));

    let ratio_json: serde_json::Value =
        serde_json::from_str(include_str!("../docs/development/rust_ratio.json"))
            .expect("rust_ratio.json");
    assert_eq!(ratio_json["sprint"].as_str().unwrap(), "PH-S995");
    assert!(ratio_json["in_formal_band"].as_bool().unwrap_or(false));

    let notes = ratio_json["notes"].as_array().expect("notes");
    let joined = notes
        .iter()
        .filter_map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(joined.contains("PH-S998"));
}
