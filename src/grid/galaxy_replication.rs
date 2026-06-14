//! Galaxy Grid replication verification tier config (PH-S171, §6.4).
//!
//! N-of-M quorum profiles for high-value jobs; no parallel executor enqueue wire.

/// Replication verification profile (Galaxy §6.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicationProfile {
    Light,
    Standard,
    /// Financial / settlement-critical — 3-of-3 digest match.
    Strict,
}

/// Coordinator replication tier parameters (M executors, K quorum).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplicationTierConfig {
    pub profile: ReplicationProfile,
    pub executors_m: u8,
    pub quorum_k: u8,
}

/// `replication_light`: 2-of-2 (mining probe, small batch).
pub const REPLICATION_LIGHT: ReplicationTierConfig = ReplicationTierConfig {
    profile: ReplicationProfile::Light,
    executors_m: 2,
    quorum_k: 2,
};

/// `replication_standard`: 2-of-3 (high gross inference).
pub const REPLICATION_STANDARD: ReplicationTierConfig = ReplicationTierConfig {
    profile: ReplicationProfile::Standard,
    executors_m: 3,
    quorum_k: 2,
};

/// `replication_strict`: 3-of-3 (settlement-critical).
pub const REPLICATION_STRICT: ReplicationTierConfig = ReplicationTierConfig {
    profile: ReplicationProfile::Strict,
    executors_m: 3,
    quorum_k: 3,
};

/// Map profile → tier config (Galaxy §6.4 table).
#[inline]
pub fn replication_tier_config(profile: ReplicationProfile) -> ReplicationTierConfig {
    match profile {
        ReplicationProfile::Light => REPLICATION_LIGHT,
        ReplicationProfile::Standard => REPLICATION_STANDARD,
        ReplicationProfile::Strict => REPLICATION_STRICT,
    }
}

/// Parse `verification_policy` / job profile string to replication profile.
pub fn parse_replication_profile(raw: &str) -> Option<ReplicationProfile> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "replication_light" | "replicate-2" => Some(ReplicationProfile::Light),
        "replication_standard" | "replicate-3" | "replication" => {
            Some(ReplicationProfile::Standard)
        }
        "replication_strict" | "strict_verification" => Some(ReplicationProfile::Strict),
        _ => None,
    }
}

/// Resolve tier from optional grid job `verification_policy`; missing/unknown → [`REPLICATION_STANDARD`].
pub fn replication_tier_from_policy(policy: Option<&str>) -> ReplicationTierConfig {
    policy
        .and_then(parse_replication_profile)
        .map(replication_tier_config)
        .unwrap_or(REPLICATION_STANDARD)
}

/// §6.4 guardrail: `replication_strict` disallowed for telegram_edge-only executor pools.
#[inline]
pub fn strict_tier_allows_worker_pool(edge_only: bool, config: ReplicationTierConfig) -> bool {
    !(edge_only && config.profile == ReplicationProfile::Strict)
}

/// Stub quorum check: accept when the mode digest count ≥ `quorum_k` (first `executors_m` digests considered).
pub fn replication_quorum_met(digests: &[&str], config: ReplicationTierConfig) -> bool {
    if digests.len() < config.executors_m as usize {
        return false;
    }
    let window = &digests[..config.executors_m as usize];
    let mut counts = std::collections::HashMap::new();
    for &d in window {
        *counts.entry(d).or_insert(0usize) += 1;
    }
    counts.values().copied().max().unwrap_or(0) >= config.quorum_k as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replication_strict_tier_is_three_of_three_ph_s171() {
        assert_eq!(REPLICATION_STRICT.executors_m, 3);
        assert_eq!(REPLICATION_STRICT.quorum_k, 3);
        assert_eq!(
            replication_tier_config(ReplicationProfile::Strict),
            REPLICATION_STRICT
        );
    }

    #[test]
    fn parse_replication_profile_aliases() {
        assert_eq!(
            parse_replication_profile("replication_strict"),
            Some(ReplicationProfile::Strict)
        );
        assert_eq!(
            parse_replication_profile("strict_verification"),
            Some(ReplicationProfile::Strict)
        );
        assert_eq!(
            parse_replication_profile("replicate-3"),
            Some(ReplicationProfile::Standard)
        );
        assert!(parse_replication_profile("unknown").is_none());
    }

    #[test]
    fn replication_tier_from_policy_defaults_standard() {
        assert_eq!(replication_tier_from_policy(None), REPLICATION_STANDARD);
        assert_eq!(
            replication_tier_from_policy(Some("replication_strict")),
            REPLICATION_STRICT
        );
    }

    #[test]
    fn strict_tier_disallows_edge_only_pool_ph_s171() {
        assert!(!strict_tier_allows_worker_pool(true, REPLICATION_STRICT));
        assert!(strict_tier_allows_worker_pool(false, REPLICATION_STRICT));
        assert!(strict_tier_allows_worker_pool(true, REPLICATION_STANDARD));
    }

    #[test]
    fn replication_quorum_met_strict_requires_unanimous() {
        let cfg = REPLICATION_STRICT;
        assert!(replication_quorum_met(&["a", "a", "a"], cfg));
        assert!(!replication_quorum_met(&["a", "a", "b"], cfg));
        assert!(!replication_quorum_met(&["a", "b"], cfg));
    }

    #[test]
    fn replication_quorum_met_standard_two_of_three() {
        let cfg = REPLICATION_STANDARD;
        assert!(replication_quorum_met(&["a", "a", "b"], cfg));
        assert!(!replication_quorum_met(&["a", "b", "c"], cfg));
    }
}
