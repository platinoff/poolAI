//! PH-S990…S994: Integration test gap fill audit — archived Playwright API-smoke → Rust wire canon.

use poolai_ui_core::integration_gap_depth::{
    integration_gap_depth_stub, IntegrationGapDepth, INTEGRATION_GAP_BAND34_CANON,
    INTEGRATION_GAP_BAND34_ROWS, JOBS_RAID_RESTART_STAND_SMOKE,
};
use serde_json::json;
use std::path::Path;

#[test]
fn integration_gap_audit_band34_ph_s990_s994() {
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

    for (sprint, archived, rust_canon) in INTEGRATION_GAP_BAND34_CANON {
        if archived.ends_with(".spec.ts") {
            let archive_path = format!("e2e/archive/api-smoke/{archived}");
            assert!(
                Path::new(&archive_path).is_file(),
                "{sprint}: archived API spec missing: {archive_path}"
            );
        }
        assert!(
            Path::new(rust_canon).is_file(),
            "{sprint}: Rust wire canon missing for {archived}: {rust_canon}"
        );
    }

    let archive_readme = include_str!("../e2e/archive/api-smoke/README.md");
    assert!(archive_readme.contains("telegram_wallet.spec.ts"));
    assert!(archive_readme.contains("tests/telegram_wallet_integration.rs"));

    let e2e_scope = include_str!("integration_gap_audit.rs");
    assert!(e2e_scope.contains("PH-S990"));

    let stand_smoke_src = include_str!("../src/bin/poolai_http_stand_smoke.rs");
    assert!(stand_smoke_src.contains("jobs_raid_restart"));
    assert!(stand_smoke_src.contains("--raid-restart"));
    assert!(JOBS_RAID_RESTART_STAND_SMOKE.contains("raid-restart"));

    let e2e_playwright = include_str!("../bin/e2e-playwright.sh");
    assert!(e2e_playwright.contains("poolai-http-stand-smoke --raid-restart"));

    let policy = include_str!("../.cursor/rules/poolai-testing-policy.mdc");
    assert!(policy.contains("band 34"));
    assert!(policy.contains("integration_gap_audit.rs"));

    let roadmap = include_str!("../docs/development/GALAXY_GRID_ROADMAP_2026-05-27.md");
    for row in INTEGRATION_GAP_BAND34_ROWS {
        assert!(
            roadmap.contains(row),
            "GALAXY_GRID_ROADMAP missing band-34 row {row}"
        );
    }
}
