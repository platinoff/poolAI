# GSV — Memory mark (what/why)

Стан проєкту **Galaxy StarWalker Vision** — окремого Rust-first проєкту в `GSV/` репо PoolAI.
Оновлюється в кінці кожного band. Лічильники — вимірювані (`wc -l`, `cargo test`,
`cargo run --bin gsv-loc-audit`), не з пам'яті.

## Стан (2026-08-10 · band 121 ✅)

- **Канон:** Rust **95–100%** / wasm **0–5%** (завжди), без Python/Java; bins — лише `src/bin/`.
- **Ratio (виміряно):** `cargo run --bin gsv-loc-audit -- --stretch-96` → **96.73%** (rust 10191 / product 10536) —
  gate ≥95% ✅, stretch-96 ≥96% ✅.
  Звіт: `GSV/data/rust_ratio.json` (gitignored).
- **Тести (виміряно):** `cargo test` → **207** (97 unit + 8 `gsv_omni_contracts` + 7 `gsv_ratio_contracts`
  + 30 `gsv_server_contracts` + 7 `gsv_ui_contracts` + 8 `gsv_update_flow` + 50 `gsv_vision_contracts`).
  `cargo clippy --all-targets` → **0** warnings. `cargo fmt` clean.
- **Бокси:** Tracker · SLI console · Toolchain · IDE · Update/offline · Box preview · SLI terminal ·
  Tests/bench hooks · **Ratio** · **Vision** · **Vision Map** · **Sprint Map** · **Doc Preview** ·
  **Vision Sync** · **Sprint Queue** · **Sprint Board** · **Sprint Progress** · **Sprint Focus** ·
  **Galaxy UI parity** (band 119) · **UI fragments** (band 121: `GET /api/ui/card/:name`, 13 Rust renderers) ·
  **OmniRouter** (Rust AI-проксі/роутер; card `omni` band 121).

## Що зроблено

### Band 102 (PH-S1659…S1668, ✅ 2026-08-01) — GSV migration
- `GSV/docs/gsv/` канон + `GSV/Cargo.toml` (окремий workspace, `.cargo/config.toml` → `target-dir`).
- `gsv-server` bin (axum + tokio, SSE `/events`, single-page UI `ui/index.html` embedded).
- Бокси: Tracker, SLI console, Toolchain, IDE, Update/offline, Box preview, SLI terminal, Tests/bench hooks.
- 52 tests green (на той момент), clippy 0. FM §5.12 §5.83 ✅.

### Band 108 (PH-S1719…S1728, ✅ 2026-08-05) — roles/ratio/roles canon (poolAI дисципліна)
- **PH-S1719** `GSV/docs/GSV_ROLES.md` — ролі VDT (Власник/Оркестратор/Субагенти) + канон сесії
  (S0 disk-first → project scan warnings-first → drain ≤10 PH-S* → Speeds + Rust panel → vision-sync
  → один commit → `git push` + самарі).
- **PH-S1720** `GSV/src/bin/gsv_loc_audit.rs` + `GSV/src/boxes/ratio.rs` — LOC ratio audit
  (дзеркало poolAI `poolai_loc_audit.rs`): `git ls-files --full-name`, `classify_product_path`,
  `--print/--no-write/--advisory/--min-ratio/--output/--data-dir`, gate ≥95%.
- **PH-S1721** `tests/gsv_ratio_contracts.rs` — 7 integration contracts (audit/save/load/wire/API).
- **PH-S1722** Ratio box + `GET /api/ratio` + UI Ratio card.
- **PH-S1723** `GSV/docs/MEMORY.md` (цей файл) + `GSV/docs/README.md` індекс.
- **PH-S1724** `GSV/docs/HANDOFF_NEW_SESSION.md` + `NEXT_SESSION_PROMPT.md`.
- **PH-S1725** FM §5.12 §5.89 band 108 + `GSV/docs/gsv/GSV_TECH_ROADMAP.md` band 108.
- **PH-S1726** poolAI docs parity (FUNCTIONALITY_DIGEST / vision README / GSV rows).
- **PH-S1727** poolAI `docs/development/HANDOFF_NEW_SESSION.md` + `NEXT_SESSION_PROMPT.md`.
- **PH-S1728** Band close: ratio hold, fmt, clippy, cargo test, docs canon, vision-sync, push.

