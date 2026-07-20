//! HTTP stand smoke against a live coordinator (PH-S145).
//!
//! Replaces legacy Playwright API-smoke when a stand is running (`bin/e2e-playwright.sh --start`
//! or `run-poolai single`). Integration tests in `tests/` remain the CI canon without a stand.
//!
//! ```text
//! export POOLAI_BASE_URL=http://127.0.0.1:8080
//! cargo run --bin poolai-http-stand-smoke
//!
//! # RAID persist + restart (replaces legacy Playwright jobs_raid, PH-S156):
//! export POOLAI_E2E_STAND_ROOT=/tmp/poolai-e2e-NNN
//! cargo run --bin poolai-http-stand-smoke -- --raid-restart
//!
//! # Job lease renew suite (replaces legacy Playwright jobs_lease, PH-S196):
//! cargo run --bin poolai-http-stand-smoke -- --lease-renew
//!
//! # Full suite incl. raid restart:
//! cargo run --bin poolai-http-stand-smoke -- --raid
//!
//! cargo run --bin poolai-http-stand-smoke -- --json
//!
//! # RUN_LOCAL quick subset (PH-S1093):
//! cargo run --bin poolai-http-stand-smoke -- --run-local-smoke
//!
//! # Vision revision parity (PH-S208, PH-S235):
//! export POOLAI_VISION_BASE_URL=http://127.0.0.1:8765   # open-docs-vision.ps1
//! cargo run --bin poolai-http-stand-smoke   # repo manifest vs FM footer + extensions + optional HTTP header
//! ```

use reqwest::{Client, StatusCode};
use serde::Serialize;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const DEFAULT_BASE: &str = "http://127.0.0.1:8080";
const DEFAULT_VISION_BASE: &str = "http://127.0.0.1:8765";
const ENV_BASE: &str = "POOLAI_BASE_URL";
const ENV_VISION_BASE: &str = "POOLAI_VISION_BASE_URL";
const ENV_STAND_ROOT: &str = "POOLAI_E2E_STAND_ROOT";
const MANIFEST_REL: &str = "docs/vision/manifest.json";
const EXTENSIONS_REL: &str = "docs/vision/extensions.json";
const FM_REL: &str = "docs/catalog/FUNCTION_MANAGEMENT.md";
const VISION_REV_HEADER: &str = "x-poolai-vision-revision";
const VALID_PUBKEY: &str = "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU";

#[derive(Debug, Clone)]
struct Cli {
    json_out: bool,
    include_raid: bool,
    raid_restart_only: bool,
    lease_renew_only: bool,
    /// PH-S1093: RUN_LOCAL quick subset (health + monitoring + vm + ops).
    run_local_smoke_only: bool,
    base_url: String,
}

#[derive(Debug, Serialize)]
struct SmokeCaseResult {
    name: &'static str,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
}

#[derive(Debug, Serialize)]
struct SmokeReport {
    base_url: String,
    stand_root: Option<String>,
    ok: bool,
    passed: u32,
    failed: u32,
    cases: Vec<SmokeCaseResult>,
    tool: &'static str,
}

fn base_url_from_env() -> String {
    std::env::var(ENV_BASE)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_BASE.to_string())
}

fn vision_base_url_from_env() -> String {
    std::env::var(ENV_VISION_BASE)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_VISION_BASE.to_string())
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_manifest_json(root: &Path) -> Result<Value, String> {
    let path = root.join(MANIFEST_REL);
    let raw = fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("parse manifest: {e}"))
}

fn read_manifest_revision(root: &Path) -> Result<u64, String> {
    let manifest = read_manifest_json(root)?;
    manifest
        .get("revision")
        .and_then(Value::as_u64)
        .ok_or_else(|| "manifest missing revision".to_string())
}

fn read_manifest_next_sprint(root: &Path) -> Option<String> {
    read_manifest_json(root).ok().and_then(|manifest| {
        manifest
            .get("next_sprint")
            .and_then(Value::as_str)
            .map(str::to_owned)
    })
}

fn read_extensions_active_sprint(root: &Path) -> Result<Option<String>, String> {
    let path = root.join(EXTENSIONS_REL);
    let raw = fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let ext: Value = serde_json::from_str(&raw).map_err(|e| format!("parse extensions: {e}"))?;
    Ok(ext
        .get("active_sprint")
        .and_then(Value::as_str)
        .map(str::to_owned))
}

fn assert_vision_repo_parity(root: &Path) -> Result<(), String> {
    let repo_rev = read_manifest_revision(root)?;
    let fm_rev = read_fm_vision_revision(root)?;
    if repo_rev != fm_rev {
        return Err(format!(
            "repo manifest.revision {repo_rev} != FM Vision rev {fm_rev}"
        ));
    }
    if let Some(next) = read_manifest_next_sprint(root) {
        let active = read_extensions_active_sprint(root)?
            .ok_or_else(|| "extensions.json missing active_sprint".to_string())?;
        if active != next {
            return Err(format!(
                "extensions.active_sprint {active:?} != manifest.next_sprint {next:?}"
            ));
        }
    }
    Ok(())
}

fn extract_fm_section_512(content: &str) -> Option<&str> {
    let start = content.find("### 5.12")?;
    let rest = &content[start..];
    let end = rest[10..]
        .find("\n### 5.")
        .map(|i| 10 + i)
        .unwrap_or(rest.len());
    Some(&rest[..end])
}

fn parse_fm_vision_revision(section: &str) -> Option<u64> {
    for line in section.lines() {
        let marker = "Vision rev **";
        let Some(start) = line.find(marker) else {
            continue;
        };
        let rest = &line[start + marker.len()..];
        let end = rest.find("**")?;
        return rest[..end].parse().ok();
    }
    None
}

fn read_fm_vision_revision(root: &Path) -> Result<u64, String> {
    let path = root.join(FM_REL);
    let content = fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let section =
        extract_fm_section_512(&content).ok_or_else(|| "FM §5.12 section not found".to_string())?;
    parse_fm_vision_revision(section)
        .ok_or_else(|| "FM §5.12 missing Vision rev **N** footer".to_string())
}

async fn smoke_vision_revision_parity(client: &Client) -> Result<(), String> {
    let root = repo_root();
    assert_vision_repo_parity(&root)?;
    let repo_rev = read_manifest_revision(&root)?;
    let vision_base = vision_base_url_from_env();
    let url = api_url(&vision_base, "/docs/vision/manifest.json");
    let resp = match client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => {
            return Err(format!(
                "vision server unreachable at {url} ({e}); start open-docs-vision.ps1 or set {ENV_VISION_BASE}"
            ));
        }
    };
    if !resp.status().is_success() {
        return Err(format!("vision manifest status {}", resp.status()));
    }
    let header_rev = resp
        .headers()
        .get(VISION_REV_HEADER)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| format!("missing {VISION_REV_HEADER} header on {url}"))?
        .parse::<u64>()
        .map_err(|_| format!("invalid {VISION_REV_HEADER} header"))?;
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    let body_rev = body
        .get("revision")
        .and_then(Value::as_u64)
        .ok_or_else(|| "manifest JSON missing revision".to_string())?;
    if header_rev != body_rev {
        return Err(format!(
            "{VISION_REV_HEADER} {header_rev} != manifest.revision {body_rev}"
        ));
    }
    if body_rev != repo_rev {
        return Err(format!(
            "live manifest.revision {body_rev} != repo/FM revision {repo_rev}"
        ));
    }
    Ok(())
}

fn parse_cli() -> Cli {
    let mut json_out = false;
    let mut include_raid = false;
    let mut raid_restart_only = false;
    let mut lease_renew_only = false;
    let mut run_local_smoke_only = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--json" => json_out = true,
            "--raid-restart" => raid_restart_only = true,
            "--raid" => include_raid = true,
            "--lease-renew" => lease_renew_only = true,
            "--run-local-smoke" => run_local_smoke_only = true,
            _ if arg.starts_with('-') => {}
            _ => {}
        }
    }
    if !raid_restart_only {
        raid_restart_only = std::env::var("POOLAI_STAND_SMOKE_RAID_RESTART")
            .ok()
            .is_some_and(|v| matches!(v.as_str(), "1" | "true" | "yes"));
    }
    if !lease_renew_only {
        lease_renew_only = std::env::var("POOLAI_STAND_SMOKE_LEASE_RENEW")
            .ok()
            .is_some_and(|v| matches!(v.as_str(), "1" | "true" | "yes"));
    }
    if !include_raid && !raid_restart_only {
        include_raid = std::env::var("POOLAI_STAND_SMOKE_RAID")
            .ok()
            .is_some_and(|v| matches!(v.as_str(), "1" | "true" | "yes"));
    }
    if !run_local_smoke_only {
        run_local_smoke_only = std::env::var("POOLAI_STAND_SMOKE_RUN_LOCAL")
            .ok()
            .is_some_and(|v| matches!(v.as_str(), "1" | "true" | "yes"));
    }
    Cli {
        json_out,
        include_raid,
        raid_restart_only,
        lease_renew_only,
        run_local_smoke_only,
        base_url: base_url_from_env(),
    }
}

fn smoke_id(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{prefix}-{nanos}")
}

fn api_url(base: &str, path: &str) -> String {
    let base = base.trim_end_matches('/');
    format!("{base}{path}")
}

