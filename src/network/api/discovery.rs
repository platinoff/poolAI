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
use serde_json::Value;
use std::collections::HashMap;

use crate::core::discovery_types::{PeerCapabilities, PeerInfo};
use crate::core::error::{AppError, ErrorContext};
use crate::core::state::ApiContext;
use crate::grid::galaxy_network_profile::normalize_register_metadata;
use crate::grid::protocol_compat::{negotiate, CompatStatus, MIN_COORDINATOR_VERSION_DOCS_URL};
use crate::grid::GridEnvelope;
use crate::network::api::common::HttpAppError;
use crate::network::api::grid::ingest_grid_envelope_handler;
use crate::services::discovery_service::{
    DiscoveryAnnounceError, DiscoveryNotReady, DiscoveryService, RemoteHealthProbe,
    VirtualNodeStatus,
};
use crate::services::virtual_node_task_service::VirtualNodeTaskService;
use crate::services::virtual_node_telegram_binding_service::VirtualNodeTelegramBindingService;

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
    /// Galaxy §9.3 wire protocol (`1.0`, `1.1`, `1.2.x`). Omitted = legacy accept.
    #[serde(default)]
    protocol_version: Option<String>,
    /// Release / build identifier (optional signed-release pin, PH-S65).
    #[serde(default)]
    build_id: Option<String>,
    #[serde(default)]
    signature_fingerprint: Option<String>,
    #[serde(default)]
    capabilities: PeerCapabilities,
    #[serde(default)]
    metadata: HashMap<String, Value>,
}

#[derive(Serialize)]
struct RegisterRemotePeerResponse {
    peer_id: String,
    registered: bool,
    compat_status: CompatStatus,
    coordinator_protocol_version: String,
    min_coordinator_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_protocol_version: Option<String>,
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
        .route(
            "/discovery/grid/envelope",
            post(grid_ingest_envelope_handler),
        )
}

async fn grid_ingest_envelope_handler(
    State(_ctx): State<ApiContext>,
    envelope: Json<GridEnvelope>,
) -> Result<
    (
        StatusCode,
        Json<crate::network::api::grid::GridIngestResponse>,
    ),
    HttpAppError,
> {
    ingest_grid_envelope_handler(envelope).await
}

fn discovery_not_ready(op: &'static str) -> HttpAppError {
    HttpAppError::new(AppError::SubsystemUnavailable(
        "Discovery service is not initialized".to_string(),
    ))
    .with_context(ErrorContext::new(op))
    .with_status(StatusCode::SERVICE_UNAVAILABLE)
}

fn discovery_validation(op: &'static str, message: impl Into<String>) -> HttpAppError {
    HttpAppError::new(AppError::ValidationError(message.into())).with_context(ErrorContext::new(op))
}

fn discovery_announce_failed(op: &'static str, e: impl std::fmt::Display) -> HttpAppError {
    tracing::warn!("{op} failed: {e}");
    HttpAppError::new(AppError::InternalError(format!("{op} failed: {e}")))
        .with_context(ErrorContext::new(op))
}

fn discovery_peer_not_found(
    op: &'static str,
    peer_id: &str,
    e: impl std::fmt::Display,
) -> HttpAppError {
    tracing::warn!("{op} failed for {peer_id}: {e}");
    HttpAppError::new(AppError::ApiNotFound(format!("{op} failed: {e}")))
        .with_context(ErrorContext::new(op).with_resource("peer_id", peer_id))
        .with_status(StatusCode::NOT_FOUND)
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
        Ok(()) => StatusCode::OK.into_response(),
        Err(DiscoveryAnnounceError::Failed(e)) => {
            discovery_announce_failed("discovery_register", e).into_response()
        }
        Err(DiscoveryAnnounceError::NotReady) => {
            discovery_not_ready("discovery_register").into_response()
        }
    }
}

fn register_compat_response(
    peer_id: String,
    negotiation: &crate::grid::protocol_compat::ProtocolNegotiation,
    registered: bool,
) -> RegisterRemotePeerResponse {
    RegisterRemotePeerResponse {
        peer_id,
        registered,
        compat_status: negotiation.status,
        coordinator_protocol_version: negotiation.coordinator_protocol_version.clone(),
        min_coordinator_version: MIN_COORDINATOR_VERSION_DOCS_URL.to_string(),
        worker_protocol_version: negotiation.worker_protocol_version.clone(),
    }
}