### Band 109 (PH-S1729…S1738, ✅ 2026-08-05) — Vision box (poolAI vision canon mirror)
- **PH-S1729** `GSV/src/boxes/vision.rs` + `boxes/mod.rs` + Cargo `[[bin]]` — serde-структури
  (manifest nodes/edges/layers + feed) та реєстрація боксу.
- **PH-S1730** manifest wire: read `GSV/docs/vision/manifest.json` → `GSV/data/gsv_manifest.json`;
  `GET /api/vision/manifest` (nodes/edges/layers).
- **PH-S1731** feed wire: `GSV/docs/vision/feed.json` → `GSV/data/gsv_feed.json`; `GET /api/vision/feed`.
- **PH-S1732** `GSV/src/bin/gsv_vision_sync.rs` — mirror + `--check` drift gate (source parse +
  revision parity). Live: rev 458, 1218 nodes, 535 edges, 12 feed items.
- **PH-S1733** Vision UI card (`ui/index.html`): summary + sprint feed ticker; ratio-safe (без 161 KB legacy JS).
- **PH-S1734** `tests/gsv_vision_contracts.rs` — 7 integration contracts.
- **PH-S1735** `GSV/docs/VISION.md` + `GSV_MIGRATION.md` rows ✅ + MEMORY mark.
- **PH-S1736** poolAI vision parity (`GSV/docs/vision/README.md` + cross-check).
- **PH-S1737** Ratio hold advisory (`gsv-loc-audit --min-ratio 0.95 --advisory`).
- **PH-S1738** Band close: ratio hold, fmt, clippy, cargo test, docs canon, vision-sync, push.

### Band 110 (PH-S1739…S1748, ✅ 2026-08-05) — Vision map UI (svg + map wire)
- **PH-S1739** `boxes/vision.rs` `map_report`/`wire_map` — легкий map-звіт (layers L0..L5 z-sorted,
  `node_count`/`edges_from`, `edge_kinds` tally, totals); `GET /api/vision/map`.
- **PH-S1740** `GSV/ui/vision.svg` (порт `GSV/docs/vision/vision.svg`, include_str!) + `GET /assets/vision.svg`
  (`image/svg+xml`). `.svg` = audit Ignored → ratio-neutral.
- **PH-S1741** Vision Map card у `ui/index.html`: per-layer chips + edge kinds + посилання на svg.
- **PH-S1742** `tests/gsv_vision_contracts.rs` +3 → **10** (map endpoint: 6 layers z-sorted, layer_sum;
  feed `?status=closed`; `/assets/vision.svg` 200 + content-type).
- **PH-S1743** `GET /api/vision/feed?status=closed|open|all` — серверний фільтр.
- **PH-S1744** `GSV/docs/VISION.md` (map/feed-filter/svg) + `GSV_MIGRATION.md` rows ✅ (svg, map, feed filter).
- **PH-S1745** poolAI vision parity + `GSV_TECH_ROADMAP.md` band 110 row.
- **PH-S1746** Ratio hold advisory (`gsv-loc-audit --min-ratio 0.95 --advisory`).
- **PH-S1747** vision-sync close: `gsv-vision-sync` refresh + poolAI vision rev **461**.
- **PH-S1748** Band close: ratio hold, fmt, clippy, cargo test, docs canon, vision-sync, push.

### Band 111 (PH-S1749…S1758, ✅ 2026-08-05) — sprint-map + doc-preview (Vision UI логіка)
- **PH-S1749** `boxes/vision.rs` `SprintMapReport`/`SprintNode` structs + `sprint_map_report`/`wire_sprint_map` —
  sprint-queue map (scoping/tracking edges: `sprint-scope`+`queue`+`session-tracks` → links з `NodeRef`,
  per-node targets tally → `modules`, kinds/layers stats); `GET /api/vision/sprint-map`.
- **PH-S1750** `DocPreviewReport`/`LinkTarget` structs + `doc_preview`/`wire_doc_preview` — node + 1-hop
  neighbors (`links_out`/`links_in`); `GET /api/vision/doc-preview?id=<node>` (missing → `ok:false` + error).
