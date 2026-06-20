//! Galaxy Grid pricing oracle stub (PH-S68): unit keys, `floor(market_min×0.9)` quote,
//! cache TTL/SWR from `POOLAI_GALAXY_PRICE_*` env; L2 force-fallback ops wire (PH-S81);
//! L1 stale-served metric (PH-S83); L1 fresh-served metric (PH-S91); L1 cache TTL metadata (PH-S89);
//! `POOLAI_GALAXY_PRICING_PROVIDERS` allow-list catalog stub (PH-S92).
//! See `docs/concept/POOLAI_GALAXY_GRID.md` §4.2.
//!
//! Oracle determines **gross quote** in micro-USD; settlement uses `galaxy_fee_split`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tracing::info;

/// 1 USD = 1_000_000 micro-USD (§4.2.1).
pub const USD_MICRO_PER_USD: u64 = 1_000_000;

/// PoolAI discount: −10% → `floor(market_min × 9_000 / 10_000)` (§4.2.2).
pub const POOLAI_DISCOUNT_NUMERATOR: u64 = 9_000;
pub const POOLAI_DISCOUNT_DENOMINATOR: u64 = 10_000;

/// Default fresh cache TTL (`POOLAI_GALAXY_PRICE_CACHE_TTL_SECS`).
pub const DEFAULT_CACHE_TTL_SECS: u64 = 300;

/// Default stale-while-revalidate window (`POOLAI_GALAXY_PRICE_MAX_STALE_SECS`).
pub const DEFAULT_MAX_STALE_SECS: u64 = 3600;

/// Env: fresh cache TTL seconds.
pub const ENV_CACHE_TTL_SECS: &str = "POOLAI_GALAXY_PRICE_CACHE_TTL_SECS";

/// Env: max age for stale-while-revalidate seconds.
pub const ENV_MAX_STALE_SECS: &str = "POOLAI_GALAXY_PRICE_MAX_STALE_SECS";

/// Env: `1` forces L2 fallback path (§4.2.4).
pub const ENV_FORCE_FALLBACK: &str = "POOLAI_GALAXY_PRICING_FORCE_FALLBACK";

/// Structured log event when ops override serves L2 (§4.2.4).
pub const FORCED_FALLBACK_LOG_EVENT: &str = "pricing_forced_fallback";

/// In-process counter name for forced L2 quotes (mirrored on `GET /metrics`, PH-S127).
pub const METRIC_FORCED_FALLBACK_TOTAL: &str = "galaxy_pricing_forced_fallback_total";

static FORCED_FALLBACK_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Structured log event when L1 stale cache is served (§4.2.4).
pub const STALE_SERVED_LOG_EVENT: &str = "pricing_oracle_stale_served";

/// In-process counter for L1 stale cache serves (§4.2.4, PH-S83).
pub const METRIC_STALE_SERVED_TOTAL: &str = "galaxy_pricing_stale_served";

static STALE_SERVED_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Structured log event when L1 fresh cache is served (§4.2.5).
pub const FRESH_SERVED_LOG_EVENT: &str = "pricing_oracle_fresh_served";

/// In-process counter for L1 fresh cache serves (§4.2.5, PH-S91).
pub const METRIC_FRESH_SERVED_TOTAL: &str = "galaxy_pricing_fresh_served";

/// Latest observed L1 cache age in seconds (§4.2.3, PH-S168 `/metrics` gauge).
pub const METRIC_CACHE_AGE_SECONDS: &str = "galaxy_pricing_cache_age_seconds";

/// Last served PoolAI quote in micro-USD (§4.2, PH-S174 `/metrics` gauge).
pub const METRIC_QUOTE_USD_MICRO: &str = "galaxy_pricing_quote_usd_micro";

/// Last observed market min in micro-USD (§4.2, PH-S181 `/metrics` gauge).
pub const METRIC_MARKET_MIN_USD_MICRO: &str = "galaxy_pricing_market_min_usd_micro";

static FRESH_SERVED_TOTAL: AtomicU64 = AtomicU64::new(0);

static L1_CACHE_AGE_OBSERVED_SECS: AtomicU64 = AtomicU64::new(0);
static LAST_QUOTE_USD_MICRO: AtomicU64 = AtomicU64::new(0);
static LAST_MARKET_MIN_USD_MICRO: AtomicU64 = AtomicU64::new(0);

/// Env: JSON map for L2 fallback floor quotes in micro-USD (§4.2.4).
/// Example:
/// `{"inference_blended_token":450000,"gpu_second":12000}`
pub const ENV_FALLBACK_JSON: &str = "POOLAI_GALAXY_PRICING_FALLBACK_JSON";

/// Env: JSON allow-list of US pricing providers + optional endpoints (§4.2.5).
/// Array `[{...}]` or object `{"providers":[{...}]}` — see [`parse_pricing_providers_json`].
pub const ENV_PRICING_PROVIDERS: &str = "POOLAI_GALAXY_PRICING_PROVIDERS";

/// Env: HTTP timeout for live provider fetch (PH-S102), milliseconds.
pub const ENV_PROVIDER_HTTP_TIMEOUT_MS: &str = "POOLAI_GALAXY_PRICING_TIMEOUT_MS";

/// Default timeout for provider HTTP calls (PH-S102).
pub const DEFAULT_PROVIDER_HTTP_TIMEOUT_MS: u64 = 1500;

/// Billing unit keys shared by oracle and scheduling (§4.2.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GalaxyPriceUnitKey {
    InferenceInputToken,
    InferenceOutputToken,
    InferenceBlendedToken,
    GpuSecond,
    JobFlat,
}

impl GalaxyPriceUnitKey {
    pub const ALL: [Self; 5] = [
        Self::InferenceInputToken,
        Self::InferenceOutputToken,
        Self::InferenceBlendedToken,
        Self::GpuSecond,
        Self::JobFlat,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::InferenceInputToken => "inference_input_token",
            Self::InferenceOutputToken => "inference_output_token",
            Self::InferenceBlendedToken => "inference_blended_token",
            Self::GpuSecond => "gpu_second",
            Self::JobFlat => "job_flat",
        }
    }
}

