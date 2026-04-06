//! Thin axum entrypoints for the distributed RAID wire protocol.
//!
//! Orchestration: [`crate::services::raid_distributed_protocol_service::RaidDistributedProtocolService`].

use axum::extract::{Json, State};
use axum::response::IntoResponse;

use crate::core::state::ApiContext;
use crate::raid::protocol::ProtocolMessage;
use crate::services::raid_distributed_protocol_service::RaidDistributedProtocolService;

pub async fn put_artifact_handler(
    State(ctx): State<ApiContext>,
    Json(message): Json<ProtocolMessage>,
) -> impl IntoResponse {
    RaidDistributedProtocolService::put_artifact(&ctx, message).await
}

pub async fn get_artifact_handler(
    State(ctx): State<ApiContext>,
    Json(message): Json<ProtocolMessage>,
) -> impl IntoResponse {
    RaidDistributedProtocolService::get_artifact(&ctx, message).await
}

pub async fn delete_artifact_handler(
    State(ctx): State<ApiContext>,
    Json(message): Json<ProtocolMessage>,
) -> impl IntoResponse {
    RaidDistributedProtocolService::delete_artifact(&ctx, message).await
}

pub async fn sync_artifacts_handler(
    State(ctx): State<ApiContext>,
    Json(message): Json<ProtocolMessage>,
) -> impl IntoResponse {
    RaidDistributedProtocolService::sync_artifacts(&ctx, message).await
}

pub async fn health_check_handler(
    State(ctx): State<ApiContext>,
    Json(message): Json<ProtocolMessage>,
) -> impl IntoResponse {
    RaidDistributedProtocolService::health_check(&ctx, message).await
}

pub async fn join_cluster_handler(
    State(ctx): State<ApiContext>,
    Json(message): Json<ProtocolMessage>,
) -> impl IntoResponse {
    RaidDistributedProtocolService::join_cluster(&ctx, message).await
}

pub async fn leave_cluster_handler(Json(message): Json<ProtocolMessage>) -> impl IntoResponse {
    RaidDistributedProtocolService::leave_cluster(message).await
}
