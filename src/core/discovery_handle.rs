//! Trait + shared handle for injecting discovery into `AppState` without a global singleton.

use crate::core::discovery_types::{PeerCapabilities, PeerInfo};
use crate::core::error::AppError;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type SharedDiscoverySlot = Arc<RwLock<Option<Arc<dyn DiscoveryHandle>>>>;

#[async_trait]
pub trait DiscoveryHandle: Send + Sync {
    async fn get_peers(&self) -> Vec<PeerInfo>;
    fn local_peer_id(&self) -> String;
    async fn get_peer(&self, peer_id: &str) -> Option<PeerInfo>;
    async fn send_announcement(&self) -> Result<(), AppError>;
    /// Register a peer that connected over HTTP (e.g. Telegram / virtual node worker).
    async fn register_remote_peer(&self, peer: PeerInfo) -> Result<(), AppError>;
    /// Refresh `last_seen` for a previously registered remote peer (FM-016 phase 2).
    async fn heartbeat_remote_peer(
        &self,
        peer_id: &str,
        capabilities: Option<PeerCapabilities>,
    ) -> Result<(), AppError>;
}
