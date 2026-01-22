//! RAID Administrative Control Plane API endpoints
//!
//! Provides administrative endpoints for managing RAID strategies:
//! - Strategy status and metrics
//! - Manual rebalancing
//! - Artifact and node statistics
//! - Strategy configuration

use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::network::api::common::check_permission;
use crate::network::auth::{auth_middleware, Claims};
use crate::raid::admin::RaidAdmin;
use crate::raid::{RaidManager, StrategyStatus};

#[derive(serde::Serialize)]
struct StrategyStatusResponse {
    status: StrategyStatus,
}

#[derive(serde::Serialize)]
struct RebalanceResponse {
    artifacts_moved: usize,
    success: bool,
    message: String,
}

#[derive(serde::Serialize)]
struct BurstRaidMetricsResponse {
    metrics: crate::raid::burst_raid::BurstRaidMetrics,
}

#[derive(serde::Serialize)]
struct SmallWorldMetricsResponse {
    metrics: crate::raid::small_world::SmallWorldMetrics,
}

#[derive(serde::Serialize)]
struct ArtifactBurstStatsResponse {
    stats: crate::raid::burst_raid::ArtifactBurstStats,
}

#[derive(serde::Serialize)]
struct NodeClusteringResponse {
    node_id: u64,
    clustering_coefficient: f64,
}

#[derive(Deserialize)]
struct NodeIdQuery {
    node_id: u64,
}

/// Get current strategy status
///
/// Returns information about the active RAID strategy.
async fn get_strategy_status_handler(
    Extension(raid_manager): Extension<std::sync::Arc<tokio::sync::RwLock<RaidManager>>>,
) -> impl IntoResponse {
    let admin = RaidAdmin::new(raid_manager);
    
    match admin.get_strategy_status().await {
        Ok(status) => (
            StatusCode::OK,
            Json(StrategyStatusResponse { status }),
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Failed to get strategy status",
                "message": e.to_string()
            })),
        ).into_response(),
    }
}

/// Trigger manual rebalancing
///
/// Manually triggers rebalancing of artifacts across nodes.
async fn trigger_rebalance_handler(
    Extension(claims): Extension<Claims>,
    Extension(raid_manager): Extension<std::sync::Arc<tokio::sync::RwLock<RaidManager>>>,
) -> impl IntoResponse {
    // Check admin permission
    if let Err((status, json)) = check_permission(&claims, "admin") {
        return (status, json).into_response();
    }

    let admin = RaidAdmin::new(raid_manager);
    
    match admin.trigger_rebalance().await {
        Ok(result) => (
            StatusCode::OK,
            Json(RebalanceResponse {
                artifacts_moved: result.artifacts_moved,
                success: result.success,
                message: format!("Rebalanced {} artifacts", result.artifacts_moved),
            }),
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Failed to trigger rebalancing",
                "message": e.to_string()
            })),
        ).into_response(),
    }
}

/// Get BurstRAID metrics
///
/// Returns detailed metrics about BurstRAID strategy if active.
async fn get_burst_raid_metrics_handler(
    Extension(raid_manager): Extension<std::sync::Arc<tokio::sync::RwLock<RaidManager>>>,
) -> impl IntoResponse {
    let admin = RaidAdmin::new(raid_manager);
    
    match admin.get_burst_raid_metrics().await {
        Some(metrics) => (
            StatusCode::OK,
            Json(BurstRaidMetricsResponse { metrics }),
        ).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "BurstRAID strategy not active",
                "message": "BurstRAID metrics are only available when BurstRAID strategy is active"
            })),
        ).into_response(),
    }
}

/// Get SmallWorld metrics
///
/// Returns detailed metrics about SmallWorld strategy if active.
async fn get_small_world_metrics_handler(
    Extension(raid_manager): Extension<std::sync::Arc<tokio::sync::RwLock<RaidManager>>>,
) -> impl IntoResponse {
    let admin = RaidAdmin::new(raid_manager);
    
    match admin.get_small_world_metrics().await {
        Some(metrics) => (
            StatusCode::OK,
            Json(SmallWorldMetricsResponse { metrics }),
        ).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "SmallWorld strategy not active",
                "message": "SmallWorld metrics are only available when SmallWorld strategy is active"
            })),
        ).into_response(),
    }
}

/// Get artifact burst stats (BurstRAID only)
///
/// Returns burst statistics for a specific artifact.
async fn get_artifact_burst_stats_handler(
    Path(artifact_id): Path<String>,
    Extension(raid_manager): Extension<std::sync::Arc<tokio::sync::RwLock<RaidManager>>>,
) -> impl IntoResponse {
    let artifact_uuid = match Uuid::parse_str(&artifact_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Invalid artifact ID",
                    "message": "Artifact ID must be a valid UUID"
                })),
            ).into_response();
        }
    };

    let admin = RaidAdmin::new(raid_manager);
    
    match admin.get_artifact_burst_stats(artifact_uuid).await {
        Some(stats) => (
            StatusCode::OK,
            Json(ArtifactBurstStatsResponse { stats }),
        ).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Artifact burst stats not found",
                "message": "Artifact is not tracked or BurstRAID strategy is not active"
            })),
        ).into_response(),
    }
}

/// Get node clustering coefficient (SmallWorld only)
///
/// Returns clustering coefficient for a specific node.
async fn get_node_clustering_handler(
    Query(params): Query<NodeIdQuery>,
    Extension(raid_manager): Extension<std::sync::Arc<tokio::sync::RwLock<RaidManager>>>,
) -> impl IntoResponse {
    let admin = RaidAdmin::new(raid_manager);
    
    match admin.get_node_clustering_coefficient(params.node_id).await {
        Some(coeff) => (
            StatusCode::OK,
            Json(NodeClusteringResponse {
                node_id: params.node_id,
                clustering_coefficient: coeff,
            }),
        ).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Node clustering coefficient not found",
                "message": "Node does not exist or SmallWorld strategy is not active"
            })),
        ).into_response(),
    }
}

/// Create RAID admin routes
pub fn create_raid_admin_routes() -> Router {
    Router::new()
        .route("/raid/admin/status", get(get_strategy_status_handler))
        .route(
            "/raid/admin/rebalance",
            post(trigger_rebalance_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route("/raid/admin/metrics/burst", get(get_burst_raid_metrics_handler))
        .route("/raid/admin/metrics/smallworld", get(get_small_world_metrics_handler))
        .route(
            "/raid/admin/artifacts/{id}/burst",
            get(get_artifact_burst_stats_handler),
        )
        .route(
            "/raid/admin/nodes/clustering",
            get(get_node_clustering_handler),
        )
}
