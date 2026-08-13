//! UI fragments — server-rendered card body HTML (Rust, ratio-safe).
//!
//! The dashboard cards are rendered here instead of in `ui/index.html` so the
//! JS glue stays thin and the Rust ratio holds. Each renderer takes the card's
//! wire `Value` and returns the card-body HTML exactly as the former JS
//! `render*` functions produced it.

use serde_json::Value;

/// HTML-escape a string (`&`, `<`, `>`), matching the JS `esc` helper.
pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Table markup, matching the JS `tab` helper (headers raw, cells pre-built).
pub fn tab(headers: &[&str], rows: Vec<Vec<String>>) -> String {
    let rows = if rows.is_empty() {
        vec![headers
            .iter()
            .map(|_| "<span class='dim'>—</span>".to_string())
            .collect()]
    } else {
        rows
    };
    let mut out = String::from("<table><tr>");
    for h in headers {
        out.push_str(&format!("<th>{h}</th>"));
    }
    out.push_str("</tr>");
    for row in rows {
        out.push_str("<tr>");
        for cell in row {
            out.push_str(&format!("<td>{cell}</td>"));
        }
        out.push_str("</tr>");
    }
    out.push_str("</table>");
    out
}

/// Progress bar markup, matching the JS `bar` helper.
pub fn bar(pct: f64) -> String {
    let w = pct.clamp(0.0, 100.0);
    format!(
        "<div style='background:var(--line);border-radius:6px;height:10px;margin:6px 0;overflow:hidden'><div style='width:{w}%;height:100%;background:var(--accent)'></div></div><div class='dim'>{w}% closed</div>"
    )
}

fn s(v: &Value) -> String {
    v.as_str().unwrap_or("").to_string()
}

fn u(v: &Value) -> u64 {
    v.as_u64().unwrap_or(0)
}

fn b(v: &Value) -> bool {
    v.as_bool().unwrap_or(false)
}

fn f(v: &Value) -> f64 {
    v.as_f64().unwrap_or(0.0)
}

fn arr(v: &Value) -> Vec<Value> {
    v.as_array().cloned().unwrap_or_default()
}

/// Tracker card (`/api/tracker`).
pub fn render_tracker(d: &Value) -> String {
    let sp = &d["sprints"];
    let open = arr(&sp["open"]).len();
    let closed = arr(&sp["closed"]).len();
    let total = u(&sp["total"]);
    let next_raw = s(&sp["next"]);
    let next = if next_raw.is_empty() {
        "—".to_string()
    } else {
        esc(&next_raw)
    };
    let mut out = format!(
        "<div class='dim'>open {open} · closed {closed} · total {total} · next <kbd>{next}</kbd></div>"
    );
    let rows = arr(&d["records"])
        .iter()
        .map(|r| {
            vec![
                esc(&s(&r["kind"])),
                esc(&s(&r["label"])),
                esc(&s(&r["status"])),
                format!("<span class='dim'>{}</span>", esc(&s(&r["at"]))),
            ]
        })
        .collect();
    out.push_str(&tab(&["kind", "label", "status", "at"], rows));
    out
}

/// SLI console card (`/api/sli`).
pub fn render_sli(d: &Value) -> String {
    let c = &d["catalog"];
    let used = u(&c["used_count"]);
    let unused = u(&c["unused_count"]);
    let mut out =
        format!("<div class='dim'>used {used} · unused (new SLI candidates) {unused}</div>");
    let rows = arr(&c["entries"])
        .iter()
        .map(|e| {
            let mark = if b(&e["used"]) {
                "<span class='ok'>●</span>".to_string()
            } else {
                "<span class='dim'>○</span>".to_string()
            };
            vec![
                mark,
                format!("<kbd>{}</kbd>", esc(&s(&e["name"]))),
                esc(&s(&e["kind"])),
                format!("<span class='dim'>{}</span>", esc(&s(&e["description"]))),
            ]
        })
        .collect();
    out.push_str(&tab(&["", "cmd", "kind", "desc"], rows));
    out
}

/// Toolchain card (`/api/toolchain`).
pub fn render_toolchain(d: &Value) -> String {
    let rows = arr(&d["entries"])
        .iter()
        .map(|e| {
            vec![
                format!("<kbd>{}</kbd>", esc(&s(&e["tool"]))),
                esc(&s(&e["version"])),
                format!("<span class='dim'>{}</span>", esc(&s(&e["source"]))),
            ]
        })
        .collect();
    tab(&["tool", "version", "source"], rows)
}

