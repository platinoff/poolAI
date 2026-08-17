//! Rust compiler / Clippy warning+error index for Galaxy vision **Rust** panel.
//!
//! Canonical JSON: [`docs/development/rust_diagnostics.json`](../../docs/development/rust_diagnostics.json)
//! Vision mirror: `docs/vision/rust_diagnostics.json`.
//!
//! ```text
//! cargo run --bin poolai-rust-diagnostics -- --print
//! cargo run --bin poolai-rust-diagnostics -- --scan
//! cargo run --bin poolai-rust-diagnostics -- --record --warnings 0 --errors 0 --ok
//! cargo run --bin poolai-rust-diagnostics -- --from-json path/to/cargo.json
//! ```

use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

const DEFAULT_OUTPUT: &str = "docs/development/rust_diagnostics.json";
const VISION_MIRROR: &str = "docs/vision/rust_diagnostics.json";
const HISTORY_CAP: usize = 32;
const DEFAULT_SCAN_CMD: &str =
    "cargo clippy --message-format=json --all-targets --features jwt,https";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct RustDiagnostics {
    #[serde(default)]
    schema_version: u32,
    #[serde(default)]
    generated_at: String,
    #[serde(default)]
    host_label: String,
    #[serde(default)]
    git_head: String,
    #[serde(default)]
    source: String,
    #[serde(default)]
    notes: Vec<String>,
    #[serde(default)]
    latest: LatestDiagnostics,
    #[serde(default)]
    history: Vec<DiagnosticsEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct LatestDiagnostics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    warnings: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    errors: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ok: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    recorded_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    command: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    wall_secs: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    top_codes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiagnosticsEntry {
    kind: String,
    command: String,
    warnings: u32,
    errors: u32,
    ok: bool,
    recorded_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    wall_secs: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    git_head: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    top_codes: Option<Vec<String>>,
}

#[derive(Debug, Default)]
struct MessageCounts {
    warnings: u32,
    errors: u32,
    /// code → count (clippy::foo / E0xxx)
    codes: std::collections::BTreeMap<String, u32>,
}

fn repo_root() -> PathBuf {
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn git_head_short(root: &Path) -> String {
    Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .current_dir(root)
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "unknown".into())
}

fn load_index(path: &Path) -> RustDiagnostics {
    match fs::read_to_string(path) {
        Ok(raw) => serde_json::from_str(&raw).unwrap_or_default(),
        Err(_) => RustDiagnostics {
            schema_version: 1,
            notes: vec![
                "Machine-readable Rust/Clippy warning+error index for vision Rust panel.".into(),
                "Record via poolai-rust-diagnostics or bin/record-rust-diagnostics.sh (abrakadabra drain + CI)."
                    .into(),
            ],
            ..Default::default()
        },
    }
}

fn write_index(path: &Path, index: &RustDiagnostics) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let pretty = serde_json::to_string_pretty(index).map_err(|e| e.to_string())? + "\n";
    fs::write(path, pretty).map_err(|e| e.to_string())
}

fn mirror_to_vision(root: &Path, index: &RustDiagnostics) -> Result<(), String> {
    write_index(&root.join(VISION_MIRROR), index)
}

fn ingest_cargo_json_line(counts: &mut MessageCounts, line: &str) {
    let Ok(v) = serde_json::from_str::<Value>(line) else {
        return;
    };
    if v.get("reason").and_then(|r| r.as_str()) != Some("compiler-message") {
        return;
    }
    let msg = match v.get("message") {
        Some(m) => m,
        None => return,
    };
    // Skip children / rendered-only duplicates: only primary messages with a level.
    let level = msg.get("level").and_then(|l| l.as_str()).unwrap_or("");
    match level {
        "warning" => counts.warnings += 1,
        "error" | "error: internal compiler error" => counts.errors += 1,
        _ => return,
    }
    if let Some(code) = msg
        .get("code")
        .and_then(|c| c.get("code"))
        .and_then(|c| c.as_str())
    {
        *counts.codes.entry(code.to_string()).or_insert(0) += 1;
    }
}

