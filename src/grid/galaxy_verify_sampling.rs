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

/// Env: elevated sample rate after mismatch (0.0..=1.0, PH-S455).
pub const ENV_VERIFY_ELEVATED_RATE: &str = "POOLAI_GALAXY_VERIFY_ELEVATED_RATE";

/// Env: checker task timeout seconds before inconclusive policy (PH-S542).
pub const ENV_CHECKER_TIMEOUT_SECS: &str = "POOLAI_GALAXY_CHECKER_TIMEOUT_SECS";

/// Default checker timeout before inconclusive escalation (Galaxy §6.2).
pub const DEFAULT_CHECKER_TIMEOUT_SECS: u64 = 300;

/// Metric: checker timeout → verification inconclusive (PH-S542).
pub const METRIC_VERIFY_CHECKER_TIMEOUT_INCONCLUSIVE_TOTAL: &str =
    "galaxy_verification_checker_timeout_inconclusive_total";

static CHECKER_TIMEOUT_INCONCLUSIVE_TOTAL: AtomicU64 = AtomicU64::new(0);
static CHECKER_TIMEOUT_RETRY_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Concept default elevated rate after mismatch: 25%.
pub const DEFAULT_VERIFY_ELEVATED_SAMPLE_RATE: f64 = 0.25;

/// Concept default: 5% of edge results sampled for verification.
pub const DEFAULT_VERIFY_BASE_SAMPLE_RATE: f64 = 0.05;

static VERIFY_SAMPLE_SCHEDULED_TOTAL: AtomicU64 = AtomicU64::new(0);
static VERIFY_SAMPLE_SKIPPED_TOTAL: AtomicU64 = AtomicU64::new(0);
static VERIFY_SAMPLE_NOT_APPLICABLE_TOTAL: AtomicU64 = AtomicU64::new(0);
static VERIFY_SAMPLING_EVALUATIONS_TOTAL: AtomicU64 = AtomicU64::new(0);
static VERIFY_ELEVATED_APPLIED_TOTAL: AtomicU64 = AtomicU64::new(0);

/// In-process counter for stub scheduled verification samples (grid result path).
pub const METRIC_VERIFY_SAMPLE_SCHEDULED_TOTAL: &str = "galaxy_verification_sample_scheduled_total";

/// In-process counter for edge samples not selected by deterministic stub (PH-S345).
pub const METRIC_VERIFY_SAMPLE_SKIPPED_TOTAL: &str = "galaxy_verification_sample_skipped_total";

/// In-process counter for verification sampling not applicable (local origin, PH-S356).
pub const METRIC_VERIFY_SAMPLE_NOT_APPLICABLE_TOTAL: &str =
    "galaxy_verification_sample_not_applicable_total";

/// In-process counter for verification sampling evaluations on grid result path (PH-S414).
pub const METRIC_VERIFY_SAMPLING_EVALUATIONS_TOTAL: &str =
    "galaxy_verification_sampling_evaluations_total";

/// Elevated sample rate applied after mismatch (PH-S455).
pub const METRIC_VERIFY_ELEVATED_APPLIED_TOTAL: &str = "galaxy_verification_elevated_applied_total";

/// Coordinator verification sampling policy (env-backed stub).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VerifySamplingConfig {
    pub base_sample_rate: f64,
    pub elevated_sample_rate: f64,
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
    /// Checker timed out — settlement inconclusive, elevated sample rate (PH-S542).
    VerificationInconclusive,
}

impl VerifySamplingConfig {
    pub const fn default_stub() -> Self {
        Self {
            base_sample_rate: DEFAULT_VERIFY_BASE_SAMPLE_RATE,
            elevated_sample_rate: DEFAULT_VERIFY_ELEVATED_SAMPLE_RATE,
        }
    }