/// Ratio card (`/api/ratio`).
pub fn render_ratio(d: &Value) -> String {
    if !b(&d["ok"]) {
        let msg = d["error"].as_str().unwrap_or("missing rust_ratio.json");
        return format!("<span class='err'>{}</span>", esc(msg));
    }
    let cls = if b(&d["meets_min_ratio"]) {
        "ok"
    } else {
        "err"
    };
    let pct = f(&d["rust_ratio_pct"]);
    let band = f(&d["formal_band_min"]) * 100.0;
    let mut out =
        format!("<div>Rust ratio <span class='{cls}'>{pct:.2}%</span> · band min {band:.0}%</div>");
    out.push_str(&format!(
        "<div class='dim'>rust {} / non-rust {} · product {}</div>",
        u(&d["rust_loc"]),
        u(&d["non_rust_product_loc"]),
        u(&d["product_loc_total"]),
    ));
    let mut rows: Vec<Vec<String>> = Vec::new();
    if let Some(obj) = d["by_category"].as_object() {
        rows = obj
            .iter()
            .map(|(k, v)| {
                vec![
                    k.clone(),
                    u(&v["files"]).to_string(),
                    u(&v["loc"]).to_string(),
                ]
            })
            .collect();
    }
    out.push_str(&tab(&["category", "files", "loc"], rows));
    out
}

/// Sprint Queue card (`/api/vision/sprint-queue`).
pub fn render_sprint_queue(d: &Value) -> String {
    if !b(&d["ok"]) {
        let msg = d["error"].as_str().unwrap_or("sprint queue unavailable");
        return format!("<span class='err'>{}</span>", esc(msg));
    }
    let active = esc(&s(&d["active_sprint"]));
    let mut out = format!(
        "<div class='dim'>rev <kbd>{}</kbd> · next <span class='sprint-pill'>{}</span> · last closed <kbd>{}</kbd></div>",
        esc(&s(&d["revision"])),
        esc(&s(&d["next_sprint"])),
        esc(&s(&d["last_sprint_closed"])),
    );
    out.push_str(&format!(
        "<div>active <span class='sprint-pill'>{active}</span> · open <span class='ok'>{}</span></div>",
        u(&d["open_count"]),
    ));
    let planned = arr(&d["planned"]);
    out.push_str(&format!(
        "<details style='margin-top:6px'><summary>planned ({})</summary>",
        planned.len()
    ));
    let rows = planned
        .iter()
        .map(|p| {
            let pclass = if s(&p["id"]) == s(&d["active_sprint"]) {
                "open"
            } else if s(&p["id"]) == s(&d["next_sprint"]) {
                "next"
            } else {
                "closed"
            };
            vec![
                format!("<span class='squeue {pclass}'>{}</span>", esc(&s(&p["id"]))),
                format!("<span class='squeue-st'>{}</span>", esc(&s(&p["status"]))),
                esc(&s(&p["category"])),
                esc(&s(&p["title"])),
            ]
        })
        .collect();
    out.push_str(&tab(&["id", "status", "category", "title"], rows));
    out.push_str("</details>");
    out
}

/// Sprint Progress card (`/api/vision/sprint-progress`).
pub fn render_sprint_progress(d: &Value) -> String {
    if !b(&d["ok"]) {
        let msg = d["error"].as_str().unwrap_or("sprint progress unavailable");
        return format!("<span class='err'>{}</span>", esc(msg));
    }
    let mut out = format!(
        "<div class='dim'>rev <kbd>{}</kbd></div>",
        esc(&s(&d["revision"]))
    );
    out.push_str(&format!(
        "<div>open <span class='warn'>{}</span> · closed <span class='ok'>{}</span> · planned <span class='dim'>{}</span> · total <span class='dim'>{}</span></div>",
        u(&d["open_count"]),
        u(&d["closed_count"]),
        u(&d["planned_count"]),
        u(&d["total"]),
    ));
    out.push_str(&bar(f(&d["progress_pct"])));
    let rows = arr(&d["layers"])
        .iter()
        .map(|l| {
            vec![
                format!("<kbd>{}</kbd>", esc(&s(&l["id"]))),
                s(&l["z"]),
                format!("<span class='dim'>{}</span>", u(&l["node_count"])),
                format!("<span class='ok'>{}</span>", u(&l["linked_count"])),
            ]
        })
        .collect();
    out.push_str(&tab(&["layer", "z", "nodes", "linked"], rows));
    out
}

