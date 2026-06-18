//! Galaxy Grid edge `trust_score` settlement gate stub (PH-S130).
//!
//! Pure gate sketch on the grid result path per `docs/concept/POOLAI_GALAXY_GRID.md` §6.5.
//! Settlement verdict counters mirrored on `GET /metrics` (PH-S137 stub; PH-S163 grid result wire).
//! Last observed `trust_score` gauge on grid result path (PH-S182).

use std::sync::atomic::{AtomicU64, Ordering};

/// Worker origin subset for settlement gate (Galaxy §3.2 / §6.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerOrigin {
    LocalSrv,
    TelegramEdge,
}

/// Settlement gate verdict (stub — no fee-split or payout wire).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlementGateVerdict {
    /// Trusted origin or `trust_score` ≥ threshold; payout stub may proceed.
    PayoutEligible,
    /// `telegram_edge` below threshold — hold payout (`pending_verification`).
    PayoutHeld,
    /// Gate not applied (non-edge origin).
    NotApplicable,
}

/// `trust_score` range on the grid stub path (PH-S130 uses 0..=100).
pub type TrustScore = u8;

/// Default mid trust on 0..=100 stub scale (concept §6.5 default 500 on 0..1000).
pub const DEFAULT_TRUST_SCORE: TrustScore = 50;

/// Default minimum trust for auto payout on 0..=100 scale (concept `400` on 0..1000).
pub const DEFAULT_MIN_TRUST_FOR_PAYOUT: TrustScore = 40;

/// Env: minimum `trust_score` (0..=100) for auto payout on `telegram_edge` results.
pub const ENV_MIN_TRUST_PAYOUT: &str = "POOLAI_GALAXY_MIN_TRUST_PAYOUT";

/// In-process counter for edge results eligible for payout stub (mirrored on `GET /metrics`, PH-S137).
pub const METRIC_PAYOUT_ELIGIBLE_TOTAL: &str = "galaxy_trust_payout_eligible_total";

/// In-process counter for edge results held pending verification (PH-S137).
pub const METRIC_PAYOUT_HELD_TOTAL: &str = "galaxy_trust_payout_held_total";

/// In-process counter for local-origin results where trust gate is not applicable (PH-S364).
pub const METRIC_PAYOUT_NOT_APPLICABLE_TOTAL: &str = "galaxy_trust_payout_not_applicable_total";

/// Last observed grid result `trust_score` on 0..=100 scale (PH-S182 `/metrics` gauge).
pub const METRIC_TRUST_SCORE: &str = "galaxy_trust_score";

/// Configured minimum trust (0..=100) for edge auto payout (PH-S374 `/metrics` gauge).
pub const METRIC_TRUST_GATE_MIN_THRESHOLD: &str = "galaxy_trust_gate_min_threshold";

/// Default trust score used when grid result omits `trust_score` (PH-S384 `/metrics` gauge).
pub const METRIC_TRUST_GATE_DEFAULT_SCORE: &str = "galaxy_trust_gate_default_score";

/// Total settlement gate evaluations on grid result path (PH-S394 `/metrics` gauge).
pub const METRIC_GATE_EVALUATIONS_TOTAL: &str = "galaxy_trust_gate_evaluations_total";

/// Total grid results where default trust score was applied (PH-S395 `/metrics` gauge).
pub const METRIC_DEFAULT_SCORE_APPLIED_TOTAL: &str = "galaxy_trust_default_score_applied_total";

static PAYOUT_ELIGIBLE_TOTAL: AtomicU64 = AtomicU64::new(0);
static PAYOUT_HELD_TOTAL: AtomicU64 = AtomicU64::new(0);
static PAYOUT_NOT_APPLICABLE_TOTAL: AtomicU64 = AtomicU64::new(0);
static LAST_TRUST_SCORE: AtomicU64 = AtomicU64::new(0);
static GATE_EVALUATIONS_TOTAL: AtomicU64 = AtomicU64::new(0);
static DEFAULT_SCORE_APPLIED_TOTAL: AtomicU64 = AtomicU64::new(0);