/// POST /api/v1/discovery/register-remote — HTTP registration for virtual nodes (FM-016).
async fn register_remote_handler(
    State(ctx): State<ApiContext>,
    Json(payload): Json<RegisterRemotePeerRequest>,
) -> impl IntoResponse {
    if payload.peer_id.trim().is_empty() {
        return discovery_validation("register_remote", "peer_id must not be empty")
            .into_response();
    }

    let negotiation = negotiate(payload.protocol_version.as_deref());
    let peer_id = payload.peer_id.clone();

    if negotiation.status == CompatStatus::Unsupported {
        return (
            StatusCode::FORBIDDEN,
            Json(register_compat_response(peer_id, &negotiation, false)),
        )
            .into_response();
    }

    let mut metadata = match normalize_register_metadata(payload.metadata) {
        Ok(m) => m,
        Err(e) => {
            return discovery_validation("register_remote", e.message).into_response();
        }
    };
    if let Some(build_id) = payload.build_id {
        metadata.insert("build_id".to_string(), build_id);
    }
    if let Some(fp) = payload.signature_fingerprint {
        metadata.insert("signature_fingerprint".to_string(), fp);
    }
    if let Some(worker_ver) = &negotiation.worker_protocol_version {
        metadata.insert("protocol_version".to_string(), worker_ver.clone());
    }

    let is_virtual_node = metadata.get("role").map(String::as_str) == Some("virtual_node");
    let telegram_id = metadata.get("telegram_id").cloned();
    let telegram_chat_id = metadata.get("telegram_chat_id").cloned();

    match DiscoveryService::register_remote_peer(
        &ctx,
        peer_id.clone(),
        payload.address,
        payload.port,
        payload.capabilities,
        metadata,
    )
    .await
    {
        Ok(()) => {
            if is_virtual_node {
                VirtualNodeTaskService::enqueue_bootstrap_tasks(&peer_id);
                if let Some(tg) = telegram_id {
                    VirtualNodeTelegramBindingService::bind(&tg, telegram_chat_id, &peer_id);
                }
            }
            let status = match negotiation.status {
                CompatStatus::UpgradeRequired => StatusCode::UPGRADE_REQUIRED,
                _ => StatusCode::OK,
            };
            (
                status,
                Json(register_compat_response(peer_id, &negotiation, true)),
            )
                .into_response()
        }
        Err(DiscoveryAnnounceError::Failed(e)) => {
            discovery_announce_failed("register_remote", e).into_response()
        }
        Err(DiscoveryAnnounceError::NotReady) => {
            discovery_not_ready("register_remote").into_response()
        }
    }
}

/// POST /api/v1/discovery/heartbeat-remote — refresh virtual node liveness (FM-016 phase 2).
async fn heartbeat_remote_handler(
    State(ctx): State<ApiContext>,
    Json(payload): Json<HeartbeatRemotePeerRequest>,
) -> impl IntoResponse {
    if payload.peer_id.trim().is_empty() {
        return discovery_validation("heartbeat_remote", "peer_id must not be empty")
            .into_response();
    }
    let peer_id = payload.peer_id.clone();
    match DiscoveryService::heartbeat_remote_peer(&ctx, &peer_id, payload.capabilities).await {
        Ok(()) => (
            StatusCode::OK,
            Json(HeartbeatRemotePeerResponse { peer_id, ok: true }),
        )
            .into_response(),
        Err(DiscoveryAnnounceError::Failed(e)) => {
            discovery_peer_not_found("heartbeat_remote", &peer_id, e).into_response()
        }
        Err(DiscoveryAnnounceError::NotReady) => {
            discovery_not_ready("heartbeat_remote").into_response()
        }
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
        Err(DiscoveryNotReady) => discovery_not_ready("virtual_nodes_list").into_response(),
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
            tracing::warn!("probe virtual node health failed for {peer_id}: {e}");
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
        Err(DiscoveryAnnounceError::NotReady) => {
            discovery_not_ready("probe_virtual_node_health").into_response()
        }
    }
}
