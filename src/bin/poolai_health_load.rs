//! In-tree HTTP load generator for `GET /api/v1/health` (Rust only, no wrk/hey/Python).
//!
//! ```text
//! # Terminal 1: start server
//! cargo run --release
//!
//! # Terminal 2: load test (URL, duration seconds, concurrent workers)
//! cargo run --release --bin poolai_health_load -- http://127.0.0.1:8080/api/v1/health 30 400
//! ```

use parking_lot::Mutex;
use rand::Rng;
use reqwest::Client;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::task::JoinSet;

const DEFAULT_URL: &str = "http://127.0.0.1:8080/api/v1/health";
const RESERVOIR_CAP: usize = 200_000;

struct Reservoir {
    total: AtomicU64,
    samples: Mutex<Vec<u64>>,
}

impl Reservoir {
    fn new() -> Self {
        Self {
            total: AtomicU64::new(0),
            samples: Mutex::new(Vec::with_capacity(RESERVOIR_CAP)),
        }
    }

    fn record_ns(&self, latency_ns: u64) {
        let n = self.total.fetch_add(1, Ordering::Relaxed) + 1;
        let mut buf = self.samples.lock();
        if n <= RESERVOIR_CAP as u64 {
            buf.push(latency_ns);
            return;
        }
        let j = rand::rng().random_range(0..n);
        if (j as usize) < RESERVOIR_CAP {
            buf[j as usize] = latency_ns;
        }
    }

    fn into_sorted_samples(self) -> (u64, Vec<u64>) {
        let total = self.total.load(Ordering::Relaxed);
        let mut v = self.samples.into_inner();
        v.sort_unstable();
        (total, v)
    }
}

fn percentile(sorted: &[u64], p: f64) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }
    let idx = ((sorted.len() as f64 - 1.0) * p / 100.0).round() as usize;
    let idx = idx.min(sorted.len() - 1);
    Some(sorted[idx] as f64 / 1_000_000.0)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    let url = args
        .first()
        .filter(|s| !s.starts_with('-'))
        .cloned()
        .unwrap_or_else(|| DEFAULT_URL.to_string());
    if !args.is_empty() && !args[0].starts_with('-') {
        args.remove(0);
    }
    let duration_secs: f64 = args.first().and_then(|s| s.parse().ok()).unwrap_or(30.0);
    if !args.is_empty() {
        args.remove(0);
    }
    let concurrency: usize = args
        .first()
        .and_then(|s| s.parse().ok())
        .unwrap_or(400)
        .max(1);

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(concurrency)
        .build()?;

    let client = Arc::new(client);
    let url = Arc::<str>::from(url.into_boxed_str());
    let reservoir = Arc::new(Reservoir::new());
    let errors = Arc::new(AtomicU64::new(0));

    let run_for = Duration::from_secs_f64(duration_secs);
    let start = Instant::now();

    let mut set = JoinSet::new();
    for _ in 0..concurrency {
        let client = client.clone();
        let url = url.clone();
        let reservoir = reservoir.clone();
        let errors = errors.clone();
        set.spawn(async move {
            let deadline = Instant::now() + run_for;
            while Instant::now() < deadline {
                let t0 = Instant::now();
                match client.get(url.as_ref()).send().await {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            if resp.bytes().await.is_ok() {
                                reservoir.record_ns(t0.elapsed().as_nanos() as u64);
                            } else {
                                errors.fetch_add(1, Ordering::Relaxed);
                            }
                        } else {
                            errors.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                    Err(_) => {
                        errors.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
        });
    }

    while let Some(joined) = set.join_next().await {
        joined?;
    }

    let elapsed = start.elapsed();
    let (total_ok, sorted) = match Arc::try_unwrap(reservoir) {
        Ok(r) => r.into_sorted_samples(),
        Err(_) => panic!("BUG: reservoir Arc should be unique after workers finish"),
    };
    let err = match Arc::try_unwrap(errors) {
        Ok(a) => a.load(Ordering::Relaxed),
        Err(_) => panic!("BUG: errors Arc should be unique after workers finish"),
    };
    let wall_secs = elapsed.as_secs_f64().max(1e-9);
    let rps = total_ok as f64 / wall_secs;

    eprintln!("URL: {}", url);
    eprintln!(
        "Duration (wall): {:.3}s  Workers: {}  OK: {}  Errors: {}",
        wall_secs, concurrency, total_ok, err
    );
    eprintln!("Requests/sec (OK only): {:.1}", rps);
    if !sorted.is_empty() {
        let sum: u128 = sorted.iter().map(|&x| x as u128).sum();
        let mean_ms = sum as f64 / sorted.len() as f64 / 1_000_000.0;
        let reservoir_note = (total_ok as usize) > sorted.len();
        let mean_label = if reservoir_note {
            "mean (reservoir sample)"
        } else {
            "mean"
        };
        eprintln!(
            "Latency OK (n={} latencies recorded) — {}: {:.3} ms  p50: {:.3} ms  p95: {:.3} ms  p99: {:.3} ms",
            sorted.len(),
            mean_label,
            mean_ms,
            percentile(&sorted, 50.0).unwrap_or(0.0),
            percentile(&sorted, 95.0).unwrap_or(0.0),
            percentile(&sorted, 99.0).unwrap_or(0.0),
        );
        if reservoir_note {
            eprintln!(
                "Total OK requests: {} (percentiles approximate; reservoir cap {})",
                total_ok, RESERVOIR_CAP
            );
        }
    } else {
        eprintln!("No successful responses; check server and URL.");
    }

    Ok(())
}