impl FromStr for GalaxyPriceUnitKey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "inference_input_token" => Ok(Self::InferenceInputToken),
            "inference_output_token" => Ok(Self::InferenceOutputToken),
            "inference_blended_token" => Ok(Self::InferenceBlendedToken),
            "gpu_second" => Ok(Self::GpuSecond),
            "job_flat" => Ok(Self::JobFlat),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for GalaxyPriceUnitKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Oracle runtime config from environment (§4.2.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GalaxyPricingConfig {
    pub cache_ttl_secs: u64,
    pub max_stale_secs: u64,
    pub force_fallback: bool,
}

impl Default for GalaxyPricingConfig {
    fn default() -> Self {
        Self {
            cache_ttl_secs: DEFAULT_CACHE_TTL_SECS,
            max_stale_secs: DEFAULT_MAX_STALE_SECS,
            force_fallback: false,
        }
    }
}

impl GalaxyPricingConfig {
    pub fn from_env() -> Self {
        let cache_ttl_secs = env_u64(ENV_CACHE_TTL_SECS).unwrap_or(DEFAULT_CACHE_TTL_SECS);
        let max_stale_secs = env_u64(ENV_MAX_STALE_SECS).unwrap_or(DEFAULT_MAX_STALE_SECS);
        let force_fallback = std::env::var(ENV_FORCE_FALLBACK)
            .ok()
            .is_some_and(|v| matches!(v.trim(), "1" | "true" | "TRUE" | "yes" | "YES"));
        Self {
            cache_ttl_secs,
            max_stale_secs,
            force_fallback,
        }
    }
}

fn env_u64(name: &str) -> Option<u64> {
    std::env::var(name).ok().and_then(|v| v.trim().parse().ok())
}

/// Total forced L2 quotes served since process start (ops/metrics snapshot).
pub fn forced_fallback_total() -> u64 {
    FORCED_FALLBACK_TOTAL.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_forced_fallback_total_for_test() {
    FORCED_FALLBACK_TOTAL.store(0, Ordering::Relaxed);
}

/// Total L1 stale cache quotes served since process start (ops/metrics snapshot).
pub fn stale_served_total() -> u64 {
    STALE_SERVED_TOTAL.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_stale_served_total_for_test() {
    STALE_SERVED_TOTAL.store(0, Ordering::Relaxed);
}

/// Total L1 fresh cache quotes served since process start (ops/metrics snapshot).
pub fn fresh_served_total() -> u64 {
    FRESH_SERVED_TOTAL.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_fresh_served_total_for_test() {
    FRESH_SERVED_TOTAL.store(0, Ordering::Relaxed);
}

#[cfg(any(test, feature = "test-utils"))]
pub fn bump_fresh_served_for_test() {
    record_fresh_served(GalaxyPriceUnitKey::InferenceBlendedToken);
}

#[cfg(any(test, feature = "test-utils"))]
pub fn bump_stale_served_for_test() {
    record_stale_served(GalaxyPriceUnitKey::InferenceBlendedToken);
}

#[cfg(any(test, feature = "test-utils"))]
pub fn bump_forced_fallback_for_test() {
    record_forced_fallback(GalaxyPriceUnitKey::InferenceBlendedToken);
}

fn record_forced_fallback(unit_key: GalaxyPriceUnitKey) {
    let total = FORCED_FALLBACK_TOTAL.fetch_add(1, Ordering::Relaxed) + 1;
    info!(
        event = FORCED_FALLBACK_LOG_EVENT,
        unit_key = %unit_key,
        metric = METRIC_FORCED_FALLBACK_TOTAL,
        total,
        "pricing oracle forced L2 fallback"
    );
}

fn record_stale_served(unit_key: GalaxyPriceUnitKey) {
    let total = STALE_SERVED_TOTAL.fetch_add(1, Ordering::Relaxed) + 1;
    info!(
        event = STALE_SERVED_LOG_EVENT,
        unit_key = %unit_key,
        metric = METRIC_STALE_SERVED_TOTAL,
        total,
        "pricing oracle served L1 stale cache"
    );
}

fn record_fresh_served(unit_key: GalaxyPriceUnitKey) {
    let total = FRESH_SERVED_TOTAL.fetch_add(1, Ordering::Relaxed) + 1;
    info!(
        event = FRESH_SERVED_LOG_EVENT,
        unit_key = %unit_key,
        metric = METRIC_FRESH_SERVED_TOTAL,
        total,
        "pricing oracle served L1 fresh cache"
    );
}

/// Record L1 stale metric when serving from cache (oracle or HTTP snapshot path).
pub fn record_l1_stale_served(unit_key: GalaxyPriceUnitKey) {
    record_stale_served(unit_key);
}

/// Record L1 fresh metric when serving from cache (oracle or HTTP snapshot path).
pub fn record_l1_fresh_served(unit_key: GalaxyPriceUnitKey) {
    record_fresh_served(unit_key);
}

/// Observe L1 cache age for Prometheus gauge (PH-S168).
pub fn observe_l1_cache_age_secs(age_secs: u64) {
    L1_CACHE_AGE_OBSERVED_SECS.store(age_secs, Ordering::Relaxed);
}

/// Latest observed L1 cache age since process start (mirrored on `GET /metrics`).
pub fn pricing_cache_age_seconds() -> u64 {
    L1_CACHE_AGE_OBSERVED_SECS.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_pricing_cache_age_for_test() {
    L1_CACHE_AGE_OBSERVED_SECS.store(0, Ordering::Relaxed);
}

/// Observe last served quote for Prometheus gauge (PH-S174).
pub fn observe_last_quote_usd_micro(usd_micro: u64) {
    LAST_QUOTE_USD_MICRO.store(usd_micro, Ordering::Relaxed);
}

/// Last served PoolAI quote in micro-USD since process start (mirrored on `GET /metrics`).
pub fn last_quote_usd_micro() -> u64 {
    LAST_QUOTE_USD_MICRO.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_last_quote_usd_micro_for_test() {
    LAST_QUOTE_USD_MICRO.store(0, Ordering::Relaxed);
}

/// Observe last served market min for Prometheus gauge (PH-S181).
pub fn observe_last_market_min_usd_micro(usd_micro: u64) {
    LAST_MARKET_MIN_USD_MICRO.store(usd_micro, Ordering::Relaxed);
}

/// Last observed market min in micro-USD since process start (mirrored on `GET /metrics`).
pub fn last_market_min_usd_micro() -> u64 {
    LAST_MARKET_MIN_USD_MICRO.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_last_market_min_usd_micro_for_test() {
    LAST_MARKET_MIN_USD_MICRO.store(0, Ordering::Relaxed);
}

fn observe_served_quote_metrics(quote: &GalaxyPricingQuote) {
    observe_last_quote_usd_micro(quote.poolai_quote_usd_micro);
    observe_last_market_min_usd_micro(quote.market_min_usd_micro);
}

fn serve_l1_cache_quote(
    entry: GalaxyPricingCacheEntry,
    freshness: CacheFreshness,
) -> GalaxyPricingQuote {
    match freshness {
        CacheFreshness::Fresh => record_fresh_served(entry.quote.unit_key),
        CacheFreshness::Stale => record_stale_served(entry.quote.unit_key),
        CacheFreshness::Expired => {}
    }
    entry.quote
}

/// US provider row from env catalog or bundled allow-list (§4.2.1, PH-S92).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GalaxyPricingProviderEntry {
    pub provider_id: String,
    #[serde(default = "default_provider_region")]
    pub region: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_profile: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub task_profiles: Vec<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub units: HashMap<String, u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default = "default_provider_enabled")]
    pub enabled: bool,
}

fn default_provider_region() -> String {
    "us".to_string()
}

fn default_provider_enabled() -> bool {
    true
}

/// Parsed provider catalog (`POOLAI_GALAXY_PRICING_PROVIDERS` or bundled default).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct GalaxyPricingProviderCatalog {
    #[serde(default)]
    pub providers: Vec<GalaxyPricingProviderEntry>,
}

impl GalaxyPricingProviderCatalog {
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    /// Enabled US-region rows matching task/model filters (empty filter = wildcard).
    pub fn matching_entries(
        &self,
        task_profile: &str,
        model_profile: &str,
    ) -> Vec<&GalaxyPricingProviderEntry> {
        let matched: Vec<_> = self
            .providers
            .iter()
            .filter(|p| {
                p.enabled
                    && p.region.eq_ignore_ascii_case("us")
                    && provider_matches_profiles(p, task_profile, model_profile)
            })
            .collect();
        crate::grid::galaxy_pricing_provider_metrics::record_provider_catalog_lookup(matched.len());
        matched
    }
}

fn provider_matches_profiles(
    entry: &GalaxyPricingProviderEntry,
    task_profile: &str,
    model_profile: &str,
) -> bool {
    let task_ok =
        entry.task_profiles.is_empty() || entry.task_profiles.iter().any(|t| t == task_profile);
    let model_ok = entry
        .model_profile
        .as_deref()
        .is_none_or(|m| m.is_empty() || m == model_profile);
    task_ok && model_ok
}

fn normalize_provider_units(units: HashMap<String, u64>) -> HashMap<String, u64> {
    let mut out = HashMap::new();
    for (k, v) in units {
        if GalaxyPriceUnitKey::from_str(&k).is_ok() && v > 0 {
            out.insert(k, v);
        }
    }
    out
}

fn sanitize_provider_entry(
    mut entry: GalaxyPricingProviderEntry,
) -> Option<GalaxyPricingProviderEntry> {
    if !entry.enabled || entry.provider_id.trim().is_empty() {
        return None;
    }
    entry.provider_id = entry.provider_id.trim().to_string();
    entry.region = entry.region.trim().to_string();
    if entry.region.is_empty() {
        entry.region = default_provider_region();
    }
    entry.units = normalize_provider_units(entry.units);
    if entry.units.is_empty() {
        return None;
    }
    if let Some(endpoint) = entry.endpoint.as_mut() {
        let trimmed = endpoint.trim();
        if trimmed.is_empty() {
            entry.endpoint = None;
        } else {
            *endpoint = trimmed.to_string();
        }
    }
    Some(entry)
}

/// Parse `POOLAI_GALAXY_PRICING_PROVIDERS` JSON (array or `{"providers":[...]}`).
pub fn parse_pricing_providers_json(raw: &str) -> GalaxyPricingProviderCatalog {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return GalaxyPricingProviderCatalog::default();
    }
    if let Ok(rows) = serde_json::from_str::<Vec<GalaxyPricingProviderEntry>>(trimmed) {
        return catalog_from_rows(rows);
    }
    if let Ok(wrapped) = serde_json::from_str::<GalaxyPricingProviderCatalog>(trimmed) {
        return catalog_from_rows(wrapped.providers);
    }
    GalaxyPricingProviderCatalog::default()
}

fn catalog_from_rows(rows: Vec<GalaxyPricingProviderEntry>) -> GalaxyPricingProviderCatalog {
    let providers = rows
        .into_iter()
        .filter_map(sanitize_provider_entry)
        .collect();
    GalaxyPricingProviderCatalog { providers }
}

/// Bundled US allow-list used when env catalog is unset (§4.2.5).
pub fn bundled_pricing_provider_catalog() -> GalaxyPricingProviderCatalog {
    catalog_from_rows(vec![
        GalaxyPricingProviderEntry {
            provider_id: "openai_us".into(),
            region: "us".into(),
            model_profile: Some("gpt-4o-mini".into()),
            task_profiles: vec!["inference:text".into()],
            units: HashMap::from([(
                GalaxyPriceUnitKey::InferenceBlendedToken
                    .as_str()
                    .to_string(),
                500_000,
            )]),
            endpoint: None,
            enabled: true,
        },
        GalaxyPricingProviderEntry {
            provider_id: "anthropic_us".into(),
            region: "us".into(),
            model_profile: Some("gpt-4o-mini".into()),
            task_profiles: vec!["inference:text".into()],
            units: HashMap::from([(
                GalaxyPriceUnitKey::InferenceBlendedToken
                    .as_str()
                    .to_string(),
                600_000,
            )]),
            endpoint: None,
            enabled: true,
        },
    ])
}

/// Env catalog when set and non-empty; otherwise [`bundled_pricing_provider_catalog`].
pub fn pricing_provider_catalog_from_env() -> GalaxyPricingProviderCatalog {
    match std::env::var(ENV_PRICING_PROVIDERS) {
        Ok(raw) if !raw.trim().is_empty() => {
            let parsed = parse_pricing_providers_json(&raw);
            if parsed.is_empty() {
                bundled_pricing_provider_catalog()
            } else {
                parsed
            }
        }
        _ => bundled_pricing_provider_catalog(),
    }
}

/// HTTP timeout for provider fetch path (PH-S102), milliseconds.
pub fn provider_http_timeout_ms_from_env() -> u64 {
    env_u64(ENV_PROVIDER_HTTP_TIMEOUT_MS).unwrap_or(DEFAULT_PROVIDER_HTTP_TIMEOUT_MS)
}

#[derive(Debug, Deserialize)]
struct ProviderEndpointWrappedUnits {
    #[serde(default)]
    units: HashMap<String, u64>,
}

fn parse_live_units_map(raw: &str) -> Option<HashMap<String, u64>> {
    if let Ok(wrapped) = serde_json::from_str::<ProviderEndpointWrappedUnits>(raw) {
        if !wrapped.units.is_empty() {
            return Some(wrapped.units);
        }
    }
    serde_json::from_str::<HashMap<String, u64>>(raw).ok()
}

fn parse_live_units_map_bytes(raw: &[u8]) -> Option<HashMap<String, u64>> {
    if let Ok(wrapped) = serde_json::from_slice::<ProviderEndpointWrappedUnits>(raw) {
        if !wrapped.units.is_empty() {
            return Some(wrapped.units);
        }
    }
    serde_json::from_slice::<HashMap<String, u64>>(raw).ok()
}

/// PH-S102: live provider HTTP fetch for a single unit key.
pub async fn fetch_live_provider_quotes(
    catalog: &GalaxyPricingProviderCatalog,
    task_profile: &str,
    model_profile: &str,
    unit_key: GalaxyPriceUnitKey,
    timeout_ms: u64,
) -> Vec<MockProviderQuote> {
    let Ok(client) = reqwest::Client::builder()
        .timeout(Duration::from_millis(timeout_ms))
        .build()
    else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for entry in catalog.matching_entries(task_profile, model_profile) {
        let Some(endpoint) = entry.endpoint.as_deref() else {
            continue;
        };
        let url = format!(
            "{endpoint}?task_profile={task_profile}&model_profile={model_profile}&unit_key={}",
            unit_key.as_str()
        );
        let Ok(resp) = client.get(url).send().await else {
            crate::grid::galaxy_pricing_provider_metrics::record_provider_fetch_error();
            continue;
        };
        if !resp.status().is_success() {
            crate::grid::galaxy_pricing_provider_metrics::record_provider_fetch_error();
            continue;
        }
        let body = match resp.bytes().await {
            Ok(bytes) => bytes,
            Err(_) => {
                crate::grid::galaxy_pricing_provider_metrics::record_provider_fetch_error();
                continue;
            }
        };
        let Some(units) = parse_live_units_map_bytes(body.as_ref()) else {
            crate::grid::galaxy_pricing_provider_metrics::record_provider_fetch_error();
            continue;
        };
        let Some(usd_micro) = units.get(unit_key.as_str()).copied() else {
            crate::grid::galaxy_pricing_provider_metrics::record_provider_fetch_error();
            continue;
        };
        if usd_micro == 0 {
            crate::grid::galaxy_pricing_provider_metrics::record_provider_fetch_error();
            continue;
        }
        out.push(MockProviderQuote {
            provider_id: entry.provider_id.clone(),
            unit_key,
            usd_micro,
            healthy: true,
        });
    }
    out
}

fn parse_fallback_json(raw: &str) -> HashMap<GalaxyPriceUnitKey, u64> {
    let parsed = serde_json::from_str::<HashMap<String, u64>>(raw);
    let mut out = HashMap::new();
    if let Ok(map) = parsed {
        for (k, v) in map {
            if let Ok(unit_key) = GalaxyPriceUnitKey::from_str(&k) {
                if v > 0 {
                    out.insert(unit_key, v);
                }
            }
        }
    }
    out
}

/// `floor(market_min_usd_micro × 0.9)` — integer safe (§4.2.2).
#[inline]
pub fn floor_poolai_quote_usd_micro(market_min_usd_micro: u64) -> u64 {
    let numer = u128::from(market_min_usd_micro) * u128::from(POOLAI_DISCOUNT_NUMERATOR);
    (numer / u128::from(POOLAI_DISCOUNT_DENOMINATOR)) as u64
}

/// Mock provider row for tests and future fetchers (§4.2.1 example JSON).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MockProviderQuote {
    pub provider_id: String,
    pub unit_key: GalaxyPriceUnitKey,
    pub usd_micro: u64,
    pub healthy: bool,
}

/// Minimum normalized unit price among healthy providers (§4.2.2).
pub fn market_min_usd_micro(
    providers: &[MockProviderQuote],
    unit_key: GalaxyPriceUnitKey,
) -> Option<(u64, String)> {
    providers
        .iter()
        .filter(|p| p.healthy && p.unit_key == unit_key && p.usd_micro > 0)
        .min_by_key(|p| p.usd_micro)
        .map(|p| (p.usd_micro, p.provider_id.clone()))
}

/// Published quote snapshot (cache value + lookup result).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GalaxyPricingQuote {
    pub task_profile: String,
    pub model_profile: String,
    pub unit_key: GalaxyPriceUnitKey,
    pub market_min_usd_micro: u64,
    pub poolai_quote_usd_micro: u64,
    pub provider_id_at_min: String,
    pub cached_at_secs: u64,
}

