//! Discovery API endpoints
//!
//! Provides endpoints for device/worker discovery:
//! - List discovered peers
//! - Get peer information
//! - Register/unregister peer

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::discovery_types::{PeerCapabilities, PeerInfo};
use crate::core::state::ApiContext;
use crate::services::discovery_service::{
    DiscoveryAnnounceError, DiscoveryNotReady, DiscoveryService, RemoteHealthProbe,
    VirtualNodeStatus,
};

/// Discovery API response types
#[derive(Serialize)]
struct PeersResponse {
    peers: Vec<PeerInfo>,
    local_peer_id: String,
}

#[derive(Serialize)]
struct PeerResponse {
    peer: Option<PeerInfo>,
}

#[derive(Deserialize)]
struct RegisterRemotePeerRequest {
    peer_id: String,
    address: String,
    port: u16,
    #[serde(default)]
    capabilities: PeerCapabilities,
    #[serde(default)]
    metadata: HashMap<String, String>,
}

#[derive(Serialize)]
struct RegisterRemotePeerResponse {
    peer_id: String,
    registered: bool,
}

#[derive(Deserialize)]
struct HeartbeatRemotePeerRequest {
    peer_id: String,
    #[serde(default)]
    capabilities: Option<PeerCapabilities>,
}

#[derive(Serialize)]
struct HeartbeatRemotePeerResponse {
    peer_id: String,
    ok: bool,
}

#[derive(Serialize)]
struct VirtualNodesResponse {
    nodes: Vec<VirtualNodeStatus>,
    stale_threshold_secs: i64,
}

/// Create discovery routes
pub fn create_discovery_routes() -> Router<ApiContext> {
    Router::new()
        .route("/discovery/peers", get(peers_handler))
        .route("/discovery/peers/{peer_id}", get(peer_handler))
        .route("/discovery/register", post(register_handler))
        .route("/discovery/register-remote", post(register_remote_handler))
        .route(
            "/discovery/heartbeat-remote",
            post(heartbeat_remote_handler),
        )
        .route("/discovery/virtual-nodes", get(virtual_nodes_handler))
        .route(
            "/discovery/virtual-nodes/{peer_id}/health",
            get(probe_virtual_node_health_handler),
        )
}

/// Handler for GET /api/v1/discovery/peers
/// Returns list of all discovered peers
async fn peers_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match DiscoveryService::list_peers(&ctx).await {
        Ok(snapshot) => {
            let response = PeersResponse {
                peers: snapshot.peers,
                local_peer_id: snapshot.local_peer_id,
            };
            (StatusCode::OK, Json(response))
        }
        Err(DiscoveryNotReady) => {
            let response = PeersResponse {
                peers: vec![],
                local_peer_id: "not-initialized".to_string(),
            };
            (StatusCode::SERVICE_UNAVAILABLE, Json(response))
        }
    }
}

/// Handler for GET /api/v1/discovery/peers/:peer_id
/// Returns information about a specific peer
async fn peer_handler(
    State(ctx): State<ApiContext>,
    Path(peer_id): Path<String>,
) -> impl IntoResponse {
    match DiscoveryService::get_peer(&ctx, &peer_id).await {
        Ok(peer) => (StatusCode::OK, Json(PeerResponse { peer })),
        Err(DiscoveryNotReady) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(PeerResponse { peer: None }),
        ),
    }
}

/// Handler for POST /api/v1/discovery/register
/// Registers this node as a peer
async fn register_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match DiscoveryService::send_announcement(&ctx).await {
        Ok(()) => StatusCode::OK,
        Err(DiscoveryAnnounceError::Failed(e)) => {
            tracing::warn!("Failed to send discovery announcement: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
        Err(DiscoveryAnnounceError::NotReady) => StatusCode::SERVICE_UNAVAILABLE,
    }
}

/// POST /api/v1/discovery/register-remote — HTTP registration for virtual nodes (FM-016).
async fn register_remote_handler(
    State(ctx): State<ApiContext>,
    Json(payload): Json<RegisterRemotePeerRequest>,
) -> impl IntoResponse {
    if payload.peer_id.trim().is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    match DiscoveryService::register_remote_peer(
        &ctx,
        payload.peer_id.clone(),
        payload.address,
        payload.port,
        payload.capabilities,
        payload.metadata,
    )
    .await
    {
        Ok(()) => (
            StatusCode::OK,
            Json(RegisterRemotePeerResponse {
                peer_id: payload.peer_id,
                registered: true,
            }),
        )
            .into_response(),
        Err(DiscoveryAnnounceError::Failed(e)) => {
            tracing::warn!("Failed to register remote peer: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
        Err(DiscoveryAnnounceError::NotReady) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}

/// POST /api/v1/discovery/heartbeat-remote — refresh virtual node liveness (FM-016 phase 2).
async fn heartbeat_remote_handler(
    State(ctx): State<ApiContext>,
    Json(payload): Json<HeartbeatRemotePeerRequest>,
) -> impl IntoResponse {
    if payload.peer_id.trim().is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    match DiscoveryService::heartbeat_remote_peer(&ctx, &payload.peer_id, payload.capabilities)
        .await
    {
        Ok(()) => (
            StatusCode::OK,
            Json(HeartbeatRemotePeerResponse {
                peer_id: payload.peer_id,
                ok: true,
            }),
        )
            .into_response(),
        Err(DiscoveryAnnounceError::Failed(e)) => {
            tracing::warn!("heartbeat-remote failed: {}", e);
            StatusCode::NOT_FOUND.into_response()
        }
        Err(DiscoveryAnnounceError::NotReady) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}

/// GET /api/v1/discovery/virtual-nodes — Telegram / device workers (FM-016 phase 2).
async fn virtual_nodes_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    const STALE_SECS: i64 = 90;
    match DiscoveryService::list_virtual_nodes(&ctx, STALE_SECS).await {
        Ok(nodes) => (
            StatusCode::OK,
            Json(VirtualNodesResponse {
                nodes,
                stale_threshold_secs: STALE_SECS,
            }),
        )
            .into_response(),
        Err(DiscoveryNotReady) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}

/// GET /api/v1/discovery/virtual-nodes/:peer_id/health — probe worker HTTP /health.
async fn probe_virtual_node_health_handler(
    State(ctx): State<ApiContext>,
    Path(peer_id): Path<String>,
) -> impl IntoResponse {
    match DiscoveryService::probe_remote_health(&ctx, &peer_id).await {
        Ok(probe) => {
            let status = if probe.reachable {
                StatusCode::OK
            } else {
                StatusCode::SERVICE_UNAVAILABLE
            };
            (status, Json(probe)).into_response()
        }
        Err(DiscoveryAnnounceError::Failed(e)) => {
            tracing::warn!("probe virtual node health failed: {}", e);
            (
                StatusCode::NOT_FOUND,
                Json(RemoteHealthProbe {
                    peer_id,
                    reachable: false,
                    http_status: None,
                    detail: Some(e.to_string()),
                }),
            )
                .into_response()
        }
        Err(DiscoveryAnnounceError::NotReady) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}