- **PH-S1751** `tests/gsv_vision_contracts.rs` sprint-map contracts (endpoint kinds ⊆
  {sprint-scope,queue,session-tracks}, real-workspace report + module ids) → 12.
- **PH-S1752** doc-preview contracts (endpoint node/link_count, missing+empty params, real-workspace
  1-hop read) → **14**.
- **PH-S1753** Sprint Map card у `ui/index.html`: modules/kinds/links (details), rev/next/last header.
- **PH-S1754** Doc Preview card: node id input (`galaxy_grid` default) + out/in links + sections/path.
- **PH-S1755** `GSV/docs/VISION.md` (sprint-map/doc-preview API) + `MEMORY.md` band 111 + HANDOFF/NEXT_SESSION.
- **PH-S1756** poolAI parity: `GSV/docs/gsv/GSV_MIGRATION.md` row 21 ✅, `GSV/docs/vision/README.md`,
  `GSV_TECH_ROADMAP.md` band 111.
- **PH-S1757** Ratio hold advisory (`gsv-loc-audit --min-ratio 0.95 --advisory`) + poolAI parity hold.
- **PH-S1758** Band close: ratio hold, fmt, clippy, cargo test, docs canon, vision-sync, push.

### Band 112 (PH-S1759…S1768, ✅ 2026-08-05) — vision auto-sync + sprint-queue planning
- **PH-S1759** `boxes/vision.rs` `Extensions` struct (active_sprint/revision/ui_version/updated_at +
  opaque `scopes` map) + `read_extensions`/`save_extensions`/`load_extensions`/`source_extensions` +
  `extensions_source`/`extensions_target` paths; `sync()` також мірорить `gsv_extensions.json`;
  `SyncReport` + extensions_source/target; `gsv-vision-sync` bin друкує extensions target;
  `collect_drift` парсить extensions. `wire_extensions` → `GET /api/vision/extensions`
  (active_sprint, revision, ui_version, scope_count + sorted scopes).
- **PH-S1760** `wire_sync` → `GET /api/vision/sync` — auto-sync: re-mirror canon у знімки +
  drift gate у відповіді (`drift: []` = зелено); route у `server/mod.rs`.
- **PH-S1761** `SprintQueueReport`/`sprint_queue_report`/`wire_sprint_queue` → `GET /api/vision/sprint-queue` —
  manifest.sprint_queue → `entries`/`open_count`, extensions.active_sprint → `active_sprint`,
  `planned` = entries ∪ активний спринт.
- **PH-S1762** `tests/gsv_vision_contracts.rs` — extensions contracts (real-workspace read:
  revision>0, scopes present, active == manifest.next; sync snapshot now also asserts
  `gsv_extensions.json` + extensions revision parity) → 17.
- **PH-S1763** sprint-queue + sync contracts (`/api/vision/sync` ok + empty drift + synced_at;
  `/api/vision/sprint-queue` ok + active == next + planned includes active; real-workspace
  `sprint_queue_report`) → **19**.
- **PH-S1764** UI cards у `ui/index.html`: **Vision Sync card** (Resync snapshot button + drift status)
  та **Sprint Queue card** (rev/next/last + active/open + planned details).
- **PH-S1765** `GSV/docs/VISION.md` (sync/extensions/sprint-queue API + sync док-секції) +
  `MEMORY.md` band 112 + HANDOFF/NEXT_SESSION.
- **PH-S1766** poolAI parity: `GSV/docs/gsv/GSV_MIGRATION.md` rows ✅, `GSV/docs/vision/README.md`,
  `GSV_TECH_ROADMAP.md` band 112, FM §5.93, poolAI HANDOFF/NEXT.
- **PH-S1767** Ratio hold advisory (`gsv-loc-audit --min-ratio 0.95 --advisory` → 95.56%) +
  poolAI parity hold.
- **PH-S1768** Band close: ratio hold, fmt, clippy, cargo test (118), docs canon, vision-sync, push.

### Band 113 (PH-S1769…S1778, ✅ 2026-08-05) — Galaxy UI: node search + interactive map
- **PH-S1769** `boxes/vision.rs` `NodeSearchReport`/`NodeSearchResult` structs + `node_search`/
  `wire_node_search` — case-insensitive match по id/label/path/sections, `top-N 25`
  (`NODE_SEARCH_LIMIT`) layer-z-sorted, `links_out`/`links_in` tallies;
  `GET /api/vision/node-search?q=&layer=` route + handler у `server/mod.rs`.