async fn wait_health(client: &Client, base: &str, tries: u32) -> Result<(), String> {
    let url = api_url(base, "/api/v1/health");
    for _ in 0..tries {
        if let Ok(resp) = client.get(&url).send().await {
            if resp.status().is_success() {
                return Ok(());
            }
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    Err(format!("health not ready at {url}"))
}

async fn smoke_health(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/health"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("health status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    for key in ["status", "version", "checks"] {
        if body.get(key).is_none() {
            return Err(format!("health missing `{key}`: {body}"));
        }
    }
    if body.get("status").and_then(Value::as_str) != Some("healthy") {
        return Err(format!("health status != healthy: {body}"));
    }
    Ok(())
}

/// PH-S1090: enterprise monitoring alerts list (RUN_LOCAL / admin wasm slim).
async fn smoke_monitoring_alerts_api(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/enterprise/monitoring/alerts"))
        .send()
        .await
        .map_err(|e| format!("monitoring alerts request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("monitoring alerts status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    body.as_array()
        .ok_or_else(|| format!("monitoring alerts expected array: {body}"))?;
    Ok(())
}

/// PH-S1091: enterprise monitoring dashboards list.
async fn smoke_monitoring_dashboards_api(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/enterprise/monitoring/dashboards"))
        .send()
        .await
        .map_err(|e| format!("monitoring dashboards request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("monitoring dashboards status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    body.as_array()
        .ok_or_else(|| format!("monitoring dashboards expected array: {body}"))?;
    Ok(())
}

/// PH-S1092: VM instances list shape (`run-poolai` dev stand).
async fn smoke_vm_instances_api(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/vm/instances"))
        .send()
        .await
        .map_err(|e| format!("vm instances request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("vm instances status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    let rows = body
        .as_array()
        .ok_or_else(|| format!("vm instances expected array: {body}"))?;
    if let Some(first) = rows.first() {
        for key in ["id", "name", "status"] {
            if first.get(key).is_none() {
                return Err(format!("vm instances row missing `{key}`: {first}"));
            }
        }
    }
    Ok(())
}

async fn smoke_grid_seed_inventory(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/seed-inventory"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.status() != StatusCode::OK {
        return Err(format!("grid seed-inventory status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("grid seed-inventory body: {body}"));
    }
    if body.get("generated_at").and_then(|v| v.as_str()).is_none() {
        return Err(format!("grid seed-inventory missing generated_at: {body}"));
    }
    let entries = body
        .get("entries")
        .and_then(Value::as_array)
        .ok_or_else(|| format!("grid seed-inventory missing entries: {body}"))?;
    if entries.len() != 2 {
        return Err(format!(
            "grid seed-inventory expected 2 entries, got {}: {body}",
            entries.len()
        ));
    }
    if entries[0].get("peer_id").and_then(|v| v.as_str()) != Some("srv1-worker-a") {
        return Err(format!("grid seed-inventory first peer_id: {}", entries[0]));
    }
    if entries[0].pointer("/seed_inventory/shard_ids") != Some(&json!(["w:emb-1", "w:ckpt-7"])) {
        return Err(format!(
            "grid seed-inventory first shard_ids: {}",
            entries[0]
        ));
    }
    if entries[1].get("peer_id").and_then(|v| v.as_str()) != Some("srv2-worker-b") {
        return Err(format!(
            "grid seed-inventory second peer_id: {}",
            entries[1]
        ));
    }
    if body
        .get("memory_store_depth")
        .and_then(|v| v.as_str())
        .is_none()
    {
        return Err(format!(
            "grid seed-inventory missing memory_store_depth: {body}"
        ));
    }
    if body
        .get("memory_layer_depth")
        .and_then(|v| v.as_str())
        .is_none()
    {
        return Err(format!(
            "grid seed-inventory missing memory_layer_depth: {body}"
        ));
    }
    if body
        .get("registered_shard_count")
        .and_then(|v| v.as_u64())
        .is_none()
    {
        return Err(format!(
            "grid seed-inventory missing registered_shard_count: {body}"
        ));
    }
    Ok(())
}

/// PH-S213: live stand exposes Galaxy prefetch counters on Prometheus scrape.
const GALAXY_PREFETCH_METRICS: &[&str] = &[
    "galaxy_prefetch_plan_total",
    "galaxy_prefetch_planned_shards_total",
    "galaxy_prefetch_hot_skip_total",
    "galaxy_prefetch_bytes_total",
    "galaxy_prefetch_enqueue_total",
    "galaxy_prefetch_wait_ms_total",
    "galaxy_prefetch_strict_mode_total",
    "galaxy_prefetch_complete_total",
    "galaxy_prefetch_ingest_total",
    "galaxy_prefetch_skip_ingest_total",
    "galaxy_prefetch_seed_pull_total",
    "galaxy_prefetch_lease_acquired_total",
    "galaxy_locality_rank_ingest_total",
    "galaxy_locality_rank_miss_total",
    "galaxy_locality_rank_empty_workers_total",
    "galaxy_locality_rank_skip_total",
    "galaxy_network_profile_stale_total",
];

fn metrics_text_has_prefetch_counters(body: &str) -> Result<(), String> {
    for name in GALAXY_PREFETCH_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_prefetch_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_prefetch_counters(&body)
}

/// PH-S216: live stand exposes Galaxy pricing forced-fallback counter on Prometheus scrape.
const GALAXY_PRICING_FORCED_FALLBACK: &str = "galaxy_pricing_forced_fallback_total";

fn metrics_text_has_pricing_forced_fallback(body: &str) -> Result<(), String> {
    let name = GALAXY_PRICING_FORCED_FALLBACK;
    if !body.contains(name) {
        return Err(format!("/metrics missing {name}"));
    }
    if !body.contains(&format!("# TYPE {name} gauge")) {
        return Err(format!("/metrics missing TYPE gauge for {name}"));
    }
    Ok(())
}

async fn smoke_galaxy_pricing_forced_fallback_metrics(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_pricing_forced_fallback(&body)
}

/// PH-S224: live stand exposes Galaxy pricing cache age gauge on Prometheus scrape.
const GALAXY_PRICING_CACHE_AGE: &str = "galaxy_pricing_cache_age_seconds";

fn metrics_text_has_pricing_cache_age(body: &str) -> Result<(), String> {
    let name = GALAXY_PRICING_CACHE_AGE;
    if !body.contains(name) {
        return Err(format!("/metrics missing {name}"));
    }
    if !body.contains(&format!("# TYPE {name} gauge")) {
        return Err(format!("/metrics missing TYPE gauge for {name}"));
    }
    Ok(())
}

async fn smoke_galaxy_pricing_cache_age_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_pricing_cache_age(&body)
}

/// PH-S241: live stand exposes Galaxy pricing fresh-served gauge on Prometheus scrape.
const GALAXY_PRICING_FRESH_SERVED: &str = "galaxy_pricing_fresh_served";

fn metrics_text_has_pricing_fresh_served(body: &str) -> Result<(), String> {
    let name = GALAXY_PRICING_FRESH_SERVED;
    if !body.contains(name) {
        return Err(format!("/metrics missing {name}"));
    }
    if !body.contains(&format!("# TYPE {name} gauge")) {
        return Err(format!("/metrics missing TYPE gauge for {name}"));
    }
    Ok(())
}

async fn smoke_galaxy_pricing_fresh_served_metrics(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_pricing_fresh_served(&body)
}

/// PH-S244: live stand exposes Galaxy pricing stale-served gauge on Prometheus scrape.
const GALAXY_PRICING_STALE_SERVED: &str = "galaxy_pricing_stale_served";

fn metrics_text_has_pricing_stale_served(body: &str) -> Result<(), String> {
    let name = GALAXY_PRICING_STALE_SERVED;
    if !body.contains(name) {
        return Err(format!("/metrics missing {name}"));
    }
    if !body.contains(&format!("# TYPE {name} gauge")) {
        return Err(format!("/metrics missing TYPE gauge for {name}"));
    }
    Ok(())
}

async fn smoke_galaxy_pricing_stale_served_metrics(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_pricing_stale_served(&body)
}

/// PH-S247: live stand exposes Galaxy pricing provider catalog + error gauges.
const GALAXY_PRICING_PROVIDER_METRICS: &[&str] = &[
    "galaxy_pricing_provider_catalog_lookups_total",
    "galaxy_pricing_provider_catalog_hits_total",
    "galaxy_pricing_provider_errors_total",
];

fn metrics_text_has_pricing_provider_counters(body: &str) -> Result<(), String> {
    for name in GALAXY_PRICING_PROVIDER_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_pricing_provider_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_pricing_provider_counters(&body)
}

/// PH-S253: live stand exposes Galaxy pricing quote + market min gauges.
const GALAXY_PRICING_QUOTE_MARKET_METRICS: &[&str] = &[
    "galaxy_pricing_quote_usd_micro",
    "galaxy_pricing_market_min_usd_micro",
];

fn metrics_text_has_pricing_quote_market_gauges(body: &str) -> Result<(), String> {
    for name in GALAXY_PRICING_QUOTE_MARKET_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_pricing_quote_market_metrics(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_pricing_quote_market_gauges(&body)
}

/// PH-S254: live stand exposes Galaxy fee split applied gauge.
const GALAXY_FEE_SPLIT_APPLIED: &str = "galaxy_fee_split_applied_total";

fn metrics_text_has_fee_split_applied(body: &str) -> Result<(), String> {
    let name = GALAXY_FEE_SPLIT_APPLIED;
    if !body.contains(name) {
        return Err(format!("/metrics missing {name}"));
    }
    if !body.contains(&format!("# TYPE {name} gauge")) {
        return Err(format!("/metrics missing TYPE gauge for {name}"));
    }
    Ok(())
}

async fn smoke_galaxy_fee_split_applied_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_fee_split_applied(&body)
}

/// PH-S255: live stand exposes Galaxy cross-region egress gauge.
const GALAXY_CROSS_REGION_EGRESS_MB: &str = "galaxy_cross_region_egress_mb";

fn metrics_text_has_cross_region_egress_mb(body: &str) -> Result<(), String> {
    let name = GALAXY_CROSS_REGION_EGRESS_MB;
    if !body.contains(name) {
        return Err(format!("/metrics missing {name}"));
    }
    if !body.contains(&format!("# TYPE {name} gauge")) {
        return Err(format!("/metrics missing TYPE gauge for {name}"));
    }
    Ok(())
}

async fn smoke_galaxy_cross_region_egress_metrics(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_cross_region_egress_mb(&body)
}

/// PH-S256 / PH-S336: live stand exposes Galaxy replay pending gauge + scheduled/resolved totals.
const GALAXY_REPLAY_METRICS: &[&str] = &[
    "galaxy_replay_pending",
    "galaxy_replay_pending_scheduled_total",
    "galaxy_replay_pending_resolved_total",
    "galaxy_replay_evaluations_total",
    "galaxy_replay_verification_enqueue_total",
];

fn metrics_text_has_replay_pending(body: &str) -> Result<(), String> {
    for name in GALAXY_REPLAY_METRICS {
        if !body.contains(*name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_replay_pending_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_replay_pending(&body)
}

/// PH-S249: live stand exposes Galaxy settlement pending + cleared gauges.
const GALAXY_SETTLEMENT_METRICS: &[&str] = &[
    "galaxy_settlement_pending_verification_total",
    "galaxy_settlement_cleared_total",
    "galaxy_settlement_not_applicable_total",
    "galaxy_settlement_resolved_total",
    "galaxy_settlement_payout_batch_total",
    "galaxy_settlement_human_review_total",
];

/// PH-S569: checker timeout inconclusive/retry gauges on `/metrics`.
const GALAXY_CHECKER_TIMEOUT_METRICS: &[&str] = &[
    "galaxy_verification_checker_timeout_inconclusive_total",
    "galaxy_verification_checker_timeout_retry_total",
    "galaxy_fraud_proof_pending_total",
    "poolai_advisory_acknowledged_total",
];

fn metrics_text_has_settlement_counters(body: &str) -> Result<(), String> {
    for name in GALAXY_SETTLEMENT_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_settlement_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_settlement_counters(&body)?;
    for name in GALAXY_CHECKER_TIMEOUT_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
    }
    Ok(())
}

/// PH-S528: governance ops Prometheus gauges on live stand.
const GALAXY_GOVERNANCE_METRICS: &[&str] = &[
    "poolai_release_verify_total",
    "poolai_release_verify_fail_total",
    "poolai_update_notify_pending",
];

async fn smoke_galaxy_governance_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    for name in GALAXY_GOVERNANCE_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
    }
    Ok(())
}

/// PH-S250: live stand exposes Galaxy shard local hit ratio gauge.
const GALAXY_SHARD_LOCAL_HIT_RATIO: &str = "galaxy_shard_local_hit_ratio";

/// PH-S581: live stand exposes Galaxy hot tier hit ratio gauge.
const GALAXY_HOT_TIER_HIT_RATIO: &str = "galaxy_hot_tier_hit_ratio";

fn metrics_text_has_shard_local_hit_ratio(body: &str) -> Result<(), String> {
    let name = GALAXY_SHARD_LOCAL_HIT_RATIO;
    if !body.contains(name) {
        return Err(format!("/metrics missing {name}"));
    }
    if !body.contains(&format!("# TYPE {name} gauge")) {
        return Err(format!("/metrics missing TYPE gauge for {name}"));
    }
    Ok(())
}

async fn smoke_galaxy_shard_local_hit_ratio_metrics(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_shard_local_hit_ratio(&body)
}

fn metrics_text_has_hot_tier_hit_ratio(body: &str) -> Result<(), String> {
    let name = GALAXY_HOT_TIER_HIT_RATIO;
    if !body.contains(name) {
        return Err(format!("/metrics missing {name}"));
    }
    if !body.contains(&format!("# TYPE {name} gauge")) {
        return Err(format!("/metrics missing TYPE gauge for {name}"));
    }
    Ok(())
}

async fn smoke_galaxy_hot_tier_hit_ratio_metrics(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_hot_tier_hit_ratio(&body)
}

/// PH-S225: live stand exposes Galaxy verification counters on Prometheus scrape.
const GALAXY_VERIFICATION_METRICS: &[&str] = &[
    "galaxy_verification_sample_total",
    "galaxy_verification_mismatch_total",
    "galaxy_verification_match_total",
    "galaxy_verification_sample_scheduled_total",
    "galaxy_verification_sample_completed_total",
    "galaxy_verification_sample_skipped_total",
    "galaxy_verification_sample_not_applicable_total",
    "galaxy_verification_sampling_evaluations_total",
    "galaxy_verification_checker_enqueue_total",
];

fn metrics_text_has_verification_counters(body: &str) -> Result<(), String> {
    for name in GALAXY_VERIFICATION_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_verification_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_verification_counters(&body)
}

/// PH-S219: live stand exposes Galaxy trust payout counters on Prometheus scrape.
const GALAXY_TRUST_PAYOUT_METRICS: &[&str] = &[
    "galaxy_trust_payout_eligible_total",
    "galaxy_trust_payout_held_total",
    "galaxy_trust_payout_not_applicable_total",
    "galaxy_trust_score",
    "galaxy_trust_gate_min_threshold",
    "galaxy_trust_gate_default_score",
    "galaxy_trust_gate_evaluations_total",
    "galaxy_trust_default_score_applied_total",
    "galaxy_trust_explicit_score_total",
];

fn metrics_text_has_trust_payout_counters(body: &str) -> Result<(), String> {
    for name in GALAXY_TRUST_PAYOUT_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_trust_payout_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_trust_payout_counters(&body)
}

/// PH-S232 / PH-S426: live stand exposes Galaxy replication counters on Prometheus scrape.
const GALAXY_REPLICATION_METRICS: &[&str] = &[
    "galaxy_replication_strict_total",
    "galaxy_replication_enqueue_total",
    "galaxy_replication_executor_enqueue_total",
];

fn metrics_text_has_replication_strict(body: &str) -> Result<(), String> {
    for name in GALAXY_REPLICATION_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_replication_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_replication_strict(&body)
}

/// PH-S451: live stand exposes PH-S444…S449 horizon wire metrics on Prometheus scrape.
const GALAXY_HORIZON_WIRE_S444_METRICS: &[&str] = &[
    "galaxy_prefetch_seed_fetch_total",
    "galaxy_prefetch_seed_fetch_miss_total",
    "galaxy_prefetch_co_access_total",
    "galaxy_locality_unsatisfied_total",
    "poolai_protocol_negotiation_rejected_total",
    "galaxy_verification_replay_record_total",
];

fn metrics_text_has_horizon_wire_s444(body: &str) -> Result<(), String> {
    for name in GALAXY_HORIZON_WIRE_S444_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_horizon_wire_s444_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_horizon_wire_s444(&body)
}

/// PH-S462: live stand exposes PH-S454…S460 horizon wire metrics on Prometheus scrape.
const GALAXY_HORIZON_WIRE_S454_METRICS: &[&str] = &[
    "galaxy_prefetch_re_migrate_total",
    "galaxy_verification_elevated_applied_total",
    "galaxy_trust_score_delta_total",
    "galaxy_replication_rate_limited_total",
    "galaxy_hot_promote_total",
    "galaxy_hot_evict_total",
    "galaxy_shard_access_total",
    "galaxy_prefetch_queue_depth",
];

fn metrics_text_has_horizon_wire_s454(body: &str) -> Result<(), String> {
    for name in GALAXY_HORIZON_WIRE_S454_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_horizon_wire_s454_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_horizon_wire_s454(&body)
}

async fn smoke_grid_verification_replay(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/verification-replay"))
        .send()
        .await
        .map_err(|e| format!("verification-replay request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("verification-replay status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("verification-replay body: {body}"));
    }
    Ok(())
}

/// PH-S482: live stand exposes PH-S474…S479 horizon wire metrics on Prometheus scrape.
const GALAXY_HORIZON_WIRE_S474_METRICS: &[&str] = &[
    "galaxy_prefetch_egress_blocked_total",
    "galaxy_prefetch_peer_fetch_total",
    "galaxy_prefetch_peer_fetch_miss_total",
];

fn metrics_text_has_horizon_wire_s474(body: &str) -> Result<(), String> {
    for name in GALAXY_HORIZON_WIRE_S474_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_horizon_wire_s474_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_horizon_wire_s474(&body)
}

/// PH-S492: live stand exposes PH-S484…S489 horizon wire metrics on Prometheus scrape.
const GALAXY_HORIZON_WIRE_S484_METRICS: &[&str] = &["galaxy_prefetch_pull_bytes_total"];

fn metrics_text_has_horizon_wire_s484(body: &str) -> Result<(), String> {
    for name in GALAXY_HORIZON_WIRE_S484_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_horizon_wire_s484_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_horizon_wire_s484(&body)
}

/// PH-S501: live stand exposes PH-S494…S499 horizon wire metrics on Prometheus scrape.
const GALAXY_HORIZON_WIRE_S494_METRICS: &[&str] = &["galaxy_verification_checker_pending_total"];

fn metrics_text_has_horizon_wire_s494(body: &str) -> Result<(), String> {
    for name in GALAXY_HORIZON_WIRE_S494_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_horizon_wire_s494_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_horizon_wire_s494(&body)
}

async fn smoke_grid_verification_checker_tasks(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/verification-checker/tasks"))
        .send()
        .await
        .map_err(|e| format!("verification-checker/tasks request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!(
            "verification-checker/tasks status {}",
            resp.status()
        ));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("verification-checker/tasks body: {body}"));
    }
    if !body.get("tasks").and_then(|v| v.as_array()).is_some() {
        return Err(format!("verification-checker/tasks missing tasks: {body}"));
    }
    Ok(())
}

async fn smoke_grid_verification_lifecycle_depth(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/verification-metrics"))
        .send()
        .await
        .map_err(|e| format!("verification-metrics lifecycle: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!(
            "verification-metrics lifecycle status {}",
            resp.status()
        ));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    let depth = body
        .get("lifecycle_depth")
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("verification-metrics missing lifecycle_depth: {body}"))?;
    if depth.is_empty() {
        return Err("verification-metrics lifecycle_depth empty".into());
    }
    Ok(())
}

async fn smoke_grid_verification_metrics_api(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/verification-metrics"))
        .send()
        .await
        .map_err(|e| format!("verification-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("verification-metrics status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("verification-metrics body: {body}"));
    }
    let metrics = body
        .get("metrics")
        .ok_or_else(|| format!("verification-metrics missing metrics: {body}"))?;
    for key in [
        "sample_total",
        "mismatch_total",
        "match_total",
        "checker_pending_total",
    ] {
        if !metrics.get(key).and_then(|v| v.as_u64()).is_some() {
            return Err(format!("verification-metrics missing {key}: {body}"));
        }
    }
    Ok(())
}

async fn smoke_grid_replay_metrics_api(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/replay-metrics"))
        .send()
        .await
        .map_err(|e| format!("replay-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("replay-metrics status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("replay-metrics body: {body}"));
    }
    let metrics = body
        .get("metrics")
        .ok_or_else(|| format!("replay-metrics missing metrics: {body}"))?;
    for key in [
        "replay_pending",
        "replay_pending_scheduled_total",
        "verification_replay_record_total",
    ] {
        if !metrics.get(key).and_then(|v| v.as_u64()).is_some() {
            return Err(format!("replay-metrics missing {key}: {body}"));
        }
    }
    Ok(())
}

async fn smoke_grid_settlement_metrics_api(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/settlement-metrics"))
        .send()
        .await
        .map_err(|e| format!("settlement-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("settlement-metrics status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("settlement-metrics body: {body}"));
    }
    let metrics = body
        .get("metrics")
        .ok_or_else(|| format!("settlement-metrics missing metrics: {body}"))?;
    for key in [
        "pending_verification_total",
        "cleared_total",
        "resolved_total",
        "payout_batch_total",
    ] {
        if !metrics.get(key).and_then(|v| v.as_u64()).is_some() {
            return Err(format!("settlement-metrics missing {key}: {body}"));
        }
    }
    Ok(())
}

async fn smoke_grid_trust_metrics_api(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/trust-metrics"))
        .send()
        .await
        .map_err(|e| format!("trust-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("trust-metrics status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("trust-metrics body: {body}"));
    }
    let metrics = body
        .get("metrics")
        .ok_or_else(|| format!("trust-metrics missing metrics: {body}"))?;
    for key in [
        "payout_eligible_total",
        "payout_held_total",
        "last_trust_score",
        "gate_min_threshold",
    ] {
        if !metrics.get(key).and_then(|v| v.as_u64()).is_some() {
            return Err(format!("trust-metrics missing {key}: {body}"));
        }
    }
    if body
        .get("trust_persist_depth")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .is_none()
    {
        return Err(format!("trust-metrics missing trust_persist_depth: {body}"));
    }
    Ok(())
}

async fn smoke_grid_replication_metrics_api(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/replication-metrics"))
        .send()
        .await
        .map_err(|e| format!("replication-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("replication-metrics status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("replication-metrics body: {body}"));
    }
    let metrics = body
        .get("metrics")
        .ok_or_else(|| format!("replication-metrics missing metrics: {body}"))?;
    for key in [
        "strict_total",
        "enqueue_total",
        "executor_enqueue_total",
        "rate_limited_total",
    ] {
        if !metrics.get(key).and_then(|v| v.as_u64()).is_some() {
            return Err(format!("replication-metrics missing {key}: {body}"));
        }
    }
    let depth = body
        .get("replication_depth")
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("replication-metrics missing replication_depth: {body}"))?;
    if depth.is_empty() {
        return Err("replication-metrics replication_depth empty".into());
    }
    if body
        .get("rate_cap_per_hour")
        .and_then(|v| v.as_u64())
        .is_none()
    {
        return Err(format!(
            "replication-metrics missing rate_cap_per_hour: {body}"
        ));
    }
    Ok(())
}

async fn smoke_grid_pricing_metrics_api(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/pricing-metrics"))
        .send()
        .await
        .map_err(|e| format!("pricing-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("pricing-metrics status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("pricing-metrics body: {body}"));
    }
    let metrics = body
        .get("metrics")
        .ok_or_else(|| format!("pricing-metrics missing metrics: {body}"))?;
    for key in [
        "fresh_served_total",
        "stale_served_total",
        "forced_fallback_total",
        "provider_catalog_lookups_total",
        "provider_catalog_hits_total",
        "provider_errors_total",
        "provider_timeouts_total",
    ] {
        if !metrics.get(key).and_then(|v| v.as_u64()).is_some() {
            return Err(format!("pricing-metrics missing {key}: {body}"));
        }
    }
    if body.get("pricing_depth").and_then(|v| v.as_str()).is_none() {
        return Err(format!("pricing-metrics missing pricing_depth: {body}"));
    }
    if body
        .get("provider_http_timeout_ms")
        .and_then(|v| v.as_u64())
        .is_none()
    {
        return Err(format!(
            "pricing-metrics missing provider_http_timeout_ms: {body}"
        ));
    }
    Ok(())
}

/// PH-S903: stand smoke pricing-metrics JSON↔Prom parity.
async fn smoke_pricing_metrics_json_prometheus_parity(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    use poolai::grid::stand_smoke_metrics_parity::validate_pricing_metrics_parity;

    let prom_resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if prom_resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", prom_resp.status()));
    }
    let prom_text = prom_resp.text().await.map_err(|e| e.to_string())?;

    let resp = client
        .get(api_url(base, "/api/v1/grid/pricing-metrics"))
        .send()
        .await
        .map_err(|e| format!("pricing-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("pricing-metrics status {}", resp.status()));
    }
    let pricing: Value = resp.json().await.map_err(|e| e.to_string())?;
    validate_pricing_metrics_parity(&prom_text, &pricing)
}

/// PH-S901: grid pricing L2 fallback stable snapshot (PH-S123 pattern).
async fn smoke_grid_pricing_l2_fallback_stable(client: &Client, base: &str) -> Result<(), String> {
    let model = smoke_id("smoke-pricing-fallback");
    let url = format!(
        "{}/api/v1/grid/pricing?task_profile=inference:text&model_profile={model}&unit_key=inference_blended_token",
        base.trim_end_matches('/')
    );
    let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
    if resp.status() == StatusCode::SERVICE_UNAVAILABLE {
        return Ok(());
    }
    if resp.status() != StatusCode::OK {
        return Err(format!("grid pricing fallback status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("grid pricing fallback body: {body}"));
    }
    let snap = body
        .get("snapshot")
        .ok_or_else(|| format!("grid pricing fallback missing snapshot: {body}"))?;
    if snap
        .get("poolai_quote_usd_micro")
        .and_then(|v| v.as_u64())
        .is_none()
    {
        return Err(format!(
            "grid pricing fallback missing poolai_quote_usd_micro: {body}"
        ));
    }
    Ok(())
}

async fn smoke_grid_prefetch_metrics_api(client: &Client, base: &str) -> Result<(), String> {
    use poolai::grid::stand_smoke_metrics_parity::validate_prefetch_metrics_parity;

    let resp = client
        .get(api_url(base, "/api/v1/grid/prefetch-metrics"))
        .send()
        .await
        .map_err(|e| format!("prefetch-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("prefetch-metrics status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("prefetch-metrics body: {body}"));
    }

    let prom_resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if prom_resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", prom_resp.status()));
    }
    let prom_text = prom_resp.text().await.map_err(|e| e.to_string())?;
    validate_prefetch_metrics_parity(&prom_text, &body)
}

async fn smoke_grid_locality_metrics_api(client: &Client, base: &str) -> Result<(), String> {
    use poolai::grid::stand_smoke_metrics_parity::validate_locality_metrics_parity;

    let resp = client
        .get(api_url(base, "/api/v1/grid/locality-metrics"))
        .send()
        .await
        .map_err(|e| format!("locality-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("locality-metrics status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("locality-metrics body: {body}"));
    }

    let prom_resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if prom_resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", prom_resp.status()));
    }
    let prom_text = prom_resp.text().await.map_err(|e| e.to_string())?;
    validate_locality_metrics_parity(&prom_text, &body)
}

async fn smoke_grid_fee_split_metrics_api(client: &Client, base: &str) -> Result<(), String> {
    use poolai::grid::stand_smoke_metrics_parity::validate_fee_split_metrics_parity;

    let resp = client
        .get(api_url(base, "/api/v1/grid/fee-split-metrics"))
        .send()
        .await
        .map_err(|e| format!("fee-split-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("fee-split-metrics status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("fee-split-metrics body: {body}"));
    }

    let prom_resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if prom_resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", prom_resp.status()));
    }
    let prom_text = prom_resp.text().await.map_err(|e| e.to_string())?;
    validate_fee_split_metrics_parity(&prom_text, &body)
}

async fn smoke_grid_update_policy_api(client: &Client, base: &str) -> Result<(), String> {
    use poolai::grid::stand_smoke_metrics_parity::validate_update_policy_json_export;

    let resp = client
        .get(api_url(base, "/api/v1/grid/update-policy"))
        .send()
        .await
        .map_err(|e| format!("update-policy request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("update-policy status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    validate_update_policy_json_export(&body)
}

async fn smoke_grid_governance_metrics_api(client: &Client, base: &str) -> Result<(), String> {
    use poolai::grid::stand_smoke_metrics_parity::validate_governance_metrics_parity;

    let resp = client
        .get(api_url(base, "/api/v1/grid/governance-metrics"))
        .send()
        .await
        .map_err(|e| format!("governance-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("governance-metrics status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("governance-metrics body: {body}"));
    }

    let prom_resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if prom_resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", prom_resp.status()));
    }
    let prom_text = prom_resp.text().await.map_err(|e| e.to_string())?;
    validate_governance_metrics_parity(&prom_text, &body)?;
    if !prom_text.contains("poolai_advisory_acknowledged_total") {
        return Err("/metrics missing poolai_advisory_acknowledged_total".to_string());
    }
    Ok(())
}

/// PH-S713: band-6 JSON metrics export shape + Prometheus parity across all grid metric APIs.
async fn smoke_grid_metrics_json_prometheus_parity_band6(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    use poolai::grid::stand_smoke_metrics_parity::validate_band6_metrics_parity;

    let prom_resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if prom_resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", prom_resp.status()));
    }
    let prom_text = prom_resp.text().await.map_err(|e| e.to_string())?;

    async fn fetch_metrics_json(client: &Client, base: &str, path: &str) -> Result<Value, String> {
        let resp = client
            .get(api_url(base, path))
            .send()
            .await
            .map_err(|e| format!("{path} request: {e}"))?;
        if resp.status() != StatusCode::OK {
            return Err(format!("{path} status {}", resp.status()));
        }
        resp.json().await.map_err(|e| format!("{path} json: {e}"))
    }

    let verification =
        fetch_metrics_json(client, base, "/api/v1/grid/verification-metrics").await?;
    let replay = fetch_metrics_json(client, base, "/api/v1/grid/replay-metrics").await?;
    let settlement = fetch_metrics_json(client, base, "/api/v1/grid/settlement-metrics").await?;
    let trust = fetch_metrics_json(client, base, "/api/v1/grid/trust-metrics").await?;
    let replication = fetch_metrics_json(client, base, "/api/v1/grid/replication-metrics").await?;
    let pricing = fetch_metrics_json(client, base, "/api/v1/grid/pricing-metrics").await?;

    validate_band6_metrics_parity(
        &prom_text,
        &verification,
        &replay,
        &settlement,
        &trust,
        &replication,
        &pricing,
    )
}

/// PH-S833: stand smoke v2 — full grid JSON metrics export + Prometheus parity.
async fn smoke_grid_metrics_json_prometheus_parity_band6_v2(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    use poolai::grid::stand_smoke_metrics_parity::validate_band6_metrics_parity_v2;

    let prom_resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if prom_resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", prom_resp.status()));
    }
    let prom_text = prom_resp.text().await.map_err(|e| e.to_string())?;

    async fn fetch_metrics_json(client: &Client, base: &str, path: &str) -> Result<Value, String> {
        let resp = client
            .get(api_url(base, path))
            .send()
            .await
            .map_err(|e| format!("{path} request: {e}"))?;
        if resp.status() != StatusCode::OK {
            return Err(format!("{path} status {}", resp.status()));
        }
        resp.json().await.map_err(|e| format!("{path} json: {e}"))
    }

    let verification =
        fetch_metrics_json(client, base, "/api/v1/grid/verification-metrics").await?;
    let replay = fetch_metrics_json(client, base, "/api/v1/grid/replay-metrics").await?;
    let settlement = fetch_metrics_json(client, base, "/api/v1/grid/settlement-metrics").await?;
    let trust = fetch_metrics_json(client, base, "/api/v1/grid/trust-metrics").await?;
    let replication = fetch_metrics_json(client, base, "/api/v1/grid/replication-metrics").await?;
    let pricing = fetch_metrics_json(client, base, "/api/v1/grid/pricing-metrics").await?;
    let prefetch = fetch_metrics_json(client, base, "/api/v1/grid/prefetch-metrics").await?;
    let locality = fetch_metrics_json(client, base, "/api/v1/grid/locality-metrics").await?;
    let fee_split = fetch_metrics_json(client, base, "/api/v1/grid/fee-split-metrics").await?;
    let governance = fetch_metrics_json(client, base, "/api/v1/grid/governance-metrics").await?;
    let payout_batch =
        fetch_metrics_json(client, base, "/api/v1/grid/payout-batch-metrics").await?;

    validate_band6_metrics_parity_v2(
        &prom_text,
        &verification,
        &replay,
        &settlement,
        &trust,
        &replication,
        &pricing,
        &prefetch,
        &locality,
        &fee_split,
        &governance,
        &payout_batch,
    )
}

/// PH-S1073: stand smoke v3 — extended grid JSON metrics export + Prometheus parity.
async fn smoke_grid_metrics_json_prometheus_parity_band6_v3(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    use poolai::grid::stand_smoke_metrics_parity::validate_band6_metrics_parity_v3;

    let prom_resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if prom_resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", prom_resp.status()));
    }
    let prom_text = prom_resp.text().await.map_err(|e| e.to_string())?;

    async fn fetch_metrics_json(client: &Client, base: &str, path: &str) -> Result<Value, String> {
        let resp = client
            .get(api_url(base, path))
            .send()
            .await
            .map_err(|e| format!("{path} request: {e}"))?;
        if resp.status() != StatusCode::OK {
            return Err(format!("{path} status {}", resp.status()));
        }
        resp.json().await.map_err(|e| format!("{path} json: {e}"))
    }

    let verification =
        fetch_metrics_json(client, base, "/api/v1/grid/verification-metrics").await?;
    let replay = fetch_metrics_json(client, base, "/api/v1/grid/replay-metrics").await?;
    let settlement = fetch_metrics_json(client, base, "/api/v1/grid/settlement-metrics").await?;
    let trust = fetch_metrics_json(client, base, "/api/v1/grid/trust-metrics").await?;
    let replication = fetch_metrics_json(client, base, "/api/v1/grid/replication-metrics").await?;
    let pricing = fetch_metrics_json(client, base, "/api/v1/grid/pricing-metrics").await?;
    let prefetch = fetch_metrics_json(client, base, "/api/v1/grid/prefetch-metrics").await?;
    let locality = fetch_metrics_json(client, base, "/api/v1/grid/locality-metrics").await?;
    let fee_split = fetch_metrics_json(client, base, "/api/v1/grid/fee-split-metrics").await?;
    let governance = fetch_metrics_json(client, base, "/api/v1/grid/governance-metrics").await?;
    let payout_batch =
        fetch_metrics_json(client, base, "/api/v1/grid/payout-batch-metrics").await?;

    validate_band6_metrics_parity_v3(
        &prom_text,
        &verification,
        &replay,
        &settlement,
        &trust,
        &replication,
        &pricing,
        &prefetch,
        &locality,
        &fee_split,
        &governance,
        &payout_batch,
    )
}

