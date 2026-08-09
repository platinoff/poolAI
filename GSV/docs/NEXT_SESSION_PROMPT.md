# Промпт наступної сесії (GSV)

**Оновлено:** 2026-08-08 (band 119 **PH-S1829…S1838** ✅ · ratio **95.18%** · tests **183**)

```
абракадабра
```

**Порядок:** **S0 диск** (`df -h /s` + `check_target_disk.sh` → `cargo clean` за потреби) →
project scan (**warnings first** — `cargo clippy --all-targets` у GSV, `poolai-rust-diagnostics` у poolAI) →
drain наступного band (черга — FM §5.12 §5.100 / GSV_TECH_ROADMAP; **без** mid-push) →
Speeds · Rust panel → vision-sync (`poolai-vision-sync`) → **один** commit → **`git push` + самарі**.

**⚠️ Зупинити `gsv-server` перед `cargo test`/`build`** (блокує `target/debug/gsv-server.exe`);
після тестів перезапустити на порт 8891.

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
  `docs/vision/index.html` → GSV pointer page (no `vision.js`/`vision.css` refs);
  `vision.js`/`vision.css` DEACTIVATED banner (band 117, архів — не видаляємо);
  `docs/vision/README.md` deactivation note; live link retarget: poolai-vision-sync feed +
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
  **Наступний band 120**: master backlog (за пріоритетом власника) — FM §5.12 / GSV_TECH_ROADMAP.

## Канон GSV

- Rust **95–100%** / wasm 0–5%, без Python/Java; bins — лише `src/bin/`. Ratio: `cargo run --bin gsv-loc-audit`.
- Ролі/сесія: [`GSV/docs/GSV_ROLES.md`](GSV_ROLES.md) · пам'ять: [`GSV/docs/MEMORY.md`](MEMORY.md).
- Архітектура: [`docs/gsv/`](../../docs/gsv/README.md) · TechPreroadMap: [`GSV_TECH_ROADMAP.md`](../../docs/gsv/GSV_TECH_ROADMAP.md).
- OmniRouter dry-run у тестах — `X-Omni-Dry-Run: 1` (жодного реального запиту).

## Не повторювати

Band 107 ✅ (poolAI Ratio96 docs canon) · band 106 ✅ (Ratio96 loc-audit) · band 105 ✅ (Ratio96 stand smoke) ·
band 104 ✅ (Ratio96 admin/ops) · band 103 ✅ · band 102 ✅ (GSV migration) · band 109 ✅ (GSV vision sync) ·
band 110 ✅ (GSV vision map UI) · band 111 ✅ (GSV sprint-map + doc-preview) · band 112 ✅ (GSV vision auto-sync + sprint-queue) ·
band 113 ✅ (GSV node search + interactive map) · band 114 ✅ (GSV sprint-board + progress UI) ·
band 115 ✅ (GSV migration completion — legacy vision supersession) · band 116 ✅ (GSV history charts — speed/rust analytics) ·
band 117 ✅ (GSV legacy vision deactivation) · band 118 ✅ (GSV sprint UI migration — theme + focus map) ·
band 119 ✅ (GSV Galaxy UI full parity — colors + box behaviors) ·
staging `GSV/data/*` / `certs/*.pem` /
`.env` · mid-push · build/test при запущеному `gsv-server` · обхід ratio-смуги Rust-кодом замість compact UI ·
перенесення legacy `vision.js`/`vision.css` у `GSV/ui/` (знищило б ratio canon).
