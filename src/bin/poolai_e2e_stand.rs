//! E2E stand lifecycle (PH-S158): start / restart / stop poolai for Playwright + stand smoke.
//!
//! ```text
//! cargo run --bin poolai-e2e-stand -- start --port 8080 --print-stand-root
//! cargo run --bin poolai-e2e-stand -- restart --stand-root /tmp/poolai-e2e-NNN
//! cargo run --bin poolai-e2e-stand -- stop --stand-root /tmp/poolai-e2e-NNN
//! ```

use reqwest::Client;
use serde::Serialize;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

const ENV_STAND_ROOT: &str = "POOLAI_E2E_STAND_ROOT";
const ENV_PORT: &str = "POOLAI_HTTP_PORT";
const ENV_BASE: &str = "POOLAI_BASE_URL";
const ENV_PROFILE: &str = "POOLAI_E2E_PROFILE";
const ENV_FEATURES: &str = "POOLAI_FEATURES";
const ENV_JOB_STORE: &str = "POOLAI_JOB_STORE";
const ENV_LEASE_TTL: &str = "POOLAI_JOB_LEASE_TTL_SECS";
const ENV_PRICING_FB: &str = "POOLAI_GALAXY_PRICING_FALLBACK_JSON";
const ENV_K8S: &str = "K8S_OPENAPI_ENABLED_VERSION";
const ENV_RUST_LOG: &str = "RUST_LOG";
const ENV_STAND_BIN: &str = "POOLAI_E2E_STAND_BIN";
const ENV_POOLAI_BIN: &str = "POOLAI_BIN";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommandKind {
    Start,
    Restart,
    Stop,
}

#[derive(Debug, Clone)]
struct Cli {
    command: CommandKind,
    stand_root: Option<PathBuf>,
    port: u16,
    profile: String,
    print_stand_root: bool,
    json_out: bool,
    health_tries: u32,
}

#[derive(Debug, Serialize)]
struct StandReport {
    ok: bool,
    command: &'static str,
    stand_root: String,
    port: u16,
    pid: Option<u32>,
    base_url: String,
    poolai_bin: String,
    stand_bin: String,
    tool: &'static str,
}

fn default_port() -> u16 {
    std::env::var(ENV_PORT)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8080)
}

fn default_profile() -> String {
    let from_env = std::env::var(ENV_PROFILE).ok();
    if let Some(p) = from_env.filter(|s| !s.is_empty()) {
        return p;
    }
    if std::env::var("CI").ok().as_deref() == Some("true") {
        "debug".to_string()
    } else {
        "release".to_string()
    }
}

fn parse_cli() -> Result<Cli, String> {
    let mut command = None;
    let mut stand_root = None;
    let mut port = default_port();
    let mut profile = default_profile();
    let mut print_stand_root = false;
    let mut json_out = false;
    let mut health_tries = 90;

    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "start" => command = Some(CommandKind::Start),
            "restart" => command = Some(CommandKind::Restart),
            "stop" => command = Some(CommandKind::Stop),
            "--stand-root" => {
                i += 1;
                let val = args.get(i).ok_or("--stand-root requires path")?;
                stand_root = Some(PathBuf::from(val));
            }
            "--port" => {
                i += 1;
                port = args
                    .get(i)
                    .and_then(|v| v.parse().ok())
                    .ok_or("--port requires u16")?;
            }
            "--profile" => {
                i += 1;
                profile = args.get(i).ok_or("--profile requires value")?.clone();
            }
            "--print-stand-root" => print_stand_root = true,
            "--json" => json_out = true,
            "--health-tries" => {
                i += 1;
                health_tries = args
                    .get(i)
                    .and_then(|v| v.parse().ok())
                    .ok_or("--health-tries requires u32")?;
            }
            _ if args[i].starts_with('-') => {}
            _ => {}
        }
        i += 1;
    }

    let command = command.unwrap_or(CommandKind::Start);
    if stand_root.is_none() {
        stand_root = std::env::var(ENV_STAND_ROOT)
            .ok()
            .filter(|s| !s.is_empty())
            .map(PathBuf::from);
    }

    Ok(Cli {
        command,
        stand_root,
        port,
        profile,
        print_stand_root,
        json_out,
        health_tries,
    })
}

