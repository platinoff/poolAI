//! Galaxy worker heartbeat health stub (PH-S522, Galaxy §4.3.3 / §6.1).

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{LazyLock, Mutex};

/// Env: consecutive stale/miss marks before unhealthy (default `3`).
pub const ENV_HEARTBEAT_UNHEALTHY_THRESHOLD: &str = "POOLAI_GALAXY_HEARTBEAT_UNHEALTHY_THRESHOLD";

const DEFAULT_THRESHOLD: u32 = 3;

/// Prometheus metric name (mirrored on `GET /metrics`).
pub const METRIC_WORKER_UNHEALTHY_TOTAL: &str = "galaxy_worker_unhealthy_total";

static UNHEALTHY_TOTAL: AtomicU64 = AtomicU64::new(0);
static CONSECUTIVE_MISSES: LazyLock<Mutex<HashMap<String, u32>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static UNHEALTHY_PEERS: LazyLock<Mutex<HashMap<String, bool>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn unhealthy_threshold() -> u32 {
    std::env::var(ENV_HEARTBEAT_UNHEALTHY_THRESHOLD)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .filter(|&n| n > 0)
        .unwrap_or(DEFAULT_THRESHOLD)
}

pub fn galaxy_worker_unhealthy_total() -> u64 {
    UNHEALTHY_TOTAL.load(Ordering::Relaxed)
}

pub fn record_worker_unhealthy() {
    UNHEALTHY_TOTAL.fetch_add(1, Ordering::Relaxed);
}

/// Successful heartbeat clears miss streak and unhealthy flag.
pub fn on_heartbeat_success(peer_id: &str) {
    if let Ok(mut misses) = CONSECUTIVE_MISSES.lock() {
        misses.remove(peer_id);
    }
    if let Ok(mut unhealthy) = UNHEALTHY_PEERS.lock() {
        unhealthy.remove(peer_id);
    }
}

/// Stale peer or missed heartbeat expectation; returns true when newly marked unhealthy.
pub fn on_heartbeat_miss(peer_id: &str) -> bool {
    let threshold = unhealthy_threshold();
    let mut misses = CONSECUTIVE_MISSES.lock().unwrap_or_else(|e| e.into_inner());
    let count = misses.entry(peer_id.to_string()).or_insert(0);
    *count = count.saturating_add(1);
    if *count < threshold {
        return false;
    }
    let mut unhealthy = UNHEALTHY_PEERS.lock().unwrap_or_else(|e| e.into_inner());
    if unhealthy.insert(peer_id.to_string(), true).is_none() {
        record_worker_unhealthy();
        return true;
    }
    false
}

pub fn is_peer_unhealthy(peer_id: &str) -> bool {
    UNHEALTHY_PEERS
        .lock()
        .ok()
        .and_then(|g| g.get(peer_id).copied())
        .unwrap_or(false)
}

#[cfg(any(test, feature = "test-utils"))]
pub fn reset_worker_health_for_test() {
    UNHEALTHY_TOTAL.store(0, Ordering::Relaxed);
    if let Ok(mut m) = CONSECUTIVE_MISSES.lock() {
        m.clear();
    }
    if let Ok(mut u) = UNHEALTHY_PEERS.lock() {
        u.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consecutive_misses_mark_unhealthy_ph_s522() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        reset_worker_health_for_test();
        std::env::set_var(ENV_HEARTBEAT_UNHEALTHY_THRESHOLD, "3");
        assert!(!on_heartbeat_miss("peer-a"));
        assert!(!on_heartbeat_miss("peer-a"));
        assert!(on_heartbeat_miss("peer-a"));
        assert!(is_peer_unhealthy("peer-a"));
        assert_eq!(galaxy_worker_unhealthy_total(), 1);
        on_heartbeat_success("peer-a");
        assert!(!is_peer_unhealthy("peer-a"));
        std::env::remove_var(ENV_HEARTBEAT_UNHEALTHY_THRESHOLD);
        reset_worker_health_for_test();
    }
}
