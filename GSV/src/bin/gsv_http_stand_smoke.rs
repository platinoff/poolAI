//! `gsv-http-stand-smoke` — live HTTP stand smoke against a running GSV server.
//!
//! Mirrors the poolAI `poolai-http-stand-smoke` pattern for the GSV project: hits
//! key box endpoints over HTTP, reports per-case ok/fail and exits non-zero when
//! any case fails. Empty-tolerant by design — a 200 with a non-JSON or missing
//! `ok` payload is still reported (degraded), never panics.
//!
//! ```text
//! # GSV server on the canon port (9999):
//! cargo run --manifest-path GSV/Cargo.toml --bin gsv-http-stand-smoke
//!
//! # Custom base URL + JSON report:
//! cargo run --manifest-path GSV/Cargo.toml --bin gsv-http-stand-smoke -- --base-url http://127.0.0.1:9999 --json
//! ```
//!
//! Returns exit code 0 when all cases pass, 1 otherwise.

use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;

const DEFAULT_BASE: &str = "http://127.0.0.1:9999";
const ENV_BASE: &str = "GSV_BASE_URL";

/// Card names backed by the Rust UI fragment renderers (`boxes::ui::CARD_NAMES`).
/// Kept in sync via the `cards` contract test in `tests/gsv_stand_smoke_contracts.rs`.
const CARDS: [&str; 20] = [
    "tracker",
    "sli",
    "toolchain",
    "ratio",
    "hooks-tests",
    "hooks-bench",
    "sprint-map",
    "sprint-queue",
    "sprint-progress",
    "sprint-board",
    "speed-index",
    "rust-diagnostics",
    "omni",
    "galaxy-backdrop",
    "starfield",
    "rss-ticker",
    "gpu-mode",
    "power-menu",
    "panel-dock",
    "fullscreen",
];

#[derive(Debug, Clone)]
struct Cli {
    json_out: bool,
    base_url: String,
}

fn parse_cli() -> Cli {
    let mut json_out = false;
    let mut base_url = std::env::var(ENV_BASE)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_BASE.to_string());
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--json" => json_out = true,
            "--base-url" => {
                if let Some(v) = args.next() {
                    base_url = v;
                }
            }
            "--help" | "-h" => {
                println!(
                    "Usage: gsv-http-stand-smoke [--base-url URL] [--json]\n\
                     env: GSV_BASE_URL (default {DEFAULT_BASE})"
                );
                std::process::exit(0);
            }
            _ => {}
        }
    }
    Cli { json_out, base_url }
}

#[derive(Debug, Serialize)]
struct SmokeCaseResult {
    name: String,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
}

#[derive(Debug, Serialize)]
struct SmokeReport {
    base_url: String,
    ok: bool,
    passed: u32,
    failed: u32,
    cases: Vec<SmokeCaseResult>,
    tool: &'static str,
}

fn api_url(base: &str, path: &str) -> String {
    format!(
        "{}/{}",
        base.trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}

/// 200-level response that parses as JSON (struct wires may omit `ok`).
async fn check_json(client: &Client, base: &str, path: &str) -> Result<(), String> {
    let url = api_url(base, path);
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("{url}: {e}"))?;
    let status = resp.status();
    if !status.is_success() {
        return Err(format!("{url}: HTTP {}", status.as_u16()));
    }
    resp.json::<Value>()
        .await
        .map_err(|e| format!("{url}: invalid JSON ({e})"))?;
    Ok(())
}

/// 200-level response that also carries `ok: true` when the body is JSON.
async fn check_ok(client: &Client, base: &str, path: &str) -> Result<(), String> {
    let url = api_url(base, path);
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("{url}: {e}"))?;
    let status = resp.status();
    if !status.is_success() {
        return Err(format!("{url}: HTTP {}", status.as_u16()));
    }
    let body = resp
        .json::<Value>()
        .await
        .map_err(|e| format!("{url}: invalid JSON ({e})"))?;
    match body.get("ok").and_then(Value::as_bool) {
        Some(true) => Ok(()),
        Some(false) => Err(format!(
            "{url}: ok=false{}",
            body.get("error")
                .and_then(Value::as_str)
                .map(|e| format!(" ({e})"))
                .unwrap_or_default()
        )),
        None => Err(format!("{url}: missing ok flag (degraded wire)")),
    }
}

/// 200-level response with a declared `Content-Type` (SVG/assets etc.), body ignored.
async fn check_status(client: &Client, base: &str, path: &str) -> Result<(), String> {
    let url = api_url(base, path);
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("{url}: {e}"))?;
    let status = resp.status();
    if !status.is_success() {
        return Err(format!("{url}: HTTP {}", status.as_u16()));
    }
    Ok(())
}

