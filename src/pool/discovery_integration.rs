//! Discovery integration with Worker Pool
//!
//! This module provides automatic integration between discovered peers
//! and the worker pool, automatically adding discovered peers as workers.

use crate::core::error::AppError;
use crate::network::discovery::{get_global_discovery_service, PeerInfo};
use crate::pool::{worker, Pool};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use tracing::{debug, info, warn};

/// Sync discovery peers with worker pool
pub struct DiscoveryPoolSync {
    pool: Arc<RwLock<Pool>>,
    known_peer_ids: Arc<RwLock<std::collections::HashSet<String>>>,
}

impl DiscoveryPoolSync {
    /// Creates a new discovery pool sync
    pub fn new(pool: Arc<RwLock<Pool>>) -> Self {
        Self {
            pool,
            known_peer_ids: Arc::new(RwLock::new(std::collections::HashSet::new())),
        }
    }

    /// Starts the sync task
    pub async fn start(&self) -> Result<(), AppError> {
        let pool = Arc::clone(&self.pool);
        let known_peer_ids = Arc::clone(&self.known_peer_ids);

        // Spawn sync task
        tokio::spawn(Self::sync_task(pool, known_peer_ids));

        info!("Discovery pool sync started");
        Ok(())
    }

    /// Background task for syncing peers with worker pool
    async fn sync_task(
        pool: Arc<RwLock<Pool>>,
        known_peer_ids: Arc<RwLock<std::collections::HashSet<String>>>,
    ) {
        let mut interval = interval(Duration::from_secs(5)); // Sync every 5 seconds

        loop {
            interval.tick().await;

            if let Some(discovery) = get_global_discovery_service() {
                let peers = discovery.get_peers().await;
                let mut known_ids = known_peer_ids.write().await;

                // Add new peers as workers
                for peer in &peers {
                    if !known_ids.contains(&peer.peer_id) {
                        // Convert peer to worker
                        match Self::create_worker_from_peer(peer) {
                            Ok((worker_id, worker)) => {
                                let pool_guard = pool.read().await;
                                if let Err(e) =
                                    pool_guard.add_worker(worker_id.clone(), worker).await
                                {
                                    warn!("Failed to add discovered peer as worker: {}", e);
                                } else {
                                    info!(
                                        "Added discovered peer {} as worker {}",
                                        peer.peer_id, worker_id
                                    );
                                    known_ids.insert(peer.peer_id.clone());
                                }
                            }
                            Err(e) => {
                                warn!("Failed to create worker from peer {}: {}", peer.peer_id, e);
                            }
                        }
                    }
                }

                // Remove stale workers (peers that are no longer in discovery)
                let current_peer_ids: std::collections::HashSet<String> =
                    peers.iter().map(|p| p.peer_id.clone()).collect();
                let to_remove: Vec<String> = known_ids
                    .iter()
                    .filter(|id| !current_peer_ids.contains(*id))
                    .cloned()
                    .collect();

                for peer_id in to_remove {
                    let worker_id = format!("discovered-{}", peer_id);
                    {
                        let pool_guard = pool.read().await;
                        if let Err(e) = pool_guard.remove_worker(&worker_id).await {
                            warn!("Failed to remove stale worker {}: {}", worker_id, e);
                        } else {
                            info!("Removed stale worker: {}", worker_id);
                            known_ids.remove(&peer_id);
                        }
                    }
                }
            } else {
                debug!("Discovery service not available, skipping sync");
            }
        }
    }

    /// Creates a worker from peer information
    fn create_worker_from_peer(peer: &PeerInfo) -> Result<(String, worker::Worker), AppError> {
        // Use peer_id as worker_id
        let worker_id = format!("discovered-{}", peer.peer_id);

        // Map peer capabilities to worker config
        let worker_config = worker::WorkerConfig {
            worker_id: worker_id.clone(),
            max_concurrent_requests: 10, // Default, can be adjusted based on peer capabilities
            request_timeout_ms: 30000,
            health_check_interval_ms: 5000,
            enable_caching: true,
            cache_size: 1000,
            max_memory_mb: peer.capabilities.memory_mb,
            cpu_priority: 5,
            gpu_device: peer.capabilities.gpu_devices.first().copied(),
            auto_restart: true,
            resource_monitoring: true,
        };

        let worker = worker::Worker::new(worker_config);
        Ok((worker_id, worker))
    }
}