- **PH-S1770** `tests/gsv_vision_contracts.rs` node-search contracts (real-workspace
  id/label/path/links + layer-z sort, layer filter, no-match empty + cap) → 22.
- **PH-S1771** `tests/gsv_server_contracts.rs` node-search endpoint contract
  (ok + revision + results + links_out/in u64; empty `q` → ok true) → 19.
- **PH-S1772** Vision Map card рендерить **inline** `assets/vision.svg` (`<img>` через
  `GET /assets/vision.svg`) + chips/kinds.
- **PH-S1773** Layer filter + search UX у `ui/index.html`: клікабельні layer chips
  (active filter → `toggleMapLayer`), node-search input + results table
  (`searchVisionNodes`) → deep-link у Doc Preview (`openSearchNode`).
- **PH-S1774** `GSV/docs/VISION.md` (node-search API + інтерактивна мапа) + `MEMORY.md` band 113 +
  HANDOFF/NEXT_SESSION.
- **PH-S1775** poolAI parity: `GSV/docs/gsv/GSV_MIGRATION.md` row ✅, `GSV/docs/vision/README.md`,
  `GSV_TECH_ROADMAP.md` band 113, FM §5.94.
- **PH-S1776** Ratio hold advisory (`gsv-loc-audit --min-ratio 0.95 --advisory`) + poolAI parity hold.
- **PH-S1777** vision-sync close: `gsv-vision-sync` refresh + poolAI vision rev++.
- **PH-S1778** Band close: ratio hold, fmt, clippy, cargo test, docs canon, vision-sync, push.

### Band 114 (PH-S1779…S1788, ✅ 2026-08-05) — GSV Sprint-board + progress UI
- **PH-S1779** `boxes/vision.rs` `SprintBoardReport`/`SprintBoardColumn` structs +
  `sprint_board_report`/`wire_sprint_board` — доска зі спільного `planned` queue:
  columns open/closed/planned (active або `open` → open; `closed`/`done` → closed; решта → planned),
  counts + `progress_pct` = closed/total; `GET /api/vision/sprint-board` route + handler
  у `server/mod.rs`.
- **PH-S1780** `SprintProgressReport`/`SprintLayerProgress` structs +
  `sprint_progress_report`/`wire_sprint_progress` — status counts + per-layer розподіл
  (`node_count`/`linked_count` проти чергових спринтів, z-ascending);
  `GET /api/vision/sprint-progress` route + handler.
- **PH-S1781** `tests/gsv_vision_contracts.rs` sprint-board contracts (grouping, progress pct
  formula, column order, active in open, unique across columns, closed-only-done, revision parity,
  wire ok) → 30.
- **PH-S1782** sprint-progress contracts (layers match manifest + node sums, statuses sum,
  z-ordered, linked reflects queue sprints, planned formula, wire ok) → **38**.
- **PH-S1783** `tests/gsv_server_contracts.rs` sprint-board + sprint-progress endpoint contracts
  (ok + status sums + columns/layers shape) → 21.
- **PH-S1784** Sprint Board card у `ui/index.html`: progress bar + open/closed/planned
  колонки-details (`bar()` helper).
- **PH-S1785** Sprint Progress card: progress bar + per-layer таблиця nodes/linked.
- **PH-S1786** `GSV/docs/VISION.md` (sprint-board/sprint-progress API + band 114 section) +
  `MEMORY.md` band 114 + HANDOFF/NEXT_SESSION.
- **PH-S1787** poolAI parity: `GSV/docs/gsv/GSV_MIGRATION.md` rows ✅, `GSV/docs/vision/README.md`,
  `GSV_TECH_ROADMAP.md` band 114, FM §5.95.
- **PH-S1788** Band close: ratio hold (**95.02%**), fmt, clippy 0, cargo test (140), docs canon,
  vision-sync rev 467, push.

### Band 115 (PH-S1789…S1798, ✅ 2026-08-07) — GSV migration completion (legacy vision supersession)
- **PH-S1789** `GSV/docs/LEGACY_PARITY.md` — parity audit: кожна legacy-панель
  (`GSV/docs/vision/index.html`: layers/queue/map/speeds/rust/links/preview + chrome) → GSV
  endpoint+card / superseded / out-of-scope. Єдині прогалини: Speeds + Rust diagnostics.
