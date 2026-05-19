//! MVP sidecar: read domain events (NDJSON on stdin), write acks on stdout.
//!
//! No Solana RPC in S37 — validates schema v1 only. Future versions may submit transactions.

use poolai_solana_adapter::sidecar::process_event_line;
use std::io::{self, BufRead, Write};
use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(io::stderr)
        .init();

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                tracing::error!("stdin read error: {e}");
                break;
            }
        };
        let ack = process_event_line(&line);
        if let Ok(json) = serde_json::to_string(&ack) {
            let _ = writeln!(stdout, "{json}");
            let _ = stdout.flush();
        }
    }
}
