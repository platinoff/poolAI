//! JWT signing secrets loaded from environment with optional previous key during rotation.

use parking_lot::RwLock;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

const ENV_JWT_SECRET: &str = "POOLAI_JWT_SECRET";
const ENV_JWT_PREVIOUS: &str = "POOLAI_JWT_SECRET_PREVIOUS";
const ENV_JWT_GRACE_SECS: &str = "POOLAI_JWT_ROTATION_GRACE_SECS";
const DEFAULT_DEV_SECRET: &str = "your-super-secret-key-change-in-production";
const DEFAULT_GRACE_SECS: u64 = 86_400;

#[derive(Debug, Clone)]
pub struct JwtSecretState {
    pub current: String,
    pub previous: Option<String>,
    pub grace_until_unix: Option<u64>,
    pub loaded_at_unix: u64,
}

impl JwtSecretState {
    pub fn grace_active(&self) -> bool {
        let Some(until) = self.grace_until_unix else {
            return self.previous.is_some();
        };
        let now = unix_now();
        now <= until && self.previous.is_some()
    }

    /// Candidate secrets for decode (current first, then previous during grace).
    pub fn decoding_secrets(&self) -> Vec<&str> {
        let mut out = vec![self.current.as_str()];
        if self.grace_active() {
            if let Some(prev) = self.previous.as_ref() {
                if prev != &self.current {
                    out.push(prev.as_str());
                }
            }
        }
        out
    }
}

static JWT_STORE: OnceLock<RwLock<JwtSecretState>> = OnceLock::new();

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn grace_secs_from_env() -> u64 {
    std::env::var(ENV_JWT_GRACE_SECS)
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|&n| n > 0)
        .unwrap_or(DEFAULT_GRACE_SECS)
}

/// Load JWT secrets from environment (current + optional previous + grace window).
pub fn load_from_env() -> JwtSecretState {
    let current = std::env::var(ENV_JWT_SECRET).unwrap_or_else(|_| DEFAULT_DEV_SECRET.to_string());
    let previous = std::env::var(ENV_JWT_PREVIOUS)
        .ok()
        .filter(|s| !s.is_empty());
    let grace_until_unix = previous
        .as_ref()
        .map(|_| unix_now().saturating_add(grace_secs_from_env()));
    JwtSecretState {
        current,
        previous,
        grace_until_unix,
        loaded_at_unix: unix_now(),
    }
}

pub fn jwt_store() -> &'static RwLock<JwtSecretState> {
    JWT_STORE.get_or_init(|| RwLock::new(load_from_env()))
}

/// Reload JWT secrets from environment (rotation hook).
pub fn reload_jwt_from_env() -> Result<(), String> {
    *jwt_store().write() = load_from_env();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decoding_secrets_includes_previous_during_grace() {
        let state = JwtSecretState {
            current: "new-secret".into(),
            previous: Some("old-secret".into()),
            grace_until_unix: Some(unix_now() + 3600),
            loaded_at_unix: unix_now(),
        };
        let keys = state.decoding_secrets();
        assert_eq!(keys.len(), 2);
        assert_eq!(keys[0], "new-secret");
        assert_eq!(keys[1], "old-secret");
    }
}