- **PH-S1790** `SpeedIndexReport`/`SpeedIndexLatest` structs + `read_speed_index`/
  `save_speed_index`/`load_speed_index`/`source_speed_index` (live → snapshot → empty default) +
  `wire_speed_index` → `GET /api/vision/speeds` (route + handler у `server/mod.rs`).
- **PH-S1791** `RustDiagnosticsReport`/`RustDiagLatest` + `read_rust_diagnostics`/
  `save_rust_diagnostics`/`load_rust_diagnostics`/`source_rust_diagnostics` +
  `wire_rust_diagnostics` → `GET /api/vision/rust-diagnostics`.
- **PH-S1792** contracts: `tests/gsv_vision_contracts.rs` (real-workspace speed_index/
  rust_diagnostics reads + wire shapes) + `tests/gsv_server_contracts.rs`
  (`/api/vision/speeds` + `/api/vision/rust-diagnostics` 200/ok/present/shape).
- **PH-S1793** Speed Index card + Rust Diagnostics card у `ui/index.html` (present/empty
  states, latest metrics, top clippy codes).
- **PH-S1794** `GSV/docs/gsv/GSV_MIGRATION.md` rows ✅ (speed_index/rust_diagnostics moved;
  `vision.js`/`vision.css` superseded) + `GSV/docs/gsv/GSV_TECH_ROADMAP.md` band 115.
- **PH-S1795** `GSV/docs/VISION.md` +band 115 endpoints/section; `MEMORY.md` band 115;
  HANDOFF/NEXT band 115.
- **PH-S1796** poolAI parity: FM §5.12 §5.96, HANDOFF/NEXT band 115, `GSV/docs/vision/` canon.
- **PH-S1797** ratio hold advisory: `gsv-loc-audit` **95.04%**; legacy JS не переносимо.
- **PH-S1798** Band close: ratio hold (**95.04%**), fmt, clippy 0, cargo test (150), docs canon,
  vision-sync rev 468, push.

### Band 116 (PH-S1799…S1808, ✅ 2026-08-07) — GSV history charts (speed/rust analytics)
- **PH-S1799** FM §5.97 queue (band 116) + manifest sync (10 open).
- **PH-S1800** typed `SpeedTestCiRecord`/`SpeedBenchRecord` + `test_ci_history`/`bench_history`
  у `SpeedIndexReport`; `read_speed_index`/`SpeedIndexFile` carry history (source fallback unchanged).
- **PH-S1801** typed `RustDiagRecord` + `history` у `RustDiagnosticsReport`; `read_rust_diagnostics`
  carry history (source fallback unchanged).
- **PH-S1802** vision tests 20 → **23**: `history_records_parse_typed_fields`,
  `speed_chart_svg_renders_bars_and_empty_state`, `rust_chart_svg_renders_bars_and_empty_state`
  (+ `data_dir_of` helper).
- **PH-S1803** `speed_index_chart_svg` + `/api/vision/speeds.svg` (Rust-rendered SVG: test-ci
  wall bars green ok / red fail, ≤24 runs, footer latest bench) + `<img id="i-speed-chart">`.
- **PH-S1804** `rust_diagnostics_chart_svg` + `/api/vision/rust-diagnostics.svg` (warnings
  orange + errors red grouped bars, command footer) + `<img id="i-rust-chart">`.
- **PH-S1805** stand smoke: `/api/vision/speeds.svg` + `/api/vision/rust-diagnostics.svg` →
  200 `image/svg+xml`; `poolai-ui-wasm` defer row у `GSV_MIGRATION.md` + roadmap.
- **PH-S1806** `GSV/docs/VISION.md` +band 116 section/endpoints; MEMORY band 116; HANDOFF/NEXT band 116.
- **PH-S1807** poolAI parity: `GSV/docs/vision/README.md`; FM §5.12 §5.97; `GSV_TECH_ROADMAP.md` band 116;
  poolAI HANDOFF/NEXT band 116.
