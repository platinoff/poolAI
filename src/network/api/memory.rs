//! Memory shard API (FM-022) — shard refs backed by RAID artifacts.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::core::error::AppError;
use crate::core::state::ApiContext;
use crate::memory::{memory_shard_from_raid, MemoryShardId, MemoryShardRef, MemoryShardStore};
use crate::network::api::common::HttpAppError;

#[derive(Deserialize)]
struct RegisterShardRequest {
    artifact_id: String,
    version: String,
    #[serde(default)]
    shard_id: Option<String>,
    #[serde(default)]
    raid_logical_name: Option<String>,
    #[serde(default)]
    seed_hints: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct ListShardsQuery {
    raid_logical_name: Option<String>,
}

#[derive(Serialize)]
struct MemoryShardsListResponse {
    shards: Vec<MemoryShardRef>,
}

#[derive(Serialize)]
struct MemoryShardResponse {
    shard: MemoryShardRef,
}

fn store() -> &'static MemoryShardStore {
    MemoryShardStore::global()
}

pub fn create_memory_routes() -> Router<ApiContext> {
    Router::new()
        .route("/memory/shards", get(list_shards).post(register_shard))
        .route("/memory/shards/{shard_id}", get(get_shard))
}

async fn list_shards(
    State(_ctx): State<ApiContext>,
    Query(query): Query<ListShardsQuery>,
) -> Result<Json<MemoryShardsListResponse>, HttpAppError> {
    let shards = match query.raid_logical_name.as_deref() {
        Some(name) => store().list_by_raid_logical_name(name)?,
        None => store().list()?,
    };
    Ok(Json(MemoryShardsListResponse { shards }))
}

async fn register_shard(
    State(_ctx): State<ApiContext>,
    Json(body): Json<RegisterShardRequest>,
) -> Result<(StatusCode, Json<MemoryShardResponse>), HttpAppError> {
    if body.artifact_id.trim().is_empty() {
        return Err(HttpAppError::new(AppError::ValidationError(
            "artifact_id is required".into(),
        )));
    }
    if body.version.trim().is_empty() {
        return Err(HttpAppError::new(AppError::ValidationError(
            "version is required".into(),
        )));
    }

    let shard = match (&body.shard_id, &body.raid_logical_name) {
        (Some(id), _) if !id.trim().is_empty() => MemoryShardRef {
            shard_id: MemoryShardId::new(id.trim()),
            artifact_id: body.artifact_id,
            version: body.version,
            raid_logical_name: body.raid_logical_name,
            seed_hints: body.seed_hints,
        },
        (None, Some(name)) if !name.trim().is_empty() => {
            let mut shard = memory_shard_from_raid(name.trim(), body.artifact_id, body.version);
            shard.seed_hints = body.seed_hints;
            shard
        }
        _ => {
            return Err(HttpAppError::new(AppError::ValidationError(
                "shard_id or raid_logical_name is required".into(),
            )));
        }
    };

    let saved = store().upsert(shard)?;
    Ok((
        StatusCode::CREATED,
        Json(MemoryShardResponse { shard: saved }),
    ))
}

async fn get_shard(
    State(_ctx): State<ApiContext>,
    Path(shard_id): Path<String>,
) -> Result<Json<MemoryShardResponse>, HttpAppError> {
    let shard = store().get(&shard_id)?.ok_or_else(|| {
        HttpAppError::new(AppError::ApiNotFound(format!(
            "memory shard '{shard_id}' not found"
        )))
    })?;
    Ok(Json(MemoryShardResponse { shard }))
}