/// Speed Index card (`/api/vision/speeds`).
pub fn render_speed_index(d: &Value) -> String {
    if !b(&d["ok"]) {
        let msg = d["error"].as_str().unwrap_or("unavailable");
        return format!("<span class='err'>{}</span>", esc(msg));
    }
    if !b(&d["present"]) {
        return "<div class='dim'>no speed_index.json — run bin/record-test-ci-speed.sh</div>"
            .to_string();
    }
    let si = &d["speed_index"];
    let l = &si["latest"];
    let wall = f(&l["test_ci_wall_secs"]);
    let bench_ns = l["last_bench_median_ns"].as_f64();
    let bench = match bench_ns {
        Some(ns) => format!("{:.2} ms", ns / 1e6),
        None => "—".to_string(),
    };
    let okmark = if b(&l["test_ci_ok"]) {
        " <span class='ok'>ok</span>".to_string()
    } else {
        " <span class='err'>fail</span>".to_string()
    };
    let mut out = format!(
        "<div class='dim'>{} · {} · git {}</div>",
        esc(&s(&si["host_label"])),
        esc(&s(&si["generated_at"])),
        esc(&s(&si["git_head"])),
    );
    out.push_str(&format!(
        "<div>test-ci <kbd>{wall:.1}s</kbd>{okmark} · bench <kbd>{bench}</kbd></div>"
    ));
    out.push_str(&format!(
        "<div class='dim'>{} test-ci rows · {} bench rows</div>",
        u(&si["test_ci_count"]),
        u(&si["bench_count"]),
    ));
    out
}

/// Rust Diagnostics card (`/api/vision/rust-diagnostics`).
pub fn render_rust_diagnostics(d: &Value) -> String {
    if !b(&d["ok"]) {
        let msg = d["error"].as_str().unwrap_or("unavailable");
        return format!("<span class='err'>{}</span>", esc(msg));
    }
    if !b(&d["present"]) {
        return "<div class='dim'>no rust_diagnostics.json — run bin/record-rust-clippy.sh</div>"
            .to_string();
    }
    let rd = &d["rust_diagnostics"];
    let l = &rd["latest"];
    let warnings = u(&l["warnings"]);
    let errors = u(&l["errors"]);
    let wcls = if warnings > 0 { "warn" } else { "ok" };
    let ecls = if errors > 0 { "err" } else { "ok" };
    let okmark = if b(&l["ok"]) {
        " <span class='ok'>clean</span>".to_string()
    } else {
        " <span class='err'>fail</span>".to_string()
    };
    let mut out = format!(
        "<div class='dim'>{} · {} · git {}</div>",
        esc(&s(&rd["host_label"])),
        esc(&s(&rd["generated_at"])),
        esc(&s(&rd["git_head"])),
    );
    out.push_str(&format!(
        "<div>warnings <span class='{wcls}'>{warnings}</span> · errors <span class='{ecls}'>{errors}</span>{okmark} · <span class='dim'>{} history rows</span></div>",
        u(&rd["history_count"]),
    ));
    let codes = arr(&l["top_codes"]);
    if !codes.is_empty() {
        let top: Vec<String> = codes
            .iter()
            .take(5)
            .map(|c| format!("<kbd>{}</kbd>", esc(&s(c))))
            .collect();
        out.push_str(&format!("<div class='dim'>top: {}</div>", top.join(" ")));
    }
    out
}

/// Tests hooks card (`/api/hooks/tests`).
pub fn render_hooks_tests(d: &Value) -> String {
    let diag = &d["diagnostics"];
    let bins = arr(&d["test_bins"]);
    let mut out = format!(
        "<div class='dim'>status: <kbd>{}</kbd> · test bins: {}</div>",
        esc(&s(&d["status"])),
        bins.len(),
    );
    if !diag.is_null() {
        let errors = u(&diag["errors"]);
        let cls = if errors > 0 { "err" } else { "ok" };
        out.push_str(&format!(
            "<div>diagnostics: <span class='{cls}'>warnings {} · errors {errors}</span></div>",
            u(&diag["warnings"]),
        ));
    }
    let joined: Vec<String> = bins.iter().take(30).map(|b| esc(&s(b))).collect();
    out.push_str(&format!(
        "<div style='margin-top:6px' class='dim'>{}</div>",
        joined.join(" · ")
    ));
    out
}

