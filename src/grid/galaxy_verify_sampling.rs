//! Galaxy Grid edge verification sampling config + stub (PH-S142 / PH-S164).
//!
//! Parses `POOLAI_GALAXY_VERIFY_BASE_SAMPLE_RATE` per `docs/concept/POOLAI_GALAXY_GRID.md` §6.2.
//! HTTP grid middleware exposes configured rate; grid result ingest applies deterministic stub
//! selection for `telegram_edge` peers. No live verification enqueue wire.

use std::sync::atomic::{AtomicU64, Ordering};

use crate::grid::galaxy_trust_score::{infer_worker_origin, WorkerOrigin};

/// Response header: configured base sample rate on grid wire routes (PH-S164).
pub const HEADER_VERIFY_BASE_SAMPLE_RATE: &str = "x-poolai-verify-base-sample-rate";

/// Env: base sampling rate for `telegram_edge` verification (0.0..=1.0).
pub const ENV_VERIFY_BASE_SAMPLE_RATE: &str = "POOLAI_GALAXY_VERIFY_BASE_SAMPLE_RATE";

/// Concept default: 5% of edge results sampled for verification.
pub const DEFAULT_VERIFY_BASE_SAMPLE_RATE: f64 = 0.05;

static VERIFY_SAMPLE_SCHEDULED_TOTAL: AtomicU64 = AtomicU64::new(0);
static VERIFY_SAMPLE_SKIPPED_TOTAL: AtomicU64 = AtomicU64::new(0);
static VERIFY_SAMPLE_NOT_APPLICABLE_TOTAL: AtomicU64 = AtomicU64::new(0);

/// In-process counter for stub scheduled verification samples (grid result path).
pub const METRIC_VERIFY_SAMPLE_SCHEDULED_TOTAL: &str = "galaxy_verification_sample_scheduled_total";

/// In-process counter for edge samples not selected by deterministic stub (PH-S345).
pub const METRIC_VERIFY_SAMPLE_SKIPPED_TOTAL: &str = "galaxy_verification_sample_skipped_total";

/// In-process counter for verification sampling not applicable (local origin, PH-S356).
pub const METRIC_VERIFY_SAMPLE_NOT_APPLICABLE_TOTAL: &str =
    "galaxy_verification_sample_not_applicable_total";

/// Coordinator verification sampling policy (env-backed stub).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VerifySamplingConfig {
    pub base_sample_rate: f64,
}

/// Stub verdict on grid result ingest (no enqueue wire).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifySamplingVerdict {
    /// Edge peer selected by deterministic stub for verification scheduling.
    SampleScheduled,
    /// Sampling not applicable (non-edge origin).
    NotApplicable,
    /// Edge peer but stub did not select this result (below configured rate).
    NotSelected,
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

/// Deterministic stub: stable fraction in `[0,1)` from `job_id` (no RNG wire).
pub fn deterministic_sample_fraction(job_id: &str) -> f64 {
    let mut hash: u64 = 0;
    for b in job_id.bytes() {
        hash = hash.wrapping_mul(997).wrapping_add(u64::from(b));
    }
    (hash % 10_000) as f64 / 10_000.0
}

/// Whether deterministic stub selects this job for verification sampling.
pub fn deterministic_sample_selected(job_id: &str, rate: f64) -> bool {
    if rate <= 0.0 {
        return false;
    }
    if rate >= 1.0 {
        return true;
    }
    deterministic_sample_fraction(job_id) < rate
}

/// Grid result path helper: edge-only deterministic verification sample stub (PH-S164).
pub fn evaluate_result_verify_sampling(
    source_peer_id: Option<&str>,
    job_id: &str,
    config: &VerifySamplingConfig,
) -> VerifySamplingVerdict {
    let verdict = match infer_worker_origin(source_peer_id) {
        WorkerOrigin::LocalSrv => VerifySamplingVerdict::NotApplicable,
        WorkerOrigin::TelegramEdge => {
            if deterministic_sample_selected(job_id, config.base_sample_rate) {
                VerifySamplingVerdict::SampleScheduled
            } else {
                VerifySamplingVerdict::NotSelected
            }
        }
    };
    record_verify_sampling_verdict(verdict);
    verdict
}

