//! Galaxy `network_profile` depth classification stub (PH-S734, §8.1).

use serde_json::Value;

use crate::grid::galaxy_network_profile::{parse_network_profile_value, GalaxyEgressPolicy};

/// Network profile telemetry depth (Galaxy §8.1 egress/locality).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkProfileDepth {
    None,
    LocalityOnly,
    EgressRestricted,
    FullTelemetry,
}

/// Classify profile depth from optional wire stub (PH-S734).
pub fn network_profile_depth_stub(profile: Option<&Value>) -> NetworkProfileDepth {
    let Some(raw) = profile else {
        return NetworkProfileDepth::None;
    };
    let Ok(parsed) = parse_network_profile_value(raw) else {
        return NetworkProfileDepth::None;
    };
    let has_bandwidth = parsed.bandwidth_mbps.is_some();
    let has_egress = parsed.egress_policy.is_some();
    let has_measured = parsed.last_measured_at.is_some();
    match parsed.egress_policy {
        Some(GalaxyEgressPolicy::LanOnly) | Some(GalaxyEgressPolicy::WhiteIp) => {
            NetworkProfileDepth::EgressRestricted
        }
        _ if has_bandwidth && has_egress && has_measured => NetworkProfileDepth::FullTelemetry,
        _ if has_egress => NetworkProfileDepth::EgressRestricted,
        _ => NetworkProfileDepth::LocalityOnly,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn network_profile_depth_stub_ph_s734() {
        assert_eq!(network_profile_depth_stub(None), NetworkProfileDepth::None);
        assert_eq!(
            network_profile_depth_stub(Some(&json!({
                "region": "eu-west",
                "latency_ms_p50": 20
            }))),
            NetworkProfileDepth::LocalityOnly
        );
        assert_eq!(
            network_profile_depth_stub(Some(&json!({
                "region": "eu-west",
                "latency_ms_p50": 20,
                "egress_policy": "lan_only"
            }))),
            NetworkProfileDepth::EgressRestricted
        );
        assert_eq!(
            network_profile_depth_stub(Some(&json!({
                "region": "us-east",
                "latency_ms_p50": 40,
                "bandwidth_mbps": 1000,
                "egress_policy": "direct",
                "last_measured_at": "2026-06-20T12:00:00Z"
            }))),
            NetworkProfileDepth::FullTelemetry
        );
    }
}