/// Bench hooks card (`/api/hooks/bench`).
pub fn render_hooks_bench(d: &Value) -> String {
    let sp = &d["speed_index"];
    let dirs = arr(&d["criterion_dirs"]);
    let mut out = format!(
        "<div class='dim'>status: <kbd>{}</kbd> · criterion dirs: {}</div>",
        esc(&s(&d["status"])),
        dirs.len(),
    );
    if !sp.is_null() {
        let cls = if b(&sp["test_ci_ok"]) { "ok" } else { "err" };
        out.push_str(&format!(
            "<div>test-ci wall: <span class='{cls}'>{}s</span></div>",
            f(&sp["test_ci_wall_secs"]),
        ));
    }
    let joined: Vec<String> = dirs.iter().map(|dir| esc(&s(dir))).collect();
    out.push_str(&format!(
        "<div style='margin-top:6px' class='dim'>{}</div>",
        joined.join(" · ")
    ));
    out
}

/// Sprint Map card (`/api/vision/sprint-map`).
pub fn render_sprint_map(d: &Value) -> String {
    if !b(&d["ok"]) {
        let msg = d["error"].as_str().unwrap_or("sprint map unavailable");
        return format!("<span class='err'>{}</span>", esc(msg));
    }
    let mut out = format!(
        "<div class='dim'>rev <kbd>{}</kbd> · nodes {} · next <kbd>{}</kbd> · last closed <kbd>{}</kbd></div>",
        esc(&s(&d["revision"])),
        u(&d["nodes_count"]),
        esc(&s(&d["next_sprint"])),
        esc(&s(&d["last_sprint_closed"])),
    );
    let rows: Vec<Vec<String>> = arr(&d["modules"])
        .iter()
        .map(|m| {
            vec![
                format!("<kbd>{}</kbd>", esc(&s(&m["id"]))),
                esc(&s(&m["layer"])),
                format!("<span class='ok'>{}</span>", u(&m["targets"])),
            ]
        })
        .collect();
    out.push_str(&tab(&["module", "layer", "targets"], rows));
    let kinds: Vec<String> = arr(&d["kinds"])
        .iter()
        .map(|k| format!("<kbd>{}</kbd>×{}", esc(&s(&k["kind"])), u(&k["count"])))
        .collect();
    out.push_str(&format!(
        "<div class='dim' style='margin-top:6px'>kinds: {}</div>",
        kinds.join(" ")
    ));
    let links = arr(&d["links"]);
    out.push_str(&format!(
        "<details style='margin-top:6px'><summary>links ({})</summary>",
        links.len()
    ));
    let rows: Vec<Vec<String>> = links
        .iter()
        .map(|l| {
            vec![
                format!("<kbd>{}</kbd>", esc(&s(&l["kind"]))),
                esc(&s(&l["from"]["id"])),
                "→".to_string(),
                esc(&s(&l["to"]["id"])),
            ]
        })
        .collect();
    out.push_str(&tab(&["kind", "from", "→", "to"], rows));
    out.push_str("</details>");
    out
}

/// Sprint Board card (`/api/vision/sprint-board`).
pub fn render_sprint_board(d: &Value) -> String {
    if !b(&d["ok"]) {
        let msg = d["error"].as_str().unwrap_or("sprint board unavailable");
        return format!("<span class='err'>{}</span>", esc(msg));
    }
    let mut out = format!(
        "<div class='dim'>rev <kbd>{}</kbd> · next <span class='sprint-pill'>{}</span> · active <span class='sprint-pill'>{}</span></div>",
        esc(&s(&d["revision"])),
        esc(&s(&d["next_sprint"])),
        esc(&s(&d["active_sprint"])),
    );
    out.push_str(&format!(
        "<div>open <span class='warn'>{}</span> · closed <span class='ok'>{}</span> · total <span class='dim'>{}</span></div>",
        u(&d["open_count"]),
        u(&d["closed_count"]),
        u(&d["total"]),
    ));
    out.push_str(&bar(f(&d["progress_pct"])));
    for c in arr(&d["columns"]) {
        let entries = arr(&c["entries"]);
        out.push_str(&format!(
            "<details style='margin-top:6px'><summary>{} ({})</summary>",
            esc(&s(&c["name"])),
            u(&c["count"]),
        ));
        let rows: Vec<Vec<String>> = entries
            .iter()
            .map(|e| {
                let pclass = if s(&e["id"]) == s(&d["active_sprint"]) {
                    "open"
                } else if s(&e["status"]) == "closed" {
                    "closed"
                } else {
                    ""
                };
                vec![
                    format!("<span class='squeue {pclass}'>{}</span>", esc(&s(&e["id"]))),
                    format!("<span class='squeue-st'>{}</span>", esc(&s(&e["status"]))),
                    esc(&s(&e["title"])),
                ]
            })
            .collect();
        out.push_str(&tab(&["id", "status", "title"], rows));
        out.push_str("</details>");
    }
    out
}