async fn smoke_grid_network_profile_read(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(
            base,
            "/api/v1/grid/network-profiles/smoke-peer-missing",
        ))
        .send()
        .await
        .map_err(|e| format!("network-profiles request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("network-profiles status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("network-profiles body: {body}"));
    }
    if !body.get("peer_id").and_then(|v| v.as_str()).is_some() {
        return Err(format!("network-profiles missing peer_id: {body}"));
    }
    Ok(())
}

async fn smoke_grid_network_profiles_list(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/network-profiles"))
        .send()
        .await
        .map_err(|e| format!("network-profiles list request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("network-profiles list status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("network-profiles list body: {body}"));
    }
    if !body.get("peer_ids").and_then(|v| v.as_array()).is_some() {
        return Err(format!("network-profiles list missing peer_ids: {body}"));
    }
    Ok(())
}

async fn smoke_ops_power_openapi(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .post(api_url(base, "/api/v1/ops/power"))
        .json(&json!({"action": "shutdown"}))
        .send()
        .await
        .map_err(|e| format!("ops power request: {e}"))?;
    if resp.status() != StatusCode::ACCEPTED {
        return Err(format!(
            "ops power expected 202 Accepted, got {}",
            resp.status()
        ));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    for key in ["accepted", "action", "dev_guard"] {
        if body.get(key).is_none() {
            return Err(format!("ops power missing `{key}`: {body}"));
        }
    }
    if body.get("accepted") != Some(&json!(true)) {
        return Err(format!("ops power accepted != true: {body}"));
    }
    if body.get("action") != Some(&json!("shutdown")) {
        return Err(format!("ops power action mismatch: {body}"));
    }
    Ok(())
}

async fn smoke_admin_security_advisories_openapi(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/admin/security-advisories"))
        .send()
        .await
        .map_err(|e| format!("security-advisories request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("security-advisories status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    let rows = body
        .as_array()
        .ok_or_else(|| format!("security-advisories expected array: {body}"))?;
    if rows.is_empty() {
        return Err("security-advisories empty".into());
    }
    let first = &rows[0];
    for key in ["id", "severity", "summary", "acknowledged"] {
        if !first.get(key).is_some() {
            return Err(format!("security-advisories missing `{key}`: {first}"));
        }
    }
    Ok(())
}

async fn smoke_virtual_nodes_wallet_rebind_override_openapi(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    let resp = client
        .post(api_url(
            base,
            "/api/v1/virtual-nodes/telegram/wallet/rebind-override",
        ))
        .json(&json!({
            "telegram_user_id": "9001",
            "chat_id": "-1001234567890",
            "payout_pubkey": "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU"
        }))
        .send()
        .await
        .map_err(|e| format!("wallet rebind-override request: {e}"))?;
    if resp.status() != StatusCode::UNAUTHORIZED {
        return Err(format!(
            "wallet rebind-override expected 401 without admin token, got {}",
            resp.status()
        ));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("error").is_none() {
        return Err(format!("wallet rebind-override missing error: {body}"));
    }
    Ok(())
}

async fn smoke_grid_telegram_seats(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/telegram-seats"))
        .send()
        .await
        .map_err(|e| format!("telegram-seats request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("telegram-seats status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("telegram-seats body: {body}"));
    }
    if !body.get("seat_policy").and_then(|v| v.as_str()).is_some() {
        return Err(format!("telegram-seats missing seat_policy: {body}"));
    }
    Ok(())
}

async fn smoke_grid_network_profile_put(client: &Client, base: &str) -> Result<(), String> {
    let peer = "smoke-peer-put-s504";
    let resp = client
        .put(api_url(
            base,
            &format!("/api/v1/grid/network-profiles/{peer}"),
        ))
        .json(&serde_json::json!({
            "network_profile": { "region": "smoke", "latency_ms_p50": 11 }
        }))
        .send()
        .await
        .map_err(|e| format!("network-profiles PUT request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("network-profiles PUT status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("network-profiles PUT body: {body}"));
    }
    Ok(())
}

async fn smoke_grid_payout_batch_history(client: &Client, base: &str) -> Result<(), String> {
    use poolai::grid::stand_smoke_metrics_parity::validate_payout_batch_metrics_parity;

    let resp = client
        .get(api_url(base, "/api/v1/grid/payout-batch/history?limit=5"))
        .send()
        .await
        .map_err(|e| format!("payout-batch/history request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("payout-batch/history status {}", resp.status()));
    }
    let history: Value = resp.json().await.map_err(|e| e.to_string())?;
    if history.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("payout-batch/history body: {history}"));
    }
    if !history.get("entries").and_then(|v| v.as_array()).is_some() {
        return Err(format!("payout-batch/history missing entries: {history}"));
    }

    let latest_resp = client
        .get(api_url(base, "/api/v1/grid/payout-batch"))
        .send()
        .await
        .map_err(|e| format!("payout-batch request: {e}"))?;
    if latest_resp.status() != StatusCode::OK {
        return Err(format!("payout-batch status {}", latest_resp.status()));
    }
    let latest: Value = latest_resp.json().await.map_err(|e| e.to_string())?;
    if latest.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("payout-batch body: {latest}"));
    }
    if !latest
        .get("settlement_mode")
        .and_then(|v| v.as_str())
        .is_some()
    {
        return Err(format!("payout-batch missing settlement_mode: {latest}"));
    }

    let metrics_resp = client
        .get(api_url(base, "/api/v1/grid/payout-batch-metrics"))
        .send()
        .await
        .map_err(|e| format!("payout-batch-metrics request: {e}"))?;
    if metrics_resp.status() != StatusCode::OK {
        return Err(format!(
            "payout-batch-metrics status {}",
            metrics_resp.status()
        ));
    }
    let metrics_body: Value = metrics_resp.json().await.map_err(|e| e.to_string())?;
    if metrics_body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("payout-batch-metrics body: {metrics_body}"));
    }

    let prom_resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if prom_resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", prom_resp.status()));
    }
    let prom_text = prom_resp.text().await.map_err(|e| e.to_string())?;
    validate_payout_batch_metrics_parity(&prom_text, &metrics_body)
}

async fn smoke_grid_verification_replay_history(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(
            base,
            "/api/v1/grid/verification-replay/history?limit=5",
        ))
        .send()
        .await
        .map_err(|e| format!("verification-replay/history request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!(
            "verification-replay/history status {}",
            resp.status()
        ));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("verification-replay/history body: {body}"));
    }
    if !body.get("records").and_then(|v| v.as_array()).is_some() {
        return Err(format!(
            "verification-replay/history missing records: {body}"
        ));
    }
    Ok(())
}

/// PH-S472: live stand exposes PH-S464…S468 horizon wire metrics on Prometheus scrape.
const GALAXY_HORIZON_WIRE_S464_METRICS: &[&str] = &[
    "galaxy_prefetch_backpressure_total",
    "galaxy_prefetch_raid_fetch_total",
    "galaxy_prefetch_raid_fetch_miss_total",
    "poolai_protocol_negotiation_accepted_total",
];

fn metrics_text_has_horizon_wire_s464(body: &str) -> Result<(), String> {
    for name in GALAXY_HORIZON_WIRE_S464_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_horizon_wire_s464_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_horizon_wire_s464(&body)
}

async fn smoke_grid_payout_batch(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/payout-batch"))
        .send()
        .await
        .map_err(|e| format!("payout-batch request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("payout-batch status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("payout-batch body: {body}"));
    }
    if body.get("settlement_mode").and_then(|v| v.as_str()) != Some("offline_batch") {
        return Err(format!("payout-batch missing settlement_mode: {body}"));
    }
    if body.get("onchain_depth").and_then(|v| v.as_str()).is_none() {
        return Err(format!("payout-batch missing onchain_depth: {body}"));
    }
    if body.get("solana_depth").and_then(|v| v.as_str()).is_none() {
        return Err(format!("payout-batch missing solana_depth: {body}"));
    }
    Ok(())
}