- **PH-S1808** Band close: ratio hold (**95.26%**), fmt, clippy 0, cargo test (153), docs canon,
  vision-sync rev 469, push.

### Band 117 (PH-S1809…S1818, ✅ 2026-08-07) — GSV legacy vision deactivation
- **PH-S1809** FM §5.98 queue (band 117) + manifest sync (10 open, rev 469).
- **PH-S1810** `GSV/docs/vision/index.html` → minimal GSV pointer page (no `vision.js`/`vision.css` refs).
- **PH-S1811** `vision.js`/`vision.css` DEACTIVATED banner (band 117); `GSV/docs/vision/README.md` deactivation note.
- **PH-S1812** live link retarget: poolai-vision-sync feed links → `http://127.0.0.1:8891/#b-sprint-board`;
  GSV `vision.rs` sample feed links; RUN_LOCAL/GSV_SERVER/docs-gsv README/SPEED_INDEX/RUST_DIAGNOSTICS → GSV.
- **PH-S1813** legacy test retirement: `poolai_vision_sync.rs` unit ×4 + `galaxy_horizon_s1011/s1019/s1039`
  → deactivated pointer state; e2e `vision.spec.ts`/`a11y.spec.ts` pointer assertions; `VISION_MAP_BAND40_ROWS` markers.
- **PH-S1814** `LEGACY_PARITY.md` + `GSV_MIGRATION.md` band 117 (index/JS/CSS deactivated).
- **PH-S1815** `VISION.md`/`MEMORY.md`/HANDOFF/NEXT band 117.
- **PH-S1816** poolAI parity: `GSV/docs/vision/README.md`; FM §5.12 §5.98; `GSV_TECH_ROADMAP.md` band 117;
  poolAI HANDOFF/NEXT band 117.
- **PH-S1817** ratio hold advisory (**95.26%**) + vision-sync rev 470 (poolai + gsv + --check).
- **PH-S1818** Band close: ratio hold, fmt, clippy 0, cargo test (poolAI test-ci + GSV 153), docs canon,
  vision-sync rev 470, push.

### Band 118 (PH-S1819…S1828, ✅ 2026-08-08) — GSV sprint UI migration (theme + focus map)
- **PH-S1819** FM §5.99 queue (band 118 sprint UI migration) + §5.12 header (master horizon).
- **PH-S1820** `SprintThemeReport`/`SprintPillTheme`/`SprintChipTheme`/`SprintQueueStateTheme`/
  `SprintLayerColor`/`SprintEdgeKindColor` structs + `sprint_theme_report`/`wire_sprint_theme` →
  `GET /api/vision/sprint-theme` (sprint `#a78bfa`/next `#c4b5fd`, pill/chip/queue colors,
  layer L0–L5 + edge-kind palettes, revision/git_head/active/next).
- **PH-S1821** `sprint_token_matches`/`path_matches_glob`/`nodes_for_sprint` +
  `sprint_focus_svg` → `GET /api/vision/sprint-focus.svg?sprint=` (sprint-dim: in-scope accent,
  out-of-scope opacity 0.22/text 0.28, edges tinted; default active sprint; empty-state).
- **PH-S1822** contracts: `gsv_vision_contracts` (theme real-workspace + wire shapes + focus svg
  highlight/dim/empty) → 44; `gsv_server_contracts` (theme + focus endpoints, `get_text` helper) → 25.
- **PH-S1823** `GSV/ui/index.html`: `--sprint*` CSS-змінні + sprint-pill/queue-state chips у
  Sprint Queue/Board cards; Sprint Focus card (input + button + `<img id="i-sprint-focus">`);
  `loadSprintTheme` apply + `loadSprintFocus` ре-запит svg.
- **PH-S1824** `GSV/docs/VISION.md` +band 118 (theme/focus endpoints + section); `MEMORY.md` band 118;
  GSV HANDOFF/NEXT band 118.
- **PH-S1825** poolAI parity: `GSV/docs/vision/README.md`; FM §5.12 §5.99; `GSV_TECH_ROADMAP.md` band 118.
- **PH-S1826** Ratio hold advisory: `gsv-loc-audit --min-ratio 0.95 --advisory` → **95.35%** +
  poolAI ratio96 advisory hold.
