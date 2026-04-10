//! RAID Administrative Control Plane API endpoints
//!
//! Provides administrative endpoints for managing RAID strategies:
//! - Strategy status and metrics
//! - Manual rebalancing
//! - Artifact and node statistics
//! - Strategy configuration

use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::core::error::ErrorContext;
use crate::core::state::ApiContext;
use crate::network::api::common::check_permission;
use crate::network::api::raid_http::{raid_api_err, raid_manager_unavailable};
use crate::network::auth::{auth_middleware, Claims};
use crate::raid::StrategyStatus;
use crate::services::raid_service::{RaidService, RaidServiceError};

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
async fn get_strategy_status_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match RaidService::admin_strategy_status(&ctx).await {
        Ok(status) => (StatusCode::OK, Json(StrategyStatusResponse { status })).into_response(),
        Err(RaidServiceError::ManagerUnavailable) => raid_manager_unavailable().into_response(),
        Err(RaidServiceError::Operation(e)) => raid_api_err(
            "RAID_ADMIN_STRATEGY_STATUS_FAILED",
            format!("Failed to get strategy status: {}", e),
            Some(ErrorContext::new("raid_admin_strategy_status")),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
        Err(e) => raid_api_err(
            "RAID_ADMIN_STRATEGY_STATUS_FAILED",
            format!("Unexpected RAID service error: {:?}", e),
            Some(ErrorContext::new("raid_admin_strategy_status")),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

/// Trigger manual rebalancing
///
/// Manually triggers rebalancing of artifacts across nodes.
async fn trigger_rebalance_handler(
    State(ctx): State<ApiContext>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    if let Err(err) = check_permission(&claims, "admin") {
        return err.into_response();
    }

    match RaidService::admin_trigger_rebalance(&ctx).await {
        Ok(result) => (
            StatusCode::OK,
            Json(RebalanceResponse {
                artifacts_moved: result.artifacts_moved,
                success: result.success,
                message: format!("Rebalanced {} artifacts", result.artifacts_moved),
            }),
        )
            .into_response(),
        Err(RaidServiceError::ManagerUnavailable) => raid_manager_unavailable().into_response(),
        Err(RaidServiceError::Operation(e)) => raid_api_err(
            "RAID_ADMIN_REBALANCE_FAILED",
            format!("Failed to trigger rebalancing: {}", e),
            Some(ErrorContext::new("raid_admin_rebalance")),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
        Err(e) => raid_api_err(
            "RAID_ADMIN_REBALANCE_FAILED",
            format!("Unexpected RAID service error: {:?}", e),
            Some(ErrorContext::new("raid_admin_rebalance")),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

/// Get BurstRAID metrics
///
/// Returns detailed metrics about BurstRAID strategy if active.
async fn get_burst_raid_metrics_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match RaidService::admin_burst_raid_metrics(&ctx).await {
        Ok(Some(metrics)) => {
            (StatusCode::OK, Json(BurstRaidMetricsResponse { metrics })).into_response()
        }
        Ok(None) => raid_api_err(
            "BURSTRAID_NOT_ACTIVE",
            "BurstRAID strategy not active; metrics are only available when BurstRAID is active.",
            Some(ErrorContext::new("raid_admin_burst_metrics")),
            StatusCode::NOT_FOUND,
        )
        .into_response(),
        Err(RaidServiceError::ManagerUnavailable) => raid_manager_unavailable().into_response(),
        Err(e) => raid_api_err(
            "RAID_ADMIN_BURST_METRICS_FAILED",
            format!("Unexpected RAID service error: {:?}", e),
            Some(ErrorContext::new("raid_admin_burst_metrics")),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

/// Get SmallWorld metrics
///
/// Returns detailed metrics about SmallWorld strategy if active.
async fn get_small_world_metrics_handler(State(ctx): State<ApiContext>) -> impl IntoResponse {
    match RaidService::admin_small_world_metrics(&ctx).await {
        Ok(Some(metrics)) => {
            (StatusCode::OK, Json(SmallWorldMetricsResponse { metrics })).into_response()
        }
        Ok(None) => raid_api_err(
            "SMALLWORLD_NOT_ACTIVE",
            "SmallWorld strategy not active; metrics are only available when SmallWorld is active.",
            Some(ErrorContext::new("raid_admin_smallworld_metrics")),
            StatusCode::NOT_FOUND,
        )
        .into_response(),
        Err(RaidServiceError::ManagerUnavailable) => raid_manager_unavailable().into_response(),
        Err(e) => raid_api_err(
            "RAID_ADMIN_SMALLWORLD_METRICS_FAILED",
            format!("Unexpected RAID service error: {:?}", e),
            Some(ErrorContext::new("raid_admin_smallworld_metrics")),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

/// Get artifact burst stats (BurstRAID only)
///
/// Returns burst statistics for a specific artifact.
async fn get_artifact_burst_stats_handler(
    Path(artifact_id): Path<String>,
    State(ctx): State<ApiContext>,
) -> impl IntoResponse {
    let artifact_uuid = match Uuid::parse_str(&artifact_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return raid_api_err(
                "INVALID_ARTIFACT_ID",
                "Artifact ID must be a valid UUID",
                Some(
                    ErrorContext::new("raid_admin_artifact_burst_stats")
                        .with_resource("artifact_id", &artifact_id),
                ),
                StatusCode::BAD_REQUEST,
            )
            .into_response();
        }
    };

    match RaidService::admin_artifact_burst_stats(&ctx, artifact_uuid).await {
        Ok(Some(stats)) => {
            (StatusCode::OK, Json(ArtifactBurstStatsResponse { stats })).into_response()
        }
        Ok(None) => raid_api_err(
            "ARTIFACT_BURST_STATS_NOT_FOUND",
            "Artifact burst stats not found; artifact may be untracked or BurstRAID is not active.",
            Some(
                ErrorContext::new("raid_admin_artifact_burst_stats")
                    .with_resource("artifact_id", &artifact_id),
            ),
            StatusCode::NOT_FOUND,
        )
        .into_response(),
        Err(RaidServiceError::ManagerUnavailable) => raid_manager_unavailable().into_response(),
        Err(e) => raid_api_err(
            "RAID_ADMIN_ARTIFACT_BURST_STATS_FAILED",
            format!("Unexpected RAID service error: {:?}", e),
            Some(
                ErrorContext::new("raid_admin_artifact_burst_stats")
                    .with_resource("artifact_id", &artifact_id),
            ),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

/// Get node clustering coefficient (SmallWorld only)
///
/// Returns clustering coefficient for a specific node.
async fn get_node_clustering_handler(
    Query(params): Query<NodeIdQuery>,
    State(ctx): State<ApiContext>,
) -> impl IntoResponse {
    match RaidService::admin_node_clustering_coefficient(&ctx, params.node_id).await {
        Ok(Some(coeff)) => (
            StatusCode::OK,
            Json(NodeClusteringResponse {
                node_id: params.node_id,
                clustering_coefficient: coeff,
            }),
        )
            .into_response(),
        Ok(None) => raid_api_err(
            "NODE_CLUSTERING_NOT_FOUND",
            "Node clustering coefficient not found; node may not exist or SmallWorld is not active.",
            Some(
                ErrorContext::new("raid_admin_node_clustering").with_resource(
                    "node_id",
                    params.node_id.to_string(),
                ),
            ),
            StatusCode::NOT_FOUND,
        )
        .into_response(),
        Err(RaidServiceError::ManagerUnavailable) => raid_manager_unavailable().into_response(),
        Err(e) => raid_api_err(
            "RAID_ADMIN_NODE_CLUSTERING_FAILED",
            format!("Unexpected RAID service error: {:?}", e),
            Some(
                ErrorContext::new("raid_admin_node_clustering")
                    .with_resource("node_id", params.node_id.to_string()),
            ),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response(),
    }
}

/// Create RAID admin routes
pub fn create_raid_admin_routes() -> Router<ApiContext> {
    Router::new()
        .route("/raid/admin/status", get(get_strategy_status_handler))
        .route(
            "/raid/admin/rebalance",
            post(trigger_rebalance_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/raid/admin/metrics/burst",
            get(get_burst_raid_metrics_handler),
        )
        .route(
            "/raid/admin/metrics/smallworld",
            get(get_small_world_metrics_handler),
        )
        .route(
            "/raid/admin/artifacts/{id}/burst",
            get(get_artifact_burst_stats_handler),
        )
        .route(
            "/raid/admin/nodes/clustering",
            get(get_node_clustering_handler),
        )
}
