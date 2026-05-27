//! Grid wire ingress (FM-023) — Job/Result/MemoryShard via `GridEnvelope` v1.

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::error::AppError;
use crate::core::state::ApiContext;
use crate::grid::galaxy_pricing_oracle::{
    CacheFreshness, GalaxyPriceUnitKey, GalaxyPricingCacheKey, GalaxyPricingConfig,
    GalaxyPricingOracle, GalaxyPricingQuote, PRICING_UNAVAILABLE_ERROR_CODE,
};
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
    Router::new()
        .route("/grid/envelope", post(ingest_grid_envelope))
        .route("/grid/pricing", get(get_grid_pricing_snapshot))
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
    let mut oracle = pricing_oracle()
        .lock()
        .map_err(|_| AppError::InternalError("pricing oracle mutex poisoned".to_string()))?;

    if let Some((entry, freshness)) = oracle.lookup(now, &cache_key) {
        let serve_cached = if oracle.config().force_fallback {
            entry.quote.provider_id_at_min == "fallback_l2_config"
        } else {
            true
        };
        if serve_cached {
            if let Some(freshness) = freshness_to_response(freshness) {
                return Ok((
                    StatusCode::OK,
                    Json(GridPricingSnapshotResponse {
                        ok: true,
                        source: GridPricingSnapshotSource::Cache,
                        freshness,
                        snapshot: snapshot_from_quote(entry.quote),
                    }),
                ));
            }
        }
    }

    let quote = oracle.try_quote(now, cache_key, &[]).map_err(|_| {
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
        }),
    ))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::galaxy_pricing_oracle::MockProviderQuote;
    use std::collections::HashMap;
    use std::sync::{Mutex as StdMutex, OnceLock as StdOnceLock};

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
                provider_id: "openai_us",
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
}