/// Cache key `(task_profile, model_profile, unit_key)` (§4.2.3).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GalaxyPricingCacheKey {
    pub task_profile: String,
    pub model_profile: String,
    pub unit_key: GalaxyPriceUnitKey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GalaxyPricingCacheEntry {
    pub quote: GalaxyPricingQuote,
}

/// L3 hard stop: no L1 cache (fresh/stale) and no L2 configured fallback (§4.2.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GalaxyPricingUnavailable;

/// Stable REST `error.code` for L3 responses (Galaxy §4.2.4).
pub const PRICING_UNAVAILABLE_ERROR_CODE: &str = "pricing_unavailable";

/// Cache age classification for TTL / SWR (§4.2.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheFreshness {
    Fresh,
    Stale,
    Expired,
}

/// L1 cache TTL metadata for API/ops (§4.2.3, PH-S89).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GalaxyPricingCacheMetadata {
    /// `now_secs - cached_at_secs`
    pub cache_age_secs: u64,
    pub cache_ttl_secs: u64,
    pub max_stale_secs: u64,
    /// `cached_at_secs + cache_ttl_secs`
    pub cache_fresh_until_secs: u64,
    /// `cached_at_secs + max_stale_secs` (SWR upper bound)
    pub cache_stale_until_secs: u64,
}