fn repo_root() -> PathBuf {
    std::env::var("POOLAI_REPO_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
}

fn resolve_poolai_bin(profile: &str) -> Result<PathBuf, String> {
    let root = repo_root();
    let candidates = [
        root.join(format!("target/{profile}/poolai")),
        root.join(format!("target/{profile}/poolai.exe")),
    ];
    for path in candidates {
        if path.is_file() {
            return Ok(path);
        }
    }
    let features = std::env::var(ENV_FEATURES).unwrap_or_else(|_| {
        if std::env::var("CI").ok().as_deref() == Some("true") {
            "enterprise,cloud,test-utils".to_string()
        } else {
            "enterprise,ml,cloud,test-utils".to_string()
        }
    });
    Err(format!(
    "poolai binary not found for profile `{profile}`; run: cargo build --{profile} --features {features}"
  ))
}

fn resolve_stand_bin() -> PathBuf {
    std::env::current_exe().unwrap_or_else(|_| PathBuf::from("poolai-e2e-stand"))
}

fn default_stand_root() -> PathBuf {
    let pid = std::process::id();
    PathBuf::from(format!("/tmp/poolai-e2e-{pid}"))
}

fn base_url(port: u16) -> String {
    std::env::var(ENV_BASE)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| format!("http://127.0.0.1:{port}"))
}

fn read_pid(stand_root: &Path) -> Option<u32> {
    let raw = fs::read_to_string(stand_root.join("pid")).ok()?;
    raw.trim().parse().ok()
}

fn write_pid(stand_root: &Path, pid: u32) -> Result<(), String> {
    fs::write(stand_root.join("pid"), format!("{pid}\n")).map_err(|e| e.to_string())?;
    Ok(())
}

fn kill_pid(pid: u32) -> Result<(), String> {
    let status = Command::new("kill")
        .arg(pid.to_string())
        .status()
        .map_err(|e| format!("kill failed: {e}"))?;
    if !status.success() {
        return Err(format!("kill pid {pid} exit {status}"));
    }
    Ok(())
}

fn stop_stand(stand_root: &Path) -> Result<Option<u32>, String> {
    let pid = read_pid(stand_root);
    if let Some(pid) = pid {
        if Command::new("kill")
            .args(["-0", &pid.to_string()])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            kill_pid(pid)?;
        }
    }
    Ok(pid)
}