/// Gate configuration (env-backed stub).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrustScoreGateConfig {
    pub min_trust_for_payout: TrustScore,
}

impl TrustScoreGateConfig {
    pub const fn default_stub() -> Self {
        Self {
            min_trust_for_payout: DEFAULT_MIN_TRUST_FOR_PAYOUT,
        }
    }

    /// Parse [`ENV_MIN_TRUST_PAYOUT`] (0..=100); invalid/missing → default.
    pub fn from_env() -> Self {
        match std::env::var(ENV_MIN_TRUST_PAYOUT) {
            Ok(raw) => match raw.trim().parse::<u16>() {
                Ok(v) if v <= 100 => Self {
                    min_trust_for_payout: v as TrustScore,
                },
                _ => Self::default_stub(),
            },
            Err(_) => Self::default_stub(),
        }
    }
}

/// Clamp arbitrary input to 0..=100.
#[inline]
pub fn clamp_trust_score(score: u16) -> TrustScore {
    score.min(100) as TrustScore
}

/// Infer worker origin from peer id prefix (stub until unified worker DTO wire).
#[inline]
pub fn infer_worker_origin(peer_id: Option<&str>) -> WorkerOrigin {
    let Some(id) = peer_id else {
        return WorkerOrigin::LocalSrv;
    };
    if id.starts_with("tg-") || id.starts_with("telegram-") {
        WorkerOrigin::TelegramEdge
    } else {
        WorkerOrigin::LocalSrv
    }
}

/// Evaluate settlement gate for a grid result producer (no payout wire).
pub fn evaluate_settlement_gate(
    origin: WorkerOrigin,
    trust_score: TrustScore,
    config: &TrustScoreGateConfig,
) -> SettlementGateVerdict {
    match origin {
        WorkerOrigin::LocalSrv => SettlementGateVerdict::NotApplicable,
        WorkerOrigin::TelegramEdge => {
            if trust_score >= config.min_trust_for_payout {
                SettlementGateVerdict::PayoutEligible
            } else {
                SettlementGateVerdict::PayoutHeld
            }
        }
    }
}

/// Grid result path helper: infer origin from `source_peer_id`, apply gate with optional score.
pub fn evaluate_result_settlement_gate(
    source_peer_id: Option<&str>,
    trust_score: Option<TrustScore>,
    config: &TrustScoreGateConfig,
) -> SettlementGateVerdict {
    GATE_EVALUATIONS_TOTAL.fetch_add(1, Ordering::Relaxed);
    let origin = infer_worker_origin(source_peer_id);
    let score = match trust_score {
        Some(s) => s,
        None => {
            DEFAULT_SCORE_APPLIED_TOTAL.fetch_add(1, Ordering::Relaxed);
            DEFAULT_TRUST_SCORE
        }
    };
    observe_last_trust_score(score);
    let verdict = evaluate_settlement_gate(origin, score, config);
    record_settlement_gate_verdict(verdict);
    verdict
}

/// Observe last grid result trust score for Prometheus gauge (PH-S182).
pub fn observe_last_trust_score(score: TrustScore) {
    LAST_TRUST_SCORE.store(u64::from(score), Ordering::Relaxed);
}

