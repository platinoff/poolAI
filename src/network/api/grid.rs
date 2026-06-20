//! Grid wire ingress (FM-023) — Job/Result/MemoryShard via `GridEnvelope` v1.

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::error::AppError;
use crate::core::state::ApiContext;
use crate::grid::galaxy_pricing_oracle::{
    cache_metadata, fetch_live_provider_quotes, observe_l1_cache_age_secs,
    provider_http_timeout_ms_from_env, record_l1_fresh_served, record_l1_stale_served,
    CacheFreshness, GalaxyPriceUnitKey, GalaxyPricingCacheEntry, GalaxyPricingCacheKey,
    GalaxyPricingCacheMetadata, GalaxyPricingConfig, GalaxyPricingOracle,
    GalaxyPricingProviderCatalog, GalaxyPricingProviderEntry, GalaxyPricingQuote,
    MockProviderQuote, PRICING_UNAVAILABLE_ERROR_CODE,
};
use crate::grid::galaxy_settlement_mode::{current_settlement_mode, settlement_on_chain_pending};
use crate::grid::{
    coordinator_seed_inventory_snapshot, ingest_envelope, GridEnvelope, GridIngestKind,
    GridIngestOutcome,
};
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
    Router::new()
        .route("/grid/envelope", post(ingest_grid_envelope))
        .route("/grid/pricing", get(get_grid_pricing_snapshot))
        .route("/grid/seed-inventory", get(get_grid_seed_inventory))
        .route(
            "/grid/verification-replay",
            get(get_grid_verification_replay),
        )
        .route(
            "/grid/verification-replay/history",
            get(get_grid_verification_replay_history),
        )
        .route(
            "/grid/verification-checker/tasks",
            get(get_grid_verification_checker_tasks),
        )
        .route(
            "/grid/verification-metrics",
            get(get_grid_verification_metrics),
        )
        .route("/grid/replay-metrics", get(get_grid_replay_metrics))
        .route("/grid/settlement-metrics", get(get_grid_settlement_metrics))
        .route("/grid/trust-metrics", get(get_grid_trust_metrics))
        .route(
            "/grid/network-profiles/{peer_id}",
            get(get_grid_network_profile).put(put_grid_network_profile),
        )
        .route("/grid/network-profiles", get(list_grid_network_profiles))
        .route("/grid/telegram-seats", get(get_grid_telegram_seats))
        .route("/grid/payout-batch", get(get_grid_payout_batch))
        .route(
            "/grid/payout-batch/history",
            get(get_grid_payout_batch_history),
        )
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

#[derive(Debug, Serialize)]
struct GridSeedInventoryResponse {
    ok: bool,
    entries: Vec<crate::grid::SeedInventoryPeerSnapshot>,
    generated_at: String,
}

async fn get_grid_seed_inventory(
    State(_ctx): State<ApiContext>,
) -> Result<(StatusCode, Json<GridSeedInventoryResponse>), HttpAppError> {
    Ok((
        StatusCode::OK,
        Json(GridSeedInventoryResponse {
            ok: true,
            entries: coordinator_seed_inventory_snapshot(),
            generated_at: chrono::Utc::now().to_rfc3339(),
        }),
    ))
}

#[derive(Debug, Serialize)]
struct GridVerificationReplayResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    record: Option<crate::grid::GalaxyVerificationReplayRecord>,
}

async fn get_grid_verification_replay(
    State(_ctx): State<ApiContext>,
) -> Result<(StatusCode, Json<GridVerificationReplayResponse>), HttpAppError> {
    Ok((
        StatusCode::OK,
        Json(GridVerificationReplayResponse {
            ok: true,
            record: crate::grid::galaxy_replay_metrics::last_verification_replay_record(),
        }),
    ))
}

#[derive(Debug, Serialize)]
struct PayoutRoutingSnapshot {
    settlement_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_dev_lamports: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_admin_lamports: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_lamports: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payout_pubkey: Option<String>,
}

#[derive(Debug, Serialize)]
struct GridPayoutBatchResponse {
    ok: bool,
    /// Settlement mechanism stub (PH-S531, Galaxy §8.2).
    settlement_mode: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_chain_pending: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing: Option<PayoutRoutingSnapshot>,
    entry: Option<crate::grid::galaxy_settlement::PayoutBatchLedgerEntry>,
}

fn payout_routing_snapshot(
    mode: &'static str,
    entry: &crate::grid::galaxy_settlement::PayoutBatchLedgerEntry,
) -> PayoutRoutingSnapshot {
    PayoutRoutingSnapshot {
        settlement_mode: mode.to_string(),
        primary_dev_lamports: entry.primary_dev_lamports,
        secondary_admin_lamports: entry.secondary_admin_lamports,
        worker_lamports: entry.worker_lamports,
        payout_pubkey: entry.payout_pubkey.clone(),
    }
}

