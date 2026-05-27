//! Job lease TTL from environment (PH-S97, Galaxy §4.3.1 `lease_ttl` stub).
//!
//! Renew/failover wire — future sprints; this module only parses
//! `POOLAI_JOB_LEASE_TTL_SECS` for coordinator default TTL seconds.

/// Env: default lease TTL seconds when acquiring/renewing (Galaxy §4.3.1).
pub const ENV_JOB_LEASE_TTL_SECS: &str = "POOLAI_JOB_LEASE_TTL_SECS";

/// Default lease TTL — middle of Galaxy §4.3.1 range (30–120 s).
pub const DEFAULT_JOB_LEASE_TTL_SECS: u64 = 90;

/// Coordinator job-lease runtime config from environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JobLeaseConfig {
    pub lease_ttl_secs: u64,
}

impl Default for JobLeaseConfig {
    fn default() -> Self {
        Self {
            lease_ttl_secs: DEFAULT_JOB_LEASE_TTL_SECS,
        }
    }
}

impl JobLeaseConfig {
    pub fn from_env() -> Self {
        let lease_ttl_secs = env_u64(ENV_JOB_LEASE_TTL_SECS)
            .filter(|&v| v > 0)
            .unwrap_or(DEFAULT_JOB_LEASE_TTL_SECS);
        Self { lease_ttl_secs }
    }

    /// Recommended renew/heartbeat interval (`lease_ttl / 3`, Galaxy §4.3.1).
    pub fn lease_renew_interval_secs(&self) -> u64 {
        self.lease_ttl_secs / 3
    }
}

fn env_u64(name: &str) -> Option<u64> {
    std::env::var(name).ok().and_then(|v| v.trim().parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_lease_ttl_is_90_seconds() {
        let cfg = JobLeaseConfig::default();
        assert_eq!(cfg.lease_ttl_secs, 90);
        assert_eq!(cfg.lease_renew_interval_secs(), 30);
    }

    #[test]
    fn from_env_reads_poolai_job_lease_ttl_secs() {
        std::env::remove_var(ENV_JOB_LEASE_TTL_SECS);
        assert_eq!(
            JobLeaseConfig::from_env().lease_ttl_secs,
            DEFAULT_JOB_LEASE_TTL_SECS
        );

        std::env::set_var(ENV_JOB_LEASE_TTL_SECS, "120");
        let cfg = JobLeaseConfig::from_env();
        assert_eq!(cfg.lease_ttl_secs, 120);
        assert_eq!(cfg.lease_renew_interval_secs(), 40);

        std::env::set_var(ENV_JOB_LEASE_TTL_SECS, "not-a-number");
        assert_eq!(
            JobLeaseConfig::from_env().lease_ttl_secs,
            DEFAULT_JOB_LEASE_TTL_SECS
        );

        std::env::set_var(ENV_JOB_LEASE_TTL_SECS, "0");
        assert_eq!(
            JobLeaseConfig::from_env().lease_ttl_secs,
            DEFAULT_JOB_LEASE_TTL_SECS
        );

        std::env::remove_var(ENV_JOB_LEASE_TTL_SECS);
    }
}