/// OmniRouter card (`/api/omni`).
pub fn render_omni(d: &Value) -> String {
    let providers = arr(&d["providers"]);
    let models = arr(&d["models"]);
    let rec = arr(&d["recommended"]);
    let routing = &d["routing"];
    let default_provider = s(&routing["default_provider"]);
    let mut out = format!(
        "<div class='dim'>providers {} · models {} · default <kbd>{}</kbd></div>",
        providers.len(),
        models.len(),
        esc(&default_provider),
    );
    if default_provider.is_empty() {
        out.push_str("<span class='err'>omni unavailable</span>");
        return out;
    }
    let rec_ids: Vec<String> = rec
        .iter()
        .map(|m| format!("<kbd>{}</kbd>", esc(&s(&m["id"]))))
        .collect();
    out.push_str(&format!(
        "<div style='margin-top:6px'>recommended: {}</div>",
        if rec_ids.is_empty() {
            "—".to_string()
        } else {
            rec_ids.join(" ")
        }
    ));
    out.push_str(&format!(
        "<details style='margin-top:6px'><summary>Providers ({})</summary>",
        providers.len()
    ));
    let rows: Vec<Vec<String>> = providers
        .iter()
        .map(|p| {
            vec![
                format!("<kbd>{}</kbd>", esc(&s(&p["id"]))),
                esc(&s(&p["name"])),
                if b(&p["enabled"]) {
                    "<span class='ok'>on</span>".to_string()
                } else {
                    "<span class='err'>off</span>".to_string()
                },
                if b(&p["key_set"]) {
                    "<span class='ok'>key</span>".to_string()
                } else {
                    "<span class='dim'>no key</span>".to_string()
                },
                format!("<span class='dim'>{}</span>", esc(&s(&p["base_url"]))),
            ]
        })
        .collect();
    out.push_str(&tab(&["id", "name", "state", "key", "base_url"], rows));
    out.push_str("</details>");
    out.push_str(&format!(
        "<details style='margin-top:6px'><summary>Models ({})</summary>",
        models.len()
    ));
    let rows: Vec<Vec<String>> = models
        .iter()
        .map(|m| {
            let ctx = m["context_window"]
                .as_u64()
                .map(format_number)
                .unwrap_or_else(|| "varies".to_string());
            let out_max = m["max_output"]
                .as_u64()
                .map(format_number)
                .unwrap_or_else(|| "varies".to_string());
            vec![
                format!("<kbd>{}</kbd>", esc(&s(&m["id"]))),
                esc(&s(&m["provider"])),
                ctx,
                out_max,
                if b(&m["free"]) {
                    "free".to_string()
                } else {
                    String::new()
                },
                if b(&m["recommended"]) {
                    "<span class='ok'>★</span>".to_string()
                } else {
                    String::new()
                },
            ]
        })
        .collect();
    out.push_str(&tab(&["id", "provider", "ctx", "out", "", ""], rows));
    out.push_str("</details>");
    out
}

fn format_number(n: u64) -> String {
    let mut s = n.to_string();
    let mut i = s.len();
    while i > 3 {
        i -= 3;
        s.insert(i, ',');
    }
    s
}