- **PH-S1827** vision-sync close: `poolai-vision-sync` rev **471**; `--check` ok; feed/manifest updated.
- **PH-S1828** Band close: ratio hold (95.35%), fmt, clippy 0, cargo test (163), docs canon,
  vision-sync rev 471, push.

### Band 119 (PH-S1829…S1838, ✅ 2026-08-08) — GSV Galaxy UI full parity (colors + box behaviors)
- **PH-S1829** `GSV/docs/gsv/GSV_TECH_ROADMAP.md` band 119 (PH-S1829…S1838): full `vision.css`
  `:root` palette + header chrome (ticker, GPU modes, power menu) + panel dock/collapse/
  fullscreen + starfield/galaxy backdrop scope.
- **PH-S1830** `GalaxyPalette` struct (bg-deep/bg/panel/panel-solid/border/border-bright/
  text/muted/accent/accent-2/glow/sidebar-w, `layers`+`layers_dim` L0–L5, `edge_docs`/
  `edge_code`/`edge_toml`, `ext_md`/`ext_rs`/`ext_json`/`ext_toml`, `sprint`, `bg_tone`,
  `galaxy_bg_opacity`) + `wire_palette` → `GET /api/vision/palette` (exact legacy `:root`
  values + `ok`/`revision`).
- **PH-S1831** `StarfieldMode`/`starfield_svg` (deterministic LCG per mode; eco sparse/static,
  fx dense+glow, ms medium) → `GET /api/vision/starfield.svg?mode=eco|fx|ms` (`image/svg+xml`).
- **PH-S1832** `galaxy_svg` (radial nebula gradients + spiral-arm ellipses) →
  `GET /api/vision/galaxy.svg` (`image/svg+xml`).
- **PH-S1833** header chrome: RSS ticker (`loadRssTicker` → `/api/vision/feed?status=all`,
  duplicated track), GPU mode button (`btnGpu` Eco/FX/Ms cycle → `body.vision-(eco|fx|ms)` +
  starfield re-request), power menu (`powerSoft` → `/api/vision/sync`, `powerReload` → resync,
  `powerOffline` → forced offline), meta-rev/meta-trail.
- **PH-S1834** panel dock + Esc-fullscreen: card `–` collapse → `syncDock()` chips (restore);
  `□` fullscreen + `Esc` exits; `.galaxy-backdrop` `<img>` + `#starfield` fixed backdrops
  (ratio-safe `.svg`).
- **PH-S1835** contracts: `gsv_vision_contracts` (palette == legacy `:root`, starfield/galaxy
  svg shape + mode variance, empty-state) → **50**; `gsv_server_contracts` (palette +
  starfield + galaxy 200 + `image/svg+xml`) → **29**.
- **PH-S1836** `GSV/docs/VISION.md` +band 119 (palette/starfield/galaxy/header UI) +
  `LEGACY_PARITY.md` rows migrated; MEMORY band 119; HANDOFF/NEXT band 119.
- **PH-S1837** Ratio hold advisory: `gsv-loc-audit` **95.18%** (UI delta компенсовано Rust
  tests; JS compact); vision-sync rev **472** (poolai + gsv + `--check`).
- **PH-S1838** Band close: ratio hold (95.18%), fmt, clippy 0, cargo test (**183**), docs canon,
  vision-sync rev 472, push.

### Band 120 (PH-S1839…S1848, ✅ 2026-08-08) — GSV Ratio 96% stretch
- **PH-S1839** `GSV/docs/gsv/GSV_TECH_ROADMAP.md` band 120 (PH-S1839…S1848): ratio **95.18% → ≥96%**
  via `gsv-loc-audit --stretch-96` advisory + server-rendered UI card fragments
  (`GET /api/ui/card/:name`, Rust HTML renderers) + compact UI (JS/CSS).
- **PH-S1840** `--stretch-96` advisory: `ratio.rs` `STRETCH_96_TARGET = 0.96` + `AuditConfig.stretch_96`
  + `RustRatioReport.stretch_target`/`meets_stretch_96` (`#[serde(default)]` — старий `rust_ratio.json`
  читається); `gsv_loc_audit.rs` `--stretch-96` flag → advisory (exit 0).