/// Build TTL metadata for an L1 cache hit (pairs with [`cache_freshness`]).
pub fn cache_metadata(
    now_secs: u64,
    cached_at_secs: u64,
    config: &GalaxyPricingConfig,
) -> GalaxyPricingCacheMetadata {
    let cache_age_secs = now_secs.saturating_sub(cached_at_secs);
    GalaxyPricingCacheMetadata {
        cache_age_secs,
        cache_ttl_secs: config.cache_ttl_secs,
        max_stale_secs: config.max_stale_secs,
        cache_fresh_until_secs: cached_at_secs.saturating_add(config.cache_ttl_secs),
        cache_stale_until_secs: cached_at_secs.saturating_add(config.max_stale_secs),
    }
}

/// Classify cache entry age (pure; inject `now_secs` in tests).
pub fn cache_freshness(
    now_secs: u64,
    cached_at_secs: u64,
    cache_ttl_secs: u64,
    max_stale_secs: u64,
) -> CacheFreshness {
    let age = now_secs.saturating_sub(cached_at_secs);
    if age <= cache_ttl_secs {
        CacheFreshness::Fresh
    } else if age <= max_stale_secs {
        CacheFreshness::Stale
    } else {
        CacheFreshness::Expired
    }
}

/// In-process pricing oracle stub with TTL cache (no HTTP wire yet).
#[derive(Debug, Default)]
pub struct GalaxyPricingOracle {
    config: GalaxyPricingConfig,
    cache: HashMap<GalaxyPricingCacheKey, GalaxyPricingCacheEntry>,
    fallback_quotes_usd_micro: HashMap<GalaxyPriceUnitKey, u64>,
    provider_catalog: GalaxyPricingProviderCatalog,
}

