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
//! # Full suite incl. raid restart:
//! cargo run --bin poolai-http-stand-smoke -- --raid
//!
//! cargo run --bin poolai-http-stand-smoke -- --json
//! ```

use reqwest::{Client, StatusCode};
use serde::Serialize;
use serde_json::{json, Value};
use std::path::Path;
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const DEFAULT_BASE: &str = "http://127.0.0.1:8080";
const ENV_BASE: &str = "POOLAI_BASE_URL";
const ENV_STAND_ROOT: &str = "POOLAI_E2E_STAND_ROOT";
const VALID_PUBKEY: &str = "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU";

#[derive(Debug, Clone)]
struct Cli {
    json_out: bool,
    include_raid: bool,
    raid_restart_only: bool,
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

fn parse_cli() -> Cli {
    let mut json_out = false;
    let mut include_raid = false;
    let mut raid_restart_only = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--json" => json_out = true,
            "--raid-restart" => raid_restart_only = true,
            "--raid" => include_raid = true,
            _ if arg.starts_with('-') => {}
            _ => {}
        }
    }
    if !raid_restart_only {
        raid_restart_only = std::env::var("POOLAI_STAND_SMOKE_RAID_RESTART")
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

async fn smoke_jobs_lease(client: &Client, base: &str) -> Result<(), String> {
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
    let epoch = job
        .get("lease_epoch")
        .and_then(|v| v.as_u64())
        .ok_or("missing lease_epoch")?;
    let renew = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease/renew")))
        .json(&json!({ "lease_epoch": epoch }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if renew.status() != StatusCode::OK {
        return Err(format!("lease renew status {}", renew.status()));
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
        "jobs_lease",
        smoke_jobs_lease(&client, &cli.base_url).await,
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
        let args: Vec<String> = vec!["poolai-http-stand-smoke".into(), "--raid-restart".into()];
        assert!(args.iter().any(|a| a == "--raid-restart"));
    }
}
