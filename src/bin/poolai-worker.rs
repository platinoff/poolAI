//! PoolAI worker process — virtual node client (FM-016 phase 1).
//!
//! Registers with a coordinator via `POST /api/v1/discovery/register-remote` and
//! sends periodic heartbeats by re-registering. Extend toward full Telegram/device worker.

use std::collections::HashMap;
use std::time::Duration;
use tracing::{error, info, warn};

#[derive(Debug, Clone)]
struct Args {
    worker_id: String,
    coordinator_url: String,
    advertise_address: String,
    advertise_port: u16,
    max_memory_mb: usize,
    register_interval_secs: u64,
    channel: String,
    telegram_id: Option<String>,
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
        .unwrap_or(30);
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
        channel,
        telegram_id,
    }
}

async fn register_remote(args: &Args) -> Result<(), String> {
    let mut metadata = HashMap::new();
    metadata.insert("channel".to_string(), args.channel.clone());
    metadata.insert("role".to_string(), "virtual_node".to_string());
    if let Some(tg) = &args.telegram_id {
        metadata.insert("telegram_id".to_string(), tg.clone());
    }

    let body = serde_json::json!({
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
    });

    let url = format!("{}/api/v1/discovery/register-remote", args.coordinator_url);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .post(&url)
        .json(&body)
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = parse_args();
    info!(
        "PoolAI worker (virtual node): id={}, coordinator={}, advertise={}:{}",
        args.worker_id, args.coordinator_url, args.advertise_address, args.advertise_port
    );

    loop {
        match register_remote(&args).await {
            Ok(()) => info!("Registered with coordinator as {}", args.worker_id),
            Err(e) => error!("Registration failed: {}", e),
        }
        tokio::time::sleep(Duration::from_secs(args.register_interval_secs)).await;
    }
}
