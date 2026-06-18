//! Network discovery module for automatic device/worker detection
//!
//! This module provides:
//! - Automatic worker discovery via UDP broadcast (mDNS/Bonjour fallback)
//! - Worker registration protocol with heartbeat
//! - Discovery API endpoints for peer discovery
//!
//! Inspired by exo's automatic device discovery feature.

use crate::core::error::AppError;
use crate::runtime::instance::InstanceManager;
use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::sync::Arc;
use tokio::net::UdpSocket as TokioUdpSocket;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

pub use crate::core::discovery_types::{PeerCapabilities, PeerInfo};

/// Discovery configuration
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    /// Broadcast port for discovery messages
    pub broadcast_port: u16,
    /// Heartbeat interval in seconds
    pub heartbeat_interval_secs: u64,
    /// Timeout for considering a worker offline (seconds)
    pub worker_timeout_secs: u64,
    /// Enable discovery
    pub enabled: bool,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            broadcast_port: 8888,
            heartbeat_interval_secs: 5,
            worker_timeout_secs: 15,
            enabled: true,
        }
    }
}

/// Detect local system capabilities
///
/// Automatically detects CPU cores, memory, and GPU devices on the current system.
/// This function attempts to detect actual system resources, falling back to
/// reasonable defaults if detection fails.
pub fn detect_local_capabilities() -> PeerCapabilities {
    // Detect CPU cores
    let cpu_cores = num_cpus::get();

    // Detect total and available system memory
    let (memory_mb, _available_mb) = detect_system_memory().unwrap_or((8192, 6144)); // Default to 8GB total, 6GB available

    // Detect GPU devices (simplified - check for GPU presence)
    let gpu_devices = detect_gpu_devices();

    // Determine parallelism support based on capabilities
    let supports_tensor_parallelism = gpu_devices.len() >= 2; // Requires 2+ GPUs
    let supports_pipeline_parallelism = cpu_cores >= 4 && memory_mb >= 8192; // Requires sufficient resources

    PeerCapabilities {
        cpu_cores,
        gpu_devices,
        memory_mb,
        supports_tensor_parallelism,
        supports_pipeline_parallelism,
        active_requests: 0,
        capacity: 10, // Default capacity: 10 concurrent requests
        current_load: 0.0,
    }
}

/// Detect total and available system memory in MB
/// Returns (total_memory_mb, available_memory_mb)
fn detect_system_memory() -> Option<(usize, usize)> {
    #[cfg(target_os = "linux")]
    {
        // Read from /proc/meminfo on Linux
        use std::fs;
        if let Ok(content) = fs::read_to_string("/proc/meminfo") {
            let mut total_mb = None;
            let mut available_mb = None;

            for line in content.lines() {
                if line.starts_with("MemTotal:") {
                    if let Some(value) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = value.parse::<usize>() {
                            total_mb = Some(kb / 1024); // Convert KB to MB
                        }
                    }
                } else if line.starts_with("MemAvailable:") {
                    // Prefer MemAvailable (kernel 3.14+) as it's more accurate
                    if let Some(value) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = value.parse::<usize>() {
                            available_mb = Some(kb / 1024);
                        }
                    }
                } else if available_mb.is_none() && line.starts_with("MemFree:") {
                    // Fallback to MemFree if MemAvailable not available
                    if let Some(value) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = value.parse::<usize>() {
                            available_mb = Some(kb / 1024);
                        }
                    }
                }
            }

            if let Some(total) = total_mb {
                let available = available_mb.unwrap_or(total / 4); // Fallback to 25% if can't detect
                return Some((total, available));
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        // On Windows, try to use system commands
        // Could use PowerShell: (Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory
        // For now, return None and use fallback
    }

    #[cfg(target_os = "macos")]
    {
        // On macOS, use sysctl for memory detection
        let mut total_mb = None;
        let mut available_mb = None;

        // Get total memory
        if let Ok(output) = std::process::Command::new("sysctl")
            .arg("-n")
            .arg("hw.memsize")
            .output()
        {
            if output.status.success() {
                if let Ok(contents) = String::from_utf8(output.stdout) {
                    if let Ok(bytes) = contents.trim().parse::<usize>() {
                        total_mb = Some(bytes / (1024 * 1024)); // Convert bytes to MB
                    }
                }
            }
        }

        // Get available memory (vm_stat on macOS)
        if let Ok(output) = std::process::Command::new("vm_stat").output() {
            if output.status.success() {
                let content = String::from_utf8_lossy(&output.stdout);
                // Parse vm_stat output for free pages (simplified)
                // In production, would parse "Pages free" and "Pages inactive"
                if let Some(total) = total_mb {
                    available_mb = Some(total / 4); // Rough estimate: 25% available
                }
            }
        }

        if let Some(total) = total_mb {
            let available = available_mb.unwrap_or(total / 4);
            return Some((total, available));
        }
    }

    None
}

