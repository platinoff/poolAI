//! Discovery API endpoints
//!
//! Provides endpoints for device/worker discovery:
//! - List discovered peers
//! - Get peer information
//! - Register/unregister peer

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;

use crate::network::discovery::{get_global_discovery_service, PeerInfo};

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
pub fn create_discovery_routes() -> Router {
    Router::new()
        .route("/discovery/peers", get(peers_handler))
        .route("/discovery/peers/:peer_id", get(peer_handler))
        .route("/discovery/register", post(register_handler))
}

/// Handler for GET /api/v1/discovery/peers
/// Returns list of all discovered peers
async fn peers_handler() -> impl IntoResponse {
    if let Some(discovery) = get_global_discovery_service() {
        let peers = discovery.get_peers().await;
        let local_peer_id = discovery.local_peer_id().to_string();
        
        let response = PeersResponse {
            peers,
            local_peer_id,
        };
        
        return (StatusCode::OK, Json(response));
    }
    
    // Discovery service not initialized
    let response = PeersResponse {
        peers: vec![],
        local_peer_id: "not-initialized".to_string(),
    };
    
    (StatusCode::SERVICE_UNAVAILABLE, Json(response))
}

/// Handler for GET /api/v1/discovery/peers/:peer_id
/// Returns information about a specific peer
async fn peer_handler(Path(peer_id): Path<String>) -> impl IntoResponse {
    if let Some(discovery) = get_global_discovery_service() {
        let peer = discovery.get_peer(&peer_id).await;
        let response = PeerResponse { peer };
        return (StatusCode::OK, Json(response));
    }
    
    // Discovery service not initialized
    let response = PeerResponse { peer: None };
    (StatusCode::SERVICE_UNAVAILABLE, Json(response))
}

/// Handler for POST /api/v1/discovery/register
/// Registers this node as a peer
async fn register_handler() -> impl IntoResponse {
    if let Some(discovery) = get_global_discovery_service() {
        if let Err(e) = discovery.send_announcement().await {
            tracing::warn!("Failed to send discovery announcement: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
        return StatusCode::OK;
    }
    
    StatusCode::SERVICE_UNAVAILABLE
}
