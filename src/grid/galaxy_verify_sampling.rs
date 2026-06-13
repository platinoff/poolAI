//! Galaxy Grid edge verification sampling config stub (PH-S142).
//!
//! Parses `POOLAI_GALAXY_VERIFY_BASE_SAMPLE_RATE` per `docs/concept/POOLAI_GALAXY_GRID.md` §6.6.
//! No live sampling enqueue wire.

/// Env: base sampling rate for `telegram_edge` verification (0.0..=1.0).
pub const ENV_VERIFY_BASE_SAMPLE_RATE: &str = "POOLAI_GALAXY_VERIFY_BASE_SAMPLE_RATE";

/// Concept default: 5% of edge results sampled for verification.
pub const DEFAULT_VERIFY_BASE_SAMPLE_RATE: f64 = 0.05;

/// Coordinator verification sampling policy (env-backed stub).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VerifySamplingConfig {
    pub base_sample_rate: f64,
}

impl VerifySamplingConfig {
    pub const fn default_stub() -> Self {
        Self {
            base_sample_rate: DEFAULT_VERIFY_BASE_SAMPLE_RATE,
        }
    }

    /// Read [`ENV_VERIFY_BASE_SAMPLE_RATE`]; invalid/missing → [`default_stub`].
    pub fn from_env() -> Self {
        match std::env::var(ENV_VERIFY_BASE_SAMPLE_RATE) {
            Ok(raw) => Self {
                base_sample_rate: parse_verify_base_sample_rate(&raw)
                    .unwrap_or(DEFAULT_VERIFY_BASE_SAMPLE_RATE),
            },
            Err(_) => Self::default_stub(),
        }
    }
}

/// Parse sample rate in **0.0..=1.0** (fraction). Rejects NaN/inf and out-of-range values.
pub fn parse_verify_base_sample_rate(raw: &str) -> Option<f64> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    let value: f64 = trimmed.parse().ok()?;
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return None;
    }
    Some(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_verify_base_sample_rate_accepts_fraction() {
        assert_eq!(parse_verify_base_sample_rate("0.05"), Some(0.05));
        assert_eq!(parse_verify_base_sample_rate(" 0.2 "), Some(0.2));
        assert_eq!(parse_verify_base_sample_rate("0"), Some(0.0));
        assert_eq!(parse_verify_base_sample_rate("1"), Some(1.0));
    }

    #[test]
    fn parse_verify_base_sample_rate_rejects_out_of_range() {
        assert_eq!(parse_verify_base_sample_rate("-0.01"), None);
        assert_eq!(parse_verify_base_sample_rate("1.01"), None);
        assert_eq!(parse_verify_base_sample_rate("nan"), None);
        assert_eq!(parse_verify_base_sample_rate(""), None);
        assert_eq!(parse_verify_base_sample_rate("  "), None);
    }

    #[test]
    fn from_env_reads_base_sample_rate() {
        std::env::set_var(ENV_VERIFY_BASE_SAMPLE_RATE, "0.25");
        let cfg = VerifySamplingConfig::from_env();
        assert_eq!(cfg.base_sample_rate, 0.25);
        std::env::remove_var(ENV_VERIFY_BASE_SAMPLE_RATE);
    }

    #[test]
    fn from_env_falls_back_on_invalid() {
        std::env::set_var(ENV_VERIFY_BASE_SAMPLE_RATE, "2.0");
        let cfg = VerifySamplingConfig::from_env();
        assert_eq!(cfg, VerifySamplingConfig::default_stub());
        std::env::remove_var(ENV_VERIFY_BASE_SAMPLE_RATE);
    }

    #[test]
    fn default_stub_matches_concept() {
        assert_eq!(
            VerifySamplingConfig::default_stub().base_sample_rate,
            DEFAULT_VERIFY_BASE_SAMPLE_RATE
        );
    }
}
