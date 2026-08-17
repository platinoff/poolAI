# Промпт наступної сесії (GSV)

**Оновлено:** 2026-08-16 (band 126 **PH-S1899…S1908** ✅ · ratio **96.87%** · tests **230**)

```
абракадабра
```

**Порядок:** **S0 диск** (`df -h /s` + `check_target_disk.sh` → `cargo clean` за потреби) →
project scan (**warnings first** — `cargo clippy --all-targets` у GSV, `poolai-rust-diagnostics` у poolAI) →
drain наступного band (черга — FM §5.12 §5.106 / GSV_TECH_ROADMAP; **без** mid-push) →
Speeds · Rust panel → vision-sync (`poolai-vision-sync`) → **один** commit → **`git push` + самарі`.

**⚠️ Зупинити `gsv-server` перед `cargo test`/`build`** (блокує `target/debug/gsv-server.exe`);
після тестів перезапустити на порт 9999.

## Band стан

- **band 102** (PH-S1659…S1668) ✅ — GSV migration (bin, бокси, docs).
- **band 108** (PH-S1719…S1728) ✅ — roles/ratio canon: `GSV/docs/GSV_ROLES.md`; `gsv-loc-audit`
  (95.52% gate ✅); `tests/gsv_ratio_contracts.rs` (7); Ratio box + `GET /api/ratio` + UI card;
  `GSV/docs/{MEMORY,HANDOFF,NEXT,README}`; FM §5.12 §5.89; poolAI docs parity + HANDOFF/NEXT; vision-sync rev 458.
- **band 109** (PH-S1729…S1738) ✅ — Vision box: `GSV/src/boxes/vision.rs` (manifest/feed serde +
  read/save/load/wire/sync/drift); `gsv-vision-sync` bin (`--check`); `GET /api/vision*`; Vision UI card;
  `tests/gsv_vision_contracts.rs` (7); `GSV/docs/VISION.md` + `GSV_MIGRATION.md` rows ✅; poolAI vision
  README parity; FM §5.12 §5.90; GSV tests **101** (95.45% gate ✅); vision-sync rev 459.
- **band 110** (PH-S1739…S1748) ✅ — Vision map UI: `map_report`/`wire_map` → `GET /api/vision/map`
  (layers L0..L5 z-sorted + edge kinds); `GSV/ui/vision.svg` порт + `GET /assets/vision.svg` (audit
  Ignored, ratio-neutral); Vision Map card; `?status=` feed filter; contracts **106**; GSV ratio
  **96.01%** gate ✅; poolAI vision rev **461**; FM §5.12 §5.91.
- **band 111** (PH-S1749…S1758) ✅ — Sprint map + doc-preview: `sprint_map_report`/`wire_sprint_map`
  → `GET /api/vision/sprint-map` (sprint-scope/queue/session-tracks links + modules + kinds);
  `doc_preview`/`wire_doc_preview` → `GET /api/vision/doc-preview?id=` (node + 1-hop neighbors);
  Sprint Map + Doc Preview UI cards; contracts **113** (14 vision); GSV ratio **95.77%** gate ✅;
  poolAI vision rev **462**; FM §5.12 §5.92.
- **band 112** (PH-S1759…S1768) ✅ — Vision auto-sync + sprint-queue planning:
  `Extensions` mirror (read/save/load + `gsv_extensions.json` snapshot + `wire_extensions`
  → `GET /api/vision/extensions`); `wire_sync` → `GET /api/vision/sync` (re-mirror + drift gate);
  `sprint_queue_report`/`wire_sprint_queue` → `GET /api/vision/sprint-queue` (entries ∪ active plan);
  Vision Sync + Sprint Queue UI cards; contracts **118** (19 vision); GSV ratio **95.56%** gate ✅;
  poolAI vision rev **463**; FM §5.12 §5.93.
- **band 113** (PH-S1769…S1778) ✅ — Galaxy UI: node search + interactive map:
  `node_search`/`wire_node_search` → `GET /api/vision/node-search?q=&layer=` (case-insensitive
  id/label/path/sections, top-N 25 layer-z-sorted, links_out/in tallies); Vision Map card inline
  `assets/vision.svg` + layer filter chips + search → doc-preview deep-link; contracts **122**
  (22 vision + 19 server); poolAI vision rev **465**; FM §5.12 §5.94.
- **band 114** (PH-S1779…S1788) ✅ — GSV Sprint-board + progress UI:
  `sprint_board_report`/`wire_sprint_board` → `GET /api/vision/sprint-board` (open/closed/planned
  columns + `progress_pct` = closed/total); `sprint_progress_report`/`wire_sprint_progress`
  → `GET /api/vision/sprint-progress` (status counts + per-layer `node_count`/`linked_count`
  distribution, z-ascending); Sprint Board + Sprint Progress UI cards; contracts **140**
  (38 vision + 21 server); GSV ratio **95.02%** gate ✅; poolAI vision rev **467**; FM §5.12 §5.95.
- **band 115** (PH-S1789…S1798) ✅ — GSV migration completion (legacy vision supersession):
  `GET /api/vision/speeds` (`SpeedIndexReport` — latest test-CI + bench + history counts,
  mirror `gsv_speed_index.json`, empty-tolerant) та `GET /api/vision/rust-diagnostics`
  (`RustDiagnosticsReport` — latest warnings/errors/top_codes + history count, mirror
  `gsv_rust_diagnostics.json`, empty-tolerant); Speed Index + Rust Diagnostics UI cards;
  `GSV/docs/LEGACY_PARITY.md` audit (Speeds + Rust — єдині прогалини, закрито;
  `vision.js`/`vision.css` superseded); contracts **150** (40 vision + 23 server);
  GSV ratio **95.04%** gate ✅; poolAI vision rev **468**; FM §5.12 §5.96.
- **band 116** (PH-S1799…S1808) ✅ — GSV history charts (speed/rust analytics):
  typed `SpeedTestCiRecord`/`SpeedBenchRecord`/`RustDiagRecord` + `test_ci_history`/
  `bench_history`/`history` через wire (`read_speed_index`/`read_rust_diagnostics`,
  source fallback unchanged); `speed_index_chart_svg` → `GET /api/vision/speeds.svg`
  (Rust-rendered SVG: test-CI wall bars green ok / red fail, ≤24 runs, footer latest bench) та
  `rust_diagnostics_chart_svg` → `GET /api/vision/rust-diagnostics.svg` (warnings orange +
  errors red grouped bars, command footer); `<img>` charts у Speed Index + Rust Diagnostics
  cards; vision tests **153** (67 unit + 40 vision + 23 server); GSV ratio **95.26%** gate ✅;
  stand smoke: обидва SVG → 200 `image/svg+xml`; `poolai-ui-wasm` defer; poolAI vision rev
  **469**; FM §5.12 §5.97.
- **band 117** (PH-S1809…S1818) ✅ — GSV legacy vision deactivation:
  `GSV/docs/vision/index.html` → GSV pointer page (no `vision.js`/`vision.css` refs);
  `vision.js`/`vision.css` DEACTIVATED banner (band 117, архів — не видаляємо);
  `GSV/docs/vision/README.md` deactivation note; live link retarget: poolai-vision-sync feed +
  GSV `vision.rs` sample links → `http://127.0.0.1:8891/#b-sprint-board`;
  RUN_LOCAL/GSV_SERVER/docs-gsv README/SPEED_INDEX/RUST_DIAGNOSTICS → GSV;
  legacy test retirement (`poolai_vision_sync.rs` unit ×4, `galaxy_horizon_s1011/s1019/s1039`,
  e2e `vision.spec.ts`/`a11y.spec.ts` → deactivated pointer state; `VISION_MAP_BAND40_ROWS`);
  `LEGACY_PARITY.md`/`GSV_MIGRATION.md` band 117; GSV ratio **95.26%** gate ✅;
  poolAI vision rev **470**; FM §5.12 §5.98.