async fn wait_health(base: &str, tries: u32) -> Result<(), String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
        .map_err(|e| e.to_string())?;
    let url = format!("{}/api/v1/health", base.trim_end_matches('/'));
    for _ in 0..tries {
        if let Ok(resp) = client.get(&url).send().await {
            if resp.status().is_success() {
                eprintln!("OK  health -> {url}");
                return Ok(());
            }
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    Err(format!("health not ready at {url}"))
}

struct StandPaths {
    stand_root: PathBuf,
    poolai_bin: PathBuf,
    stand_bin: PathBuf,
    port: u16,
    profile: String,
    job_store: String,
    lease_ttl: String,
    pricing_fallback: String,
    k8s_openapi: String,
    rust_log: String,
}

fn build_stand_paths(cli: &Cli, stand_root: PathBuf) -> Result<StandPaths, String> {
    Ok(StandPaths {
        stand_root,
        poolai_bin: resolve_poolai_bin(&cli.profile)?,
        stand_bin: resolve_stand_bin(),
        port: cli.port,
        profile: cli.profile.clone(),
        job_store: std::env::var(ENV_JOB_STORE).unwrap_or_else(|_| "raid".to_string()),
        lease_ttl: std::env::var(ENV_LEASE_TTL).unwrap_or_else(|_| "2".to_string()),
        pricing_fallback: std::env::var(ENV_PRICING_FB)
            .unwrap_or_else(|_| "{\"inference_blended_token\":470000}".to_string()),
        k8s_openapi: std::env::var(ENV_K8S).unwrap_or_else(|_| "1.28".to_string()),
        rust_log: std::env::var(ENV_RUST_LOG).unwrap_or_else(|_| "warn".to_string()),
    })
}

fn write_stand_env(paths: &StandPaths) -> Result<(), String> {
    let content = format!(
        "POOLAI_HTTP_PORT={}\n\
POOLAI_RAID_BASE_PATH={}/raid\n\
POOLAI_DATA_PATH={}/data\n\
POOLAI_JOB_STORE={}\n\
POOLAI_JOB_LEASE_TTL_SECS={}\n\
POOLAI_GALAXY_PRICING_FALLBACK_JSON={}\n\
POOLAI_E2E_PROFILE={}\n\
RUST_LOG={}\n\
K8S_OPENAPI_ENABLED_VERSION={}\n\
POOLAI_BIN={}\n\
{}={}\n",
        paths.port,
        paths.stand_root.display(),
        paths.stand_root.display(),
        paths.job_store,
        paths.lease_ttl,
        paths.pricing_fallback,
        paths.profile,
        paths.rust_log,
        paths.k8s_openapi,
        paths.poolai_bin.display(),
        ENV_STAND_BIN,
        paths.stand_bin.display(),
    );
    fs::write(paths.stand_root.join("stand.env"), content).map_err(|e| e.to_string())?;
    Ok(())
}

fn write_restart_script(paths: &StandPaths) -> Result<(), String> {
    let script = r#"#!/usr/bin/env bash
set -euo pipefail
STAND_ROOT="$(cd "$(dirname "$0")" && pwd)"
# shellcheck source=/dev/null
source "${STAND_ROOT}/stand.env"
exec "${POOLAI_E2E_STAND_BIN}" restart --stand-root "${STAND_ROOT}"
"#;
    let path = paths.stand_root.join("restart.sh");
    fs::write(&path, script).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path)
            .map_err(|e| e.to_string())?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&path, perms).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn spawn_poolai(paths: &StandPaths) -> Result<u32, String> {
    let log_path = paths.stand_root.join("poolai.log");
    let log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .map_err(|e| format!("open log: {e}"))?;
    let log_err = log.try_clone().map_err(|e| e.to_string())?;

    let child = Command::new(&paths.poolai_bin)
        .env(ENV_PORT, paths.port.to_string())
        .env("POOLAI_RAID_BASE_PATH", paths.stand_root.join("raid"))
        .env("POOLAI_DATA_PATH", paths.stand_root.join("data"))
        .env(ENV_JOB_STORE, &paths.job_store)
        .env(ENV_LEASE_TTL, &paths.lease_ttl)
        .env(ENV_PRICING_FB, &paths.pricing_fallback)
        .env(ENV_RUST_LOG, &paths.rust_log)
        .env(ENV_K8S, &paths.k8s_openapi)
        .stdout(Stdio::from(log))
        .stderr(Stdio::from(log_err))
        .spawn()
        .map_err(|e| format!("spawn poolai: {e}"))?;

    Ok(child.id())
}

fn prepare_stand_dirs(stand_root: &Path) -> Result<(), String> {
    fs::create_dir_all(stand_root.join("raid")).map_err(|e| e.to_string())?;
    fs::create_dir_all(stand_root.join("data")).map_err(|e| e.to_string())?;
    Ok(())
}

async fn start_stand(cli: &Cli) -> Result<StandReport, String> {
    let stand_root = cli.stand_root.clone().unwrap_or_else(default_stand_root);
    prepare_stand_dirs(&stand_root)?;
    let paths = build_stand_paths(cli, stand_root)?;
    write_stand_env(&paths)?;
    write_restart_script(&paths)?;

    eprintln!(
        "Starting poolai ({}) on port {} (job store: {})...",
        paths.poolai_bin.display(),
        paths.port,
        paths.job_store
    );

    let pid = spawn_poolai(&paths)?;
    write_pid(&paths.stand_root, pid)?;
    let base = base_url(paths.port);
    wait_health(&base, cli.health_tries).await?;

    Ok(StandReport {
        ok: true,
        command: "start",
        stand_root: paths.stand_root.display().to_string(),
        port: paths.port,
        pid: Some(pid),
        base_url: base,
        poolai_bin: paths.poolai_bin.display().to_string(),
        stand_bin: paths.stand_bin.display().to_string(),
        tool: "poolai-e2e-stand",
    })
}

