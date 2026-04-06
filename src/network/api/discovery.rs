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
use serde::Serialize;

use crate::core::discovery_types::PeerInfo;
use crate::core::state::ApiContext;
use crate::services::discovery_service::{
    DiscoveryAnnounceError, DiscoveryNotReady, DiscoveryService,
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

/// Create discovery routes
pub fn create_discovery_routes() -> Router<ApiContext> {
    Router::new()
        .route("/discovery/peers", get(peers_handler))
        .route("/discovery/peers/{peer_id}", get(peer_handler))
        .route("/discovery/register", post(register_handler))
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