/// Detect available GPU devices
///
/// Returns a list of GPU device indices (0, 1, 2, ...) for detected GPUs.
/// Currently uses heuristics to detect GPU presence.
fn detect_gpu_devices() -> Vec<usize> {
    let mut devices = Vec::new();

    // Try to detect NVIDIA GPUs via nvidia-smi
    if let Ok(output) = std::process::Command::new("nvidia-smi")
        .arg("--list-gpus")
        .output()
    {
        if output.status.success() {
            let count = String::from_utf8_lossy(&output.stdout).lines().count();
            for i in 0..count {
                devices.push(i);
            }
            if !devices.is_empty() {
                return devices;
            }
        }
    }

    // Try to detect AMD GPUs via rocm-smi (Linux)
    #[cfg(target_os = "linux")]
    {
        if let Ok(output) = std::process::Command::new("rocm-smi")
            .arg("--listid")
            .output()
        {
            if output.status.success() {
                let count = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .filter(|l| !l.trim().is_empty())
                    .count();
                for i in 0..count {
                    devices.push(i);
                }
                if !devices.is_empty() {
                    return devices;
                }
            }
        }

        // Try to detect GPUs via /sys/class/drm on Linux
        if let Ok(entries) = std::fs::read_dir("/sys/class/drm") {
            let mut gpu_count = 0;
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.starts_with("card")
                    && name_str.chars().skip(4).all(|c| c.is_ascii_digit())
                {
                    gpu_count += 1;
                }
            }
            if gpu_count > 0 {
                for i in 0..gpu_count {
                    devices.push(i);
                }
                return devices;
            }
        }
    }

    // Fallback: check if platform module reports GPU (even if placeholder)
    // This allows for future platform-specific detection
    // Note: Platform detection can be added via platform module if needed

    // Default: assume no GPUs detected
    devices
}

/// Discovery message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DiscoveryMessage {
    /// Announcement from a peer
    Announce {
        peer_id: String,
        address: String,
        port: u16,
        capabilities: PeerCapabilities,
        metadata: HashMap<String, String>,
    },
    /// Heartbeat from an existing peer
    Heartbeat { peer_id: String },
    /// Query for available peers
    Query,
    /// Response to a query
    Response { peers: Vec<PeerInfo> },
}

/// Discovery service for automatic peer/worker detection
pub struct DiscoveryService {
    config: DiscoveryConfig,
    /// Local peer ID
    local_peer_id: String,
    /// Local address and port
    local_address: SocketAddr,
    /// Discovered peers
    peers: Arc<RwLock<HashMap<String, PeerInfo>>>,
    /// Whether the service is running
    running: Arc<RwLock<bool>>,
    /// Optional instance manager (same `Arc` as `AppState::instance_manager`); avoids `get_global_instance_manager` in HTTP/discovery paths.
    instance_manager: Option<Arc<RwLock<InstanceManager>>>,
}

/// Minimal clone of discovery service for listener task
struct DiscoveryServiceClone {
    peers: Arc<RwLock<HashMap<String, PeerInfo>>>,
    local_address: SocketAddr,
    local_peer_id: String,
}

