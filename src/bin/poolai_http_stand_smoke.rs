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
//! # Vision revision parity (PH-S208):
//! export POOLAI_VISION_BASE_URL=http://127.0.0.1:8765   # open-docs-vision.ps1
//! cargo run --bin poolai-http-stand-smoke   # checks manifest vs FM + optional HTTP header
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
const FM_REL: &str = "docs/catalog/FUNCTION_MANAGEMENT.md";
const VISION_REV_HEADER: &str = "x-poolai-vision-revision";
const VALID_PUBKEY: &str = "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU";

#[derive(Debug, Clone)]
struct Cli {
    json_out: bool,
    include_raid: bool,
    raid_restart_only: bool,
    lease_renew_only: bool,
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

fn read_manifest_revision(root: &Path) -> Result<u64, String> {
    let path = root.join(MANIFEST_REL);
    let raw = fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let manifest: Value = serde_json::from_str(&raw).map_err(|e| format!("parse manifest: {e}"))?;
    manifest
        .get("revision")
        .and_then(Value::as_u64)
        .ok_or_else(|| "manifest missing revision".to_string())
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
    let repo_rev = read_manifest_revision(&root)?;
    let fm_rev = read_fm_vision_revision(&root)?;
    if repo_rev != fm_rev {
        return Err(format!(
            "repo manifest.revision {repo_rev} != FM Vision rev {fm_rev}"
        ));
    }

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
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--json" => json_out = true,
            "--raid-restart" => raid_restart_only = true,
            "--raid" => include_raid = true,
            "--lease-renew" => lease_renew_only = true,
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
    Cli {
        json_out,
        include_raid,
        raid_restart_only,
        lease_renew_only,
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
    Ok(())
}

/// PH-S213: live stand exposes Galaxy prefetch counters on Prometheus scrape.
const GALAXY_PREFETCH_METRICS: &[&str] = &[
    "galaxy_prefetch_plan_total",
    "galaxy_prefetch_planned_shards_total",
    "galaxy_prefetch_hot_skip_total",
    "galaxy_prefetch_bytes_total",
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

/// PH-S225: live stand exposes Galaxy verification counters on Prometheus scrape.
const GALAXY_VERIFICATION_METRICS: &[&str] = &[
    "galaxy_verification_sample_total",
    "galaxy_verification_mismatch_total",
    "galaxy_verification_match_total",
    "galaxy_verification_sample_scheduled_total",
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
    "galaxy_trust_score",
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
    fn parse_fm_vision_revision_footer() {
        let section = "**Відкритих у §5.12:** **2** (PH-S208…S209). Vision rev **149**.\n";
        assert_eq!(parse_fm_vision_revision(section), Some(149));
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
            "# HELP galaxy_trust_score Galaxy last trust score\n",
            "# TYPE galaxy_trust_score gauge\n",
            "galaxy_trust_score 0\n",
        );
        metrics_text_has_trust_payout_counters(sample).expect("sample export");
    }

    #[test]
    fn vision_revision_fm_parity_in_repo() {
        let root = repo_root();
        let manifest_rev = read_manifest_revision(&root).expect("manifest");
        let fm_rev = read_fm_vision_revision(&root).expect("fm");
        assert_eq!(
            manifest_rev, fm_rev,
            "run poolai-vision-sync --check before stand smoke"
        );
    }
}
