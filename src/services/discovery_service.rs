//! Discovery operations for the HTTP API (peers list, lookup, announce).

use crate::core::discovery_types::PeerInfo;
use crate::core::error::AppError;
use crate::core::state::ApiContext;

/// Discovery handle is not wired (e.g. server started without discovery).
#[derive(Debug, Clone, Copy)]
pub struct DiscoveryNotReady;

/// Successful `GET /discovery/peers` payload from the wired discovery handle.
#[derive(Debug, Clone)]
pub struct DiscoveryPeersSnapshot {
    pub peers: Vec<PeerInfo>,
    pub local_peer_id: String,
}

#[derive(Debug)]
pub enum DiscoveryAnnounceError {
    NotReady,
    Failed(AppError),
}

pub struct DiscoveryService;

impl DiscoveryService {
    pub async fn list_peers(ctx: &ApiContext) -> Result<DiscoveryPeersSnapshot, DiscoveryNotReady> {
        let guard = ctx.discovery.read().await;
        let discovery = guard.as_ref().ok_or(DiscoveryNotReady)?;
        Ok(DiscoveryPeersSnapshot {
            peers: discovery.get_peers().await,
            local_peer_id: discovery.local_peer_id(),
        })
    }

    pub async fn get_peer(
        ctx: &ApiContext,
        peer_id: &str,
    ) -> Result<Option<PeerInfo>, DiscoveryNotReady> {
        let guard = ctx.discovery.read().await;
        let discovery = guard.as_ref().ok_or(DiscoveryNotReady)?;
        Ok(discovery.get_peer(peer_id).await)
    }

    pub async fn send_announcement(ctx: &ApiContext) -> Result<(), DiscoveryAnnounceError> {
        let guard = ctx.discovery.read().await;
        let discovery = guard.as_ref().ok_or(DiscoveryAnnounceError::NotReady)?;
        discovery
            .send_announcement()
            .await
            .map_err(DiscoveryAnnounceError::Failed)
    }
}
