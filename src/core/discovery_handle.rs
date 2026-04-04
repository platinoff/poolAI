//! Trait + shared handle for injecting discovery into `AppState` without a global singleton.

use crate::core::discovery_types::PeerInfo;
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
}