/// `/api/ui/card/{name}` renders a non-empty `html` fragment with `ok: true`.
async fn check_card(client: &Client, base: &str, card: &str) -> Result<(), String> {
    let url = api_url(base, &format!("/api/ui/card/{card}"));
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("{url}: {e}"))?;
    let status = resp.status();
    if !status.is_success() {
        return Err(format!("{url}: HTTP {}", status.as_u16()));
    }
    let body = resp
        .json::<Value>()
        .await
        .map_err(|e| format!("{url}: invalid JSON ({e})"))?;
    if body.get("ok").and_then(Value::as_bool) != Some(true) {
        return Err(format!("{url}: ok!=true"));
    }
    let html = body.get("html").and_then(Value::as_str).unwrap_or_default();
    if html.trim().is_empty() {
        return Err(format!("{url}: empty card html"));
    }
    Ok(())
}

async fn run_smokes(cli: &Cli) -> SmokeReport {
    let client = match Client::builder().timeout(Duration::from_secs(30)).build() {
        Ok(c) => c,
        Err(e) => {
            return SmokeReport {
                base_url: cli.base_url.clone(),
                ok: false,
                passed: 0,
                failed: 1,
                cases: vec![SmokeCaseResult {
                    name: "client_build".into(),
                    ok: false,
                    detail: Some(e.to_string()),
                }],
                tool: "gsv-http-stand-smoke",
            };
        }
    };

    let mut cases: Vec<SmokeCaseResult> = Vec::new();
    async fn record(
        cases: &mut Vec<SmokeCaseResult>,
        name: &str,
        result: impl std::future::Future<Output = Result<(), String>>,
    ) {
        cases.push(match result.await {
            Ok(()) => SmokeCaseResult {
                name: name.to_string(),
                ok: true,
                detail: None,
            },
            Err(e) => SmokeCaseResult {
                name: name.to_string(),
                ok: false,
                detail: Some(e),
            },
        });
    }

    // Core boxes.
    record(
        &mut cases,
        "health",
        check_ok(&client, &cli.base_url, "/api/health"),
    )
    .await;
    record(
        &mut cases,
        "tracker",
        check_json(&client, &cli.base_url, "/api/tracker"),
    )
    .await;
    record(
        &mut cases,
        "sli",
        check_json(&client, &cli.base_url, "/api/sli"),
    )
    .await;
    record(
        &mut cases,
        "toolchain",
        check_json(&client, &cli.base_url, "/api/toolchain"),
    )
    .await;
    record(
        &mut cases,
        "update",
        check_json(&client, &cli.base_url, "/api/update"),
    )
    .await;
    record(
        &mut cases,
        "ratio",
        check_ok(&client, &cli.base_url, "/api/ratio"),
    )
    .await;
    record(
        &mut cases,
        "omni_status",
        check_json(&client, &cli.base_url, "/api/omni/status"),
    )
    .await;

    // Vision endpoints (wire shapes with `ok`).
    record(
        &mut cases,
        "vision_summary",
        check_ok(&client, &cli.base_url, "/api/vision"),
    )
    .await;
    record(
        &mut cases,
        "vision_manifest",
        check_ok(&client, &cli.base_url, "/api/vision/manifest"),
    )
    .await;
    record(
        &mut cases,
        "vision_feed",
        check_ok(&client, &cli.base_url, "/api/vision/feed"),
    )
    .await;
    record(
        &mut cases,
        "vision_map",
        check_ok(&client, &cli.base_url, "/api/vision/map"),
    )
    .await;
    record(
        &mut cases,
        "vision_sprint_map",
        check_ok(&client, &cli.base_url, "/api/vision/sprint-map"),
    )
    .await;
    record(
        &mut cases,
        "vision_sprint_queue",
        check_ok(&client, &cli.base_url, "/api/vision/sprint-queue"),
    )
    .await;
    record(
        &mut cases,
        "vision_sprint_board",
        check_ok(&client, &cli.base_url, "/api/vision/sprint-board"),
    )
    .await;
    record(
        &mut cases,
        "vision_sprint_progress",
        check_ok(&client, &cli.base_url, "/api/vision/sprint-progress"),
    )
    .await;
    record(
        &mut cases,
        "vision_speeds",
        check_ok(&client, &cli.base_url, "/api/vision/speeds"),
    )
    .await;
    record(
        &mut cases,
        "vision_rust_diagnostics",
        check_ok(&client, &cli.base_url, "/api/vision/rust-diagnostics"),
    )
    .await;
    record(
        &mut cases,
        "vision_extensions",
        check_ok(&client, &cli.base_url, "/api/vision/extensions"),
    )
    .await;
    record(
        &mut cases,
        "vision_node_search",
        check_ok(&client, &cli.base_url, "/api/vision/node-search?q=vision"),
    )
    .await;
    record(
        &mut cases,
        "vision_sprint_theme",
        check_ok(&client, &cli.base_url, "/api/vision/sprint-theme"),
    )
    .await;
    record(
        &mut cases,
        "vision_palette",
        check_ok(&client, &cli.base_url, "/api/vision/palette"),
    )
    .await;
    record(
        &mut cases,
        "vision_sync",
        check_ok(&client, &cli.base_url, "/api/vision/sync"),
    )
    .await;

    // Rust-rendered SVG assets (status only).
    record(
        &mut cases,
        "assets_vision_svg",
        check_status(&client, &cli.base_url, "/assets/vision.svg"),
    )
    .await;
    record(
        &mut cases,
        "vision_speeds_svg",
        check_status(&client, &cli.base_url, "/api/vision/speeds.svg"),
    )
    .await;
    record(
        &mut cases,
        "vision_rust_diagnostics_svg",
        check_status(&client, &cli.base_url, "/api/vision/rust-diagnostics.svg"),
    )
    .await;
    record(
        &mut cases,
        "vision_starfield_svg",
        check_status(&client, &cli.base_url, "/api/vision/starfield.svg?mode=eco"),
    )
    .await;
    record(
        &mut cases,
        "vision_galaxy_svg",
        check_status(&client, &cli.base_url, "/api/vision/galaxy.svg"),
    )
    .await;
    record(
        &mut cases,
        "vision_sprint_focus_svg",
        check_status(&client, &cli.base_url, "/api/vision/sprint-focus.svg"),
    )
    .await;

    // Rust-rendered UI card fragments (all registered cards).
    for card in CARDS {
        let name = format!("ui_card_{card}");
        record(&mut cases, &name, check_card(&client, &cli.base_url, card)).await;
    }

    let passed = cases.iter().filter(|c| c.ok).count() as u32;
    let failed = cases.iter().filter(|c| !c.ok).count() as u32;
    SmokeReport {
        base_url: cli.base_url.clone(),
        ok: failed == 0,
        passed,
        failed,
        cases,
        tool: "gsv-http-stand-smoke",
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = parse_cli();
    let report = run_smokes(&cli).await;
    if cli.json_out {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        for case in &report.cases {
            let mark = if case.ok { "ok" } else { "FAIL" };
            println!(
                "[{mark}] {}{}",
                case.name,
                case.detail
                    .as_deref()
                    .map(|d| format!(" — {d}"))
                    .unwrap_or_default()
            );
        }
        println!(
            "gsv-http-stand-smoke: {} passed / {} failed (base {})",
            report.passed, report.failed, report.base_url
        );
    }
    if !report.ok {
        std::process::exit(1);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_url_joins_path() {
        assert_eq!(
            api_url("http://127.0.0.1:9999", "/api/health"),
            "http://127.0.0.1:9999/api/health"
        );
        assert_eq!(
            api_url("http://127.0.0.1:9999/", "/api/health"),
            "http://127.0.0.1:9999/api/health"
        );
    }

    #[test]
    fn cards_match_ui_fragment_registry() {
        let ui_src = include_str!("../boxes/ui.rs");
        assert!(ui_src.contains("pub const CARD_NAMES"));
        for card in CARDS {
            assert!(
                ui_src.contains(&format!("\"{card}\"")),
                "card {card} missing from boxes/ui.rs CARD_NAMES"
            );
        }
        assert_eq!(CARDS.len(), 20);
    }

    #[test]
    fn report_shape_fields_present() {
        let report = SmokeReport {
            base_url: "http://127.0.0.1:9999".to_string(),
            ok: true,
            passed: 1,
            failed: 0,
            cases: vec![SmokeCaseResult {
                name: "health".to_string(),
                ok: true,
                detail: None,
            }],
            tool: "gsv-http-stand-smoke",
        };
        let json = serde_json::to_value(&report).expect("report serializes");
        assert_eq!(json["base_url"], "http://127.0.0.1:9999");
        assert_eq!(json["ok"], true);
        assert_eq!(json["passed"], 1);
        assert_eq!(json["failed"], 0);
        assert_eq!(json["cases"][0]["name"], "health");
        assert!(json["cases"][0].get("detail").is_none());
    }
}