- **band 118** (PH-S1819…S1828) ✅ — GSV sprint UI migration (theme + focus map):
  `SprintThemeReport`/`sprint_theme_report`/`wire_sprint_theme` → `GET /api/vision/sprint-theme`
  (sprint `#a78bfa`/next `#c4b5fd`, pill/chip/queue colors, layer L0–L5 + edge-kind palettes);
  `sprint_token_matches`/`path_matches_glob`/`nodes_for_sprint`/`sprint_focus_svg`
  → `GET /api/vision/sprint-focus.svg?sprint=` (sprint-dim: in-scope accent, out-of-scope
  opacity 0.22/text 0.28, edges tinted; default active sprint; empty-state);
  Sprint Focus card + `--sprint*` CSS-змінні + sprint-pill/queue chips у Sprint Queue/Board
  cards; contracts **163** (44 vision + 25 server); GSV ratio **95.35%** gate ✅;
  poolAI vision rev **471**; FM §5.12 §5.99.
- **band 119** (PH-S1829…S1838) ✅ — GSV Galaxy UI full parity (colors + box behaviors):
  `GalaxyPalette` struct + `wire_palette` → `GET /api/vision/palette` (повний legacy `:root`
  palette: bg-deep/bg/panel/panel-solid/border/border-bright/text/muted/accent/accent-2/glow/
  sidebar-w, layers+layers_dim L0–L5, edge-docs/code/toml, ext-md/rs/json/toml, sprint,
  bg-tone, galaxy-bg-opacity + `ok`/`revision`); `starfield_svg(mode)` (deterministic LCG:
  eco sparse/static, fx dense+glow, ms medium) → `GET /api/vision/starfield.svg?mode=eco|fx|ms`;
  `galaxy_svg()` (radial nebula + spiral arms) → `GET /api/vision/galaxy.svg`; header chrome
  (RSS ticker `loadRssTicker`, GPU mode button Eco/FX/Ms cycle → `body.vision-(eco|fx|ms)`,
  power menu soft sync/reload/force offline); panel dock + Esc-fullscreen (`syncDock`,
  `.card.fullscreen`); contracts **183** (50 vision + 29 server); GSV ratio **95.18%** gate ✅;
  poolAI vision rev **472**; FM §5.12 §5.100.
