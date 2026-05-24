//! Inbound Raft RPC over HTTP (`/raft/*`). Used by `HttpRaftTransport` and PH-S06 harness.

#[cfg(feature = "raft")]
use crate::core::error::AppError;
#[cfg(feature = "raft")]
use crate::raid::raft::{RaidRaftNode, RaidRaftOperation};
#[cfg(feature = "raft")]
use async_raft::raft::{
    AppendEntriesRequest, AppendEntriesResponse, InstallSnapshotRequest, InstallSnapshotResponse,
    VoteRequest, VoteResponse,
};
#[cfg(feature = "raft")]
use axum::{extract::State, routing::post, Json, Router};
#[cfg(feature = "raft")]
use std::sync::Arc;

#[cfg(feature = "raft")]
async fn raft_append_entries_handler(
    State(node): State<Arc<RaidRaftNode>>,
    Json(rpc): Json<AppendEntriesRequest<RaidRaftOperation>>,
) -> Result<Json<AppendEntriesResponse>, AppError> {
    let resp = node.handle_append_entries(rpc).await?;
    Ok(Json(resp))
}

#[cfg(feature = "raft")]
async fn raft_vote_handler(
    State(node): State<Arc<RaidRaftNode>>,
    Json(rpc): Json<VoteRequest>,
) -> Result<Json<VoteResponse>, AppError> {
    let resp = node.handle_vote(rpc).await?;
    Ok(Json(resp))
}

#[cfg(feature = "raft")]
async fn raft_install_snapshot_handler(
    State(node): State<Arc<RaidRaftNode>>,
    Json(rpc): Json<InstallSnapshotRequest>,
) -> Result<Json<InstallSnapshotResponse>, AppError> {
    let resp = node.handle_install_snapshot(rpc).await?;
    Ok(Json(resp))
}

/// Routes expected by [`crate::raid::raft_transport::HttpRaftTransport`].
#[cfg(feature = "raft")]
pub fn create_raft_rpc_routes(node: Arc<RaidRaftNode>) -> Router {
    Router::new()
        .route("/raft/append-entries", post(raft_append_entries_handler))
        .route("/raft/vote", post(raft_vote_handler))
        .route(
            "/raft/install-snapshot",
            post(raft_install_snapshot_handler),
        )
        .with_state(node)
}
