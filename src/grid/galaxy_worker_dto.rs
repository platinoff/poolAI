//! Unified Galaxy worker DTO for virtual-node list (PH-S507, Galaxy §2.3).

use crate::core::discovery_types::{PeerCapabilities, PeerInfo};
use crate::grid::dispatch::SeedInventoryEntry;
use crate::grid::galaxy_network_profile::{parse_network_profile_value, GalaxyNetworkProfile};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Worker origin class (Galaxy §2.3).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GalaxyWorkerOrigin {
    Local,
    Cloud,
    TelegramEdge,
    Unknown,
}

impl GalaxyWorkerOrigin {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Cloud => "cloud",
            Self::TelegramEdge => "telegram_edge",
            Self::Unknown => "unknown",
        }
    }
}

/// Resource limits subset from peer metadata (Galaxy §2.3 sketch).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GalaxyWorkerLimits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_memory_mb: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_requests: Option<u32>,
    /// Telegram cold-mining CPU cap percent (PH-S541, Galaxy §2.3 / §8.2 TBD #2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cpu_pct: Option<u32>,
    /// Telegram cold-mining RAM cap MB (PH-S541).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ram_mb: Option<u32>,
    /// Telegram cold-mining disk cap MB (PH-S541).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_disk_mb: Option<u32>,
}

/// Capabilities subset from discovery peer (PH-S516, Galaxy §2.3).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GalaxyWorkerCapabilities {
    pub cpu_cores: usize,
    pub memory_mb: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gpu_devices: Vec<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_requests: Option<usize>,
}

/// Telemetry subset for admin sort/filter (Galaxy §2.3).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GalaxyWorkerTelemetry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms_p50: Option<u32>,
}

/// Unified worker row for `GET /api/v1/discovery/virtual-nodes` (PH-S507).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GalaxyWorkerDto {
    pub origin: GalaxyWorkerOrigin,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<GalaxyWorkerCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_inventory: Option<SeedInventoryEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srv_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<GalaxyNetworkProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<GalaxyWorkerLimits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetry: Option<GalaxyWorkerTelemetry>,
}

fn parse_origin(metadata: &std::collections::HashMap<String, String>) -> GalaxyWorkerOrigin {
    let raw = metadata
        .get("origin")
        .map(String::as_str)
        .or_else(|| metadata.get("role").map(String::as_str))
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    match raw.as_str() {
        "local" | "local_srv" => GalaxyWorkerOrigin::Local,
        "cloud" => GalaxyWorkerOrigin::Cloud,
        "telegram_edge" => GalaxyWorkerOrigin::TelegramEdge,
        "" => GalaxyWorkerOrigin::Unknown,
        _ => GalaxyWorkerOrigin::Unknown,
    }
}

fn parse_network_profile(
    metadata: &std::collections::HashMap<String, String>,
) -> Option<GalaxyNetworkProfile> {
    let raw = metadata.get("network_profile")?;
    let value: Value = serde_json::from_str(raw).ok()?;
    parse_network_profile_value(&value).ok()
}

fn parse_u32(meta: &std::collections::HashMap<String, String>, key: &str) -> Option<u32> {
    meta.get(key)?.trim().parse().ok()
}

fn capabilities_from_peer(caps: &PeerCapabilities) -> GalaxyWorkerCapabilities {
    GalaxyWorkerCapabilities {
        cpu_cores: caps.cpu_cores,
        memory_mb: caps.memory_mb,
        gpu_devices: caps.gpu_devices.clone(),
        active_requests: Some(caps.active_requests),
    }
}

fn parse_seed_inventory(
    metadata: &std::collections::HashMap<String, String>,
) -> Option<SeedInventoryEntry> {
    let raw = metadata.get("seed_inventory")?;
    let value: Value = serde_json::from_str(raw).ok()?;
    serde_json::from_value(value).ok()
}

