//! Map Grid envelope bodies ↔ existing discovery / RAID DTOs.

use crate::core::discovery_types::{PeerCapabilities, PeerInfo};
use crate::grid::envelope::{GridEnvelope, GridMemoryShardBody, GridMessage, GridPeerStatusBody};
use crate::raid::protocol::{ArtifactMetadata, PutArtifactPayload, SyncMode};
use chrono::Utc;
use std::collections::HashMap;

/// Build a v1 envelope from discovery [`PeerInfo`].
pub fn envelope_from_peer_info(peer: &PeerInfo) -> GridEnvelope {
    GridEnvelope::new(
        GridMessage::PeerStatus(peer_status_body_from_peer(peer)),
        None,
    )
}

fn peer_status_body_from_peer(peer: &PeerInfo) -> GridPeerStatusBody {
    GridPeerStatusBody {
        peer_id: peer.peer_id.clone(),
        address: peer.address.clone(),
        port: peer.port,
        last_seen: peer.last_seen,
        cpu_cores: peer.capabilities.cpu_cores,
        memory_mb: peer.capabilities.memory_mb,
        gpu_devices: peer.capabilities.gpu_devices.clone(),
        current_load: peer.capabilities.current_load,
        role: peer.metadata.get("role").cloned(),
    }
}

/// Extract [`PeerInfo`] when the envelope carries `peer_status`.
pub fn peer_info_from_envelope(env: &GridEnvelope) -> Option<PeerInfo> {
    match &env.msg {
        GridMessage::PeerStatus(body) => Some(peer_info_from_peer_status(body)),
        _ => None,
    }
}

fn peer_info_from_peer_status(body: &GridPeerStatusBody) -> PeerInfo {
    let mut metadata = HashMap::new();
    if let Some(role) = &body.role {
        metadata.insert("role".to_string(), role.clone());
    }
    PeerInfo {
        peer_id: body.peer_id.clone(),
        address: body.address.clone(),
        port: body.port,
        last_seen: body.last_seen,
        capabilities: PeerCapabilities {
            cpu_cores: body.cpu_cores,
            gpu_devices: body.gpu_devices.clone(),
            memory_mb: body.memory_mb,
            supports_tensor_parallelism: false,
            supports_pipeline_parallelism: false,
            active_requests: 0,
            capacity: 0,
            current_load: body.current_load,
        },
        metadata,
    }
}

/// Memory shard view of a distributed **PutArtifact** payload.
pub fn memory_shard_from_put_artifact(payload: &PutArtifactPayload) -> GridMemoryShardBody {
    GridMemoryShardBody {
        shard_id: format!("{}:{}", payload.metadata.name, payload.metadata.version),
        artifact_id: payload.artifact_id.clone(),
        version: payload.metadata.version.clone(),
        raid_logical_name: Some(payload.metadata.name.clone()),
        seed_hints: payload.metadata.tags.clone(),
    }
}

/// Wrap PutArtifact as a Grid envelope.
pub fn envelope_from_put_artifact(
    payload: &PutArtifactPayload,
    source_peer_id: Option<String>,
) -> GridEnvelope {
    GridEnvelope::new(
        GridMessage::MemoryShard(memory_shard_from_put_artifact(payload)),
        source_peer_id,
    )
}

/// Reconstruct a minimal **PutArtifact** payload from a memory shard body (metadata-only replication).
pub fn put_artifact_from_memory_shard(
    body: &GridMemoryShardBody,
    source_node: String,
) -> PutArtifactPayload {
    let name = body
        .raid_logical_name
        .clone()
        .unwrap_or_else(|| body.shard_id.clone());
    PutArtifactPayload {
        artifact_id: body.artifact_id.clone(),
        source_node,
        data: None,
        metadata: ArtifactMetadata {
            name,
            version: body.version.clone(),
            size_bytes: 0,
            checksum: String::new(),
            created_at: Utc::now(),
            content_type: None,
            tags: body.seed_hints.clone(),
        },
        replication_factor: 1,
        sync_mode: SyncMode::Async,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peer_info_round_trip_via_envelope() {
        let peer = PeerInfo {
            peer_id: "p-42".into(),
            address: "10.0.0.2".into(),
            port: 9090,
            last_seen: Utc::now(),
            capabilities: PeerCapabilities {
                cpu_cores: 4,
                gpu_devices: vec![0, 1],
                memory_mb: 8192,
                supports_tensor_parallelism: true,
                supports_pipeline_parallelism: false,
                active_requests: 2,
                capacity: 10,
                current_load: 0.5,
            },
            metadata: [("role".to_string(), "hub".to_string())]
                .into_iter()
                .collect(),
        };
        let env = envelope_from_peer_info(&peer);
        assert_eq!(env.v, GRID_ENVELOPE_VERSION);
        let back = peer_info_from_envelope(&env).unwrap();
        assert_eq!(back.peer_id, peer.peer_id);
        assert_eq!(back.address, peer.address);
        assert_eq!(back.port, peer.port);
        assert_eq!(back.capabilities.cpu_cores, peer.capabilities.cpu_cores);
        assert_eq!(back.metadata.get("role"), peer.metadata.get("role"));
    }

    #[test]
    fn put_artifact_memory_shard_round_trip() {
        let payload = PutArtifactPayload {
            artifact_id: "550e8400-e29b-41d4-a716-446655440000".into(),
            source_node: "node-a".into(),
            data: None,
            metadata: ArtifactMetadata {
                name: "weights".into(),
                version: "1.0.0".into(),
                size_bytes: 1024,
                checksum: "sha256:abc".into(),
                created_at: Utc::now(),
                content_type: Some("application/octet-stream".into()),
                tags: Some(vec!["seed".into()]),
            },
            replication_factor: 2,
            sync_mode: SyncMode::Sync,
        };
        let env = envelope_from_put_artifact(&payload, Some("node-a".into()));
        let shard = match &env.msg {
            GridMessage::MemoryShard(s) => s.clone(),
            _ => panic!("expected memory_shard"),
        };
        let back = put_artifact_from_memory_shard(&shard, "node-b".into());
        assert_eq!(back.artifact_id, payload.artifact_id);
        assert_eq!(back.metadata.name, payload.metadata.name);
        assert_eq!(back.metadata.version, payload.metadata.version);
        assert_eq!(back.metadata.tags, payload.metadata.tags);
    }

    #[test]
    fn non_peer_envelope_returns_none() {
        let env = envelope_from_put_artifact(
            &PutArtifactPayload {
                artifact_id: "id".into(),
                source_node: "n".into(),
                data: None,
                metadata: ArtifactMetadata {
                    name: "m".into(),
                    version: "1".into(),
                    size_bytes: 0,
                    checksum: String::new(),
                    created_at: Utc::now(),
                    content_type: None,
                    tags: None,
                },
                replication_factor: 1,
                sync_mode: SyncMode::Async,
            },
            None,
        );
        assert!(peer_info_from_envelope(&env).is_none());
    }
}