async fn get_grid_payout_batch(
    State(_ctx): State<ApiContext>,
) -> Result<(StatusCode, Json<GridPayoutBatchResponse>), HttpAppError> {
    let mode = current_settlement_mode();
    let entry = crate::grid::galaxy_settlement_metrics::last_payout_batch_ledger_entry();
    let routing = entry.as_ref().map(|e| payout_routing_snapshot(mode, e));
    Ok((
        StatusCode::OK,
        Json(GridPayoutBatchResponse {
            ok: true,
            settlement_mode: mode,
            on_chain_pending: Some(settlement_on_chain_pending()),
            routing,
            entry,
        }),
    ))
}

#[derive(Debug, Deserialize)]
struct GridHistoryLimitQuery {
    #[serde(default = "default_history_limit")]
    limit: usize,
}

fn default_history_limit() -> usize {
    10
}

#[derive(Debug, Serialize)]
struct GridPayoutBatchHistoryResponse {
    ok: bool,
    entries: Vec<crate::grid::galaxy_settlement::PayoutBatchLedgerEntry>,
}

async fn get_grid_payout_batch_history(
    State(_ctx): State<ApiContext>,
    Query(params): Query<GridHistoryLimitQuery>,
) -> Result<(StatusCode, Json<GridPayoutBatchHistoryResponse>), HttpAppError> {
    Ok((
        StatusCode::OK,
        Json(GridPayoutBatchHistoryResponse {
            ok: true,
            entries: crate::grid::galaxy_settlement_metrics::payout_batch_history(params.limit),
        }),
    ))
}

#[derive(Debug, Serialize)]
struct GridVerificationReplayHistoryResponse {
    ok: bool,
    records: Vec<crate::grid::GalaxyVerificationReplayRecord>,
}

async fn get_grid_verification_replay_history(
    State(_ctx): State<ApiContext>,
    Query(params): Query<GridHistoryLimitQuery>,
) -> Result<(StatusCode, Json<GridVerificationReplayHistoryResponse>), HttpAppError> {
    Ok((
        StatusCode::OK,
        Json(GridVerificationReplayHistoryResponse {
            ok: true,
            records: crate::grid::galaxy_replay_metrics::verification_replay_history(params.limit),
        }),
    ))
}

#[derive(Debug, Serialize)]
struct GridVerificationCheckerTasksResponse {
    ok: bool,
    tasks: Vec<crate::grid::galaxy_verification_metrics::VerificationCheckerTask>,
}

async fn get_grid_verification_checker_tasks(
    State(_ctx): State<ApiContext>,
) -> Result<(StatusCode, Json<GridVerificationCheckerTasksResponse>), HttpAppError> {
    Ok((
        StatusCode::OK,
        Json(GridVerificationCheckerTasksResponse {
            ok: true,
            tasks: crate::grid::galaxy_verification_metrics::verification_checker_tasks(),
        }),
    ))
}

#[derive(Debug, Serialize)]
struct GridVerificationMetricsResponse {
    ok: bool,
    metrics: crate::grid::galaxy_verification_metrics::VerificationMetricsSnapshot,
}

async fn get_grid_verification_metrics(
    State(_ctx): State<ApiContext>,
) -> Result<(StatusCode, Json<GridVerificationMetricsResponse>), HttpAppError> {
    Ok((
        StatusCode::OK,
        Json(GridVerificationMetricsResponse {
            ok: true,
            metrics: crate::grid::galaxy_verification_metrics::verification_metrics_snapshot(),
        }),
    ))
}

#[derive(Debug, Serialize)]
struct GridReplayMetricsResponse {
    ok: bool,
    metrics: crate::grid::galaxy_replay_metrics::ReplayMetricsSnapshot,
}

async fn get_grid_replay_metrics(
    State(_ctx): State<ApiContext>,
) -> Result<(StatusCode, Json<GridReplayMetricsResponse>), HttpAppError> {
    Ok((
        StatusCode::OK,
        Json(GridReplayMetricsResponse {
            ok: true,
            metrics: crate::grid::galaxy_replay_metrics::replay_metrics_snapshot(),
        }),
    ))
}

#[derive(Debug, Serialize)]
struct GridSettlementMetricsResponse {
    ok: bool,
    metrics: crate::grid::galaxy_settlement_metrics::SettlementMetricsSnapshot,
}