impl GalaxyPricingOracle {
    pub fn new(config: GalaxyPricingConfig) -> Self {
        Self {
            config,
            cache: HashMap::new(),
            fallback_quotes_usd_micro: HashMap::new(),
            provider_catalog: GalaxyPricingProviderCatalog::default(),
        }
    }

    pub fn from_env() -> Self {
        let mut oracle = Self::new(GalaxyPricingConfig::from_env());
        oracle.provider_catalog = pricing_provider_catalog_from_env();
        if let Ok(raw) = std::env::var(ENV_FALLBACK_JSON) {
            oracle.fallback_quotes_usd_micro = parse_fallback_json(&raw);
        }
        oracle
    }

    pub fn provider_catalog(&self) -> &GalaxyPricingProviderCatalog {
        &self.provider_catalog
    }

    pub fn with_provider_catalog(mut self, provider_catalog: GalaxyPricingProviderCatalog) -> Self {
        self.provider_catalog = provider_catalog;
        self
    }

    pub fn with_l2_fallback_quotes(
        mut self,
        fallback_quotes_usd_micro: HashMap<GalaxyPriceUnitKey, u64>,
    ) -> Self {
        self.fallback_quotes_usd_micro = fallback_quotes_usd_micro;
        self
    }

    pub fn config(&self) -> &GalaxyPricingConfig {
        &self.config
    }

    pub fn lookup(
        &self,
        now_secs: u64,
        key: &GalaxyPricingCacheKey,
    ) -> Option<(GalaxyPricingCacheEntry, CacheFreshness)> {
        let entry = self.cache.get(key)?;
        let freshness = cache_freshness(
            now_secs,
            entry.quote.cached_at_secs,
            self.config.cache_ttl_secs,
            self.config.max_stale_secs,
        );
        if freshness == CacheFreshness::Expired {
            return None;
        }
        Some((entry.clone(), freshness))
    }

    /// Refresh from provider quotes and store in cache; returns computed quote.
    pub fn refresh_from_providers(
        &mut self,
        now_secs: u64,
        key: GalaxyPricingCacheKey,
        providers: &[MockProviderQuote],
    ) -> Option<GalaxyPricingQuote> {
        if self.config.force_fallback {
            return None;
        }
        let (market_min, provider_id) = market_min_usd_micro(providers, key.unit_key)?;
        let poolai_quote = floor_poolai_quote_usd_micro(market_min);
        let quote = GalaxyPricingQuote {
            task_profile: key.task_profile.clone(),
            model_profile: key.model_profile.clone(),
            unit_key: key.unit_key,
            market_min_usd_micro: market_min,
            poolai_quote_usd_micro: poolai_quote,
            provider_id_at_min: provider_id.to_string(),
            cached_at_secs: now_secs,
        };
        self.cache.insert(
            key,
            GalaxyPricingCacheEntry {
                quote: quote.clone(),
            },
        );
        Some(quote)
    }

    /// L2 fallback fixed quote from config when providers are unavailable (§4.2.4).
    fn l2_fallback_quote(
        &mut self,
        now_secs: u64,
        key: GalaxyPricingCacheKey,
    ) -> Option<GalaxyPricingQuote> {
        let poolai_quote = *self.fallback_quotes_usd_micro.get(&key.unit_key)?;
        let quote = GalaxyPricingQuote {
            task_profile: key.task_profile.clone(),
            model_profile: key.model_profile.clone(),
            unit_key: key.unit_key,
            market_min_usd_micro: poolai_quote,
            poolai_quote_usd_micro: poolai_quote,
            provider_id_at_min: "fallback_l2_config".to_string(),
            cached_at_secs: now_secs,
        };
        self.cache.insert(
            key,
            GalaxyPricingCacheEntry {
                quote: quote.clone(),
            },
        );
        Some(quote)
    }

    /// Lookup fresh/stale cache, provider refresh, or L2 fallback (§4.2.3–4.2.4).
    pub fn try_quote(
        &mut self,
        now_secs: u64,
        key: GalaxyPricingCacheKey,
        providers: &[MockProviderQuote],
    ) -> Result<GalaxyPricingQuote, GalaxyPricingUnavailable> {
        if self.config.force_fallback {
            let quote = self
                .l2_fallback_quote(now_secs, key)
                .ok_or(GalaxyPricingUnavailable)?;
            record_forced_fallback(quote.unit_key);
            observe_served_quote_metrics(&quote);
            return Ok(quote);
        }
        if let Some((entry, freshness)) = self.lookup(now_secs, &key) {
            if freshness == CacheFreshness::Fresh || freshness == CacheFreshness::Stale {
                observe_l1_cache_age_secs(now_secs.saturating_sub(entry.quote.cached_at_secs));
                let quote = serve_l1_cache_quote(entry, freshness);
                observe_served_quote_metrics(&quote);
                return Ok(quote);
            }
        }
        let quote = self
            .refresh_from_providers(now_secs, key.clone(), providers)
            .or_else(|| self.l2_fallback_quote(now_secs, key))
            .ok_or(GalaxyPricingUnavailable)?;
        observe_served_quote_metrics(&quote);
        Ok(quote)
    }

    /// Convenience wrapper over [`Self::try_quote`].
    pub fn quote(
        &mut self,
        now_secs: u64,
        key: GalaxyPricingCacheKey,
        providers: &[MockProviderQuote],
    ) -> Option<GalaxyPricingQuote> {
        self.try_quote(now_secs, key, providers).ok()
    }

    #[cfg(test)]
    pub fn set_force_fallback_for_test(&mut self, force_fallback: bool) {
        self.config.force_fallback = force_fallback;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex as StdMutex, OnceLock as StdOnceLock};

    static METRIC_TEST_LOCK: StdOnceLock<StdMutex<()>> = StdOnceLock::new();

