//! Discovery operations for the HTTP API (peers list, lookup, announce).

use crate::core::discovery_types::{PeerCapabilities, PeerInfo};
use crate::core::error::AppError;
use crate::core::state::ApiContext;
use crate::grid::galaxy_worker_dto::{galaxy_worker_from_peer, GalaxyWorkerDto};
use chrono::Utc;
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;

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

    pub async fn register_remote_peer(
        ctx: &ApiContext,
        peer_id: String,
        address: String,
        port: u16,
        capabilities: PeerCapabilities,
        metadata: HashMap<String, String>,
    ) -> Result<(), DiscoveryAnnounceError> {
        let guard = ctx.discovery.read().await;
        let discovery = guard.as_ref().ok_or(DiscoveryAnnounceError::NotReady)?;
        let peer = PeerInfo {
            peer_id,
            address,
            port,
            last_seen: Utc::now(),
            capabilities,
            metadata,
        };
        discovery
            .register_remote_peer(peer)
            .await
            .map_err(DiscoveryAnnounceError::Failed)
    }

    pub async fn heartbeat_remote_peer(
        ctx: &ApiContext,
        peer_id: &str,
        capabilities: Option<PeerCapabilities>,
    ) -> Result<(), DiscoveryAnnounceError> {
        let guard = ctx.discovery.read().await;
        let discovery = guard.as_ref().ok_or(DiscoveryAnnounceError::NotReady)?;
        discovery
            .heartbeat_remote_peer(peer_id, capabilities)
            .await
            .map_err(DiscoveryAnnounceError::Failed)
    }

    /// Virtual nodes registered with `metadata.role = virtual_node`.
    pub async fn list_virtual_nodes(
        ctx: &ApiContext,
        stale_after_secs: i64,
    ) -> Result<Vec<VirtualNodeStatus>, DiscoveryNotReady> {
        let snapshot = Self::list_peers(ctx).await?;
        let now = Utc::now();
        Ok(snapshot
            .peers
            .into_iter()
            .filter(|p| p.metadata.get("role").map(String::as_str) == Some("virtual_node"))
            .map(|p| {
                let age = now.signed_duration_since(p.last_seen).num_seconds();
                let stale = age > stale_after_secs;
                if stale {
                    crate::grid::galaxy_worker_health::on_heartbeat_miss(&p.peer_id);
                }
                VirtualNodeStatus {
                    peer: p.clone(),
                    stale,
                    last_seen_age_secs: age,
                    galaxy: galaxy_worker_from_peer(&p),
                }
            })
            .collect())
    }

    /// HTTP GET `http://{peer}/health` (FM-016 phase 2).
    pub async fn probe_remote_health(
        ctx: &ApiContext,
        peer_id: &str,
    ) -> Result<RemoteHealthProbe, DiscoveryAnnounceError> {
        let peer = Self::get_peer(ctx, peer_id)
            .await
            .map_err(|_| DiscoveryAnnounceError::NotReady)?
            .ok_or_else(|| {
                DiscoveryAnnounceError::Failed(AppError::ApiNotFound(format!(
                    "peer not found: {peer_id}"
                )))
            })?;

        let url = format!("http://{}:{}/health", peer.address, peer.port);
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .map_err(|e| DiscoveryAnnounceError::Failed(AppError::NetworkError(e.to_string())))?;

        match client.get(&url).send().await {
            Ok(response) => {
                let status = response.status();
                let code = status.as_u16();
                let body = response.text().await.unwrap_or_default();
                Ok(RemoteHealthProbe {
                    peer_id: peer_id.to_string(),
                    reachable: status.is_success(),
                    http_status: Some(code),
                    detail: if body.is_empty() { None } else { Some(body) },
                })
            }
            Err(e) => Ok(RemoteHealthProbe {
                peer_id: peer_id.to_string(),
                reachable: false,
                http_status: None,
                detail: Some(e.to_string()),
            }),
        }
    }
}

/// Dashboard row for a Telegram / device virtual node.
#[derive(Debug, Clone, Serialize)]
pub struct VirtualNodeStatus {
    pub peer: PeerInfo,
    pub stale: bool,
    pub last_seen_age_secs: i64,
    /// Galaxy §2.3 unified worker fields (PH-S507).
    pub galaxy: GalaxyWorkerDto,
}

#[derive(Debug, Clone, Serialize)]
pub struct RemoteHealthProbe {
    pub peer_id: String,
    pub reachable: bool,
    pub http_status: Option<u16>,
    pub detail: Option<String>,
}