async fn get_grid_settlement_metrics(
    State(_ctx): State<ApiContext>,
) -> Result<(StatusCode, Json<GridSettlementMetricsResponse>), HttpAppError> {
    Ok((
        StatusCode::OK,
        Json(GridSettlementMetricsResponse {
            ok: true,
            metrics: crate::grid::galaxy_settlement_metrics::settlement_metrics_snapshot(),
        }),
    ))
}

#[derive(Debug, Serialize)]
struct GridTrustMetricsResponse {
    ok: bool,
    metrics: crate::grid::galaxy_trust_score::TrustMetricsSnapshot,
}

async fn get_grid_trust_metrics(
    State(_ctx): State<ApiContext>,
) -> Result<(StatusCode, Json<GridTrustMetricsResponse>), HttpAppError> {
    Ok((
        StatusCode::OK,
        Json(GridTrustMetricsResponse {
            ok: true,
            metrics: crate::grid::galaxy_trust_score::trust_metrics_snapshot(),
        }),
    ))
}

#[derive(Debug, Serialize)]
struct GridNetworkProfileResponse {
    ok: bool,
    peer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_profile: Option<serde_json::Value>,
}

async fn get_grid_network_profile(
    State(_ctx): State<ApiContext>,
    axum::extract::Path(peer_id): axum::extract::Path<String>,
) -> Result<(StatusCode, Json<GridNetworkProfileResponse>), HttpAppError> {
    let network_profile =
        crate::grid::galaxy_network_profile_store::load_peer_network_profile(&peer_id)
            .and_then(|raw| serde_json::from_str(&raw).ok());
    Ok((
        StatusCode::OK,
        Json(GridNetworkProfileResponse {
            ok: true,
            peer_id,
            network_profile,
        }),
    ))
}

#[derive(Debug, Serialize)]
struct GridNetworkProfileListResponse {
    ok: bool,
    peer_ids: Vec<String>,
    count: usize,
}

/// `GET /api/v1/grid/network-profiles` — list persisted peer ids (PH-S570).
async fn list_grid_network_profiles(
    State(_ctx): State<ApiContext>,
) -> Result<(StatusCode, Json<GridNetworkProfileListResponse>), HttpAppError> {
    let mut peer_ids =
        crate::grid::galaxy_network_profile_store::list_persisted_network_profile_peer_ids();
    peer_ids.sort();
    let count = peer_ids.len();
    Ok((
        StatusCode::OK,
        Json(GridNetworkProfileListResponse {
            ok: true,
            peer_ids,
            count,
        }),
    ))
}

#[derive(Debug, Deserialize)]
struct PutGridNetworkProfileRequest {
    network_profile: serde_json::Value,
}

async fn put_grid_network_profile(
    State(_ctx): State<ApiContext>,
    axum::extract::Path(peer_id): axum::extract::Path<String>,
    Json(body): Json<PutGridNetworkProfileRequest>,
) -> Result<(StatusCode, Json<GridNetworkProfileResponse>), HttpAppError> {
    if peer_id.trim().is_empty() {
        return Err(AppError::RestError {
            code: "invalid_peer_id",
            message: "peer_id must not be empty".into(),
        }
        .into());
    }
    let profile =
        crate::grid::galaxy_network_profile::parse_network_profile_value(&body.network_profile)
            .map_err(|e| AppError::RestError {
                code: "invalid_network_profile",
                message: e.message,
            })?;
    let canonical = profile.to_storage_json().map_err(|e| AppError::RestError {
        code: "invalid_network_profile",
        message: e.message,
    })?;
    crate::grid::galaxy_network_profile_store::persist_peer_network_profile(&peer_id, &canonical)
        .map_err(HttpAppError::from)?;
    Ok((
        StatusCode::OK,
        Json(GridNetworkProfileResponse {
            ok: true,
            peer_id,
            network_profile: Some(serde_json::from_str(&canonical).unwrap_or(body.network_profile)),
        }),
    ))
}

#[derive(Debug, Serialize)]
struct GridTelegramSeatsResponse {
    ok: bool,
    #[serde(flatten)]
    snapshot: crate::services::telegram_seat_service::TelegramSeatCoordinatorSnapshot,
}

async fn get_grid_telegram_seats(
    State(_ctx): State<ApiContext>,
) -> Result<(StatusCode, Json<GridTelegramSeatsResponse>), HttpAppError> {
    Ok((
        StatusCode::OK,
        Json(GridTelegramSeatsResponse {
            ok: true,
            snapshot: crate::services::telegram_seat_service::telegram_seat_coordinator_snapshot(),
        }),
    ))
}

#[derive(Debug, Deserialize)]
struct GridPricingSnapshotQuery {
    task_profile: String,
    model_profile: String,
    unit_key: String,
}

