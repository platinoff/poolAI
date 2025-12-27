//! PoolAI worker process (stub)
//!
//! This binary is spawned by `runtime::worker` on startup.
//! For now it is a minimal long-running process that can be extended into a real worker.

use std::time::Duration;
use tracing::{info, warn};

#[derive(Debug, Clone)]
struct Args {
    worker_id: String,
    max_memory_mb: usize,
}

fn parse_args() -> Args {
    let mut worker_id: Option<String> = None;
    let mut max_memory_mb: Option<usize> = None;

    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--worker-id" => worker_id = it.next(),
            "--max-memory" => {
                max_memory_mb = it.next().and_then(|v| v.parse::<usize>().ok());
            }
            other => {
                if other.starts_with('-') {
                    warn!("Ignoring unknown argument: {}", other);
                }
            }
        }
    }

    Args {
        worker_id: worker_id.unwrap_or_else(|| "worker-unknown".to_string()),
        max_memory_mb: max_memory_mb.unwrap_or(2048),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = parse_args();
    info!(
        "PoolAI worker started: id={}, max_memory_mb={}",
        args.worker_id, args.max_memory_mb
    );

    // Stub: keep the worker alive. The parent process will terminate us via `Child.kill()`.
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
