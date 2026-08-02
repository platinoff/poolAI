//! Galaxy Grid routing policy locality gate (PH-S721, §4.1).

use crate::core::error::AppError;
use crate::grid::dispatch::check_strict_locality_gate;
use serde::{Deserialize, Serialize};

/// Strict routing locality gate verdict (Galaxy §4.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoutingPolicyLocalityVerdict {
    Allowed,
    StrictLocalityBlock,
}

/// Evaluate strict routing locality gate without propagating HTTP error (PH-S721).
#[inline]
pub fn routing_policy_locality_gate(required_shard_ids: &[String]) -> RoutingPolicyLocalityVerdict {
    match check_strict_locality_gate(required_shard_ids) {
        Ok(()) => RoutingPolicyLocalityVerdict::Allowed,
        Err(_) => RoutingPolicyLocalityVerdict::StrictLocalityBlock,
    }
}

/// Strict routing gate that returns coordinator error when blocked (PH-S721 hook).
#[inline]
pub fn enforce_routing_policy_locality_gate(required_shard_ids: &[String]) -> Result<(), AppError> {
    check_strict_locality_gate(required_shard_ids)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn routing_policy_locality_gate_ph_s721() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        std::env::remove_var(crate::grid::dispatch::ENV_LOCALITY_MODE);

        assert_eq!(
            routing_policy_locality_gate(&[]),
            RoutingPolicyLocalityVerdict::Allowed
        );
        assert_eq!(
            routing_policy_locality_gate(&["w:missing".into()]),
            RoutingPolicyLocalityVerdict::Allowed
        );

        std::env::set_var(crate::grid::dispatch::ENV_LOCALITY_MODE, "strict_locality");
        assert_eq!(
            routing_policy_locality_gate(&["w:missing-shard".into()]),
            RoutingPolicyLocalityVerdict::StrictLocalityBlock
        );
        std::env::remove_var(crate::grid::dispatch::ENV_LOCALITY_MODE);
    }
}