/// Render a named card's body HTML, or `None` for an unknown card name.
pub fn render_card(name: &str, d: &Value) -> Option<String> {
    match name {
        "tracker" => Some(render_tracker(d)),
        "sli" => Some(render_sli(d)),
        "toolchain" => Some(render_toolchain(d)),
        "ratio" => Some(render_ratio(d)),
        "hooks-tests" => Some(render_hooks_tests(d)),
        "hooks-bench" => Some(render_hooks_bench(d)),
        "sprint-map" => Some(render_sprint_map(d)),
        "sprint-queue" => Some(render_sprint_queue(d)),
        "sprint-progress" => Some(render_sprint_progress(d)),
        "sprint-board" => Some(render_sprint_board(d)),
        "speed-index" => Some(render_speed_index(d)),
        "rust-diagnostics" => Some(render_rust_diagnostics(d)),
        "omni" => Some(render_omni(d)),
        "galaxy-backdrop" => Some(render_galaxy_backdrop(d)),
        "starfield" => Some(render_starfield(d)),
        "rss-ticker" => Some(render_rss_ticker(d)),
        "gpu-mode" => Some(render_gpu_mode(d)),
        "power-menu" => Some(render_power_menu(d)),
        "panel-dock" => Some(render_panel_dock(d)),
        "fullscreen" => Some(render_fullscreen(d)),
        _ => None,
    }
}