impl DiscoveryService {
    /// Creates a new discovery service.
    ///
    /// Pass `instance_manager` when the runtime instance manager is already initialized
    /// (typically `app_state.instance_manager.get().cloned()` after `attach_core_http_singletons`).
    pub fn new(
        config: DiscoveryConfig,
        local_address: SocketAddr,
        instance_manager: Option<Arc<RwLock<InstanceManager>>>,
    ) -> Self {
        let local_peer_id = format!("poolai-{}", &Uuid::new_v4().to_string()[..8]);

        Self {
            config,
            local_peer_id,
            local_address,
            peers: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(false)),
            instance_manager,
        }
    }

    /// Starts the discovery service
    pub async fn start(&self) -> Result<(), AppError> {
        if !self.config.enabled {
            info!("Discovery service is disabled");
            return Ok(());
        }

        let mut running = self.running.write().await;
        if *running {
            warn!("Discovery service is already running");
            return Ok(());
        }
        *running = true;
        drop(running);

        info!(
            "Starting discovery service (peer_id: {}, address: {})",
            self.local_peer_id, self.local_address
        );

        // Spawn background tasks
        let peers = Arc::clone(&self.peers);
        let config = self.config.clone();
        let local_peer_id = self.local_peer_id.clone();
        let local_address = self.local_address;
        let discovery_service = Arc::new(self.clone_for_listener());

        // UDP listener task
        tokio::spawn(Self::udp_listener_task(
            discovery_service.clone(),
            config.clone(),
        ));

        // Broadcast task
        tokio::spawn(Self::broadcast_task(
            peers.clone(),
            config.clone(),
            local_peer_id.clone(),
            local_address,
        ));

        // Cleanup task
        tokio::spawn(Self::cleanup_task(peers, config));

        // Send initial announcement
        self.send_announcement().await?;

        Ok(())
    }

    /// Clone discovery service for listener (minimal clone)
    fn clone_for_listener(&self) -> DiscoveryServiceClone {
        DiscoveryServiceClone {
            peers: Arc::clone(&self.peers),
            local_address: self.local_address,
            local_peer_id: self.local_peer_id.clone(),
        }
    }

    /// Stops the discovery service
    pub async fn stop(&self) -> Result<(), AppError> {
        let mut running = self.running.write().await;
        if !*running {
            return Ok(());
        }
        *running = false;
        drop(running);

        info!("Stopping discovery service");
        Ok(())
    }

    /// Broadcasts an announcement message
    pub async fn send_announcement(&self) -> Result<(), AppError> {
        // Detect local system capabilities
        let mut capabilities = detect_local_capabilities();

        // Update load metrics if instance manager is available (injected from AppState)
        if let Some(instance_manager) = self.instance_manager.as_ref() {
            let manager = instance_manager.read().await;
            let instances = manager.list_instances().await;

            // Note: We can't await in filter, so we collect instances first
            let mut active_count = 0;
            for inst in instances {
                let status = inst.status.read().await;
                if matches!(
                    *status,
                    crate::runtime::instance::InstanceStatus::Ready
                        | crate::runtime::instance::InstanceStatus::Active
                ) {
                    active_count += 1;
                }
            }

            capabilities.active_requests = active_count;
            capabilities.current_load = if capabilities.capacity > 0 {
                (active_count as f32 / capabilities.capacity as f32).min(1.0)
            } else {
                0.0
            };
        }

        let metadata = HashMap::new();

        let message = DiscoveryMessage::Announce {
            peer_id: self.local_peer_id.clone(),
            address: self.local_address.ip().to_string(),
            port: self.local_address.port(),
            capabilities,
            metadata,
        };

        self.send_broadcast(&message).await?;
        Ok(())
    }

    /// Sends a broadcast message to all peers on the network
    async fn send_broadcast(&self, message: &DiscoveryMessage) -> Result<(), AppError> {
        let socket = UdpSocket::bind("0.0.0.0:0")
            .map_err(|e| AppError::NetworkError(format!("Failed to bind UDP socket: {}", e)))?;

        socket
            .set_broadcast(true)
            .map_err(|e| AppError::NetworkError(format!("Failed to enable broadcast: {}", e)))?;

        let data = serde_json::to_vec(message).map_err(|e| {
            AppError::ConfigError(format!("Failed to serialize discovery message: {}", e))
        })?;

        let broadcast_addr = SocketAddr::new(
            IpAddr::from([255, 255, 255, 255]),
            self.config.broadcast_port,
        );

        socket
            .send_to(&data, broadcast_addr)
            .map_err(|e| AppError::NetworkError(format!("Failed to send broadcast: {}", e)))?;

        debug!("Sent discovery broadcast: {:?}", message);
        Ok(())
    }

    /// Background task for periodic broadcasting
    async fn broadcast_task(
        _peers: Arc<RwLock<HashMap<String, PeerInfo>>>,
        config: DiscoveryConfig,
        local_peer_id: String,
        _local_address: SocketAddr,
    ) {
        let mut interval = interval(Duration::from_secs(config.heartbeat_interval_secs));

        loop {
            interval.tick().await;

            // Send heartbeat
            let message = DiscoveryMessage::Heartbeat {
                peer_id: local_peer_id.clone(),
            };

            let socket = match UdpSocket::bind("0.0.0.0:0") {
                Ok(s) => s,
                Err(e) => {
                    warn!("Failed to bind UDP socket for broadcast: {}", e);
                    continue;
                }
            };

            if let Err(e) = socket.set_broadcast(true) {
                warn!("Failed to enable broadcast: {}", e);
                continue;
            }

            let data = match serde_json::to_vec(&message) {
                Ok(d) => d,
                Err(e) => {
                    warn!("Failed to serialize heartbeat: {}", e);
                    continue;
                }
            };

            let broadcast_addr =
                SocketAddr::new(IpAddr::from([255, 255, 255, 255]), config.broadcast_port);

            if let Err(e) = socket.send_to(&data, broadcast_addr) {
                warn!("Failed to send heartbeat: {}", e);
            } else {
                debug!("Sent heartbeat: {}", local_peer_id);
            }
        }
    }

    /// Background task for UDP listener
    async fn udp_listener_task(discovery: Arc<DiscoveryServiceClone>, config: DiscoveryConfig) {
        let bind_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), config.broadcast_port);

        let socket = match TokioUdpSocket::bind(bind_addr).await {
            Ok(s) => {
                info!("Discovery UDP listener started on {}", bind_addr);
                s
            }
            Err(e) => {
                error!("Failed to bind UDP socket for discovery listener: {}", e);
                return;
            }
        };

        let mut buf = vec![0u8; 4096];

        loop {
            match socket.recv_from(&mut buf).await {
                Ok((len, source)) => {
                    let data = &buf[..len];

                    match serde_json::from_slice::<DiscoveryMessage>(data) {
                        Ok(message) => {
                            debug!("Received discovery message from {}: {:?}", source, message);

                            // Handle message
                            let mut peers = discovery.peers.write().await;
                            match message {
                                DiscoveryMessage::Announce {
                                    peer_id,
                                    address,
                                    port,
                                    capabilities,
                                    metadata,
                                } => {
                                    // Ignore our own messages
                                    if peer_id == discovery.local_peer_id
                                        || source == discovery.local_address
                                    {
                                        continue;
                                    }

                                    let peer_info = PeerInfo {
                                        peer_id: peer_id.clone(),
                                        address,
                                        port,
                                        last_seen: Utc::now(),
                                        capabilities,
                                        metadata,
                                    };

                                    if peers.contains_key(&peer_id) {
                                        debug!(
                                            "Updated peer: {} (last seen: {})",
                                            peer_id, peer_info.last_seen
                                        );
                                    } else {
                                        info!(
                                            "Discovered new peer: {} at {}:{}",
                                            peer_id, peer_info.address, port
                                        );
                                    }
                                    peers.insert(peer_id, peer_info);
                                }
                                DiscoveryMessage::Heartbeat { peer_id } => {
                                    if let Some(peer_info) = peers.get_mut(&peer_id) {
                                        peer_info.last_seen = Utc::now();
                                        debug!("Received heartbeat from: {}", peer_id);
                                    }
                                }
                                DiscoveryMessage::Query => {
                                    // Respond with list of peers
                                    let peers_list: Vec<PeerInfo> =
                                        peers.values().cloned().collect();
                                    let response = DiscoveryMessage::Response { peers: peers_list };

                                    if let Ok(data) = serde_json::to_vec(&response) {
                                        if let Err(e) = socket.send_to(&data, source).await {
                                            warn!("Failed to send discovery response: {}", e);
                                        }
                                    }
                                }
                                DiscoveryMessage::Response { peers: peers_list } => {
                                    // Update peer list from response
                                    for peer in peers_list {
                                        peers.insert(peer.peer_id.clone(), peer);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Failed to parse discovery message from {}: {}", source, e);
                        }
                    }
                }
                Err(e) => {
                    error!("Error receiving UDP packet: {}", e);
                    break;
                }
            }
        }
    }

    /// Background task for cleaning up stale peers
    async fn cleanup_task(peers: Arc<RwLock<HashMap<String, PeerInfo>>>, config: DiscoveryConfig) {
        let mut interval = interval(Duration::from_secs(config.worker_timeout_secs));

        loop {
            interval.tick().await;

            let now = Utc::now();
            let timeout_duration = Duration::from_secs(config.worker_timeout_secs);

            let mut peers_guard = peers.write().await;
            peers_guard.retain(|peer_id, peer_info| {
                let elapsed = now.signed_duration_since(peer_info.last_seen);
                let is_valid = elapsed.num_seconds() < timeout_duration.as_secs() as i64;

                if !is_valid {
                    info!(
                        "Removing stale peer: {} (last seen: {})",
                        peer_id, peer_info.last_seen
                    );
                }

                is_valid
            });
        }
    }

    /// Refresh last-seen (and optional capabilities) for an HTTP-registered peer.
    pub async fn heartbeat_remote_peer(
        &self,
        peer_id: &str,
        capabilities: Option<PeerCapabilities>,
    ) -> Result<(), AppError> {
        let mut peers = self.peers.write().await;
        let peer = peers.get_mut(peer_id).ok_or_else(|| {
            AppError::ApiNotFound(format!("remote peer not registered: {peer_id}"))
        })?;
        peer.last_seen = Utc::now();
        if let Some(cap) = capabilities {
            peer.capabilities = cap;
        }
        if let Some(np_json) = peer.metadata.get("network_profile").cloned() {
            if let Ok(updated) =
                crate::grid::galaxy_network_profile::refresh_network_profile_measured_at(
                    &np_json,
                    peer.last_seen,
                )
            {
                peer.metadata
                    .insert("network_profile".to_string(), updated.clone());
                let _ = crate::grid::galaxy_network_profile_store::persist_peer_network_profile(
                    peer_id, &updated,
                );
            }
        }
        debug!("Heartbeat from remote peer: {}", peer_id);
        Ok(())
    }

    /// Register a remote peer announced over HTTP (virtual node / Telegram worker).
    pub async fn register_remote_peer(&self, peer: PeerInfo) -> Result<(), AppError> {
        let peer_id = peer.peer_id.clone();
        let mut peers = self.peers.write().await;
        if peers.contains_key(&peer_id) {
            debug!(
                "Updated remote peer: {} at {}:{}",
                peer_id, peer.address, peer.port
            );
        } else {
            info!(
                "Registered remote peer: {} at {}:{}",
                peer_id, peer.address, peer.port
            );
        }
        peers.insert(peer_id, peer);
        Ok(())
    }

    /// Gets all discovered peers
    pub async fn get_peers(&self) -> Vec<PeerInfo> {
        let peers = self.peers.read().await;
        peers.values().cloned().collect()
    }

    /// Gets a specific peer by ID
    pub async fn get_peer(&self, peer_id: &str) -> Option<PeerInfo> {
        let peers = self.peers.read().await;
        peers.get(peer_id).cloned()
    }

    /// Gets the local peer ID
    pub fn local_peer_id(&self) -> &str {
        &self.local_peer_id
    }

    /// Handles an incoming discovery message (called by UDP listener)
    pub async fn handle_message(
        &self,
        message: DiscoveryMessage,
        source: SocketAddr,
    ) -> Result<(), AppError> {
        // Ignore our own messages
        if source == self.local_address {
            return Ok(());
        }

        match message {
            DiscoveryMessage::Announce {
                peer_id,
                address,
                port,
                capabilities,
                metadata,
            } => {
                let peer_info = PeerInfo {
                    peer_id: peer_id.clone(),
                    address,
                    port,
                    last_seen: Utc::now(),
                    capabilities,
                    metadata,
                };

                let mut peers = self.peers.write().await;
                if peers.contains_key(&peer_id) {
                    debug!(
                        "Updated peer: {} (last seen: {})",
                        peer_id, peer_info.last_seen
                    );
                } else {
                    info!(
                        "Discovered new peer: {} at {}:{}",
                        peer_id, peer_info.address, port
                    );
                }
                peers.insert(peer_id, peer_info);
            }
            DiscoveryMessage::Heartbeat { peer_id } => {
                let mut peers = self.peers.write().await;
                if let Some(peer_info) = peers.get_mut(&peer_id) {
                    peer_info.last_seen = Utc::now();
                    debug!("Received heartbeat from: {}", peer_id);
                }
            }
            DiscoveryMessage::Query => {
                // Respond with list of peers
                let peers_list = self.get_peers().await;
                let response = DiscoveryMessage::Response { peers: peers_list };

                // Send response back to source
                let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| {
                    AppError::NetworkError(format!("Failed to bind UDP socket: {}", e))
                })?;

                let data = serde_json::to_vec(&response).map_err(|e| {
                    AppError::ConfigError(format!("Failed to serialize response: {}", e))
                })?;

                socket.send_to(&data, source).map_err(|e| {
                    AppError::NetworkError(format!("Failed to send response: {}", e))
                })?;
            }
            DiscoveryMessage::Response { peers } => {
                // Update peer list from response
                let mut peers_guard = self.peers.write().await;
                for peer in peers {
                    peers_guard.insert(peer.peer_id.clone(), peer);
                }
            }
        }

        Ok(())
    }
}