    /// Read env vars; invalid/missing → [`default_stub`].
    pub fn from_env() -> Self {
        let base = match std::env::var(ENV_VERIFY_BASE_SAMPLE_RATE) {
            Ok(raw) => {
                parse_verify_base_sample_rate(&raw).unwrap_or(DEFAULT_VERIFY_BASE_SAMPLE_RATE)
            }
            Err(_) => DEFAULT_VERIFY_BASE_SAMPLE_RATE,
        };
        let elevated = match std::env::var(ENV_VERIFY_ELEVATED_RATE) {
            Ok(raw) => {
                parse_verify_base_sample_rate(&raw).unwrap_or(DEFAULT_VERIFY_ELEVATED_SAMPLE_RATE)
            }
            Err(_) => DEFAULT_VERIFY_ELEVATED_SAMPLE_RATE,
        };
        Self {
            base_sample_rate: base,
            elevated_sample_rate: elevated,
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
    VERIFY_SAMPLING_EVALUATIONS_TOTAL.fetch_add(1, Ordering::Relaxed);
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
        VerifySamplingVerdict::VerificationInconclusive => {
            VERIFY_SAMPLE_SCHEDULED_TOTAL.fetch_add(1, Ordering::Relaxed);
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

/// Total verification sampling evaluations since process start (PH-S414).
pub fn verify_sampling_evaluations_total() -> u64 {
    VERIFY_SAMPLING_EVALUATIONS_TOTAL.load(Ordering::Relaxed)
}

/// Post-mismatch elevated sampling stub (PH-S455).
pub fn evaluate_post_mismatch_elevated_sampling(
    job_id: &str,
    config: &VerifySamplingConfig,
) -> bool {
    if deterministic_sample_selected(job_id, config.elevated_sample_rate) {
        VERIFY_ELEVATED_APPLIED_TOTAL.fetch_add(1, Ordering::Relaxed);
        true
    } else {
        false
    }
}

pub fn verify_elevated_applied_total() -> u64 {
    VERIFY_ELEVATED_APPLIED_TOTAL.load(Ordering::Relaxed)
}

pub fn checker_timeout_secs_from_env() -> u64 {
    std::env::var(ENV_CHECKER_TIMEOUT_SECS)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .filter(|&n| n > 0)
        .unwrap_or(DEFAULT_CHECKER_TIMEOUT_SECS)
}

pub fn checker_timeout_inconclusive_total() -> u64 {
    CHECKER_TIMEOUT_INCONCLUSIVE_TOTAL.load(Ordering::Relaxed)
}

pub fn checker_timeout_retry_total() -> u64 {
    CHECKER_TIMEOUT_RETRY_TOTAL.load(Ordering::Relaxed)
}

/// Apply checker_timeout policy: one retry, else verification_inconclusive + elevated sample (PH-S542).
pub fn evaluate_checker_timeout_policy(
    job_id: &str,
    metrics: Option<&serde_json::Value>,
    config: &VerifySamplingConfig,
) -> VerifySamplingVerdict {
    let timed_out = metrics
        .and_then(|m| m.get("checker_timeout"))
        .and_then(|v| v.as_bool())
        == Some(true);
    if !timed_out {
        return VerifySamplingVerdict::NotSelected;
    }
    let retry_count = metrics
        .and_then(|m| m.get("checker_retry_count"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    if retry_count == 0 {
        CHECKER_TIMEOUT_RETRY_TOTAL.fetch_add(1, Ordering::Relaxed);
        return VerifySamplingVerdict::SampleScheduled;
    }
    CHECKER_TIMEOUT_INCONCLUSIVE_TOTAL.fetch_add(1, Ordering::Relaxed);
    let _ = evaluate_post_mismatch_elevated_sampling(job_id, config);
    VerifySamplingVerdict::VerificationInconclusive
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
    VERIFY_SAMPLING_EVALUATIONS_TOTAL.store(0, Ordering::Relaxed);
    VERIFY_ELEVATED_APPLIED_TOTAL.store(0, Ordering::Relaxed);
    CHECKER_TIMEOUT_INCONCLUSIVE_TOTAL.store(0, Ordering::Relaxed);
    CHECKER_TIMEOUT_RETRY_TOTAL.store(0, Ordering::Relaxed);
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
            elevated_sample_rate: DEFAULT_VERIFY_ELEVATED_SAMPLE_RATE,
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
            elevated_sample_rate: DEFAULT_VERIFY_ELEVATED_SAMPLE_RATE,
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

    #[test]
    fn verify_sampling_evaluations_total_ph_s414() {
        reset_verify_sampling_metrics_for_test();
        let cfg = VerifySamplingConfig::default_stub();
        evaluate_result_verify_sampling(Some("peer-a"), "job-1", &cfg);
        evaluate_result_verify_sampling(Some("tg-edge"), "job-2", &cfg);
        assert_eq!(verify_sampling_evaluations_total(), 2);
        reset_verify_sampling_metrics_for_test();
    }

    #[test]
    fn evaluate_post_mismatch_elevated_sampling_ph_s455() {
        reset_verify_sampling_metrics_for_test();
        let cfg = VerifySamplingConfig {
            base_sample_rate: 0.05,
            elevated_sample_rate: 1.0,
        };
        assert!(evaluate_post_mismatch_elevated_sampling(
            "job-mismatch",
            &cfg
        ));
        assert_eq!(verify_elevated_applied_total(), 1);
        reset_verify_sampling_metrics_for_test();
    }

    #[test]
    fn evaluate_checker_timeout_policy_ph_s542() {
        reset_verify_sampling_metrics_for_test();
        let cfg = VerifySamplingConfig::default_stub();
        assert_eq!(
            evaluate_checker_timeout_policy(
                "job-t",
                Some(&serde_json::json!({"checker_timeout": true, "checker_retry_count": 0})),
                &cfg,
            ),
            VerifySamplingVerdict::SampleScheduled
        );
        assert_eq!(
            evaluate_checker_timeout_policy(
                "job-t",
                Some(&serde_json::json!({"checker_timeout": true, "checker_retry_count": 1})),
                &cfg,
            ),
            VerifySamplingVerdict::VerificationInconclusive
        );
        assert_eq!(checker_timeout_inconclusive_total(), 1);
        reset_verify_sampling_metrics_for_test();
    }
}
