//! In-tree HTTP load generator for `GET /api/v1/health` (Rust only, no wrk/hey/Python).
//!
//! ```text
//! # Terminal 1: start server
//! cargo run --release
//!
//! # Terminal 2: load test (URL, duration seconds, concurrent workers)
//! cargo run --release --bin poolai_health_load -- http://127.0.0.1:8080/api/v1/health 30 400
//!
//! # Machine-readable summary on stdout (for baselines / jq)
//! cargo run --release --bin poolai_health_load -- --json http://127.0.0.1:8080/api/v1/health 10 100
//! ```

use parking_lot::Mutex;
use rand::Rng;
use reqwest::Client;
use serde::Serialize;
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

/// Strip `--json` from argv; remaining tokens are positional URL / duration / concurrency.
fn parse_cli_args(mut args: Vec<String>) -> (bool, String, f64, usize) {
    let mut json_out = false;
    args.retain(|a| {
        if a == "--json" {
            json_out = true;
            false
        } else {
            true
        }
    });

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

    (json_out, url, duration_secs, concurrency)
}

#[derive(Debug, Serialize)]
struct LoadReport {
    url: String,
    wall_seconds: f64,
    concurrency: usize,
    ok_requests: u64,
    error_requests: u64,
    rps_ok_only: f64,
    latency_sample_count: usize,
    /// Mean latency in ms over `latency_sample_count` (reservoir subset if `total_ok_exceeds_sample`).
    latency_mean_ms: Option<f64>,
    latency_mean_is_reservoir: bool,
    latency_p50_ms: Option<f64>,
    latency_p95_ms: Option<f64>,
    latency_p99_ms: Option<f64>,
    reservoir_cap: usize,
    total_ok_exceeds_sample: bool,
    tool: &'static str,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (json_out, url, duration_secs, concurrency) = parse_cli_args(args);

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(concurrency)
        .build()?;

    let client = Arc::new(client);
    let url_arc = Arc::<str>::from(url.clone().into_boxed_str());
    let reservoir = Arc::new(Reservoir::new());
    let errors = Arc::new(AtomicU64::new(0));

    let run_for = Duration::from_secs_f64(duration_secs);
    let start = Instant::now();

    let mut set = JoinSet::new();
    for _ in 0..concurrency {
        let client = client.clone();
        let url_arc = url_arc.clone();
        let reservoir = reservoir.clone();
        let errors = errors.clone();
        set.spawn(async move {
            let deadline = Instant::now() + run_for;
            while Instant::now() < deadline {
                let t0 = Instant::now();
                match client.get(url_arc.as_ref()).send().await {
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

    let reservoir_note = !sorted.is_empty() && (total_ok as usize) > sorted.len();
    let mean_ms = if sorted.is_empty() {
        None
    } else {
        let sum: u128 = sorted.iter().map(|&x| x as u128).sum();
        Some(sum as f64 / sorted.len() as f64 / 1_000_000.0)
    };

    let report = LoadReport {
        url,
        wall_seconds: wall_secs,
        concurrency,
        ok_requests: total_ok,
        error_requests: err,
        rps_ok_only: rps,
        latency_sample_count: sorted.len(),
        latency_mean_ms: mean_ms,
        latency_mean_is_reservoir: reservoir_note,
        latency_p50_ms: percentile(&sorted, 50.0),
        latency_p95_ms: percentile(&sorted, 95.0),
        latency_p99_ms: percentile(&sorted, 99.0),
        reservoir_cap: RESERVOIR_CAP,
        total_ok_exceeds_sample: reservoir_note,
        tool: "poolai_health_load",
    };

    if json_out {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        eprintln!("URL: {}", report.url);
        eprintln!(
            "Duration (wall): {:.3}s  Workers: {}  OK: {}  Errors: {}",
            wall_secs, concurrency, total_ok, err
        );
        eprintln!("Requests/sec (OK only): {:.1}", rps);
        if !sorted.is_empty() {
            let mean_label = if reservoir_note {
                "mean (reservoir sample)"
            } else {
                "mean"
            };
            eprintln!(
                "Latency OK (n={} latencies recorded) — {}: {:.3} ms  p50: {:.3} ms  p95: {:.3} ms  p99: {:.3} ms",
                sorted.len(),
                mean_label,
                mean_ms.unwrap_or(0.0),
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
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse_cli_args, DEFAULT_URL};

    #[test]
    fn parse_defaults() {
        let (j, u, d, c) = parse_cli_args(vec![]);
        assert!(!j);
        assert_eq!(u, DEFAULT_URL);
        assert_eq!(d, 30.0);
        assert_eq!(c, 400);
    }

    #[test]
    fn parse_json_flag_only() {
        let (j, u, d, c) = parse_cli_args(vec!["--json".into()]);
        assert!(j);
        assert_eq!(u, DEFAULT_URL);
        assert_eq!(d, 30.0);
        assert_eq!(c, 400);
    }

    #[test]
    fn parse_json_before_url() {
        let (j, u, d, c) = parse_cli_args(vec![
            "--json".into(),
            "http://example/health".into(),
            "5".into(),
            "10".into(),
        ]);
        assert!(j);
        assert_eq!(u, "http://example/health");
        assert_eq!(d, 5.0);
        assert_eq!(c, 10);
    }

    #[test]
    fn parse_json_after_positionals() {
        let (j, u, d, c) = parse_cli_args(vec![
            "http://x".into(),
            "1".into(),
            "2".into(),
            "--json".into(),
        ]);
        assert!(j);
        assert_eq!(u, "http://x");
        assert_eq!(d, 1.0);
        assert_eq!(c, 2);
    }

    #[test]
    fn parse_concurrency_at_least_one() {
        let (_j, _u, d, c) =
            parse_cli_args(vec!["http://127.0.0.1/h".into(), "1".into(), "0".into()]);
        assert_eq!(d, 1.0);
        assert_eq!(c, 1);
    }
}