- **band 120** (PH-S1839…S1848) ✅ — GSV Ratio 96% stretch:
  `gsv-loc-audit --stretch-96` advisory (`ratio.rs` `STRETCH_96_TARGET`/`stretch_target`/
  `meets_stretch_96`, `#[serde(default)]` compat) → **96.51%** (rust 10027 / product 10390) ✅;
  `boxes/ui.rs` (`esc`/`tab`/`bar` + 12 Rust card renderers + `render_card` + `CARD_NAMES` 12) →
  `GET /api/ui/card/{name}`; `ui/index.html` thin glue (`getText` → `rustCards` 12, 8 JS renderers
  видалено); contracts **204** (95 unit + 30 server + 6 ui); FM §5.12 §5.101.
- **band 121** (PH-S1849…S1855) ✅ — GSV OmniRouter box parity:
  `boxes/ui.rs` `render_omni` (summary/routing + recommended + providers table + models table +
  `format_number` grouping) + `render_card`/`CARD_NAMES` 13 + 2 unit tests →
  `GET /api/ui/card/omni` (`server/mod.rs` `"omni"` → `boxes::omni::wire`);
  `ui/index.html` `renderOmni` JS видалено, `rustCards` 13, `resync()` url drop;
  contracts **207** (97 unit + 30 server + 7 ui); ratio **96.73%** (rust 10191 / product 10536) ✅;
  FM §5.12 §5.102.
- **band 125** (PH-S1889…S1898) ✅ — GSV Vision/UI polish (a11y/error/offline/stand contracts):
  `boxes/ui.rs` 13 renderers error/empty-state HTML маркери (`err_html`/`empty_html`/`not_ok`,
  `<span class='err'>` + «— no data», no panic) + `gsv_ui_contracts` stand contracts for all 13
  (`RUST_CARDS` + error/empty markers) + a11y (`role=status`/`aria-live`/`aria-label`/`alt`/
  `aria-haspopup`) + offline-stable (`data-card` hooks, `getText` keep-last-good, `.card-status`
  badge на fetch fail); `server/mod.rs` canonical JSON error shape `{ok:false,error}` (`err_json`
  → preview/ui-card/ui-path/data-file/error-response/omni-test/spawn_cargo) +
  `gsv_server_contracts` canonical-shape contracts; `boxes/vision.rs` `wire_summary`
  empty-tolerant (`degraded` flag, error тільки при fallback) + consistent `ok`/`error` across
  `/api/vision*` (`gsv_vision_contracts` wire-shape contracts);
  contracts **221** (102 unit + 8 omni + 7 ratio + 32 server + 12 ui + 8 update + 52 vision);
  ratio **96.87%** (rust 11176 / product 11537) ✅; vision rev **492**; FM §5.12 §5.106.
