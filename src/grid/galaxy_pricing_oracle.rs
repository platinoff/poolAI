//! Galaxy Grid pricing oracle stub (PH-S68): unit keys, `floor(market_min×0.9)` quote,
//! cache TTL/SWR from `POOLAI_GALAXY_PRICE_*` env; L2 force-fallback ops wire (PH-S81);
//! L1 stale-served metric (PH-S83). See `docs/concept/POOLAI_GALAXY_GRID.md` §4.2.
//!
//! Oracle determines **gross quote** in micro-USD; settlement uses `galaxy_fee_split`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
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

/// In-process counter name for forced L2 quotes (Prometheus wire — future).
pub const METRIC_FORCED_FALLBACK_TOTAL: &str = "galaxy_pricing_forced_fallback_total";

static FORCED_FALLBACK_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Structured log event when L1 stale cache is served (§4.2.4).
pub const STALE_SERVED_LOG_EVENT: &str = "pricing_oracle_stale_served";

/// In-process counter for L1 stale cache serves (§4.2.4, PH-S83).
pub const METRIC_STALE_SERVED_TOTAL: &str = "galaxy_pricing_stale_served";

static STALE_SERVED_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Env: JSON map for L2 fallback floor quotes in micro-USD (§4.2.4).
/// Example:
/// `{"inference_blended_token":450000,"gpu_second":12000}`
pub const ENV_FALLBACK_JSON: &str = "POOLAI_GALAXY_PRICING_FALLBACK_JSON";

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

#[cfg(test)]
pub fn reset_forced_fallback_total_for_test() {
    FORCED_FALLBACK_TOTAL.store(0, Ordering::Relaxed);
}

/// Total L1 stale cache quotes served since process start (ops/metrics snapshot).
pub fn stale_served_total() -> u64 {
    STALE_SERVED_TOTAL.load(Ordering::Relaxed)
}

#[cfg(test)]
pub fn reset_stale_served_total_for_test() {
    STALE_SERVED_TOTAL.store(0, Ordering::Relaxed);
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

/// Record L1 stale metric when serving from cache (oracle or HTTP snapshot path).
pub fn record_l1_stale_served(unit_key: GalaxyPriceUnitKey) {
    record_stale_served(unit_key);
}

fn serve_l1_cache_quote(
    entry: GalaxyPricingCacheEntry,
    freshness: CacheFreshness,
) -> GalaxyPricingQuote {
    if freshness == CacheFreshness::Stale {
        record_stale_served(entry.quote.unit_key);
    }
    entry.quote
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
    pub provider_id: &'static str,
    pub unit_key: GalaxyPriceUnitKey,
    pub usd_micro: u64,
    pub healthy: bool,
}

/// Minimum normalized unit price among healthy providers (§4.2.2).
pub fn market_min_usd_micro(
    providers: &[MockProviderQuote],
    unit_key: GalaxyPriceUnitKey,
) -> Option<(u64, &'static str)> {
    providers
        .iter()
        .filter(|p| p.healthy && p.unit_key == unit_key && p.usd_micro > 0)
        .min_by_key(|p| p.usd_micro)
        .map(|p| (p.usd_micro, p.provider_id))
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
}

impl GalaxyPricingOracle {
    pub fn new(config: GalaxyPricingConfig) -> Self {
        Self {
            config,
            cache: HashMap::new(),
            fallback_quotes_usd_micro: HashMap::new(),
        }
    }

    pub fn from_env() -> Self {
        let mut oracle = Self::new(GalaxyPricingConfig::from_env());
        if let Ok(raw) = std::env::var(ENV_FALLBACK_JSON) {
            oracle.fallback_quotes_usd_micro = parse_fallback_json(&raw);
        }
        oracle
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
            return self
                .l2_fallback_quote(now_secs, key)
                .map(|quote| {
                    record_forced_fallback(quote.unit_key);
                    quote
                })
                .ok_or(GalaxyPricingUnavailable);
        }
        if let Some((entry, freshness)) = self.lookup(now_secs, &key) {
            if freshness == CacheFreshness::Fresh || freshness == CacheFreshness::Stale {
                return Ok(serve_l1_cache_quote(entry, freshness));
            }
        }
        self.refresh_from_providers(now_secs, key.clone(), providers)
            .or_else(|| self.l2_fallback_quote(now_secs, key))
            .ok_or(GalaxyPricingUnavailable)
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

    fn mock_us_blended() -> Vec<MockProviderQuote> {
        vec![
            MockProviderQuote {
                provider_id: "openai_us",
                unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
                usd_micro: 500_000,
                healthy: true,
            },
            MockProviderQuote {
                provider_id: "anthropic_us",
                unit_key: GalaxyPriceUnitKey::InferenceBlendedToken,
                usd_micro: 600_000,
                healthy: true,
            },
            MockProviderQuote {
                provider_id: "stale_vendor",
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
            provider_id: "cheap_now",
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
            provider_id: "x",
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
        reset_stale_served_total_for_test();
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

        oracle
            .try_quote(400, key.clone(), &[])
            .expect("L1 stale serve");
        assert_eq!(stale_served_total(), 1);

        oracle.try_quote(100, key, &[]).expect("L1 fresh serve");
        assert_eq!(
            stale_served_total(),
            1,
            "fresh cache must not increment stale metric"
        );
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
}
