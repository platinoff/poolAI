//! Job lease TTL and renew interval from environment (PH-S97/S111, Galaxy §4.3.1).

/// Env: default lease TTL seconds when acquiring/renewing (Galaxy §4.3.1).
pub const ENV_JOB_LEASE_TTL_SECS: &str = "POOLAI_JOB_LEASE_TTL_SECS";

/// Env: optional renew/heartbeat interval override (default `lease_ttl_secs / 3`; PH-S111).
pub const ENV_JOB_LEASE_RENEW_INTERVAL_SECS: &str = "POOLAI_JOB_LEASE_RENEW_INTERVAL_SECS";

/// Default lease TTL — middle of Galaxy §4.3.1 range (30–120 s).
pub const DEFAULT_JOB_LEASE_TTL_SECS: u64 = 90;

/// Coordinator job-lease runtime config from environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JobLeaseConfig {
    pub lease_ttl_secs: u64,
    pub lease_renew_interval_secs: u64,
}

impl Default for JobLeaseConfig {
    fn default() -> Self {
        let lease_ttl_secs = DEFAULT_JOB_LEASE_TTL_SECS;
        Self {
            lease_ttl_secs,
            lease_renew_interval_secs: default_renew_interval_secs(lease_ttl_secs),
        }
    }
}

impl JobLeaseConfig {
    pub fn from_env() -> Self {
        let lease_ttl_secs = env_u64(ENV_JOB_LEASE_TTL_SECS)
            .filter(|&v| v > 0)
            .unwrap_or(DEFAULT_JOB_LEASE_TTL_SECS);
        let lease_renew_interval_secs =
            resolve_renew_interval_secs(lease_ttl_secs, env_u64(ENV_JOB_LEASE_RENEW_INTERVAL_SECS));
        Self {
            lease_ttl_secs,
            lease_renew_interval_secs,
        }
    }

    /// Recommended renew/heartbeat interval (Galaxy §4.3.1; from env or `lease_ttl / 3`).
    pub fn lease_renew_interval_secs(&self) -> u64 {
        self.lease_renew_interval_secs
    }
}

fn default_renew_interval_secs(lease_ttl_secs: u64) -> u64 {
    lease_ttl_secs / 3
}

fn resolve_renew_interval_secs(lease_ttl_secs: u64, env_override: Option<u64>) -> u64 {
    let fallback = default_renew_interval_secs(lease_ttl_secs);
    let Some(raw) = env_override.filter(|&v| v > 0) else {
        return fallback;
    };
    raw.min(lease_ttl_secs)
}

fn env_u64(name: &str) -> Option<u64> {
    std::env::var(name).ok().and_then(|v| v.trim().parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard};

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn lease_env_lock() -> MutexGuard<'static, ()> {
        ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn clear_lease_env() {
        std::env::remove_var(ENV_JOB_LEASE_TTL_SECS);
        std::env::remove_var(ENV_JOB_LEASE_RENEW_INTERVAL_SECS);
    }

    #[test]
    fn default_lease_ttl_is_90_seconds() {
        let cfg = JobLeaseConfig::default();
        assert_eq!(cfg.lease_ttl_secs, 90);
        assert_eq!(cfg.lease_renew_interval_secs(), 30);
    }

    #[test]
    fn from_env_reads_poolai_job_lease_ttl_secs() {
        let _guard = lease_env_lock();
        clear_lease_env();
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

        clear_lease_env();
    }

    #[test]
    fn from_env_reads_poolai_job_lease_renew_interval_secs() {
        let _guard = lease_env_lock();
        clear_lease_env();
        std::env::set_var(ENV_JOB_LEASE_TTL_SECS, "90");
        std::env::set_var(ENV_JOB_LEASE_RENEW_INTERVAL_SECS, "15");
        let cfg = JobLeaseConfig::from_env();
        assert_eq!(cfg.lease_renew_interval_secs(), 15);

        std::env::set_var(ENV_JOB_LEASE_RENEW_INTERVAL_SECS, "200");
        assert_eq!(JobLeaseConfig::from_env().lease_renew_interval_secs(), 90);

        std::env::set_var(ENV_JOB_LEASE_RENEW_INTERVAL_SECS, "0");
        assert_eq!(JobLeaseConfig::from_env().lease_renew_interval_secs(), 30);

        std::env::set_var(ENV_JOB_LEASE_RENEW_INTERVAL_SECS, "bad");
        assert_eq!(JobLeaseConfig::from_env().lease_renew_interval_secs(), 30);

        clear_lease_env();
    }
}