- **band 126** (PH-S1899…S1908) ✅ — GSV stand smoke + ops canon:
  `gsv-http-stand-smoke` bin (`src/bin/gsv_http_stand_smoke.rs`, мірор poolAI
  `poolai-http-stand-smoke`): CLI `--base-url`/`--json`, `SmokeCaseResult`/`SmokeReport`,
  `check_ok`/`check_json`/`check_status`/`check_card`, `CARDS` 20, 48 live checks
  (core boxes + vision* ok-gate + SVG status + 20 ui cards non-empty html), exit 1 при FAIL,
  3 bin unit tests; `Cargo.toml` `[[bin]]`; `tests/gsv_stand_smoke_contracts.rs` (6):
  vision ok-gate (15) + struct-wire JSON (5) + status-only 200 (5) + cards render ok+html +
  report shape + card-list registry parity; docs canon (GSV_SERVER stand-smoke section,
  GSV_BOXES row, README 230 tests / structure / endpoints / status, roadmap band 126);
  contracts **230** (102 unit + 3 bin + 8 omni + 7 ratio + 32 server + 6 stand-smoke +
  12 ui + 8 update + 52 vision); ratio **96.87%** (rust 11176 / product 11537) ✅;
  vision rev **493**; FM §5.12 §5.107.
  **Наступний band 127**: master backlog (за пріоритетом власника) — FM §5.12 / GSV_TECH_ROADMAP.

## Канон GSV

- Rust **95–100%** / wasm 0–5%, без Python/Java; bins — лише `src/bin/`. Ratio: `cargo run --bin gsv-loc-audit`.
- Ролі/сесія: [`GSV/docs/GSV_ROLES.md`](GSV_ROLES.md) · пам'ять: [`GSV/docs/MEMORY.md`](MEMORY.md).
- Архітектура: [`GSV/docs/gsv/`](gsv/README.md) · TechPreroadMap: [`GSV_TECH_ROADMAP.md`](gsv/GSV_TECH_ROADMAP.md).
- OmniRouter dry-run у тестах — `X-Omni-Dry-Run: 1` (жодного реального запиту).

## Не повторювати

Band 107 ✅ (poolAI Ratio96 docs canon) · band 106 ✅ (Ratio96 loc-audit) · band 105 ✅ (Ratio96 stand smoke) ·
band 104 ✅ (Ratio96 admin/ops) · band 103 ✅ · band 102 ✅ (GSV migration) · band 109 ✅ (GSV vision sync) ·
band 110 ✅ (GSV vision map UI) · band 111 ✅ (GSV sprint-map + doc-preview) · band 112 ✅ (GSV vision auto-sync + sprint-queue) ·
band 113 ✅ (GSV node search + interactive map) · band 114 ✅ (GSV sprint-board + progress UI) ·
band 115 ✅ (GSV migration completion — legacy vision supersession) · band 116 ✅ (GSV history charts — speed/rust analytics) ·
band 117 ✅ (GSV legacy vision deactivation) · band 118 ✅ (GSV sprint UI migration — theme + focus map) ·
band 119 ✅ (GSV Galaxy UI full parity — colors + box behaviors) · band 120 ✅ (GSV Ratio 96% stretch) ·
band 121 ✅ (GSV OmniRouter box parity) · band 125 ✅ (GSV Vision/UI polish — a11y/error/offline/stand contracts) ·
band 126 ✅ (GSV stand smoke + ops canon) ·
staging `GSV/data/*` / `certs/*.pem` /
`.env` · mid-push · build/test при запущеному `gsv-server` · обхід ratio-смуги Rust-кодом замість compact UI ·
перенесення legacy `vision.js`/`vision.css` у `GSV/ui/` (знищило б ratio canon).
