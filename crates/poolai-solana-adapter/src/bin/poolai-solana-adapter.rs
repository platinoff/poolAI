//! Sidecar: read domain events (NDJSON on stdin), write acks (+ mock RPC metadata) on stdout.
//!
//! FM-024: loads devnet config (`config/devnet.toml` or `POOLAI_SOLANA_CONFIG`); mock RPC only.

use poolai_solana_adapter::config::AdapterConfig;
use poolai_solana_adapter::sidecar::SidecarProcessor;
use std::io::{self, BufRead, Write};
use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(io::stderr)
        .init();

    let config = match AdapterConfig::from_env() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("adapter config: {e}");
            std::process::exit(2);
        }
    };
    tracing::info!(
        cluster = ?config.cluster,
        rpc_url = %config.rpc_url,
        mock_rpc = config.mock_rpc,
        "poolai-solana-adapter started"
    );

    let mut processor = SidecarProcessor::new(config);
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
        let ack = processor.process_line(&line);
        if let Ok(json) = serde_json::to_string(&ack) {
            let _ = writeln!(stdout, "{json}");
            let _ = stdout.flush();
        }
    }
}
