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

use crate::network::discovery::PeerInfo;

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
    // TODO: Get discovery service from app state
    // For now, return empty list
    let response = PeersResponse {
        peers: vec![],
        local_peer_id: "poolai-local".to_string(),
    };
    
    (StatusCode::OK, Json(response))
}

/// Handler for GET /api/v1/discovery/peers/:peer_id
/// Returns information about a specific peer
async fn peer_handler(Path(_peer_id): Path<String>) -> impl IntoResponse {
    // TODO: Get discovery service from app state
    // For now, return None
    let response = PeerResponse {
        peer: None,
    };
    
    (StatusCode::OK, Json(response))
}

/// Handler for POST /api/v1/discovery/register
/// Registers this node as a peer
async fn register_handler() -> impl IntoResponse {
    // TODO: Trigger discovery announcement
    StatusCode::OK
}
