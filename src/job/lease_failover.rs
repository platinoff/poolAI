//! Job lease failover retry budget (PH-S518, Galaxy §4.3.3).

use serde::{Deserialize, Serialize};

/// Env: max lease-timeout re-migrations before job fails (default `3`).
pub const ENV_JOB_MAX_MIGRATIONS_PER_JOB: &str = "POOLAI_JOB_MAX_MIGRATIONS_PER_JOB";

const DEFAULT_MAX_MIGRATIONS: u32 = 3;

/// Fail reason codes on lease failover path (Galaxy §4.3.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LeaseFailReason {
    LeaseTimeout,
    BudgetExhausted,
}

impl LeaseFailReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LeaseTimeout => "lease-timeout",
            Self::BudgetExhausted => "budget-exhausted",
        }
    }
}

/// Configured max migrations per job from env.
pub fn max_migrations_per_job() -> u32 {
    std::env::var(ENV_JOB_MAX_MIGRATIONS_PER_JOB)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .filter(|&n| n > 0)
        .unwrap_or(DEFAULT_MAX_MIGRATIONS)
}

/// Next migration count after one lease-timeout requeue attempt.
pub fn next_migration_count(current: u32) -> u32 {
    current.saturating_add(1)
}

/// Whether migration budget is exhausted after increment.
pub fn migration_budget_exhausted(migration_count: u32) -> bool {
    migration_count >= max_migrations_per_job()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_migrations_default_ph_s518() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        std::env::remove_var(ENV_JOB_MAX_MIGRATIONS_PER_JOB);
        assert_eq!(max_migrations_per_job(), 3);
    }

    #[test]
    fn migration_budget_exhausted_ph_s518() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        std::env::set_var(ENV_JOB_MAX_MIGRATIONS_PER_JOB, "2");
        assert!(!migration_budget_exhausted(1));
        assert!(migration_budget_exhausted(2));
        std::env::remove_var(ENV_JOB_MAX_MIGRATIONS_PER_JOB);
    }

    #[test]
    fn fail_reason_wire_labels() {
        assert_eq!(LeaseFailReason::LeaseTimeout.as_str(), "lease-timeout");
        assert_eq!(
            LeaseFailReason::BudgetExhausted.as_str(),
            "budget-exhausted"
        );
    }
}