#[derive(Debug, Serialize)]
struct GridPricingSnapshot {
    task_profile: String,
    model_profile: String,
    unit_key: GalaxyPriceUnitKey,
    market_min_usd_micro: u64,
    poolai_quote_usd_micro: u64,
    provider_id_at_min: String,
    cached_at_secs: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum GridPricingSnapshotSource {
    Cache,
    Oracle,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum GridPricingSnapshotFreshness {
    Fresh,
    Stale,
}

#[derive(Debug, Serialize)]
struct GridPricingSnapshotResponse {
    ok: bool,
    source: GridPricingSnapshotSource,
    freshness: GridPricingSnapshotFreshness,
    snapshot: GridPricingSnapshot,
    /// L1 TTL windows when `source` is `cache` (PH-S89); omitted for oracle/L2 paths.
    #[serde(skip_serializing_if = "Option::is_none")]
    l1_cache: Option<GalaxyPricingCacheMetadata>,
}

fn pricing_oracle() -> &'static Mutex<GalaxyPricingOracle> {
    static ORACLE: OnceLock<Mutex<GalaxyPricingOracle>> = OnceLock::new();
    ORACLE.get_or_init(|| {
        #[cfg(test)]
        {
            Mutex::new(GalaxyPricingOracle::new(GalaxyPricingConfig::default()))
        }
        #[cfg(not(test))]
        {
            Mutex::new(GalaxyPricingOracle::from_env())
        }
    })
}

fn now_secs() -> Result<u64, HttpAppError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| AppError::InternalError(format!("failed to read unix clock: {e}")))?;
    Ok(now.as_secs())
}

fn snapshot_from_quote(quote: GalaxyPricingQuote) -> GridPricingSnapshot {
    GridPricingSnapshot {
        task_profile: quote.task_profile,
        model_profile: quote.model_profile,
        unit_key: quote.unit_key,
        market_min_usd_micro: quote.market_min_usd_micro,
        poolai_quote_usd_micro: quote.poolai_quote_usd_micro,
        provider_id_at_min: quote.provider_id_at_min,
        cached_at_secs: quote.cached_at_secs,
    }
}

fn freshness_to_response(f: CacheFreshness) -> Option<GridPricingSnapshotFreshness> {
    match f {
        CacheFreshness::Fresh => Some(GridPricingSnapshotFreshness::Fresh),
        CacheFreshness::Stale => Some(GridPricingSnapshotFreshness::Stale),
        CacheFreshness::Expired => None,
    }
}

fn cache_hit_response(
    entry: GalaxyPricingCacheEntry,
    freshness: CacheFreshness,
    config: &GalaxyPricingConfig,
    now_secs: u64,
) -> GridPricingSnapshotResponse {
    match freshness {
        CacheFreshness::Fresh => record_l1_fresh_served(entry.quote.unit_key),
        CacheFreshness::Stale => record_l1_stale_served(entry.quote.unit_key),
        CacheFreshness::Expired => {}
    }
    let l1_cache = Some(cache_metadata(now_secs, entry.quote.cached_at_secs, config));
    if let Some(ref meta) = l1_cache {
        observe_l1_cache_age_secs(meta.cache_age_secs);
    }
    GridPricingSnapshotResponse {
        ok: true,
        source: GridPricingSnapshotSource::Cache,
        freshness: freshness_to_response(freshness).expect("fresh or stale"),
        snapshot: snapshot_from_quote(entry.quote),
        l1_cache,
    }
}