async fn restart_stand(cli: &Cli) -> Result<StandReport, String> {
    let stand_root = cli
        .stand_root
        .clone()
        .ok_or(format!("restart requires --stand-root or {ENV_STAND_ROOT}"))?;
    if !stand_root.is_dir() {
        return Err(format!("stand root not found: {}", stand_root.display()));
    }
    stop_stand(&stand_root)?;
    let paths = build_stand_paths(cli, stand_root)?;
    if !paths.stand_root.join("stand.env").is_file() {
        write_stand_env(&paths)?;
        write_restart_script(&paths)?;
    }
    let pid = spawn_poolai(&paths)?;
    write_pid(&paths.stand_root, pid)?;
    let base = base_url(paths.port);
    wait_health(&base, cli.health_tries).await?;
    Ok(StandReport {
        ok: true,
        command: "restart",
        stand_root: paths.stand_root.display().to_string(),
        port: paths.port,
        pid: Some(pid),
        base_url: base,
        poolai_bin: paths.poolai_bin.display().to_string(),
        stand_bin: paths.stand_bin.display().to_string(),
        tool: "poolai-e2e-stand",
    })
}

fn stop_stand_report(cli: &Cli) -> Result<StandReport, String> {
    let stand_root = cli
        .stand_root
        .clone()
        .ok_or(format!("stop requires --stand-root or {ENV_STAND_ROOT}"))?;
    let pid = stop_stand(&stand_root)?;
    Ok(StandReport {
        ok: true,
        command: "stop",
        stand_root: stand_root.display().to_string(),
        port: cli.port,
        pid,
        base_url: base_url(cli.port),
        poolai_bin: resolve_poolai_bin(&cli.profile)
            .map(|p| p.display().to_string())
            .unwrap_or_default(),
        stand_bin: resolve_stand_bin().display().to_string(),
        tool: "poolai-e2e-stand",
    })
}

fn print_report(cli: &Cli, report: &StandReport) {
    if cli.json_out {
        println!(
            "{}",
            serde_json::to_string_pretty(report).unwrap_or_default()
        );
    } else if cli.print_stand_root {
        println!("{}", report.stand_root);
    } else {
        eprintln!(
            "poolai-e2e-stand {} OK stand={} pid={:?} base={}",
            report.command, report.stand_root, report.pid, report.base_url
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = match parse_cli() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("poolai-e2e-stand: {e}");
            std::process::exit(1);
        }
    };

    let report = match cli.command {
        CommandKind::Start => start_stand(&cli).await?,
        CommandKind::Restart => restart_stand(&cli).await?,
        CommandKind::Stop => stop_stand_report(&cli)?,
    };

    print_report(&cli, &report);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn write_stand_env_contains_paths() {
        let tmp = TempDir::new().expect("tempdir");
        let paths = StandPaths {
            stand_root: tmp.path().to_path_buf(),
            poolai_bin: PathBuf::from("/tmp/poolai"),
            stand_bin: PathBuf::from("/tmp/poolai-e2e-stand"),
            port: 8080,
            profile: "release".to_string(),
            job_store: "raid".to_string(),
            lease_ttl: "2".to_string(),
            pricing_fallback: "{\"inference_blended_token\":470000}".to_string(),
            k8s_openapi: "1.28".to_string(),
            rust_log: "warn".to_string(),
        };
        write_stand_env(&paths).expect("write stand.env");
        let content = fs::read_to_string(tmp.path().join("stand.env")).expect("read");
        assert!(content.contains("POOLAI_JOB_STORE=raid"));
        assert!(content.contains("POOLAI_BIN=/tmp/poolai"));
    }

    #[test]
    fn restart_script_invokes_stand_bin() {
        let tmp = TempDir::new().expect("tempdir");
        let paths = StandPaths {
            stand_root: tmp.path().to_path_buf(),
            poolai_bin: PathBuf::from("/tmp/poolai"),
            stand_bin: PathBuf::from("/tmp/poolai-e2e-stand"),
            port: 9090,
            profile: "debug".to_string(),
            job_store: "raid".to_string(),
            lease_ttl: "2".to_string(),
            pricing_fallback: "{}".to_string(),
            k8s_openapi: "1.28".to_string(),
            rust_log: "warn".to_string(),
        };
        write_restart_script(&paths).expect("restart.sh");
        let script = fs::read_to_string(tmp.path().join("restart.sh")).expect("read");
        assert!(script.contains("restart --stand-root"));
        assert!(script.contains(ENV_STAND_BIN));
    }

    #[test]
    fn default_profile_ci_is_debug() {
        std::env::set_var("CI", "true");
        std::env::remove_var(ENV_PROFILE);
        assert_eq!(default_profile(), "debug");
        std::env::remove_var("CI");
    }
}
