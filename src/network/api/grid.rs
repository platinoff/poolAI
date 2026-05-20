//! Grid wire ingress (FM-023) — Job/Result/MemoryShard via `GridEnvelope` v1.

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::Serialize;

use crate::core::state::ApiContext;
use crate::grid::{ingest_envelope, GridEnvelope, GridIngestKind, GridIngestOutcome};
use crate::job::{JobStatus, JobStore};
use crate::memory::MemoryShardStore;
use crate::network::api::common::HttpAppError;

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum GridIngestResponseKind {
    Job { job_id: String, status: JobStatus },
    Result { job_id: String, status: JobStatus },
    MemoryShard { shard_id: String },
    PeerStatus { peer_id: String },
}

#[derive(Serialize)]
pub struct GridIngestResponse {
    ok: bool,
    #[serde(flatten)]
    kind: GridIngestResponseKind,
}

fn jobs() -> &'static JobStore {
    JobStore::global()
}

fn memory() -> &'static MemoryShardStore {
    MemoryShardStore::global()
}

pub fn create_grid_routes() -> Router<ApiContext> {
    Router::new().route("/grid/envelope", post(ingest_grid_envelope))
}

pub async fn ingest_grid_envelope_handler(
    envelope: Json<GridEnvelope>,
) -> Result<(StatusCode, Json<GridIngestResponse>), HttpAppError> {
    let outcome = ingest_envelope(envelope.0, jobs(), memory())?;
    Ok((StatusCode::OK, Json(response_from_outcome(outcome))))
}

async fn ingest_grid_envelope(
    State(_ctx): State<ApiContext>,
    envelope: Json<GridEnvelope>,
) -> Result<(StatusCode, Json<GridIngestResponse>), HttpAppError> {
    ingest_grid_envelope_handler(envelope).await
}

fn response_from_outcome(outcome: GridIngestOutcome) -> GridIngestResponse {
    let kind = match outcome.kind {
        GridIngestKind::Job { job_id, status } => GridIngestResponseKind::Job { job_id, status },
        GridIngestKind::Result { job_id, status } => {
            GridIngestResponseKind::Result { job_id, status }
        }
        GridIngestKind::MemoryShard { shard_id } => {
            GridIngestResponseKind::MemoryShard { shard_id }
        }
        GridIngestKind::PeerStatus { peer_id } => GridIngestResponseKind::PeerStatus { peer_id },
    };
    GridIngestResponse { ok: true, kind }
}
