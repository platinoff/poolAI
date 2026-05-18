//! PoolAI worker — virtual node client (FM-016).
//!
//! Phase 1: `POST /api/v1/discovery/register-remote`
//! Phase 2: `GET /health`, `POST /discovery/heartbeat-remote`
//! Phase 3: poll/complete virtual-node tasks + RAID distributed health wire

use axum::{extract::State, routing::get, Json, Router};
use poolai::raid::protocol::ProtocolMessage;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

#[derive(Debug, Clone)]
struct Args {
    worker_id: String,
    coordinator_url: String,
    advertise_address: String,
    advertise_port: u16,
    max_memory_mb: usize,
    register_interval_secs: u64,
    heartbeat_interval_secs: u64,
    channel: String,
    telegram_id: Option<String>,
}

#[derive(Clone)]
struct WorkerRuntime {
    args: Args,
    tasks_polled: Arc<AtomicU64>,
    tasks_completed: Arc<AtomicU64>,
    coordinator_reachable: Arc<RwLock<bool>>,
    pool_api_reachable: Arc<RwLock<bool>>,
    raid_wire_ok: Arc<RwLock<bool>>,
    last_task: Arc<RwLock<Option<String>>>,
}

#[derive(Serialize)]
struct WorkerHealthResponse {
    status: &'static str,
    worker_id: String,
    role: &'static str,
    channel: String,
    coordinator_reachable: bool,
    pool_api_reachable: bool,
    raid_wire_ok: bool,
    tasks_polled: u64,
    tasks_completed: u64,
    last_task: Option<String>,
}

#[derive(Deserialize)]
struct PollTasksResponse {
    task: Option<VirtualNodeTaskDto>,
    pending: usize,
}

#[derive(Deserialize)]
struct VirtualNodeTaskDto {
    id: String,
    task_type: String,
}

fn parse_args() -> Args {
    let mut worker_id: Option<String> = None;
    let mut coordinator_url = std::env::var("POOLAI_COORDINATOR_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
    let mut advertise_address =
        std::env::var("POOLAI_WORKER_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    let mut advertise_port: u16 = std::env::var("POOLAI_WORKER_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let mut max_memory_mb: Option<usize> = None;
    let mut register_interval_secs: u64 = std::env::var("POOLAI_REGISTER_INTERVAL_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(120);
    let mut heartbeat_interval_secs: u64 = std::env::var("POOLAI_HEARTBEAT_INTERVAL_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);
    let mut channel =
        std::env::var("POOLAI_WORKER_CHANNEL").unwrap_or_else(|_| "telegram".to_string());
    let mut telegram_id = std::env::var("POOLAI_TELEGRAM_ID").ok();

    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--worker-id" => worker_id = it.next(),
            "--coordinator" => coordinator_url = it.next().unwrap_or(coordinator_url),
            "--address" => advertise_address = it.next().unwrap_or(advertise_address),
            "--port" => {
                advertise_port = it
                    .next()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(advertise_port);
            }
            "--max-memory" => {
                max_memory_mb = it.next().and_then(|v| v.parse::<usize>().ok());
            }
            "--interval" => {
                register_interval_secs = it
                    .next()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(register_interval_secs);
            }
            "--heartbeat-interval" => {
                heartbeat_interval_secs = it
                    .next()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(heartbeat_interval_secs);
            }
            "--channel" => channel = it.next().unwrap_or(channel),
            "--telegram-id" => telegram_id = it.next(),
            other if other.starts_with('-') => warn!("Ignoring unknown argument: {}", other),
            _ => {}
        }
    }

    if advertise_port == 0 {
        advertise_port = 9090;
    }

    Args {
        worker_id: worker_id.unwrap_or_else(|| format!("worker-{}", uuid::Uuid::new_v4())),
        coordinator_url: coordinator_url.trim_end_matches('/').to_string(),
        advertise_address,
        advertise_port,
        max_memory_mb: max_memory_mb.unwrap_or(2048),
        register_interval_secs,
        heartbeat_interval_secs,
        channel,
        telegram_id,
    }
}

fn http_client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| e.to_string())
}