    fn metric_test_lock() -> std::sync::MutexGuard<'static, ()> {
        METRIC_TEST_LOCK
            .get_or_init(|| StdMutex::new(()))
            .lock()
            .unwrap_or_else(|e| e.into_inner())
    }

    fn mock_us_blended() -> Vec<MockProviderQuote> {
        vec![
            MockProviderQuote {
                provider_id: "openai_us".into(),
                unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
                usd_micro: 500_000,
                healthy: true,
            },
            MockProviderQuote {
                provider_id: "anthropic_us".into(),
                unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
                usd_micro: 600_000,
                healthy: true,
            },
            MockProviderQuote {
                provider_id: "stale_vendor".into(),
                unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
                usd_micro: 100_000,
                healthy: false,
            },
        ]
    }

    #[test]
    fn floor_nine_tenths_example_from_concept() {
        assert_eq!(floor_poolai_quote_usd_micro(500_000), 450_000);
    }

    #[test]
    fn floor_never_rounds_up() {
        assert_eq!(floor_poolai_quote_usd_micro(1), 0);
        assert_eq!(floor_poolai_quote_usd_micro(11), 9);
    }

    #[test]
    fn market_min_ignores_unhealthy_and_other_units() {
        let providers = mock_us_blended();
        let (min, id) =
            market_min_usd_micro(&providers, GalaxyPriceUnitKey::InferenceBlendedToken).unwrap();
        assert_eq!(min, 500_000);
        assert_eq!(id, "openai_us");
        assert!(market_min_usd_micro(&providers, GalaxyPriceUnitKey::GpuSecond).is_none());
    }

    #[test]
    fn cache_metadata_fresh_vs_stale_windows() {
        let config = GalaxyPricingConfig {
            cache_ttl_secs: 300,
            max_stale_secs: 3600,
            force_fallback: false,
        };
        let cached_at = 1_000u64;
        let fresh_meta = cache_metadata(cached_at + 100, cached_at, &config);
        assert_eq!(fresh_meta.cache_age_secs, 100);
        assert_eq!(fresh_meta.cache_fresh_until_secs, 1_300);
        assert_eq!(fresh_meta.cache_stale_until_secs, 4_600);
        assert_eq!(
            cache_freshness(cached_at + 100, cached_at, 300, 3600),
            CacheFreshness::Fresh
        );

        let stale_meta = cache_metadata(cached_at + 500, cached_at, &config);
        assert_eq!(stale_meta.cache_age_secs, 500);
        assert_eq!(
            cache_freshness(cached_at + 500, cached_at, 300, 3600),
            CacheFreshness::Stale
        );
    }

    #[test]
    fn cache_fresh_then_stale_then_expired() {
        let cached_at = 1_000u64;
        let ttl = 300;
        let max_stale = 3600;
        assert_eq!(
            cache_freshness(cached_at + 100, cached_at, ttl, max_stale),
            CacheFreshness::Fresh
        );
        assert_eq!(
            cache_freshness(cached_at + 301, cached_at, ttl, max_stale),
            CacheFreshness::Stale
        );
        assert_eq!(
            cache_freshness(cached_at + 3601, cached_at, ttl, max_stale),
            CacheFreshness::Expired
        );
    }

    #[test]
    fn oracle_cache_hit_without_refresh() {
        let mut oracle = GalaxyPricingOracle::new(GalaxyPricingConfig {
            cache_ttl_secs: 300,
            max_stale_secs: 3600,
            force_fallback: false,
        });
        let key = GalaxyPricingCacheKey {
            task_profile: "inference:text".into(),
            model_profile: "gpt-4o-mini".into(),
            unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
        };
        let providers = mock_us_blended();
        let q1 = oracle
            .refresh_from_providers(10_000, key.clone(), &providers)
            .unwrap();
        assert_eq!(q1.poolai_quote_usd_micro, 450_000);

        let expensive = vec![MockProviderQuote {
            provider_id: "cheap_now".into(),
            unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
            usd_micro: 50_000,
            healthy: true,
        }];
        let q2 = oracle.quote(10_100, key, &expensive).unwrap();
        assert_eq!(q2.poolai_quote_usd_micro, 450_000, "fresh cache hit");
    }

    #[test]
    fn oracle_stale_swr_then_refresh_after_expiry() {
        let mut oracle = GalaxyPricingOracle::new(GalaxyPricingConfig {
            cache_ttl_secs: 300,
            max_stale_secs: 3600,
            force_fallback: false,
        });
        let key = GalaxyPricingCacheKey {
            task_profile: "inference:text".into(),
            model_profile: "default".into(),
            unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
        };
        let providers = mock_us_blended();
        oracle.refresh_from_providers(0, key.clone(), &providers);

        let (_, stale) = oracle.lookup(400, &key).unwrap();
        assert_eq!(stale, CacheFreshness::Stale);

        let cheap = vec![MockProviderQuote {
            provider_id: "x".into(),
            unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
            usd_micro: 100_000,
            healthy: true,
        }];
        let still_old = oracle.quote(400, key.clone(), &cheap).unwrap();
        assert_eq!(still_old.market_min_usd_micro, 500_000);

        assert!(oracle.lookup(4000, &key).is_none());
        let refreshed = oracle.quote(4000, key, &cheap).unwrap();
        assert_eq!(refreshed.market_min_usd_micro, 100_000);
        assert_eq!(refreshed.poolai_quote_usd_micro, 90_000);
    }

    #[test]
    fn unit_key_roundtrip_str() {
        for key in GalaxyPriceUnitKey::ALL {
            let s = key.to_string();
            assert_eq!(GalaxyPriceUnitKey::from_str(&s).ok(), Some(key));
        }
    }

    #[test]
    fn config_defaults_match_concept() {
        let cfg = GalaxyPricingConfig::default();
        assert_eq!(cfg.cache_ttl_secs, 300);
        assert_eq!(cfg.max_stale_secs, 3600);
        assert!(!cfg.force_fallback);
    }

    #[test]
    fn parse_fallback_json_ignores_invalid_keys_and_zero_values() {
        let map = parse_fallback_json(
            r#"{"inference_blended_token":450000,"gpu_second":0,"unknown_key":777}"#,
        );
        assert_eq!(
            map.get(&GalaxyPriceUnitKey::InferenceBlendedToken).copied(),
            Some(450_000)
        );
        assert_eq!(map.get(&GalaxyPriceUnitKey::GpuSecond).copied(), None);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn try_quote_l3_when_no_l1_l2_or_providers() {
        let oracle = GalaxyPricingOracle::new(GalaxyPricingConfig::default());
        let key = GalaxyPricingCacheKey {
            task_profile: "inference:text".into(),
            model_profile: "no-fallback".into(),
            unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
        };
        let mut oracle = oracle;
        assert_eq!(
            oracle.try_quote(100, key, &[]),
            Err(GalaxyPricingUnavailable)
        );
    }

    #[test]
    fn try_quote_l3_when_force_fallback_without_l2_config() {
        let mut oracle = GalaxyPricingOracle::new(GalaxyPricingConfig {
            cache_ttl_secs: 300,
            max_stale_secs: 3600,
            force_fallback: true,
        });
        let key = GalaxyPricingCacheKey {
            task_profile: "inference:text".into(),
            model_profile: "forced-no-l2".into(),
            unit_key: GalaxyPriceUnitKey::GpuSecond,
        };
        assert_eq!(oracle.try_quote(1, key, &[]), Err(GalaxyPricingUnavailable));
    }

    #[test]
    fn try_quote_l1_stale_before_l3() {
        let mut oracle = GalaxyPricingOracle::new(GalaxyPricingConfig {
            cache_ttl_secs: 300,
            max_stale_secs: 3600,
            force_fallback: false,
        });
        let key = GalaxyPricingCacheKey {
            task_profile: "inference:text".into(),
            model_profile: "stale-ok".into(),
            unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
        };
        oracle.refresh_from_providers(0, key.clone(), &mock_us_blended());
        let quote = oracle.try_quote(400, key, &[]).expect("L1 stale");
        assert_eq!(quote.poolai_quote_usd_micro, 450_000);
    }

    #[test]
    fn try_quote_stale_served_increments_metric_not_fresh() {
        let _lock = metric_test_lock();
        reset_stale_served_total_for_test();
        reset_fresh_served_total_for_test();
        let mut oracle = GalaxyPricingOracle::new(GalaxyPricingConfig {
            cache_ttl_secs: 300,
            max_stale_secs: 3600,
            force_fallback: false,
        });
        let key = GalaxyPricingCacheKey {
            task_profile: "inference:text".into(),
            model_profile: "stale-metric".into(),
            unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
        };
        oracle.refresh_from_providers(0, key.clone(), &mock_us_blended());
        assert_eq!(stale_served_total(), 0);
        assert_eq!(fresh_served_total(), 0);

        oracle
            .try_quote(400, key.clone(), &[])
            .expect("L1 stale serve");
        assert_eq!(stale_served_total(), 1);
        assert_eq!(
            fresh_served_total(),
            0,
            "stale must not increment fresh metric"
        );

        oracle.try_quote(100, key, &[]).expect("L1 fresh serve");
        assert_eq!(
            stale_served_total(),
            1,
            "fresh cache must not increment stale metric"
        );
        assert_eq!(fresh_served_total(), 1);
    }

    #[test]
    fn try_quote_observes_l1_cache_age_ph_s168() {
        reset_pricing_cache_age_for_test();
        let mut oracle = GalaxyPricingOracle::new(GalaxyPricingConfig {
            cache_ttl_secs: 300,
            max_stale_secs: 3600,
            force_fallback: false,
        });
        let key = GalaxyPricingCacheKey {
            task_profile: "inference:text".into(),
            model_profile: "cache-age".into(),
            unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
        };
        oracle.refresh_from_providers(1_000, key.clone(), &mock_us_blended());
        oracle.try_quote(1_600, key, &[]).expect("stale L1");
        assert_eq!(pricing_cache_age_seconds(), 600);
        reset_pricing_cache_age_for_test();
    }

    #[test]
    fn try_quote_observes_last_quote_usd_micro_ph_s174() {
        reset_last_quote_usd_micro_for_test();
        let mut oracle = GalaxyPricingOracle::new(GalaxyPricingConfig {
            cache_ttl_secs: 300,
            max_stale_secs: 3600,
            force_fallback: false,
        });
        let key = GalaxyPricingCacheKey {
            task_profile: "inference:text".into(),
            model_profile: "quote-metric".into(),
            unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
        };
        assert_eq!(last_quote_usd_micro(), 0);
        oracle
            .try_quote(10_000, key, &mock_us_blended())
            .expect("provider refresh quote");
        assert_eq!(last_quote_usd_micro(), 450_000);
        reset_last_quote_usd_micro_for_test();
    }

    #[test]
    fn try_quote_observes_last_market_min_usd_micro_ph_s181() {
        reset_last_market_min_usd_micro_for_test();
        let mut oracle = GalaxyPricingOracle::new(GalaxyPricingConfig {
            cache_ttl_secs: 300,
            max_stale_secs: 3600,
            force_fallback: false,
        });
        let key = GalaxyPricingCacheKey {
            task_profile: "inference:text".into(),
            model_profile: "market-min-metric".into(),
            unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
        };
        assert_eq!(last_market_min_usd_micro(), 0);
        oracle
            .try_quote(10_000, key, &mock_us_blended())
            .expect("provider refresh quote");
        assert_eq!(last_market_min_usd_micro(), 500_000);
        reset_last_market_min_usd_micro_for_test();
    }

    #[test]
    fn try_quote_fresh_served_increments_metric_not_stale() {
        let _lock = metric_test_lock();
        reset_stale_served_total_for_test();
        reset_fresh_served_total_for_test();
        let mut oracle = GalaxyPricingOracle::new(GalaxyPricingConfig {
            cache_ttl_secs: 300,
            max_stale_secs: 3600,
            force_fallback: false,
        });
        let key = GalaxyPricingCacheKey {
            task_profile: "inference:text".into(),
            model_profile: "fresh-metric".into(),
            unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
        };
        oracle.refresh_from_providers(10_000, key.clone(), &mock_us_blended());
        assert_eq!(fresh_served_total(), 0);

        oracle
            .try_quote(10_100, key.clone(), &[])
            .expect("L1 fresh serve");
        assert_eq!(fresh_served_total(), 1);
        assert_eq!(
            stale_served_total(),
            0,
            "fresh must not increment stale metric"
        );

        oracle.try_quote(10_400, key, &[]).expect("L1 stale serve");
        assert_eq!(
            fresh_served_total(),
            1,
            "stale must not increment fresh metric"
        );
        assert_eq!(stale_served_total(), 1);
    }

    #[test]
    fn config_from_env_reads_force_fallback_flag() {
        const KEY: &str = ENV_FORCE_FALLBACK;
        std::env::set_var(KEY, "1");
        let cfg = GalaxyPricingConfig::from_env();
        std::env::remove_var(KEY);
        assert!(cfg.force_fallback);
    }

    #[test]
    fn provider_timeout_from_env_uses_override_or_default() {
        std::env::remove_var(ENV_PROVIDER_HTTP_TIMEOUT_MS);
        assert_eq!(
            provider_http_timeout_ms_from_env(),
            DEFAULT_PROVIDER_HTTP_TIMEOUT_MS
        );
        std::env::set_var(ENV_PROVIDER_HTTP_TIMEOUT_MS, "2500");
        assert_eq!(provider_http_timeout_ms_from_env(), 2500);
        std::env::remove_var(ENV_PROVIDER_HTTP_TIMEOUT_MS);
    }

    #[test]
    fn try_quote_force_fallback_skips_l1_cache() {
        reset_forced_fallback_total_for_test();
        let key = GalaxyPricingCacheKey {
            task_profile: "inference:text".into(),
            model_profile: "l1-should-be-skipped".into(),
            unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
        };

        let mut fallback = HashMap::new();
        fallback.insert(GalaxyPriceUnitKey::InferenceBlendedToken, 470_000);
        let mut oracle = GalaxyPricingOracle::new(GalaxyPricingConfig {
            cache_ttl_secs: 300,
            max_stale_secs: 3600,
            force_fallback: false,
        })
        .with_l2_fallback_quotes(fallback);
        oracle
            .refresh_from_providers(0, key.clone(), &mock_us_blended())
            .expect("seed L1 cache");
        oracle.set_force_fallback_for_test(true);

        let quote = oracle
            .try_quote(100, key, &mock_us_blended())
            .expect("forced L2");
        assert_eq!(quote.poolai_quote_usd_micro, 470_000);
        assert_eq!(quote.provider_id_at_min, "fallback_l2_config");
    }

    #[test]
    fn quote_uses_l2_fallback_when_provider_refresh_unavailable() {
        reset_forced_fallback_total_for_test();
        let mut fallback = HashMap::new();
        fallback.insert(GalaxyPriceUnitKey::InferenceBlendedToken, 470_000);
        let mut oracle = GalaxyPricingOracle::new(GalaxyPricingConfig {
            cache_ttl_secs: 300,
            max_stale_secs: 3600,
            force_fallback: true,
        })
        .with_l2_fallback_quotes(fallback);

        let key = GalaxyPricingCacheKey {
            task_profile: "inference:text".into(),
            model_profile: "fallback-profile".into(),
            unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
        };

        let quote = oracle.quote(1234, key.clone(), &[]).expect("l2 fallback");
        assert_eq!(quote.poolai_quote_usd_micro, 470_000);
        assert_eq!(quote.provider_id_at_min, "fallback_l2_config");

        let (cached, freshness) = oracle.lookup(1235, &key).expect("cached");
        assert_eq!(freshness, CacheFreshness::Fresh);
        assert_eq!(cached.quote.poolai_quote_usd_micro, 470_000);
    }

    #[test]
    fn parse_pricing_providers_json_accepts_array_and_wrapped_object() {
        let array = parse_pricing_providers_json(
            r#"[
              {"provider_id":"openai_us","region":"us","units":{"inference_blended_token":500000}},
              {"provider_id":"disabled","enabled":false,"units":{"inference_blended_token":1}}
            ]"#,
        );
        assert_eq!(array.providers.len(), 1);
        assert_eq!(array.providers[0].provider_id, "openai_us");

        let wrapped = parse_pricing_providers_json(
            r#"{"providers":[
              {"provider_id":"mistral_us","units":{"gpu_second":12000,"bad_unit":9}}
            ]}"#,
        );
        assert_eq!(wrapped.providers.len(), 1);
        assert_eq!(wrapped.providers[0].provider_id, "mistral_us");
        assert_eq!(
            wrapped.providers[0].units.get("gpu_second").copied(),
            Some(12_000)
        );
        assert!(!wrapped.providers[0].units.contains_key("bad_unit"));
    }

    #[test]
    fn bundled_catalog_has_us_openai_and_anthropic() {
        let catalog = bundled_pricing_provider_catalog();
        assert_eq!(catalog.providers.len(), 2);
        let ids: Vec<_> = catalog
            .providers
            .iter()
            .map(|p| p.provider_id.as_str())
            .collect();
        assert!(ids.contains(&"openai_us"));
        assert!(ids.contains(&"anthropic_us"));
    }

    #[test]
    fn catalog_matching_entries_filters_region_task_and_model() {
        let catalog = GalaxyPricingProviderCatalog {
            providers: vec![
                GalaxyPricingProviderEntry {
                    provider_id: "openai_us".into(),
                    region: "us".into(),
                    model_profile: Some("gpt-4o-mini".into()),
                    task_profiles: vec!["inference:text".into()],
                    units: HashMap::from([("inference_blended_token".into(), 500_000)]),
                    endpoint: None,
                    enabled: true,
                },
                GalaxyPricingProviderEntry {
                    provider_id: "eu_only".into(),
                    region: "eu".into(),
                    model_profile: None,
                    task_profiles: vec![],
                    units: HashMap::from([("inference_blended_token".into(), 100_000)]),
                    endpoint: None,
                    enabled: true,
                },
            ],
        };
        let matched: Vec<_> = catalog
            .matching_entries("inference:text", "gpt-4o-mini")
            .into_iter()
            .map(|p| p.provider_id.as_str())
            .collect();
        assert_eq!(matched, vec!["openai_us"]);
    }

    #[test]
    fn from_env_loads_provider_catalog_when_env_set() {
        const KEY: &str = ENV_PRICING_PROVIDERS;
        std::env::set_var(
            KEY,
            r#"[{"provider_id":"custom_us","region":"us","units":{"job_flat":990000}}]"#,
        );
        let oracle = GalaxyPricingOracle::from_env();
        std::env::remove_var(KEY);
        assert_eq!(oracle.provider_catalog().providers.len(), 1);
        assert_eq!(
            oracle.provider_catalog().providers[0].provider_id,
            "custom_us"
        );
    }

    #[test]
    fn pricing_provider_catalog_from_env_falls_back_to_bundled_on_invalid_json() {
        const KEY: &str = ENV_PRICING_PROVIDERS;
        std::env::set_var(KEY, "not-json");
        let catalog = pricing_provider_catalog_from_env();
        std::env::remove_var(KEY);
        assert_eq!(catalog.providers.len(), 2);
        assert_eq!(catalog.providers[0].provider_id, "openai_us");
    }

    #[tokio::test]
    async fn fetch_live_provider_quotes_records_errors_on_unreachable_endpoint_ph_s173() {
        use crate::grid::galaxy_pricing_provider_metrics::{
            provider_errors_total, reset_provider_catalog_metrics_for_test,
        };
        use std::collections::HashMap;

        reset_provider_catalog_metrics_for_test();
        let catalog = GalaxyPricingProviderCatalog {
            providers: vec![GalaxyPricingProviderEntry {
                provider_id: "bad_live".into(),
                region: "us".into(),
                model_profile: Some("fail-model".into()),
                task_profiles: vec!["inference:text".into()],
                units: HashMap::from([(
                    GalaxyPriceUnitKey::InferenceBlendedToken.as_str().into(),
                    500_000,
                )]),
                endpoint: Some("http://127.0.0.1:1/unreachable".into()),
                enabled: true,
            }],
        };
        let quotes = fetch_live_provider_quotes(
            &catalog,
            "inference:text",
            "fail-model",
            GalaxyPriceUnitKey::InferenceBlendedToken,
            500,
        )
        .await;
        assert!(quotes.is_empty());
        assert_eq!(provider_errors_total(), 1);
        reset_provider_catalog_metrics_for_test();
    }
}