/// Record stub verdict counter (grid result path only).
pub fn record_verify_sampling_verdict(verdict: VerifySamplingVerdict) {
    match verdict {
        VerifySamplingVerdict::SampleScheduled => {
            VERIFY_SAMPLE_SCHEDULED_TOTAL.fetch_add(1, Ordering::Relaxed);
        }
        VerifySamplingVerdict::NotSelected => {
            VERIFY_SAMPLE_SKIPPED_TOTAL.fetch_add(1, Ordering::Relaxed);
        }
        VerifySamplingVerdict::NotApplicable => {
            VERIFY_SAMPLE_NOT_APPLICABLE_TOTAL.fetch_add(1, Ordering::Relaxed);
        }
    }
}

/// Total stub verification samples scheduled since process start.
pub fn verify_sample_scheduled_total() -> u64 {
    VERIFY_SAMPLE_SCHEDULED_TOTAL.load(Ordering::Relaxed)
}

/// Total edge samples skipped by deterministic stub since process start.
pub fn verify_sample_skipped_total() -> u64 {
    VERIFY_SAMPLE_SKIPPED_TOTAL.load(Ordering::Relaxed)
}

/// Total verification samples not applicable (local origin) since process start.
pub fn verify_sample_not_applicable_total() -> u64 {
    VERIFY_SAMPLE_NOT_APPLICABLE_TOTAL.load(Ordering::Relaxed)
}

/// Format sample rate for the HTTP response header (6 decimal places).
pub fn format_verify_base_sample_rate_header(rate: f64) -> String {
    format!("{rate:.6}")
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_verify_sampling_metrics_for_test() {
    VERIFY_SAMPLE_SCHEDULED_TOTAL.store(0, Ordering::Relaxed);
    VERIFY_SAMPLE_SKIPPED_TOTAL.store(0, Ordering::Relaxed);
    VERIFY_SAMPLE_NOT_APPLICABLE_TOTAL.store(0, Ordering::Relaxed);
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

    #[test]
    fn deterministic_sample_fraction_is_stable() {
        let a = deterministic_sample_fraction("job-abc");
        let b = deterministic_sample_fraction("job-abc");
        assert_eq!(a, b);
        assert!((0.0..1.0).contains(&a));
    }

    #[test]
    fn evaluate_result_verify_sampling_local_not_applicable() {
        reset_verify_sampling_metrics_for_test();
        let cfg = VerifySamplingConfig::default_stub();
        assert_eq!(
            evaluate_result_verify_sampling(Some("peer-a"), "job-1", &cfg),
            VerifySamplingVerdict::NotApplicable
        );
        assert_eq!(verify_sample_scheduled_total(), 0);
        reset_verify_sampling_metrics_for_test();
    }

    #[test]
    fn evaluate_result_verify_sampling_edge_schedules_counter() {
        reset_verify_sampling_metrics_for_test();
        let cfg = VerifySamplingConfig {
            base_sample_rate: 1.0,
        };
        assert_eq!(
            evaluate_result_verify_sampling(Some("tg-edge"), "job-edge", &cfg),
            VerifySamplingVerdict::SampleScheduled
        );
        assert_eq!(verify_sample_scheduled_total(), 1);
        reset_verify_sampling_metrics_for_test();
    }

    #[test]
    fn format_verify_base_sample_rate_header_six_decimals() {
        assert_eq!(format_verify_base_sample_rate_header(0.05), "0.050000");
    }

    #[test]
    fn evaluate_result_verify_sampling_skipped_counter_ph_s345() {
        reset_verify_sampling_metrics_for_test();
        let cfg = VerifySamplingConfig {
            base_sample_rate: 0.0,
        };
        assert_eq!(
            evaluate_result_verify_sampling(Some("tg-edge"), "job-skip", &cfg),
            VerifySamplingVerdict::NotSelected
        );
        assert_eq!(verify_sample_skipped_total(), 1);
        reset_verify_sampling_metrics_for_test();
    }

    #[test]
    fn evaluate_result_verify_sampling_not_applicable_counter_ph_s356() {
        reset_verify_sampling_metrics_for_test();
        let cfg = VerifySamplingConfig::default_stub();
        assert_eq!(
            evaluate_result_verify_sampling(Some("peer-local"), "job-local", &cfg),
            VerifySamplingVerdict::NotApplicable
        );
        assert_eq!(verify_sample_not_applicable_total(), 1);
        reset_verify_sampling_metrics_for_test();
    }
}