fn registration_body(args: &Args) -> serde_json::Value {
    let mut metadata = HashMap::new();
    metadata.insert("channel".to_string(), args.channel.clone());
    metadata.insert("role".to_string(), "virtual_node".to_string());
    if let Some(tg) = &args.telegram_id {
        metadata.insert("telegram_id".to_string(), tg.clone());
    }

    serde_json::json!({
        "peer_id": args.worker_id,
        "address": args.advertise_address,
        "port": args.advertise_port,
        "capabilities": {
            "cpu_cores": num_cpus::get(),
            "memory_mb": args.max_memory_mb,
            "gpu_devices": [],
            "supports_tensor_parallelism": false,
            "supports_pipeline_parallelism": false,
        },
        "metadata": metadata,
    })
}

async fn register_remote(client: &reqwest::Client, args: &Args) -> Result<(), String> {
    let url = format!("{}/api/v1/discovery/register-remote", args.coordinator_url);
    let response = client
        .post(&url)
        .json(&registration_body(args))
        .send()
        .await
        .map_err(|e| format!("register-remote request failed: {e}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(format!("register-remote HTTP {status}: {text}"));
    }
    Ok(())
}

async fn heartbeat_remote(client: &reqwest::Client, args: &Args) -> Result<(), String> {
    let url = format!("{}/api/v1/discovery/heartbeat-remote", args.coordinator_url);
    let body = serde_json::json!({
        "peer_id": args.worker_id,
        "capabilities": {
            "cpu_cores": num_cpus::get(),
            "memory_mb": args.max_memory_mb,
            "gpu_devices": [],
            "supports_tensor_parallelism": false,
            "supports_pipeline_parallelism": false,
            "active_requests": 0,
            "capacity": 10,
            "current_load": 0.0,
        }
    });
    let response = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("heartbeat-remote failed: {e}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(format!("heartbeat-remote HTTP {status}: {text}"));
    }
    Ok(())
}

async fn complete_task(
    client: &reqwest::Client,
    args: &Args,
    task_id: &str,
    status: &str,
    detail: Option<String>,
) -> Result<(), String> {
    let url = format!(
        "{}/api/v1/virtual-nodes/{}/tasks/{}/complete",
        args.coordinator_url, args.worker_id, task_id
    );
    let response = client
        .post(&url)
        .json(&serde_json::json!({ "status": status, "detail": detail }))
        .send()
        .await
        .map_err(|e| format!("task complete failed: {e}"))?;
    if !response.status().is_success() {
        return Err(format!("task complete HTTP {}", response.status()));
    }
    Ok(())
}

async fn run_raid_health_check(client: &reqwest::Client, args: &Args) -> bool {
    let message = ProtocolMessage::health_check(args.worker_id.clone());
    let url = format!("{}/api/v1/raid/distributed/health", args.coordinator_url);
    match client.post(&url).json(&message).send().await {
        Ok(response) => response.status().is_success(),
        Err(e) => {
            warn!("RAID wire health_check failed: {}", e);
            false
        }
    }
}

async fn execute_task(
    client: &reqwest::Client,
    rt: &WorkerRuntime,
    task: VirtualNodeTaskDto,
) -> (String, bool) {
    match task.task_type.as_str() {
        "ping" => ("ok".to_string(), true),
        "raid_health_check" => {
            let ok = run_raid_health_check(client, &rt.args).await;
            *rt.raid_wire_ok.write().await = ok;
            (
                if ok {
                    "raid_ok".to_string()
                } else {
                    "raid_failed".to_string()
                },
                ok,
            )
        }
        other => (format!("unsupported:{other}"), false),
    }
}

async fn poll_and_run_tasks(client: &reqwest::Client, rt: &WorkerRuntime) {
    let url = format!(
        "{}/api/v1/virtual-nodes/{}/tasks/poll",
        rt.args.coordinator_url, rt.args.worker_id
    );
    let Ok(response) = client.get(&url).send().await else {
        return;
    };
    if !response.status().is_success() {
        return;
    }
    let Ok(body) = response.json::<PollTasksResponse>().await else {
        return;
    };
    rt.tasks_polled.fetch_add(1, Ordering::Relaxed);

    let Some(task) = body.task else {
        return;
    };

    let task_id = task.id.clone();
    let task_type = task.task_type.clone();
    info!("Running task {} ({})", task_id, task_type);
    let (status, _ok) = execute_task(client, rt, task).await;
    if complete_task(client, &rt.args, &task_id, &status, Some(task_type.clone()))
        .await
        .is_ok()
    {
        rt.tasks_completed.fetch_add(1, Ordering::Relaxed);
        *rt.last_task.write().await = Some(format!("{task_type}:{status}"));
        info!("Task {} completed ({})", task_id, status);
    }
}

async fn poll_coordinator_links(client: &reqwest::Client, rt: &WorkerRuntime) {
    let health_url = format!("{}/api/v1/health", rt.args.coordinator_url);
    let health_ok = client
        .get(&health_url)
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false);
    *rt.coordinator_reachable.write().await = health_ok;

    let pool_url = format!("{}/api/v1/workers", rt.args.coordinator_url);
    let pool_ok = client
        .get(&pool_url)
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false);
    *rt.pool_api_reachable.write().await = pool_ok;
}

