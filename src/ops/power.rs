//! Dev-stand power ops (`POST /api/v1/ops/power`, PH-S1016).

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, Ordering};

static POWER_OPS_INVOCATIONS: AtomicU32 = AtomicU32::new(0);

/// Power action requested by admin / vision UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PowerAction {
    Shutdown,
    Reboot,
}

impl PowerAction {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "shutdown" => Some(Self::Shutdown),
            "reboot" => Some(Self::Reboot),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Shutdown => "shutdown",
            Self::Reboot => "reboot",
        }
    }
}

/// Request body for `POST /api/v1/ops/power`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PowerRequest {
    pub action: String,
}

/// Accepted power op response (dev-stand safe — no host reboot).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PowerResponse {
    pub accepted: bool,
    pub action: &'static str,
    pub dev_guard: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<&'static str>,
}

/// Execute power op in dev-stand mode (never reboots host).
pub fn apply_power_action(action: PowerAction) -> PowerResponse {
    POWER_OPS_INVOCATIONS.fetch_add(1, Ordering::SeqCst);
    match action {
        PowerAction::Shutdown => PowerResponse {
            accepted: true,
            action: "shutdown",
            dev_guard: true,
            note: Some(
                "graceful shutdown queued; host process exit only when POOLAI_OPS_POWER_EXECUTE=1",
            ),
        },
        PowerAction::Reboot => PowerResponse {
            accepted: true,
            action: "reboot",
            dev_guard: true,
            note: Some("host reboot skipped in dev stand"),
        },
    }
}

/// Test / integration hook — invocation count since process start.
pub fn power_ops_invocation_count() -> u32 {
    POWER_OPS_INVOCATIONS.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_action_parse_and_apply_ph_s1016() {
        assert_eq!(PowerAction::parse("shutdown"), Some(PowerAction::Shutdown));
        assert_eq!(PowerAction::parse("reboot"), Some(PowerAction::Reboot));
        assert_eq!(PowerAction::parse("invalid"), None);
        let resp = apply_power_action(PowerAction::Shutdown);
        assert!(resp.accepted);
        assert!(resp.dev_guard);
    }
}