async fn get_grid_pricing_snapshot(
    State(_ctx): State<ApiContext>,
    Query(query): Query<GridPricingSnapshotQuery>,
) -> Result<(StatusCode, Json<GridPricingSnapshotResponse>), HttpAppError> {
    let unit_key = GalaxyPriceUnitKey::from_str(&query.unit_key).map_err(|_| {
        AppError::ValidationError(format!(
            "invalid `unit_key` '{}'; expected one of: inference_input_token, inference_output_token, inference_blended_token, gpu_second, job_flat",
            query.unit_key
        ))
    })?;
    let cache_key = GalaxyPricingCacheKey {
        task_profile: query.task_profile,
        model_profile: query.model_profile,
        unit_key,
    };
    let now = now_secs()?;
    let (config, provider_catalog) = {
        let oracle = pricing_oracle()
            .lock()
            .map_err(|_| AppError::InternalError("pricing oracle mutex poisoned".to_string()))?;
        if let Some((entry, freshness)) = oracle.lookup(now, &cache_key) {
            let serve_cached = if oracle.config().force_fallback {
                entry.quote.provider_id_at_min == "fallback_l2_config"
            } else {
                true
            };
            if serve_cached && freshness_to_response(freshness).is_some() {
                return Ok((
                    StatusCode::OK,
                    Json(cache_hit_response(entry, freshness, oracle.config(), now)),
                ));
            }
        }
        (*oracle.config(), oracle.provider_catalog().clone())
    };

    let live_providers = fetch_live_provider_quotes(
        &provider_catalog,
        &cache_key.task_profile,
        &cache_key.model_profile,
        cache_key.unit_key,
        provider_http_timeout_ms_from_env(),
    )
    .await;

    let mut oracle = pricing_oracle()
        .lock()
        .map_err(|_| AppError::InternalError("pricing oracle mutex poisoned".to_string()))?;
    if let Some((entry, freshness)) = oracle.lookup(now, &cache_key) {
        let serve_cached = if config.force_fallback {
            entry.quote.provider_id_at_min == "fallback_l2_config"
        } else {
            true
        };
        if serve_cached && freshness_to_response(freshness).is_some() {
            return Ok((
                StatusCode::OK,
                Json(cache_hit_response(entry, freshness, &config, now)),
            ));
        }
    }

    let quote = oracle
        .try_quote(now, cache_key, &live_providers)
        .map_err(|_| {
            HttpAppError::new(AppError::RestError {
            code: PRICING_UNAVAILABLE_ERROR_CODE,
            message:
                "grid pricing unavailable: no fresh or stale cache and L2 fallback not configured"
                    .to_string(),
        })
        .with_status(StatusCode::SERVICE_UNAVAILABLE)
        })?;
    Ok((
        StatusCode::OK,
        Json(GridPricingSnapshotResponse {
            ok: true,
            source: GridPricingSnapshotSource::Oracle,
            freshness: GridPricingSnapshotFreshness::Fresh,
            snapshot: snapshot_from_quote(quote),
            l1_cache: None,
        }),
    ))
}

fn response_from_outcome(outcome: GridIngestOutcome) -> GridIngestResponse {
    let kind = match outcome.kind {
        GridIngestKind::Job {
            job_id,
            status,
            replication_tier: _,
        } => GridIngestResponseKind::Job { job_id, status },
        GridIngestKind::Result {
            job_id,
            status,
            settlement_gate: _,
            verification_sample: _,
            settlement_status: _,
        } => GridIngestResponseKind::Result { job_id, status },
        GridIngestKind::MemoryShard { shard_id } => {
            GridIngestResponseKind::MemoryShard { shard_id }
        }
        GridIngestKind::PeerStatus { peer_id } => GridIngestResponseKind::PeerStatus { peer_id },
    };
    GridIngestResponse { ok: true, kind }
}

