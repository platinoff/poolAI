//! `gsv-server` — Galaxy StarWalker Vision binary server.
//!
//! Self-contained Rust bin serving the GSV UI (`/`), REST box API (`/api/*`) and
//! SSE events (`/events`).
//!
//! ```text
//! cargo run --manifest-path GSV/Cargo.toml --bin gsv-server
//! cargo run --manifest-path GSV/Cargo.toml --bin gsv-server -- --port 9999
//! cargo run --manifest-path GSV/Cargo.toml --bin gsv-server -- --repo-root /s/rust/poolAI
//! ```

use std::net::SocketAddr;
use std::path::PathBuf;

use tokio::sync::broadcast;
use tracing_subscriber::EnvFilter;

use gsv::{AppState, DEFAULT_HOST, DEFAULT_PORT, GSV_SERVER_NAME};

/// Simple CLI parser (no external deps): `--port N`, `--host H`, `--repo-root P`,
/// `--data-dir P`, `--help`.
fn parse_args() -> (String, u16, Option<PathBuf>, Option<PathBuf>) {
    let mut host = DEFAULT_HOST.to_string();
    let mut port = DEFAULT_PORT;
    let mut repo_root = None;
    let mut data_dir = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--port" => {
                port = args
                    .next()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(DEFAULT_PORT)
            }
            "--host" => host = args.next().unwrap_or_else(|| DEFAULT_HOST.to_string()),
            "--repo-root" => repo_root = args.next().map(PathBuf::from),
            "--data-dir" => data_dir = args.next().map(PathBuf::from),
            "--help" | "-h" => {
                println!("Usage: gsv-server [--host H] [--port N] [--repo-root P] [--data-dir P]");
                std::process::exit(0);
            }
            _ => {}
        }
    }
    (host, port, repo_root, data_dir)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,gsv=debug")),
        )
        .init();

    let (host, port, repo_root, data_dir) = parse_args();
    let (tx, _rx) = broadcast::channel(256);
    let state = AppState::new(repo_root.clone(), data_dir.clone(), tx);

    let addr: SocketAddr = format!("{host}:{port}").parse()?;
    let app = gsv::server::router(state.clone());
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(
        name = GSV_SERVER_NAME,
        version = %*state.version,
        %addr,
        repo_root = %state.repo_root.display(),
        data_dir = %state.data_dir.display(),
        "gsv-server listening"
    );

    // Emit an initial SSE event on startup.
    let _ = state.events.send(format!(
        "event: ready\ndata: {{\"version\":\"{}\",\"addr\":\"{addr}\"}}",
        state.version.as_ref()
    ));

    axum::serve(listener, app).await?;
    Ok(())
}