/// Last observed grid result trust score (0..=100) since process start.
pub fn last_trust_score() -> u64 {
    LAST_TRUST_SCORE.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_last_trust_score_for_test() {
    LAST_TRUST_SCORE.store(0, Ordering::Relaxed);
}

/// Record settlement gate verdict for Prometheus scrape (grid result path only).
pub fn record_settlement_gate_verdict(verdict: SettlementGateVerdict) {
    match verdict {
        SettlementGateVerdict::PayoutEligible => {
            PAYOUT_ELIGIBLE_TOTAL.fetch_add(1, Ordering::Relaxed);
        }
        SettlementGateVerdict::PayoutHeld => {
            PAYOUT_HELD_TOTAL.fetch_add(1, Ordering::Relaxed);
        }
        SettlementGateVerdict::NotApplicable => {
            PAYOUT_NOT_APPLICABLE_TOTAL.fetch_add(1, Ordering::Relaxed);
        }
    }
}

/// Total local-origin grid results where trust gate is not applicable (PH-S364).
pub fn payout_not_applicable_total() -> u64 {
    PAYOUT_NOT_APPLICABLE_TOTAL.load(Ordering::Relaxed)
}

/// Total edge payout-eligible grid results since process start.
pub fn payout_eligible_total() -> u64 {
    PAYOUT_ELIGIBLE_TOTAL.load(Ordering::Relaxed)
}

/// Total edge payout-held grid results since process start.
pub fn payout_held_total() -> u64 {
    PAYOUT_HELD_TOTAL.load(Ordering::Relaxed)
}

/// Configured minimum trust for edge payout from env (mirrored on `GET /metrics`, PH-S374).
pub fn configured_min_trust_for_payout() -> u64 {
    u64::from(TrustScoreGateConfig::from_env().min_trust_for_payout)
}

/// Default trust score constant for edge settlement stub (mirrored on `GET /metrics`, PH-S384).
pub fn configured_default_trust_score() -> u64 {
    u64::from(DEFAULT_TRUST_SCORE)
}

/// Total settlement gate evaluations since process start (PH-S394).
pub fn gate_evaluations_total() -> u64 {
    GATE_EVALUATIONS_TOTAL.load(Ordering::Relaxed)
}

/// Total grid results where default trust score was applied (PH-S395).
pub fn default_score_applied_total() -> u64 {
    DEFAULT_SCORE_APPLIED_TOTAL.load(Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_settlement_gate_metrics_for_test() {
    PAYOUT_ELIGIBLE_TOTAL.store(0, Ordering::Relaxed);
    PAYOUT_HELD_TOTAL.store(0, Ordering::Relaxed);
    PAYOUT_NOT_APPLICABLE_TOTAL.store(0, Ordering::Relaxed);
    GATE_EVALUATIONS_TOTAL.store(0, Ordering::Relaxed);
    DEFAULT_SCORE_APPLIED_TOTAL.store(0, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_trust_score_caps_at_100() {
        assert_eq!(clamp_trust_score(0), 0);
        assert_eq!(clamp_trust_score(100), 100);
        assert_eq!(clamp_trust_score(150), 100);
    }

    #[test]
    fn infer_worker_origin_telegram_prefixes() {
        assert_eq!(
            infer_worker_origin(Some("tg-peer-1")),
            WorkerOrigin::TelegramEdge
        );
        assert_eq!(
            infer_worker_origin(Some("telegram-bot-7")),
            WorkerOrigin::TelegramEdge
        );
        assert_eq!(infer_worker_origin(Some("peer-a")), WorkerOrigin::LocalSrv);
        assert_eq!(infer_worker_origin(None), WorkerOrigin::LocalSrv);
    }

    #[test]
    fn evaluate_settlement_gate_local_not_applicable() {
        let cfg = TrustScoreGateConfig::default_stub();
        assert_eq!(
            evaluate_settlement_gate(WorkerOrigin::LocalSrv, 10, &cfg),
            SettlementGateVerdict::NotApplicable
        );
    }

    #[test]
    fn evaluate_settlement_gate_edge_hold_below_threshold() {
        let cfg = TrustScoreGateConfig::default_stub();
        assert_eq!(
            evaluate_settlement_gate(WorkerOrigin::TelegramEdge, 39, &cfg),
            SettlementGateVerdict::PayoutHeld
        );
    }

    #[test]
    fn evaluate_settlement_gate_edge_eligible_at_threshold() {
        let cfg = TrustScoreGateConfig::default_stub();
        assert_eq!(
            evaluate_settlement_gate(WorkerOrigin::TelegramEdge, 40, &cfg),
            SettlementGateVerdict::PayoutEligible
        );
        assert_eq!(
            evaluate_settlement_gate(WorkerOrigin::TelegramEdge, 90, &cfg),
            SettlementGateVerdict::PayoutEligible
        );
    }

    #[test]
    fn evaluate_result_settlement_gate_uses_default_score() {
        reset_settlement_gate_metrics_for_test();
        let cfg = TrustScoreGateConfig::default_stub();
        assert_eq!(
            evaluate_result_settlement_gate(Some("tg-low"), None, &cfg),
            SettlementGateVerdict::PayoutEligible
        );
        assert_eq!(payout_eligible_total(), 1);
        assert_eq!(payout_held_total(), 0);

        assert_eq!(
            evaluate_result_settlement_gate(Some("tg-low"), Some(10), &cfg),
            SettlementGateVerdict::PayoutHeld
        );
        assert_eq!(payout_eligible_total(), 1);
        assert_eq!(payout_held_total(), 1);
        reset_settlement_gate_metrics_for_test();
    }

    #[test]
    fn record_settlement_gate_verdict_increments_counters() {
        reset_settlement_gate_metrics_for_test();
        record_settlement_gate_verdict(SettlementGateVerdict::PayoutEligible);
        record_settlement_gate_verdict(SettlementGateVerdict::PayoutEligible);
        record_settlement_gate_verdict(SettlementGateVerdict::PayoutHeld);
        record_settlement_gate_verdict(SettlementGateVerdict::NotApplicable);
        assert_eq!(payout_eligible_total(), 2);
        assert_eq!(payout_held_total(), 1);
        assert_eq!(payout_not_applicable_total(), 1);
        reset_settlement_gate_metrics_for_test();
    }

    #[test]
    fn record_settlement_gate_verdict_not_applicable_ph_s364() {
        reset_settlement_gate_metrics_for_test();
        record_settlement_gate_verdict(SettlementGateVerdict::NotApplicable);
        record_settlement_gate_verdict(SettlementGateVerdict::NotApplicable);
        assert_eq!(payout_not_applicable_total(), 2);
        assert_eq!(payout_eligible_total(), 0);
        reset_settlement_gate_metrics_for_test();
    }

    #[test]
    fn observe_last_trust_score_stores_effective_score() {
        reset_last_trust_score_for_test();
        observe_last_trust_score(72);
        assert_eq!(last_trust_score(), 72);
        reset_last_trust_score_for_test();
    }

    #[test]
    fn configured_default_trust_score_is_constant_ph_s384() {
        assert_eq!(configured_default_trust_score(), 50);
    }

    #[test]
    fn gate_evaluations_and_default_score_counters_ph_s394_s395() {
        reset_settlement_gate_metrics_for_test();
        let cfg = TrustScoreGateConfig::default_stub();
        evaluate_result_settlement_gate(Some("tg-peer"), Some(80), &cfg);
        evaluate_result_settlement_gate(Some("tg-peer"), None, &cfg);
        evaluate_result_settlement_gate(Some("peer-local"), None, &cfg);
        assert_eq!(gate_evaluations_total(), 3);
        assert_eq!(default_score_applied_total(), 2);
        reset_settlement_gate_metrics_for_test();
    }

    #[test]
    fn configured_min_trust_for_payout_reads_env_ph_s374() {
        let prior = std::env::var(ENV_MIN_TRUST_PAYOUT).ok();
        std::env::set_var(ENV_MIN_TRUST_PAYOUT, "62");
        assert_eq!(configured_min_trust_for_payout(), 62);
        match prior {
            Some(v) => std::env::set_var(ENV_MIN_TRUST_PAYOUT, v),
            None => std::env::remove_var(ENV_MIN_TRUST_PAYOUT),
        }
    }
}
