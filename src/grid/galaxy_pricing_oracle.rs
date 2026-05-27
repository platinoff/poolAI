//! Galaxy Grid pricing oracle stub (PH-S68): unit keys, `floor(market_min×0.9)` quote,
//! cache TTL/SWR from `POOLAI_GALAXY_PRICE_*` env. See `docs/concept/POOLAI_GALAXY_GRID.md` §4.2.
//!
//! Oracle determines **gross quote** in micro-USD; settlement uses `galaxy_fee_split`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

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

/// Env: `1` forces L2 fallback path (§4.2.4); stub records flag only.
pub const ENV_FORCE_FALLBACK: &str = "POOLAI_GALAXY_PRICING_FORCE_FALLBACK";

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
}

impl GalaxyPricingOracle {
    pub fn new(config: GalaxyPricingConfig) -> Self {
        Self {
            config,
            cache: HashMap::new(),
        }
    }

    pub fn from_env() -> Self {
        Self::new(GalaxyPricingConfig::from_env())
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

    /// Lookup fresh/stale cache or refresh on miss/expired (§4.2.3 behaviour sketch).
    pub fn quote(
        &mut self,
        now_secs: u64,
        key: GalaxyPricingCacheKey,
        providers: &[MockProviderQuote],
    ) -> Option<GalaxyPricingQuote> {
        if let Some((entry, freshness)) = self.lookup(now_secs, &key) {
            if freshness == CacheFreshness::Fresh || freshness == CacheFreshness::Stale {
                return Some(entry.quote);
            }
        }
        self.refresh_from_providers(now_secs, key, providers)
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
}
