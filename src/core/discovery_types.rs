//! Shared discovery DTOs (used by `network::discovery`, API, pool topology).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Information about a discovered peer/worker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub peer_id: String,
    pub address: String,
    pub port: u16,
    pub last_seen: DateTime<Utc>,
    pub capabilities: PeerCapabilities,
    pub metadata: HashMap<String, String>,
}

/// Peer capabilities
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PeerCapabilities {
    pub cpu_cores: usize,
    pub gpu_devices: Vec<usize>,
    pub memory_mb: usize,
    pub supports_tensor_parallelism: bool,
    pub supports_pipeline_parallelism: bool,
    #[serde(default)]
    pub active_requests: usize,
    #[serde(default)]
    pub capacity: usize,
    #[serde(default)]
    pub current_load: f32,
}