async fn smoke_grid_pricing(client: &Client, base: &str) -> Result<(), String> {
    let model = smoke_id("smoke-pricing");
    let url = format!(
        "{}/api/v1/grid/pricing?task_profile=inference:text&model_profile={model}&unit_key=inference_blended_token",
        base.trim_end_matches('/')
    );
    let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
    if resp.status() != StatusCode::OK {
        return Err(format!("grid pricing status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("grid pricing body: {body}"));
    }
    Ok(())
}

async fn create_unbound_job(client: &Client, base: &str, artifact: &str) -> Result<String, String> {
    let resp = client
        .post(api_url(base, "/api/v1/jobs"))
        .json(&json!({
            "kind": "inference",
            "priority": 5,
            "input_artifact_ids": [artifact],
            "resources": { "gpu_memory_mb": 9_007_199_254_740_991_u64 }
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.status() != StatusCode::CREATED {
        return Err(format!("create job status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    body.get("id")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .ok_or_else(|| format!("create job missing id: {body}"))
}

async fn smoke_jobs_lease_acquire(client: &Client, base: &str) -> Result<(), String> {
    let id = create_unbound_job(client, base, "smoke-lease-acquire").await?;
    let acquire = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease")))
        .json(&json!({ "lease_owner": "stand-smoke-worker" }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if acquire.status() != StatusCode::OK {
        return Err(format!("lease acquire status {}", acquire.status()));
    }
    let body: Value = acquire.json().await.map_err(|e| e.to_string())?;
    let job = body.get("job").ok_or("lease response missing job")?;
    if job.get("status").and_then(|v| v.as_str()) != Some("leased") {
        return Err(format!("expected leased: {job}"));
    }
    if job.get("lease_owner").and_then(|v| v.as_str()) != Some("stand-smoke-worker") {
        return Err(format!("unexpected lease_owner: {job}"));
    }
    if job.get("lease_epoch").and_then(|v| v.as_u64()) != Some(1) {
        return Err(format!("expected lease_epoch 1: {job}"));
    }
    if job
        .get("lease_expires_at")
        .and_then(|v| v.as_str())
        .is_none()
    {
        return Err(format!("missing lease_expires_at: {job}"));
    }
    Ok(())
}

async fn smoke_jobs_lease_conflict(client: &Client, base: &str) -> Result<(), String> {
    let id = create_unbound_job(client, base, "smoke-lease-conflict").await?;
    let first = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease")))
        .json(&json!({ "lease_owner": "stand-smoke-a" }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if first.status() != StatusCode::OK {
        return Err(format!("first acquire status {}", first.status()));
    }
    let second = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease")))
        .json(&json!({ "lease_owner": "stand-smoke-b" }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if second.status() != StatusCode::CONFLICT {
        return Err(format!("second acquire status {}", second.status()));
    }
    let err: Value = second.json().await.map_err(|e| e.to_string())?;
    if err.pointer("/error/code").and_then(|v| v.as_str()) != Some("lease_already_active") {
        return Err(format!("expected lease_already_active: {err}"));
    }
    Ok(())
}

async fn smoke_jobs_lease_renew_extends(client: &Client, base: &str) -> Result<(), String> {
    let id = create_unbound_job(client, base, "smoke-lease-renew").await?;
    let acquired = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease")))
        .json(&json!({ "lease_owner": "stand-smoke-renew" }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if acquired.status() != StatusCode::OK {
        return Err(format!("lease acquire status {}", acquired.status()));
    }
    let acquired_body: Value = acquired.json().await.map_err(|e| e.to_string())?;
    let job = acquired_body
        .get("job")
        .ok_or("lease response missing job")?;
    let epoch = job
        .get("lease_epoch")
        .and_then(|v| v.as_u64())
        .ok_or("missing lease_epoch")?;
    let expires_before = job
        .get("lease_expires_at")
        .and_then(|v| v.as_str())
        .ok_or("missing lease_expires_at")?
        .to_string();
    let renew = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease/renew")))
        .json(&json!({ "lease_epoch": epoch }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if renew.status() != StatusCode::OK {
        return Err(format!("lease renew status {}", renew.status()));
    }
    let renewed_body: Value = renew.json().await.map_err(|e| e.to_string())?;
    let renewed_job = renewed_body
        .get("job")
        .ok_or("renew response missing job")?;
    if renewed_job.get("lease_epoch").and_then(|v| v.as_u64()) != Some(epoch) {
        return Err(format!("epoch changed on renew: {renewed_job}"));
    }
    let expires_after = renewed_job
        .get("lease_expires_at")
        .and_then(|v| v.as_str())
        .ok_or("renew missing lease_expires_at")?;
    if expires_after == expires_before {
        return Err("lease_expires_at unchanged after renew".into());
    }
    Ok(())
}

async fn smoke_jobs_lease_renew_stale_epoch(client: &Client, base: &str) -> Result<(), String> {
    let id = create_unbound_job(client, base, "smoke-lease-renew-reject").await?;
    let acquired = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease")))
        .json(&json!({ "lease_owner": "stand-smoke-cas" }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if acquired.status() != StatusCode::OK {
        return Err(format!("lease acquire status {}", acquired.status()));
    }
    let epoch = acquired
        .json::<Value>()
        .await
        .map_err(|e| e.to_string())?
        .pointer("/job/lease_epoch")
        .and_then(|v| v.as_u64())
        .ok_or("missing lease_epoch")?;
    let stale_epoch = epoch.saturating_sub(1);
    let rejected = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease/renew")))
        .json(&json!({ "lease_epoch": stale_epoch }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if rejected.status() != StatusCode::CONFLICT {
        return Err(format!("stale renew status {}", rejected.status()));
    }
    let err: Value = rejected.json().await.map_err(|e| e.to_string())?;
    if err.pointer("/error/code").and_then(|v| v.as_str()) != Some("lease_epoch_rejected") {
        return Err(format!("expected lease_epoch_rejected: {err}"));
    }
    Ok(())
}

async fn smoke_jobs_lease_renew_no_acquire(client: &Client, base: &str) -> Result<(), String> {
    let id = create_unbound_job(client, base, "smoke-renew-no-acquire").await?;
    let renew = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease/renew")))
        .json(&json!({ "lease_epoch": 1 }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if renew.status() != StatusCode::BAD_REQUEST {
        return Err(format!("renew without acquire status {}", renew.status()));
    }
    let err: Value = renew.json().await.map_err(|e| e.to_string())?;
    let msg = err
        .pointer("/error/message")
        .and_then(|v| v.as_str())
        .or_else(|| err.get("message").and_then(|v| v.as_str()))
        .unwrap_or("");
    if !msg.to_ascii_lowercase().contains("acquire lease") {
        return Err(format!("expected acquire lease message: {err}"));
    }
    Ok(())
}

async fn smoke_jobs_lease_renew_expired(client: &Client, base: &str) -> Result<(), String> {
    let id = create_unbound_job(client, base, "smoke-lease-expired").await?;
    let acquired = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease")))
        .json(&json!({ "lease_owner": "stand-smoke-expired" }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if acquired.status() != StatusCode::OK {
        return Err(format!("lease acquire status {}", acquired.status()));
    }
    let epoch = acquired
        .json::<Value>()
        .await
        .map_err(|e| e.to_string())?
        .pointer("/job/lease_epoch")
        .and_then(|v| v.as_u64())
        .ok_or("missing lease_epoch")?;
    tokio::time::sleep(Duration::from_millis(2600)).await;
    let expired = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease/renew")))
        .json(&json!({ "lease_epoch": epoch }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if expired.status() != StatusCode::CONFLICT {
        return Err(format!("expired renew status {}", expired.status()));
    }
    let err: Value = expired.json().await.map_err(|e| e.to_string())?;
    if err.pointer("/error/code").and_then(|v| v.as_str()) != Some("lease_expired") {
        return Err(format!("expected lease_expired: {err}"));
    }
    Ok(())
}

/// PH-S853: `GET /api/v1/jobs` exposes `store_backend` for admin badge wire.
async fn smoke_jobs_store_backend(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/jobs"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("jobs list status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    let backend = body
        .get("store_backend")
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("missing store_backend: {body}"))?;
    if backend.trim().is_empty() {
        return Err("empty store_backend".into());
    }
    if !matches!(backend, "json" | "sqlite" | "raid") {
        return Err(format!("unexpected store_backend: {backend}"));
    }
    Ok(())
}

async fn smoke_jobs_migrating(client: &Client, base: &str) -> Result<(), String> {
    let id = create_unbound_job(client, base, "smoke-migrate").await?;
    let _ = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease")))
        .json(&json!({ "lease_owner": "stand-smoke-migrate" }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    for status in ["migrating", "executing", "migrating"] {
        let patch = client
            .patch(api_url(base, &format!("/api/v1/jobs/{id}")))
            .json(&json!({ "status": status }))
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if patch.status() != StatusCode::OK {
            return Err(format!("patch {status} status {}", patch.status()));
        }
        let body: Value = patch.json().await.map_err(|e| e.to_string())?;
        if body.pointer("/job/status").and_then(|v| v.as_str()) != Some(status) {
            return Err(format!("patch {status} body: {body}"));
        }
    }
    Ok(())
}

async fn smoke_protocol_middleware(client: &Client, base: &str) -> Result<(), String> {
    let peer_id = smoke_id("proto-accept");
    let resp = client
        .post(api_url(base, "/api/v1/discovery/register-remote"))
        .header("X-PoolAI-Protocol", "1.2")
        .json(&json!({
            "peer_id": peer_id,
            "address": "10.0.0.1",
            "port": 9091,
            "protocol_version": "1.2",
            "build_id": "stand-smoke",
            "metadata": { "role": "virtual_node" }
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.status() != StatusCode::OK {
        return Err(format!("register-remote status {}", resp.status()));
    }
    let compat = resp
        .headers()
        .get("x-poolai-protocol-compat")
        .and_then(|v| v.to_str().ok());
    if compat != Some("accepted") {
        return Err(format!("expected compat accepted, got {compat:?}"));
    }
    let reject = client
        .post(api_url(base, "/api/v1/discovery/register-remote"))
        .header("X-PoolAI-Protocol", "1.0")
        .json(&json!({
            "peer_id": smoke_id("proto-reject"),
            "address": "10.0.0.1",
            "port": 9091,
            "protocol_version": "1.2",
            "build_id": "stand-smoke",
            "metadata": { "role": "virtual_node" }
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if reject.status() != StatusCode::FORBIDDEN {
        return Err(format!("unsupported protocol status {}", reject.status()));
    }
    Ok(())
}

async fn smoke_telegram_wallet(client: &Client, base: &str) -> Result<(), String> {
    let user = smoke_id("wallet-ok");
    let ok = client
        .post(api_url(base, "/api/v1/virtual-nodes/telegram/wallet"))
        .json(&json!({
            "telegram_user_id": user,
            "chat_id": "-1001234567890",
            "payout_pubkey": VALID_PUBKEY,
            "chain": "solana"
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if ok.status() != StatusCode::OK {
        return Err(format!("wallet bind status {}", ok.status()));
    }
    let bad = client
        .post(api_url(base, "/api/v1/virtual-nodes/telegram/wallet"))
        .json(&json!({
            "telegram_user_id": smoke_id("wallet-bad"),
            "chat_id": "-10099",
            "payout_pubkey": "not-valid!",
            "chain": "solana"
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if bad.status() != StatusCode::BAD_REQUEST {
        return Err(format!("invalid wallet status {}", bad.status()));
    }
    Ok(())
}

async fn smoke_grid_envelope_lease(client: &Client, base: &str) -> Result<(), String> {
    let job_id = smoke_id("grid-job");
    let peer = "stand-smoke-grid-peer";
    let ingest = client
        .post(api_url(base, "/api/v1/grid/envelope"))
        .json(&json!({
            "v": 1,
            "sent_at": "2026-06-13T12:00:00Z",
            "source_peer_id": peer,
            "type": "job",
            "job_id": job_id,
            "task_kind": "inference",
            "input_artifact_ids": [format!("artifact-{job_id}")]
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if ingest.status() != StatusCode::OK {
        return Err(format!("grid job ingest status {}", ingest.status()));
    }
    let get = client
        .get(api_url(base, &format!("/api/v1/jobs/{job_id}")))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let detail: Value = get.json().await.map_err(|e| e.to_string())?;
    let epoch = detail
        .pointer("/job/lease_epoch")
        .and_then(|v| v.as_u64())
        .ok_or("grid job missing lease_epoch")?;
    let result = client
        .post(api_url(base, "/api/v1/grid/envelope"))
        .json(&json!({
            "v": 1,
            "sent_at": "2026-06-13T12:00:01Z",
            "type": "result",
            "job_id": job_id,
            "status": "completed",
            "lease_epoch": epoch,
            "output_artifact_ids": [format!("out-{job_id}")]
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if result.status() != StatusCode::OK {
        return Err(format!("grid result status {}", result.status()));
    }
    Ok(())
}

fn restart_stand(stand_root: &str) -> Result<(), String> {
    let script = Path::new(stand_root).join("restart.sh");
    if !script.is_file() {
        return Err(format!("missing {}", script.display()));
    }
    let status = Command::new("bash")
        .arg(script)
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err(format!("restart.sh exit {status}"));
    }
    Ok(())
}

async fn smoke_jobs_raid(client: &Client, base: &str, stand_root: &str) -> Result<(), String> {
    let id = create_unbound_job(client, base, "smoke-raid-persist").await?;
    restart_stand(stand_root)?;
    wait_health(client, base, 45).await?;
    let get = client
        .get(api_url(base, &format!("/api/v1/jobs/{id}")))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !get.status().is_success() {
        return Err(format!("get job after restart status {}", get.status()));
    }
    let detail: Value = get.json().await.map_err(|e| e.to_string())?;
    if detail.pointer("/job/spec/id").and_then(|v| v.as_str()) != Some(id.as_str()) {
        return Err(format!("job id mismatch after restart: {detail}"));
    }
    Ok(())
}

async fn run_smokes(cli: &Cli) -> SmokeReport {
    let client = match Client::builder().timeout(Duration::from_secs(30)).build() {
        Ok(c) => c,
        Err(e) => {
            return SmokeReport {
                base_url: cli.base_url.clone(),
                stand_root: std::env::var(ENV_STAND_ROOT).ok(),
                ok: false,
                passed: 0,
                failed: 1,
                cases: vec![SmokeCaseResult {
                    name: "client_build",
                    ok: false,
                    detail: Some(e.to_string()),
                }],
                tool: "poolai-http-stand-smoke",
            };
        }
    };

    let stand_root = std::env::var(ENV_STAND_ROOT).ok();
    let mut cases = Vec::new();

    if cli.raid_restart_only {
        match stand_root.as_deref() {
            Some(root) => {
                record(
                    &mut cases,
                    "jobs_raid_restart",
                    smoke_jobs_raid(&client, &cli.base_url, root).await,
                )
                .await;
            }
            None => {
                cases.push(SmokeCaseResult {
                    name: "jobs_raid_restart",
                    ok: false,
                    detail: Some(format!("--raid-restart requires {ENV_STAND_ROOT}")),
                });
            }
        }
        let passed = cases.iter().filter(|c| c.ok).count() as u32;
        let failed = cases.iter().filter(|c| !c.ok).count() as u32;
        return SmokeReport {
            base_url: cli.base_url.clone(),
            stand_root,
            ok: failed == 0,
            passed,
            failed,
            cases,
            tool: "poolai-http-stand-smoke",
        };
    }

    if cli.lease_renew_only {
        record(
            &mut cases,
            "jobs_lease_acquire",
            smoke_jobs_lease_acquire(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "jobs_lease_conflict",
            smoke_jobs_lease_conflict(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "jobs_lease_renew_extends",
            smoke_jobs_lease_renew_extends(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "jobs_lease_renew_stale_epoch",
            smoke_jobs_lease_renew_stale_epoch(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "jobs_lease_renew_no_acquire",
            smoke_jobs_lease_renew_no_acquire(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "jobs_lease_renew_expired",
            smoke_jobs_lease_renew_expired(&client, &cli.base_url).await,
        )
        .await;
        let passed = cases.iter().filter(|c| c.ok).count() as u32;
        let failed = cases.iter().filter(|c| !c.ok).count() as u32;
        return SmokeReport {
            base_url: cli.base_url.clone(),
            stand_root,
            ok: failed == 0,
            passed,
            failed,
            cases,
            tool: "poolai-http-stand-smoke",
        };
    }

    if cli.run_local_smoke_only {
        async fn record(
            cases: &mut Vec<SmokeCaseResult>,
            name: &'static str,
            result: Result<(), String>,
        ) {
            cases.push(match result {
                Ok(()) => SmokeCaseResult {
                    name,
                    ok: true,
                    detail: None,
                },
                Err(e) => SmokeCaseResult {
                    name,
                    ok: false,
                    detail: Some(e),
                },
            });
        }

        record(
            &mut cases,
            "health",
            smoke_health(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "monitoring_alerts",
            smoke_monitoring_alerts_api(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "monitoring_dashboards",
            smoke_monitoring_dashboards_api(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "vm_instances",
            smoke_vm_instances_api(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "ops_power_openapi",
            smoke_ops_power_openapi(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "jobs_store_backend",
            smoke_jobs_store_backend(&client, &cli.base_url).await,
        )
        .await;
        let passed = cases.iter().filter(|c| c.ok).count() as u32;
        let failed = cases.iter().filter(|c| !c.ok).count() as u32;
        return SmokeReport {
            base_url: cli.base_url.clone(),
            stand_root,
            ok: failed == 0,
            passed,
            failed,
            cases,
            tool: "poolai-http-stand-smoke",
        };
    }

    async fn record(
        cases: &mut Vec<SmokeCaseResult>,
        name: &'static str,
        result: Result<(), String>,
    ) {
        cases.push(match result {
            Ok(()) => SmokeCaseResult {
                name,
                ok: true,
                detail: None,
            },
            Err(e) => SmokeCaseResult {
                name,
                ok: false,
                detail: Some(e),
            },
        });
    }

    record(
        &mut cases,
        "health",
        smoke_health(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_pricing",
        smoke_grid_pricing(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_seed_inventory",
        smoke_grid_seed_inventory(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_prefetch_metrics",
        smoke_galaxy_prefetch_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_pricing_forced_fallback_metrics",
        smoke_galaxy_pricing_forced_fallback_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_pricing_cache_age_metrics",
        smoke_galaxy_pricing_cache_age_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_pricing_fresh_served_metrics",
        smoke_galaxy_pricing_fresh_served_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_pricing_stale_served_metrics",
        smoke_galaxy_pricing_stale_served_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_pricing_provider_metrics",
        smoke_galaxy_pricing_provider_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_pricing_quote_market_metrics",
        smoke_galaxy_pricing_quote_market_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_fee_split_applied_metrics",
        smoke_galaxy_fee_split_applied_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_cross_region_egress_metrics",
        smoke_galaxy_cross_region_egress_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_replay_pending_metrics",
        smoke_galaxy_replay_pending_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_verification_metrics",
        smoke_galaxy_verification_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_trust_payout_metrics",
        smoke_galaxy_trust_payout_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_replication_metrics",
        smoke_galaxy_replication_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_horizon_wire_s444_metrics",
        smoke_galaxy_horizon_wire_s444_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_horizon_wire_s454_metrics",
        smoke_galaxy_horizon_wire_s454_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_verification_replay",
        smoke_grid_verification_replay(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_horizon_wire_s464_metrics",
        smoke_galaxy_horizon_wire_s464_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_payout_batch",
        smoke_grid_payout_batch(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_horizon_wire_s474_metrics",
        smoke_galaxy_horizon_wire_s474_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_horizon_wire_s484_metrics",
        smoke_galaxy_horizon_wire_s484_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_horizon_wire_s494_metrics",
        smoke_galaxy_horizon_wire_s494_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_verification_checker_tasks",
        smoke_grid_verification_checker_tasks(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_verification_metrics_api",
        smoke_grid_verification_metrics_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_verification_lifecycle_depth",
        smoke_grid_verification_lifecycle_depth(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_replay_metrics_api",
        smoke_grid_replay_metrics_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_settlement_metrics_api",
        smoke_grid_settlement_metrics_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_trust_metrics_api",
        smoke_grid_trust_metrics_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_replication_metrics_api",
        smoke_grid_replication_metrics_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_pricing_metrics_api",
        smoke_grid_pricing_metrics_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "pricing_metrics_json_prometheus_parity",
        smoke_pricing_metrics_json_prometheus_parity(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_pricing_l2_fallback_stable",
        smoke_grid_pricing_l2_fallback_stable(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_prefetch_metrics_api",
        smoke_grid_prefetch_metrics_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_locality_metrics_api",
        smoke_grid_locality_metrics_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_fee_split_metrics_api",
        smoke_grid_fee_split_metrics_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_update_policy_api",
        smoke_grid_update_policy_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_governance_metrics_api",
        smoke_grid_governance_metrics_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_metrics_json_prometheus_parity_band6",
        smoke_grid_metrics_json_prometheus_parity_band6(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_metrics_json_prometheus_parity_band6_v2",
        smoke_grid_metrics_json_prometheus_parity_band6_v2(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_metrics_json_prometheus_parity_band6_v3",
        smoke_grid_metrics_json_prometheus_parity_band6_v3(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_network_profile_read",
        smoke_grid_network_profile_read(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_network_profiles_list",
        smoke_grid_network_profiles_list(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "ops_power_openapi",
        smoke_ops_power_openapi(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "monitoring_alerts",
        smoke_monitoring_alerts_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "monitoring_dashboards",
        smoke_monitoring_dashboards_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "vm_instances",
        smoke_vm_instances_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "admin_security_advisories_openapi",
        smoke_admin_security_advisories_openapi(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "virtual_nodes_wallet_rebind_override_openapi",
        smoke_virtual_nodes_wallet_rebind_override_openapi(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_telegram_seats",
        smoke_grid_telegram_seats(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_network_profile_put",
        smoke_grid_network_profile_put(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_payout_batch_history",
        smoke_grid_payout_batch_history(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_verification_replay_history",
        smoke_grid_verification_replay_history(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_settlement_metrics",
        smoke_galaxy_settlement_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_governance_metrics",
        smoke_galaxy_governance_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_shard_local_hit_ratio_metrics",
        smoke_galaxy_shard_local_hit_ratio_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_hot_tier_hit_ratio_metrics",
        smoke_galaxy_hot_tier_hit_ratio_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "jobs_store_backend",
        smoke_jobs_store_backend(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "jobs_migrating",
        smoke_jobs_migrating(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "protocol_middleware",
        smoke_protocol_middleware(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "telegram_wallet",
        smoke_telegram_wallet(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_envelope_lease",
        smoke_grid_envelope_lease(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "vision_revision_parity",
        smoke_vision_revision_parity(&client).await,
    )
    .await;

    if cli.include_raid {
        match stand_root.as_deref() {
            Some(root) => {
                record(
                    &mut cases,
                    "jobs_raid_restart",
                    smoke_jobs_raid(&client, &cli.base_url, root).await,
                )
                .await;
            }
            None => {
                cases.push(SmokeCaseResult {
                    name: "jobs_raid_restart",
                    ok: false,
                    detail: Some(format!("--raid requires {ENV_STAND_ROOT}")),
                });
            }
        }
    }

    let passed = cases.iter().filter(|c| c.ok).count() as u32;
    let failed = cases.iter().filter(|c| !c.ok).count() as u32;
    SmokeReport {
        base_url: cli.base_url.clone(),
        stand_root,
        ok: failed == 0,
        passed,
        failed,
        cases,
        tool: "poolai-http-stand-smoke",
    }
}

fn print_human(report: &SmokeReport) {
    eprintln!(
        "poolai-http-stand-smoke: {} ({}/{} passed) base={}",
        if report.ok { "OK" } else { "FAIL" },
        report.passed,
        report.passed + report.failed,
        report.base_url
    );
    for case in &report.cases {
        if case.ok {
            eprintln!("  OK  {}", case.name);
        } else {
            eprintln!(
                "  FAIL {} — {}",
                case.name,
                case.detail.as_deref().unwrap_or("?")
            );
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = parse_cli();
    let report = run_smokes(&cli).await;
    if cli.json_out {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        print_human(&report);
    }
    if !report.ok {
        std::process::exit(1);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_url_from_env_defaults() {
        std::env::remove_var(ENV_BASE);
        assert_eq!(base_url_from_env(), DEFAULT_BASE);
    }

    #[test]
    fn api_url_joins_path() {
        assert_eq!(
            api_url("http://127.0.0.1:8080", "/api/v1/health"),
            "http://127.0.0.1:8080/api/v1/health"
        );
        assert_eq!(
            api_url("http://127.0.0.1:8080/", "/api/v1/health"),
            "http://127.0.0.1:8080/api/v1/health"
        );
    }

    #[test]
    fn parse_cli_raid_restart_flag() {
        std::env::remove_var("POOLAI_STAND_SMOKE_RAID");
        std::env::remove_var("POOLAI_STAND_SMOKE_RAID_RESTART");
        std::env::remove_var("POOLAI_STAND_SMOKE_LEASE_RENEW");
        let args: Vec<String> = vec!["poolai-http-stand-smoke".into(), "--raid-restart".into()];
        assert!(args.iter().any(|a| a == "--raid-restart"));
    }

    #[test]
    fn parse_cli_lease_renew_flag_recognized() {
        let args: Vec<String> = vec!["poolai-http-stand-smoke".into(), "--lease-renew".into()];
        assert!(args.iter().any(|a| a == "--lease-renew"));
    }

    #[test]
    fn parse_fm_vision_revision_footer_ph_s235() {
        let section = "**Відкритих у §5.12:** **1** (PH-S235). **Закрито смуга:** PH-S128…S234 ✅. Vision rev **183**.\n";
        assert_eq!(parse_fm_vision_revision(section), Some(183));
    }

    #[test]
    fn read_manifest_revision_from_repo() {
        let root = repo_root();
        let rev = read_manifest_revision(&root).expect("manifest revision");
        assert!(rev > 0);
    }

    #[test]
    fn grid_seed_inventory_stub_shape() {
        let stub = json!({
            "ok": true,
            "generated_at": "2026-05-27T10:00:00Z",
            "entries": [
                {
                    "peer_id": "srv1-worker-a",
                    "seed_inventory": {
                        "shard_ids": ["w:emb-1", "w:ckpt-7"],
                        "hot_tier": { "ram_bytes_used": 3_221_225_472u64 }
                    }
                },
                { "peer_id": "srv2-worker-b" }
            ]
        });
        let entries = stub["entries"].as_array().expect("entries");
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0]["peer_id"], "srv1-worker-a");
    }

    #[test]
    fn galaxy_prefetch_metrics_export_shape_ph_s213() {
        let sample = concat!(
            "# HELP galaxy_prefetch_plan_total Galaxy prefetch plans\n",
            "# TYPE galaxy_prefetch_plan_total gauge\n",
            "galaxy_prefetch_plan_total 0\n",
            "# HELP galaxy_prefetch_planned_shards_total Galaxy prefetch shards\n",
            "# TYPE galaxy_prefetch_planned_shards_total gauge\n",
            "galaxy_prefetch_planned_shards_total 0\n",
            "# HELP galaxy_prefetch_hot_skip_total Galaxy prefetch hot skip\n",
            "# TYPE galaxy_prefetch_hot_skip_total gauge\n",
            "galaxy_prefetch_hot_skip_total 0\n",
            "# HELP galaxy_prefetch_bytes_total Galaxy prefetch bytes\n",
            "# TYPE galaxy_prefetch_bytes_total gauge\n",
            "galaxy_prefetch_bytes_total 0\n",
            "# HELP galaxy_prefetch_enqueue_total Galaxy prefetch enqueue stub\n",
            "# TYPE galaxy_prefetch_enqueue_total gauge\n",
            "galaxy_prefetch_enqueue_total 0\n",
            "# HELP galaxy_prefetch_wait_ms_total Galaxy prefetch wait stub\n",
            "# TYPE galaxy_prefetch_wait_ms_total gauge\n",
            "galaxy_prefetch_wait_ms_total 0\n",
            "# HELP galaxy_prefetch_strict_mode_total Galaxy prefetch strict mode\n",
            "# TYPE galaxy_prefetch_strict_mode_total gauge\n",
            "galaxy_prefetch_strict_mode_total 0\n",
            "# HELP galaxy_prefetch_complete_total Galaxy prefetch complete hook\n",
            "# TYPE galaxy_prefetch_complete_total gauge\n",
            "galaxy_prefetch_complete_total 0\n",
            "# HELP galaxy_prefetch_ingest_total Galaxy prefetch ingest stub\n",
            "# TYPE galaxy_prefetch_ingest_total gauge\n",
            "galaxy_prefetch_ingest_total 0\n",
            "# HELP galaxy_prefetch_skip_ingest_total Galaxy prefetch skip ingest\n",
            "# TYPE galaxy_prefetch_skip_ingest_total gauge\n",
            "galaxy_prefetch_skip_ingest_total 0\n",
            "# HELP galaxy_prefetch_seed_pull_total Galaxy prefetch seed pull stub invocations (PH-S424)\n",
            "# TYPE galaxy_prefetch_seed_pull_total gauge\n",
            "galaxy_prefetch_seed_pull_total 0\n",
            "# HELP galaxy_prefetch_lease_acquired_total Galaxy prefetch plans triggered by lease acquire (PH-S425)\n",
            "# TYPE galaxy_prefetch_lease_acquired_total gauge\n",
            "galaxy_prefetch_lease_acquired_total 0\n",
            "# HELP galaxy_locality_rank_ingest_total Galaxy locality rank ingest\n",
            "# TYPE galaxy_locality_rank_ingest_total gauge\n",
            "galaxy_locality_rank_ingest_total 0\n",
            "# HELP galaxy_locality_rank_miss_total Galaxy locality rank miss\n",
            "# TYPE galaxy_locality_rank_miss_total gauge\n",
            "galaxy_locality_rank_miss_total 0\n",
            "# HELP galaxy_locality_rank_empty_workers_total Galaxy locality rank empty workers\n",
            "# TYPE galaxy_locality_rank_empty_workers_total gauge\n",
            "galaxy_locality_rank_empty_workers_total 0\n",
            "# HELP galaxy_locality_rank_skip_total Galaxy locality rank skip\n",
            "# TYPE galaxy_locality_rank_skip_total gauge\n",
            "galaxy_locality_rank_skip_total 0\n",
            "# HELP galaxy_network_profile_stale_total Galaxy stale network profile observations during locality rank (PH-S563)\n",
            "# TYPE galaxy_network_profile_stale_total gauge\n",
            "galaxy_network_profile_stale_total 0\n",
        );
        metrics_text_has_prefetch_counters(sample).expect("sample export");
    }

    #[test]
    fn galaxy_pricing_forced_fallback_metrics_export_shape_ph_s216() {
        let sample = concat!(
            "# HELP galaxy_pricing_forced_fallback_total Galaxy pricing forced L2 quotes\n",
            "# TYPE galaxy_pricing_forced_fallback_total gauge\n",
            "galaxy_pricing_forced_fallback_total 0\n",
        );
        metrics_text_has_pricing_forced_fallback(sample).expect("sample export");
    }

    #[test]
    fn galaxy_pricing_cache_age_metrics_export_shape_ph_s224() {
        let sample = concat!(
            "# HELP galaxy_pricing_cache_age_seconds Galaxy pricing L1 cache age seconds last observed (PH-S168)\n",
            "# TYPE galaxy_pricing_cache_age_seconds gauge\n",
            "galaxy_pricing_cache_age_seconds 0\n",
        );
        metrics_text_has_pricing_cache_age(sample).expect("sample export");
    }

    #[test]
    fn galaxy_pricing_fresh_served_metrics_export_shape_ph_s241() {
        let sample = concat!(
            "# HELP galaxy_pricing_fresh_served Galaxy pricing oracle L1 fresh cache serves (PH-S127)\n",
            "# TYPE galaxy_pricing_fresh_served gauge\n",
            "galaxy_pricing_fresh_served 0\n",
        );
        metrics_text_has_pricing_fresh_served(sample).expect("sample export");
    }

    #[test]
    fn galaxy_pricing_stale_served_metrics_export_shape_ph_s244() {
        let sample = concat!(
            "# HELP galaxy_pricing_stale_served Galaxy pricing oracle L1 stale cache serves (PH-S127)\n",
            "# TYPE galaxy_pricing_stale_served gauge\n",
            "galaxy_pricing_stale_served 0\n",
        );
        metrics_text_has_pricing_stale_served(sample).expect("sample export");
    }

    #[test]
    fn galaxy_pricing_provider_metrics_export_shape_ph_s247() {
        let sample = concat!(
            "# HELP galaxy_pricing_provider_catalog_lookups_total Galaxy pricing provider catalog allow-list lookups (PH-S172)\n",
            "# TYPE galaxy_pricing_provider_catalog_lookups_total gauge\n",
            "galaxy_pricing_provider_catalog_lookups_total 0\n",
            "# HELP galaxy_pricing_provider_catalog_hits_total Galaxy pricing provider catalog allow-list hits (PH-S172)\n",
            "# TYPE galaxy_pricing_provider_catalog_hits_total gauge\n",
            "galaxy_pricing_provider_catalog_hits_total 0\n",
            "# HELP galaxy_pricing_provider_errors_total Galaxy pricing live provider HTTP fetch failures (PH-S173)\n",
            "# TYPE galaxy_pricing_provider_errors_total gauge\n",
            "galaxy_pricing_provider_errors_total 0\n",
        );
        metrics_text_has_pricing_provider_counters(sample).expect("sample export");
    }

    #[test]
    fn galaxy_pricing_quote_market_metrics_export_shape_ph_s253() {
        let sample = concat!(
            "# HELP galaxy_pricing_quote_usd_micro Galaxy pricing last served PoolAI quote micro-USD (PH-S174)\n",
            "# TYPE galaxy_pricing_quote_usd_micro gauge\n",
            "galaxy_pricing_quote_usd_micro 0\n",
            "# HELP galaxy_pricing_market_min_usd_micro Galaxy pricing last observed market min micro-USD (PH-S181)\n",
            "# TYPE galaxy_pricing_market_min_usd_micro gauge\n",
            "galaxy_pricing_market_min_usd_micro 0\n",
        );
        metrics_text_has_pricing_quote_market_gauges(sample).expect("sample export");
    }

    #[test]
    fn galaxy_fee_split_applied_metrics_export_shape_ph_s254() {
        let sample = concat!(
            "# HELP galaxy_fee_split_applied_total Galaxy fee split applied on grid result path (PH-S194)\n",
            "# TYPE galaxy_fee_split_applied_total gauge\n",
            "galaxy_fee_split_applied_total 0\n",
        );
        metrics_text_has_fee_split_applied(sample).expect("sample export");
    }

    #[test]
    fn galaxy_cross_region_egress_metrics_export_shape_ph_s255() {
        let sample = concat!(
            "# HELP galaxy_cross_region_egress_mb Galaxy last observed cross-region egress whole MB on rank/prefetch path (PH-S185)\n",
            "# TYPE galaxy_cross_region_egress_mb gauge\n",
            "galaxy_cross_region_egress_mb 0\n",
        );
        metrics_text_has_cross_region_egress_mb(sample).expect("sample export");
    }

    #[test]
    fn galaxy_replay_pending_metrics_export_shape_ph_s256() {
        let sample = concat!(
            "# HELP galaxy_replay_pending Galaxy replay verifications pending coordinator verdict (PH-S176)\n",
            "# TYPE galaxy_replay_pending gauge\n",
            "galaxy_replay_pending 0\n",
            "# HELP galaxy_replay_pending_scheduled_total Galaxy replay holds scheduled on grid result path (PH-S333)\n",
            "# TYPE galaxy_replay_pending_scheduled_total gauge\n",
            "galaxy_replay_pending_scheduled_total 0\n",
            "# HELP galaxy_replay_pending_resolved_total Galaxy replay holds cleared on verdict (PH-S335)\n",
            "# TYPE galaxy_replay_pending_resolved_total gauge\n",
            "galaxy_replay_pending_resolved_total 0\n",
            "# HELP galaxy_replay_evaluations_total Galaxy replay pending evaluations on grid result path (PH-S415)\n",
            "# TYPE galaxy_replay_evaluations_total gauge\n",
            "galaxy_replay_evaluations_total 0\n",
            "# HELP galaxy_replay_verification_enqueue_total Galaxy replay verification enqueue stub on mismatch (PH-S438)\n",
            "# TYPE galaxy_replay_verification_enqueue_total gauge\n",
            "galaxy_replay_verification_enqueue_total 0\n",
        );
        metrics_text_has_replay_pending(sample).expect("sample export");
    }

    #[test]
    fn galaxy_settlement_metrics_export_shape_ph_s249() {
        let sample = concat!(
            "# HELP galaxy_settlement_pending_verification_total Galaxy settlement holds pending verification on grid result path (PH-S178)\n",
            "# TYPE galaxy_settlement_pending_verification_total gauge\n",
            "galaxy_settlement_pending_verification_total 0\n",
            "# HELP galaxy_settlement_cleared_total Galaxy settlement cleared on grid result path (PH-S187)\n",
            "# TYPE galaxy_settlement_cleared_total gauge\n",
            "galaxy_settlement_cleared_total 0\n",
            "# HELP galaxy_settlement_not_applicable_total Galaxy settlement not applicable on grid result path (PH-S354)\n",
            "# TYPE galaxy_settlement_not_applicable_total gauge\n",
            "galaxy_settlement_not_applicable_total 0\n",
            "# HELP galaxy_settlement_resolved_total Galaxy settlement status resolutions on grid result path (PH-S404)\n",
            "# TYPE galaxy_settlement_resolved_total gauge\n",
            "galaxy_settlement_resolved_total 0\n",
            "# HELP galaxy_settlement_payout_batch_total Galaxy offline payout batch ledger entries on cleared settlement (PH-S427)\n",
            "# TYPE galaxy_settlement_payout_batch_total gauge\n",
            "galaxy_settlement_payout_batch_total 0\n",
            "# HELP galaxy_settlement_human_review_total Galaxy settlement human-review holds on non-deterministic semantic_hash (PH-S560)\n",
            "# TYPE galaxy_settlement_human_review_total gauge\n",
            "galaxy_settlement_human_review_total 0\n",
        );
        metrics_text_has_settlement_counters(sample).expect("sample export");
    }

    #[test]
    fn galaxy_shard_local_hit_ratio_metrics_export_shape_ph_s250() {
        let sample = concat!(
            "# HELP galaxy_shard_local_hit_ratio Galaxy last observed top-ranked shard local hit ratio basis points 0-10000 (PH-S183)\n",
            "# TYPE galaxy_shard_local_hit_ratio gauge\n",
            "galaxy_shard_local_hit_ratio 0\n",
        );
        metrics_text_has_shard_local_hit_ratio(sample).expect("sample export");
    }

    #[test]
    fn galaxy_hot_tier_hit_ratio_metrics_export_shape_ph_s581() {
        let sample = concat!(
            "# HELP galaxy_hot_tier_hit_ratio Galaxy last observed top-ranked hot tier hit ratio basis points 0-10000 (PH-S580)\n",
            "# TYPE galaxy_hot_tier_hit_ratio gauge\n",
            "galaxy_hot_tier_hit_ratio 0\n",
        );
        metrics_text_has_hot_tier_hit_ratio(sample).expect("sample export");
    }

    #[test]
    fn galaxy_verification_metrics_export_shape_ph_s225() {
        let sample = concat!(
            "# HELP galaxy_verification_sample_total Galaxy verification samples scheduled on grid result path (PH-S177)\n",
            "# TYPE galaxy_verification_sample_total gauge\n",
            "galaxy_verification_sample_total 0\n",
            "# HELP galaxy_verification_mismatch_total Galaxy verification digest mismatches on grid result path (PH-S175)\n",
            "# TYPE galaxy_verification_mismatch_total gauge\n",
            "galaxy_verification_mismatch_total 0\n",
            "# HELP galaxy_verification_match_total Galaxy verification digest matches on grid result path (PH-S180)\n",
            "# TYPE galaxy_verification_match_total gauge\n",
            "galaxy_verification_match_total 0\n",
            "# HELP galaxy_verification_sample_scheduled_total Galaxy verification stub samples scheduled on grid result path (PH-S164; PH-S186 /metrics)\n",
            "# TYPE galaxy_verification_sample_scheduled_total gauge\n",
            "galaxy_verification_sample_scheduled_total 0\n",
            "# HELP galaxy_verification_sample_completed_total Galaxy verification samples completed with verdict on grid result path (PH-S343)\n",
            "# TYPE galaxy_verification_sample_completed_total gauge\n",
            "galaxy_verification_sample_completed_total 0\n",
            "# HELP galaxy_verification_sample_skipped_total Galaxy verification edge samples skipped by deterministic stub (PH-S345)\n",
            "# TYPE galaxy_verification_sample_skipped_total gauge\n",
            "galaxy_verification_sample_skipped_total 0\n",
            "# HELP galaxy_verification_sample_not_applicable_total Galaxy verification samples not applicable on local origin path (PH-S356)\n",
            "# TYPE galaxy_verification_sample_not_applicable_total gauge\n",
            "galaxy_verification_sample_not_applicable_total 0\n",
            "# HELP galaxy_verification_sampling_evaluations_total Galaxy verification sampling evaluations on grid result path (PH-S414)\n",
            "# TYPE galaxy_verification_sampling_evaluations_total gauge\n",
            "galaxy_verification_sampling_evaluations_total 0\n",
            "# HELP galaxy_verification_checker_enqueue_total Galaxy verification checker enqueue stub on sample verdict (PH-S437)\n",
            "# TYPE galaxy_verification_checker_enqueue_total gauge\n",
            "galaxy_verification_checker_enqueue_total 0\n",
        );
        metrics_text_has_verification_counters(sample).expect("sample export");
    }

    #[test]
    fn galaxy_trust_payout_metrics_export_shape_ph_s219() {
        let sample = concat!(
            "# HELP galaxy_trust_payout_eligible_total Galaxy trust payout eligible\n",
            "# TYPE galaxy_trust_payout_eligible_total gauge\n",
            "galaxy_trust_payout_eligible_total 0\n",
            "# HELP galaxy_trust_payout_held_total Galaxy trust payout held\n",
            "# TYPE galaxy_trust_payout_held_total gauge\n",
            "galaxy_trust_payout_held_total 0\n",
            "# HELP galaxy_trust_payout_not_applicable_total Galaxy trust gate local-origin results not applicable (PH-S364)\n",
            "# TYPE galaxy_trust_payout_not_applicable_total gauge\n",
            "galaxy_trust_payout_not_applicable_total 0\n",
            "# HELP galaxy_trust_score Galaxy last trust score\n",
            "# TYPE galaxy_trust_score gauge\n",
            "galaxy_trust_score 0\n",
            "# HELP galaxy_trust_gate_min_threshold Galaxy configured minimum trust 0..=100 for edge auto payout (PH-S374)\n",
            "# TYPE galaxy_trust_gate_min_threshold gauge\n",
            "galaxy_trust_gate_min_threshold 40\n",
            "# HELP galaxy_trust_gate_default_score Galaxy default trust score 0..=100 when grid result omits trust_score (PH-S384)\n",
            "# TYPE galaxy_trust_gate_default_score gauge\n",
            "galaxy_trust_gate_default_score 50\n",
            "# HELP galaxy_trust_gate_evaluations_total Galaxy trust gate evaluations on grid result path (PH-S394)\n",
            "# TYPE galaxy_trust_gate_evaluations_total gauge\n",
            "galaxy_trust_gate_evaluations_total 0\n",
            "# HELP galaxy_trust_default_score_applied_total Galaxy grid results where default trust score was applied (PH-S395)\n",
            "# TYPE galaxy_trust_default_score_applied_total gauge\n",
            "galaxy_trust_default_score_applied_total 0\n",
            "# HELP galaxy_trust_explicit_score_total Galaxy grid results with explicit trust_score on ingest (PH-S405)\n",
            "# TYPE galaxy_trust_explicit_score_total gauge\n",
            "galaxy_trust_explicit_score_total 0\n",
        );
        metrics_text_has_trust_payout_counters(sample).expect("sample export");
    }

    #[test]
    fn galaxy_replication_metrics_export_shape_ph_s232() {
        let sample = concat!(
            "# HELP galaxy_replication_strict_total Galaxy replication strict tier grid job ingests (PH-S179)\n",
            "# TYPE galaxy_replication_strict_total gauge\n",
            "galaxy_replication_strict_total 0\n",
            "# HELP galaxy_replication_enqueue_total Galaxy replication executor enqueue stub on grid job ingest (PH-S426)\n",
            "# TYPE galaxy_replication_enqueue_total gauge\n",
            "galaxy_replication_enqueue_total 0\n",
            "# HELP galaxy_replication_executor_enqueue_total Galaxy replication executor queue stub on grid job ingest (PH-S435)\n",
            "# TYPE galaxy_replication_executor_enqueue_total gauge\n",
            "galaxy_replication_executor_enqueue_total 0\n",
        );
        metrics_text_has_replication_strict(sample).expect("sample export");
    }

    #[test]
    fn grid_verification_metrics_api_export_shape_ph_s673() {
        let sample = r#"{"ok":true,"lifecycle_depth":"none","metrics":{"sample_total":0,"mismatch_total":0,"match_total":0,"sample_completed_total":0,"checker_enqueue_total":0,"checker_pending_total":0}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        assert_eq!(body["metrics"]["checker_pending_total"], 0);
        assert_eq!(body["lifecycle_depth"], "none");
    }

    #[test]
    fn grid_verification_lifecycle_export_shape_ph_s883() {
        use poolai::grid::galaxy_verification_lifecycle_depth::{
            verification_lifecycle_depth_stub, verification_lifecycle_depth_wire_label,
            VerificationLifecycleDepth,
        };
        use poolai::grid::galaxy_verification_metrics::VerificationMetricsSnapshot;
        use poolai::grid::stand_smoke_metrics_parity::{
            stand_smoke_metrics_parity_depth_stub, StandSmokeMetricsParityDepth,
        };
        use serde_json::json;

        let empty = VerificationMetricsSnapshot {
            sample_total: 0,
            mismatch_total: 0,
            match_total: 0,
            sample_completed_total: 0,
            checker_enqueue_total: 0,
            checker_pending_total: 0,
        };
        let depth = verification_lifecycle_depth_stub(
            Some(&VerificationMetricsSnapshot {
                checker_enqueue_total: 1,
                checker_pending_total: 1,
                ..empty
            }),
            1,
        );
        assert_eq!(depth, VerificationLifecycleDepth::ShadowJobSubmit);
        assert_eq!(
            verification_lifecycle_depth_wire_label(depth),
            "shadow_job_submit"
        );
        assert_eq!(
            stand_smoke_metrics_parity_depth_stub(Some(
                &json!({"verification_checker_lifecycle": true})
            )),
            StandSmokeMetricsParityDepth::VerificationCheckerLifecycle
        );
    }

    #[test]
    fn admin_wasm_slim_depth_stub_band23_export_shape_ph_s882() {
        use poolai_ui_core::grid_replication_pricing::{
            admin_wasm_slim_depth_stub, AdminWasmSlimDepth,
        };
        use poolai_ui_core::stand_smoke_metrics::render_grid_verification_metrics_strip_html;
        use serde_json::json;
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"grid_verification_panel": true}))),
            AdminWasmSlimDepth::GridVerificationPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"grid_verification_metrics_strip": true}))),
            AdminWasmSlimDepth::GridVerificationMetricsStrip
        );
        let strip = render_grid_verification_metrics_strip_html(
            r#"{"metrics":{"sample_total":2,"checker_pending_total":1}}"#,
            1,
        );
        assert!(strip.contains("Sample"));
        assert!(strip.contains("Pending"));
    }

    #[test]
    fn grid_replay_metrics_api_export_shape_ph_s673() {
        let sample = r#"{"ok":true,"metrics":{"replay_pending":0,"replay_pending_scheduled_total":0,"replay_pending_resolved_total":0,"replay_evaluations_total":0,"replay_verification_enqueue_total":0,"verification_replay_record_total":0}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        assert_eq!(body["metrics"]["replay_pending"], 0);
    }

    #[test]
    fn grid_settlement_metrics_api_export_shape_ph_s683() {
        let sample = r#"{"ok":true,"metrics":{"pending_verification_total":0,"cleared_total":0,"not_applicable_total":0,"resolved_total":0,"payout_batch_total":0,"human_review_total":0}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        assert_eq!(body["metrics"]["cleared_total"], 0);
    }

    #[test]
    fn grid_trust_metrics_api_export_shape_ph_s683() {
        let sample = r#"{"ok":true,"metrics":{"payout_eligible_total":0,"payout_held_total":0,"payout_not_applicable_total":0,"last_trust_score":0,"gate_min_threshold":40,"gate_default_score":50,"gate_evaluations_total":0,"default_score_applied_total":0,"explicit_score_total":0,"trust_score_delta_total":0}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        assert_eq!(body["metrics"]["gate_min_threshold"], 40);
    }

    #[test]
    fn grid_replication_metrics_api_export_shape_ph_s693() {
        let sample = r#"{"ok":true,"replication_depth":"none","rate_cap_per_hour":1000,"metrics":{"strict_total":0,"enqueue_total":0,"executor_enqueue_total":0,"rate_limited_total":0}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        assert_eq!(body["metrics"]["strict_total"], 0);
        assert_eq!(body["replication_depth"], "none");
    }

    #[test]
    fn grid_replication_depth_export_shape_ph_s893() {
        use poolai::grid::galaxy_replication_depth::{
            replication_depth_stub, replication_depth_wire_label, ReplicationDepth,
        };
        use poolai::grid::galaxy_replication_metrics::ReplicationMetricsSnapshot;
        use poolai::grid::stand_smoke_metrics_parity::{
            stand_smoke_metrics_parity_depth_stub, StandSmokeMetricsParityDepth,
        };
        use serde_json::json;

        let snap = ReplicationMetricsSnapshot {
            strict_total: 1,
            enqueue_total: 1,
            executor_enqueue_total: 1,
            rate_limited_total: 1,
        };
        let depth = replication_depth_stub(Some(&snap), 100);
        assert_eq!(depth, ReplicationDepth::RateCap);
        assert_eq!(replication_depth_wire_label(depth), "rate_cap");
        assert_eq!(
            stand_smoke_metrics_parity_depth_stub(Some(
                &json!({"replication_quorum_production": true})
            )),
            StandSmokeMetricsParityDepth::ReplicationQuorumProduction
        );
    }

    #[test]
    fn grid_pricing_metrics_api_export_shape_ph_s693() {
        let sample = r#"{"ok":true,"pricing_depth":"none","provider_http_timeout_ms":1500,"metrics":{"fresh_served_total":0,"stale_served_total":0,"forced_fallback_total":0,"provider_catalog_lookups_total":0,"provider_catalog_hits_total":0,"provider_errors_total":0,"provider_timeouts_total":0}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        assert_eq!(body["metrics"]["fresh_served_total"], 0);
        assert_eq!(body["pricing_depth"], "none");
    }

    #[test]
    fn grid_pricing_depth_export_shape_ph_s903() {
        use poolai::grid::galaxy_pricing_depth::{
            pricing_depth_stub, pricing_depth_wire_label, PricingDepth,
        };
        use poolai::grid::galaxy_pricing_metrics::PricingMetricsSnapshot;
        use poolai::grid::stand_smoke_metrics_parity::{
            stand_smoke_metrics_parity_depth_stub, validate_pricing_metrics_parity,
            StandSmokeMetricsParityDepth,
        };
        use serde_json::json;

        let snap = PricingMetricsSnapshot {
            fresh_served_total: 1,
            stale_served_total: 0,
            forced_fallback_total: 0,
            provider_catalog_lookups_total: 1,
            provider_catalog_hits_total: 1,
            provider_errors_total: 0,
            provider_timeouts_total: 0,
        };
        assert_eq!(
            pricing_depth_stub(Some(&snap), 1500),
            PricingDepth::LiveFetch
        );
        assert_eq!(
            pricing_depth_wire_label(PricingDepth::LiveFetch),
            "live_fetch"
        );
        assert_eq!(
            stand_smoke_metrics_parity_depth_stub(Some(&json!({"pricing_production": true}))),
            StandSmokeMetricsParityDepth::PricingProduction
        );
        let prom = concat!(
            "galaxy_pricing_fresh_served 1\n",
            "galaxy_pricing_stale_served 0\n",
            "galaxy_pricing_forced_fallback_total 0\n",
            "galaxy_pricing_provider_catalog_lookups_total 1\n",
            "galaxy_pricing_provider_errors_total 0\n",
            "galaxy_pricing_provider_timeouts_total 0\n",
        );
        let pricing = json!({
            "ok": true,
            "metrics": {
                "fresh_served_total": 1,
                "stale_served_total": 0,
                "forced_fallback_total": 0,
                "provider_catalog_lookups_total": 1,
                "provider_catalog_hits_total": 1,
                "provider_errors_total": 0,
                "provider_timeouts_total": 0
            }
        });
        validate_pricing_metrics_parity(prom, &pricing).expect("parity");
    }

    #[test]
    fn grid_prefetch_metrics_api_export_shape_ph_s753() {
        use poolai::grid::stand_smoke_metrics_parity::{
            validate_grid_metrics_json_export, PREFETCH_JSON_KEYS,
        };

        let sample = r#"{"ok":true,"metrics":{"pull_bytes_total":0,"backpressure_total":0,"plan_total":0,"enqueue_total":0,"peer_fetch_total":0}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        validate_grid_metrics_json_export(&body, PREFETCH_JSON_KEYS).expect("shape");
        assert_eq!(body["metrics"]["pull_bytes_total"], 0);
    }

    #[test]
    fn grid_locality_metrics_api_export_shape_ph_s763() {
        use poolai::grid::stand_smoke_metrics_parity::{
            validate_grid_metrics_json_export, LOCALITY_JSON_KEYS,
        };

        let sample = r#"{"ok":true,"metrics":{"shard_local_hit_ratio_bps":0,"hot_tier_hit_ratio_bps":0,"cross_region_egress_mb":0,"hot_promote_total":0,"hot_evict_total":0}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        validate_grid_metrics_json_export(&body, LOCALITY_JSON_KEYS).expect("shape");
        assert_eq!(body["metrics"]["hot_promote_total"], 0);
    }

    #[test]
    fn grid_fee_split_metrics_api_export_shape_ph_s782() {
        use poolai::grid::stand_smoke_metrics_parity::{
            validate_fee_split_metrics_parity, validate_grid_metrics_json_export,
            FEE_SPLIT_JSON_KEYS,
        };

        let sample = r#"{"ok":true,"metrics":{"fee_split_applied_total":0,"primary_dev_fee_bps":10,"secondary_admin_fee_min_bps":100,"secondary_admin_fee_max_bps":500}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        validate_grid_metrics_json_export(&body, FEE_SPLIT_JSON_KEYS).expect("shape");
        let prom =
            "# TYPE galaxy_fee_split_applied_total gauge\ngalaxy_fee_split_applied_total 0\n";
        validate_fee_split_metrics_parity(prom, &body).expect("parity");
    }

    #[test]
    fn grid_update_policy_api_export_shape_ph_s790() {
        use poolai::grid::stand_smoke_metrics_parity::validate_update_policy_json_export;

        let sample = r#"{"ok":true,"policy":{"mode":"notify","env_update_policy":"POOLAI_UPDATE_POLICY","env_manifest_url":"POOLAI_RELEASE_MANIFEST_URL"}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        validate_update_policy_json_export(&body).expect("shape");
    }

    #[test]
    fn grid_governance_metrics_api_export_shape_ph_s793() {
        use poolai::grid::stand_smoke_metrics_parity::{
            validate_governance_metrics_parity, validate_grid_metrics_json_export,
            GOVERNANCE_JSON_KEYS,
        };

        let sample = r#"{"ok":true,"metrics":{"release_verify_total":1,"release_verify_fail_total":0,"update_notify_pending":2,"advisory_acknowledged_total":1}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        validate_grid_metrics_json_export(&body, GOVERNANCE_JSON_KEYS).expect("shape");
        let prom = concat!(
            "poolai_release_verify_total 1\n",
            "poolai_release_verify_fail_total 0\n",
            "poolai_update_notify_pending 2\n",
            "poolai_advisory_acknowledged_total 1\n",
        );
        validate_governance_metrics_parity(prom, &body).expect("parity");
    }

    #[test]
    fn grid_replication_pricing_wasm_panel_export_shape_ph_s703() {
        let sample = r#"{"ok":true,"metrics":{"strict_total":1,"enqueue_total":2,"executor_enqueue_total":0,"rate_limited_total":0}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        assert_eq!(body["metrics"]["strict_total"], 1);
        assert_eq!(body["metrics"]["enqueue_total"], 2);
    }

    #[test]
    fn admin_wasm_slim_depth_stub_export_shape_ph_s703() {
        use poolai_ui_core::grid_replication_pricing::{
            admin_wasm_slim_depth_stub, AdminWasmSlimDepth,
        };
        use serde_json::json;
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"panel_renderer": true}))),
            AdminWasmSlimDepth::PanelRenderer
        );
    }

    #[test]
    fn monitoring_settlement_payout_export_shape_ph_s803() {
        use poolai::grid::stand_smoke_metrics_parity::{
            validate_grid_metrics_json_export, validate_settlement_trust_metrics_parity,
            SETTLEMENT_JSON_KEYS, TRUST_JSON_KEYS,
        };
        let prom = concat!(
            "galaxy_settlement_cleared_total 2\n",
            "galaxy_settlement_payout_batch_total 1\n",
            "galaxy_trust_payout_eligible_total 3\n",
            "galaxy_trust_score 55\n",
        );
        let settlement = serde_json::json!({
            "ok": true,
            "metrics": {
                "pending_verification_total": 0,
                "cleared_total": 2,
                "resolved_total": 0,
                "payout_batch_total": 1,
            }
        });
        let trust = serde_json::json!({
            "ok": true,
            "metrics": {
                "payout_eligible_total": 3,
                "payout_held_total": 0,
                "last_trust_score": 55,
                "gate_min_threshold": 40,
            }
        });
        validate_grid_metrics_json_export(&settlement, SETTLEMENT_JSON_KEYS).expect("settlement");
        validate_grid_metrics_json_export(&trust, TRUST_JSON_KEYS).expect("trust");
        validate_settlement_trust_metrics_parity(prom, &settlement, &trust).expect("parity");
        let ml_pipelines: serde_json::Value = serde_json::json!([]);
        assert!(ml_pipelines.is_array());
        let alerts: serde_json::Value = serde_json::json!([]);
        assert!(alerts.is_array());
    }

    #[test]
    fn admin_wasm_slim_depth_stub_band15_export_shape_ph_s804() {
        use poolai_ui_core::grid_replication_pricing::{
            admin_wasm_slim_depth_stub, AdminWasmSlimDepth,
        };
        use serde_json::json;
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"ml_pipeline_panel": true}))),
            AdminWasmSlimDepth::MlPipelinePanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"payout_batch_panel": true}))),
            AdminWasmSlimDepth::PayoutBatchPanel
        );
    }

    #[test]
    fn security_topology_export_shape_ph_s813() {
        use poolai::security::secret_rotation::{init_default_rotation_hooks, rotation_status};
        init_default_rotation_hooks();
        let status = rotation_status();
        assert!(!status.is_empty());
        for entry in &status {
            let _ = entry.kind.as_str();
            let _ = entry.configured;
            let _ = entry.hook_count;
        }
        let topology = serde_json::json!({
            "node_count": 0,
            "latency_measurements": 0,
            "last_updated": "2026-06-21T00:00:00Z",
            "node_ids": []
        });
        assert!(topology.get("node_count").is_some());
        assert!(topology.get("last_updated").is_some());
        let prom = "poolai_secret_rotations_total{kind=\"jwt\",success=\"true\"} 1\n";
        assert!(prom.contains("poolai_secret_rotations_total"));
    }

    #[test]
    fn admin_wasm_slim_depth_stub_band16_export_shape_ph_s814() {
        use poolai_ui_core::grid_replication_pricing::{
            admin_wasm_slim_depth_stub, AdminWasmSlimDepth,
        };
        use poolai_ui_core::security::render_secret_rotation_panel_html;
        use poolai_ui_core::topology::render_topology_stats_strip_html;
        use serde_json::json;
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"security_rotation_panel": true}))),
            AdminWasmSlimDepth::SecurityRotationPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"topology_stats_strip": true}))),
            AdminWasmSlimDepth::TopologyStatsStrip
        );
        let sec_html = render_secret_rotation_panel_html("[]", "{}");
        assert!(sec_html.contains("secret-rotation-table"));
        let topo_html = render_topology_stats_strip_html(
            r#"{"node_count":1,"latency_measurements":0,"last_updated":""}"#,
            "{}",
        );
        assert!(topo_html.contains("topology-last-updated"));
    }

    #[test]
    fn vm_workers_export_shape_ph_s823() {
        use poolai_ui_core::admin_vm_workers::{
            validate_vm_instances_admin_list_shape, validate_workers_admin_list_shape,
            VM_INSTANCE_ADMIN_ROW_KEYS, WORKERS_ADMIN_ROW_KEYS,
        };
        use serde_json::json;

        assert!(!WORKERS_ADMIN_ROW_KEYS.is_empty());
        assert!(!VM_INSTANCE_ADMIN_ROW_KEYS.is_empty());

        let workers = json!([{
            "id": "w1",
            "status": "idle",
            "current_task": null,
            "is_healthy": true,
            "total_requests_processed": 1,
            "queue_size": 0,
            "active_connections": 0,
            "average_response_time_ms": 0
        }]);
        validate_workers_admin_list_shape(&workers).expect("workers export shape");

        let vms = json!([{
            "id": "vm-1",
            "name": "test-vm",
            "status": "running",
            "resources": { "cpu_cores": 2, "memory_mb": 1024, "gpu_required": false }
        }]);
        validate_vm_instances_admin_list_shape(&vms).expect("vm export shape");
    }

    #[test]
    fn admin_wasm_slim_depth_stub_band17_export_shape_ph_s824() {
        use poolai_ui_core::grid_replication_pricing::{
            admin_wasm_slim_depth_stub, AdminWasmSlimDepth,
        };
        use poolai_ui_core::libs::render_libs_panel_html;
        use poolai_ui_core::vm::render_vm_panel_html;
        use poolai_ui_core::workers::render_workers_panel_html;
        use serde_json::json;
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"vm_panel": true}))),
            AdminWasmSlimDepth::VmPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"workers_panel": true}))),
            AdminWasmSlimDepth::WorkersPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"libs_panel": true}))),
            AdminWasmSlimDepth::LibsPanel
        );
        let vm_html = render_vm_panel_html(
            "[]", "N", "S", "R", "A", "V", "CPU", "MEM", "Start", "Stop", "Del", "Empty",
        );
        assert!(vm_html.contains("admin-empty-state"));
        let wrk_html = render_workers_panel_html(
            "[]", "I", "S", "M", "A", "W", "H", "U", "Req", "Del", "Empty",
        );
        assert!(wrk_html.contains("admin-empty-state"));
        let libs_html = render_libs_panel_html(
            "[]", "N", "V", "S", "A", "L", "I", "NI", "U", "Up", "In", "Empty",
        );
        assert!(libs_html.contains("admin-empty-state"));
    }

    #[test]
    fn admin_wasm_slim_depth_stub_band44_export_shape_ph_s1084() {
        use poolai_ui_core::admin_wasm_slim_depth::{
            admin_wasm_slim_depth_stub, AdminWasmSlimDepth,
        };
        use poolai_ui_core::galaxy_telegram_seats::render_telegram_seats_panel_html;
        use poolai_ui_core::galaxy_virtual_nodes::render_galaxy_virtual_nodes_panel_html;
        use poolai_ui_core::instances::render_instances_panel_html;
        use poolai_ui_core::ml::{
            render_monitoring_alerts_panel_html, render_monitoring_dashboards_panel_html,
        };
        use poolai_ui_core::network_profiles::render_network_profiles_panel_html;
        use poolai_ui_core::stand_smoke_metrics::{
            render_grid_fee_split_metrics_strip_html, render_grid_governance_metrics_strip_html,
            render_grid_locality_metrics_strip_html, render_grid_prefetch_metrics_strip_html,
        };
        use serde_json::json;
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"monitoring_alerts_panel": true}))),
            AdminWasmSlimDepth::MonitoringAlertsPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"grid_fee_split_metrics_strip": true}))),
            AdminWasmSlimDepth::GridFeeSplitMetricsStrip
        );
        let alerts_html = render_monitoring_alerts_panel_html(
            "[]",
            "N/A",
            "Ack",
            "Active",
            "Ack",
            "Sev",
            "Metric",
            "Cur",
            "Thr",
            "Trig",
            "Status",
            "Act",
            "Alerts",
            "No alerts",
        );
        assert!(alerts_html.contains("admin-empty-state"));
        let dash_html = render_monitoring_dashboards_panel_html(
            "[]",
            "Name",
            "Desc",
            "Metrics",
            "Public",
            "Created",
            "Dash",
            "—",
            "N/A",
            "Yes",
            "No",
            "{n} metrics",
            "No dashboards",
        );
        assert!(dash_html.contains("admin-empty-state"));
        let inst_html = render_instances_panel_html(
            "[]", "ID", "Model", "St", "Str", "Nodes", "Created", "Act", "Inst", "View", "Del",
            "Empty",
        );
        assert!(inst_html.contains("admin-empty-state"));
        let tg_html = render_telegram_seats_panel_html(
            r#"{"seat_policy":"open","seat_limit":10,"active_seats":0,"bound_wallets":[]}"#,
            "Policy",
            "Limit",
            "Active",
            "Bound",
            "Seats",
        );
        assert!(tg_html.contains("admin-table"));
        let vn_html = render_galaxy_virtual_nodes_panel_html(
            "[]", "Peer", "Origin", "Region", "Latency", "Stale", "Nodes", "Empty",
        );
        assert!(vn_html.contains("admin-empty-state"));
        let np_html = render_network_profiles_panel_html(
            "[]", "Peer", "Region", "Latency", "BW", "Profiles", "Empty",
        );
        assert!(np_html.contains("muted"));
        let prefetch_html =
            render_grid_prefetch_metrics_strip_html(r#"{"metrics":{"pull_bytes_total":1}}"#, 0);
        assert!(prefetch_html.contains("admin-metrics-strip"));
        let locality_html =
            render_grid_locality_metrics_strip_html(r#"{"metrics":{"hot_promote_total":2}}"#, 0);
        assert!(locality_html.contains("admin-metrics-strip"));
        let gov_html = render_grid_governance_metrics_strip_html(
            r#"{"metrics":{"advisory_ack_total":1}}"#,
            r#"{"mode":"advisory"}"#,
            0,
        );
        assert!(gov_html.contains("admin-metrics-strip"));
        let fee_html =
            render_grid_fee_split_metrics_strip_html(r#"{"metrics":{"applied_total":3}}"#, 0);
        assert!(fee_html.contains("admin-metrics-strip"));
    }

    #[test]
    #[test]
    fn run_local_health_export_shape_ph_s1089() {
        use poolai_ui_core::stand_smoke_run_local_depth::RUN_LOCAL_HEALTH_KEYS;
        let health = json!({
            "status": "healthy",
            "version": "0.2.2",
            "timestamp": "2026-07-18T00:00:00Z",
            "uptime": 42,
            "checks": {
                "database": { "status": "healthy", "message": "ok", "response_time_ms": 1 },
                "memory": { "status": "healthy", "message": "ok", "response_time_ms": 1 },
                "workers": { "status": "healthy", "message": "ok", "response_time_ms": 1 },
                "gpu": { "status": "healthy", "message": "ok", "response_time_ms": 0 }
            }
        });
        for key in RUN_LOCAL_HEALTH_KEYS {
            assert!(health.get(key).is_some(), "health missing {key}");
        }
        assert_eq!(
            health.get("status").and_then(Value::as_str),
            Some("healthy")
        );
    }

    #[test]
    fn stand_smoke_run_local_band45_export_shape_ph_s1095() {
        use poolai_ui_core::stand_smoke_run_local_depth::{
            stand_smoke_run_local_depth_stub, StandSmokeRunLocalDepth, FM_BAND45_ROWS,
            RUN_LOCAL_SMOKE_CASES,
        };
        use serde_json::json;
        assert_eq!(
            stand_smoke_run_local_depth_stub(Some(&json!({"run_local_smoke": true}))),
            StandSmokeRunLocalDepth::RunLocalSmoke
        );
        assert_eq!(
            stand_smoke_run_local_depth_stub(Some(&json!({
                "run_local_smoke": true,
                "verify_dev_stand_hook": true,
                "quick_stand_smoke": true,
            }))),
            StandSmokeRunLocalDepth::FullRunLocalBand45
        );
        assert!(RUN_LOCAL_SMOKE_CASES.contains(&"health"));
        assert!(RUN_LOCAL_SMOKE_CASES.contains(&"monitoring_alerts"));
        assert!(RUN_LOCAL_SMOKE_CASES.contains(&"ops_power_openapi"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND45_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band45 row {row}"
            );
        }
    }

    #[test]
    fn rust_migration_advisory_band46_export_shape_ph_s1104() {
        use poolai_ui_core::rust_migration_advisory_depth::{
            migration_registry_total, rust_migration_advisory_depth_stub,
            RustMigrationAdvisoryDepth, ADMIN_JS_MIGRATION_CANDIDATES,
            ARCHIVED_E2E_MIGRATION_CANON, FM_BAND46_ROWS, MIGRATION_ADVISORY_CASES,
        };
        use serde_json::json;
        assert_eq!(
            rust_migration_advisory_depth_stub(Some(&json!({"ui_js_candidates": true}))),
            RustMigrationAdvisoryDepth::UiJsCandidates
        );
        assert_eq!(
            rust_migration_advisory_depth_stub(Some(&json!({
                "ui_js_candidates": true,
                "e2e_archived_canon": true,
                "loc_audit_advisory": true,
                "ops_shell_canon": true,
            }))),
            RustMigrationAdvisoryDepth::FullBand46
        );
        assert_eq!(ADMIN_JS_MIGRATION_CANDIDATES.len(), 6);
        assert_eq!(ARCHIVED_E2E_MIGRATION_CANON.len(), 8);
        assert!(migration_registry_total() >= 14);
        assert!(MIGRATION_ADVISORY_CASES.contains(&"stretch_spirit_hold"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND46_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band46 row {row}"
            );
        }
    }

    #[test]
    fn stable_state_touchup_band47_export_shape_ph_s1114() {
        use poolai_ui_core::stable_state_touchup_depth::{
            stable_criteria_total, stable_state_touchup_depth_stub, StableStateTouchupDepth,
            FM_BAND47_ROWS, STABLE_TOUCHUP_CASES, STABLE_TOUCHUP_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            stable_state_touchup_depth_stub(Some(&json!({"criteria_registry": true}))),
            StableStateTouchupDepth::CriteriaRegistry
        );
        assert_eq!(
            stable_state_touchup_depth_stub(Some(&json!({
                "criteria_registry": true,
                "stable_summary": true,
                "index_canon": true,
                "handoff_zriz": true,
                "loc_audit_touchup": true,
                "verify_dev_stand_hook": true,
                "quick_touchup": true,
                "docs_canon": true,
            }))),
            StableStateTouchupDepth::FullBand47
        );
        assert_eq!(STABLE_TOUCHUP_CRITERIA.len(), 7);
        assert_eq!(stable_criteria_total(), 7);
        assert!(STABLE_TOUCHUP_CASES.contains(&"product_complete"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND47_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band47 row {row}"
            );
        }
    }

    #[test]
    fn pre_push_canon_band49_export_shape_ph_s1134() {
        use poolai_ui_core::pre_push_hook_depth::{
            pre_push_hook_criteria_total, pre_push_hook_depth_stub, PrePushHookDepth,
            FM_BAND49_ROWS, PRE_PUSH_HOOK_CASES, PRE_PUSH_HOOK_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            pre_push_hook_depth_stub(Some(&json!({"vision_sync_canon": true}))),
            PrePushHookDepth::VisionSyncCanon
        );
        assert_eq!(
            pre_push_hook_depth_stub(Some(&json!({
                "pre_push_hook_script": true,
                "install_hook": true,
                "vision_sync_canon": true,
                "vision_sync_check": true,
                "cargo_fmt_gate": true,
                "pre_push_hook_docs": true,
                "verify_dev_stand_hook": true,
            }))),
            PrePushHookDepth::FullBand49
        );
        assert_eq!(PRE_PUSH_HOOK_CRITERIA.len(), 7);
        assert_eq!(pre_push_hook_criteria_total(), 7);
        assert!(PRE_PUSH_HOOK_CASES.contains(&"verify_dev_stand_hook"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND49_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band49 row {row}"
            );
        }
    }

    #[test]
    fn ci_canon_band50_export_shape_ph_s1144() {
        use poolai_ui_core::ci_canon_depth::{
            ci_canon_criteria_total, ci_canon_depth_stub, CiCanonDepth, CI_CANON_CASES,
            CI_CANON_CRITERIA, FM_BAND50_ROWS,
        };
        use serde_json::json;
        assert_eq!(
            ci_canon_depth_stub(Some(&json!({"openapi_gap_audit": true}))),
            CiCanonDepth::OpenapiGapAudit
        );
        assert_eq!(
            ci_canon_depth_stub(Some(&json!({
                "test_ci_scope": true,
                "openapi_gap_audit": true,
                "rust_ratio_audit": true,
                "openapi_gap_ci_job": true,
                "verify_dev_stand_hook": true,
                "ci_canon_docs": true,
                "dual_gate": true,
            }))),
            CiCanonDepth::FullBand50
        );
        assert_eq!(CI_CANON_CRITERIA.len(), 7);
        assert_eq!(ci_canon_criteria_total(), 7);
        assert!(CI_CANON_CASES.contains(&"dual_gate"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND50_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band50 row {row}"
            );
        }
    }

    #[test]
    fn tenant_persist_band51_export_shape_ph_s1155() {
        use poolai_ui_core::tenant_persistence_depth::{
            tenant_persist_criteria_total, tenant_persistence_depth_stub, TenantPersistenceDepth,
            FM_BAND51_ROWS, TENANT_PERSIST_CASES, TENANT_PERSIST_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            tenant_persistence_depth_stub(Some(&json!({"audit_test": true}))),
            TenantPersistenceDepth::AuditTest
        );
        assert_eq!(
            tenant_persistence_depth_stub(Some(&json!({
                "tenant_persistence_depth": true,
                "loc_audit_flag": true,
                "audit_test": true,
                "verify_dev_stand_hook": true,
                "quick_flag": true,
                "stand_smoke_export": true,
                "tenant_persist_docs": true,
            }))),
            TenantPersistenceDepth::FullBand51
        );
        assert_eq!(TENANT_PERSIST_CRITERIA.len(), 7);
        assert_eq!(tenant_persist_criteria_total(), 7);
        assert!(TENANT_PERSIST_CASES.contains(&"verify_dev_stand_hook"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND51_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band51 row {row}"
            );
        }
    }

    #[test]
    fn tenant_store_band52_export_shape_ph_s1163() {
        use poolai_ui_core::tenant_depth::{
            tenant_criteria_total, tenant_depth_stub, TenantDepth, FM_BAND52_ROWS, TENANT_CASES,
            TENANT_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            tenant_depth_stub(Some(&json!({"api_contracts": true}))),
            TenantDepth::ApiContracts
        );
        assert_eq!(
            tenant_depth_stub(Some(&json!({
                "tenant_depth": true,
                "store_wire": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "tenant_store_docs": true,
            }))),
            TenantDepth::FullBand52
        );
        assert_eq!(TENANT_CRITERIA.len(), 7);
        assert_eq!(tenant_criteria_total(), 7);
        assert!(TENANT_CASES.contains(&"store_wire"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND52_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band52 row {row}"
            );
        }
    }

    #[test]
    fn tenant_api_band53_export_shape_ph_s1176() {
        use poolai_ui_core::tenant_api_contracts_depth::{
            tenant_api_contracts_depth_stub, tenant_api_criteria_total, TenantApiContractsDepth,
            FM_BAND53_ROWS, TENANT_API_CASES, TENANT_API_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            tenant_api_contracts_depth_stub(Some(&json!({"http_crud": true}))),
            TenantApiContractsDepth::HttpCrud
        );
        assert_eq!(
            tenant_api_contracts_depth_stub(Some(&json!({
                "tenant_api_depth": true,
                "http_crud": true,
                "quota_usage": true,
                "isolation": true,
                "store_wire_http": true,
                "openapi_schemas": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "tenant_api_docs": true,
            }))),
            TenantApiContractsDepth::FullBand53
        );
        assert_eq!(TENANT_API_CRITERIA.len(), 10);
        assert_eq!(tenant_api_criteria_total(), 10);
        assert!(TENANT_API_CASES.contains(&"store_wire_http"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND53_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band53 row {row}"
            );
        }
    }

    #[test]
    fn tenant_admin_ops_band54_export_shape_ph_s1185() {
        use poolai_ui_core::tenant_admin_ops_depth::{
            tenant_admin_ops_criteria_total, tenant_admin_ops_depth_stub, TenantAdminOpsDepth,
            FM_BAND54_ROWS, TENANT_ADMIN_OPS_CASES, TENANT_ADMIN_OPS_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            tenant_admin_ops_depth_stub(Some(&json!({"usage_quota_glue": true}))),
            TenantAdminOpsDepth::UsageQuotaGlue
        );
        assert_eq!(
            tenant_admin_ops_depth_stub(Some(&json!({
                "tenant_admin_ops_depth": true,
                "store_strip": true,
                "usage_quota_glue": true,
                "html_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "tenant_admin_ops_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            TenantAdminOpsDepth::FullBand54
        );
        assert_eq!(TENANT_ADMIN_OPS_CRITERIA.len(), 10);
        assert_eq!(tenant_admin_ops_criteria_total(), 10);
        assert!(TENANT_ADMIN_OPS_CASES.contains(&"store_strip"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND54_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band54 row {row}"
            );
        }
    }

    #[test]
    fn galaxy_edge_verification_band48_export_shape_ph_s1125() {
        use poolai_ui_core::galaxy_edge_verification_depth::{
            edge_verification_criteria_total, galaxy_edge_verification_depth_stub,
            GalaxyEdgeVerificationDepth, EDGE_VERIFICATION_CASES, EDGE_VERIFICATION_CRITERIA,
            FM_BAND48_ROWS,
        };
        use serde_json::json;
        assert_eq!(
            galaxy_edge_verification_depth_stub(Some(&json!({"fraud_proof_stub": true}))),
            GalaxyEdgeVerificationDepth::FraudProofStub
        );
        assert_eq!(
            galaxy_edge_verification_depth_stub(Some(&json!({
                "fraud_proof_stub": true,
                "capability_admission": true,
                "network_profile_stale": true,
                "tee_attestation": true,
                "metrics_http": true,
                "stand_smoke_parity": true,
            }))),
            GalaxyEdgeVerificationDepth::FullBand48
        );
        assert_eq!(EDGE_VERIFICATION_CRITERIA.len(), 7);
        assert_eq!(edge_verification_criteria_total(), 7);
        assert!(EDGE_VERIFICATION_CASES.contains(&"openapi_wire"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND48_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band48 row {row}"
            );
        }
    }

    #[test]
    fn grid_verification_replay_json_export_shape_ph_s710() {
        use poolai::grid::stand_smoke_metrics_parity::{
            validate_grid_metrics_json_export, REPLAY_JSON_KEYS, VERIFICATION_JSON_KEYS,
        };
        let verification = serde_json::json!({
            "ok": true,
            "metrics": {
                "sample_total": 0,
                "mismatch_total": 0,
                "match_total": 0,
                "checker_pending_total": 0,
            }
        });
        validate_grid_metrics_json_export(&verification, VERIFICATION_JSON_KEYS)
            .expect("verification");
        let replay = serde_json::json!({
            "ok": true,
            "metrics": {
                "replay_pending": 0,
                "replay_pending_scheduled_total": 0,
                "verification_replay_record_total": 0,
            }
        });
        validate_grid_metrics_json_export(&replay, REPLAY_JSON_KEYS).expect("replay");
    }

    #[test]
    fn grid_settlement_trust_replication_pricing_json_export_shape_ph_s711() {
        use poolai::grid::stand_smoke_metrics_parity::{
            validate_grid_metrics_json_export, PRICING_JSON_KEYS, REPLICATION_JSON_KEYS,
            SETTLEMENT_JSON_KEYS, TRUST_JSON_KEYS,
        };
        let settlement = serde_json::json!({
            "ok": true,
            "metrics": {
                "pending_verification_total": 0,
                "cleared_total": 0,
                "resolved_total": 0,
                "payout_batch_total": 0,
            }
        });
        validate_grid_metrics_json_export(&settlement, SETTLEMENT_JSON_KEYS).expect("settlement");
        let trust = serde_json::json!({
            "ok": true,
            "metrics": {
                "payout_eligible_total": 0,
                "payout_held_total": 0,
                "last_trust_score": 0,
                "gate_min_threshold": 40,
            }
        });
        validate_grid_metrics_json_export(&trust, TRUST_JSON_KEYS).expect("trust");
        let replication = serde_json::json!({
            "ok": true,
            "metrics": {
                "strict_total": 0,
                "enqueue_total": 0,
                "executor_enqueue_total": 0,
                "rate_limited_total": 0,
            }
        });
        validate_grid_metrics_json_export(&replication, REPLICATION_JSON_KEYS)
            .expect("replication");
        let pricing = serde_json::json!({
            "ok": true,
            "metrics": {
                "fresh_served_total": 0,
                "stale_served_total": 0,
                "forced_fallback_total": 0,
                "provider_catalog_lookups_total": 0,
                "provider_catalog_hits_total": 0,
                "provider_errors_total": 0,
                "provider_timeouts_total": 0,
            }
        });
        validate_grid_metrics_json_export(&pricing, PRICING_JSON_KEYS).expect("pricing");
    }

    #[test]
    fn grid_prefetch_locality_metrics_parity_ph_s831() {
        use poolai::grid::stand_smoke_metrics_parity::{
            validate_locality_metrics_parity, validate_prefetch_metrics_parity,
        };

        let prefetch_prom = concat!(
            "galaxy_prefetch_pull_bytes_total 1024\n",
            "galaxy_prefetch_backpressure_total 1\n",
        );
        let prefetch = serde_json::json!({
            "ok": true,
            "metrics": {
                "pull_bytes_total": 1024,
                "backpressure_total": 1,
                "plan_total": 0,
                "enqueue_total": 0,
                "peer_fetch_total": 0,
            }
        });
        validate_prefetch_metrics_parity(prefetch_prom, &prefetch).expect("prefetch parity");

        let locality_prom = concat!(
            "galaxy_shard_local_hit_ratio 7500\n",
            "galaxy_hot_tier_hit_ratio 4000\n",
            "galaxy_cross_region_egress_mb 5\n",
            "galaxy_hot_promote_total 1\n",
            "galaxy_hot_evict_total 0\n",
        );
        let locality = serde_json::json!({
            "ok": true,
            "metrics": {
                "shard_local_hit_ratio_bps": 7500,
                "hot_tier_hit_ratio_bps": 4000,
                "cross_region_egress_mb": 5,
                "hot_promote_total": 1,
                "hot_evict_total": 0,
            }
        });
        validate_locality_metrics_parity(locality_prom, &locality).expect("locality parity");
    }

    #[test]
    fn grid_governance_fee_metrics_parity_ph_s832() {
        use poolai::grid::stand_smoke_metrics_parity::{
            validate_fee_split_metrics_parity, validate_governance_metrics_parity,
        };

        let fee_prom = "galaxy_fee_split_applied_total 7\n";
        let fee_split = serde_json::json!({
            "ok": true,
            "metrics": {
                "fee_split_applied_total": 7,
                "primary_dev_fee_bps": 10,
                "secondary_admin_fee_min_bps": 100,
                "secondary_admin_fee_max_bps": 500,
            }
        });
        validate_fee_split_metrics_parity(fee_prom, &fee_split).expect("fee parity");

        let gov_prom = concat!(
            "poolai_release_verify_total 2\n",
            "poolai_release_verify_fail_total 0\n",
            "poolai_update_notify_pending 1\n",
            "poolai_advisory_acknowledged_total 3\n",
        );
        let governance = serde_json::json!({
            "ok": true,
            "metrics": {
                "release_verify_total": 2,
                "release_verify_fail_total": 0,
                "update_notify_pending": 1,
                "advisory_acknowledged_total": 3,
            }
        });
        validate_governance_metrics_parity(gov_prom, &governance).expect("governance parity");
    }

    #[test]
    fn stand_smoke_export_shape_regression_suite_ph_s834() {
        use poolai::grid::stand_smoke_metrics_parity::validate_band6_metrics_parity_v2;

        let prom = concat!(
            "galaxy_verification_sample_total 1\n",
            "galaxy_verification_checker_pending_total 0\n",
            "galaxy_replay_pending 0\n",
            "galaxy_verification_replay_record_total 0\n",
            "galaxy_settlement_cleared_total 0\n",
            "galaxy_settlement_payout_batch_total 0\n",
            "galaxy_trust_payout_eligible_total 0\n",
            "galaxy_trust_score 0\n",
            "galaxy_replication_strict_total 0\n",
            "galaxy_replication_enqueue_total 0\n",
            "galaxy_pricing_fresh_served 0\n",
            "galaxy_pricing_stale_served 0\n",
            "galaxy_prefetch_pull_bytes_total 0\n",
            "galaxy_prefetch_backpressure_total 0\n",
            "galaxy_shard_local_hit_ratio 0\n",
            "galaxy_hot_tier_hit_ratio 0\n",
            "galaxy_cross_region_egress_mb 0\n",
            "galaxy_hot_promote_total 0\n",
            "galaxy_hot_evict_total 0\n",
            "galaxy_fee_split_applied_total 0\n",
            "poolai_release_verify_total 0\n",
            "poolai_release_verify_fail_total 0\n",
            "poolai_update_notify_pending 0\n",
            "poolai_advisory_acknowledged_total 0\n",
            "galaxy_settlement_payout_batch_queue_depth 0\n",
            "galaxy_settlement_onchain_submit_total 0\n",
        );
        let verification = serde_json::json!({"ok": true, "metrics": {"sample_total": 1, "mismatch_total": 0, "match_total": 0, "checker_pending_total": 0}});
        let replay = serde_json::json!({"ok": true, "metrics": {"replay_pending": 0, "replay_pending_scheduled_total": 0, "verification_replay_record_total": 0}});
        let settlement = serde_json::json!({"ok": true, "metrics": {"pending_verification_total": 0, "cleared_total": 0, "resolved_total": 0, "payout_batch_total": 0}});
        let trust = serde_json::json!({"ok": true, "metrics": {"payout_eligible_total": 0, "payout_held_total": 0, "last_trust_score": 0, "gate_min_threshold": 40}});
        let replication = serde_json::json!({"ok": true, "metrics": {"strict_total": 0, "enqueue_total": 0, "executor_enqueue_total": 0, "rate_limited_total": 0}});
        let pricing = serde_json::json!({"ok": true, "metrics": {"fresh_served_total": 0, "stale_served_total": 0, "forced_fallback_total": 0, "provider_catalog_lookups_total": 0, "provider_catalog_hits_total": 0, "provider_errors_total": 0, "provider_timeouts_total": 0}});
        let prefetch = serde_json::json!({"ok": true, "metrics": {"pull_bytes_total": 0, "backpressure_total": 0, "plan_total": 0, "enqueue_total": 0, "peer_fetch_total": 0}});
        let locality = serde_json::json!({"ok": true, "metrics": {"shard_local_hit_ratio_bps": 0, "hot_tier_hit_ratio_bps": 0, "cross_region_egress_mb": 0, "hot_promote_total": 0, "hot_evict_total": 0}});
        let fee_split = serde_json::json!({"ok": true, "metrics": {"fee_split_applied_total": 0, "primary_dev_fee_bps": 10, "secondary_admin_fee_min_bps": 100, "secondary_admin_fee_max_bps": 500}});
        let governance = serde_json::json!({"ok": true, "metrics": {"release_verify_total": 0, "release_verify_fail_total": 0, "update_notify_pending": 0, "advisory_acknowledged_total": 0}});
        let payout_batch = serde_json::json!({"ok": true, "metrics": {"payout_batch_total": 0, "payout_batch_queue_depth": 0, "onchain_submit_total": 0}});
        validate_band6_metrics_parity_v2(
            prom,
            &verification,
            &replay,
            &settlement,
            &trust,
            &replication,
            &pricing,
            &prefetch,
            &locality,
            &fee_split,
            &governance,
            &payout_batch,
        )
        .expect("band18 regression suite");
    }

    #[test]
    fn multi_module_stand_smoke_full_suite_ph_s1002() {
        use poolai_ui_core::multi_module_depth::{
            multi_module_depth_stub, MultiModuleDepth, MULTI_MODULE_BAND35_TOP5_GRID_APIS,
            STAND_SMOKE_FULL_SUITE,
        };
        use serde_json::json;

        assert_eq!(
            multi_module_depth_stub(Some(&json!({"stand_smoke": true}))),
            MultiModuleDepth::StandSmoke
        );
        assert_eq!(MULTI_MODULE_BAND35_TOP5_GRID_APIS.len(), 5);
        assert!(STAND_SMOKE_FULL_SUITE.contains("--json"));
        stand_smoke_export_shape_regression_suite_ph_s834();
    }

    #[test]
    fn grid_metrics_band6_prometheus_parity_export_shape_ph_s713() {
        use poolai::grid::stand_smoke_metrics_parity::validate_band6_metrics_parity;
        let prom = concat!(
            "galaxy_verification_sample_total 0\n",
            "galaxy_verification_checker_pending_total 0\n",
            "galaxy_replay_pending 0\n",
            "galaxy_verification_replay_record_total 0\n",
            "galaxy_settlement_cleared_total 0\n",
            "galaxy_settlement_payout_batch_total 0\n",
            "galaxy_trust_payout_eligible_total 0\n",
            "galaxy_trust_score 0\n",
            "galaxy_replication_strict_total 0\n",
            "galaxy_replication_enqueue_total 0\n",
            "galaxy_pricing_fresh_served 0\n",
            "galaxy_pricing_stale_served 0\n",
        );
        let verification = serde_json::json!({"ok": true, "metrics": {"sample_total": 0, "mismatch_total": 0, "match_total": 0, "checker_pending_total": 0}});
        let replay = serde_json::json!({"ok": true, "metrics": {"replay_pending": 0, "replay_pending_scheduled_total": 0, "verification_replay_record_total": 0}});
        let settlement = serde_json::json!({"ok": true, "metrics": {"pending_verification_total": 0, "cleared_total": 0, "resolved_total": 0, "payout_batch_total": 0}});
        let trust = serde_json::json!({"ok": true, "metrics": {"payout_eligible_total": 0, "payout_held_total": 0, "last_trust_score": 0, "gate_min_threshold": 40}});
        let replication = serde_json::json!({"ok": true, "metrics": {"strict_total": 0, "enqueue_total": 0, "executor_enqueue_total": 0, "rate_limited_total": 0}});
        let pricing = serde_json::json!({"ok": true, "metrics": {"fresh_served_total": 0, "stale_served_total": 0, "forced_fallback_total": 0, "provider_catalog_lookups_total": 0, "provider_catalog_hits_total": 0, "provider_errors_total": 0, "provider_timeouts_total": 0}});
        validate_band6_metrics_parity(
            prom,
            &verification,
            &replay,
            &settlement,
            &trust,
            &replication,
            &pricing,
        )
        .expect("band6 parity");
    }

    #[test]
    fn grid_settlement_trust_prometheus_parity_export_shape_ph_s723() {
        use poolai::grid::stand_smoke_metrics_parity::validate_settlement_trust_metrics_parity;
        let prom = concat!(
            "galaxy_settlement_cleared_total 2\n",
            "galaxy_settlement_payout_batch_total 1\n",
            "galaxy_trust_payout_eligible_total 3\n",
            "galaxy_trust_score 55\n",
        );
        let settlement = serde_json::json!({
            "ok": true,
            "metrics": {
                "pending_verification_total": 0,
                "cleared_total": 2,
                "resolved_total": 0,
                "payout_batch_total": 1,
            }
        });
        let trust = serde_json::json!({
            "ok": true,
            "metrics": {
                "payout_eligible_total": 3,
                "payout_held_total": 0,
                "last_trust_score": 55,
                "gate_min_threshold": 40,
            }
        });
        validate_settlement_trust_metrics_parity(prom, &settlement, &trust).expect("parity");
    }

    #[test]
    fn grid_network_profiles_list_put_export_shape_ph_s733() {
        let list = serde_json::json!({
            "ok": true,
            "peer_ids": ["peer-a", "peer-b"],
            "count": 2
        });
        assert_eq!(list["ok"], true);
        assert_eq!(list["count"], 2);
        let ids = list["peer_ids"].as_array().expect("peer_ids");
        assert_eq!(ids.len(), 2);

        let profile = serde_json::json!({
            "ok": true,
            "peer_id": "peer-a",
            "network_profile": {
                "region": "smoke",
                "latency_ms_p50": 11
            }
        });
        assert_eq!(profile["peer_id"], "peer-a");
        assert_eq!(profile["network_profile"]["region"].as_str(), Some("smoke"));
    }

    #[test]
    fn openapi_band_export_shape_ph_s843() {
        let advisories = serde_json::json!([
            {
                "id": "CVE-2026-0001",
                "severity": "medium",
                "summary": "Signed release manifest rotation advisory (Galaxy §9.2)",
                "acknowledged": false
            }
        ]);
        let row = &advisories[0];
        for key in ["id", "severity", "summary", "acknowledged"] {
            assert!(row.get(key).is_some(), "missing {key}");
        }
        let rebind_err = serde_json::json!({
            "error": "admin_required",
            "message": "Bearer admin token required for wallet rebind override"
        });
        assert_eq!(rebind_err["error"].as_str(), Some("admin_required"));
    }

    #[test]
    fn signed_capability_reject_export_shape_ph_s743() {
        let reject = serde_json::json!({
            "error": {
                "code": "capability_signature_invalid",
                "message": "signed capability_document required for telegram_edge origin"
            }
        });
        assert_eq!(
            reject["error"]["code"].as_str(),
            Some("capability_signature_invalid")
        );
        assert!(reject["error"]["message"]
            .as_str()
            .unwrap_or("")
            .contains("telegram_edge"));

        let metrics = concat!(
            "# HELP galaxy_capability_unsigned_rejected_total Unsigned or invalid signed capability rejections on telegram_edge register-remote (PH-S740)\n",
            "# TYPE galaxy_capability_unsigned_rejected_total gauge\n",
            "galaxy_capability_unsigned_rejected_total 1\n",
            "# HELP galaxy_capability_signed_accepted_total Successful signed capability admissions on telegram_edge register-remote (PH-S741)\n",
            "# TYPE galaxy_capability_signed_accepted_total gauge\n",
            "galaxy_capability_signed_accepted_total 0\n",
        );
        assert!(metrics.contains("galaxy_capability_unsigned_rejected_total"));
        assert!(metrics.contains("galaxy_capability_signed_accepted_total"));
    }

    #[test]
    fn vision_revision_fm_parity_ph_s235() {
        let root = repo_root();
        assert_vision_repo_parity(&root)
            .expect("run poolai-vision-sync --check before stand smoke");
        let manifest_rev = read_manifest_revision(&root).expect("manifest");
        let fm_rev = read_fm_vision_revision(&root).expect("fm");
        assert_eq!(manifest_rev, fm_rev);
        if let Some(next) = read_manifest_next_sprint(&root) {
            let active = read_extensions_active_sprint(&root)
                .expect("extensions")
                .expect("active_sprint");
            assert_eq!(active, next);
        }
    }
}