#[async_trait]
impl crate::core::discovery_handle::DiscoveryHandle for DiscoveryService {
    async fn get_peers(&self) -> Vec<PeerInfo> {
        DiscoveryService::get_peers(self).await
    }

    fn local_peer_id(&self) -> String {
        self.local_peer_id().to_string()
    }

    async fn get_peer(&self, peer_id: &str) -> Option<PeerInfo> {
        DiscoveryService::get_peer(self, peer_id).await
    }

    async fn send_announcement(&self) -> Result<(), AppError> {
        DiscoveryService::send_announcement(self).await
    }

    async fn register_remote_peer(&self, peer: PeerInfo) -> Result<(), AppError> {
        DiscoveryService::register_remote_peer(self, peer).await
    }

    async fn heartbeat_remote_peer(
        &self,
        peer_id: &str,
        capabilities: Option<PeerCapabilities>,
    ) -> Result<(), AppError> {
        DiscoveryService::heartbeat_remote_peer(self, peer_id, capabilities).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, SocketAddrV4};

    #[tokio::test]
    async fn test_discovery_service_creation() {
        let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080));
        let config = DiscoveryConfig::default();
        let service = DiscoveryService::new(config, addr, None);

        assert!(!service.local_peer_id().is_empty());
        assert!(service.local_peer_id().starts_with("poolai-"));
    }

    #[tokio::test]
    async fn test_heartbeat_remote_peer_updates_last_seen() {
        let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080));
        let service = DiscoveryService::new(DiscoveryConfig::default(), addr, None);
        let peer = PeerInfo {
            peer_id: "tg-worker-hb".to_string(),
            address: "10.0.0.6".to_string(),
            port: 9091,
            last_seen: Utc::now() - chrono::Duration::seconds(120),
            capabilities: PeerCapabilities::default(),
            metadata: HashMap::from([("role".to_string(), "virtual_node".to_string())]),
        };
        service.register_remote_peer(peer).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        service
            .heartbeat_remote_peer("tg-worker-hb", None)
            .await
            .unwrap();
        let updated = service.get_peer("tg-worker-hb").await.expect("peer");
        assert!(updated.last_seen > Utc::now() - chrono::Duration::seconds(5));
    }

    #[tokio::test]
    async fn test_register_remote_peer() {
        let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080));
        let service = DiscoveryService::new(DiscoveryConfig::default(), addr, None);
        let peer = PeerInfo {
            peer_id: "tg-worker-1".to_string(),
            address: "10.0.0.5".to_string(),
            port: 9090,
            last_seen: Utc::now(),
            capabilities: PeerCapabilities::default(),
            metadata: HashMap::from([("channel".to_string(), "telegram".to_string())]),
        };
        service.register_remote_peer(peer).await.unwrap();
        let peers = service.get_peers().await;
        assert_eq!(peers.len(), 1);
        assert_eq!(peers[0].peer_id, "tg-worker-1");
        assert_eq!(
            peers[0].metadata.get("channel").map(String::as_str),
            Some("telegram")
        );
    }

    #[tokio::test]
    async fn test_discovery_message_serialization() {
        let message = DiscoveryMessage::Announce {
            peer_id: "test-peer".to_string(),
            address: "127.0.0.1".to_string(),
            port: 8080,
            capabilities: PeerCapabilities::default(),
            metadata: HashMap::new(),
        };

        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: DiscoveryMessage = serde_json::from_str(&serialized).unwrap();

        match deserialized {
            DiscoveryMessage::Announce {
                peer_id,
                address,
                port,
                ..
            } => {
                assert_eq!(peer_id, "test-peer");
                assert_eq!(address, "127.0.0.1");
                assert_eq!(port, 8080);
            }
            _ => panic!("Unexpected message type"),
        }
    }
}
