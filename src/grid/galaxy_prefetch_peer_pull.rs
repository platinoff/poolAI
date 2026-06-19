//! Peer HTTP seed-pull prefetch wire (PH-S537, Galaxy §5.5).
//!
//! When `POOLAI_GALAXY_PREFETCH_PEER_HTTP_URL` is set, fetch seed inventory over HTTP
//! instead of the in-process coordinator snapshot stub.

use std::time::Duration;

use crate::grid::dispatch::{PrefetchPlan, SeedInventoryEntry};
use crate::grid::galaxy_prefetch_metrics::{
    record_prefetch_peer_fetch, record_prefetch_peer_fetch_miss,
};

/// Env: peer seed-inventory HTTP URL for live pull (PH-S537).
pub const ENV_PREFETCH_PEER_HTTP_URL: &str = "POOLAI_GALAXY_PREFETCH_PEER_HTTP_URL";

/// Env: HTTP timeout ms for peer seed pull (default 2000).
pub const ENV_PREFETCH_PEER_HTTP_TIMEOUT_MS: &str = "POOLAI_GALAXY_PREFETCH_PEER_HTTP_TIMEOUT_MS";

const DEFAULT_PEER_HTTP_TIMEOUT_MS: u64 = 2000;

fn peer_http_timeout() -> Duration {
    std::env::var(ENV_PREFETCH_PEER_HTTP_TIMEOUT_MS)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .filter(|&ms| ms > 0)
        .map(Duration::from_millis)
        .unwrap_or(Duration::from_millis(DEFAULT_PEER_HTTP_TIMEOUT_MS))
}

fn peer_http_url_from_env() -> Option<String> {
    std::env::var(ENV_PREFETCH_PEER_HTTP_URL)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Parse seed inventory entries from `GET /api/v1/grid/seed-inventory` JSON body.
pub fn parse_seed_inventory_http_body(body: &str) -> Option<Vec<SeedInventoryEntry>> {
    let v: serde_json::Value = serde_json::from_str(body).ok()?;
    let entries = v.get("entries")?.as_array()?;
    let mut out = Vec::new();
    for row in entries {
        let inv = row.get("seed_inventory")?;
        if let Ok(entry) = serde_json::from_value::<SeedInventoryEntry>(inv.clone()) {
            out.push(entry);
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

/// HTTP fetch seed inventory shard ids (blocking stub for sync prefetch hook).
pub fn fetch_peer_seed_inventory_http(url: &str) -> Option<Vec<SeedInventoryEntry>> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .ok()?;
    rt.block_on(async {
        let client = reqwest::Client::builder()
            .timeout(peer_http_timeout())
            .build()
            .ok()?;
        let resp = client.get(url).send().await.ok()?;
        if !resp.status().is_success() {
            return None;
        }
        let body = resp.text().await.ok()?;
        parse_seed_inventory_http_body(&body)
    })
}

/// Resolve shard hits from HTTP-fetched peer inventories (PH-S537).
pub fn fetch_seed_shards_from_peer_http(plan: &PrefetchPlan) -> usize {
    let Some(url) = peer_http_url_from_env() else {
        return 0;
    };
    let Some(inventories) = fetch_peer_seed_inventory_http(&url) else {
        for _ in &plan.items {
            record_prefetch_peer_fetch_miss();
        }
        return 0;
    };
    let mut hits = 0usize;
    for item in &plan.items {
        let found = inventories
            .iter()
            .any(|inv| inv.shard_ids.iter().any(|id| id == &item.shard_id));
        if found {
            hits += 1;
            record_prefetch_peer_fetch(1);
        } else {
            record_prefetch_peer_fetch_miss();
        }
    }
    hits
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::dispatch::{
        PrefetchPlan, PrefetchPlanItem, PrefetchPolicyMode, PrefetchTargetTier, PrefetchTrigger,
    };

    #[test]
    fn parse_seed_inventory_http_body_ph_s537() {
        let body = r#"{"ok":true,"entries":[{"peer_id":"p1","seed_inventory":{"shard_ids":["w:emb-1"],"hot_tier":{"ram_bytes_used":1,"vram_bytes_used":0,"profiles":[]},"local_replica_regions":[]}}]}"#;
        let inv = parse_seed_inventory_http_body(body).expect("parse");
        assert_eq!(inv[0].shard_ids, vec!["w:emb-1".to_string()]);
    }

    #[test]
    fn fetch_peer_http_miss_without_env_ph_s537() {
        std::env::remove_var(ENV_PREFETCH_PEER_HTTP_URL);
        let plan = PrefetchPlan {
            items: vec![PrefetchPlanItem {
                shard_id: "w:emb-1".into(),
                target_tier: PrefetchTargetTier::Ram,
            }],
            trigger: PrefetchTrigger::JobAdmitted,
            deadline_ms: 1000,
            mode: PrefetchPolicyMode::BestEffort,
        };
        assert_eq!(fetch_seed_shards_from_peer_http(&plan), 0);
    }
}