- **PH-S1841** `gsv_ratio_contracts` — roundtrip + JSON shape + wire stretch fields.
- **PH-S1842** `boxes/ui.rs`: `esc`/`tab`/`bar` helpers + 12 Rust renderers (tracker/sli/toolchain/
  ratio/hooks-tests/hooks-bench/sprint-map/sprint-queue/sprint-progress/sprint-board/speed-index/
  rust-diagnostics) + `render_card` dispatch + `CARD_NAMES` (12).
- **PH-S1843** `GET /api/ui/card/{name}` в `server/mod.rs` (`api_ui_card` handler; 404 unknown).
- **PH-S1844** `ui/index.html` thin glue: `getText(card)` → `api/ui/card/:name` + `rustCards` (12);
  8 JS renderers видалено.
- **PH-S1845** `gsv_ui_contracts` (6) + `gsv_server_contracts` (**30**, `ui_card_endpoint_renders_fragment_and_rejects_unknown`).
- **PH-S1846** Ratio 96% measurement: `gsv-loc-audit --stretch-96` → **96.51%** (rust 10027 / product 10390) ✅.
- **PH-S1847** GSV docs canon: MEMORY band 120; HANDOFF/NEXT band 120; `GSV_TECH_ROADMAP.md` band 120.
- **PH-S1848** Band close: ratio **≥96%**; fmt/clippy 0; cargo test (**204**); docs canon; vision-sync rev bump; push.

### Band 121 (PH-S1849…S1855, ✅ 2026-08-10) — GSV OmniRouter box parity
- **PH-S1849** `GSV/docs/gsv/GSV_TECH_ROADMAP.md` band 121 (PH-S1849…S1855): port the last hand-rolled
  JS card renderer (`renderOmni`) to the Rust UI fragment box — `GET /api/ui/card/omni`, `CARD_NAMES` 13.
- **PH-S1850** `boxes/ui.rs`: `render_omni` (summary/routing + recommended + providers table +
  models table) + `format_number` (grouping) + `render_card`/`CARD_NAMES` 13 + 2 unit tests.
- **PH-S1851** `server/mod.rs` `api_ui_card`: `"omni"` → `boxes::omni::wire`; `ui/index.html`:
  `renderOmni` JS видалено, `rustCards` 13, `resync()` url drop (test control залишено).
- **PH-S1852** `gsv_ui_contracts` (7: `ui_card_omni_renders_summary_providers_models`) +
  `gsv_server_contracts` (omni card endpoint 200 + markers).
- **PH-S1853** Ratio hold: `gsv-loc-audit --stretch-96` → **96.73%** (rust 10191 / product 10536) ✅;
  cargo test (**207**); clippy 0; fmt clean.
- **PH-S1854** GSV docs canon: MEMORY band 121; HANDOFF/NEXT band 121; VISION.md omni card section;
  `GSV_TECH_ROADMAP.md` band 121.
- **PH-S1855** Band close: ratio **≥96%**; fmt/clippy 0; cargo test (**207**); docs canon; vision-sync rev bump; push.

## Важливі факти (не забувати)

1. **GSV — окремий Rust-проєкт** у `S:\rust\poolAI\GSV` (own workspace, own `target/`).
2. **Ratio аудит іде по git-tracked файлах** репо poolAI під префіксом `GSV/` (не `GSV/target/`, не `data/`).
   git-топ має MSYS-стиль `/s/rust/poolAI` — нормалізуємо в `S:/rust/poolAI` (`normalize_git_root`).
3. **Запущений `gsv-server` блокує `target/debug/gsv-server.exe`** → `cargo test`/`build` падає
   з `Access is denied (os error 5)` → спочатку зупинити сервер.
4. **Data dir:** `GSV/data/*` gitignored (омні-конфіг, rust_ratio.json, трекер). Запуск:
   `--repo-root S:/rust/poolAI --data-dir S:/rust/poolAI/GSV/data --port 8891`.
5. **Збірка:** terminal MSYS2 bash; PATH префікс `C:\Users\plati\.cargo\bin`.
6. **OmniRouter** прокидає через OpenAI-сумісний proxy; dry-run заголовок `X-Omni-Dry-Run: 1` —
   жодного реального мережевого запиту в тестах.
7. **UI канон:** тонкий JS/DOM glue; якщо ratio падає <95% — **compact UI/CSS**, не Rust-обхід.