fn parse_cargo_json_reader<R: BufRead>(reader: R) -> MessageCounts {
    let mut counts = MessageCounts::default();
    for line in reader.lines().map_while(Result::ok) {
        let trimmed = line.trim();
        if trimmed.is_empty() || !trimmed.starts_with('{') {
            continue;
        }
        ingest_cargo_json_line(&mut counts, trimmed);
    }
    counts
}

fn top_codes(counts: &MessageCounts, lim: usize) -> Vec<String> {
    let mut pairs: Vec<_> = counts.codes.iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
    pairs
        .into_iter()
        .take(lim)
        .map(|(k, n)| format!("{k}×{n}"))
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn record_snapshot(
    index: &mut RustDiagnostics,
    warnings: u32,
    errors: u32,
    ok: bool,
    command: String,
    host: String,
    head: String,
    source: String,
    wall_secs: Option<f64>,
    codes: Vec<String>,
) {
    let now = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    index.schema_version = 1;
    index.generated_at = now[..10].to_string();
    index.host_label = host.clone();
    index.git_head = head.clone();
    index.source = source.clone();
    index.latest.warnings = Some(warnings);
    index.latest.errors = Some(errors);
    index.latest.ok = Some(ok);
    index.latest.recorded_at = Some(now.clone());
    index.latest.command = Some(command.clone());
    index.latest.wall_secs = wall_secs;
    index.latest.top_codes = if codes.is_empty() {
        None
    } else {
        Some(codes.clone())
    };
    index.history.push(DiagnosticsEntry {
        kind: "rust_diagnostics".into(),
        command,
        warnings,
        errors,
        ok,
        recorded_at: now,
        wall_secs,
        host_label: Some(host),
        git_head: Some(head),
        source: Some(source),
        top_codes: if codes.is_empty() { None } else { Some(codes) },
    });
    if index.history.len() > HISTORY_CAP {
        let drop_n = index.history.len() - HISTORY_CAP;
        index.history.drain(0..drop_n);
    }
}

fn print_summary(index: &RustDiagnostics) {
    println!("rust_diagnostics schema={}", index.schema_version.max(1));
    println!("generated_at={}", index.generated_at);
    println!("host={}", index.host_label);
    println!("git_head={}", index.git_head);
    println!("source={}", index.source);
    match (
        index.latest.warnings,
        index.latest.errors,
        index.latest.ok,
        &index.latest.recorded_at,
    ) {
        (Some(w), Some(e), Some(ok), Some(at)) => {
            println!(
                "latest: warnings={w} errors={e} ok={ok} at={at} cmd={}",
                index.latest.command.as_deref().unwrap_or(DEFAULT_SCAN_CMD)
            );
            if let Some(codes) = &index.latest.top_codes {
                if !codes.is_empty() {
                    println!("top_codes: {}", codes.join(", "));
                }
            }
        }
        _ => println!("latest: (none)"),
    }
    println!("history: {}", index.history.len());
}

fn run_scan(root: &Path, command: &str) -> Result<(MessageCounts, bool, f64), String> {
    let start = std::time::Instant::now();
    // Split like a simple shell command: first token = program, rest = args.
    let mut parts = command.split_whitespace();
    let prog = parts
        .next()
        .ok_or_else(|| "empty scan command".to_string())?;
    let args: Vec<&str> = parts.collect();
    let mut child = Command::new(prog)
        .args(&args)
        .current_dir(root)
        .env("K8S_OPENAPI_ENABLED_VERSION", "1.28")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("spawn failed: {e}"))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "missing stdout".to_string())?;
    let counts = parse_cargo_json_reader(BufReader::new(stdout));
    let status = child.wait().map_err(|e| format!("wait failed: {e}"))?;
    let wall = start.elapsed().as_secs_f64();
    // Treat compiler errors as fail; clippy may exit non-zero on warnings depending on RUSTFLAGS.
    let ok = status.success() && counts.errors == 0;
    Ok((counts, ok, wall))
}