/// Server-rendered card names (stable contract for `/api/ui/card/:name`).
pub const CARD_NAMES: [&str; 20] = [
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

/// Galaxy backdrop card body (SVG-backed visual).
fn render_galaxy_backdrop(d: &Value) -> String {
    let mode = d.get("mode").and_then(Value::as_str).unwrap_or("dark");
    let stars = d.get("stars").and_then(Value::as_u64).unwrap_or(0);
    format!(
        "<div class='dim'>galaxy backdrop · {mode} mode · {stars} stars</div><div>generated <kbd>{mode}</kbd> svg</div>"
    )
}

/// Starfield card body (eco/fx/ms star counts).
fn render_starfield(d: &Value) -> String {
    let eco = d.get("eco").and_then(Value::as_u64).unwrap_or(0);
    let fx = d.get("fx").and_then(Value::as_u64).unwrap_or(0);
    let ms = d.get("ms").and_then(Value::as_u64).unwrap_or(0);
    format!(
        "<div>starfield · Eco <span class='ok'>{eco}</span> · FX <span class='ok'>{fx}</span> · Ms <span class='ok'>{ms}</span> stars</div>"
    )
}

/// RSS ticker card body (latest vision/status feed lines).
fn render_rss_ticker(d: &Value) -> String {
    let items = d
        .get("items")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut out = String::from("<div class='dim'>vision feed · rss ticker</div>");
    let rows: Vec<Vec<String>> = items
        .iter()
        .map(|it| {
            let label = it.get("label").and_then(Value::as_str).unwrap_or("");
            let status = it.get("status").and_then(Value::as_str).unwrap_or("");
            vec![
                format!("<kbd>{}</kbd>", esc(label)),
                format!("<span class='ok'>{}</span>", esc(status)),
            ]
        })
        .collect();
    out.push_str(&tab(&["item", "status"], rows));
    out
}

/// GPU mode card body (current accelerator mode).
fn render_gpu_mode(d: &Value) -> String {
    let mode = d.get("mode").and_then(Value::as_str).unwrap_or("unknown");
    let active = d.get("active").and_then(Value::as_bool).unwrap_or(false);
    format!(
        "<div>gpu mode <span class='ok'>{}</span> {}</div>",
        esc(mode),
        if active { "· active" } else { "· idle" }
    )
}

/// Power menu card body (power level).
fn render_power_menu(d: &Value) -> String {
    let level = d.get("level").and_then(Value::as_str).unwrap_or("eco");
    let watts = d.get("watts").and_then(Value::as_u64).unwrap_or(0);
    format!(
        "<div>power <span class='ok'>{}</span> · {} W</div>",
        esc(level),
        watts
    )
}

/// Panel dock card body (docked panel list).
fn render_panel_dock(d: &Value) -> String {
    let panels = d
        .get("panels")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let names: Vec<String> = panels
        .iter()
        .filter_map(|p| p.as_str())
        .map(|p| format!("<kbd>{}</kbd>", esc(p)))
        .collect();
    let joined = if names.is_empty() {
        "<span class='dim'>—</span>".to_string()
    } else {
        names.join(" ")
    };
    format!("<div>panel dock · {joined}</div>")
}

/// Fullscreen card body (fullscreen toggle state).
fn render_fullscreen(d: &Value) -> String {
    let active = d.get("active").and_then(Value::as_bool).unwrap_or(false);
    let label = d.get("label").and_then(Value::as_str).unwrap_or("fullscreen");
    format!(
        "<div>{} <span class='{}'>{}</span></div>",
        esc(label),
        if active { "ok" } else { "dim" },
        if active { "on" } else { "off" }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn esc_matches_js_helper() {
        assert_eq!(esc("<a & b>"), "&lt;a &amp; b&gt;");
        assert_eq!(esc("plain"), "plain");
        assert_eq!(esc(""), "");
        assert_eq!(esc("\"q\""), "\"q\"");
    }

    #[test]
    fn tab_renders_empty_fallback_row() {
        let html = tab(&["a", "b"], Vec::new());
        assert!(html.contains("<table><tr><th>a</th><th>b</th></tr>"));
        assert!(html.contains("<td><span class='dim'>—</span></td>"));
    }

    #[test]
    fn tab_renders_rows() {
        let html = tab(
            &["k", "v"],
            vec![vec!["<kbd>x</kbd>".to_string(), "y".to_string()]],
        );
        assert_eq!(
            html,
            "<table><tr><th>k</th><th>v</th></tr><tr><td><kbd>x</kbd></td><td>y</td></tr></table>"
        );
    }

    #[test]
    fn bar_clamps_and_formats() {
        let html = bar(120.0);
        assert!(html.contains("width:100%"));
        assert!(html.contains("100% closed"));
        let low = bar(-5.0);
        assert!(low.contains("width:0%"));
    }

    #[test]
    fn render_tracker_uses_escaping_and_counts() {
        let d = serde_json::json!({
            "sprints": { "open": ["A"], "closed": [], "total": 1, "next": "<next>" },
            "records": [{ "kind": "k", "label": "<l>", "status": "open", "at": "t" }]
        });
        let html = render_tracker(&d);
        assert!(html.contains("open 1 · closed 0 · total 1"));
        assert!(html.contains("&lt;next&gt;"));
        assert!(html.contains("&lt;l&gt;"));
    }

    #[test]
    fn render_ratio_error_when_not_ok() {
        let d = serde_json::json!({ "ok": false, "error": "missing rust_ratio.json" });
        let html = render_ratio(&d);
        assert!(html.contains("missing rust_ratio.json"));
        assert!(html.contains("err"));
    }

    #[test]
    fn render_sprint_queue_marks_active_next_closed() {
        let d = serde_json::json!({
            "ok": true, "revision": "472", "next_sprint": "PH-S1839",
            "last_sprint_closed": "PH-S1838", "active_sprint": "PH-S1839", "open_count": 1,
            "planned": [
                { "id": "PH-S1839", "status": "open", "category": "GSV canon", "title": "a" },
                { "id": "PH-S1848", "status": "planned", "category": "GSV_ROLES", "title": "b" }
            ]
        });
        let html = render_sprint_queue(&d);
        assert!(html.contains("class='squeue open'>PH-S1839"));
        assert!(html.contains("class='squeue closed'>PH-S1848"));
        assert!(html.contains("planned (2)"));
    }

    #[test]
    fn render_hooks_tests_marks_diagnostics_and_bins() {
        let d = serde_json::json!({
            "status": "ready", "test_bins": ["poolai-abc", "test_xyz"],
            "diagnostics": { "warnings": 3, "errors": 1, "ok": false, "recorded_at": "t" }
        });
        let html = render_hooks_tests(&d);
        assert!(html.contains("test bins: 2"));
        assert!(html.contains("class='err'>warnings 3 · errors 1"));
        assert!(html.contains("poolai-abc"));
        let nodiag =
            serde_json::json!({ "status": "no-artifacts", "test_bins": [], "diagnostics": null });
        let html2 = render_hooks_tests(&nodiag);
        assert!(!html2.contains("diagnostics"));
        assert!(html2.contains("no-artifacts"));
    }

    #[test]
    fn render_hooks_bench_marks_speed_index() {
        let d = serde_json::json!({
            "status": "ready", "criterion_dirs": ["hash1", "hash2"],
            "speed_index": { "test_ci_wall_secs": 12.5, "test_ci_ok": true, "recorded_at": "t" }
        });
        let html = render_hooks_bench(&d);
        assert!(html.contains("criterion dirs: 2"));
        assert!(html.contains("class='ok'>12.5s</span>"));
        assert!(html.contains("hash1"));
    }

    #[test]
    fn render_sprint_map_renders_modules_kinds_links() {
        let d = serde_json::json!({
            "ok": true, "revision": "472", "nodes_count": 3, "next_sprint": "PH-S1839",
            "last_sprint_closed": "PH-S1838",
            "modules": [{ "id": "GSV", "layer": "core", "targets": 12 }],
            "kinds": [{ "kind": "depends_on", "count": 2 }],
            "links": [{ "kind": "depends_on", "from": { "id": "a" }, "to": { "id": "<b>" } }]
        });
        let html = render_sprint_map(&d);
        assert!(html.contains("nodes 3"));
        assert!(html.contains("&lt;b&gt;"));
        assert!(html.contains("<span class='ok'>12</span>"));
        assert!(html.contains("links (1)"));
        let bad = serde_json::json!({ "ok": false, "error": "boom" });
        assert!(render_sprint_map(&bad).contains("boom"));
    }

    #[test]
    fn render_sprint_board_renders_columns_and_bar() {
        let d = serde_json::json!({
            "ok": true, "revision": "472", "next_sprint": "PH-S1839", "active_sprint": "PH-S1839",
            "open_count": 1, "closed_count": 2, "total": 3, "progress_pct": 66.0,
            "columns": [
                { "name": "done", "count": 2, "entries": [
                    { "id": "PH-S1838", "status": "closed", "title": "x" },
                    { "id": "PH-S1839", "status": "closed", "title": "y" }
                ] }
            ]
        });
        let html = render_sprint_board(&d);
        assert!(html.contains("class='squeue open'>PH-S1839"));
        assert!(html.contains("class='squeue closed'>PH-S1838"));
        assert!(html.contains("66% closed"));
        assert!(html.contains("done (2)"));
        let bad = serde_json::json!({ "ok": false, "error": "nope" });
        assert!(render_sprint_board(&bad).contains("nope"));
    }

    #[test]
    fn render_card_dispatch_known_and_unknown() {
        let d = serde_json::json!({ "ok": true, "revision": "472", "open_count": 0, "closed_count": 0, "planned_count": 0, "total": 0, "progress_pct": 0.0, "layers": [] });
        assert!(render_card("sprint-progress", &d).is_some());
        assert!(render_card("hooks-tests", &d).is_some());
        assert!(render_card("omni", &d).is_some());
        assert!(render_card("galaxy-backdrop", &d).is_some());
        assert!(render_card("starfield", &d).is_some());
        assert!(render_card("rss-ticker", &d).is_some());
        assert!(render_card("gpu-mode", &d).is_some());
        assert!(render_card("power-menu", &d).is_some());
        assert!(render_card("panel-dock", &d).is_some());
        assert!(render_card("fullscreen", &d).is_some());
        assert!(render_card("nope", &d).is_none());
        assert_eq!(CARD_NAMES.len(), 20);
    }

    #[test]
    fn render_omni_renders_summary_recommended_providers_models() {
        let d = serde_json::json!({
            "providers": [
                { "id": "openai", "name": "OpenAI", "enabled": true, "key_set": true, "base_url": "https://api.openai.com/v1" },
                { "id": "free-h", "name": "Free Host", "enabled": false, "key_set": false, "base_url": "" }
            ],
            "models": [
                { "id": "gpt-5.2", "provider": "openai", "context_window": 400000, "max_output": 128000, "free": false, "recommended": true }
            ],
            "recommended": [
                { "id": "gpt-5.2", "provider": "openai" }
            ],
            "routing": { "default_provider": "openai", "auto": true }
        });
        let html = render_omni(&d);
        assert!(html.contains("providers 2 · models 1"));
        assert!(html.contains("default <kbd>openai</kbd>"));
        assert!(html.contains("recommended: <kbd>gpt-5.2</kbd>"));
        assert!(html.contains("Providers (2)"));
        assert!(html.contains("Models (1)"));
        assert!(html.contains("400,000"));
        assert!(html.contains("128,000"));
        assert!(html.contains("<span class='ok'>★</span>"));
        assert!(html.contains("<span class='err'>off</span>"));
        assert!(html.contains("no key"));
    }

    #[test]
    fn render_omni_empty_state_marks_unavailable() {
        let d = serde_json::json!({
            "providers": [], "models": [], "recommended": [],
            "routing": { "default_provider": "", "auto": false }
        });
        let html = render_omni(&d);
        assert!(html.contains("providers 0 · models 0"));
        assert!(html.contains("omni unavailable"));
    }
}