/// Reset pricing oracle for HTTP integration tests (PH-S144).
#[cfg(any(test, feature = "test-utils"))]
pub fn reset_pricing_oracle_for_tests(force_fallback: bool, fallback_quote_micro: Option<u64>) {
    use std::collections::HashMap;
    let mut fallback = HashMap::new();
    if let Some(v) = fallback_quote_micro {
        fallback.insert(GalaxyPriceUnitKey::InferenceBlendedToken, v);
    }
    let mut guard = pricing_oracle().lock().expect("pricing oracle lock");
    *guard = GalaxyPricingOracle::new(GalaxyPricingConfig {
        cache_ttl_secs: 300,
        max_stale_secs: 3600,
        force_fallback,
    })
    .with_l2_fallback_quotes(fallback);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::galaxy_pricing_oracle::MockProviderQuote;
    use axum::{routing::get, Router};
    use serde_json::json;
    use std::collections::HashMap;
    use std::sync::{Mutex as StdMutex, OnceLock as StdOnceLock};
    use tokio::net::TcpListener;

    static PRICING_TEST_LOCK: StdOnceLock<StdMutex<()>> = StdOnceLock::new();

    fn pricing_test_lock() -> std::sync::MutexGuard<'static, ()> {
        PRICING_TEST_LOCK
            .get_or_init(|| StdMutex::new(()))
            .lock()
            .expect("pricing test lock")
    }

    fn reset_oracle(force_fallback: bool, fallback_quote: Option<u64>) {
        let mut fallback = HashMap::new();
        if let Some(v) = fallback_quote {
            fallback.insert(GalaxyPriceUnitKey::InferenceBlendedToken, v);
        }
        let mut guard = pricing_oracle().lock().expect("pricing oracle lock");
        *guard = GalaxyPricingOracle::new(GalaxyPricingConfig {
            cache_ttl_secs: 300,
            max_stale_secs: 3600,
            force_fallback,
        })
        .with_l2_fallback_quotes(fallback);
    }

    async fn spawn_provider_server(usd_micro: u64) -> String {
        let app = Router::new().route(
            "/quote",
            get(move || async move {
                Json(json!({
                    "units": {
                        "inference_blended_token": usd_micro
                    }
                }))
            }),
        );
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        tokio::spawn(async move {
            let _ = axum::serve(listener, app).await;
        });
        format!("http://{addr}/quote")
    }

    fn query(unit_key: &str, model_profile: &str) -> GridPricingSnapshotQuery {
        GridPricingSnapshotQuery {
            task_profile: "inference:text".to_string(),
            model_profile: model_profile.to_string(),
            unit_key: unit_key.to_string(),
        }
    }

    #[tokio::test]
    async fn grid_pricing_snapshot_rejects_invalid_unit_key() {
        let _lock = pricing_test_lock();
        let res = get_grid_pricing_snapshot(
            State(ApiContext::default()),
            Query(query("not_a_valid_unit", "invalid-unit-test")),
        )
        .await;
        let err = res.expect_err("expected validation error");
        assert!(matches!(err.err, AppError::ValidationError(_)));
    }

    #[tokio::test]
    async fn grid_pricing_snapshot_returns_pricing_unavailable_without_cache_or_fallback() {
        let _lock = pricing_test_lock();
        reset_oracle(false, None);
        let res = get_grid_pricing_snapshot(
            State(ApiContext::default()),
            Query(query("inference_blended_token", "l3-hard-stop-test")),
        )
        .await;
        let err = res.expect_err("expected pricing unavailable");
        assert!(matches!(
            err.err,
            AppError::RestError {
                code: PRICING_UNAVAILABLE_ERROR_CODE,
                ..
            }
        ));
        assert_eq!(err.status_override, Some(StatusCode::SERVICE_UNAVAILABLE));
    }

    #[tokio::test]
    async fn grid_pricing_snapshot_uses_fallback_and_then_cache() {
        let _lock = pricing_test_lock();
        reset_oracle(true, Some(470_000));
        let first = get_grid_pricing_snapshot(
            State(ApiContext::default()),
            Query(query("inference_blended_token", "l2-fallback-test")),
        )
        .await
        .expect("fallback snapshot")
        .1
         .0;
        assert!(matches!(first.source, GridPricingSnapshotSource::Oracle));
        assert_eq!(first.snapshot.poolai_quote_usd_micro, 470_000);

        let second = get_grid_pricing_snapshot(
            State(ApiContext::default()),
            Query(query("inference_blended_token", "l2-fallback-test")),
        )
        .await
        .expect("cached snapshot")
        .1
         .0;
        assert!(matches!(second.source, GridPricingSnapshotSource::Cache));
        assert_eq!(second.snapshot.poolai_quote_usd_micro, 470_000);
        let meta = second.l1_cache.expect("L1 cache metadata on cache hit");
        assert_eq!(meta.cache_ttl_secs, 300);
        assert_eq!(meta.max_stale_secs, 3600);
    }

    #[tokio::test]
    async fn grid_pricing_snapshot_l1_cache_metadata_fresh_vs_stale() {
        let _lock = pricing_test_lock();
        reset_oracle(false, None);
        crate::grid::galaxy_pricing_oracle::reset_pricing_cache_age_for_test();
        let model = "l1-metadata-test";
        let key = GalaxyPricingCacheKey {
            task_profile: "inference:text".into(),
            model_profile: model.into(),
            unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
        };
        let wall_now = now_secs().expect("clock");
        let cached_at = wall_now.saturating_sub(500);
        let providers = [MockProviderQuote {
            provider_id: "openai_us".into(),
            unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
            usd_micro: 500_000,
            healthy: true,
        }];
        {
            let mut guard = pricing_oracle().lock().expect("pricing oracle lock");
            guard
                .refresh_from_providers(cached_at, key.clone(), &providers)
                .expect("seed cache");
        }

        let stale = get_grid_pricing_snapshot(
            State(ApiContext::default()),
            Query(query("inference_blended_token", model)),
        )
        .await
        .expect("stale cache snapshot")
        .1
         .0;
        assert!(matches!(
            stale.freshness,
            GridPricingSnapshotFreshness::Stale
        ));
        let stale_meta = stale.l1_cache.expect("stale metadata");
        assert!(stale_meta.cache_age_secs >= 500);
        assert!(stale_meta.cache_age_secs > stale_meta.cache_ttl_secs);
        assert_eq!(stale_meta.cache_fresh_until_secs, cached_at + 300);
        assert!(
            crate::grid::galaxy_pricing_oracle::pricing_cache_age_seconds() >= 500,
            "PH-S168: L1 stale hit observes cache age"
        );

        let fresh_cached_at = wall_now.saturating_sub(60);
        {
            let mut guard = pricing_oracle().lock().expect("pricing oracle lock");
            guard
                .refresh_from_providers(fresh_cached_at, key, &providers)
                .expect("fresh seed");
        }
        let fresh = get_grid_pricing_snapshot(
            State(ApiContext::default()),
            Query(query("inference_blended_token", model)),
        )
        .await
        .expect("fresh cache snapshot")
        .1
         .0;
        assert!(matches!(
            fresh.freshness,
            GridPricingSnapshotFreshness::Fresh
        ));
        let fresh_meta = fresh.l1_cache.expect("fresh metadata");
        assert!(fresh_meta.cache_age_secs <= fresh_meta.cache_ttl_secs);
        assert!(fresh_meta.cache_stale_until_secs > wall_now);
    }

    #[tokio::test]
    async fn grid_pricing_snapshot_force_fallback_skips_l1_cache() {
        let _lock = pricing_test_lock();
        reset_oracle(false, None);
        let mut fallback = HashMap::new();
        fallback.insert(GalaxyPriceUnitKey::InferenceBlendedToken, 470_000);
        let key = GalaxyPricingCacheKey {
            task_profile: "inference:text".into(),
            model_profile: "force-skip-l1".into(),
            unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
        };
        {
            let mut guard = pricing_oracle().lock().expect("pricing oracle lock");
            *guard = GalaxyPricingOracle::new(GalaxyPricingConfig {
                cache_ttl_secs: 300,
                max_stale_secs: 3600,
                force_fallback: false,
            })
            .with_l2_fallback_quotes(fallback);
            let providers = [MockProviderQuote {
                provider_id: "openai_us".into(),
                unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
                usd_micro: 500_000,
                healthy: true,
            }];
            guard
                .refresh_from_providers(0, key.clone(), &providers)
                .expect("seed L1 cache");
            guard.set_force_fallback_for_test(true);
        }

        let first = get_grid_pricing_snapshot(
            State(ApiContext::default()),
            Query(query("inference_blended_token", "force-skip-l1")),
        )
        .await
        .expect("forced L2 snapshot")
        .1
         .0;
        assert!(matches!(first.source, GridPricingSnapshotSource::Oracle));
        assert_eq!(first.snapshot.poolai_quote_usd_micro, 470_000);

        let second = get_grid_pricing_snapshot(
            State(ApiContext::default()),
            Query(query("inference_blended_token", "force-skip-l1")),
        )
        .await
        .expect("cached L2 snapshot")
        .1
         .0;
        assert!(matches!(second.source, GridPricingSnapshotSource::Cache));
        assert_eq!(second.snapshot.poolai_quote_usd_micro, 470_000);
        assert_eq!(second.snapshot.provider_id_at_min, "fallback_l2_config");
        reset_oracle(false, None);
    }

    #[tokio::test]
    async fn grid_pricing_snapshot_live_provider_http_refresh() {
        let _lock = pricing_test_lock();
        let endpoint = spawn_provider_server(500_000).await;
        {
            let mut guard = pricing_oracle().lock().expect("pricing oracle lock");
            *guard = GalaxyPricingOracle::new(GalaxyPricingConfig {
                cache_ttl_secs: 300,
                max_stale_secs: 3600,
                force_fallback: false,
            })
            .with_provider_catalog(GalaxyPricingProviderCatalog {
                providers: vec![GalaxyPricingProviderEntry {
                    provider_id: "live_openai_us".into(),
                    region: "us".into(),
                    model_profile: Some("live-http-model".into()),
                    task_profiles: vec!["inference:text".into()],
                    units: HashMap::from([("inference_blended_token".into(), 500_000)]),
                    endpoint: Some(endpoint),
                    enabled: true,
                }],
            });
        }

        let res = get_grid_pricing_snapshot(
            State(ApiContext::default()),
            Query(query("inference_blended_token", "live-http-model")),
        )
        .await
        .expect("live http quote")
        .1
         .0;

        assert!(matches!(res.source, GridPricingSnapshotSource::Oracle));
        assert_eq!(res.snapshot.market_min_usd_micro, 500_000);
        assert_eq!(res.snapshot.poolai_quote_usd_micro, 450_000);
        assert_eq!(res.snapshot.provider_id_at_min, "live_openai_us");
    }

    #[tokio::test]
    async fn grid_verification_checker_tasks_read_ph_s494() {
        use crate::grid::galaxy_verification_metrics::{
            enqueue_verification_checker_task, reset_verification_checker_tasks_for_test,
            reset_verification_metrics_for_test, verification_checker_tasks,
        };

        reset_verification_metrics_for_test();
        reset_verification_checker_tasks_for_test();
        enqueue_verification_checker_task("job-api-vc-1");

        let res = get_grid_verification_checker_tasks(State(ApiContext::default()))
            .await
            .expect("checker tasks")
            .1
             .0;
        assert!(res.ok);
        assert_eq!(res.tasks.len(), verification_checker_tasks().len());
        assert_eq!(res.tasks[0].job_id, "job-api-vc-1");

        reset_verification_metrics_for_test();
        reset_verification_checker_tasks_for_test();
    }

    #[tokio::test]
    async fn grid_verification_metrics_read_ph_s670() {
        use crate::grid::galaxy_verification_metrics::{
            record_verification_match, record_verification_mismatch,
            reset_verification_metrics_for_test, verification_metrics_snapshot,
        };

        reset_verification_metrics_for_test();
        record_verification_match();
        record_verification_mismatch();

        let res = get_grid_verification_metrics(State(ApiContext::default()))
            .await
            .expect("verification metrics")
            .1
             .0;
        assert!(res.ok);
        assert_eq!(res.metrics.match_total, 1);
        assert_eq!(res.metrics.mismatch_total, 1);
        assert_eq!(verification_metrics_snapshot().match_total, 1);

        reset_verification_metrics_for_test();
    }

    #[tokio::test]
    async fn grid_replay_metrics_read_ph_s671() {
        use crate::grid::galaxy_replay_metrics::{
            record_replay_pending_scheduled, replay_metrics_snapshot,
            reset_replay_pending_metrics_for_test,
        };

        reset_replay_pending_metrics_for_test();
        record_replay_pending_scheduled();

        let res = get_grid_replay_metrics(State(ApiContext::default()))
            .await
            .expect("replay metrics")
            .1
             .0;
        assert!(res.ok);
        assert_eq!(res.metrics.replay_pending, 1);
        assert_eq!(res.metrics.replay_pending_scheduled_total, 1);
        assert_eq!(replay_metrics_snapshot().replay_pending, 1);

        reset_replay_pending_metrics_for_test();
    }

    #[tokio::test]
    async fn grid_settlement_metrics_read_ph_s680() {
        use crate::grid::galaxy_settlement_metrics::{
            record_settlement_cleared, reset_settlement_metrics_for_test,
            settlement_metrics_snapshot,
        };

        reset_settlement_metrics_for_test();
        record_settlement_cleared();

        let res = get_grid_settlement_metrics(State(ApiContext::default()))
            .await
            .expect("settlement metrics")
            .1
             .0;
        assert!(res.ok);
        assert_eq!(res.metrics.cleared_total, 1);
        assert_eq!(settlement_metrics_snapshot().cleared_total, 1);

        reset_settlement_metrics_for_test();
    }

    #[tokio::test]
    async fn grid_trust_metrics_read_ph_s681() {
        use crate::grid::galaxy_trust_score::{
            evaluate_result_settlement_gate, reset_settlement_gate_metrics_for_test,
            trust_metrics_snapshot, TrustScoreGateConfig,
        };

        reset_settlement_gate_metrics_for_test();
        let cfg = TrustScoreGateConfig::default_stub();
        evaluate_result_settlement_gate(Some("tg-peer-1"), Some(55), &cfg);

        let res = get_grid_trust_metrics(State(ApiContext::default()))
            .await
            .expect("trust metrics")
            .1
             .0;
        assert!(res.ok);
        assert_eq!(res.metrics.payout_eligible_total, 1);
        assert_eq!(trust_metrics_snapshot().payout_eligible_total, 1);

        reset_settlement_gate_metrics_for_test();
    }

    #[tokio::test]
    async fn grid_network_profile_read_ph_s497() {
        use crate::grid::galaxy_network_profile_store::{
            persist_peer_network_profile, reset_network_profile_store_for_test,
        };

        reset_network_profile_store_for_test();
        let json = r#"{"region":"us-east","latency_ms_p50":20}"#;
        persist_peer_network_profile("peer-read-1", json).expect("persist");

        let res = get_grid_network_profile(
            State(ApiContext::default()),
            axum::extract::Path("peer-read-1".into()),
        )
        .await
        .expect("network profile")
        .1
         .0;
        assert!(res.ok);
        assert_eq!(res.peer_id, "peer-read-1");
        assert_eq!(
            res.network_profile
                .and_then(|v| v.get("region").and_then(|r| r.as_str()).map(String::from)),
            Some("us-east".into())
        );

        let missing = get_grid_network_profile(
            State(ApiContext::default()),
            axum::extract::Path("peer-missing".into()),
        )
        .await
        .expect("missing profile")
        .1
         .0;
        assert!(missing.network_profile.is_none());

        reset_network_profile_store_for_test();
    }
}