/// Map discovery peer metadata to Galaxy worker DTO.
pub fn galaxy_worker_from_peer(peer: &PeerInfo) -> GalaxyWorkerDto {
    let network_profile = parse_network_profile(&peer.metadata);
    let latency_ms_p50 = network_profile.as_ref().map(|p| p.latency_ms_p50);
    GalaxyWorkerDto {
        origin: parse_origin(&peer.metadata),
        admin_id: peer.metadata.get("admin_id").cloned(),
        capabilities: Some(capabilities_from_peer(&peer.capabilities)),
        seed_inventory: parse_seed_inventory(&peer.metadata),
        srv_id: peer
            .metadata
            .get("srv_id")
            .cloned()
            .or_else(|| Some(peer.peer_id.clone())),
        network_profile,
        limits: Some(GalaxyWorkerLimits {
            max_memory_mb: parse_u32(&peer.metadata, "max_memory_mb")
                .or(Some(peer.capabilities.memory_mb as u32)),
            max_concurrent_requests: parse_u32(&peer.metadata, "max_concurrent_requests"),
            max_cpu_pct: parse_u32(&peer.metadata, "max_cpu_pct")
                .or(parse_u32(&peer.metadata, "cold_mining_max_cpu_pct")),
            max_ram_mb: parse_u32(&peer.metadata, "max_ram_mb")
                .or(parse_u32(&peer.metadata, "cold_mining_max_ram_mb")),
            max_disk_mb: parse_u32(&peer.metadata, "max_disk_mb")
                .or(parse_u32(&peer.metadata, "cold_mining_max_disk_mb")),
        }),
        telemetry: latency_ms_p50.map(|latency_ms_p50| GalaxyWorkerTelemetry {
            latency_ms_p50: Some(latency_ms_p50),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::discovery_types::PeerCapabilities;
    use chrono::Utc;
    use std::collections::HashMap;

    #[test]
    fn galaxy_worker_from_peer_ph_s507() {
        let mut metadata = HashMap::new();
        metadata.insert("origin".into(), "telegram_edge".into());
        metadata.insert("admin_id".into(), "admin-1".into());
        metadata.insert(
            "network_profile".into(),
            r#"{"region":"eu-west","latency_ms_p50":42}"#.into(),
        );
        let peer = PeerInfo {
            peer_id: "peer-1".into(),
            address: "127.0.0.1".into(),
            port: 9000,
            last_seen: Utc::now(),
            capabilities: PeerCapabilities {
                memory_mb: 4096,
                ..Default::default()
            },
            metadata,
        };
        let dto = galaxy_worker_from_peer(&peer);
        assert_eq!(dto.origin, GalaxyWorkerOrigin::TelegramEdge);
        assert_eq!(dto.admin_id.as_deref(), Some("admin-1"));
        assert_eq!(dto.network_profile.as_ref().unwrap().latency_ms_p50, 42);
        assert_eq!(dto.telemetry.as_ref().unwrap().latency_ms_p50, Some(42));
        assert_eq!(dto.capabilities.as_ref().unwrap().memory_mb, 4096);
    }

    #[test]
    fn galaxy_worker_seed_inventory_ph_s516() {
        let mut metadata = HashMap::new();
        metadata.insert("origin".into(), "local".into());
        metadata.insert(
            "seed_inventory".into(),
            r#"{"shard_ids":["s1"],"hot_tier":{"ram_bytes_used":100,"vram_bytes_used":0,"profiles":[]},"local_replica_regions":[]}"#.into(),
        );
        let peer = PeerInfo {
            peer_id: "peer-2".into(),
            address: "127.0.0.1".into(),
            port: 9000,
            last_seen: Utc::now(),
            capabilities: PeerCapabilities::default(),
            metadata,
        };
        let dto = galaxy_worker_from_peer(&peer);
        assert_eq!(
            dto.seed_inventory.as_ref().unwrap().shard_ids,
            vec!["s1".to_string()]
        );
    }

    #[test]
    fn galaxy_worker_cold_mining_limits_ph_s541() {
        let mut metadata = HashMap::new();
        metadata.insert("origin".into(), "telegram_edge".into());
        metadata.insert("cold_mining_max_cpu_pct".into(), "25".into());
        metadata.insert("cold_mining_max_ram_mb".into(), "512".into());
        metadata.insert("cold_mining_max_disk_mb".into(), "2048".into());
        let peer = PeerInfo {
            peer_id: "peer-cold".into(),
            address: "127.0.0.1".into(),
            port: 9000,
            last_seen: Utc::now(),
            capabilities: PeerCapabilities::default(),
            metadata,
        };
        let dto = galaxy_worker_from_peer(&peer);
        let limits = dto.limits.expect("limits");
        assert_eq!(limits.max_cpu_pct, Some(25));
        assert_eq!(limits.max_ram_mb, Some(512));
        assert_eq!(limits.max_disk_mb, Some(2048));
    }
}