async fn health_handler(State(rt): State<Arc<WorkerRuntime>>) -> Json<WorkerHealthResponse> {
    Json(WorkerHealthResponse {
        status: "healthy",
        worker_id: rt.args.worker_id.clone(),
        role: "virtual_node",
        channel: rt.args.channel.clone(),
        coordinator_reachable: *rt.coordinator_reachable.read().await,
        pool_api_reachable: *rt.pool_api_reachable.read().await,
        raid_wire_ok: *rt.raid_wire_ok.read().await,
        tasks_polled: rt.tasks_polled.load(Ordering::Relaxed),
        tasks_completed: rt.tasks_completed.load(Ordering::Relaxed),
        last_task: rt.last_task.read().await.clone(),
    })
}

async fn run_health_server(rt: Arc<WorkerRuntime>) -> Result<(), String> {
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/api/v1/health", get(health_handler))
        .with_state(rt.clone());

    let addr = format!("{}:{}", rt.args.advertise_address, rt.args.advertise_port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| format!("bind {addr}: {e}"))?;
    info!("Worker health server listening on http://{}", addr);
    axum::serve(listener, app)
        .await
        .map_err(|e| format!("health server: {e}"))?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = parse_args();
    let rt = Arc::new(WorkerRuntime {
        args: args.clone(),
        tasks_polled: Arc::new(AtomicU64::new(0)),
        tasks_completed: Arc::new(AtomicU64::new(0)),
        coordinator_reachable: Arc::new(RwLock::new(false)),
        pool_api_reachable: Arc::new(RwLock::new(false)),
        raid_wire_ok: Arc::new(RwLock::new(false)),
        last_task: Arc::new(RwLock::new(None)),
    });

    info!(
        "PoolAI virtual node: id={}, coordinator={}, health=http://{}:{}/health",
        args.worker_id, args.coordinator_url, args.advertise_address, args.advertise_port
    );

    let rt_server = rt.clone();
    tokio::spawn(async move {
        if let Err(e) = run_health_server(rt_server).await {
            error!("Health server exited: {}", e);
        }
    });

    let client = http_client()?;
    register_remote(&client, &args).await?;
    info!("Registered with coordinator (bootstrap tasks queued)");

    let mut ticks: u64 = 0;
    loop {
        poll_coordinator_links(&client, &rt).await;
        poll_and_run_tasks(&client, &rt).await;

        match heartbeat_remote(&client, &args).await {
            Ok(()) => tracing::debug!("Heartbeat OK"),
            Err(e) => {
                warn!("Heartbeat failed (will re-register): {}", e);
                if let Err(reg_err) = register_remote(&client, &args).await {
                    error!("Re-register failed: {}", reg_err);
                }
            }
        }

        ticks = ticks.saturating_add(1);
        let register_every = (args.register_interval_secs / args.heartbeat_interval_secs).max(1);
        if ticks.is_multiple_of(register_every) {
            if let Err(e) = register_remote(&client, &args).await {
                error!("Periodic re-register failed: {}", e);
            }
        }

        tokio::time::sleep(Duration::from_secs(args.heartbeat_interval_secs)).await;
    }
}