fn is_our_flag(s: &str) -> bool {
    matches!(
        s,
        "--print"
            | "--scan"
            | "--record"
            | "--ok"
            | "--fail"
            | "--from-json"
            | "--warnings"
            | "--errors"
            | "--wall-secs"
            | "--command"
            | "--host"
            | "--source"
            | "--output"
            | "-h"
            | "--help"
    )
}

fn usage() {
    eprintln!(
        "Usage:
  poolai-rust-diagnostics --print [--output PATH]
  poolai-rust-diagnostics --scan [--command CMD] [--host LABEL] [--source local|ci] [--output PATH]
  poolai-rust-diagnostics --from-json FILE [--command CMD] [--host LABEL] [--source local|ci] [--output PATH]
  poolai-rust-diagnostics --record --warnings N --errors N [--ok|--fail] [--command CMD] [--host LABEL] [--source local|ci] [--wall-secs SECS] [--output PATH]"
    );
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        usage();
        return ExitCode::SUCCESS;
    }

    let root = repo_root();
    let mut output = root.join(DEFAULT_OUTPUT);
    let mut mode_print = false;
    let mut mode_scan = false;
    let mut mode_record = false;
    let mut from_json: Option<PathBuf> = None;
    let mut warnings: Option<u32> = None;
    let mut errors: Option<u32> = None;
    let mut ok = true;
    let mut command = DEFAULT_SCAN_CMD.to_string();
    let mut host = env::var("COMPUTERNAME")
        .or_else(|_| env::var("HOSTNAME"))
        .unwrap_or_else(|_| "local".into());
    let mut source = "local".to_string();
    let mut wall_secs: Option<f64> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--print" => mode_print = true,
            "--scan" => mode_scan = true,
            "--record" => mode_record = true,
            "--ok" => ok = true,
            "--fail" => ok = false,
            "--from-json" => {
                i += 1;
                from_json = args.get(i).map(|s| root.join(s));
            }
            "--warnings" => {
                i += 1;
                warnings = args.get(i).and_then(|s| s.parse().ok());
            }
            "--errors" => {
                i += 1;
                errors = args.get(i).and_then(|s| s.parse().ok());
            }
            "--wall-secs" => {
                i += 1;
                wall_secs = args.get(i).and_then(|s| s.parse().ok());
            }
            "--command" => {
                i += 1;
                let mut parts = Vec::new();
                while i < args.len() && !is_our_flag(&args[i]) {
                    parts.push(args[i].clone());
                    i += 1;
                }
                if i < args.len() && is_our_flag(&args[i]) {
                    i -= 1; // re-process next flag in outer loop
                }
                if !parts.is_empty() {
                    command = parts.join(" ");
                }
            }
            "--host" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    host = v.clone();
                }
            }
            "--source" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    source = v.clone();
                }
            }
            "--output" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    output = root.join(v);
                }
            }
            other => {
                eprintln!("unknown arg: {other}");
                usage();
                return ExitCode::FAILURE;
            }
        }
        i += 1;
    }

    let head = git_head_short(&root);
    let mut index = load_index(&output);

    if let Ok(env_cmd) = env::var("RUST_DIAGNOSTICS_CMD") {
        if !env_cmd.trim().is_empty() && command == DEFAULT_SCAN_CMD {
            command = env_cmd;
        }
    }

    if mode_scan {
        match run_scan(&root, &command) {
            Ok((counts, scan_ok, wall)) => {
                let codes = top_codes(&counts, 8);
                record_snapshot(
                    &mut index,
                    counts.warnings,
                    counts.errors,
                    scan_ok,
                    command,
                    host,
                    head,
                    source,
                    Some(wall),
                    codes,
                );
            }
            Err(e) => {
                eprintln!("scan failed: {e}");
                return ExitCode::FAILURE;
            }
        }
        if let Err(e) = write_index(&output, &index) {
            eprintln!("write failed: {e}");
            return ExitCode::FAILURE;
        }
        if let Err(e) = mirror_to_vision(&root, &index) {
            eprintln!("vision mirror failed: {e}");
            return ExitCode::FAILURE;
        }
        print_summary(&index);
        // Non-zero if errors present (warnings alone do not fail local record).
        return if index.latest.errors.unwrap_or(0) > 0 {
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        };
    }

    if let Some(path) = from_json {
        let file = match fs::File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("open {}: {e}", path.display());
                return ExitCode::FAILURE;
            }
        };
        let counts = parse_cargo_json_reader(BufReader::new(file));
        let codes = top_codes(&counts, 8);
        let scan_ok = counts.errors == 0;
        record_snapshot(
            &mut index,
            counts.warnings,
            counts.errors,
            scan_ok,
            command,
            host,
            head,
            source,
            wall_secs,
            codes,
        );
        if let Err(e) = write_index(&output, &index) {
            eprintln!("write failed: {e}");
            return ExitCode::FAILURE;
        }
        if let Err(e) = mirror_to_vision(&root, &index) {
            eprintln!("vision mirror failed: {e}");
            return ExitCode::FAILURE;
        }
        print_summary(&index);
        return if counts.errors > 0 {
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        };
    }

    if mode_record {
        let (Some(w), Some(e)) = (warnings, errors) else {
            eprintln!("--record requires --warnings and --errors");
            return ExitCode::FAILURE;
        };
        record_snapshot(
            &mut index,
            w,
            e,
            ok && e == 0,
            command,
            host,
            head,
            source,
            wall_secs,
            vec![],
        );
        if let Err(err) = write_index(&output, &index) {
            eprintln!("write failed: {err}");
            return ExitCode::FAILURE;
        }
        if let Err(err) = mirror_to_vision(&root, &index) {
            eprintln!("vision mirror failed: {err}");
            return ExitCode::FAILURE;
        }
        print_summary(&index);
        return ExitCode::SUCCESS;
    }

    // default / --print
    if !output.exists() && !mode_print {
        index.schema_version = 1;
        index.generated_at = Utc::now().format("%Y-%m-%d").to_string();
        index.host_label = host;
        index.git_head = head;
        index.source = source;
        let _ = write_index(&output, &index);
        let _ = mirror_to_vision(&root, &index);
    }
    print_summary(&index);
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parse_compiler_messages_counts_levels() {
        let sample = r#"
{"reason":"compiler-message","message":{"level":"warning","code":{"code":"clippy::needless_borrow"},"message":"x"}}
{"reason":"compiler-message","message":{"level":"warning","code":{"code":"clippy::needless_borrow"},"message":"y"}}
{"reason":"compiler-message","message":{"level":"error","code":{"code":"E0425"},"message":"z"}}
{"reason":"build-finished","success":false}
"#;
        let counts = parse_cargo_json_reader(Cursor::new(sample));
        assert_eq!(counts.warnings, 2);
        assert_eq!(counts.errors, 1);
        assert_eq!(counts.codes.get("clippy::needless_borrow"), Some(&2));
        let tops = top_codes(&counts, 2);
        assert!(tops[0].starts_with("clippy::needless_borrow"));
    }

    #[test]
    fn history_caps() {
        let mut idx = RustDiagnostics::default();
        for i in 0..40 {
            record_snapshot(
                &mut idx,
                i,
                0,
                true,
                "cmd".into(),
                "host".into(),
                "abc".into(),
                "local".into(),
                Some(1.0),
                vec![],
            );
        }
        assert_eq!(idx.history.len(), HISTORY_CAP);
        assert_eq!(idx.latest.warnings, Some(39));
    }
}
