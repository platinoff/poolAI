//! Integration test gap fill band depth (PH-S990, band 34).

use serde_json::Value;

/// Integration gap band depth flags (archived Playwright → Rust wire canon).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationGapDepth {
    None,
    TelegramWallet,
    GridJobLease,
    ProtocolMiddleware,
    JobsRaidRestart,
    VmWriteLifecycle,
    FullGap,
}

/// Archived API-smoke spec → Rust integration canon (band 34, PH-S990…S994).
pub const INTEGRATION_GAP_BAND34_CANON: &[(&str, &str, &str)] = &[
    (
        "PH-S990",
        "telegram_wallet.spec.ts",
        "tests/telegram_wallet_integration.rs",
    ),
    (
        "PH-S991",
        "grid_job_lease.spec.ts",
        "tests/grid_envelope_lease_integration.rs",
    ),
    (
        "PH-S992",
        "protocol_middleware.spec.ts",
        "tests/protocol_middleware_integration.rs",
    ),
    (
        "PH-S993",
        "jobs_raid.spec.ts",
        "tests/job_store_raid_persistence.rs",
    ),
    ("PH-S994", "vm_write_lifecycle", "tests/vm_api_contracts.rs"),
];

/// Stand-smoke supplement for jobs RAID restart (PH-S993 / PH-S156).
pub const JOBS_RAID_RESTART_STAND_SMOKE: &str = "poolai-http-stand-smoke --raid-restart";

/// GALAXY_GRID_ROADMAP band-34 marker rows (PH-S996 docs cross-link).
pub const INTEGRATION_GAP_BAND34_ROWS: &[&str] = &[
    "band 34 PH-S990",
    "integration gap fill",
    "telegram_wallet_integration.rs",
    "grid_envelope_lease_integration.rs",
    "protocol_middleware_integration.rs",
    "job_store_raid_persistence.rs",
    "vm_api_contracts.rs",
];

/// Classify integration gap band depth from optional feature stub (PH-S990).
pub fn integration_gap_depth_stub(features: Option<&Value>) -> IntegrationGapDepth {
    let Some(f) = features else {
        return IntegrationGapDepth::None;
    };
    let wallet = f
        .get("telegram_wallet")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let lease = f
        .get("grid_job_lease")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let protocol = f
        .get("protocol_middleware")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let raid = f
        .get("jobs_raid_restart")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let vm = f
        .get("vm_write_lifecycle")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let flags = wallet as u8 + lease as u8 + protocol as u8 + raid as u8 + vm as u8;
    match flags {
        0 => IntegrationGapDepth::None,
        1 if wallet => IntegrationGapDepth::TelegramWallet,
        1 if lease => IntegrationGapDepth::GridJobLease,
        1 if protocol => IntegrationGapDepth::ProtocolMiddleware,
        1 if raid => IntegrationGapDepth::JobsRaidRestart,
        1 if vm => IntegrationGapDepth::VmWriteLifecycle,
        5 => IntegrationGapDepth::FullGap,
        _ => IntegrationGapDepth::FullGap,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn integration_gap_depth_stub_ph_s990() {
        assert_eq!(integration_gap_depth_stub(None), IntegrationGapDepth::None);
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
        assert_eq!(INTEGRATION_GAP_BAND34_CANON.len(), 5);
        assert_eq!(INTEGRATION_GAP_BAND34_ROWS.len(), 7);
    }
}
